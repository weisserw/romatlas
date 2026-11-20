# Investigation ledger

## Round 01 disassembly — current supported batch

Baseline: [baseline-verification.json](baseline-verification.json), configured binary `/root/romatlas/target/release/romatlas`, toolkit commit `97ae1079e17baaae098f074f9965bba2ef7529fb`. Input unchanged. Initial leads list empty. No commentary or redirect executed in this phase.

- **R01-VECTORS (verified structural).** Physical vector words 131082..131087 encode NMI E1AE, reset E000, IRQ E3B4 in effective initial banks `[0,1,14,15]`. Reset initializes CPU/PPU/mapper; interrupt roots have coherent save/dispatch contracts. `round-01/vector-*.json`, `static-discovery.json`: 149 instruction candidates accepted (325 bytes). Mapper/indirect/call barriers preserved; unobserved static paths are not marked Observed.
- **R01-METADATA (verified structural).** Header 0..15 precisely parses PRG/CHR lengths and absence of trainer. CHR 131088..393231 is physically separate PPU 2bpp pattern storage; inspected title rendering and exact header boundaries. Image role does not imply semantic grouping of tiles. Hardware vector words classified Data separately.
- **R01-PATTERNS (verified generic data).** Bounded inspection of references available from vector/menu code, neighboring bytes and conflicts establishes maximal uniform runs as non-code. `round-01/pattern-evidence.json` (12 intervals, 2064 bytes) and `pattern-evidence-additional.json` (29 intervals, 1554 bytes) retain exact physical boundaries and adjacent bytes. Specific consumer/generator/fill roles unresolved; generic Static Data suffices. No execution-absence inference or unused/dead claim.
- **R01-RUNTIME (verified fetch roots, static continuations).** Corrected routes in `round-01/route.py` and scenarios.json visit legal/logo/title, main selection, Daytona short race overview, car configuration/cockpit initialization, Sears Point selection/cockpit, practice setup. `*-contexts.json` retain physical fetch identities and bank context. `*-discovery-*.json` use static discovery from supported entries; `*-inspection-*.json` inspect exact candidate instruction boundaries, then map.annotate confirms only nonconflicting complete instructions. Additional branches/continuations are Static, even when their root was naturally fetched.
- **R01-DISPATCH (verified structural).** Physical 0x4AC1..0x4ACE is seven return-minus-one handler words. RAM9D wraps 0..6 in navigation code (INC/CMP#07/reset and negative wrap6); dispatcher doubles index, pushes high/low table bytes then RTS. Consumer plus independent finite selector domain proves 14-byte extent. `menu-selector.json`, `menu-dispatch.json`, `dispatch-contexts.json` retain static entry contracts for handlers; handlers add70 bytes. All seven natural selections not asserted. Follow related handler bodies in redirect where concrete role/extent questions remain.
- **R01-DIRECT-CALLS (verified static contracts).** `call-roots.py` checks already-classified instructions and resolves direct JSR callee entry using pre-call effective banks. Six supported entries inspected,80 bytes added; no callee-return bank assumption. `calls-contexts.json` has hits0 and explicit structural evidence, not runtime evidence.
- **R01-TRACK3 (verified artificial selection/init).** Natural menu comparison Daytona0/Sears1 plus source reads/stores of RAM0557/0519 establish track selector contract. Restore clean main2 snapshot, buttons=[], patch RAM0557=3 and RAM0519=3, ordinary Down/Right/Right/A yields rendered Watkins Glen overview. A/Start/Start enters its cockpit initialization. `patch-track-clean.json`, `patch-track-after.json`, `patched-watkins-3.png`, `watkins-drive-*`; effective banks before patch2,3,14,15. Subsequent fetches explicitly artificial, not natural reachability. No new code from Watkins branch.

## Bounded negative/interrupted experiments

- Initial 180-frame all-event traced startup with max_operations10000000/timeout_ms30000 returned execution timeout. Retried with 30 frames, timeout300000, capacity0, kinds Instruction/ControlFlow/BankSwitch; succeeded. Later use brief trace samples and bounded untraced advancement. This is a changed execution budget/capture configuration, not repeated unchanged failure.
- Initial detached RPC host did not persist as expected; self-contained Python subprocess drivers replaced it. Frame capture collisions correctly reject overwriting; unique filenames used. No toolkit defect inferred from either.
- **NES-001:** original `button`/`pressed` calls silently no-op under NES adapter; earlier supposed Start/A steps are no-input routes. Corrected `buttons:[name]`/`buttons:[]` source-confirmed and visually tested. Actual instruction-fetch evidence remains valid. See toolkit-issues.md.
- Holding A for600 frames from drive snapshot left prerace animation. An additional600-frame state exists in racing-step-1.json, but the following traced sample was interrupted (process143); no snapshot/coverage from that unfinished sample claimed. Snapshot drive remains replay frontier. Controller release-wait at CPU E424 is relevant, but not proven sole progression obstacle.
- Releasing buttons and600-frame wait from active snapshot still shows prerace man animation (`release-wait.png`, snapshot); bank9/fixed15 fetch sample added46 bytes. Extra1200-frame no-input continuation pending as latest frontier; no results/ending claim yet.

## Remaining questions for subsequent phases

These are concrete redirect questions, not terminal exhaustion or proof that regions cannot be classified. Race-active progression, pit entry, results, championship continuation and ending require the scene/timer/transition contract. Natural control sequence reaches cockpit but does not yet demonstrate racing laps. Track selectors are understood; arbitrary scene/PC edits lack an entry contract and were not brute-forced. Dispatcher menu selectors, NMI RAM19/1A and IRQ RAM30/31 trampolines provide useful specific consumer paths. Unknown PRG code-versus-data intervals remain for targeted consumer/structural study; header/CHR and accepted uniform data do not require semantic research to stay classified.

## Final round-01 outcome

ROM classification: 69.13% (271832 / 393232 bytes classified; 121400 bytes unknown).

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

Final1200-frame no-button continuation completed without additional classification or scene progression. Prerace RAMBF=5, B8=14 and NMI pointer19/1A=BFA4 remain unchanged at banks8,9,14,15. This falsifies the narrower suggestion that holding A alone explained the obstacle; release wait is relevant but insufficient. No arbitrary scene-register patch attempted without a contract. Preserve old experiment and resume targeted redirect from prerace-later.snapshot. Phase checkpoint pass-001 validates and rebuilds byte-identically. Full commentary remains next. One suspected toolkit issue, workaround available; see toolkit-issues.md.

## Redirect r01-20261120-a — productive combined handoff

Supersedes the unresolved prerace question with source/stack wait contract and natural Start route; preserves all failed input/wait histories. Artificial results/pit/ending routes now have cleanup-aware entry contracts and representative interactions. Nine ready structural leads with exact proposals and reproducible evidence are in leads.json;15495 proposed unknown-byte union,0 accepted here. All concrete fixed-bank gaps and explicit commentary continuation questions were triaged/followed; exact147-range complement121400bytes remains unchanged until disassembly review. Detailed methods, localized ambiguities and actual deferrals: round-01-redirect-handoff.md and redirect/r01-20261120-a/frontier.md. Not terminal exhaustion; no rejection review or classification claim substituted for independent review.


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
