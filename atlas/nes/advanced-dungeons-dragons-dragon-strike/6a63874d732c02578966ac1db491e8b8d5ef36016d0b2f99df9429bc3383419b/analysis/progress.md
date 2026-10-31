# Disassembly progress

Round 001 disassembly complete as a phase; ROM analysis remains incomplete. No commentary or redirect was run by this worker.

ROM classification: 75.81% (397452 / 524304 bytes classified; 126852 bytes unknown).

Code 26,229; Data 109,079; Image 262,144. Structural evidence text covers all 397,452 classified bytes across 13,129 regions (12,994 code regions; 38 labels). This is structural annotation coverage, not full readable-source/editorial completion. Source commentary remains next.

Fresh checkpoint: [maps/pass-001-disassembly.json](../maps/pass-001-disassembly.json), [analysis/pass-001-disassembly-verification.json](pass-001-disassembly-verification.json), source ../generated/pass-001-disassembly. map.validate passed; generated source rebuilt byte-identically with the configured binary /root/romatlas/target/release/romatlas. State, current export and fresh checkpoint have identical SHA256 `c9f7a2f627454c733c0096ca352eab3da6fbf744c06f6c3153d106c2cfd4846e`. Input hash unchanged: `6a63874d732c02578966ac1db491e8b8d5ef36016d0b2f99df9429bc3383419b`. Older verified baseline, pass-001-initial, pass-001 and pass-001-final checkpoints remain preserved; they are historical and have older counts.

Coverage is separate: 8,342 distinct retained physical opcode starts, covering 16,812 instruction bytes. Every retained observed start is structurally classified. This combines natural startup/attract/first gameplay and explicitly artificial password/progression/language routes; it is not natural reachability for every classified byte. Zero trace detail capacity intentionally dropped detail while retaining aggregates; all saved scenario summaries report aggregate_dropped=0. RPC error stops preserve completed state and actual cycles in round-001/executed-routes.json; planned frame counts may be partial.

Evidence batches: reset/NMI vectors and startup contexts; known stage jump-slot API in pairs4/5 through18/19; immutable fixed-last-bank control flow; relocated sound driver and non-code sound payloads; exact uniform non-code runs after boundary/reference inspection; header/vector/declared CHR storage. Static code does not claim execution; specific data semantics were not fabricated. Pattern data uses generic Data where only non-code is established. A false static candidate at physical229431 ($8027 under menu context28/29) conflicts with known JMP operand; skipped consistently. Its fixed-engine branch belongs to a stage-only condition, not proof of menu-entry code. Other accepted code is retained.

Gameplay: natural first mission after player/dragon/difficulty/map/briefing menus. Artificial password routes visited missions3,5,8,11,14; representative A/B/directions were exercised. Supported success continuation exposed Queen boss, victory text, home flight and terminal return to logo. No separate credits demonstrated. Japanese title/player menu confirmed in26/27 via RAM0601/reset contract. See scenarios.json, gameplay.md and round-001/executed-routes.json. Not all dragon types, every mission or rare interaction were visited; representative systems plus ending are sufficient for this bounded pass, not a compatibility claim.

Stopping rationale: initial broad gains are collected, including the obvious language alternative. Later representative mission/ending batch added1,251 code bytes; final language batch986. Current connected discovery stops at unresolved mapper/callee effects and indirect dispatch contracts; large remaining stage payloads mix patterned records and possible embedded routines, so exact code/data slices need consumer/bounds research. Do not bulk-classify those gaps solely for missing execution or semantics. Clear uniform/data slices have already been classified. Further repetitive same-loop play is lower value than the concrete redirect questions. This is an ordinary easy-gain handoff, not a terminal exhaustion assessment.

Lead outcomes: no inherited ready leads. R001-E001 superseded invalid input-schema premise; E002 rejected direct objective-counter patch after observed recomputation; E003 verified password buffer; E004 verified scene-success entry contract; E005 verified language variant. E002 does not reject the narrower E004 entry contract. Produced R001-Q001 stage record/code bounds, Q002 callee bank preservation/indirect dispatch, Q003 sound-data semantics for commentary. Exact recipes and missing facts are in leads.json. Do not repeat unchanged E002.

