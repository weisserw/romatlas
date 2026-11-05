# Disassembly progress

Workspace initialized. Baseline and classification have not run.

## Round 001 — disassembly in progress
Baseline maps/baseline.json, generated/baseline, analysis/baseline-verification.json verified byte-identical. Identity unchanged; mapper 1, PRG 131072, CHR 131072, total 262160. Game identified visually as Adventures of Tom Sawyer (SETA 1989 USA). Startup and player/start menus visited naturally. Bounded debugger RPC log and reusable bridge in analysis/round-001; early-game.snapshot.json actually records start menu, not gameplay. Early contexts (1423 starts) and discovery candidates persisted; not yet annotations. Current server is subprocess session 16126 (analysis/round-001/rpc.py). Clients import call from rpc.py; durable file queue retained. Continue with third Start selection, gameplay and progression shortcuts. No classifications yet; no toolkit issues established.

## Round 001 — disassembly complete; handoff to commentary

ROM classification: 78.61% (206092 / 262160 bytes classified; 56068 bytes unknown).

Outcome: incomplete ROM classification. Code 12637, generic/specific Data 62383, CHR Image 131072. All 649 regions carry structural evidence. Code provenance: {"Observed": 8887, "Static": 3750, "Manual": 0}. Current-source readability/editorial audit belongs to commentary and remains pending.

| Physical PRG 8 KiB bank | Code | Data | Unknown |
|---:|---:|---:|---:|
| 0 | 0 | 8192 | 0 |
| 1 | 78 | 0 | 8114 |
| 2 | 0 | 8192 | 0 |
| 3 | 828 | 0 | 7364 |
| 4 | 0 | 8192 | 0 |
| 5 | 50 | 0 | 8142 |
| 6 | 0 | 8192 | 0 |
| 7 | 59 | 0 | 8133 |
| 8 | 0 | 8192 | 0 |
| 9 | 465 | 0 | 7727 |
| 10 | 0 | 8192 | 0 |
| 11 | 93 | 0 | 8099 |
| 12 | 0 | 8192 | 0 |
| 13 | 1614 | 4945 | 1633 |
| 14 | 3603 | 0 | 4589 |
| 15 | 5847 | 78 | 2267 |

Header: 16 Data bytes. CHR: sixteen 8 KiB graphics banks, 8192 Image bytes classified each, zero unknown each. Exact residual complement: [unknown ranges](round-001/unknown-ranges.json).

Verification: baseline, interim pass-001, and final [pass-001-final](pass-001-final-verification.json) all validate and rebuild byte-identically; final map SHA256 `9311c48c78733a3ba4333c5140b905cb26fd0c32def1e1ffb49f7c16bc69094a`. Current internal state [state.json](state.json), portable map [current.json](../maps/current.json), generated source [game.asm](../generated/pass-001-final/game.asm). Input SHA256 rechecked unchanged.

Progression: natural title/player/start menus and classroom opening; artificial debug route visited six distinct stages (platforming, raft, forest, mansion, clouds, caves), exercised movement/jump/attack, saved per-stage fetch contexts and entry snapshots. Ending dispatcher initialization attempted; malformed display prevents claiming visually successful ending/credits. Boss spawning/checkpoint contract remains deferred to redirect. Exact recipes and milestone status: [scenarios](scenarios.json), [gameplay](gameplay.md), [phase handoff](handoffs/round-001-disassembly.md).

Execution metric: 4450 unique accepted ROM opcode starts in union of saved scenarios; natural and artificial observations mixed, separately identified by scenario artifacts. This is not classification percentage. Per-scenario aggregate_dropped=0; detail capacities zero after initial startup, so detail drops are expected and not discarded aggregate roots.

Lead outcomes: R001-DISPATCH verified structural helper/tables; R001-STAGE-ROUTE verified six representative stage routes; R001-DIRECT-COUNTERS superseded by actual debug-menu setup; R001-ENDING attempted, missing CHR/scene prerequisite; R001-SCENE-DATA verified generic lower-half scene data. No inherited leads existed. New actionable questions recorded in leads.json; no terminal exhaustion claim.

