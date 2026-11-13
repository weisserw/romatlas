# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.


## Round 1 disassembly evidence and outcomes

Baseline separately verified, not classification. Accepted actual ROM instruction
fetches and coherent connected static paths from explicit effective bank contexts.
The saved code-review.json contains all first-batch instruction semantics and
candidate views. late-code-review.json adds observed code from late scenarios.
No pointer hypothesis alone was accepted as code.

Static discovery false positives were localized: physical $215c..$2164 is an
eight-byte table indexed by random low2bits and optional OR4 at $212a, bypassed by
LDA0/STA/BPL; $ee6f..$ee73 is four bytes indexed by AND3/TAX/LDA $ae5f,X at $ee57;
$1da78..$1da7c is four low-valued records read at $1dabf, bypassed by complementary
BNE/BEQ. These specific consumers and exact independently fetched adjacent entries
support Data. No blanket rejection of static unexecuted code followed.

Uniform $ff runs totaling35257 bytes: exact changed-byte boundaries and candidate
conflict checks in data-boundaries.json; generic Data because repeated ISC decoding
across long runs has no coherent code contract. Purpose as linker fill is not
asserted. Seven nine-byte bank bootstrap copies and six-byte vector blocks were
classified from matching validated reset contract and whole-bank trailer structure.
CHR Image classification derives independently from validated iNES section layout
and PPU-only pattern backing, not a CPU-coverage assumption.

Secret button masks/table and110-byte zero-terminated credits have inspected
consumers. Drawing data $2c34..$3121 shows repeated tile-index/count/palette/address
patterns following a terminal JMP and ending at long $ff run, with no conflicting
code; exact individual format/record boundaries remain unresolved. Generic Data
classification is sufficient and not contingent on complete graphics schema.

Gameplay evidence and counterexamples: NES button/pressed calls were ignored;
buttons array confirmed by RAM and normal scene2→3 transition. Only corrected
routes claim presses. Natural sports-store scene16 and artificial scenes13–17
exercised representative mechanics; result/story/shell/fountain/ballroom scenes
also visited. Screen7 test returned to title. Boss trigger and true terminal flag
unknown; practical scene shortcuts succeeded, so no arbitrary RAM brute force
followed. Each branch restores clean scene4.snapshot and records exact patches,
banks/budgets/results. Natural reachability is not claimed for artificial scenes.

Open leads R1-001 (remaining table/graphics consumer extents), R1-002 (goal/boss
and ending contract), R1-003 (RAM source identities/indirect dispatch). Recipes
in leads.json. This pass stops at a supported connected batch and transfers deeper
consumer/contract research to redirect after full current-source commentary.


## Round 1 disassembly completed — incomplete ROM

ROM classification: 71.05% (186253 / 262160 bytes classified; 75907 bytes unknown).

Checkpoint: [pass-001-final verification](pass-001-final-verification.json),
[map](../maps/pass-001-final.json), [source](../generated/pass-001-final/game.asm).
Internal state analysis/state.json and portable maps/current.json validated. Fresh
ca65/ld65 reconstruction byte-identical true; ROM SHA256 remains unchanged.
Earlier baseline and pass-001 checkpoints retained. Counts: Code18465, Data36606,
Image131072, Text110; remaining75907 unknown. Header16 and CHR131072 are fully
classified outside PRG table below. All 693 regions have structural evidence;
code execution views explicit. Semantic commentary/readability is next phase,
not claimed complete. Provenance byte totals: {'Data/Static': 36606, 'Image/Static': 131072, 'Code/Static': 2327, 'Code/Observed': 16138, 'Text/Static': 110}.

