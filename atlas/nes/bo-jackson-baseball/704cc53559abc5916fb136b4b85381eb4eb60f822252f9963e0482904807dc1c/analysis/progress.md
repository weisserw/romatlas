# Disassembly progress

Workspace initialized. Baseline and classification have not run.

## Round 1 disassembly — active
Verified raw baseline: maps/baseline.json, generated/baseline, analysis/baseline-verification.json. ROM unchanged. Boot identified Bo Jackson Baseball (1991); gameplay route discovery active. Persistent local RPC server held by analysis/r001/server-files.py; full bounded request/result ledger analysis/r001/rpc.jsonl. Trace aggregates retained (zero event detail), no classification edits yet. Frontier: enter team/menu states, collect bank-aware execution, then inspect connected static paths and CHR role. No terminal claim.

### Batch A — verified structural foothold
ROM classification: 67.71% (266261 / 393232 bytes classified; 126971 bytes unknown).
4,095 code bytes (3,331 physical bytes with actual fetch evidence in the boot/menu scenario), 262,144 CHR image bytes, 16 header bytes, 6 vector bytes. Checkpoint `disassembly-r001-a` validated and rebuilt byte-identically. CHR is header-bounded hardware pattern storage, not inferred from lack of execution. Static overlap is a supported BIT-skip dual entry at file0x1C545/$8535 and file0x1C546/$8536; outer BIT encoding retained, alternate LDY entry documented in investigation. No completion claim. Continuing gameplay from deterministic replay; earlier in-memory machine lost during RPC helper interruption, durable aggregate roots retained in r001/contexts.json. Need durable menu snapshot before further exploration.

### Batch B — PCM and uniform data
ROM classification: 74.68% (293662 / 393232 bytes classified; 99570 bytes unknown).
Eight consumer-anchored packed PCM samples file0x10–0x4EA8 classified with exact count-derived extents and independently matching direct caller bank/pointer constants. Added supported PCM player and MMC3 sample-window helper plus bounded repeated non-code runs/tails. Internal map validated/exported; final pass checkpoint still pending. Natural 1P team-selection route replayed, saved same-build snapshots at r001/teams2-snapshot.json and associated observed roots. Local file-queue helper proved fragile across process interruptions, so subsequent batches use direct stdin/stdout RPC and export snapshots/aggregates before process exit. Old evidence remains retained; no unsupported execution claims added.

### Batch C checkpoint — active gameplay frontier
ROM classification: 78.29% (307844 / 393232 bytes classified; 85388 bytes unknown).
Verified checkpoint `disassembly-r001-b`: map and generated source validated; whole-ROM byte-identical verification succeeded (`analysis/disassembly-r001-b-verification.json`). Natural pitching reached a live ball-in-play fielding view (`r001/pitch-result.ppm`). Artificial two-strike/two-out branch yielded strikeout (`r001/late-inning.ppm`) but inning remained1: a pending IRQ consumed the first register-gadget operation, caught by immediate RAM readback. Corrected bounded recipe masks IRQ, asserts opcode bytes, retries at most3, requires successful target readback; observed accepted IRQ operation is distinguished from store. Active: ninth-inning/endgame attempt with verified RAM edit; batting after half-inning transition; representative settings/playoff menus; then merge all durable fetch roots, final verification and disassembly handoff. Not terminal exhaustion.

## Round 1 disassembly — completed pass, ROM incomplete

ROM classification: 81.65% (321066 / 393232 bytes classified; 72166 bytes unknown).

Final verified checkpoint: [map](../maps/disassembly-r001.json), [source](../generated/disassembly-r001/game.asm), [verification](disassembly-r001-verification.json). `map.validate` passed; generated ca65 assembly rebuilt the entire393232-byte image byte-identically. ROM SHA256 unchanged: `704cc53559abc5916fb136b4b85381eb4eb60f822252f9963e0482904807dc1c`. Map SHA256 `ba7762de5c6a4cad48861b06d427da49bfddccbe788b19b0f9d72f32dd588fad`. Current internal store `analysis/state.json` and exported `maps/current.json` match this checkpoint. No toolkit edits, publishing, archive or workspace deletion occurred.

Structural totals:20869 Code,38053 Data,262144 Image. All1038 regions have evidence;42 have labels; every Code region has explicit bank-aware CPU view. CHR accounts for262144 classified bytes, so classification is distinct from executable understanding. There are260 exact unknown intervals in [unknown-final.json](r001/unknown-final.json).

| PRG 8 KiB bank | Code | Data | Classified | Unknown |
|---:|---:|---:|---:|---:|
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 0 | 8192 | 8192 | 0 |
| 2 | 1704 | 5319 | 7023 | 1169 |
| 3 | 0 | 213 | 213 | 7979 |
| 4 | 0 | 3039 | 3039 | 5153 |
| 5 | 0 | 7983 | 7983 | 209 |
| 6 | 0 | 1004 | 1004 | 7188 |
| 7 | 0 | 1208 | 1208 | 6984 |
| 8 | 0 | 749 | 749 | 7443 |
| 9 | 468 | 953 | 1421 | 6771 |
| 10 | 3730 | 338 | 4068 | 4124 |
| 11 | 1321 | 73 | 1394 | 6798 |
| 12 | 2672 | 225 | 2897 | 5295 |
| 13 | 3092 | 388 | 3480 | 4712 |
| 14 | 3397 | 72 | 3469 | 4723 |
| 15 | 4485 | 89 | 4574 | 3618 |

