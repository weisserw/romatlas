# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## Round 1 investigation results

ROM classification: 96.01% (62,938 / 65,552 bytes classified; 2,614 bytes unknown).

Input checkpoint: initialized (no prior classifications). Output: `analysis/state.json`, `maps/current.json`, verified `maps/pass-001-final.json`, `generated/pass-001-final`, `analysis/pass-001-final-verification.json`. Intermediate baseline/pass-001-disassembly preserved. Final map SHA256 `2f4dfde1c132c80d671777408ceb49043c4c25897ec35bb4244eb0e747d42e88`; input hash unchanged `a109072d9fd1e2190b263b825ac74ce4ea040da2fd8e60d91e844f5e484f39bf`. All map validations and final byte-identical rebuild passed.

| Bank | Code | Data/image | Unknown |
|---|---:|---:|---:|
| header | 0 | 16 | 0 |
| PRG-0 | 4699 | 3242 | 251 |
| PRG-1 | 5610 | 835 | 1747 |
| PRG-2 | 512 | 7064 | 616 |
| PRG-3 | 3343 | 4849 | 0 |
| CHR-0 | 0 | 8192 | 0 |
| CHR-1 | 0 | 8192 | 0 |
| CHR-2 | 0 | 8192 | 0 |
| CHR-3 | 0 | 8192 | 0 |

104 regions, all evidence annotated. 14,164 code bytes;48,774 data/image bytes. Annotation completeness applies to classified regions only; editorial readability is next phase. Exact18 unknown physical intervals in `round-001/final-counts.json` (2,614bytes).

Game: Family Fun Fitness Athletic World,1986 title. CNROM fixed PRG context [0,1,2,3]. Reset/NMI/task/screen contracts,32-command audio return dispatcher, PPU/graphics/audio data corpora and bounded low-alphabet/record slices classified. Uniform IRQ-target FF tail is Data, not executed code. APU RAM upload origin proven by exact four-byte copy from physical7025..7029; RAM execution retained as RAM, not direct ROM execution.

Lead outcomes: R1-VECTORS,R1-EVENTS,R1-AUDIO,R1-TERMINAL verified. R1-MOVEMENT-RETURNS and R1-OBJECT-HANDLERS open for redirect (concrete producer/contract recipes in leads.json). No failed experiment should be replayed unchanged.

Runtime: natural title/training-menu/Hurdles; initialized-menu selector patches visited all five event backgrounds and initialization code. Sixteen bounded decoded-mat injections exercised player state continuation mechanics. Completion-flag shortcut remained in pre-race wait. Synthetic RenderScreen JSR with explicit stack/return/CHR contract rendered readable results and Beginner5thDegree certificate. Natural completion not established. Corrected CHR request030B=4 required; failed uninitialized-CHR captures retained. All exact patches,inputs,budgets and snapshots in scenarios.json and round-001 scripts/transcripts.

Execution coverage is separate: retained page union provides at least 1146 natural ROM instruction starts, 1747 artificial starts, 2163 unique ROM starts across scenarios; RAM starts01C6/01C9. Early coverage pages were partial; these counts are lower bounds, not execution percentages. All retained fetched ROM starts lie within classified regions. Discovery marks candidates Static; Observed is not inferred from a reached PC.

Toolkit report: one documented missing feature NES-001 (floor mat peripheral/input);0confirmed bugs,0suspected bugs. Does not block static analysis. See `analysis/toolkit-issues.md`. No toolkit changes. No live experiment/server processes remain. Next: commentary across all current classified source, then redirect investigate the two connected residual contracts and adjacent mixed gaps. This is an easy-gain phase handoff, not terminal research exhaustion.

## Round 001 redirect contracts attempt

Attempt: analysis/redirect/round-001-contracts, input pass-001-final. Followed all productive continuation installs and shared start/finish/recovery helpers, all distinct object handlers selected by ten source-anchored spawn streams, and independent tile helper/collision gate/resource arrays. Combined proposal2607 bytes (2238 code369 data), zero accepted map changes. Exact proposals, raw decoder evidence, domain parsing, verification and competing-interpretation review retained in that directory. Seven isolated bytes remain structurally ambiguous, with localized reasoning in review.md; no terminal exhaustion claim. Leads R1-MOVEMENT-RETURNS,R1-OBJECT-HANDLERS improved to ready; new R1-RESIDUAL-STRUCTURE ready. Runtime changed prerequisite completes start/intro before finish flag; actual finish-animation execution reaches AF35 and flag42. Natural reachability unclaimed. No unchanged failed-state replay; no toolkit edits.

