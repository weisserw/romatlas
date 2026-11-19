# Commentary progress

Prepared verified baseline (2026-11-18, round 2) from checkpoint pass-002
(maps/current.json == maps/pass-002.json, sha256 9728112890a1...). The complete
round-1 commentary (prepared from pass-001) is preserved unchanged in
analysis/round-1/commentary/ (and its tools in analysis/round-1/tools-commentary-r1/).

## Round 2 commentary pass (2026-11-18) -- complete for current classified source

Method: source/game.asm is rendered from the frozen pass-002 before/game.asm plus
annotation notes by analysis/tools-commentary-r2/apply.py. The round-1 notes were
copied and carried by CPU address (= physical offset - $BFF0 in this NROM view);
only records touching changed regions were removed/replaced. Inline-argument sites
were re-derived from before/ (87 sites: analysis/tools-commentary-r2/r2-inline-sites.json;
new: $C959->$C963, $DCA9->$E29B, $EAF5->$EA05). review.json is rebuilt from the
source by tools-commentary-r2/review.py on the new 472-entry scaffold.

Review carry-over: 438 scaffold entries carried from round 1 by identical physical
range and kind (review_round 1); 34 new/changed entries (the 0x2AEE split into 3, the 30
formerly unknown ranges now Code/Data, and the residual unknown 0x2604) reviewed
this round (review_round 2). 619 unit records (routines/structures plus split
records for routines spanning scaffold entries); every Code entry now has units.

Newly classified ranges named and explained (no known caller = reachability unknown):
ClearBallPosition $C959; AnimateCycleAndSleep $D810 + AnimCycleFirst/Last/Sleep;
ObjectAtBaseOffset $DC06 + BaseCheckOffsets $DC29; FielderReturnTask $DC97 /
RestartFieldersReturn $DCA2 / PointFielderReturn $E4B8; FielderReadyFrames (4 bytes)
and FielderGoRole1..5 stubs $DFEF-$E01A; BallOffsetFromObject $E2A5;
SetFirstCameraHideSprY $E741; HideSpritesAtOamFill $E7BC; ScaleAlongVectorA $E822;
VectorAngleCode $E8B7; CopyInlineToZeroPage $E9C2; SetPpuIncrementFromCarry $E9D2;
SetPpuAddrRelative $E9E0; WritePpuDataByte $E9FC; WritePpuInlineRelative $EA02;
PrintCalled inline Data $EAF8; SleepUntilFrameAligned $EB3C / ...Jmp $EBB4;
SubtractY32nd $FAAE.

Readability backlog completed this round:
- Sound driver rewritten commentary: request-bit map (SoundSfxB, SoundSfxA, SoundMusic),
  per-effect blocks, music stream format; labels corrected (Sq2 bits 0/1 are the
  OUT/SAFE umpire calls, bit 2 the runner footstep; the "noise" section is mostly
  triangle tones -> SoundSfxTriNoise/TriFx_*; Sq1 pitch/bounce/whistle).
- CORDIC: shared conventions block, line comments; CordicShrink identified as
  hyperbolic CORDIC vectoring (shift sequence 1,2,3,4,4,5,6,7) giving sqrt(X^2-Y^2),
  used by InterceptBall to keep the forward speed after matching the ball's sideways speed.
- Runner/fielder decisions: block comments on the RunnerTask, FielderTask,
  FielderHasBall, ThrowToBase branches; line comments in RunnerDecision,
  FieldingTask, InterceptBall.
- Line comments for PitchTask, BatterTask, PitchResultTask, PickoffTask,
  MessageTask (MSG_* operands), BatterBoxTask, TryCatchBall, ClassifyLanding,
  AddRunsToScore, PlaceRunnersOnBases, HomeRunBannerTask.

Corrections to round-1 names/comments:
- $82 PickoffActive -> PitchCaught (only set by CatcherTask on catching the pitch).
- $66 GameOverFlag -> HalfInningRuns (runs this half-inning, mod 10 line-score digit).
- $CB named CameraFrameOfs (L-010); $25 PpuDataByte.
- SetPpuAddrInline: the "zero high byte = relative" claim was wrong (the branch is
  always taken); the relative code is the separate entry SetPpuAddrRelative.
- DrawIndexForSlot comment no longer claims ">= $16 = not in this camera's list".
- HomeRunBannerTask requests crowd noise (bit 7), not a "sound tick".
- C959 calls SetBallPositionInline (not velocity) -> ClearBallPosition (L-010).

Verification: commentary/verification.json -- ok, byte_identical true
(game.asm sha256 d0c097d923f722e3b10a0ec2f531729149579e9b1aa27a0c9d6cf1fe0ebb4c48).
Reading copy refreshed: commentary/reading-after.asm. input/game.nes unchanged.
Editorial audit: pending (reserved for final acceptance). Deferred ranges: none.

## Round 3 (2026-11-18) -- pass-003 baseline

