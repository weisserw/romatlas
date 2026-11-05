# Round 1 disassembly handoff

Status: incomplete. Exactly one disassembly pass completed; commentary/redirect remain coordinator work.

ROM classification: 64.55% (42311 / 65552 bytes classified; 23241 bytes unknown).

Checkpoint: [pass-001 map](../maps/pass-001.json), [verification](pass-001-verification.json), [assembly](../generated/pass-001/game.asm). Internal [state](state.json), current portable [map](../maps/current.json), exact [counts and unknown complement](pass001-counts.json). Map validation and ca65 rebuild passed; byte_identical=true. ROM SHA256 unchanged. Earlier baseline and pass-001-a preserved.

| Physical section (8KiB banks) | Code | Data/image | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG0 | 843 | 2322 | 5027 |
| PRG1 | 2390 | 67 | 5735 |
| PRG2 | 3040 | 20 | 5132 |
| PRG3 | 839 | 6 | 7347 |
| CHR0 | 0 | 8192 | 0 |
| CHR1 | 0 | 8192 | 0 |
| CHR2 | 0 | 8192 | 0 |
| CHR3 | 0 | 8192 | 0 |

7112 code bytes, 35199 data/image bytes. Every classified byte has structural evidence (42311/42311); no full readability/editorial claim. Map provenance is 5474 Observed and36837 Static bytes; preserving old Static regions means those numbers are not total execution coverage. 2820 unique physical ROM instruction starts across saved final/replay contexts (4410 contextual roots); original forward timeline summary3566 roots, replay2091, aggregate_dropped=0. Detail intentionally capacity0, not a complete bus-event trace. See pass001-contexts-final2.json, pass001-contexts-replay.json and both trace-summary files.

## Productive work and evidence

- Header, exact vectors (NMI DC88/reset C000/IRQ DD04), dedicated CHR-ROM image section and bounded code discoveries inspected. PRG physical offsets include16-byte header. Effective PRG slot banks[0,1,2,3] used for ordinary routes; individual observed contexts retained in inspection files and instruction views. Discovery conservatively stops mapper effects, calls and unresolved indirect paths; static candidates are never claimed naturally reachable.
- Natural intro/menu/Floor1 room reached, then representative movement/shooting. Actual room1 screenshot frame1541 (earlier filenames level1 and room1 capture transitions/briefing). [Scenarios](scenarios.json) is authoritative.
- Supported RAM shortcut: $55 room index, $57 lives, $67 death flag. Natural room1 snapshot -> writes55=24,57=99,67=1 -> release inputs/run240frames reaches distinct room25;55=49,67=1 similarly reaches final room50. Each exercised48 traced input frames and retained snapshots/RAM/screenshots. Main-loop evidence at98CD/98E3/998F/999F, transcript pass001-rpc.jsonl.
- Final room clear gate61=1,6F=0 accepted; index increments49->50, comparison99E4 and jump9F74 enter ending. Corridor reunion and Lolo/Lala heart close-up observed; later castle view captured. No unsupported claim that last castle frame is terminal credits.
- Replay one successful final-room reload with trace enabled for5x48frames, restoring room1-play snapshot and clearing trace first. This captured initialization omitted by earlier untraced delays. Frame1829 replay screenshot retained. No random RAM search.
- Inline helpers DA54/DA5A/DA60/D8DE consume2 bytes after JSR and adjust stacked return by2. Classified pointer barriers and supported continuations. D8DE consumes32 palette entries;5 distinct32-byte palette seed slices annotated. DA54 fill rectangle uses4-byte header+1 payload, DA60 positive-dimension literal rectangle uses4+width*height; supported slices add1140 bytes (prior data preserved). Read primary helper inspection in RPC transcript, pass001-rectangle-inspection.json, pass001-inline-inspection*.json, helper scripts.
- Generic fallback1027 bytes: maximal uniform runs>=32 with boundary/neighborhood and known-code checks. Generic Data only; no invented padding/table purpose. See pass001-uniform-inspection.json. Other mixed gaps remain unknown.

## Readability scope

Current generated source is structurally annotated and byte-identical, but most code labels remain region_offset and most instructions lack semantic commentary. Commentary must cover all7112 currently classified code bytes and classified data: routine/branch names, RAM variables/contracts, useful explanations; retain existing evidence and distinguish unresolved semantics. Major useful known roles: D9AE/D9B8 NMI enable control, D9BE frame wait, D9D9 MMC1 control serial write, D9ED/DA08 CHR bank setters, DA23 PRG setter, DA54/5A/60 inline video rectangle transfer, D8DE palette adjustment,98CD room initializer,9931 gameplay loop,999F room increment,9F74 ending entry. These are grounded starting points, not a completed editorial audit.

## Remaining questions / next phase

- L005: General static continuation across JSR. Ready research recipe: classify call effects/stack contracts from saved code; ordinary RTS callee with stable PRG context can support fallthrough without execution. Inline helpers above are exceptions. Discovery currently stops every MMC1 JSR and does not follow unsupported return assumptions. Do not demand live callers or branch-by-branch execution. This contract review, not more long gameplay, should unlock coherent connected batches.
- L006: Room loader/data. $98E3 supplies room index55 to B44A; successful room0/24/49 reloads prove setup contract. Inspect B44A and its downstream index/table calculations, determine independently bounded room records and object schemas. Only loaded/layout-visible rooms were visited; no full domain extent claim yet.
- L007: APU/audio graph. Actual writes at DEED->$4000,X, DF0A->$4004,X, DF29->$4008,X, DF47->$400C,X anchor a consumer graph into unknown PRG3 (7347 unknown). Follow saved routines/data references before assuming all residual bank-tail bytes are music. Need record/control contracts and exact ends; no broad tail annotation.
- L008: Video records outside proven positive rectangle modes. DA5A no-destination mode and high-bit orientation branch DB1F require complete nested-loop/source-advance review before claiming full record extent. Existing positive DA54/DA60 slices are independently supported and should not be rejected by missing proof for other modes.

This is an easy-gain handoff, not terminal exhaustion. Strongest residual candidates were not rejected as non-code/unused: call-effect uncertainty is localized, room/audio/video objects need consumer extent review. They remain concrete redirect avenues; no repeated failed experiment is needed. Static classification does not require natural progression or recovery of original generator/layout.

Toolkit: [issues](toolkit-issues.md), no toolkit issues found. No toolkit edits. Initial overlong traced180frame request timed out at80frames; bounded shorter steps and untraced known delays worked. Server closed cleanly after saving room50-replay.snapshot.json at replay frame1829, trace disabled. Future workers should load state and start emulator, importing a durable snapshot if needed. Session snapshots are not assumed persistent.
