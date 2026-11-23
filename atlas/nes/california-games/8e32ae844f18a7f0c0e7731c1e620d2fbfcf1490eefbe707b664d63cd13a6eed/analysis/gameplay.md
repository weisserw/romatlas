# Gameplay research

Game identified from in-ROM strings: California Games (NES), (C) 1987 Epyx, program by Rare Ltd.,
distributed by Milton Bradley (1988). Mapper 2 (UxROM), 128 KiB PRG, CHR-RAM. Region/revision not
independently confirmed (NTSC assumed by the debugger baseline).

## Sources

No online sources consulted in round 1 (local play was sufficient for easy gains). All notes below are
local confirmations against this ROM (round-01 disassembly), not external claims.

## Locally confirmed controls and routes (round 1)

- Title: Start (press and release) opens the main menu. Title alternates license/logo screens; no
  attract gameplay demo seen in ~1300 frames.
- Menus: Up/Down move cursor, A selects (Start does not select in menus).
- Main menu: 1) compete in all events, 2) compete in some events, 3) practice one event, 4) view high scores.
- Practice list: 1 half pipe, 2 foot bag, 3 surfing, 4 skating, 5 BMX, 6 flying disk, 7 done.
- Every event: "PRESS BUTTON A TO START"; afterwards "PRACTICE AGAIN? YES/NO" (Down toggles, A confirms).
- Foot bag: 1:15 timer; RAM $0052 holds the seconds counter (decrements once per 60 frames). At 0:00
  press A to reach FOOTBAG SUMMARY.
- Flying disk: Left held ~20 frames, Right held ~15 frames, then A releases a throw (score stayed 0 with
  this recipe). "PRESS A TO CONTINUE" between throws.
- Compete: name grid (A enters letter, END at row 4 col 7), team/sponsor grid (9 sponsors), player 2
  END with empty name ends registration, "IS THIS CORRECT? A) YES".
- Compete order observed: half pipe, foot bag, surfing, skating, BMX, flying disk, then
  "NEXT EVENT: HALF PIPE" again (loop); overall ending not reached (see analysis/scenarios.json S12/S14).

## Input API note

The NES emulator.input takes a full `buttons` array per port (e.g. {"buttons":["start"],"port":1});
`button`/`pressed` fields are silently ignored (see analysis/toolkit-issues.md TK-01).
