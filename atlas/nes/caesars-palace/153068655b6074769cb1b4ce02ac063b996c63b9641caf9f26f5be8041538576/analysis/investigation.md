# Investigation ledger

## Round 1 disassembly (2026-11-22)

Checkpoint: maps/round1-disasm-v2.json (generated/round1-disasm-v2, analysis/round1-disasm-v2-verification.json).
Superseded intermediate checkpoint: maps/round1-disasm.json (same method, before the $E49F fragment split).

### Architecture facts established (Static unless noted)
- UxROM: fixed 16K bank 7 = 8K banks 14/15 at $C000-$FFFF. SelectPrgBank at $FF80: X = 16K bank,
  stores X in $0305, bus-conflict write `lda/sta $FFA2,x` (table $FFA2-$FFB1 = 00..0F).
- Vectors (every 16K bank has the same tail template): NMI $C000 (jmp $F955), RESET/IRQ $FFB2.
- Code banks: 16K bank 0 (sound driver $8000-$8424 + jump table $8001-$800F; command dispatch
  `jmp ($07F2)` from lo/hi tables $8425/$8441, 28 entries bounded by `cpy #$1C`), 16K bank 6
  (all casino game screens; entry $8001 and the dispatch targets $A657/$9C67/$8E7F/$95F6/$82F2/$BF2E),
  fixed bank 7. Bank-6 select wrappers: $C594, $D1C6, $D1CE, $DF26 (`ldx #6; jmp $FF80`).
- 16K banks 1-5: no instruction fetch observed, no static control transfer into the switchable
  window under those banks; data loaders (e.g. $E296 `ldx $0435; jsr $FF80; lda ($00),y`) read them
  sequentially (Read stream S1-S3). Classified as generic Data (+$FF padding runs); bank 5 upper
  half ($A000-$BF7F) is almost entirely $FF fill.
- Per-bank tail template copies at $BF80-$BFFF in banks 0-5 classified as code (SelectPrgBank,
  reset stub; byte-identical to fixed bank), data (bank table, vectors) and padding. Reachability not claimed.

### Method
1. Observed contexts (analysis.contexts) from traced scenarios S1-S6 (5377 unique instruction contexts)
   used as roots -> Observed provenance for 8004 code bytes.
2. Local bank-aware recursive traversal (analysis/round1-tools/trav.py) using analysis.decode for every
   instruction; rules as above (TK-003 explains why toolkit discovery was not used). No overlaps, no
   BRK/KIL reached, only diagnostics: `jmp ($07F2)` (resolved via table) and one jump to $BF2E from
   fixed code with unknown slot (reached in bank 6 via other paths).
3. Gap review (gapref.py): each remaining gap in code banks checked for operand-reference consumers,
   uniform fill, ASCII text, and an unreferenced-fragment criterion (frag.py). Manual decisions recorded
   in build.py MANUAL table (e.g. $C049 data via pointer table $CFB5; $E5AE PPU row-address table).
4. Annotated with map.annotate (469 regions), validated, saved, exported, checkpointed.

### Lead outcomes
No input leads (fresh workspace). Produced L-001..L-005 (analysis/leads.json).

### Remaining unknown
- File 0x18010 (bank 6 $8000, byte $EA): code-vs-data ambiguous (L-003).

### Weaker claims to re-check (not unknown, but lower confidence)
- Unreferenced fragments (confidence 0.7, L-004).
- Generic data in banks 1-5 (confidence 0.85) rests on absence of control transfers + observed sequential reads in banks 1,2,3,8 only; banks 4-7, 9-11 had no observed reads (not visited screens).
- Data gaps in code banks with consumer references: a table followed by unreferenced code in the same
  gap would be misclassified; a jsr-signature scan found only $E495 (fixed by splitting at $E49F).

