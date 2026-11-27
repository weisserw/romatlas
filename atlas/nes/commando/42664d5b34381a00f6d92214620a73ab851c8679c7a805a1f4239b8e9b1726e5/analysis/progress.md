# Disassembly progress

Workspace initialized. Baseline and classification have not run.

## Round 001 — disassembly in progress
Verified raw baseline: [baseline-verification.json](baseline-verification.json).
Existing map inspected: zero regions. Mapper 2 fixed context [0,1,14,15],
reset $C06D, NMI $C0AB. RPC ledger and startup snapshot: round-001/.
Startup 60 frames completed; static discovery API parameter investigation ongoing.

Round001 durable intermediate: 101725 classified bytes, 29363 unknown. Graphics banks1–7 and repeated record banks8–11 reviewed; gameplay ground combat/scrolling verified. Later stage initializer interception ongoing, not terminal. Map saved/exported; current checkpoint still baseline pending pass verification.

## Round 001 disassembly completed — incomplete ROM

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

## Round 001 — commentary in progress
Current phase: commentary. Input checkpoint remains maps/round-001.json and
analysis/round-001-verification.json; classifications are frozen at101725 bytes.
Prepared frozen commentary/before and working commentary/source with configured
release binary. Reviewed all3303 instruction records in compact physical/bank order;
working source now has282 functional anchors,540 branch/entry labels, shared RAM,
hardware and object-field symbols, and retained data/unknown boundary notes.
First coherent source rebuild passed byte identity (commentary/verification.json).
Remaining in this phase: check annotations/symbol overlays and source boundaries,
record concrete uncertainties, refresh reading copy and perform handoff verification.
Potential structural conflict at physical$01C22B..$01C254: existing code records
also serve the $C21B palette table read by $C20F. Classification is preserved;
commentary flags the ambiguity for disassembly/redirect rather than silently fixing it.

## Round001 commentary completed — incomplete ROM
Current phase: commentary complete; coordinator may proceed to redirect.
All current classified source is readable:3303 instruction records,282 meaningful
functional units and540 entry/branch labels; shared hardware, RAM, pool and object
field symbols; classified data/image formats and all unknown boundaries annotated.
Full3495-entry review scaffold preserved and reviewed. No classified-source
readability backlog remains; final editorial_audit remains pending.

Source checkpoint: commentary/source/game.asm plus symbols.inc; reading copy
commentary/reading-after.asm. Fresh commentary/verification.json byte_identical=true
with source game.asm hash ec8a63779703923b78f8dfa5c510c54071c0258eccb9a72dfc207f098db45c65.
Round001 map/state hashes unchanged, input hash unchanged. Counts remain101725
classified,29363 unknown; generated/round-001 remains the structural checkpoint.
Exact questions and source scope: round-001-commentary-handoff.md and
commentary/uncertainties.md. Palette overlap claim is precisely32 bytes physical
$01C22B..$01C24B, separate from the longer existing decoded-code span.
Toolkit: no new issues; NES-001 remains nonblocking. Disk3.5GiB free at handoff;
no durable evidence was pruned, no bulk runtime capture was added.

## Round001 redirect — active attempt r001-connected
Input checkpoint: analysis/state.json / maps/round-001.json; classified101725,
unknown29363. Commentary current-source gate and verification read before research.
Read evidence-policy.md; no map, ROM or toolkit changes. Attempt directory:
analysis/redirect/r001-connected/. Current phase is redirect, research ongoing.
Recovered C389 stack-adjusted four-byte inline RAM-clear contract and followed
startup, stage/continue, projectile initializer, audio sequencer/command dispatcher,
object30-word/secondary16-word/spawn48-word dispatch neighborhoods. closure.py
saves bank-aware candidate records, exact inline records and explicit barriers;
these are proposals requiring independent disassembly review, never accepted bytes.
Stage shortcut: title Start reaches C5CC after clears at frame160,116674 operations;
patch049D/E there survives ordinary initializer. Snapshots/captures stage1,3,15
show coherent distinct terrain and enemies after360 idle+120 up/a/b frames.
Final-stage scroll endpoint F2:01 patch caused death/respawn, then4-4 fortress-gate
combat; no ending claimed. Primary runtime.json/endpoint.json and captures retained.
Current frontier: finish fixed-bank continuations with switch banks explicitly
unknown after unresolved mapper values; inspect residual obvious data against
consumers and boundaries; aggregate evidence/ranges/lead records and handoff.