Header:16 Data/0 unknown. CHR:32 reporting chunks of8192 bytes; each0 Code/8192 Image/0 unknown. These reporting chunks do not redefine effective mapper CHR bank indices.

Execution coverage, separate:7567 distinct physical accepted opcode starts across24618 effective contextual roots;6312 starts from natural scenarios,5300 from artificial scenarios (overlap allowed). Decoded instruction-byte union16745, natural13990, artificial11676. No observed root conflicts with Data/Image. [Exact metrics](r001/metrics-final.json).

Gameplay: natural menu/team/pitcher selection, anthem, pitching, ball-in-play fielding, settings with Music/Icons toggles, and Play Offs entry; artificial half-inning transition, batting/strike, verified ninth inning, third out, handshake and credits. Bounded later credits continuation produced zero unknown code roots; final-score billboard was not separately captured. Full playoff championship/password behavior remains unvisited pending persistent-state contract. See [scenarios](scenarios.json), [gameplay](gameplay.md), and [investigation](investigation.md).

The next phase is full current-source commentary; this is not a terminal exhaustion assessment. Concrete redirect questions in [leads.json](leads.json) concern remaining menu/sprite/math/audio structures and series progression. Classification remains incomplete; fidelity verification alone does not resolve unknowns or certify readability. Required commentary/editorial work is pending. [Toolkit issues](toolkit-issues.md): no toolkit issues found; no blocker. [Compact handoff](handoff-disassembly-r001.md).

## Round 1 commentary — active

Input checkpoint disassembly-r001; frozen commentary baseline prepared and verified byte-identical. Read all 231 contiguous classified code spans (966 structural code blocks) through physical 0x1FFB7. Mechanical/routine interpretations saved in analysis/commentary-notes-r001.tsv; next step applies names, control-flow symbols and local data/unknown notes across commentary/source. Current-source readability is not yet complete. Review scaffold remains intact; final editorial acceptance reserved.

## Round 1 commentary — current-source readability complete

Frozen input disassembly-r001 is unchanged. Readability now covers all 20,869 classified code bytes / 966 code blocks / 231 contiguous spans across PRG banks 2 and 9–15, all 71 Data regions and one complete CHR Image region. All 260 unknown intervals have explicit local notes; no readability ranges remain. Full 1,298-entry physical review scaffold retained, current readability reviewed throughout, with 313 unique routine/structure units. Final editorial/archive acceptance remains pending; no final audit was run.

Buildable commentary source has 1,458 routine/control labels, 231 shared RAM/I/O symbols, five bounded-domain constants and five supported BIT-skip alternate-entry constants. Bank-aware control references use source labels only where saved views resolve their physical identity; 35 numeric control operands retain unresolved body/CPU-alias naming. Significant mechanical interpretations and uncertainty are preserved in commentary/source and commentary/uncertainties.md. All original structural classifications, evidence and history remain intact.

Fresh byte-identical verification passed after final source changes; commentary/reading-after.asm refreshed. Verification: analysis/commentary-r001-verification.json. Local text checkpoint: commentary/checkpoints/r001. Scope/review assessment and source hashes: analysis/commentary-r001-scope.json. Input ROM and maps/current.json are unchanged; no classification gain is claimed (321,066 classified / 72,166 unknown; 81.65%). Compact handoff: analysis/handoff-commentary-r001.md. Existing leads remain nonterminal redirect work. No toolkit issues found; no blocker, toolkit edits, publication, archive, cleanup or deletion.

## Redirect r001 active — 2026-11-20
Input maps/disassembly-r001.json; readability gate analysis/handoff-commentary-r001.md accepted. Attempt analysis/redirect/r001-20261120. Frontier: consumer-anchored menu, metasprite, quantized math, audio and playoff state; residual ledger analysis/r001/unknown-final.json unchanged. Map/source untouched.

Redirect r001 checkpoint: analysis/redirect/r001-20261120/research.md now records concrete direct-callee and safe indexed-RAM-store methods, 2499 candidate code bytes, first 14 sprite containers and active balanced-return contract solver. Frontier remains productive; continue same redirect before combined lead handoff. Map/source unchanged.

Redirect r001:22 packed screens now bound all7979 bank3 residual bytes;132 sequence graphs/158 audio patterns and8185 static-code proposal retained. Inline script19-handler/14-script graph inspected with stack-consuming barriers preserved. Generic non-code role inspection of remaining banks4/6–8 completed in data-inspection.txt; playoff team-selection experiment active. Detailed frontier in attempt research.md. No classification edits.

Redirect r001 checkpoint: preliminary coherent proposal union10345code+48317data=58662 bytes, exact residual13504 (input map still72166unknown), no code/data proposal conflicts. Constant-A MMC3 helper contracts and inline-script native returns extended connected flow. Naturally reached playoff bracket; exercising built-in NES-vs-NES Quick simulation. Durable current frontier/reproducible routes in analysis/redirect/r001-20261120/research.md and playoffs-bracket-small*. No phase completion yet.

