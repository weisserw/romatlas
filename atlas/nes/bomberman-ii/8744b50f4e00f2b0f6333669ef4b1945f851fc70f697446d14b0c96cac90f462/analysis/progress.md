# Disassembly progress

Workspace initialized. Baseline created in round 1 (see round 1 section below).

## Coordinator log

Coordinator: goal.md sequential phases (disassembly -> commentary -> redirect), one Agent worker per phase.

| Round | Phase | Status | Input checkpoint | Output checkpoint | Handoff | Notes |
|---|---|---|---|---|---|---|
| 1 | disassembly | done (incomplete) | baseline | pass-001 (verified byte-identical) | analysis/handoffs/round-1-disassembly.md | 99.98% (131057/131088; 31 unknown); leads produced L-001..L-007; consumed none |
| 1 | commentary | done (readability complete) | pass-001 | commentary/ (verified byte-identical) | analysis/handoffs/round-1-commentary.md | 657 routines named, 837 branch labels, review 978/978; editorial audit pending |
| 1 | redirect | done -> another round | pass-001 + commentary | (no map change) | analysis/handoffs/round-1-redirect.md | ready leads L-001,L-008 (high), L-004,L-009 (med), L-010 (low); L-007 commentary; attempt analysis/redirect/r1-a1 |
| 2 | disassembly | done (classification 100%; status incomplete pending commentary/editorial) | pass-001 | pass-002 (verified byte-identical) | analysis/handoffs/round-2-disassembly.md | 100.00% (131088/131088; 0 unknown); consumed L-001,L-004,L-008,L-009,L-010 (verified); produced none |
| 2 | commentary | done (readability complete) | pass-002 | commentary/ (verified byte-identical) | analysis/handoffs/round-2-commentary.md | round-1 copy analysis/round-1/commentary/; carried by physical CPU/bank keys; 662 globals, 836 locals, review 1287/1287 (1041 units); raw RAM operands 641->6; editorial audit pending |
| 2 | redirect | done -> final acceptance recommended | pass-002 + round-2 commentary | (no map change) | analysis/handoffs/round-2-redirect.md | attempt analysis/redirect/r2-a1; code/data audit clean; ending via stage-clear transition; L-012 ready (low, optional provenance); L-006/L-011 still blocked; L-007 verified |
| 2 | final acceptance | done -> accepted (audit pass) | pass-002 + round-2 commentary | (no map change) | analysis/handoffs/final-acceptance.md | editorial audit: 8 raw RAM operands named, 3 renames, 33 routine comments, 5 wrong comments fixed, review.json line-number fix (2328 locations exact); verify+audit byte-identical, map/store validate ok. Coordinator decision: no round 3; L-012 deferred (provenance-only) |

## Round 1 disassembly pass (pass-001), 2026-11-20

ROM classification: 99.98% (131057 / 131088 bytes classified; 31 bytes unknown).

- Checkpoint: pass-001 (maps/pass-001.json, generated/pass-001, analysis/pass-001-verification.json) — verification ok, byte_identical true,
  map sha256 f4ef36f87a330c00852c2584def61de0826088570731f348045d7dd559577c5d. Baseline created and verified (analysis/baseline-verification.json).
- Totals: code 24893 bytes (709 regions: 293 Observed, 416 Static), data 106164 bytes (264 regions), unknown 31.
- Execution coverage (separate metric): 9801 unique observed instruction addresses, 12646 contextual roots.

| 8K bank | code | data | unknown |
|---|---|---|---|
| hdr | 0 | 16 | 0 |
| 0 | 5246 | 2946 | 0 |
| 1 | 385 | 7807 | 0 |
| 2 | 0 | 8192 | 0 |
| 3 | 23 | 8169 | 0 |
| 4 | 3043 | 5149 | 0 |
| 5 | 23 | 8169 | 0 |
| 6 | 0 | 8192 | 0 |
| 7 | 23 | 8169 | 0 |
| 8 | 0 | 8192 | 0 |
| 9 | 858 | 7334 | 0 |
| 10 | 4509 | 3678 | 5 |
| 11 | 5027 | 3145 | 20 |
| 12 | 0 | 8192 | 0 |
| 13 | 23 | 8163 | 6 |
| 14 | 5733 | 2459 | 0 |
| 15 | 0 | 8192 | 0 |

