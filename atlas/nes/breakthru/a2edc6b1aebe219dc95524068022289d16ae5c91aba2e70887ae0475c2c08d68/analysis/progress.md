# Disassembly progress

Workspace initialized. Baseline and classification have not run.

## Round 001 — disassembly in progress

Baseline established 2026-11-21 with configured release binary. `python3 run.py inspect`
and `python3 run.py baseline` succeeded; analysis/baseline-verification.json confirms
byte identity for maps/baseline.json and generated/baseline. This raw baseline is not
classification. ROM identity remains the configured SHA-256.

Local title confirms BreakThru, Data East USA 1987. Mapper 1; physical file 262160
bytes: 16-byte header, 131072-byte PRG and 131072-byte CHR. Live JSON-RPC process is
managed by analysis/round-001/driver.py (exec session 74073); requests/responses and
bounded analysis artifacts remain under analysis/round-001. No classification state
saved yet; current evidence contains startup/title, normal Start route, MOUNTAIN intro
and moving/firing/jumping vehicle on mountain gameplay (play.ppm). Actual trace is
bounded aggregate + initial 1000 retained details; initial 300/120-frame requests
timed out after 45/118 cumulative frames and preserved state. Later 60-frame gameplay
request completed at frame 437. Exact request/result history: rpc-log.jsonl.

Observed contextual roots were explicitly paged into discovery: 3614 roots,
4239 contextual candidates, 7977 unique physical code bytes, 40 paged diagnostic
records (mostly repeated indirect target barriers). Inspection/decode of contiguous
slices is in progress in code-review.json. These are hypotheses until reviewed and
annotated. analysis/leads.json initially contained no leads. Next: annotate reviewed
connected code and header/CHR/uniform data, investigate stage/transition RAM contract
for broad shortcut visits; export/save/verify a fresh pass checkpoint before handoff.

### Durable classification batch 1

Saved analysis/state.json and maps/current.json through map.annotate/save/export;
map.validate succeeded. Fresh checkpoint pass-001-initial validates and rebuilds
byte-identically (analysis/pass-001-initial-verification.json). It preserves the
7977-byte initial code batch, header, CHR planar image section, six CPU vector
bytes, and individually inspected maximal uniform non-code runs; generic Data
runs make no specific fill-purpose claim. Mixed runtime/static closure is conservatively
Static; actual fetch reachability remains in local trace records.

A second connected static batch follows the 103-entry command-handler pointer table
at physical 8508..8714, independently bounded by command < $67 consumers at CPU
$a06f/$a0ef and doubled indexing. Its 29 unique handlers have synthesized-return
entry contracts, not just pointer-shaped bytes. Direct JSR-entry closure added 3041
code bytes with official opcodes and no conflicting boundaries, exact review in
analysis/round-001/static-more-review.json. State/map have been saved after this batch;
final verification will use a separate checkpoint. Broad gameplay shortcuts pending.

### Broad progression and patterned-data batch

Actual natural stage1 game snapshot now durably retained at
analysis/round-001/game-clean.snapshot.json (earlier mountain-clean snapshot was
still title; retain that historical misnamed artifact and use game-clean).
Artificial shortcut recipe restores game-clean, releases buttons, writes RAM
$8f=stage,$90=0,$b2=9,$be=0, sets PC$c0c0/SP$ed/P$24 at an instruction boundary,
then runs 90 untraced frames and 12 traced gameplay frames with right+B+A.
Stage2 BRIDGES and stage4 city gameplay initialized and exercised successfully,
confirmed by framebuffer and effective banks [4,5,14,15]/[8,9,14,15]. Stage3 direct
also completed with 1962 contextual roots; framebuffer review pending. Stage5's
first 12-frame traced request timed out under concurrent CPU load; retry uses 8
frames and 30-second bound, saving a separate v2 artifact. Ending attempt waits
for that durable snapshot. Intro-only stage2/3 routes and failed first attempts are
preserved, not counted as gameplay visits.

Inspected graphics/tile/attribute/map records across seven bounded ranges using
consumer references to$8000/$8100-$8700 and repeated 240/256-byte layouts, exact
edges, neighboring fill and connected-code conflicts. Added 36389 generic Data
bytes, with role-specific evidence where known; no original source layout or
complete semantic schema claimed. data-layout-review.txt and data-layout-ranges.json
retain checks. State/current map saved and validated. Remaining gaps mostly code
continuations, command handlers/data islands, audio tables and stage metadata.

## Round 001 disassembly finished — incomplete

ROM classification: 97.91% (256690 / 262160 bytes classified; 5470 bytes unknown).