Redirect r001: naturally exercised playoff bracket + built-in Quick scoring; verified artificial win-counter shortcut reached distinct trophy presentation. Explicit22/23-state actor,4 quadrant,9 IRQ and5 text-control dispatch graphs inspected. Corrected MMC3 non-PRG-register write premise expanded static graph to14383 unique candidate code bytes before union. Latest complete frontier/results: attempt research.md fourth checkpoint. Packaging supported batch; ROM/map/source unchanged.

Redirect packaging checkpoint: attribute RAM-pointer proof completed; all remaining explicit dispatch tables investigated; replay inverse/background bootstrap/counted actor helper contracts recovered. Latest pre-window union67213 proposed bytes, exact unproposed4953. One final scoped6000-RAM menu helper continuation running; then update combined handoff/lead IDs. Current map classification remains321066/393232 (81.65%) with72166unknown; no annotation.

## Redirect r001 complete — combined supported batch ready

Checkpoint remains disassembly-r001; map/state/ROM/source unchanged. Classification remains321066/393232 (81.65%),72166unknown. Proposed new bytes67974 (16341Code +51633Data), accepted0; exact unproposed4192/79 spans with complete residual dispositions. Eight stable ready leads in analysis/leads.json; combined recipe, context/boundary/overlap audit and fresh byte-identical/map-valid verification in analysis/redirect/r001-20261120. Natural playoff bracket/Quick plus artificial verified trophy visit added to scenarios. All initial promising avenues and connected explicit dispatch systems investigated; no unfinished active runtime experiment. Next phase: independent disassembly review using analysis/handoff-redirect-r001.md, then full current-source commentary and redirect. No terminal or whole-ROM completion claim.

## Round 2 disassembly — active

Input disassembly-r001 / analysis/state.json; redirect r001 combined 67974-byte proposal under independent structural review. Existing commentary r001 credit/source/checkpoint preserved; new classification will require delta and complete current-source carry-forward review. Reviewing standalone routine contracts, physical/context bounds, three BIT alternate entries and specific/generic data evidence before annotation; residual4192 bytes remain actionable unknown frontier. No unchanged gameplay replay planned.

## Round 2 disassembly — completed pass, ROM incomplete

ROM classification: 98.93% (389040 / 393232 bytes classified; 4192 bytes unknown).

Accepted the independently reviewed redirect-r001 batch: 16,341 Static Code + 51,633 Static Data = 67,974 new classified bytes. All prior regions, IDs, views and evidence are unchanged. Structural totals are 37,210 Code, 89,686 Data and 262,144 Image bytes. Header: 16 Data; CHR: 32 reporting chunks each 8,192 Image/0 unknown. The 4,192-byte/79-span residual is actual code/data or return/bank/inline uncertainty; unnamed but clearly non-code resource bytes have been classified. No terminal exhaustion claim.

| PRG 8 KiB bank | Code | Data | Classified | Unknown |
|---:|---:|---:|---:|---:|
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 0 | 8192 | 8192 | 0 |
| 2 | 2310 | 5745 | 8055 | 137 |
| 3 | 0 | 8192 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 0 | 8192 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 0 | 8192 | 8192 | 0 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 950 | 7093 | 8043 | 149 |
| 10 | 6080 | 641 | 6721 | 1471 |
| 11 | 1450 | 6742 | 8192 | 0 |
| 12 | 7027 | 721 | 7748 | 444 |
| 13 | 6650 | 1072 | 7722 | 470 |
| 14 | 6332 | 1049 | 7381 | 811 |
| 15 | 6411 | 1071 | 7482 | 710 |

Independent review: [notes](r002/review-notes.md), [fresh contextual decode/boundary audit](r002/independent-audit.json), [screen audit](r002/screen-audit.json), [annotation ledger](r002/annotation-rpc.jsonl), [metrics](r002/metrics-final.json), [exact complement](r002/unknown-final.json). Three BIT operand alternate entries are documented without overlapping annotations. All regions carry evidence; every Code region has bank-aware views. Specific script/frame/PPU containers have meaningful labels; generic non-code data retains localized role limits.

Fresh checkpoint [disassembly-r002](../maps/disassembly-r002.json), [generated assembly](../generated/disassembly-r002/game.asm), [verification](disassembly-r002-verification.json): map validated and entire 393232-byte ROM rebuilt byte-identically with the configured binary. ROM SHA256 remains 704cc53559abc5916fb136b4b85381eb4eb60f822252f9963e0482904807dc1c.

Execution coverage is separate: this static review adds no new observed fetches or gameplay replay. Prior r001 metrics remain valid for their recorded scenarios (7567 accepted opcode starts, 16745 decoded-byte union); redirect-r001's additional natural playoff bracket/Quick and artificial trophy visits retain their own scenario evidence without being relabeled here. Broad progression checklist already includes pitching, batting, fielding, ninth inning, handshake, credits and trophy. Repeating these scenarios would not resolve the remaining structural contracts.

