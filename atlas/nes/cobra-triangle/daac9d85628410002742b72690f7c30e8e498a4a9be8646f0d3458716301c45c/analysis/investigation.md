# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## Round 001 — disassembly, 2026-11-27

Established a byte-identical raw baseline, then classified actual fetched instructions from startup, attract/demo, eight distinct stage types and ending/credits, plus supported static vector/dispatcher control flow. The input SHA-256 remained unchanged. Structural evidence and reachability are separate: initial startup fetches are natural; stage/timer edits are artificial; unobserved direct paths and dispatcher siblings are Static.

- **R1-BOOT (verified):** reset vectors in each 32 KiB AxROM image select the same reset address with effective 8 KiB PRG banks [0..3], [4..7], [8..11], [12..15]. Physical header, vector records, boot signature and 48-byte boot RAM defaults were separately bounded. See `round-001/discover-vectors.json`, `inspect-initial-13.json`, `uniform-inspection.json`.
- **R1-STATIC-INLINE (rejected, narrowed):** automatic discovery's apparent ISC at physical $1A55E was a false branch fallthrough into `FF FF C0 FF 00`, indexed controller masks before the next independently fetched instruction $1A563. Accepted a five-byte Data barrier, not code. This does not reject neighboring valid code. See `inspect-initial-33/34/35/36.json`; static reachability through always-taken branches is not a table-bound proof by itself.
- **R1-ATTRACT (verified):** early Start requests using shared button/pressed schema were ignored by NES; complete buttons arrays fix the API use. A separate noninteractive attract flag and slow countdown still prevented short title routes. NMI decrements $9F every 16 frames; snapshot-backed patches $9F=1,$02=15 expose the next transition. Recipes and all inputs remain in `round-001/rpc.jsonl`, `skip_attract.py`, `scenarios.json`.
- **R1-STAGE-ENTRY (verified after corrected failed premise):** initial PC edits at the snapshot frame boundary were immediately interrupted by a pending NMI, so the written stage numbers alone did not prove visits. `entry-trace.json` discriminates this failure. Advance 10,000 cycles and finish one operation before editing PC to bank-0 CPU $816D; this understood nonreturning initialization entry consumes $0737/$0702 and initializes geometry/type. Exact patches, registers, inputs, budgets and snapshots are in `stage_visits_v2.py`, `visit_one.py` and `scenarios.json`. Confirmed rendered courses, guard-people ring, waterfall terrain and ending tableau; objective completion for each course is not claimed.
- **R1-DISPATCH (verified static batch):** FF80 thunk stores X into RAM $E4, selects a bank and calls $8000, whose JMP ($00E4) uses high byte $E5=$80. Independently bounded consecutive JMP stub tables contain 6/6/11/33 entries in the four AxROM banks. Body control/data-flow and exact decoded boundaries support static code without demanding natural selector use. Accepted 3,086 additional bytes through this connected batch; see `dispatch-tables-inspection.json`, `dispatch-roots.json`, `dispatch-discovery-*`, `dispatch-inspect-*`.
- **R1-ENDING (verified artificial):** selector49, preceding display count24 enters map26/type9, the five-plane rescue ending. A mistaken `$05FF=0` boss interpretation was corrected: it was already the ending and proves no boss defeat. Restored the clean pre-edit snapshot for automatic rescue progression. Traced milestone samples after bounded untraced waits reached staff credits and thanks-for-playing (`credits-2.ppm`, `terminal-0.ppm`). Natural full-game completion is not claimed.
- **R1-RAM-PC (rejected inference):** people-defense frame stepping returned a transient mid-instruction PC $0095. That next-PC state is not an accepted instruction fetch. `ram_check.py` / `ram-instruction-check.json` records subsequent actual ROM fetches, not RAM execution. No ROM upload-origin claim or RAM-derived ROM annotation was made. Retained coverage metrics count observed ROM roots separately.

A process-local trace was lost before all v2 visits could export their roots. Durable snapshots were retained; short six-frame replays recovered actual context evidence without repeating complete initialization tests. Initial skip-context capture was capped at 5,000 roots. Ending bulk waits were deliberately untraced with short traced samples at milestones; aggregate and detail-drop limits are explicit in saved summaries. Zero detail capacity intentionally drops event detail, while aggregate_dropped remained zero in retained scenario summaries.

This is a completed bounded easy-gain pass, not terminal exhaustion of the ROM. Remaining work is investigation of data consumers/formats, indirect state dispatch and residual code-vs-data boundaries, with full-source commentary still pending. Do not infer that unexecuted bytes are data or dead. Future redirect should use saved snapshots rather than repeat title play.

### Round 001 final classification and verification

ROM classification: 17.55% (23000 / 131088 bytes classified; 108088 bytes unknown).

