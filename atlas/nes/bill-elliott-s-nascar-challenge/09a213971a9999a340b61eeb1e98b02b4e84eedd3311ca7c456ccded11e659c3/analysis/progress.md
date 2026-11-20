# Current terminal status

**Whole-ROM complete terminal handoff verified.** ROM classification:100.00% (393232 / 393232 bytes classified;0bytes unknown). Final checkpoint pass-004; current-source readability, final editorial audit and byte-identical verification all pass. Separate final redirect and focused editorial-feedback closure accepted. No required phase/frontier remains active; the records below are chronological history.

Current deliverables/evidence: [final handoff](final-handoff.md), [requirement audit](completion-audit.json), [final evidence](final-evidence.json), [document inventory](archive-document-inventory.json). Archive-options.json explicitly selects additional final Markdown and omits only inspected disposable drafts/duplicate instruction templates. No archive execution, publication or cleanup performed. Semantic/natural-reachability/hardware limits remain documented separately.

# Preserved chronological progress

# Disassembly progress

Round 01 disassembly in progress. Baseline established and byte-identical verified: analysis/baseline-verification.json. Required workflow, evidence policy and NES/protocol docs read. Bounded startup trace request in progress; RPC driver in analysis/round-01/driver.py, experiment analysis/round-01/probe.py. No classifications accepted yet; startup/static discovery and gameplay remain pending. No terminal stopping assessment made.

Baseline/startup frontier advanced: rendered legal notice identifies Bill Elliott NASCAR Challenge (Konami 1991). 30-frame startup and title snapshots persisted; 150 additional no-input frames recorded (title-step.json). Vector roots NMI E1AE/reset E000/IRQ E3B4 inspected, 149 conservative static instruction candidates accepted. Header, interrupt vector words and physically separate CHR pattern ROM classified. Current classified 262489/393232 bytes; phase ongoing, percentage will be recomputed at checkpoint. Menu input experiment menu.py currently pressing/releasing Start/A with bounded frame steps. Broad racing progression remains pending; no terminal stopping claim.

Additional accepted batch: 12 generic non-code uniform intervals (2064 bytes; pattern-evidence.json); bank-aware coherent code from 1082 startup/intro fetch contexts (35 runs, 2603 bytes; menu-discovery and inspection records). Current C=267156, U=126076. Title reached at frame 904 using bounded untraced 600-frame advance from menu snapshot; sampled menu exploration continues. Trace detail capacity zero does not eliminate significant per-frame aggregate overhead; use brief traces around milestone samples.

Main menu and track selection visited; Daytona configured to 10 miles/4 laps (configured-1.png proves return main). Menu navigation investigation ongoing; D-pad recipe race-entry.py route batch from returned.snapshot. Current classified 268747/393232 bytes. scenarios.json now persists progression checklist; source manual hypotheses and NES-001 input correction documented. No artificial RAM edits yet.

Latest round-01 frontier: C=271832/393232, U=121400. Accepted additional uniform non-code slices (1554 bytes), bounded seven-entry menu return-target table14bytes with selector domain0..6, static dispatcher handlers70bytes, source-anchored direct call entries80bytes. Natural Daytona cockpit and Sears Point road initialization, practice/car setup visited; artificial RAM track3 shortcut visually confirmed Watkins overview and race initialization, no code gain. Accepted active/release sample code161bytes. Main progression obstacle is long prerace animation; holding A tested then released, one extra600frame wait still prerace. Final bounded1200frame no-input continuation executing as chunk.py release-wait prerace-later1200none (session95252), with end snapshot planned. This is still same disassembly phase, not terminal exhaustion. Later race/pit/result/ending contracts remain unresolved and will be handed to redirect if this route fails.

## Round 01 disassembly pass finished — ROM incomplete

ROM classification: 69.13% (271832 / 393232 bytes classified; 121400 bytes unknown).

One easy-gain disassembly pass completed. Final 1200-frame no-input continuation succeeded but remained at prerace, added zero bytes (`round-01/prerace-later.png`, `.snapshot`, `-step.json`, `-sample.json` and `-contexts.json`). Total6034 Code bytes,265798 Data/Image bytes,121400 unknown. All333 accepted regions have structural evidence. Exact unknown complement: [round-01/unknown.json](round-01/unknown.json). All unclassified PRG remains unknown, including raw generated fallbacks. No unused/dead classification claimed.

