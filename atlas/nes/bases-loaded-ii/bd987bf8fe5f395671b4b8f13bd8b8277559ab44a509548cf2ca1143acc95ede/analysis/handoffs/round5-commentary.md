# Handoff: round 5, phase 2 (commentary)

**Status:** the current-source readability gate is met for all classified source on the
round-5 map. A fresh byte-identical verification passed after the last edit and the reading
copy is refreshed. The final editorial audit is NOT accepted: review.json `editorial_audit`
stays pending for final acceptance. Whole-ROM classification (in the map) is 393,232 of
393,232 bytes, 0 unknown; this pass changed no classification.

## Checkpoint
- Map: maps/round5-disassembly.json (= maps/current.json = analysis/state.json, sha256
  d669b4ac...). input/game.nes unchanged (sha256 bd987bf8...). Toolkit not modified.
  No stale edit process was running.
- Round-4 commentary preserved in full in analysis/round-4/commentary/ (`diff -r` identical
  before the old directory was removed). Earlier round copies untouched.
- Fresh baseline from `commentary.py prepare` (2,223 scaffold entries); before/ frozen.
  Tools and specs carried from round 4.
- Source: commentary/source/game.asm (sha256 43a863ce...), reproducible from the round-4
  source by tools/merge_r5.py, then tools/edits_r5.py, then tools/edits_r5b.py
  (checked: rerunning the chain reproduces the file exactly).
- leads.json backed up to analysis/round5/leads.before-commentary.json.

## Verification
- `commentary.py verify`: ok, byte_identical true (final run after the last edit).
- `commentary.py read`: commentary/reading-after.asm refreshed.
- `commentary.py audit` (informational): stops only at "final editorial audit is pending",
  as intended. Not an accepted audit.

## Scope and readability
- Carried verbatim by physical range: every block except the changed bank-2 range.
- **Changed range** (physical 48109..48932, $FBDD-$FF14): TeamPitcherPtrs is a `.word` list
  of TeamPitchers00..11; the 8-byte pitcher record format is documented once at
  TeamPitchers00; old names remapped (D_B02_FBFD/FBFE -> TeamPitcherPtrs(+1),
  D_B02_FC2D -> TeamPitchers00+$18, the demo pitcher). The stale L-014 "island" notes are gone.
- **Pitcher record fields** (constants PREC_*; REC_PLAYER_INDEX = $0F): 0 handedness bit6,
  1 speed rating 0-31, 2 unknown (no reader), 3/4 break index for Right/Left, 5 pitch type
  0-2, 6 ERA-style figure (tentative), 7 delivery style 0-8. Used at all 32 fixed-offset
  ReadSideTableByte calls and in LaunchPitchedBall / FindFreePlayer_DrawEra.
- Tables named from those fields: PitcherRecordPtrs (was D_B00_89E7, now `.word`),
  PitcherFormTiming, PitchBreakTable, PitchSpeedTable, PitcherFormChrBank.
- **New finding:** D_B02_FAE9 is DefaultLineupPtrs, 12 pointers to 21-byte default lineup
  records ($FB01-$FBFC: 9 player/position pairs + 3 rotation starters). The 32-byte generic
  block at $FBDD is the tail of records 10 and 11. Named in source; published as lead L-024.
- Wording fixes: wind-up timing is the pitcher's (CpuInput_SwingDecision, CpuState42_Aim,
  PlayState_22); PlayState_20/CpuBat_SwitchBox read the pitcher's handedness bit;
  DrawBatterSprites reads only the pitcher record (probably the wind-up; name kept).
- Round-4 redirect G1/G2 applied (password letter-25 behaviour; pitch-type domain 0-2).
- tools/review.py fixed (label offsets now follow .incbin/.org; units capped at region or
  code-run end). 70 round-4 units that ran into following data/padding got corrected
  lengths. review.json: 2,223/2,223 entries reviewed, 1,177 units (20 new).

## Leads
- Produced: L-024 (low; map label/split of the default-lineup table, Data stays Data).
- Consumed: none (L-023 was consumed by disassembly). Pass entry added to analysis/leads.json.

## Unresolved (commentary/uncertainties.md, round-5 section)
- Pitcher byte 2 role; which handedness value is left; ERA/"delivery style" readings.
- Trailing bytes of PitchBreakTable/PitchSpeedTable blocks; bank 2 $FA14-$FAE8.
- DrawBatterSprites' visual subject not checked at runtime.
- Earlier items (natural stamina relief, static-only stamina effects) unchanged.

## Deferred
- Final editorial acceptance (`commentary.py audit` with editorial_audit reviewed).
- Toolkit: no new issues (TK-001..TK-005 unchanged).
