# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.


# Round 01 disassembly handoff

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

Round01 redirect complete-ready: analysis/redirect/r01-scene-contract-20261030/handoff.md. Combined87913 previously unknown-byte proposal, accepted0; frozen classification151767 unknown110393. Final audit no code/data or instruction alignment conflicts; helper transforms resolved and FC99 operand root corrected toFC9A. Exact final bank-aware unknown ledger and per-bank counts retained. Gameplay includes train,three guardians,later rooms and Granny rescue/password; global ending unproven. All five stable lead IDs ready with exact ranges/recipes. No toolkit issues found; no blockers. Next independent disassembly, not terminal exhaustion.


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

## R02 redirect combined batch ready
Attempt analysis/redirect/r02-contract-20261030/handoff.md. Frozen abfbf643 map:241168 classified,20992 Unknown. Proposed20966 Unknown (11892 Code,9074 Data),plus1 existing Data->Code correction7F90;accepted0. Six ready leads;0 final boundary/kind conflicts and0 queued work. Full source-derived callback/stack/bank contracts, bounded tagged scripts,13 tile containers,waves,reset copies,reviewed Data margins and final-guardian ending route. THE END visited artificially atframe5433; previous unsuccessful routes retained. Hypothetical26 residual bytes remain ambiguous; current map/source unchanged. Next independent disassembly/full commentary/redirect; no terminal-exhaustion claim.


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
