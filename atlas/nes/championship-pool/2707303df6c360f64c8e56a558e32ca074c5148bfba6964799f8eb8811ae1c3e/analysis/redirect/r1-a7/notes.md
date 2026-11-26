# Redirect r1-a7 (round 1) - research notes

Input checkpoint: pass-001-final (maps/pass-001-final.json, byte_identical). Map/state/source untouched.
Server: `python3 run.py serve` via analysis/tools/rpcd.py daemon, scratch /tmp/ra-2707-r1a7 (env ROMATLAS_SCRATCH;
rpcd.py/nes.py now honour it, default unchanged). Daemon killed by its own PID at the end.

## Tools (this dir)
- scripts.py: sequential decoder of bank-6 screen scripts (RunScreenScript command lengths:
  01:3 02:1 03:3 04:2 05:4 06:3 07:5 08:3 09:3 0E:4 10:3 12:1 13-17:0 18:1 19:1 1C:3 1E:0 1F:0, 00 end).
  Output for $BB48-$BE8D: bank6-screen-scripts.txt (no unknown command bytes; $BE8E.. is $FF padding).
- swalk.py: static walk (analysis/tools/walk.py) from roots, report unknown bytes covered / conflicts.
- cov.py: dump analysis.contexts to JSON and list hits in unknown gaps. cmp.py: observed vs static per gap.
- lab.py: commentary label lookup by bank/cpu.

## Avenue 1 - screen-script entries (commentary Q1, Q5) -> RESOLVED
Decoded scripts give every FrameHookAddr installer and far-call entry:
- $BB48 (logo): hook $8326 ClearPressedHook. $BB6A: far call $832B bank 3 (L-002 entry).
- $BB92: far call $830E bank 3; $BBB4: far call $831A bank 3 (both inside L-001 $830E-$8325).
- $BBCA (mode menu screen): far call $8699 (TitleSpritesHookInit), hook $86C5.
- $BC45/$BC70/$BC86 (open-setup/open/close of MENU_MAIN_MODE $A2CA via menu op 08): far call $94A4
  RackPreviewHookInit, hook $94D2 RackPreviewHook  => preview hook installed by the GAME MODE menu (Q6).
- $BCA3 / $BCB8+$BCE8 (selection menu $A317): hook $93BD, far calls $93AE, $9385.
- $BD67/$BD9D (coin toss menus $A292/$A3A8/$A3E8): hook $BA27 bank 6 (CoinTossHook).
- $BDF9/$BE32/$BE3B (11 bank-2 opponent-turn messages, e.g. $9A6E, menu op 08): far call $F885 bank 7, $F899 bank 7.
- $BE46 (ShowIntermission $95E7): far call $95F6 bank 3.
- TitleSkipWord ($04D4/$04D5) writer: the $832B routine (sta at $835A/$835F, dec at $8458/$845B) - Q5 answered.
Single-occurrence check: each target value occurs exactly where decoded; no conflicting references.

## Avenue 2 - runtime confirmation
- Boot frames 140-1340 traced (no edits): observed $830E-$8325 (12 starts, ctx [6,7,14,15]), $832B-$8644
  (355 starts, 767 bytes; all on static walk), $FC65-$FC78. observed-boot-140-1340.json.
- Opponent-turn message (after RAM patch $D9=0 for the 2nd shot): $F885-$F8AC all 22 starts, bank-3 $899E-$89C0,
  $8A51-$8A74, fixed $E3A3-$E3E1 (32/36 starts). observed-oppmsg.json. Screen: TimeWarp header, portrait,
  "JOHN DOYLE", "YOUR OPPONENT SANK SOME BALLS, THEN MISSED. NOW IT'S YOUR TURN."
- Intermission (artificial GameCmd $D2=$19 at aiming): bank-3 $95F6-$981F 130 starts; SCOREBOARD screen.
  observed-intermission-artificial.json.
- Static walks agree with every observed start (0 observed-not-static).

## Avenue 3 - table extents (L-002/L-003/L-004/L-005), checked with consumers
- $8645-$8664: 4 x 8-byte records; consumer $848E-$84BB reads fields +0..+7 with x=Y*8; callers $83F6-$8407 use Y=0..3.
- $88A8-$899D: 8-entry tile-pointer, rect-pointer and 4-byte palette tables; 11 name pointers that point exactly at
  the 11 zero-terminated names $88FE-$8992 (11 terminators, last byte $8992); 11-byte opponent->portrait table
  $8993-$899D with values 0..7 (matches the 8-entry tables). Indexed by TournamentRound $EF.
- $8A76-$8AFF: header 16x8 + 128 tile bytes + 8 bytes (DrawTileRect format; no reference found; generic data).
- $8B00-$8B13 PanelLayout read at $8BE6.. ; $9820-$987F 12 x 8-byte big-digit records (y=digit*8 at $F9EC);
  $9880 sprite tile (lda $9880 at $9D1D); $977D/$9790 19-entry GameType-indexed bytes; $97A3 8 words.
- $E3E2-$E3E6: reset signature compared/copied by Reset ($E41F/$E485, x=4..0).

## Avenue 4 - shot input and progression (L-007) -> RESOLVED
TableControlInput: A in GameCmd 1 -> 2; A in GameCmd 2 -> LaunchShot unless GameTypeFlags[GameType] and
CalledPocket<0 and GameOptions bit1 clear (error sound). So 8-ball needs a call (Select+A) except on the break.
StartTournament: TournamentRound $EF 0..9, round 5 = World Championship stage, $0A -> TournamentComplete.
MatchFrameLoop returns on MatchExitCode $DD 2/3; @won_match needs CurrentPlayer $E6 != 1.
Shortcut: at aiming write $EF=n-1, $E6=0, $DD=2. Reached round 5 and the ending (scenarios S11/S12).

## Avenue 5 - small fragments (L-006) -> families documented in leads.json L-006.
$D9B0 (txa) and $DE1C (pha) are one-byte prologues falling into UploadPalette / VramBufFlushNow; no reference.

## Not pursued / deferred
- RunMenu $2000/$2001 reads (Q4): commentary-only, no classification impact.
- Scoreboard wheel meaning (Q6b): commentary only.
- Ending sequence trace: no unknown bytes involved (all bank-3 unknowns now explained); untraced visit only.
