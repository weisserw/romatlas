# Disassembly progress

Workspace initialized. Baseline and classification have not run.

## Pass 001 in progress
Baseline maps/baseline.json and generated/baseline verified byte-identical (analysis/baseline-verification.json). Input SHA verified by run.py. Emulator initial effective PRG slots [0,1,14,15], reset PC $FFD0. Vectors at physical $2000A..$2000F encode NMI $C003/reset $FFD0/IRQ $C006. RPC evidence in analysis/pass-001/rpc.jsonl. Classification still in progress; no completion claim.

### Initial classification saved
analysis/state.json and maps/current.json now classify 1,615 PRG code bytes, 131,072 CHR image bytes, 16 header bytes and 6 interrupt-vector bytes (132,709 / 262,160; 129,451 unknown). Startup actual fetch contexts (736 contextual roots / 551 unique offsets) preserved in pass-001/response.json.saved-contexts and rpc.jsonl. Static expansion in pass-001/discovery.json. Observed provenance applies only to saved fetch starts; static code is separate. Emulator process expired before snapshot export; replay restarted with trace disabled. Initial story is long and Start/A did not skip it; replay now at frame1683 and bounded advancement continues toward playable title. Circus Caper US likely; researched controls, boss shortcut, health/boss-health RAM hypotheses in gameplay.md. No RAM patches yet. No toolkit issues found so far. Checkpoint will follow structural/run batch.

Initial evidence-backed map now has verified checkpoint maps/initial-001.json / generated/initial-001, report analysis/initial-001-verification.json (byte identical true). Current trace-disabled replay has reached frame3483 of the animated introductory story; intermediate screenshots in pass-001/advance0..2.png confirm successive dialogue, so this is progressing rather than an emulator hang. Six 600-frame batches remain bounded, with routes saved in rpc.jsonl and advance.json.

### Gameplay and static merge milestone
Scenarios now maintained in analysis/scenarios.json. Screen/init selector experiments1/4/8 yielded accepted artificial fetches, but malformed backgrounds; they are not successful later-level visits. $039F was corrected from an initial stage hypothesis to graphics/init-dispatch selector based on its sole store in bank4 CPU$8000. Parent transition at CPU$C0BB entered story helper CE9C; revised skip-helper entry$C0BE remains next concrete test. Full recorded patches/routes and captures retained. Current static/observed merge autosaves every50 annotations; latest saved classification exceeds137,000 bytes. Final counts/checkpoint follow the batch. This is an ongoing easy-gain pass, not terminal exhaustion; later scenes/bosses/ending remain unconfirmed.

## Pass 001 complete — analysis incomplete
ROM classification: 55.49% (145482 / 262160 bytes classified; 116678 bytes unknown).

Latest verified checkpoint: maps/pass-001-final.json and generated/pass-001-final/game.asm; report analysis/pass-001-final-verification.json confirms byte_identical=true,ok=true. Internal map.validate passed. Input SHA unchanged. 8,118 code,131,072 image,6,292 data;3,648/3,648 regions have evidence;6,274 classified code bytes have observed fetch-start provenance (includes artificial scenarios). Editorial/readability review not performed; full natural execution coverage not claimed. Counts/unknown complement analysis/pass-001/counts.json. Saved authoritative handoff analysis/pass-001/handoff.md.

Natural story progressed through frame4683; normal left input moved sprite on black background, no complete ordinary level confirmed. Artificial selector1/4/8 and parent-entry experiments retained precise evidence but did not establish later-level/ending visits. The display-selector correction and missing event/room contract are recorded in scenarios.json, investigation.md and leads.json. Main-late artificial roots were retained but not blindly merged. Existing evidence, maps, source checkpoints and snapshots retained.

Lead outcomes: D001 validated vector/reset paths; D00274generic constant-data runs/6242bytes; D003 supported screen-init dispatch/code and rejected selector-only progression premise. R001 parent scene/state blocked on explicit contract; R002 NMI/audio dispatch and R003 record-data consumers are concrete redirect-ready avenues. This is an initial substantive batch handoff, not terminal exhaustion; broad gameplay remains incomplete and useful research remains. No toolkit issues found; see analysis/toolkit-issues.md. No toolkit edits, ROM edits, archive or publishing.
ROM classification: 55.49% (145482 / 262160 bytes classified; 116678 bytes unknown).

| Bank | Code | Data/image | Unknown |
|---|---:|---:|---:|
| header | 0 | 16 | 0 |
| PRG00 | 2267 | 28 | 5897 |
| PRG01 | 2356 | 1240 | 4596 |
| PRG02 | 108 | 0 | 8084 |
| PRG03 | 4 | 1285 | 6903 |
| PRG04 | 362 | 1430 | 6400 |
| PRG05 | 4 | 281 | 7907 |
| PRG06 | 0 | 1434 | 6758 |
| PRG07 | 4 | 497 | 7691 |
| PRG08 | 1411 | 0 | 6781 |
| PRG09 | 4 | 0 | 8188 |
| PRG10 | 0 | 0 | 8192 |
| PRG11 | 4 | 75 | 8113 |
| PRG12 | 0 | 0 | 8192 |
| PRG13 | 4 | 0 | 8188 |
| PRG14 | 457 | 0 | 7735 |
| PRG15 | 1133 | 6 | 7053 |
| CHR00 | 0 | 8192 | 0 |
| CHR01 | 0 | 8192 | 0 |
| CHR02 | 0 | 8192 | 0 |
| CHR03 | 0 | 8192 | 0 |
| CHR04 | 0 | 8192 | 0 |
| CHR05 | 0 | 8192 | 0 |
| CHR06 | 0 | 8192 | 0 |
| CHR07 | 0 | 8192 | 0 |
| CHR08 | 0 | 8192 | 0 |
| CHR09 | 0 | 8192 | 0 |
| CHR10 | 0 | 8192 | 0 |
| CHR11 | 0 | 8192 | 0 |
| CHR12 | 0 | 8192 | 0 |
| CHR13 | 0 | 8192 | 0 |
| CHR14 | 0 | 8192 | 0 |
| CHR15 | 0 | 8192 | 0 |

## Round 1 commentary in progress
Prepared commentary/ from the unchanged pass-001-final map and verified baseline. Full classified instruction review and initial source naming/comments are saved. See commentary/progress.md for exact remaining same-phase defect sweep and verification. Initial edited build passes byte-identical verification; current-source handoff remains pending until semantic checks and refreshed reading copy. No redirect/disassembly performed.

## Round 1 commentary complete — classification remains incomplete
Full current-source readability completed for all 145482 classified bytes (8118 code,6292 data,131072 CHR), with honest source notes for116678 unknown bytes. 3861 scaffold records and267 meaningful units reviewed; final editorial audit pending. Fresh commentary/verification.json confirms byte_identical=true,ok=true; commentary/reading-after.asm refreshed. Maintained source commentary/source/game.asm + symbols.inc. No classified-source ranges deferred. Exact coverage, examples, unresolved semantics and redirect questions: analysis/handoffs/round1-commentary.md and commentary/uncertainties.md. Authoritative state/map and input SHA unchanged. No toolkit issues found; no toolkit/ROM changes, redirect/disassembly, publishing, archive or cleanup performed. This phase success does not establish whole-ROM completion, natural gameplay coverage or terminal exhaustion.

