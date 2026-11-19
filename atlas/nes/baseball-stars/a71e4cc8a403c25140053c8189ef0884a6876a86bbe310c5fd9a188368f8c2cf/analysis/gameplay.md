# Gameplay research

Game identified locally from the title screen: **Baseball Stars** (SNK, (c) 1989 SNK Corp. of America,
licensed by Nintendo). No external sources consulted in Round 1 (local observation sufficed for easy gains);
statements below are confirmed locally unless marked otherwise.

## Locally confirmed (Round 1)
- Title appears ~1200 frames after power-on; START opens the main menu.
- Main menu: LEAGUE PLAY, VS, VIEW TEAM, MAKE TEAM, END PLAY; four blank slots. Bank-3 strings
  ($ACA4-$ACE9) show conditional items TRADE PLAYER, HIRE PLAYER, REMOVE TEAM, FIRE PLAYER (not yet seen).
- VS -> 1PLAYER / 2PLAYER / COMPUTER PLAY, then SELECT TEAM (A picks; second pick for opponent).
- COMPUTER PLAY runs a full CPU-vs-CPU game with no input (~45k frames reached top of 5th).
- Pre-game lineup screen: START GAME / CHANGE BATTING ORDER / CHANGE FIELDER / PLAYER CHANGE /
  CHANGE PITCHER / STARTING PITCHER, cycled with left/right.
- MAKE TEAM: choose strength (8 options incl. '?'), enter name (letter strip with SP/arrows/END,
  SELECT scrolls), ARE YOU SURE? YES -> MAKE A LEAGUE.
- MAKE A LEAGUE: left/right change team count and games; then SELECT TEAM picks per team.
- LEAGUE PLAY menu: CONTINUE PLAY / SEE STANDINGS / NEW LEAGUE. Standings grid; choose match cell;
  "DO YOU WANT TO WATCH THE GAME?" NO simulates instantly -> box score -> attendance/money screen.
- END PLAY shows a battery-memory WARNING screen.

## Open hypotheses / routes (not yet tested)
- TRADE/HIRE/FIRE PLAYER and REMOVE TEAM likely need league progress or money; a RAM shortcut for
  money/league state is the obvious route (contract unknown; see analysis/leads.json).
- Game end / season end screens for watched games (CPU game end not reached within sampled frames).