| PRG 8KiB bank | Code | Non-code | Unknown |
|---|---:|---:|---:|
| 0 | 2606 | 0 | 5586 |
| 1 | 1442 | 5201 | 1549 |
| 2 | 496 | 589 | 7107 |
| 3 | 9 | 8183 | 0 |
| 4 | 1182 | 0 | 7010 |
| 5 | 49 | 509 | 7634 |
| 6 | 420 | 2675 | 5097 |
| 7 | 2926 | 4203 | 1063 |
| 8 | 1940 | 1289 | 4963 |
| 9 | 9 | 929 | 7254 |
| 10 | 0 | 0 | 8192 |
| 11 | 9 | 687 | 7496 |
| 12 | 0 | 1471 | 6721 |
| 13 | 9 | 8183 | 0 |
| 14 | 4906 | 4 | 3282 |
| 15 | 2462 | 2777 | 2953 |


Evidence: analysis/disassembly-r1/code-review.json (7188 candidate instructions;
6131 fetched physical starts,1057 static additions before three corrected inline
data barriers), late-code-review.json (1226 additional accepted instruction starts),
contexts-all.json, coverage-final.json, edges-final.json, trace-summary.json.
Zero aggregate drops; detail drops expected for capacity0. Execution coverage is
separate from classification and includes RAM and context identities. Actual
trace summary unique-instructions12466 at late batch, later ballroom may add
roots; see saved final summary for authoritative totals.

Gameplay: corrected natural route through story to scene16/stage4 sports store;
five initialized gameplay scenes13–17 exercised Right+A+B; Select/Down exercised
naturally. Artificial scenes5–11 include result, soda exterior, bedroom, shell,
fountain and ballroom. Scene7 attempt returned to title. Ballroom persisted and
advanced300 frames; terminal/ending semantics unverified. See scenarios.json,
gameplay.md and exact shortcut-scenarios.json/late-scenarios.json. Earlier note
calling natural gameplay scene13 was corrected by saved RAM (scene16).

Easy-gain frontier: no further repetitive inputs. Residual PRG ranges mix code,
compressed drawing streams and tables; banks10/11 need consumer-format inspection,
not classification based on absence of execution. Boss/goal-state contract and RAM
upload/synthesis identity need redirect. Leads produced R1-001..003, none consumed
(no initial leads). Static fallthrough ORA/BRK candidates at $215c,$ee6f,$1da78
were superseded by consumer-supported tables, not left as executable bytes.
No hidden unreviewed candidate batch awaits automatic import.

Toolkit findings: one suspected input-schema usability/API-validation issue
NES-001, no confirmed bugs or blocking issues; [report](toolkit-issues.md).
Disk handoff: over6GiB free, exploratory scratch under16KiB; durable evidence and
snapshots local. No toolkit edits, archive/publication, ROM modifications or deletion.
Next phase commentary should cover all currently classified source, then redirect
investigates the three concrete frontiers. This is an easy-gain handoff, not terminal
exhaustion or a whole-ROM completion claim.

## Redirect R1 active resource/world contract
See redirect/r1-resource-contract/plan.md. Exact83-byte immutable CHRbank5 toRAM0200 comparison established; after bounded scene13 transition, actual RAM instruction contexts captured separately from backing role. World bank5/6 PRG resource directories give12 non-placeholder encoded structures; section bounds under audit. Pending stage loader static continuation has supported synthetic-return contract and bank3 handler table. No classification changes.

## Redirect R1 outcome — combined ready batch
See redirect/r1-resource-contract/handoff.md and proposals.json. Research complete for identified coherent batch: world12-container extent proof, CHR resource/copy/compressed-stream roles and all32-bank graphical audit, finite scene callback descriptors, all five object-handler graphs through bank-preserving helpers and finite response/collision dispatch, five final-checkpoint environments and supported artificial terminal presentation route. Final candidates34397 unknown bytes plus9472 role corrections, accepted0. V6 manually reviewed contracts and machine boundary/overlap audit; sole remaining graph edge is existing selector Data, with primary-evidence rejection scope in selector-barrier-audit.md. Other residuals remain unknown for future source-driven redirect; not terminal exhaustion. ROM/map/state/source unchanged; six ready leads require separate independent disassembly.

## Disassembly R2 completed — pass-002-final

ROM classification: 84.23% (220816 / 262160 bytes classified; 41344 bytes unknown).

