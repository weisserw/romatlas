# Commentary progress

Prepared verified baseline (round 1, from checkpoint pass-001; maps/current.json
sha256 21a4a6bf...a946, identical to maps/pass-001.json).

## Round 1 commentary pass (2026-11-24) - COMPLETE for current source

Method. source/game.asm is produced from the frozen before/game.asm by the
local overlay tools/overlay.py using the annotation file tools/notes.txt
(labels, RAM/register symbols, bank-local zero-page names, data-table names,
block/inline comments, notes for every unknown range and data block). The
overlay only adds labels, symbol assignments and comments and replaces numeric
operands by symbols/labels of identical value; `scripts/commentary.py verify`
proves byte identity. Future rounds: keep tools/notes.txt as the master record
and re-run tools/overlay.py against a fresh before/ (or carry notes by physical
range) rather than hand-editing source/ - otherwise edit source/ directly and
stop using the overlay. tools/review.py refreshes review.json (scaffold intact).

Scope covered: all 459 Code regions (36,708 bytes) in banks 2-7, all 36 Data
regions and all 67 unknown ranges (explanatory notes, no new classification).
- Fixed bank: bank switching/far calls, reset/title/options flow, game loop,
  tower setup, in-tower frame loop with sprite-0/DMC lag-frame handling, NMIs,
  controller reading, enemy objects, shot, occlusion masking and RAM tile
  copy, player state machine (15 states), lifts/doors/triggers/breakables,
  map collision, tower renderer, text screens, score/extra life.
- Bank 2: side-column fill and per-tower player init.
- Bank 3: player frame upload, CHR buffer upload, options screen + NMI,
  game-over secret, sprite slot sort, checksum, timer, attract input,
  high score, camera, continue screen.
- Bank 4: unrolled nametable row copies, sound driver (API, sequencer,
  envelopes, effects), RAM routine copy, background tile generator, tally text.
- Bank 5: timer/score HUD, tower parameter loading, climb points, bank 5 cell
  drawers, tally counting, player sprite.
- Bank 6: complete bonus stage (setup, frame loop, NMI, player states,
  scrolling map columns, collectables, hazard, palette/tile fades).

Numbers: 1319 code labels in source: 1182 explicit names from notes (414
routine/entry names, 768 descriptive branch labels), 137 remaining automatic
branch labels (@loop_/@skip_ + address, all inside documented routines);
279 RAM/register symbols, 64 bank-local zero-page names (bonus stage, options
screen, tile generator), 103 named data tables (+ generator data labels),
293 block comments, 67 unknown-range notes, 38 data-block notes, 689 review
units.

Names/interpretations established this pass (selection): HiScore ($15-$17,
not a random seed), PlayerScore 24-bit at $0C+3*player, SoundOptionFlag
(0 MUSIC / 1 EFFECTS), HeroMode bit 7, PlayerContinues ($20), JoyPressed/
JoyEdgeMask semantics, TowerMapBase (map in ROM: 16 pages, one per column),
DMC sample used as a lag-frame timer (StartDmcTimer/LagFrameSplit),
RAM routine $06D5 copied from bank 4 $B692 (CopyRamTileRoutine), AttractMode
set by the title NMI after ~2670 idle frames, bank 3 $8000-$982F = player
frames, bank 5 $8000-$9FDF = title graphics, bank 3 $9BAE.. = text font,
bank 4 $A8BC/$AD38.. = note periods and sound data.

Verification (handoff): `python3 /root/romatlas/scripts/commentary.py verify`
-> ok=true, byte_identical=true (commentary/verification.json). Reading copy
refreshed: commentary/reading-after.asm.

Deferred / unresolved (no readability backlog remains): exact game meaning of
enemy states 2-9 and of several object/map cell kinds; sound command
semantics beyond their mechanical effect; bonus-stage fade purpose; see
uncertainties.md. Editorial audit remains pending (final acceptance only).
