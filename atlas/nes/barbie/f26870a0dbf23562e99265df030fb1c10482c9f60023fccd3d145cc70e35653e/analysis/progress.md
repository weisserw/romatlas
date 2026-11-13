# Disassembly progress

Round 1, disassembly phase active. Verified baseline established: maps/baseline.json,
generated/baseline, analysis/baseline-verification.json (byte-identical true).
Classification investigation active in analysis/disassembly-r1; no baseline bytes
are treated as classified. Startup copyright screen identifies Barbie (1991).
Tracing and bounded gameplay continue; server state is session-local until saved.

Runtime observations: initial accepted fetch roots 1,072 physical ROM instructions;
contextual discovery yielded 1,365 unique candidates, not yet annotations. Evidence:
analysis/disassembly-r1/contexts-1.json, observed-decodes.json, discovery-candidates.json.
Title and scrolling introduction observed; investigating screen state $2e and
frame callbacks $30/$32 to bypass slow presentation using supported transitions.

Durable map now classifies validated 16-byte header, 131072-byte CHR image section,
and six interrupt-vector bytes. Saved analysis/state.json. Prior button/pressed
calls did not press NES buttons; corrected buttons-array input naturally transitions
credits scene 2 to story scene 3. Only corrected routes will claim actual presses.

Corrected natural input route reached sports-store gameplay in scene $0d, PRG
context [6,7,14,15]. Saved naturalgame.snapshot and played.snapshot. Right+A+B
60 frames and Select+Down 10 frames exercised movement/jump/charm mechanisms;
coverage now 5,050 fetched instruction identities. Supported scene transition
shortcuts $2f=13..17 and $24=1 now executing from scene4.snapshot, with exact
patch/results/captures durably saved incrementally in shortcut-scenarios.json.

Broad progression shortcut batch completed: scenes 13–17 initialized coherent
bakery/platform, galleria, underwater, sports-store and music-stage environments,
then exercised Right+A+B 30 frames. Each scenario restored scene4.snapshot,
released buttons, wrote $2f=scene and $24=1, ran up to 90 frames/4M operations/15s,
then 30 frames/1.5M operations/8s. Timeouts remain partial execution, with actual
frames/state retained. Coverage increased 5050→10861 fetched identities. These
are artificial reachability only. Exact results/banks/patches/RAM:
analysis/disassembly-r1/shortcut-scenarios.json; screenshots scene-13..17.png.


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

## Round 1 commentary completed — current source readable, ROM incomplete

All 966 frozen scaffold regions reviewed; 376 semantic routines cover 837 code
fragments, 8,438 instruction starts and all 18,465 classified code bytes. Data/text,
CHR and unknown boundaries have notes; no naming/comment backlog remains. Full
hand-off: analysis/commentary-r1/handoff.md, coverage.json; source and reading copy
in commentary/, review.json preserves full physical scaffold and bank contexts.
Final byte-identical verification passed; source SHA256
3bbab1d7b02c64a748a3db3c801698ff385609d0513e1767270da8dddf8cfc4f.
Input, maps/current.json and analysis/state.json unchanged against frozen baseline.
Final editorial audit remains pending for coordinator; classification remains
71.05%, 75,907 unknown bytes, not whole-ROM completion.

New structural question for redirect: CHR backing section does not prove a graphics
role for all bytes; loader at $01d44b reads CHR resource directory/payload through
PPU and may execute copied RAM$0200. Review Image role/provenance without assuming
PRG upload origin. World record consumer stride clarified as eleven bytes; current
source explains parser and indexed consumers without inventing unknown extents.
Existing R1-001..003 remain actionable. No new toolkit issues found in commentary;
NES-001 remains suspected and nonblocking. No toolkit edits or external publication.

## Redirect R1 active — resource-contract
Input pass-001-final and commentary-r1 gate. Research directory analysis/redirect/r1-resource-contract; map/state/source untouched. CHR4KiB bank5 directory and bank6 secondary directory found through actual loader semantics. Blanket Image role is under review; exact range/provenance proposals pending. Remaining unknown75907, classified186253 unchanged. Active frontier in attempt plan.

Redirect R1 world extent proof: twelve non-placeholder bank5/6 PRG world containers have independent encoded endpoints matching all adjacent resource headers/suffixes. Gross spans14016..17d58 and18016..19a51; pending disassembly validation, accepted bytes0.11-byte metatile records confirmed with exact domain maxima. Localized one-past page selectors in resource4/8 retained. Secondary CHR RAM copy414 bytes verified. Remaining75907 unknown unchanged; stage/goal dispatcher frontier active in attempt plan.

