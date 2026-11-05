
## Round 5 disassembly — pass-014

ROM classification: 99.99% (131081 / 131088 bytes classified; 7 bytes unknown).

Accepted R4-L01/L02/L03 independently:22 bytes (13 Static Code +9 Static Data). Total code 48860; data/text/header 82221. Exact residuals723D(1),15F7B(1),1FFC7..1FFCC(5), physical half-open. Review/evidence: analysis/round5/review.md and rpc.json. Preserved every existing accepted region unchanged; no source edits. Operand-byte FDE6 alternate entry recorded on new transform annotation; retain primary INC56/PLP/RTS representation and explain numeric call explicitly in subsequent commentary. No unchanged-decoder, intended-X or natural-use claim.

| Physical bank8K | Code | Data/text | Classified | Unknown |
|---|---:|---:|---:|---:|
| header | 0 | 16 | 16 | 0 |
| 0 | 334 | 7858 | 8192 | 0 |
| 1 | 775 | 7417 | 8192 | 0 |
| 2 | 3031 | 5161 | 8192 | 0 |
| 3 | 1065 | 7126 | 8191 | 1 |
| 4 | 424 | 7768 | 8192 | 0 |
| 5 | 5603 | 2589 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 372 | 7820 | 8192 | 0 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 3858 | 4334 | 8192 | 0 |
| 10 | 6408 | 1783 | 8191 | 1 |
| 11 | 6146 | 2046 | 8192 | 0 |
| 12 | 3335 | 4857 | 8192 | 0 |
| 13 | 2559 | 5633 | 8192 | 0 |
| 14 | 6938 | 1254 | 8192 | 0 |
| 15 | 8012 | 175 | 8187 | 5 |


Map validation and pass-014 byte-identical rebuild pass (analysis/pass-014-verification.json). Connected continuations closed at accepted fixed helpers/returns; no concrete additional easy gain emerged. No runtime scenarios replayed: prior broad artificial progression/quest-over coverage reused (analysis/scenarios.json). Seven bytes remain structurally ambiguous; see review.md for scoped alternatives. Single pass complete; ROM incomplete, next full-current-source commentary then redirect; final editorial audit pending. Existing NES-001/NES-002 nonblocking (analysis/toolkit-issues.md); no new toolkit issues found. No toolkit edits, source/commentary edits, deletion, archive, publication or subagents. Disk8.3GiB free/scratch17MiB.

Compact evidence: counts.json, review.md, accepted-regions.json, decode-*.json, rpc.json; state-before.json preserves input store. Current checkpoint maps/pass-014.json / generated/pass-014.
