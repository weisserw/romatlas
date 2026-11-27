# Disassembly progress

Workspace initialized. Baseline and classification have not run.

## Round 1 disassembly active
Baseline established and byte-identical: analysis/baseline-verification.json.
Configured release binary; mapper 1, 131072 PRG + 131072 CHR + 16 header bytes.
Live file-command bridge analysis/round01/bridge.py (exec session 67692), server through run.py serve.
Startup trace uses enabled=true, capacity=0 (bounded aggregates), 180-frame request, 10M CPU-tick budget, 10s timeout. Awaiting bounded response before inspecting frame.
No prior leads. Classification not yet saved. Input unchanged.

### Startup/title batch
Observed Cobra Command US title, copyright 1988 Data East USA. `analysis/round01/title180.ppm` shows intro at frame 213; Start 2 frames then release 60 reaches title frame275 (`started.ppm`, persisted snapshot).
Natural trace: 1275 unique instruction identities,1513 contextual roots,1619 contextual edges; aggregate_dropped=0. Detail capacity0 intentionally discarded23,554,539 events. Exact root export in natural-contexts.json; opcode-length inspection collecting in title-observed-instructions.json.
Accepted map annotations: 16-byte validated iNES header Data and 131072-byte CHR Image. Static discovery from reset/NMI/IRQ yields21 instructions before conservative mapper/call barriers (vectors-static.json). No speculative candidates accepted. Second Start2/release90 gameplay experiment now pending.

### Natural stage1 batch
Second Start reaches equipment/mission screen (`gameplay.ppm`, frame367, $4e=$0a). Holding right+A+B180 frames reaches actual stage1 jungle flight (`stage1-play.ppm`, frame547,$4e=0,$27=1,$28=1). This includes flight/shooting, not merely stage display. Header/CHR plus startup/title/equipment fetched code saved:206 regions,136183 classified bytes before latest play fetch merge. Start toggle experiment now ongoing. Strong stage/scene selector candidate $4e and orientation/scene mode $27 identified; stage-init route CPU$c04c clears state and invokes setup before main loop. Shortcut will be tested from clean snapshot with exact patches and artificial provenance.

### Resumability / stage shortcut contract
Bridge session67692 exited143 during Start toggle; saved structural map includes latest natural flight fetch annotations. Restarted bridge session6572 with map.load and emulator.start; imported gameplay-snapshot.json (equipment) and reasserted right+A+B for180 frames. Replay now saves durable stage1-play-snapshot.json. Source inspection equipment-decode.json establishes safer shortcut than PC edit: equipment exit reads saved scene RAM$cb into$4e at CPU$c1e7, then calls scene bank/CHR loader$eb7f and complete state initialization through$c07a. Test patch$cb while equipment menu is active, then exit with normal button; preserve artificial provenance. $4e0..9 are gameplay areas; $0a equipment,$0c title/intro. Investigating button toggle now; no register edit performed.
Independent bounded static discovery launched from saved validated fetched roots via run_static.py; map.load only, no mutations in that server.

### Data / static review batch
`round01/data-batch.json`:57 Static regions/57646 bytes accepted through map.annotate and validated. Three16KiB data-dominant banks excluding repeated16-byte guard/vector trailers: repeated rectangular/tile-index content, bounded record layouts, no conflicting fetches, semantic schemas deferred. Seven repeated reset guards are structurally justified by encoded$bff0/$bff7 vectors;8-byte SEI/INC$ffff/JMP$c000/RTI,2-byte generic gap,6-byte vectors each. Four precise data barriers prevent discovered fallthrough into constants after guaranteed conditional branches: physical$18573+4,$1da2d+28,$1fd3e+6,$1ec83+48. All remaining unclassified PRG$ff runs>=64 bytes annotated generic non-code after checking exact boundaries; no fill-purpose claim. Latest map saved. Static scan repeated with those barriers; original scan retained as natural-static-candidates (will supersede current filename with revised run; historical issue details retained here).
Natural Start reverse and Select equipment confirmed; `equipment-menu.ppm` at scene$0a. Clean durable equipment-menu-snapshot.json. Artificial saved scene$cb=2 experiment underway via normal Select exit; no PC edit used.

