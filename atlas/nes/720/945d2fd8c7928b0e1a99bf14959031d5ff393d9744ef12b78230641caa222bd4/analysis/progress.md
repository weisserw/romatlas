# Disassembly progress

Workspace initialized. Baseline and classification have not run.

## Round 001 disassembly in progress
Baseline established with `python3 run.py inspect` and `baseline`; generated/baseline byte-identical, map SHA256 f5d31c311797e45b732ffd2647005cc8373433c79df08ac3d4c474f7d8ec99d4.
ROM identified visually as 720° (NES, Mindscape/Tengen 1989). Mapper 1, 16 physical 8 KiB PRG banks, CHR RAM, total 131088 bytes. Startup 180 frames replayed in six bounded 30-frame calls; analysis/round-001/startup.snapshot and startup.ppm preserve result. Trace summary reports 924 distinct physical instruction starts, 1009 contextual roots, no aggregate drops. Capacity zero intentionally drops all detail; aggregates retained. No classifications applied yet; startup-discovery with empty entrypoints yields no candidates (observed roots must be supplied explicitly). Validated vectors $C372/$C000/$E898 in fixed banks 14/15; static-discovery.json contains 47 instructions, mapper/call barriers limit propagation. Runtime menu/start exploration underway via play.py, saving local snapshots, frames, RAM and contextual roots. First 180-frame single-call attempt hit 30-second timeout; bounded 30-frame batches succeed. No toolkit issue established by timeout.

Interim annotation batch: 2,468 / 131,088 bytes classified (1.88%); current internal state saved/exported. pass-001-interim checkpoint verifies prior 1,335-byte map byte-identical; second batch is map-validated pending final checkpoint. Observed roots recovered from fresh reset30 experiment, menu and copyright waiting. Static connected discovery preserves barriers. Runtime copyright screen persists through 634 frames; one-frame Start/A edges have not advanced it, one sustained-press trial underway. Dispatcher and RAM contract inspection replaces indefinite button replay. Banks 2–11 have strong graphics/data patterns, now undergoing bounded role/boundary inspection.

Mid checkpoint pass-001-mid verified byte-identical; internal/export map SHA256 61154747d9466596efd57017d15b6b3db7a92596d8b2dd238c70afe76ecc85f4. ROM classification: 65.43% (85768 / 131088 bytes classified; 45320 bytes unknown). Banks 2–11 contain coherent repeated graphics/map payloads; tile-overview.png shows silhouettes, scenery and tile patterns, and data-review.json retains boundaries/block summaries. Generic Data avoids asserting complete schemas for mixed or encoded payloads. Copyright resolved with sustained buttons; title screen seen in held-3.ppm. Later milestone routes still in progress. Main dispatcher inspection identifies class $0207 and selection $020F/$0210, demo flag $0211; these are hypotheses for controlled progression bypass, not visited-gameplay claims.

Correction TI-001: all input requests before realstart used the shared protocol's button/pressed form, which NES silently ignores. They were NO-INPUT runs; copyright/title transition was natural timer, not sustained button success. Their actual fetch evidence remains valid natural attract/title execution. Source confirms NES requires `{port:1,buttons:[...]}`. Fixed replay driver; realstart is first valid button route. See analysis/toolkit-issues.md. No arbitrary state patches yet.

Static expansion and audio batch: 109,776 classified bytes / 131,088 (83.74%), 21,312 unknown. Audio data boundaries anchored by observed driver entry CPU $8890 and final RTS $8B55 in [12,13,14,15], consumed instrument/pointer/mask/pitch/command payloads, and repeated stream records. Generic Data labels Audio_data_prefix / Audio_data_streams preserve unknown stream subdivisions; sound-listing.json and sound-tail-listing.json retained. Snapshot branches now export contexts and snapshots per 30-frame event chunk. Ramp is visibly initialized after intro (rampplay-0.ppm.png) and mechanics run underway. End-of-pass verification pending expanded gameplay code. All artificial event patches in eventN-recipe.json; no ROM edits.


## Round 001 disassembly finished — incomplete ROM
ROM classification: 91.64% (120134 / 131088 bytes classified; 10954 bytes unknown).

