# Disassembly progress

Workspace initialized. Baseline and classification have not run.

## Round 001 — disassembly active

Baseline established with `python3 run.py inspect` and `baseline`: byte-identical
262160-byte rebuild; maps/baseline.json remains raw export, not classification.
Configured release binary used unchanged. ROM SHA-256 matches workspace identity.
Game identified locally from title/copyright as Attack of the Killer Tomatoes (NES).
Local runtime: startup copyright, title, street gameplay and movement/jump inputs.
Evidence: analysis/round001/*.json and selected frames; persistent snapshots retain
replayable states. Initial 180-frame tracing timed out; bounded 30-frame trace
batches succeed, zero detail capacity intentionally drops detail while aggregates
remain intact (aggregate_dropped=0 in first summary). Trace disabled for long waits.
First gameplay discovery: 2908 ROM contextual roots, four bounded discovery batches
(max_instructions=50000), no exhausted budgets. Classification pending inspection.
Frontier: inspect candidate overlaps/paths, identify supported progression RAM
contract and attempt later areas/ending shortcuts, then annotate/save/checkpoint.

### Progression batch

Source-guided artificial transitions succeeded from clean level1.snapshot.json:
write RAM $0716=0, $0031=6/7/8/9, $0026=1, $0722=200, $0718=9;
release all inputs and run 120 frames, then right+a+b for 15 traced frames.
Modes 6=sewer, 7=factory, 8=scaffolding, 9=tomato-lined chamber (boss role pending).
Each yielded accepted ROM execution; frame/snapshot/RAM/root/discovery evidence
is saved under analysis/round001/mode*. Effective lower banks 0/1 for sewer,
2/3 for factory/scaffolding/chamber; fixed upper banks 14/15. These are artificial
visits, not natural reachability. Five gameplay contexts now produce 12223 unique
candidate code bytes, with no overlapping instruction interpretations; map edits
still pending final candidate inspection. Bounded alternate modes 0/2/3/4 are
running to identify ending/game-over screens. Frontier: inspect these screens,
exercise representative mechanics, review CHR loader data and uniform PRG tails,
annotate supported candidates and save/verify pass-001.

### Pre-classification review

Distinct later-area visits and mechanics completed; mode4 explosion scene was
also attempted as a terminal-screen shortcut, with 900 extra idle frames. No
credits appeared. Mode9 chamber wait/throw/move/jump showed no boss; spawn
prerequisites remain unknown. These are documented attempts, not confirmed boss
or ending reachability. No arbitrary RAM brute force performed.

CHR visual review: analysis/round001/chr-review.png is a 32-bank (4 KiB each)
16-byte planar tile contact sheet. Banks 0..3 and 8..31 visibly contain coherent
font/sprite/background tiles. Banks 4..7 are mixed/non-image layouts, and a PPU
loader copies into RAM, so those 16 KiB will remain unknown pending exact format
and source-copy contracts. Do not classify all CHR as graphics. Practical data
consumer/format check complete for the clearly tiled banks.

One bounded static-call-root batch follows already reviewed direct JSR targets
with their caller bank context; intended to recover straightforward unexecuted
callees without a full recursive sweep. Then save supported map and checkpoint.

## Round 001 completed — incomplete ROM

ROM classification: 59.93% (157105 / 262160 bytes classified; 105055 bytes unknown).

Code:15126; data/image:141979; unknown:105055. All 968 classified regions have evidence annotations. Observed ROM instruction bytes:11045; statically supported code bytes:4081. Execution coverage is separate from structural coverage; no natural reachability claim for artificial patches or static paths. RAM execution retains RAM identity.

Verified baseline and pass-001 retained. Final checkpoint: maps/pass-001-final.json, generated/pass-001-final, analysis/pass-001-final-verification.json. Validation succeeded and full262160-byte generated assembly rebuild is byte-identical. Final map SHA256 a10e5d449f1da16d2fcb15bb5c2ee2514d66c7caae0b953ac076906bf7de2b54. ROM SHA256 unchanged.

Easy-gain pass ends after broad representative states, source-guided shortcuts, bounded mechanic alternatives, vector/observed/static-call batches, tile-format and uniform-data checks. Further work requires indirect-dispatch/upload/actor-initialization research, not repeated early gameplay. This is ordinary easy-gain stopping, not terminal exhaustion. No commentary/redirect work started.

Lead outcomes: L001–L004 verified; L005–L008 blocked by specific contracts, with actionable redirect recipes in analysis/leads.json. Boss and full credits remain unconfirmed; explosion scene is an ending hypothesis only. See analysis/scenarios.json and analysis/gameplay.md for inputs and research uncertainties. Toolkit: no toolkit issues found; analysis/toolkit-issues.md.

Per-bank counts (data includes Image):

| Bank | Code | Data | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG00 | 1034 | 0 | 7158 |
| PRG01 | 24 | 4245 | 3923 |
| PRG02 | 474 | 0 | 7718 |
| PRG03 | 212 | 0 | 7980 |
| PRG04 | 0 | 0 | 8192 |
| PRG05 | 1031 | 114 | 7047 |
| PRG06 | 1930 | 0 | 6262 |
| PRG07 | 2314 | 217 | 5661 |
| PRG08 | 2541 | 1529 | 4122 |
| PRG09 | 0 | 1646 | 6546 |
| PRG10 | 0 | 163 | 8029 |
| PRG11 | 0 | 5553 | 2639 |
| PRG12 | 0 | 960 | 7232 |
| PRG13 | 0 | 2836 | 5356 |
| PRG14 | 3396 | 0 | 4796 |
| PRG15 | 2170 | 1969 | 4053 |
| CHR00 | 0 | 4096 | 0 |
| CHR01 | 0 | 4096 | 0 |
| CHR02 | 0 | 4096 | 0 |
| CHR03 | 0 | 4096 | 0 |
| CHR04 | 0 | 2562 | 1534 |
| CHR05 | 0 | 2464 | 1632 |
| CHR06 | 0 | 2837 | 1259 |
| CHR07 | 0 | 180 | 3916 |
| CHR08 | 0 | 4096 | 0 |
| CHR09 | 0 | 4096 | 0 |
| CHR10 | 0 | 4096 | 0 |
| CHR11 | 0 | 4096 | 0 |
| CHR12 | 0 | 4096 | 0 |
| CHR13 | 0 | 4096 | 0 |
| CHR14 | 0 | 4096 | 0 |
| CHR15 | 0 | 4096 | 0 |
| CHR16 | 0 | 4096 | 0 |
| CHR17 | 0 | 4096 | 0 |
| CHR18 | 0 | 4096 | 0 |
| CHR19 | 0 | 4096 | 0 |
| CHR20 | 0 | 4096 | 0 |
| CHR21 | 0 | 4096 | 0 |
| CHR22 | 0 | 4096 | 0 |
| CHR23 | 0 | 4096 | 0 |
| CHR24 | 0 | 4096 | 0 |
| CHR25 | 0 | 4096 | 0 |
| CHR26 | 0 | 4096 | 0 |
| CHR27 | 0 | 4096 | 0 |
| CHR28 | 0 | 4096 | 0 |
| CHR29 | 0 | 4096 | 0 |
| CHR30 | 0 | 4096 | 0 |
| CHR31 | 0 | 4096 | 0 |

Compact handoff: analysis/round001/handoff.md. Evidence/snapshots/source remain local. Scratch is small and no cited evidence pruned.

## Round001 commentary active — batch 1

Prepared commentary/ from unchanged pass-001-final map; frozen before/ retained. Added hardware/RAM symbols and mechanical routine explanations for scroll/PPU/NMI/controller/palette/health/pause helpers. Readability credit:2053/15126 code bytes, all classified data/image and honest unknown gap notes. 13073 code bytes remain pending; full current-source gate not yet met. Fresh source verification byte_identical=true, unchanged input hash. Exact pending ranges: commentary/review.json; continuation: commentary/progress.md. No toolkit issues found. Do not start redirect.

### Commentary batch2

All5566 classified code bytes in fixed physical PRG banks14/15 are now explained, including metasprite rendering/clipping, actor handlers/culling, collision/trigger page tables, scripts, camera/HUD and explicit stack contracts. One paragraph per coherent routine replaces duplicate fragment prose. Interpreter consumers corrected $0700/$0701 to a pointer and $0702 to a delay. Fresh byte-identical verification passed. Pending9560 code bytes in lower physical PRG banks0..13; phase remains active. See commentary/progress.md and review.json.

## Round001 commentary complete — ROM classification still incomplete

Full current-source readability completed across all15126 code bytes,27291 data bytes,114688 image bytes and105055 honestly opaque unknown bytes. All1299 scaffold entries reviewed,260 meaningful routine/structure units; no pending classified source. Source aliases preserve physical bank context and forced addressing sizes; hardware/controller, actor/sprite overlays, movement/camera, scripts and audio fields are consistently named. Routine algorithms and stack contracts are explained in commentary/source/game.asm, not only these notes.

Lower-bank batches cover scene updates and transitions (pairs0/1,2/3), script/panel/text uploads (4/5), actor state/spawn/item/effects and player animation (6/7), player movement/camera and music/sound effects (8/9). Fixed pair14/15 review remains credited. Corrected trigger record format: header high nibble counts initial four-byte X-only records, followed by six-byte X/Y records; all upper bounds inclusive. This is a next-phase consumer lead, not a map edit. Other uncertainty/mechanics and concrete next checks: commentary/uncertainties.md.

Fresh final commentary verification byte_identical=true for262160 bytes; reading-after.asm refreshed. Source hash37a739f50caa0fe9919aa891eed528de58e59bce6983fc2f0a37b048c2780af1. ROM SHA256375f98a1cc3d40d5b5fb529abfe66cdd8320e12eb438e82efaa9e338d7a2f48d unchanged. Current/final/frozen map and internal state SHA256a10e5d449f1da16d2fcb15bb5c2ee2514d66c7caae0b953ac076906bf7de2b54 unchanged. Exact scope/source links: commentary/scope.json and review.json; local build report: commentary/verification.json. Final editorial audit remains pending for later acceptance. No toolkit issues found; analysis/toolkit-issues.md. No redirect work started by commentary worker.

Compact commentary handoff: analysis/round001/commentary-handoff.md. All artifacts remain local.

## Round001 redirect active — round001-r01

Input unchanged pass-001-final a10e5d449f1da16d2fcb15bb5c2ee2514d66c7caae0b953ac076906bf7de2b54,59.93% classified (157105 bytes),105055 unknown. Current-source commentary gate complete. Research/frontier: analysis/redirect/round001-r01/frontier.md. Investigating connected trigger/spawn/dispatch structure and independent script/panel, mixed CHR and audio avenues. Classification/source unchanged; no next phase started.

## Round001 redirect complete — combined ready batch

Input/output checkpoint pass-001-final unchanged, state/current map SHA256 a10e5d449f1da16d2fcb15bb5c2ee2514d66c7caae0b953ac076906bf7de2b54. Classification remains59.93%:15126Code+141979Data/Image=157105classified,105055unknown. Redirect accepted/classified0 bytes. Proposed23995 distinct unknown bytes (6950Code hypotheses,17045Data) require independent disassembly review;81110would remain only if all proposals accepted.

Investigated connected allocators/inline records, spawn/state callbacks, mixed4/6-byte triggers and event/transition helpers; independent panel18/23-index domains/scripts, CHR5 bounded copy-template container and audio pitch/SFX/music consumers. Fully parsed music graph40sections/40phrases/342note streams without errors or payload holes. Corrected overrun pointer hypothesis at9C95 and long phrase index/overlap premises against primary evidence. Source-supported artificial stage0A route and alternate lives0/substage5 terminal route tested; no identified boss/credits or natural-reachability claim. Exact patches/budgets/context/outcomes retained in attempt and scenarios.json.

L005–L008 materially improved to ready; new L009–L011 ready. Leads preserve old outcomes. Combined proposal, primary evidence, reproducible recipes, exact unknown complement and actual local deferral reasons: analysis/redirect/round001-r01/. Full handoff there; compact analysis/round001/redirect-handoff.md. Current-source commentary/byte-identical gate remains valid because source, map, state and ROM hashes unchanged. No new phase, archive, publication, cleanup or toolkit edits. No toolkit issues found;8.2GiB free, shared scratch108KiB. Next: independent disassembly of combined batch, then full current-source commentary and redirect. This is productive phase completion, not terminal exhaustion.

### Coordinator acceptance and arithmetic correction

Round001 redirect findings and primary artifact references inspected: productive combined ready batch, not terminal exhaustion. Independent disassembly next. Historical handoff/progress conditional remainder 81110 is an arithmetic typo: 105055 - 23995 = 81060, as proposal-validation.json correctly records. Accepted classification remains unchanged. Completed attempt records preserved.

## Round002 disassembly active

Input pass-001-final/state/current map SHA256 a10e5d449f1da16d2fcb15bb5c2ee2514d66c7caae0b953ac076906bf7de2b54;157105 classified,105055 unknown. Independently reviewing combined L005–L011 proposal23995 bytes and connected continuations against primary consumers, inline/stack barriers and mapper contracts. No baseline reinitialization; commentary/source preserved. Frontier and review evidence: analysis/round002/. No next phase started.

## Round002 disassembly accepted batch

ROM classification: 69.12% (181217 / 262160 bytes classified; 80943 bytes unknown).

Accepted24112 new bytes:6992 Code and17120 Data. All23995 proposed bytes passed independent review; ordinary bank-service continuations add42 Code and adjacent inspected resource data add75 Data. map.validate passed, state/current exported. Primary review: analysis/round002/review.md; exact accepted inventory: accepted.json; exact unknown complement and per-bank counts: counts.json. New annotations Static; prior provenance and commentary/source preserved. No runtime replay or natural-reachability expansion. Checkpoint verification next.

| Bank | Code | Data/Image | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG00 | 1637 | 1253 | 5302 |
| PRG01 | 1339 | 4624 | 2229 |
| PRG02 | 1890 | 1932 | 4370 |
| PRG03 | 1096 | 2384 | 4712 |
| PRG04 | 0 | 0 | 8192 |
| PRG05 | 1031 | 1014 | 6147 |
| PRG06 | 2341 | 36 | 5815 |
| PRG07 | 2527 | 289 | 5376 |
| PRG08 | 3430 | 3455 | 1307 |
| PRG09 | 0 | 8177 | 15 |
| PRG10 | 0 | 163 | 8029 |
| PRG11 | 0 | 5553 | 2639 |
| PRG12 | 0 | 960 | 7232 |
| PRG13 | 0 | 2836 | 5356 |
| PRG14 | 3961 | 75 | 4156 |
| PRG15 | 2866 | 1969 | 3357 |
| CHR00 | 0 | 4096 | 0 |
| CHR01 | 0 | 4096 | 0 |
| CHR02 | 0 | 4096 | 0 |
| CHR03 | 0 | 4096 | 0 |
| CHR04 | 0 | 2562 | 1534 |
| CHR05 | 0 | 4096 | 0 |
| CHR06 | 0 | 2837 | 1259 |
| CHR07 | 0 | 180 | 3916 |
| CHR08 | 0 | 4096 | 0 |
| CHR09 | 0 | 4096 | 0 |
| CHR10 | 0 | 4096 | 0 |
| CHR11 | 0 | 4096 | 0 |
| CHR12 | 0 | 4096 | 0 |
| CHR13 | 0 | 4096 | 0 |
| CHR14 | 0 | 4096 | 0 |
| CHR15 | 0 | 4096 | 0 |
| CHR16 | 0 | 4096 | 0 |
| CHR17 | 0 | 4096 | 0 |
| CHR18 | 0 | 4096 | 0 |
| CHR19 | 0 | 4096 | 0 |
| CHR20 | 0 | 4096 | 0 |
| CHR21 | 0 | 4096 | 0 |
| CHR22 | 0 | 4096 | 0 |
| CHR23 | 0 | 4096 | 0 |
| CHR24 | 0 | 4096 | 0 |
| CHR25 | 0 | 4096 | 0 |
| CHR26 | 0 | 4096 | 0 |
| CHR27 | 0 | 4096 | 0 |
| CHR28 | 0 | 4096 | 0 |
| CHR29 | 0 | 4096 | 0 |
| CHR30 | 0 | 4096 | 0 |
| CHR31 | 0 | 4096 | 0 |

## Round002 disassembly complete — incomplete ROM

ROM classification: 69.12% (181217 / 262160 bytes classified; 80943 bytes unknown).

Fresh checkpoint pass-002-final validated and rebuilt byte-identical for262160 bytes; state/current/checkpoint SHA256 b87f350b989813988facedc2b41e1bc591cbeeedc0d0a5cab9d0e01f566e944f. Verification: analysis/pass-002-final-verification.json. Exact review/accepted views/counts/unknown complement: analysis/round002/. L005–L011 verified with preserved historical attempts and localized remaining questions. Static new annotations do not expand natural/observed execution claims. Existing commentary source hash37a739f50caa0fe9919aa891eed528de58e59bce6983fc2f0a37b048c2780af1 unchanged; its old-map readability credit needs carry-forward and full current-map review next. Final editorial audit pending.

Ordinary easy-gain stop: saved broad gameplay and new static contracts reused, no unchanged terminal waits. Boss/credits selection, complexA040 schema, unrelated graphics resource endpoints and other mixed CHR formats remain useful future redirect questions. This is not terminal exhaustion. Compact handoff: analysis/round002/handoff.md. No commentary/redirect started. No toolkit issues found; analysis/toolkit-issues.md.8.2GiB free, scratch108KiB. No ROM/toolkit changes, archive, publication or cleanup.

Round002 commentary ACTIVE: entire old commentary preserved under analysis/round-002/prior-commentary-round001 before fresh current baseline. Exact-range source and review carry in progress;6992 new Code and17120 new Data bytes pending. No redirect.

Round002 commentary batch1 saved: all6992 new Code+17120 new Data now source-annotated; zero pending classified regions in full4287-entry scaffold. Fresh byte-identical verify passed. ACTIVE consistency/readability compaction and fresh final verify/read remain; no redirect/final editorial audit.

Round002 commentary COMPLETE for current-source readability:22118 Code+159099 Data/Image bytes and80943 explained unknown bytes;4287 reviewed scaffold entries/550 units, zero pending. Entire prior commentary retained at analysis/round-002/prior-commentary-round001. Fresh byte-identical verification passed; final source hash9925f61269c76ed79ab4cf8b93fb91822fdb855859a8f60b48a14f5176fc9e58; reading copy refreshed. Compact handoff analysis/round002/commentary-handoff.md. Whole-ROM classification incomplete; terminal editorial audit pending. No redirect/archive/publish/cleanup/toolkit or structural-map changes.

## Round002 redirect active — round002-r01
Input pass-002-final b87f350b989813988facedc2b41e1bc591cbeeedc0d0a5cab9d0e01f566e944f; full current-source commentary verified. Research frontier: analysis/redirect/round002-r01/frontier.md. No map/source changes.

## Round002 redirect complete — combined ready batch
Attempt analysis/redirect/round002-r01; input/output accepted pass-002-final/state/map b87f350b989813988facedc2b41e1bc591cbeeedc0d0a5cab9d0e01f566e944f unchanged. Accepted181217/262160=69.12%,80943 unknown. Added ready L012/L013/L014; preserved L001..L011 and outcomes. Proposed40434 distinct bytes (1065Code,37081Data,2288Image), accepted0. Source9925f61269c76ed79ab4cf8b93fb91822fdb855859a8f60b48a14f5176fc9e58 unchanged; full current readability retained.
Corrected CB02 selector/bank premise yields bounded graphics-resource Data and full arithmetic planes; coherent factory script/helper/VRAM graph closes new direct endpoints; mixedCHR visual/alternate-directory/generic storage claims localized. Primary findings, exact unknown ledger and510 proposal entries saved; no source/map/ROM/toolkit mutation or runtime replay. Boss/ending discriminator investigated statically, unchanged waiting/flag permutations not justified. No terminal exhaustion claim. No toolkit issues found. Next phase independent disassembly review of combined L012-L014; compact handoff analysis/redirect/round002-r01/handoff.md.

## Round003 disassembly ACTIVE
Independent review of L012–L014 combined batch; input pass-002-final b87f350b989813988facedc2b41e1bc591cbeeedc0d0a5cab9d0e01f566e944f. Frontier analysis/round003/frontier.md. Source and prior evidence preserved.

## Round003 disassembly accepted batch

ROM classification: 84.55% (221651 / 262160 bytes classified; 40509 bytes unknown).

Accepted40434 new bytes:1065 Code,37081 Data,2288 Image. All510 exact intervals independently reviewed and annotated Static; existing classifications/source unchanged. Evidence: analysis/round003/review.md and independent-review.json; exact accepted views accepted.json; exact complement counts.json. Fresh pass-003-final validates and rebuilds byte-identical.

| Bank | Code | Data/Image | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG00 | 1637 | 1253 | 5302 |
| PRG01 | 1339 | 4624 | 2229 |
| PRG02 | 2338 | 2449 | 3405 |
| PRG03 | 1530 | 2524 | 4138 |
| PRG04 | 0 | 8186 | 6 |
| PRG05 | 1031 | 2393 | 4768 |
| PRG06 | 2341 | 36 | 5815 |
| PRG07 | 2527 | 289 | 5376 |
| PRG08 | 3459 | 3455 | 1278 |
| PRG09 | 0 | 8177 | 15 |
| PRG10 | 0 | 8186 | 6 |
| PRG11 | 0 | 8177 | 15 |
| PRG12 | 0 | 8186 | 6 |
| PRG13 | 0 | 6361 | 1831 |
| PRG14 | 4084 | 1099 | 3009 |
| PRG15 | 2897 | 1985 | 3310 |
| CHR00 | 0 | 4096 | 0 |
| CHR01 | 0 | 4096 | 0 |
| CHR02 | 0 | 4096 | 0 |
| CHR03 | 0 | 4096 | 0 |
| CHR04 | 0 | 4096 | 0 |
| CHR05 | 0 | 4096 | 0 |
| CHR06 | 0 | 4096 | 0 |
| CHR07 | 0 | 4096 | 0 |
| CHR08 | 0 | 4096 | 0 |
| CHR09 | 0 | 4096 | 0 |
| CHR10 | 0 | 4096 | 0 |
| CHR11 | 0 | 4096 | 0 |
| CHR12 | 0 | 4096 | 0 |
| CHR13 | 0 | 4096 | 0 |
| CHR14 | 0 | 4096 | 0 |
| CHR15 | 0 | 4096 | 0 |
| CHR16 | 0 | 4096 | 0 |
| CHR17 | 0 | 4096 | 0 |
| CHR18 | 0 | 4096 | 0 |
| CHR19 | 0 | 4096 | 0 |
| CHR20 | 0 | 4096 | 0 |
| CHR21 | 0 | 4096 | 0 |
| CHR22 | 0 | 4096 | 0 |
| CHR23 | 0 | 4096 | 0 |
| CHR24 | 0 | 4096 | 0 |
| CHR25 | 0 | 4096 | 0 |
| CHR26 | 0 | 4096 | 0 |
| CHR27 | 0 | 4096 | 0 |
| CHR28 | 0 | 4096 | 0 |
| CHR29 | 0 | 4096 | 0 |
| CHR30 | 0 | 4096 | 0 |
| CHR31 | 0 | 4096 | 0 |

## Round003 disassembly complete — incomplete ROM

ROM classification: 84.55% (221651 / 262160 bytes classified; 40509 bytes unknown).

Fresh checkpoint pass-003-final validated and rebuilt byte-identical for262160 bytes; state/current/checkpoint SHA256 a5fd2a6856cc5964af3fe38872eacf33f3dbea93b29eb983e72f3e0d294477bc. Independent exact Code decoding and primary consumer/resource/CHR review passed; all40434 proposal bytes accepted Static. Source 9925f61269c76ed79ab4cf8b93fb91822fdb855859a8f60b48a14f5176fc9e58 unchanged; full current-map commentary pending for new1065Code+37081Data+2288Image. Lead histories preserved; broad gameplay artifacts reused without unchanged replay. Ordinary easy-gain stop, not terminal exhaustion. Compact handoff analysis/round003/handoff.md. No commentary/redirect started. No toolkit issues found; analysis/toolkit-issues.md. No toolkit/ROM/source edits, archive, publication or cleanup.

Round003 commentary ACTIVE: preserved entire previous commentary at analysis/round-003/prior-commentary-round002; fresh verified current baseline prepared. Physical source carry applied; review.json pending frontier covers new classifications and affected dependencies. No redirect or final editorial audit.

Round003 commentary ACTIVE: preserved entire previous commentary at analysis/round-003/prior-commentary-round002; fresh verified current baseline prepared. Physical source carry applied; review.json pending frontier covers new classifications and affected dependencies. No redirect or final editorial audit.

Round003 commentary ACTIVE: preserved entire previous commentary at analysis/round-003/prior-commentary-round002; fresh verified current baseline prepared. Physical source carry applied; review.json pending frontier covers new classifications and affected dependencies. No redirect or final editorial audit.

Round003 commentary ACTIVE: preserved entire previous commentary at analysis/round-003/prior-commentary-round002; fresh verified current baseline prepared. Physical source carry applied; review.json pending frontier covers new classifications and affected dependencies. No redirect or final editorial audit.

Round003 commentary COMPLETE for current source:4790 reviewed scaffold entries/619 units, zero pending;23183Code+81492Data+116976Image with40509 unknown bytes explained. Fresh byte-identical verification and reading refresh complete. Compact handoff analysis/round003/commentary-handoff.md; no redirect started. Whole-ROM classification84.55%; final editorial audit pending.

Round003 redirect ACTIVE — analysis/redirect/round003-r01/frontier.md. Input pass-003-final a5fd2a6856cc5964af3fe38872eacf33f3dbea93b29eb983e72f3e0d294477bc; current-source gate complete. Read-only structural research, no accepted byte changes.

## Round003 redirect complete — ready combined batch

Input pass-003-final a5fd2a6856cc5964af3fe38872eacf33f3dbea93b29eb983e72f3e0d294477bc. Accepted classification84.55% (221651/262160);40509 unknown/313 ranges unchanged. Proposed36989 new Static bytes=19506Code+17483Data asreadyL015-L017; accepted new0. Fresh configureddecode8585 exactCodeentries/322 boundedcalls/zeroerrors. Exactledger andcompacthandoff analysis/redirect/round003-r01/handoff.md; contracts/mixed-gap corrections findings.md. Proposal leaves3520 bytes/121subintervals outsidebatch; not terminalexhaustion. Current-source gate andsourcehash d98197e634454628f70120490f5481c472e0505e7cd9ff66f2c629305096b368 unchanged. No runtime visit/map/source/ROM/toolkit edits,archive,publish,cleanup. Next independentdisassemblyreview required. No toolkit issues found.


Round004 disassembly ACTIVE — input pass-003-final/state/current a5fd2a6856cc5964af3fe38872eacf33f3dbea93b29eb983e72f3e0d294477bc; analysis/round004/frontier.md. Independently reviewing ready L015–L017 exact intervals, physical views and scene/stack/bank/data contracts. Baseline preserved; no runtime replay or accepted map changes yet.


## Round004 disassembly complete — incomplete ROM

ROM classification: 98.66% (258640 / 262160 bytes classified; 3520 bytes unknown).

Accepted36989 new Static bytes:19506 Code+17483 Data, all8712 independently reviewed exact intervals. Independent configured decoding8585 Code entries/322 calls/zero errors supplements primary scene/actor/stack/bank/geometry/queue/consumer evidence. Review: analysis/round004/review.md and independent-review.json; exact annotations accepted.json; exact complement counts.json. Fresh pass-004-final validates and rebuilds all262160 bytes byte-identical. ROM and entire prior readable source remain unchanged. Full current-map commentary is required for new classifications; final editorial audit remains pending. No new runtime execution or milestone visit; preserved broad progression evidence reused. Ordinary easy-gain stop, not terminal exhaustion; no supported connected continuation omitted at initial bounds. No toolkit issues found; analysis/toolkit-issues.md. No toolkit/ROM/source changes, archive, publication or cleanup.

| Bank | Code | Data/Image/Header | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG00 | 3886 | 4040 | 266 |
| PRG01 | 1940 | 5931 | 321 |
| PRG02 | 3809 | 4136 | 247 |
| PRG03 | 2479 | 5709 | 4 |
| PRG04 | 6 | 8186 | 0 |
| PRG05 | 3439 | 4573 | 180 |
| PRG06 | 5534 | 2451 | 207 |
| PRG07 | 6376 | 1207 | 609 |
| PRG08 | 4125 | 3816 | 251 |
| PRG09 | 9 | 8183 | 0 |
| PRG10 | 6 | 8186 | 0 |
| PRG11 | 9 | 8183 | 0 |
| PRG12 | 6 | 8186 | 0 |
| PRG13 | 9 | 8183 | 0 |
| PRG14 | 6174 | 1575 | 443 |
| PRG15 | 4882 | 2318 | 992 |
| CHR00 | 0 | 4096 | 0 |
| CHR01 | 0 | 4096 | 0 |
| CHR02 | 0 | 4096 | 0 |
| CHR03 | 0 | 4096 | 0 |
| CHR04 | 0 | 4096 | 0 |
| CHR05 | 0 | 4096 | 0 |
| CHR06 | 0 | 4096 | 0 |
| CHR07 | 0 | 4096 | 0 |
| CHR08 | 0 | 4096 | 0 |
| CHR09 | 0 | 4096 | 0 |
| CHR10 | 0 | 4096 | 0 |
| CHR11 | 0 | 4096 | 0 |
| CHR12 | 0 | 4096 | 0 |
| CHR13 | 0 | 4096 | 0 |
| CHR14 | 0 | 4096 | 0 |
| CHR15 | 0 | 4096 | 0 |
| CHR16 | 0 | 4096 | 0 |
| CHR17 | 0 | 4096 | 0 |
| CHR18 | 0 | 4096 | 0 |
| CHR19 | 0 | 4096 | 0 |
| CHR20 | 0 | 4096 | 0 |
| CHR21 | 0 | 4096 | 0 |
| CHR22 | 0 | 4096 | 0 |
| CHR23 | 0 | 4096 | 0 |
| CHR24 | 0 | 4096 | 0 |
| CHR25 | 0 | 4096 | 0 |
| CHR26 | 0 | 4096 | 0 |
| CHR27 | 0 | 4096 | 0 |
| CHR28 | 0 | 4096 | 0 |
| CHR29 | 0 | 4096 | 0 |
| CHR30 | 0 | 4096 | 0 |
| CHR31 | 0 | 4096 | 0 |

Round004 commentary ACTIVE: preserved entire previous commentary at analysis/round-004/prior-commentary-round003; fresh verified current baseline prepared. Physical source carry applied; review.json pending frontier covers new classifications and affected dependencies. No redirect or final editorial audit.

Round004 commentary batches1–3 ACTIVE: entire round003 commentary preserved at analysis/round-004/prior-commentary-round003; fresh baseline and carried source verified byte-identical. Batch1 reviewed5276 new Code bytes; batch2 reviewed2414. Batch3 adds actor movement/phase/factory/request routines through frozen group156; fresh corrected verification saved analysis/round004/commentary-batch3-corrected-verification.json. Exact remaining Code/Data scaffold entries retain pending status. Multi-entry prose/branch consistency and carried dependency corrections remain required before final verification/read. No redirect or terminal editorial audit, ROM/map/toolkit mutation, archive or publication.

Round004 commentary first-pass coverage now all36989 new classified bytes:19506 Code in302 coherent fragments and17483 Data in127 structures, with successful batch4/batch5/data byte-identical reports. Active remaining work is localized multi-entry routine/branch prose, exact CPU data and bank aliases, carried allocator/script/geometry/lookup consistency, source-location/scope updates and fresh final verification/reading copy. Pending editorial consistency prevents handoff despite reviewed scaffold statuses; no redirect or terminal audit.

Round004 commentary COMPLETE for current-source readability:258640 classified bytes plus121 explained unknown ranges/3520bytes; classification remains98.66% incomplete. Source inventory1880 exact physical spans;13310 scaffold entries intact;619 inherited+569 current review units, no pending readability. Localized entry/branch, CPU-vs-physical alias and carried allocator/script/bank/geometry/queue/text/lookup/CHR consistency finished. Fresh final byte-identical verification analysis/round004/commentary-final-verification.json and reading refresh passed; source hash52a3bc9c124b14652b9a19261aa4b7c71ed08f264bd5da77850dc69fd046f9f5. Compact scope/handoff analysis/round004/commentary-completion.json and commentary-handoff.md. Entire previous commentary preserved analysis/round-004/prior-commentary-round003. Final editorial audit pending; no next redirect, runtime/map/ROM/toolkit changes, archive/publication/cleanup. No toolkit issues found.

Round004 redirect active: analysis/redirect/round004-r01/frontier.md. Input pass-004-final; no proposals yet.

Round004 redirect COMPLETE ready combinedbatch (round004-r01): L018/L019/L020 added, proposed3486Static=3307Code179Data; acceptednew0. Localizedfield/stack/bank/control contracts, currentD71Ecallroots, connectedpalette/script/transitions andindependentparameter/resourcecells investigated. Configured1535instructionentries/89boundeddecodecalls/errors0; allresolvabledirecttargets checkedagainstinstructionboundaries. Exactacceptedunknown121ranges3520bytes; unproposed34bytes29isolatedRTSfragments afterproposal, no pointer-alonepromotion. Currentmap/state/source/ROM unchanged; source currentreadability gate remainsvalid, classification98.66%. Handoff analysis/redirect/round004-r01/handoff.md. Ready independentdisassemblyreview; no nextphase/finalaudit/archive/publish/cleanup, no terminalexhaustionclaim.


Round005 disassembly ACTIVE: analysis/round005/frontier.md. Input pass-004-final; independent contract/context/extent review of L018–L020 underway.


## Round005 disassembly complete — incomplete ROM

ROM classification: 99.99% (262125 / 262160 bytes classified; 35 bytes unknown).

Accepted3485 new Static bytes:3306Code+179Data;1534 exact Code instruction annotations and27 Data intervals. Fresh configured1535-entry/89-call decode zeroerrors included one rejectedRTS; independent355 directROMtargets (threebankcalls resolved explicitly) and ordinary continuations zeroerrors. Review analysis/round005/review.md, reviewed-contracts.json, independent-review.json; exact acceptedrecords accepted.json; perbankcounts/exactcomplement counts.json. Claim109/E5E6 rejected: E5F9/E606 targets areE5E1. A55B..55E genericData claim narrowed; no complete selectedqueue-directory claim. Other mechanical index/stack/carry preconditions corrected. Original proposal/evidence preserved.

Fresh pass-005-final validates and rebuilds all262160 bytes byte-identical; analysis/pass-005-final-verification.json. All35unknown bytes remain60opcode fragments in29ranges; neither code nor padding inferred. Entire prior currentcommentarysource preserved (SHA256 52a3bc9c124b14652b9a19261aa4b7c71ed08f264bd5da77850dc69fd046f9f5), but new3306Code/179Data require full current-source commentary. Final editorial audit pending. Existing input unchanged. Structural classification is distinct from execution coverage: no new execution observations/route replay, historical ObservedCode structural bytes 11045. Annotation coverage: {'evidence_bytes': 262125, 'comment_bytes': 254060}. Prior broad progression/scenarios retained; no new concrete successful boss/credits condition. Ordinary easy-gain stop, not terminal exhaustion. No toolkit issues found; analysis/toolkit-issues.md. No toolkit/ROM/source changes, archive, publication, cleanup or nextphase work.

| Bank | Code | Data/Image/Header | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG00 | 4117 | 4074 | 1 |
| PRG01 | 2216 | 5969 | 7 |
| PRG02 | 4035 | 4148 | 9 |
| PRG03 | 2479 | 5709 | 4 |
| PRG04 | 6 | 8186 | 0 |
| PRG05 | 3592 | 4595 | 5 |
| PRG06 | 5729 | 2463 | 0 |
| PRG07 | 6949 | 1243 | 0 |
| PRG08 | 4371 | 3820 | 1 |
| PRG09 | 9 | 8183 | 0 |
| PRG10 | 6 | 8186 | 0 |
| PRG11 | 9 | 8183 | 0 |
| PRG12 | 6 | 8186 | 0 |
| PRG13 | 9 | 8183 | 0 |
| PRG14 | 6607 | 1584 | 1 |
| PRG15 | 5855 | 2330 | 7 |
| CHR00 | 0 | 4096 | 0 |
| CHR01 | 0 | 4096 | 0 |
| CHR02 | 0 | 4096 | 0 |
| CHR03 | 0 | 4096 | 0 |
| CHR04 | 0 | 4096 | 0 |
| CHR05 | 0 | 4096 | 0 |
| CHR06 | 0 | 4096 | 0 |
| CHR07 | 0 | 4096 | 0 |
| CHR08 | 0 | 4096 | 0 |
| CHR09 | 0 | 4096 | 0 |
| CHR10 | 0 | 4096 | 0 |
| CHR11 | 0 | 4096 | 0 |
| CHR12 | 0 | 4096 | 0 |
| CHR13 | 0 | 4096 | 0 |
| CHR14 | 0 | 4096 | 0 |
| CHR15 | 0 | 4096 | 0 |
| CHR16 | 0 | 4096 | 0 |
| CHR17 | 0 | 4096 | 0 |
| CHR18 | 0 | 4096 | 0 |
| CHR19 | 0 | 4096 | 0 |
| CHR20 | 0 | 4096 | 0 |
| CHR21 | 0 | 4096 | 0 |
| CHR22 | 0 | 4096 | 0 |
| CHR23 | 0 | 4096 | 0 |
| CHR24 | 0 | 4096 | 0 |
| CHR25 | 0 | 4096 | 0 |
| CHR26 | 0 | 4096 | 0 |
| CHR27 | 0 | 4096 | 0 |
| CHR28 | 0 | 4096 | 0 |
| CHR29 | 0 | 4096 | 0 |
| CHR30 | 0 | 4096 | 0 |
| CHR31 | 0 | 4096 | 0 |

Round005 commentary ACTIVE: preserved entire previous commentary at analysis/round-005/prior-commentary-round004; fresh verified current baseline prepared. Physical source carry applied; review.json pending frontier covers new classifications and affected dependencies. No redirect or final editorial audit.

Round005 commentary ACTIVE verified coherent batch: all 3306 new Code +179 Data bytes now have first-pass prose in 117 compact units; byte-identical verification after control/subentry refinement passed. Sound carried review retained (569 top-level,619 embedded units). Gate still active: alias/local-entry and affected carried-dependency consistency, honest residual notes, exact inventory/source refresh and final verify/read remain. Scaffold reviewed statuses are coverage bookkeeping, not phase completion. Entire old commentary preserved at analysis/round-005/prior-commentary-round004.

Round005 commentary COMPLETE current-source readability:45995Code+99154Data+116976Image,35Unknown/29ranges explicitly explained. New3306StaticCode+179Data reviewed in117 coherent units; valid569 top-level+619 embedded prior units carried.14779-entry scaffold and1905-span exact source inventory refreshed,0 pending readability. Fresh final byte-identical verify262160bytes; source SHA25686617be0edc2380ff8c3114a26453679fc8b9cfdc4abfbd8b31c789d139507e6, reading refreshed. Map/state hash0cecafc728ba0e4f20d8722f3e6771ce0f38a3ff89e6318a0d41cbac0a4e2f07 unchanged. Full old tree analysis/round-005/prior-commentary-round004 preserved; scope commentary/scope.json, handoff analysis/round005/commentary-handoff.md. ROM classification remains INCOMPLETE99.99%; final editorial audit pending. No redirect/archive/publish/cleanup. No toolkit issues found.

Final localized correction reverified: B2DF BCC reaches B2FA JMP B361. The failure path pops saved slot into A but leaves fallback X unchanged; B2FD is TXA/TAY, not TAX. Source prose/branch name corrected, fresh final verify byte-identical and reading refreshed again. Current final game.asm SHA256df9559083a4d1fd1d42e789b60e7a4dc7a20e86bee4930370ecae49fe91233bc; current scope/handoff reflect this refinement. Readability complete, no remaining frontier; classification and final editorial acceptance remain unchanged.

Round005 redirect ACTIVE: unique attempt analysis/redirect/round005-r01; current-source gate and checkpoint inherited verified. Frontier: localized return-versus-separator contracts, independent current-source reference/context check, precise counted indirect selectors, corrected B2 fallback dependencies. No classification or terminal claim.

Round005 redirect READY combined batch in analysis/redirect/round005-r01/handoff.md: L021 one Static Code byte physical71932/CPU98EC [8,9,14,15], independently immediate-installed sound callback; L022 targeted annotation correction61 current Code regions/132 bytes for falseB2FA TAX wording. Proposednew1/acceptednew0. Current262125/262160=99.99%,35Unknown/29ranges unchanged; exact unknown-ledger/current-source excerpts, configured decode primaries and pointer/context checks retained. Scene writer STX86C9 included, initializer not success-linked; no new runtime. Map/state0cecafc728ba0e4f20d8722f3e6771ce0f38a3ff89e6318a0d41cbac0a4e2f07 and sourcedf9559083a4d1fd1d42e789b60e7a4dc7a20e86bee4930370ecae49fe91233bc unchanged. No terminal claim or final audit; independent disassembly next. No toolkit issues found.


Round006 disassembly ACTIVE: independent review of L021 producer/setter/dispatcher/ordinary return contract and L022 PLA/BCC/JMP correction accepted in principle; current map/state unchanged. Frontier: fresh configured decode, supported metadata-only edits, exact identity preservation, validate/save/export/fresh checkpoint. No runtime replay, source edit or next phase.


## Round006 disassembly complete — incomplete ROM

ROM classification: 99.99% (262126 / 262160 bytes classified; 34 bytes unknown).

L021 verified one new Static Code byte, physical71932 CPU$98EC effective banks[8,9,14,15]. Independent configured decoding matched producer/setter/dispatch/RTS primary contracts; $98ED remains existing APU Data. Natural selection and observed execution unresolved. L022 verified metadata correction:61 existing Code comments/132bytes, only false failure-TAX wording replaced. Every old ID, bounds, kind, provenance, confidence, execution view, evidence and label preserved. Historical round005 review/contract records remain immutable; review.md appends the erratum. Audit annotation-corrections-applied.json and accepted.json. All connected sound continuations already classified; no additional supported interval.

Incomplete:34 Unknown bytes/28 ranges; exact complement/counts in counts.json, inherited scoped neighbors in unknown-ledger.json. Remaining standalone RTS ambiguity remains; no Data inference from absent calls, no Code inference from opcode alone. Prior direct-edge, immediate-pair, actual count-delimited entry-minus1 and unaligned-word negatives retain only their stated scope (analysis/redirect/round005-r01/findings.md and primaries). No unchanged runtime/waits or arbitrary RAM experiments. Prior broad gameplay checklist/negative ending attempts preserved; no changed source-supported successful scenario.

Configured validation/save/export and fresh unique pass-006-final byte-identical rebuild of262160bytes passed; analysis/pass-006-final-verification.json. Source unchanged df9559083a4d1fd1d42e789b60e7a4dc7a20e86bee4930370ecae49fe91233bc; no source editing occurred. Full current-source commentary must follow for new byte and metadata consistency, retaining complete current source/history; final editorial audit pending. No next phase, terminal exhaustion, toolkit modification, publication, archive or cleanup. No toolkit issues found; analysis/toolkit-issues.md.

Annotation coverage: {'evidence_bytes': 262126, 'comment_bytes': 254061}. Historical Observed Code bytes 11045, new execution observations0.

| Bank | Code | Data/Image/Header | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG00 | 4117 | 4074 | 1 |
| PRG01 | 2216 | 5969 | 7 |
| PRG02 | 4035 | 4148 | 9 |
| PRG03 | 2479 | 5709 | 4 |
| PRG04 | 6 | 8186 | 0 |
| PRG05 | 3592 | 4595 | 5 |
| PRG06 | 5729 | 2463 | 0 |
| PRG07 | 6949 | 1243 | 0 |
| PRG08 | 4372 | 3820 | 0 |
| PRG09 | 9 | 8183 | 0 |
| PRG10 | 6 | 8186 | 0 |
| PRG11 | 9 | 8183 | 0 |
| PRG12 | 6 | 8186 | 0 |
| PRG13 | 9 | 8183 | 0 |
| PRG14 | 6607 | 1584 | 1 |
| PRG15 | 5855 | 2330 | 7 |
| CHR00 | 0 | 4096 | 0 |
| CHR01 | 0 | 4096 | 0 |
| CHR02 | 0 | 4096 | 0 |
| CHR03 | 0 | 4096 | 0 |
| CHR04 | 0 | 4096 | 0 |
| CHR05 | 0 | 4096 | 0 |
| CHR06 | 0 | 4096 | 0 |
| CHR07 | 0 | 4096 | 0 |
| CHR08 | 0 | 4096 | 0 |
| CHR09 | 0 | 4096 | 0 |
| CHR10 | 0 | 4096 | 0 |
| CHR11 | 0 | 4096 | 0 |
| CHR12 | 0 | 4096 | 0 |
| CHR13 | 0 | 4096 | 0 |
| CHR14 | 0 | 4096 | 0 |
| CHR15 | 0 | 4096 | 0 |
| CHR16 | 0 | 4096 | 0 |
| CHR17 | 0 | 4096 | 0 |
| CHR18 | 0 | 4096 | 0 |
| CHR19 | 0 | 4096 | 0 |
| CHR20 | 0 | 4096 | 0 |
| CHR21 | 0 | 4096 | 0 |
| CHR22 | 0 | 4096 | 0 |
| CHR23 | 0 | 4096 | 0 |
| CHR24 | 0 | 4096 | 0 |
| CHR25 | 0 | 4096 | 0 |
| CHR26 | 0 | 4096 | 0 |
| CHR27 | 0 | 4096 | 0 |
| CHR28 | 0 | 4096 | 0 |
| CHR29 | 0 | 4096 | 0 |
| CHR30 | 0 | 4096 | 0 |
| CHR31 | 0 | 4096 | 0 |

## Round006 redirect round006-r01 ACTIVE
Current-source gate accepted;34Unknown/28ranges. Reassessing residual local contracts against decisive primary evidence and scoped prior negatives; frontier analysis/redirect/round006-r01/frontier.md. No classification changes.

## Round006 redirect — terminal incomplete proposal

Resumed existing analysis/redirect/round006-r01 from pass-006-final, after complete current-source commentary. Refreshed exact residual complement:34Unknown bytes/28ranges;262126 classified. Proposed/accepted0/0,no ready leads. Read current source neighbors and primary counted-pointer/constructed-callback/scene-writer evidence; inspected all15 actual indirect-jump consumer neighborhoods for omitted pointer-building alternatives. The strongest repeated$60, wrapper tail, fixed shared-return and data-boundary candidates remain materially Code-versus-Data ambiguous; no universal reachability/fetch prerequisite. Rejection scope and accepted98EC positive comparison are explicit in analysis/redirect/round006-r01/rejection-review.md. Broad gameplay checklist and practical prior stage0A/alternate terminal shortcuts reviewed; no newly supported shortcut or success-linked actor/flag arises, no unchanged runtime replay. Appended leads pass with empty added/updated IDs and scenario review. Terminal incomplete proposed for independent coordinator inspection; final editorial acceptance pending. Handoff:analysis/redirect/round006-r01/handoff.md. No ROM/map/state/source/toolkit changes,archive,publication,deletion or cleanup. No toolkit issues found; no blockers. Disk7.6GiB available,scratch108KiB.

Coordinator accepted round006-r01 terminal-INCOMPLETE reasoning after primary-artifact inspection and independent ROM branch arithmetic; analysis/coordinator-rejection-review.json. Classification99.99%,34Unknown/28ranges, no ready lead. Final editorial acceptance remains required; no archive/cleanup/publication. Prior turn made concrete redirect and archive-selection progress.

## Final editorial acceptance — Round006


Final toolkit audit PASSED: ok=true, byte_identical=true, editorial_audit=true; 14779 reviewed regions and 18892 embedded unit associations (1461 unique units), plus 687 preserved reviewed top-level units. reading-after.asm refreshed. Source hash 0313c6e9e5d5ee6389595e388c031c2e519417632c7f27578091d3f93234ca8e; map/state unchanged 6f60cafafa039c6f901acf0666aa2fa81db83a96594ca1181105c9497520248b. Classification INCOMPLETE: 262126/262160 classified, 34Unknown/28ranges. Evidence: commentary/audit.json, editorial-audit.md, analysis/round006/final-editorial-completion.json. No toolkit issues found.

## Coordinator terminal handoff — justified INCOMPLETE

Sequential round006 redirect rejection review accepted after primary inspection; separate final editorial worker completed fresh audit/byte-identical verification/read refresh. Final classification99.99% (262126/262160),34Unknown/28ranges; no ready lead or productive interrupted frontier. Current-source readability and editorial acceptance complete. Final checkpoint pass-006-final; source0313c6e9e5d5ee6389595e388c031c2e519417632c7f27578091d3f93234ca8e; map/state6f60cafafa039c6f901acf0666aa2fa81db83a96594ca1181105c9497520248b. Current audit analysis/current-audit.json; final analysis/terminal-handoff.md. Archive-selection contract inspected; archive-options.json and analysis/archive-document-selection.json account for all current unrecognized Markdown, preserving final/historical results. Orchestration complete, ROM classification incomplete. No archive, cleanup/deletion, publication or toolkit change. No toolkit issues found.