Next phase: full current-source commentary carrying forward r001's complete review and adding all new classification; then redirect on the refreshed complement. Prior commentary/source and checkpoint remain untouched and are complete only for r001. Eight ready redirect leads are verified for their supported batch; preserve historical attempts and residual uncertainties. Residual investigation is deferred to redirect, not declared exhausted: narrower standalone helper/stack/return/pointer-domain invariants are possible continuations; see [residual dispositions](redirect/r001-20261120/residual-frontier.json) and r002 residual-review-decodes.json. No toolkit issues found; [persistent report](toolkit-issues.md). No toolkit edits, archive, publication, cleanup or deletion occurred. [Compact handoff](handoff-disassembly-r002.md).

### Commentary r002 active checkpoint

Entire prior commentary preserved at `analysis/round-2/commentary-r001`; original before/baseline/source credit remains reusable. Current comparison inputs are `commentary/baseline-r002.json` and `phase1-state-r002.json`. New code has 229 contiguous additions (16,341 bytes); hand-reading notes in `analysis/commentary-notes-r002.tsv` now cover additions in physical banks 2,9,10,11,12,13, through 0x1BA4D. Bank14 additions at 0x1C010 onward and bank15 additions remain to finish; all 51,633 newly classified Data bytes still need source explanation integration. Exact full addition ranges are `analysis/commentary-groups-r002.json`. No new current-source readability completion claim, redirect or final audit. Source integration/verification follows full code reading.

### Commentary r002 complete — current-source readability gate

All 67,974 new classified bytes are integrated into the maintained readable source: 229 hand-read native spans / 16,341 Code bytes and 195 Data regions / 51,633 bytes. Existing Code/Data/Image review credit and useful prose carried by exact physical range, with understood RAM/control operands normalized across old and new source. All 79 residual unknown intervals are locally explained; no readability ranges remain. Canonical commentary baseline/state/manifest/before match r002; the complete r001 tree is preserved at `analysis/round-2/commentary-r001`, and `checkpoints/r001` remains unchanged.

Current review: 8,657 full-ROM scaffold entries, 737 meaningful units including 313 carried. Fresh configured-release verification rebuilt all 393,232 bytes identically; reading copy refreshed. `analysis/commentary-r002-consistency.json` checks source locations, source hashes, baseline consistency and unchanged input ROM. Scope/hashes: `analysis/commentary-r002-scope.json`; compact next-phase input: `analysis/handoff-commentary-r002.md`. Text checkpoint: `commentary/checkpoints/r002`. No current-source readability backlog remains; final editorial audit/acceptance remains pending for coordinator. Classification remains 98.93%, with 4,192 unknown bytes. Source-derived residual constraints are in `commentary/uncertainties.md`; no redirect or terminal-exhaustion investigation was performed in this phase.

### Redirect r002 active

Attempt `analysis/redirect/r002-20261120` starts from disassembly-r002 and the exact 4192-byte/79-span complement. Reviewing narrowed standalone contracts and localized return/bank effects against primary source; no inherited missing-caller prerequisite. Input hashes and changed inference are durable in attempt records. No map/source edits.

### Redirect r002 research checkpoint — active, no classification edits

Attempt `redirect/r002-20261120` has a corrected flow12 candidate union of3775 currently unknown Code bytes and286 independently inspected Data bytes; these4061 proposed bytes leave131 unproposed bytes before ongoing timeout/continuation review. Accepted new bytes0; maps/state/source remain disassembly-r002 (98.93%,4192 unknown/79spans).

Two independently testable existing-type corrections are ready: `boundary-correction.json` narrows Data8528 (1BBF4 length18) to its first16bytes and proposes STY14 at1BC04/2bytes as Code; `roster-template-correction.json` proposes Code5737 (1AFBB/3bytes) as Data, the first three spaces of a25-space+NUL template. Deterministic CFA5 LDA01/STA28/BNE proves skip; CFCB copy consumer proves Data independently. The old CPU RAM2020 claim is retired by this narrower interpretation (2020 is PPU-register mirror space). All old evidence preserved.

Fresh native-return review localized and resolved scroll helpers DB1C and DA3C using proven06=byte>>3 selector bounds, exact16-entry03xx pointer columns and four bounded dispatch handlers. `scroll-pointer-contracts.json` records exact callsite constraints; no broad indirect-write exemption. Candidate roots now use source-backed8000 bank14 aliases and exact per-instruction PCs; provisional prior flow outputs are retained but superseded.

Current productive frontier: opcode6 menu timeout returns with consumed length7 and unchanged script pointer, a successor omitted by the prior parser. `inline-scripts-timeout.json` follows both selected and timeout routes:41 scripts,6 menu records,48 native exits, zero parser errors, including new0C/08 tails and title attract setup. Flow13 is following these exits and narrowly reviewed native continuation contracts. Final proposal audit, exact residual ledger, lead records and fresh fidelity/map verification remain pending. This is an active research checkpoint, not phase completion or terminal exhaustion.

### Redirect r002 complete — supported batch ready, ROM incomplete

Input/output checkpoint remains `maps/disassembly-r002.json`; map/state/source/ROM unchanged. Current classification98.93% (389040/393232;4192Unknown/79spans). Proposed3888StaticCode+297StaticData=4185 unknown bytes; accepted0. Existing-type corrections: Data8528 tail1BC04/2bytes→Code; Code5737 at1AFBB/3bytes→Data. Ready IDs R002-CONTROL-CONTRACTS, R002-MIXED-DATA, R002-TYPE-CORRECTIONS; prior histories retained.