Toolkit: one confirmed documentation/API-validation bug (NES-001), zero suspected bugs, zero missing-feature reports, no blocker. See [toolkit-issues.md](toolkit-issues.md). Correct NES input uses complete buttons arrays; all earlier single-button/pressed calls were ignored and their records corrected to attract playback. No toolkit changes, deletion, cleanup, publishing or archive actions occurred. Captures/snapshots/source are local. Disk check at handoff remains healthy; scratch contains no bulk output.

| PRG bank | Code | Data | Unknown |
|---:|---:|---:|---:|
| 0 | 9 | 8183 | 0 |
| 1 | 688 | 7504 | 0 |
| 2 | 9 | 8183 | 0 |
| 3 | 688 | 7504 | 0 |
| 4 | 1387 | 1592 | 5213 |
| 5 | 0 | 2235 | 5957 |
| 6 | 794 | 1206 | 6192 |
| 7 | 0 | 2513 | 5679 |
| 8 | 758 | 2785 | 4649 |
| 9 | 0 | 3397 | 4795 |
| 10 | 933 | 740 | 6519 |
| 11 | 0 | 1681 | 6511 |
| 12 | 830 | 965 | 6397 |
| 13 | 148 | 2364 | 5680 |
| 14 | 2182 | 1657 | 4353 |
| 15 | 0 | 1575 | 6617 |
| 16 | 1250 | 54 | 6888 |
| 17 | 0 | 1228 | 6964 |
| 18 | 1195 | 847 | 6150 |
| 19 | 0 | 1496 | 6696 |
| 20 | 0 | 8192 | 0 |
| 21 | 0 | 8192 | 0 |
| 22 | 0 | 8192 | 0 |
| 23 | 0 | 8192 | 0 |
| 24 | 0 | 8192 | 0 |
| 25 | 0 | 8192 | 0 |
| 26 | 676 | 918 | 6598 |
| 27 | 310 | 191 | 7691 |
| 28 | 1827 | 509 | 5856 |
| 29 | 2048 | 262 | 5882 |
| 30 | 3335 | 179 | 4678 |
| 31 | 7162 | 143 | 887 |

Header: 16 data bytes. CHR banks 0–31: each 8,192 Image bytes, zero unknown.

Exact residual complement and complete PRG/CHR counts: round-001/unknown.json and round-001/classification-counts.json.

## Round 001 commentary, active checkpoint

Prepared frozen commentary baseline and reviewed 1,964 / 26,229 classified code
bytes: physical20..29,15442..16130,16404..16413,31335..32023,261421..261991.
Both sound drivers and fixed-bank controller/NMI/reset tail now have functional
names/mechanical explanations. All Data/CHR and unresolved PRG gaps have local
boundary/uncertainty notes. Code backlog remains12,106 instruction records /
24,265 bytes, exactly the pending Code records in commentary/review.json.
Continue this same commentary phase before redirect. Fresh byte-identical source
verification commentary/verification.json passed; ROM and classification unchanged.
Details/frontier: commentary/progress.md. Final editorial audit remains reserved.

### Round001 commentary interruption checkpoint

Readability phase remains unfinished and must resume before redirect. Latest
reviewed code2,811/26,229 bytes (1,271 instruction records), pending23,418 bytes
(11,723 records); exact278-range frontier: round-001/commentary-frontier.json.
Connected fixed-bank additions explain bank switching/audio priority, scene-loop
stack marker and carry completion contract, pause, and completion dispatch.
All non-code/CHR/unknown inventory notes retained. Fresh byte-identical verify and
reading-copy refresh passed; source SHA256 ecd59d5151001b85fd28444a1409b7f5a3e1b188499ac1db53f1fc30d7eebb44.
Input SHA2566a63874d732c02578966ac1db491e8b8d5ef36016d0b2f99df9429bc3383419b;
map/state SHA256c9f7a2f627454c733c0096ca352eab3da6fbf744c06f6c3153d106c2cfd4846e unchanged.
Handoff: handoffs/round-001-commentary.md. No toolkit change or new issue; NES-001
persists, no blocker. No editorial audit, archive, cleanup, deletion or publication.

Commentary cont02 checkpoint: reviewed 5,725/26,229 code bytes; 20,504 pending.
Exact 265-range frontier updated in analysis/round-001/commentary-frontier.json.
Fixed31 render/PPU/palette/CHR and five-byte scene-record mechanisms now explained.
Source SHA256 c4ddb789f896378a742a6ed8ddbe07581721782d2b6b611e07e31c1eb6144279;
fresh byte-identical verify passed and reading copy refreshed. Same commentary
phase remains active and incomplete; no redirect or final audit.

