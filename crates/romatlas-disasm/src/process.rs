//! Bounded external commands on the supported Unix toolchain host.
use anyhow::{Result, bail, ensure};
use std::{
    io::Read,
    os::unix::process::CommandExt,
    process::{Child, Command, Stdio},
    thread,
    time::{Duration, Instant},
};
const OUTPUT_LIMIT: usize = 65536;
pub struct Budget {
    deadline: Instant,
}
impl Budget {
    pub fn new(timeout_ms: u64) -> Result<Self> {
        ensure!(
            (1..=300000).contains(&timeout_ms),
            "external tool timeout must be 1..300000 ms"
        );
        Ok(Self {
            deadline: Instant::now() + Duration::from_millis(timeout_ms),
        })
    }
    pub fn check(&self) -> Result<()> {
        ensure!(
            Instant::now() < self.deadline,
            "external tool deadline exhausted"
        );
        Ok(())
    }
    pub fn run(&self, command: &mut Command) -> Result<String> {
        ensure!(
            Instant::now() < self.deadline,
            "external tool deadline exhausted"
        );
        command
            .process_group(0)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let mut child = Guard(command.spawn()?);
        let stdout = child.0.stdout.take().unwrap();
        let stderr = child.0.stderr.take().unwrap();
        let out = thread::spawn(move || capture(stdout));
        let err = thread::spawn(move || capture(stderr));
        let mut timeout = false;
        let status = loop {
            if let Some(status) = child.0.try_wait()? {
                break status;
            }
            if Instant::now() >= self.deadline {
                timeout = true;
                child.kill_group();
                break child.0.wait()?;
            }
            thread::sleep(Duration::from_millis(10));
        };
        // A tool can exit while descendants still hold its output pipes.
        child.kill_group();
        let output = out
            .join()
            .map_err(|_| anyhow::anyhow!("stdout reader failed"))??;
        let error = err
            .join()
            .map_err(|_| anyhow::anyhow!("stderr reader failed"))??;
        if timeout {
            bail!("external tool timeout; process group stopped: {command:?}\n{output}\n{error}");
        }
        if !status.success() {
            bail!("tool failed ({status}): {output}\n{error}");
        }
        Ok(output)
    }
}
struct Guard(Child);
impl Guard {
    fn kill_group(&mut self) {
        // Each spawned command owns this process group; this cannot signal the caller's group.
        unsafe {
            libc::kill(-(self.0.id() as libc::pid_t), libc::SIGKILL);
        }
    }
}
impl Drop for Guard {
    fn drop(&mut self) {
        self.kill_group();
        let _ = self.0.wait();
    }
}
fn capture(mut stream: impl Read) -> std::io::Result<String> {
    let mut data = Vec::new();
    let mut total = 0usize;
    let mut buffer = [0u8; 8192];
    loop {
        let n = stream.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        total = total.saturating_add(n);
        let keep = n.min(OUTPUT_LIMIT.saturating_sub(data.len()));
        data.extend_from_slice(&buffer[..keep]);
    }
    let mut text = String::from_utf8_lossy(&data).into_owned();
    if total > data.len() {
        text.push_str(&format!(
            "\n[{} output bytes discarded]",
            total - data.len()
        ));
    }
    Ok(text)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn noisy_failure_is_bounded() {
        let error = Budget::new(5000)
            .unwrap()
            .run(Command::new("sh").args(["-c", "yes x | head -c 200000; exit 9"]))
            .unwrap_err()
            .to_string();
        assert!(error.len() < OUTPUT_LIMIT + 1024);
        assert!(error.contains("output bytes discarded"));
    }
    #[test]
    fn deadline_stops_descendants_and_drains_pipes() {
        let start = Instant::now();
        let error = Budget::new(100)
            .unwrap()
            .run(Command::new("sh").args(["-c", "sleep 10 & wait"]))
            .unwrap_err()
            .to_string();
        assert!(error.contains("timeout"));
        assert!(start.elapsed() < Duration::from_secs(3));
    }
    #[test]
    fn budget_is_shared_across_commands() {
        let budget = Budget::new(100).unwrap();
        assert!(
            budget
                .run(Command::new("sh").args(["-c", "sleep 10"]))
                .is_err()
        );
        let error = budget
            .run(&mut Command::new("true"))
            .unwrap_err()
            .to_string();
        assert!(error.contains("deadline exhausted"));
        assert!(Budget::new(0).is_err());
    }
}