Final audit `redirect/r002-20261120/audit.json`:1748 instruction records/2494 effective views, exact complement partition, no instruction/code-data overlaps or control-to-data conflicts. All final bank14 native aliases are8000. Fresh configured-release `verification.json` confirms map validity, whole-ROM byte identity and unchanged input/source hashes. `unknown-ledger.json` independently checks actual79-span complement; after proposals only7bytes/2spans remain (56D4/6;1DF7F/1), with primary competing-model review and no dead/unreachable claim.

Coherent connected/independent research, exact constrained recipes, deferred-route reasons and corrected exploratory premises: `redirect/r002-20261120/research.md`, `root-contracts.json`, `rejection-review.md`. The earlier auxiliary-return suggestion for9F6F was explicitly withdrawn; that byte follows the separate memory-test failure-loop JMP. Saved-mode906D cleanup is instead supported by an explicit independent stack contract, not a claim of948B actual return. No other concrete promising structural target is left outside this batch.

Compact handoff: `analysis/handoff-redirect-r002.md`. Next independent disassembly, then full current-source commentary and redirect. Current r002 readability credit is reused except correction-record dependencies; final editorial/archive acceptance pending. Representative later/ending routes and shortcuts reviewed/reused in scenarios.json. No new runtime/observed coverage. No toolkit issues found; no toolkit edits, archive/publication, cleanup or deletion. This is a nonterminal batch handoff, not whole-ROM completion or terminal exhaustion acceptance. Disk at handoff:3.2GiB free; no scratch directory, no deletion.

## Round 3 disassembly — pass complete, ROM incomplete

ROM classification: 100.00% (393225 / 393232 bytes classified; 7 bytes unknown).

Accepted 3888 new Static Code +297 new Static Data =4185 bytes after independent primary-contract review, plus2 Data→Code and3 Code→Data corrections. Totals: 41097 Code +89984 Data +262144 Image. Header16 Data; CHR32 reporting chunks each8192 Image/0Unknown. All classified regions have evidence; every Code region has bank-aware views. Exact unique-byte provenance: {'Observed': 16070, 'Static': 377155}. No new observed/natural reachability claim.

| PRG bank | Code | Data | Classified | Unknown | Provenance |
|---:|---:|---:|---:|---:|---|
| 0 | 0 | 8192 | 8192 | 0 | {'Static': 8192} |
| 1 | 0 | 8192 | 8192 | 0 | {'Static': 8192} |
| 2 | 2431 | 5755 | 8186 | 6 | {'Observed': 1436, 'Static': 6750} |
| 3 | 0 | 8192 | 8192 | 0 | {'Static': 8192} |
| 4 | 0 | 8192 | 8192 | 0 | {'Static': 8192} |
| 5 | 0 | 8192 | 8192 | 0 | {'Static': 8192} |
| 6 | 0 | 8192 | 8192 | 0 | {'Static': 8192} |
| 7 | 0 | 8192 | 8192 | 0 | {'Static': 8192} |
| 8 | 0 | 8192 | 8192 | 0 | {'Static': 8192} |
| 9 | 1099 | 7093 | 8192 | 0 | {'Static': 7746, 'Observed': 446} |
| 10 | 7380 | 812 | 8192 | 0 | {'Static': 4995, 'Observed': 3197} |
| 11 | 1450 | 6742 | 8192 | 0 | {'Observed': 803, 'Static': 7389} |
| 12 | 7452 | 740 | 8192 | 0 | {'Static': 6618, 'Observed': 1574} |
| 13 | 7119 | 1073 | 8192 | 0 | {'Observed': 2308, 'Static': 5884} |
| 14 | 7096 | 1095 | 8191 | 1 | {'Observed': 2461, 'Static': 5730} |
| 15 | 7070 | 1122 | 8192 | 0 | {'Observed': 3845, 'Static': 4347} |

Review: [notes and narrow invalidation](r003/review-notes.md), [fresh2494-view audit](r003/independent-audit.json), [primary witnesses](r003/primary-witness-decodes.json), [annotations](r003/annotation-rpc.jsonl), [metrics](r003/metrics-final.json). Both corrections independently confirmed before dependencies; Data8528 retains first16bytes and original ID, Code5737 retired in favor of exact roster template Data. All other prior regions are unchanged. Historical evidence and current commentary preserved.

Authoritative fresh checkpoint: [disassembly-r003-final](../maps/disassembly-r003-final.json), [whole-ROM generated source](../generated/disassembly-r003-final/game.asm), [byte-identical verification](disassembly-r003-final-verification.json). Configured binary validates map and byte-identically rebuilds393232bytes; ROM hash unchanged. The earlier disassembly-r003 checkpoint was inadvertently created while the annotation process had failed before saving; it contains the unchanged r002 map and is preserved as historical superseded verification. First annotation attempt removed a region then attempted to reuse its removed ID; in-memory-only failure left persisted state unchanged. Retry edits existing8528 directly; no toolkit bug.

