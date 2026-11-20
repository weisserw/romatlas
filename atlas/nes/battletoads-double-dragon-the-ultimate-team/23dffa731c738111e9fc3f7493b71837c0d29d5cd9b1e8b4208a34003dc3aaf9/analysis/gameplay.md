# Gameplay research

Game identified locally from ROM strings (bank 6 at file $36806..$37C4F):
"BATTLETOADS & DOUBLE DRAGON - THE ULTIMATE TEAM", "1993 RARE LTD", "LICENSED TO
TRADEWEST". Mapper 7 (AxROM), 8 x 32 KiB PRG banks, CHR RAM. Region/revision not
established (title shows "LICENSED BY NINTENDO"; NTSC assumed by the debugger).

External guides: not consulted in round 1 pass 1 (local RAM shortcut was sufficient
for stage visits). ROM strings hint at features to look up later: "SELECT LEVEL",
"SUPER WARP ZONE / 5 LIVES", "MEGA WARP ZONE / 10 LIVES", "T.BIRD CALLS UP THE
TOADS", "GAME OVER", "NO CONTINUE / CONTINUE", "STARRING ... THE DRAGONS ... THE
BOSSES" (likely ending/credits text). These are hypotheses, not confirmed routes.

## Locally confirmed (round 1, pass 1)

- Controls: debugger `emulator.input` must use `{"buttons":[...],"port":1}` (see
  toolkit-issues TK-001). Start advances title -> mode menu (1 PLAYER / 2 PLAYER
  GAME A / GAME B) -> character select (Rash, Zitz, Pimple, Billy, Jimmy).
  At character select wait ~60 frames, press A, wait 60, press Start -> intro
  cutscene (spaceship) -> stage 1 (Ratship hull, $10 = $03).
- RAM $0010 = current stage id. $08 during title/attract, $00 during the intro;
  bank 6 code at $9736/$9739 (lda $903A,y / sta $10) stores the next stage.
- Shortcut (artificial): after the intro writes $10 (run_until $10 != 0 from the
  intro), overwrite $10 with another stage id, then run ~400 frames.
  Observed: $03 Ratship hull (stage 1), $04 space chase/asteroids (bike-like
  flying stage), $08 colossus/tower exterior, $02 pink crystal interior with
  Billy/Jimmy enemy, $06 purple organic corridor, $05 grey industrial corridor,
  $09 red brick interior. $01 and $07 produced a blank grey screen after 400
  frames (not a confirmed stage; maybe needs longer or other state).
- Stage order hypothesis from table $903A (bank 6): $00->$03->$04->$08->...; not verified.

## Round 3 / disassembly pass 3
- 2P Game A replay from traces/r02a1_snap_2p_gameA.json (1,200 frames, mixed two-pad inputs) did not
  exercise MirrorPlayerAppearance (enemy latching onto a player); no new milestones attempted.