Round1 redirect started: analysis/redirect/round1-20261126/frontier.md. Exact input complement verified,116678 bytes. Current frontier R002/R003/R001; all research only.

Round1 redirect: stage1/3/6 rendering-safe parent initialization succeeded visually. See analysis/redirect/round1-20261126/findings.md and frontier.md; PC C374, synthetic RTS C0F4, RAM stage2B, room2C, countdown29 fields. Artificial only. Dispatch/graphics batch remains research, map unchanged.

Round1 redirect combined readybatch: {"proposed_code": 8484, "proposed_data": 39443, "proposed_total": 47927, "accepted_new_bytes": 0, "current_unknown": 116678, "prospective_unknown_if_all_accepted": 68751}. Allsixstages/boss/THE END artificial visits recordedscenarios. Safegraph/pointerbankcorrections and rejectedwidegenericdraft retained. See analysis/handoffs/round1-redirect.md. ROM,map,state,commentarysource unchanged.

Round1 redirect final counts supersede provisional packaging: {"proposed_code": 8882, "proposed_data": 41898, "proposed_total": 50780, "accepted_new_bytes": 0, "current_unknown": 116678, "prospective_unknown_if_all_accepted": 65898}. Handoff analysis/handoffs/round1-redirect.md, handoff-checks.json confirms preserved map/state/ROM and conflict-free proposal. Five ready leads. No terminal exhaustion claim; no toolkit issues found.

## Round 2 disassembly in progress
Restored review frontier at unchanged pass-001-final checkpoint:145482 classified/116678 unknown. Independent review of final safe graphs, primary consumers, inline return handling and fixed/lower-bank contracts precedes annotations. Current question: distinguish coherent proposed control-flow from accidental JSR fallthrough across inline or bank-changing calls; inspect practical generic asset boundaries rather than trusting manifest totals. Evidence and decisions will persist in analysis/pass-002/. No new accepted bytes yet.

Round2 disassembly milestone: all50780manifest bytes accepted after primary decode/fetch/inline/bank/pattern review. Connected consumer tables, generic metatile/scalar gap review and final bank4 graphics loop closure added3326bytes; total54106newbytes. Current199588classified/62572unknown (76.13%); state/map saved/validated. See analysis/pass-002/review.md and frontier.md. Interim pass-002-final checkpoint retained; fresh final verification pending after closure.

## Round 2 disassembly complete — analysis incomplete
ROM classification: 76.13% (199588 / 262160 bytes classified; 62572 bytes unknown).

Accepted54106newbytes (8890code,45216data): all50780reviewed manifest bytes plus3326connected consumer/pattern/loop bytes. Totals17008code,51508data,131072CHRimage;8152/8152regions have evidence. All prior3648regions unchanged. Exact physical unknown complement and per-bank counts: analysis/pass-002/counts.json and final-checks.json. Allfive ready leads R001..R005 consumed/verified; exact per-lead outcomes in lead-outcomes.json. Structural evidence is separate from reachability;8562codebytes/3840instructionstarts have Observed provenance across passes, including artificial routes; whole-game natural execution coverage unclaimed.

Fresh checkpoint maps/pass-002-verified.json, generated/pass-002-verified/game.asm and analysis/pass-002-verified-verification.json passed mapvalidation and byte-identical rebuild (ok=true,byte_identical=true). State/map/checkpoint bytes identical, original inputSHAunchanged. Earlier pass-002-final verified checkpoint retained as intermediate history before graphics gap closure; pass-001-final remains immutable. Maintained commentary/source remains round1 and next phase must migrate/review all current classified source; final editorial audit pending.

Independent stack/bank/consumer/generic review in analysis/pass-002/review.md; primary fresh decodes and exact artificial-fetch matches in instruction-review.json. Consumer extras and graphics closures recorded separately. No new gameplay replay: prior sixstages/laterrooms/boss/THE END coverage was sufficient and reused. Remaining lower-window call-return bank effects, inline selector domains and mixed audio/text/actor fragments need new focused redirect evidence. No terminal exhaustion claim. No toolkit issues found; analysis/toolkit-issues.md. No toolkit/ROM edits, publishing, archive, deletion, cleanup or commentary phase performed.

ROM classification: 76.13% (199588 / 262160 bytes classified; 62572 bytes unknown).

| Bank | Code | Data/image | Unknown |
|---|---:|---:|---:|
| header | 0 | 16 | 0 |
| PRG00 | 2267 | 28 | 5897 |
| PRG01 | 2439 | 1260 | 4493 |
| PRG02 | 288 | 0 | 7904 |
| PRG03 | 456 | 1285 | 6451 |
| PRG04 | 370 | 7822 | 0 |
| PRG05 | 4 | 2008 | 6180 |
| PRG06 | 0 | 8192 | 0 |
| PRG07 | 36 | 979 | 7177 |
| PRG08 | 1939 | 338 | 5915 |
| PRG09 | 4 | 0 | 8188 |
| PRG10 | 0 | 8192 | 0 |
| PRG11 | 296 | 4907 | 2989 |
| PRG12 | 0 | 8192 | 0 |
| PRG13 | 4 | 7681 | 507 |
| PRG14 | 4416 | 288 | 3488 |
| PRG15 | 4489 | 320 | 3383 |
| CHR00 | 0 | 8192 | 0 |
| CHR01 | 0 | 8192 | 0 |
| CHR02 | 0 | 8192 | 0 |
| CHR03 | 0 | 8192 | 0 |
| CHR04 | 0 | 8192 | 0 |
| CHR05 | 0 | 8192 | 0 |
| CHR06 | 0 | 8192 | 0 |
| CHR07 | 0 | 8192 | 0 |
| CHR08 | 0 | 8192 | 0 |
| CHR09 | 0 | 8192 | 0 |
| CHR10 | 0 | 8192 | 0 |
| CHR11 | 0 | 8192 | 0 |
| CHR12 | 0 | 8192 | 0 |
| CHR13 | 0 | 8192 | 0 |
| CHR14 | 0 | 8192 | 0 |
| CHR15 | 0 | 8192 | 0 |


## Round 2 commentary checkpoint (in progress)
Preserved entire round-1 commentary under analysis/round-2/commentary-round1 and prepared a fresh pass-002-verified baseline. All added instruction starts semantically read; 333 draft unit explanations and new data comments applied. First coherent batch byte-identical (analysis/round-2/verify-batch1.json). Remaining alias/prose/dependency sweep and review inventory listed in commentary/progress.md; source readability not yet declared complete. Resume this same commentary phase.

