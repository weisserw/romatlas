# Round 3 -- commentary handoff (2026-11-18)

Status: COMPLETE for current classified source (editorial audit reserved for final acceptance).

## Scope
- maps/current.json (pass-003) differed from the frozen commentary baseline (pass-002) only in
  region 648 evidence text (L-011). The whole round-2 commentary was preserved in
  analysis/round-2/commentary/ (+ analysis/round-2/tools-commentary-r2/), then a fresh baseline was
  prepared from pass-003 (472 scaffold entries; before/game.asm identical to round 2).
- Source commentary/source/game.asm is rendered by analysis/tools-commentary-r3/apply.py
  (frozen before/ + notes/*.txt); the unedited notes reproduced the round-2 source exactly.
  review.json rebuilt by analysis/tools-commentary-r3/review.py: 472/472 reviewed, 619 units,
  review credit carried by id/range/kind (425 round 1, 29 round 2); 18 entries re-reviewed in
  round 3 (camera-related routines/tables and region 648). editorial_audit pending.

## L-012 outcome (applied; leads.json status consumed)
- First CameraTables record = full-field view with scoreboard ($CB=0, ScreenSel $00, nametable
  $2000, draw indices $01-$14). Second record = infield close-up used while pitching ($CB=6,
  ScreenSel $08, nametable $2800, draw $16-$21 = INFIELD_DRAW).
- Updated: SetCamera, CameraTables, CameraParam/ScreenSel/DrawFirst/DrawLast/CameraFrameOfs,
  INFIELD_DRAW, FrameTask_camera, RenderTask scoreboard skip, AnimateCycleAndSleep + AnimCycle
  tables ($D810-$D85A), RunFrameFirst, RunnerStandPose/Frames, DrawIndexForSlot.
- Also fixed two comments where the views had been swapped: FielderReadyPose/FielderReadyFrames
  and the backup fielder idle frames. SetFirstCameraHideSprY renamed to SetFullFieldViewHideSprY;
  new symbol CameraRecC8 ($C8, no reader). The uncertainties.md camera item is resolved.

## Region 648 ($DF92-$DFC3)
New label SumOfSquaresXY with a block comment: A:Y = 4*(X^2+Y^2); no effective entry found (BEQ at
$DF90 always taken); reachability unknown, not claimed dead. SquareByte comment corrected
(A:X = 4*|A|^2 from its 7-step loop, exact for |A| <= 127).

## Verification
- `commentary.py verify`: ok, byte_identical true; game.asm sha256
  d2739ac33d042ff604050f747a46bd56c559e935749d35e917f7b7680518164a. input/game.nes unchanged.
- Reading copy refreshed: commentary/reading-after.asm. 0 auto labels, 0 raw address operands.

## Unresolved meanings (commentary/uncertainties.md)
HitPower vs HitAngle role swap; ForceCheck N flag; runner progress arithmetic at $D981;
song-to-event map; purposes of unreferenced fragments (AnimateCycleAndSleep,
RestartFieldersReturn, FielderGoRole5, VectorAngleCode, SubtractY32nd, SumOfSquaresXY);
CameraRecC8 has no reader.

## Remaining gaps
None for current-source readability. Whole-ROM classification 24591/24592 (1 unknown byte,
0x2604/$E5F4, L-009). Final editorial audit not run (reserved).

## Continuation (L-013) (2026-11-18)

Status: COMPLETE (commentary-only; maps/current.json and analysis/state.json unchanged since the
pass-003 baseline, so the existing commentary was resumed, not re-prepared).
Pre-edit copies: analysis/round-3/pre-l013/ (notes, game.asm, review.json, uncertainties.md,
verification.json, reading-after.asm, leads.json, review.py).

### Applied
- ForceCheck ($E246): N SET = runner heading for base Y is forced (batter-runner present and
  >= Y RunnerRole entries with progress <= 2Y-1); N clear = not forced / no batter-runner.
  ForceCheckSelf: Y = slot/2 - 2 (= runner index + 1; the old "slot/2 - 1" ignored the clear carry).
- Caller comments corrected: Runner_onBase ($D921, $D937), Runner_ballInFlight ($D955),
  CheckTagOrForce ($DAF7/$DB0C), RunnerDecision ($DB57), FielderHasBall walk chase ($E049/$E05A)
  and CPU throw ($E098/$E0BE), OutOnTag ($E38C).
- New finding: RunnerDecision_stay returns carry clear with N unchanged; from the ForceCheck path N
  is still set, so a forced runner GOES. Renamed RunnerDecision_keepN.
- TryTagRunnerAtBase -> TryForceOutAtBase (force out under the corrected contract).
- $D981 Runner_awardedBases: A = 2*(ceil(p/2)+3-BasesAwarded); Runner_retreat when
  ceil(p/2) < r + BasesAwarded (r = slot/2 - 3), else Runner_advance; intent noted as a reading.
- RunnerRole ($0598) symbol: progress encoding 0 home / 1 batter running / 2k on base k /
  2k+1 leading off or running from base k / 8 scored, runtime-confirmed.
- Concrete inconsistency fixed: RunnerFlags+1,x with X = slot/2 actually addresses $99-$9C. New
  symbol RunnerCommitted ($99, 4 bytes; operands RunnerCommitted-3,x at $D8A7/$D8D0/$D929).
  $95-$98 renamed RunnerOnStartBase (MarkRunnerAtStart sets it, OutOnFlyCatch clears it: tag-up
  flag, tentative); TryForceAtBase -> TryTagUpOutAtBase (tentative; its BEQ at $E358 is always taken).
- BasePoints / PointNextBase / PointPrevBase comments give the exact offsets and targets.

### Kept open (explicit uncertainty, source + uncertainties.md)
- Runner direction naming: statically PointPrevBase(2k+1) is base k+1 (ahead) and PointNextBase(2k)
  is the point beside base k, so Runner_retreat/Runner_backToBase advance the runner and
  Runner_advance / the Runner_go* path keep or return him. The round-1 names look swapped. This
  fits $D981, the walk/bounce force paths and the two-out path. Not renamed without a per-frame trace of a full advance.
- HitPower/HitAngle: no new evidence; names kept, existing tentative note unchanged.

### Verification
- `commentary.py verify`: ok, byte_identical true; game.asm sha256
  b89dd66ccadebed10ed67d78c6118ed7dc5817522147261fd2cd7a9cb41aeb50. input/game.nes unchanged.
- Reading copy refreshed (commentary/reading-after.asm). 0 auto labels, 0 raw address operands.
- review.json (review.py, L013 address list): 472/472 reviewed, 619 units; 42 entries now
  review_round 3 (25 re-reviewed for L-013, notes appended). editorial_audit still pending.
- leads.json: L-013 consumed (attempt + pass record appended).

### Remaining gaps
None for current-source readability. Whole-ROM classification 24591/24592 (0x2604/$E5F4, L-009).
Final editorial audit reserved.

## Continuation (L-014) (2026-11-18)

Status: COMPLETE (commentary-only; maps/current.json and analysis/state.json unchanged since
pass-003, existing commentary resumed). Pre-edit copies: analysis/round-3/pre-l014/ (notes,
game.asm, review.json, review.py, uncertainties.md, verification.json, reading-after.asm, leads.json).

### Applied (evidence: analysis/redirect/r3-b/, runtime-confirmed on a natural route)
- Recipe renames: Runner_retreat -> Runner_runNextBase; Runner_backToBase/backWait/backCheck/
  backArrived -> Runner_runToBase/runWait/runCheck/runArrived; Runner_toRetreat -> Runner_toRunNext;
  Runner_flightRetreat -> Runner_flightRun; Runner_advance/advanceStep/advanceWait ->
  Runner_holdPastBase/holdStep/holdWait; PointPrevBase -> PointBaseAhead; PointNextBase -> PointPastBase.
- Comments rewritten as runtime-confirmed: Runner_onBase, Runner_ballInFlight (two-out and forced
  paths run on), Runner_awardedBases (run on until base r + BasesAwarded, then hold; awarded path
  not exercised), Runner_holdPastBase, Runner_runNextBase, Runner_runToBase, Runner_runArrived,
  Runner_scoredExit ($44 past home), BasePoints, point-helper overview, PointPastBase, PointBaseAhead.
- Consistency fixes in touched code: Runner_waitPitch/checkSteal comment said RunnerDecision "sends
  him back (carry set)"; the code runs him on to the next base on N set (a steal). Runner_runStep/
  runToNext -> Runner_settleStep/settleOnBase (its target is PointPastBase, the standing point past
  the base, not the next base; avoids a clash with Runner_runNextBase). Runner_afterPickoff ->
  Runner_arrivedNoAward (reached a base with BasesAwarded clear; nothing pickoff-specific).
  Lead-off comment no longer claims a steal start "goes at once" (it jumps to Runner_goDelay1).
- Kept as "static reading only" in source and uncertainties.md: Runner_goDelay1 -> PointBaseAlt
  (now documented as $24 + 4*(floor(p/2) mod 4), base k for 2k+1) -> Runner_leaveBase -> settle
  (return to base k); not executed in the r3-b trace.
- uncertainties.md: OPEN runner-direction item replaced by a RESOLVED item plus the static-only item.

### Verification
- `commentary.py verify`: ok, byte_identical true; game.asm sha256
  a0f6b465aeb5042f10832668cec98a787e5d05a6b9319e94ec9df7058dd3b7ef. input/game.nes unchanged.
- Reading copy refreshed (commentary/reading-after.asm). 0 auto labels.
- review.json (review.py, new L014 address list): 472/472 reviewed, 619 units; 20 entries carry the
  L-014 re-review note (49 entries review_round 3). editorial_audit still pending.
- leads.json: L-014 consumed (attempt record appended).

### Remaining gaps
None for current-source readability. Open meanings: return-to-base path (static), HitPower/HitAngle,
song-to-event map, unreferenced fragments, CameraRecC8. Whole-ROM classification 24591/24592
(0x2604/$E5F4, L-009). Final editorial audit reserved.
