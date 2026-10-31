# Disassembly progress

Round 01, current phase: disassembly finished incomplete; next phase commentary. Latest checkpoint: maps/round-01-disassembly.json; verification: analysis/round-01-disassembly-verification.json. ROM classification: 57.89% (151767 / 262160 bytes classified; 110393 bytes unknown).
Baseline established with `python3 run.py inspect` and `baseline`; generated/baseline byte-identical, maps/baseline.json remains export-only, not structural evidence.
Live JSON-RPC session running configured binary through run.py. Startup advanced 128 frames with effective banks [10,11,14,15]. Trace detail capacity zero; aggregates retained; first short default-timeout run stopped at frame 8 and was continued under explicit budget. Evidence transcript: traces/round01-rpc.jsonl. Historical startup frontier (superseded by checkpoints below): classifications had not yet been committed; reset review and gameplay progression then followed.
No preexisting leads or state. No toolkit blocker; no source/toolkit edits. Next checkpoint will include reviewed code, header and CHR structural regions and persistent emulator snapshot.

Batch A: 142001 / 262160 classified (54.17%); header 16, CHR 131072, PRG code plus six vector bytes 10913. Saved analysis/state.json, maps/current.json. Verified checkpoint maps/round-01-batch-a.json / generated/round-01-batch-a, byte-identical. Static closure batch limited to first 5000 candidates (not exhaustive); observed fetches separately reviewed. Actual gameplay: entered Addams residence, Hall of Stairs, moved/jumped right across hall. Door/climb exploration and late-game shortcut research ongoing. Game identified from title: The Addams Family: Pugsley's Scavenger Hunt (NES), copyright 1993. Guide research available; NES passwords from GameFAQs are hypotheses pending runtime confirmation.

Durable update: game research in analysis/gameplay.md with source URLs and explicit unconfirmed NES password recipes. House/hall snapshots exported persistently. Hall traversal reached right boundary but no later door yet; not declaring gameplay exhausted. Trace-disabled travel now used to bound elapsed time, with tracing reenabled for representative observations. Current frontier reset frame473, still presentation sequence while establishing password entry route.

Current structural total after uniform-data batch: 146163 / 262160 = 55.75%, 115997 unknown. Practical RAM shortcut identified from comparison: player world X little-endian $5F/$60 and Y $61/$62. Hall initial Y=$0178. Restored clean hall, wrote RAM $61=[10,01] (Y=$0110), idle20 then Up35: observed effective PRG banks changed [6,7,14,15] -> [2,3,14,15] during transition. Artificial state correctly tagged by toolkit; waiting for screen initialization before claiming visited. Exact recipe retained transcript. Natural all-rescued password 6R2KK confirmed displaying family portraits; route/snapshot in gameplay.md.

Correction to prior RAM hypothesis: $5F-$62 are derived viewport bounds (E126-E13E), not player coordinates. $61 patches were overwritten, Up returned to residence; no later area visited. Source origin candidate $57/$58 under one bounded test now. Preserve old observation but reject its player-coordinate premise. Useful redirect question: identify player object coordinates and supported room-load/door contract (D2/1F/20 candidates) so later areas can be reached reproducibly.


## Round 01 disassembly completed


Status: incomplete; one disassembly pass finished. Next phase: current-source commentary, then redirect. No terminal exhaustion claim.

ROM classification: 57.89% (151767 / 262160 bytes classified; 110393 bytes unknown).

| PRG 8K bank | Code | Data | Classified | Unknown |
|---|---:|---:|---:|---:|
| 0 | 0 | 819 | 819 | 7373 |
| 1 | 0 | 491 | 491 | 7701 |
| 2 | 0 | 264 | 264 | 7928 |
| 3 | 580 | 299 | 879 | 7313 |
| 4 | 1250 | 0 | 1250 | 6942 |
| 5 | 0 | 1039 | 1039 | 7153 |
| 6 | 0 | 375 | 375 | 7817 |
| 7 | 0 | 109 | 109 | 8083 |
| 8 | 0 | 396 | 396 | 7796 |
| 9 | 0 | 162 | 162 | 8030 |
| 10 | 545 | 60 | 605 | 7587 |
| 11 | 1005 | 0 | 1005 | 7187 |
| 12 | 2682 | 93 | 2775 | 5417 |
| 13 | 490 | 0 | 490 | 7702 |
| 14 | 4959 | 0 | 4959 | 3233 |
| 15 | 5055 | 6 | 5061 | 3131 |