| PRG 8 KiB bank | Code | Data/text | Classified | Unknown |
|---:|---:|---:|---:|---:|
| 0 | 3001 | 9 | 3010 | 5182 |
| 1 | 1651 | 0 | 1651 | 6541 |
| 2 | 3371 | 48 | 3419 | 4773 |
| 3 | 1898 | 134 | 2032 | 6160 |
| 4 | 1381 | 0 | 1381 | 6811 |
| 5 | 1637 | 0 | 1637 | 6555 |
| 6 | 1079 | 0 | 1079 | 7113 |
| 7 | 321 | 730 | 1051 | 7141 |
| 8 | 886 | 0 | 886 | 7306 |
| 9 | 0 | 0 | 0 | 8192 |
| 10 | 34 | 1025 | 1059 | 7133 |
| 11 | 298 | 134 | 432 | 7760 |
| 12 | 2578 | 0 | 2578 | 5614 |
| 13 | 2011 | 5 | 2016 | 6176 |
| 14 | 50 | 0 | 50 | 8142 |
| 15 | 388 | 315 | 703 | 7489 |

Header separately: 16 Data bytes. Final checkpoint analysis/round-001-disassembly-final-verification.json validates and rebuilds byte-identically. Code 20584 bytes; data/text 2416 bytes; structural evidence 782/782 regions. Full-source commentary pending. Final shark appearance and combat controls verified after long intro (`boss-long-0.ppm`, `boss-fire.py`); boss defeat unproved. Retained fetch coverage and its bounded limitations are in classification-counts.json. Toolkit: one confirmed issue NES-001, no blockers; see toolkit-issues.md.

## Round 002 disassembly complete; commentary next

ROM classification: 88.64% (116193 / 131088 bytes classified; 14895 bytes unknown).

ROM remains incomplete; this is one completed substantive pass, not terminal research exhaustion. Accepted93193 new Static bytes (23072 Code/70121 Data) in482 groups from11 ready leads. Full independently supported connected batch accepted; no canonical proposal rejected. Preserved exclusions: five controller masks remain Data, contradictory stage table25355..25365 remains Data despite conditional JMP, isolated RTI5597 remains Unknown, graphics reads overlapping instructions keep Code primary. No new runtime/natural reachability claim. Reused existing broad artificial later-game/ending/credits routes and saved packed-consumer probes; no redundant replay.

Checkpoint `maps/round-002-disassembly-final.json`; internal `analysis/state.json`; portable `maps/current.json`; fresh verification `analysis/round-002-disassembly-final-verification.json` confirms byte-identical assembly. Code43656; other classified72537 (including16-byte header). Exact counts/residuals: `analysis/round-002/classification-counts.json`, `unknown-ranges.json` (199 intervals). All newly accepted annotations carry Static provenance and evidence. Review: `analysis/round-002/review.md`, independent checks `independent-review.json`, reproducible `review.py`; annotation transport `annotation-rpc.jsonl`. Lead outcomes: `analysis/round-002/lead-outcomes.json` and preserved history in `analysis/leads.json`.

| PRG 8 KiB bank | Code | Data/text | Classified | Unknown |
|---:|---:|---:|---:|---:|
| 0 | 5558 | 9 | 5567 | 2625 |
| 1 | 6728 | 127 | 6855 | 1337 |
| 2 | 6469 | 282 | 6751 | 1441 |
| 3 | 6128 | 384 | 6512 | 1680 |
| 4 | 1434 | 6742 | 8176 | 16 |
| 5 | 3412 | 4190 | 7602 | 590 |
| 6 | 1797 | 6307 | 8104 | 88 |
| 7 | 354 | 7739 | 8093 | 99 |
| 8 | 967 | 7035 | 8002 | 190 |
| 9 | 0 | 8192 | 8192 | 0 |
| 10 | 34 | 8158 | 8192 | 0 |
| 11 | 316 | 7759 | 8075 | 117 |
| 12 | 3697 | 3694 | 7391 | 801 |
| 13 | 5106 | 560 | 5666 | 2526 |
| 14 | 1205 | 4458 | 5663 | 2529 |
| 15 | 451 | 6885 | 7336 | 856 |


Next: current-source commentary for all newly classified code/data, preserving existing prose/history. Correct three known prior-source statements: reverse-bit mode consumes16 source bytes/tile; packed low-counter0 adds256 output bytes; stage initialization consumes COUNT literal bytes rather than repeated constants. Then redirect reviews residual source gaps/199 exact physical intervals and63 saved missing callee/indirect bank contracts. No universal post-bankcall propagation or pointer-only code assumption. No currently ready lead remains unattempted. Full-source readability and final editorial audit remain separate gates.