Saved 599 regions, all with structural evidence: 15837 code bytes,
109781 Data bytes and 131072 CHR Image bytes.
Header: 16 Data bytes outside the PRG table. All 16 CHR banks have 8192 Image
bytes each and zero unknown bytes. Exact counts and unknown complement are in
[round-001/counts.json](round-001/counts.json) and
[unknown-ranges.json](unknown-ranges.json). Raw fallback does not count.

| PRG 8 KiB bank | Code | Data | Classified | Unknown |
|---|---:|---:|---:|---:|
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 3080 | 1701 | 4781 | 3411 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 7 | 8185 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 7 | 8185 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 7 | 8185 | 8192 | 0 |
| 8 | 7 | 8185 | 8192 | 0 |
| 9 | 0 | 8192 | 8192 | 0 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 7 | 8185 | 8192 | 0 |
| 12 | 2354 | 4186 | 6540 | 1652 |
| 13 | 100 | 7842 | 7942 | 250 |
| 14 | 7805 | 275 | 8080 | 112 |
| 15 | 2463 | 5684 | 8147 | 45 |

Execution is separate: 3269 natural ROM opcode starts,
3136 artificial ROM opcode starts,
4196 unique starts in their union. These are bounded instruction-start
counts, not byte coverage percentages. Saved trace summaries report no aggregate
drops; detail drops were intentional. See [scenarios.json](scenarios.json).

Natural title/Mountain gameplay and artificial Bridges, Prairie, City, Airfield,
late Airfield enemy line, congratulations screen and aircraft ending animation
were confirmed visually and through accepted instruction fetches. Post-animation
return to the next loop was not observed. The ending animation was followed after
recognizing that congratulations precedes a separate $c45b routine.

Verification: map.validate and save/export succeeded. The final checkpoint
[maps/pass-001-final.json](../maps/pass-001-final.json) generated
[generated/pass-001-final/game.asm](../generated/pass-001-final/game.asm).
[pass-001-final-verification.json](pass-001-final-verification.json) confirms
byte_identical=true. Input SHA-256 remains unchanged. Configured toolkit commit:
6920600485871b6449a9f972c9854c07e8b92828; configured binary SHA-256:
8ff56b0b1cf8d2a3aca5430c70d34280ae3db36fc6774919062db4ea63843f82.
Baseline, initial and pass-001 checkpoints remain intact.

Pass stopping rationale: broad representative scenarios and bounded static review
provided a coherent supported batch. The late checkpoint added only two code bytes;
the separate ending animation was a productive alternative and added 823 bytes.
The remaining command, audio and animation fragments need local return-stack,
selector-domain or inline-data boundaries. Blind continuation crosses real data and
bank-changing calls. These are concrete redirect questions, not permanent
rejections or a terminal global exhaustion claim. No unchanged failed lead was
repeated: intro shortcuts and timeout retries changed entry contract or budget.

Full current-source commentary and final editorial review remain outstanding.
Byte-identical rebuilding establishes fidelity, not whole-ROM classification.
Input, internal state, snapshots, traces, extracted/generated assets and source
remain local. No archive, cleanup, publication, toolkit edit or workspace deletion
was performed. Toolkit findings: zero confirmed bugs, zero suspected bugs, zero
missing features and no blockers; no toolkit issues found. See
[toolkit-issues.md](toolkit-issues.md).

## Round 001 — commentary in progress

Current phase: commentary. Prepared with the supported commentary.py script from
verified pass-001-final; preparation rebuilt byte-identically. Frozen before/,
baseline.json and phase1-state.json retain original artifacts. Full source was
read in bounded chunks (game.asm lines 75–2600 and 2950–10442); source instruction
mechanics inspected, but no review credit is claimed solely from that read.
119 specific routine/continuation notes for physical PRG bank 1 are durably saved
in analysis/commentary/notes-bank1.json and its authoring script. These include
six-byte object dispatch, synthetic RTS returns, metasprite clipping, allocation,
visibility/sound policies and every classified handler fragment. These notes await
source integration and verification. All review scaffold entries remain pending.
Next frontier: finish audio bank 12/13 and fixed-bank notes, integrate descriptive
symbols/RAM/constants and per-range data/unknown notes, review the transformed
source, verify byte identity and refresh reading-after.asm. Full current-source
readability remains incomplete; do not start redirect.

### Commentary source integration milestone