### Artificial scene2 validated
Select alone failed to exit menu, preserving area2-* records. Reproducing bank5 supported exit RAM contract succeeded: `$cb/$88/$4e=2`; restore RAM$52<-$83,$51<-$84,$53<-$85,$54<-$86,$55<-$87; clear$05a2/$05a3/$05a4/$8a/$8b; right+A+B80frames,3M tick/45s cap. Fixed-bank menu exit naturally executes scene setup; scene2.ppm shows distinct desert/mountain/cloud terrain with actual helicopter flight/shooting. Runtime$4e2,$27=1, effective banks[2,3,14,15]; context/snapshot/observed fetches saved. Artificial, no natural later-level reachability claim.
Revised static natural paths accepted258 regions/4276 new code bytes; all unofficial/BRK false fallthroughs excluded through bounded data barriers. Scenes4,6,8 use same clean equipment snapshot/exit contract and bounded80frame interactions next. Standalone per-experiment servers now avoid long live-bridge lifetime exits143; scripts retain reproducible RPC calls.

### Broad progression and connected static continuation
Artificial scenes4/6/8 show distinct dark jungle,sea islands,night archipelago; all80frame flight/fire interactions validated by frame and fetch evidence. Scene9 with saved mode$d2=0 shows final underground fortress/cave,$4e9,$27=0,$28=1. Each starts clean equipment-menu snapshot; artifacts sceneN-* preserve states and contexts. Original five-stage external guide agrees with even/odd surface/underground source selector structure, but scene visits are described by verified local indices rather than inferred stage titles.
Direct-call code added2508bytes/69regions, data barriers+23 and+72 bytes (correction: physical$1e078..1e08e is23bytes, not21). Bounded local contracts.py proves800 bank/stack-preserving routine contexts,314 conservative failures,12348 decodes; accepted candidate instructions9795 contextual entries. Conservative rejection on mapper writes, indirectcontrol,unbalanced stack and data barriers; it never assumes mapper-changing callees preserve bank state. Acceptance review pending.
Ending self-initializing fixed$fd34 path tested artificially from clean flight after instruction boundary (PC$fd34,stack$fd,P$24),120frame/4M tick/60s cap; no natural completion claim. Boss-condition test in clean scene9 patches$0505=1,$02c9=$80 per inspected fixed$e07f gate then down+A+B60; records actual result separately from branch hypothesis. Both pending.

### Ending and contract review completed
Ending artificial fixed$fd34 entry reached full message (`ending-late.ppm`): hostages saved, enemy defeated, concluding liberty/peace statement. Initial traced120-frame request timed out after88frames (bounded, state preserved). Trace-disabled900 frames advanced text without creating execution evidence;30 traced frames collected at completed message. Followed inspected finite delay into credits: six bounded300-frame captures in progress; credits0.ppm shows victory soldier/flag and STAFF, credits4.ppm shows GRAPHICS names. Thus ending and credits visited artificially; natural completion still unproven. Boss-condition60frames did not demonstrate a boss; visual fortress remains, helicopter can be obscured/destroyed; no boss reachability claim, no unchanged retry.
Bank-preserving contract batch accepted89regions/2344bytes after validation;0overlap conflicts. Stack/mapper/indirect-control failures remain conservative research leads, not evidence of data. Boss-condition fetched extra6bytes accepted. Total state before current credits capture merge209948classified/52212unknown (progress measure only, final exact line follows checkpoint).
Largest residual review (`large-data-review.json`) finds five clear non-code blocks: physical$18b10+8014 sound streams (6references),$1121c+7279 pointer/composite sprite records (prefix code excluded; no direct reference currently known),$14a86+4145 menu text/lookup/packed graphics (8refs),$e94d+2544 actor sprite records (24refs),$f818+2024 animation/coordinate records (9refs). All checked against every saved observed-instruction record: zero conflicts. Samples every256bytes, exact neighboring code/trailer bounds and reference consumers preserved locally. Generic data annotations pending credits server save to avoid map races.


## Round 1 disassembly handoff — incomplete

ROM classification: 89.32% (234161 / 262160 bytes classified; 27999 bytes unknown).

Single authorized disassembly pass completed. Code 21382 bytes; non-code 212779 bytes; 1011 evidence-annotated regions (1011/1011 nonempty evidence). Exact physical unknown complement and all bank totals: `round01/final-metrics.json`. Header16/16 classified. CHR banks0–15 each8192/8192 classified Image,0 unknown.

| PRG bank | Code | Data | Classified | Unknown |
|---:|---:|---:|---:|---:|
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 8 | 8184 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 8 | 8184 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 8 | 8184 | 8192 | 0 |
| 6 | 5484 | 0 | 5484 | 2708 |
| 7 | 1780 | 4576 | 6356 | 1836 |
| 8 | 379 | 3572 | 3951 | 4241 |
| 9 | 462 | 6437 | 6899 | 1293 |
| 10 | 1064 | 4470 | 5534 | 2658 |
| 11 | 42 | 395 | 437 | 7755 |
| 12 | 1864 | 5681 | 7545 | 647 |
| 13 | 8 | 6774 | 6782 | 1410 |
| 14 | 6107 | 28 | 6135 | 2057 |
| 15 | 4168 | 630 | 4798 | 3394 |

