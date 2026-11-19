# Round 2 — commentary phase handoff (pass-002)

Status: **complete** for the current-source readability scope (depth limits stated below).
ROM classification unchanged by this phase: 100.00% (393229 / 393232; 3 unknown bytes).

- Input checkpoint: pass-002 (maps/current.json = analysis/state.json, sha256 9a0cd81b…7069),
  frozen as commentary/baseline.json and commentary/phase1-state.json.
- Previous commentary preserved first: analysis/round-1/commentary/ (verified identical copy of the
  round-1 commentary/ directory), then a fresh `commentary.py prepare`.
- Buildable source: commentary/source/game.asm (ca65, 35,121 lines).
- Reading copy (refreshed): commentary/reading-after.asm. Frozen baseline: commentary/reading-before.asm.
- Verification: `python3 /root/romatlas/scripts/commentary.py verify <ws>` -> ok, byte_identical
  true; commentary/verification.json (game.asm sha256 035de1a1…08b4).
- Records: commentary/progress.md, commentary/uncertainties.md, commentary/review.json (1,232
  scaffold entries reviewed with summaries/source lines; 1,938 units incl. 166 data units;
  editorial_audit pending for final acceptance).

## Method (reusable)
analysis/tools/round-002-commentary/: parse.py (scratch /tmp/ra67d) -> emit.py -> review.py, then
commentary.py verify/read. Names keyed by bank:CPU in names_core.py + names_bNN.py (round 1) and
names_bNNr2.py (round 2, loaded last, override). gen_copies.py regenerates names_bcopies.py for the
bank 0/4 (of bank 2) and bank 6 (of bank 16) copies. Round-1 names carried 923/923 (pass-002 kept all
old code); new globals from FarCall tables are forced into the label set.

## Scope covered
- Code: all 48,576 bytes; 1,426 routine labels all named with header comments (0 Sub_ labels,
  0 'role not yet explained'); ~470 new names this round. FarCall tables (13) and all 63 inline jump
  tables are `.addr` handler lists; every FarCall site names its target.
- Data: every region has a prose note (format, consumers by routine name, uncertainty); labelled
  regions get `.org` so their labels are operands (Field*_NN, Metasprite_*, CartHeader*...).
- Unknown: 3 one-byte notes (23:$B402, 28:$8620, 28:$9883).
- Contiguous code regions merged into single assembler blocks ('(continues)').

## Readability assessment (honest)
- Strong: bank 31; menus/submode flow (banks 20/21 incl. substitution, relief, inning change,
  score/level, rating, final scoreboard); ending and GAME SET (bank 19); score/level/rating logic
  (bank 23); runner state model (22/23); field walls and fielder actions (26); defensive plays (27);
  result scoreboard/team banner (28/29); copies (0/4/6) by reference.
- Adequate but shallow: many round-1 one-line helper comments in banks 22/23/26/28 (pitch physics,
  ball flight, fielder AI helpers) and mechanical @skip/@join labels; 76 Var_xxxx RAM and ~80
  Tbl_BB_AAAA equates whose role/format is unknown; generic data blocks have boundary/consumer
  notes only.

## Examples
- `RunnerStateDispatch` table: RunnerBatterToFirst, RunnerOnFirst, RunnerToSecond … RunnerAtHome.
- `SelectNextLevel` (23:$B386) with NextLevelFrom0-4 and SetLevel0-5; `ComputeRating` formula.
- `MoveWithWalls` / `WallZoneDispatch` (renamed from round-1 Throw*), `FielderActionDispatch`.
- `InitInningChange` -> `CheckLargeLead` / `CheckGameOver` (end-of-game rule).
- `FarCallTable_22` as .addr entries incl. `RatingScreenNmi` ($B420, previously unresolved).

## Corrections to earlier commentary
Stadium/bank 0/2/4 claim removed (StadiumFieldTable); $66 Level, $03FB RatingTotal; $037A/$037B
pitcher fatigue and CheckPitcherTiredSound (was CheckWalkOffSound); bank 26 Throw* -> wall
collision; pitcher dispatch -> fielder jump/dive actions; TargetBase_* -> Cover*/BackUp*.

## Unresolved meanings / questions for redirect (details: commentary/uncertainties.md)
1. The 3 lone RTS bytes (23:$B402, 28:$8620, 28:$9883): policy decision only.
2. Wall ids 1-5 and field axis orientation (watch ball X/Y at WallHit).
3. Lineup slot model / pitcher index range (watch $03A3-$03BD after a substitution).
4. Which event gives PitchOutcome 2 (team banner, GameMode 2 submode 2).
5. InitRamTables 20/64-byte blocks; scoreboard values $0356/$0357/$0363/$03DA; clock; BootABFlag.

## Remaining gaps
Depth limits above; final editorial audit not run (reserved for final acceptance). No toolkit
issues found this phase (commentary.py prepare/verify/read worked).