Toolkit: no toolkit issues found; zero confirmed/suspected bugs or feature reports, no toolkit blocker. [Persistent report](toolkit-issues.md). Long-lived shell subprocesses were terminated externally during orchestration; bounded self-contained RPC scripts restored snapshots and persisted complete successful scenarios. No toolkit changes or cleanup performed.

## Round 001 commentary completed

Full current-source readability is complete across all 804 physical partitions, including the entire classified-code backlog. Added 201 functional explanations/units, consistent bank-aware routine/branch labels, RAM/object/hardware symbols and sound-record field constants. Data and CHR formats or generic-role uncertainties are explained locally; all 155 unknown gaps retain honest boundary/ambiguity notes. No readability ranges remain deferred. Final editorial acceptance stays pending.

Classification is unchanged: Code12637 + Data62383 + Image131072 =206092/262160 (78.61%), Unknown56068. Source maintenance does not alter classification or execution metrics. Current portable map SHA256 remains `9311c48c78733a3ba4333c5140b905cb26fd0c32def1e1ffb49f7c16bc69094a`.

Fresh [commentary verification](round-001-commentary/verification.json) is byte-identical; [reading-after.asm](../commentary/reading-after.asm) refreshed. [Scope assessment](round-001-commentary/scope-assessment.json), [review](../commentary/review.json), [compact handoff](handoffs/round-001-commentary.md). Existing four redirect leads remain actionable; this is not terminal exhaustion or whole-ROM completion. No toolkit issues found, no blockers; no toolkit/source binary changes, archive execution, cleanup or publishing.

## Round 001 redirect — active connected research checkpoint

Attempt: [round-001-contracts](redirect/round-001-contracts/plan.md). Input checkpoint pass-001-final / map SHA256 9311c48c78733a3ba4333c5140b905cb26fd0c32def1e1ffb49f7c16bc69094a. No classification/source/map edits; classified206092, unknown56068 unchanged.

Closed ordinary-return analysis initially proved125 contextual contracts and1270 unknown code bytes. Connected scene dispatch review now identifies six independently bounded scene resource/type/state blocks, combines header origins, object dispatcher semantics, stream consumers and coherent object-handler contracts. [Scene resource evidence](redirect/round-001-contracts/scene-resource-review.json), [reproducible call analysis](redirect/round-001-contracts/contracts.py), [current proposals](redirect/round-001-contracts/call-contract-proposals.json): 24162 prospective unique code bytes, zero overlap conflicts; all proposed instructions official6502 mnemonics. These are proposals for independent disassembly review, not accepted classifications. Data-barrier proposals total13443 bytes, pending detailed validation.

Ending prerequisite resolved experimentally: prior edit entered substate3 and skipped fixed-bank preparation. Setting mode$22/substate0 executes delay/fade and queues mapper resource$27 before ending initialization. [Ending-prefix snapshot/frame/results](redirect/round-001-contracts/ending-prefix-results.json), image ending-prefix.png: correctly composed sleeping Tom classroom vignette, ending substate7 after200 frames. Need follow remaining ending handlers/timers and representative terminal state before final milestone assessment. Boss/checkpoint and mapper/palette extent research remain active. Exact frontier: validate scene pointer/table proposals using configured decoder, follow all19 ending handlers and scene-event handlers, derive late-area object stream/spawn contract, assess remaining actionable call hazards. This is an active redirect checkpoint, not exhaustion or a disassembly handoff.

## Round 001 redirect completed — ready combined batch

All four inherited avenues investigated as a connected batch; [handoff](handoffs/round-001-redirect.md). No accepted classification changes:206092 classified/56068 unknown, checkpoint pass-001-final and map SHA2569311c48c78733a3ba4333c5140b905cb26fd0c32def1e1ffb49f7c16bc69094a unchanged. [Final proposals](redirect/round-001-contracts/call-contract-proposals.json):28125 unique Code bytes under849 contextual contracts;17304 unique Data bytes in data-proposals.json; zero overlap. Configured decoder validates all12255 unique proposed instructions in136 groups.45429 total proposed bytes are not accepted yet; projected residual10639 only if all accepted. Exact current155 unknown intervals and split proposal dispositions in unknown-ledger.json.

