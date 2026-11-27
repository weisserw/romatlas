# Round 1 disassembly phase handoff


## Round 1 disassembly handoff — incomplete

ROM classification: 89.32% (234161 / 262160 bytes classified; 27999 bytes unknown).

Single authorized disassembly pass completed. Code 21382 bytes; non-code 212779 bytes; 1011 evidence-annotated regions (1011/1011 nonempty evidence). Exact physical unknown complement and all bank totals: `round01/final-metrics.json`. Header16/16 classified. CHR banks0–15 each8192/8192 classified Image,0 unknown.

| PRG bank | Code | Data | Classified | Unknown |
|---:|---:|---:|---:|---:|
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 8 | 8184 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 8 | 8184 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 8 | 8184 | 8192 | 0 |
| 6 | 5484 | 0 | 5484 | 2708 |
| 7 | 1780 | 4576 | 6356 | 1836 |
| 8 | 379 | 3572 | 3951 | 4241 |
| 9 | 462 | 6437 | 6899 | 1293 |
| 10 | 1064 | 4470 | 5534 | 2658 |
| 11 | 42 | 395 | 437 | 7755 |
| 12 | 1864 | 5681 | 7545 | 647 |
| 13 | 8 | 6774 | 6782 | 1410 |
| 14 | 6107 | 28 | 6135 | 2057 |
| 15 | 4168 | 630 | 4798 | 3394 |

Fresh checkpoint **pass-001**: map.validate passed; `maps/current.json` and `maps/pass-001.json`; generated `generated/pass-001`; `analysis/pass-001-verification.json` confirms ok=true,byte_identical=true. Map SHA256 `30a0e9f8324b21c1d9582312c4fde1da220b1d0f3600ed103cc5bb694765d69a`. Input ROM SHA256 unchanged `4a8b649b8ef253e178875f1a3f3614e16ac46484d8d49bb3ef7ff8f41633c818`. No toolkit edits/publishing/archive/cleanup.

Execution is separate:5928 unique physical fetch starts;11575 naturally observed opcode bytes;9975 artificial-scenario observed opcode bytes;12675 union (overlap counted once). Trace aggregate drops0 in saved scenario summaries; detail capacity0 intentionally discarded bulk events. Disabled-trace ending progression supplies frame/state evidence, no fabricated fetch coverage.

Natural: intro,title,equipment,stage1 flight/missiles/guns,Start reversal,Select menu. Artificial: later surface scenes2/4/6/8,final underground9,complete ending message and victory/STAFF/GRAPHICS credits. Boss-condition patch attempted without confirmed boss; rescue-specific/spatial contracts remain for redirect. All final outcomes and exact replay/patch budgets consolidated in `analysis/scenarios.json`; source hypotheses in `analysis/gameplay.md`.

Consumed leads:none (initial queue empty). Produced research leads R1-L001..R1-L004 in `analysis/leads.json`. Broad later-surface tests yielded14,0,17,0 new fetched code bytes; cave9 added7 and boss-condition6. Direct static paths, direct-call target closure and bounded stack/bank-preserving continuations were classified; remaining promising continuations need indirect-selector/return-stack/bank-restoration or mixed-block boundary research. No raw unknown gap was treated as classified. This is the easy-gain phase handoff to commentary/redirect, not terminal workflow exhaustion.

Five large non-code blocks (24006 bytes) accepted after precise neighboring boundaries, references/pattern inspection and checking every saved fetched instruction for conflicts; `round01/large-data-review.json`. These generic structural roles do not claim complete original schemas. Classification complete only for accepted ranges. Readability/editorial audit remains for commentary across all current classified source.

**Toolkit findings: one missing feature (NES-MF-001, literal constant-flag branch pruning), zero bugs, no blockers**; persistent report: [toolkit-issues.md](toolkit-issues.md). Latest verification valid; no live server required. Resume with configured binary via run.py serve,map.load analysis/state.json,emulator.start,import chosen persisted snapshot,then explicitly reassert buttons and configure/clear traces. All standalone round01 scripts are local reproducible evidence, not portable sharing artifacts. Scratch12KiB; analysis/round01~151MiB;3.6GiB free at handoff.