[pass-001 verification](pass-001-verification.json) confirms map validation and generated assembly byte identity, original ROM hash09a213971a9999a340b61eeb1e98b02b4e84eedd3311ca7c456ccded11e659c3. Current state/map SHA256 53b31ef28a2c0d0397d16245d8813a40f28cc4a9c6478c1fdd4f24fafb3e193a. [State](state.json), [portable current map](../maps/current.json), [frozen checkpoint map](../maps/pass-001.json), [generated assembly](../generated/pass-001/game.asm). Whole-ROM classification, gameplay compatibility, current-source commentary/readability and final editorial gates remain separate; commentary has not run.

Execution sampling, separate from classification: 2285 unique natural fetched ROM opcode starts retained across recorded context artifacts; 1254 artificial fetched starts, 0 artificial-only. Counts are sampled opcode starts, not execution-byte coverage or full game reachability. Accepted code is Static with root evidence distinguished in artifacts.

| Bank | Code | Data/image | Unknown |
|---|---:|---:|---:|
| header | 0 | 16 | 0 |
| PRG00 | 280 | 165 | 7747 |
| PRG01 | 1 | 532 | 7659 |
| PRG02 | 875 | 14 | 7303 |
| PRG03 | 21 | 85 | 8086 |
| PRG04 | 0 | 0 | 8192 |
| PRG05 | 0 | 733 | 7459 |
| PRG06 | 0 | 0 | 8192 |
| PRG07 | 156 | 501 | 7535 |
| PRG08 | 0 | 0 | 8192 |
| PRG09 | 161 | 181 | 7850 |
| PRG10 | 538 | 81 | 7573 |
| PRG11 | 225 | 438 | 7529 |
| PRG12 | 0 | 36 | 8156 |
| PRG13 | 0 | 258 | 7934 |
| PRG14 | 0 | 532 | 7660 |
| PRG15 | 3777 | 82 | 4333 |
| Each CHR00–CHR31 (32 banks) | 0 | 8192 | 0 |


Natural milestones: legal/logo/title, main/track/distance selection, Daytona race overview/configuration/cockpit initialization, Sears Point selection/cockpit initialization and practice setup. Artificial milestone: source-anchored RAM0557/0519=3 followed by ordinary transition rendered Watkins Glen and cockpit. Race laps, pits, results and championship ending remain unvisited; no suitable scene/result entry contract established. Extended no-input prerace continuation stopped providing useful changes. Redirect should investigate concrete RAMBF=5/B8=14 state and NMI pointer19/1A=BFA4 (banks8,9,14,15) from prerace-later.snapshot rather than repeat long button routes or guess arbitrary state patches. Main2 snapshot is clean natural menu frontier.

Toolkit findings: one suspected NES input-schema/documentation issue, zero confirmed bugs, no blocking issue. [Persistent toolkit report](toolkit-issues.md); buttons-array workaround validated. Toolkit and ROM unchanged. Free disk~4.0GB; scratch12KB, no cited artifacts pruned. This is an ordinary incomplete pass handoff to commentary/redirect, not terminal method exhaustion.

## Coordinator gate — round 01 commentary started

Disassembly handoff accepted as ordinary incomplete pass, not terminal exhaustion. Independently checked unchanged input hash and identical state/current/pass-001 hashes; merged physical classified intervals total 271832 bytes, unknown121400. Required 69.13% report printed to user before commentary. Separate worker round01_commentary launched with phase-only prompt and compact checkpoint/handoff; no competing phase writer. Current phase commentary; input checkpoint pass-001. Output checkpoint/readability verification pending. No leads consumed from prior redirect (initial list empty).

## Round 01 commentary completed

Complete current-source readability reviewed all480 physical scaffold entries and143 meaningful units. Source covers6034 code bytes,265798 data/image bytes, plus explicit notes retaining121400 unknown bytes. Functional routine/internal-entry names, control-flow labels and consistent hardware/RAM symbols added in commentary/source/game.asm. Multi-purpose scratch aliases checked; explanations distinguish mechanics from tentative game meaning. No structural/map changes. Fresh byte-identical verification commentary/verification.json passed; source SHA19795550bd396556c250ddcfd6a4df1be11ec5b89f6f1de0f802fd20a303084e, unchanged ROM SHA09a213971a9999a340b61eeb1e98b02b4e84eedd3311ca7c456ccded11e659c3. reading-after.asm refreshed. Final editorial audit remains pending. Scope/examples/uncertainties and unchanged prerace/menu-table redirect questions: round-01-commentary-handoff.md. One preexisting suspected toolkit issue, zero confirmed/blocking issues; toolkit/ROM unchanged. ROM still incomplete at69.13%; no final archive/compatibility claim.

