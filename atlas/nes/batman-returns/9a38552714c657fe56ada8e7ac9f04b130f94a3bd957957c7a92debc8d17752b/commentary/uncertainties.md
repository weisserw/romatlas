# Commentary uncertainties

- Bank pair roles: inferred from callers (MapBanksN_N before calls). $B7 is now BossClass
  (0 ordinary enemies -> pair 6/7, 6 = stage 5-2 boss -> pair 4/5, other classes -> 10/11).
  Part 2 identified the boss handler sets by stage with bounded runs (StageIndex write on the
  pre_stage snapshot, ~500 untraced + ~60 traced frames per stage, artificial state):
  1-2 -> BossW1 (ObjType $0B), 2-3 -> BossW2 ($19), 4-3 -> BossW4 ($28), 5-2 -> BossW5
  (bank 5, ObjType $2D), 6-2 part 2 -> BossW6 ($34). Stage 3-3 (classes 3/4) was not reached;
  its handlers are not identified (likely in the unclassified bank-11 gaps $AEE5-$B536 /
  $B642-$B992). [Superseded since pass-002: those gaps are classified; BossW3_Handler at bank 11 $AEF5
  (ObjType $1F, BossClass 4) is the stage 3-3 boss, see its source comment.] The characters are not named from local evidence. EnemyB/EnemyC (pair 6/7)
  remain arbitrary behaviour-set letters: the enemy groups using them are not established.
- Vehicle routines: StageModeFlags bit7 is set only in the headers of stages 4-1 and 6-1
  (bit6 also for 6-1). Runs confirmed: 4-1 executes the bank-6 vehicle objects
  (VehicleObjectsUpdate), 6-1 the bank-13 ones (VehicleObjectsUpdate2). Which vehicle the
  player drives is not confirmed locally (the intro shows the Batmobile).
- Anti-tamper: Mode1_KonamiLogoScreen sums 41 bytes at $8159 (pair mapped at that time,
  probably 4/5) and sets DamageDoubler on mismatch; SpawnChecksumCheck sums 19 bytes at $999A
  and sets GameOptionFlags bit4. Part 2: PasswordDecode rejects any password whose decoded
  GameOptionFlags has bit4 set (one consumer of bit4); other consumers not traced.
- Renamed after Part-2 review (earlier names were wrong): PlayerStartDeath -> PlayerSetStanding,
  PlayerSetPose -> PlayerClearMoveParams, PlayerStartAction -> PlayerSetJumpFrame,
  PlayerAirborneUpdate -> PlayerMoveGround, PlayerAttackCollide -> Weapon2HitEnemy (+ per-type
  EnemyContact_* handlers), BossTakeHit/EnemyAttackHitsPlayer/EnemyGrabUpdate -> BossW5_* (stage
  5-2 only), MenuInitNewGame -> PasswordEncodeShow, CutsceneUpdate -> IntroWaitTimer,
  CutsceneObj_Mover -> CutsceneSpawnParticle, VehObj_Enemy/ScriptedMover/Boss -> VehObjActivate/
  VehObjSetAnimCtl/VehObjRemoveOffLeft. BgClassMap moved to $0590-$064F (two pages).
- Sound ids: SFX_xx names without a role word are mechanical; ids reached only through tables
  (HitReactSound, ComboHitSound, StageMusicTable) are documented at the tables.
- GameOptionFlags bit5 enables a P2 stage select (StageSelectByP2); how the bit is set
  (code/password) is not traced.
- Sound driver: command semantics named from handler behaviour; channel 4/5 roles tentative.
- SelectedWeapon ($015E) meaning (sub-weapon) is tentative; it also selects HUD strings.
- (Resolved in round 2: the hit-box tables at $FAF2-$FBCB and ObjHandlerTable $DE19 are classified
  data in pass-002; `jsr $A303` is VehStopX; the bank-7 jumps into bank 6 now land on classified code.)


## Round 2 (pass-002) additions and revisions

Renamed after round-2 evidence (old name -> new name, reason):
- Lives -> WeaponAmmo ($0141), LoseLife/AddLives/HudUpdateLives -> UseWeaponAmmo/AddWeaponAmmo/
  HudUpdateAmmo: the weapon-1 projectile spawn (ObjT01) spends one unit; death leads straight to the
  game-over screen (no life counter found). Tentative but better supported than "lives".
- ScoreSecondary -> DamageTotal ($0104): +damage*8 per hit, +$0400 per continue; high byte selects
  the ending branch (EndingChooseBranch thresholds $15/$27/$3A). Which branch is the "best" ending is
  not established.
- ResetWord108 -> HighScore ($0108-$010A): compared with Score at game over and at the ending trigger.
- VehicleWaveObjDispatch -> ObjBoxTestByType ($A007, bank 11) and VehWaveObjHandlers -> ObjTypeHitSize:
  the routine is a box test with per-type half sizes, not a dispatcher.
- VehObjExplode -> VehObjOffscreen ($A315, bank 7): carry = outside the visible window.
- ObjClearVelocity -> CutsceneAddVelX ($9936, bank 10), CutsceneObjStop -> CutsceneAccelSlot1: they add
  Tmp to the speeds.
- VehicleWaveCall/VehicleWaveHandler/VehicleBossDispatch -> VehPlayerContact/VehShotContact/
  VehPickupContact (vehicle-stage collision reaction dispatchers by contact class).
- Mode6_Dispatch/Mode7_Dispatch -> Mode6_GameOver/Mode7_Ending.

