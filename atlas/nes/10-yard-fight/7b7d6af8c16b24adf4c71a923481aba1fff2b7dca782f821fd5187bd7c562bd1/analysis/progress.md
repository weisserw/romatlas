# Disassembly progress

Workspace initialized. Baseline and classification have not run.

## Round 01 disassembly in progress
Baseline established and byte-identical verification passed. NROM, 32 KiB PRG and 8 KiB CHR. Validated vectors at physical $800A-$800F: NMI $8471, reset $8000, IRQ $84FF. Bounded vector-root discovery and startup trace underway; no classifications claimed yet.

### Intermediate checkpoint pass-001
Map validates and generated/pass-001 rebuilds byte-identically; map hash 19eb9302f02946a2d434cfc186c1e5009b1f150a1081745db02a198c38defca8. Input audit found documented single-button calls silently ignored by NES adapter; prior routes were attract/artificial state execution, not button-driven play. Corrected buttons-array trials underway. See NES-INPUT-001.

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

## Round01 commentary complete — full current-source readability
Prepared commentary from pass-002 without changing its map/state or input. All 117 scaffold regions/279 units reviewed: 11,290 code bytes, 19,394 data bytes, 8,192 CHR bytes, and explicit opaque notes for 2,100 unknown bytes. Source now has routine/branch/RAM/player-field/hardware/table names and mechanical descriptions of main/NMI control, stack exits, scrolling, players/contact/ball/AI, audio and menus. Uncertain event identities and table extents remain ordinary tentative prose. No classified-source readability backlog remains; final editorial audit stays pending and ROM classification remains incomplete at 94.88%.

Fresh commentary.py verify passed byte-identically, source SHA-256 7685e3a7f6e442e1e929ea05d0baea2b7641c0e699902c8239f88921f448c560; reading-after.asm refreshed. Map/state/frozen baseline identities checked unchanged. See round01-commentary-handoff.md, ../commentary/verification.json, ../commentary/review.json and commentary-round01/readability-accounting.json. No new toolkit findings; existing NES-INPUT-001 confirmed protocol mismatch remains nonblocking. Source/ROM/assets local; no toolkit modification, audit, archive, publication or deletion.

## Round01 redirect complete — combined ready proposal

No classification changes: pass-002 remains 94.88%, 2,100 unknown bytes in 19 intervals. Combined bounded static contract review investigated every residual and all supported connected player subdispatch/handler/helper/table continuations, then independent sprite, message, score/scoreboard, audio-wrapper and OAM-upload avenues. Six ready leads R01-R01..R01-R06 propose 1,884 Code + 216 Data in 35 exact segments for independent disassembly. Natural selector/reachability claims remain unresolved. Small wrappers and the upload prefix are localized review claims, especially STA $CC18 into immutable NROM (no working self-modification claim). See redirect/round01-contract-review/findings.md, proposal-segments.json and unknown-ledger.json; round01-redirect-handoff.md supplies checkpoint and recipe. Internal unknown complement and unchanged ROM/map/state/source/reading hashes verified. No terminal exhaustion claim; full editorial audit still pending. No new toolkit findings; NES-INPUT-001 remains nonblocking.

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

## Round02 redirect complete — editorial next

Fresh saved-state complement is empty at pass-003 (40,976 classified; zero unknown). Source/ROM/map/state identities unchanged. No new/reopened ready lead; empty pass appended to leads.json. Unique attempt redirect/round02-terminal-review retains primary source excerpts, validation and compact rejection/progression review. All five natural skill levels and successful artificial terminal shortcut reused; no untried practical milestone obstacle identified. Two precise PPU comment discrepancies and handoff308/48 versus309/49 count error go to final editorial audit; no source or classification edits. See round02-redirect-handoff.md. No new toolkit findings; NES-INPUT-001 remains nonblocking. No archive/publication/deletion.

## Final editorial acceptance complete

Corrected both precise PPU descriptions and their inventory/dependency prose, reused valid current-source review, and reconciled 308 total/48 new units without overwriting historical 309/49 reports. commentary.py audit passed: all 133 regions/308 units reviewed, fresh byte-identical rebuild and reading refresh. Configured binary validates unchanged map and saved state, both ok; analysis/current-audit.json retains results/identities. Structural counts remain 40,976 classified, zero unknown (Code13,174/Data19,610/Image8,192), pass-003. See ../commentary/editorial-audit.md and final-editorial-handoff.md. No new toolkit issues; NES-INPUT-001 remains nonblocking. Coordinator terminal/accounting work remains; no archive, publication, deletion or toolkit modification.

## Reconciled round02 commentary phase record

Input pass-003 map/state a2c637a340703bcd0cbafb8a4e8106d75d5ddbfa82060fd22bd82d615ca09960.
Prior entire commentary preserved under round-02/commentary-pass-002/ before fresh
prepare. Carried 260 units; added 48 (actual total308, correcting historical309/49).
Reviewed all133 current regions, with no remaining readability frontier. Fresh
byte-identical verification and reading refresh passed. Output source was
d21ffc43c25297a386d3955220258d342cc66b9c8acf0ddfecf4949a50513444 before final
editorial comment corrections; map/classification unchanged. Input leads R01-R01..R01-R06
were already verified by disassembly; no lead consumed/produced in commentary.
round02-commentary-handoff.md plus round-02/commentary-accounting.json preserve
scope, dependency carry and verification. Final editorial handoff supersedes its
two PPU prose errors and mistaken unit count; historical record retained.

## Terminal coordinator handoff — whole-ROM complete

ROM classification: 100.00% (40976 / 40976 bytes classified; 0 bytes unknown).

Current phase: terminal handoff; no pending/blocked phase or active research frontier.
Two complete sequential rounds and separate final editorial acceptance have finished.
Final input/output checkpoint pass-003; current/internal map SHA-256
a2c637a340703bcd0cbafb8a4e8106d75d5ddbfa82060fd22bd82d615ca09960.
All six ready leads R01-R01..R01-R06 verified; final redirect added no ready leads.
Final source db996732dfc53607ee766f63aef9abac5211503312a20e9d5df3ff7a2bdcfa54.
133 regions/308 units reviewed, fresh editorial audit and byte-identical verification
passed; reading copy refreshed. Independent coordinator partition/inventory/hash
checks pass (coordinator-verification.json). Code13,174/Data19,610/Image8,192.
No unknown complement or required annotation/readability work remains. This is
verified whole-ROM completion, not a classification claim inferred from goal status.

See terminal-handoff.md and coordinator-audit.md for requirement evidence and limits.
Root archive-options.json explicitly selects final/historical result documents;
archive-document-audit.json accounts for every one of29 Markdown files:16 additional
final documents,3 omitted duplicate historical instructions, others default/exempt,
zero unaccounted. No final results are omitted. No archive execution, cleanup,
publication, workspace deletion or toolkit change. ROM identity unchanged; local
state/evidence/source/assets retained. Existing NES-INPUT-001 is nonblocking with
tested controller-array workaround. Natural callers/selectors and precise meanings
remain explained uncertainties; no natural-win/absent-credits/full-compatibility claim.