Incomplete. Gain34563 newly classified bytes: reviewed R1 manifest34397 plus166 connected/gameplay Code;9472 prior Image bytes corrected to Data. Totals Code28867,Data70239,Text110,Image121600. All structural regions carry evidence; no missing evidence annotations. Static provenance remains separate from execution: {'Data/Static': 70239, 'Code/Static': 12645, 'Code/Observed': 16222, 'Text/Static': 110, 'Image/Static': 121600}.

Independent review: [audit](disassembly-r2/independent-audit.json), all134 proposed Code extents decoded with configured toolkit,4246 object starts/lengths matched; source-reviewed bank restoration, finite RAM writes, stack/inline collision/spawn contracts. Recomputed12 world containers (five plane strides,16-byte columns,11-byte metatiles,dimension/end constraints),18 descriptors,5 response matrices,49-page prefix (duplicate and shared empty records),10 CHR streams outputting1024 bytes. Visual32-bank contact reviewed; rejected header-only Image premise corrected. World fields and CHR directory/payload/stream/fill roles subdivided; remaining unknown semantics not extra classification prerequisites. Existing selector[00ee6f,00ee73) remains Data; no pointer-overrun or negative-frame fallthrough imported.

Connected continuation: allocator failure/alternate entry and primary-slot scan adds67 Static bytes; exact stack skip7/8(+3 motion) and return carry contracts in allocator-full.json/allocator-special.json. Later music-stage normal inputs within retained artificial final checkpoint state add99 Code bytes:37 fetched starts/84 bytes Observed plus6 branch successors/15 bytes Static. Restore redirect/r1-resource-contract/boss-17.snapshot, release, Right+A+B30,release15,Left+B30,release15 frames; each3M CPU-tick/10s bound.90 frames advanced3898->3988; no aggregate drops, detail capacity0. Snapshot/frame/results retained. An initial context query exceeded documented1000 limit; reran same bounded route with paginated queries to preserve evidence, no inference from failed query.

| PRG8KiB bank | Code | Non-code | Unknown |
|---|---:|---:|---:|
| 0 | 2785 | 152 | 5255 |
| 1 | 1498 | 5216 | 1478 |
| 2 | 1685 | 659 | 5848 |
| 3 | 9 | 8183 | 0 |
| 4 | 2925 | 164 | 5103 |
| 5 | 2517 | 1596 | 4079 |
| 6 | 2471 | 2798 | 2923 |
| 7 | 3010 | 4203 | 979 |
| 8 | 2399 | 1327 | 4466 |
| 9 | 9 | 929 | 7254 |
| 10 | 0 | 8186 | 6 |
| 11 | 9 | 8183 | 0 |
| 12 | 0 | 8186 | 6 |
| 13 | 9 | 8183 | 0 |
| 14 | 5344 | 10 | 2838 |
| 15 | 4197 | 2886 | 1109 |

Verification: map.validate passed; state==portable current map; input hash unchanged. Fresh [pass-002-final verification](pass-002-final-verification.json) confirms byte-identical262160-byte rebuild using configured release binary; exact counts/provenance/hash in [final-counts](disassembly-r2/final-counts.json). Prior pass-002-reviewed remains preserved intermediate checkpoint. Generated source: generated/pass-002-final/game.asm. Commentary source unchanged; full current-source commentary catch-up still required.

Lead outcomes R1-001..006 verified structurally; natural use/selector domains/ordinary ending timing remain separately unresolved. Updated leads/scenarios. Progression evidence already covers five stages and artificial result/exterior/ballroom; this pass reuses it and adds bounded final encounter movement. No broad gameplay or research exhaustion claim. Further repetitive inputs currently offer reduced expected gain; residuals should be reexamined against new source/references in redirect after commentary. Unknown ledger retains physical banks and exact41344-byte complement.

Toolkit: no new issues; existing one suspected bug NES-001 and two documented missing features NES-002/003, no blocker. See [toolkit issues](toolkit-issues.md). No toolkit changes, publication, deletion or source commentary changes.