Fresh checkpoint **pass-001**: map.validate passed; `maps/current.json` and `maps/pass-001.json`; generated `generated/pass-001`; `analysis/pass-001-verification.json` confirms ok=true,byte_identical=true. Map SHA256 `30a0e9f8324b21c1d9582312c4fde1da220b1d0f3600ed103cc5bb694765d69a`. Input ROM SHA256 unchanged `4a8b649b8ef253e178875f1a3f3614e16ac46484d8d49bb3ef7ff8f41633c818`. No toolkit edits/publishing/archive/cleanup.

Execution is separate:5928 unique physical fetch starts;11575 naturally observed opcode bytes;9975 artificial-scenario observed opcode bytes;12675 union (overlap counted once). Trace aggregate drops0 in saved scenario summaries; detail capacity0 intentionally discarded bulk events. Disabled-trace ending progression supplies frame/state evidence, no fabricated fetch coverage.

Natural: intro,title,equipment,stage1 flight/missiles/guns,Start reversal,Select menu. Artificial: later surface scenes2/4/6/8,final underground9,complete ending message and victory/STAFF/GRAPHICS credits. Boss-condition patch attempted without confirmed boss; rescue-specific/spatial contracts remain for redirect. All final outcomes and exact replay/patch budgets consolidated in `analysis/scenarios.json`; source hypotheses in `analysis/gameplay.md`.

Consumed leads:none (initial queue empty). Produced research leads R1-L001..R1-L004 in `analysis/leads.json`. Broad later-surface tests yielded14,0,17,0 new fetched code bytes; cave9 added7 and boss-condition6. Direct static paths, direct-call target closure and bounded stack/bank-preserving continuations were classified; remaining promising continuations need indirect-selector/return-stack/bank-restoration or mixed-block boundary research. No raw unknown gap was treated as classified. This is the easy-gain phase handoff to commentary/redirect, not terminal workflow exhaustion.

Five large non-code blocks (24006 bytes) accepted after precise neighboring boundaries, references/pattern inspection and checking every saved fetched instruction for conflicts; `round01/large-data-review.json`. These generic structural roles do not claim complete original schemas. Classification complete only for accepted ranges. Readability/editorial audit remains for commentary across all current classified source.

**no toolkit issues found**; persistent report: [toolkit-issues.md](toolkit-issues.md). Latest verification valid; no live server required. Resume with configured binary via run.py serve,map.load analysis/state.json,emulator.start,import chosen persisted snapshot,then explicitly reassert buttons and configure/clear traces. All standalone round01 scripts are local reproducible evidence, not portable sharing artifacts. Scratch12KiB; analysis/round01~151MiB;3.6GiB free at handoff.

### Final toolkit report correction
Concrete repeated static-discovery table fallthroughs documented as missing feature NES-MF-001 (constant Z/C propagation), not an advertised-behavior bug. One missing feature,zero confirmed/suspected bugs,no blockers; [toolkit-issues.md](toolkit-issues.md). This supersedes preliminary no-issues wording. Workaround validated Data barriers; no toolkit edits. Final immutable map/checkpoint/input identity and count audit passed; classification line and checkpoint remain unchanged.

## Round 1 commentary active — batch 1
Prepared current pass-001 baseline. Data/CHR and unknown-range boundary notes added; reset guards and fixed CPU$c000..$cafc reviewed with hardware/RAM/entry/branch names and purpose comments. Fresh commentary verification byte-identical true. Full current-source readability remains incomplete; exact pending code intervals in commentary-round01/remaining.json. Continue same phase; no redirect handoff. Details: ../commentary/progress.md. Toolkit report unchanged: NES-MF-001 one missing feature, no bugs/blockers.

### Commentary batch 2
Current fixed-bank readability frontier CPU$dc9e (physical$1dcae), with all prior credit preserved; 713 exact remaining Code regions in commentary-round01/remaining.json. Meaningful routine units added. Fresh commentary verification byte-identical true (7ef28acb source). Concrete inline sprite-zero descriptor correction at physical$1cedd+4 recorded in commentary/uncertainties.md for redirect; frozen map remains unchanged. Full phase remains active/incomplete.

### Commentary batch 3
All currently classified fixed-bank code reviewed/named; unknown gaps remain honest. 465 banked Code intervals remain in commentary-round01/remaining.json. Fresh commentary rebuild byte-identical true (bd1fb399 source); phase active, no handoff. Corrected $ec73/$ec83/$ec93 table meaning to sound commands by direct consumer; no structural map mutation.