Header: 16 classified Data bytes. CHR: 131072 classified Image bytes (all CHR banks); no unknown. Exact residual complement: analysis/round01-counts.json. Every classified region has structural evidence; code regions carry explicit CPU addresses/effective PRG contexts. Labels/readability are provisional pending commentary; classification does not imply editorial completion.

Verified: baseline generated/baseline; intermediate generated/round-01-batch-a; final generated/round-01-disassembly all byte-identical. Final map maps/round-01-disassembly.json (SHA256 d15fb068b9c45af0892979e629e2796737e0c61c15dfda1e84e34e971fd06b17); maps/current.json and analysis/state.json saved and map.validate passed. Input SHA256 remains 6038f5d85b862d5c8a896da8d73f94fa6d6446698a20cd76a83b12a757adc20c.

Evidence: traces/round01-rpc.jsonl is bounded local full RPC transcript. Observed instruction contexts decoded individually: analysis/round01-observed-instructions.json (6552 unique physical starts retained from contextual roots). Trace summary reports 9527 unique instruction addresses, 12236 contextual roots, 13217 contextual edges, aggregate_dropped=0. Detail capacity zero intentionally drops raw detail; aggregate count is separate from classification. Some later representative observations follow recorded artificial RAM edits; Observed means accepted fetch, not natural reachability. Trace-disabled travel is never used as observed execution proof.

Static gains: 6144 candidates in reset/NMI/IRQ supported closure, with runtime edges and conservative stops at uncertain mapper effects/indirects. Exact source artifact analysis/round01-static-candidates.json; physical vector interval 131082..131088 is Data. No arbitrary pointer target was accepted as code. Generic data: 55 exact uniform interiors, 4162 bytes; 16-byte flanks, nearby contextual decode/reference attempts and conflicting code checked; analysis/round01-data-inspection.json. No specific table/consumer meaning asserted. No conflict between observed instruction boundaries and saved code/data annotations in explicit check.

Gameplay: identified visually as The Addams Family: Pugsley's Scavenger Hunt, NES, copyright1993. Residence and Hall of Stairs visited naturally, movement and representative jumps exercised. Natural NES password 6R2KK entered exactly and accepted; rescued portraits observed. Persistent title/password/house/hall/all-rescued snapshots and replay recipes in analysis/gameplay.md and analysis/scenarios.json. Actual later areas/bosses/ending NOT visited. Practical shortcuts attempted after clean snapshot and inspected; $61/$62 patch rejected as player-coordinate hypothesis because E126-E13E recomputes viewport bounds from $55-$58. Revised $57/$58 source patch also returned hall. Avoid repeating unchanged edits. No register edits, ROM backing edits or ROM download.

Concrete redirect leads:
- R01-door-contract: use saved hall/all-rescued snapshots; identify player object position/vertical movement and door dispatch selector. $1F controls bank selection at C7E4->C182; $D2 saved around E41F initialization; $20/$21 differ between residence/hall. Existing source generated/round-01-disassembly/game.asm provides fixed-bank consumers. Need supported room transition contract before more patches. Missing player-object state prevents useful late-game shortcut; early manual traversal ceased giving useful gains.
- R01-indirect-closure: inspect static diagnostics/references in transcript and bank-switch/indirect dispatch routines to establish additional entries and connected tables. Static graph did not exhaust ROM.
- R01-data-records: first PRG banks contain clear repeated low-valued tile/record patterns; uniform interiors accepted, broader record bounds/consumers remain to establish. Use immutable bytes and reference consumers; do not require natural selector for supported structural classification.

No preexisting leads existed. No terminal rejection review is required for this ordinary easy-gain handoff; actionable research remains. Toolkit findings: no toolkit issues found (analysis/toolkit-issues.md), no blocker. Harness map-page mistake caused correctly rejected duplicate annotations; full persisted region inventory now used, maps validated. Baseline export-only raw gaps remain unknown.

Fresh verification report: analysis/round-01-disassembly-verification.json. Final equality check: internal state == current portable map == checkpoint map. All 55 generic-data runs were inserted with map.annotate, Static provenance, exact run bounds and rationale; no raw export gaps were counted. Live server closed after durable saves.


Round 01 commentary started. Supported preparation created frozen commentary/before and full-ROM review scaffold, 8241 regions; baseline verification byte-identical. Current source inventory: 47,948 lines. No previous commentary credit to carry. Reading and naming all classified source; unknown complement remains unchanged. Disk available 11 GiB; dedicated scratch analysis/commentary-scratch.