Six scene resource/type/state graphs and every bounded handler were followed through supported helpers. Independent header origins and consumer semantics bound pointer/stream structures; fixed mapper42/palette50 record corpora close with zero holes/overruns. Narrow queue/sound-RAM/CHR/mirroring contracts expand call continuations without assuming PRG writes preserve lower banks. Three16-entry motion dispatchers' selected handlers received ordinary-return proofs. Boundary audit corrected vertical event table toE59D and limited sound command table to7 entries; original broader scratch hypotheses preserved and never annotated.

Ending: substate0 fixed preparation queues mapper39 and fixes the earlier missing graphics prerequisite; valid vignette, STAGE CLEAR TIME results and A-return-to-title observed artificially. Boss: stage6 palette/mapper/type14 events use ordinary spawn initialization, health10 and active AI; camera-command freeze prevents the exact event-window miss. Supported coarse-position adjustment brings the initialized flying boss onscreen duringA+B20; background redraw after camera jump remains unresolved. All prior failures/history and exact bounded scripts/results/contexts preserved; scenarios/gameplay updated.

Return to independent disassembly review, then full current-source commentary. Remaining precise frontiers include conditional copy destinations, dynamic PRG output contexts, nonordinary stack/RTS dispatch and auxiliary/sound-effect pointer contracts; not terminal exhaustion and no unchanged replay requested. No toolkit issues found. No map/source/state/ROM edits, toolkit changes, deletion/cleanup, archive execution or publishing.

## Round 002 disassembly — independent review active

Input pass-001-final and existing commentary preserved. Original28125-Code proposal reproduced exactly; independent primary review checks scene origin/consumer boundaries, mapper/palette count records with shared aliases, sound/motion inline barriers and generic stream patterns. Narrow E349 RAM-destination contract exposes31 additional connected Code bytes. Configured decoder revalidation and supported API annotation are underway; no new runtime replay, toolkit edit or deletion. Review/frontier: [contract-review](round-002/contract-review.md). State has not yet been committed at this checkpoint.


## Round 002 disassembly — completed, incomplete ROM

ROM classification: 95.95% (251552 / 262160 bytes classified; 10608 bytes unknown).

Accepted28156 new Code and17304 new Data bytes, including31 Code bytes beyond redirect's batch via narrow RAM-copy continuation. Totals Code40793, Data79687, Image131072. All new bytes Static; code provenance {'Observed': 8887, 'Static': 31906}. No new runtime execution metric is claimed. Prior six-stage/ending/boss observations reused.

| PRG 8 KiB bank | Code | Data | Classified | Unknown |
|---:|---:|---:|---:|---:|
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 5003 | 2590 | 7593 | 599 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 4699 | 2450 | 7149 | 1043 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 4831 | 2798 | 7629 | 563 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 4076 | 1882 | 5958 | 2234 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 4619 | 2091 | 6710 | 1482 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 3409 | 1908 | 5317 | 2875 |
| 12 | 0 | 8192 | 8192 | 0 |
| 13 | 2294 | 4945 | 7239 | 953 |
| 14 | 5027 | 2548 | 7575 | 617 |
| 15 | 6835 | 1115 | 7950 | 242 |

Header16 Data; sixteen CHR banks8192 Image each, zero unknown. [Exact counts](round-002/counts.json), [unknown complement](round-002/unknown-ranges.json). All837 regions carry evidence; raw fallback/Unknown does not count.

[Independent contract/data review](round-002/contract-review.md), [reproduced/decoder checks](round-002/review.json) and [map validation](round-002/validation.json). Fresh [pass-002-final verification](pass-002-final-verification.json) byte-identical. Portable map SHA2562789b29f7594c8358c26ea716742b8445dfaaf1c4e971e3c1012f5f1bdd4403f; ROM identity unchanged. [New source](../generated/pass-002-final/game.asm) is a fresh baseline for commentary; prior fully reviewed commentary/source is preserved. Full current-source readability and final editorial acceptance remain pending for this new map.

