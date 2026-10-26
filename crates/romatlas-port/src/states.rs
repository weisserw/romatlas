//! Bounded, checksummed native snapshots. Battery files use a separate format.
use crate::{Game, audio::SampleClock};
use anyhow::{Result, ensure};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const MAX_BYTES: usize = 4 * 1024 * 1024;
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct State {
    version: u32,
    identity: String,
    payload: String,
    checksum: String,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Payload {
    game: String,
    samples: SampleClock,
}
pub fn capture(game: &impl Game, samples: &SampleClock, identity: &str) -> Result<Vec<u8>> {
    let payload = serde_json::to_string(&Payload {
        game: game.snapshot_json()?,
        samples: samples.clone(),
    })?;
    let state = State {
        version: 1,
        identity: identity.into(),
        checksum: format!("{:x}", Sha256::digest(payload.as_bytes())),
        payload,
    };
    let bytes = serde_json::to_vec(&state)?;
    ensure!(bytes.len() <= MAX_BYTES, "snapshot exceeds 4 MiB");
    Ok(bytes)
}
pub fn restore(
    game: &mut impl Game,
    samples: &mut SampleClock,
    identity: &str,
    bytes: &[u8],
) -> Result<()> {
    ensure!(bytes.len() <= MAX_BYTES, "snapshot exceeds 4 MiB");
    let state: State = serde_json::from_slice(bytes)?;
    ensure!(
        state.version == 1 && state.identity == identity,
        "snapshot belongs to a different game, build or assets"
    );
    ensure!(
        state.checksum == format!("{:x}", Sha256::digest(state.payload.as_bytes())),
        "snapshot checksum mismatch"
    );
    let payload: Payload = serde_json::from_str(&state.payload)?;
    ensure!(
        payload.samples.snapshot_compatible(samples),
        "snapshot audio configuration mismatch"
    );
    game.restore_snapshot_json(&payload.game)?;
    *samples = payload.samples;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{graphics::Frame, input::InputState, timing::TickRate};
    struct Counter(u64);
    impl Game for Counter {
        fn tick(&mut self, _: &InputState) {
            self.0 += 1;
        }
        fn render(&self, _: &mut Frame) {}
        fn snapshot_json(&self) -> Result<String> {
            Ok(self.0.to_string())
        }
        fn restore_snapshot_json(&mut self, json: &str) -> Result<()> {
            let value = json.parse()?;
            self.0 = value;
            Ok(())
        }
    }
    #[test]
    fn native_state_restores_sample_phase_without_replay_and_rejects_corruption() -> Result<()> {
        let mut game = Counter(123);
        let mut samples = SampleClock::new(TickRate::NES_NTSC, 48000)?;
        for _ in 0..19 {
            samples.next_tick();
        }
        let saved = capture(&game, &samples, "build+rom+assets")?;
        let expected = (0..100).map(|_| samples.next_tick()).collect::<Vec<_>>();
        game.0 = 999;
        restore(&mut game, &mut samples, "build+rom+assets", &saved)?;
        assert_eq!(game.0, 123);
        assert_eq!(
            (0..100).map(|_| samples.next_tick()).collect::<Vec<_>>(),
            expected
        );
        let before = serde_json::to_string(&samples)?;
        let mut corrupt: serde_json::Value = serde_json::from_slice(&saved)?;
        corrupt["payload"] = "broken".into();
        for bytes in [
            serde_json::to_vec(&corrupt)?,
            b"broken".to_vec(),
            vec![0; MAX_BYTES + 1],
        ] {
            assert!(restore(&mut game, &mut samples, "build+rom+assets", &bytes).is_err());
            assert_eq!(game.0, 123);
            assert_eq!(serde_json::to_string(&samples)?, before);
        }
        assert!(restore(&mut game, &mut samples, "different build", &saved).is_err());
        let mut other_rate = SampleClock::new(TickRate::NES_NTSC, 44100)?;
        assert!(restore(&mut game, &mut other_rate, "build+rom+assets", &saved).is_err());
        let payload = serde_json::to_string(&Payload {
            game: "invalid game".into(),
            samples: samples.clone(),
        })?;
        let invalid_game = State {
            version: 1,
            identity: "build+rom+assets".into(),
            checksum: format!("{:x}", Sha256::digest(payload.as_bytes())),
            payload,
        };
        assert!(
            restore(
                &mut game,
                &mut samples,
                "build+rom+assets",
                &serde_json::to_vec(&invalid_game)?
            )
            .is_err()
        );
        assert_eq!(game.0, 123);
        assert_eq!(serde_json::to_string(&samples)?, before);
        Ok(())
    }
}
