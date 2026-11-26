# Round 2 redirect handoff

**Recommendation: another disassembly/commentary cycle.** Ready leads: **L-016**, **L-014**, **L-017**
(L-004 is satisfied through L-016/L-017).

## Scope and identity

- Input checkpoint **pass-002**: maps/pass-002.json = maps/current.json = analysis/state.json
  (sha256 1476b029...830fb, 407 regions). Commentary round 2 verified byte-identical.
- ROM classification unchanged at 100.00% (65552/65552, 0 unknown; the unknown complement is empty).
- Proposed, not accepted (redirect does not edit the map):
  - L-016: split/name 54 generic Data regions into 158 pieces; one kind change, Data -> Padding
    for phys 29090-29103 ($F192-$F19F, 14 bytes);
  - L-014: Static -> Observed provenance for 8 code regions (144 bytes).
- Not modified: map, state, commentary source, ROM, toolkit, configured binary.
- Attempt directory: analysis/redirect/r2-a1/ (README.md with the experiment table, tools/, evidence/).
- Both run.py serve instances (/tmp/r2a1.sock and /tmp/r2a1b.sock) were stopped and scratch was removed.

## Findings

1. **Data extents from consumers (L-003/L-010 → L-016).**
   - The commentary source rebuilds byte-identically, so a code operand written as a label proves that
     table's start.
   - 101 data labels are code-anchored (absolute or indexed reads, or `#<`/`#>` pointer setups).
   - Top-level frame tables and screen attribute/RLE blocks are named by those templates and descriptors.
   - Plan: evidence/l016_split_plan_refined.json. Every piece has an anchor class and evidence, and
     the pieces tile each region exactly (asserted).
2. **New tables decoded.** Five raw immediates in code point at unlabelled tables. Each extent is the
   index domain times the record size, checked against the ROM words:
   - $8A90: 9 HookCurve pointers, then 9 x 16-byte curves (`cpy #$10`);
   - $B161: BallWeightDigits (5 x 2), between BallColorTable (5) and BallWeightPpuAddr (4 words);
   - $C3EE: PlayerCharacter*4, two list pointers per character → 8 lists of 26 (tile, attr) pairs
     at $C402-$C5A1. ScreenDescPtr is reused as the list pointer for SetTilePairsFromList, Y=0/2;
   - $C87A: the same layout for the 250 award → 8 x 22 pairs at $C88A-$C9E9;
   - $CD37: CurrentPlayer*4, read by SetTilesFromList → 8 x 20 tiles at $CD47-$CDE6.
     PlayerSignColors is 4 bytes, not 299.
   - PerfectFramePtrs' raw words → four per-character 3-word frame tables at $BFDE-$BFF5.
   - Still unresolved: $CDE7-$CE0C (19 (tile, $02) pairs with no reference). It stays generic data.
3. **Sound (L-004).** soundparse.py (independent of commentary) parses all 64 headers, 51 streams and 44 envelopes:
   - envelopes occupy $E620-$E70D;
   - headers and streams occupy $E70E-$F191, and the parse never reaches past $F191;
   - $F192-$F19F is unreached $FF fill before code at $F1A0. It is a Padding candidate, consistent
     with the existing $FF Padding regions;
   - duration indices used are 1-27 and envelope ids used are ≤ $1B;
   - unreferenced: header $EFC7 (channels 4-7 → End bytes $EFD4-$EFD7), fragments $ED45-$ED4B,
     $EDD8-$EDDA and $EF83-$EF85, and 15 End bytes after looping Jumps;
   - no $80-$EF command byte occurs.
4. **Natural coverage (L-014).** All runs used natural inputs; traces were enabled only for the parts measured.
   - E12, lanes 2/4/5 with spin index 3: AdjustHookForLane $9DA9, $9DBB and $9DC7 executed.
   - E13, title wraps: TitleMenu $B51D and $B545 executed.
   - E11/E11b, a 3P lane-1 game with totals **300 / 278 / 257**:
     - all three award scenes ran in one results sequence;
     - HighScores became 300/278/257, so the rank-2 and rank-3 insertions ran: $BD33, $BD6C and
       RevealHighScoreName $BDC6.
   - Every instruction start of these 8 regions (144 bytes) is covered (evidence/gain_r2.json).
5. **L-001/L-002 reassessment.** Deferral kept; see the rejection review below.

## Ready leads (analysis/leads.json)

- **L-016** (medium): map split, naming and one Padding piece, following l016_split_plan_refined.json.
  - Recipe: in leads.json.
  - Validate, rebuild, verify byte-identical, checkpoint pass-003.
  - Disassembly may skip pieces it judges too fine.
  - Unnamed remainders (sprite records, unresolved bytes) stay generic Data.
- **L-014** (low): set Observed for region ids 373, 375, 377, 393, 227, 397, 399 and 401 (all fully
  covered; evidence/gain_r2.json). No boundary change. Static 2228 → 2084.
- **L-017** (low, commentary):
  - replace the five raw immediates and the raw .words with labels;
  - split and comment the new tables;
  - add the sound-data notes;
  - adopt the L-016 names.

## Rejection review (L-001/L-002, 1461 bytes Static code)

- **Claim tested:** these routines and islands are reachable code. Structural classification is not in question.
- **Decisive evidence** (evidence/l001_ref_search.json):
  - a word search (entry and entry-1) and a new immediate-pair search found no reference from code
    outside the group;
  - the hits are intra-group calls ($F4AF → $F3C9, $9D74 → $F863), self-loops and operand-byte
    coincidences;
  - the only indirect transfers are three inline-argument returns inside uncalled routines and the
    16-entry sound RTS table, whose handlers are all accounted for;
  - CNROM has a single fixed PRG, so no bank context can hide a caller;
  - no natural milestone executed them.
- **What this rules out:** a direct or pointer-table caller in this ROM. It does not rule out leftover
  debug/library code that is simply never called, which is the current Static classification.
- **Strongest competing interpretation:** that they are data. It is rejected by the coherent contracts
  (the inline-argument pop/`jmp (ptr)` convention, and island immediates sized to the data that
  follows them).
- **Conclusion:** no corrected premise or discriminating test remains, so the deferral stands.

## Deferred avenues (reasons)

- Remaining L-014 branches (about 0.62 KB):
  - 10th-frame residual mark/score paths;
  - the CelebrationTilesA/doubleB random choice;
  - fade, NMI and PPU helper branches;
  - page-crossing carries and envelope restart.
  These give provenance only, with no classification effect. Each needs a specific rare state or a
  random outcome, and traced play costs about 3.5 frames/s (TK-004).
- L-010: per-frame sprite-record and stream splitting in the source is commentary formatting. L-016
  provides the top-level boundaries.
- Unvisited gameplay: CelebrationTilesA (random) and pause (CheckPause has no caller). The game has no
  levels or bosses. The results screen and all three award scenes are visited naturally, now also in
  one 3P game. Lanes 1-5 and 1-4 players are all visited.

## Toolkit

No new issues; TK-001..TK-004 kept. Tracing speed (TK-004) again dominated the runtime.

## Markdown files created this phase

- analysis/handoffs/round-2-redirect.md (this file)
- analysis/redirect/r2-a1/README.md
