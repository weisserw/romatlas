# Redirect r3-p1 (round 3) - commentary questions, FontTablesTail, Challenge shortcut

Input checkpoint: pass-003 (maps/current.json = analysis/state.json, byte_identical; 100.00% classified, 0 unknown).
Map/state/commentary source/ROM/toolkit untouched (state.json mtime 2026-11-25 23:35, before this attempt).
Server: `run.py serve` via analysis/tools/rpcd.py daemon, ROMATLAS_SCRATCH=/tmp/ra-2707-r3; its own PIDs killed at
the end; scratch removed. All runtime results below except the plain SET POWER traces are ARTIFICIAL-STATE.

## Tools (this dir)
- tailxref.py -> tailxref.json: static references into bank-7 $CDAE-$D0C3 (abs/indexed operands incl. 256-byte
  indexed reach, flow targets, #imm pairs, 16-bit words in all Data regions) + linear-decode stats.
- run_r3.py: `base` (boot -> S8 route -> snapshot aim0, tournament round 0, GameCmd 1, no edits) and
  `exp win0|match0|match1` (GameCmd $D2=$12 shortcut; RecordGameWin; run_until PC $B1F9 bank 3; captures).
- run_power.py -> power-panel-reads.json: SET POWER panel (press B at aim0, no RAM edits) with ROM Read traces.
- run_challenge.py -> challenge-route/unlock/finish/result.json: Challenge Speed Pool shortcut.
- crop.py: PNG crop/scale helper used for the portrait crops.

## Q1 PortraitFrame 1/2 - RESOLVED (artificial, visual)
Method: GameCmdDispatch case $12 (bank 7, `jsr SelectBank3; jsr RecordGameWin; GameCmd=1`) is an existing in-game
command that runs the game-won path. From aim0 write $D2=$12 (plus, per case, CurrentPlayer $E6 and PlayerWins
$0457/$0458 = race-1 = 3; MatchSetting1 $E1 = 4).
- win0 (exp-win0.json, sheet-win0-entry0.png, portrait-frame1-win0.png): PortraitFrame=1; random comment window;
  after ~30 frames ShowPortraitExpression redraws the portrait: the neutral face changes to a scowling/frowning face.
- match0 (exp-match0.json, sheet-match0-stalks-off.png): comment "YOU HAVE WON THE GAME. YOUR OPPONENT
  CONGRATULATES YOU" (natural LFSR value $AC -> &$1F=$0C, &3=0 -> 1 -> entry 1 $9BE3), intermission "1: 4",
  then "YOU WIN THE MATCH. YOUR OPPONENT STALKS OFF, MUTTERING SOMETHING ABOUT A WARPED STICK AND BUMPY FELT."
  with PortraitFrame 1 (scowl), MatchExitCode 2, next FLIP FOR BREAK (round 2).
- match1 (exp-match1.json, sheet-match1-opp-wins.png, portrait-frame2-match1.png): CurrentPlayer 1: no comment;
  intermission "2: 4"; "YOUR OPPONENT WINS THE MATCH." with PortraitFrame 2: the face changes to a broad
  grin/smile; then ELIMINATED and return to the title.
Conclusion: frame 1 = displeased (opponent lost), frame 2 = grinning (opponent won). The commentary's tentative
PORTRAIT_OPP_LOST / PORTRAIT_OPP_WON meanings are visually confirmed (artificial entry via GameCmd $12; the
displayed screens and portrait redraws are the game's own code after that write).

## Q3 RandomMessageTable entry 0 - CONFIRMED (artificial)
exp-win0.json: run_until PC=$B1F9 (bank 3 `lda RandomLo` right after `jsr Random`; 3,119 operations), natural
RandomLo $AC, forced $A0 (&$1F=0); run_until $B226 (`jsr ShowMessageB`): Ptr04 = $9C2F = RandomMessageTable
entry 0. Screen: "SO, YOU THINK YOU'RE GOOD NOW, EH? THINK THAT THIS IS GONNA BE A PIECE OF CAKE, EH? WELL, I'LL
SHOW YOU! I'LL SHOW YOU ALL!!! HAHAHAHAHA-COUGH-...-GAG-...". The round-3 commentary correction stands.
RandomMessageTable words: entry0 $9C2F, 1 $9BE3, 2 $9B4E, 3 $9B9D.

## Q2 FontTablesTail $CDAE-$D0C3 - NEW CONSUMER AND EXACT SUB-STRUCTURE (data role unchanged)
Static (tailxref.json):
- No absolute operand, flow target or (meaningful) #imm pair inside the range. The 12 #imm-pair hits are
  coincidental (bank-3 lo/hi bytes of different script symbols; $F41F `lda #$cf` is an attribute mask). The
  BigFontTop/BigFontBottom indexed reads (char-$41) cannot reach it for letters A-Z.
- Linear decode: 486 "instructions", 148 unofficial, 272 BRK, 340/790 bytes $00: non-code. No code evidence
  contradicts the data classification.
- **Consumer found**: 16-bit word $CDC3 at phys $C038 = bank-3 WindowTileSets record 3. WindowTileSets is 7 records
  of 4 bytes (source pointer, PRG bank, 0) - not 3-byte records as the bank-3 header comment says. Record 3 =
  ($CDC3, bank 7 = fixed bank, no switch). OpenWindowBanked loads it into WinTilesPtr/WinTilesCount when
  WindowFrameTable record 1 (bytes 03 04 FF FF: tile set 3 at $1000, tile set 4 at $0C00) is opened; OpenWindow
  uploads it with QueueTileUploadAndWait (first byte = tile count, then 16-byte tiles).
- Extent: count byte at $CDC3 = $30 (48 tiles) -> 1 + 48*16 = 769 bytes = $CDC3-$D0C3, ending exactly at the last
  byte before PlayMatch ($D0C4, code). Independent constraints: encoded count, consumer contract (UploadTiles),
  code boundary. All 7 WindowTileSets records were checked: each count-prefixed block fits inside its existing Data
  region (records 1 and 4 in bank 4 are back-to-back: $89F2+1+23*16 = $8B63).
- Caller: PowerMeterPanel (bank 3, "SET POWER") calls OpenFramedWindowDraw with A=1 (frame record 1), X=1.
- $CDAE-$CDB4 (7 bytes): values $39-$3F continue the ascending NameCharMap tile sequence ($CDA3-$CDAD = codes
  $60-$6A -> tiles $2E-$38), i.e. NameCharMap codes $6B-$71. $CDB5-$CDC2 (14 bytes): $00 = codes $72-$7F. So a
  128-entry 7-bit character map $CD43 + code ends exactly at $CDC2, adjacent to the tile set. Codes $6B-$7F were
  not read in any trace (structural inference; whether they are used, or the zeros are just padding, is open).
Runtime (power-panel-reads.json, power-panel.png; NATURAL input: aim0 + press B, no RAM edits):
- Read trace bank-7 $CDAE-$D0C3: 841 reads, 769 distinct offsets = exactly $CDC3-$D0C3 (one run); nothing read in
  $CDAE-$CDC2. The panel shows the metal-framed power gauge window.
- Read trace bank-3 $8A76-$8AFF (region 1064, generic data, "no reference found"): all 138 bytes read (one run).
  PowerMeterPanel sets WinParams = $8A76 (`lda #$76 / sta WinParams / lda #$8a / sta WinParams+1`) and
  WinParams+2/+3 = $8AF8; OpenWindowAndDraw passes WinParams to DrawWindowRect. $8A76 = header (16, 8) + 128 tile
  bytes, $8AF8 = the 8 attribute bytes: the power panel's window rectangle. This corrects the r2-q1 / L-004
  "unreferenced" statement (r2-q1 datacheck searched only absolute operands into $8A76-$8AFF, not split #imm stores).
- The r2-q1 S17 trace (menus, names, ending) never opened frame style 1, which is why it saw no tail reads.
Classification impact: none (both blocks stay Data; boundaries of the existing regions are unaffected). Specific
roles, sub-ranges and stale evidence text are metadata/commentary refinements -> L-011 (map evidence) and L-012
(commentary).

## Gameplay milestone: Challenge mode / World Championship unlock (S20, S21; ARTIFICIAL)
- Route (natural inputs, challenge-route.json): boot1340, start(4,120), down(4,40), a(4,120) [CHALLENGE GAMES],
  down(4,40) x5 [SPEED POOL], a(4,150) [GET READY, GameMode 1, GameType $12, MS2=3, MS3=1], a(4,150) -> snapshot
  sp_aim (aiming, frame 2160).
- Unlock (challenge-unlock.json, sheet-speedpool-unlock.png): write BallState low byte := 0 for balls 1-15
  ($075C,$075E,...,$0778); natural shot a(4,30) a(4,30). The speed-pool rule found no active ball, PlayerScore 7 <
  141 -> WorldChampUnlock $EE 0 -> 1 (written by the game code), MatchOverCheck -> scoreboard "ROUND 1 OF 3",
  round 2 racked (MS3=2).
- Result (challenge-result.json, sheet-challenge-result.png): from sp_aim write MS3 $E3=3 plus the same ball clear;
  shot -> scoreboard ROUND 3 OF 3, "PLAYER 1 GAME OVER", EVALUATION "YOUR RANKING IS: NOVICE / SURELY YOU CAN DO
  BETTER ...", then GAME MODE menu and BREAK TYPE. Observation: a Speed Pool total of 7 ranks NOVICE because
  ChallengeResult only inverts THREE BALL; Speed Pool thresholds (291, 311, 351, 421, 501) rank a low total as index 0.
  This looks like a game quirk or an inverted script order; not investigated further (commentary-only note).
- A full three-round automated run (challenge-finish.json) desynchronised on rounds 2-3 (timing of GET READY
  dismissals); superseded by the MS3=3 variant. The tournament World Championship after the unlock was already
  shown by S15 (consumer of $EE=1).

## Deferred / not pursued
- Natural (unpatched) game win/loss and Speed Pool clear: reachability only; manual shot play remains impractical
  and has no structural value at 100% classification.
- Whether NameCharMap codes $6B-$7F are ever printed: no text was found using them in traced screens; open.
- Purpose of WindowFrameTable records 2-6 / other tile sets in bank 1/4/5/6 data blocks: not needed (they lie in
  already-classified Data regions); optional commentary sub-labels only.

## Addendum: other WinParams sources
All `sta WinParams` with immediate sources in the commentary source: bank 2 $052E (RAM), bank 3 TitleScreen
lo-only $00 store, PowerMeterPanel $8A76/$8AF8, SpinPanel $9043/$9095. $9043 = header (10, 8), +2+80 = $9095: region
998 (Data, $9043-$90D2) begins with the SET SPIN window rectangle, followed by the indexed tables read at $90BB..
No pointer lands in Code. Added to L-011/L-012.