Exact residual:56D4/6bytes (native versus inline/mixed ambiguity),1DF7F/1byte (orphan RTS after failure-loop JMP). Neither is classified by size/appearance. No terminal exhaustion assertion. Existing broad progression evidence and scenarios reused, no unchanged gameplay replay. Next full current-source commentary must review all additions, correct DBF4 boundary and roster/template/CPU2020 claims, carry reusable prior review, and preserve full historical copies; then redirect on refreshed7-byte complement. No commentary or redirect performed here. No toolkit issues found; [report](toolkit-issues.md). No toolkit edits, archive, publication, cleanup or deletion.

## Commentary r003 active

Preserved the entire prior commentary tree at `round-3/commentary-r002`. Hand-read all eighty new native spans; exact ranges and mechanics are in `commentary-groups-r003.json` and `commentary-notes-r003.tsv`. Integrated 4185 additions and the two narrow corrections without changing classification. Active canonical artifacts match final r003. All new data and both unknown spans have local notes; wrong roster JSR/RAM-upload prose is replaced and 906D explicitly requires a saved mode byte above its ordinary return address. Initial configured-release rebuild passed byte identity. Review/source locations and final consistency verification/read refresh are being finalized; no native/data readability ranges remain under review. Checkpoint and compact handoff remain, and final editorial audit is deferred.

## Commentary r003 complete

Current-source readability is complete across all classified source; no readability ranges remain. Preserved prior tree, refreshed canonical artifacts and integrated all additions plus exact corrections. Inventory:10345 entries/835 meaningful units; two unknown notes cover7bytes. Final fresh configured-release rebuild is byte-identical and reading copy refreshed. Local checkpoint: `../commentary/checkpoints/r003`; compact handoff: [commentary r003](handoff-commentary-r003.md). Next redirect uses exact `r003/unknown-final.json`; final editorial audit remains deferred and was not run.

## Redirect r003 active — corrected saved-tail continuation

Attempt `redirect/r003-20261120` starts from final r003/current state,7unknown/2spans. Primary inspection found a concrete corrected premise: preceding0C at56D1 skips toD6CA immediately but saves old-tailD6C3; selected DrawWinningTeamName explicitly JMPs83CD atD791, restoring saved cursor and interpreting D6C4 onward. Strong candidate first5bytesData, final60Code via00 synthetic-return arithmetic. Exact research/precondition/preservation check and primary source windows saved in attempt `research.md`, `primary-evidence.json`, `proposal.json`. No accepted/classification/source changes. Fresh configured map/source verification running; remaining independent1DF7F still materially ambiguous. Terminal assessment suspended; supported batch packaging underway.

## Redirect r003 complete — six-byte saved-tail batch ready

Corrected prior rejection by following explicit D791 JMP83CD saved-tail restoration; opcode0C computes57/58=D6C3 before nativeD6CA, so interpreter resumes D6C4. First5bytes inlineData, final60Code under exact terminator/stack contract. Ready R003-INLINE-SAVED-TAIL proposes6Unknown bytes; accepted0, current classification unchanged393225/393232 (99.9982198804%, display100.00%),7Unknown/2spans. Other independent singleton1DF7F retains scoped role ambiguity; no unsupported missing-caller requirement or deadness claim. Complete connected/independent avenue assessment in attempt research/rejection-review; no productive continuation withheld.

Fresh configured binary verification: map valid, source byte-identical, ROM/map/state/source hashes unchanged; exact current unknown complement verified. Broad later/ending shortcuts reviewed/reused, no unchanged runtime replay. Leads/scenarios updated preserving all history. Compact next-phase input: `handoff-redirect-r003.md`; primary proof `redirect/r003-20261120/primary-evidence.json`. Next disassembly/commentary/redirect, no terminal assertion. Final editorial audit not run; no toolkit edits, cleanup, archive/publication.

## Round 4 disassembly — single pass complete, ROM incomplete

ROM classification: 100.00% (393231 / 393232 bytes classified; 1 bytes unknown).

Exact percentage 99.999745697197%; rounding does not imply completion. Accepted R003-INLINE-SAVED-TAIL:5Static Data at56D4 and1Static Code at56D9. Totals 41098Code +89989Data +262144Image; header16Data; CHR32 reporting chunks each8192Image/0Unknown. Provenance {'Observed': 16070, 'Static': 377161}. All prior regions and entire commentary tree unchanged.

| PRG bank | Code | Data | Classified | Unknown | Provenance |
|---:|---:|---:|---:|---:|---|
| 0 | 0 | 8192 | 8192 | 0 | {'Static': 8192} |
| 1 | 0 | 8192 | 8192 | 0 | {'Static': 8192} |
| 2 | 2432 | 5760 | 8192 | 0 | {'Observed': 1436, 'Static': 6756} |
| 3 | 0 | 8192 | 8192 | 0 | {'Static': 8192} |
| 4 | 0 | 8192 | 8192 | 0 | {'Static': 8192} |
| 5 | 0 | 8192 | 8192 | 0 | {'Static': 8192} |
| 6 | 0 | 8192 | 8192 | 0 | {'Static': 8192} |
| 7 | 0 | 8192 | 8192 | 0 | {'Static': 8192} |
| 8 | 0 | 8192 | 8192 | 0 | {'Static': 8192} |
| 9 | 1099 | 7093 | 8192 | 0 | {'Static': 7746, 'Observed': 446} |
| 10 | 7380 | 812 | 8192 | 0 | {'Static': 4995, 'Observed': 3197} |
| 11 | 1450 | 6742 | 8192 | 0 | {'Observed': 803, 'Static': 7389} |
| 12 | 7452 | 740 | 8192 | 0 | {'Static': 6618, 'Observed': 1574} |
| 13 | 7119 | 1073 | 8192 | 0 | {'Observed': 2308, 'Static': 5884} |
| 14 | 7096 | 1095 | 8191 | 1 | {'Observed': 2461, 'Static': 5730} |
| 15 | 7070 | 1122 | 8192 | 0 | {'Observed': 3845, 'Static': 4347} |