## Coordinator gate — round 01 redirect started

Commentary handoff accepted: all480 inventory entries and143 meaningful units reviewed, no current-source scope omitted. Inspected menu navigation, PPU command queue and audio envelope source explanations; verified current game.asm hash matches final commentary/verification.json (19795550bd396556c250ddcfd6a4df1be11ec5b89f6f1de0f802fd20a303084e), byte_identical=true, refreshed reading-after newer than source. Structural checkpoint remains pass-001, classification unchanged69.13%. Final editorial audit remains pending for terminal acceptance. Separate round01_redirect worker launched with phase prompt and compact commentary/checkpoint handoff. Current phase redirect; no ready lead consumed yet.

## Round 01 redirect — r01-20261120-a ongoing

Research-only attempt in redirect/r01-20261120-a, checkpoint pass-001 unchanged C271832/U121400. Readability handoff complete. Resolved stalled cockpit with source/stack contract: BFA4 is CHR readback callback already finished; mainB A27 calls60000frame timedinputwaitE3E7. Start3/release60 and180frames reaches live race, A540frames144mph. Source-supported artificial8112returnA0 atBF7E/SPFF executes realE485 cleanup and reaches Race Results; resultStart returnsmenu. Details/scenarios and snapshots saved; no natural completionclaim.

Ready proposals saved persistently as R01R-AUDIO-GAPS, R01R-TEXT-HELPERS and R01R-MENU-BATCH, with precise physical ranges and verification recipes. TextFB85..FD6D splits338noncode+150code; central instrumentmodulation now mechanically understood. No classifications or source changes here. Further race/pit/ending research ongoing: directE0E2 test omittedcleanup and lostselectorpatch, preservednegativeevidence; revised run_until stopped actualBF7E continuation atE0E2 afterE485 (14019operations), savedshutdown-clean.snapshot. Clean scene0finalseasonselector88 and scene4pitselector1 boundedtests running. Current exact frontier in attempt/frontier.md; fullresidual dispositions pending. No terminalclaim.

## Round 01 redirect completed — combined ready batch

Research attempt r01-20261120-a finished with nine ready leads and15495 proposed previously unknown bytes (union), **zero accepted new bytes**. Input pass-001 unchanged: C271832/U121400 (69.13% accepted); exact147-range residual ledger and proposed intersections saved in redirect/r01-20261120-a/residual-ledger.json.5470 previously unknown fetched instruction bytes independently decoded; natural race roots separate from artificial pit/results/ending. Source/map/ROM hashes unchanged; final editorial audit pending.

Natural Start route resolved long prerace wait and reached live race144mph. Cleanup-aware artificial wrapper return reached results; supported scene/pit contracts visited top-down pit, service crew interaction and return live racing133mph. Final-season88 shortcut reached trophy/champion portrait and animation; Start exited to championship standings. Initial direct scene jump failures preserved and corrected by actualE485 then bounded run_until E0E2.

Coherent ready batch includes prerace sequence/RLE/sprites/readback, race scene roots, ending code and fully bounded RLE/palette/sprite/10-frame container, pit copied backing, all seven menu handlers, audio modulation/command gaps, fixed text helpers/corpus, fixed-bank core/math/numeric tables and source-wide intro/practice/setup continuations. Generic data proposed where mechanical non-code evidence suffices; semantic labels not required. Setup downstream streams and localized intro2-byte ambiguity have actual deferred reasons in handoff. This is **not terminal method exhaustion**. Independent disassembly review next, then full current-source commentary and redirect. Full report: round-01-redirect-handoff.md. Toolkit unchanged; one preexisting suspected issue, zero confirmed/blocking, no new issue.

## Coordinator gate — round 02 disassembly started