### Redirect r001-connected ready combined handoff
Research completed and packaged at analysis/round-001-redirect-handoff.md.
Actual map remains101725 classified/29363 unknown; proposed28801 unknown bytes
(12723 Code,16078 Data), plus41 existing Code bytes proposed palette correction.
562 bytes unproposed. No proposals accepted here. Seven ready lead records retain
prior history; scenarios include successful post-clear initialization for later
and final stages and failed raw scroll shortcut. All graph continuations were
followed through explicit barriers, with unknown switch banks preserved locally.
Next phase: independent disassembly review of combined batch. This is not terminal
exhaustion or ROM completion. No new toolkit findings; NES-001 nonblocking.

## Round002 — disassembly review active
Current phase: disassembly. Input accepted checkpoint maps/round-001.json,
101725 classified /29363 unknown. Independently adjudicating connected redirect
batch and palette correction; previous commentary/source preserved for next phase.
All proposed instruction contexts are being freshly decoded with the configured
release binary; primary stack, mapper, dispatch and bounded consumer contracts
are reviewed separately from natural reachability. No successful runtime replay.

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

## Round002 commentary active
Input: maps/round-002.json; generated/round-002; analysis/round-002-verification.json. Entire prior commentary preserved in round-002/commentary-prior before fresh preparation. Physical-range carry and complete current-source readability underway.

## Round002 commentary complete
Current-source readability completed across all9381 code records,105 data records,7 image banks and28 unknown boundaries, including6096 new physical instructions and41 palette retyped bytes. Prior commentary preserved intact in round-002/commentary-prior;281 prior units reused and one superseded palette unit invalidated with history retained. Added372 functional code anchors,35 data units and444 descriptive flow names. Fresh byte-identical verification and refreshed reading copy pass; ROM/map/state unchanged. No pending readability gaps. Classification remains130691/131088,397 unknown; final editorial audit pending. Next: redirect using round-002-commentary-handoff.md and exact residual ledger. No new toolkit issues; NES-001 nonblocking.

## Round002 redirect active — r002-contracts
Input maps/round-002.json and fresh current-source commentary. Classified130691,397 unknown in28 ranges. Exact residual contract and practical ending prerequisite review ongoing; no map/source/ROM/toolkit edits. Frontier: residual consumer slices, aliases and standalone stack-safe helpers, then dependent final-presentation state writers.

### Round002 redirect r002-contracts ready combined handoff
Current phase redirect completed-ready-batch; next independent disassembly.
Actual130691 classified/397 unknown unchanged. R002-RESIDUAL proposes397 bytes
(254 Static Code143 Static Data) in30 exact slices with primary contracts and full
28-range ledger. Source broader uncertainty review completed; stale old helper
uncertainty prose identified for commentary. Supported artificial global/player
readiness shortcut reached timed ending/all-missions-over presentation and index
reset; no natural boss-clear or secret ending claim. Handoff:
analysis/round-002-redirect-handoff.md. ROM/map/state/source hashes unchanged.

## Round003 disassembly active
Input checkpoint maps/round-002.json; baseline130691/131088 classified,397 unknown. Independently adjudicating R002-RESIDUAL thirty slices and exact fixed/bank6 contexts. Prior commentary/history preserved; successful broad progression/ending evidence reused.

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