Commentary reading milestone: all 7917 classified instructions inspected in condensed bank listings. Curated notes for banks3/4/10/11/12/13 saved; fixed-bank notes and installation underway. Source/review still pending, no readability completion claimed. Player world position identified mechanically as $9A-$9D; camera is $55-$58. Scene loader FC87 contains the concrete room-entry contract needed by redirect. See commentary/progress.md for remaining installation scope.


Commentary source-install milestone: 302 behavior explanations, 1048 descriptive control labels and 380 operand symbols installed across all 7917 classified instructions. Two coherent verification runs passed byte-identical. All 8241 review scaffold entries reviewed, with Unknown entries explicitly limited to boundary/uncertainty review. No classification/state/map edits. Final verification and reading-copy refresh next; final editorial audit deliberately pending.


Round 01 commentary complete: zero current-source readability backlog. Final supported verification byte-identical; reading-after.asm refreshed after final control-label refinement. Exact scope/partition checks passed (analysis/round-01-commentary-scope.json). Handoff: analysis/round-01-commentary-handoff.md; verification: analysis/round-01-commentary-verification.json. Current phase complete, next redirect; classification remains incomplete57.89%, final editorial audit pending. Historical arithmetic correction: actual map uniform interiors total4107 bytes, not4162 in prior prose; total classified151767 unchanged. No toolkit issues found.


Round 01 redirect active: analysis/redirect/r01-scene-contract-20261030. Read commentary handoff/leads/evidence policy/workflow. Map/source/state frozen. Player coordinate and scene-loader contracts justify improved progression method. Investigating connected scene/data and independent object/audio/low-bank batch. Disk11GiB free.

Redirect milestone: FC87 scene consumer yielded91 scene records and38 selected themes; dimension/stride-bounded maps produce38539 unknown-byte proposal union (accepted0, map frozen). All seven Hall destinations initialized via supported647/648/649 request; six playable distinct environments exercised, Fester interstitial reached. Evidence analysis/redirect/r01-scene-contract-20261030/findings.md,scene-records.json,proposals.json,unknown-ledger.json,visit-exit-*.json/png. Continuing actor/audio closure and later milestones; no exhaustion assessment.

Redirect late milestone persisted in attempt/findings.md: train mechanic29, bosses33/35/final38 and rooms88/89/post-boss90 visited;2 traced frames each confirm artificial execution. Ending still pending; Start paused. Actor/audio dispatch closure2260 candidates with conservative verified continuations; asset review corrected special quadrant transforms, so initial33374-byte asset union provisional and filtered. All map/source/state frozen; active research continues.

Redirect defeat/closure checkpoint: signed06A3=FF reached guardian-defeated/Granny rescue password screen. No global ending branch established; 043B=1D test already includes dragon bit08 and therefore skipped battle. Static object/audio/boss init/render graph fixed point3436 candidates,52 reviewed preserving continuations. Middle boss callback bank unresolved and excluded. Two bounded repeated-data candidates and24 palette records added for review; combining exact proposals and lead recipes, no accepted classifications.


Coordinator resume: inspected current goal/policy/workflow and durable Round01 handoffs; redirect remains active and productive, not terminal. Separate sequential worker `/root/redirect_resume` resumes the existing attempt frontier; no competing phase writer launched. Prior goal turn established concrete defeat/closure evidence (progress). Coordinator inspected toolkit scripts/archive.py document-selection contract (FINAL plus explicit documents/omitted_documents; unrecognized Markdown must be accounted for). Interim coverage check analysis/archive-document-selection-check.json found zero unrecognized Markdown; refresh after new final handoffs. No archive execution, cleanup, toolkit edit or publication. Next gate remains completed redirect combined-batch handoff, followed by a separate disassembly worker and current-source commentary.

Round01 redirect complete-ready: analysis/redirect/r01-scene-contract-20261030/handoff.md. Combined87913 previously unknown-byte proposal, accepted0; frozen classification151767 unknown110393. Final audit no code/data or instruction alignment conflicts; helper transforms resolved and FC99 operand root corrected toFC9A. Exact final bank-aware unknown ledger and per-bank counts retained. Gameplay includes train,three guardians,later rooms and Granny rescue/password; global ending unproven. All five stable lead IDs ready with exact ranges/recipes. No toolkit issues found; no blockers. Next independent disassembly, not terminal exhaustion.