Accepted productive round01 redirect handoff after inspecting coherent connected/independent batch, physical proposals, exact residual ledger and substantive deferral reasons. Nine ready IDs: R01R-AUDIO-GAPS, R01R-TEXT-HELPERS, R01R-MENU-BATCH, R01R-ENDING-BATCH, R01R-PIT-COPY, R01R-SCENE-RACE-ROOTS, R01R-FIXED-CORE, R01R-SOURCE-CONTINUATIONS, R01R-PRERACE-BATCH. Proposed15495 unknown bytes are not accepted classification; input pass-001 remains C271832/U121400. No terminal exhaustion decision. Prior worker completed, then separate round02_disassembly launched with phase-only prompt and compact handoff/checkpoint. Current phase round02 disassembly; output pass-002 pending. Source/map hashes independently unchanged at transition.


## Round 02 disassembly — in progress

Initial nine-lead review accepted15495 unique new bytes; map/state saved and clean validation in round-02/validation-initial.json. Current C287327/T393232 U105905. Connected menu helper/callback review continues; no commentary edits. Frontier: round-02/frontier.md.


## Round 02 disassembly completed

ROM classification: 74.44% (292707 / 393232 bytes classified; 100525 bytes unknown).

Single pass accepted 20875 unique new physical bytes; all nine R01R IDs verified, exact subclaims and application-order gains in round-02/summary.json. Initial batch15495bytes plus connected menu extension5380bytes (3212code/2168data). Unknown complement now297ranges in round-02/unknown.json. Existing classifications preserved.

Independent code audit redecoded62 proposed code intervals with exact instruction endpoints. Root/caller/branch/stack and bank contracts support reset/NMI/IRQ, PPU/attribute/decimal/math/audio/text, intro/practice/setup and prerace/ending paths. Getters are Static callable contracts without natural-caller claim. Observed extra instructions retain scenario-specific physical identities and effective contexts; artificial pit/results/ending fetches do not claim natural victory. Existing broad representative gameplay evidence reused, no repeated failed routes.

Connected menu work extended original handler gaps through password codec/editor, ordered results, field rendering and installed callbacks. Complete direct CFG1803instructions, no data-barrier/conflicting-decode/unknown-callee stops. Fixed E626/FDC1 helper contracts preserve PRG; terminal E0E2 stops context propagation. Intentional AF3D nonlocal PLA/PLA exits match callers. Four1024-output RLE streams,32-byte palettes, counted sprites and bounded menu descriptor/word/mask corpora accepted; less understood password backing remains generic Data with pattern/consumer/boundary evidence. FBA0 exploratory text decoding rejected as code. Intro87BD..87BF stillunknown; variable setup actor streams and other residual PRG remain unresolved, no whole-gap rejection or terminal exhaustion claim.

Fresh map.validate, save/export and checkpointpass-002 succeeded; generated assembly rebuild is byte-identical. Map/state/current/pass002 SHA256`e5112691a35118605d5fe42ebe56cfeb23a268056883e512e3712dbb87dfd7af`. Commentary source preserved byte-for-byte SHA256`19795550bd396556c250ddcfd6a4df1be11ec5b89f6f1de0f802fd20a303084e`; current-source readability must be carried forward/reviewed in next commentary phase against new generated baseline. Toolkit/ROM unchanged. One preexisting suspected input-schema/documentation issue, zero confirmed/blocking issues; no new toolkit issue, see [toolkit-issues.md](toolkit-issues.md).

| Physical bank | Classified | Unknown | Code | Data/image |
|---|---:|---:|---:|---:|
| header | 16 | 0 | 0 | 16 |
| PRG00 | 2674 | 5518 | 1062 | 1612 |
| PRG01 | 533 | 7659 | 1 | 532 |
| PRG02 | 4860 | 3332 | 3351 | 1509 |
| PRG03 | 1641 | 6551 | 883 | 758 |
| PRG04 | 0 | 8192 | 0 | 0 |
| PRG05 | 733 | 7459 | 0 | 733 |
| PRG06 | 0 | 8192 | 0 | 0 |
| PRG07 | 913 | 7279 | 396 | 517 |
| PRG08 | 744 | 7448 | 744 | 0 |
| PRG09 | 2902 | 5290 | 893 | 2009 |
| PRG10 | 1613 | 6579 | 1253 | 360 |
| PRG11 | 1070 | 7122 | 632 | 438 |
| PRG12 | 1377 | 6815 | 1341 | 36 |
| PRG13 | 1123 | 7069 | 865 | 258 |
| PRG14 | 2172 | 6020 | 1640 | 532 |
| PRG15 | 8192 | 0 | 6552 | 1640 |
| Each CHR00–CHR31 | 8192 | 0 | 0 | 8192 |