Commentary cont02 interrupted at cumulative9,140/26,229 code bytes reviewed;
17,089 pending bytes /8,474 records, exact256-range frontier saved. All classified
fixed31 source is reviewed; fixed30 still2,751 pending bytes, stage/menu backlog
unchanged. Source f184d62efe5bbf7ee0120fa3caef8d90a611c090511158d60b2f2ecca4c96c98
freshly byte-identical verified and reading refreshed. Resume same phase using
analysis/handoffs/round-001-commentary-cont02.md; no successful handoff or redirect.

Commentary cont03 interrupted after 21,368/26,229 code bytes reviewed; 4,861 bytes
remain in64 exact ranges, entirely banks26..29. All classified stage and fixed
bank code reviewed. Latest byte-identical verify/read source 8e4fad73bf4dea657880ef8bad356743148430d7050e97ebe8f9555d9b93c51f.
Same phase incomplete; see analysis/handoffs/round-001-commentary-cont03.md.
No redirect, final audit, archive, map/ROM/toolkit edits or publication.

### Round001 commentary accepted and redirect active (2026-10-30)
Coordinator independently accepted commentary completion from handoffs/round-001-commentary-complete.md: all26229 Code bytes,13488 scaffold records/399 units reviewed;0 pending. Fresh byte-identical/read verification accepted source7c2412a800002f18dbeae885737e0aefad239220b4e2624dd47a8c43651b2130. Map/state unchanged c9f7a2f627454c733c0096ca352eab3da6fbf744c06f6c3153d106c2cfd4846e; classification75.81%,126852 unknown. Final editorial audit remains pending.
Redirect sole writer active in redirect/round001-20261030-a/plan.md. Research only: no map/state/source/toolkit/ROM mutation. Initial frontier is bank-aware missing code/data continuations and exact residual ledger; no terminal assessment.

Redirect milestone3 (same active phase, research only): redirect/round001-20261030-a/plan.md and proposal-union.json now113037 proposed new bytes, hypothetical13815 residual; actual126852 unknown unchanged. Independent stage map/pattern/attribute consistency, sentinel records, finite composite/movement streams and bank-aware code/dispatch contracts accumulated. Generic class data and paired object descriptors inspected with localized bank14 semantic-range counterexample preserved. Existing source14/15 commentary correct; only new evidence draft corrected. Current frontier fixed30 tables, menu payloads, actor row-event/remaining call barriers, then combined lead/ledger audit. Not a handoff or terminal assessment.


### Round001 redirect complete: combined supported proposal
Attempt redirect/round001-20261030-a and handoffs/round-001-redirect.md. Proposes126846 NEW bytes (Code22645 /10759 instructions, Data104201), accepted new bytes0. Of new Code,143 actual keyboard initialization/active-edit opcode addresses295 bytes are Observed; remainderStatic. Fresh configured-toolkit decode cross-check244 groups and audit pass: no instruction overlap, Code/Data conflict or physical-view mismatch; CHR selector constant dominance checked.
Actual map/state c9f7a2f627454c733c0096ca352eab3da6fbf744c06f6c3153d106c2cfd4846e and source7c2412a800002f18dbeae885737e0aefad239220b4e2624dd47a8c43651b2130 unchanged. Actual classification397452/524304 (75.81%),126852 unknown/359 ranges. If every proposal accepted, only6 bytes in2 bare-JMP ranges remain, NOT a completeness claim. Final editorial audit pending; current-source readability accepted unchanged.
All identified promising connected/independent avenues examined through finite formats, consumers and bank/stack contracts; ledger gives every actual remaining interval a disposition. Persistent leadsQ001/Q002 materially improved; Q004/Q005/Q006 ready, Q007 localized ambiguity; histories preserved. New ending idle route visibly captures named credits,06F5 reaches42 by4200 extra frames; distinct THE END screen not shown. Active keyboard A/B verified after ready wait; initial probe only initialized. Source-only gate effects and ending text/row schema resolved. No new toolkit issue; existing NES-001 workaround buttons array, no blocker. No ROM/map/source/toolkit modification, archive, cleanup, deletion or publication.