Integrated the full-source pass into commentary/source/game.asm: 483 specific
routine/continuation explanations, 1111 context-resolved instruction/branch symbols,
197 RAM/hardware aliases and typed object-field constants. Includes all object
handlers, ending actors/audio, fixed startup/gameplay/map streaming, player controls,
terrain/collision/score and audio routing. Classified data slices now carry format
or practical uncertainty notes; every unknown scaffold slice has an honest local
continuation/data-boundary note. No phase-1 mapping/classification was changed.
Supported commentary verification succeeded byte_identical=true with source hash
efb3b51c96fe8755656a2214c9c1137bf9c8b04ee6440ddea0588dd18d831b95.
An initial duplicate symbolic name was corrected before the successful build.
Remaining frontier: semantic consistency/gap review of integrated source, attach
review inventory credit only after that check, final verify and refreshed reading copy.

## Round 001 commentary finished — current-source readability complete

Current phase: commentary complete; next authorized workflow phase is redirect.
Reviewed all 737 full-ROM scaffold ranges: 338 code, 260 data, one CHR image and
138 explained unknown ranges. There are 512 compact reviewed units, 502 specific
routine/continuation notes, 1125 resolved code/branch symbols and 225 RAM/hardware
aliases. All current classified source meets the local naming/comment standard;
unknown code-vs-data boundaries and unresolved game meanings remain honest.
Semantic checks corrected terrain response/shape roles, internal handler entry
positions, no-op RTS entries, shared score/collision scratch names and branch joins.
Every classified target in the four established dispatch tables has specific notes.

Fresh supported verification passes byte_identical=true for source SHA-256
8292800b9b7ca4fad4f5e65d9f3d8b2e8098c225ace6c07ad771ac34b55448fb;
commentary/reading-after.asm is refreshed. analysis/commentary/readability-checks.json
confirms map/state identical to the verified/frozen baseline and ROM identity
unchanged. Classification remains 256690/262160 (5470 unknown); final editorial
audit remains pending. No global completion claim. Compact next-worker handoff:
[round-001-commentary-handoff.md](round-001-commentary-handoff.md).
No toolkit issues found; no blockers, toolkit edits, archive, cleanup, deletion or
publication. All source/assets/evidence remain local.

## Round 001 redirect in progress

Attempt `analysis/redirect/round001-static-continuations/` uses unchanged pass-001-final/current state (256690 classified, 5470 unknown), complete source commentary hash 8292800b9b7ca4fad4f5e65d9f3d8b2e8098c225ace6c07ad771ac34b55448fb. R001–R003 active. Exact gap decode saved; frontier: synthetic return edges, ordinary return/stack-unwind review, then independent fixed islands. No proposals accepted or map/source edits in redirect.

Redirect substantive checkpoint: corrected bank1 gap premise (post RAM indirect-store fallthrough), synthetic return $A08A, connected first-pool allocator $B7DC, all three ending actor graphs, split $9572 selector data, audio command pointer/stack contracts, independent fixed/data slices. Research-only proposals currently 5451 bytes, retained 19 if accepted; accepted state still 256690/262160. Exact proposals and disposition ledger: `analysis/redirect/round001-static-continuations/proposals.json`. Frontier: exact instruction-boundary/pointer cross-check and residual review, same redirect phase.

## Round 001 redirect batch handed to independent disassembly

Attempt `round001-static-continuations` completed connected R001 object, R002 all-three ending actors/audio, and independent R003 fixed consumer avenues. Updated R001/R002/R003 ready with exact physical contexts and append-only attempt history. Combined proposals5451 bytes (4832 code,619 data): bank1 3407, banks12/13 1901, fixed14/15 143. Accepted0 here; state/current/pass-001-final unchanged SHA256 e03b78197b212592dc77dc59832daa4ce172f26e09e0e067218d93a53bed440c. Accepted256690,unknown5470 remain authoritative. Hypothetical residual19 is five narrowly reviewed intervals, not accepted classification or terminal exhaustion.

Exact instruction-boundary checks and49 dispatch target checks passed,2428 proposed code instructions have documented CFG/dispatch/static-contract roots. Important splits:$9572 three-byte data; $FEF8 consumed pointer separate from preceding words; title ASCII separate from mixed trailer; orphan CMP/RTS separate from duplicate resolver. Actual undocumented SHX preserved in proposal. No new execution/reachability claims. Full results `analysis/redirect/round001-static-continuations/results.md`; compact handoff `analysis/round-001-redirect-handoff.md`. Next phase independent disassembly, then current-source commentary and redirect; no global completion. No toolkit issues found. No map/source/ROM edits, archive/cleanup/delete/publish/toolkit changes.

## Round 002 disassembly in progress

