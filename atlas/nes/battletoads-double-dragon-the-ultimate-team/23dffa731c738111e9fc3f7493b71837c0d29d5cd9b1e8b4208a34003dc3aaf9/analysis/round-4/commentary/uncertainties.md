# Commentary uncertainties (round 1, revised in rounds 2 and 3)

- Object fields: only position/height/velocity, type, frame, flags bit 7, direction,
  speed, animation id/position and the grab links are established. Fields named
  Obj_XXXX keep their address because their meaning is mechanical only
  (timers/state counters). Obj_051B is used as health by ApplyHitReaction and the
  status bar, but its low bits are also used as a stun refill counter.
- $05F0/$05F1 and $05F2/$05F3: per-player selection values set by the selection
  screen; $05F2,x indexes per-character tables (values 0-4 assumed). Which is the
  character id is not proven.
- RAM $0680-$074F is shared: particles ($0681+9n) during gameplay, loader/menu work
  variables in bank 6. Object-field symbols are suppressed in bank 6 $8EFA-$FFFF.
- Stage numbers: round-1 redirect (L-007) observed the level order $03, $05, $02, $04,
  $08, $09, $06 and the ending screen $0B. The round-1 claim "$08 = title/attract" was
  wrong ($08 is level 5) and has been corrected in symbols.txt.
- Scripted enemy bodies (ObjType07_ScriptedEnemy, FallAndBounce tail) are named
  mechanically; boss identities unresolved. Round 1's ScriptedEnemyA/B_Update were
  misattributed spans: they are SparkEvery4thFrame and ApplyGravityFastAndDrawOfs,
  and the large body belongs to ObjType33_Boss ($E87A).
- The PCM player ($46 events, DMC_RAW writes) is inferred from code, not heard.
- RAM code: $0782 comes from RamRoutineImage_0782 (bank 7 $805F, runtime-verified in
  round 2); $0532/$0600/$0720/$0728 are buffers built at run time (L-003 redirect).
- Heuristic fall-through cuts (bank 0 $B45D, $B9CA; bank 2 $FC92): reviewed; each is
  `lda table,y / bne` (or asl/adc #$A4 / bne) where the table bytes are non-zero for
  the indices used, so treating the branch as always taken is consistent. Commented
  in source.

## Round 2 (pass-002) additions

- Object type handlers ($9A71, 62 newly classified bodies): names are ObjTypeXX_<role>.
  Roles such as Boss, Flyer, Crusher, HazardSpawner, PlayerReticle, Vehicle are
  inferred from mechanics only (spawning, damage via HazardHitPlayers, scroll locks,
  boss music/phase helpers); which game enemy each one is was not checked at run time.
  Types $3B/$3C/$43/$4A/$4F/$52/$5A/$33 are called bosses because they use the phase,
  defeat and boss-intro helpers (BossIntroLoadScreen, EnemyChangePhase, DefeatSequence).
- ObjType25_PlayerReticle reads the owner's joypad and locks onto targetable objects;
  the "lock-on weapon" reading is tentative.
- ObjTypeFlags ($9BE0) bit 0 = targetable, bit 7 = stun instead of knock-back: inferred
  from CheckTargetInRangeA, SpawnBlastPieces and the homing-shot hit code; other bits
  and the tables $9B29/$9B85 remain mechanical.
- Obj_03E0/ObjAnimPos and Obj_04EE/Obj_05A2 double as X and depth velocities for
  thrown/lobbed objects (AimThrowAtSlot, ApplyThrowVelocity); the field names stay
  address-based because their meaning differs per object type.
- Screen actors (bank 6 $06DA-$0739, BgObjHandlers $9C4E): record layout is
  established from StepBgObjScript/ApplyBgObjVelocity/AnimateBgObj; which screen uses
  which actor type is not mapped. Round 1 called these background columns.
- Uncalled Static fragments (e.g. $822B, $82F5, $9A5E, $C7AB, $D1D8, bank 6 $80CE,
  $E552, $F9A8, $FE6A, bank 5 $8401/$B353, banks 1/7 inverted copy loops): mechanics
  commented; reachability unresolved.
- Data blocks: most notes repeat the consumer-based evidence; internal record formats
  (sound scripts, animation scripts, metasprites, spawn lists, maps) are not decoded.

## Round 3 (pass-003) additions

- Remaining unknown bytes (L-027, 16 bytes): five lone $60 bytes (b0 $8D92, b4 $B979,
  b5 $ACFA, b6 $9745, b6 $EFE5) could be dead rts instructions or padding; b0 $E536-$E537
  could be `lda #$02` before $E538 or the tail of PlayerFrameMapA/B (player frame domain
  unbounded); b6 $FE24-$FE2C decodes as an overlapping-entry variant of RunRamColumnCode
  (jsr HideSprite / stx OAMADDR / lda #2 / sta $00A2 using the ldx #$00 bytes) but has no
  reference. Each has a source note; none is decided.
- Stage tables ($8253, $9C80, $AA05, $CCF3, $E34C in bank 0): index ranges come from the
  readers' StageId domain; the RAM targets ($C2, $A9, Obj_0584 step) are mechanical only.
  StagePlayerMode/StageDriftMask/StageObj0584Step for StageId 0-1 read the preceding jmp
  operand bytes (operands are written as Table-2).
- $E522 (20 bytes) are probably later PlayerFrameMapA/B entries; not split into entries.
- Bank 2 $FC94 (160 bytes of pre-shifted bit patterns) and bank 5 $80C3 / bank 6 $EFA6 /
  $8C13: no reader; role unknown.
- Bank-switch module copies (all banks $FF6E-$FFB2): commented as not executed in their
  own bank by the module contract; this is a static argument, not a runtime trace.
- Bank 5 $FF50 snippets are called leftovers of an older bank-switch module (no caller,
  no matching continuation code in other banks); tentative.
- Uncalled fragments: names/comments describe what they would do on entry; whether any
  is reached through an unidentified computed jump is unknown.
- ObjDrawHookTable: hook routines named DrawHook_TypeXX by the object types that use
  them; their drawing effects are not individually decoded.

## Round 4 (pass-004) additions

- The round-3 entry on b0 $E536-$E537 and b6 $FE24-$FE2C is superseded: pass 4 classified
  them (L-028). $E536 `lda #$02` is commented as an unused alternate entry
  (TryLatchOntoPlayer_Id2); no caller found, and the frame-map-tail reading is judged
  unsupported but not exhaustively disproved. b6 $FE24-$FE2B (HideSpritesThenOamDma_Fragment)
  is a truncated sprite-clear/OAM-DMA prefix with no reference; $FE2C is its leftover $8D
  opcode byte (data). Reachability of all three is unresolved.
- Remaining unknown bytes (L-027, 5 bytes): the five lone $60 bytes listed under Round 3;
  notes unchanged.
