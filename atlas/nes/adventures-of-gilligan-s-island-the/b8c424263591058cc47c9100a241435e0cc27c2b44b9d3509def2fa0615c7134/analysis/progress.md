# Disassembly progress

Workspace initialized. Baseline and classification have not run.

Round 01 active: baseline byte-identical (analysis/baseline-verification.json); bounded startup and vector-root static discovery saved in analysis/round01. Classification/progression ongoing.

Round 01 static frontier saved: 702 regions, 1414 classified bytes; backup analysis/round01/static-state.json. Header/vectors and 912 vector-root instructions accepted, map validated. Runtime route ongoing; final merge preserves this backup.

Round 01 durable batch: 19712 bytes classified in 1366 validated regions, including 18 uniform non-code runs (16,896 bytes); runtime trace observed 1,662 physical opcode starts across 679 frames (aggregate_dropped=0). Intro portraits identify Gilligan’s Island. See analysis/round01/trace-summary.json and gameplay.md.

Round 01 static atlas batch: 50829 / 131088 classified bytes; banks0–3 remaining unknown spans now Data from coherent planar artwork and exact code boundary at bank4. Evidence analysis/round01/prg-planar-atlas.png. Generic payload role avoids unsupported internal tile-set extent claims. Protocol mismatch workaround documented in analysis/toolkit-issues.md. Continuing episode1 and password-supported episode4 progression.