### Commentary batch 4
Classified sound driver reviewed with opcode handlers and nonlocal stack contracts; sound-bank-specific scratch aliases prevent false global roles. 384 exact pending Code intervals in commentary-round01/remaining.json. Fresh commentary rebuild byte-identical true (352307cb source), reading refreshed. Phase still active/incomplete.

### Commentary batches 5/6
Classified actor/equipment bank code reviewed with bank-local symbols, dispatch contracts and menu/text/data explanations. 309 exact pending code ranges remain exclusively in gameplay/ending bank3; commentary-round01/remaining.json. Fresh verification byte-identical true (5e751e79 source). Continue same phase.

## Round 1 commentary handoff — current-source readability complete
Checkpoint pass-001 unchanged, map SHA256 30a0e9f8324b21c1d9582312c4fde1da220b1d0f3600ed103cc5bb694765d69a. All 21382 classified Code bytes, 212779 non-code bytes and 27999 Unknown bytes inspected/commented to their supported scope; unknowns stay unknown. All 1224 scaffold entries reviewed; 514 compact source units,1131 clipped Code-unit associations. No pending classified readability ranges (commentary-round01/remaining.json=[]). Descriptive routines/branches/hardware/player/projectile/bank/menu/sound names and mechanical algorithms now in maintained source.
Fresh commentary verification byte-identical true; source SHA256 21c78f4cea6618ea976b733b87b3a66b35a473f8dfe435749cb3ca7403774378; reading-after refreshed. Final editorial audit pending/not run. Classification remains incomplete:234161/262160,89.32%,27999unknown. Handoff: handoffs/round01-commentary.md. Produced R1-C001 descriptor structural split,R1-C002 sound table evidence correction,R1-C003 reach/actor alignment,R1-C004 narrow bank-return contracts; prior leads retained. No new toolkit findings; existing missing feature NES-MF-001,zero bugs/no blockers. No toolkit edits,archive,cleanup,publishing.

## Round01 redirect started — r01-static-contracts
Input pass-001, map SHA25630a0e9f8324b21c1d9582312c4fde1da220b1d0f3600ed103cc5bb694765d69a;234161 classified,27999unknown. Read current-source commentary handoff, leads and evidence policy. Research only. Durable exact complement and progressive frontier: analysis/redirect/r01-static-contracts/{unknown-ledger.json,frontier.md}. Active leads R1-L001..L004 and R1-C001..C004; no accepted map edits. Disk3.6GiB free,scratch12KiB.

Redirect progressive checkpoint: actor main75-entry/type22-setter table, four synthetic-return dispatcher variants, connected state/movement/damage/spawn graph recovered; bank-preserving sound proof localized to scratch and balanced control, ffcf explicitly exits bank4, ffd8 score helper. Primary results and exact contracts in analysis/redirect/r01-static-contracts/findings.md. Traversal candidates unaccepted; continuing bank5 and actor7 spawn plus residual independent opportunities.

Redirect rescue improved method succeeded: supported proximity descriptor initialized actor7a6; exact player alignment consumed actor, incremented0507/05bf and cleared0503. Artificial only, no boss claim; exact patches,budgets,RAM and fetched evidence in redirect/r01-static-contracts/rescue*. Auxiliary80-type initialization/handler domain, bounded quadrant/state dispatch and initialized main condition selector0..3 resolved; preliminary CFG candidate set remains unaccepted and is under boundary review. Continuing independent sound/projectile/menu opportunities.

Redirect independent continuation checkpoint: sound51, projectile5/16, menu13 literal-bound action/subaction tables, fixed scene/camera/spawn dispatches inspected; Z/C pruning and complementary-branch data barriers localized. Corrected conservative input-latch stack rejection by nonzero held-bit contract, not replay. Exact roots/tables/profile constraints in redirect/r01-static-contracts/findings.md and manifests. Active final residual role/pattern review and proposal clipping; classification/map/source unchanged.

Redirect high-health actor tests saved: genuine descriptor type70 initialized health121 but defaultXff culled; changed supported kind40 position initialization retains activeactor30frames with firing. Actual fetched evidence and frame recorded; boss identity/natural route still unconfirmed. Current proposed25878unknownbytes,accepted0; exact2121byte112fragment residual review continuing, including visible small constant tables before generic-data fallback.

## Round01 redirect handoff — combined proposal ready
Research completed in analysis/redirect/r01-static-contracts after connected and independent actor, auxiliary, sound, projectile, menu, scene/camera/spawn, fixed data and IRQ investigations plus source-supported rescue/high-health actor experiments. No productive structural continuation or experiment left pending. This hands a coherent batch to independent disassembly, not terminal ROM completion.

