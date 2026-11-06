# Disassembly progress

Workspace initialized. Baseline and classification have not run.

## Round 1 — disassembly in progress
Input checkpoint: initialized empty classification. Baseline now established: maps/baseline.json, generated/baseline, analysis/baseline-verification.json; byte-identical true.
Configured binary /root/romatlas/target/release/romatlas. Mapper 3, fixed PRG banks [0,1,2,3]. Live RPC driver exec session 28465; protocol transcript analysis/round-001/rpc.jsonl. Startup reached frame 54 before 10-second bounded execution timeout; state retained. Classification not yet committed. Static vector candidate inspection in progress; IRQ vector $FFF0 points into FF run and must not be accepted as executable solely from pointer.

Round 1 live update: RPC daemon 28465 terminated (exit 143); recovered through direct per-experiment subprocess driver analysis/round-001/session.py. Durable title.snapshot and event-menu.snapshot preserve replay. Current integrated five-event experiment exec session 80268; each menu restore reasserts buttons [], patches $032C to event index (0..4), Start 2 frames then release, advances 3 x 40 frames, records framebuffer/RAM/coverage/context/discovery and durable event-N.snapshot. Event 0 and 1 completed so far. Hurdles visual confirmed; other event visuals pending review. $0328 selects difficulty/course, $2C menu substate. Saved connected static graph has 5,885 unique candidates (not yet annotations), from reset/NMI and bounded scheduler dispatch table at physical $7ACC-$7ADB. IRQ vector $FFF0 into FF run explicitly excluded. Documented floor-mat missing feature NES-001 in toolkit-issues.md; not blocking static classification. The pass remains active; no terminal exhaustion decision.

## Round 1 — disassembly complete; ROM incomplete

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

## Round 1 — commentary complete; ROM incomplete

Full current-source readability gate passed on frozen `pass-001-final`: all 62,938 classified bytes reviewed, plus explicit source notes for all 18 unknown intervals (2,614 bytes). Source `commentary/source/game.asm` and `symbols.inc`; refreshed `commentary/reading-after.asm`; 122 reviewed scaffold entries, 307 routine/data/unknown units, 249 meaningful routine/entry contracts, 823 code symbols and 383 storage/resource symbols. No known remaining naming/comment gaps. Mechanical explanations cover all systems, with uncertain movement/object/data meanings stated locally and in commentary/uncertainties.md. No structural, provenance or execution-coverage changes.

Fresh `commentary/verification.json`: ok=true, byte_identical=true, original input SHA unchanged. Source SHA game.asm `2b4116a6fcc0ea09d56483de69b5dfe886bad421854d132e4f4aea54c56dd39e`, symbols.inc `279878d10868cc13bc5e33b7bdfc224b35f7c167e61d85cb1ea28bf8785f7027`. Current maps/state exactly match frozen SHA `2f4dfde1c132c80d671777408ceb49043c4c25897ec35bb4244eb0e747d42e88`. All prior checkpoints/evidence and commentary/before preserved. Final editorial/archive acceptance remains pending; no audit performed.

Handoff: `analysis/handoff-round-001-commentary.md`; full scope/readability assessment: `analysis/commentary-round-001/readability-assessment.md` and `scope-assessment.json`. Next redirect investigates connected movement continuations and object handler/selector contracts plus adjacent unknowns, reusing disassembly evidence under evidence-policy.md. No artificial execution or toolkit work was needed for commentary. Toolkit report retains one missing feature NES-001, zero confirmed/suspected bugs and no blocker. No live processes, toolkit changes, ROM edits, publishing, deletion, archive or commits.

## Round 001 redirect — contracts attempt active
Input pass-001-final map 2f4dfde1c132c80d671777408ceb49043c4c25897ec35bb4244eb0e747d42e88; commentary gate accepted. Leads R1-MOVEMENT-RETURNS and R1-OBJECT-HANDLERS under investigation. Read redirect prompt, evidence policy, workflow, compact phase handoffs and uncertainties. Exact unknown complement 2614 bytes/18 intervals preserved; no map edits. Initial connected continuation and object bodies plus independent resource/patterned-data opportunities established in analysis/redirect/round-001-contracts/findings.md. api.describe and fixed-bank decodes retained there. Research active; no terminal assessment. Existing checkpoint byte identity remains verified; no new classification/rebuild claimed.

