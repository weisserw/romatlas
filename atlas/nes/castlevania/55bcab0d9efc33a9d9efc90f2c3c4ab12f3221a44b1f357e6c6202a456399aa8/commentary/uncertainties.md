# Commentary uncertainties

Round 3 (input checkpoint pass-003, final acceptance). Round-1 and round-2 copies are
preserved in analysis/round-1/commentary/ and analysis/round-2/commentary/; still-valid
items are carried here and revised where later evidence changed them. Details live next to the code in source/game.asm.

## Carried from round 1 (still open)

- Regular enemy identities (fishman, swooper, jumper, shooter, crawler, crushers,
  flyers, walker) are inferred from motion and spawn code; not confirmed in play.
- Sub-weapon kind mapping (dagger/axe/holy water/cross/stopwatch) and item kinds in
  CheckObjectCollisions are inferred from cost and effect.
- GameStates 9/10/11/14 scene meanings are inferred from positions/timers.
- NmiLagFrame's `jsr $838A` runs without a bank switch; the intended target is the bank-0
  sound update, the effective target depends on the interrupted bank (redirect r1: latent
  game issue, no classification impact).
- Collision-map codes in GetTerrainAt and the attribute shadow at $05A0 are only partly decoded.
- Object fields $04F8/$0514/$0530 are multi-use (ObjAux*); per-site comments give the role.
- SpawnerUnused0178, DelayedInvisTimer ($7D) and PauseBlocked ($23) have no reader or
  no writer; ObjT17_SubWeapon re-dispatches kinds 2/4 to itself (presumably never created).
- Bank-6 data $B800-$B822 looks like a palette record but has no known reader.

## Resolved or revised in round 2

- Bank context of fixed-bank calls from object code: StageSpecialHandler, ObjT10_StageObject
  and the new ThrowBoneDispatch ($ED38) jump to bank-5 code (handlers run with bank 5).
  The bank-6 code at the same addresses is independently supported (redirect r1).
- CallStageSplashRoutine/CallStageAttackRoutine map bank 5 before calling $88B7/$8910, so
  they reach SpawnSplashDropsAlt/SpawnFireballAlt in bank 5 (round 1 resolved the
  `$88B7` call to bank 6 because its bank-select bracket analysis stopped at a jmp).
- Spawn14_Chain: slot $0A (type $16) is the fixed base, slot 4 (type $14) the shooting head
  (round 1 called type $16 the head).
- TextScreenPtrs entry 4 ($918D) is TextScreen4Streams; the DPCM sample is DmcSample ($FF40).
- Dracula's first-form state $8D is set by Hit_Boss when the head hit box (type 4, slot 8)
  runs out of health; first-form state $0C has no known setter.

## New in round 2

- Boss identities (Phantom Bat, Medusa, mummies, Frankenstein/Igor, Death, Dracula) follow
  from the boss spawner order, boss-room stages and behaviour; only the stage-3 bat and
  Dracula were visited (SC-BOSS3, SC-BOSS18). Medusa's EraseBossStatueTiles target
  (nametable $220C-$226F) is not verified in play.
- Regular bank-5 enemy identities are tentative: ObjT07_DivingFlyer (vampire bat?),
  ObjT11_Pouncer (panther?), ObjT10_BoneThrower (skeleton?), ObjT0A_BoomerangThrower
  (armoured knight?); ObjT08_Patroller, ObjT1E_EvasiveFlyer and the type $14-$16 chain
  are unidentified.
- Spawner placement per stage was read with fixed 40-column scans of the overlapping
  column lists (scratch/c2/stages.py), so stage numbers attached to types are approximate.
- Death_SpawnSickles writes ObjTimer = $28 to whatever slot X holds (boss, then previous
  blade); the last blade keeps the timer InitObject left, so its blink-in length differs.
- Mummy_State5 (state 5) and Dracula first-form state $0C have no setter in the code read.
- Unreferenced fragments classified as code from alignment only (round-2 L-003/L-013,
  confidence 0.6): $85BF, $C04F, $C0DB, $C0FD, $C312, $C31E, $C8C2, $CC3D, $CF61, $DA61,
  $E9AC/$E9B2, bank-6 $81F3/$A2E2/$AB07/$B8F2/$B8F5; bank-6 $90A1/$910A and $FF06 have no
  caller either. Their comments describe mechanics only; they are probably leftovers.
- (Superseded in round 3, see below.) The 6 bytes left unknown in round 2.

## Round 3 (final acceptance)

- The 6 formerly unknown bytes are now Static code with confidence 0.5 (L-015):
  UnrefReturnStub_C916, UnrefReturnStub_CA5B, the local @UnrefReturnStub ($C405, inside
  State08_StageExit), bank-6 UnrefReturnStub_A1E4 and UnrefReturnCarrySet ($9B8B, sec; rts).
  No caller, pointer or table reaches any of them; reachability stays unknown (not observed,
  not claimed dead). Source comments state the low confidence.
- UnusedFill9005 stores to UnusedFillRomTarget ($9005-$900B, ROM inside the credit data) and
  reads its entry number from UnusedFillEntry ($07F0), which has no writer; purpose unknown.
- Zero-page scratch $00-$17 stays numeric by convention (documented in the RAM symbol header);
  $4C/$4D/$4F are now named Temp4C/Temp4D/Temp4F as generic scratch, like Temp4B.
