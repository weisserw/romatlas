# Round03 disassembly handoff

One pass complete; ROM incomplete. Next full current-source commentary, then redirect.


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
