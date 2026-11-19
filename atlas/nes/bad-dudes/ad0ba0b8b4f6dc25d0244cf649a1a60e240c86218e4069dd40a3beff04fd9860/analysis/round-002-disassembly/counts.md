
## ROUND002 disassembly — accepted structural counts and verification

ROM classification: 91.65% (240281 / 262160 bytes classified; 21879 bytes unknown).

Accepted74695 new bytes:25289Code and49406Data; corrected1 existingCode byte to
Data at physical576. Totals:47711Code,61498Data,131072Image,21879Unknown. Header16
andCHR131072 are classified. Exact physical unknown complement has328ranges.
Independent structural adjudication, boundaries and contracts:
analysis/round-002-disassembly/adjudication.md; independent-checks.json and
statistics.json. Accepted manifest and map.annotate transcript retained there.

| PRG bank | Code | Data | Unknown |
|---|---:|---:|---:|
| 0 | 2237 | 5954 | 1 |
| 1 | 0 | 8182 | 10 |
| 2 | 0 | 8192 | 0 |
| 3 | 0 | 8192 | 0 |
| 4 | 0 | 8192 | 0 |
| 5 | 258 | 6765 | 1169 |
| 6 | 2213 | 1 | 5978 |
| 7 | 5448 | 139 | 2605 |
| 8 | 2579 | 2409 | 3204 |
| 9 | 6509 | 1629 | 54 |
| 10 | 6841 | 1341 | 10 |
| 11 | 7139 | 1050 | 3 |
| 12 | 521 | 7671 | 0 |
| 13 | 3328 | 1502 | 3362 |
| 14 | 5933 | 12 | 2247 |
| 15 | 4705 | 251 | 3236 |

Execution is separate:13920 physical Code bytes carry Observed provenance,
33791 are Static. The new batch includes772 artificial fetched bytes and24517
Static bytes; existing natural/artificial observed provenance is preserved.
Observed map provenance is not a whole-runtime execution-coverage percentage.
Annotation coverage is240281 unique structural bytes; generic Data has evidence
and explicit role limits, with no invented consumer or natural-selection claims.

Fresh checkpoint: maps/round-002-disassembly.json and generated/round-002-disassembly.
analysis/round-002-disassembly-verification.json confirms byte_identical:true.
map.validate passed; internal state/export/checkpoint agree. Existing classifications
preserved except documented duty-byte correction; originalROM and priorcommentary
source/reading/historical checkpoints hashes unchanged (preservation-checks.json).
The expanded current map needs a fresh full-source commentary pass; prior completed
commentary is preserved, not reused as automatic credit for newly classified code.
Final editorial audit and whole-ROM completion remain pending.

Lead outcomes: L001/L002/L003/L004/L007/L008/L009/L010 verified for exact accepted
structural slices; global history appended. L005 verified/L006 local rejection
unchanged. Broader residual mixed screen/event/player/path/engine structures need
local entry/boundary/bank/schema investigation in redirect, not arbitrary byte
classification or repeated unchanged play. This is an ordinary incomplete handoff,
not a terminal whole-ROM exhaustion assessment.

Toolkit report: analysis/toolkit-issues.md retains1 confirmed documentation/API
bugNES-001,1 grounded missingfeatureNES-002,0 suspectedbugs,no blocker. Fullbuttons
lists and full-address-domain static bank checks worked. No toolkit edit/build
substitution, publication, cleanup, deletion or archive performed.
