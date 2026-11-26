# Commentary uncertainties (round 2, checkpoint pass-002)

Structural classification is pass-002 (262159/262160 bytes classified; only
fixed $F62C unknown). Items are interpretation limits recorded while
commenting; source comments give the detail. Round-1 items carried forward
are marked (r1).

## Carried from round 1
- (r1) Object types (bank 6): handlers are described mechanically; which enemy
  or item each type is in the game is mostly not established (Zipper $60 and
  the rideable platform $61 remain tentative).
- (r1) Sound engine (bank 3): effect-script, music-track and instrument byte
  formats are only partly decoded; track-flag bits ($0730) are tentative.
  Round 2: SoundCommandTable starts at $816C (8 entries, entry 0 = SoundStopAll),
  not $816E as written in round 1. The commands formerly called SoundCmd4/5/6
  are entries 5/6/7 (SoundCmd5RampFlagged / SoundCmd6Ramp / SoundCmd7Offset);
  their effect on $CC/$CD/$D8 is mechanical, the audible meaning is not traced.
- (r1) TileType classes 7-9: slopes in SnapOntoFloor/SlopeWalk*, conveyors in
  level 5 (ConveyorPush); game meaning per level not verified.
- (r1) PlayerAction 7/8 triggers beyond ObjectContact / the type-$61 platform
  were not searched.
- (r1) HUD: HudCountersP1/P2 layout is now partly mapped (name tiles, 3-digit
  flower counter at +$0A..+$0E, second 2-digit counter at +$10/+$12, lives at
  +$14/+$16); the remaining bytes and the "$03" attribute-like bytes are not.
- (r1) 8 bytes before each bank's vectors (B9 7F 33 04 00 00 08 89): no consumer.

## Resolved or revised in round 2
- The $CB9C+TileType read (PlayerWalk_LiftBlock) lands in LiftTileAnim
  ($CBAA/$CBAB = $23/$22), classified as data in pass 002; the round-1 note
  about JMP operand bytes was wrong and has been removed.
- Base-offset reads now name their tables: DroppedBlockYVel*-$22,
  BossPartMoveDir-$46, MetaspriteHitAdjust-$22/-$1D, PickupHandlerLo/Hi/
  PickupSound-$2D, CarryHeightByAnim/HeadOffsetByAnim/PlaceOffsetByAnim-$22,
  LiftTileAnim-$0E. Still raw (role or index domain unclear): EnvelopeSustain
  $8913/$891A, DrawObject $9B82-$9DAE, SetVelocityFromDirection $B565-$B568,
  SpawnBossPart52 $B305, the cross-bank level block tables $BAE0-$BEE0 and
  $D690/$D691 (no reader).
- "Level-clear tally" ($A410) is the bonus-stage intro (BonusStageIntro):
  texts ZONE x CLEAR! / GET READY! / GO!! and eight random pick-ups on level 13.

## New in round 2
- Moving-item metasprites $23/$24/$25: handlers are explained mechanically
  (hop-back block that stacks on other $23 items; rebounding item; item that
  follows its carrier's Y). The game objects they represent are not identified.
- Second two-digit HUD counter (pick-up metasprite $2E, PickupAddSecondCounter):
  mechanics clear (life every 10 in 2P, every 20 in 1P); game item not named.
- HeartOamByPlayer (OAM $04/$18, metasprite-$64 pick-up) vs. the three-heart row
  (HeartRowOam $08/$1C): the extra sprite's meaning (fourth heart?) is tentative.
- Pause screen: the PlayerConfig byte compared with 1 (slot-0 figure becomes
  metasprite $94) is assumed to be the character choice; not traced.
- GIVE UP? sub-screen: reading of the sprite text uses the letter tiles $8A='A'
  seen in the bank-2 text lists; behaviour (forcing PlayerAction 10) is exact.
- Structure-only code (no known caller, confidence 0.75 in phase 1):
  PpuCtrlMaskNmiOff/SetNmiOn, FillRamPages, ClearSpritesAndFadeIn,
  InlinePointerDispatch, PlaceCarriedObjectAlt (uses an object-field layout that
  does not match the current tables), StandOnPartnerOrphanTest, TaskFree,
  TaskExitCurrent, PlatformAdvanceRightEdge, DemoInputRecorder. DemoInputRecorder
  writes $6000-$67FF, which needs cartridge RAM; whether this board has PRG RAM
  was not checked.
- $F62C (only unknown byte): an RTS with no reference; code versus filler open.
