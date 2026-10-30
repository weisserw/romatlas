# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.


## Round 1 evidence and outcomes

ROM classification: 61.06% (160069 / 262160 bytes classified; 102091 bytes unknown).

Internal map contains 509 nonoverlapping evidence-backed regions: Code10830, Data18167, Image131072 bytes. Counts derive from the internal store, not raw export. Header16 and all16 CHR8KiB banks classified; each CHR bank8192 classified/0 unknown. PRG physical8KiB identities:

| PRG bank | Code | Data | Classified | Unknown |
|---|---:|---:|---:|---:|
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 0 | 5572 | 5572 | 2620 |
| 2 | 0 | 0 | 0 | 8192 |
| 3 | 432 | 0 | 432 | 7760 |
| 4 | 0 | 186 | 186 | 8006 |
| 5 | 0 | 128 | 128 | 8064 |
| 6 | 0 | 188 | 188 | 8004 |
| 7 | 0 | 448 | 448 | 7744 |
| 8 | 0 | 184 | 184 | 8008 |
| 9 | 0 | 652 | 652 | 7540 |
| 10 | 235 | 186 | 421 | 7771 |
| 11 | 305 | 1438 | 1743 | 6449 |
| 12 | 2032 | 32 | 2064 | 6128 |
| 13 | 80 | 0 | 80 | 8112 |
| 14 | 5006 | 939 | 5945 | 2247 |
| 15 | 2740 | 6 | 2746 | 5446 |

Checkpoint [pass-001 verification](pass-001-verification.json) validates portable maps/pass-001.json and current.json, generates generated/pass-001, and rebuilds byte-identically to input. Input SHA256 remains 2719fb04f979dcd72060adfc33592ab9ea4f05b1117d28d69eea9537294962e5. Exact unknown complement and per-bank metrics: round1/unknown-ranges.json, round1/classification-summary.json. Execution is separate: 3937 unique physical accepted instruction starts across 9633 contextual roots; this is not byte coverage or natural sequential reachability. Zero aggregate drops in saved runtime summaries; detail capacity0 intentionally discards individual events. Untraced waits are explicitly marked in recipes.

Current frontier: all drivers exited; restore durable snapshots with a new configured server and load analysis/state.json. Ending staff credits verified visually in round1/ending-later-result.png after corrected optional-A route. Stage6 artificial later branch saved boss6-later.snapshot.json, RAM$037C=1; recognizable enemy/cannon mechanics exercised but specific boss not established. Stage3/5 deferred with exact route and reason in scenarios.json. See leads R1-STAGE-DATA, R1-FIXED-CLOSURE, R1-BOSS-PROGRESSION. Ordinary easy-gain phase handoff, not terminal exhaustion: redirect should investigate banked consumers/dispatch contracts. No further unchanged gameplay replay justified; unknowns remain substantial.

Toolkit: **no toolkit issues found**; [persistent report](toolkit-issues.md). No toolkit changes, publication, deletion or workspace cleanup. Round1 local artifacts approximately118MiB; >10GiB free at final check. Compact phase handoff: round1/disassembly-handoff.md.

Structural evidence: accepted fetches independently prove observed code; bounded graph closure with validated effective-bank views proves static descendants, with incompatible instruction-boundary filtering (zero conflicts). Mapper-affecting calls/writes stop static propagation. Header/CHR are physical-format claims, metasprite bounds combine minimum pointer endpoints with exact count4*n partitions and attribute bits, DMC structure combines actual consumer register writes and sample encodings. Uniform data runs are maximal32+ byte repeats outside code graph, generic data with purpose unresolved. Never infer data merely from nonexecution.

No preexisting leads existed. All new attempted IDs and specific retry guidance are preserved in leads.json. Ready structural slices were applied; unresolved stage-format and control contracts are redirect research, not grounds to weaken classification. Final late probes yielded419 additional code bytes (smaller than initial6006-byte gameplay batch); no repeated route is needed without a new target. Ending credits achieved; major boss transition contract remains useful intermediate question.


## Round 2 independent structural acceptance — incomplete ROM

ROM classification: 100.00% (262158 / 262160 bytes classified; 2 bytes unknown).

The displayed percentage rounds to100.00%; two bytes remain unknown and completion is not claimed. Exact fraction262158/262160 = 99.999237107%. Accepted102089 new bytes (Code8773,Data93316), all Static; existing classifications/provenance preserved. Internal totals: Code19603,Data111483,Image131072;695 nonoverlapping regions. Header16/16; each of16 CHR8KiB banks8192 classified/0 unknown.

| PRG bank | Code | Data | Classified | Unknown |
|---|---:|---:|---:|---:|
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 39 | 8153 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 2254 | 5938 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 39 | 8153 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 39 | 8153 | 8192 | 0 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 39 | 8153 | 8192 | 0 |
| 10 | 657 | 7535 | 8192 | 0 |
| 11 | 703 | 7489 | 8192 | 0 |
| 12 | 2085 | 6106 | 8191 | 1 |
| 13 | 476 | 7716 | 8192 | 0 |
| 14 | 6877 | 1314 | 8191 | 1 |
| 15 | 6395 | 1797 | 8192 | 0 |

Independent review: [findings](round2/review-findings.md), [checks](round2/review.json), [accepted batch](round2/classification-batch.json). Rechecked4278 instruction records, exact physical bank views, record/pointer/terminator boundaries and code/data conflicts (none). Structural/static classification is separate from natural execution. Saved gameplay later-stage/credits routes reused; no unchanged replay. Two unknown isolated RTS offsets99897 ($8629,banks12/13/14/15) and121601 ($DAF1,fixed14/15) have no discriminating static entry or data constraint; retained for commentary/redirect.

Actual output checkpoint **pass-002-reviewed**: [verification](pass-002-reviewed-verification.json), map maps/pass-002-reviewed.json, generated/pass-002-reviewed; validated and rebuilt byte-identically. Current map SHA256 5e38346a7db9fab08897b82d33c5f1ba7c862ded185f9ddc714248eb1dbaf9c6. maps/current.json and analysis/state.json saved. `pass-002` is an unchanged-input historical checkpoint accidentally created after rejected map.annotate missing-id request; no annotation saved in that attempt. This is local orchestration error, not toolkit defect. Preserved all historical artifacts. Existing commentary/source SHA256 remains5afc0ed8bf35642faf1e7d289b25203b7432e07ab959f8ccbfa557c1e74fdfb6; commentary must catch up the accepted batch independently.

All ready structural lead IDs verified together; combined lead is not additive with component leads. R1-BOSS-PROGRESSION remains scenario-limited; no relation to residual byte questions established. New R2-ISOLATED-RTS records exact remaining uncertainty. Current phase: disassembly complete, commentary next; no terminal orchestration decision. Toolkit: **no toolkit issues found**; [persistent report](toolkit-issues.md). No toolkit edits, publication, archive or deletion.
