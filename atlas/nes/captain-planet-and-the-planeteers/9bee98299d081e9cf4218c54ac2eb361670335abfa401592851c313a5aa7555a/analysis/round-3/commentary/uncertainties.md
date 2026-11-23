# Commentary uncertainties (round 3, checkpoint pass-003)

Round-2 record (pass-002) is preserved unchanged in
analysis/round-2/commentary/uncertainties.md; round 1 in analysis/round-1/.

Round-1 record (pass-001, 71 Unknown ranges) is preserved unchanged in
analysis/round-1/commentary/uncertainties.md.  All of those ranges except the
three below are now classified; their round-1 notes were removed from the
annotation modules and replaced by names/comments for the classified code/data.

## Interpretation limits (carried from round 1, still valid)
- Game identities of Geo-Cruiser objects (banks 3, 6) and Captain Planet stage
  objects (banks 7, 8, 13) are inferred from code only.  Names describe mechanics
  (PodTask, ShooterTask, GrabberTask, WallCrawlerTask, PusherTask ...), not
  confirmed game characters.  Power numbering (ActivePower/CurrentPower) is
  mechanical; which number is Fire/Water/Wind/Earth/Heart is not confirmed.
- RAM reused per stage type/bank: $C0 IrqPhase/SpawnX, $057B PasswordDigits/SpawnLog,
  $F5 EventPhase, $078F SuperModeTimer (CP stages) / SpecialEventFlag (Geo banks 3/6),
  $CA/$D2 ShotX/ShotY reused as ClawX/ClawY in bank 13, $0719/$0721/$0741/$0764/$0795
  reused by bank 13 objects (bank-specific ramalias names).
- PauseState/PauseRequest "pause" role untested.

## New in round 2 (pass-002 code)
- Renamed with new evidence: $F4 TerrainHitCount -> CpHurtCount (incremented by
  enemy contact/shots as well as damaging tiles; CpPlayerLoop converts it to energy
  loss); $05D6/$05D7 are anchor map column/row for PlaceAtAnchor (old LevelSwitchTile*
  names were wrong); $0572 CpBumpFlag, $0576 CpLiftSpeed, $0578 CpHeldBySlot,
  $055D-$0560 camera column/row limits, $0709 GoalHitCount; fixed-bank trampolines
  JumpBank13_A00F -> BecomeDebris, CallBank7_A015 -> CallSpawnItemDrop;
  GameOverScreenSetup -> LevelClearScreenSetup (redirect r001-a observed the
  level-clear password screen).
- Tentative readings (mechanics clear, purpose not): Level3BossTask (map bank switch
  to $0B arena, EventPhase 2/3/4; escape/restart path observed in round 3, see below), Level4BossTask and its
  claw/shield interplay (ShieldTurn table), SpecialPatrolObject/SpecialDamWalker
  (bank 3 special types $20/$21), ArmoredHitCheck hit counts, PusherTask.
- SuperModeTimer: set to $0F by SuperCapsule tasks (power 5 contact); enemies die on
  contact while nonzero; decremented every 64 frames.  Game name of this state unknown.
- Static code with no known caller (structure only, reachability unknown): bank 6
  UnusedSq1Sound/UnusedNoiseSound/NegateTmp93, bank 8 CpWithinObjY, bank 12
  SetHitBoxB6Copy12, bank 13 SetTaskBank7_13, Level3BossTriggerFail,
  ProjectileHitsCp13_splat, bank 14 SubFromPtr0, WriteSwitchColours, SetMusicPlaying,
  VecSpare* slots, bank 7 Bank7UnusedSlot6/8.
- Tables whose first entries overlap preceding bytes (indexed by Level 3-4 or from
  index 1): LevelObjChrR2_13.. (base 3 bytes before), ObjectAnimLists13 (base $A0ED
  inside StepObjectAnimation13), BarrierPathX/ShieldPathX/PusherRiseSteps (-1).
  Rendered as Label-n operands.

## Round 3 (pass-003; leads L009-L011)
- Corrected from redirect r002-a runs (artificial patches) and static re-reading (L011):
  bank 7 $B0CD task word is $A015 of CpStageBank = CreateBossZoneObjects(13)
  (was rendered as Bank7SpawnItem); LevelInitTasks -> BossZoneTasks (goal/boss
  objects created on boss-zone entry, not at level start); AfterPasswordEnding ->
  StageEndDispatch, StartEndingSequence -> StartGaiaMessage, EndingTask ->
  GaiaMessageTask, EndingPictureTask -> GameOverMenuTask, Ending* Gaia screen helpers
  renamed Gaia*; RingLevel -> Lives; EndingFlag -> StageEndFlag (0/1/2/$FF).
- Deviation from the L011 recipe: StartEndingSequence was proposed as
  StartGameOverSequence, but StageClearTask (final CP stage cleared) and the ending
  password also reach it, so it is named StartGaiaMessage (shared screen).
- Level3BossTask: escape = boss off the right edge (X hi set, lo >= $20) costs a
  life (observed, artificial); defeat path (A-move hit, StageEndFlag+1, score 50) is
  static reading only.  Why the life loss is skipped when CpHurtCount >= $10 (only
  $63 = death is known) is not established.
- Message texts quoted in GaiaMessageTask come from runtime screens, not decoded text.
- StageSummarySetup/StageSummarySpriteTask use Level-1 while StageEndFlag is set;
  which caller relies on that case is not established.
- EmptySpriteDef8 (L010): its first byte doubles as the empty levels 3-4
  MapSpawnTable list.  In bank 13 no MapSpawnIndex13 entry points at the byte after
  the level-4 list (DebrisSpriteDef); the round-2 "empty list" note there was wrong.
- New mechanical RAM names: AccelCounterX/Y, AccelDelay (value meaning tentative),
  CpLastStepX/Y, TextReadOffsetLo/Hi, TextEndPause; PendingPalette11/15 and CpPalette
  rendered as 3-byte arrays.  109 Unk_* RAM symbols keep "role uncertain" notes.

## Remaining Unknown bytes (phase 1, 4 bytes; lead L009b rejected)
- phys $0C185 (bank 6 $A175) and $0C19B (bank 6 $A18B): lone $60 after an RTS,
  returned to Unknown in pass-003 for consistency (were Static Code in pass-002).
- phys $1FB79 (bank 15 $FB69): $60 after PpuBufferBegin's RTS.
- phys $1FDD0 (bank 15 $FDC0): $60 after FlushPpuBuffer's RTS.
  All four: dead duplicate RTS or padding; no reference, never fetched.  Reopen only
  with a reference/fetch or a source-pattern model that separates the two readings.
- Bank 13 $BA5D-$BA5F is now Static Code (L009a, duplicate `jmp DebrisSpawner`,
  PusherTask_deadJump); reachability not claimed.