Current phase: independent disassembly; input pass-001-final, state/current SHA256 e03b78197b212592dc77dc59832daa4ce172f26e09e0e067218d93a53bed440c. Reviewing combined R001–R003 proposal5451 bytes. Decisive dispatcher RAM/synthetic return, ending inline selector and audio unwind contracts inspected; frontier remaining per-slice decode/data consumer checks, supported annotations then fresh pass-002 checkpoint. No new runtime/reachability claim or commentary edits.

## Round 002 disassembly finished — incomplete

Current phase: disassembly complete; next phase full current-source commentary, then redirect. Input checkpoint pass-001-final (256690 classified/5470 unknown); output fresh pass-002-final. R001/R002/R003 independently verified:5451 bytes accepted (4832 Code,619 Data), all138 proposal slices. Per-slice [decisions](round-002/decisions.json), [contract review](round-002/review.md), [fresh configured-binary decode](round-002/independent-decode.json), [live internal-store counts](round-002/counts.json). No observed/natural reachability claim added.

ROM classification: 99.99% (262141 / 262160 bytes classified; 19 bytes unknown).

737 evidence-backed structural regions:20669 Code,110400 Data (including16-byte header),131072 CHR Image. All16 CHR banks retain8192 Image bytes and zero unknown each. PRG counts below use unique physical bytes, not CPU aliases/raw fallback.

| PRG 8KiB bank | Code | Data | Classified | Unknown |
|---|---:|---:|---:|---:|
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 6296 | 1892 | 8188 | 4 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 7 | 8185 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 7 | 8185 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 7 | 8185 | 8192 | 0 |
| 8 | 7 | 8185 | 8192 | 0 |
| 9 | 0 | 8192 | 8192 | 0 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 7 | 8185 | 8192 | 0 |
| 12 | 3691 | 4500 | 8191 | 1 |
| 13 | 344 | 7848 | 8192 | 0 |
| 14 | 7827 | 365 | 8192 | 0 |
| 15 | 2476 | 5702 | 8178 | 14 |

Corrected RAM-store fallthrough and synthetic dispatcher return were accepted with enclosing contracts; connected first-pool allocator and second-pool copy included. All three ending actor graphs and downstream helper paths inspected. Explicit three-byte selector103810..103813 remains Data; next Code begins103813. Audio49-pointer storage and coherent routines accepted without inventing full token domain; explicit stack discards treated as terminal locally. Fixed pause helper preserves actual SHX abs,Y9E0007 as exact .byte with mnemonic in generated source (line10685); no repair of unusual semantics.

Exactly19 unknown bytes in five intervals:8714:1,13913:3,105610:1,130820:4,131072:10. [Current unknown complement](round-002/unknown-ranges.json), with prior [narrow residual evidence](redirect/round001-static-continuations/residual-review.json). Historical analysis/unknown-ranges.json remains the old pass001 complement and is preserved as evidence; use round002 file for current work. Noncode proposals received practical consumer/format inspection before Data annotations. Unsupported residuals were not guessed.

No gameplay repetition: prior natural Mountain and artificial Bridges/Prairie/City/Airfield/late enemy line/congratulations/aircraft visits remain recorded in scenarios.json. Post-aircraft next loop remains unobserved; no changed concrete method presently connects that milestone to these residuals. Execution metrics unchanged:3269 natural and3136 artificial ROM starts,4196 union (not byte coverage).

map.validate, map.save and map.export succeeded. [Fresh checkpoint verification](pass-002-final-verification.json) confirms byte_identical=true. State/current/checkpoint hash e88d90dd5992842c20317e59619cee3148dd84ce23395d5f01bb40f9f63e0eff; [identities](round-002/checkpoint-identities.json). Input ROM SHA256 unchanged. Configured binary used throughout; no toolkit changes.

Pass complete at supported easy-gain handoff, not global completion/terminal exhaustion. Current commentary source retains8292800b9b7ca4fad4f5e65d9f3d8b2e8098c225ace6c07ad771ac34b55448fb and frozen phase1 reference, but covers previous checkpoint: full current-source readability now needs the authorized commentary phase. Preserve prior notes/symbols while incorporating newly classified spans. Then redirect reviews exact19 residuals and strongest competing structural interpretations; this pass does not supply a terminal review.

One local pagination mistake repeatedly requested an empty map page until stopped, before annotations; its97MiB log is retained, with separate bounded final-rpc-log.jsonl. Two annotation-shape mistakes were corrected (views instead of top-level context, execution views only for Code); failed in-memory attempts were discarded before state save. No toolkit defect inferred. No toolkit issues found:zero confirmed/suspected bugs, missing features or blockers; [persistent report](toolkit-issues.md). Disk3.4GiB available, round002111MiB retained. No cleanup/archive/delete/publish/toolkit edit. Compact [handoff](round-002-disassembly-handoff.md).

