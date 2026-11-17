# Gameplay research

Game identified locally from title screen and PRG strings: **Baseball Stars II** (Romstar, 1992, US NES;
"TM & (c) 1992 ROMSTAR, INC. / LICENSED BY NINTENDO"). Mapper 4 (MMC3), battery-backed PRG RAM
(warning text "WHEN TURNING GAME OFF, FIRST PRESS RESET...").

No external sources were consulted in round 1 (not needed for the easy-gain pass); everything below is
locally observed in this ROM with the romatlas debugger unless marked otherwise.

## Locally confirmed flow and controls (round 1)

- Power-on: copyright/title screen; Start (about frame 135) shows an animated title, second Start opens
  the main menu: LEAGUE PLAY, VIEW TEAM, MAKE TEAM, (blank), (blank) on the left; VS, (blank), (blank),
  END PLAY on the right. D-pad moves the cursor, A selects.
- VS menu: 1 PLAYER VS COMPUTER / 1 PLAYER VS 2 PLAYER / COMPUTER PLAY, then an ERROR OFF/ON toggle
  (Right then A selects ON).
- SELECT TEAM grid (12 teams, e.g. TM Tempe Matadors, LS LA Superstars, HE Houston Eagles); A picks
  team 1 then team 2 (cursor moves with D-pad).
- MATCH screen, then per-team STARTING LINE UP / GAME START screens (A advances), then a 4-stadium picker
  (Romstar, City, Country, Dome; D-pad + A), then the scoreboard ("TO THE ROMSTAR STADIUM"), then the
  batter/pitcher view. Player 1 bats first as the visiting team; A swings.
- COMPUTER PLAY runs a whole CPU-vs-CPU game without input (A presses only advance interstitials).

Exact replayable routes and budgets are in analysis/scenarios.json.

## Open questions / ideas for later rounds

- League mode, team editing (HIRE/FIRE/RELEASE/ACQUIRE/TRADE PLAYER, ROOK/VETE/STAR point classes),
  league standings/averages screens and the ending of a league season were not fully exercised.
- RAM contract for inning/outs/score (to jump to late innings or game end) not yet identified.

## Round 1 additions (pass-001, locally confirmed)

- League: MAKE A LEAGUE (2-6 teams, 1-25 games) -> team grid -> KEEP/CHANGE; league menu CONTINUE PLAY /
  SEE STANDINGS / NEW LEAGUE. Each league game asks "DO YOU WANT TO WATCH THE GAME?" - NO simulates the game
  instantly (line score + HOMERUNS list). Winning the league shows a LEAGUE CHAMPION trophy/celebration screen.
- VIEW TEAM: roster (batters/bench/pitchers) -> per-player ability bars (HITTING/BATTING/RUNNING/DEFENSE/LUCK/
  PRESTIGE; pitchers STAMINA/SPEED/L CURVE/R CURVE/DROP + OTHER ABILITIES), salary figures.
- MAKE TEAM: speciality (CONSISTENT HITTERS, POWER HITTERS, GOOD DEFENSE, FAST RUNNERS, STRONG PITCHERS,
  VETERANS, BALANCED TEAM, ?) then one of 6 extra teams. Afterwards main menu gains ACQUIRE PLAYER,
  RELEASE PLAYER and REMOVE TEAM. RELEASE shows a city cut-scene; ACQUIRE (needs a free slot) offers
  ROOKIE/VETERAN/STAR PLAYER, contract signing and BONUS POINT allocation.
- In-game Start = TIME: LINE UP / RELIEF PITCHER.
- END PLAY shows the battery warning screen.
- RAM contract (from bank 8 $9CA2-$9D46): $0497 = half-inning index (0 = 1st top, odd = bottom), $0498 =
  visitor runs, $0499 = home runs. After half-inning >= 16 the game ends when the score order decides;
  tie continues to extra innings up to index $23; a 10-run lead also ends the game early. Patching $0499=1 during
  an extra-inning top half produced GAME OVER (artificial state).