Redirect R1 independent object-dispatch inspection: all five installed tables and selected handlers followed through bounded bank-aware static traversal;453 new unique instruction candidates/1065bytes with coherent slot-field/control contracts; manual no-inline/no-illegal audit retained. Initial pointer-only extent heuristic counterexamples localized and corrected, no map edits. Scene descriptor table now gives exact18 scene contracts and two missing routine entries. Ending source chain: final stage goal->fade->music-zero gate->scene5 observed; scene5 result progression fast-forward pending. See attempt plan/frontier.

Redirect R1 terminal-chain/source and CHR audit checkpoint: stage5 final goal route reached scene5 result, then supported activity overflow scene6 exterior, then exact source gate scene11 ballroom; A interaction exercised. Explicit artificial stack/RAM recipes, captures and snapshots retained; natural completion/full credits not claimed. All32 CHR banks visually decoded locally; resource bank5, bank6tail and compressed bank7 differ from coherent tile banks. Propose9472 Image->Data correction with independent runtime/copy/container evidence; residual121600 Image has tile-content evidence rather than header-only premise. No map/state/source mutation; final batch records being assembled.

Redirect R1 expanded object research beyond toolkit call/indexed-write stops: v4 contract walker now8335 new candidate bytes; still no accepted/map edits. Always-taken branch data barriers, collision inline schemas, explicit slot0 motion record and scene-bank restoration contracts preserve bank/stack correctness. Bakery response matrix3x5 and six concrete streamed initializers independently constrain low3 selector domain0..4;49 page prefix records all fit consumer group bounds. Current frontier/primary artifacts in attempt plan; final proposal manifest remains provisional pending this connected closure.

Redirect R1 final object continuation checkpoint: v6 closes supported finite handler/response graph with9731 new candidate code bytes,4246 unique starts/110 ranges; zero instruction-boundary or existing/new Data conflicts in contract-audit-v6.json. Extra982-byte stream prefix+records and105-byte response tables are consumer-bounded. All actual mapper/copy/inline contracts explicit; only existing selector-table negative-frame fallthrough remains excluded. Final combined manifest will replace provisional proposal after preserving initial version. Accepted bytes remain0; current75907 unknown unchanged.

## Redirect R1 ready combined handoff
Completed identified connected and independent avenues in analysis/redirect/r1-resource-contract/handoff.md. Final end-exclusive manifest proposes34397 currently unknown bytes (10236 Code,24161 Data) plus9472 Image->Data corrections; accepted0. Current Code18465/Data36606/Text110/Image131072/Unknown75907 unchanged; hashes input/map/state/current source all unchanged. Forecast if all proposals accepted: Code28701/Data70239/Text110/Image121600/Unknown41510. LeadsR1-001..006 ready with stable histories and exact recipes. V6 supported object graph closure audit9731 code candidate bytes, no overlaps, single existing-data invalid/unsupported negative-frame edge independently reviewed. Five final checkpoints and source-backed artificial stage17->5->6->11 route observed; not natural completion/full credits. All servers exited. Toolkit one suspected bug/two documented missing features, no blockers; no edits to toolkit. Separate disassembly next; no final audit/archive/publication run.

## Disassembly R2 active
Input pass-001-final; independent connected-batch review begun. No classification changes yet. Plan/frontier: [disassembly-r2/plan.md](disassembly-r2/plan.md). Reusing broad five-stage and terminal evidence; physical source and bank/inline contracts being checked before acceptance.

R2 independent audit passed34397-byte proposal union and9472-byte CHR role correction. Bounded90-frame music-stage continuation from retained boss17 snapshot exercised right/A/B, release,left/B,release:37 additional unknown fetched starts plus6 supported static branch starts. Existing allocator failure control adds50 static bytes. Runtime aggregation drops0, detail intentionally capacity0. Evidence disassembly-r2/runtime-steps.json, runtime-summary.json, runtime-discovery.json; map acceptance underway, no saved classification mutation yet.

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

## Commentary R2 completed — pass-002-final
Full current-source readability complete, classification incomplete. Entire prior commentary preserved under analysis/round-2/commentary-r1 before current baseline. All1296 scaffold regions reviewed;12980 instruction starts/all28867 Code bytes;634 semantic code units/1156 fragments and151 explicit new/refined data structures. Valid prior8438 instruction starts carried by physical identity;4542 new starts/10402 bytes read in full and explained. All classified non-code and unknown boundaries explained; no naming/comment/review backlog. Ten stale inherited dependency summaries corrected; exact allocator, collision, response, stage-event, world/CHR and bank restoration mechanics named locally. Coverage and phase handoff under analysis/commentary-r2; questions in commentary/uncertainties.md.