Fresh `pass-001` checkpoint verifies byte-identical assembly, map SHA256 44848ce7a8323b7fcffbc8cb9542913b0d6a37253c6c8ef2f4bf7e9e48890e4b; internal state, current export and checkpoint map match. See analysis/pass-001-verification.json and generated/pass-001/game.asm. Input hash remains unchanged. 14,761 Code bytes, 105,373 Data bytes, 643 evidence-bearing regions; header16 Data bytes outside PRG banks. Unknown complement126 intervals in analysis/round-001/unknown-final.json.

| Bank | Code | Data | Unknown |
|---|---:|---:|---:|
| 0 | 751 | 7043 | 398 |
| 1 | 3059 | 714 | 4419 |
| 2 | 0 | 8192 | 0 |
| 3 | 0 | 8192 | 0 |
| 4 | 0 | 8192 | 0 |
| 5 | 0 | 8192 | 0 |
| 6 | 0 | 8192 | 0 |
| 7 | 0 | 8192 | 0 |
| 8 | 0 | 8192 | 0 |
| 9 | 0 | 8192 | 0 |
| 10 | 0 | 8192 | 0 |
| 11 | 0 | 8192 | 0 |
| 12 | 710 | 7482 | 0 |
| 13 | 0 | 8192 | 0 |
| 14 | 5910 | 0 | 2282 |
| 15 | 4331 | 6 | 3855 |

Natural Class1/Main Park visited with corrected full buttons array. All four event modes and Class4 Ramp initialized and sampled under recorded artificial entry contracts; TOP TEN failure-path terminal screen visited artificially. Event results/medals, shops and distinct ending remain deferred with concrete missing contracts in scenarios.json and leads L005–L007. L001–L004 verified. Detailed handoff: analysis/handoffs/round-001-disassembly.md. Static/data evidence and exact replay artifacts remain local in round-001. Natural fetch starts4,868, artificial5,180, union5,868 (overlapping); execution coverage is separate from structural classification. No aggregate drops in retained scenario summaries; detail capacity0 intentionally drops event detail. Several SIGTERM experiments preserve partial evidence and resumable per-action snapshots; no running experiment required at handoff.

Toolkit summary: one confirmed documentation/API-validation mismatch TI-001 (emulator.input old schema silently ignored); corrected workaround in driver, no blocker, no toolkit edits. See analysis/toolkit-issues.md. Static easy gains and representative broad progression complete for this pass; mixed code/inline data and progression contracts go to redirect after commentary. No terminal exhaustion claim; no full-ROM/editorial/hardware compatibility claim.

## Round 001 commentary finished — current-source readability complete
Reviewed all currently classified source from verified pass-001 without changing structural maps: 14,761 Code bytes, 105,373 Data bytes; 10,954 bytes remain Unknown. Local commentary/source/game.asm and symbols.inc explain 315 meaningful routine/control entries, six-slot object fields, RAM/hardware/ROM-table symbols, branch flow, audio streams, PPU resource formats, NMI/VRAM/OAM/CHR scheduling, HUD, ramp/player motion, map streaming/collision and high scores. Every classified data block has accepted role/bounds evidence nearby; all 126 unknown gaps carry physical boundary/uncertainty notes. Generic data semantics and exact trick/terrain/state identities remain explicit uncertainties. See commentary/progress.md and uncertainties.md.

Coverage/accounting: analysis/commentary/round-001-coverage.json (769 scaffold entries reviewed, 715 unit fragments, 315 unique units). Fresh commentary verification is byte-identical, reading-after.asm refreshed; analysis/commentary/round-001-verification.json. Internal/current/commentary frozen maps retain SHA256 44848ce7a8323b7fcffbc8cb9542913b0d6a37253c6c8ef2f4bf7e9e48890e4b and input hash unchanged. No readability range deferred; final editorial acceptance stays pending and whole-ROM classification remains incomplete. No final audit run. Concrete redirect questions preserve L005 mixed high-bank code/data, L006 results/medals/class progression, L007 shop/world interaction contracts. Toolkit TI-001 remains the single confirmed issue, workaround known, no blocker; no new toolkit issues found in commentary. No toolkit/ROM/archive/publication changes. Writer done; coordinator review and redirect next.