### Round002 disassembly active — independent review
Input map/state c9f7a2f627454c733c0096ca352eab3da6fbf744c06f6c3153d106c2cfd4846e; current commentary source remains protected. Reviewed redirect scripts and primary consumer/count/stride/sentinel/bank contracts for stage/menu/composite/movement/generic families and closure barriers. Current frontier: independently cross-check exact manifest identities, encoded extents and provenance, then annotate unknown-only supported slices using documented map APIs; preserve 2 bare-JMP ambiguities. Unique evidence: round-002/. No unchanged gameplay replay or source edits.

Round002 independent review accepted full126846-byte batch; application live session4164,4000/12501 annotations applied in memory. Durable input state unchanged until final validate/save/export. Do not restart/reapply; exact ongoing frontier round-002/application-frontier.json. Review rationale round-002/independent-review.md; old commentary untouched.

### Round002 disassembly complete — incomplete ROM
ROM classification: 100.00% (524298 / 524304 bytes classified; 6 bytes unknown).
Accepted126846 unique formerly unknown bytes: Code22645 (10759 instructions), Data104201 (1742 object-boundary slices); new Observed143 opcode starts/295 bytes, remaining new annotations Static. Final Code48874/Data213280/Image262144/Unknown6. Exact per-bank counts: round-002/counts.md and classification-counts.json. All PRG banks except30/31 now have0 unknown; fixed30/fixed31 each retain3 unknown. Structural evidence covers524298 bytes. Provenance/execution are separate: retained Observed instruction bytes17107 (round00116812 plus new295), including explicitly artificial routes; not universal natural reachability or total gameplay execution coverage.

Independent review: round-002/independent-review.md, review.py/review-checks.json, accepted-regions.json; original redirect evidence preserved. Recomputed backing identities/hashes, finite movement command endpoints and18 menu RLE streams; manually reviewed caller/bank/stack summaries, dispatch domains, practical generic non-code evidence and local utility correction. Data accounting unions split at underlying object boundaries; ending21-row/text roles supersede weaker generic explanation locally. Old annotations unchanged byte-for-byte, old entire commentary tree untouched. API id0 creates entries; initial invalid nonzero/missing-id attempts rejected without mutation, corrected before actual application; not a toolkit defect.

map.validate passed; state/current SHA2568c90238b3ebc1e65c9ea29eb0c92fe1d942bc7fc1e7e8a798945cc58fa5902ac. Fresh unique checkpoint maps/pass-002-disassembly-verified.json / generated/pass-002-disassembly-verified / analysis/pass-002-disassembly-verified-verification.json verifies byte_identical=true. ROM SHA2566a63874d732c02578966ac1db491e8b8d5ef36016d0b2f99df9429bc3383419b unchanged; commentary source remains7c2412a800002f18dbeae885737e0aefad239220b4e2624dd47a8c43651b2130. No baseline recreation. Source generated at checkpoint is scaffolding requiring next full current-source commentary; prior reviewed source stays intact, final editorial audit pending.

Lead outcomes: R001-Q001/Q002/Q004/Q005 verified and annotated; Q006 verified37-byte subset ofQ002, no double count; Q007 blocked only6 isolated bare-JMP bytes, exact frontier round-002/unknown.json. Q003 optional semantic naming stays commentary dependency, no structural blocker. Retained all prior histories and evidence. Utility contracts support Static structure without actual caller; no jump-only inference used.

Gameplay reused representative early/middle/late/Queen/home/scrolling credits and active keyboard evidence from scenarios.json. No unchanged replay; ending event06F5=42 and visible named credits documented, distinct THE END capture unproven. No additional mission permutation has useful discriminator for two isolated jumps. This ends one easy-gain pass, not terminal exhaustion or complete ROM; next phases commentary then redirect under coordinator. Read redirect/round001-20261030-a/resolved-behaviors.md and ending-data.json for corrected gate effects/text-helper cursor preservation/editor B behavior.

Toolkit:2 confirmed issues NES-001 and new NES-002 save/export serialization exceeding reader16MiB bound;0 suspected,0 missing-feature reports,no blocker. See[toolkit-issues.md](toolkit-issues.md) and round-002/checkpoint-size-reproduction.json. Lossless JSON whitespace compaction reduced18607346 to12205564 bytes; preserves all parsed records and permits valid load/rebuild. Failed maps/pass-002-disassembly.json retained. Full buttons-array workaround retained. No toolkit change, cleanup/deletion, archive, publication or ROM mutation. Disk9.8GiB free at start; only bounded review/API output/checkpoint retained.

