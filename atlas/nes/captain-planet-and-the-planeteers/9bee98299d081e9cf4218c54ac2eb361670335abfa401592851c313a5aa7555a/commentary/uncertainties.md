# Commentary uncertainties (round 4 final, checkpoint pass-004)

Round-3 record (pass-003) is preserved unchanged in
analysis/round-3/commentary/uncertainties.md; round 2 in analysis/round-2/, round 1 in
analysis/round-1/.

Round-1 record (pass-001, 71 Unknown ranges) is preserved unchanged in
analysis/round-1/commentary/uncertainties.md.  All of those ranges are now
classified (the last four only by the pass-004 manual fallback, see below); their round-1 notes were removed from the
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

## Former Unknown bytes: pass-004 user-directed manual fallback (4 bytes)
History (kept): through pass-003 these four bytes were the only phase-1 Unknown
ranges (lead L009b rejected; redirect r003-a rejection review, see
analysis/handoffs/round-003-redirect.md).  Each is a lone $60 directly after a
routine's RTS: no reference (incl. RTS-trick/computed tables), never fetched in
any traced route, not alignment, not the $00/$FF fill.  Dead duplicate RTS and
padding fit equally well; code vs data was unresolved.
- phys $0C185 (bank 6 $A175): after PlaceShot6's RTS, before UnusedSq1Sound
  (pass-002 Static Code, pass-003 back to Unknown for consistency).
- phys $0C19B (bank 6 $A18B): between UnusedSq1Sound and UnusedNoiseSound (same history).
- phys $1FB79 (bank 15 $FB69): after PpuBufferBegin's RTS.
- phys $1FDD0 (bank 15 $FDC0): after FlushPpuBuffer's RTS.
Pass-004: each is now kind Data, provenance Manual, set only by an explicit
user-directed batch fallback.  This is generic Data with unknown meaning, not an
observed or statically proven data role; the code-vs-data question above is still
open.  Source notes and review.json summaries say so; no label or semantics were
assigned.  Reopen only with a reference/fetch or a model that separates the readings.
- Bank 13 $BA5D-$BA5F is Static Code (L009a, duplicate `jmp DebrisSpawner`,
  PusherTask_deadJump); reachability not claimed.

## Round 4 (final editorial audit) - interpretation notes
- $057B is PasswordDigits only on the password screen; stage code uses the same RAM
  as the spawn log: alias SpawnLog (banks 3, 12; Geo 8 map pointers) and SpawnLogLo
  (banks 7, 8, 13, and ObjKill in bank 14; low bytes paired with SpawnLogHi).
- $E6/$E7/$E8 named MapScrollLeft/Down/Up (were Unk_*): ObjApplyMapScroll applies
  them like MapScrollRight; CpScrollUpdate copies ScrollStepLeft/Down/Up into them.
- $0795 in bank 15 is BriefingSpeakerIndex (speaker record counter); its uses in
  other banks stay Unk_SpecialPatrolObjectEtc_0795 (role uncertain).
- SetPriorityIf0745/_6 and SetPriorityIf0746 keep address-based names because
  $0745/$0746 roles are still unknown (Unk_* RAM symbols with notes).
