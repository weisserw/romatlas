# Handoff: round 6, phase 1 (disassembly)

ROM classification: 100.00% (393232 / 393232 bytes classified; 0 bytes unknown).

## Summary
- Input checkpoint maps/round5-disassembly.json -> output maps/round6-disassembly.json (map sha256 a5b53604...;
  analysis/round6-disassembly-verification.json: ok, byte_identical true; map.validate ok). maps/current.json =
  checkpoint. input/game.nes unchanged (sha256 bd987bf8...); toolkit not modified.
- Script analysis/round6/tools/apply_r6.py (independent asserts from ROM bytes + exact cross-check against
  analysis/redirect/r5-redirect-a/batch-proposal.json, then edits); consumer re-read helper
  analysis/round6/tools/refs.py; changed ranges analysis/round6/changed-ranges.json; pre-edit copies
  analysis/round6/state.before.json, analysis/round6/leads.before.json.
- Counts unchanged: code 55746 / data 123102 / padding 83298 / text 14 / CHR image 131072. Regions 2223 -> 2248.
- No gameplay needed (static batch). No toolkit issues found (TK-001..TK-005 unchanged; analysis/toolkit-issues.md).

## Accept/reject per item (57 accepted, 0 rejected; all Data, Static)
- A (13) accepted, conf 0.95: TeamRosterPtrs $E811 (12 words = $E829+400t), TeamRoster00..11 (400 B = 25 x 16 B;
  byte $0D 1-12 / $0E 1-31 in all 300 records; consumers $D635/$D6DF/$DFF2/$FF2E, Y = EditTeam*2).
- B (13, L-024) accepted, 0.95: DefaultLineupPtrs $FAE9, DefaultLineup00..11 (21 B; positions exactly 1-9, starters 0-7;
  record 11 ends at TeamPitcherPtrs). Absorbs the round-5 32 B generic block $FBDD.
- C (6) accepted, 0.95: bank 14 CloseupMetaspriteSidePtrs/FormPtrs/Frames ($C0FA/$C0FE/$C122) and
  CloseupExtraSpriteSidePtrs/FormPtrs/Frames ($C5A2/$C5A6/$C5CA). Consumer bank-14 DrawBatterSprites ($C050/$C0C5,
  context [0,1,28,29]). Boundary: the lowest CloseupExtraSpriteFrames target is exactly $CA4A, so the tables end at
  $CA49 and region 1290's remainder now starts at $CA4A (physical 232026, was 232025); both sides stay Data
  (remainder = generic metasprite records, unlabelled).
- D (6) accepted, 0.95: bank 3 Layout_PpuAddr/MetatileMap/AttrData/Palette/MetatileDefs/ChrBank, $C04E + 44i,
  22 words each (index ScreenLoadId*2; bases 44 B apart; last ends at Observed code $C156).
- E (4) accepted, 0.95: PitcherRecordPtrs $89E7, PitcherFormTiming $8BC3, BaseCoordTable $EFF0 (label only, same
  ranges); PitcherFormChrBank $D518 split from the 2-byte handedness table $D516 (stays generic, unlabelled).
- F (15) accepted, 0.9: D_B11_C30F, D_B11_C36D, D_B11_C3E9, D_B11_CAA6, D_B11_C1BF, D_B07_CD0D, D_B07_D079 (pointer +
  uniform records); D_B07_D05F, D_B07_D0B9, D_B13_ECA3, D_B15_DEA5, D_B15_F689, D_B15_F699, D_B06_CC82, D_B06_CCA2
  (pointer tables). Caveats in evidence: CD0D extent from 24 stride-exact entries (consumer index seen 0..11);
  CC82 index domain untraced (bounded by the referenced CCA2 and the pair's lowest target).

## Changed ranges (removed regions 494-505, 518, 889, 929, 931, 1043, 1279, 1288-1290, 1489, 1982, 1983,
## 2053-2055, 2059, 2077, 2080, 2097-2107, 2110, 2138, 2151, 2152, 2204, 2206, 2219; new ids assigned)
- Bank 2 physical 43041..48141 ($E811-$FBFC, context [0,1,4,5]): entirely the A+B tables (no generic left).
- Bank 3 physical 49246..49510 ($C04E-$C155, [0,1,6,7]): entirely the six Layout_ tables.
- Bank 14 physical 229642..232026 ($C0FA-$CA49, [0,1,28,29]): entirely the C tables; generic remainder $CA4A-$D039.
- Small tables (exact ranges in changed-ranges.json); generic remainders: bank 6 $CCC2-$CD6D; bank 7 $CCE7-$CD0C,
  $CD85-$CE25, $CF06-$D05E, $D063-$D078, $D0C9-$D265; bank 11 $C1A0-$C1BE, $C21F-$C26A, $C36A-$C36C, $CA76-$CAA5;
  bank 13 $EC06-$ECA2, $ECB3-$EDA7; bank 15 $D516-$D517, $DEC5-$DF24, $F6A9-$F6F0. Remainders keep their old
  generic evidence plus a round-6 note.

## Old -> new names commentary must remap
- D_B02_E811 -> TeamRosterPtrs; D_B02_E812 -> TeamRosterPtrs+1; D_B02_ED89 -> TeamRoster03+176 (player 11 record).
- D_B14_C0FA/C0FB -> CloseupMetaspriteSidePtrs(+1); D_B14_C5A2/C5A3 -> CloseupExtraSpriteSidePtrs(+1).
- Source-only labels now map labels (do not duplicate): DefaultLineupPtrs, DefaultLineup00..11, Layout_* (6),
  PitcherRecordPtrs, PitcherFormTiming, PitcherFormChrBank, BaseCoordTable.
- Group F keep their names as map labels; second-byte aliases become +1: D_B06_CC83, D_B06_CCA3, D_B07_CD0E,
  D_B07_D060, D_B07_D07A, D_B07_D0BA, D_B11_C1C0, D_B11_C310, D_B11_C36E, D_B11_C3EA, D_B11_CAA7, D_B13_ECA4,
  D_B15_DEA6, D_B15_F68A, D_B15_F69A.
- Block comments to replace: L-014 "island" notes for 2097-2107/2110/2151/2152, the "$FA14-$FAE8 unidentified" and
  "$FBDD-$FBFC role unresolved" notes; bank-14 "Role/consumer not identified" for 1288/1289; D_B15_D516 keeps the
  2-byte handedness table.
- Commentary to add: 16-byte roster record (bytes $0D/$0E birth month/day), 21-byte lineup record, close-up 3-level
  table scheme (side -> form -> 32-word frame tables).

## Recommendation
Commentary pass for the remap/format notes, then final acceptance; no redirect leads remain open from this batch.
