//! Optional bounded local JSONL event stream. Independent of emulator state.
use romatlas_core::TraceEvent;
use serde::Serialize;
use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::Path,
};
#[derive(Clone, Debug, Serialize)]
pub struct StreamStatus {
    pub records: u64,
    pub bytes: u64,
    pub max_bytes: u64,
    pub dropped: u64,
    pub stopped: bool,
    pub error: Option<String>,
}
pub(crate) struct Stream {
    file: File,
    pub status: StreamStatus,
}
impl Stream {
    pub fn open(path: &Path, max_bytes: u64) -> Result<Self, String> {
        if !(1..=67108864).contains(&max_bytes) {
            return Err("stream max_bytes must be 1..67108864".into());
        }
        let file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)
            .map_err(|e| e.to_string())?;
        Ok(Self {
            file,
            status: StreamStatus {
                records: 0,
                bytes: 0,
                max_bytes,
                dropped: 0,
                stopped: false,
                error: None,
            },
        })
    }
    pub fn capture(&mut self, event: &TraceEvent) {
        if self.status.stopped {
            self.status.dropped += 1;
            return;
        }
        #[derive(Serialize)]
        struct Record<'a> {
            sequence: u64,
            event: &'a TraceEvent,
        }
        let mut buffer = RecordBuffer(Vec::new());
        if let Err(e) = serde_json::to_writer(
            &mut buffer,
            &Record {
                sequence: self.status.records,
                event,
            },
        ) {
            self.status.stopped = true;
            self.status.error = Some(e.to_string());
            self.status.dropped += 1;
            return;
        }
        let mut record = buffer.0;
        record.push(b'\n');
        if record.len() as u64 > self.status.max_bytes - self.status.bytes {
            self.status.stopped = true;
            self.status.dropped += 1;
            return;
        }
        match self.file.write_all(&record) {
            Ok(()) => {
                self.status.bytes += record.len() as u64;
                self.status.records += 1;
            }
            Err(e) => {
                self.status.error = Some(e.to_string());
                self.status.stopped = true;
                self.status.dropped += 1;
            }
        }
    }
}

struct RecordBuffer(Vec<u8>);
impl Write for RecordBuffer {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        if self.0.len() + bytes.len() > 65535 {
            return Err(std::io::Error::other("stream record exceeds 65536 bytes"));
        }
        self.0.extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use romatlas_core::{Address, EventKind};
    use std::collections::BTreeMap;
    #[test]
    fn oversized_opaque_record_stops_before_disk_write() {
        let path =
            std::env::temp_dir().join(format!("romatlas-stream-large-{}", std::process::id()));
        let mut stream = Stream::open(&path, 1000000).unwrap();
        let event = TraceEvent {
            kind: EventKind::Read,
            cycle: 0,
            address: Address {
                space: "wide-mock".into(),
                offset: 1 << 40,
                context: BTreeMap::new(),
            },
            target: None,
            value: None,
            context: serde_json::json!({"payload":"x".repeat(100000)}),
        };
        stream.capture(&event);
        assert!(stream.status.stopped);
        assert!(
            stream
                .status
                .error
                .as_ref()
                .unwrap()
                .contains("record exceeds")
        );
        assert_eq!(stream.status.records, 0);
        assert_eq!(std::fs::metadata(&path).unwrap().len(), 0);
        drop(stream);
        std::fs::remove_file(path).unwrap();
    }
}
