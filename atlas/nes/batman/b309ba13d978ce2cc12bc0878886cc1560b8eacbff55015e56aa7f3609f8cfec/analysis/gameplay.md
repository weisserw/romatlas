# Gameplay research

## Identification
- Local evidence: boot screens read "TM & (c)1989 DC COMICS INC. / LICENSED BY NINTENDO OF AMERICA INC. /
  SUN CORPORATION OF AMERICA / (c)1989 SUNSOFT"; title "BATMAN ... PUSH START" (scratch frames, round 1).
  Identified as Sunsoft's *Batman: The Video Game* (NES, North American release), mapper 4, 128 KiB PRG/CHR.
- Staff text in ROM (bank 5, ~0xAF00): SUZUKI KEIICHI, ... "SPECYAL THANKS" (ending credits).

## Sources
- Data Crystal RAM map (stub, last edited 2024-01-28):
  https://datacrystal.tcrf.net/wiki/Batman:_The_Video_Game_(NES)/RAM_map
  Lists: $A6 weapon ($00 fists, $50 Batarang, $60 Ninja stars(?), $C0 gun), $B7 health, $B8 ammo, $BE lives,
  $C0-$C5 CHR banks, $D0-$D7 Batman position, $F5-$F8 joypad, $0540 object action IDs, $0580/$0590 obj X,
  $05A0/$05B0 obj Y. Version not stated; treat as hypotheses. $B7 (health, 8 = full) and $BE (lives) confirmed
  locally (values change on death/HUD).
- No stage list / stage-select / debug feature found on that page. TCRF page not consulted (round 1).

## Locally established RAM contract (round 1, static + runtime)
- $88 = area index * 2. Area-start code at $DCCD..$DF35 (bank 14) reads $88>>1 to index tables:
  $E0CC (area PRG bank register value: $18 x3, $19 x4, $1B x4, $16 x4, $17 x3, $22,$22,$21...),
  $E103 (area flags; nonzero values 01..05 at idx 3,7,11,15,17 look like boss/stage-end markers),
  $DF1C (-> $E8).
- $CF = area result. At $DE5D: bit7 -> life lost path ($DE70), bit6 -> area cleared ($DE7D: $88 += 2;
  $88==$20 with $76 bit7 -> $22; $88==$24 -> ending branch at $DE9D, which maps PRG reg values $12/$13).
- Writing $88 then forcing a death ($B7=0) restarts at the patched area (supported death/restart transition),
  but CHR bank variables are not re-initialised across stages, so later-stage graphics can look wrong.

## Controls (observed locally)
- A pause screen (SCORE/REST) appeared after a scripted Select press in round 1 (cause not isolated; Start may also pause). A = jump, B = attack assumed, not individually confirmed.

## Confirmed shortcuts (round 1)
- Stage transition: from stage-1 gameplay write $88 = 2*(k-1) and $CF = $40; the game's clear path runs the stage
  cutscene and initialises area k with correct graphics (k=3,7,11,15,17 confirmed visually). See analysis/scenarios.json S3.
- Ending: $88 = $22, $CF = $40 -> ending cutscene and staff credits (PRG banks 2/5). S4.