Fresh final commentary verification byte_identical=true, source SHA256 ef256dc12fead6e84524ca6bfc197b17741706e71e8fd7508348cffa91ccd323; reading-after refreshed. Input unchanged; map/state both3c2c0330789ea6820e893eac361019445f417b017e8511ca6eda3d07a6d0962b, matching frozen current commentary baseline. Classification unchanged Code28867/Data70239/Text110/Image121600/Unknown41344 (84.23%). Final editorial/archive audit pending for coordinator; no audit run. No new toolkit issues found; persistent suspected NES-001 and missing NES-002/003 retained, no blockers. No map/runtime/toolkit/archive/deletion/publication work performed.

## Redirect R2 active — r2-lookup-contract
Input pass-002-final; current-source readability complete, classification unchanged220816/262160 (41344 unknown). Research-only finite consumer/record and generic non-code review started; exact ledger and plan in analysis/redirect/r2-lookup-contract/. Prior R1 experiments remain immutable. No server, map/state/source/toolkit change.
Redirect R2 research update: alias/frontier primary artifacts and active plan persisted. Exact map complement unchanged. Conditional-root seed corrected after LDA1/BNE counterexample; current971/2187 instruction candidate set remains unaccepted. Finite/parallel lookup and generic non-code opportunities plus primary-spawn, CHR-write, restoration and fixed-bank continuation contracts remain productive; continuing same redirect.
Redirect R2 checkpoint: consumer schema parsers now retain eight sprite-container sets, five complete stream directories and five trigger directories with exact body endpoints/callback entries. Trigger RTS words supply new source-backed roots; connected contract walk currently5471 unaccepted Code candidate bytes. Exact initial state extraction closes all five73-byte bank1 motion lists, anchors and19-state music respawn next-state closure. plan.md records bounds/caveats and remaining primary-spawn contracts; research continues same phase, no map/state/source edits.
Redirect R2 durable results: packed text parser exactly tiles dictionary/corpus/51 offsets; music parser exactly tiles7856-byte active-track resource suffix after excluding track0 overlapping alias. One long audio stream's full natural traversal remains unclaimed. Source-driven trigger+audio callback graph now6315 candidate Code bytes with direct inline spawn contracts; independent audit/proposals remain pending. Current classification unchanged.
Redirect R2 checkpoint: five collision list containers and five drawing rectangle sets parsed against source. Exact legal one-byte terminator/header overlaps retained; natural camera selector bounds distinct. Graph6493 candidate Code bytes. Source review exposed omitted presentation object directoryA432 and two installed drawing handler directories; continuing their connected static contracts before combined proposal.
Redirect R2 presentation and drawing dispatch prefixes followed as combined connected batch;8683 candidate Code bytes. Legal92B4/92B7 overlapping LDY/BIT entry retained for independent review; no silent instruction-overlap acceptance. Scoped scene12 bank0 and bounded decimal-write contracts added; remaining bank/stack barriers being reviewed with explicit limits. Classification unchanged.
Redirect R2 current task checkpoint: overlap representation tested with configured binary on copied probe only (validate/generate/byte-identical true; overlapping second region rejected). Envelope explicit bounds cover44 payload bytes plus5 generic candidate bytes. Fixed noise is a relocated-layout match, not exact-byte duplicate; exact=false preserved, scope localized to nine pointer words. Graph9449 decoded candidate bytes, no indirect callback pointer targets inferred. Final role manifest/audit remains active.

## Redirect R2 ready — r2-lookup-contract
Combined research manifest proposes39360 currently Unknown bytes (9776Code/29584Data),0 accepted; if all reviewed/accepted,1984 unknown remain. Current classification220816/262160 and41344 unknown unchanged. All sixR2-001..006 leads ready with persistent histories; exact current/forecast ledgers, contexts and bounded parsers in analysis/redirect/r2-lookup-contract/. Source-connected sprite/drawing/spawn/trigger/collision/music/text/fixed resources and complete callback/helper graph followed before handoff. Sole intentional92B4/92B7 instruction overlap primary-view probe validates/generates/verifies byte-identical; physical overlapping second region rejected. Input/map/state/source hashes unchanged. Nonterminal ready batch, no exhaustion claim; separate disassembly/current-source commentary required. No new toolkit issues found; existing suspectedNES-001/missingNES-002/003 no blocker. No server, toolkit edit, archive, deletion or publication.

