# Commentary uncertainties

Carry forward relevant phase-1 ambiguities as their regions are reviewed.

## Round 1 (carried; still valid unless revised below)
- Tentative game meanings (worded tentatively in source): HitFlag ($88, bunt vs
  swing), BallFlightFlag ($86), RunnerFlags ($95, renamed RunnerOnStartBase in round 3), FielderRole ($059C..), LeadRunnerBase
  ($BB), DoublePlayTried ($BC), BallZoneTable values, PitcherHand/BatterHands as
  left/right-handed sprite sets, CameraTables byte $C8 (now CameraRecC8: no reader found).
- $0100 written with APU enable value but never read; $85 cleared but never read.

## Round 2
- Residual Unknown $E5F4 (0x2604): single $60, no reference; L-009 rejection stands.
  (Round 4: now typed Data by a user-directed fallback only; see Round 4 below.)
- Camera naming: RESOLVED in round 3 (L-012, see below).
- BatterTask: RotateByAngleBits is called with HitPower as the angle bits and the
  HitAngleBase/Thresholds value as the vector length, so HitPower behaves like an
  elevation angle and the "HitAngle" tables like a speed. Round-1 names kept.
- Runner progress values: encoding confirmed in round 3 (L-013, see below); the
  BasesAwarded formula at $D981 is now given exactly, its intent is still unverified.
- ForceCheck's N-flag meaning: RESOLVED in round 3 (L-013): N set = forced (see below).
- Unreferenced fragments: purpose tentative for AnimateCycleAndSleep (running cycles),
  RestartFieldersReturn/PointFielderReturn (slot*2-12 point pairing unclear),
  FielderGoRole5 (role 5 would read past FielderReadyFrames), VectorAngleCode (format
  compatibility with RotateByAngleBits not proven), SubtractY32nd (sound decay?).
- InterceptBall: $DF92-$DFC3 (sum of squares + SquareByte, Static Code) follows an
  always-taken BEQ and has no effective entry (raised for redirect as SQ-R2-2; the map
  evidence was corrected in round 3 by L-011).
- Sound: song numbers -> game events only partly known (bit 0 home run, bits 5/6
  half-inning jingles, bit 3/4 start/title by call sites).

## Round 3
- Camera naming resolved (L-012, bounded natural-snapshot runtime check in round-2
  redirect, SQ-R2-3): the first CameraTables record (carry clear, $CB=0, ScreenSel $00,
  nametable $2000, draw indices $01-$14) is the full-field view with the scoreboard; the
  second (carry set, $CB=6, ScreenSel $08, nametable $2800, draw indices $16-$21 =
  INFIELD_DRAW) is the infield close-up shown while pitching. INFIELD_DRAW was right;
  SetCamera's round-1 comment and the r1 "$00 infield / $06 outfield" note were inverted
  (historical records left unchanged). Source now uses "full-field view" / "infield
  close-up" throughout.
- SumOfSquaresXY/SquareByte ($DF92-$DFC3): no effective entry found (BEQ at $DF90 always
  taken); reachability unknown, not claimed dead. SquareByte's 7-step loop returns
  4*|A|^2 (exact for |A| <= 127), so the block yields 4*(X^2+Y^2); its intended use is
  only guessed (an earlier InterceptBall distance test).
- CameraRecC8 ($C8): copied from the camera records ($15 / $0D) but no reader found.

## Round 3 continuation (L-013, commentary only)
- RESOLVED: ForceCheck ($E246) returns N set when the runner heading for base Y is forced
  (batter is a runner and at least Y RunnerRole entries have progress <= 2Y-1); N clear =
  not forced or no batter-runner. Exact model over all 20 occupancy cases plus 719 natural
  calls (analysis/redirect/r3-a/). The source said the reverse; ForceCheck, ForceCheckSelf
  (Y = slot/2 - 2, not slot/2 - 1) and every caller comment (Runner_onBase, Runner_ballInFlight,
  RunnerDecision, CheckTagOrForce, FielderHasBall walk/CPU paths, TryForceOutAtBase, OutOnTag)
  were corrected. RunnerDecision_stay was renamed RunnerDecision_keepN: from the ForceCheck
  path it returns with N still set, i.e. a forced runner goes.