Checkpoint pass-001 unchanged: map SHA25630a0e9f8324b21c1d9582312c4fde1da220b1d0f3600ed103cc5bb694765d69a;234161classified/27999unknown. ROM SHA2564a8b649b8ef253e178875f1a3f3614e16ac46484d8d49bb3ef7ff8f41633c818 and source21c78f4cea6618ea976b733b87b3a66b35a473f8dfe435749cb3ca7403774378 unchanged. Current-source readability complete and prior byte-identical verification retained; final editorial audit pending. No map/state/source/toolkit mutation.

PROPOSED27992 new structural bytes:15691 Code+12301 Data; ACCEPTED0. Separate zero-gain Code→Data4byte sprite-zero descriptor correction(region729) and48byte scene sound-command semantic correction(region522). combined-proposal.json contains exact7089 contextual instruction candidates and176 Data spans. proposal-validation.json passed physical-context, disjoint-interpretation, exact-complement, no partial-opcode, no Code/Data overlap and zero saved-fetch conflict checks. Fresh changed-map validation/rebuild belongs to independent disassembly after acceptance.

Exact current complement213ranges/27999bytes persists in unknown-ledger.json, with per-range proposed counts/dispositions. Seven bytes/six fragments remain outside proposal:108fb+1,10e2c+1,14422+2,14558+1 (isolated RTS),184c3+1,1fd8e+1 (isolated00). Residual-review.md evaluates actual competing code/data interpretations and positive IRQ/metatile counterexamples; no arbitrary PC probe would resolve intended role. Generic data proposals did not demand semantic names, natural consumers or original generators where non-code was clear.

Source-supported rescue route verified: initialized type26 actor7 and direct alignment consumes actor, clears0503, increments0507/05bf. High-health descriptor type70 initialized health121; defaultXff cull preserved as negative result, kind40 position revision keeps actor active30frames firing. Boss identity/natural route qualified, no old boss flag replay. scenarios.json updated; exact patches/budgets/context and fetched evidence retained.

Leads: R1-L001/L002/L003,C001/C002/C004 ready; L004/C003 verified research. Added ready R1-R001..R004. All original lead and experiment history retained with appended results; pass/attempt identity recorded in leads.json. Full primary artifacts/findings and compact handoff handoffs/round01-redirect.md. Preserve finalized lead history; finalization scripts are one-shot provenance.

No new toolkit issues found. Existing report NES-MF-001 one missing feature,zero confirmed/suspected bugs,no blockers. No toolkit edits,archive,cleanup,publishing. Free disk3.5GiB,owned research34MiB,scratch12KiB.

## Round02 disassembly active — independent proposal review
Input pass-001, map SHA25630a0e9f8324b21c1d9582312c4fde1da220b1d0f3600ed103cc5bb694765d69a;234161classified/27999unknown. Single pass; reviewing exact contextual code, bank/stack contracts, branch barriers and generic-data consumer/boundary evidence before acceptance. Existing commentary preserved. Durable review frontier: analysis/round02/review.md.

Round02 independent review accepted connected structural batch after primary stack/bank/dispatch and generic-data role checks; exact decision analysis/round02/review.md. Independent immutable decoding and physical/fetch conflict checks are in progress; per-batch durable acceptance artifacts will precede final checkpoint. Seven residual bytes retain genuine code/data ambiguity.

## Round02 disassembly pass complete — incomplete classification

ROM classification: 100.00% (262153 / 262160 bytes classified; 7 bytes unknown).

Accepted27992 previously unknown bytes:15691 Code/12301 Data. Corrected4 existing Code bytes to sprite-zero descriptor Data and48-byte scene sound-command evidence. Totals37069 Code/225084 non-code;1438 regions, all evidence annotated. Independent primary contract/bank/stack/branch/data review: round02/review.md; every proposed physical opcode decoded again with configured binary, exact candidate instruction boundaries/context match. Data spans' immutable bytes, nearby roles/references and fetch-conflict check: round02/independent-data-review.json. Per-batch data acceptance saved before code acceptance, final map validation true.

Input pass-001 map30a0e9f8324b21c1d9582312c4fde1da220b1d0f3600ed103cc5bb694765d69a. Output pass-002 map328a244dca1aece5e30489a075a5f36aca7fb35ae277de0b933ce97a0352e760; analysis/pass-002-verification.json confirms byte_identical=true and unchanged ROM SHA2564a8b649b8ef253e178875f1a3f3614e16ac46484d8d49bb3ef7ff8f41633c818. State saved, maps/current.json exported, checkpoint map/source retained. Existing maintained commentary/source unchanged; new generated/pass-002 is checkpoint source, not reconciled commentary.