Coordinator accepted Round01 redirect as ordinary ready-batch handoff after reading compact scope and final-audit.json; not terminal. Redirect worker finished before separate `/root/round02_disassembly` started. Round02 input checkpoint maps/round-01-disassembly.json; five ready lead IDs consumed for independent review. Archive options explicitly preserve redirect handoff; interim selection check remains complete. Previous goal turn was progress (helper/boundary corrections) plus verified live-worker wait.

Round02 disassembly active: independent primary contract review in analysis/round02-disassembly. Fresh configured-binary analysis.decode verified all3567 contextual candidate instructions exactly (decode-review.json,decoded-code.json,rpc.jsonl). consumer-review.txt freshly decodes scene/theme,actor initializer/behavior,audio dispatch,boss wrapper,metatile/attribute and special graphical transforms. Existing classifications and commentary remain unchanged. Reviewing semantic/bound evidence and graph continuations before annotation. Generic low-value matrix actual maximum62 (redirect prose said61); this is a numeric evidence correction, not a counterexample to non-code. Disk11GiB free. Next: independently rederive data cells/descriptor extents, graph/overlap checks, annotate supported unknown pieces, validate and fresh round-02-disassembly checkpoint. No toolkit issues found so far.


# Round02 disassembly completed, incomplete ROM

ROM classification: 91.99% (241168 / 262160 bytes classified; 20992 bytes unknown).

Accepted89,401 new bytes:5,192 Code and84,209 Data, all Static. Independent review accepted redirect combined87,913-byte union, then followed connected repeated-record/script margins for another1,488 generic Data bytes. Existing regions preserved verbatim, input unchanged; no baseline recreated. All five R01 lead IDs verified; no unchanged gameplay experiments repeated. Exact current complement and counts: analysis/round02-disassembly/counts.json; per-lead outcomes: lead-outcomes.json.

| PRG8KiB bank | Code | Data | Classified | Unknown |
|---|---:|---:|---:|---:|
| 0 | 0 | 8008 | 8008 | 184 |
| 1 | 0 | 7695 | 7695 | 497 |
| 2 | 0 | 8112 | 8112 | 80 |
| 3 | 580 | 7404 | 7984 | 208 |
| 4 | 1444 | 6245 | 7689 | 503 |
| 5 | 0 | 7489 | 7489 | 703 |
| 6 | 0 | 8032 | 8032 | 160 |
| 7 | 0 | 7644 | 7644 | 548 |
| 8 | 0 | 8040 | 8040 | 152 |
| 9 | 1359 | 4797 | 6156 | 2036 |
| 10 | 545 | 4808 | 5353 | 2839 |
| 11 | 1090 | 4397 | 5487 | 2705 |
| 12 | 3321 | 3616 | 6937 | 1255 |
| 13 | 2272 | 1721 | 3993 | 4199 |
| 14 | 5597 | 308 | 5905 | 2287 |
| 15 | 5550 | 6 | 5556 | 2636 |

Header16 Data, CHR131072 Image classified; full-ROM totals Code21758,Data88338,Image131072. All10782 regions have evidence. Provenance byte totals: Static233508,Observed7660; this is not execution coverage. No new runtime traces in this pass; prior accepted execution/scenarios remain unchanged (natural versus artificial distinguished there). Structural annotations do not claim natural selection/reachability.

Primary verification: fresh configured-binary re-decode of3567 contextual instructions; independent rederivation of15 specific Data role unions;61 callee continuations rechecked, preserving ordinary scratch/stack and nonwrapping OAM allocator preconditions. Generic data inspected against byte patterns, neighboring classified regions, consumer/reference review; specific original semantic partitions remain unresolved. Generic matrix max62 corrects redirect's max61 prose, without changing non-code conclusion. Evidence: independent-review.json,consumer-review.txt,decoded-code.json,annotation-plan.json,connected-data-review.json/.hex,rpc.jsonl under analysis/round02-disassembly.

map.validate passed; state/current/checkpoint byte-equal with SHA256 abfbf643f0f7d3612f9ac5484cbedd1c3b0e60fcc12023f2d0f15bf3e2bd23b9. Fresh checkpoint maps/round-02-disassembly.json generates generated/round-02-disassembly; analysis/round-02-disassembly-verification.json confirms byte-identical rebuild. final-verification.json confirms preserved ROM/original regions/previous commentary sources and identity.