## Round002 commentary active frontier

Entire previous commentary copied intact to round-002/commentary-pass001-preserved;
original also retained under commentary-pass001-working-preserved. Supported fresh
prepare succeeded against pass-002-final.742-entry current scaffold; all512 prior
units carried by physical identity and180 new units integrated (692 total).160 new
code explanations and20 data structures cover newly classified5451 bytes, including
exact SHX directive and selector-data barrier. First integrated verify byte-identical
source af99940e82e84aa5d50e84957537edfe24038ece89c36d3bd591fea74e3c5d06.
Active frontier: changed-dependency prose/numeric-alias consistency, source inventory
checks, final verification and reading copy; no redirect handoff yet. Corrected older
false uniform descriptions where mixed data evidence mentioned neighboring fill.
No terminal editorial audit, structural/ROM edits, cleanup or publication.

## Round002 commentary complete — next redirect

Current-source readability complete across742 scaffold entries (456 Code,280 Data,
one Image,five explained Unknown), with692 reviewed meaningful units. All512 prior
units retained by physical identity;180 new units explain the5451-byte accepted batch.
Whole previous commentary remains intact under round-002/commentary-pass001-preserved
and round-002/commentary-pass001-working-preserved. New frozen baseline is pass002.

New160 code explanations,20 structures and scoped scratch aliases cover object
continuations/allocators, three ending actors/background streaming, audio stream
commands/stack unwinds and fixed helpers. Corrected old mixed-data uniform claims,
stale continuation uncertainty and understood numeric call aliases. Selector Data
103810..103813 and independently called code103813 retained; exact SHX$9e0007 remains
with local behavioral uncertainty, no STA substitution.

Fresh verification byte_identical=true, game.asm SHA256
24eb4e18af23f9773c64a4383f9d65d07d069deb03ee95048ad8b8344b087b9c;
reading-after.asm refreshed. [Inventory checks](round-002/commentary-readability-checks.json)
confirm frozen/current state/map identity e88d90dd5992842c20317e59619cee3148dd84ce23395d5f01bb40f9f63e0eff
and unchanged ROM. Current-source readability gaps:none; terminal editorial audit pending.
Structural classification stays262141/262160,19 unknown/five ranges, no new observed
execution. Redirect must assess strongest residual inferences under evidence-policy,
not inherit the narrow prior review as terminal exhaustion. Compact
[handoff](round-002-commentary-handoff.md). No toolkit issues found, no blockers;
configured binary only. Disk3.4GiB free,round002122MiB retained. No cleanup, deletion,
publication, toolkit changes or global completion.

## Round002 redirect complete — independent disassembly next

Attempt [round002-residual-roles](redirect/round002-residual-roles/results.md) reopens
prior narrow residual judgments under evidence-policy using combined primary
neighbors/control/data context. Ready R004 proposes16 Static Data bytes:
8714:1,105610:1,130820:4,131072:10. Exact slices/bank contexts/rationales in
[proposals](redirect/round002-residual-roles/proposals.json). Palette-origin words
match independently encoded$FB6F/$FBFF; singleton separators and metadata-like
trailer have independent bounds and explicit competing-code review. No invented
schema/sentinel/consumer/original generator. Proposed, acceptedzero; state still
262141/262160 with19 unknown. CMP#DF/RTS13913:3 remains separate genuine ambiguity,
with a coherent compare-return contract but no independent code/data discriminator.
No terminal exhaustion decision; if16 accepted,3 remain for next redirect assessment.

Changed practical progression method restored ending-animation snapshot, released
buttons and patched supported completion latch RAM$9E=1 while retaining PC/stack.
Six bounded frames reached stage1/checkpoint0 and banks[0,1,14,15]. Focused fresh
trace captured seven actual restart instructions$C12F..$C13C including both stores
and JMP$C095. Stage map image preserved; scenarios.json updated with prior history.
Artificial completion shortcut only, no claim of naturally finishing aircraft or
playing second loop. Initial client trace-key/capture-overwrite errors preserved;
focused filter resolves the measurement question. No toolkit defect inferred.