## Round 001 redirect ongoing — r01
Unique research analysis/redirect/round-001-r01/status.md records configured-binary static decoding, result branch/table contracts and mixed object-script/native callback frontier. Classification remains pass-001 (10954 unknown); no map/source edits. L006 result graph and L007 special-interaction contract are resolving together with L005 interpreter grammar. Productive research continues; no phase completion/exhaustion claim. No new toolkit issues found so far (TI-001 workaround remains).

## Round001 redirect finished — combined ready batch, accepted ROM incomplete
Completed connected L005 script/native, L006 result/class and L007 map/purchase investigation, then concrete independent directconsumer/returncontinuation avenues L008. Research: analysis/redirect/round-001-r01/; phase handoff analysis/handoffs/round-001-redirect.md. proposals.json contains10942 distinct currentlyunknown bytes (4078Code,6864Data),166 typed fragments; initial7017proposal preserved. These are Static proposals awaiting independent disassembly, not accepted annotations. Acceptedmap remains120134/131088 classified (91.64%),10954unknown. Exact baseline complement126intervals with neighbors/bankcontext/split dispositions in unknown-ledger.json. If all proposals accepted,12RTS-only bytes remain unsupported; no terminal exhaustion/ROMcomplete claim.

Confirmed artificialmilestones:GoldMedal/CashPrize55/NoBonus/medalmap; parkinformationmapfourevents/fourequipmentlocations; purchaseconsumercash100->75/equipment0->1/availabilityFF->0; unacceleratedclass4ClassFinished. OverlayexitREADY andblackloading preserve unprovedparkreturn. ClassrunSIGTERM after1740savedframes, nofinalaggregate/snapshot;1530frameClassFinished primaryframe retained. No unchanged replay required. Leads/scenarios updated; oldshop hypothesis corrected toinformationmap, actualpurchase script14->BE90. Cash account misnamedTicketDigits/CompareTicketCost in currentcommentary queued for nextreadability pass. All known finite script/callback and concrete directreference opportunities followed; no forced first-ready stop.

Read-only integrity: inputSHA945d2fd8c7928b0e1a99bf14959031d5ff393d9744ef12b78230641caa222bd4; currentmapSHA44848ce7a8323b7fcffbc8cb9542913b0d6a37253c6c8ef2f4bf7e9e48890e4b; sourceSHA971b5f2b5547020aaa126a936e6706bb61634a993c674fc80e4ea70d42e5c249; symbolsSHAbd625a3b8cc5e6814703d275859ad86983d66b13234e32584d8b5e4a4d66bb47 unchanged. No newstructural verification/checkpoint created because redirect never changedmap/source; existing byte-identical pass001/commentaryverification remains baseline, finaleditorialacceptancepending. Toolkit: no newtoolkitissuesfound; TI001confirmed schema mismatch/workaroundknown/no blocker, analysis/toolkit-issues.md. No toolkit/ROM/map/state/source/archive/cleanup/publication changes. Phasecomplete readyforindependentdisassembly; runtimeinterruption separately preserved.


## Round 002 disassembly finished — incomplete ROM
ROM classification: 99.99% (131076 / 131088 bytes classified; 12 bytes unknown).

Independently accepted L005–L008 combined batch: 4,078 Code + 6,864 Data = 10,942 formerly unknown bytes, in 174 role/context-preserving annotations. Total Code18,839/Data112,237, 817/817 evidence-bearing regions (header16 Data outside PRG). Fresh native decoding1,900 instructions, independent771 script widths/bytes, no script/native overlap or unsupported direct successor. Primary interpreter/stack/consumer/bank review: analysis/round-002/review.md; native-review.json, pattern-review.json, accepted.json, accounting.json preserve checks. Static structural acceptance makes no natural-use claim.

