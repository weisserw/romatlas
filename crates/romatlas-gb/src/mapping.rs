//! Game Boy bridge from rich internal regions to stock mgbdis metadata.
use crate::{
    Rom,
    decode::{Flow, decode},
};
use anyhow::{Result, ensure};
use romatlas_core::{ImageAdapter, Region, RegionKind};
use romatlas_map::{Entry, Map, store::Store};

fn named_regions(store: &Store) -> Result<Vec<Entry>> {
    let mut entries = Vec::new();
    for (id, region) in &store.regions {
        if let Some(metadata) = store.region_metadata.get(id) {
            ensure!(metadata.is_object(), "region metadata must be object");
            for field in ["name", "role", "evidence", "comment"] {
                if let Some(value) = metadata.get(field) {
                    ensure!(value.is_string(), "region {field} must be string");
                }
            }
            if let Some(name) = metadata.get("name").and_then(|v| v.as_str()) {
                ensure!(!name.starts_with('.'), "region name must be a stock symbol");
                ensure!(
                    !name.is_empty() && !name.chars().any(char::is_whitespace),
                    "region name must be a stock symbol"
                );
                let (bank, address) = Rom::sym_location(region.start.offset as usize);
                entries.push(Entry {
                    bank,
                    address,
                    annotation: name.into(),
                });
            }
        }
    }
    Ok(entries)
}