Input pass-002-final map/state hash e88d90dd5992842c20317e59619cee3148dd84ce23395d5f01bb40f9f63e0eff
and source24eb4e18af23f9773c64a4383f9d65d07d069deb03ee95048ad8b8344b087b9c
unchanged, as is input ROM. Current-source readability742 entries/692 units remains
input gate; final editorial audit pending. One stale ending dispatcher sentence at
source6999 reported for editorial correction. Leads/pass history appended; previous
records preserved. No toolkit issues found:zero bugs/suspected bugs/missing features
or blockers. Configured release binary; no ROM/map/state/source/toolkit edits,
archive/deletion/cleanup/publication/global completion. Disk3.4GiB free,attempt9.1MiB.
Compact [handoff](round-002-redirect-handoff.md).

## Round003 disassembly active frontier

Independent review of R004 at pass-002-final; exact16-byte generic non-code proposal under review, input262141 classified/19 unknown. Current frontier: apply four independently bounded Static Data slices after primary physical/source inspection; preserve13913:3 ambiguity. No gameplay replay or commentary edits. Evidence and bounded RPC in round-003/. Fresh pass-003-final required before handoff.

## Round003 disassembly complete — incomplete ROM

Current phase: disassembly finished; next full current-source commentary, then redirect. Independently accepted R004 four exact generic Static Data slices8714:1,105610:1,130820:4,131072:10 (16 bytes), from pass-002-final. Output pass-003-final. [Review](round-003/review.md) assesses combined primary physical/source constraints and concrete competing code interpretations without requiring a consumer/schema for generic data. [Fresh inspection](round-003/independent-review.json), [live internal counts](round-003/counts.json), [current complement](round-003/unknown-ranges.json). No new Observed or natural reachability claim.

ROM classification: 100.00% (262157 / 262160 bytes classified; 3 bytes unknown).

Unique physical classification:20669 Code,110416 Data (including16-byte header),131072 CHR Image;741 annotated regions. Rounded100.00% does not establish completion. Exactly13913:3 remains unknown: bank1 CPU$B649, C9 DF60. CMP#DF/RTS has coherent useful compare-return contract; isolated constants also possible. Independent bounds do not discriminate. No arbitrary forced execution or guessed code/data role. No terminal exhaustion judgment supplied this phase.

| PRG 8KiB bank | Code | Data | Classified | Unknown |
|---|---:|---:|---:|---:|
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 6296 | 1893 | 8189 | 3 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 7 | 8185 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 7 | 8185 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 7 | 8185 | 8192 | 0 |
| 8 | 7 | 8185 | 8192 | 0 |
| 9 | 0 | 8192 | 8192 | 0 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 7 | 8185 | 8192 | 0 |
| 12 | 3691 | 4501 | 8192 | 0 |
| 13 | 344 | 7848 | 8192 | 0 |
| 14 | 7827 | 365 | 8192 | 0 |
| 15 | 2476 | 5716 | 8192 | 0 |

All16 CHR banks8192 Image classified/zero unknown. Previous natural/artifical execution coverage remains separate; this static pass performs no runtime replay. Scenarios/gameplay reviewed: representative areas, late Airfield mechanics, congratulations, aircraft and artificial post-ending restart already observed. No changed gameplay recipe discriminates the remaining compare island.

map.validate/save/export succeeded. [Fresh verification](pass-003-final-verification.json): byte_identical=true; map/state/checkpoint SHA256 e7d18ba36a6dee5f96484039ecd08668a198eb503f38fe550743e7189cd935c4. [Identities](round-003/checkpoint-identities.json) confirm ROM unchanged and commentary game.asm remains24eb4e18af23f9773c64a4383f9d65d07d069deb03ee95048ad8b8344b087b9c. All prior commentary/evidence/checkpoints retained intact; next full current-source commentary must update four formerly Unknown spans and stale adjacent prose. Final editorial audit remains separate.

No toolkit issues found:zero confirmed/suspected bugs, missing features or blockers; [report](toolkit-issues.md). Configured release binary only; no toolkit/ROM/commentary edits, archive, cleanup, deletion or publication. Bounded RPC861KiB, fresh review output22KiB; disk3.3GiB available. Handoff [round-003-disassembly-handoff.md](round-003-disassembly-handoff.md). Global goal not marked complete.

## Round003 coordinator commentary acceptance

Separate commentary worker completed all742 ranges and696 units; prior entire commentary preserved under round-003/commentary-pass002-preserved. Four Data slices and affected neighboring prose/ending dispatcher repaired. Coordinator verified all source locations, current reading copy and final byte-identical source hash6a4b44a4b937f9be225b74ca2e80fd13aeeec5ef997545fae7072f014f9ddf2c; evidence round-003-commentary-handoff.md and round-003-commentary-coordinator-check.json. Classification remains262157/262160,3unknown at13913. Next separate redirect worker running; final editorial audit pending. No archive/cleanup/delete/publish/toolkit edits.