Toolkit:1 confirmed documentation/API-validation issue NES-001,1 documented precision missing feature NES-002,0 blockers; [toolkit-issues.md](analysis/toolkit-issues.md) preserved. Toolkit and ROM unchanged; no archive/deletion/publication. Mutations ceased after final records/checkpoint. Handoff: `analysis/handoffs/round-002-disassembly.md`.


## Round003 disassembly complete; commentary next

ROM classification: 98.95% (129712 / 131088 bytes classified; 1376 bytes unknown).

Accepted13519 new Static bytes:3144 Code/10375 Data in157 groups from R2-CODE-CONTRACTS and R2-DATA-CONSUMERS. Canonical3144 Code batch accepted fully; four59-byte uniform Data claims shortened to56-byte prefixes. Rejected12 suffix bytes at physical32767,65535,98303,131071 (three bytes each, CPUFFEF), which contain JMPs rather than uniformFF. They remainUnknown pending independent entry/role evidence. No new observed execution or natural reachability claim. One substantive pass completed; ROM incomplete, not terminal research exhaustion.

Fresh checkpoint maps/round-003-disassembly-final.json; internal analysis/state.json; portable maps/current.json; generated/round-003-disassembly-final. Fresh analysis/round-003-disassembly-final-verification.json confirms byte-identical rebuild. Classified Code46800; other82912 including16-byte header. Exact counts/bank totals and67 unknown intervals: analysis/round-003/classification-counts.json and unknown-ranges.json. Review/evidence: review.md, independent-review.json, review.py, accepted-*-proposals.json, localized-rejections.json, annotation-rpc.jsonl, map-validation.json; lead-outcomes.json and preserved analysis/leads.json histories.

| PRG 8 KiB bank | Code | Data/text | Classified | Unknown |
|---:|---:|---:|---:|---:|
| 0 | 6143 | 2016 | 8159 | 33 |
| 1 | 6805 | 927 | 7732 | 460 |
| 2 | 7046 | 1049 | 8095 | 97 |
| 3 | 6288 | 1302 | 7590 | 602 |
| 4 | 1434 | 6758 | 8192 | 0 |
| 5 | 3449 | 4723 | 8172 | 20 |
| 6 | 1797 | 6391 | 8188 | 4 |
| 7 | 369 | 7820 | 8189 | 3 |
| 8 | 967 | 7225 | 8192 | 0 |
| 9 | 0 | 8192 | 8192 | 0 |
| 10 | 34 | 8158 | 8192 | 0 |
| 11 | 325 | 7864 | 8189 | 3 |
| 12 | 3722 | 4448 | 8170 | 22 |
| 13 | 6556 | 1570 | 8126 | 66 |
| 14 | 1399 | 6730 | 8129 | 63 |
| 15 | 466 | 7723 | 8189 | 3 |


Broad gameplay through later areas/boss/ending/credits and saved discriminating probes reused; no redundant run. PreservedAD05 operand/AD06 data, constant-takenB500/B502 data, BE16 five-byte exclusion, E2FB Data and unresolved global state contradiction, code-primary graphics reads, isolatedRTI and local mapper/stack barriers. Conditional static routine preconditions are explicit; unresolved actual selection/return facts remain separate.

Next commentary must cover all currently classified source while preserving entire prior commentary. Correct EB8B consumer to26-entry response tableC121..C155; independent C109 remains eight-state table. Current commentary/source/game.asm untouched; final editorial gate pending. Then redirect examines exact residuals plus newFFEF rejection; strongest prior islands need independent entry/register/stack/bank evidence per analysis/redirect/round-002-a/residual-obstacles.md. No terminal exhaustion claimed.

Toolkit: existing confirmed NES-001 documentation/API issue and NES-002 precision missing feature, zero blockers, no new issues; [toolkit-issues.md](analysis/toolkit-issues.md) preserved. ROM, toolkit and commentary source unchanged; no archive/deletion/cleanup/publication.

## Round004 disassembly complete; commentary next

ROM classification: 99.98% (131058 / 131088 bytes classified; 30 bytes unknown).

Accepted all59 R3-LOCAL-STRUCTURE claims:1346 Static bytes=1310 Code+36 Data;562 instructions independently checked and freshly decoded by configured release binary. Local B63A prose corrected: B63E wrap exit or B640 JMP B5B9 comparator repeat, not B5B5. No new execution/natural reachability assertion. Exact independent semantic/context/physical review: analysis/round-004/review.md and independent-review.json; accepted-proposals.json and annotation-rpc.jsonl. No slices rejected. Structural classification incomplete; not terminal exhaustion.

