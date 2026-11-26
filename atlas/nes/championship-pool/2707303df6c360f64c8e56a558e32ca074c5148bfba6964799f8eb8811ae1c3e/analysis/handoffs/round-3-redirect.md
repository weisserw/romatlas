# Round 3 - redirect handoff

Status: **complete**. No classification change is needed. Recommendation: **proceed to final acceptance**. Two
low-priority leads are ready and can be folded into it: **L-011** (map evidence refresh, no byte, kind or boundary
change) and **L-012** (commentary).

- Attempt dir: **analysis/redirect/r3-p1/**. It contains notes.md, tailxref.py, run_r3.py, run_power.py,
  run_challenge.py and crop.py, plus their JSON and PNG evidence.
- Input checkpoint: pass-003 (byte_identical).
- ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown). Accepted bytes are unchanged and
  no bytes are proposed.
- Not modified: the map, analysis/state.json, the commentary source, input/game.nes and the toolkit. The
  state.json mtime predates this attempt.
- Server: started through the rpcd bridge (scratch /tmp/ra-2707-r3). Only its own PIDs were killed, and the scratch
  directory was removed. About 2.5 GB is free; the attempt dir is about 0.3 MB.

## Outcomes per question

### (1) PortraitFrame 1/2: resolved (artificial entry, visual)
- **Method.** GameCmdDispatch case $12 (`jsr RecordGameWin`, then GameCmd=1) is an existing game command. From the
  tournament aiming snapshot aim0 (reached by natural inputs), I wrote $D2=$12. Two cases also had extra writes:
  - match0: PlayerWins $0457 = 3. The race is to 4.
  - match1: CurrentPlayer $E6 = 1 and PlayerWins $0458 = 3.
- **Frame 1** shows a scowl. It was seen with the game-won comment (win0) and with "YOU WIN THE MATCH. YOUR OPPONENT
  STALKS OFF..." (match0).
- **Frame 2** shows a grin. It was seen with "YOUR OPPONENT WINS THE MATCH." (match1), which is followed by
  ELIMINATED.
- In each case the neutral portrait is redrawn with the new frame 30 frames after the message opens
  (ShowPortraitExpression).
- **Evidence:** portrait-frame1-win0.png, portrait-frame2-match1.png, sheet-match0-stalks-off.png,
  sheet-match1-opp-wins.png and exp-*.json.
- The tentative PORTRAIT_OPP_LOST/WON meanings are confirmed. The entry was artificial; everything after the write
  is the game's own code.

### (2) FontTablesTail $CDAE-$D0C3: data role confirmed, with a new consumer and exact structure
- **Static check (tailxref.json).**
  - No operand, flow target or meaningful immediate-load pair lands inside the range. The 12 immediate-pair hits are
    coincidental: they combine lo and hi bytes of different symbols, or an attribute mask.
  - The linear decode is non-code: 148 of 486 decoded opcodes are unofficial, 272 are BRK, and 340 bytes are $00.
  - No code evidence contradicts the Data classification.
- **Consumer:** the word at bank 3 $8028 (phys $C038) is WindowTileSets record 3 = ($CDC3, bank 7).
  - WindowTileSets has 7 records of 4 bytes each: source, bank, 0. The source comment says "3 bytes each", which is
    wrong.
  - OpenWindowBanked selects this record for window frame record 1, and OpenWindow uploads it as a count-prefixed
    CHR tile set.
  - The count byte at $CDC3 is 48, so the set is 1 + 48*16 = 769 bytes: $CDC3-$D0C3. It ends exactly where PlayMatch
    code begins.
  - Caller: PowerMeterPanel (SET POWER), which uses frame record 1.
- **Runtime check (natural input: aim0, press B; power-panel-reads.json).** A bank-7 Read trace saw exactly
  $CDC3-$D0C3, all 769 offsets. It saw nothing in $CDAE-$CDC2.
- **$CDAE-$CDB4** holds the values $39-$3F. They continue NameCharMap's ascending tile run for codes $6B-$71.
- **$CDB5-$CDC2** holds 14 zero bytes, i.e. codes $72-$7F.
  - With these, NameCharMap is a 128-entry map ending at $CDC2, directly before the tile set.
  - Codes $6B-$7F were never observed read, so the zeros may be padding. This is a structural reading only.
- **Side finding: bank-3 $8A76-$8AFF has a consumer.** This is region 1064, generic data, whose evidence says "no
  reference found".
  - PowerMeterPanel stores WinParams = $8A76 and WinParams+2 = $8AF8 with split `#imm` stores.
  - DrawWindowRect draws it: a 16x8 header, 128 tile bytes, then 8 attribute bytes at $8AF8.
  - The natural trace read all 138 bytes.
  - Why r2-q1 missed it: its check covered only absolute operands into the block, not split `#imm` stores.
- **Classification impact:** none. Both blocks stay Data, and the existing region bounds are unaffected.

### (3) RecordGameWin entry 0: confirmed (artificial)
- I ran to PC $B1F9 in bank 3 (after `jsr Random`) and wrote RandomLo $AC → $A0, so that RandomLo & $1F = 0.
- At the `jsr ShowMessageB` ($B226), Ptr04 = $9C2F, which is entry 0.
- The screen showed "SO, YOU THINK YOU'RE GOOD NOW, EH? ...".
- Evidence: exp-win0.json and sheet-win0-entry0.png.

## Gameplay milestones (analysis/scenarios.json)
| Scenario | Result |
|---|---|
| S19 portrait expressions | Visited (artificial GameCmd/RAM writes): game-won comment, match won and match lost, ELIMINATED. |
| S20 SET POWER panel reads | Visited (natural input, traced). |
| S21 Challenge Speed Pool | Visited. The route to the table uses natural inputs; the rest is artificial. |

S21 details:
- **Unlock.** BallState was cleared for balls 1-15, followed by a natural shot. The speed-pool rule incremented
  WorldChampUnlock (0 → 1) itself, and the "ROUND 1 OF 3" scoreboard followed.
- **Result.** With MatchSetting3 forced to 3, the run showed GAME OVER, then EVALUATION "YOUR RANKING IS: NOVICE",
  then the GAME MODE menu.
- **Observation:** a Speed Pool total of 7 ranks NOVICE, because ChallengeResult inverts only THREE BALL. This is
  noted in L-012 as uncertain meaning.

M-challenge-play is now visited through an artificial shortcut. S18 is superseded by S19.

All major milestones have now been visited, mostly through RAM shortcuts: boot/credits, menus, every mode,
intermission, round 5 / World Championship, the ending, match won/lost screens and the Challenge end. Natural
manual-shot clears remain unattempted. They would only show reachability and have no structural value at 100%.

## Leads produced (analysis/leads.json)
- **L-011 (ready, low, map metadata).**
  - Refresh the evidence for region 1016: $CDC3-$D0C3 is WindowTileSets record 3, a 48-tile set, with a natural
    read trace. Optionally split the region at phys $1CDD3, keeping Data on both sides.
  - Refresh the evidence for region 1064: it is the power-panel window rectangle; drop "no reference".
  - Use map.annotate on the same id/offset/length/kind, then validate and checkpoint. Byte identity is unaffected.
- **L-012 (ready, low, commentary).**
  1. Split FontTablesTail into NameCharMap codes $6B-$7F and a window tile set (e.g. PowerPanelTiles).
  2. Correct the WindowTileSets and UploadWindowTiles headers (4-byte records: source, bank, 0). Rename
     WinTilesCount, which actually holds the bank.
  3. Label $8A76 as the power-panel rectangle.
  4. Drop "tentative" from the PortraitFrame constants (frame 1 scowl, frame 2 grin).
  5. Note the runtime confirmation of entry 0.
  6. Optionally add a mechanical note on the ChallengeResult Speed Pool ranking.
  7. Name the GameCmd $12 handler.
- History was appended to L-004 (consumer found for $8A76) and L-010 (open questions answered).
- No lead requires a disassembly round.

## Compact rejection / terminal review (evidence-policy.md)
1. **FontTablesTail kept as Data.**
   - Decisive evidence: the WindowTileSets consumer contract (count-prefixed 16-byte tiles), count 48 ending at the
     PlayMatch code boundary, a complete natural read of $CDC3-$D0C3, and a non-code linear decode.
   - This rules out code in $CDC3-$D0C3. It also corrects the "no reader" claim.
   - Strongest competitor for $CDAE-$CDC2: code or an unrelated table. But there is no flow, the 7 bytes continue the
     ascending char-map run, and the zeros fit 7-bit codes $72-$7F.
   - The remaining uncertainty (char-map entries vs. padding) is about meaning, not code vs. data.
   - Decision: keep Data; refine the metadata and commentary only.
2. **Region 1064 $8A76-$8AFF.**
   - The r2-q1 rejection of a specific role rested on "no reference". Retrieving datacheck.json showed that it only
     checked absolute operands.
   - The new split-`#imm` WinParams reference, the DrawWindowRect contract (header, tiles, attributes at +2/+3) and a
     full natural read establish the specific role and its exact extent (138 bytes).
   - The overbroad premise is corrected. Kind and bounds are unchanged.
3. **Prior kept low-confidence code** ($D9B0, $DE1C, $97B1, $FB32, gap fills).
   - The r2-q1 review premises are unchanged, and this round found no new data consumer for any of them.
   - A targeted follow-up: none of the new WinParams or WindowTileSets pointers land in Code regions. Every
     WindowTileSets record fits inside an existing Data region.
   - There are only two ROM WinParams sources. PowerMeterPanel uses $8A76. SpinPanel uses $9043, a 10x8 header with
     attributes at $9095; this is the start of Data region 998. This was added to L-011/L-012 as an evidence note.
   - The review is reused; no revised test is indicated.
4. **Why no further redirect step is useful.**
   - The commentary questions are answered.
   - No unknown bytes remain, and no code/data conflict was found.
   - The remaining open items are meaning-only and low value:
     - whether NameCharMap codes $6B-$7F are ever printed;
     - the intended Speed Pool ranking direction;
     - natural (unpatched) reachability of the win/loss screens.
   - None of them can change classification.
   - Deferred, with reason: a natural manual-shot clear. It is impractical, and it would only demonstrate
     reachability.

## Recommendation
Proceed to **final acceptance** (editorial audit). Apply L-011 (map evidence, then a new checkpoint) and L-012
(commentary) there. No further disassembly or redirect round is needed.