## Round003 redirect ready — proposed3 Static Code bytes

Input pass-003-final state/current e7d18ba36a6dee5f96484039ecd08668a198eb503f38fe550743e7189cd935c4; verified current-source readability742 ranges/696 units, source6a4b44a4b937f9be225b74ca2e80fd13aeeec5ef997545fae7072f014f9ddf2c. Exact complement remains13913:3, bank1 CPU B649 C9DF60; classified262157/262160, unknown3.

[Research](redirect/round003-compare-contract/results.md) corrects prior isolated-decode framing: DF is independently actual vertical clipping boundary in nearby object/ending/projectile consumers; complete CMP/RTS contract returns comparison flags while preserving A/X/Y; independent B648/B64C boundaries and neighboring uncalled duplicate metasprite helper support code in this local helper cluster. R005 proposes3 Static Code bytes, acceptedzero; no actual caller/Observed/dead-code claim. Strongest constants alternative and lack of exact duplicate recorded. New bounded PRG pattern comparison excludes cross-instruction false hit122613.

No connected unknown continuation or independent credible changed route identified. Broad all-five-area/ending/artificial restart routes already observed; no replay. Map/state/source/ROM hashes unchanged, proposal returns for independent disassembly; final editorial audit untouched. No toolkit issues found; configured binary unchanged. No runtime/toolkit edits/archive/cleanup/deletion/publication/global completion. [Handoff](round-003-redirect-handoff.md).

## Round004 disassembly active frontier

Independent R005 review active at pass-003-final (262157 classified;3 unknown). Assess exact13913:3 CMP#DF/RTS using primary neighbor boundaries, independently used rendering threshold and local helper cluster; live caller is not prerequisite. Preserve source/commentary and prior evidence. Exactly one pass; fresh pass-004-final checkpoint required. No unchanged gameplay replay.

## Round004 disassembly complete — structural classification complete; workflow pending

Current phase: disassembly finished; next full current-source commentary, then redirect and final editorial acceptance. Independently accepted R005 exactly13913:3 Static Code, CPU$B649 bank1 context[0,1,14,15], mechanical CompareAccumulatorWithDf. [Review](round-004/review.md) explains combined independent helper boundaries, real DF clipping-domain semantics, local uncalled duplicate helper and flag-return contract; it assesses the constants competitor and prior rejection scope. [Fresh physical/decode](round-004/independent-review.json). Caller, exact intended use and natural reachability remain unresolved. No Observed/dead-code claim.

ROM classification: 100.00% (262160 / 262160 bytes classified; 0 bytes unknown).

Unique physical bytes:20672 Code,110416 Data (including16-byte header),131072 CHR Image;742 annotated regions. Exact unknown complement empty, [internal unknown query](round-004/unknown-ranges.json); [internal unique counts](round-004/counts.json). All physical bytes now meet the structural classification contract. This is not global workflow completion.

| PRG 8KiB bank | Code | Data | Classified | Unknown |
|---|---:|---:|---:|---:|
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 6299 | 1893 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 7 | 8185 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 7 | 8185 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 7 | 8185 | 8192 | 0 |
| 8 | 7 | 8185 | 8192 | 0 |
| 9 | 0 | 8192 | 8192 | 0 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 7 | 8185 | 8192 | 0 |
| 12 | 3691 | 4501 | 8192 | 0 |
| 13 | 344 | 7848 | 8192 | 0 |
| 14 | 7827 | 365 | 8192 | 0 |
| 15 | 2476 | 5716 | 8192 | 0 |

All16 CHR banks individually8192 Image classified/zero unknown; header16 Data. Per-bank details retained in counts.json. Existing broad natural/artificial progression, five areas, later mechanics, ending/aircraft and artificial restart reviewed and reused; no unchanged gameplay replay or new execution evidence. No connected unknown continuation remains after RTS.

map.annotate/validate/save/export succeeded. Fresh pass-004-final checkpoint [verification](pass-004-final-verification.json) byte_identical=true. State/current/checkpoint SHA256 a8d3fafaa2954317e567303d28a06d265a2fa885f65747dc604805bbab5b060d; [identities](round-004/checkpoint-identities.json). Input ROM unchanged. Commentary source remains6a4b44a4b937f9be225b74ca2e80fd13aeeec5ef997545fae7072f014f9ddf2c, intact. Prior742-range/696-unit readability applies to previous map: full current-source commentary must incorporate this newly classified routine and correct stale unknown/unsupported-entry prose before final editorial audit. Preserve previous symbols/comments/evidence.

