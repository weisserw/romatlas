# Final acceptance: commentary editorial audit and whole-ROM verdict (2026-11-19)

ROM classification: 100.00% (327696 / 327696 bytes classified; 0 bytes unknown).
Verdict: **COMPLETE.** Every byte has an evidence-backed classification. The required
annotations are in place, the map is valid, the final commentary audit passed and the
rebuild is byte-identical. The open questions are semantic only; none affects
classification.

## Audit result
- `python3 /root/romatlas/scripts/commentary.py audit <workspace>` passed.
  - Output: commentary/audit.json (ok true, byte_identical true).
  - reviewed_regions 1497, reviewed_units 1889, editorial_audit true.
- commentary/review.json: editorial_audit.status is "reviewed", with dated findings notes.
  - All 1,497 scaffold entries are reviewed.
  - 852 entries carried their round-3 credit: their physical range and rendered text are identical.
  - 645 entries were re-reviewed because their rendered text changed. Most changed only through
    branch-label renames. I sampled those renames and checked the wording.

## Verification (artifact paths)
- `commentary.py verify`: ok, byte_identical true. ROM sha256 ca0c8372... (commentary/verification.json).
- `commentary.py audit` ran a fresh rebuild/verify and refreshed commentary/reading-after.asm.
- `romatlas map validate maps/current.json --rom input/game.nes`: {"ok":true}.
- maps/current.json is byte-equal to maps/pass-003.json (`cmp`). Its sha256 is 4b3c1d85794b....
  - analysis/pass-003-verification.json: byte_identical true.
  - Coverage recomputed from maps/current.json: 1,497 regions, 327,696 / 327,696 bytes non-Unknown.
- input/game.nes sha256 is unchanged (ca0c8372a20cf3cea17fd6c9f460789662fa5c3f1411e8a4af2d9fab12c141cd).