Remaining20992 bytes are unknown; no full classification/editorial completion claimed. Broad progression and failed ending attempts preserved in analysis/scenarios.json and redirect handoff; no new supported initialized ending entry to retry. Other local barriers: unresolved bank at boss middle callback, arbitrary F8/FC script prerequisites,127 stopped call/mapper contracts and mixed code/data/asset margins. These require local bank/stack/inline or bounded data facts, not blanket rejection. Current source after this acceptance can expose concrete new redirect avenues after full commentary. This is an ordinary incomplete handoff, not terminal exhaustion. No toolkit issues found; analysis/toolkit-issues.md; no blockers. Disk11GiB free. No cleanup/archive/publish/toolkit edits. Next phase current-source commentary, preserving prior commentary/source, then redirect.

Round02 commentary active: preserved entire previous commentary at analysis/round-02/commentary-round01; fresh supported commentary preparation verified byte-identical. Current-source review/carry pending. Frontier: analysis/round-02/commentary-checkpoint.md. Same phase continues; no redirect or editorial completion.

Round02 full current-source commentary complete. Prior entire commentary preserved under analysis/round-02/commentary-round01. Reviewed all2443 newly classified instructions and364 Data records, reusing sound prior commentary. Current10360 Code/421 Data/1 Image/438 Unknown scaffold entries reviewed;470 curated routine/block notes and413 symbols. Corrected older CPU9C08/9C0A control labels and new initializer prose: flags/coordinate staggering, distinct from animation frames. Source script-coordinate/timer/callback fields and record/table formats explained, remaining game meanings explicit. Scope proof analysis/round-02-commentary-scope.json; handoff analysis/round-02-commentary-handoff.md. Fresh supported byte-identical verification analysis/round-02-commentary-verification.json and refreshed commentary/reading-after.asm. Map/state unchanged abfbf643f0f7d3612f9ac5484cbedd1c3b0e60fcc12023f2d0f15bf3e2bd23b9. ROM remains91.99% classified,20992 Unknown; no new execution claim. Zero readability backlog, final editorial audit pending. No toolkit issues found. Next redirect; ordinary incomplete phase transition, not terminal exhaustion.

## R02 redirect active
Unique attempt analysis/redirect/r02-contract-20261030/plan.md. Frozen abfbf643 map,20992 Unknown; improved balanced-stack and local bank contracts; no replay/classification.

R02 redirect checkpoint: fixed-slot continuation produced7749 preliminary Unknown Code bytes with0 exact saved-map boundary conflicts; new source roots and9 F8 contracts followed. FA49 one-hot table independently excludes a blind CFG fallthrough; copied reset-template review identifies prior generic Data byte007F90 requiring independent correction. Full records/current frontier in analysis/redirect/r02-contract-20261030/findings.md. Map/state/source remain unchanged.

R02 redirect ending correction: residual bank9 contains THE END/credits. Fixed D1D3 gate uses current guardian mask043C bit02 and explicit PRG4 -> BAE7. Source selector maps scene37(decimal) to A2BB; stream A2D1 sets043C02. Prior all-family1D /scene38 does not test this gate. Preparing supported transition+signed06A3 defeat test from persisted hall snapshot; exact frontier/evidence in current redirect findings.md. This is new concrete source evidence, not repeated flag brute force.

R02 supported ending route succeeded: final guardian37/Morticia (043C02), signed06A3FF defeat, Start dialog continuation, ending intro and credits; THE END captured frame5433. Reusable local snapshots and exact bounded route in current redirect findings/scenarios. Old failed routes preserved; ROM/state/map/source unchanged.

## R02 redirect ready handoff
analysis/redirect/r02-contract-20261030/handoff.md and final-verification.json. Proposed20966 Unknown bytes (11892 Code,9074 Data),plus explicit existing Data7F90->Code1;accepted0. Frozen input classified241168/262160,Unknown20992,hashabfbf643 unchanged. Six stable ready leads appended. Final combined audit0 conflicts/queue; hypothetical26 Unknown/21ranges with canonical fixed C000/E000 aliases. First-word behavior/motion tag and complementary-branch proof corrections retained; final THE END artificial route/snapshot recorded. Source hashes/input byte-identical verifier preserved; fresh verification required after next classification. No toolkit issues found; no toolkit/source/map/ROM edits,cleanup or publication.

