# Round 3 — disassembly phase handoff (2026-11-19)

ROM classification: 100.00% (327696 / 327696 bytes classified; 0 bytes unknown).

Status: whole-ROM classification complete (0 unknown bytes). Commentary for L-031 and the changed regions, plus the
final editorial audit, still remain. Classification alone does not complete the project.

## Checkpoint
- pass-003: maps/pass-003.json, generated/pass-003, analysis/pass-003-verification.json, byte_identical true,
  map sha256 4b3c1d85794b... Internal store analysis/state.json (1,497 regions), export maps/current.json.
- Input pass-002 preserved. input/game.nes sha256 unchanged. The server started this pass was stopped.

## Lead outcomes (details: analysis/investigation.md "Round 3 disassembly", analysis/leads.json)
- L-028 verified (+1 byte): phys $19CBC = bank 3 $9CAC [12,13,14,15], lone RTS between JMP $9C94 (end of
  FillNametableTile1) and the Observed DrawBatterLine at $9CAD, now Code/Static at confidence 0.6. Accepted on
  independent checks: exact 1-byte gap, no reference or data consumer, and every comparable $60 byte between a
  terminator and code is already Code. Reachability unresolved. Suggested commentary name: NullHook9CAC (no map label set).
- L-029 verified (boundary): AnimationStepTimes Data $1A3A8+5 (bank 3 $A398-$A39C); DrawStepFigure Code/Static 0.75
  $1A3AD+31 starting at $A39D. Region ids changed, so commentary must re-target the notes on these two regions.
- L-031 (commentary-only): left ready for the commentary phase.

## Per-bank summary (8K banks)
All banks have 0 unknown bytes. The only changes from round 2 are bank 12 (Code 6902, +1 from L-028) and bank 13
(Code 7302 / Data 518, one byte moved Code->Data by L-029). Full table: analysis/progress.md (round 2 table + round 3 delta).
Totals: Code Observed 31,968; Code Static 27,550; Data 136,138; Padding 66,492; Image 65,536; Text 12.

## Lower-confidence classifications for the final audit
- L-028 byte $19CBC: Code 0.6. Evidence is structural and consistency only, with no reference.
- L-024 static code without a known entry: 2,426 bytes at 0.75 (reachability unresolved), plus 33 bytes of data at 0.8.
- Round-1 heuristic generic data: 74,961 bytes at 0.75 (pattern-based, no specific format claimed).
- AnimationStepTimes entries 2-4: the table extent rests on a sibling-layout inference, because the consumer reads only X<2.

## Remaining unknowns
None.

## Gameplay / toolkit
No new gameplay runs (analysis/scenarios.json unchanged). No new toolkit issues found (analysis/toolkit-issues.md, TK-001..TK-004).