Evidence, exact per-bank/unknown ledger and verification: [round-02/summary.json](round-02/summary.json), [handoff](round-02-disassembly-handoff.md). Incomplete; next phases commentary then redirect.

## Coordinator gate — round 02 commentary started

Accepted disassembly handoff and independently checked state/current/pass-002 identity e5112691a35118605d5fe42ebe56cfeb23a268056883e512e3712dbb87dfd7af, unchanged ROM and prior commentary source hashes, unique classified interval count292707/U100525. Fresh pass-002 reports byte_identical=true. Required74.44% printed before commentary. Nine ready lead outcomes verified; ordinary productive incomplete handoff, not terminal assessment. Separate round02_commentary worker launched with compact phase input; required preservation/carryover of round01 commentary before changed-baseline preparation. Current phase commentary, input pass-002; output readability/verification pending.

## Coordinator gate — round 02 redirect started

Commentary handoff accepted after coverage/identity evidence and representative source inspection (password validation, actor command grammar, pitch modulation). All3026 records/547 units reviewed;270 codegroups have prose, no pending ranges, scaffold/source links valid. Current source hash4889916dfdf3fad9cd1789fb5e0f6beb0289cd7d1cf904f39d84e9837e8f9ead matches fresh byte-identical report, refreshed reading copy verified. Prior143 unit reviews retained, historical commentary preserved. Final editorial_audit pending. Separate round02_redirect launched after commentary worker completed; current phase redirect, input pass002 C292707/U100525. Concrete race/stream/residual questions supplied; no terminal stop claim.

## Round02 redirect — ready research batch

Research-only attempt [r02-20261120-a](redirect/r02-20261120-a/frontier.md) completed against pass002/current readability. Three new R02R ready leads in leads.json propose27743 code,4318 setup object backing and68462 other generic noncode bytes (100523 unique proposed, zero accepted in redirect). Exact379 interval proposals,97Data profiles and full100525-byte unknown dispositions preserve CPU aliases/effective banks. Connected race/pit/track producers, all supplied finite handlers, actual installed callbacks and independently coherent small helpers investigated; setup corpus has33 animation lists and48 counted sprites exactly endingA954. Generic data role/boundary checks are localized by bank/group in contracts.md, with exact neighbors/references per profile. Only7CD..7CF2bytes unproposed; this is no terminal exhaustion assertion. All state/map/source/ROM hashes unchanged. Independent disassembly review next; source readability/editorial remain separate gates. Existing NES-001 remains suspected/nonblocking, no new toolkit issue. No runtime replay, archive, publish or deletion.

## Coordinator gate — round 03 disassembly started

Accepted productive round02 redirect handoff after inspecting connected graph/setup/independent noncode scope, contracts and thin-island caveats. Three ready IDs: R02R-CONNECTED-CODE, R02R-SETUP-OBJECTS, R02R-RESIDUAL-NONCODE.379 interval claims propose100523 unknown bytes; none accepted yet, pass002 C292707/U100525 unchanged. Independently checked current state/map/source identities. Local intro2-byte ambiguity is not terminal exhaustion; thin data/return contracts require independent review. Previous worker completed before separate round03_disassembly launched with phase-only prompt+compact checkpoint/handoff. Current phase round03 disassembly; pass003 pending.


## Round 03 disassembly completed

ROM classification: 100.00% (393230 / 393232 bytes classified; 2 bytes unknown).

Incomplete: the displayed percentage rounds upward; physical7CD..7CF remains unknown. Exactly one pass accepted100523 new bytes (27743 Code;72780 Data:4318 setup objects+68462 other backing). All three R02R IDs verified with Static provenance; no new observed fetch/natural-use claim. Existing classifications and historical scenario evidence preserved.

Independent review fresh-decoded281 normalized Code intervals; one original bank-boundary split bisected the9FFE JMP and was coalesced into a correct contiguous execution view. Full graph physical identities/overlaps, ordinary/nonlocal stack contracts, CHR-only MMC3 writes, finite dispatch/callback roots and two constant-branch data barriers reviewed. All97 Data claims have positive byte-pattern/consumer/boundary checks. Setup parser independently reproduces16 stream groups,33 lists and48 counted sprites ending exactlyA954. Thin1C2F/440F have actual display consumers;735F80,78 matches independent editor origin constants;AB52/finalA0 are localized variations in bitpacked backing. Generic Data retains unresolved schemas/subjects. Detailed decisions: [round-03/decisions.md](round-03/decisions.md).