Independent primary-window comparison, configured-release decodes and saved-cursor/stack/context review: [review notes](r004/review-notes.md), [fresh witnesses](r004/primary-witness-decodes.json), [annotations](r004/annotation-rpc.jsonl), [exact metrics](r004/metrics-final.json). Earlier immediate-skip rejection is corrected by following D791 JMP83CD, restoring old-tailD6C3; handlers1/9 consume1/3 bytes and zero atD6C8 synthesizes entry toRTS D6C9. Structural inference only; no new runtime observation or natural reachability claim. Original historical rejection remains preserved.

Fresh [checkpoint](../maps/disassembly-r004.json) and [whole-ROM byte-identical verification](disassembly-r004-verification.json) pass; current map/state persist exact accepted annotations. [Consistency](r004/final-consistency.json) confirms all prior regions, input ROM and full commentary unchanged. Residual [unknown complement](r004/unknown-final.json): singleton1DF7F/bank14 CPU9F6F; preceding JMP9F57 blocks fallthrough only, stored60/opcode size proves neither code nor data. No terminal exhaustion acceptance. Representative later/endgame gameplay routes reused from existing scenarios, no unchanged replay.

Next full current-source commentary integrates six newly classified bytes, replaces the stale six-byte Unknown note, explains saved-tail restoration and terminal caller-return, carries reusable full-source review and preserves historical copies. Then redirect on exact singleton complement. No commentary/redirect or final editorial audit performed here. No toolkit issues found; [report](toolkit-issues.md). No toolkit edits, archive, publication, cleanup or deletion. Compact [handoff](handoff-disassembly-r004.md).

## Redirect r004 — terminal INCOMPLETE proposed

Unique attempt `redirect/r004-20261120`, authoritative `maps/disassembly-r004.json`/current state. Exact393231/393232 classified,1Unknown file1DF7F bank14 CPU9F6F byte60; proposed0/accepted0, ready leads0. Scoped primary instruction/stack/consumer reassessment found no analogue to accepted bank2 saved-tail: success returns9F56, failure loops9F57, next sound entry9F70. Barrier excludes fallthrough only; standalone return and data remain indistinguishable, with no independent data/code discriminator. No universal live-caller prerequisite, deadness or whole-ROM claim. Compact `rejection-review.md` and direct ROM/source `primary-evidence.json` support coordinator stopping review; no credible revised inference/test deferred. Changed-premise reopen criteria and competing models in `research.md`/`handoff-redirect-r004.md`.

Fresh configured release map validation and whole-ROM current-source byte identity pass; ROM/map/state/source unchanged. Current readability complete; final editorial audit left to coordinator. Actual initialized later/endgame shortcut evidence reviewed/reused, no unchanged runtime replay. Leads/scenarios history retained and empty pass recorded. No toolkit issues found; no toolkit edits, classification/source edits, archive/publication, cleanup/deletion. Terminal INCOMPLETE is proposed pending coordinator acceptance, not a blocker or ROM completion.


## Coordinator final acceptance — active

Round4 commentary gate accepted from `handoff-commentary-r004.md`: 10346 reviewed entries/837 meaningful units, complete current-source readability, fresh byte identity and reading refresh, six-byte saved-tail correction integrated. Entire earlier commentary preserved under `round-4/commentary-r003`. Round4 redirect terminal INCOMPLETE rationale accepted after direct primary ROM-window comparison and scope/competing-model review in `coordinator-rejection-review.md`; zero ready leads, exact one-byte Unknown1DF7F remains. Current classification393231/393232 (99.9997456972%); whole-ROM classification is incomplete.

Separate final editorial worker corrected stale prose/review summaries and five supported CPU aliases; final source audit and read-only archive editorial-status sessions are live and pending. No completion claim until actual audit results, terminal handoff, document accounting and coordinator requirement audit pass. Independent current structural accounting in `current-audit.json` confirms physical code views/evidence, exact complement and map/state/baseline agreement. Toolkit unmodified; no archive/restore/publication/cleanup/deletion.


## Coordinator terminal handoff — orchestration complete, ROM INCOMPLETE

Final editorial sessions returned success: supported audit full-ROM byte identity,10346 reviewed entries/10522 unit references/837 unique units and read-only archive editorial-status complete with no reasons. Targeted stale prose,43 affected summaries and five supported CPU aliases corrected; final hashes/read copy independently checked. Coordinator rejection review accepted singleton ambiguity on decisive primary evidence, not missing-live-caller rules. Exact393231/393232 classified (99.9997456972%);1Unknown1DF7F, zero ready leads; no blocker. Authoritative checkpoint disassembly-r004, no further phase required on unchanged premises.