## Round 2 commentary completed
All 199,588 currently classified bytes readable; 62,572 unknown bytes carry honest notes. Preserved round-1 directory, migrated unchanged physical records, reviewed 8,890 new code bytes and 45,216 data bytes, corrected affected older prose/aliases and refreshed 8,356 scaffold records/895 units. No remaining classified-source gaps. Fresh commentary/verification.json byte-identical; reading-after.asm refreshed. Inventory and source hashes: analysis/round-2/readability-check.json. Final editorial audit pending, whole-ROM classification still incomplete. Focused redirect questions in commentary/uncertainties.md; no new structural findings or gameplay replay. No toolkit issues found. No ROM/map/state/toolkit changes, publishing, archive or cleanup.

## Round2 redirect active research
Attempt analysis/redirect/round2-20261126, exact204range complement62572unknown at unchanged pass-002-verified. Current-source readability gate met. Productive lower bank2 [4,5,14,15] actor/init/transition graph1601instructions; separate audio-bank PPU/transition172instruction graph. Coherent local inline return-removing helpers and fixed FF80 RAM-only sound enqueue contract inspected. Independent mixed audio/sprite and actor OAM/scalar boundaries under review. Preliminary combined proposal17984bytes (3754code/14230data) is research only, not accepted classification. No map/state/source/ROM/toolkit changes. Detailed evolving frontier and primary artifacts retained in attempt directory; no terminal/exhaustion claim.

## Round2 redirect combined ready handoff
Completed connected and independent static investigations: four ready leadsR006..R009,20761proposedbytes (3922code/16839data), zeroaccepted. Authoritative199588classified/62572unknown unchanged;41811unknown if all accepted. Final evidence/recipes in analysis/handoffs/round2-redirect.md and analysis/redirect/round2-20261126/contracts.md; exact204gap complement/dispositions and final hashes preserved. All latest graphs checked for physicalCPUidentity and proposal/known-map conflicts (zero). Mixed audio/sprite/code islands separated, inline saved-return barriers explicit, actor responses and room dispatch continuations followed. Existing gameplay reused, no new execution claims. No toolkit issues found; no ROM/map/state/source/binary edits, final audit, archive, publishing, deletion or cleanup. Proceed to independent disassembly, not terminal assessment.

## Round3 disassembly active
Independent review of R006..R009 at unchanged pass-002-verified baseline. Frontier analysis/pass-003/frontier.md; no accepted bytes yet. No gameplay replay needed for inherited static evidence.

## Round3 disassembly completed — ROM incomplete
ROM classification: 84.10% (220484 / 262160 bytes classified; 41676 bytes unknown).

Accepted20896newbytes (3944code/16952data):20761reviewed manifest bytes with corrected13E72sprite/transition boundary plus135connected bytes. Original redirect's data/code split was corrected by1byte after independent semantic review; fresh decode agreement alone had not detected the mistaken operand root. Primary contracts, complete pattern/neighbor inspection, bank/stack/inline semantics and consumer behavior supplied role evidence. See analysis/pass-003/review.md, instruction-review.json, helper-contract-review.json, corrected-audio-tail-graph.json, connected-lookups.json and connected-tail-review.json. Original redirect history remains unchanged.

Totals20952code/68460data/131072CHRimage. All10080regions have evidence; all8152baseline regions unchanged. Exact unique physical union and188range complement, per-bank breakdown and lead outcomes in analysis/pass-003/final-checks.json, counts.json and lead-outcomes.json. R006..R009 all verified. New annotations entirelyStatic; observed execution coverage unchanged8562bytes/3840instructionstarts, including saved artificial scenarios, no natural full-game completion claim. Sixstages/laterrooms/boss/fairy/THE END saved recipes reused; no gameplay replay without changed evidence.

Final fresh checkpoint maps/pass-003-final.json, generated/pass-003-final/game.asm and analysis/pass-003-final-verification.json validate and rebuild byte-identically. analysis/state.json/maps/current.json/finalcheckpoint are identical; ROMSHA unchanged. Earlier pass-003-verified checkpoint preserved as intermediate before connected tail closure. No new toolkit issues found; analysis/toolkit-issues.md. Commentary remains preceding round and requires full current-source migration/review next; final editorial audit pending. No toolkit/ROM/maintained commentary edits, archive, publishing, deletion or cleanup. No terminal exhaustion claim.

ROM classification: 84.10% (220484 / 262160 bytes classified; 41676 bytes unknown).

| Bank | Code | Data/image | Unknown |
|---|---:|---:|---:|
| header | 0 | 16 | 0 |
| PRG00 | 2267 | 28 | 5897 |
| PRG01 | 2439 | 1260 | 4493 |
| PRG02 | 288 | 0 | 7904 |
| PRG03 | 519 | 1321 | 6352 |
| PRG04 | 370 | 7822 | 0 |
| PRG05 | 3423 | 4766 | 3 |
| PRG06 | 0 | 8192 | 0 |
| PRG07 | 36 | 979 | 7177 |
| PRG08 | 1954 | 6238 | 0 |
| PRG09 | 373 | 7817 | 2 |
| PRG10 | 0 | 8192 | 0 |
| PRG11 | 296 | 4907 | 2989 |
| PRG12 | 0 | 8192 | 0 |
| PRG13 | 4 | 7681 | 507 |
| PRG14 | 4416 | 456 | 3320 |
| PRG15 | 4567 | 593 | 3032 |
| CHR00 | 0 | 8192 | 0 |
| CHR01 | 0 | 8192 | 0 |
| CHR02 | 0 | 8192 | 0 |
| CHR03 | 0 | 8192 | 0 |
| CHR04 | 0 | 8192 | 0 |
| CHR05 | 0 | 8192 | 0 |
| CHR06 | 0 | 8192 | 0 |
| CHR07 | 0 | 8192 | 0 |
| CHR08 | 0 | 8192 | 0 |
| CHR09 | 0 | 8192 | 0 |
| CHR10 | 0 | 8192 | 0 |
| CHR11 | 0 | 8192 | 0 |
| CHR12 | 0 | 8192 | 0 |
| CHR13 | 0 | 8192 | 0 |
| CHR14 | 0 | 8192 | 0 |
| CHR15 | 0 | 8192 | 0 |


## Round3 commentary complete

Current-source readability complete across all 220,484 classified bytes (20,952 code / 68,460 data / 131,072 CHR); 41,676 unknown bytes remain explicitly bounded in 188 intervals. Preserved prior commentary at analysis/round-3/commentary-round2; reused unchanged source and valid review credit. Reviewed all 1,858 added instructions and added data; current 10,268 scaffold entries / 1,123 meaningful units reviewed, zero pending. All current-source readability gaps closed; ROM classification remains 84.10%, incomplete; final editorial audit pending and not run.

