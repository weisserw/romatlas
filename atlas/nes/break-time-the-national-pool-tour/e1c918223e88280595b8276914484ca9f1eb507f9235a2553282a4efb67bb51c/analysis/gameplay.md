# Gameplay research

## Identification (local, confirmed)

- Title screen of this image reads "BREAK TIME - THE NATIONAL POOL TOUR", "(c) 1992 FCI,
  produced by Ponycanyon & Opera House, licensed by Nintendo" (frame captured at ~285
  frames after power-on, round 1). NES, MMC1 (mapper 1), 128 KiB PRG, 32 KiB CHR, NTSC.

## External sources (round 1)

- Web search (2026-11-21) found Wikipedia/MobyGames/TASVideos entries but no usable
  controls text. A GameFAQs mirror URL returned "not found"; the TCRF page fetch returned
  injected non-game content and was ignored. Sources:
  - https://en.wikipedia.org/wiki/Break_Time:_The_National_Pool_Tour (not fetched in detail)
  - https://tasvideos.org/2754G (search snippet: "physics are wonky")
- Search snippet (unconfirmed): A selects a mode, B backs out to previous menu.
- Conclusion: controls below were learned by local experiment, not from a manual.

## Locally confirmed controls and menus (round 1, debugger inputs)

- Title: START (or A) after ~285 frames opens MAIN MENU.
- MAIN MENU items: NINE BALL, ROTATION, EIGHT BALL, 14-1 RACK GAME, THE NATIONAL POOL
  TOUR, PRACTICE MODE. The first d-pad press shows the cursor (on NINE BALL); later DOWN
  presses move it; A selects (border turns gold, then next screen).
- "HOW MANY PLAYERS?": 1 PLAYER / 2 PLAYERS / WATCH. START/A on 1 PLAYER.
- "WHICH PLAYER?": opponents ED/Beginner, JIM/Advanced, JANE/Semi-pro, MIKE/Pro.
  START then A selects (ED in the recorded route).
- "HOW MANY GAMES DO YOU WANT TO PLAY?": 1 GAME / 3 GAMES; A selects.
- "PLEASE INPUT YOUR NAME": letter grid; A enters a letter, d-pad moves, START ends
  (recorded route enters "ABII").
- Table: "NINE BALL"/"ROTATION" banner, then "SHOOT OFF" (lag); A opens the power-meter
  panel (cue stick animates), a second A shoots. Messages such as "IT'S <name>'S TURN",
  "ED BREAKS!" wait for A. CPU opponent shoots by itself.
- Controller state RAM: $56/$57 held buttons (port 1/2), $58/$59 newly pressed
  (routine at $C4BD in fixed bank 14 reads $4016/$4017; see analysis/investigation.md).

## Untested / deferred

- Eight Ball, 14-1, National Pool Tour progression and ending, Practice mode, WATCH
  mode, 2-player mode, spin selection (B/SELECT effects not yet understood).
- No passwords or debug shortcuts known.

## Correction (round 1 commentary)

- Controller RAM is the other way round: $58/$59 hold the buttons currently held and
  $56/$57 the buttons newly pressed this frame (ReadJoypads, bank 14 $C49D). Button bits:
  0 A, 1 B, 2 Select, 3 Start, 4 Up, 5 Down, 6 Left, 7 Right.
- Code-derived (not yet tested): holding SELECT at power-on sets $C5 (sound debug flag);
  TourStage values 5/6/7 open a sound test, the ending and a voice test in bank 0; the
  text "THE PASSWORD" exists in bank 0 (password path suspected at $83C2, lead L-001);
  A+B+SELECT on the tour map skips to the match result (MainLoop $C21A).

## Round 2 (disassembly, locally confirmed)

- PRACTICE MODE: first DOWN moves the cursor to TRICK SHOT; A opens 7 named trick shots
  (4 TO 4 BY JOHN EDWARDS, SIX SHOOTER, HIGH NOON, LOOP SHOT, DOUBLE TROUBLE, 3-TIMES,
  OLDIE BUT GOODIE). A, A shoots.
- Tour shortcut confirmed: repeatedly pressing and holding A+B+SELECT through the tour map /
  opponent profile skips each match to a win; all 5 cities lead to the ending (~15,000 frames
  from power-on). Route in analysis/scenarios.json SC-015.

## Round 2 commentary notes (static reading, not yet exercised)
- Tour passwords exist: after a won stage ShowPasswordScreen shows a 7-consonant
  password (name + stage + checksum); the tour menu's first item reads one back
  (TourMenu_Password / DecodePassword).  Three special passwords jump to the sound
  test, the ending and the voice test (TourStage 5/6/7).  Lead L-018.
- Hidden CPU handicap: with pad 1 holding SELECT+Up and pad 2 holding A+Down+Left
  while the CPU plans, it skips its skilled shot types (CpuChooseShot).
- Unreferenced debug-like code: DebugScrollWithPad2 (bank 10), DebugShowBallPosition.
