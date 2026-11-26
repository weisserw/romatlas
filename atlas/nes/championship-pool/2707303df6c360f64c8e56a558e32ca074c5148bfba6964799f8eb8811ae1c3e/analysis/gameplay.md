# Gameplay research

## Identification (confirmed locally, round 1)

- Title screen: "Championship Pool", Billiard Congress of America presents;
  (c) 1993 Mindscape Inc. / Software Toolworks; (c) 1993 Bitmasters Inc.
  Boot shows a "Bitmasters" logo, then staff credits, then the title (attract loop).
- iNES mapper 2 (UxROM), 8 x 16 KiB PRG, CHR-RAM, vertical mirroring. NTSC USA release assumed
  (manual is NES-P3-USA); revision not otherwise established.

## Sources

- NES instruction manual scan (NES-P3-USA), gamesdatabase.org:
  https://www.gamesdatabase.org/Media/SYSTEM/Nintendo_NES//Manual/formated/Championship_Pool_-_1993_-_Mindscape.pdf
  (read pages: contents, controls p.2-3, special features p.8-9). Applies to the NES USA version.
- Retro Replay description (power/spin gauges, ghost ball): https://retro-replay.com/db/nes/championship-pool
- The archive.org manual text found by search is for the SNES version (different buttons); not used.

## Controls (manual, partly confirmed)

- Start: begin / open in-game Main Menu (confirmed: SHOW NUMBERS, GAME CONTROL, ACTIONS, BACK TO GAME).
- Select tap: change table view (confirmed: whole-table overview with GAMES / mode header).
  Hold Select + A: call shot / move balls (Freestyle); Select + B: spin; Select + pad: move camera.
- A: activate phantom ball; after aiming press A again to shoot (manual). NOT yet confirmed:
  repeated A presses in round 1 produced phantom/cursor animation but no observed ball motion.
- B tap: power meter panel (confirmed panel opens); hold B + pad: fine aim.
- Game Control menu: SET POWER (meter; left/right), SET SPIN (cue-ball image), ZOOM IN/OUT, JUKEBOX
  (BG MUSIC ON/OFF). Actions menu: SPECIAL ACTIONS (CALL A SAFETY), VIEW REPLAY, SCOREBOARD,
  LEAVE TOURNAMENT. All visited in round 1 (see analysis/scenarios.json).

## Modes / progression (manual contents + local screens)

- Game Mode menu: TOURNAMENT, CHALLENGE, PARTY, FREESTYLE (confirmed screen).
- Tournament: Break type (AUTO/MANUAL) -> Tournament games (8 BALL / 9 BALL) -> "BCA Regional
  Qualifying Tournament, 32 contestants, single elimination; winner goes to World Championship"
  -> coin flip (HEADS/TAILS) -> table (confirmed).
- Manual lists games: Three Ball, Eight Ball, Nine Ball, Ten Ball, 14.1 Continuous, Rotation,
  Fifteen Ball, Straight Pool, Equal Offense, Speed Pool (presumably Party/Freestyle; unconfirmed).
- Later progression (winning matches, World Championship, ending) not reached in round 1. The RAM
  contract for tournament round/opponent is unknown; candidate for a redirect investigation.

## Round 1 redirect findings (confirmed locally, see analysis/scenarios.json S8-S13)
- Shooting: A (aim -> confirm, GameCmd 1->2) then A again fires. In call-shot games (8-ball after the break) a shot
  without a called pocket is refused with an error sound; call with Select+A first. Break shots need no call.
- Opponent-turn messages show a portrait screen ("TimeWarp" header, opponent name from bank 3; 11 opponents
  JOHN DOYLE .. JOE NOZERAK).
- Tournament: 10 matches; after 5 wins the BCA World Championship (Bloomington, Indiana) starts. Winning round 10 gives
  "CONGRATULATIONS YOU WON THE TOURNAMENT" and an interview by Joe Nozerak (A shows answer choices, Start continues),
  ending "... WISHING YOU A GOOD NIGHT", then back to the title.
- RAM: TournamentRound $EF, CurrentPlayer $E6, MatchExitCode $DD (2/3 leave match), GameCmd $D2, CalledPocket $D9,
  GameType $DA (party NINE BALL = 1; Freestyle = $13), WorldChampUnlock $EE.
