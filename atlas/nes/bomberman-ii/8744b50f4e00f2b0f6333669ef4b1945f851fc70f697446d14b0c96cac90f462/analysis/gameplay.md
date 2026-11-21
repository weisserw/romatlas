# Gameplay research

Game: Bomberman II (NES, Hudson Soft 1992; build string "BOMBERMAN2 92817" at fixed-bank $FFE0).
Mapper 1 (MMC1), 128 KiB PRG, CHR RAM.

Online sources: not consulted in round 1 (local RAM/code analysis was sufficient for shortcuts).
All findings below are confirmed locally on this ROM unless marked otherwise.

## Controls / menus (confirmed)
- Intro story plays from power-on (~1700 frames) then title "PUSH START!"; Start -> mode menu:
  NORMAL MODE / VS MODE / BATTLE MODE / CONTINUE (cursor Up/Down, Start selects).
- CONTINUE opens "INPUT PASSWORD!" grid (8 chars from 1-8,B,C,D,F,G,H,J,K; arrows, DEL, END).
- NORMAL MODE: truck cutscene -> "AREA w-s" card -> stage (timer 200/300, "LEFT n" lives).
- A = place bomb. VS mode: two-player arena (P1/P2 score, T:298).

## RAM contract (confirmed by code + experiment)
- $4B = world index 0..5 (6 worlds; $CA53 inc/cmp #6), $4C = stage index 0..7 ($CA47 inc/cmp #8).
  Poking $4B/$4C during the truck cutscene (snapshot `truck`) selects the AREA card and stage
  (e.g. $4B=2,$4C=3 -> "AREA 3-4"). Stage 8 ($4C=7) uses a wider scrolling arena (timer 300).
- $04E5 = lives remaining (set 2 at $C97F; dec at $CAC4, bmi -> game over $D07C).
- $49 = game mode (0 normal; nonzero vs/battle paths at $CA6F).
- $054F nonzero at the $C946 loop -> bonus-stage path ($CCB7 far-call bank 5 $9FC0; "BONUS STAGE" text bank 10 $9FB1).
- After world 6 completes the code calls $D1A2 (far-call bank 5 $B07D = ending) then jmp $C003.
  Artificial route: from snapshot `area11`, run until PC=$C27D, set PC=$CA5B -> ending cutscene (two Bombermen walking).
- Joypad: $04B5/$04B6/$04B7 raw (P1, P2, expansion), read in NMI only when $11==1 (main loop WaitForNmi).

## Input API note
Use `emulator.input {"buttons":[...],"port":1}` (TK-001); `button/pressed` is silently ignored.

## Hypotheses (unconfirmed)
- Bank-11 routine $A94A (no caller found) maps P2 pad $04B6 (B,A?,...) to $04C9, which $C988 uses as the starting world
  ($04C9 >= 0 -> $4B). Possibly a hidden world select; test by holding P2 buttons on the menu (lead L-005).
- Online manual/FAQ research not performed in round 1.

## Round 1 redirect additions (confirmed locally, attempt analysis/redirect/r1-a1)
- Special passwords (SpecialPasswords bank 5 $9703 hold ASCII names, matched on raw entry digits through PasswordDigitMap):
  K3456712 = sound room ("PCDEFGAB"), K138BHDJ / K1HB3G1H = carry remote ("PACHINKO"/"PANICMAN"),
  KJH5C13D = bonus stage ("PONEJACK"), K2JG2135 / K2JG2G1H / K2JG2JF4 = bomb fuse $2D/$4B/$69 ("PBOMBACE"/"PBOMBMAN"/"PBOMBOLD").
  Entered with the grid: A enters the cell under the cursor (rows: 1-8, B C D F G H J K, arrows/DEL/END), Start finishes. Sound room and bonus stage verified.
- Sound room: Up/Down row, A/B change value, Start plays, Select stops; PCM row plays DMC samples $1E-$2A.
- Battle mode needs a Four Score ($48 FourScorePresent, re-detected every NMI); with 2 ports the menu refuses it (sound $04).
- Game over: Lives $04E5=0 then die -> GAME OVER with CONTINUE/END and the generated password.
- Ending continues into a STAFF credits roll and an END screen (~6500 frames after the ending starts).
- No boss enemies in normal-mode stage lists; scripted actors are used by the opening cutscene.