| PRG bank | Code | Data | Unknown |
|---|---:|---:|---:|
| 0 | 9 | 8183 | 0 |
| 1 | 688 | 7504 | 0 |
| 2 | 9 | 8183 | 0 |
| 3 | 688 | 7504 | 0 |
| 4 | 2504 | 5688 | 0 |
| 5 | 0 | 8192 | 0 |
| 6 | 2222 | 5970 | 0 |
| 7 | 0 | 8192 | 0 |
| 8 | 2133 | 6059 | 0 |
| 9 | 0 | 8192 | 0 |
| 10 | 2355 | 5837 | 0 |
| 11 | 0 | 8192 | 0 |
| 12 | 2802 | 5390 | 0 |
| 13 | 474 | 7718 | 0 |
| 14 | 3530 | 4662 | 0 |
| 15 | 0 | 8192 | 0 |
| 16 | 4021 | 4171 | 0 |
| 17 | 0 | 8192 | 0 |
| 18 | 2295 | 5897 | 0 |
| 19 | 0 | 8192 | 0 |
| 20 | 0 | 8192 | 0 |
| 21 | 0 | 8192 | 0 |
| 22 | 0 | 8192 | 0 |
| 23 | 0 | 8192 | 0 |
| 24 | 0 | 8192 | 0 |
| 25 | 0 | 8192 | 0 |
| 26 | 3782 | 4410 | 0 |
| 27 | 2452 | 5740 | 0 |
| 28 | 3676 | 4516 | 0 |
| 29 | 2486 | 5706 | 0 |
| 30 | 5284 | 2905 | 3 |
| 31 | 7464 | 725 | 3 |

Round002 metric clarification: map Code provenance is Observed13485 bytes/Static35389 bytes (classification-counts.json). Retained execution aggregate17107 instruction bytes/8485 physical opcode starts is separate and includes records classified Static before later observed execution; the preceding phrase Observed instruction bytes17107 refers to that execution aggregate, not annotation provenance. Compact state/current parsed-equal to preserved18,607,346-byte failed checkpoint; fresh new-server map.load/map.validate pass (round-002/compact-reload-verification.json). New serialized map hash8c90238b3ebc1e65c9ea29eb0c92fe1d942bc7fc1e7e8a798945cc58fa5902ac. All12501 annotations applied and live session terminated.

Round002 COMMENTARY active: expanded baseline prepared after preserving entire prior commentary at analysis/round-002/commentary-preserved-round001. Physical-range carry of 26,229 old code bytes/399 units verified byte-identically at source913ec57d883535f919d9f2a8e44a30a70c7b774b73446bd4d818b8a3c3d7f2d7. New22,645 Code bytes plus Data remain; exact live frontier analysis/round-002/commentary-frontier.json. Continue same phase, not redirect. No toolkit/map/ROM edits; final audit pending.

Round002 COMMENTARY connected checkpoint: reviewed29,814 Code bytes/14,703 instructions, pending19,060/9,050 plus1,708 Data records. Fixed31 classified code complete; ending/sound-test/language/password corrections and eight-bank actor aim/render/window helpers complete. Fresh byte-identical verify bc8067f4; continue same commentary phase. Exact frontier analysis/round-002/commentary-frontier.json.

Round002 COMMENTARY interrupted by context scope, continue SAME phase. Reviewed29,814 Code bytes/14,703 records; pending19,060/9,050 across180 ranges. Data1876/Unknown2/Image1 fully reviewed; units2183 reviewed/source-linked. Fresh byte-identical verify/read d22540a6846b71dd4e48bb78571c0c98b20d87fb518bde5a0011c9ea7c4812f1. Exact frontier analysis/round-002/commentary-frontier.json; completion/redirect not authorized. Handoff analysis/handoffs/round-002-commentary-cont01.md.