## Redirect R2 — complete ready research batch
Inputpass-002-final; current-source commentary complete. See redirect/r2-lookup-contract/handoff.md,proposals.json,audit.json anddeferred-frontier.json. R2-001..006 ready;39360 unknown-byte proposal,0 accepted,1984 forecast residual. Connected finite containers/dispatch plus independent text/music/auxiliary/static-helper contracts; exact known overlap and relocated-noise counterexample localized. No terminal exhaustion assessment. Current map/state/source unchanged; no server started. Independent disassembly then full current-source commentary/redirect next.

## Disassembly R3 completed — pass-003-final

ROM classification: 99.24% (260176 / 262160 bytes classified; 1984 bytes unknown).

Incomplete. Accepted full independent R2 structural union39360 bytes:9776 Static Code and29584 Static Data. Totals Code38643,Data99823,Text110,Image121600. No rejected proposed span; no natural-reachability claim added. All structural regions have evidence. Provenance {'Data/Static': 99823, 'Code/Static': 22421, 'Code/Observed': 16222, 'Text/Static': 110, 'Image/Static': 121600}.

[Independent audit](disassembly-r3/independent-audit.json) recomputes all eight resource parsers and connected walker separately, then checks all4320 contextual instruction starts with configured decoder plus205 Code extents, immutable physical contexts, packed width/height and exact endpoints, collision sharedFF, and nine relocated noise words. Source-reviewed sprite installer/stream consumers, DDDF10/11 versus DDD8 7/8 inline bytes, explicit DE0B slot/CPX7 paths, trigger entry-minus-one, collision inline and bank/stack restoration contracts. Track0 alias,577-byte stream8-bit traversal and generic non-code semantics caveats preserved. Complete70/110-byte helpers accepted for structural contracts without requiring natural callers. No arbitrary indirectFX targets, universal resource-bank preservation or ordinary clipping-unwind returns inferred.

Intentional modulation92B4/92B7 Code overlap uses one physical primary LDY#4/BIT$02A0 view and explicit alternate AudioModulationTwo=$92B7 comment. Both literal entry words retained independently. Artificial targeted trial from saved boss17, release inputs, registers PC92B7/Y0: fetched LDY#2 and reached92B9. Primary92B4/Y0 fetched LDY#4 then pending interrupt preempted next step; bounded500-operation trial stopped within interrupt. This limits runtime confirmation only; static BIT representation independently validates/rebuilds. Trial details [overlap-runtime](disassembly-r3/overlap-runtime.json), initial short trial preserved. All accepted new bytes remain Static.

| PRG8KiB bank | Code | Non-code | Unknown |
|---|---:|---:|---:|
| 0 | 3770 | 4037 | 385 |
| 1 | 2488 | 5487 | 217 |
| 2 | 4484 | 3637 | 71 |
| 3 | 9 | 8183 | 0 |
| 4 | 3829 | 4203 | 160 |
| 5 | 4122 | 3927 | 143 |
| 6 | 2848 | 5261 | 83 |
| 7 | 3078 | 5048 | 66 |
| 8 | 3640 | 4373 | 179 |
| 9 | 9 | 8183 | 0 |
| 10 | 0 | 8186 | 6 |
| 11 | 9 | 8183 | 0 |
| 12 | 0 | 8186 | 6 |
| 13 | 9 | 8183 | 0 |
| 14 | 5579 | 2262 | 351 |
| 15 | 4769 | 3106 | 317 |

Saved map/state equal; map.validate passed. Fresh [pass-003-final verification](pass-003-final-verification.json) confirms byte-identical262160-byte rebuild using configured release binary. Map SHA256 7da73015f4ea41aff44f2448f6d45e43d85883b6b93b1c134b692e5375239230. Input unchanged; current prior commentary source unchanged. Generated current source generated/pass-003-final/game.asm awaits separate full commentary. [Counts](disassembly-r3/final-counts.json); [exact168-range residual ledger](disassembly-r3/unknown-ledger.json); [annotations](disassembly-r3/accepted-regions.json).

