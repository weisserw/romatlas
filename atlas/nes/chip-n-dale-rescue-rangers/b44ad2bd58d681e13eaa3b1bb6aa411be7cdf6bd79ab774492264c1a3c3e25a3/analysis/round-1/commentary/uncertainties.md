# Commentary uncertainties (round 1)

Structural classification is unchanged (pass-001-final). Items below are
interpretation limits recorded while commenting; see source comments for detail.

- Object types (bank 6): behaviour of every handler is described mechanically
  (movement, timers, spawns, hits). Which enemy/item each type is in the game
  is mostly not established; Zipper (type $60, via item boxes $59/$5A) and the
  rideable platform ($61) are tentative identifications.
- Sound engine (bank 3): routine roles, command tables and channel/envelope
  structure are named; the exact byte formats of effect scripts, music tracks
  and instruments are only partly decoded, and track-flag bits ($0730) are
  tentative.
- TileType classes: 0/1/2/3+/4/5/6/$0B/$0D/$0E+ are supported by consumers;
  classes 7-9 act as slopes in SnapOntoFloor/SlopeWalk*, while level 5 also
  treats 8/9 as conveyors (ConveyorPush). Both uses are in code; the game
  meaning per level is not verified.
- PlayerAction 7 (stunned) is entered from ObjectContact (bumping a type-1
  object); PlayerAction 8 is riding the type-$61 platform. Other triggers not
  searched.
- HUD: hearts are OAM sprites (health = visible heart sprites); HudCountersP1/P2
  ($05A0/$05D0) hold item and life digits ($80+digit); full layout not mapped.
- Base-offset tables: several consumers index a base below the real data
  (e.g. SectionScreen1-1, BoxContentAnim-$55, BossPartLaunch-$46, the
  $F9D0/$F9D5 hit adjustments, Collectible_Take's $8D25/$8D2A/$8D2F tables).
  Operands keep exact values; labels sit on the first used entry where known.
- LiftedBlock metasprite read at $CB9C+TileType lands in the operand bytes of
  a following JMP ($CBAA/$CBAB); documented, not resolved further.
- 8 bytes before each bank's vectors (B9 7F 33 04 00 00 08 89) have no known
  consumer.
