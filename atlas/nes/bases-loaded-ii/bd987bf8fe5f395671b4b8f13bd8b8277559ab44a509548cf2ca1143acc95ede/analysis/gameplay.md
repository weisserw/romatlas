# Gameplay research

Game identified locally as **Bases Loaded II: Second Season** (Jaleco, 1989; title screen
"Bases Loaded II", internal footer text `BASES LOADED 2` at $FFE0 of bank 15). NES, NTSC,
MMC1 (mapper 1), 256 KiB PRG / 128 KiB CHR, no battery. Revision unknown.

No external manuals/FAQs were consulted in round 1 (not needed for the easy-gain pass);
everything below is **locally observed** in this ROM with the configured debugger
(scenario details and inputs: analysis/scenarios.json).

## Confirmed locally (round 1)
- Title: START / CONTINUE cursor. START → "SELECT 1PLAY OR 2PLAY". Route used for most
  scenarios (power-on, 360 frames, START, START, A) reaches "SELECT YOUR LEAGUE"
  EASTERN/WESTERN → team → "STARTING LINEUP" menu (PLAYBALL / PLAYER CHANGE / LINE UP) →
  "STARTING PITCHERS" (A picks) → "EASTERN LEAGUE GAME 1  N.Y. VS PHILLY" card → scoreboard →
  game. Other observed screens: "SELECT YOUR TEAM" letter row (1L K O T H U), 2PLAY →
  "INPUT PLAY DAY 01/01/89" → 2-player team select (1N B J P M D / 2L K O T H U).
  Exact button semantics on these screens were not fully mapped (see scenarios.json).
- In the default route the player's team (N.Y.) is home and pitches first. Pitching: A
  throws (d-pad selects pitch); a hit switches to a field view where the d-pad moves the
  controlled fielder; play resolves when the ball is fielded/thrown (A+direction).
- RAM notes (round 1, partially verified):
  - $0051 = current 16 KiB PRG bank written by MMC1_WritePrgBank ($8566); $0050 = CHR bank
    value for $8588; $0054 = far-call entry index*2 for FarCallBank ($84BC).
  - $05E5 = runs scored on the current play (added to per-inning score arrays via pointer
    table $D26C and to $06C6); $05E6/$05E7 track team score (both 1 then 4 after the
    observed scoring plays). $0066 = half-inning counter (0 in the 1st inning top).
  - $0354 is drawn on the B/S/O display row but writing 2/3 did NOT force a side change
    (it cycles 0..2 during at-bats) – the outs/inning contract is still unknown.

## Open / next experiments
- Find the real outs/half-inning contract (search code around $D200-$D270 bank 15, and
  writers of $0066) to shortcut to batting, late innings, game end and season results.
- Try CONTINUE (password) and 2PLAY flows further; Western league teams.

## Round 1 redirect (r1-redirect-a) - progression RAM contract confirmed locally
Artificial-state experiments (debugger RAM writes; not natural reachability). Inputs: only
`tap('a',4,56)` repeated unless noted; scripts scratch-only, recipes in analysis/scenarios.json.
- Outs/inning contract (L-003): StrikeCount $0354, BallCount $0355, OutCount $0356,
  HalfInning $0066 (bit0 = bottom), ScoreTable $05E6 = visitor total, TotalRunsB $05F3 = home
  total, SeasonDay $0610, SeasonWins $0616, FrontEndScreen/Step $0605/$0606, PlayState $41.
  From S5.snap (2 outs, full count, top 1st) plain pitching produced a strikeout and a natural
  side change: HalfInning 0 -> 1, PlayState $65 -> $20, SysFlags bit0 = 1 (human bats).
- Batting (bottom 1st) visited: PlayState $21-$25/$31/$32 cycle, strikes/balls counted.
- Game end: patch HalfInning=$10 (top 9th) and $05F3=$0A (home leads) before pitching; after
  the third out PlayState $62 -> $70, HalfInning $11, SeasonWins +1, FrontEndScreen $41 ->
  $20/$22/$24/$25/$27 (result, "NEXT PASSWORD" 7-letter screen), then SeasonDay +1 and the
  biorhythm screens ("BIORHYTHM ... GAME002", Sports News "Biocondition"), next game.
- Mercy rule: with HalfInning >= 9 and a run difference >= 10 the game ends at once as
  "CALLED GAME" (observed after patching HalfInning $10 / home 10-0 in a post-season game).
- Season end: patching SeasonDay=$81 at game end makes the next day $82 and starts a game vs
  L.A. (post-season series, SeasonDay >= $82).