## Round 001 redirect — completed coherent batch
Attempt analysis/redirect/round-001-contracts complete. Ready leads R1-MOVEMENT-RETURNS,R1-OBJECT-HANDLERS,R1-RESIDUAL-STRUCTURE; proposed2607bytes (2238code369data), accepted0; exact proposals.json and unknown-ledger.json cover original18 intervals. Retained residual7bytes only if batch accepted; current actual unknown remains2614. proposal-verification.json confirms every proposed code endpoint decodes exactly, ROM/map/source hashes unchanged. review.md records independent contracts, inline descriptor alias caveat and localized seven-byte competing interpretations. Followed connected handlers/transitions and actionable independent arrays/helpers; no arbitrary first-lead stop.
Runtime: decoded-mat start prerequisite then end flag executes complete finish animationAF35, flag42; mode0328=1 patch on completed last-event snapshot reaches readable results via ordinary lifecycle.389 distinct instruction starts in original unknown intervals actually fetched artificially; exact identities/provenance in new-artificial-fetches.json. Natural race/certificate progression unclaimed. New scenarios retain all edits/budgets/snapshots; prior failed shortcut preserved. No live process remains. Existing verified checkpoint pass-001-final and current full-source readability unchanged; new structural batch requires independent disassembly then complete commentary. Toolkit: existing NES-001 missingfeature1, confirmedbugs0,suspectedbugs0,no blocker; no new toolkit issues found.

## Round 002 — disassembly active
Input verified checkpoint pass-001-final; exactly one disassembly pass active. Independent review has read complete proposed decodes, residual competing interpretations, object source-domain parser/records, artificial physical-fetch ledger and accepted neighboring tile/resource/collision consumers. Coherent full 2607-byte batch is supported under Static provenance; seven isolated bytes remain ambiguous. No map changes yet; next bounded API endpoint validation, annotate, save/export, fresh checkpoint. Prior commentary/history preserved.

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

## Round 002 — commentary active

Input pass-002-disassembly-final, current map eacaab08bf873b7224a198c2c6d6988a84474041c5c37c1d01e4246ee6491fab. Entire prior commentary preserved at analysis/round-002/commentary-prior-round-001 before supported fresh preparation. Carried 104 unchanged classified spans; added 66 mechanical routine contracts and named branches across all 20 newly accepted ranges, plus three explicit residual unknown notes. First coherent edited-source verify passes byte_identical=true. Remaining phase work: validate carried/new dependency prose and symbol consistency, reconcile all 127 review scaffold entries/units with current physical bounds and source locations, retain/update uncertainties, scope assessment, fresh final verify/read and compact handoff. This active checkpoint is not readability completion. Final editorial audit remains reserved.

## Round 002 — commentary complete; ROM incomplete

Full CURRENT-source readability passed for pass-002-disassembly-final: all65,545 classified bytes, plus explicit notes for7 Unknown bytes. Entire previous commentary preserved at analysis/round-002/commentary-prior-round-001. Supported fresh baseline plus physical-range reconciliation carried104 prior classified entries, added66 mechanical routine contracts, and corrected changed continuation/object/data dependencies. All127 scaffold entries and363 units reviewed with current locations/bounds; no known naming/comment backlog or raw numeric transfer/memory operands. Scope assessment analysis/commentary-round-002/readability-assessment.md and scope-assessment.json; exact handoff analysis/handoff-round-002-commentary.md.