Round03 disassembly active. Frozen abfbf643 state confirmed; independently reviewing six R02 leads and explicit7F90 correction. Primary handoff/clerical BF12 correction read; no gameplay replay or commentary edits. Review frontier analysis/round03-disassembly; proposal remains unaccepted pending code/bank/consumer review.

Round03 independent structural review accepted20966 new bytes plus separate7F90 correction; exact fresh23231context decodes and independently rederived field/record/union checks saved. Annotation active with durable700-span checkpoints. BF12 source correction is8644/type132; actual LDA84BF33 feeds ObjectType storeBF35. Old completed redirect remains immutable. Next validate/export/new checkpoint, byte-identical verification and per-bank handoff; commentary source preserved.


## Round03 disassembly completed; ROM incomplete

ROM classification: 99.99% (262134 / 262160 bytes classified; 26 bytes unknown).

Accepted20966 new Static bytes:11892 Code and9074 Data. Separately corrected one existing generic Data byte7F90 to the first copied-reset LDA opcode; original region5241 preserved in prior-region-correction.json and prior checkpoints, shortened245-byte zero region retains corrected rationale. Every other original region unchanged. All six R02 leads verified; no gameplay replay. Literal BF12 header lookup independently corrected to physical018654/CPU8644,type132; LDA84 atBF33 feeds ObjectType producer storeBF35. Completed redirect evidence remains immutable.

| PRG8KiB bank | Code | Data | Classified | Unknown |
|---|---:|---:|---:|---:|
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 81 | 8111 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 673 | 7519 | 8192 | 0 |
| 4 | 1571 | 6620 | 8191 | 1 |
| 5 | 178 | 8014 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 178 | 8014 | 8192 | 0 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 2463 | 5727 | 8190 | 2 |
| 10 | 2143 | 6049 | 8192 | 0 |
| 11 | 2821 | 5371 | 8192 | 0 |
| 12 | 3807 | 4383 | 8190 | 2 |
| 13 | 5093 | 3093 | 8186 | 6 |
| 14 | 7527 | 664 | 8191 | 1 |
| 15 | 7116 | 1062 | 8178 | 14 |

Header16 Data,CHR131072 Image; totals {'Data': 97411, 'Code': 33651, 'Unknown': 26, 'Image': 131072}. All16422regions have evidence. Provenance bytes {'Observed': 7660, 'Static': 254474} are structural annotation provenance, not execution coverage. No new runtime coverage measured in this pass; existing natural/artificial fetch records and ending route preserved. Inferred successors remain Static.

Independent primary review:23231 contextual instructions freshly decoded with configured binary; source-backed local stack/bank/restoring-wrapper/return witnesses checked with consumer-review.txt.13 graphics dimensions/payload endpoints,128 low table cells,35 credits records,21 intro pairs,script token bytes/lengths,8 reset copies,PPU rows/masks and exact union independently rederived. Review rationale independent-review.json; raw/field evidence rederived-structure.json,data-review.hex,decoded-code.json; annotations and saved validation in annotation-plan.json/annotations.json/save-report.json. Specific roles preferred over generic patterned margins; no original semantic subdivision invented.

map.validate passed. state/current/checkpoint byte-equal SHA256 bad6945c7ca45082e65fcee90ca02e94f805c1f3770062e0cdfbc4063e5d82d0. Fresh checkpoint maps/round-03-disassembly.json,generated/round-03-disassembly and analysis/round-03-disassembly-verification.json confirm byte-identical rebuilding. final-verification.json confirms ROM identity, all other original regions preserved and unchanged prior commentary source hashes.

Remaining26 Unknown bytes in21 intervals:25 literalRTS and oneE0 before reset. Exact current residual counts.json. Prior residual flanks/reasoning in analysis/redirect/r02-contract-20261030/residual-review-final.hex distinguish ambiguous empty routine/return epilogue from noncode padding/table residue; no sufficient entry/format fact resolves them in this bounded pass. No additional supported connected easy gain identified. Full classification/editorial completion not claimed; this is ordinary incomplete phase handoff, not terminal exhaustion. Next full current-source commentary, preserving prior commentary before preparation, then redirect residual reassessment.

No toolkit issues found; analysis/toolkit-issues.md,zero blockers. Removed-ID annotation attempt was correctly rejected before save, retried from frozen map with freshID; analyst API usage,not toolkit defect. No toolkit edits,cleanup,archive,publication or ROM modification.