pass-002 fresh checkpoint map validates and generated assembly rebuilds byte-identically; analysis/pass-002-verification.json. State/current/checkpoint map SHA256 c87510db28caf769ac3674f40e7628477fd591c3a21ac9e5c6ee3a8c66d642e4; input SHA unchanged. Exact physical accounting corrects redirect handoff bank totals: first three residual RTS are physical bank0 at7886..7888 (CPU9EBE..9EC0), not bank1. A native instruction crossing9FFF/A000 contributes bytes to both physical banks. Use per-byte counts below.

| Bank | Code | Data | Unknown |
|---|---:|---:|---:|
| 0 | 1071 | 7118 | 3 |
| 1 | 4577 | 3615 | 0 |
| 2 | 0 | 8192 | 0 |
| 3 | 0 | 8192 | 0 |
| 4 | 0 | 8192 | 0 |
| 5 | 0 | 8192 | 0 |
| 6 | 0 | 8192 | 0 |
| 7 | 0 | 8192 | 0 |
| 8 | 0 | 8192 | 0 |
| 9 | 0 | 8192 | 0 |
| 10 | 0 | 8192 | 0 |
| 11 | 0 | 8192 | 0 |
| 12 | 710 | 7482 | 0 |
| 13 | 0 | 8192 | 0 |
| 14 | 7168 | 1018 | 6 |
| 15 | 5313 | 2876 | 3 |

Twelve Unknown bytes in eight intervals: bank0 CPU9EBE..9EC0; bank14 C2DD,CE88,CEAB,CEB8,CECE,D0F7; bank15 EFBE..EFC0. They are bare unreferenced RTS values; no forced code/data inference. Neighboring classifications do not require them. Exact offsets/lengths in accounting.json. This is an ordinary incomplete pass handoff, not terminal exhaustion.

Broad gameplay review reused existing four-event/class4/result/medal/information-map/purchase/ClassFinished artifacts and scenarios.json. No changed progression contract or boundary question justified unchanged replay. Park reappearance and a distinct ending remain unproved. Execution coverage is unchanged by this pass: no new emulator fetches; prior disassembly natural starts4868/artificial5180/union5868 remain historical, redirect artificial fetches separately retained in its contexts. Structural classification is independent of execution coverage.

Current-source commentary now requires full pass-002 review, including 10,942 added bytes and corrected cash/TicketDigits/CompareTicketCost names. Prior commentary/source/history preserved. No final editorial audit or ROM-complete claim. After commentary, redirect reassesses exact residuals and performs evidence-policy rejection review before any terminal decision. No new toolkit issues found; existing TI-001 confirmed input-schema mismatch/workaround known/no blocker (analysis/toolkit-issues.md). No toolkit edits, archive execution, deletion, publication, or ROM changes. Disk11GiB free; scratch4KiB before bounded review.

## Round002 redirect terminal assessment — incomplete, review required
Research-only analysis/redirect/round-002-r01/review.md evaluates exact12-byte/eight-interval residual using fresh configured-release neighbor decodes, accepted script/stack contracts, full735-Code-view/8664-instruction reference-successor check (zero hits/diagnostics), and localized competing code/data claims. No ready proposal, no classification/source changes; pass-002/current/state SHA remainsc87510db28caf769ac3674f40e7628477fd591c3a21ac9e5c6ee3a8c66d642e4. Exact ledger and compact handoff analysis/handoffs/round-002-redirect.md. Generic data fallback considered explicitly; return-encoding versus padding ambiguity persists. Coordinator must inspect primary-linked review before terminal acceptance. Broad milestone attempts preserved, natural park return/distinct ending unproved, no unchanged runtime replay. leads.json appended empty pass result. No new toolkit issues found; existingTI001 workaround known. Final editorial acceptance pending; no complete-ROM/archive claim.

## Coordinator terminal handoff — orchestration complete, ROM incomplete
ROM classification:99.99% (131076/131088 classified;12unknown). Final pass002 verified byte-identical; exact primary reviewed rejection reasoning accepted, no useful revised method remains. Full current-source commentary and final editorial audit accepted825regions/1008units with fresh byte identity/refreshed reading. See analysis/final-handoff.md, analysis/current-audit.json, commentary/editorial-audit.md and commentary/audit.json. Archive selection finalized/checked separately; no archive execution or cleanup. Classification is INCOMPLETE, not blocked; resumable retry conditions recorded.
