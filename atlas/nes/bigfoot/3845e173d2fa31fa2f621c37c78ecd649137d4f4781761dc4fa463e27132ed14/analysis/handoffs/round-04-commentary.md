# Round 4 commentary handoff (final acceptance)

Status: COMPLETE. The current-source readability scope (pass-004, 100.00% classified)
is done and the final editorial review is recorded. `commentary.py audit` passed once,
at the end: ok, byte_identical true, 1,401 regions and 1,321 units reviewed,
editorial_audit true, reading copy refreshed (commentary/reading-after.asm).

## Baseline / carry-forward
- The map changed (pass-004: 6 annotate edits, 1,398 -> 1,401 regions, no kind
  changes). The whole round-3 commentary is preserved untouched at
  analysis/round-3/commentary/. Round-1/2 copies were not touched.
- `commentary.py prepare` built a fresh frozen baseline (commentary/before,
  baseline.json, phase1-state.json). notes/ and tools/ were carried by bank/CPU
  address; tools/checknotes.py reports 0 problems.
- review.json: 1,244 entries keep earlier credit (round 1: 934, round 2: 294,
  round 3: 16). 157 entries whose source text changed this round are round 4
  (tools/review.py now diffs region blocks against the round-3 source).

## Changed regions (from round-04-disassembly / round-03-redirect)
- 2:$B0E7 LegalScreenTiles (960) + $B4A7 LegalScreenAttrs (64): raw 1 KiB
  legal/trademark nametable; reader unknown, worded as "may be unused".
- 5:$8388 Data_5_8388: label length fixed to 640 (zero run, unread; role unresolved).
- 5:$8608 EventGroundHeights: 46 x 8 records (column id x tile-column group),
  read by UpdateGroundHeight (F:$D2AC). Header and inline notes explain the
  $8600 + 8 + id*8 + group pointer, the 9-bit average into TruckGroundY and the
  dead `cmp RowWithinBlock` with A = 0 (hill copy compares Tmp20).
- Corrections: RampAirVel/RampBumpSpeed entry-1 reads observed; F:$FF2C and
  6:$8701 trace reads are rts dummy reads (notes updated).

## Editorial fixes beyond the changed regions
- Stale alias: event/hill RAM $0493-$04D2 is now LaneTerrainBuf (80 operands, was
  PaletteBuf+N, incl. HillSwapTerrainRows); PaletteBuf note updated.
- F:$D157 MetatileRowOfTileRow (24 B) named; the round-3 note wrongly called it
  unread (it is read as LaneTerrainBufOffset+2). WorldToTileIndex header rewritten.
- Race $04AF,x RespawnSearchCount (was PaletteBuf+30; tentative name).
- CheckCanAfford pointer constants symbolic (#<TempMoney, #<GarageSlotA ...).
- 223 comment blocks had a stray leading "| " (notes `C X:Y | text`); removed.
- All 17 remaining generic branch labels named (notes/r4_labels.txt). Remaining
  generic labels: 0. Raw addresses remain only in RAM-clear loops, the dead
  `jsr $0014`, the zero-page `and $EF` slip and `lda ($00,x)` timing pads.

## Remaining uncertainties
commentary/uncertainties.md (round-4 section added). Main open items: readers of
LegalScreenTiles, 5:$8388 zero run, 4:$A474, 3:$8991/$894D, 0:$8F84/$9087/$BE6F,
spare bytes; consumer-derived table names; partly decoded record fields.

## Structural questions for redirect
None blocking; no map change proposed. Optional: look for a LegalScreenTiles reader
on an untraced power-on/attract path; a hill-stage read watch on LaneTerrainBuf.

## Artifacts
commentary/source/{game.asm,ram.inc,hardware.inc}, review.json, audit.json,
verification.json, reading-after.asm, progress.md, uncertainties.md;
notes/r4_labels.txt (new), edits in r2_b012/r2_b3456/bF_1/bF_2/r3_ram/r3_regions/
r3_tables_F/r2_headers and others (the "| " cleanup); tools/review.py updated.
Toolkit: no new issues (TK-001..TK-006 unchanged); toolkit not modified.
