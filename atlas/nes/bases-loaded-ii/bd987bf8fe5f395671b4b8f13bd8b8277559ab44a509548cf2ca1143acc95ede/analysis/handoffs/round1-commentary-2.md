# Handoff: round 1, phase 2 (commentary) - depth pass (resume of round1-commentary.md)

**Status**: current-source readability gate MET for all classified source (with documented
uncertainties).  Final editorial audit not run (reserved for final acceptance;
review.json editorial_audit = pending).  Whole-ROM classification remains INCOMPLETE
(95.44%, unchanged; separate requirement).

**Checkpoint / baseline**: maps/round1-disassembly-b.json (unchanged); analysis/state.json
not modified; input/game.nes unchanged.  Source: commentary/source/game.asm.  Edit batches:
commentary/specs/28-48; tools: commentary/tools/branchnames.py (new), edit.py (scoped local
renames, scope-change guard, cached index).

**Verification**: `commentary.py verify` -> ok, byte_identical true (fresh, after the last
edit; source sha256 beb91efe...).  `commentary.py read` refreshed commentary/reading-after.asm.
review.json regenerated (1,966/1,966 scaffold entries reviewed, 869 units).

## Scope / readability assessment
- Previously listed gaps, now read and commented routine by routine with role-named
  branch labels and RAM symbols: bank 2 $C696-$FF44, bank 4 $C0C3-$E01F, bank 6 $C045-$CC80,
  bank 11 $C043-$D320, bank 12 $C5A3-$CC40, bank 13 $C04B-$EC20, bank 15 $E014-$FBDD,
  banks 8/9 field scroller internals.
- Mechanical summaries: 0 remain (128 outside the ranges rewritten by hand).
- Address-only labels: 0 remain (1,547 renamed; hand-named in routines read this pass,
  structural names such as @loop_X/@next/@done/@test_X elsewhere).  Address-suffixed
  routine-internal globals (Owner_XXXX) renamed.  Only D_/U_/C_ data/table labels keep
  addresses (table identities).
- RAM: ~110 new symbols (sound channel records, field staging buffers, CPU AI, password,
  biorhythm, line score, multi-byte offsets); raw operands 663 -> 289 (unknown-purpose bytes).
- Examples: TitleFrame, PlayDayEdit, FindLineupCatcher_PickStarter, PasswordPackFields,
  BiorhythmFromDates, CpuFieldingAI, CpuBaserunning_Decide, SoundUpdate/Sound_Command,
  BatterFrame, RunnerReadControl, FielderJumpOrDive, JudgePlayAtBase, FieldScroll_TileRow.

## Findings
- Bank 4 entry 1 computes player biorhythms (23/28/33-day cycles) from the birth date in
  player-record bytes $0C-$0E and the play date; results stored in the in-game records
  (bank 2 BuildBatterRecords/FillPitcherBiorhythm; SetupTeamPalette -> LoadPlayerBiorhythm).
- Corrected names: Bank2_Return -> BinToDecimal; CpuFielding_Abs16/CompareDistance ->
  Div16/Mul16x8; FielderAtBase -> FielderThrowPose; SelectThrowTarget -> FielderSetHoldPose;
  SwitchControlledFielder -> FielderJumpOrDive; ControlledFielder -> CurFielder (loop
  index/4); FielderSlots_Facing -> FielderSlots_XFrac; BatterPower -> BatHeight;
  CpuFieldingTimer -> CpuSteerDir; ReturnA10 -> StubRequestA10 (ignores X).
- Field view is drawn rotated (field X -> vertical scroll pages of 240 lines).
- Starting pitchers rotate by SeasonDay mod 3 with a two-game rest list (RecentPitchers).
- Base coordinate table at bank 15 $EFF0 starts with a byte classified as code (L-010).

## Leads
Consumed: none closed (L-007..L-009 remain open).  Produced: L-010 (misclassified $EFF0
table), L-011 (runtime confirmation of biorhythm use).

## Unresolved questions
See commentary/uncertainties.md: effect of biorhythm on play; CPU state tables partly in
unclassified bytes (bank 11 $C282, $C49B, $D08E; bank 4 script tables); flag-bit meanings
of BallFlags/AtBatFlags/PitchResult/PlayResult still partial; errors column never seen
incremented; ~289 raw RAM operands of unknown purpose.

## Deferred work
Final editorial audit (`commentary.py audit`) at archive acceptance; reclassification of
unknown ranges and L-010 in redirect/disassembly; runtime confirmations L-009/L-011.
Toolkit: no new issues (TK-005 still applies).
