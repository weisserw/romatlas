# Round 1 commentary handoff

Status: COMPLETE (current-source readability for all classified source; fresh
byte-identical verification; reading copy refreshed). Whole-ROM classification is
unchanged (94.39%, 14,697 unknown bytes) and final editorial audit is still pending.

## Scope / readability
- Prepared commentary/ from pass-001 (first round). source/ is regenerated from the
  frozen before/ + commentary/notes/*.txt by commentary/tools/annotate.py (symbol
  substitution only; method in commentary/progress.md, format in notes/README.txt).
- Covered every code region in all banks (fixed, 0, 2-6; bank 1 is data only):
  449 named, commented routines (no sub_/loc_ address labels remain), named shared
  tails, heuristic local branch labels elsewhere, 216 named tables, 533 RAM names
  with mode-specific aliases (event / race / text queue / results reuse the same RAM),
  PPU/APU/MMC1 register names, cross-bank calls resolved where the mode fixes the bank.
- Every generic data block has a consumer/uncertainty note; all 271 unknown ranges
  have a bank note or a specific "read by X" note. review.json: 1361 entries reviewed,
  666 units; editorial_audit pending.
- Weakest area: long event/race routines rely on routine headers plus @skip_N labels;
  some field names (meters, gears, parts) are tentative.

## Examples
- Fixed bank: EventStart/EventMainLoop/NmiEvent (cycle-timed split-screen engine),
  FarCallIndexed + FarCallTable, PlaySoundEffect, AddDecimalAscii (ASCII money).
- Bank 0: ColdStart -> TitleMenu -> RunTour -> PlayTourStop; ShopMain/CpuShop;
  RunObjectScript (32-op race object byte code, ScriptOpTable).
- Bank 2 SoundTick driver; bank 3 CrossCountryRace/ScrollTrack/UpdateCpuDriver;
  bank 4 ShowStopPayout; bank 5 UpdateTruckEngine/ReadPedalStrokes; bank 6 HillEventStart.

## Unresolved meanings
- Exact identity of garage parts A-D, TruckMeter, gear counters; secret button
  sequences not decoded; bank of NmiHandler2's $8592; ScriptOp1D `jsr $0014`.
  Details: commentary/uncertainties.md.

## Structural questions for redirect (bounded checks suggested)
1. bank 0 $8055-$805D (phys $00065-$0006D) is data in the map but is the WarmRestart
   target and decodes as a 4-instruction warm-start stub -> reclassify as code.
2. bank 0 ScriptOpTable $B1F1-$B230: 32 valid handler words; map has 28 bytes data +
   unknown $B20D-$B239 -> extend the jump-table data region (and check $B231 pointers).
3. bank 6 $8D84-$902C (681 unknown bytes): HillNmiJump pointer table (`jmp ($00CE)`
   at $8C4C, index HillSpecial) whose targets are probably code -> decode table, descend.
4. ~35 short unknown tables with direct absolute-indexed consumers (list in
   commentary/uncertainties.md) -> classify as data with consumer evidence.
5. Large unknown runs between generic data in banks 1-6 (text, tile rows, metatiles,
   AI paths) remain unresolved; pointer tables named in source (StopNamePtrs,
   TrackMapPtrs, InstrumentTable, $B4AD/$B4B5 scenery, $B231 scripts) are entry points.

## Verification
- `python3 /root/romatlas/scripts/commentary.py verify <workspace>`: byte_identical true
  (commentary/verification.json). Reading copy: commentary/reading-after.asm.

## Remaining gaps
- None for current-source readability scope. Final editorial audit and whole-ROM
  classification remain separate gates.

## Artifacts
- commentary/source/{game.asm,ram.inc,hardware.inc}, commentary/reading-after.asm,
  commentary/review.json, commentary/progress.md, commentary/uncertainties.md,
  commentary/notes/, commentary/tools/{annotate.py,review.py}
- Toolkit: TK-005 added (missing feature: no labels in commentary baseline) in
  analysis/toolkit-issues.md; non-blocking.