Fresh byte-identical verification and reading refresh passed; final invariants confirm source hashes, unchanged ROM/map/state and current frozen pass-003-final baseline. Corrected $13E72 LDA $C8 boundary and affected older interlude/selector/PPU/pointer/reset prose; local actor RAM differs from ordinary gameplay actors. Compact next-phase handoff analysis/handoffs/round3-commentary.md; detailed scope analysis/round-3/readability-check.json and commentary/progress.md. Redirect questions remain localized actor source bounds/selector contracts and mixed PRG1 islands; no new execution claims. No toolkit issues found; analysis/toolkit-issues.md. No publication/archive/deletion/cleanup or toolkit changes.

## Round3 redirect active

Input pass-003-final remains220484classified/41676unknown/188intervals. Research only at analysis/redirect/round3-20261126; exact unknown complement and frontier retained. Actor template/dispatch consumer investigation closes1489instruction graph; narrow PRG1text consumer bounds expose initializer and69-state graph1937instructions. Independent bank0accepted14way init/update dispatch plus known source edges now4389instructions; graph/domain/bank/stack/record-boundary audits pending. No proposedbytes accepted and no map/state/source/ROM/toolkit edits. Current-source readability gate complete; final editorialaudit pending and not run. Next operations in frontier.md; do not mistake exploratory graph closure for final acceptance or phase exhaustion.

Round3redirect package audit: exactspawn13streams/104rows fullycovers1E1F0..1E6BC; connectedactor,banks0/1/3/5graphs andfixedaccepted-codecontinuationchecks closed withzeroexistingData/instruction/tableoverlaps. Bank5returnpreconditionRAM44=5 preserved; bank3latecopiedcodeA7F1contractconflict localizedratherthanblanketdata. Currentresearchmanifest/role/boundaryreviewandunknownledgerdispositions inprogress; acceptedclassificationstill220484/41676, source/state/mapunchanged. Seeanalysis/redirect/round3-20261126/frontier.md. Finaleditorialauditnotrun.

## Round3 redirect complete — ready combined batch

Research-only package analysis/redirect/round3-20261126 and compact analysis/handoffs/round3-redirect.md complete. R010..R014 ready:40272proposed (18237Code/22035Data), zeroaccepted; authoritative220484classified/41676unknown188intervals unchanged,1404prospectiveunknown45intervals ifallaccepted. Connecteddisplay/text/actor/bank5/PRG3/fixedplayer-room graphs andgenericdatabounds independentlysupported; finalmanifest/invariants pass, no overlaps, ROM/map/state/sourceunchanged. Primarycontracts andexactledgers retained. Nextindependentdisassembly, thenfullcurrent-sourcecommentary/redirect; finaleditorialauditpending/notrun. No toolkitissues, cleanup, archive orpublishing.

## Round4 disassembly active

Independent semantic/bank/stack/inline and exact-boundary review of R010..R014, not acceptance by count or decode fidelity. Baseline pass-003-final preserved; no new annotations. Frontier analysis/pass-004/frontier.md. Review primary graph, consumer, writer, record and pattern evidence before Static edits; no unchanged gameplay replay.

## Round4 disassembly complete — ROM incomplete

ROM classification: 99.53% (260918 / 262160 bytes classified; 1242 bytes unknown).

Accepted40434newbytes (18349Code/22085Data):40272 independently reviewed R010..R014 proposals plus162connected (112Code/50Data). All five leads verified. Primary consumer, writer, record, stack/inline/bank and pattern evidence reviewed independently; fresh8312instruction decode checks were supplemental. Additional independent loop startsF14B, not initialF149; twozero bytes retained as data. review.md, semantic-review.json, instruction-review.json, connected-review.json and lead-outcomes.json retain decisions and exact bounds. Original evidence/history preserved.

Totals 39301Code/90545Data/131072CHRimage. All18634regions evidence-backed; all10080baseline regions unchanged. Exact physical union, per-bank counts and 34range/1242unknown complement in analysis/pass-004/counts.json/final-checks.json. Execution coverage unchanged8562bytes/3840instructionstarts, including prior artificial scenarios; no new natural-selection or full-game completion claims. Saved sixstage/laterroom/boss/fairy/THE END routes reused; no unchanged replay.

Final fresh maps/pass-004-final.json matches analysis/state.json and maps/current.json exactly, validates and rebuilds byte-identically: analysis/pass-004-final-verification.json, generated/pass-004-final/game.asm. ROM and maintained commentary/source unchanged; full current-source migration/readability review next phase, final editorial audit pending/not run. No toolkit issues found; analysis/toolkit-issues.md. No toolkit/ROM edits, publication/archive/deletion/cleanup. This is not terminal exhaustion.

ROM classification: 99.53% (260918 / 262160 bytes classified; 1242 bytes unknown).

| Bank | Code | Data/image | Unknown |
|---|---:|---:|---:|
| header | 0 | 16 | 0 |
| PRG00 | 6336 | 1855 | 1 |
| PRG01 | 4430 | 3717 | 45 |
| PRG02 | 2693 | 5498 | 1 |
| PRG03 | 2951 | 5228 | 13 |
| PRG04 | 370 | 7822 | 0 |
| PRG05 | 3423 | 4766 | 3 |
| PRG06 | 0 | 8192 | 0 |
| PRG07 | 2247 | 4809 | 1136 |
| PRG08 | 1954 | 6238 | 0 |
| PRG09 | 373 | 7817 | 2 |
| PRG10 | 0 | 8192 | 0 |
| PRG11 | 1364 | 6814 | 14 |
| PRG12 | 0 | 8192 | 0 |
| PRG13 | 322 | 7854 | 16 |
| PRG14 | 7242 | 945 | 5 |
| PRG15 | 5596 | 2590 | 6 |
| CHR00 | 0 | 8192 | 0 |
| CHR01 | 0 | 8192 | 0 |
| CHR02 | 0 | 8192 | 0 |
| CHR03 | 0 | 8192 | 0 |
| CHR04 | 0 | 8192 | 0 |
| CHR05 | 0 | 8192 | 0 |
| CHR06 | 0 | 8192 | 0 |
| CHR07 | 0 | 8192 | 0 |
| CHR08 | 0 | 8192 | 0 |
| CHR09 | 0 | 8192 | 0 |
| CHR10 | 0 | 8192 | 0 |
| CHR11 | 0 | 8192 | 0 |
| CHR12 | 0 | 8192 | 0 |
| CHR13 | 0 | 8192 | 0 |
| CHR14 | 0 | 8192 | 0 |
| CHR15 | 0 | 8192 | 0 |


## Round4 commentary active — pass-004-final

Preserved the entire prior maintained commentary at analysis/round-4/commentary-round3, prepared a fresh pass004 baseline and migrated all10,080 unchanged physical classified blocks with valid prior review credit. Display, narrative/text, PRG3scene, bank5actor and fixed actor additions are now reviewed in coherent semantic batches and verified byte-identically; latest actor result analysis/round-4/fixed-actor-verification.json. All34 unknown intervals have exact honest boundary notes. Classification remains99.53%/1242unknown bytes; no map/state/ROM/toolkit changes.