pub fn validate_store(rom: &Rom, store: &Store) -> Result<()> {
    store.validate()?;
    use sha2::{Digest, Sha256};
    store.validate_identity(
        "gb",
        &format!("{:x}", Sha256::digest(&rom.bytes)),
        rom.bytes.len() as u64,
    )?;
    let mut label_offsets = std::collections::BTreeSet::new();
    let mut all_labels = named_regions(store)?;
    if let Some(labels) = store.metadata.get("gb.sym.labels") {
        let entries: Vec<Entry> = serde_json::from_value(labels.clone())?;
        ensure!(
            entries
                .iter()
                .all(|e| !e.annotation.starts_with('.') || !e.annotation.contains(':')),
            "label metadata contains region directives"
        );
        Map {
            entries: entries.clone(),
        }
        .validate(Some(rom.info.banks))?;
        all_labels.extend(entries.clone());
        for entry in entries.iter().filter(|entry| entry.address < 0x8000) {
            let offset = if entry.address < 0x4000 {
                entry.address as u64
            } else {
                entry.bank as u64 * 0x4000 + entry.address as u64 % 0x4000
            };
            label_offsets.insert(offset);
        }
    }
    Map {
        entries: all_labels,
    }
    .validate(Some(rom.info.banks))?;
    for (id, region) in &store.regions {
        if let Some(value) = store.region_metadata.get(id).and_then(|v| v.get("sym")) {
            let annotation = value
                .as_str()
                .ok_or_else(|| anyhow::anyhow!("invalid mgbdis region metadata"))?;
            let (bank, address) = Rom::sym_location(region.start.offset as usize);
            let parsed = Map::parse(&format!("{bank:02x}:{address:04x} {annotation}\n"))?;
            let parts: Vec<_> = parsed.entries[0].annotation.split(':').collect();
            ensure!(parts.len() >= 2, "expected directive in region metadata");
            let kind = directive_kind(parts[0])?;
            ensure!(
                kind == region.kind && u64::from_str_radix(parts[1], 16)? == region.length,
                "directive metadata differs from region kind or length"
            );
        }
        validate_region_with_annotation(
            rom,
            region,
            store
                .region_metadata
                .get(id)
                .and_then(|v| v.get("sym"))
                .and_then(|v| v.as_str()),
        )?;
        let start = region.start.offset;
        let end = start + region.length;
        match region.kind {
            RegionKind::Code => {
                let mut cursor = start;
                while cursor < end {
                    let (_, pc) = Rom::sym_location(cursor as usize);
                    let instruction = decode(&rom.bytes[cursor as usize..end as usize], pc as u16)?;
                    let next = cursor + instruction.length as u64;
                    ensure!(
                        label_offsets.range(cursor + 1..next).next().is_none(),
                        "label splits an instruction at physical offset {cursor:#x}; move the label or use raw data"
                    );
                    cursor = next;
                }
            }
            RegionKind::Padding | RegionKind::Image => ensure!(
                label_offsets.range(start + 1..end).next().is_none(),
                "label inside {:?} region at physical offset {start:#x}; split the region or use raw data",
                region.kind
            ),
            _ => {}
        }
    }
    Ok(())
}
pub fn validate_region(rom: &Rom, r: &Region) -> Result<()> {
    validate_region_with_annotation(rom, r, None)
}
fn validate_region_with_annotation(rom: &Rom, r: &Region, annotation: Option<&str>) -> Result<()> {
    ensure!(
        rom.physical_address(r.start.offset).as_ref() == Some(&r.start),
        "region must use canonical physical ROM address"
    );
    let start = usize::try_from(r.start.offset)?;
    let length = usize::try_from(r.length)?;
    let end = start
        .checked_add(length)
        .ok_or_else(|| anyhow::anyhow!("region overflow"))?;
    ensure!(length > 0 && end <= rom.bytes.len(), "region outside ROM");
    ensure!(
        start / 0x4000 == (end - 1) / 0x4000,
        "region crosses physical bank; split it"
    );
    match r.kind {
        RegionKind::Padding => ensure!(
            rom.bytes[start..end].iter().all(|b| *b == rom.bytes[start]),
            "padding region contains unequal bytes"
        ),
        RegionKind::Image => ensure!(
            length.is_multiple_of(
                if annotation.is_some_and(|a| a
                    .split(':')
                    .nth(2)
                    .is_some_and(|args| args.split(',').any(|arg| arg == "1bpp")))
                {
                    8
                } else {
                    16
                }
            ),
            "default 2bpp image length must be a multiple of 16"
        ),
        RegionKind::Code => {
            let mut cursor = start;
            while cursor < end {
                let (_, pc) = Rom::sym_location(cursor);
                let d = decode(&rom.bytes[cursor..end], pc as u16)?;
                ensure!(
                    d.flow != Flow::Invalid,
                    "invalid opcode at physical offset {cursor:#x}; use data or unknown"
                );
                ensure!(
                    d.warnings.is_empty(),
                    "encoding needs raw bytes at physical offset {cursor:#x}"
                );
                cursor += d.length as usize;
            }
        }
        _ => {}
    }
    Ok(())
}
/// Unknown bytes get lossless .data emission, without becoming data claims in the store.
pub fn export(rom: &Rom, store: &Store) -> Result<Map> {
    validate_store(rom, store)?;
    let mut map = Map::default();
    if let Some(labels) = store.metadata.get("gb.sym.labels") {
        map.entries = serde_json::from_value(labels.clone())?;
    }
    map.entries.extend(named_regions(store)?);
    let label_locations: std::collections::BTreeSet<_> =
        map.entries.iter().map(|e| (e.bank, e.address)).collect();
    for bank in 0..rom.info.banks {
        let start = bank * 0x4000;
        let end = start + 0x4000;
        let mut cursor = start;
        let mut regions: Vec<_> = store
            .regions
            .iter()
            .filter(|(_, r)| r.start.offset as usize / 0x4000 == bank)
            .collect();
        regions.sort_by_key(|(_, r)| r.start.offset);
        for (id, r) in regions {
            let pos = r.start.offset as usize;
            if cursor < pos {
                push_region(&mut map, cursor, pos - cursor, "data");
            }
            let kind = match r.kind {
                RegionKind::Code => "code",
                RegionKind::Data => "data",
                RegionKind::Text => "text",
                RegionKind::Padding => "padding",
                RegionKind::Image => "image",
                RegionKind::Unknown => "data",
            };
            if let Some(annotation) = store
                .region_metadata
                .get(id)
                .and_then(|v| v.get("sym"))
                .and_then(|v| v.as_str())
            {
                let (bank, address) = Rom::sym_location(pos);
                map.entries.push(Entry {
                    bank,
                    address,
                    annotation: annotation.into(),
                });
            } else {
                push_region(&mut map, pos, r.length as usize, kind);
            }
            if r.kind == RegionKind::Code {
                let (b, a) = Rom::sym_location(pos);
                if !label_locations.contains(&(b, a)) {
                    map.entries.push(Entry {
                        bank: b,
                        address: a,
                        annotation: format!("code_{b:02x}_{a:04x}"),
                    });
                }
            }
            cursor = pos + r.length as usize;
        }
        if cursor < end {
            push_region(&mut map, cursor, end - cursor, "data");
        }
    }
    map.validate(Some(rom.info.banks))?;
    Ok(map)
}
fn directive_kind(name: &str) -> Result<RegionKind> {
    Ok(match name.to_ascii_lowercase().as_str() {
        ".code" => RegionKind::Code,
        ".data" | ".byt" => RegionKind::Data,
        ".text" | ".asc" => RegionKind::Text,
        ".padding" => RegionKind::Padding,
        ".image" => RegionKind::Image,
        _ => anyhow::bail!("unsupported directive"),
    })
}
/// Import supported directives and label metadata; no ROM content enters the store.
/// Build a new store first, so failure never mutates a live session.
pub fn import(rom: &Rom, map: &Map) -> Result<Store> {
    use romatlas_core::Provenance;
    map.validate(Some(rom.info.banks))?;
    let mut store = Store::new("gb", &rom.info.sha256, rom.info.size as u64)?;
    let mut labels = Vec::new();
    let mut regions = Vec::new();
    let mut annotations = Vec::new();
    for entry in &map.entries {
        if entry.annotation.starts_with('.') && entry.annotation.contains(':') {
            let parts: Vec<_> = entry.annotation.split(':').collect();
            let kind = directive_kind(parts[0])?;
            let offset = entry.bank as u64 * 0x4000 + (entry.address as u64 % 0x4000);
            let region = Region {
                start: rom
                    .physical_address(offset)
                    .ok_or_else(|| anyhow::anyhow!("directive outside ROM"))?,
                length: u64::from_str_radix(parts[1], 16)?,
                kind,
                provenance: Provenance::Manual,
                confidence: 1.0,
            };
            validate_region_with_annotation(rom, &region, Some(&entry.annotation))?;
            regions.push(region);
            annotations.push(entry.annotation.clone());
        } else {
            labels.push(entry.clone());
        }
    }
    for (id, annotation) in store.annotate_batch(regions)?.into_iter().zip(annotations) {
        store
            .region_metadata
            .insert(id, serde_json::json!({"sym":annotation}));
    }
    store
        .metadata
        .insert("gb.sym.labels".into(), serde_json::to_value(labels)?);
    validate_store(rom, &store)?;
    Ok(store)
}
fn push_region(map: &mut Map, offset: usize, length: usize, kind: &str) {
    // Pinned stock mgbdis keeps interior default blocks unless their exact starts
    // are overridden. Include DMG and CGB defaults so unknown bytes stay raw.
    const DEFAULT_STARTS: &[usize] = &[
        0x08, 0x10, 0x18, 0x20, 0x28, 0x30, 0x38, 0x40, 0x48, 0x50, 0x58, 0x60, 0x100, 0x104,
        0x134, 0x13f, 0x143, 0x144,
    ];
    if kind == "data" {
        let end = offset + length;
        let mut cursor = offset;
        for &boundary in DEFAULT_STARTS {
            if boundary > cursor && boundary < end {
                push_directive(map, cursor, boundary - cursor, kind);
                cursor = boundary;
            }
        }
        push_directive(map, cursor, end - cursor, kind);
    } else {
        push_directive(map, offset, length, kind);
    }
}
fn push_directive(map: &mut Map, offset: usize, length: usize, kind: &str) {
    let (bank, address) = Rom::sym_location(offset);
    map.entries.push(Entry {
        bank,
        address,
        annotation: format!(".{kind}:{length:x}"),
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use romatlas_core::Provenance;
    fn fixture() -> (Rom, Store) {
        let rom = Rom::from_bytes(vec![0; 0x8000]).unwrap();
        let store = Store::new("gb", &rom.info.sha256, 0x8000).unwrap();
        (rom, store)
    }
    fn region(rom: &Rom, offset: u64, length: u64, kind: RegionKind) -> Region {
        Region {
            start: rom.physical_address(offset).unwrap(),
            length,
            kind,
            confidence: 1.0,
            provenance: Provenance::Manual,
        }
    }
    #[test]
    fn explanatory_metadata_without_directive_exports_names() {
        let (rom, mut store) = fixture();
        let id = store
            .annotate(region(&rom, 0x100, 3, RegionKind::Code))
            .unwrap();
        store.region_metadata.insert(
            id,
            serde_json::json!({
                "name":"Entry", "role":"entry routine", "evidence":"observed fetches"
            }),
        );
        let text = export(&rom, &store).unwrap().export().unwrap();
        assert!(text.contains("00:0100 Entry\n"));
        assert!(text.contains("00:0100 .code:3\n"));
        assert!(!text.contains("code_00_0100"));
        assert!(!text.contains("observed fetches"));
        store.region_metadata.get_mut(&id).unwrap()["name"] = serde_json::json!("bad symbol");
        assert!(validate_store(&rom, &store).is_err());
    }
    #[test]
    fn imported_annotations_persist_and_edit() {
        let (rom, _) = fixture();
        let map=Map::parse("00:0100 entry\n00:0100 .code:3\n00:0200 .byt:20:2\n00:0300 .asc:10:cm=0\n01:4000 Tiles\n01:4000 .image:8:w8,pe4,1bpp\n00:c000 wBuffer\n").unwrap();
        let mut store = import(&rom, &map).unwrap();
        let restored: Store =
            serde_json::from_str(&serde_json::to_string(&store).unwrap()).unwrap();
        let output = export(&rom, &restored).unwrap();
        for entry in &map.entries {
            assert!(output.entries.contains(entry), "lost {entry:?}");
        }
        let id = *store
            .regions
            .iter()
            .find(|(_, r)| r.start.offset == 0x200)
            .unwrap()
            .0;
        let mut r = store.regions[&id].clone();
        r.confidence = 0.5;
        store.edit(id, r.clone()).unwrap();
        assert!(store.region_metadata.contains_key(&id));
        r.kind = RegionKind::Unknown;
        store.edit(id, r).unwrap();
        assert!(!store.region_metadata.contains_key(&id));
        assert!(
            !export(&rom, &store)
                .unwrap()
                .export()
                .unwrap()
                .contains(".byt:20:2")
        );
        store
            .region_metadata
            .insert(id, serde_json::json!({"sym":".code:20"}));
        assert!(validate_store(&rom, &store).is_err());
    }
    #[test]
    fn gaps_and_stable_labels() {
        let (rom, mut s) = fixture();
        s.annotate(region(&rom, 0x100, 2, RegionKind::Code))
            .unwrap();
        let text = export(&rom, &s).unwrap().export().unwrap();
        assert!(text.contains("00:0100 .code:2\n"));
        assert!(text.contains("00:0100 code_00_0100\n"));
        assert!(text.contains("00:0102 .data:2\n"));
        assert!(text.contains("00:0144 .data:3ebc\n"));
        assert_eq!(s.regions.len(), 1);
    }
    #[test]
    fn reject_alias_boundary_and_content() {
        let (rom, s) = fixture();
        let mut r = region(&rom, 0x3fff, 2, RegionKind::Data);
        assert!(validate_region(&rom, &r).is_err());
        r.length = 1;
        r.start.context.clear();
        assert!(validate_region(&rom, &r).is_err());
        let mut changed = rom.clone();
        changed.bytes[0x100] = 0xd3;
        assert!(validate_region(&changed, &region(&changed, 0x100, 1, RegionKind::Code)).is_err());
        assert!(validate_store(&changed, &s).is_err());
    }
    #[test]
    fn rejects_unsafe_content() {
        let (mut rom, _) = fixture();
        rom.bytes[0x100..0x103].copy_from_slice(&[0x10, 1, 0]);
        assert!(validate_region(&rom, &region(&rom, 0x100, 2, RegionKind::Code)).is_err());
        assert!(validate_region(&rom, &region(&rom, 0x100, 1, RegionKind::Code)).is_err());
        assert!(validate_region(&rom, &region(&rom, 0x100, 2, RegionKind::Padding)).is_err());
        assert!(validate_region(&rom, &region(&rom, 0x100, 15, RegionKind::Image)).is_err());
        assert!(validate_region(&rom, &region(&rom, 0x100, 16, RegionKind::Image)).is_ok());
    }
    #[test]
    fn hash_mismatch() {
        let (rom, s) = fixture();
        let other = Rom::from_bytes(vec![1; 0x8000]).unwrap();
        assert!(validate_store(&other, &s).is_err());
        assert!(validate_store(&rom, &s).is_ok());
    }
    #[test]
    fn labels_must_survive_stock_emission() {
        let (mut rom, _) = fixture();
        rom.bytes[0x200..0x203].copy_from_slice(&[0xc3, 0, 2]);
        let rom = Rom::from_bytes(rom.bytes).unwrap();
        for text in [
            "00:0200 .code:3\n00:0201 Operand\n",
            "00:0300 .padding:20\n00:0301 Hidden\n",
            "00:0300 .image:20\n00:0310 Hidden\n",
        ] {
            assert!(import(&rom, &Map::parse(text).unwrap()).is_err());
        }
        assert!(
            import(
                &rom,
                &Map::parse("00:0200 .code:3\n00:0200 Entry\n00:0203 After\n").unwrap()
            )
            .is_ok()
        );
    }
}
