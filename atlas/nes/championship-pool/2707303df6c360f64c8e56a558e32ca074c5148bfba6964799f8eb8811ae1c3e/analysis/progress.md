# Disassembly progress

Workspace initialized 2026-11-25.

## Round 1 disassembly pass (2026-11-25) - status: incomplete (easy gains taken)

- Baseline created and verified (maps/baseline.json, analysis/baseline-verification.json, byte_identical).
- Output checkpoint: pass-001-final (maps/pass-001-final.json, generated/pass-001-final,
  analysis/pass-001-final-verification.json: ok, byte_identical true). Intermediate checkpoint pass-001 also verified.
- ROM classification: 98.30% (128863 / 131088 bytes classified; 2225 bytes unknown).

Per-bank (bytes; header = 16-byte iNES header):

| Bank | Code | Data | Padding | Unknown |
|---|---|---|---|---|
| header | 0 | 16 | 0 | 0 |
| PRG 0 | 0 | 16165 | 219 | 0 |
| PRG 1 | 0 | 16384 | 0 | 0 |
| PRG 2 | 3492 | 12755 | 134 | 3 |
| PRG 3 | 13570 | 769 | 0 | 2045 |
| PRG 4 | 0 | 16369 | 15 | 0 |
| PRG 5 | 13266 | 3045 | 41 | 32 |
| PRG 6 | 2309 | 13705 | 370 | 0 |
| PRG 7 | 11479 | 4671 | 89 | 145 |

By provenance: Code Observed 21,331; Code Static 22,785 (of which 998 exact gap-fill, confidence 0.7);
Data Static 83,879; Padding Static 868. Execution coverage (separate metric): 10,457 unique observed
instruction starts. Annotation coverage: every classified region has evidence text; labels only on vectors,
bank stubs and tables (commentary not started).

Evidence and methods: analysis/investigation.md. Scenarios: analysis/scenarios.json (S1-S7).
Leads produced: L-001..L-007 (analysis/leads.json). Toolkit issues: TK-001 (missing feature: discovery stops at
JSR on UxROM), TK-002 (suspected bug: 12x trace slowdown), TK-003 (confirmed bug: Read events include fetches);
see analysis/toolkit-issues.md. Handoff: analysis/handoffs/round-1-disassembly.md.

## Coordinator ledger (goal.md orchestration)

Handoffs live in analysis/handoffs/round-N-<phase>.md (one per phase worker).

| Round | Phase | Status | Input checkpoint | Output checkpoint | Verification | Leads consumed/produced | Handoff |
|---|---|---|---|---|---|---|---|
| 1 | disassembly | complete-pass | baseline | pass-001-final | ok, byte_identical (analysis/pass-001-final-verification.json) | produced L-001..L-007 | analysis/handoffs/round-1-disassembly.md |
| 1 | commentary (part 1) | gate not met: ~1180 generic @skipN labels, 139 address-only RAM names, shallow non-8-ball rules/bank-3 hooks/aiming comments | pass-001-final | commentary/source (verify byte_identical) | ok | redirect questions Q1-Q4 | analysis/handoffs/round-1-commentary.md |
| 1 | commentary (part 2, resume) | complete (gate met: 0 @skipN, 0 address-only RAM names; editorial audit reserved) | pass-001-final | commentary/source | commentary.py verify ok, byte_identical | redirect Q1,Q2,Q4 + 2 new (title-skip value, preview hook/scoreboard); Q3 answered statically | analysis/handoffs/round-1-commentary-part2.md |
| 1 | redirect | complete (ready batch) | pass-001-final | (none; research only) | n/a | updated L-001..L-007 (L-007 verified), produced L-008; ready L-001..L-006 (~2,225 bytes) | analysis/handoffs/round-1-redirect.md |


Coordinator note (round 1): ROM classification: 98.30% (128863 / 131088 bytes classified; 2225 bytes unknown). Worker cleanup used a broad `pkill -f "romatlas serve"`, which may have terminated servers of other workspaces on this host; future workers must kill only their own PIDs.

## Round 1 redirect (attempt analysis/redirect/r1-a7)
Bank-6 screen scripts decoded -> explicit far-call entries for all large unknown blocks; bounded traces observed
$830E-$8325, $832B-$8644, $FC65, $F885-$F8AC, $899E-$89C0, $8A51-$8A74, $E3A3-$E3E1, $95F6-$981F (some routes
artificial). Table extents tied to consumers. Shot contract (call-shot rule) and tournament RAM shortcut found;
World Championship and ending visited. Ready: L-001..L-006. Handoff: analysis/handoffs/round-1-redirect.md.

## Round 2 disassembly (checkpoint pass-002-final)
Consumed ready leads L-001..L-006 (analysis/tools/annotate_r2.py) with re-checked walks, observed starts and table
consumers; corrected the L-003 boundary ($97A3 word table = 7 words $97A3-$97B0; wait loop code $97B1-$97BA).
L-006 fragments accepted per family as low-confidence Static code (no reachability claimed). Provenance upgrades:
38 fully observed Static code regions (971 bytes) -> Observed from redirect traces and a new traced ending route (S14).
Gameplay: S14 ending (RAM shortcut, traced) visited through the interview back to a new tournament.

ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).

