//! Internal metadata store, separate from the portable platform export format.
use anyhow::{Result, bail, ensure};
use romatlas_core::Region;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::Path};
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Store {
    pub version: u32,
    pub platform: String,
    pub image_sha256: String,
    pub image_size: u64,
    pub regions: BTreeMap<u64, Region>,
    pub next_id: u64,
    #[serde(default)]
    pub metadata: BTreeMap<String, serde_json::Value>,
    #[serde(default)]
    pub region_metadata: BTreeMap<u64, serde_json::Value>,
}
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct Conflict {
    pub first: u64,
    pub second: u64,
    pub reason: String,
}
impl Store {
    pub fn new(platform: &str, hash: &str, size: u64) -> Result<Self> {
        let store = Self {
            version: 1,
            platform: platform.into(),
            image_sha256: hash.into(),
            image_size: size,
            regions: BTreeMap::new(),
            next_id: 1,
            metadata: BTreeMap::new(),
            region_metadata: BTreeMap::new(),
        };
        store.validate()?;
        Ok(store)
    }
    pub fn validate_identity(&self, platform: &str, hash: &str, size: u64) -> Result<()> {
        ensure!(self.platform == platform, "platform mismatch");
        ensure!(
            self.image_sha256 == hash && self.image_size == size,
            "ROM identity mismatch: SHA-256 or size differs"
        );
        Ok(())
    }
    pub fn conflicts(&self) -> Vec<Conflict> {
        let mut result = Vec::new();
        let entries: Vec<_> = self.regions.iter().collect();
        for (i, (a, x)) in entries.iter().enumerate() {
            for (b, y) in &entries[i + 1..] {
                if x.start.space == y.start.space
                    && x.start.context == y.start.context
                    && x.start.offset < y.start.offset.saturating_add(y.length)
                    && y.start.offset < x.start.offset.saturating_add(x.length)
                {
                    result.push(Conflict {
                        first: **a,
                        second: **b,
                        reason: "overlapping structural regions".into(),
                    });
                }
            }
        }
        result
    }
    pub fn validate(&self) -> Result<()> {
        ensure!(self.version == 1, "unsupported internal store version");
        ensure!(!self.platform.is_empty(), "missing platform identity");
        ensure!(
            self.image_sha256.len() == 64
                && self
                    .image_sha256
                    .bytes()
                    .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase()),
            "expected lowercase SHA-256"
        );
        ensure!(
            self.next_id > 0 && self.regions.keys().all(|id| *id > 0 && *id < self.next_id),
            "invalid region IDs"
        );
        ensure!(
            self.region_metadata
                .keys()
                .all(|id| self.regions.contains_key(id)),
            "metadata references missing region"
        );
        for r in self.regions.values() {
            validate_region(r)?;
        }
        let conflicts = self.conflicts();
        if !conflicts.is_empty() {
            bail!("conflicting regions: {conflicts:?}");
        }
        Ok(())
    }
    pub fn annotate(&mut self, region: Region) -> Result<u64> {
        validate_region(&region)?;
        let id = self.next_id;
        let next = id
            .checked_add(1)
            .ok_or_else(|| anyhow::anyhow!("region IDs exhausted"))?;
        self.regions.insert(id, region);
        self.next_id = next;
        if let Err(e) = self.validate() {
            self.regions.remove(&id);
            self.next_id = id;
            return Err(e);
        }
        Ok(id)
    }
    pub fn edit(&mut self, id: u64, region: Region) -> Result<()> {
        validate_region(&region)?;
        let old = self
            .regions
            .get(&id)
            .ok_or_else(|| anyhow::anyhow!("unknown region ID {id}"))?
            .clone();
        let changed =
            old.start != region.start || old.length != region.length || old.kind != region.kind;
        self.regions.insert(id, region);
        if let Err(e) = self.validate() {
            self.regions.insert(id, old);
            return Err(e);
        }
        if changed {
            self.region_metadata.remove(&id);
        }
        Ok(())
    }
    pub fn remove(&mut self, id: u64) -> Result<Region> {
        let region = self
            .regions
            .remove(&id)
            .ok_or_else(|| anyhow::anyhow!("unknown region ID {id}"))?;
        self.region_metadata.remove(&id);
        Ok(region)
    }
    pub fn save(&self, path: &Path) -> Result<()> {
        self.validate()?;
        // Keep a full old or new store on failure; do not truncate the live file.
        let temp = path.with_extension(format!("romatlas-{}.tmp", std::process::id()));
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temp)?;
        let result = (|| {
            use std::io::Write;
            file.write_all(&serde_json::to_vec_pretty(self)?)?;
            file.sync_all()?;
            std::fs::rename(&temp, path)?;
            Ok(())
        })();
        if result.is_err() {
            let _ = std::fs::remove_file(temp);
        }
        result
    }
    pub fn load(path: &Path) -> Result<Self> {
        let store: Self = serde_json::from_slice(&crate::read_limited(path, 64 * 1024 * 1024)?)?;
        store.validate()?;
        Ok(store)
    }
}
fn validate_region(r: &Region) -> Result<()> {
    ensure!(!r.start.space.is_empty(), "missing address space");
    ensure!(
        r.length > 0 && r.start.offset.checked_add(r.length).is_some(),
        "empty or overflowing region"
    );
    ensure!(
        r.confidence.is_finite() && (0.0..=1.0).contains(&r.confidence),
        "confidence must be within 0..1"
    );
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use romatlas_core::{Address, Provenance, RegionKind};
    fn region(start: u64, length: u64) -> Region {
        Region {
            start: Address {
                space: "mock-wide".into(),
                offset: start,
                context: BTreeMap::new(),
            },
            length,
            kind: RegionKind::Code,
            provenance: Provenance::Static,
            confidence: 0.8,
        }
    }
    fn store() -> Store {
        Store::new("mock", &"a".repeat(64), u64::MAX).unwrap()
    }
    #[test]
    fn transactional_conflict_and_edit() {
        let mut s = store();
        let id = s.annotate(region(0x1_0000_0000, 10)).unwrap();
        assert_eq!(id, 1);
        assert!(s.annotate(region(0x1_0000_0005, 8)).is_err());
        assert_eq!(s.next_id, 2);
        assert_eq!(s.regions.len(), 1);
        let other = s.annotate(region(0x1_0000_0010, 8)).unwrap();
        assert!(s.edit(other, region(0x1_0000_0002, 8)).is_err());
        assert_eq!(s.regions[&other].start.offset, 0x1_0000_0010);
        s.remove(id).unwrap();
        assert!(s.remove(id).is_err());
    }
    #[test]
    fn invalid_and_identity() {
        let mut s = store();
        assert!(s.annotate(region(u64::MAX, 1)).is_err());
        assert!(s.annotate(region(0, 0)).is_err());
        let mut r = region(0, 1);
        r.confidence = f32::NAN;
        assert!(s.annotate(r).is_err());
        assert!(
            s.validate_identity("mock", &"b".repeat(64), u64::MAX)
                .is_err()
        );
        assert!(
            s.validate_identity("mock", &"a".repeat(64), u64::MAX)
                .is_ok()
        );
        let json = serde_json::to_string(&s).unwrap();
        let restored: Store = serde_json::from_str(&json).unwrap();
        restored.validate().unwrap();
    }
}
