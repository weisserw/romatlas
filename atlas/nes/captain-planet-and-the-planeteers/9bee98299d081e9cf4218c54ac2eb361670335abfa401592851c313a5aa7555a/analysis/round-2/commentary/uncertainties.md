# Commentary uncertainties (round 2, checkpoint pass-002)

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
  to $0B arena, EventPhase 2/3/4, ring-level loss on failure), Level4BossTask and its
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
- Trailing 11 00 00 01 after both MapSpawnTable lists: same bytes as the bank 13
  DebrisSpriteDef header; no reader found in bank 8.

## Remaining Unknown ranges (phase 1, 5 bytes; lead L009)
- phys $1BA6D-$1BA6F (bank 13 $BA5D-$BA5F): 4C C2 B4 = 'jmp DebrisSpawner' right after
  'jmp DebrisSpawner / rts' in PusherTask; unreferenced, never fetched.  Probably a
  dead duplicate jump; code vs data unproven.
- phys $1FB79 (bank 15 $FB69): $60 after PpuBufferBegin's RTS.
- phys $1FDD0 (bank 15 $FDC0): $60 after FlushPpuBuffer's RTS.
  Both: dead duplicate RTS or padding; no reference, never fetched.