| Bank | Code | Data | Padding | Unknown |
|---|---|---|---|---|
| header | 0 | 16 | 0 | 0 |
| PRG 0 | 0 | 16165 | 219 | 0 |
| PRG 1 | 0 | 16384 | 0 | 0 |
| PRG 2 | 3495 | 12755 | 134 | 0 |
| PRG 3 | 15023 | 1361 | 0 | 0 |
| PRG 4 | 0 | 16369 | 15 | 0 |
| PRG 5 | 13298 | 3045 | 41 | 0 |
| PRG 6 | 2309 | 13705 | 370 | 0 |
| PRG 7 | 11619 | 4676 | 89 | 0 |

By provenance: Code Observed 23,556; Code Static 22,188 (878 bytes at confidence < 0.75: round-1 exact gap fills,
L-006 fragments, uncalled $FB32/$97B1 routines); Data Static 84,476; Padding Static 868.
Verification: pass-002 and pass-002-final both ok, byte_identical (analysis/pass-002-final-verification.json).
Classification is complete by count; commentary for the newly classified bank-3/fixed code is still required
(editorial gate separate). No toolkit issues found in this pass (analysis/toolkit-issues.md unchanged; TK-002 slowdown
still applies). Handoff: analysis/handoffs/round-2-disassembly.md.

Coordinator note (round 2): ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown). Byte coverage only; 878 B static code <0.75 confidence (round-1 gap fills, L-006 fragments, $FB32-$FB3F, bank3 $97B1-$97BA) and generic data $8A76-$8AFF to be reviewed by redirect before completion.

## Round 2 redirect (attempt analysis/redirect/r2-q1)
Review of all 878 B low-confidence code (56 regions) and code-bank data under evidence-policy.md: static xref,
indexed-table overrun check, code-evidence-into-data check, Observed provenance audit (558 regions all backed by
traces; artificial ones labelled). Result: keep all classifications (0 reverts, 0 boundary fixes). Produced L-009
(metadata-only refresh) and L-010 (commentary findings). Gameplay: S15 World Championship unlock consumer
(artificial), S16 credits entry, S17 char-map read trace, S18 attempted. Map/state/source unchanged.
ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).
Recommendation: proceed to final acceptance (editorial audit). Handoff: analysis/handoffs/round-2-redirect.md.

Coordinator review (round 2 redirect): inspected the compact rejection/acceptance review ($D9B0, $97B1-$97BA, $FB32-$FB3F, gap-fill routines, $8A76-$8AFF) and its artifacts list (analysis/redirect/r2-q1/xref-lowconf.json, idxscan.json, datacheck.json). Competitors and bounded index domains are addressed; no corrected inference found. Accepted.

## Round 3 disassembly (checkpoint pass-003)
Metadata-only pass. Ready lead L-009 was verified and applied (analysis/tools/annotate_r3.py). Evidence and
confidence were refreshed on 20 regions: 989, 974, 1112 and 1122, the 10 S14 regions (43, 173, 637, 643, 645,
655, 657, 692, 938, 959) and the 6 opponent-message regions (1060, 1062, 1123, 1125, 1127, 1128). No offset,
length, kind, provenance or view changed. map.validate is ok. pass-003 is ok and byte_identical
(analysis/pass-003-verification.json).

ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).

| Bank | Code | Data | Padding | Unknown |
|---|---|---|---|---|
| header | 0 | 16 | 0 | 0 |
| PRG 0 | 0 | 16165 | 219 | 0 |
| PRG 1 | 0 | 16384 | 0 | 0 |
| PRG 2 | 3495 | 12755 | 134 | 0 |
| PRG 3 | 15023 | 1361 | 0 | 0 |
| PRG 4 | 0 | 16369 | 15 | 0 |
| PRG 5 | 13298 | 3045 | 41 | 0 |
| PRG 6 | 2309 | 13705 | 370 | 0 |
| PRG 7 | 11619 | 4676 | 89 | 0 |

Low-confidence code (<0.75) drops from 878 bytes in 56 regions to 716 bytes in 53 regions, because regions 989, 974 and 1112 (162 bytes) now reach
0.9; region 1122 rises to 0.65. No new toolkit issues (analysis/toolkit-issues.md unchanged).
Handoff: analysis/handoffs/round-3-disassembly.md.

Coordinator note (round 3): ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).

## Round 3 redirect (attempt analysis/redirect/r3-p1)
Status: complete. No classification change.

ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).

Answers to the three commentary questions:
- Portrait frames 1 and 2 are confirmed visually (artificial GameCmd $12 entry).
- RandomMessageTable entry 0 is confirmed at runtime.
- FontTablesTail $CDC3-$D0C3 is the SET POWER window tile set: WindowTileSets record 3, 48 tiles, read in a natural
  trace.
- Side finding: bank-3 $8A76-$8AFF is the SET POWER window rectangle.