| PRG bank | Code | Data | Classified | Unknown |
|---:|---:|---:|---:|---:|
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 8 | 8184 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 8 | 8184 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 8 | 8184 | 8192 | 0 |
| 6 | 7924 | 268 | 8192 | 0 |
| 7 | 2974 | 5218 | 8192 | 0 |
| 8 | 4194 | 3996 | 8190 | 2 |
| 9 | 462 | 7730 | 8192 | 0 |
| 10 | 3438 | 4751 | 8189 | 3 |
| 11 | 2755 | 5437 | 8192 | 0 |
| 12 | 2200 | 5991 | 8191 | 1 |
| 13 | 8 | 8184 | 8192 | 0 |
| 14 | 6973 | 1219 | 8192 | 0 |
| 15 | 6117 | 2074 | 8191 | 1 |

Header16/16 classified,0unknown. Every CHR bank0..15 is8192/8192 Data/Image,0unknown. Exact metrics/complement: round02/final-metrics.json. Seven bytes/six residual intervals108fb+1,10e2c+1,14422+2,14558+1,184c3+1,1fd8e+1 remain materially ambiguous isolated RTS/zero; reviewed actual evidence/alternative roles rather than requiring natural execution or semantic names. Rounded100.00 does not mean complete.

Consumed ready leads R1-L001/L002/L003,C001/C002/C004,R001/R002/R003/R004 verified; L004/C003 previously verified experiments retained. Connected batch includes downstream ordinary paths and independently bounded records, not only original lead intervals. No rejected proposed bytes; residual7 deliberately unproposed/unknown.

No new emulator scenarios executed: reused broad stages/menus/mechanics/ending/credits and redirect rescue/high-health artificial evidence; scenarios.json/gameplay.md retained. Unchanged startup or failed boss replay supplies no substantial new evidence. Natural boss identity remains qualified. This is one easy-gain pass, not a terminal all-methods exhaustion decision. Next commentary must freeze/reconcile prior maintained source against pass-002, then redirect may evaluate remaining isolated-byte ambiguity.

Toolkit: no new toolkit issues found; existing NES-MF-001 is one missing feature,zero confirmed/suspected bugs,no blockers. See toolkit-issues.md. No toolkit edits,archive,cleanup,publishing or ROM mutation.


## Round 02 commentary reconciliation — active
Pass-002/map328a244dca1aece5e30489a075a5f36aca7fb35ae277de0b933ce97a0352e760. Entire previous commentary preserved at round-02/commentary-pass-001 before supported prepare. Carried1010 identical classified physical regions and prior review credit;428 changed/new regions require reconciliation, including251 Code regions (15720 bytes including29 prior bytes around corrected descriptor) and177 Data. Exact frontier: round-02/pending.json; carry manifest round-02/carry.json. Descriptor split corrected source; retained established pre/post code labels. New baseline verified, carried-source byte-identical rebuild passed. Readability ACTIVE, final audit pending; no redirect handoff. Classification remains262153/262160,7unknown. No map/ROM/toolkit edits.

Round02 commentary Code batch saved: all251 changed/new Code blocks read and mechanically explained/named, global persistent actor/auxiliary fields plus scoped menu/sound aliases applied; control labels carried by physical bank/address. Fresh byte-identical verification true,source2e5e8dea21745ef19747e2cc3cc04dff27df0866a872979278fd23e3e9ec0096. Next177 Data regions plus stale unknown-gap dependencies/source inventory. commentary/progress.md updated; phase remains active.


## Round02 commentary completed → redirect
Full current-source readability COMPLETE for pass-002/map328a244dca1aece5e30489a075a5f36aca7fb35ae277de0b933ce97a0352e760. Frozen prior whole commentary: round-02/commentary-pass-001. Reconciled251 Code/177 Data spans plus48 stale dependency comments; carried1010 unchanged physical regions and prior review credit. All1444 scaffold entries reviewed,1853 unit associations,remaining classified readability frontier empty (round-02/remaining.json,scope-check.json). New names cover actors/synthetic returns, auxiliary families, menu subactions, projectile targeting, sound nonlocal stream exits, contacts/scene checkpoints/score/ending;50 exact vector spans use symbolic words. Descriptor/source/classification and scene-sound correction now agree.

