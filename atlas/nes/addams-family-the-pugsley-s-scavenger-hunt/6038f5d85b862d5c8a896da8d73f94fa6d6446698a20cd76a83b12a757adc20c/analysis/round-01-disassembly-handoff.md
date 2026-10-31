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
