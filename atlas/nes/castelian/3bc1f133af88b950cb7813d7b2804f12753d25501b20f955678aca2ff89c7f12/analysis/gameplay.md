# Gameplay research

Game identified from ROM text: CASTELIAN (NES, (c) 1991 Triffix Entertainment / Hewson
Consultants / Rollogame; programming Duncan Meech, graphics Mark Jones, music David Whittaker),
a Nebulus-style tower-climbing game. Mapper 2 (UxROM), 128 KiB PRG, CHR-RAM.

Online research: not performed in round 1 (local inspection and RAM shortcuts were sufficient
for the easy-gain pass). External facts below are local confirmations only.

## Locally confirmed (round 1)
- Controls: Start begins game from the credits screen; Select opens OPTIONS; in options,
  Up/Down move, Select toggles (PLAYERS 1/2, EFFECTS/MUSIC, NOVICE/HERO). In game: Left/Right
  walk around the tower, A jumps, B fires, Up enters doors, Start pauses ($4B toggled at $C601).
- 8 towers: TOWER OF EYES, REALM OF ROBOTS, TRAP OF TRICKS, SLIPPERY SLIDE, BROKEN PATH,
  SWIMMERS DELIGHT, NASTY ONE, EDGE OF DOOM. A side-view bonus stage (bank 6) runs between towers.
- After a completed tower: tally (TIME LEFT/TECHNIQUE/EXTRAS x10), A to continue, bonus stage,
  next tower. After tower 8: CONGRATULATIONS / SCORE / HISCORE screen (terminal in 200f test).
- Lose all lives: CONTINUE (YES A / NO B, CREDITS n) then GAME OVER / SCORE / HISCORE.
- 2-player mode alternates players ("PLAYER 2 ENTERING THE ..."), ends with GAME DRAWN /
  PLAYER n WINS.

## RAM (locally confirmed, round 1)
- $0A,x tower number per player (x=$14 current player); $12,x lives per player.
- $41/$42/$43 timer digits (hundreds/tens/units), also used by bonus stage.
- $46 tower-complete flag (tested at $C2AF after the gameplay loop returns).
- $22/$23 controller state (read at $CCE8 strobe loop / $CCFF bit reader); $4B pause.
- $00/$01 NMI handler vector (NMI stub $C008 = jmp ($0000)).
See analysis/scenarios.json for exact patch recipes.

## Locally confirmed (round 2)
- Options menu items: START / PLAYERS n / EFFECTS|MUSIC / NOVICE|HERO; cursor starts on START,
  so Select immediately after opening starts the game; Down then Select toggles PLAYERS (RAM $09=1 for 2P).
- Scores: 3 bytes per player at $0C (P1) / $0F (P2), most significant byte last; $F4D4 compares them
  when both players' lives ($12/$13) are 0 and prints GAME DRAWN / PLAYER 1 WINS / PLAYER 2 WINS.
- In 2P mode player 2's CONTINUE prompt reads controller 2 (press B/A on port 2).
- No high-score name entry seen: game-over shows SCORE and HISCORE only.
