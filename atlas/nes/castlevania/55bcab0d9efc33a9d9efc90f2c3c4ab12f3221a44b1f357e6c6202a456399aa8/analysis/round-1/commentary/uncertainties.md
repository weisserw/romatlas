# Commentary uncertainties

Round 1. Investigated limitations and tentative interpretations (details live next
to the code in source/game.asm).

- Object identities (fishman, swooper, jumper, shooter, crawler, crushers, flyers)
  are inferred from motion/spawn code only; no gameplay confirmation this round.
- Sub-weapon kind mapping (dagger/axe/holy water/cross/stopwatch) inferred from
  heart cost and motion; item kinds in CheckObjectCollisions are inferred from
  their effects (hearts, health, whip level, multi-shot, invisibility, orb).
- GameStates 9/10/11/14 scene meanings (entrance walk, climb-in, scroll scene) are
  inferred from positions/timers.
- The bank mapped during fixed-bank code reached from object handlers is bank 5
  (handlers re-select bank 5 after PlaySound); ED25/ED4B targets are therefore
  bank-5 addresses, but phase 1 decoded bank-6 code at $82A2/$84FF/$8C9A partly
  from those jumps (question for redirect).
- CallStageSplashRoutine calls $88B7 after mapping bank 5 then restoring; its
  `jsr $88B7` runs with the saved bank 6 (mid-routine entry inside Spawn14_Chain),
  while bank 5 has a splash routine at $88B7 too - one of the two is probably a
  mis-contexted call; unresolved.
- NmiLagFrame's `jsr $838A` runs without a bank switch; intended target is the
  bank-0 sound update, effective target depends on the interrupted bank.
- Collision-map codes in GetTerrainAt and attribute shadow layout at $05A0 are
  only partly decoded.

## Round 1 resume additions

- Object fields $04F8/$0514/$0530 are genuinely multi-use (see the RAM definitions);
  renamed ObjAux04F8/ObjAux0514/ObjAux0530 with every established role listed, and
  per-site comments give the role in context. Moving platforms and the intro scene use
  them through the "X + $38" alternate-field trick (OscAdvancePosAlt etc.).
- SpawnerUnused0178 ($0178) is cleared with the spawner fields but never read.
- DelayedInvisTimer ($7D) is only ever cleared; its SetPlayerPose path looks unused.
- PauseBlocked ($23) is read by CheckPause but no writer was found.
- ObjT17_SubWeapon re-dispatches weapon kinds 2 and 4 to itself (would loop forever);
  presumably never created.
- ReadJoypads tests `ModeFlags & z:$10` (zero page, not immediate): the input lock
  depends on the scratch byte AddScore leaves there (player index, normally 0).
- Sub-weapon/item identities (dagger, cross, holy water, axe, stopwatch, rosary flash,
  potion, multi-shot) are inferred from costs and behaviour, not verified in play.
- Bank-6 data $B800-$B822 starts like a $3F00 palette record but has no known reader.
- TextScreenPtrs entry 4 ($918D, used by State0F_Ending with X = 8) points into the
  unknown bytes 102577-103259, so that range holds at least one text-screen stream set
  (redirect question 7).