ROUND002 COMMENTARY COMPLETE (coordinator accepted): maintained source SHA256 0b99d165d71dbdccebe869a990a3f1ad7c8fb49ba7bfe912bff1b10ca7e6bbf8; all48,874 Code bytes/23,753 Code records,1,876 Data records,2 Unknown records/6 bytes,1 Image record and2,461 meaningful units reviewed; zero current-source backlog. Fresh verify/read accepted. Earlier cont01 interruption above is historical and must NOT resume. Final editorial/own-unit projection remains reserved and pending. Input map/state8c90238b3ebc1e65c9ea29eb0c92fe1d942bc7fc1e7e8a798945cc58fa5902ac unchanged; see analysis/handoffs/round-002-commentary-complete.md.

ROUND002 REDIRECT active, attempt analysis/redirect/round002-20261031-a. Input identities independently hashed in identity.json. Exact map complement524298/524304 classified,6 Unknown:245776..245779 fixed30:C000 JMPFE98;255289..255292 fixed31:E529 JMPE614. Map omits Unknown records; ledger derives sorted annotated complement, not exporter gaps. Current frontier: source/primary rejection review and bounded bank14 phase/render representation/context investigation. No map/source/state/ROM changes; no live emulator experiment or final editorial audit. Bank14 representation test already narrows prior claim: phase table second bytes are bases; all eight102-byte tables are physically identical, but bank14 composite spans181/189 differ. Type25 base199 still exceeds both spans with phase0/category either0or1. Further natural selector coverage not a prerequisite to the already accepted Data classification, and no connection to six residual bytes established. Primary details in bank14-representation.json; compact reasoning/handoff being prepared for coordinator audit.

ROUND002 REDIRECT research complete as INCOMPLETE PROPOSAL awaiting coordinator independent rejection review. Exact classified524298/524304;6 Unknown unchanged at245776..245779 and255289..255292; proposed/accepted new bytes0/0. Added verifiedR002-E007 (corrected bank14 image-base arithmetic plus byte-identical shared descriptors; concrete type25 counterexample persists), updatedR001-Q007 primary residual review and retired staleR001-Q003 ready flag. Ready annotation leads0. Handoff analysis/handoffs/round-002-redirect.md; decisive compact reasoning analysis/redirect/round002-20261031-a/rejection-review.md. No productive interrupted frontier, no runtime replay, no final editorial audit. Coordinator must audit before accepting terminal incomplete; a useful corrected inference/test resumes redirect directly. Source/map/state/ROM identities unchanged; no archive/publication/toolkit changes.

## Coordinator terminal handoff — justified incomplete

Both rounds and final editorial acceptance are finished; no active phase writer or interrupted productive frontier remains. Coordinator accepted primary rejection review for the two3-byte intervals at245776..245779/C000 and255289..255292/E529. No ready structural lead remains; classification is incomplete.

ROM classification: 100.00% (524298 / 524304 bytes classified; 6 bytes unknown).

Current Code48874/Data213280/Image262144; annotation provenance Observed13485/Static35389 Code bytes, separate retained execution17107 instruction bytes/8485 starts. Full current-source readability and official editorial audit passed, byte_identical=true;25632 reviewed scaffold records,2465 unique meaningful units/25503 membership records,0 pending. Final source badaa8e4be4cd064d4c533321436ef01b18d0a4acfdda69c6d67eb11f662b2a2; reading6dc23fb15c60cfd3e3ec4cff64adb26f5230da971c30cb73a5b9fb0353055871. Source edits74 comment lines only. ROM original6a63874d... unchanged; compact state/current/verified checkpoint8c90238b... unchanged. Official audit session12807 exited0 and is complete, not resumable/live.

Final artifacts: terminal-handoff.md, coordinator/completion-audit.md, current-audit.json, commentary/audit.json, commentary/editorial-audit.md and handoffs/terminal-editorial-acceptance.md. Rejection reasoning coordinator/rejection-review.md; original question history and old commentary preserved. LeadsQ001/Q002/Q004/Q005/Q006 verified; Q007 six-byte ambiguity and Q003 optional identity limits retained, no ready annotations; R002-E007 representation correction verified. No new classification cycle warranted by unchanged evidence.

Archive-options.json explicitly selects final/historical documents and omits only exact duplicate operational templates; final document audit records coverage. Toolkit2 confirmed bugs+1 missing performance feature,0 blockers; see toolkit-issues.md. No toolkit implementation changes, archive execution, cleanup/deletion or publication. Only the orchestration terminal handoff is achieved; ROM classification is NOT complete.