Map change: pass-003 differs from the round-2 frozen baseline (pass-002) only in region 648
evidence text (L-011); no boundary/kind change. Following the phase rule, the entire
round-2 commentary was preserved in analysis/round-2/commentary/ (tools in
analysis/round-2/tools-commentary-r2/), then `commentary.py prepare` built a fresh
pass-003 baseline (472 scaffold entries; before/game.asm identical to round 2).
Source is re-rendered by analysis/tools-commentary-r3/apply.py (copy of r2 tools + notes);
before L-012 edits the rendering was byte-for-byte identical to the round-2 game.asm.
review.json: 472/472 reviewed, 619 units; credit carried by identical id/range/kind
(425 round 1, 29 round 2); 18 entries re-reviewed this round (review_round 3).

Changes this round:
- L-012 camera naming (settled by round-2 runtime check): first CameraTables record =
  full-field view with scoreboard ($CB=0, ScreenSel $00, nametable $2000, draw $01-$14);
  second = infield close-up used while pitching ($CB=6, ScreenSel $08, nametable $2800,
  draw $16-$21 = INFIELD_DRAW). Updated SetCamera, CameraTables (record layout and values),
  CameraParam, ScreenSel, DrawFirst, DrawLast, CameraFrameOfs, INFIELD_DRAW,
  FrameTask_camera (+inline at the SetCamera call), RenderTask scoreboard skip (no longer
  tentative), AnimateCycleAndSleep and AnimCycleFirst/Last/Sleep, RunFrameFirst,
  RunnerStandPose/RunnerStandFrames, DrawIndexForSlot (carry set = infield close-up active).
- Fixed two comments that had the views inverted: FielderReadyPose/FielderReadyFrames
  (table frames go to the infield close-up slots, $5A to the full-field slot) and the
  backup fielder idle frames ($67 full-field, $43 infield).
- Renamed SetFirstCameraHideSprY -> SetFullFieldViewHideSprY; new symbol CameraRecC8 ($C8,
  no reader).
- Region 648 ($DF92-$DFC3): new label SumOfSquaresXY with a block comment ("no effective
  entry found: BEQ at $DF90 always taken"; reachability unknown, not claimed dead).
  SquareByte comment corrected: returns A:X = 4*|A|^2 (7-step loop), not X:A = A^2.

Verification: commentary.py verify ok, byte_identical true (game.asm sha256
d2739ac33d042ff604050f747a46bd56c559e935749d35e917f7b7680518164a). Reading copy refreshed.
input/game.nes unchanged. Editorial audit: pending (final acceptance). Deferred ranges: none.

## Round 4 (2026-11-18) -- pass-004-fallback baseline, final editorial acceptance

Baseline: maps/current.json == maps/pass-004-fallback.json == analysis/state.json (sha256
fc5d69ba...). Only change from pass-003: region 796, the former Unknown byte 0x2604 ($E5F4),
typed generic Data with Manual provenance by an explicit user-directed fallback (not
evidence-backed). The complete round-3 commentary was preserved unchanged in
analysis/round-3/commentary/ (tools: analysis/round-3/tools-commentary-r3/), then this
baseline was prepared with `commentary.py prepare`; before/game.asm differs from round 3 only
at the $E5F4 region header.

Method: analysis/tools-commentary-r4/ (copy of r3 tools; apply.py renders source from the
frozen before/ + notes, review.py rebuilds review.json and carries review credit by identical
physical offset/length/kind from the preserved round-3 review.json). Unedited notes
reproduced the round-3 source except the $E5F4 region, so all carried edits re-render and
re-verify.

Changes:
- $E5F4: "Unknown (unclassified)" note replaced by an honest generic-Data note (user-directed
  fallback, Manual provenance, no evidence, L-009 unresolved, would decode as RTS, reachability
  unknown); label Unknown_E5F4 -> neutral Data_E5F4. Header note updated (no Unknown ranges remain).
- $DFC4: BaseApproachOffsets ($DFCA) and ThrowSideX ($DFCD) labels; comment now describes the
  overlapping stride-1 windows exactly (old text said "Y/X pairs per base"); reader operands
  Data_DFC4+6/+7/+9 -> BaseApproachOffsets/BaseApproachOffsets+1/ThrowSideX.
- $E01D throw-choice tables (ThrowDirBase R/L/D/U -> 1st/3rd/home/2nd, BaseLineSplitX,
  RunnerSideOffset, RunnerSideTable) and HomeRunFlashColors ($CE1A): format comments at the tables.
- OutAtBase ($E39D): routine comment added.
- review.json: data tables no longer recorded as split continuations of preceding code routines.

Result: 472/472 scaffold entries reviewed (7 re-reviewed in round 4, incl. new region-796),
620 units; editorial_audit reviewed. `commentary.py audit`: ok, byte_identical, reviewed_regions
472, reviewed_units 620; game.asm sha256 62a7a702...; reading copy refreshed.
Open meanings: commentary/uncertainties.md (Round 4 section and carried items).