Existing natural live race and artificial pit/service/results/trophy/standings routes reused; no unchanged runtime retry. No actionable unreviewed connected direct edge remains; residual intro bytes need new independent evidence, not absorption or a terminal claim. All prior attempts retained in leads/scenarios.

Fresh map.validate, save/export and unique pass-003 checkpoint succeeded; byte_identical=true. State/current/pass003 SHA256`b1324b7c5ecec7c5670619a96c02ceec9aacd4e8a4b71b6c6a313c61b8bbcfbb`. ROM unchanged; prior commentary source preserved SHA256`4889916dfdf3fad9cd1789fb5e0f6beb0289cd7d1cf904f39d84e9837e8f9ead`. Full current-source readability must be carried forward against new generated baseline by the next commentary phase; final editorial audit pending. No toolkit edits or new issues. One existing suspected nonblockingNES-001 with buttons-array workaround: [toolkit-issues.md](toolkit-issues.md).

| Physical bank | Classified | Unknown | Code | Data/image |
|---|---:|---:|---:|---:|
| header | 16 | 0 | 0 | 16 |
| PRG00 | 8190 | 2 | 2982 | 5208 |
| PRG01 | 8192 | 0 | 246 | 7946 |
| PRG02 | 8192 | 0 | 4623 | 3569 |
| PRG03 | 8192 | 0 | 1012 | 7180 |
| PRG04 | 8192 | 0 | 2135 | 6057 |
| PRG05 | 8192 | 0 | 0 | 8192 |
| PRG06 | 8192 | 0 | 5478 | 2714 |
| PRG07 | 8192 | 0 | 2901 | 5291 |
| PRG08 | 8192 | 0 | 2853 | 5339 |
| PRG09 | 8192 | 0 | 1106 | 7086 |
| PRG10 | 8192 | 0 | 5097 | 3095 |
| PRG11 | 8192 | 0 | 871 | 7321 |
| PRG12 | 8192 | 0 | 3530 | 4662 |
| PRG13 | 8192 | 0 | 2150 | 6042 |
| PRG14 | 8192 | 0 | 5820 | 2372 |
| PRG15 | 8192 | 0 | 6552 | 1640 |
| Each CHR00–CHR31 | 8192 | 0 | 0 | 8192 |

Exact counts/identities: [round-03/summary.json](round-03/summary.json). Unknown ledger: [round-03/unknown.json](round-03/unknown.json). Handoff: [round-03-disassembly-handoff.md](round-03-disassembly-handoff.md). No terminal exhaustion assertion; commentary then redirect.

## Coordinator gate — round 03 commentary started

Independently counted unique physical classification393230/T393232, U2; state/current/pass003 identical hashb1324b7c5ecec7c5670619a96c02ceec9aacd4e8a4b71b6c6a313c61b8bbcfbb. Inspected independent decisions (instruction coalescence, nonlocal stack contracts, positive data claims) and fresh pass003 byte-identical report. Required100.00% printed with explicit2unknown/incomplete distinction before commentary. Separate round03_commentary launched after disassembly completion, input pass003; full prior commentary preservation/carryover and expanded readability required. Next-commentary source-link normalization explicitly tasked under actual archive editorial_status consumer; final editorial audit still pending. No terminal assessment, remaining2byte claim must go through redirect after readability gate.

## Coordinator gate — round 03 redirect started

Accepted current-source commentary after full scope/handoff/inventory and representative source inspection (pit frame and alternate road profile).3108records1014canonical units reviewed, no codegroups missing prose;547prior review credits retained. Final source0b2d691cbeb7429775600e63b35c8b11282cf8a17ce53854f3f4052b5c15f46c matches byte-identical report and refreshed reading. Actual ordinary archive editorial_status source locations independently valid; only final editorial audit pending. Separate round03_redirect launched after commentary completed with exact2byte residual and explicit layout-versus-code/data rejection-premise question. Current phase redirect, input pass003 C393230/U2. No terminal assessment yet.

## Coordinator gate — round 04 disassembly started