Fresh final verification byte_identical=true,ok=true; reading-after refreshed. Maps/current,state,new frozen map/state remain eacaab08bf873b7224a198c2c6d6988a84474041c5c37c1d01e4246ee6491fab. ROM unchanged. Meaningful caveats retained: C2F0 descriptor selector can alias following C2FC code; indexed temporary sprite-base domain unresolved; static contracts do not assert natural entry. Residual physical end-exclusive0BDE..0BE0,34AA..34AD,37A9..37AB remain Unknown. No new structural proposal from commentary. Next redirect/reconciliation; terminal review and targeted final editorial/archive acceptance remain separate, not completed here. Existing NES-001 missingfeature1,confirmedbugs0,suspectedbugs0,no blocker. No toolkit/ROM edits, deletion, archive,publish,commit,subagents,server or emulator replay. Disk8.3GiB free.

## Round 002 — redirect residual contracts review complete

Attempt analysis/redirect/round-002-residual-contracts, input/output unchanged pass-002-disassembly-final eacaab08bf873b7224a198c2c6d6988a84474041c5c37c1d01e4246ee6491fab. Proposed0 accepted0; no ready leads. Revisited strongest corrected inference by combining LDY0/JMP with accepted queue/saturation consumers; assessed each RTS byte separately and generic Data alternatives independently. Primary source excerpts, exact bytes/limited encoded producer audit and compact claim/counterinterpretation review saved. No independent structural constraint distinguishes these isolated encodings; no concrete productive computed-producer/data-format/runtime avenue remains. Recommends terminal incomplete subject to coordinator primary-artifact review, no goal status changed. Exact complement physical [0BDE,0BE0),[34AA,34AD),[37A9,37AB),7unknown;65545classified. Full current-source readability and fresh byte-identical verification remain valid; hashes unchanged. Reused completed gameplay checklist without unchanged replay. Handoff analysis/handoff-round-002-redirect.md; review analysis/redirect/round-002-residual-contracts/review.md. No toolkit issues newly found; NES-001 missingfeature1,bugs0,no blocker. No ROM/toolkit/map/source edits, deletion,archive,publish,commit,subagents,server. Disk8.3GiB free, no scratch cleanup.

## Final targeted editorial acceptance — complete; classification incomplete

Following coordinator inspection of round-002 terminal residual reasoning, full valid source review was reused and concrete alias/name/boundary/certainty risks checked. Fourteen misleading scratch-derived branch names corrected without bytes/line changes; ledger paths made relative to commentary/source. Pre-final artifacts retained in analysis/final-editorial/before. commentary/editorial-audit.md contains findings; review editorial_audit reviewed. All 127 scaffold entries, 363 units and 79 code extents checked, including unit actual PCs/source lines. No known pending editorial work.

Supported commentary audit passed ok=true, byte_identical=true, editorial_audit=true, reviewed_regions=127, reviewed_units=363; fresh rebuild and reading refresh complete. Exact hashes/results analysis/final-editorial/result.json; compact analysis/handoff-final-editorial.md. Current source game.asm 915518f61e3abff050040b52e9e0fed274ea1c3ee601f2187da41968f4b0bea9, reading copy 828e6f88bd1147a1590ca8be2e92788f42c2d6879598e201e037db4d49c7ddbe. Maps/state/frozen baseline eacaab08 unchanged; ROM a109072d unchanged. Classification remains 65,545/65,552 with seven Unknown bytes, terminal incomplete. No archive, deletion, publish, commit, toolkit/ROM edits, subagents or live processes. Coordinator now handles remaining acceptance/options; writer released.

## Coordinator terminal handoff — orchestration complete, ROM incomplete

ROM classification: 99.99% (65,545 / 65,552 bytes classified; 7 bytes unknown).

Accepted justified terminal incomplete after inspecting round002 rejection reasoning and primary decodes/source; see coordinator-rejection-review.md. Targeted final editorial acceptance and fresh byte-identical audit passed; all127 regions/363units reviewed. Independent terminal-verification.json checks exact complement, evidence/provenance, current hashes and configured-binary store/map validation. Final checkpoint pass-002-disassembly-final; no ready leads or active blocker. Complete terminal-handoff.md links final source, reading copy, verification, residuals, progression limits and resume policy. Archive options account for final/history Markdown; no archive or cleanup performed. Orchestration completion does not mark ROM classification complete.