All six R2 leads verified structurally. Reuse prior five-stage/final-encounter and artificial result/exterior/ballroom progression; no repeat startup, route or broad-exhaustion assertion. Supported connected batch fully reviewed; remaining1984 unknown need localized entry/code-data/inline/bank/stack constraints, not arbitrary byte quotas or natural-selector proof for accepted non-code. Specific frontier in redirect/r2-lookup-contract/deferred-frontier.json; current-source commentary then redirect may reassess new references. No identified ready supported continuation withheld.

Toolkit: no new toolkit issues found. Existing NES-001 suspected and NES-002/003 documented missing features retained, no blocker. [Report](toolkit-issues.md). No toolkit/ROM/commentary edits, archive, deletion or publication. All own servers exited.

## Disassembly R4 completed — pass-004-final

ROM classification: 100.00% (262147 / 262160 bytes classified; 13 bytes unknown).

Incomplete despite rounding100.00%. Accepted166disjoint spans1971unique bytes:978StaticCode/993StaticData; no rejected proposed span. Totals Code39621/Data100816/Text110/Image121600. Provenance{'Data/Static': 100816, 'Code/Static': 23399, 'Code/Observed': 16222, 'Text/Static': 110, 'Image/Static': 121600}. No new Observed/natural reachability assertion.

Independent consumer/field/control/sharedsuffix/stack review [review](disassembly-r4/review.md); [audit](disassembly-r4/independent-audit.json) recomputes58inline572-byte endpoints from actualJSR/slot/type/save-return contracts and configured55Codeextents427contextual instruction starts. Three2byte initializers accepted under complete sharedsuffix contracts, not arbitrary valid decode; resource continuations have coherent scene setup contracts separate from selected-resource-return preconditions.53Data421bytes include bounded pitch prefixes, explicit8byteFXrecords, scene arrays and localized generic non-code after reference/source/neighbor inspection. [Accepted annotations](disassembly-r4/accepted-regions.json).

| PRG8KiB bank | Code | Non-code | Unknown |
|---|---:|---:|---:|
| 0 | 3878 | 4314 | 0 |
| 1 | 2538 | 5654 | 0 |
| 2 | 4495 | 3697 | 0 |
| 3 | 9 | 8183 | 0 |
| 4 | 3874 | 4315 | 3 |
| 5 | 4150 | 4042 | 0 |
| 6 | 2925 | 5266 | 1 |
| 7 | 3130 | 5062 | 0 |
| 8 | 3657 | 4534 | 1 |
| 9 | 9 | 8183 | 0 |
| 10 | 6 | 8186 | 0 |
| 11 | 9 | 8183 | 0 |
| 12 | 6 | 8186 | 0 |
| 13 | 9 | 8183 | 0 |
| 14 | 5889 | 2298 | 5 |
| 15 | 5037 | 3152 | 3 |

Nine isolatedRTS and four-byte1D53A..1D53E remain13Unknown bytes/10ranges [exactledger](disassembly-r4/unknown-ledger.json). Their empty-return/scalar and pointer/sentinel/instruction interpretations remain unresolved; no arbitrary execution or raw fallback classification. No known ready supported continuation withheld; separate full current-source commentary then redirect may supply controlling entry or typed boundary. This pass does not claim terminal exhaustion. Prior broad five-stage/final/artificial result/exterior/ballroom scenarios reused; no new discriminating progression question, no repeated route. Intentional92B4/92B7 alias, relocated noise pointers, collisionFFsharing, track0/envelope alias and577byte traversal caveats preserved.

map.validate passed; saved state equals current portable map. Fresh [verification](pass-004-final-verification.json) byte-identical262160-byte rebuild using configured binary. Map/state SHA25623adea33fb84196e636168943c2024d693895008653a6ac8d81d2ae306655c5d; ROM and prior commentary unchanged. Generated current source generated/pass-004-final/game.asm requires separate commentary. [Counts/provenance](disassembly-r4/final-counts.json).

R3-001..003 verified structurally; no rejected proposed span. No new toolkit issues found; [report](toolkit-issues.md) retains NES-001suspected/NES-002/003documented missing features, no blocker. No toolkit, ROM or existing commentary changes; no archive, deletion, publication or editorial audit. Own servers exited.
