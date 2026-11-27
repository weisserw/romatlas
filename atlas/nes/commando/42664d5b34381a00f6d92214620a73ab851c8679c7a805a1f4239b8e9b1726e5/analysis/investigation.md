# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## Round 001 structural and gameplay evidence

ROM classification: 77.60% (101725 / 131088 bytes classified; 29363 bytes unknown).

Checkpoint: [round-001-verification.json](round-001-verification.json), map
[../maps/round-001.json](../maps/round-001.json), source [../generated/round-001/game.asm](../generated/round-001/game.asm).
Map validation and generated ca65 rebuild pass; byte_identical=true. Input SHA256 unchanged.

|Bank (8KiB)|Code|Data|Image|Unknown|
|---|---:|---:|---:|---:|
|0|192|896|0|7104|
|1|0|0|8192|0|
|2|0|0|8192|0|
|3|0|0|8192|0|
|4|0|0|8192|0|
|5|0|0|8192|0|
|6|0|0|8192|0|
|7|0|0|8192|0|
|8|0|8192|0|0|
|9|0|8192|0|0|
|10|0|8192|0|0|
|11|0|8192|0|0|
|12|0|2349|0|5843|
|13|1977|976|0|5239|
|14|1929|0|0|6263|
|15|2505|773|0|4914|


Header:16 Data bytes. 3354 regions, all have structural evidence.
Observed opcode starts across retained samples: 2233; observed instruction bytes
3885 (2.96% whole ROM), separate from classification. Samples cover
title/music, helicopter/landing, upward scrolling and combat; artificial pre-start
patch samples returned to first area. Runtime aggregation has zero detail capacity;
detail drops are intentional, not absent fetches. Full trace summary was not retained;
no exhaustive execution coverage claim. Static traversal uses per-batch20,000 instruction
budget and <=900 explicit contextual roots; no budget exhaustion in final discoveries.

Broad progression: later area indices1,2,3 and final mission15 attempted through
pre-start patch and active-state patch; title initialization overwrote pre-start
indices, active patches corrupted terrain. Ending/boss unavailable in this pass
pending a supported level-reinitialization contract. CPU B1E2 interception reached
1,000,000-operation limit without matching; no register edit performed.

Ready follow-ups: identify natural stage-load call/transition, preserve the menu
selection across init, inspect fixed-bank dispatchers/call contracts, and music
bank0 residual data. See scenarios.json, leads.json, investigation.md and handoff.
No initial leads existed. No claim of exhaustion for redirect; this is one easy-gain
pass with a concrete unresolved RAM transition prerequisite.

Toolkit: one confirmed documentation mismatch/missing method parameter schemas,
nonblocking workaround available; [toolkit-issues.md](toolkit-issues.md) NES-001.

## Redirect r001-connected
Recovered stack-adjusted inline RAM clears, sparse address-minus-one dispatch
contracts and consumer-bounded data regions. Combined independent proposals and
all related graph continuations are in redirect/r001-connected/findings.md and
../analysis/round-001-redirect-handoff.md. State/map unchanged. Supported C5CC
post-clear edit visits later/final stages; raw scroll endpoint edit fails through
death/respawn. Existing failed methods preserved, no unchanged replay. Ending and
seventh projectile-slot game purpose remain unresolved. Independent disassembly
review next; proposed byte counts are not classification. No terminal claim.

## Round002 disassembly completed — incomplete ROM
Current phase: disassembly completed; next full current-source commentary, then redirect.

ROM classification: 99.70% (130691 / 131088 bytes classified; 397 bytes unknown).

|Bank (8KiB)|Code|Data|Image|Unknown|
|---|---:|---:|---:|---:|
|0|1567|6625|0|0|
|1|0|0|8192|0|
|2|0|0|8192|0|
|3|0|0|8192|0|
|4|0|0|8192|0|
|5|0|0|8192|0|
|6|0|0|8192|0|
|7|0|0|8192|0|
|8|0|8192|0|0|
|9|0|8192|0|0|
|10|0|8192|0|0|
|11|0|8192|0|0|
|12|110|8044|0|38|
|13|4585|3515|0|92|
|14|7205|847|0|140|
|15|5818|2247|0|127|

Header:16 Data bytes. Accepted12723 new Code and16078 new Data from connected
batch; independently retyped41 old Code palette bytes to Data, then165 additional
residual Data bytes from bounded consumer/pattern inspection. New Code is Static;
artificial saved fetches do not imply natural reachability. Total9493 regions.
Exact adjudication: [round-002/review.md](round-002/review.md), primary selected
contracts and all6448 instruction-context decode checks preserved. Thirteen residual
non-code blocks have direct readers/repeated finite patterns; generic Data avoids
unproved per-table schemas. Exact remaining ranges: round-002/final-residual.json.

All seven ready connected leads verified structurally; natural selector domains,
seventh projectile role and ending remain qualified. Existing later/final milestone
recipes/captures reused; no unchanged successful replay or failed raw-scroll retry.
Ending requires dependent terrain/object transition contract; source refresh and
residual-specific redirect remain useful, so this is not terminal exhaustion.

Checkpoint [round-002-verification.json](round-002-verification.json),
[../maps/round-002.json](../maps/round-002.json), generated/round-002.
Fresh map validation and ca65 rebuild passed byte_identical=true; ROM SHA256
unchanged. Prior commentary/source game.asm hash remains
`ec8a63779703923b78f8dfa5c510c54071c0258eccb9a72dfc207f098db45c65`;
its old palette interpretation is superseded only structurally, preserved for review.
Current-source commentary is pending for newly classified bytes; no editorial
completion claimed. No new toolkit issues; existing NES-001 nonblocking, see
[toolkit-issues.md](toolkit-issues.md). No toolkit/source/ROM modification or publication.

## Round003 disassembly completed — editorial work pending
Current phase disassembly completed; next full current-source commentary and final audit.

ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).

|Bank (8KiB)|Code|Data|Image|Unknown|
|---|---:|---:|---:|---:|
|0|1567|6625|0|0|
|1|0|0|8192|0|
|2|0|0|8192|0|
|3|0|0|8192|0|
|4|0|0|8192|0|
|5|0|0|8192|0|
|6|0|0|8192|0|
|7|0|0|8192|0|
|8|0|8192|0|0|
|9|0|8192|0|0|
|10|0|8192|0|0|
|11|0|8192|0|0|
|12|110|8082|0|0|
|13|4612|3580|0|0|
|14|7327|865|0|0|
|15|5923|2269|0|0|

Header16 Data bytes. Accepted254 Static Code bytes (118 instructions) and143
Static Data bytes across30 independently adjudicated slices, zero residuals.
Review: [round-003/review.md](round-003/review.md); exact decisions and fresh
decodes/primary contracts retained. Tiny B58B/CF8B/FA45 islands separately
accepted generic Data after pattern, boundary and conflicting-code review.
No invented specific schemas or natural callers. R002-RESIDUAL verified structurally.
Later/final/ending artificial gameplay evidence reused; no unchanged replay.

Fresh [round-003-verification.json](round-003-verification.json),
[../maps/round-003.json](../maps/round-003.json), generated/round-003: map validation
and ca65 byte-identical rebuild passed; ROM SHA256 unchanged. Existing full
commentary preserved. New118 instruction records and data slices need current-source
commentary refresh; stale helper uncertainty prose identified by prior redirect
needs correction. Whole-ROM completion is not claimed before full editorial audit.
No new toolkit issues; existing NES-001 documentation bug/missing schemas remains
nonblocking: [toolkit-issues.md](toolkit-issues.md). No toolkit/ROM edits or publication.