Fresh supported commentary verify ok=true,byte_identical=true; sourceSHA25624406f925d28c9230f910160832b512e4561abf13ef2138c2f6513600332bbc6,round-02/final-verification.json. Reading-after refreshed after last source edit. ROM/map/state unchanged; classification262153/262160,7unknown in six ranges (108fb+1,10e2c+1,14422+2,14558+1,184c3+1,1fd8e+1),rounded100.00%INCOMPLETE. Unknown source notes distinguish five RTS/two zero. Final editorial audit pending,not run. No new structural lead; prior verified outcomes retained, no new runtime/repeated routes. Exact semantic gaps/limitations commentary/uncertainties.md. Compact handoff handoffs/round02-commentary.md; next redirect reasoning review,not terminal exhaustion.

No new toolkit issues found; NES-MF-001 existing missing feature only,zero bugs/blockers. Disk3.5GiB free,scratch12KiB; no cleanup/archive/publish/toolkit/ROM/map mutation.

## Round02 redirect — residual constraint review complete, coordinator inspection pending
Unique attempt redirect/r02-residual-constraints. Input pass-002/map328a244dca1aece5e30489a075a5f36aca7fb35ae277de0b933ce97a0352e760 unchanged. Fresh exact map complement ledger.json:262153/262160 classified,7Unknown in108fb+1,10e2c+1,14422+2,14558+1,184c3+1,1fd8e+1. Proposed0/accepted0. Current-source readability complete/source24406f925d28c9230f910160832b512e4561abf13ef2138c2f6513600332bbc6; final editorial audit pending.
Retrieved actual current neighboring source and prior primary dispatch/menu/vector/consumer evidence. Reopened narrower generic-data/table-extent/synthetic-return alternatives on unchanged evidence; tested literal target/target-minus-one hypotheses with immutable bank/record context. Apparent8412/8411 references lie in sound data under another bank;8547 occurrences are ADC47/STA instruction boundary coincidences. Corrected zero84b3 predecessor description to JMP, not return; this changes fallthrough scope, not structural role. Split8412/8413 claims explicitly; two60 bytes do not establish non-code or intended stubs. fd7e singleton ambiguity does not contaminate independently supportedff suffix. Exact claims, negative scope, combined constraints/competing roles and insufficiency rationale: rejection-review.md; current source excerpts primary-current-source.txt; reproducible review.py.
No productive frontier interrupted, no new ready lead or justified discriminating test identified. leads.json appends empty pass and preserves prior outcomes. Terminal incomplete is a proposal requiring coordinator primary-evidence/reasoning inspection, not ROM completion. Broad saved progression/ending/credits/rescue and corrected high-health actor audit reused; no unchanged runtime or exhaustive actor/room sweep. ToolkitNES-MF-001 one missing feature,zero bugs/blockers; no new toolkit issues found. Disk3.5GiB,scratch12KiB; no map/state/source/ROM/toolkit edits,cleanup,archive or publishing.


## Coordinator terminal handoff — justified incomplete, final acceptance verified

ROM classification: 100.00% (262153 / 262160 bytes classified; 7 bytes unknown).

Rounded100.00% is INCOMPLETE. Two sequential disassembly/commentary/redirect rounds are complete. The final redirect produced no ready lead and left no productive experiment interrupted. Coordinator independently inspected the full rejection reasoning, current assembly/raw residual windows, dispatch/table constraints and IRQ/metatile positive controls; narrower generic-data/suffix/table/return interpretations do not resolve the seven byte roles. Assessment: coordinator-rejection-review.md. No external blocker or unfinished phase is being treated as terminal success.

Final editorial acceptance completed after redirect: eight focused stale explanation/naming repairs and source-reference metadata normalization; valid prior review credit retained. Supported commentary/audit.json is ok=true,byte_identical=true,editorial_audit=true,1444regions/1853units. Maintained source SHA256733b383ddb54e191e7df18048de76361e70ddac3bcae31e01204376d2e41c1ee, current reading refreshed; no current classified readability/editorial backlog. Final handoff handoffs/final-editorial.md, commentary/editorial-audit.md. Older source hashes in historical handoffs remain checkpoint identities, superseded only for current-source acceptance.

Final coordinator checks: configured binary store validate and map validate both pass; independently recounted internal store37069Code+225084non-code=262153classified, exact7Unknown in six spans. Current map equals immutable pass-002 map328a244dca1aece5e30489a075a5f36aca7fb35ae277de0b933ce97a0352e760; input SHA2564a8b649b8ef253e178875f1a3f3614e16ac46484d8d49bb3ef7ff8f41633c818 unchanged. Supported archive editorial-status read-only check complete, source matches audit, reading current, no ready leads. Exact requirements/evidence/current document accounting: completion-audit.json. Terminal result and resumable limits: final-handoff.md.