No toolkit issues found:zero confirmed/suspected bugs, missing features or blockers; [persistent report](toolkit-issues.md). Configured release binary only; no toolkit/ROM/commentary edits, archive, cleanup, deletion or publication. Round004 evidence1.9MiB, disk3.4GiB free. Exactly one disassembly pass; prior checkpoints/history intact. Compact [handoff](round-004-disassembly-handoff.md). Global goal not marked complete.

## Round004 commentary active frontier

Full current-source readability active on pass-004-final. Preserve entire pass003 commentary before fresh supported preparation; reuse sound742-range/696-unit review by physical identity. Integrate13913:3 CompareAccumulatorWithDf and affected structural-unknown prose; check full inventory, symbols, effective origins, exact SHX and selector barrier. Fresh verification/reading copy pending. Final editorial audit remains pending; redirect follows.

## Round004 commentary complete — redirect and final acceptance pending

All742 current-source ranges/697 meaningful units reviewed;696 prior units carried. Entire old commentary preserved in round-004/commentary-pass003-preserved before supported fresh prepare. Added CompareAccumulatorWithDf contract at13913:3; consistent exact $df threshold alias in3 independent renderers; actual caller/use unresolved. Removed stale residual claims; corrected post-aircraft uncertainty to acknowledge prior artificial restart. Exact emissions/symbols/origins, source locations, SHX and selector barrier verified; no readability gaps.

Fresh byte-identical verification/read refresh passed; source07130266f50f9f8cbef57159397a2d6d1c81067cd4a1197c7d735552e46dc257. Map/state/frozen baseline unchanged a8d3fafaa2954317e567303d28a06d265a2fa885f65747dc604805bbab5b060d; classification262160/262160 unknown0. Final editorial_audit pending and audit not run. Separate redirect precedes final acceptance. [Handoff](round-004-commentary-handoff.md), [checks](round-004/commentary-readability-checks.json). No toolkit issues found; no toolkit/ROM edits, runtime replay, archive, cleanup, deletion or publication. Global workflow not marked complete.

## Final editorial acceptance complete — coordinator final accounting pending

Separate sequential final worker resolved E004 allocator stale unknown-copy prose in
source/inventory; used consistent allocator aliases and corrected older spawn/projectile
and ending-renderer argument naming. Reused valid742-range/697-unit scope credits,
checked full inventory/source locations, canonical operand widths/emissions/control flow,
frozen effective origins, SHX9E0007 and selector Data103810..103813. All pass.
Supported final commentary audit passed byte_identical=true/editorial_audit=true;
reading copy refreshed. Source002f17948e94bfaf4120f484ab222696ee47913073870a3c71a5556d64fbf918.
Classification262160/262160(100%),unknown0; state/map/frozen baselines unchanged
a8d3fafaa2954317e567303d28a06d265a2fa885f65747dc604805bbab5b060d.
No open editorial action; ordinary semantics/reachability uncertainties remain.
[Handoff](final-editorial-handoff.md),[checks](final-editorial/checks.json),
[findings](../commentary/editorial-audit.md),[audit](../commentary/audit.json).
No toolkit issue,archive,cleanup,deletion or publication. Global goal still active:
coordinator final requirement audit and archive-options document selection/accounting pending.

## Coordinator terminal handoff — whole-ROM complete

ROM classification: 100.00% (262160 / 262160 bytes classified; 0 bytes unknown).

All four rounds used separate sequential disassembly/commentary/redirect workers; final separate editorial worker resolved E004 and scoped naming gaps. Final audit byte_identical=true/editorial_audit=true,742 reviewed ranges/697 units. Final source SHA256002f17948e94bfaf4120f484ab222696ee47913073870a3c71a5556d64fbf918; pass-004-final state/current/frozen map SHA256a8d3fafaa2954317e567303d28a06d265a2fa885f65747dc604805bbab5b060d. Configured binary store/map validation independently passed. Exact partition has no gaps/overlaps or unsupported Unknown/raw fallback. All leads verified, no ready/blocked required work.

Completion audited requirement-by-requirement in terminal-verification.json; terminal-handoff.md links final mapping, source, evidence, review and limitations. Current reading copy and source locations checked. Archive contract inspected/unchanged; ../archive-options.json accounts for every relevant Markdown file with final conclusions/handoffs/results selected, copied instructions/verified duplicates omitted without removal. All artifacts remain local; no archive,cleanup,delete,publish or toolkit/ROM edit. No toolkit issues found. Static structure and readable source complete; uncertain intended semantics/natural reachability and compatibility limits remain explicit. No required work remains; coordinator may mark orchestration goal complete.
