# Gameplay research

## Identification

- Internal string at file offset 0x3fff0 (bank 31 tail): "BASES LOADED 4". Title
  screen (locally observed): "Bases Loaded 4", "TM AND (C) 1993 JALECO", "LICENSED BY
  NINTENDO". North American release; MMC3, 256 KiB PRG / 128 KiB CHR.
- Sources (consulted 2026-11-18):
  - https://en.wikipedia.org/wiki/Bases_Loaded_4 — Japanese title "Moero!! Pro Yakyuu
    (Saikyou Hen)"-family; US ending is a one-screen ending; JP has staff credits.
  - Web search summary (manual mirror https://64scener.com/manuals/basload4.txt was
    403 for automated fetch; unconfirmed): password given after every regular season
    game (10 characters); season of up to 130 games; a playthrough review says the
    season can be clinched early (~74-80 wins, sources disagree).
  - Uncertainty: season length/win target and password format are external claims,
    not yet confirmed locally.

## Locally confirmed controls/menus (round 1)

- Controller is read by fixed-bank routine at CPU $CAC3 (bank 30) into zero-page $32
  (port 1) / $33 (port 2). Title screen advances on Start.
- Title -> PLAY / OPTIONS. PLAY -> REGULAR SEASON / EXHIBITION. EXHIBITION -> 1P / 2P /
  WATCH. WATCH -> team select (COMPUTER1 vs COMPUTER2; 12 city teams) -> "X vs Y"
  stadium intro -> batting order screens for each team (press A on READY) -> game.
- WATCH mode is CPU vs CPU and plays a whole game without further input; useful for
  broad engine coverage (pitching, batting, fielding, base running, scoreboard).
- Top-level mode byte appears to be zero-page $2E (bit 7 = "initialize/transition"),
  sub-mode $2F; dispatch via the inline jump table after `jsr $C734` (static, see
  analysis/investigation.md). Hypothesis only; not yet used as a RAM shortcut.

## Hypotheses / routes to test

- Regular season with passwords may give a shortcut to late-season/ending states;
  password entry visited in round 1 (see below). A RAM edit of season win counters is a
  candidate shortcut once the season state bytes are identified (deferred).

## Round 1 confirmed observations (local, 2026-11-18)

- Input: the NES adapter needs `emulator.input {buttons:[...],port:1}` (TK-001).
- PLAY -> REGULAR SEASON -> START/CONTINUE. CONTINUE opens ENTER PASSWORD
  (10 characters; up/down change a character, left/right move, A when done). An invalid
  code shows INVALID PASSWORD with TRY AGAIN / GO BACK TO START (confirmed).
- REGULAR SEASON START: SELECT TEAM (PLAYER vs COMPUTER), then "GAME 1 / NEW YORK vs
  BOSTON" intro. The batting-order screen allows swaps (pitcher Lewis<->Bonds confirmed).
- OPTIONS: MUSIC ON/OFF, DH ON/OFF, EXIT (confirmed).
- Start pauses during play (confirmed).
- RAM (hypotheses supported by bounded experiments): $2E = top-level mode (02/01 in
  play, 03 scoreboard; bit 7 = transition), $2F = sub-mode ($0B final scoreboard).
  $037B = half-inning counter, 0-based (2*(inning-1) + 1 for bottom). Writing $0F in
  the bottom of the 6th made the game end after the top of the 9th with the home team
  leading (counter $10 -> $12, final scoreboard with X). This is an artificial shortcut.
- A full CPU-vs-CPU WATCH game runs about 50k frames.
- Not yet visited: late season, season end, the one-screen US ending, 2P mode
  (leads L005 and S10).

## Round 1 redirect (r1-a, 2026-11-18): season end, ending, 2P (local, confirmed)

- Season RAM contract (from source U20_SeasonProgress / U18_PennantWaitStart, exercised):
  SeasonGame $66 (games played; $C8+ = Super Series), SeasonWins $042D, SeasonLosses $042E,
  AwayRuns $0386, HomeRuns $0391, PlayerTeam/AwayTeam/HomeTeam $0381-$0383. The final
  scoreboard can be entered with $2E=$83/$2F=$0B (counts the result on Start).
- Regular season: 70 wins -> "LEAGUE CHAMPION <team>" cut-scene; games - wins >= 61 ->
  GAME OVER (byte arithmetic: wins larger than games also triggers it). Start on the pennant
  scene starts the SUPER SERIES (SeasonGame $C8) and shows its password
  (000R6CB640 for New York in the tested artificial state; natural entry untested).
- Super Series: 4 wins -> "SUPER SERIES CHAMPION <team>" cut-scene, Start -> title. This is
  the US terminal screen (consistent with the external "one-screen US ending"); no credits.
- GameMode 3 sub-mode 19 (U20_InitEnding) shows a Japanese "祝 ペナントレース 優勝" scene that
  nothing in this ROM selects (artificial visit only).
- EXHIBITION 2P: both controllers advance the team/lineup screens; game runs with
  PlayerTeam $FF (confirmed, natural route).
- Details/routes: analysis/scenarios.json S09a-S09e, S10; analysis/redirect/r1-a/handoff.md.