## Round03 redirect residual investigation

Current phase redirect; attempt analysis/redirect/r03-residual-contract-20261030. Input map/state/frozen checkpoint bad6945c7ca45082e65fcee90ca02e94f805c1f3770062e0cdfbc4063e5d82d0; current-source readability gate complete (round-03-commentary-handoff.md). Exact complement refreshed from map regions:262134/262160 classified,26Unknown in21intervals. No classification or source changes. Source flanks, bank-aware direct/absolute reference scan and all-residual-byte lexical pointer cross-check persisted. Descriptor-tail/dispatcher/terminal epilogue contracts, short repeatedRTS Data alternative, table endpoints and pre-resetE0 assessed independently. No direct accepted instruction reference targets a residual byte; this bounded negative result is not a universal caller requirement or Data proof. Primary reasoning: attempt/rejection-review.md, source-flanks.txt, residual-ledger.json,lexical-review.txt. Frontier: coordinator inspect scoped rejection review; record final frozen-input checks and empty ready-lead result. No runtime replay or queued concrete experiment; no toolkit edits. Ending/THE END route reused by reference. Final editorial audit not invoked.

Round03 redirect completed investigation with0proposed/0accepted bytes and0ready leads; empty pass appended to leads.json. Terminal incomplete recommendation is subject to coordinator primary-reasoning inspection of analysis/redirect/r03-residual-contract-20261030/rejection-review.md. Exact26Unknown/21interval complement and localized competing interpretations persist in residual-ledger.json; compact findings/handoff adjacent. Frozen-verification.json confirms unchanged ROM,map/state/checkpoint,commentary source/symbols/reading copy; reuse unchanged fresh byte-identical commentary verifier. Current-source readability complete; final editorial audit separate and pending. No unfinished concrete test or useful justified next method remains; not an interrupted/blocked resumable frontier. No toolkit issues found,no cleanup/deletion/publication/toolkit edits.

Coordinator primary review accepted Round03 terminal-incomplete rationale after examining rejection reasoning,all residual source flanks,lexical matches,raw table/E0 boundaries and actual adjacent return callers. No concrete revised structural inference/test found. Final editorial phase remains separately authorized by coordinator; mapper-reset STA $8000 operand alias currently prints SignedWaveSamples and needs editorial correction. Redirect phase ended; no automatic new planning.


## Round03 final editorial acceptance

Final current-source editorial acceptance complete; supported audit passed ok=true,byte_identical=true,editorial_audit=true with16443 reviewed regions and16147 unit records (including14892 portions of existing meaningful units). Fresh configured-binary rebuild and reading-copy refresh complete. Fixed one mapper-reset STA $8000 misleading data alias; correct wave/MMC1 aliases retained. Inventory source paths normalized and spanning-unit portions supplied to meet supported audit contract; physical scaffold and review credit preserved. Full prior readability review reused; zero pending ranges. Findings: commentary/editorial-audit.md; targeted evidence analysis/round-03-final-editorial-alias-check.json. Final handoff: analysis/round-03-final-editorial-handoff.md; verification analysis/round-03-final-editorial-verification.json; identities analysis/round-03-final-editorial-identities.json. Classification remains terminal incomplete262134/262160 (99.99%),26Unknown; coordinator redirect rationale accepted. ROM,map/state/frozen checkpoints unchanged. No archive,cleanup,deletion,publication or toolkit edit performed.


Coordinator terminal handoff verified. Orchestration workflow reached justified terminal incomplete after substantive primary rejection review; ROM classification remains99.99% (262134 / 262160 classified;26 Unknown in21intervals), not whole-ROM complete. Final editorial acceptance passed with current-source byte-identical rebuild, reviewed full inventory and refreshed reading copy. Independent checks analysis/terminal-verification.json and requirement audit analysis/coordinator-completion-audit.json prove current identities, exact residuals, no ready lead queue and complete Markdown accounting. archive-options.json explicitly preserves final/rejection/editorial handoffs and historical plans/checkpoints; no disposable unrecognized Markdown exists, omitted_documents=[]. No archive,cleanup,publication or toolkit edits performed. Final checkpoint maps/round-03-disassembly.json; current readable source commentary/source/game.asm, commentary/reading-after.asm; final editorial handoff analysis/round-03-final-editorial-handoff.md. No required work remains for this orchestration objective; classification remains incomplete and can reopen only on materially new/corrected structural evidence or a justified discriminating method.