## Disassembly R3 active
Independent R2 connected-batch review started; no accepted changes yet. Plan/frontier analysis/disassembly-r3/plan.md. Full prior commentary and five-stage/artificial terminal progression retained.

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

## Redirect R3 active — r3-inline-residual

Current phase redirect; independent worker input pass-003-final map/state SHA2567da73015f4ea41aff44f2448f6d45e43d85883b6b93b1c134b692e5375239230. Commentary-r3 handoff readability complete,2475/2475/no backlog, fresh byte-identical source SHA2561543a2efd50c7f8c0ec99a64bf52d64d7c09d4394fda01db1ddc6074fa630ca5. Classification260176/262160(99.24%),Unknown1984. Research only. Plan/frontier: [r3-inline-residual/plan](redirect/r3-inline-residual/plan.md). First question58 unique saved inline intervals572 bytes, then connected and other concrete promising residuals. No map/state/source/ROM/toolkit changes or runtime replay.

Redirect R3 checkpoint1: [inline-audit](redirect/r3-inline-residual/inline-audit.json) independently verifies58 unique currently Unknown intervals572 bytes, all exact saved-return or typed collision endpoints and accepted continuations. Proposal Static Data,0 accepted. Continued same redirect through bounded audio/table prefixes and fixed helper/mapper/resource fragment contracts; mixed code/data gaps localized. Research frontier durable in plan.md; no phase stop/exhaustion assertion.

Redirect R3 checkpoint2: [additional claims](redirect/r3-inline-residual/additional-claims.json)105 provisional localized roles1393 bytes (972Code/421Data), plus58inline572,0accepted. Mixed gaps split code/data and isolatedreturn fragments excluded. Currentfrontier exact bounded-index/literalrecord/branch/context audit; rawRAM andcallee labels crosschecked against source before finalizingrationales. Planupdated; no map/state/source mutations. Initial combined forecast19Unknown remains proposalonly.

Redirect R3 checkpoint3:30 bounded-table/mask/literal-pointer assertions pass;121 control edges meet accepted/proposed instruction boundaries. Reassessed three2byte initializer prefixes as complete routines with sharedsuffix contracts (zero scroll,16colorcopy,firstpalette triplet), adding6Codebytes. Final proposed1971Unknown bytes:978Code/993Data,0accepted; forecast13Unknown. Individual structural contracts, not decode/edgechecks alone, support batch. Remaining isolatedreturn/pointer island ambiguity localized; preparing one combined independent disassembly handoff.

## Redirect R3 completed — combined ready batch, no classification edits

Input pass-003-final map/state SHA2567da73015f4ea41aff44f2448f6d45e43d85883b6b93b1c134b692e5375239230 unchanged; sourceSHA1543a2efd50c7f8c0ec99a64bf52d64d7c09d4394fda01db1ddc6074fa630ca5 unchanged andexactsourcecontainedinreadingcopywrapper. Accepted260176/262160(99.24%),Unknown1984. [Handoff](redirect/r3-inline-residual/handoff.md), [manifest](redirect/r3-inline-residual/proposals.json), [identity audit](redirect/r3-inline-residual/audit.json).

ReadyR3-001/R3-002/R3-003 add1971unique proposed bytes:978StaticCode/993StaticData,0accepted.58inlineintervals572bytes independentlyreconciled;53additionalData claims421bytes;55Codeextents427starts978bytes.30boundedtable/mask/pointerassertions and121branch/calledges pass; configuredreleasephysicaldecoder confirms exact427boundaries. Per-claim consumer/stack/RAM/sharedsuffix contracts, notdecoderagreementalone, justifyproposals. Mixedcode/Data islands split; effectivefixedviews preserveunknownlowerbanksnull/null. Naturalreachability anduniversalresource-return assumptions excluded.

Currentexact168range complement refreshedwithsplitdispositions/neighbors/context atredirect/r3-inline-residual/unknown-ledger.json; forecast13bytes/10ranges inproposal-residual-ledger.json (nineisolatedRTS plus4byteambiguousisland), compact localizedreviewresidual-review.json. Priorrecords intact; noreadyconnectedpart withheld. Nonterminalcombinedbatch returns to independentdisassembly/fullcommentary/redirect. No runtime/server/toolkit/source/map/state/ROM changes, archive/deletion/publication. No newtoolkitissues; existingNES-001suspected/NES-002/003documentedmissingfeatures retain no blocker. Workerterminal afterhandoff.