Final `terminal-handoff.md`, `current-audit.json`, `editorial-final-verification.json`, `completion-audit.json` and `archive-document-selection.json` bind the terminal result. `archive-options.json` preserves all additional final/historical conclusions and handoffs while explicitly omitting duplicate operational instruction copies; no final result omitted. ROM classification remains incomplete. No archive/restore, toolkit changes, publication, cleanup/deletion performed. Reopen research only under the documented changed-evidence/inference criteria.


## User-authorized batch residual fallback — disassembly complete

ROM classification: 100.00% (393232 / 393232 bytes classified; 0 bytes unknown).

The user explicitly authorized this batch-only exception: verify the exact current physical unknown complement first; only if at most ten bytes remain, annotate the remaining expected singleton physical offset122751/length1 as generic Data of unknown meaning using supported map.annotate and Manual provenance. If more than ten remain, stop blocked. The user prohibited further residual research and requested preservation of all prior ambiguity/history. This authorization overrides the structural-evidence requirement only for this residual fallback; it supplies no new structural evidence.

[Before accounting](before.json) verified393231 classified/393232 total, exactly one unknown byte at physical$1DF7F (122751), PRG bank14 CPU$9F6F. It satisfied the gate before mutation. The byte$60 remains materially ambiguous between standalone RTS and data. The preceding JMP excludes immediate fallthrough only. All prior research and rejection evidence remain unchanged, including ../redirect/r004-20261120/rejection-review.md and primary-evidence.json. No observed execution, natural reachability, deadness, specific format or game meaning is asserted. Confidence1.0 records the explicit manual decision, not independent structural certainty.

Supported map.annotate added region10347, Data/Manual, no invented semantic label. [Successful RPC ledger](annotation-retry-rpc.jsonl) records load/annotate/validate/save/export. Every prior region is byte-for-byte unchanged; [after accounting](after.json) verifies exact complement empty, totals41098Code +89990Data +262144Image =393232, provenance16070Observed +377161Static +1Manual. PRG bank14 now7096Code/1096Data/0Unknown, provenance2461Observed/5730Static/1Manual; all other banks retain prior counts. Header16Data; CHR262144Image; full per-bank counts are in after.json. Structural evidence independently supports393231bytes; the last byte is the explicitly authorized manual fallback.

Authoritative fresh checkpoint: maps/disassembly-fallback-final-verified.json, generated/disassembly-fallback-final-verified/game.asm, analysis/disassembly-fallback-final-verified-verification.json. Configured release binary validates the map and byte-identically rebuilds all393232bytes; original ROM SHA256 remains704cc53559abc5916fb136b4b85381eb4eb60f822252f9963e0482904807dc1c. First annotation attempt used the wrong NES parameter envelope and was rejected without persisted changes. An inadvertently continued checkpoint disassembly-fallback-final contains unchanged pre-fallback state; it is preserved as superseded history, not the authoritative final checkpoint. Retry used the supported region envelope. This was an analysis-helper error, not a toolkit defect.

No research, runtime replay, commentary phase or editorial audit was performed in this disassembly phase. Coordinator must integrate this manual decision into current-source commentary/final accounting and perform remaining final gates. Prior readability and terminal records remain historical; zero unknown bytes does not turn the manual exception into independent evidence. No toolkit issues found; ../toolkit-issues.md preserved. No toolkit/ROM edits, deletion, archive or publication.

Full local record: [fallback final](fallback-final/final-record.md).


## Coordinator final acceptance — user-authorized residual exception COMPLETE

ROM classification: **100.00% (393232 / 393232 bytes classified; 0 bytes unknown)**. Exactly393231 bytes retain independent structural evidence; the final1byte at physical122751/length1 is generic Data/Manual solely under the user-directed at-most-ten-byte fallback. Prior RTS/data ambiguity remains; no new structural/reachability claim. Exact pre-mutation complement, annotation authorization and prior research are retained in `fallback-final/final-record.md`. Every prior region is unchanged.

Separate sequential disassembly, full current-source commentary and final strict editorial workers completed. No residual redirect research was reopened. Final supported audit actually exited successfully with fresh byte-identical rebuild; independent read-only editorial status passed10346regions/10522unit references,837unique units, no reasons. Coordinator independently matched source/report hashes, reading expansion, exact scaffold, state/map/frozen baseline/final checkpoint and unchanged ROM. No readability/editorial/verification blocker remains. Authoritative checkpoint `disassembly-fallback-final-verified`; current-source readability covers all classified source. Prior terminal and full commentary remain preserved.

Final `terminal-handoff.md`, `current-audit.json`, `editorial-fallback-final-verification.json` and `completion-audit.json` record current acceptance. `archive-options.json` includes the final fallback record and all final/historical conclusions; every unrecognized Markdown accounted under inspected archive.py contract, only duplicate historical operational instructions omitted. No archive/delete/cleanup/publication or toolkit edits. No toolkit issues found. This completes the explicit fallback workflow, not independent proof of the Manual byte's structural role.
