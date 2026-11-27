# Round 002 disassembly complete; commentary next

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

Reusable evidence: `analysis/redirect/round-001-a/findings.md`, canonical manifests `code-proposals.json` / `data-proposals.json`, exact original claim roles `data-claims.json`, per-instruction parents `code-entry-evidence.json`, bank contracts `bank-preservation-contracts4.json`, special queue selectors `queue-call-sites.json`, decisive unresolved sinks `stop-contract-audit.json`. Source stays `commentary/source/game.asm` from round001 until next commentary; new generated checkpoint is `generated/round-002-disassembly-final`. Historical maps/source/failed probes remain intact. Read next phase prompt and policy normally; source readability is not yet met for this expanded classification. All11 lead acceptance amounts are in `lead-outcomes.json`.