## Round 002 — disassembly complete; ROM incomplete

ROM classification: 99.99% (65,545 / 65,552 bytes classified; 7 bytes unknown).

Exactly one pass. Input pass-001-final; fresh output checkpoint `pass-002-disassembly-final`. Accepted2607 Static bytes (2238 Code369 Data),124 regions,16402 Code49143 Data/image. Full independent review `analysis/round-002/review.md`; fresh exact decoder endpoints `endpoint-checks.json`, validation `validation.json`, bounded RPC `rpc.jsonl`. Current state saved/exported; checkpoint map validation,generation and whole-ROM rebuilding passed: ok=true,byte_identical=true. `analysis/pass-002-disassembly-final-verification.json`; map SHA256 `eacaab08bf873b7224a198c2c6d6988a84474041c5c37c1d01e4246ee6491fab`. ROM hash unchanged `a109072d9fd1e2190b263b825ac74ce4ea040da2fd8e60d91e844f5e484f39bf`.

| Bank | Code | Data/image | Unknown |
|---|---:|---:|---:|
| header | 0 | 16 | 0 |
| PRG-0 | 4803 | 3387 | 2 |
| PRG-1 | 7140 | 1047 | 5 |
| PRG-2 | 1116 | 7076 | 0 |
| PRG-3 | 3343 | 4849 | 0 |
| CHR-0 | 0 | 8192 | 0 |
| CHR-1 | 0 | 8192 | 0 |
| CHR-2 | 0 | 8192 | 0 |
| CHR-3 | 0 | 8192 | 0 |

Exact complement physical end-exclusive:0BDE..0BE0(2),34AA..34AD(3),37A9..37AB(2), in `analysis/round-002/counts.json`. These isolated LDY0/JMP/RTS encodings remain materially ambiguous. No supported longer contract was rejected for missing live caller. Inline C2F0..C2FC Data barrier preserved, fixed PRG banks[0,1,2,3]. All389 saved artificial instruction-start identities are separate corroboration, no natural reachability claim.

Lead outcomes R1-MOVEMENT-RETURNS,R1-OBJECT-HANDLERS,R1-RESIDUAL-STRUCTURE verified as static structural batches, with their histories retained in leads.json. Runtime and progression checklist reused without unchanged replays: all five events initialized, corrected mat-injection start/finish completes AF35 and flag42, completed final-event snapshot follows ordinary lifecycle into readable results; prior synthetic certificate remains separate. Natural event/course completion unclaimed; unavailable floor mat remains documented. No fresh productive gameplay route or supported residual discriminator emerged. Easy-gain pass complete, not terminal research exhaustion.

Prior full-source commentary remains intact but applies to pass-001-final; expanded current map requires full current-source commentary/reconciliation, then redirect on seven residual bytes. New generated checkpoint is byte-identical, not a readability acceptance. Compact handoff `analysis/handoff-round-002-disassembly.md`. Toolkit: no new toolkit issues found; existing NES-001 missing feature1,confirmedbugs0,suspectedbugs0,no blocker; see `analysis/toolkit-issues.md`. No ROM/toolkit changes, commits,publish,archive,deletions,subagents or live server.

## Round 002 redirect — residual contracts

Unique attempt analysis/redirect/round-002-residual-contracts. Proposed/accepted0; exact7 residual bytes unchanged. Applied evidence-policy to strongest revised inference (minimal prefix/jump plus accepted consumer versus longer accepted uncalled helpers), split each return stub and generic-data claim. Review links exact primary decodes and current source excerpts, identifies positive code-pattern support and its insufficiency, localized counterinterpretations and limits of empty target/minus-one search. All current explicit structural uncertainties/indirect JMP domains triaged; no missing concrete producer or format affords a discriminating next test. Reused gameplay milestones without replay. Recommends incomplete pending coordinator inspection; no empty disassembly/commentary cycle. Compact handoff analysis/handoff-round-002-redirect.md; primary rejection review analysis/redirect/round-002-residual-contracts/review.md. Current-source readability and byte identity remain complete, final editorial/archive acceptance pending.