Accepted productive focused round03 redirect handoff: readyR03R-INTRO-PREFIX-NONCODE proposes2bytes, accepted0. Inspected corrected neighboring descriptor constraints and specific noncode-versus-CLV/AHX competing claim; no exact prefix natural use/schema assumed. Generic-role claim independently reviewed next. Separate round04_disassembly launched after redirect completion with phase-only prompt and compact pass003 checkpoint/handoff. Current phase round04 disassembly; input C393230/U2, pass004 pending. Concrete stale source/uncertainties wording captured by redirect must be corrected in following commentary and targeted final editorial acceptance. No terminal success claimed.


## Round 04 disassembly completed

ROM classification: 100.00% (393232 / 393232 bytes classified; 0 bytes unknown).

Exactly one targeted pass independently accepted R03R-INTRO-PREFIX-NONCODE, physical1997..1999, CPU87BD..87BF banks0,1,14,15, as generic Data Static. Two new classified bytes, no new Code or observed execution. Structural classification is complete; full workflow completion awaits current-source reconciliation, final editorial audit and acceptance. Generic pair ownership, exact prefix meaning and natural read remain unresolved without weakening code-versus-data classification.

[Independent decisions](round-04/decisions.md) and [fresh review](round-04/review.json) verify immutable bytes, complete count16/count30 extents and coherent record contents, optional-prefix renderer semantics, accepted count17 comparator and no conflicting current execution views. The freshly decoded CLV/three-byte AHX alternative consumes established count30/first relativeY bytes; positive bounded resource context supports noncode. Prior original-layout rejection preserved, narrowed generic inference corrected. No absence-of-execution-only argument. Existing classifications, evidence and all prior source/history preserved.

Fresh map.validate, save/export and unique pass-004 verification succeeded, byte_identical=true. State/current/pass004 SHA256`b9ffea72f87f4d885472246166924f63872cf27b4a50adfa60756a57823d5ceb`. ROM unchanged. Commentary source unchanged SHA256`0b2d691cbeb7429775600e63b35c8b11282cf8a17ce53854f3f4052b5c15f46c`; source still describes the former unknown pair and carries stale wording documented in round03 redirect. Next commentary owns reconciliation and full current readability; no source edits here.

Prior representative natural race/artificial pit, service, results, ending and standings scenarios reused without unchanged replay or full-ROM rescan; no new reachability or full execution claim. No remaining structural unknowns or actionable structural continuations. Final editorial audit remains pending. No toolkit changes or new issues; one existing suspected nonblocking NES-001 input-schema/documentation issue, zero confirmed bugs/zero blockers: [toolkit-issues.md](toolkit-issues.md).

| Physical bank | Classified | Unknown | Code | Data/image |
|---|---:|---:|---:|---:|
| header | 16 | 0 | 0 | 16 |
| PRG00 | 8192 | 0 | 2982 | 5210 |
| PRG01 | 8192 | 0 | 246 | 7946 |
| PRG02 | 8192 | 0 | 4623 | 3569 |
| PRG03 | 8192 | 0 | 1012 | 7180 |
| PRG04 | 8192 | 0 | 2135 | 6057 |
| PRG05 | 8192 | 0 | 0 | 8192 |
| PRG06 | 8192 | 0 | 5478 | 2714 |
| PRG07 | 8192 | 0 | 2901 | 5291 |
| PRG08 | 8192 | 0 | 2853 | 5339 |
| PRG09 | 8192 | 0 | 1106 | 7086 |
| PRG10 | 8192 | 0 | 5097 | 3095 |
| PRG11 | 8192 | 0 | 871 | 7321 |
| PRG12 | 8192 | 0 | 3530 | 4662 |
| PRG13 | 8192 | 0 | 2150 | 6042 |
| PRG14 | 8192 | 0 | 5820 | 2372 |
| PRG15 | 8192 | 0 | 6552 | 1640 |
| Each CHR00–CHR31 | 8192 | 0 | 0 | 8192 |

Exact per-bank counts, annotation coverage, identities and verification: [round-04/summary.json](round-04/summary.json). Empty unknown ledger: [round-04/unknown.json](round-04/unknown.json). [Handoff](round-04-disassembly-handoff.md). No archive/publication/deletion or toolkit/ROM changes.

## Coordinator gate — round 04 commentary/final editorial started