## Disassembly R4 active
Independent semantic/consumer/slot/return/context review of combined R3 batch begun; no acceptance yet. Plan/frontier analysis/disassembly-r4/plan.md. Prior progression and commentary preserved.
R4 review checkpoint: independently inspected actual allocation/collision consumer source, all additional claim operations/consumers and exact adjacent boundaries. Independent configured context decodes55extents/427starts and fresh inline parsing58/572 all pass. Semantic decision review analysis/disassembly-r4/review.md; full1971-byte union supported as Static,13ambiguous bytes excluded. Accept/save/export/checkpoint next.

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

## Redirect R4 active — r4-residual-contract
Research-only independent localized rejection review against current source and exact13-byte complement. [Plan](redirect/r4-residual-contract/plan.md); input pass-004-final map/state23adea33fb84196e636168943c2024d693895008653a6ac8d81d2ae306655c5d; commentary gate complete2483/2483, no backlog. No classification or runtime changes.

## Redirect R4 completed — terminal incomplete proposed for coordinator review
No ready leads;0proposed/0accepted bytes. [Handoff](redirect/r4-residual-contract/handoff.md), [candidate-specific rejection review](redirect/r4-residual-contract/rejection-review.json), [primary audit](redirect/r4-residual-contract/primary-audit.json), [frontier](redirect/r4-residual-contract/frontier.md). Exact262147/262160classified(rounded100.00%),13Unknown/10ranges, unchangedpass-004-final/map-state23adea33fb84196e636168943c2024d693895008653a6ac8d81d2ae306655c5d. Full2483/2483commentary gate/sourceff1a71b716e3bc76465af9e9a6e17e788c5fef3b6ee9a567535bd0bb5a920e05 preserved; byte-identical verification reused on unchanged source.
Independently audited17726currentCode starts and all residual boundaries; compared all6presentation/stage table endpoints plus2drawing tables, confirmed9odd60align-to-even pattern, and tested it against actual reserved hook1EE41Code. Pattern establishes placement but leaves empty stub versus filler ambiguity. Split2FFsuffix independently remains materially instruction-compatible. No natural caller prerequisite or boundary-crossing veto imposed. Prior broad progression reused; no productive continuation withheld. Final editorial audit pending coordinator; primary reasoning must be inspected before terminal acceptance. No new toolkit issue or runtime/server/toolkit/map/state/source/ROM mutation, archive/deletion/publication.

## Coordinator terminal handoff — orchestration complete, classification incomplete

Final redirect rejection review accepted after primary-source/byte/consumer/boundary inspection, autonomous all-table endpoint comparison and verified same-ROM empty-hook counterexample, and independent splitFFFFsuffix assessment. No ready lead/useful improved method remains identified on unchanged evidence. This is justified terminal incomplete, not a blocker or whole-ROM completion. Final report analysis/final-handoff.md; exact thirteenUnknown bytes/tenranges remain.

ROM classification: 100.00% (262147 / 262160 bytes classified; 13 bytes unknown). Roundedpercentage does not establish completion. Checkpoint pass-004-final map/stateSHA23adea33fb84196e636168943c2024d693895008653a6ac8d81d2ae306655c5d remains valid and byte-identical.

Final editorial acceptance reused full review credit and targeted changed dependencies/representations, independently checked allCode review-unit coverage, repaired two stale prose-only decoded-target/helper comments with unchangedlinecount, and updatedreview summaries. FinalsourceSHA089233d039dbff51f216e5cb31556335ed22debcdd4461fab7371c03420367a0; commentary/audit.json freshbyte_identical=true,2483regions/2884units/editorial_audit=true; readingrefreshed. Priorbeforeeditorialsource/review/verification/coverage retained in analysis/final-acceptance/before-editorial. Finalcoverage andvalidation in analysis/final-acceptance. Scenario initialmissingcontractreason supersededwithhistorypreserved; broadencounter/artificialterminalcoverage valid, ordinarycompletionunclaimed.

Archiveoptions explicitlyselect finalanalysis/handoffs/historicalconclusions andomitonlyverifiedduplicateinstructions; finalMarkdowninventorycheckedread-only. Allartifacts local; no archive/cleanup/deletion/publication/toolkitedits. No liveworkers/servers or unfinishedproductivephase. Resumableonnewdiscriminatingevidence using finalfrontier, not repeatedunchangedexperiments. Finalcompletionaudit anddocumentcheck are authoritative finalverification records.