All five ready inherited lead batches verified structurally; outcomes appended to leads.json. Remaining actual barriers are dynamic PRG-list output contexts, nonordinary stack/RTS dispatcher summaries, auxiliary indirect-handler extent and additional sound indexed-pointer domains. Copy-store frontier narrowed on supported callers, not globally declared safe. These require redirect research; no terminal exhaustion claim. [Handoff](handoffs/round-002-disassembly.md). Toolkit: no toolkit issues found; [persistent report](toolkit-issues.md), zero blockers. No toolkit edits, archive execution, deletion, commercial downloads or publishing.

## Round002 redirect active — round-002-local-contracts

Read own redirect prompt, evidence-policy, workflow, current commentary handoff and decisive round001 proof/source artifacts. Input pass-002-final map/state2789b29f7594c8358c26ea716742b8445dfaaf1c4e971e3c1012f5f1bdd4403f;251552 classified/10608 unknown65 ranges. Attempt plan and full residual hex/decode are saved in analysis/redirect/round-002-local-contracts. Active method follows bounded auxiliary and inline pointer tables into handlers, revises narrow nonlocal-return and PRG-resource contracts, and inspects practical role before generic data. Explicit cloned reset stubs excluded from template/resource tails. No map/source/toolkit changes; no prior gameplay replay; no terminal assessment yet. Disk9.1GiB available, owned scratch tiny, no cleanup.

## Round002 redirect ready handoff — round-002-local-contracts

Combined connected and independent research now proposes all10608 residual bytes:4182 Code,6426 Data, no overlap, zero newly accepted. Configured run.py serve decoder validated2319 contextual instructions across549 groups with zero discrepancies; physical code coverage4182. Primary manual contracts, pointer/template records, accepted-reference search, complete65-range ledger, graph frontier, clone identities and local negative/corrected claims saved under analysis/redirect/round-002-local-contracts. Final byteFA2E RTS is supported by known FA2B call plus independently ordinary-returning A554 selector-zero handler, not by isolated opcode plausibility. Input/state/map hashes unchanged. Added ready lead R002-LOCAL-CONTRACTS; no actionable unknown range remains outside combined proposal. Independent disassembly review/classification and current-source commentary remain required; no terminal success/exhaustion asserted. No new runtime gameplay evidence. No toolkit issues found; no blocker. No edits outside research/lead records, cleanup, publishing or archive execution.


## Round 003 independent disassembly — incomplete

ROM classification: 99.98% (262103 / 262160 bytes classified; 57 bytes unknown).

Accepted4481 new Code and6070 new Data, all Static. Totals Code45274, Data85757, Image131072; header16 Data, every CHR bank8192 Image. Original graph/preparation/references/complement reproduced in redirect/round-003-reproduction;2319 proposed contextual instructions had zero configured-decoder discrepancies, and all final accepted groups including the added299 bytes were decoded again in round-003/rpc.jsonl. New state saved, map exported and validated; fresh pass-003-final verified byte-identical. Map SHA256d04812ebf8f9948b33d274383decf2b38a6e0b28f79c2ce46f77dde2be7cbff5; input ROM unchanged.

| PRG 8 KiB bank | Code | Data | Classified | Unknown |
|---:|---:|---:|---:|---:|
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 5038 | 3154 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 5238 | 2954 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 4866 | 3326 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 4751 | 3441 | 8192 | 0 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 5337 | 2798 | 8135 | 57 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 4380 | 3812 | 8192 | 0 |
| 12 | 0 | 8192 | 8192 | 0 |
| 13 | 3008 | 5184 | 8192 | 0 |
| 14 | 5601 | 2591 | 8192 | 0 |
| 15 | 7055 | 1137 | 8192 | 0 |

Independent complete data review discovered bank9 BCE7..BE4B embedded code-looking bytes inside proposed generic data. Split356 bytes:299 complete routine bytes BD20..BE4B accepted Code;57-byte prefix BCE7..BD20 remains Unknown because its outgoing destinations lie in present template bytes. Generic neighbors remain independently supported. This localized rejection avoids projected completion; see [review](round-003/contract-review.md), [fresh data hex](round-003/data-inspection.txt), [exact counts](round-003/counts.json), [residual](round-003/unknown-ranges.json), [validation](round-003/validation.json), [verification](pass-003-final-verification.json).