Open questions (round 2):
- ObjHandlerTable words are bank-ambiguous: labels in the .word list name the pair-6/7 target. Boss
  types resolve in bank 11 ($0B, $19, $1F, $28, $34) or bank 5 ($2D) by BossClass; confirmed for
  stages 1-2/2-3/3-3/4-3/5-2/6-2b by the type census, but other types might also run under another
  pair in some stage (not checked). Round 3 (L019): bounded loop-1 census under BossClass != 0 (stages 1,
  7, $0D, $10, $12, $15) saw only the boss types; still not exhaustive.
- [Resolved round 3, L019] Fixed-bank ObjT33_Handler jumps to $BE86 / calls $8754: these are
  ObjT32_Handler_St00_St01_Skip (bank 7) and ObjT03_Spawn (bank 6) under pair 6/7 (only pair where both are
  instruction starts; bounded artificial run in stage $11 fetched them with BossClass 0). Types $32/$33 are
  spawned only by the stage $11 (5-1) wave script at bank 12 $96A3. Operands are now symbolic.
- Enemy identities: ObjTNN names are mechanical. The type census (typeprobe-out.json) only gives a
  partial stage map; which game character each type is remains open.
- Slots 10-15 show ObjType 1/2 in every census sample although OBJT_WEAPON1/2 are documented as
  slot 14/15 sub-weapon objects; the meaning of these values in slots 10-13 is not established.
- [Resolved round 3, L019] StageEventByIndex: StageEventIndex 3 (set by StageEvt_Stage3_1A) is consumed in
  StageState 0/1 (StageIntro_Load skips the intro screen; StageLoadGraphics resets column/script and clears
  it) before StageEventDispatch can run, so the code byte at $CE84 is never read as an event index.
- Ending step $15: controller-2 A+B+Left restarts the ending; story-scene end: controller-1 Start +
  controller-2 A+B sets WeaponAmmo 9. Both read as hidden features; not exercised at runtime.
- Three anti-tamper checks are now known: logo-screen sum (DamageDoubler), ChecksumBank12Text
  (DamageDoubler) and ChecksumFixedBankTable (GameOptionFlags bit 4 -> passwords rejected).
- ~640 dispatch-table entries keep structural names (<Owner>_StNN, ObjTNN_Handler, VehTypeNN) with a
  generated "Entry ... / Mechanics: ..." comment; their role is explained at the owner. Per-state game
  meaning (e.g. which attack animation) is not researched.
- L017 (round 3, pass-003): 6 bytes became Code (Static 0.6, no label, reachability not claimed) - RTS
  tails at $8733 (bank 4), $B1AE and $B792 (bank 5) inside a branch span of their routine, and JMP Reset at
  $C1D2 (bank 14). Source comments call them unreachable dead tails. The 3 bytes left unknown at
  pass-003 (bank 10 $8CBA, bank 11 $BBB6, bank 15 $FD4A) are lone unreferenced RTS-valued ($60) bytes: no
  reference, enclosing branch, reader or twin copy, so code vs stray data stayed undecided. At pass-004 they
  are 1-byte generic Data regions 1907-1909 (Manual, conf 0.3) by user-directed fallback only (explicit batch
  authorization; analysis/handoffs/round-04-fallback.md): meaning unknown, no observed or proven data role,
  no reachability claim. The code-vs-data ambiguity itself is unchanged.
- Wave scripts: record format is established (9 bytes), but where each stage's script ends (terminator)
  is not; the bank-12 block $93B4-$9FFF stays generic data in the map.

## Round 4 (pass-004) final editorial pass

- User-directed fallback bytes: see the L017 item above; source notes at the three Data headers say the same.
  This is a classification by instruction, not evidence; readers should not treat these bytes as data tables.
- Renamed because the old names contradicted the code: BossW2_Hit -> BossW2_StartAttack and BossW6_Hit ->
  BossW6_StartAttack (both start the attack chosen in ObjAiFlags; hit reactions are elsewhere), BossW2_Defeat ->
  BossW2_IntroEnd (state 1: ends the intro and restarts the boss music before BossW2_FightInit), BossW4_Hit ->
  BossW4_IntroLeap (intro-sequence entry 3; "entrance leap" is a tentative reading of a knock-down style arc).
- Stale round-1 comments that were stacked on later, contradicting descriptions were removed (e.g. the old
  "enemy grab/hold" note on BossW5_Weapon1Hit, "explosion frames" on VehObjOffscreen, "check entered code"
  on PasswordPackByte). VehObjRoadSpeed now says what it computes (16-bit negated road speed); its use as the
  object drift speed is tentative.
- Named tables whose role was already documented: AttractStoryTable, IntroFallerState/X/Y, BgAnimLists (bank
  10, not 11 as one comment said), EndingCreditScriptIds. High-byte reads of named .word tables now use
  Table+1 instead of separate Data_Bnn equates; 297 Data_Bnn equates remain mechanical (consumer notes).
- RAM usage notes for generic RAM symbols (p2-ramnotes) were regenerated from the current source; they
  previously listed only Part-2 users. BossVarBE is a boss fight frame counter (BossFight, BossW4_Handler).
- Still open (unchanged): per-state game meaning of structurally named dispatch entries; enemy/boss
  character identities; ObjVar418/47C/490/4A4/4E0/4F4 and BossVar508-50B field meanings; wave-script
  terminators; BossW2 intro entries Start/Throw/TakeHit/Flash (scripted or live is not established);
  BossTimerAdvance runs with an unindexed ObjTimer (slot 0 assumed).
