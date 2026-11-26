# Investigation ledger

## Round 1 disassembly (2026-11-24/25) — checkpoint pass-001b

### Platform/bank model (established)
- iNES mapper 1 (MMC1), 8x16 KiB PRG, 16x8 KiB CHR. Reset $FF50 writes control $1E via $FF00
  (serial writes to $9FFF): PRG mode 3 (fixed $C000-$FFFF = 16K bank 7 = 8K banks 14/15; $8000 switchable),
  CHR 4K mode. The only classified-code call of $FF00 (control write) is the reset path; the other `20 00 FF`
  byte patterns (file 0xF4B6, 0xF776, 0xF7B6, 0xFB66, 0xFEA6) lie inside classified Data in bank 3.
- PRG select routine $FF3C (writes $FFFF). Fixed dispatcher $CF24/$CF5E..$CFBF selects 16K bank from table $CFC2
  (0..6) and enters `JMP $8000`. Every 16K bank 0..6 starts with `JMP main; JMP irq; NMI stub` at $8000/$8003/$8006
  (vectors NMI=$8006, RESET=$FF50, IRQ=$8003 in every bank). Bank roles observed: 0 menus/bios/results,
  1 Mate Toss, 2 Fire Starting, 3 Dino Race, 4 Saber Race, 5 Clubbing, 6 Dino Vault, 7 fixed engine.
- $FF00-$FFFF (mapper write stubs + reset + vectors) is byte-identical in all eight 16K banks.

