# Handoff: round 4, phase 1 (disassembly)

ROM classification: 100.00% (393232 / 393232 bytes classified; 0 bytes unknown).

## Summary
- Input checkpoint maps/round3-disassembly.json -> output maps/round4-disassembly.json
  (verification analysis/round4-disassembly-verification.json: ok, byte_identical true;
  map.validate ok). input/game.nes unchanged; toolkit not modified.
- Script analysis/round4/tools/apply_r4.py (asserts the layout from ROM bytes before editing);
  changed ranges analysis/round4/changed-ranges.json; pre-edit state analysis/round4/state.before.json.

## Lead outcome
- **L-021: verified.** Independent checks: 2 RAM pointer words, 3 table pointers; the first target
  is the pointer-table end; the tables are spaced 16 bytes apart and end at $91D4 just before
  Observed LaunchPitchedBall $91D5; each table strictly descends by 2. Consumers re-read:
  LaunchPitchedBall ($91F1/$91F6, $921D/$9222, $9246/$924B) and bank 15 PlayState_22 ($C4D3).
  The residual (pitch-type byte-5 domain not enumerated) does not affect the extent proof.
- No other leads were actionable for the map (L-020/L-022 are commentary-only; L-004/L-005/L-009/
  L-018 are unchanged and semantics-only). No gameplay was needed.

## Changed ranges for commentary
- Bank 0, phys 4523..4581, CPU $919B-$91D4 (former generic region 1985), now 5 Data regions:
  - $919B StaminaVarPtrs (4 B, words -> $06C5, $06C4)
  - $919F StaminaLimitPtrs (6 B)
  - $91A5 StaminaLimitTable0, $91B5 StaminaLimitTable1, $91C5 StaminaLimitTable2 (16 B each)
- Commentary still uses D_B00_919B/D_B00_919C/D_B00_919F/D_B00_91A0. Bring it onto
  maps/round4-disassembly.json with these labels, and apply L-020 (stamina rename) and L-022
  (password roll 0-25). After that, do the final editorial audit.

## Toolkit
No toolkit issues found this pass (TK-001..TK-005 unchanged; analysis/toolkit-issues.md).