Remaining active commentary frontier: fixed player-animation and action-object graphs, parent-return parser, PPU/MMC1 aliases/helpers, associated361-minus-reviewed data bytes and concrete dependency refinements, then complete source-location/fullscope assessment and fresh verification/reading refresh. Exact pending entries: analysis/round-4/pending-state.json; maintained scope/frontier: commentary/progress.md. R4-C01 palette-suffix structural question is saved in analysis/round-4/redirect-questions.md for subsequent redirect; current role unchanged. Current-source readability is not yet complete and no redirect handoff or final editorial audit is claimed. No toolkit issues found.

## Round4 commentary complete — pass-004-final

All260,918 classified bytes now meet current-source readability;1,242 unknown bytes retain34 exact explanatory boundaries. Preserved prior entire commentary at analysis/round-4/commentary-round3 and all10,080 unchanged classified blocks/review credit. Reviewed all8,364 new instruction starts/18,349 code bytes and22,085 data bytes; full18,668-region scaffold and2,028 meaningful units reviewed with fresh source locations, zero pending/deferred ranges. Exact scope/identities: analysis/round-4/readability-check.json. Handoff: analysis/handoffs/round4-commentary.md; detailed maintained outcome: commentary/progress.md.

Fresh final verification analysis/round-4/final-verification.json is byte-identical; reading refreshed in analysis/round-4/reading-refresh.json. ROM/currentmap/state identities unchanged against pass004/frozen baseline. Full classification remains99.53%/incomplete; final editorial audit pending and not run. Next phase redirect with localized R4-C01 palette-suffix question and exact legacy/short-fragment residual contracts. No fresh runtime/natural-entry claims, toolkit issues, map/state/ROM/toolkit changes or cleanup/publication/archive actions.

## Round4 redirect active — local contracts
Research analysis/redirect/round4-20261126 now has exact1242byte/34interval ledger and immutable-decoded evidence. Corrected localized legacy inference supports581code+42data independent of unresolved502byte inline-dispatch prefix; palette suffix12byte correction and28byte same-bank serializer contracts also ready for independent assessment. No classifications applied. Reset aliases/scalar80 and nominalvector-copy constraints under investigation; precise frontier/findings saved. No new gameplay/replay, toolkit issues or map/state/source/ROM edits.

## Round4 redirect combined ready —710new proposed,12reclassification
Completed localized legacy collision/state/health graph and independent same-bank serializer/reset/vector-replica contracts. Exact manifest analysis/redirect/round4-20261126/combined-proposals.json proposes636code+86data=722reviewbytes,710newlyclassified+12palette-suffix correction,accepted0. Currentclassification remains260918/1242unknown/34intervals; prospective532unknown/22intervals if allaccepted. Fullcontracts/evidence/exactcurrentledger and hypotheticalcomplement saved. Handoff analysis/handoffs/round4-redirect.md; R007/R010/R011/R013 histories reopened with changedinferences/currentrecipes while preserving priorverified outcomes. Connectedcontinuations closed and promisingindependent avenues assessed; remaining502byteprefix+30shortbytes retain concretecode/data/boundary alternatives. Nonterminalbatch, no exhaustionclaim. No newruntime, map/state/source/ROM/toolkit/binary edits or cleanup/publication/archive; no toolkitissuesfound.

## Round5 disassembly active

Independent review of722proposedbytes (710new plus12Data-to-Code correction) from round4redirect; authoritative pass004 preserved. Primary role/bounds/stack/bank/mapper/consumer contracts reviewed separately from opcode legality and reachability. Frontier analysis/pass-005/frontier.md. No unchanged gameplay replay or terminal exhaustion claim; currentclassification260918/1242unknown.

## Round5 disassembly complete — ROM incomplete

ROM classification: 99.80% (261628 / 262160 bytes classified; 532 bytes unknown).

Accepted722reviewbytes:636Code/86Data, comprising710newlyclassifiedbytes (624Code/86Data) and12Data-to-Code correction. Netcodegain636/netdatagain74. R007/R010/R011/R013 independently verified with primary role/boundary/stack/mapper/bank/consumer evidence and301fresh configured-release instruction decodes; legaldecoding not solebasis. review.md/semantic-review.json preserve59directcontrol references,74branch edges, exact domains and oldpalette record. Originalpalette id13964 shrunk60->48with allmetadata/evidence preserved; allother18633priorregions unchanged.

Totals39937Code/90619Data/131072CHRimage; all18950regions evidence-backed. Exactphysicalunion and22interval/532bytecomplement plus perbankcounts saved in counts.json/final-checks.json. Residual502bytelegacy prefix and30tinybytes retain localized code/data/boundary alternatives. Nonterminal; no exhaustionclaim. Executioncoverage unchanged8562bytes/3840observedstarts; allnew/corrected annotationsStatic. Saved broadartificial sixstage/laterroom/boss/fairy/THE END evidence reused; no unchangedgameplay/reachabilitypromotion.

Fresh maps/pass-005-final.json matches state/current exactly, validates and rebuilds byte-identically: analysis/pass-005-final-verification.json; generated/pass-005-final/game.asm. ROM, configuredtoolkit/binary and maintainedcommentary/source unchanged. Fullcurrent-source migration/readability review next; finaleditorialaudit pending/notrun. Lead histories preserved/appended. No toolkitissuesfound; analysis/toolkit-issues.md. No archive/publication/deletion/cleanup.

ROM classification: 99.80% (261628 / 262160 bytes classified; 532 bytes unknown).

| Bank | Code | Data/image | Unknown |
|---|---:|---:|---:|
| header | 0 | 16 | 0 |
| PRG00 | 6336 | 1855 | 1 |
| PRG01 | 4461 | 3724 | 7 |
| PRG02 | 2705 | 5486 | 1 |
| PRG03 | 2954 | 5235 | 3 |
| PRG04 | 370 | 7822 | 0 |
| PRG05 | 3423 | 4766 | 3 |
| PRG06 | 0 | 8192 | 0 |
| PRG07 | 2831 | 4858 | 503 |
| PRG08 | 1954 | 6238 | 0 |
| PRG09 | 373 | 7817 | 2 |
| PRG10 | 0 | 8192 | 0 |
| PRG11 | 1367 | 6825 | 0 |
| PRG12 | 0 | 8192 | 0 |
| PRG13 | 325 | 7865 | 2 |
| PRG14 | 7242 | 945 | 5 |
| PRG15 | 5596 | 2591 | 5 |
| CHR00 | 0 | 8192 | 0 |
| CHR01 | 0 | 8192 | 0 |
| CHR02 | 0 | 8192 | 0 |
| CHR03 | 0 | 8192 | 0 |
| CHR04 | 0 | 8192 | 0 |
| CHR05 | 0 | 8192 | 0 |
| CHR06 | 0 | 8192 | 0 |
| CHR07 | 0 | 8192 | 0 |
| CHR08 | 0 | 8192 | 0 |
| CHR09 | 0 | 8192 | 0 |
| CHR10 | 0 | 8192 | 0 |
| CHR11 | 0 | 8192 | 0 |
| CHR12 | 0 | 8192 | 0 |
| CHR13 | 0 | 8192 | 0 |
| CHR14 | 0 | 8192 | 0 |
| CHR15 | 0 | 8192 | 0 |

