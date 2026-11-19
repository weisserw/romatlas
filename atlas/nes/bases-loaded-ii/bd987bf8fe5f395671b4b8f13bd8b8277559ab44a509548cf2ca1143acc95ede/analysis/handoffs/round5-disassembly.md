# Handoff: round 5, phase 1 (disassembly)

ROM classification: 100.00% (393232 / 393232 bytes classified; 0 bytes unknown).

## Summary
- Input checkpoint maps/round4-disassembly.json -> output maps/round5-disassembly.json
  (verification analysis/round5-disassembly-verification.json: ok, byte_identical true;
  map.validate ok). input/game.nes unchanged; toolkit not modified.
- Script analysis/round5/tools/apply_r5.py (asserts the layout from ROM bytes before editing);
  changed ranges analysis/round5/changed-ranges.json; pre-edit state analysis/round5/state.before.json.

## Lead outcome
- **L-023: verified.** Independent checks: 12 pointer words = $FC15 + 64*team; the first target is
  the pointer-table end; the 12 x 64-byte tables end at $FF14 just before Static code region 508
  ($FF15); byte 5 is 0..2 in every record (one type-2 per team); $FC2D = team 0 record 3.
  Consumers re-read: BuildPitcherRecord $D5FF, FindFreePlayer_DrawEra $D37D, InitDemoRosters_Fielding $C7F5.
- Deviation from the recipe: one region per team table (each is a pointer target) instead of one
  768-byte TeamPitcherTables block.
- No other leads actionable for the map; no gameplay needed.

## Changed ranges for commentary
- Bank 2, physical 48109..48932, CPU $FBDD-$FF14 (context prg_banks [0,1,4,5]); former Data regions
  2108, 506, 2109, 507 now:
  - $FBDD-$FBFC generic data (32 B, unlabeled; role unresolved)
  - $FBFD TeamPitcherPtrs (24 B, 12 words -> TeamPitchers00..11)
  - $FC15 + 64*t TeamPitchers00 .. TeamPitchers11 (64 B each, 8 pitchers x 8-byte records)
- Old names commentary must remap:
  - D_B02_FBFD -> TeamPitcherPtrs; D_B02_FBFE -> TeamPitcherPtrs+1
  - D_B02_FC2D -> TeamPitchers00+$18 (team 0 pitcher 3, demo pitcher)
  - Old block comments for the four generic regions (L-014 "island" notes) no longer apply.
- Commentary to add (from r4 redirect): record byte 5 = pitch type 0..2 (StaminaLimitPtrs index),
  byte 6 = ERA-style figure (tentative); G1/G2 notes on PasswordDecode/PasswordChecksum/PasswordDigits,
  LaunchPitchedBall/CpuPitcherDecision; close the round-4 letter-25 and byte-5 uncertainty bullets.

## Toolkit
No toolkit issues found this pass (TK-001..TK-005 unchanged; analysis/toolkit-issues.md).