The Challenge Speed Pool unlock and the EVALUATION screen were visited (artificial).

New ready leads, both low priority with no byte or kind change:
- L-011: map evidence.
- L-012: commentary.

Recommendation: final acceptance. Handoff: analysis/handoffs/round-3-redirect.md. The server was stopped by its own
PID and the scratch directory was removed. About 2.5 GB is free. No toolkit issues found.

## Round 4 disassembly (checkpoint pass-004)
Metadata-only pass for ready lead L-011, which is now verified (analysis/tools/annotate_r4.py).
- **Region 1016 split at phys $1CDD3.**
  - 1016 is now $C86D-$CDC2 (phys $1C87D, 1366 B).
  - The new region **1131** is $CDC3-$D0C3 (phys $1CDD3, 769 B, Data Static 0.95). It is the SET POWER window tile set:
    WindowTileSets record 3, count 48, every byte read in a natural trace.
- **Region 1064** evidence was rewritten as the SET POWER window rectangle (0.95). The "no reference" claim came from
  an operand-only search; the address is built from split #imm stores.
- **Region 998** now notes that the SET SPIN rectangle starts it.
- **Region 991**: the "no code reference" wording was narrowed after the split-immediate scan
  (analysis/tools/splitimm_r4.py, analysis/round-4/splitimm.json, 33 regions). That scan found no new reader or
  caller for any low-confidence code region.
- No byte or kind changes. map.validate is ok. pass-004 is ok and byte_identical (analysis/pass-004-verification.json).

ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).

| Bank | Code | Data | Padding | Unknown |
|---|---|---|---|---|
| header | 0 | 16 | 0 | 0 |
| PRG 0 | 0 | 16165 | 219 | 0 |
| PRG 1 | 0 | 16384 | 0 | 0 |
| PRG 2 | 3495 | 12755 | 134 | 0 |
| PRG 3 | 15023 | 1361 | 0 | 0 |
| PRG 4 | 0 | 16369 | 15 | 0 |
| PRG 5 | 13298 | 3045 | 41 | 0 |
| PRG 6 | 2309 | 13705 | 370 | 0 |
| PRG 7 | 11619 | 4676 | 89 | 0 |

L-012 (commentary) remains ready. Toolkit: TK-004 was added to analysis/toolkit-issues.md. It is a minor doc
mismatch: on NES, map.annotate needs the `region` form. Handoff: analysis/handoffs/round-4-disassembly.md.

Coordinator note (round 4): ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).

## Round 4 redirect (attempt analysis/redirect/r4-m1)
All three optional commentary questions are answered with natural input (no RAM writes) and static checks.
- **BallMarkerSprite:** bank 4 is confirmed both by the static contract and by a trace (every read had prg_banks
  [8,9,14,15]). $B209 is a set-pointer word pointing to $B5D8.
- **SET SPIN window:** it reads exactly $9043-$909A (6 attribute bytes).
- **Speed Pool total:** it is elapsed seconds. Its ranking is not inverted, which is a likely game quirk.

There is no classification, bank-context or boundary change.
- New lead L-013 (commentary, ready, low) is for the final-acceptance editorial step.
- L-008 was marked verified (it was already applied in round 2).
- Scenarios S22 and S23 were added.

ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).

Recommendation: final acceptance. Handoff: analysis/handoffs/round-4-redirect.md. The server was stopped by its own
PIDs and the scratch directory was removed. No toolkit issues found.

Coordinator review (round 4 redirect terminal review): accepted. 0 unknown bytes; prior low-confidence review (round 2) holds on unchanged premises, round-4 split-immediate scan (analysis/round-4/splitimm.json) and r4-m1 traces show no contradicting reads; remaining questions are semantic only. Proceeding to final acceptance.

## Terminal handoff (coordinator, 2026-11-26)

Workflow reached its documented terminal handoff: whole-ROM classification complete with final editorial audit.
- ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).
- Final map: maps/current.json == maps/pass-004.json; internal store analysis/state.json; checkpoint verification analysis/pass-004-verification.json (byte_identical).
- Commentary: commentary/source (game.asm, ram.inc, bank0-7.asm), reading copy commentary/reading-after.asm, review.json fully reviewed, editorial audit commentary/editorial-audit.md, commentary/audit.json (ok, byte_identical). Prior commentary rounds preserved in analysis/round-1..3/commentary/.
- Accepted residual uncertainty (semantic only, no classification impact): commentary/uncertainties.md; ~716 B static code below 0.75 confidence reviewed and kept (round-2 redirect review, coordinator-accepted).
- Leads: L-001..L-013 resolved (analysis/leads.json). Toolkit findings: analysis/toolkit-issues.md (TK-001 missing feature, TK-002 suspected bug, TK-003 confirmed bug, TK-004 docs mismatch; none blocking).
- Archive preparation: archive-options.json written after inspecting /root/romatlas/scripts/archive.py (24 additional final documents selected; 9 toolkit-generated instruction copies in analysis/round-N/commentary/ omitted; simulated unrecognized-document check: none). Archive execution/cleanup NOT performed (not authorized).
