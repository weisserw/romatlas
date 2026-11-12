# Round 001 disassembly handoff

## Round 001 disassembly — complete phase, ROM incomplete

Input checkpoint: initialized placeholder state; no consumed leads. Output:
analysis/state.json and maps/current.json. Final verified checkpoint:
`round-001-disassembly-final` (maps/round-001-disassembly-final.json,
generated/round-001-disassembly-final,
analysis/round-001-disassembly-final-verification.json). Earlier checkpoint
round-001-disassembly preserved; final adds a localized non-code record suffix.

ROM classification: 90.76% (59496 / 65552 bytes classified; 6056 bytes unknown).

| Physical section / 8 KiB bank | Code | Data/text/image | Unknown |
| --- | ---: | ---: | ---: |
| Header | 0 | 16 | 0 |
| PRG 0 | 6258 | 759 | 1175 |
| PRG 1 | 2740 | 3047 | 2405 |
| PRG 2 | 5997 | 306 | 1889 |
| PRG 3 | 1453 | 6152 | 587 |
| CHR 0 | 0 | 8192 | 0 |
| CHR 1 | 0 | 8192 | 0 |
| CHR 2 | 0 | 8192 | 0 |
| CHR 3 | 0 | 8192 | 0 |

All accepted regions carry structural evidence and provenance. Code has explicit
CPU execution views for physical PRG slots [0,1,2,3]; CHR has no CPU view.
Vector-rooted discovery plus observed branches produced 7,258 unique instruction
candidates, 16,448 code bytes. Expanded/final discoveries preserved under
analysis/round-001. The final merge yielded zero additional instructions, showing
local static closure under the current roots, not proof that all code is reached.

Broad exploration: natural startup, map selection and street inputs; artificial
initialized school and dance visits, cafe visit, later street initialization
at stage $0F, driving attempt, and explicit terminal success-screen entry.
See scenarios.json and gameplay.md for exact routes, budgets, distinctions and
obstacles. Driving frames remained black; speed patch did not establish a visit
or success. A direct terminal-screen entry displayed congratulations, but does
not prove the full driving completion transition. Street-completion RAM recipe
exposed bonus-count underflow/long countdown; superseded by understood main-loop
entries, not endlessly repeated. Later street stage initialization attempted,
with distinct-frame evidence retained but full gameplay visit not claimed.

Execution is separate: trace-summary.json records 5,425 context/provenance-qualified ROM instruction
records (3,397 distinct physical starts) and 5,911 edges; zero aggregate drops. Trace detail capacity was zero
after startup; 7,664,122 detail events were intentionally dropped. Some waits
ran without tracing for speed, so this is partial execution coverage. Snapshots
are local, build-specific and contain ROM backing; never portable maps.

No input leads existed. New concrete redirect questions: interpreter handler
contracts/extent at $8B7C and indirect sites in final-discovery.json; split mixed
unknown code/data intervals (notably file $32FD..$39D3 and $5A0A..$625B);
resolve driving initialization prerequisites/black output, and natural ending
transition. Exact complement is in round-001/unknown-ranges.json.

Verification: map.validate passed; configured release binary generated ca65
source and rebuilt byte-identically. Input SHA-256 remains workspace identity.
Toolkit findings: one confirmed documentation/API inconsistency (NES-001),
zero suspected bugs or analysis blockers; see toolkit-issues.md. Workaround uses
max_instructions. Toolkit was not changed. No publish/archive performed.

Commentary remains for all current classified source. This phase did not run commentary or redirect.