### Method
- Toolkit static discovery stops at the first mapper write (TK-003), so a local recursive-descent tracer
  (tools/trace_static.py, tools/fulltrace.py) was used: MMC1 mode-3 context, banked->fixed transfers drop the
  bank, `LDA #n; JSR $FF3C` in fixed code sets bank n, constant-flag tracking stops fallthrough after
  always-taken branches (81 sites), manual fallthrough stops at file 0x1C56A ($C55A BNE: $AA counts 0..8,
  bytes from $C55C are a table read by `LDA $C55C,X`) and 0xD1C5 (bank 3 $91B5 after LDA #$FF).
- Roots: reset, per-bank $8000/$8003/$8006, all observed instruction starts with their observed prg_banks
  context (13651 offsets, union of two server sessions), and bounded jump tables (31 `JMP ($0200/$029B/...)`
  sites; tables read as lo/hi pairs, extent = up to first target/next code). Iterated to a fixpoint.
- Verification: map.validate decodes every Code view; no observed fetch falls inside a static instruction
  (0 conflicts); generated source rebuilds byte-identically (pass-001, pass-001b).
- Inferred routine roots (Static, confidence 0.8, 717 bytes): file 0x19F, 0x20C0, 0x144F8, 0x154A1, 0x1CA80,
  0x1EFF6, 0x5103, 0xD146, 0x154F1, 0x15523, 0x1555A. Criteria: start right after a classified terminator,
  clean decode to RTS/JMP, calls established entries, tracer reports no illegal/BRK/overlap. Not executed (L-004).
- Duplicate stubs $FF00-$FF72 in banks 0-6 annotated as Code with context [-, -, -, 2n+1] (byte identity with
  executed fixed-bank copy; reachable only if MMC1 powers up with that bank at $E000).
- Non-code gaps: split at >=4-byte runs of BRK/illegal opcode bytes (no instruction stream can pass);
  segments overlapping a code-like island (>=6 clean instructions to RTS/JMP with a call to established
  code) or a long clean decode left **Unknown**; Text = ASCII runs with $02/$03/$FF layout bytes (bios,
  event names, results strings); uniform $00 runs >=32 = Padding; rest generic Data (conf 0.9 if marker
  segment, 0.7 if only "no island" — largest such segments spot-checked: 0x11C9F metasprite 4-byte records,
  0x19145 pointer table, 0x67E0 tile map, 0x342A/0x1F651 note/duration streams, 0x17B49 records).

### Results
ROM classification: 99.12% (259842 / 262160 bytes classified; 2318 bytes unknown).
Code 41428 (Observed-provenance regions 30981 bytes), Data 67928, Text 5340, Padding 14074, Image (CHR) 131072.

Remaining unknown (exact): 0x95F6-0x9623 (46), 0xD189-0xD18E (6), 0xE61C-0xE69E (131), 0xE6A4-0xE904 (609),
0xE90C-0xEC9E (915), 0xECB2-0xEF08 (599), 0x1558C-0x15597 (12). See leads L-001..L-003.

### Lead outcomes
- No input leads (fresh workspace). Produced L-001..L-006 (analysis/leads.json).

### Rejected / not done
- Bank 3 $A60C-$AEF9 probe roots rejected for now: tracing from them reaches illegal opcodes or JSR into RAM
  ($07AD/$7620), so entries are misidentified; keep unknown (L-001).
- Bank 2 0x95F6 root rejected: trace reaches illegal opcode at $957A (L-002).
- Saber Race / Dino Race completion attempts (mix, A, A/B, L/R patterns, 400-15000 frames): no finish (L-006).
- Residual assumption: no unclassified path rewrites MMC1 control to a non-mode-3 value (no such call in classified
  code; bank-3 unknown range L-001 should be checked when resolved). Observed contexts were always [x,x,14,15].

## Round 1 redirect r01-a1 (2026-11-25) — checkpoint pass-001b (record: analysis/redirect/r01-a1/notes.md)
- L-001 (2254 B): resolved as non-code. A stale, corrupted foreign 6502 image relocated by +$1F21: 47/66 call targets
  land after an RTS at that shift (control peak 17; real bank-3 code peaks at 0). No references. In place it would
  write MMC1 via its $89Dx variables. It polls the $7F00/$7F01 dev port. Duplicated-byte corruption. Proposed generic Data.
- L-008/L-009/L-010: verified with consumer/contract/extent evidence. Proposals cover the remaining 64 unknown bytes.
- L-007: IRQ split handlers proven by the static RTS-dispatch contract. The $F26A/$C62F stubs are byte-identical to
  established code. Proposed Data->Code corrections.
- L-006: Dino Race and Saber Race finished naturally with code-derived controls (S14 visited). +537 instruction starts,
  all inside existing Code. The L-004 roots were still not executed.
- Rejected/not done: L-005 semantic refinement (no completion effect); 2P mode (S15 deferred, no dependent unknown).

## Round 2 disassembly (2026-11-25) — input pass-001b, output pass-002
- Applied the r01-a1 ready batch after independent re-checks (tools/r02_apply.py):
  - L-001: re-ran the relocation and reference scripts and added an immediate hi-byte scan (one hit, a VRAM fill
    value). Accepted as generic Data.
  - L-008, L-009, L-010: re-checked consumers, extent bounds and word references. Accepted.
  - L-007, L-011: re-decoded and searched all banks for references. Accepted as Static code. Reachability is
    recorded separately: unreferenced, or static-only for the IRQ path. None of it is claimed dead.
  - L-012: merged the coverage and upgraded provenance.
- Result: 100.00% (262160/262160; 0 unknown). Code 41610, Data 70064, Text 5340, Padding 14074, Image 131072.
- Residual assumption (round 1), now checked: the former L-001 range is not executable here, so it adds no MMC1
  control rewrite. The code added this pass only calls $CD71, $CD73 and $917F and jumps to $C036, so it makes no
  MMC1 writes either.
- Open, no classification effect:
  - L-004: reachability of 11 inferred roots. 2-player mode (S15) is untried.
  - L-005: specific roles inside generic data.

## Round 3 disassembly (2026-11-25) — input pass-002, output pass-003
- Applied the r02-a1 ready batch after independent re-checks (tools/r03_apply.py):
  - L-013: DR_ActionBase/Ticks partition arithmetic and the consumer's CMP DR_ActionTicks,Y bound re-verified; the 7 B
    at bank 3 $860E are the draw-both-racers entry falling into DR_DrawRacers. Static Code 0.8, unreferenced.
  - L-014: Caves of Fame consumer re-read; offsets 6 B, titles 100 B, $85B1 self-jump kept as one Data item.
  - L-016: coverage union checked (all on Code instruction starts); 310 B -> Observed. CL_Bracket3/6 and
    CL_PairFromWinners are naturally reachable (multi-player route, S15).
  - L-017/L-018: accepted as Static Code 0.7 instruction fragments (unreferenced). L-018 rests on an encoded
    self-reference (STA $8364 into its own operand), not on decode plausibility alone.
- Result: 100.00% (262160/262160; 0 unknown). Code 41657, Data 70006, Text 5351, Padding 14074, Image 131072.
- Remaining audit F hits (JSR $FF00 inside bank 3 bitmap-like data, JMP $87F6 inside DR_StaleForeignImage) are
  coincidental byte patterns inside already-dispositioned non-code; no change.
- Open, no classification effect: L-004 (7 roots, reachability), L-005 (semantic roles), L-015 (commentary fact).

## Round 3 redirect r03-a1 (2026-11-25) — checkpoint pass-003 (record: analysis/redirect/r03-a1/notes.md)
- Non-code role audit against consumers. Padding/Text regions that code consumes as data are now lead L-019, and
  consumer-anchored roles/extents of generic blocks are lead L-020.
- L-005 is partly resolved (bank 2 $8A9D = FS flame frames).
- Remaining without consumer: bank 3 $AEF9-$B01D, bank 7 $CEDB-$CEE7, bank 2 $9420-$944B, IrqSplitParam odd bytes.
- L-004 is unchanged (reachability only).

## Round 4 disassembly (2026-11-25) — input pass-003, output pass-004
- Applied L-019 and the disassembly part of L-020 after independent re-checks (tools/r04_verify.py, tools/r04_apply.py;
  analysis/round-4/{l019_verify.json, extents.txt, changes.json}).
- Metatile consumer contract re-read in bank 7 (UploadMetatileScreen $C4FC: 8x8 map; MetatilePtrFromA $C4DA:
  set + 17*index; record = attribute + 16 tiles). Pairing re-derived by data flow; 16 map/set extents now single Data
  regions with consumer evidence.
- Bank 3 set $9F23: records 0..64 from screen map $9561; course map $944B (DR_StreamTrack) indexes the same set with
  values up to 75 in $944B-$9560 but its exact extent is not claimed; records 65..103 continue the 17-byte grid
  (attribute $55 throughout) to $A60A. Usage of records >75 unresolved (structural claim only).
- Labelled Data: FS_FlameCels, FS_PoseCels, CL_ArenaMap, CL_ArenaMetatiles, CL_MsprCels, MT_MsprCels.
- Result: 100.00% (262160/262160; 0 unknown). Code 41657, Data 72195, Text 4862, Padding 12374, Image 131072.
- Open, no classification effect: L-004 (reachability), L-005 remainder (bank 3 $AEF9-$B01D, bank 7 $CEDB-$CEE7,
  bank 2 $9420-$944B, IrqSplitParam odd bytes), bank 3 course-map extent, bank 6 map $80E6 set context, bank 1
  $9D6B-$9DA7 stream consumer.

## Round 5 disassembly (2026-11-25) — input pass-004, output pass-005
- Applied L-021 and L-022 (tools/r05_apply.py; analysis/round-5/changes.json). Data/Text-only edits; code unchanged.
- DR_CourseMap extent closed: $944B-$954A (256 B) — fixed pointer, 8-bit (MtileMapPtr),Y window, finish look-ahead $FF.
  Tail $954B-$9560 stays generic Data (outside the window; role unresolved).
- Bank 7 $D881-$D922 and bank 0 $A8CF-$A9AB are now single generic Data regions (no Text); no consumer for either.
- Bank 2 $ACDF/$ACE0 boundary aligned with the end of SFX stream 17.
- Result: 100.00% (262160/262160; 0 unknown). Code 41657, Data 72235, Text 4822, Padding 12374, Image 131072.
- Open, no classification effect: L-004 (reachability), L-005 remainder (roles of consumerless blocks), S16.