## Round5 commentary active
Prior entire commentary preserved at analysis/round-5/commentary-round4. Fresh pass005 baseline prepared byte-identically (18,972 scaffold records). Exact physical block/review migration is running; pending inventory will be analysis/round-5/pending-initial.json. Frontier and plan: analysis/round-5/plan.md. No readability completion claimed; new/corrected source and affected dependencies remain pending.

Round5 commentary coherent batches passed byte-identical verification (analysis/round-5/batch-verification.json and refined-verification.json). Full 318-record changed scope reviewed; 301 instructions/636 bytes plus current data/unknown boundaries, with prior exact-block credit reused. Full inventory/location/identity report running; final verify/read still pending. Final editorial audit remains pending.

Round5 commentary complete: full 261,628 classified-byte readability plus 22 unknown boundary notes; 18,972 reviewed scaffold records/2,063 units, zero pending/deferred gaps. Final fresh byte-identical verification and reading refresh: analysis/round-5/final-verification.json, reading-refresh.json. Compact handoff analysis/handoffs/round5-commentary.md; scope/identity report analysis/round-5/readability-check.json. Final editorial audit remains pending; no audit run. Classification remains 99.80%/532 unknown; no toolkit issues.

## Round5 redirect active
Research analysis/redirect/round5-20261126; exact pass005 532unknown/22interval ledger persisted. Reopening localized prefix subentry/control contracts independently of rejected copied inline semantics; then tiny-parent encoding boundaries. Current-source commentary complete; no final audit or map/state/source edits.

## Round5 redirect ready combined batch
Corrected actual-bank partial contract/pattern method completed:388newStaticproposed=272Code+116Data,0accepted. R013ready with appended history/recipe; prior verified outcomes preserved. Twelve local subentries close through actual stack-safe B044 RAM-fill tail; copied A83D/A7F1/AB28/B0A1 semantics remain rejected locally. Five word arrays, ten independently destination-supported address pairs and16maskbytes proposed without functioning inline-consumer claim. Source/getter/parent/OAM/isolated-fragment independent avenues reviewed; no supported tiny extension identified. All exact counterexamples/combined constraints/alternatives in analysis/redirect/round5-20261126/contracts.md. Actual current532unknown/22interval complement independently confirmed; hypothetical144unknown/38intervals afterall388accepted. No new gameplay/runtime, map/state/source/ROM/toolkit/binary changes, final audit, archive, publication or cleanup. Ready handoff analysis/handoffs/round5-redirect.md; no terminal exhaustion/ROMcompletion claim. No toolkit issues found.

## Round6 disassembly active
Independent review of round5 redirect388-byte batch. Frontier analysis/pass-006/frontier.md. No classification or reachability change yet.

## Round6 disassembly complete — ROM incomplete

ROM classification: 99.95% (262016 / 262160 bytes classified; 144 bytes unknown).

Accepted388newStaticbytes=272Code+116Data after independent actual-bank control/stack/dataflow/boundary/pattern review and138fresh configured-release instruction decodes/141closed edges. Review analysis/pass-006/review.md explicitly assesses F242RTS against the ten-pair family; actualB044 fills finite RAM suffix00,02..07, preserving stack01/bank and possibly overwriting scratch. No renderer/functioning-inline/natural-entry claim. All18950priorregions unchanged; exact union/perbank/complement and observed8562bytes/3840starts unchanged in counts.json/final-checks.json. Remaining144unknown/38intervals=114legacy wrapper/conflict bytes+30tiny ambiguous bytes. Connected/residual alternatives reviewed; no supported extension withheld. Nonterminal, not exhaustion.

Fresh maps/pass-006-final.json equals current/state, validates and rebuilds byte-identically: analysis/pass-006-final-verification.json; generated/pass-006-final/game.asm. ROM/toolkit/binary/maintainedcommentary/source unchanged; current-source migration/readability and finaleditorialaudit pending. R013history preserved/appended; no unchanged gameplay replay, archive/publication/deletion/cleanup. No toolkit issues found; analysis/toolkit-issues.md.

ROM classification: 99.95% (262016 / 262160 bytes classified; 144 bytes unknown).

| Bank | Code | Data/image | Unknown |
|---|---:|---:|---:|
| header | 0 | 16 | 0 |
| PRG00 | 6336 | 1855 | 1 |
| PRG01 | 4461 | 3724 | 7 |
| PRG02 | 2705 | 5486 | 1 |
| PRG03 | 2954 | 5235 | 3 |
| PRG04 | 370 | 7822 | 0 |
| PRG05 | 3423 | 4766 | 3 |
| PRG06 | 0 | 8192 | 0 |
| PRG07 | 3103 | 4974 | 115 |
| PRG08 | 1954 | 6238 | 0 |
| PRG09 | 373 | 7817 | 2 |
| PRG10 | 0 | 8192 | 0 |
| PRG11 | 1367 | 6825 | 0 |
| PRG12 | 0 | 8192 | 0 |
| PRG13 | 325 | 7865 | 2 |
| PRG14 | 7242 | 945 | 5 |
| PRG15 | 5596 | 2591 | 5 |
| CHR00 | 0 | 8192 | 0 |
| CHR01 | 0 | 8192 | 0 |
| CHR02 | 0 | 8192 | 0 |
| CHR03 | 0 | 8192 | 0 |
| CHR04 | 0 | 8192 | 0 |
| CHR05 | 0 | 8192 | 0 |
| CHR06 | 0 | 8192 | 0 |
| CHR07 | 0 | 8192 | 0 |
| CHR08 | 0 | 8192 | 0 |
| CHR09 | 0 | 8192 | 0 |
| CHR10 | 0 | 8192 | 0 |
| CHR11 | 0 | 8192 | 0 |
| CHR12 | 0 | 8192 | 0 |
| CHR13 | 0 | 8192 | 0 |
| CHR14 | 0 | 8192 | 0 |
| CHR15 | 0 | 8192 | 0 |

## Round6 commentary complete

All current 262,016 classified bytes readable; 144 unknown bytes/38 exact notes remain. Preserved entire prior commentary in analysis/round-6/commentary-round5 and reused 18,950 physically exact classified blocks. Reviewed twelve local code roots (272 bytes), five word arrays/ten pairs/masks (116 bytes), changed unknown boundaries and affected prefix/B044 claims. No pending/deferred readability ranges; 19,015 reviewed scaffold entries and 2,092 meaningful units. Fresh byte-identical verification/read refresh: analysis/round-6/final-verification.json and reading-refresh.json; inventory/identity checks: readability-check.json. Final editorial audit remains pending after redirect. No map/state/ROM/toolkit/binary writes; no toolkit issues found.

