# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## Round01 disassembly, intermediate
- Baseline and pass-001 byte-identical verification passed. Internal state and portable current map saved.
- Fixed NROM context effective PRG banks [0,1,2,3]. Validated vectors physical $800A-$800F: NMI $8471/reset $8000/IRQ $84FF.
- Connected vector discovery and 11 supported NMI handler roots ($8569 pointer table, $F4 0..10) produced 11,306 candidate code bytes. Reviewed code boundaries and three false fallthroughs: ORA #$10/BNE $A9A6 cannot fall into sprite records $A9A8; LDA #0/BEQ $BFA9 cannot fall into 12 audio period words $BFAB; nonzero LDA/BNE $C17F cannot fall into audio streams $C181. Removed 16 false code bytes before classification. See evidence/round01/broad-code-review.txt and classify.py.
- Header, CHR tile section, uniform FF tail physical $4C34-$800A and bounded visibly noncode arrays annotated Static. Generic fallback follows boundary/reference/pattern review; exact semantic schemas remain open. Uniform suffix is Data, not an asserted fill purpose.
- Input API correction and broad gameplay status are recorded in gameplay.md and toolkit-issues.md. Initial attract execution is not claimed as natural gameplay. Corrected menu-driven broad trial ongoing; no terminal exhaustion decision made in this easy-gain handoff.

## Round01 disassembly complete — easy-gain handoff, incomplete ROM

ROM classification: 94.88% (38876 / 40976 bytes classified; 2100 bytes unknown).

Code 11,290 bytes; Data 19,394 bytes (includes header/vectors); Image 8,192 bytes. These are unique structural bytes from internal state, not execution coverage.

| Physical section | Classified | Unknown |
|---|---:|---:|
| Header | 16 | 0 |
| PRG0 | 7762 | 430 |
| PRG1 | 6622 | 1570 |
| PRG2 | 8092 | 100 |
| PRG3 | 8192 | 0 |
| CHR0 | 8192 | 0 |

Final map validates. Checkpoint **pass-002** generated/pass-002 verified byte-identical against unchanged input/game.nes. Portable map maps/pass-002.json and maps/current.json SHA-256 ce45be0835a8cfcc257414a9ba00a884e3bfb25b46c8c9474a2159b14b959e04; source/report analysis/pass-002-verification.json. Baseline and pass-001 retained. Final counts and exact 19 residual physical intervals: evidence/round01/counts.json and unknown-ranges.json.

Corrected normal controller trials visited all five skill levels through actual skill selection, exercised held [b,up], [a,up] and [up], and collected 1,667 accepted natural opcode starts; no conflicts with data and no additional code beyond supported CFG. Earlier ignored-input trials are superseded for gameplay claims. Artificial main initialization visited four later stages; supported timer-check shortcut reached generic 7–0 terminal win screen. See scenarios.json, gameplay.md and evidence/round01. Broad normal mechanics samples ceased adding structural code; remaining code-like gaps need entry/selector/stack-contract research rather than repetitive play. No terminal exhaustion claim.

Toolkit report: **1 confirmed protocol/API mismatch**, NES-INPUT-001, no blocking findings. Workaround complete buttons arrays tested successfully. Execution timeout under aggregate-heavy tracing is bounded-budget behavior, not reported as a toolkit bug. See toolkit-issues.md.

Deferred: investigate residual code-like islands and their entry contracts, split mixed code/data gaps after bounded review, recover schemas and labels during commentary/redirect. No original game/toolchain layout or full hardware compatibility claimed. Full-source commentary/editorial acceptance remains separate and pending. Disk at handoff: about 12 GiB free; scratch below 100 MiB plus retained local evidence.

## Round01 redirect — coherent contract review

Revisited R01-Q01 from primary bytes/decoded instructions and newly readable dispatcher/helper contracts, rather than inheriting an entry/caller prerequisite as a veto. Synthetic player return $9ED0, low-nibble finite state pointer table, two &3 substates, exact inline barriers, consumer ABIs and stack unwinds jointly support six ready subclaims. Connected graph and independent actionable avenues investigated together, totaling the exact 2,100 unknown-byte complement as proposed structure (not accepted classification). Evidence and caveats: redirect/round01-contract-review/findings.md. R01-Q01 is superseded with retained history, R01-R01..R01-R06 ready. NROM self-write historical intent remains explicitly unproved; three-byte prefix independently reviewable. No repeated gameplay or speculative state sweeps. Pending independent disassembly, checkpoint/rebuild, commentary and redirect; editorial remains pending.

## Round02 disassembly complete — structural classification complete; editorial pending

ROM classification: 100.00% (40976 / 40976 bytes classified; 0 bytes unknown).

Accepted all six R01-R01..R01-R06 after independent exact bank-aware decode and combined contract/boundary review: +1,884 Code and +216 Data in 35 new regions. Internal map now has 133 regions: Code13,174; Data19,610 (includes header/vectors); Image8,192. No old regions overwritten. All additions Static; execution coverage unchanged. Short sentinel/audio wrappers accepted from their specific shared-tail/consumer contracts. R01-R06 prefix accepted as a store to immutable NROM at precisely the following load operand, not a claim of effective self-modification, relocation or historical intent. Animation arrays with unproved complete nibble domains remain generic Data.

| Physical section | Code | Data | Image | Classified | Unknown |
|---|---:|---:|---:|---:|---:|
| Header | 0 | 16 | 0 | 16 | 0 |
| PRG0 | 6289 | 1903 | 0 | 8192 | 0 |
| PRG1 | 5101 | 3091 | 0 | 8192 | 0 |
| PRG2 | 1784 | 6408 | 0 | 8192 | 0 |
| PRG3 | 0 | 8192 | 0 | 8192 | 0 |
| CHR0 | 0 | 0 | 8192 | 8192 | 0 |

Evidence: evidence/round02/review.md, fresh-decodes.json, data-bytes.json, acceptance.json and counts.json. All prior hypotheses/attempt history preserved in leads.json. Existing broad five-level natural gameplay and artificial winner records reused; no unchanged replay. ROM SHA-256 unchanged; all preexisting commentary source/reading/review artifacts hash-preserved.

map.validate passes. Fresh checkpoint pass-003 map validates and generated/pass-003 rebuilds byte-identically; analysis/pass-003-verification.json. Current/saved map SHA-256 a2c637a340703bcd0cbafb8a4e8106d75d5ddbfa82060fd22bd82d615ca09960. The old commentary still represents pass-002 and is deliberately untouched: full current-source refresh and final editorial audit are pending. Overall workflow is therefore incomplete despite zero unknown structural bytes. Handoff round02-disassembly-handoff.md.

Toolkit: no new toolkit issues found. Existing NES-INPUT-001 is one confirmed nonblocking protocol/API bug; see toolkit-issues.md. No toolkit edits, archive/publication/deletion. All input/assets/source/evidence remain local. Disk about12GiB free; scratch92MiB. No hardware compatibility claim.
