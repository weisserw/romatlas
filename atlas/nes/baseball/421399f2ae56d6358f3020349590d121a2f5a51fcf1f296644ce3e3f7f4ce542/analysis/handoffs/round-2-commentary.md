# Round 2 -- commentary handoff (2026-11-18)

Status: COMPLETE for current classified source (editorial audit reserved for final acceptance).

## Scope
- The map changed since round 1 (pass-001 -> pass-002), so the entire round-1 commentary
  was preserved unchanged in analysis/round-1/commentary/ (tools: analysis/round-1/
  tools-commentary-r1/) and a fresh baseline was prepared with `commentary.py prepare`
  from pass-002 (472 scaffold entries).
- Source commentary/source/game.asm is rendered from frozen before/ + notes by
  analysis/tools-commentary-r2/apply.py (notes in analysis/tools-commentary-r2/notes/;
  round-1 notes carried by address, records of changed ranges replaced; new files
  40-r2-new, 41-ai, 42-cordic, 43-flow; 15-sound rewritten).
- review.json: 472/472 entries reviewed with source locations; 438 carried from round 1
  by identical physical range and kind (review_round 1), 34 reviewed this round
  (0x2AEE split into 3, 30 formerly unknown ranges, residual unknown 0x2604); 619 unit
  records; every Code entry has units. editorial_audit pending.
- Reading copy: commentary/reading-after.asm (refreshed).

## Readability assessment
- 0 address-only code labels; 0 raw RAM/ROM address operands remaining.
- All 30 newly classified ranges have descriptive labels and block comments
  (see commentary/progress.md for the list) with "no known caller (reachability unknown)".
- Backlog done: sound driver (request-bit map, per-effect blocks, stream format, labels
  corrected), CORDIC (conventions + line comments; CordicShrink = hyperbolic CORDIC
  sqrt(X^2-Y^2)), runner/fielder decision branches (RunnerTask, FielderTask,
  FielderHasBall, ThrowToBase, RunnerDecision, InterceptBall), and line comments for
  PitchTask, BatterTask, PitchResultTask, PickoffTask, MessageTask, BatterBoxTask,
  TryCatchBall, ClassifyLanding, AddRunsToScore, PlaceRunnersOnBases.
- Corrections: $82 PitchCaught (was PickoffActive), $66 HalfInningRuns (was GameOverFlag),
  SetPpuAddrInline zero-high-byte claim removed (branch always taken), Sq2/triangle effect
  meanings (OUT/SAFE calls, footstep, thud/catch/STRIKE/FOUL tones, crowd noise).

## Verification
- `commentary.py verify`: ok, byte_identical true; game.asm sha256
  d0c097d923f722e3b10a0ec2f531729149579e9b1aa27a0c9d6cf1fe0ebb4c48
  (commentary/verification.json). input/game.nes sha256 unchanged (421399f2...).

## L-010 outcome (applied; leads.json status consumed)
ClearBallPosition $C959; $CB CameraFrameOfs (0/6, read by AnimateCycleAndSleep);
FielderReadyFrames 4 bytes (operand FielderReadyFrames-1,y); reachability wording;
PointFielderReturn Y = CurrentTask*2-12 (meaning tentative); PrintCalled args as Data.

## Unresolved meanings (commentary/uncertainties.md)
- Camera record naming ("infield" record has DrawFirst 0 while INFIELD_DRAW addresses the
  other record's draw slots) -- one name likely inverted.
- HitPower vs HitAngle tables appear swapped in role (angle bits vs speed).
- ForceCheck N-flag meaning, runner progress arithmetic at $D981, song-to-event map.
- Purposes of unreferenced fragments (AnimateCycleAndSleep, RestartFieldersReturn,
  FielderGoRole5, VectorAngleCode, SubtractY32nd).

## Structural questions for redirect
- SQ-R2-1 $E5F4 (0x2604): residual 1-byte Unknown; no new evidence (L-009 stands).
- SQ-R2-2 Static Code region 0x1FA2-0x1FD3 ($DF92-$DFC3: sum-of-squares tail of
  InterceptBall + SquareByte, whose only callers are inside it) is reached statically only
  by fall-through of BEQ $DF90, whose Z flag is always set on both incoming paths (LDA #0 /
  LDX #0). It has no effective entry; Code classification is still coherent, but its
  evidence should read "no known entry; reachability unknown". Bounded check: coverage of
  $DF92 in existing traces (expected never executed).
- SQ-R2-3 (semantic, bounded runtime check suggested): which SetCamera record is shown
  while pitching vs fielding, to settle the INFIELD_DRAW / "infield camera" naming.

## Remaining gaps
- None for current-source readability. Whole-ROM classification 99.996% (1 unknown byte,
  $E5F4). Final editorial audit not run (reserved).
