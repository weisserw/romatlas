# Commentary uncertainties (round 1)

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
- Stage numbers: $03 = stage 1 (observed), $08 = title/attract; the others follow the
  loader tables $9040/$903A but their in-game identity is untested.
- Scripted enemy bodies (ObjType07_ScriptedEnemy, ScriptedEnemyA/B_Update,
  FallAndBounce tail) are named mechanically; boss identities unresolved.
- The PCM player ($46 events, DMC_RAW writes) is inferred from code, not heard.
- RAM code at $0532, $0600, $0720, $0728, $0782 is called but its upload origin is
  unknown (lead L-003).
- Heuristic fall-through cuts (bank 0 $B45D, $B9CA; bank 2 $FC92): reviewed; each is
  `lda table,y / bne` (or asl/adc #$A4 / bne) where the table bytes are non-zero for
  the indices used, so treating the branch as always taken is consistent. Commented
  in source.