## Editorial changes
Before any edits, the pre-audit state was copied to analysis/round-3/commentary/; `diff -r`
showed the copy identical. Edits were made in commentary/notes/*.txt and commentary/tools/build.py,
then rendered into commentary/source/.

1. **Branch labels.** The generator took the condition from the branch opcode only, which
   produced misleading names:
   - `cmp #$04 / bne` became `@ifNmiModeNonZero`.
   - `lda A / cmp B / bcc` became `@ifBBelow`, which reverses the meaning.

   build.py now names the condition after the flag-setting instruction:
   - `@ifNmiModeNot02`, `@ifCpuTimerBelow10`
   - `@ifHomeScoreAtLeastVisitorScore`
   - `@ifPad1NewPressBit7Set`, `@ifHalfInningBit0Clear`

   545 automatic labels were renamed. Hand-chosen, game-meaning labels are unchanged.
   Labels whose condition ends in a hex value use a `_N` suffix on collision (`@ifAutoSubRequestNot08_2`).
   Where no game meaning is established, condition-derived names remain, and they are now accurate.
2. **Process references.** Mentions of redirect r1-a1/r2-a1, "round N" and lead IDs were removed
   from source comments:
   - Hand notes: ScaleLaunchByHomeLead, SceneInit17SeasonEnd, SceneFrame17, EnterPitchingMode,
     AnimationStepTimes, DrawStepFigure, NullHook9CAC, the HomeScore history, old-name remarks.
   - Generated region notes: the phase-1 evidence prefixes are stripped at render time.

   The replacement wording is plain, for example "confirmed in runtime tests with patched RAM,
   not in natural play".
3. **Unsupported live/dead claims (L-007).**
   - Old text: the module copies in banks 3-7 were "dead" or "never entered", and bank 0 called
     them "relocated".
   - Trace evidence: the bank-switch helpers in these copies (parts of FarCall, SetChrBank and
     SetBanksFromValue) execute from the copies in banks 1-6.
   - New text: only the reset/main-loop part ($8000-$8172) "has no known entry" outside bank 0.
   - DeadModuleReset is renamed ModuleCopyReset.
   - Also reworded: PhantomEntryAA09 ("never runs"), AnimationStepTimes entries 2-4 ("never
     read"), DrawStepFigure ("may be unused") and Port7000 ("no visible effect").
4. **Lower-confidence classifications** are now explicit at every affected region. Truncated
   evidence clauses that ended in ";" now end cleanly.
5. **ram.inc changes.**
   - ModeFlags: bits 4, 5 and 6 are documented. Bit 5 makes CpuInputHook reset CpuState and
     CpuTimer. The vague "bit 6 used by play logic" is gone.
   - Port7000: the at-bat-state writer is added, and the no-reader finding is stated.
6. **L-005:** the dispatch-table extent facts are added at CpuFieldTable, CpuBatTable and DemoTable.

Consistency checks (targeted):
- No raw RAM operand appears where a ram.inc symbol exists.
- No automatic Sub_/Lxxxx labels remain. Data_9490 is kept deliberately and explained.
- tools/nocomment.py: no global code label lacks a comment.
- tools/orphans.py: only the 4 known unlabeled continuations remain (bank 0 $86E3/$86FA, bank 3 $8397/$857D).
- These all match the corrected findings:
  - Button bits: A=$80, Start=$10.
  - HomeScore $0418 and VisitorScore $0426.
  - PennantCounterA is the game number.
  - ScaleLaunchByHomeLead (caller BeginBallInPlay).
- A sample review of every bank found no further concrete defects.

## Lead final statuses (analysis/leads.json)
The previous file is kept at analysis/round-3/leads-before-final-acceptance.json.

- **L-003 deferred-semantic.** The generic-data classification stands. Semantic roles of the
  unlabeled blocks are unknown.
- **L-004 deferred-semantic.** The ending and season-end paths were visited only with RAM
  patches. Natural reachability is not claimed.
- **L-005 verified.** All six InlineDispatch tables end exactly at their first handler.
  - The three 4-word tables are bounded by AND #3.
  - DemoTable has 6 words. It is masked with AND #7, but the demo states only count from 0 to 5.
  - CpuFieldTable has 23 words. CpuState writers produce only $00-$16, and $16 is transient.
  - CpuBatTable has 22 words. The batting handlers set only $00-$15.
  - Every pitching-view set-up resets CpuState through ModeFlags bit 5.
  - No table-extent or classification boundary error was found.
  - Runtime-only residual: no known path carries state $16 across a side change without a
    pitching-view set-up.
- **L-006 deferred-semantic.** No absolute-addressed read of $FFF0-$FFF9 exists. The bytes stay
  generic data.
- **L-007 resolved.** Wording corrected as described above.
- **L-008 and L-013 deferred-semantic.** The voice sample content was not confirmed by playback.
- **L-014 deferred-semantic.** No absolute-addressed read of $6000-$7FFF exists. Port7000's
  purpose is unknown.

All other leads were already terminal (verified, resolved or superseded).

## Lower-confidence classification summary
These are honest classifications. The source states each one as unresolved, not as live or dead.
- **Code with no known entry: 2,426 B at confidence 0.75** (L-024, structure-only acceptance).
  Each region notes that whether the game runs it is unresolved.
- **NullHook9CAC (bank 3 $9CAC): 1 B at confidence 0.6** (layout). Reachability is unresolved.
- **Pattern-based generic data: about 74,961 B at confidence 0.75.**
  - 78 "non-code by content" regions (74,925 B) plus one 36 B remainder block.
  - Each carries the note "data by byte patterns only; format and consumer unknown except where labelled".
- **AnimationStepTimes ($A398, 5 B).** No known code reads entries 2-4; the extent is inferred from the sibling layout.

## Residual uncertainties (semantic, not classification)
These are listed in commentary/uncertainties.md:
- Scenes 3, 4 and 19.
- The fourth-bounce AddError.
- Port7000.
- Voice sample content.
- The CPU decision tables.
- ScreenImageA/B and the bank-2 upper data.
- RunsTally.
- The $FFF0-$FFF9 bytes.
- Natural-play confirmation of the season-end stall (a probable game bug).

## Toolkit
I made no toolkit changes and found no new toolkit issues. TK-005 and TK-006 still stand in
analysis/toolkit-issues.md.
