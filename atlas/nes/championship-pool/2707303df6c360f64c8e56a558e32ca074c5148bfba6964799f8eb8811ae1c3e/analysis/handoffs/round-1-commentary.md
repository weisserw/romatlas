# Round 1 - commentary handoff

Status: complete (current-source readability pass over all classified source; final editorial
audit intentionally not run - review.json editorial_audit stays pending for final acceptance).

## Verification
- `python3 /root/romatlas/scripts/commentary.py verify <workspace>`: ok true, byte_identical true
  (commentary/verification.json), after the final build.
- Reading copy refreshed: commentary/reading-after.asm (`commentary.py read`).
- Baseline unchanged: maps/current.json == pass-001-final; commentary prepared from it.

## Covered scope
- All code: banks 2 ($B1D3-$BF79), 3, 5, 6 ($B153-$BB47), 7; 20,698 instructions, 624 regular
  labels, all descriptively named with a routine header; all data/padding/unknown ranges noted.
- Source: commentary/source/game.asm + ram.inc + bank0..7.asm, rendered by
  commentary/tools/build.py from before/game.asm + commentary/tools/annotations.txt (edit record;
  hash guard against overwriting hand edits).  review.json refreshed by tools/review.py:
  1077/1077 scaffold regions reviewed, 518 units reviewed.

## Readability assessment
- Good: fixed-bank engine (reset, bank switching, NMI, VRAM ring buffer, RLE/tile helpers,
  palette fades, controller debouncing, sprite builder, menus/text), sound driver and screen-
  script interpreter (command sets documented), menu-script interpreter, physics contact
  prediction / ball-ball response (algorithm-level block comments), 8-ball rules section.
- Adequate but shallower: rules sections for other game types (section-level comments),
  bank-3 title/selection/preview frame hooks, aiming/input handlers, scoreboard; ~1,180 branch
  targets keep generic @skipN names; 139 low-use RAM addresses keep address names.

## Key names (examples)
SetPrgBankFromA / SelectBankN(_Temp) / RestoreSavedBank / FarCallViaZp04; VramBufBeginPacket /
VramBufPut / VramBufCommit / VramBufDrain; DecodeRleTiles; FadeIn/FadeOut/StepPalette;
PollControllers (2-of-3 debounce); BuildSprites; SoundStart / SoundUpdate / SoundTrackStep;
RunScreenScript; RunMenu / DrawMenuBody / MeasureMenu; MainProgram / PlayMatch / GameFrame;
StartTournament/Challenge/Party/Freestyle; PredictPairContact / BallBallCollision /
PredictPointContact; Mul16 / Div32 / Sqrt32; EndOfShotRules (@game_XX sections, RuleCase_*).
RAM: CurPrgBank $1E, SavedPrgBank $5C, SysFlags $1D, VramBuf $0200, OamBuf $0300, PalBuf $0400,
GameCmd $D2, GameMode $D4, GameType $DA, BallPosX/Y $06D2/$06F4, BallVelX/Y $7B/$9B, phantom
ball = ball index 16 ($20 in word arrays), FrameHookAddr $0435-$0437.

## Unresolved meanings (details in commentary/uncertainties.md)
- GameType numbers other than 0 (8-ball) / 1 (9-ball); most GameCmd values; ShotFlags/$C3/
  $D8/$D9/$E1-$E3 bit meanings; physics constants and bank-4 table contents; screen/menu script
  data blocks not split; installers of frame hooks not traced.
- Shared RAM overlays ($052E-$05AD, $0488-$04B7) named per bank; lifetimes untraced.

## Structural questions for redirect (see evidence-policy.md)
1. Bank 3 $832B-$866B (L-002): looks like a frame hook. Bounded check: search bank-6 screen
   scripts for op 05/06 writes to $0435/$0436 with value $832B (or other unreferenced bank-3
   addresses $8326/$8699/$86C5/$9385/$93AE/$93BD/$94A4/$94D2) to establish installers; those
   would also supply the entries for the unknown block.
2. $E3A3-$E3E6 (L-005) and bank 3 $8A51-$8B13 (L-004): caller/callee pair; a trace of the
   tournament chart screen (DrawTournamentChart path, trampoline $F885) could confirm both.
3. GameType mapping: tracing Party/Challenge setup menus (RunMenu result -> GameType) would name
   the rule sections (@game_02..@game_12).
4. RunMenu full-screen path reads $2000/$2001: confirm at runtime what address results (open
   bus) - affects only commentary.

## Toolkit issues
None new (analysis/toolkit-issues.md unchanged).

## Disk
commentary/ 8.3 MB; no scratch left; ~2.6 GB free.
