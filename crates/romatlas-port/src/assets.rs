//! Identity-checked immutable ROM access and ordered, extensible asset collections.
use anyhow::{Context, Result, ensure};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeSet,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

pub const MAX_ROM_BYTES: usize = 64 * 1024 * 1024;
pub const MAX_ASSET_BYTES: usize = 16 * 1024 * 1024;

pub(crate) fn read_bounded(path: &Path, max: usize) -> Result<Vec<u8>> {
    let file = File::open(path).with_context(|| format!("open {}", path.display()))?;
    ensure!(
        file.metadata()?.len() <= max as u64,
        "file exceeds {max} bytes"
    );
    let mut bytes = Vec::new();
    file.take(max as u64 + 1).read_to_end(&mut bytes)?;
    ensure!(bytes.len() <= max, "file exceeds {max} bytes");
    Ok(bytes)
}

pub struct Rom {
    bytes: Vec<u8>,
    pub sha256: String,
}
impl Rom {
    pub fn open(path: &Path, expected_sha256: &str) -> Result<Self> {
        Self::from_bytes(read_bounded(path, MAX_ROM_BYTES)?, expected_sha256)
    }
    pub fn from_bytes(bytes: Vec<u8>, expected_sha256: &str) -> Result<Self> {
        ensure!(
            !bytes.is_empty() && bytes.len() <= MAX_ROM_BYTES,
            "invalid ROM size"
        );
        let sha256 = format!("{:x}", Sha256::digest(&bytes));
        ensure!(sha256 == expected_sha256, "ROM identity mismatch");
        Ok(Self { bytes, sha256 })
    }
    pub fn read(&self, offset: usize, length: usize) -> Result<&[u8]> {
        let end = offset.checked_add(length).context("ROM range overflow")?;
        self.bytes
            .get(offset..end)
            .context("ROM range is out of bounds")
    }
}

#[derive(Debug, Clone)]
pub struct RomAsset {
    pub id: String,
    pub offset: usize,
    pub length: usize,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FileAsset {
    pub id: String,
    pub file: PathBuf,
}
/// One manifest per collection. An empty order preserves defaults then appends
/// new slots in entries order. A nonempty order must name every slot exactly once.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub version: u32,
    pub entries: Vec<FileAsset>,
    #[serde(default)]
    pub order: Vec<String>,
}
impl Manifest {
    pub fn load(path: &Path) -> Result<Self> {
        Ok(serde_json::from_slice(&read_bounded(path, 1024 * 1024)?)?)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Origin {
    Rom {
        offset: usize,
        length: usize,
    },
    File(PathBuf),
    /// Game-owned recipes may decompress or combine noncontiguous ROM ranges.
    DerivedFromRom {
        recipe: String,
    },
}
#[derive(Debug, Clone)]
pub struct Asset {
    pub id: String,
    pub bytes: Vec<u8>,
    pub origin: Origin,
}
#[derive(Debug, Clone)]
pub struct Collection {
    pub assets: Vec<Asset>,
}

fn valid_id(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= 128
        && id
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"-_./".contains(&b))
}
impl Collection {
    /// The validator owns the collection's schema (including game-specific
    /// decompression rules). Validate ROM defaults and file extensions alike.
    pub fn load(
        rom: &Rom,
        defaults: &[RomAsset],
        overlay: Option<(&Manifest, &Path)>,
        validate: impl Fn(&str, &[u8]) -> Result<()>,
    ) -> Result<Self> {
        let mut assets = Vec::<Asset>::new();
        let mut total = 0_usize;
        for entry in defaults {
            ensure!(entry.length <= MAX_ASSET_BYTES, "asset exceeds size limit");
            total = total
                .checked_add(entry.length)
                .context("collection size overflow")?;
            ensure!(total <= MAX_ROM_BYTES, "collection exceeds 64 MiB");
            let bytes = rom.read(entry.offset, entry.length)?.to_vec();
            assets.push(Asset {
                id: entry.id.clone(),
                bytes,
                origin: Origin::Rom {
                    offset: entry.offset,
                    length: entry.length,
                },
            });
        }
        Self::with_overrides(assets, overlay, |asset| validate(&asset.id, &asset.bytes))
    }

    /// Apply replacements/extensions to game-owned decoded defaults. A recipe
    /// can produce a modern format (for example JSON levels) from ROM bytes at
    /// startup, allowing editors to use that same format for overrides/new slots.
    /// The validator can also distinguish original/file schemas using origin.
    pub fn with_overrides(
        mut assets: Vec<Asset>,
        overlay: Option<(&Manifest, &Path)>,
        validate: impl Fn(&Asset) -> Result<()>,
    ) -> Result<Self> {
        let mut seen = BTreeSet::new();
        let mut total = 0_usize;
        for asset in &assets {
            ensure!(
                valid_id(&asset.id) && seen.insert(asset.id.clone()),
                "invalid or duplicate default ID"
            );
            ensure!(
                asset.bytes.len() <= MAX_ASSET_BYTES,
                "asset exceeds size limit"
            );
            total = total
                .checked_add(asset.bytes.len())
                .context("collection size overflow")?;
            ensure!(total <= MAX_ROM_BYTES, "collection exceeds 64 MiB");
            validate(asset)?;
        }
        if let Some((manifest, root)) = overlay {
            ensure!(manifest.version == 1, "unsupported asset manifest version");
            let root = root.canonicalize()?;
            let mut overrides = BTreeSet::new();
            for entry in &manifest.entries {
                ensure!(
                    valid_id(&entry.id) && overrides.insert(&entry.id),
                    "invalid or duplicate override ID"
                );
                ensure!(
                    !entry.file.is_absolute()
                        && entry
                            .file
                            .components()
                            .all(|c| matches!(c, std::path::Component::Normal(_))),
                    "asset file must be relative within override directory"
                );
                let path = root.join(&entry.file).canonicalize()?;
                ensure!(
                    path.starts_with(&root),
                    "asset path escapes override directory"
                );
                let bytes = read_bounded(&path, MAX_ASSET_BYTES)?;
                let asset = Asset {
                    id: entry.id.clone(),
                    bytes,
                    origin: Origin::File(path),
                };
                validate(&asset)?;
                if let Some(index) = assets.iter().position(|a| a.id == entry.id) {
                    total -= assets[index].bytes.len();
                    total += asset.bytes.len();
                    assets[index] = asset;
                } else {
                    total += asset.bytes.len();
                    assets.push(asset);
                }
                ensure!(total <= MAX_ROM_BYTES, "collection exceeds 64 MiB");
            }
            if !manifest.order.is_empty() {
                ensure!(
                    manifest.order.len() == assets.len(),
                    "order must include every asset"
                );
                let mut sorted = Vec::new();
                for id in &manifest.order {
                    let index = assets
                        .iter()
                        .position(|a| &a.id == id)
                        .context("unknown or repeated asset in order")?;
                    sorted.push(assets.remove(index));
                }
                assets = sorted;
            }
        }
        Ok(Self { assets })
    }
    pub fn get(&self, id: &str) -> Option<&Asset> {
        self.assets.iter().find(|asset| asset.id == id)
    }
}