Archive contract inspected directly in toolkit scripts/archive.py (FINAL plus archive() document-selection rules). Root archive-options.json explicitly selects final additional handoffs/research/coordinator records and historical commentary progress/uncertainties. Every unrecognized Markdown accounted; only three copied historical phase-instruction files omitted, no final result omitted. No archive executed. Stale rescue checklist deferral reconciled to the existing successful artificial experiment, initial reason retained.

Existing toolkit finding NES-MF-001 missing feature only,zero bugs/blockers; no new issues. No toolkit/input edits,cleanup,workspace deletion or publication. All evidence/history/checkpoints/prior commentary remain local. Structural classification remains incomplete and can reopen only on new independent evidence or a justified corrected inference/test; workflow completion is not ROM completion.

## User-directed Manual fallback disassembly → commentary

ROM classification: 100.00% (262160 / 262160 bytes classified; 0 bytes unknown).

User authorized generic Data fallback for <=10 residual bytes, overriding structural-proof requirement only for that bounded residue. Before mutation independently computed exact internal-state complement7bytes/sixspans:67835+1($108fb),69164+1($10e2c),82978+2($14422),83288+1($14558),99523+1($184c3),130446+1($1fd8e). Prior five RTS/two zero code-versus-data ambiguity retained; no new structural proof or Observed/Static claim. Six supported map.annotate calls use Manual provenance and unknown meaning. Full retained conclusions manual-fallback/conclusions.md,preflight.json,state-before.json,rpc-results.json. No research, lead replay or runtime performed.

262153 structurally supported bytes plus7user-directed fallback bytes=262160classified. Evidence-proven coverage remains99.997329875%; fallback exception completes the classification tally, not structural proof. Code37069/non-code225091,1444regions. Header16/16; every CHR bank0..15 Image8192/8192; every PRG bank0..15 classified8192/8192,0unknown. Exact Code/Data and provenance metrics manual-fallback/metrics.json; PRG8/10/12/15 gained2/3/1/1Manual Data bytes.

Input checkpoint pass-002/map328a244dca1aece5e30489a075a5f36aca7fb35ae277de0b933ce97a0352e760. Output fresh checkpoint pass-003-manual-fallback/mapb94f534aac83b7f8eb2cf735e14380fa341fb38f929c4a68fed99cbd85527dca. Supported map validation passed,state saved,current map exported; pass-003-manual-fallback-verification.json confirms byte_identical=true. Input SHA2564a8b649b8ef253e178875f1a3f3614e16ac46484d8d49bb3ef7ff8f41633c818 unchanged. Commentary untouched/source733b383ddb54e191e7df18048de76361e70ddac3bcae31e01204376d2e41c1ee. Next phase commentary reconciliation/final editorial required: handoffs/manual-fallback-disassembly.md. Old final handoff/completion audit retained unchanged as history. No completion claim before commentary.

No new toolkit issues found; existing NES-MF-001 missing feature only,zero bugs/blockers. Disk3.5GiB free,scratch12KiB. No toolkit/ROM edits,archive,delete or publish.


## Coordinator final acceptance — authorized Manual fallback complete

ROM classification: 100.00% (262160 / 262160 bytes classified; 0 bytes unknown). This explicitly comprises262153 structurally supported bytes plus7 user-directed Manual generic Data fallback bytes, not seven newly proven structural roles. Exact six ranges and prior ambiguity/authorization are retained in manual-fallback/conclusions.md and current final-handoff.md.

Separate sequential disassembly/commentary workers completed pass-003-manual-fallback and all-source reconciliation. Strict audit passed,1444regions/1853units reviewed; fresh byte-identical verification and current reading confirmed. Source ef4a245919f11f881083a30ce7960db61aefcdf8ee62600b06032c14b21399e1; map/state b94f534aac83b7f8eb2cf735e14380fa341fb38f929c4a68fed99cbd85527dca. Independent coordinator audit recomputed old exact complement/current zero complement, proved every prior region unchanged and only six authorized Manual Data additions, checked map/store validators, frozen commentary identities, strict audit/source hashes and reading freshness. No remaining readability/editorial work or other blocker.

Archive script document-selection contract inspected; archive-options.json explicitly includes fallback record, final/current/historical conclusions and handoffs, and accounts every unrecognized Markdown. Only copied historical instruction templates omitted. Prior coordinator handoff/audit preserved under manual-fallback/prior-*. Full current acceptance proof completion-audit.json. No archive/deletion/publishing/toolkit/ROM edits. Existing NES-MF-001 missing feature only; no new toolkit issues found.