Round 01 scheduler static batch: 54,062 classified bytes. Validated five-slot scheduler (CMP #05, ASL word indexing) resolves 10-byte target table and 5-byte PRG selector table; inspected handler contracts yield1,865 new executable bytes. Evidence round01/dispatch-handler-*.json and dispatch-discovery-0.json. Natural activation of individual tasks remains separate. Slot0/1 player/follower-like state, slot2 object records, slot3 health-zero/game-over-like path, slot4 sound engine; provisional semantics only. These do not themselves establish safe intro/boss entry. Intro task enable flags at92/A2/B2/C2 are zero; D2=84 active sound task. Repeated >60-second helper exits143 now mitigated by per-cycle exported snapshots and 3-cycle batches.

Round 01 intro bypass verified: episode4 now actual controlled gameplay with life/time HUD, player and Gilligan on forest road (ep4-bypass.png). AC89 common ACEE cleanup and caller return DE13 preserved; entry via FEFB A6/SPF7 after patchRAM1F8..1F9=ED AC,85=6. Full recipe in ep4-bypass-route.json. Same supported bypass has completed for episodes1/2/3; all snapshots saved. Earlier task4 sound-engine inference is superseded: full DB03/DBC3 and scene3 DCF4 show scene controller with music calls. RAM78 role remains mode/scene-related; no RAM78 patch used. Main gameplay progression/mechanics now sampling, with bounded health/time aid and scene-specific trace roots.

Round 01 gameplay/static frontier: accepted direct-JSR callee entries preserve caller banks at entry; 3 finite closure rounds added1,183 bytes without assuming post-call bank preservation. Adjacent identical Code records compacted3,579→282, retaining CPU start/context/evidence/provenance; backup round01/precompact-state.json. Actual episode4 explored751 frames with scrolling/jump/attack, sourced health/time aid, complete contextual roots round01/ep4-explore-contexts.json. Room8D stayed20; no boss/ending claim. Complete ep4 roots being merged by exact coalesced instruction boundaries; smaller episode1/2/3 samples underway. Earlier long helpers exited143; their partial routes retained, snapshot frontiers reused, zero ROM/input changes.

## Round 01 completed — disassembly pass, incomplete ROM

ROM classification: 48.19% (63165 / 131088 bytes classified; 67923 bytes unknown).

Physical total includes16-byte iNES header; header16 bytes classified Data, all16 PRG8KiB banks shown below. Unique classified bytes: Code12713, Data50452; 608 nonoverlapping annotated regions, all with evidence; every Code region has explicit CPU execution view. Provenance Observed 6085 bytes; remaining classifications Static. This is structural classification, separate from execution/readability.

| Physical PRG8KiB bank | Code | Data | Classified | Unknown |
| --- | ---: | ---: | ---: | ---: |
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 0 | 8192 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 0 | 8192 | 8192 | 0 |
| 4 | 0 | 0 | 0 | 8192 |
| 5 | 0 | 1364 | 1364 | 6828 |
| 6 | 1467 | 2560 | 4027 | 4165 |
| 7 | 0 | 6010 | 6010 | 2182 |
| 8 | 616 | 88 | 704 | 7488 |
| 9 | 3308 | 579 | 3887 | 4305 |
| 10 | 0 | 0 | 0 | 8192 |
| 11 | 0 | 2854 | 2854 | 5338 |
| 12 | 1167 | 0 | 1167 | 7025 |
| 13 | 63 | 3979 | 4042 | 4150 |
| 14 | 1943 | 0 | 1943 | 6249 |
| 15 | 4149 | 234 | 4383 | 3809 |

Baseline and fresh checkpoint verified byte-identical: [pass-001 verification](pass-001-verification.json), [frozen map](../maps/pass-001.json), [current portable map](../maps/current.json), [generated assembly](../generated/pass-001/game.asm). Internal [state](state.json) retained locally; it is not the sharing deliverable. Input sha256 unchanged. Current/frozen map SHA2568385e4f38235aaf01428c0ae6b4c9ef5aeeae27e1415c6296350f3734ffc7dd0. Exact unknown complement and metrics: round01/unknown.json, round01/metrics.json.

Gameplay: title/CONTINUE UI and episode1 intro naturally visited. Sourced passwords for episodes2–4 accepted through ordinary Start after controlled RAM character entry; each distinct intro confirmed. Native intro cleanup bypass preserved AC89 caller frame and post-intro setup, allowing actual gameplay with HUD/player/follower in all4 episodes. Each exercised representative walking/jump/attack/Select input; complete short sample traces and snapshots saved. Episode4 additional scrolling-road progression751 frames used sourced health/time aids; room8D remained20, no boss reached. All edited scenarios explicitly artificial. The saved contextual-root corpus union has 4008 distinct physical ROM opcode starts and 2 non-ROM opcode starts; this is a saved-sample metric, not exhaustive execution-byte coverage. RAM execution was not mapped to invented ROM origins. Trace summaries report zero aggregate drops; zero-capacity detail buffers intentionally dropped detail.

Ending practical attempts: first RAM77..78=[1,4] request was not consumed in ongoing gameplay coroutine; second direct DB03 call with synthetic DB03→DBC3→DAF1→FA64 return chain also left77/78/D5 unchanged with old frame visible after363 frames. Preserve ending-route.json and ending-direct-route.json as unsuccessful tests, not verified ending visits. No ending/credits or boss victory claim. Missing live coroutine/scene activation contract is handed to redirect; arbitrary flags/rooms were not brute-forced.

Static easy gains: validated vectors; inspected coherent planar banks0–3 and bank6 prefix, exact uniform runs; five-slot scheduler and banks; connected actor-list/intro tables; direct-JSR entry closure; fixed-slot continuations after three independently balanced helpers. Final episode samples yielded small incremental code gains after shared episode4 paths; supported roots/batches were classified. Further unknown banks/data consumers, indexed/native scene flow and ending activation need redirect investigation rather than repeating inputs/failed entry methods. This is a normal incomplete easy-gain handoff, not terminal exhaustion or editorial completion.

Toolkit: one confirmed documentation/API consistency bug NES-DOC-001, zero suspected bugs/missing-feature entries, no blocker. [Persistent toolkit issues](toolkit-issues.md). Configured release version0.1.0, repository0e58894598c67a60e344092fed1d3545873b4749. No toolkit changes. Workspace helper schema/transport mistakes and exits143 are not attributed to toolkit. Historical frontier paragraphs above are retained; this section is authoritative. Disk at handoff9.6GiB free; scratch32KiB, round01 evidence170MiB.


## Round01 commentary active
Prepared frozen commentary baseline; all 63,165 classified bytes and unknown boundary notes pending in commentary/review.json. Same phase resumes until complete. Frontier: round01/commentary-handoff.md.

## Round01 commentary completed — full current-source readability

All12,713 Code and50,452 Data bytes reviewed/named/commented in maintained commentary/source. Full776 inventory entries and320 semantic units reviewed;168 unknown spans/67,923 bytes locally explained without new classification.1,004 bank-aware named targets plus RAM/field/hardware/constant symbols. Fresh commentary/verification.json confirms byte identity and unchanged ROM; commentary/reading-after.asm refreshed afterward. Frozen/state/maps unchanged. Final editorial audit remains pending; full ROM classification remains48.19%. Exact scope, unresolved meanings and bounded IRQ overlap question: round01/commentary-handoff.md. No new toolkit issue or toolkit edit; existing NES-DOC-001 remains nonblocking. Current completed phase commentary; next coordinator phase redirect. No live tool process.

Redirect round01 active: unique research analysis/redirect/round01-contracts-20261031/frontier.md; input pass-001 unchanged, commentary gate met. Full instruction-level caller survey and static contract closure produced candidate batch (provisional1476 bytes; pending independent contract audit). NMI-discharge experiment now observes DB03/FAD6 fetches, revising prior activation failure. Other connected and independent research ongoing; no redirect terminal or phase handoff yet.

Round01 redirect COMPLETE, ready for separate disassembly worker. Input/output accepted checkpoint pass-001 unchanged:48.19% (63165/131088),67923 unknown; existing byte-identical commentary verification and source hashes preserved. Unique research analysis/redirect/round01-contracts-20261031; authoritative compact handoff analysis/round01/redirect-handoff.md and attempt/handoff.md. Produced R1-DATA-CONSUMERS ready / R1-CALL-CONTEXT ready; R1-ENDING-ACTIVATION verified; added R1-OVERLAP-REPRESENTATION blocked (NES-MAP-002), no toolkit edits. Combined exact proposal47661 bytes=42761 Data+4900 Code, accepted0, conflicts0; hypothetical residual20262 only if all accepted. Full instruction-level calls, supported indexed handlers, distinct bank-preserving/fixed ordinary returns, positive data consumers/patterns and practical terminal scene investigated. Artificial ending text+nine montage states+THE END visited, A/Start sampled, no natural boss-victory claim. Exact168-span unknown ledger and primary evidence retained. Not terminal exhaustion; next disassembly independently reviews batch. Redirect worker relinquishes writes/processes, no next phase started.


## Round02 completed — disassembly pass, incomplete ROM

ROM classification: 84.54% (110826 / 131088 bytes classified; 20262 bytes unknown).

Accepted 42761 Static Data bytes and 4900 Static Code bytes after independent consumer/contract and exact bank/boundary review. Unique Code 17613, Data 93213; header16 Data included in physical total. 213 unknown intervals remain; exact complement in round02/unknown.json, metrics in round02/metrics.json.

| Physical PRG8KiB bank | Code | Data | Classified | Unknown |
| --- | ---: | ---: | ---: | ---: |
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 0 | 8192 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 0 | 8192 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 0 | 8192 | 8192 | 0 |
| 6 | 1467 | 6199 | 7666 | 526 |
| 7 | 0 | 8192 | 8192 | 0 |
| 8 | 644 | 7448 | 8092 | 100 |
| 9 | 4262 | 653 | 4915 | 3277 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 283 | 7242 | 7525 | 667 |
| 12 | 1898 | 0 | 1898 | 6294 |
| 13 | 86 | 3979 | 4065 | 4127 |
| 14 | 3800 | 390 | 4190 | 4002 |
| 15 | 5173 | 1750 | 6923 | 1269 |

Output state analysis/state.json and maps/current.json validated; fresh output maps/pass-003.json, generated/pass-003 and analysis/pass-003-verification.json confirm ok=true, byte_identical=true. Input pass-001 retained. pass-002 is an additional byte-identical unchanged-input checkpoint accidentally generated after a rejected missing-id annotation; it is preserved as history, not claimed as round02 output. Annotation id and duplicate-label validation errors were atomic and corrected before accepted state save; no toolkit fault inferred.

Evidence: round02/review-notes.md, data-independent-review.json, code-independent-decodes.json and consumer-independent-decodes.json; primary redirect/round01-contracts-20261031 contracts preserved. All proposed Static bytes accepted; natural reachability remains separate. R1-DATA-CONSUMERS and R1-CALL-CONTEXT verified for the supported batch. Cached supported closures are already at their bounded fixed point; residual bank/return callbacks and mixed intervals need refreshed-source redirect research. No unchanged gameplay replay: four artificial episode gameplay routes and actual terminal THE END artifacts reused; distinct boss initialization/event contract remains unavailable. See scenarios.json and gameplay.md.

NES-DOC-001 workaround retained; NES-MAP-002 remains one-byte representation blocker at C0BF. See [toolkit-issues.md](toolkit-issues.md). No toolkit changes. Input ROM unchanged. Existing commentary/source/review history preserved; all-current commentary is now pending for newly accepted classifications, followed by redirect. No archive/publication/cleanup performed.

Round02 redirect active: analysis/redirect/round02-residual-20261031/frontier.md; pass-003 unchanged, R2-RESIDUAL-CONTRACTS underway.

## Round02 redirect complete — ready combined batch

Input/output accepted checkpoint pass-003 unchanged, map/state2cc2c92eced0f958404e2df40d3c392f7f596d10c6ce2f7ac4d2f1de84ce40bb. Current classified84.54% (110826/131088),20262 unknown/213 spans; Code17613/Data93213. Full-current readability credit preserved for unchanged accepted source; editorial_audit pending.

Produced R2-RESIDUAL-CONTRACTS14237 newCode+5097 Data=19334 unknown-byte proposals and R2-AC71-DATA-CORRECTION123 acceptedData->Code repair (zero classified gain). R1-DATA-CONSUMERS verified history retained with localized correction pending. If all accepted, research residual928/44 spans; no current-map completion claim. Fresh supported-tool boundary/context audit6313 physical instructions/6606 contextual/211 groups, zero conflicts; prior misaligned868F root corrected869F, counterexample retained. No map/state/source/ROM/toolkit changes.

Handoff: analysis/redirect/round02-residual-20261031/handoff.md. Primary contract review, proposal audit, preservation hashes and exact ledger in same attempt. Corrected nativeAC71 explicit bank5 caller overrides only123 bytes of prior broadData claim. Mapper primary confirms discovery truncation at fixed indexed mapper store; decoder successor exists. NES-DISC-003 local issue recorded, no toolkit edit.

Task/actor/interaction/scene/sound connected graphs and independent data avenues investigated; remaining local entry/stack/bank barriers in contract-review/ledger are deferrals for this ready batch, not terminal exhaustion. Four saved episode routes/THE END retained, no unchanged runtime replay. Ready for separate round03 disassembly worker; redirect relinquishes writes/processes and does not execute next phase.

Round03 disassembly active; durable frontier analysis/round03/frontier.md. Input pass-003 preserved; independent combined contract and exact AC71 correction review underway.


## Round03 completed — disassembly pass, incomplete ROM

ROM classification: 99.29% (130160 / 131088 bytes classified; 928 bytes unknown).

Independent primary contract/consumer review and222 fresh bounded decode groups accepted14237 new StaticCode bytes and5097 StaticData bytes. Corrected123 existingData bytes at93313..93436 toCode using explicit selector5/E514 FEFB/E519 AC71 native caller and62-instruction leaf endingRTS ACEB; seven-byte records/four actor slots. Split only formerData3910; surrounding resources and ACEC directory remainData with prior evidence. Zero classification gain from correction. UniqueCode31973, Data98187; header16 included. Exact44-interval complement round03/unknown.json; metrics round03/metrics.json.

| Physical PRG8KiB bank | Code | Data | Classified | Unknown |
| --- | ---: | ---: | ---: | ---: |
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 0 | 8192 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 0 | 8192 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 0 | 8192 | 8192 | 0 |
| 6 | 1776 | 6263 | 8039 | 153 |
| 7 | 0 | 8192 | 8192 | 0 |
| 8 | 744 | 7448 | 8192 | 0 |
| 9 | 6906 | 1112 | 8018 | 174 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 1013 | 7179 | 8192 | 0 |
| 12 | 6594 | 1590 | 8184 | 8 |
| 13 | 3566 | 4626 | 8192 | 0 |
| 14 | 5287 | 2555 | 7842 | 350 |
| 15 | 6087 | 1862 | 7949 | 243 |

Fresh maps/pass-004.json, generated/pass-004 and analysis/pass-004-verification.json confirm valid and byte_identical=true; analysis/state.json and maps/current.json current. ROM unchanged. Input pass-003/history preserved. Independent evidence round03/review-notes.md, code-independent-decodes.json, consumer-independent-decodes.json, correction-caller.json and data-independent-review.json; primary redirect/round02-residual-20261031 contracts/ledger retained. Both ready R2 leads verified for supported batch; structuralStatic, no new natural-reachability/execution claim.

Remaining928 bytes need localized entry, matched stack/callback or bank-effect evidence. C0BF retains NES-MAP-002 one-byte overlap limitation/IRQ evidence, noData workaround. Reuse prior four artificial episode routes and native THE END; no unchanged replay or arbitrary bossRAM sweep. Natural boss initialization/victory remains unavailable. Strong ready connected batch exhausted; refreshed-source commentary/redirect may resolve precise residual conditions. This is an easy-gain handoff, not terminal exhaustion.

Toolkit report [toolkit-issues.md](toolkit-issues.md) retains NES-DOC-001, NES-MAP-002 and NES-DISC-003; no new issue or toolkit edit. Existing maintained commentary/source/review/history preserved. Full-current commentary must reconcile pass-004 additions and localized AC71 broadData correction; final editorial remains pending. No ROM mutation, archive, publication, cleanup or deletion.


## Round03 redirect complete — ready localized structural batch

Input remains pass-004,99.29% (130160/131088;928 unknown/44 spans), full-current commentary complete, final editorial pending. R3-LOCAL-STRUCTURAL-CONTRACTS proposes832 StaticCode+56 StaticData (888unique unknown bytes); no classifications accepted/applied. Fresh386-instruction exact decode/boundary audit has zero errors. Hypothetical subtraction40unknown/13spans is research only. Sound stack/partial-return, native task scan, matched bank/stack cleanups, coherent actor/display helpers and native inventory/scene data were reviewed together; no isolated opcode promotion. Primary evidence/per-interval constraints and all44 dispositions at redirect/round03-local-contracts-20261031/{contract-review.md,unknown-ledger.json,proposal-audit.json,handoff.md}. ROM/map/state/source hashes preserved; no replay, toolkit mutation, publication or next phase. Existing NES-MAP-002 overlappingC0BF and NES-DISC-003 fixed-window discovery distinction retained. Worker relinquishes after package check; next phase independent disassembly of combined lead, then current commentary. Not terminal exhaustion.


## Round04 completed — disassembly pass, incomplete ROM

ROM classification: 99.97% (131048 / 131088 bytes classified; 40 bytes unknown).

Accepted832 StaticCode+56 StaticData bytes after independent primary per-interval structural/partial-return/stack/consumer review,386 fresh proposal instruction decodes,25 native context groups and5 supplemental boundary/interface groups. Code32805/Data98243 including16-byte header. Exact13-span complement round04/unknown.json; per-bank metrics round04/metrics.json.

| Physical PRG8KiB bank | Code | Data | Classified | Unknown |
| --- | ---: | ---: | ---: | ---: |
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 0 | 8192 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 0 | 8192 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 0 | 8192 | 8192 | 0 |
| 6 | 1929 | 6263 | 8192 | 0 |
| 7 | 0 | 8192 | 8192 | 0 |
| 8 | 744 | 7448 | 8192 | 0 |
| 9 | 7063 | 1128 | 8191 | 1 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 1013 | 7179 | 8192 | 0 |
| 12 | 6602 | 1590 | 8192 | 0 |
| 13 | 3566 | 4626 | 8192 | 0 |
| 14 | 5588 | 2595 | 8183 | 9 |
| 15 | 6300 | 1862 | 8162 | 30 |

Current state/map valid; fresh pass-006 rebuild byte-identical. Input pass-004 preserved. pass-005 is an additional byte-identical unchanged-input checkpoint accidentally generated after first annotation was atomically rejected for missing view cpu; corrected main CPU/uploaded fields applied before any accepted map save. Historical pass-005 retained, no toolkit bug inferred. Evidence round04/review-notes.md, code-independent-decodes.json, consumer-independent-decodes.json, supplemental-evidence.json and data-independent-review.json. Primary redirect/round03-local-contracts-20261031 retained. R3-LOCAL-STRUCTURAL-CONTRACTS verified; Static structure, no new observed/natural execution claims. Existing AC71 correction/source/history untouched.

Remaining40bytes/13spans need localized entry/return/field-interface or mixed-data discrimination; C0BF remains NES-MAP-002 representation issue, noData workaround. This is an easy-gain handoff, not terminal exhaustion. No obvious connected supported continuation reserved. Reused four artificial episode/native THE END evidence; no unchanged gameplay replay. Natural boss initialization/victory remains unresolved. Toolkit [toolkit-issues.md](toolkit-issues.md) preserves NES-DOC-001/NES-MAP-002/NES-DISC-003; no new issue or edits.

Full-current commentary pending for888 added bytes; prior maintained source/review/history preserved. Final editorial pending. No ROM/toolkit mutation, archive/publication, cleanup/deletion or next phase.

## Round04 commentary complete — pass-006

Current-source readability COMPLETE; no deferred ranges. Reconciled832 new StaticCode+56Data bytes with all valid pass-004 physical review credit. Full32,805 Code/98,243Data,15,096 instructions (14,710 carried+386new),9,298 scaffold records and549 semantic units (all510 prior+39new) reviewed. Fresh official baseline/map/state matches, byte-identical verification and reading-after refresh succeeded; full physical/semantic/preservation gate analysis/round04/commentary-scope-check.json. Prior entire commentary retained at round04/commentary-original/ and commentary-working-prior/; AC71 correction remains intact. pass-006 authoritative; pass-005 retained unchanged-input history.

Source-level additions cover sound envelope/partial RTS, scheduler scan, player/follower/interactions, actor/display helpers, saved-stack/fixed continuations and inventory/scene tables. Full consistency corrected exact frame/carry/controller mechanics and two stale summaries. All40unknown bytes/13spans have honest mechanical notes, C0BF overlapping IRQ representation preserved without Data workaround. ROM remains99.97% classified (131048/131088); final editorial audit pending. No gameplay replay; four artificial episodes/native THE END reused, natural victory unresolved. Toolkit report retains existing NES-DOC-001/NES-MAP-002/NES-DISC-003, no new issue or commentary blocker. No ROM/map/state/toolkit mutation, audit/archive/publication/cleanup/deletion or next phase. Compact round04/commentary-handoff.md; commentary relinquishes writes/processes.

## Round04 redirect complete — pass-006 unchanged

R4-RESIDUAL-INTERFACES ready combined22 StaticCode/0Data proposed, accepted0: D6E3 pose A argument2, DAF1 scene-index1 ordinary return1, DCF3 valid-password ordinary return1, F6E9 timer interface5, independently reviewable E762 scratch-pair arithmetic13. Exact prior premise corrections, native field/stack/bank contracts and scoped retained rejection critique at redirect/round04-residual-contracts-20261031/contract-review.md; fresh configured-binary primary/proposal/password decodes and14instruction exact-boundary audit. No annotation or runtime replay; pass-006 map/state/source/ROM hashes unchanged. Current99.97%131048/131088 classified40unknown/13spans. Hypothetical after acceptance18unknown/8spans only. Lead/history and all residual dispositions persisted. This is ready disassembly handoff, not terminal exhaustion; C0BF NES-MAP-002 external representation separately retained, noData workaround. Full-current commentary gate remains complete on unchanged input; final editorial pending. Handoff redirect/round04-residual-contracts-20261031/handoff.md; redirect relinquishes writes/processes. No next phase/archive/publication/cleanup/deletion or toolkit mutation.


## Round05 completed — disassembly pass, incomplete ROM

ROM classification: 99.99% (131070 / 131088 bytes classified; 18 bytes unknown).

Accepted22 StaticCode/0Data bytes: D6E3 argument2, F6E9 timer5, DAF1 scene return1, DCF3 password return1, separately accepted E762 mechanical scratch-pair transform13. Independent review round05/review-notes.md,14 fresh instruction decodes and12 native context groups. E762 does not claim arbitrary multiply20 or original input relation. No new observed/natural-use claim. Code32827/Data98243 including16-byte header.

| Physical PRG8KiB bank | Code | Data | Classified | Unknown |
| --- | ---: | ---: | ---: | ---: |
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 0 | 8192 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 0 | 8192 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 0 | 8192 | 8192 | 0 |
| 6 | 1929 | 6263 | 8192 | 0 |
| 7 | 0 | 8192 | 8192 | 0 |
| 8 | 744 | 7448 | 8192 | 0 |
| 9 | 7063 | 1128 | 8191 | 1 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 1013 | 7179 | 8192 | 0 |
| 12 | 6602 | 1590 | 8192 | 0 |
| 13 | 3566 | 4626 | 8192 | 0 |
| 14 | 5592 | 2595 | 8187 | 5 |
| 15 | 6318 | 1862 | 8180 | 12 |

Exact complement round05/unknown.json: isolated RTS79830,117611,122893,128500,129494; C09C114860+2; overlapping C0BF114895+1; mixed pre-vector131072+10. No speculative tinyData. State/map validated, fresh pass-007 byte-identical, mapSHA256 92c25b1185f7cc9103a90fa5bcf8b8a47b604f12747dde478becf6c028d0e2b0. Current-source maintained commentary and historical evidence untouched; full-current reconciliation required for22 added bytes, final editorial pending. R4-RESIDUAL-INTERFACES verified; all supported connected additions accepted. No productive continuation deliberately reserved. Residual same-field/stack/format questions need materially new discrimination; not terminal exhaustion. Reused broad four artificial episode/native THE END evidence, no unchanged replay; natural victory unresolved.

Toolkit [toolkit-issues.md](toolkit-issues.md): existing confirmed documentation issue NES-DOC-001, overlap representation feature NES-MAP-002, conservative discovery limitation NES-DISC-003 preserved. No new issue or toolkit edit. C0BF remains representation blocker/noData workaround. No ROM mutation, archive/publication/cleanup/deletion or next phase.


## Round05 redirect active — pass-007

Current-source readability handoff accepted. Unique attempt redirect/round05-residual-rejection-20261031; exact input hashes/frontier persisted. Reviewing remaining18bytes/eight spans against current primary source and decisive prior artifacts; no replay of accepted22byte batch. Research17byte ambiguity and external overlap1byte remain separate.

## Round05 redirect complete — pass-007 unchanged, ready2Code

R5-MARKER-CONSTANT-ENTRY proposes physical114860+2 CPUC09C A9BD StaticCode fixedbank14; accepted0. Coordinator primary critique exposed an unnecessary exactBD default-use premise: literal argument plus independently established C09E A-to-OAM-Y contract, four marker stores, ordinary return and independent boundaries support mechanical constant189 wrapper. Fresh configured-binary decode and exact native caller/body contracts retained at redirect/round05-residual-rejection-20261031/marker-correction.md and marker-fresh-decodes.json. Interim rejection-review.md preserved and locally superseded forC09C; remaining exact residual/strongest-rival/narrowData review retained, no terminal success claim. Current99.99%131070/131088,18unknown/eight spans unchanged; hypothetical after acceptance16unknown/seven spans. Research ambiguity17bytes now includes2proposed; external C0BF1byte NES-MAP-002 separate. No unchanged gameplay, ROM/map/state/source/toolkit mutation, archive/publication/cleanup/deletion or next-phase work. Full-current commentary complete; final editorial pending. Compact handoff and preservation-check saved; redirect relinquishes all writes/processes.


## Round06 completed — disassembly pass, incomplete ROM

ROM classification: 99.99% (131072 / 131088 bytes classified; 16 bytes unknown).

Accepted2 StaticCode/0Data bytes, physical114860+2 CPUC09C A9BD, InstallSpriteZeroMarkerAt189, context[null,null,14,15]. R5-MARKER-CONSTANT-ENTRY verified by independent primary decode/native caller/complete body/bounds review: round06/review-notes.md and primary-decodes.json. Narrow mechanical constant189 supplier to native A-to-OAM Y interface; no natural caller, scene-default or timing claim. Prior exactBD default prerequisite corrected; preserved interim rejection superseded on that premise only. Existing unreferenced shared-body wrappers apply the same structural standard. Code32829/Data98243 including16-byte header.

| Physical PRG8KiB bank | Code | Data | Classified | Unknown |
| --- | ---: | ---: | ---: | ---: |
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 0 | 8192 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 0 | 8192 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 0 | 8192 | 8192 | 0 |
| 6 | 1929 | 6263 | 8192 | 0 |
| 7 | 0 | 8192 | 8192 | 0 |
| 8 | 744 | 7448 | 8192 | 0 |
| 9 | 7063 | 1128 | 8191 | 1 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 1013 | 7179 | 8192 | 0 |
| 12 | 6602 | 1590 | 8192 | 0 |
| 13 | 3566 | 4626 | 8192 | 0 |
| 14 | 5594 | 2595 | 8189 | 3 |
| 15 | 6318 | 1862 | 8180 | 12 |

Exact complement round06/unknown.json: isolated RTS79830,117611,122893,128500,129494; mixedFFF0 physical131072+10; C0BF114895+1.15research-ambiguous bytes plus1external representation blocker. No automatic terminal determination from this gain. Connected native body/callers already classified; no supported continuation withheld. Fresh pass-008 valid/byte-identical; mapSHA256 eadc61fc669a5a918f0479c7062b2c86858b279a25d6eeed77f17aae941890e3. Input ROM unchanged; maintained source/review/history preserved. Next phase must reconcile current commentary for one new instruction/two bytes and corrected stale C09C uncertainty; final editorial pending.

Reused prior four artificial episodes/native THE END evidence and existing scenarios; no unchanged gameplay replay or new reachability claim. Natural boss initializer/victory unresolved. Residual scoped review remains in redirect/round05-residual-rejection-20261031/rejection-review.md with marker-correction.md; no same-method retry. Toolkit [toolkit-issues.md](toolkit-issues.md): NES-DOC-001, NES-MAP-002 and NES-DISC-003 retained, no new issue or edits. Preserve C0BF ordinary LDA/IRQ SLO overlap, noData workaround. No ROM/toolkit mutation, archive/publication/cleanup/deletion or next phase.


## Round06 current-source readability

Complete on pass-008:32,829Code/98,243Data bytes,15,111instructions,9,298physical scaffold records and555semantic units. Preserved all554prior units and15,110prior instruction renderings; entire prior source outside the two-byte C09C replacement is byte-for-byte intact. Full prior commentary/review/history retained in analysis/round06/commentary-original/ and commentary-working-prior/. Official fresh baseline matches map/state. No deferred readability range.

InstallSpriteZeroMarkerAt189 supplies the explicit complete A-to-OAM-Y argument to the native shared marker body; sprite_zero_marker_y_189 names the literal189. Explained OAM initialization, fixed-bank context, ordinary return and unknown natural callers/scene timing. Corrected the old scene-default prerequisite while preserving historical records. Full inventory, semantic coverage, source links, prior review credit, native body/callers and all seven unknown notes checked.16unknown bytes remain five isolatedRTS, mixedFFF0ten and C0BFone; noData workaround or IRQ removal.

Fresh official verification passes byte-identical; reading-after.asm refreshed. Scope evidence: analysis/round06/commentary-scope-check.json. ROM classification remains99.99% (131072/131088 classified;16unknown). Final editorial audit pending, not run. No gameplay replay; four artificial episodes/native THE END retained, natural victory unresolved. Existing NES-DOC-001/NES-MAP-002/NES-DISC-003 retained, no new toolkit issue or commentary blocker. No ROM/map/state/toolkit mutation, terminal-exhaustion inference, archive/publication/cleanup/deletion or next-phase work.

## Round06 redirect residual review

Completed scoped primary-evidence reassessment on pass-008 after full-current readability handoff; no proposal,16unknown/seven spans unchanged (15research ambiguous+1external NES-MAP-002). Current map-derived ledger and immutable ROM/current source neighbors at analysis/redirect/round06-residual-review-20261031/. Rejection review challenges isolated return/no-op, genericData, opaque footer and narrow zero pairs without repeating superseded C09C default premise. No surviving ready inference/test; independent coordinator critique required before terminal incomplete acceptance. Leads pass records empty new/updated sets. Four artificial episodes/native THE END reused; natural boss/victory contract unresolved, no unsupported RAM sweep. Existing toolkit issues retained, no new issue. Input/map/state/source preserved; no running process. Final editorial/audit pending.

## Terminal coordinator handoff — justified incomplete

Orchestration reached documented terminal incomplete after independent primary
rejection critique (terminal-rejection-review.md). ROM classification remains99.99%
(131072/131088;16unknown/seven spans),15research ambiguous+1externally representation-
blocked byte; no full-ROM success or blocked-work resolution claimed. Current pass-008
state/map retained and final configured map/store validation returned ok=true.
Final editorial acceptance complete: commentary/audit.json ok/byte_identical/
editorial_audit true, refreshed reading and final hashes in final-editorial-result.json.
Full terminal result/resume conditions: terminal-handoff.md. All prior phase/correction
history retained, no live process or future job. archive-options.json selects final
Markdown and explicitly omits only duplicate instruction scaffolds; final accounting
and requirement audit are archive-document-selection.json/final-completion-audit.json.
No toolkit edits, archive execution, publication, deletion or cleanup performed.