R002-LOCAL-CONTRACTS partially verified; R002-CONTRACT-FRONTIER superseded for classification by narrower residual question. New R003-BANK9-PARTIAL-SUFFIX ready for redirect. Investigate current structural role of partially overwritten/misaligned suffix versus resource payload using accepted neighboring object families and relocation boundaries; do not use forced execution as intended-role proof. Not terminal exhaustion. Current-source commentary and final editorial/fidelity audit remain pending; prior commentary history untouched.

Historical correction: previous redirect's 'four-boss' phrase is unsupported; actual retained evidence supports a representative flying boss, six stage mechanics and ending scenario, reused without new replay. A1DD recognizer arm is LDA #0, not RTS; inherited wording corrected in new review, old attempt preserved. Bank9 lower-slot template targets require bank8 physical context, not bank9; generic annotation does not inherit incorrect template sizes.

No toolkit issues found; [persistent report](toolkit-issues.md), zero blockers. No toolkit changes, archive execution, cleanup/deletion or publishing. Disk9.1GiB available; scratch4KiB. [Handoff](handoffs/round-003-disassembly.md).

## Round003 commentary active

Prior commentary preserved fully under round-003/commentary-prior-002 before fresh preparation. New baseline pass-003-final; exact physical carry verified byte-identically. Full new-code reading covers2075 instruction starts across all odd scene banks and fixed banks; manual mechanical routine summaries persisted in round-003-commentary/new-units.txt. Integration, changed boundary review, full inventory reconciliation and final fresh verification/reading refresh remain in this same phase. Unknown57-byte bank9 suffix remains unchanged and queued for redirect; no terminal/editorial claim.

## Round003 commentary complete — classification still incomplete

Full current-source readability complete for944 partitions (791 Code/151 Data/1 Image/1 Unknown). All45274 Code bytes accounted by20513 starts;525 functional code groups (411 reused/114 added) and106 named data units,631 review units total. Exact837 prior physical partitions carried;107 current new/changed. All40793 prior code bytes/18438 starts retain encoding dependencies; entire prior commentary preserved under round-003/commentary-prior-002. Prior data substructures/prose restored and changed unknown/dispatch/camera/nonlocal-return boundaries corrected. Source labels explain four bank9 nonlocal exits and reset copies' upper-slot aliases/actual bank switch; code/data uncertainty stays localized to57 bytes.

Fresh commentary verification passes byte-identically; reading copy refreshed. Map/state d04812ebf8f9948b33d274383decf2b38a6e0b28f79c2ce46f77dde2be7cbff5 and input hash unchanged. See [scope](round-003-commentary/scope-assessment.json), [carry](round-003-commentary/carry-report.json), [verification](round-003-commentary/verification.json), [handoff](handoffs/round-003-commentary.md). No current-source readability backlog. Final editorial audit reserved, not invoked; no whole-ROM completion claim. Next redirect: R003-BANK9-PARTIAL-SUFFIX, physical13CF7..13D30,CPU BCE7..BD20 in context[8,9,14,15], primary residual/contract evidence. No new gameplay/reachability claim. No toolkit issues found; no blocker or toolkit change, archive execution, cleanup/deletion or publication.

## Round003 redirect complete —57-byte relocated-fragment proposal

Read current-source commentary gate and decisive residual evidence. Semantic relocation comparison establishes entire356-byte BCE7..BE4B family as156 instructions matching accepted BA5B..BBBF under+$028C local-code relocation; current final template independently ends BCE7. Propose only residual57 bytes Static Code as a preserved program fragment. Present external joins remain template Data; no complete working routine, caller, natural selection or historical source-generation claim. This corrects rejection scope from broken continuation to fragment structure. See [findings](redirect/round-003-relocated-remnant/findings.md), [primary comparison](redirect/round-003-relocated-remnant/relocation.json), [proposal](redirect/round-003-relocated-remnant/proposal.json), [exact ledger](redirect/round-003-relocated-remnant/unknown-ledger.json). Current map/source/input unchanged; classified262103,Unknown57, proposed57, accepted0. All remaining bytes are in this one ready proposal; neighboring299 bytes already classified. No unchanged gameplay replay, toolkit change, archive execution, deletion or publication. Independent disassembly then full commentary/fidelity/editorial gates required.

