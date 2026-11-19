# Round 3 — commentary phase handoff (pass-003)

Status: **complete** for the current-source readability scope (depth limits stated below).
ROM classification unchanged by this phase: 100.00% (393230 / 393232; 2 unknown bytes 28:$8620, 28:$9883).

- Input checkpoint: pass-003 (maps/current.json = analysis/state.json, sha256 b810b6fa…8c0d),
  frozen as commentary/baseline.json / phase1-state.json by a fresh `commentary.py prepare`.
- Previous commentary preserved first: analysis/round-2/commentary/ (diff -r identical; 25 files
  sha256-matched) before prepare. Round-1 copy remains at analysis/round-1/commentary/.
- Buildable source: commentary/source/game.asm (sha256 8df71195…).
- Verification: `python3 /root/romatlas/scripts/commentary.py verify <ws>` -> ok, byte_identical true
  (commentary/verification.json). Reading copy refreshed: commentary/reading-after.asm.
- Records: commentary/progress.md, commentary/uncertainties.md, commentary/review.json (1232 scaffold
  entries reviewed with summaries/source lines; 1939 units; editorial_audit pending).

## Method
analysis/tools/round-003-commentary/ = round-2 tools + names_ballr3.py (all round-3 names, loaded last
by names.py). Pipeline (scratch /tmp/ra67d): parse.py -> emit.py -> (gen_copies.py; unchanged) ->
review.py, then commentary.py verify/read. New in emit/names: stale Var_/Tbl_ tokens in prose are
rewritten to current names; data notes list contained tables and immediate-pointer consumers.

## Scope covered
- Carry-over: all round-1/2 names/comments by bank:CPU; 23:$B402 unknown note replaced by the
  NextLevelFrom4_DeadRts dead-RTS comment (region 1251, L-017).
- RAM: 76/76 Var_xxxx renamed (e.g. ObjIndex, BallHolderPtr, Mmc3Reg0-7, FielderOnBase,
  FielderBaseMask, RatingCounters, BatterSpeed, PitchSpeedCalc/Shown, LandingX/Y, SndCurChannel).
- ROM tables: 81/81 Tbl_ equates named with format notes (e.g. RunnerBasePoints, PlayerRecordPtrs,
  PitcherRecordPtrs, SndNotePeriods, PadDirToGrid, ContactWindowPtrs, FielderHomePoints).
- Data: 0 blocks left at 'format not established'; 41 pointer-read blocks name their consumer;
  21 specific notes (RatingWeights, RatingValueLayout, velocity tables, metasprites, ...).
- Routines: 247 headers rewritten/corrected in banks 16/20/21/22/23/26/27/28/31; misnamed helpers fixed
  (RunnerPathOffset/RunnerPosLegA-D, FielderNearBase, SetThrowTargetFromPad, PitchInStrikeZone,
  CheckStrikeCall, LaunchBallPresetA/B, CpuBaseCovered, PadToGridIndex, ...).
- L-016 folded: BatContact (@notSwinging, state $20 block note), BatterGroup2Dispatch,
  ApplyPitchOutcome (@teamBanner, was @repitch), PitchOutcome comment.

## Readability assessment
All classified source has descriptive routine/RAM/table names (0 Sub_/Var_/Tbl_ address names) and
a header comment for every routine; every data block has a format or consumer note; the 2 unknown
bytes have specific notes. Remaining gaps (explicit): 180 routine headers are still one-line
mechanical descriptions (mostly bank 20/21 menu helpers, shared _Return/_Cont entries, NMI no-op
hooks, simple setters) — accurate but terse; local branch labels are mostly mechanical
(@skip/@join/@loop); large bulk data (bank 18 screens, bank 27 animation data, bank 14 intro) is
described only at block level. Game meanings marked tentative are unconfirmed (no runtime work).

## Unresolved meanings (commentary/uncertainties.md)
Scoreboard figures (HR/average/ERA/speed readings), umpire (object 10) and catcher (object 2)
identities, Safe/HitsAllowed counters, L-016 banner scene, wall ids 1-5, lineup model,
InitRamTables 20/64-byte blocks, clock, BootABFlag, write-only bytes.

## Remaining gaps / next
- 2 unknown bytes (policy only, L-015). Final editorial audit not run (reserved for acceptance).
- No toolkit issues found (prepare/verify/read worked). Scratch /tmp/r3c and /tmp/ra67d removed.