Note: analysis/round1-tools/* were run from workspace/scratch/ (now removed) with daemon.py bridging `python3 run.py serve` to /tmp/romatlas-1530.sock; obs.json.gz = observed contexts [offset, cpu, prg_banks, hits]; actions.log = raw input-route log.

## Round 2 disassembly (2026-11-22)

Input maps/round1-disasm-v2.json; checkpoint maps/round2-disasm.json (analysis/round2-disasm-verification.json, byte-identical).
Method: independent bounded re-verification of the r1-a1 ready leads, then one computed edit plan
(analysis/round2-tools/plan.py: 126 regions replaced by 585; tiling/label-uniqueness asserted) applied with
map.remove/map.annotate (apply.py), validated, saved, exported, checkpointed. No gameplay run (all milestones
were visited in r1-a1; remaining value is provenance only and traced stepping is slow, TK-002).

### Lead outcomes (details in analysis/leads.json)
- L-003 verified -> 0x18010 Data (bank-6 leader byte, 0.8; competing NOP-prologue reading recorded). Rescan of
  11,895 classified instructions: no $8000 operand/branch target; one bounded indirect JMP (bank 0).
- L-006 verified -> 25 Padding bytes are data (FloorBlockAttrs blocks 13-17; $FF end markers of screens 5/10/25/26/29).
- L-002 verified -> 63 labeled records in banks 0-5 (0.95) + leader bytes of banks 1-5 (0.9). Screen decoder checked
  against ScreenDecode/DecodeNibbleRuns; all 30 screens re-decoded to the recorded ends; records gapless.
- L-001 verified -> 11,749 code bytes Static->Observed (context-matched; 0 mismatches); split only at observed/unobserved
  instruction boundaries; unobserved runs keep Static provenance and their confidence.
- L-004 verified -> reached fragment parts Observed or Static 0.9 (callers listed, 7 spot-checked); 675-byte residue
  Static 0.8 "statically unreferenced; no known entry; not claimed dead".
- L-005 partially applied -> Hotspot X/W/Y/H, AttrRowAddr0/1, PokerPayTable/PokerCoinValue, FourReelPayTable (54, not 66)/
  FourReelColumns/FourReelStripPtrs. Remaining generic table areas: 6:$B0EB-$BAD8 residue, metasprite definitions
  7:$C049-$C286/$D084-$D12B, 7:$F3DE-$F8F7 (generic data is sufficient; specific layouts unproven).

### Remaining
- Unknown bytes: none. Execution coverage: 6,535 Static code bytes in 322 regions not fetched (largest 7:$D1D6 410,
  6:$9E1E 246, 7:$E296 185, 7:$F213 164, 7:$CE61 149, 7:$E49F 147); residue reachability unresolved.
- Weakest structural claims: 0x18010 leader byte (0.8, single byte, layout analogy) and the 675-byte unreferenced code residue (0.8).

## Round 3 disassembly (checkpoint maps/round3-disasm.json, sha256 24099db1...)

### Lead outcomes
- L-008 verified -> 7:$CFCF-$D024 (86 bytes) Code, Static 0.8, statically unreferenced (not dead). The linear decode has
  no gaps and ends with JMP $CFCF before PrintText. The operand and immediate-pair scans found no reference.
  MetaspriteTable is now $CFB5-$CFCE. The $D084 block is split (CharToTileTail, TextCursorColBase/RowBase/Width, 2 generic spans).
- L-007 verified -> 77 records (74 labeled) applied with the r2-a2 amendments. Pointer tables and index routines were
  re-decoded for the win texts, the reel stop tables (AND #$3F / Mod66), CardFaceTiles, ClockHourTable and NametableRowAddr.
- L-010 verified -> 13 Mspr* records plus the generic 436-byte frame block at $C0D3. Checked against the DrawSprite5x3 attr/tile read order.
- L-009 -> passed to commentary (no map change).

### Remaining
- Unknown bytes: none. These spans still have no consumer and stay generic:
  - 6:$B554 (20), 6:$85CE (32)
  - 7:$F398 (8), $F3DC (60), $D0E4 (12), $D0F0 (60), $C0D3 (436), $E195 (15), $E000 (5)
- Code with no known entry: 7:$CFCF-$D024 (86) plus the 675-byte L-004 residue. 6,621 Static code bytes have never been fetched (provenance only).