## Round004 independent disassembly — structural classification complete; final gates pending

ROM classification: 100.00% (262160 / 262160 bytes classified; 0 bytes unknown).

Accepted57 Static Code bytes, physical81143..81200, CPU BCE7..BD20 context[8,9,14,15]. Independent full356-byte/156-instruction comparison, semantic family identity, bounded preceding template and independently accepted299-byte continuation support a preserved instruction fragment. No caller, runnable external joins, natural reachability, actual fetch or overwrite chronology inferred. BCE6/BCCF/BCAD remain Data; no discovery through them. All prior classifications preserved. Totals Code45331, Data85757, Image131072; header16 Data, each CHR bank8192 Image, all unknown zero.

| PRG bank | Code | Data | Classified | Unknown |
|---:|---:|---:|---:|---:|
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 5038 | 3154 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 5238 | 2954 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 4866 | 3326 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 4751 | 3441 | 8192 | 0 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 5394 | 2798 | 8192 | 0 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 4380 | 3812 | 8192 | 0 |
| 12 | 0 | 8192 | 8192 | 0 |
| 13 | 3008 | 5184 | 8192 | 0 |
| 14 | 5601 | 2591 | 8192 | 0 |
| 15 | 7055 | 1137 | 8192 | 0 |

[Independent review](round-004/contract-review.md), [primary reproduction/decoder checks](round-004/review.json), [RPC](round-004/rpc.jsonl), [exact counts](round-004/counts.json), [validation](round-004/validation.json). Every classified region carries evidence. Execution coverage unchanged; reviewed and reused existing six-stage, ending and representative boss scenario records without replay.

Fresh [pass-004-final verification](pass-004-final-verification.json) byte-identical; map/state SHA256 95df5dcd3c8697d06ef1d6a85f9b948bc5b3d833c79c372440fe573e02989516. ROM hash unchanged. Current state saved and portable map exported. New [source](../generated/pass-004-final/game.asm) needs full fresh commentary integration; previous commentary/source and investigation history preserved. Final editorial/fidelity review remains pending: zero Unknown establishes structural classification only, not overall completion or full hardware/game compatibility.

Lead R003-BANK9-PARTIAL-SUFFIX verified for fragment structure only. No new structural residuals or gameplay experiments. No toolkit issues found; [persistent report](toolkit-issues.md), zero blockers. No toolkit edits/archive execution/cleanup/deletion/publishing. Disk9.1GiB free; round004 owned scratch4KiB. [Handoff](handoffs/round-004-disassembly.md).

## Round004 commentary complete

Full current-source readability completed from pass-004-final; whole prior commentary retained at round-004/commentary-prior-003. Exact943 unchanged physical/evidence partitions,20513 starts and631 review units carried; all23 new fragment starts explained. All946 scaffold partitions/634 units reviewed,45331 Code bytes accounted;85757 Data and131072 Image bytes retain local explanations. Current fragment comments separate X-bound/motion stream state from animation state and preserve broken outgoing Data joins and unknown entry/reachability. No pending readability range. Fresh byte-identical verification and refreshed reading copy; final editorial audit deliberately pending until redirect confirmation. [Scope](round-004-commentary/scope-assessment.json), [verification](round-004-commentary/verification.json), [handoff](handoffs/round-004-commentary.md). No toolkit issues found; no blocker, toolkit edits, archive execution, deletion/cleanup or publication.

## Round004 redirect complete — final acceptance pending