Fresh map.validate passed; state analysis/state.json and maps/current.json saved; maps/round-004-disassembly-final.json generated/round-004-disassembly-final freshly rebuilt byte-identical (analysis/round-004-disassembly-final-verification.json). Code48110; other82948 including16-byte header; exact30 unknown bytes/11 slices in analysis/round-004/unknown-ranges.json. Every accepted Code has explicit physical effective-bank CPU view and Static evidence. Annotation coverage complete for accepted regions; execution evidence unchanged from saved prior actual-fetch lower bound, separate from structure.

| PRG8KiB bank | Code | Data | Classified | Unknown |
|---:|---:|---:|---:|---:|
|0|6167|2016|8183|9|
|1|7255|927|8182|10|
|2|7143|1049|8192|0|
|3|6850|1338|8188|4|
|4|1434|6758|8192|0|
|5|3469|4723|8192|0|
|6|1801|6391|8192|0|
|7|372|7820|8192|0|
|8|967|7225|8192|0|
|9|0|8192|8192|0|
|10|34|8158|8192|0|
|11|328|7864|8192|0|
|12|3739|4448|8187|5|
|13|6620|1570|8190|2|
|14|1462|6730|8192|0|
|15|469|7723|8192|0|

FFEF saved-continuation producer/return, parameterized queue tail contract, display/PPU upload, actor heading/spawn, exact87/36F3B8 split, collision counterparts, cursor restores and tiny clear-scroll/full-record-skip/speed-stop variants accepted on independent consumer/control constraints. Conditional collision91C1/shared00/local bank preconditions remain separate from actual caller guarantees. No unsupported reachability or deadness inferred.

Broad gameplay later areas/boss/ending/credits and discriminating E2FB probes reused; no redundant replay. Preserved E2FB Data contradiction and other historical barriers. Next commentary must cover all current source, including new code/data and stale RTI correction: physical5597 is acceptedData5575..5607; actual unknown pair physical811/CPU831B=40/70. Commentary source untouched, readability scope needs refresh; final editorial audit pending. Then redirect uses exact30-byte residual and reviewed competing interpretations, without another unchanged decode/state-fabrication loop.

Toolkit: existing1 confirmed issue NES-001,1 missing feature NES-002,0 suspected/0 blockers; no new toolkit issues. [toolkit-issues.md](toolkit-issues.md) preserved. ROM SHA256 unchanged; configured toolkit unchanged. No archive/deletion/cleanup/publication.

## Round005 disassembly complete; full current-source commentary next

ROM classification: 99.98% (131063 / 131088 bytes classified; 25 bytes unknown).

Accepted R4-SHARED-TARGET-RESET5StaticCode at physical9813..9818/G0A645..A64A,banks[0,1,2,3]. Independent review combines known043F zero endpoint and accepted conditional no-call frame consumer; actual entry/natural selection remains unresolved. Original rejection and redirect correction preserved. Exact evidence analysis/round-005/review.md, independent-review.json,accepted-proposals.json,annotation-rpc.jsonl. Connected consumer/tail bytes already classified; ten remaining local ambiguities supply no newly discriminating contract. No unchanged gameplay/artificial-entry replay. One pass complete; ROM incomplete, no terminal exhaustion assertion.

Fresh configured decode, map.validate/save/export and byte-identical checkpoint succeeded: maps/round-005-disassembly-final.json; analysis/round-005-disassembly-final-verification.json; generated/round-005-disassembly-final. Internal analysis/state.json and portable maps/current.json saved. Code48115,other82948; exact counts/perbank and complement analysis/round-005/classification-counts.json and unknown-ranges.json. All classified Code has explicit CPU bank views/evidence; no new Observed bytes or gameplay coverage. Existing broad progression/boss/ending/credits reused.

Per-bank code/data/unknown: 0:6167/2016/9; 1:7260/927/5; 2:7143/1049/0; 3:6850/1338/4; 4:1434/6758/0; 5:3469/4723/0; 6:1801/6391/0; 7:372/7820/0; 8:967/7225/0; 9:0/8192/0; 10:34/8158/0; 11:328/7864/0; 12:3739/4448/5; 13:6620/1570/2; 14:1462/6730/0; 15:469/7723/0. Header16 classified non-code bytes separate. Residual physical277:1;811:2;2801:3;3140:3;11951:3;14251:2;25939:3;30710:1;106162:5;113405:2.

Source commentary unchanged and expanded-scope readability refresh now required; replace former unknown reset prefix with meaningful reset/frame contract. Review stale current source33832 “adjacent unknown reset continuation” wording. Final editorial remains pending. Toolkit report analysis/toolkit-issues.md retains1confirmed issue/NES-001 and1missing feature/NES-002,0suspected,0blockers,no new finding; toolkit unchanged. ROM unchanged; no archive/deletion/cleanup/publication.