- Evidence: see analysis/investigation.md (methods, conventions), analysis/evidence/ (observed contexts, region builds, inputs, frames).
- Scenarios: intro, title, menu, normal 1-1 (+death), RAM-selected areas 1-8,2-8,3-4,3-8,4-8,5-6,5-8,6-8, VS mode, password screen,
  artificial ending (PC=$CA5B); attempted battle, game over, bonus (analysis/scenarios.json).
- Leads consumed: none (none existed). Leads produced: L-001..L-007.
- Toolkit findings: TK-001..TK-004 (2 confirmed bugs, 2 missing features; no blocker) — analysis/toolkit-issues.md.
- Status: incomplete (31 unknown bytes; generic data meanings and Static-code reachability open).

## Round 1 commentary phase, 2026-11-20

- Prepared commentary/ from pass-001 via scripts/commentary.py prepare (baseline byte-identical).
- Current-source readability complete for all classified source (all 709 code regions, all banks): descriptive
  routine/branch/RAM/data names, routine and data-structure comments, honest notes for 31 unknown bytes.
- Durable records: commentary/annotations/*.py + commentary/tools/annotate.py; commentary/progress.md, uncertainties.md.
- Verification: commentary/verification.json byte_identical true; reading copy commentary/reading-after.asm.
- Classification unchanged (99.98%, 31 unknown). New consumer evidence for 4 unknown ranges recorded on L-001; L-005 notes added.
- Toolkit: no new findings.

## Round 1 redirect (attempt r1-a1), 2026-11-20

- Input checkpoint pass-001 (map/ROM/state unchanged). Classification unchanged: 99.98% (131057/131088; 31 unknown).
- Methods: static consumer/dispatch review of commented source; bounded Read/DMC/coverage traces; natural password routes;
  documented artificial pokes (WinCount, GameMode, Lives, ending PC). Evidence: analysis/redirect/r1-a1/.
- Results: roles for all 31 unknown bytes (L-001); BossActionTable dispatch table with 4 bytes of code inside data (L-008);
  fixed $E000-$FF67 = DMC samples (L-004); table extents (L-010); 120 Static regions executed naturally (L-009).
  Milestones visited: sound room, bonus stage, game over, battle arena, VS match point, credits/END, attract demo.
- Leads: produced L-008..L-011; updated L-001..L-007. Ready for disassembly: L-001, L-004, L-008, L-009, L-010.
- Toolkit: TK-005 (dummy reads indistinguishable in Read traces). Handoff: analysis/handoffs/round-1-redirect.md.
- Recommendation: another disassembly -> commentary -> redirect round.

## Round 2 disassembly pass (pass-002), 2026-11-20

ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).

- Checkpoint: pass-002 (maps/pass-002.json, generated/pass-002, analysis/pass-002-verification.json) — verification ok, byte_identical true,
  map sha256 79bafc80194eafdc9f8b03cad3b7ed29e7a5e1dcb47ddf5909392c004098379b (== maps/current.json == analysis/state.json). map.validate ok.
- Totals: code 24897 bytes (1005 regions: 602 Observed / 15371 B, 403 Static / 9526 B); data 106189 bytes (281 regions: 7 Observed / 7919 B,
  274 Static / 98270 B); text 2 bytes (1 Observed region); unknown 0. 1287 regions total.

| 8K bank | code | data(+text) | unknown |
|---|---|---|---|
| hdr | 0 | 16 | 0 |
| 0 | 5250 | 2942 | 0 |
| 1 | 385 | 7807 | 0 |
| 2 | 0 | 8192 | 0 |
| 3 | 23 | 8169 | 0 |
| 4 | 3043 | 5149 | 0 |
| 5 | 23 | 8169 | 0 |
| 6 | 0 | 8192 | 0 |
| 7 | 23 | 8169 | 0 |
| 8 | 0 | 8192 | 0 |
| 9 | 858 | 7334 | 0 |
| 10 | 4509 | 3683 | 0 |
| 11 | 5027 | 3165 | 0 |
| 12 | 0 | 8192 | 0 |
| 13 | 23 | 8169 | 0 |
| 14 | 5733 | 2459 | 0 |
| 15 | 0 | 8192 | 0 |

- Leads consumed (all verified and applied; evidence reused from analysis/redirect/r1-a1, statically re-checked against ROM bytes/commented source):
  - L-001 verified: 31 bytes classified — PasswordCursorSprite 0x15518+5 (Observed data), SoundTestCursorSprite 0x160C1+5 (Observed),
    VsMatchPointTiles9 0x178F9+6 / VsMatchPointTiles 0x178FF+7 (Static; reads only after artificial WinCount poke), IntroScoreText 0x179D7+2
    (Text, Observed), 0x1B037+6 generic data (Static, residue after ChrFontMenu chain).
  - L-008 verified: BossActionTable 0x1674+26 (13 words, bound cmp #$0D; added to tools/tables_spec.py), 0x168E+6 generic (beyond bound),
    BossBounceOffsets shrunk 0x1793+35 -> +31, new Code 0x17B2+2 BossActionMoveSpeed3 ($97A2) and 0x17B4+2 BossActionMoveSpeed2 ($97A4) (Static,
    INC $1C falling into BossActionMove; not executed in any window). Data->code correction of 4 bytes.
  - L-004 verified: fixed $E000-$FF67 (0x1E010+8040) split into 5 Observed DmcSamples regions (dmc_dma reads, natural) + 5 Static generic gaps
    (31/15/15/63/7 B). DmcSampleTable records re-derived from ROM (13 records, union identical to the observed intervals).
  - L-010 verified: 0x140DB+157 split -> StageItemChoices 0x140DB+144 + ItemChrIndex 0x1416B+13; StageTimeTable 0x15844+96 labelled with extent proof.
  - L-009 verified: 182 Static code regions with saved fetch evidence re-examined per instruction (analysis/pass-002/provenance_l009.py,
    l009-coverage.json). 129 split at executed/unexecuted run boundaries (476 pieces total): 5418 B Observed natural, 1859 B Observed
    artificial-only (evidence text says natural reachability not claimed), 2803 B remain Static. No byte classification change.
- Leads produced: none. L-007 stays for commentary; L-002/L-003/L-006/L-011 unchanged (not retried; no new evidence).
- Gameplay: no new runs this pass (all practical milestones were visited in round 1/redirect, analysis/scenarios.json; classification
  complete; remaining K22 spawner / boss content has no known route). Emulator not started; saved traces reused.
- Changed ranges vs pass-001: analysis/pass-002/changed-ranges.json (133 regions removed, 447 added, 54 same-extent provenance/label edits).
- Toolkit: no new toolkit issues found this pass (TK-001..TK-005 unchanged) — analysis/toolkit-issues.md.
- Status: incomplete as a project (classification contract met at 100%; commentary must carry names over to the split regions and new labels;
  editorial gates pending). Handoff: analysis/handoffs/round-2-disassembly.md.

## Round 2 commentary (checkpoint pass-002)
- Preserved round-1 commentary: analysis/round-1/commentary/ (full copy, diff -r identical) before `commentary.py prepare` on pass-002
  (fresh baseline, 1287 scaffold regions, byte-identical).
- Carry-over: workspace-local builder commentary/tools/annotate.py + annotations/*.py (keyed by bank + CPU address = physical identity).
  Branch/global label sets identical to round 1 except the intended additions (BossActionIdle/Rise/Fall/MoveSpeed2/3); L-009 split
  points stay cheap locals. Invalidated and rewritten: BossActionTable area, BossActionBlink note, BossBounceOffsets, bank-5 unknown notes
  (now named data), bank-6 $B027, fixed $E000-$FF67 (DMC samples + gaps), StageItemChoices/ItemChrIndex/StageTimeTable, SpecialPasswords.
- Backlog fixed: sound driver channel model and other RAM names (641 raw RAM operands -> 6), sound commands renamed by role,
  67 raw ROM operands -> 0 (enemy animation pointers, boss frames/scripts, score popups, noise drums, Sfx handler pairs as .word),
  verified #</#> pointer pairs, 79 uncommented routines (>= 10 instructions) commented, @exit vs @loop locals.
- Verification: `commentary.py verify` byte_identical true (commentary/verification.json, game.asm sha256 b5bc21c6...);
  reading copy refreshed (commentary/reading-after.asm). Audit not run (final acceptance only).
- Remaining ranges: none for readability. Unresolved meanings: commentary/uncertainties.md. Toolkit: no new issues.



## Round 2 redirect (r2-a1), 2026-11-20

Checkpoint pass-002 (map/state sha256 79bafc80...; ROM unchanged). Classification 100.00% (131088/131088; 0 unknown), unchanged.
- Code/data audit on pass-002: 0 executed opcodes in non-Code (8 windows), 0 code->non-code targets, 57 fall-throughs all FarCall inline triples, 0 decode problems; $FF fills and generic data not code-like (analysis/redirect/r2-a1/audit.txt, datacode.txt).
- Unreferenced code islands = exactly L-006 set (islands.txt). L-011 recipe executed: no producer of kind $16.
- Ending reached via ordinary stage-clear transition from Area 6-8 (StageClearTimer poke; no PC edit); stage sweep worlds 2-5 + 6-3. 26 Static regions (<=555 B) executed in these artificial windows -> L-012 (low, optional provenance).
- Toolkit: TK-006 (run_until timeout returns error). Leads: L-012 new ready; L-002/L-006/L-007/L-011 updated. Recommendation: (c) final acceptance.

## Final acceptance (2026-11-20)
- Classification unchanged at 100.00% (131088/131088; 0 unknown); pass-002 map/state/ROM hashes unchanged.
- Editorial audit done (commentary/editorial-audit.md): raw operands -> 0, renames Sfx10VoiceSteps/Sfx11VolumeSteps/SetEnemyDirDown,
  corrected comments (SoundTest buttons, Sfx11Repeat, CtlFadeMusic, IsEnemyBlocked, $95F9), 33 routine comments, review.json
  source lines corrected via workspace helper fix. editorial_audit = reviewed.
- `commentary.py audit` ok (byte-identical, 1287 regions / 1041 units reviewed); verify ok; map validate and store validate ok.
- Residual: unknown sound/intro bytes, $95F9 permutation, L-006/L-011 blocked, L-012 deferred. No toolkit issues found this pass.
  Handoff: analysis/handoffs/final-acceptance.md.

## Coordinator terminal handoff (2026-11-20)

- Workflow state: TERMINAL — whole-ROM classification and final editorial acceptance reached; no round 3 (no material ready leads; L-012 provenance-only deferred).
- ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).
- Final checkpoint: pass-002 (maps/pass-002.json == maps/current.json; analysis/pass-002-verification.json byte-identical).
- Commentary: commentary/ (audit pass: commentary/audit.json, 1287/1287 regions, 1041 units, editorial_audit reviewed; commentary/verification.json byte-identical; reading copy commentary/reading-after.asm; commentary/editorial-audit.md).
- Residual uncertainties (semantic, not classification): commentary/uncertainties.md; L-006 (5 uncalled entries), L-011 (enemy kind 22 no producer), L-012 deferred.
- Archive preparation: archive-options.json written (documents/omitted_documents account for every non-FINAL Markdown in analysis/ and commentary/). Archive execution and cleanup NOT performed (not authorized).
- Toolkit issues: analysis/toolkit-issues.md (TK-001..TK-006, none blocking).
