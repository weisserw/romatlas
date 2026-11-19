# Gameplay research

Game identified locally from the title screen: **Bases Loaded 3** (Jaleco, (c)1991,
"Licensed by Nintendo of America"), NTSC, MMC3 (mapper 4). Title menu: 1P / 2P / WATCH / EDIT.

## Sources (external, unconfirmed unless marked)

- Wikipedia: https://www.wikipedia.org/wiki/Bases_Loaded_3 ; MobyGames: https://www.mobygames.com/game/14509/ ;
  Take on the NES Library: https://takeontheneslibrary.com/tag/bases/ (fan review).
- Claims (hypotheses): 1P goal is a "perfect game" rating (start 100, -3 per infraction in 13
  categories); you must beat a Level-5 team with a 100 rating to finish; a rating <60 or a loss
  resets to Level 1. EDIT mode edits one special team's player names/stats.
- Confirmed locally (round 1): 1P intro text "PLAY THE PERFECT GAME!! GET A PERFECT RATING!! ...
  YOUR PLAY IS RATED IN 13 CATEGORIES" appears after 1P > Music select.

## Controls / navigation (tested on this ROM)

- NES adapter input must be sent as `emulator.input {"buttons":[...]}` (see toolkit issue TK-001).
- Title: Left/Right move the menu cursor (Select does nothing); Start chooses. Next screen is a
  MUSIC ON/OFF choice (Start accepts). WATCH then goes to TEAM SELECT (A), GROUND SELECT
  (Chicago / Los Angeles / New York; A), STARTING ORDER SELECT for each side (A on READY).
- Starting order: Up/Down move the cursor, A on a batter then Right/A swaps order (observed swap
  of Walsh/Leftly), Down to READY then A.
- 1P: after the intro text (A/Start advance) TEAM SELECT marks the player team "1P" and the CPU
  team "COM"; the 1P team (B) bats first (visitor).
- In-game: A swings when batting / pitches when fielding (random A+direction presses used).

## RAM observations (local, this ROM)

- `$037C` = half-inning index (0 = top 1st; observed 1..18 across a WATCH game; 16 = top 9th,
  18 after the final out). Found by monotonic comparison of RAM dumps every 3000 frames.
- `$68-$77` compared against the 16-byte signature at $FF5D on reset (warm-boot check).
- `$5E` holds the MMC3 $8000 mode bits ORed into every $8000 write; observed PRG mode keeps
  bank 30 at $C000 and 31 at $E000 throughout recorded play.

## Milestones (see analysis/scenarios.json for recipes)

- WATCH full 9-inning CPU-vs-CPU game to GAME OVER and back to title: visited.
- 1P intro text, team select, starting-order swap, batting/pitching with inputs: visited.
- Rating screen after a 1P game, Level progression, EDIT mode, 2P mode: see scenarios.json.

## Round 1 redirect additions (r001-a)

- 1P game end: the player team is the visitor (top). With the visitor ahead after the
  final out: GAME SET -> score/level screen (rating total, LEVEL n, menu NEXT GAME /
  SEE RATING SCREEN; Up/Down + A) -> RATING SCREEN (13 categories with +/- points and TOTAL).
  NEXT GAME starts the next opponent (HomeTeam $0392 went 6 -> 5). With the home team ahead:
  GAME SET -> GAME OVER box -> title. Recipes: scenarios S-009/S-010 (RAM-patched).
- RAM confirmed by these runs: VisitorRuns $0383 / HomeRuns $0394 drive the end check; bank 20
  menus submode 8 compares $037C with 17/$24 and the run totals (static, $8326).
- Stadium $0375 does not choose a field bank: Tbl_26_82AB maps every stadium to bank 2.
- Title cursor in title3 snapshot rests on WATCH: [right] selects EDIT, [left] selects 2P.

## Round 2 disassembly additions (pass-002)

- Correction: the redirect's "NEXT GAME HomeTeam 6 -> 5" was the Up press editing the opponent in
  a team-select screen (writer bank 21 $A818-$A836, sta ($04),y, +/-1 mod 13), not progression.
- Level contract (confirmed locally): zp `$66` = level index 0..5 (screen shows LEVEL $66+1);
  rating total `$03FB` (written bank 23 $B2A0). Bank 23 $B386 dispatches on $66: ratings
  >=95/80/70/60 map to new $66 = 4/3/2/1 (else 0); only at $66=4 does a rating >=100 give $66=5.
  This matches the sourced claim "beat a Level-5 team with a 100 rating".
- Ending reached (artificial, S-012): $66=4 + $03FB=100 after a 1P win, NEXT GAME ->
  "CONGRATULATIONS!! 100 / YOU'RE NUMBER 1. YOU BEAT THE BEST." then ending art and staff credits.
- 2P mode reached with ordinary inputs (S-011): title [left] selects 2P; port 2 drives the second
  player's team select / starting order / pitching.
