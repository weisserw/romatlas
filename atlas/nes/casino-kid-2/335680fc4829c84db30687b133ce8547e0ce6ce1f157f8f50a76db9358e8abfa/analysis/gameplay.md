# Gameplay research

Game: **Casino Kid 2** (Sofel, (c)1992, licensed by Nintendo; header string
"CASINO KID 2" at file 0x1FFF0). NES mapper 2, 128 KiB PRG, CHR RAM. Region assumed
NTSC/North America (English text); not independently confirmed.

## Sources (round 1)
- Web search (2026-11-23), no dedicated Casino Kid II walkthrough or password list found.
  Summary from search results: sequel to Casino Kid, NA release 1993; the player
  must defeat nine opponents worldwide at roulette, poker and blackjack to reveal
  the leader. https://en.wikipedia.com/wiki/Casino_Kid (and search snippet).
- Game Genie codes page (not used, hypotheses only):
  https://almarsguides.com/retro/walkthroughs/nes/games/casinokidii/gamegenie/
- TCRF page covers the first game only (passwords/debug not applicable):
  https://tcrf.net/Casino_Kid
Status: external descriptions are unconfirmed hypotheses unless noted below.

## Locally confirmed (round 1, see analysis/scenarios.json)
- Controls: title menu NEW GAME / SAVED GAME (right selects SAVED GAME, start confirms).
  Text advances with A. SAVED GAME opens a PASSWORD grid (0-9, B-H, J-R, T-Y, $ ! ?, arrows).
- World map: dpad cycles opponents: Canada Rocky Hammer (blackjack, $1000, bet $10-$100),
  England Paul Kieton (roulette), Russia Nikorai Bunin (blackjack), China Rie Lenka (roulette),
  Japan Ryu Mishima (poker; locked: "BEAT AILTON FIRST!"), Australia Jyina Nagule
  (blackjack, $100, bet $1-$10, blackjack pays 3:2). Brazil and an African flag are on the
  map but not in the initial cursor cycle (likely unlocked later). Travel cutscene (plane,
  day/night variants) then country title card.
- Blackjack: "PLEASE BET" chip panel: up adds a chip, down removes, left/right change chip
  denomination, A confirms; choices STAND / HIT / SPLIT / SURRENDER / DOUBLE-DOWN all exercised.
- Roulette: table with cursor chip; A opens chip panel; up adds chip; select opens
  "DO YOU WANNA QUIT NOW? YES/NO". How to spin the wheel is not yet known.
- Player starts with $200.

## Shortcut hypotheses (unconfirmed)
- $07AD holds a 16-bit opponent bankroll-like value at hand start (1000 / 100), but
  patching it did not change the displayed bankroll -> not the HUD value. See lead R1-L004.

## Round 1 redirect (r01-a1)
Opponent money RAM ($07A9/$075B blackjack, $075B poker, $06A0/$06DB King), OpponentResultBits
unlock recipe, poker controls (A hold, B draw), roulette spin = Start, final match vs KING reached,
credits via artificial pc=$C1FD. Details: analysis/scenarios.json S05-S12.
