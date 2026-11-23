# Gameplay research

Game identified locally from ROM text: "CAPTAIN SKYHAWK" (Milton Bradley, program by Rare,
(c) 1989), NES mapper 7 (AxROM, 4 x 32 KiB PRG, CHR-RAM). No online sources were consulted
in round 1 (local easy gains were sufficient); external descriptions remain unverified.

## Locally confirmed (round 1, this ROM, romatlas NES debugger)

- Input: the NES adapter needs `emulator.input {"buttons":[...]}` (full held set); see
  analysis/toolkit-issues.md TK-001.
- Boot: credits/copyright screen (~frame 200), letter-scatter attract animation, title
  "MILTON BRADLEY PRESENTS / Captain SKYHAWK / PRESS START" at ~frame 750.
- START on title -> Captain Skyhawk logo screen -> (START) mission briefing screen
  ("DESTROY ENEMY BASE" with a base diagram) -> (START) isometric flight over green terrain.
- In flight: A fires; d-pad steers/changes altitude. HUD shows score, lives, weapon counts.
- Losing all lives -> "TODAYS GREATEST HEROES / ENTER YOUR NAME" high-score table.
- RAM (from code + snapshots): $0669 = mission index (0 for mission 1; $FF checked by the
  briefing text selector at bank0 $DCEB; value 7 selects a special message), $02 = lives
  (5 at start of mission 1; incremented at bank0 $DDA1 for an extra life, capped at 10).
- ROM text (hypotheses for routes): mission objective strings in bank 0 $E017..$E12x
  (ALIEN INVASION, LOCATE AND DESTROY ENEMY FIGHTER, DESTROY ENEMY BASE, DROP SUPPLIES IN
  CROSSES, PICK UP SCIENTIST, DESTROY ENEMY FIGHTER PLANES, DOCK WITH SPACE STATION,
  DESTROY ENEMY SPACE STATION); shop texts in bank 3 $BCB3.. (NEXT MISSION, WEAPONS,
  AIM54 PHOENIX, MAVERICK AGM, CANNON, HAWK BOMBS, CREDITS); messages in bank 0 $EA3D..
  (EXTRA LIFE, GET READY FOR MISSION 1, CONGRATULATIONS, ENEMY BASE DESTROYED, ...).

See analysis/scenarios.json for exact routes, patches and status.