## Round6 redirect active

Research started in analysis/redirect/round6-20261126 from pass-006-final (262016 classified;144unknown/38intervals). Current-source commentary gate complete. Exact fresh complement/neighbors/windows saved in unknown-ledger.json. Reassessing localized short tail transfers separately from rejected whole wrapper semantics; tiny-role alternatives follow. No map/state/source changes.

## Round6 redirect ready combined batch

Changed local record-slice reasoning produced4StaticData proposed bytes,0accepted:1B9F3..1B9F5 two individually pointed80 cells within the established mirrored terminated-sprite family, and1F93E..1F940=C900 as actual selected fifthstage6 parent replacement pair. Exact filtered artificial reads captured from initializedstage6room4 afterRAM28=0F only; first60frame/3second request timed out after9frames with bothreads already captured/no drops, no retry. Trailing60excluded. All residual dispositions refreshed; JMPB044/getter/OAM/F0/80/localwrapper alternatives localized without speculative additions. Input/current remains262016classified,144unknown/38intervals; hypotheticalacceptance140unknown/37intervals. Research contracts/primaryevidence/manifest/finalchecks and frontier at analysis/redirect/round6-20261126; analysis/leads.json R003/R014ready, priorhistorypreserved. Next independentdisassembly, then current-sourcecommentary/redirect; no terminalclaim or finaleditorialaudit. Handoff analysis/handoffs/round6-redirect.md. No toolkit issues found; no structuralmap/state/source/ROM/toolkit/binarywrites or cleanup.

## Round7 disassembly active
Primary review of four proposed Static Data bytes; exact frontier analysis/pass-007/frontier.md. Sprite first-field80 consumer and parent two-read selector checked against maintained source and actual artificial trace. No replay or classification change yet.

## Round7 disassembly complete — ROM incomplete

ROM classification: 99.95% (262020 / 262160 bytes classified; 140 bytes unknown).

Accepted4newStaticData bytes:1B9F3..1B9F5 independently addressed80 empty-list cells and1F93E..1F940 selected stage6room4 C9/00 parent replacement pair. Independent bank/consumer/pointer/boundary/record/actual-read review:analysis/pass-007/review.md; semantic-review.json. Trailing60 and unrelated80 excluded. All18977priorregions unchanged; exact union/perbank/unknown complement in counts.json/final-checks.json. Remaining140unknown/37intervals=114legacy wrapper/conflict+26tiny ambiguous bytes. Observed instruction coverage unchanged8562bytes/3840starts; no replay. No supported connected extension withheld; nonterminal stopping point.

Fresh state/current/maps/pass-007-final.json identical, validated and byte-identically rebuilt:analysis/pass-007-final-verification.json; generated/pass-007-final/game.asm. ROM/configured binary/maintained source unchanged; R003/R014 outcome history appended. Full current-source commentary and final editorial/terminal rejection audits pending. No toolkit issues found; analysis/toolkit-issues.md. No archive/publication/cleanup.

ROM classification: 99.95% (262020 / 262160 bytes classified; 140 bytes unknown).

| Bank | Code | Data/image | Unknown |
|---|---:|---:|---:|
| header | 0 | 16 | 0 |
| PRG00 | 6336 | 1855 | 1 |
| PRG01 | 4461 | 3724 | 7 |
| PRG02 | 2705 | 5486 | 1 |
| PRG03 | 2954 | 5235 | 3 |
| PRG04 | 370 | 7822 | 0 |
| PRG05 | 3423 | 4766 | 3 |
| PRG06 | 0 | 8192 | 0 |
| PRG07 | 3103 | 4974 | 115 |
| PRG08 | 1954 | 6238 | 0 |
| PRG09 | 373 | 7817 | 2 |
| PRG10 | 0 | 8192 | 0 |
| PRG11 | 1367 | 6825 | 0 |
| PRG12 | 0 | 8192 | 0 |
| PRG13 | 325 | 7867 | 0 |
| PRG14 | 7242 | 945 | 5 |
| PRG15 | 5596 | 2593 | 3 |
| CHR00 | 0 | 8192 | 0 |
| CHR01 | 0 | 8192 | 0 |
| CHR02 | 0 | 8192 | 0 |
| CHR03 | 0 | 8192 | 0 |
| CHR04 | 0 | 8192 | 0 |
| CHR05 | 0 | 8192 | 0 |
| CHR06 | 0 | 8192 | 0 |
| CHR07 | 0 | 8192 | 0 |
| CHR08 | 0 | 8192 | 0 |
| CHR09 | 0 | 8192 | 0 |
| CHR10 | 0 | 8192 | 0 |
| CHR11 | 0 | 8192 | 0 |
| CHR12 | 0 | 8192 | 0 |
| CHR13 | 0 | 8192 | 0 |
| CHR14 | 0 | 8192 | 0 |
| CHR15 | 0 | 8192 | 0 |

## Round7 commentary completed; redirect active (coordinator reconciliation)

Separate round7 commentary worker completed all current-source readability:262020classified bytes,140unknown/37notes,19016 reviewed scaffold entries/2094units; no pending/deferred gaps. Preserved entire priorcommentary, exactblock credit and updatedempty80/C900pair/mirrorpalettebit descriptions. Fresh byte-identical verification/read refresh and current-source hashes inspected by coordinator:analysis/round-7/readability-check.json,final-verification.json,reading-refresh.json; compact handoff analysis/handoffs/round7-commentary.md. Final editorial audit pending.

Separate round7 redirect worker now active at analysis/redirect/round7-20261126/frontier.md and exact140/37ledger; primary structural/rejection reasoning review underway. Currentstate/map pass-007-final unchanged, no new classification/terminalclaim.

## Round7 redirect — active structural template inference
Input maps/pass-007-final.json:262020classified,140unknown/37intervals; current-source round7 commentary complete. Unique attempt analysis/redirect/round7-20261126. New corrected-premise method compares complete accepted bank5 B17E..B374 schema to bank7 F16C..F362 (+3FEE):192records,176same-opcode Code positions,16Data barriers116bytes. All17legacy unknown intervals114bytes map exactly to38source Code instructions. Fresh candidate decodes preserve actual [6,7,14,15]. Proposed role is encoded legacy instruction fragments, not working copied inline dispatch: actual callee conflicts remain explicit in contracts.md. No map/state/source/ROM/toolkit/binary writes or runtime. New complete direct-control audit3858edges/43longviews has zero unknown targets; tiny8/16flank homology yields no independent classified specimen. Packaging combined114Code proposal, exactledger dispositions and persistent R013 history now; no terminal presumption or final editorial audit.

Round7 redirect packaging complete:114StaticCode proposed,0accepted,17intervals/38instructions. Input current140unknown/37intervals unchanged; prospective26/20 only after independent acceptance. Complete schema and actual-callee limits in analysis/redirect/round7-20261126/contracts.md; combined-proposals.json; current exact dispositions unknown-ledger.json; fresh source-windows.txt; history in analysis/leads.json. Handoff analysis/handoffs/round7-redirect.md. No terminal or final editorial audit; no toolkit issues found.