## Round003 commentary in progress
Full prior commentary preserved in round-003/commentary-prior. Refreshing the baseline and carrying sound prose by physical range/dependencies; repairing stale helper claims. Final editorial audit remains reserved.

## Round003 commentary completed
Full current-source readability completed:118 new instructions,143 Data bytes,
22 stale prose repairs plus valid prior carry.9623 scaffold entries/731 meaningful
units reviewed with current locations. No readability gaps remain. Configured-binary
verification byte_identical=true; reading-after refreshed. Full prior commentary
preserved in round-003/commentary-prior. Map/state/ROM unchanged. Final targeted
editorial acceptance/audit pending; no redirect or toolkit edits. Details:
[round-003-commentary-handoff.md](round-003-commentary-handoff.md),
[round-003/commentary-summary.json](round-003/commentary-summary.json).
Existing NES-001 documentation/schema issue nonblocking; no new toolkit issues.

## Round003 redirect active — r003-acceptance
Input maps/round-003.json (131088/131088 classified, zero unknown) and full current-source commentary. Reviewing qualified uncertainties against primary source/contracts for concrete contradictions; final targeted editorial acceptance remains a separate next phase. Source/map/state/ROM unchanged. Attempt analysis/redirect/r003-acceptance/.

## Round003 redirect completed — final editorial acceptance pending
Targeted uncertainty/source/primary-contract review found no concrete structural
contradiction or new actionable lead. Independently refreshed physical complement:
131088/131088 classified, zero unknown/unmapped. Zero proposed/accepted changes.
Verified lead history preserved and empty r003-acceptance pass recorded. Mechanical
semantic/natural-reachability qualifications remain explicit; successful later/final
and ending/reset evidence reused without unchanged replay. No terminal-incomplete
review applies. Next separate final editorial acceptance, then final verification.
Handoff: analysis/round-003-redirect-handoff.md; findings and preservation hashes in
analysis/redirect/r003-acceptance/. No new toolkit issues; NES-001 nonblocking.
ROM/maps/state/source/toolkit unchanged; no publication/archive/deletion.

## Final editorial acceptance active
Separate final worker reuses reviewed round003 scope and targets known stale-prose repairs, helper entry/bank contracts, data boundaries and qualified uncertainties. Map131088/131088 classified;9623 scaffold entries/731 units; pending readability empty. Fresh supported audit will follow substantive acceptance.

## Final editorial acceptance completed
Targeted final acceptance repaired actual alias contracts and transfer count units;
all9623 scaffold entries/731 unique units reviewed, pending scope empty. Supported
commentary.py audit passes: byte_identical=true,editorial_audit=true,9541 per-region
unit memberships. Reading copy fresh. Exact current hashes/results:
analysis/final-acceptance-handoff.md and final-acceptance-identities.json.
ROM/map/state unchanged; no toolkit edits or archive/publication/deletion.

## Coordinator terminal handoff — whole-ROM acceptance
ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).
Three separate sequential phase rounds and final editorial acceptance finished.
Current input/output checkpoint maps/round-003.json equals current map, internal
state and frozen commentary baseline. Final source audit and byte-identical
verification pass. All leads consumed/verified; no new ready leads. All9623
review entries and731 unique meaningful units reviewed, no pending readability.

Coordinator independently rechecked exact physical coverage/evidence, map/store
validation, current source/audit hashes, reading freshness and all nine phase
handoffs: [coordinator-verification.json](coordinator-verification.json), ok=true.
Terminal conclusions and honest semantic/reachability limits:
[terminal-handoff.md](terminal-handoff.md). Supported whole-ROM completion is
proved by these artifacts, not inferred from orchestration completion alone.

Inspected toolkit scripts/archive.py selection contract. Wrote
[../archive-options.json](../archive-options.json):21 additional final documents,
6 redundant historical instruction snapshots omitted, zero unaccounted Markdown.
No final conclusion or handoff omitted. No archive execution, publication, cleanup,
ROM/toolkit changes or workspace deletion. All local evidence/history preserved.