Direct internal-map union confirms262160/262160 classified,Unknown0, exact complement[] and overlap[],946 regions; state95df5dcd3c8697d06ef1d6a85f9b948bc5b3d833c79c372440fe573e02989516 unchanged. Reviewed persistent lead statuses, current source uncertainty search and targeted fragment/template, generic data, copied suffix and sound-helper notes. No structural contradiction or promising unresolved classification avenue identified. No ready/additional leads, proposed0/accepted0; stale ending/boss remaining-question wording reconciled with preserved history. Natural callers, individual data semantics and rendering/hardware limits remain separate. Broken fragment joins remain Data; only representative flying-boss evidence supported. [Handoff](handoffs/round-004-redirect.md) passes to separate final editorial/fidelity acceptance, not overall completion. No final audit, unchanged experiment replay, map/source/toolkit changes, archive execution, deletion/cleanup or publication. No toolkit issues found; no blocker. Disk9.0GiB free.

## Final acceptance — accepted

Separate final acceptance worker completed targeted editorial checks after round004 redirect, retaining valid full-source review credit. ROM classification **100.0000%: 262160/262160 bytes classified, Unknown0**. Direct internal-state union again gives exact complement[] and overlap[]; all946 regions have evidence, Code45331/Data85757/Image131072. State/map checkpoint pass-004-final SHA256 remains95df5dcd3c8697d06ef1d6a85f9b948bc5b3d833c79c372440fe573e02989516; ROM SHA256 unchanged.

Readability/editorial acceptance complete independently:946 partitions,634 functional units/20536 instruction starts, no pending source ranges. Targeted fixes replace misleading shared-scratch sound aliases, name MMC1 CHR1/PRG helpers, qualify static reset-copy wording, and correct two inspected duplicate boundary notes. Line counts preserved. Inherited review paths normalized to the source root; existing unit credit attached as clipped Code-partition inventory records. First supported audit rejected the stale path format, corrected without relaxing requirements; second audit passed all946 regions/1056 inventory intersections (these are not1056 independent functional reviews).

Fresh supported [commentary audit](../commentary/audit.json) and [verification](../commentary/verification.json) pass byte-identically, reading copy refreshed; configured-binary [map](final-map-validation.json) and [store](final-store-validation.json) validation pass. [Editorial evidence/limits](../commentary/editorial-audit.md), [exact complement](final-complement.json), [acceptance](final-acceptance.json), [handoff](handoffs/final-acceptance.md). Source SHA256504acd47a75c35f3d3c4e845f387090a038bc9c716644bbe468f65535bbf4ca2.

Required structural/readability/editorial/byte-fidelity gates accepted. Natural callers, original data organization/visual identities, exact score units and hardware/rendering behavior remain limited; no full hardware/game compatibility claim. Existing progression supports six stage mechanics, artificial ending/results/title return and one representative flying boss; boss-room redraw remains uncertain. No new research/execution, toolkit modification, archive execution/dry-run, deletion/cleanup or publication. No toolkit issues found; no blocker. Archive-options selection is a separate coordinator preparation step, not an executed archive.


## Coordinator terminal handoff — accepted whole-ROM workflow

ROM classification: 100.00% (262160 / 262160 bytes classified; 0 bytes unknown).

Four sequential disassembly/commentary/redirect rounds and separate targeted final acceptance are complete. Final checkpoint pass-004-final; map/state95df5dcd3c8697d06ef1d6a85f9b948bc5b3d833c79c372440fe573e02989516. Final source editorial corrections and supported audit passed; all946 partitions/634 functional units reviewed, no readability backlog. Direct exact complement/overlap empty; ROM unchanged; fresh byte-identical build and reading copy recorded in commentary/audit.json. See [terminal handoff](handoffs/terminal-handoff.md), [acceptance](final-acceptance.json), and [coordinator completion audit](coordinator-completion-audit.json).

Inspected toolkit scripts/archive.py document-selection contract. archive-options.json selects final conclusions/handoffs/research history; duplicate instruction scaffolds and verified byte-identical working-copy Markdown explicitly omitted. analysis/archive-document-selection.json accounts for every analysis/commentary Markdown path. No final result omitted to make selection pass. This prepares documents only: no archive or dry-run, cleanup/deletion, publishing or toolkit edit. All assets/internal state remain local. Final hardware/gameplay and natural-reachability limits remain explicit in commentary/uncertainties.md. No toolkit issues found.