- CONFIRMED (runtime positions): RunnerRole progress 0 = batter at home, 1 = batter running,
  2k = on base k, 2k+1 = leading off / running from base k, 8 = scored.
- $D981 exact: A = 2*(ceil(p/2) + 3 - BasesAwarded); Runner_runNextBase (round-1
  Runner_retreat) when ceil(p/2) < r + BasesAwarded (r = slot/2 - 3), else Runner_holdPastBase
  (round-1 Runner_advance). Read as: run on until base r + BasesAwarded, then hold; the
  awarded-bases path itself was not exercised at runtime.
- Runner movement direction / names: RESOLVED in the L-014 continuation (runtime-confirmed on a
  natural route, analysis/redirect/r3-b/dir_runner.out and dir_point.runners.out). The round-1
  names were inverted. Renamed: Runner_retreat -> Runner_runNextBase, Runner_backToBase/backWait/
  backCheck/backArrived -> Runner_runToBase/runWait/runCheck/runArrived, Runner_toRetreat ->
  Runner_toRunNext, Runner_flightRetreat -> Runner_flightRun, Runner_advance/advanceStep/
  advanceWait -> Runner_holdPastBase/holdStep/holdWait, PointPrevBase -> PointBaseAhead (base
  floor(p/2)+1), PointNextBase -> PointPastBase (point just past base floor(p/2)). Also renamed
  for consistency: Runner_runStep/runToNext -> Runner_settleStep/settleOnBase (target is
  PointPastBase, not the next base) and Runner_afterPickoff -> Runner_arrivedNoAward (reached a
  base with no bases awarded; nothing pickoff-specific in the code).
- STILL STATIC ONLY: the return-to-base path Runner_goDelay1 -> PointBaseAlt ($24 + 4*(floor(p/2)
  mod 4), base k for progress 2k+1) -> Runner_leaveBase (progress 2k+1 -> 2k) -> Runner_settleOnBase
  was not executed in the r3-b trace. Its "back onto base k" reading (tag-up / caught pitch /
  RunnerDecision carry set) is worded as a static reading in source.
- TENTATIVE renames from static reading: RunnerFlags -> RunnerOnStartBase ($95, set by
  MarkRunnerAtStart, cleared for runners 1-3 by OutOnFlyCatch; probably the tag-up flag);
  TryForceAtBase -> TryTagUpOutAtBase (probably doubled off after a caught fly);
  TryTagRunnerAtBase -> TryForceOutAtBase (force out, from the corrected ForceCheck contract).
  New symbol RunnerCommitted ($99-$9C): the code writes RunnerFlags+1,x with X = slot/2,
  which addresses $99-$9C, a separate per-runner array, not RunnerFlags.
- HitPower/HitAngle role swap: unchanged (no new evidence; round-1 names kept, the
  RotateByAngleBits contract is documented in source).

## Round 4 (pass-004-fallback baseline, final acceptance)
- $E5F4 (phys 0x2604, 1 byte, $60): map region 796 now types it generic Data with Manual
  provenance, ONLY by an explicit user-directed fallback for this batch (<= 10 residual bytes).
  This is not an observed or statically proven data role and not evidence-backed
  classification. L-009 stays materially ambiguous: no reference, pointer, fall-through,
  dispatch, inline-argument or consumer was found; the byte would decode as RTS; ROM padding
  elsewhere is $FF. It may be a leftover RTS, filler or data; reachability unknown. Source label
  is the neutral Data_E5F4; no semantic name is given.
- $DFC4-$DFC9: the first 6 bytes of the BaseApproachOffsets/ThrowSideX block have no
  identified reader (indexes 0-4 from $DFCA/$DFCD never reach them).
- Throw-choice tables at $E01D: RunnerSideTable's exact geometric meaning (which side of the
  base line / which running direction selects which base) is described mechanically only.
- Carried unchanged from round 3: HitPower/HitAngle role swap, return-to-base path static-only,
  song-to-event map, unreferenced fragments, CameraRecC8 without reader.