Round04 independent2byte claim accepted; unique physical coverage independently393232/T393232,U0, state/current/pass004 allb9ffea72f87f4d885472246166924f63872cf27b4a50adfa60756a57823d5ceb. Fresh pass004 byte-identical report inspected; required100.00%C393232/U0 printed before commentary. Structural whole-ROM coverage achieved; fullworkflow acceptance unproven. Separate round04_commentary launched after disassembly completion, tasked entire prior preservation/carryover, current-source reconciliation and concrete stale wording corrections, targeted final editorial audit using sound prior review credit, actual consumer acceptance+fresh source verification/reading. Next separate redirect closure and coordinator requirement/document audit still required.

## Coordinator gate — round 04 redirect closure started

Accepted full current-source/final editorial handoff after substantive report/correction review and exact current source/reading/review/map/ROM hashes independently matched final checks. Source3f5de2c81f8232cafa153d0d497779ba7d9728f001bba6ef1971d1eacd1afd2f, fresh byte-identical audit, actual editorial_consumer complete=true/reasons[].3108regions/1014canonical/2924nested units valid, prior credits preserved. Separate round04_redirect launched after commentary completion; current phase final redirect against pass004 C393232/U0. Tasked concrete contradiction/lead closure and current gameplay-checklist reconciliation without replay or arbitrary semantics requirements. Final coordinator requirement+document audit remains pending.

## Round04 redirect structural closure and editorial finding

Independently verified pass004 C393232/T393232,U0 exact complement/no overlaps and current source/map/ROM identities. Entire current source triage reused sound prior rooted/resource review and inspected88semantic uncertainty hits. No concrete structural contradiction or ready lead; proposed0/accepted0. Historical scenario attempts retained, with current resolution notes and representative natural/artificial milestone summary; all evidence links valid. Talladega remains unvisited/deferred with comparable Daytona/no concrete new structural prospect reason.

Found concrete stale editorial claim at PulseEngineLikeTone source16134: visible local eighth-call target writes APUSTATUS03 and returns, despite comment/review saying unresolved path. Research-only redirect preserves source/review and reports exact correction to coordinator. Terminal structural handoff justified; final recommendation conditional on prose correction and refreshed actual editorial/source checks. No extra disassembly/unchanged gameplay needed. [Handoff](round-04-redirect-handoff.md), [results/frontier](redirect/r04-20261120-a/results.md). Existing suspected nonblocking NES-001; no new toolkit issues. Disk3.8GB free; no runtime/archive/publish/cleanup/toolkit/ROM changes. Coordinator final requirement/document audit remains.

## Final redirect feedback — targeted commentary resumed

Round04 redirect completed independent structural/current-source triage:0unknown/nooverlap/no structural ready leads, representative scenario evidence valid and historical checklist reconciled. It found one concrete current editorial contradiction in PulseEngineLikeTone eighth-call comment/current review summaries. Green prior audit is insufficient while that contradiction remains. Separate resumed commentary worker round04_editorial_repair launched only after redirect worker completed, with exact remaining handoff, prior snapshot required, targeted prose repair and refreshed actual audit/verification/read/source links. No empty disassembly or whole-source reread requested; structural closure credit reused. Terminal acceptance pending this repair and focused redirect confirmation.

## Round04 same redirect — focused post-repair closure

Independently confirmed sole stale PulseEngineLikeTone condition repaired against immutable80528..80574 bytes, corrected source/current region+canonical summaries; history intact. Fresh actual audit/verification/consumer evidence independently matches current sourcefcd0742801720ec0f0be464384b9ad1ecdcdbb16c6c5a9e91ef0bd5d2bd0eccb and pass004 unchangedb9ffea; byte_identical=true, consumer complete=true/reasons[]. Previous full-source/scenario triage reused without unchanged replay. C393232/T393232/U0, no ready leads, proposed0/accepted0. [Final redirect handoff](round-04-redirect-final-handoff.md) recommends terminal whole-ROM complete, with coordinator final requirement/document audit separate. Prior conditional artifacts preserved. No archive/goalstatus/source/map/ROM/toolkit mutation or cleanup; no new toolkit issue/blocker.

## Coordinator final acceptance

Requirement-by-requirement completion audit verified against current files, final independent redirect, complete physical map evidence, actual editorial consumer and fresh source verification. Final recursive archive-contract inventory:66Markdown paths,43additional final selections,13disposable omissions,0unaccounted. Prior final records/sources remain preserved; no source/map change after final checks. Ready for later separately authorized archive, with no archive execution or cleanup here.
