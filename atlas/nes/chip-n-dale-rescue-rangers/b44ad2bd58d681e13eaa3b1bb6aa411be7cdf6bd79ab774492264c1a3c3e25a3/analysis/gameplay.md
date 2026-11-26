# Gameplay research

Game identified locally from the title screen (round 1 disassembly pass 1):
Disney's Chip 'n Dale Rescue Rangers (Capcom, NES, MMC1, 128 KiB PRG / 128 KiB CHR,
NTSC, English title text "CAPCOM U.S.A., INC." / "NINTENDO OF AMERICA INC.").
Exact revision is not established from external sources.

## External sources (hypotheses until confirmed locally)

- Data Crystal RAM map: https://datacrystal.tcrf.net/wiki/Chip_%27n_Dale_Rescue_Rangers/RAM_map
  Lists $54 (Chip/Dale action), $5E (invincibility), $E4 (LFSR), object tables
  $0400-$058F (16 slots each). No level/zone variable listed. Not yet confirmed locally
  except that the object tables match the $04D0/$04F0/$0510 writes in the map routine
  at bank 2 $8B00.
- TASVideos user files exist (https://tasvideos.org/UserFiles/Game/30); not consulted further.

## Locally confirmed controls/inputs (tested)

- Debugger input API for NES: `emulator.input {"port":1,"buttons":[...]}` sets the whole
  pad state (the documented `button`/`pressed` form is ignored on NES; see toolkit-issues).
- Joypad routine at fixed $C162 stores new-press bits in $14 (P1) / $15 (P2) and held
  bits in $16 / $17. Start = $10, Select = $20 in $14.
- Title: Start opens 1P/2P menu, Start again selects CHIP/DALE, then intro dialogue
  (A/Start advance text). Gameplay starts in Zone 0 ("zoneA_start" snapshot, frame ~515).
- In gameplay: Right/Left walk, A jump, B pick up / throw crate, Start pauses.

## Locally confirmed RAM/progression contract (tested)

- $31 = current level / map node. Values 0-10 are action zones, 11 = first zone map
  (A-G), 12 = second map (H-I-J), 13 and 14 load further distinct screens (13: blue
  industrial/window stage; 14: stone/rubble stage, roles not yet identified).
- $37 = next level; $4E/$4F = 16-bit countdown. When it reaches zero the main loop at
  $C7B0 copies $37 to $31, runs the bank-2 cutscene $9000, the bank-2 zone map $8B00,
  and restarts the level via $C60E. Shortcut A: write $37=N, $4E=1, $4F=0.
- Shortcut B (used for the level sweep, artificial): run until PC=$C77B (main loop top,
  bank context [x,x,14,15]), write $31=N, set PC=$C60E (level start entry jumped to by
  the main loop itself at $C7EA). This loads level N directly; observed for N=0..14.
- Map screen (bank 2 $8B00): $38/$39 hold cleared-zone bit flags; nodes are entered
  automatically when the cursor arrives at an uncleared node.
- Main thread is a cooperative task (scheduler $FE90, tasks at $80-$8F, yield $FF04).
  $FF53 selects the 16 KiB PRG bank at $8000 (A = 16 KiB bank number).

## Boss / ending contract (round 1 redirect r1-a1, tested artificially)

- Door warp: PendingWarp $6C = section id (door tiles set it from the level's LevelScreenFlags
  low 5 bits); writing $6C during gameplay runs EnterSection next frame. $19 = level exit.
  Door ids per level and the boss-room sections ($0A, $16) are listed in
  analysis/redirect/r1-a1/notes.md section 3.
- Zone 0 boss: $6C=2 then walk right (~240 frames) -> boss types $4C/$4D (S12).
- Final boss Fat Cat: level 10 (Shortcut B) then $6C=$16 -> boss room (S14, frame l10boss.png).
- Defeat: BossHits $EE=5 and BossSlot $EF = boss body slot -> level clear.
- Ending: level 10 cleared -> ending dialogue -> "THE RESCUE RANGERS! - END -" (S13; frames
  analysis/redirect/r1-a1/end1.png, end2.png). No separate credits roll observed.
- Select during play opens a pause screen (bank 2 $8900); A+B there opens a sub-screen ($A597)
  (S10). Invincibility: $5E=$FF.