## Round8 disassembly active — independent review complete
Fresh configured-release target decode38starts and independent192-record/502byte source-target schema review support114StaticCode encoded legacy fragments,17intervals. Actual incompatible A83D/A7F1/AB28/B0A1 semantics remain explicit; no working inline dispatch or natural entry claimed. Strong serialized-data competitor assessed against independently established source roles and target272Code/116Data anchors. Decision before annotation: analysis/pass-008/review.md; semantic-review.json. Twenty tiny intervals26bytes excluded. Next annotate exact ranges, validate/save/export and fresh unique checkpoint.

## Round8 disassembly complete — ROM incomplete

ROM classification: 99.99% (262134 / 262160 bytes classified; 26 bytes unknown).

Accepted114newStaticCode bytes in17intervals/38instructions as encoded legacy fragments; independent complete192record/502byte homolog role/bounds and actual-bank decoding: analysis/pass-008/review.md; semantic-review.json; instruction-review.json. Actual incompatible A83D/A7F1/AB28/B0A1 behavior unchanged; no working copied wrappers, source callee substitution or natural entry inferred. TwoJMPB044 fragments separately supported by actual valid-stack finite tail. All18979priorregions unchanged. Remaining26unknown/20tinyintervals have no independent homolog/role discriminator; exact complement/perbank in counts.json and unknown-frontier.json. No productive connected extension withheld; no broad gameplay replay. Observed instruction coverage unchanged8562bytes/3840starts.

Fresh state/current/pass-008-final identical, validated, byte-identical rebuild: analysis/pass-008-final-verification.json; generated/pass-008-final/game.asm. ROM/configured binary/maintained source unchanged. R013 history preserved/appended. Full current-source commentary and final editorial/terminal rejection audits pending. No toolkit issues found; analysis/toolkit-issues.md. No archive/publication/cleanup.

ROM classification: 99.99% (262134 / 262160 bytes classified; 26 bytes unknown).

| Bank | Code | Data/image | Unknown |
|---|---:|---:|---:|
| header | 0 | 16 | 0 |
| PRG00 | 6336 | 1855 | 1 |
| PRG01 | 4461 | 3724 | 7 |
| PRG02 | 2705 | 5486 | 1 |
| PRG03 | 2954 | 5235 | 3 |
| PRG04 | 370 | 7822 | 0 |
| PRG05 | 3423 | 4766 | 3 |
| PRG06 | 0 | 8192 | 0 |
| PRG07 | 3217 | 4974 | 1 |
| PRG08 | 1954 | 6238 | 0 |
| PRG09 | 373 | 7817 | 2 |
| PRG10 | 0 | 8192 | 0 |
| PRG11 | 1367 | 6825 | 0 |
| PRG12 | 0 | 8192 | 0 |
| PRG13 | 325 | 7867 | 0 |
| PRG14 | 7242 | 945 | 5 |
| PRG15 | 5596 | 2593 | 3 |
| CHR00 | 0 | 8192 | 0 |
| CHR01 | 0 | 8192 | 0 |
| CHR02 | 0 | 8192 | 0 |
| CHR03 | 0 | 8192 | 0 |
| CHR04 | 0 | 8192 | 0 |
| CHR05 | 0 | 8192 | 0 |
| CHR06 | 0 | 8192 | 0 |
| CHR07 | 0 | 8192 | 0 |
| CHR08 | 0 | 8192 | 0 |
| CHR09 | 0 | 8192 | 0 |
| CHR10 | 0 | 8192 | 0 |
| CHR11 | 0 | 8192 | 0 |
| CHR12 | 0 | 8192 | 0 |
| CHR13 | 0 | 8192 | 0 |
| CHR14 | 0 | 8192 | 0 |
| CHR15 | 0 | 8192 | 0 |

## Round8 commentary completed; redirect active (coordinator reconciliation)

Full current-source readability complete:262134classified bytes,26unknown/20notes,19016scaffoldrecords/2111units reviewed; no pending/deferred gaps. Entire prior commentary preserved at analysis/round-8/commentary-round7. Encodedlegacyfragments documented with actualCPUoperands/incompatiblecallee limits, no functioningwrapper claim; obsoleteolderambiguity repaired. Final byte-identical verification/read refresh and current-source hashes inspected:analysis/round-8/readability-check.json,final-verification.json,reading-refresh.json; handoff analysis/handoffs/round8-commentary.md. Finaleditorialauditpending.

Separate round8redirect worker active on exact26tinybytes using current primary neighbors/constraints and reusable prior directedge/homolog/gameplay evidence; compact rejectionreview pending. No terminaldecision/classificationmutation yet.

## Round8 redirect terminal review ready for coordinator

Exact pass00826unknown/20interval ledger refreshed; all classified262134 remain unchanged. Primary-linked localized rejection review:analysis/redirect/round8-20261126/rejection-review.md, with current source windows, exact bank/CPU claims, actualcounterexample scopes and strongest competing models. Six existing lead histories and pass record appended;0proposed/accepted. Reused complete3858edge/tinyhomology/parentread/gameplay evidence; bounded delta23edges/38new instructions finds no residual target. No useful revised inference/test survives current review. This is terminal-incomplete pending coordinator primary inspection, not blocked/wholeROMcomplete/editorialacceptance. Final editorial phase separate; no unchanged replay/planning restart. Handoff analysis/handoffs/round8-redirect.md. No runtime/map/state/source/ROM/toolkit/binary mutation, cleanup/archive/publication; final-checks.json identities preserved.

## Coordinator terminal-incomplete review accepted

Inspected full round8 rejection review and decisive primary ROM/source/trace/control/homology artifacts. Exact26unknown/20interval complement independently regenerated; getter branch90EB targets earlierzero stub; parent resolver/actual C900pair reads do not extend to trailing60; isolatedreturn/OAM/F0/80 alternatives and acceptedhelper/replica criteria localized consistently. No useful revised inference/test or ready lead remains identified. Rationale:analysis/coordinator-rejection-review.md, primary check:coordinator-rejection-check.json. Workflow terminal incomplete accepted, ROM classification remains99.99%/incomplete. Final editorial acceptance/fresh verification and final document-selection accounting remain required; no archive/cleanup authorized.

## Final coordinator acceptance

2026-11-26T11:27:02.251303+00:00 — Sequential workflow finished at coordinator-accepted terminal incomplete. Classification 99.99% (262134/262160), unknown26/20intervals. Final strict editorial audit, byte-identical verify/reading refresh and independent store/maps/identity/unit checks passed. Final source and inventory remain current; see analysis/handoffs/final-editorial.md and analysis/final-handoff.md. Document selection fully accounted in archive-options.json and archive-selection-check.json. TI-001 is a nonblocking performance missing feature. No archive/publication/cleanup/deletion/toolkit changes. No ready lead or active phase remains.
