# Gameplay research

## Identification (confirmed locally, round 1)

- Title screen renders "Happy Birthday Bugs" (Bugs Bunny 50th-anniversary logo);
  PRG text at file 0xff37.. reads "tm ... warner bros inc ... Kemco ... licensed by
  nintendo of america inc" (game charmap: `~`=space). This is Kemco's 1990 NES/Famicom
  Bugs Bunny game, released in North America as *The Bugs Bunny Birthday Blowout*.
  The title string suggests the Japanese Famicom release ("Happy Birthday Bugs");
  intro text is English ("IT'S BUGS BUNNY'S 50TH BIRTHDAY"), so the exact regional
  revision is uncertain. iNES mapper 4 (MMC3), 128 KiB PRG / 128 KiB CHR.

## External sources (hypotheses until confirmed)

- Web search (2026-11-21): no password system or level select in the game; bosses
  include Yosemite Sam and Sylvester. A community FAQ exists
  (https://www.video-games-museum.com/en/faqs/Nes/4013-Bugs-Bunny-Birthday-Blowout-The.pdf,
  not read in detail). Other pages: https://www.mobygames.com/game/13974 ,
  https://almarsguides.com/retro/walkthroughs/nes/games/TheBugsBunnyBirthdayBlowout/ .
  Applicability to this (possibly Japanese) image is unverified.

## Controls and flow (tested locally)

- Input must be sent as `emulator.input {"port":1,"buttons":[...]}` (see
  toolkit-issues TK-001). Start on the title advances to a long text intro
  (A/Start advance pages); ~3100 frames from power-on to Stage 1 Round 1 play.
- In play: D-pad moves, A jumps, B swings the mallet (observed visually), Start pauses.
- After a round: "STAGE n ROUND m COMPLETED!! GO TO THE BONUS ROUND" screen, then the
  bonus-round board ("WELCOME TO THE BONUS ROUND"); with too few carrots it shows
  "YOU NEED MORE CARROTS" / "SEE YOU LATER" and continues to "STAGE n ROUND m+1 GET READY".

## RAM contract (from code, confirmed by experiments)

| RAM | Meaning | Evidence |
|---|---|---|
| $0420 | stage/world index 0..5; 6 = game finished (main loop $E81F CMP #$06 -> $E84E ending path, then JMP reset) | $E86E-$E886 increments; $E899 per-world music table |
| $0421 | round index 0..3 (4 rounds per stage) | $E86E: INC, CMP #$04 wraps and increments $0420 |
| $0429 | lives (3 at start) | decremented at $D464 on death; BMI -> game over at $E805 |
| $043F | round state: $FF/0 normal play; 1 = died; 2,4,5,6 = transition/door states; 8 = goal reached; $80 = round cleared (main loop $E7F7) | jump table $991E (9 handlers) via $65 dispatcher; $D45C/$D47A set $80 when state 8 |
| $F5 | joypad-1 latch (ORed each poll at $EC4F) | trace of $4016 reads |

## Shortcut recipe (artificial state; see scenarios.json)

- Clear the current round: write $043F=$08 while in play; the game sets $80, shows the
  completion and bonus screens and loads the next round (press Start through screens).
- Jump to stage W round R: in play, write $0420=W, $0421=R-1 (or W-1/3 for R=0), then
  $043F=$08. Writing $043F=$80 directly is overwritten (INC at $C6E3) and does not work;
  $043F=$04 starts a door/scroll transition that can kill Bugs.
- Ending: clearing stage index 5 round 3 makes $0420=6 -> $E84E (call $43 ending
  routine) -> JMP $E8C0 (reset).

## Open items

- Carrot counter RAM not identified ($0422 write did not update HUD); needed to enter
  the bonus game (distinct mechanic) — deferred.

## Round 1 commentary findings (from code, untested at runtime)

- Carrots = $042A (capped at 99, halved on death; pickup tile class $16); the bonus round
  needs at least 10 carrots.  PlayerHealth = $0428 (class $29 pickup refills to 12).
- Pair 6 entry 8 ($9FC8) is a hidden stage/round select screen with no known caller.
