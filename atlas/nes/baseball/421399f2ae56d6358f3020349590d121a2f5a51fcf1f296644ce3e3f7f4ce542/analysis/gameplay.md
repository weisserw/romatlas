# Gameplay research

Game identified locally from the title screen (scratch frame, not retained): "BASEBALL",
"(c) 1984 Nintendo of America Inc." NROM-128, horizontal mirroring.
Version/region: NTSC iNES image (sha256 in workspace.json); exact revision not established.

No online sources consulted in round 1 disassembly (not needed for the easy-gain pass);
controls below are LOCALLY TESTED observations only.

## Locally confirmed (round 1 disassembly, 2026-11-18)
- Title: cursor between "1 PLAYER GAME" / "2 PLAYER GAME"; START confirms.
- 1P: "SELECT YOUR TEAM" with six letters (A C D P R Y); RIGHT moves, A confirms;
  second A press proceeds to "PLAYBALL" and the game starts with the player batting
  in the top of the 1st inning.
- Batting: A swings (observed during random bounded input play; see analysis/scenarios.json).

## Unconfirmed / to research
- Pitching/fielding controls, pinch hitter / relief menus, 2P mode, game-over / final
  score screen. Inning/score RAM addresses for shortcut to the final inning.
