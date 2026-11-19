# Disassembly progress

Round 01, phase: commentary completed; classification incomplete. Structural checkpoint: pass-001-final. All currently classified source is readable in commentary/source with fresh byte-identical verification and refreshed reading copy. Next phase: redirect on analysis/leads.json; compact handoff analysis/handoff-commentary-round01.md. Final editorial audit remains pending for final acceptance.

Baseline established: [baseline-verification.json](baseline-verification.json), map `maps/baseline.json`, source `generated/baseline`; byte-identical true. Input SHA-256 remains a5832cc017a5ae484cf3bcc2f0829eaef380d4876ab1f4dd22f2e5a9d8e956e6.

ROM classification: 0.00% (0 / 131088 bytes classified; 131088 bytes unknown).

Runtime startup identified title **The Battle of Olympus**, MMC1, 128 KiB PRG, CHR RAM. Reset PC $FFDA physical $1FFEA, initial context [0,1,14,15]. Startup observed context [12,13,14,15]. At 20 frames trace summary: 927 unique opcode addresses, aggregate_dropped=0. Detail capacity 1000 intentionally drops old detail; retained aggregates support discovery. A 180-frame/10-second attempt timed out at frame 20, bounded partial execution retained. Selected title capture: `analysis/evidence/title.png`.

Current session: `scratch/rpc.py serve`, configured binary via run.py serve; request/response file bridge in scratch. No durable snapshots yet. Pending: startup/menu progression, inspect/confirm discovery, broad gameplay shortcuts, save state/map and fresh checkpoint. No leads existed at start.

## Resume bridge

Persistent server is tool exec session 29039 (Python file-RPC bridge), which owns `python3 run.py serve`. `python3 scratch/rpc.py METHOD 'JSON_PARAMS'` writes one request and waits for reply; do not issue concurrent bridge requests. Client helpers are in scratch/client.py. On process loss restart bridge; load analysis/state.json if present and emulator.start plus snapshot.import. No state map saved yet. `analysis/evidence/title349.snapshot` is durable same-build startup snapshot, before artificial edit. Trace history is session-local; title-wait runs disabled tracing to avoid expensive redundant loop aggregates. Source-informed $02=1 delay patch marked subsequent execution artificial; preserve that distinction. Runtime frame 392 at last inspected capture; longer untraced title wait ongoing.

## First classified checkpoint

`pass-001-initial` verified byte-identical: [verification](pass-001-initial-verification.json). Header, eight uniform non-code FF bank tails, repeated ASCII/metadata trailers, final/mirrored vector records and reviewed reset/NMI static slices classified.

ROM classification: 3.83% (5017 / 131088 bytes classified; 126071 bytes unknown).

Persistent cross-exec server handle 29039 exited 143 without stderr; bridge clients stalled awaiting response. Attribution to execution lifetime vs toolkit is uncertain, not a demonstrated toolkit bug. Recovery now uses batch-local persistent JSON-RPC subprocesses, saves after bounded experiments; `scratch/batch.py`. Static candidate pagination returns next_offset equal total at end, so stop on >=total (fixed analysis client, no toolkit change). All current durable classifications saved in analysis/state.json and maps/current.json. Startup replay being recaptured to classify actual fetches. Earlier untraced 1200-frame recovery exceeded 30-second budget; no classification claimed from that lost experiment.

## Startup fetch classification batch

Recovered batch-local server succeeded: 25 reset frames, 927 unique physical opcode starts (multiple effective context views retained), all individually decoded and inspected for immutable operand bounds. [Fetch/context evidence](evidence/startup25-observed-review.json), [summary](evidence/startup25-summary.json), [discovery](evidence/startup25-candidates.json). Map saved after validation. Snapshot `analysis/evidence/startup25.snapshot` permits replay/branching; no artificial edits in this evidence.

ROM classification: 5.23% (6858 / 131088 bytes classified; 124230 bytes unknown).

Current frontier: source inspection found title intro delay at $EE8B-$EEA5. Scroll uses $FB incremented to $F0, with $02 frame-delay loop $D40F. Snapshot frame349 is within this understood scroll contract. A bounded RAM edit $FB=$EF/$02=1, then $02=1 after eight frames, now tests shortening the title animation. It is artificial evidence and will be inspected before claiming a menu visit. Runtime source evidence: `evidence/decode-ee11.json`, stack `evidence/title-stack.json`. Main old bridge is dead; all active work uses self-contained batch scripts with direct persistent subprocesses; no dependency on old named snapshots.

## Graphics consumer batch and naming

ROM classification: 13.92% (18249 / 131088 bytes classified; 112839 bytes unknown).

Fresh checkpoint pass-001-graphics verified byte-identical (analysis/pass-001-graphics-verification.json). 10090 data bytes added: finite 22-byte graphics descriptor table, 17 selected pointer words, and 17 exact header/encoded graphics records. Consumer $E600/$E649/$E681/$E6A8/$E6CD establishes bank, header fields, 81 escape/repeat semantics and output counts; independent adjacent pointer endpoints agree. Parser result evidence/graphics-parse-proposals.json, implementation scratch/graphics_parse.py; all complete with zero pending repeats. Tile-half mode derived from selector byte position <3, not arbitrary fit.

Correct buttons-array input reached naming, accepted names 000000/000000; current region-map load actively executes $ECxx/$D2xx decode and VRAM transfer code. Prior source-informed $EE64+stackFD shortcut bypassed map transition animation; underlying DE=1, DF=0, health16. Frame499 screen still black while renderer uploads; no Arcadia visit yet. Durable snapshot arcadia-entry.snapshot is named by intended destination, actual result captured/recorded explicitly. Bounded 20-frame continuation underway.

## Area tile and map transition batch

ROM classification: 23.71% (31082 / 131088 bytes classified; 100006 bytes unknown).

Thirteen consumer-bounded area tile streams and pointer domain added; see investigation.md. Greece map visually confirmed Arcadia, then Start launched stage loading. Latest durable snapshot arcadia-scene.snapshot, frame536 (black load interval); 686 further actual instruction roots accepted with evidence/arcadia-scene-review.json. Next bounded continuation inspects rendered gameplay. All post-shortcut scenarios artificial; runtime evidence does not imply natural reachability.

## Arcadia mechanics / region shortcut test

ROM classification: 25.67% (33644 / 131088 bytes classified; 97444 bytes unknown).

Arcadia valid scene and jump/attack now visibly confirmed (arcadia-visible.png, arcadia-jump-attack.png). Inventory capture taken during loading; pending follow-up. Area01 direct C144 shortcut executes but corrupts graphics, so no area01 visit claimed. Source continue path calls D300 to blank rendering before C144; revised bounded test adds this missing precondition and proper nested return frames. Old tests retained; artificial fetch evidence remains code evidence even if display invalid. Checklist updated.

## Supported later-region visit

ROM classification: 29.03% (38058 / 131088 bytes classified; 93030 bytes unknown).

Area01b region shortcut now visibly valid after adding D300 blanking before C144; terrain/player and jump/attack observed. See scenarios.json and area01b-action.png. Area04b same corrected recipe currently loading/acting; no visit claim until screenshot inspected. Connected static data added 12 area descriptors/header-pointer records, 64-byte per-area attributes, nine palette-setup source bytes, four subtable pointers each, and 256-byte metatile quadrant tables each (4080 bytes total). Consumer decodes and area-header-proposals.json establish boundaries. Current frontier: representative regions5/7/8/final area11; boss/ending entry contracts not yet established.

## Representative progression and terminal scene disposition

ROM classification: 31.77% (41647 / 131088 bytes classified; 89441 bytes unknown).

Valid areas1/4/5/8/11 scenes and actions inspected, distinct terrain/interior/enemy systems confirmed. Large flying foe in area11 active; exact identity/defeat unproven. Ending routine EEE4 rendered rescue scene; self-initializing EF33 suffix rendered final couple scene, with source EF54 looping to EF33. Full credits text/fade timing not cycled, natural ending unproven. Last20 terminal scene frames gained zero opcode starts; no further repeated fade waits planned. Checklist and recipes updated. Inventory follow-up and one targeted house/conversation attempt remain, followed by final static review/checkpoint/handoff.

## Round 01 final handoff

ROM classification: 31.93% (41854 / 131088 bytes classified; 89234 bytes unknown).

Code: 9625 bytes; data: 32229 bytes (including all16 header bytes). Exact physical accounting: [round01-final-counts.json](evidence/round01-final-counts.json); residual intervals: [round01-unknown-ranges.json](evidence/round01-unknown-ranges.json). Counts come from validated analysis/state.json, excluding gaps/raw fallback.

| Physical 8 KiB PRG bank | Code | Data | Classified | Unknown |
|---:|---:|---:|---:|---:|
| 0 | 1001 | 0 | 1001 | 7191 |
| 1 | 703 | 416 | 1119 | 7073 |
| 2 | 0 | 2261 | 2261 | 5931 |
| 3 | 0 | 1578 | 1578 | 6614 |
| 4 | 0 | 1851 | 1851 | 6341 |
| 5 | 0 | 2027 | 2027 | 6165 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 0 | 5629 | 5629 | 2563 |
| 8 | 0 | 0 | 0 | 8192 |
| 9 | 0 | 476 | 476 | 7716 |
| 10 | 0 | 6344 | 6344 | 1848 |
| 11 | 0 | 912 | 912 | 7280 |
| 12 | 0 | 232 | 232 | 7960 |
| 13 | 278 | 1349 | 1627 | 6565 |
| 14 | 3914 | 12 | 3926 | 4266 |
| 15 | 3729 | 934 | 4663 | 3529 |

Fresh checkpoint [pass-001-final-verification.json](pass-001-final-verification.json): ok=true, byte_identical=true. Map maps/pass-001-final.json SHA-256 9307713dd31e9c9a4e42b03c88e433207a4620942ae1194ae93a6de4a710cb57. Generated source generated/pass-001-final. Input SHA-256 rechecked unchanged. Internal state and maps/current.json saved/validated. Readability/editorial review remains for commentary; generation fidelity does not prove editorial completeness.

Gameplay: valid title/menu, names, Greece map, Arcadia movement/jump/attack/inventory, distinct areas1/4/5/8/11 and large flying encounter, rescue/final-couple terminal scenes. All gameplay after title shortcut is artificial; no natural boss defeat or ending claim. Named boss identity/defeat, NPC conversation, special-item travel/water/ceiling mechanics and other areas remain unresolved. Inventory visible confirmed; two house-position/Up/B attempts produced no room/dialogue change or new code. Representative late-region/ending visits succeeded; repeated final fade20 frames gained zero starts. See scenarios.json and gameplay.md for exact routes/limitations.

No live server is required to resume: batch scripts start configured run.py serve; all selected states are durable same-build snapshots. Old dead sessions29039/39297/91729 confirmed exit143; no toolkit attribution established. Do not reuse the old file bridge. Snapshot imports require emulator.start and explicit full buttons array.

Toolkit: one confirmed documentation/API-validation finding NES-DOC-001, zero suspected bugs or missing-feature findings, no blockers; [toolkit-issues.md](toolkit-issues.md). Correct NES inputs use {buttons:[...],port:1} and buttons:[] release. Toolkit commit3bb1dda0d069721ed9ad44088f172dbd01ae5216. No toolkit edits.

Disk at handoff:4.4GiB free, evidence211MiB, scratch304KiB. No cleanup, deletion, archive, publish or commit performed. This is an ordinary incomplete easy-gain handoff, not terminal structural exhaustion; concrete research questions in leads.json and handoff-disassembly-round01.md should go through commentary then redirect.


## Round 01 commentary handoff

Current-source readability complete across all 9,625 code bytes and 32,229 data bytes. Full 3,742-entry scaffold reviewed, 342 authored routine/block explanations, 793 branch/routine aliases, 183 data-structure explanations and 284 explicit unknown-boundary notes. Contextual scratch, RAM/field/hardware/constant and consumer-table aliases preserve byte widths and bank identities. Unknown bytes remain 89,234; classification is unchanged at 31.93% (41,854 / 131,088). No current-source readability gaps remain.

Fresh [commentary verification](../commentary/verification.json) has ok=true / byte_identical=true; [reading copy](../commentary/reading-after.asm) refreshed after final source edits. Source hashes match the verification report; frozen scaffold and ROM/map/state hashes checked unchanged. Review evidence: [scope](../commentary/readability-round01.json), [review](../commentary/review.json), [phase progress](../commentary/progress.md), [handoff](handoff-commentary-round01.md). Final editorial audit intentionally pending for final complete or terminal-incomplete acceptance.

Redirect retains concrete questions about room/actor bodies, inline dispatch domains, NPC/door/quest/boss contracts, banked audio grammar and mirrored reset mapping. See [uncertainties](../commentary/uncertainties.md) and leads.json. No new toolkit issues encountered; existing confirmed NES-DOC-001 remains the only report entry and no blocker. No toolkit changes or archive/publish/commit/deletion. Source and artifacts remain local.

## Redirect round01 — attempt round01-a01 (active)
Checkpoint pass-001-final / map SHA 9307713dd31e9c9a4e42b03c88e433207a4620942ae1194ae93a6de4a710cb57; 41854 classified, 89234 unknown. Commentary gate independently accepted by coordinator. Research only; no structural/source edits. Plan and exact unchanged unknown complement: analysis/redirect/round01-a01/{plan.md,unknown-ledger.json}. R01-L01: consumer-derived six-byte room records; bounded compressed tile parser succeeds for 113 inferred records across 12 areas (check aggregate next); first-pointer-only extent rejected because area6/8 share/out-of-order bodies. Corrected earliest-target constraint yields aligned record boundaries. Four-entry room command dispatcher established by AND03; missing attribute handler supports narrow code proposal. Active frontier: independently review dictionary/stream bounds and marker-list termination; then actor initializer continuations and independent L02..05.

Redirect round01-a01 update: corrected record aggregate is105 (earlier113 was arithmetic slip). Room candidate union26487 unknown bytes; marker list consumer D6A6..D6D4 explains first4 metadata bytes then high-bit/zero-terminated entrance refs; overlapping marker/actor regions area8 are shared terminators, not layout failure. Actor connected CFG:5294 instruction starts,4562 new/9321 candidate code bytes, no internal instruction overlap or accepted-data conflict, all official decode. Supported root domains: room spawn types, initializer negative-tag low7 domain, normal per-type handler table, AND03 inline4, LSR5 inline8; small state-dispatch tables under explicit local state preconditions. Static only, no execution/natural claims. Pending independent review of state table contracts.

Audio L04 correction: physical16KiB bank6 selected by A06, not bank4.25 music records via8008..8039 pointer table,17 fixed-bank effect records via FA1D..FA3E;6032 nonoverlapping stream-command starts,7717 candidate unknown bytes. Grammar0C..1F validated against20 interpreter targets;16 nested pointer,17 return,18 loop start,19 loop end,1A..1C channel-mask terminal. Candidate intervals and exploration preserved.

L05 important prior semantic correction: accepted13th area-CHR record index12 at physical C028 points B0D0/F0E0, actually D6D8 special-room area directory. Existing first12 area-CHR entries remain supported. F0E0..F0F8 twelve area pointers;F0F8..F114 seven graphics-pointer pairs;F114..F282 sixty-one six-byte room records.58 records selected by entrance prefixes;three further records structurally supported with explicit16-column preconditions (including area11 mode3). All61 room streams parse with dictionary bounds and exact output16/32*11; metatile indices masked3F. Last additional room stream FD68..FDC0 leads lastquadrants/attrs/palette throughFE28, leaving6 trailing bytes FE28..FE2E not yet resolved. Relabel/split existing Data required next disassembly; no current map edit. Saved special-room-parse.json and parser.

Active frontier: consolidate room/audio/actor exact proposals; inspect NPC/door input consumer and a bounded interaction shortcut, copied-reset effective mapping, remaining initial dispatches, and obvious data continuations. This is active redirect with credible productive continuations; no terminal exhaustion claim. Map checkpoint unchanged.

Redirect round01-a01 milestone: door2 recipe successfully enters area0 special room2 via derived Up interaction (X168,Y176, grounded/stationary gates). door1 was interrupted by saved entrance animation and did not reachD686; door2 waits/restores stable scene and clears pendingmovement. Actual D686 fetch, roommode0->2, rendered NPC retained. NPCDown failed because visibleNPCtype1 uses89BB rather than type4/8C3A. Corrected B recipe succeeds, game_mode1, rendered text "first you must go to the Temple..."; npc2 artifacts and snapshot. This opens fixed DA6E/DB03 script interpreter and bank5 data consumer D646; currently deriving script opcode table/grammar. No natural reachability claim; RAM placement artificial, actual inputs tested.

Actor connected continuation now5535 starts incl produced types1/2/3 mechanics (actor-mechanics-cfg.json). Existing9321 candidate code count predates these additional starts; refresh pending. Area graphics/palette sources parsed (actor-graphics-parse.json): bank1 twelve per-area arrays and terminated graphics-key lists. Bank4 entire52-record pointer directory/2-byte-count RLE streams independently parses; each nonfinal endpoint equals next uniqueorigin, last ends13E2E, matching knownFFtail. Lists select49 indices; remaining3 structurally consistent records do not require natural selection. Concrete new data avenue, not generic wholebank inference. Palette/graphics-array extents differ in a few shared/extra cells; retain separate domains, do not force commoncounts.

Mirrored reset bounded RAM-executed mapper program succeeds (reset_probe3.py/reset-mapping-tests3.json): mode2 maps each bank0..6 atC000 whilelower0; actual FFDA instruction fetched atphysical3FEA/7FEA/BFEA/FFEA/13FEA/17FEA/1BFEA with contexts[0,1,2*i,2*i+1]. INC resetsMMC1, nextFFDD fetch comes from1FFED finalbank;21 new observed bytes. Othercopies'3-byteJMP suffixes remain structurally ambiguous under actual mapping, do not classify6bytes percopy automatically. First probe budgets only enteredNMI; documentedNESdebugRAM-onlywrite rejectsI/O attempt; successfulRAMprogram disablesNMI via executedSTA2000, waits2000 bounded operations and preserves complete recipe. Trace aggregates persist across snapshotrestore; union only, each newlymapped bank fetch is identifiable. Configuredbinary unchanged.

Active frontier now: script interpreter and bank5 bounded text records; fixed finite dispatch domains (main4, movement14, keyboard6, room4, entrance5, audio20), actor metadata tables associated with accepted consumers; finalexactproposalmanifest,unknownledgerdispositions and combinedhandoff. InitialL01..05 all meaningfully investigated, but productive continuations remain; no terminal assessment.

Redirect round01-a01 consolidation update: NPC dialogue completed visible sentence "first you must go to the Temple of Arcadia to meet Zeus" and B returned game_mode0 (npc-full-text.png, npc-finish.json). D62F explains two-stage script initialization: opcode0 installs actor selector and advances42; later B enters remaining script. Dialogue parser now92 origins,365 opcode nodes,134 count-prefixed five-bit text packets, zero errors, exact contiguous union158DA..179B9 (8415 bytes). Residual705 bytes179B9..17C7A exactly duplicate1F9B9..1FC7A; first3 frequency-tail bytes, standalone envelope routine B9AC..BA1B with supported Y/channel RAM/APU contract, and effect data. Copied routine has coherent local branches/no mapper effect; caller/natural reachability unknown separately.

Latest CFGs actor-final5681 starts, fixed-final6351 starts, scene-final320 starts; finite original dispatches and associated newly produced actor1/2/3 continuations examined. Fixed alternate BIT/LDA operand entries at DAA8/DAA9 and DE9B/DE9C are intentional and need raw-byte overlapping-entry comments, not misclassification. Scene bank6 B532 JMP(C000) resolves immutable vector word D419; local palette dispatch B49C has3 handlers, B602 has9 pointer records. Current raw proposal manifest proposals-raw.json contains25024 claims,4632 actual-fetch root/context combinations plus static CFG/data claims, no map writes. Consolidation now checking exact code/data conflicts and complement; this will expose any narrow remaining useful continuation.

Reset reassessment: missing local suffix fetch is not rejection evidence. Eight exact6-byte templates, FFDA vector origins and independent FF/trailer boundaries strongly support copied JMP instruction role. Logical reset contract crosses physical bank identity after INC, so continuous same-copy execution is not claimed. Record explicit competing stored-template-data interpretation and decide structural proposal separately from reachability; coordinator requested this review. Raw manifest currently includes21 observed INC bytes, suffix proposal pending this reasoned review. All prior outcomes preserved.

Redirect round01-a01 final research boundary: residual review found the missing map/title directory consumer EC00 and repeat/script consumers D808/ED57 (not an unreferenced opaque block). map-assets-parse.json now proves three960-output dictionary/RLE map bodies: two end exactly at64-byte attributes, third ends at next descriptor A611; the name UI's unused attribute word aliases dictionary bytes and is not a valid ROM pointer. Thirteen terminated PPU scripts and eight region marker/count-name records parse, marker end exactly1A9A3 area11 header. Scene-assets-parse.json proves six sprite-row records, two palette-token streams, scene PPU/fade script, four palette-index streams, coordinate/color arrays. Corrected palette-vs-PPU distinction at B6CC against B4A2 consumer; all endpoints match adjacent independent origins.

Audio command1A is a temporary channel barrier: F729 resets music_mask2 and resumes, so parser now continues1A,7030 command starts, no errors;1B restarts record pointers,1C excludes channel. Prior6032-node model preserved in history as premature barrier-stop interpretation, not exhausted streams. Alternate actor dispatch9061..90D1 has56 words for types20..75, exactmax selector55 and next handler boundary; corresponding continuations added. Complete graphs now actor-structural6113 starts, fixed-structural6639, scene-final320. A few isolated actor/player helpers have explicit static calling preconditions but no actual caller claim. All official decode; deliberate overlapping BIT/LDA entries remain localized.

Proposal consolidation found and corrected three offset-zero aliases of preceding JMP operands (9155,9FDF,F325). Also rejected an eighth area11 graphics pointer: first list pointer is not earliest because lists share/out-of-order origins; proper7-entry room prefix ends at earliest list452A. Parser now validates targets8000..BFFF and manifest validates all physical intervals0..131088, avoiding negative-offset aliasing. Current map/ROM remained untouched. Latest review before map/scene additions:17112Code+64918Data new bytes,7204 unproposed, zero conflicts. Final generic-data inspection/ref report and complement refresh pending; primary unknowns after these additions are patterned actor/render parameter arrays, copied template suffixes and a few ambiguous orphan bytes, not productive undiscovered handlers.

Reset decision for independent review: propose seven remaining copied JMP suffixes as Static Data instruction-template bytes, not falsely mapped executable continuations. Actual reset contract fetches INC from selected upper bank then switches before JMP; exact8-way template/vector/boundary evidence establishes stored duplicate role, without claiming local deadness or requiring natural fetch. Next disassembly may choose a stronger static Code image representation if it can preserve faithful effective execution views; no silent guessed view. The21 actual INC bytes remain Observed. This is structural role versus reachability separation, not rejection from no-fetch alone.

## Redirect round01 complete — awaiting independent disassembly

Final ready batch R01-L01..L06; R01-L07 blocked/non-actionable residual. Unique new proposals17112Code+72108Data=89220, zero proposal code/data conflicts; current accepted41854/131088 and89234 unknown unchanged. Remaining unproposed14bytes across4 ranges have exact primary-artifact review. Full unchanged unknown complement has evidence-linked dispositions in unknown-ledger-final.json. Current compact handoff rewritten coherently; intermediate handoff preserved under attempt history. All identified promising original and connected independent avenues investigated to coherent structural proposals, meaningful intermediate conclusions or concrete diminished/ambiguous prospects; no fixedquota or terminal exhaustion claim.

Next phase: independent disassembly validates/classifies combined batch and semantic corrections, then full current-source commentary and redirect. No new toolkit findings; existingNES-DOC-001 no blocker. No publishing/commit/archive execution or toolkit/input/source/map/state mutation. All artifacts local.

Final invariant check: ROM/map/state/commentary source/symbol hashes match accepted commentary checkpoint exactly. Sound7030 command starts have no overlapping command bytes; all bounded room/special/graphics parsers pass. Review/summary/leads/ledger/scenario records consistent. Attempt77MiB, disk4.4GiB free. Handoff ready for coordinator inspection.

## Disassembly round02 — independent review active
Input pass-001-final, map SHA256 9307713dd31e9c9a4e42b03c88e433207a4620942ae1194ae93a6de4a710cb57. Reviewing combined R01-L01..L06 structural contracts, exact proposals and primary parser/CFG evidence before map changes. Baseline 41854 classified / 89234 unknown. Commentary/source and before preserved. Frontier: validate format endpoints, finite dispatch and mapper barriers; residual14 separately retained pending review.

## Disassembly round02 — verified easy-gain handoff

ROM classification: 99.99% (131074 / 131088 bytes classified; 14 bytes unknown).

Input pass-001-final; output pass-002-final. Accepted 89220 new unique bytes (17112 Code / 72108 Data), independently reviewed coherent R01-L01..L06 batch. Total Code 26737, Data 104337. Header16 Data counted separately from bank table.

|16 KiB bank|8 KiB bank pair|Code|Data|Unknown|
|---|---|---:|---:|---:|
|0|[0, 1]|12576|3808|0|
|1|[2, 3]|3|16381|0|
|2|[4, 5]|3|16375|6|
|3|[6, 7]|3|16375|6|
|4|[8, 9]|3|16381|0|
|5|[10, 11]|116|16268|0|
|6|[12, 13]|884|15500|0|
|7|[14, 15]|13149|3233|2|

Review: [structural reasoning](disassembly/round02/review.md), [independent byte/context validation](disassembly/round02/independent-validation.json), [accepted annotations](disassembly/round02/accepted-regions.json), [exact summary](disassembly/round02/summary.json). Old849-byte false AreaTiles_0C interpretation split into specific special-room directory/records/body roles; previous annotation retained. BIT/LDA overlap entries retained with exact-byte checkpoint representation and entry-views.json. Seven reset INCs Observed, suffixes Static instruction-template Data; copied sound code Static with explicit contracts and unresolved actual callers.

Existing ordinary/late-area, rescue/terminal-scene and successful NPC gameplay evidence reused; no unchanged replays. Execution coverage is separate: no new emulator exploration was performed this pass and no natural reachability promoted. All accepted Code has effective views; all classified bytes have evidence/provenance; semantic field knowledge and current-source editorial coverage remain separate. Current-source commentary must carry forward/review the new full map; commentary/source and before remain intact.

Residual14 bytes at BE28..BE2E,FE28..FE2E,1CC9C,1CDE5 remain unknown under localized primary review; this is ordinary easy-gain exhaustion, not terminal exhaustion or ROM completion. Full current-source commentary then redirect are still required. Toolkit report: one existing confirmed documentation/API-validation issue NES-DOC-001, zero new findings, no blocker; [report](toolkit-issues.md). No toolkit/input/archive/commit/publishing changes.

## Round02 commentary active — pass-002-final

Previous commentary preserved in round-02/commentary-round01; fresh verified baseline prepared. Physical-range carry and exact pending frontier recorded in ../commentary/progress.md and commentary/round02/carry.json. Full current-source readability and final verification pending; classification remains 99.99%, 14 unresolved bytes.

## Round02 commentary outcome — pass-002-final

Full current-source readability gate completed. Classification unchanged:131074/131088 bytes (99.99%),26737 Code/104337 Data/14 Unknown. Entire previous commentary preserved under round-02/commentary-round01 before fresh preparation. All 10920 current scaffold entries reviewed;2088 inventory units/972 code units, with 630 newly named code entries/blocks across369 new Code regions. All 7090 new/corrected Data regions explained. Current exact remaining source frontier is empty: commentary/round02/frontier.json.

Fresh byte-identical final verification and refreshed reading copy passed; current source hashes, full coverage/anchor checks and remaining semantic limits are linked from [handoff-commentary-round02.md](handoff-commentary-round02.md). Corrected special-room semantics, exact BIT/LDA shared bytes, copied sound caller uncertainty and mapper-switch reset templates are retained. No structural map/state change or new toolkit issue. Next redirect only for concrete changed inference/new evidence; final editorial audit remains pending, and this is not whole-ROM classification or terminal-incomplete acceptance. No audit/archive execution, cleanup, publishing, commit, toolkit or ROM edits.

## Redirect round02 active

Attempt round02-a01 starts from pass-002-final and completed current-source commentary. Reassessing localized residual evidence and actual source questions; no map/state/source changes. Frontier: redirect/round02-a01/frontier.md. Classification remains131074/131088;14 unknown.

## Redirect round02 ready — pass-002-final unchanged

R01-L07 materially reopened/narrowed:12 Static genericData bytes BE28..BE2E/FE28..FE2E proposed,0 accepted. New third same-position occurrence13E28 is independently consumer/count-anchored graphics literal output138..143 of144, correcting the old unknown-source-only duplication premise. Exact evidence/reasoning and competing code interpretation: redirect/round02-a01/review.md. Remaining two orphan60 bytes1CC9C/1CDE5 retain ambiguity after JMP/RTS convention comparison; no forced100% or terminal claim. All identified promising connected/independent questions triaged; no interrupted productive frontier.

Current counts unchanged131074/131088 classified,26737Code104337Data14Unknown;12 proposed2 unproposed. Full exact ledger and proposal validation saved. Source-question inspection also found seven stale carried structural-gap comments; exact physical/source/reading anchors and hashes forwarded for next commentary, separate from honest semantic limits and pending final editorial audit. NPC checklist supersession appended without deleting historical outcomes. No unchanged gameplay replay, toolkit edits or map/state/source/ROM mutation. Input invariant hashes preserved. No new toolkit issues; existingNES-DOC-001 remains only finding. Handoff: handoff-redirect-round02.md.

## Disassembly round03 — active

Phase disassembly, round03; input pass-002-final, ready reopened R01-L07. Frontier: independent counted-graphics suffix review at BE28/FE28, exact generic-Data versus tiny-code discrimination; retain two orphan RTS alternatives. Reuse accepted map/source/gameplay evidence; no unchanged replay. Prior commentary/source and before preserved for next full commentary gate.

## Disassembly round03 — verified narrow suffix batch

ROM classification: 100.00% (131086 / 131088 bytes classified; 2 bytes unknown).

Incomplete despite rounding: Code26737, Data104349, Unknown2. Accepted12 Static generic Data bytes at BE28..BE2E and FE28..FE2E; independent counted graphics origin, literal-output endpoints, same-position copies and boundary/context review jointly distinguish the tiny-code alternative. No bank2/3 live consumer or original generator claim. R01-L07 verified; prior history retained. Residual1CC9C/1CDE5 remain competing redundant return versus padding/data; no forced classification.

Evidence: [independent review](disassembly/round03/review.md), [measured validation](disassembly/round03/independent-validation.json), [accepted regions](disassembly/round03/accepted-regions.json), [exact counts/provenance](disassembly/round03/summary.json). Provenance totals Static120740 / Observed10346. Header16 Data separate from banks below.

|16 KiB bank|Effective8 KiB pair|Code|Data|Unknown|
|---|---|---:|---:|---:|
|0|[0, 1]|12576|3808|0|
|1|[2, 3]|3|16381|0|
|2|[4, 5]|3|16381|0|
|3|[6, 7]|3|16381|0|
|4|[8, 9]|3|16381|0|
|5|[10, 11]|116|16268|0|
|6|[12, 13]|884|15500|0|
|7|[14, 15]|13149|3233|2|

State/current saved and validated; map/state SHA256 d60123b8f0dcffb8ab9cd1c972680b1e87509085ece9ae40282f705fd6196bdc. Fresh checkpoint pass-003-final verification linked in handoff. Existing source, symbols, reading and ROM hashes unchanged. Accepted full-map/source and representative gameplay/late-region/NPC/terminal-scene evidence reused; no unchanged runtime replay or natural reachability promotion.

Next phase full current-source commentary must retain all prior authored commentary, cover the12 new Data bytes, and close all seven exact stale-comment defects in redirect/round02-a01/editorial-followups.json. Current-source and final editorial gates pending. One existing toolkit documentation issue NES-DOC-001, no new toolkit issues found, no blockers; [report](toolkit-issues.md). No toolkit edits, input edits, archive execution, cleanup/deletion, publishing or commit. No terminal-exhaustion claim.

## Round03 commentary active — pass-003-final

Entire prior commentary preserved at round-03/commentary-round02, fresh verified baseline prepared. Exact physical-region carry retains valid review; new12 Data bytes plus all-source structural-gap dependencies are active. See ../commentary/progress.md and commentary/round03/frontier.json. Classification incomplete131086/131088;2 Unknown. Final editorial audit pending.

## Round03 commentary complete — pass-003-final

Full current-source readability complete; all10920 regions/2090 units reviewed,61 source dependency repairs and371 review-summary repairs, including all seven flagged defects. Entire prior commentary preserved at round-03/commentary-round02. Fresh byte-identical rebuild and refreshed reading copy pass; exact coverage/hashes in ../commentary/readability-round03.json. Classification remains incomplete131086/131088;2 Unknown. Next redirect assesses residual reasoning/independent possibilities without requiring a preexisting ready proposal. Final editorial audit pending; see handoff-commentary-round03.md.

## Redirect round03 — rejection review for coordinator

Input/output structural checkpoint pass-003-final unchanged;131086/131088 classified (rounded100.00%, explicitly INCOMPLETE),26737Code104349Data2Unknown. Attempt round03-a01 independently reassessed both singleton60 bytes1CC9C/1CDE5 using primary neighbors, balanced tail-call/shared-renderer contracts,13 accepted comparison joins, encoded entry/RTS-minus-one occurrences and local sprite consumer bounds. ApparentCC8C word18027 crosses music pointers; local CE8C+Y sprite reads exclude both residuals. Revised checks supply no positive role discriminator between redundant return and padding. No proposal/map change; R01-L07 residual history appended. Exact complement/views and compact rejection reasoning in redirect/round03-a01/unknown-ledger.json and review.md.

Other concrete source/uncertainty/extent/copy/dispatch and broad gameplay avenues reviewed with specific diminishing-prospect reasons; no unchanged replay, arbitrary sweep or interrupted productive frontier. No ready lead. Worker handoff INCOMPLETE pending coordinator primary-reasoning inspection; final editorial audit remains PENDING. ROM, configured binary, maps/state/source/symbols/reading preserved. No toolkit change, cleanup/deletion, archive execution, publishing or commit. See handoff-redirect-round03.md.

## Coordinator final acceptance — in progress

Round03 redirect stopping rationale independently accepted after primary byte, renderer/stack, aligned-pointer and rejection-scope inspection; see coordinator-rejection-review.md. This is justified terminal incomplete classification (131086/131088;2 Unknown), not a blocked research handoff. Final orchestration acceptance remains pending fresh official editorial audit and terminal document/invariant checks.

Map and internal state validation both returned ok=true. Initial final editorial audit failed on region-3360 lacking a per-region reviewed unit. Aggregate reviewed routine coverage was already complete, but 2952 Code scaffold slices lacked links to their existing reviewed units. Metadata links repaired without new logical units, source/scaffold/map changes or lost prior evidence; analysis/final-acceptance/audit-attempt-001.json, review-before-unit-links.json and unit-link-repairs.json preserve the diagnostic and repair. Distinct logical units remain2090 (972 Code); actual unit attachments now5042. Current source SHA45f15cb32e18ef6fb26a2c67dba0426589cdd81fbdb8983864367b404cd9cb67.

Official retry `python3 -B /root/romatlas/scripts/commentary.py audit .` is running as tool exec session53844. Poll that live handle before deciding it stopped; no restart from an observation timeout. No archive execution or cleanup is authorized/performed. Final terminal handoff, final document inventory and goal-completion audit still required.

## Coordinator terminal incomplete handoff — pass-003-final

Official final editorial audit retry passed: ok=true,byte_identical=true,editorial_audit=true,10920 reviewed regions/5042 unit attachments. These refer to2090 distinct logical units (972 Code); the2952 added attachments repair metadata links only. Source/symbol/reading hashes remain those of completed round03 commentary. The failed first attempt and repair evidence are retained; no ignored acceptance failure remains.

Coordinator independently accepted the final localized rejection review after decisive primary-byte/source/stack/pointer inspection. ROM classification: 100.00% (131086 / 131088 bytes classified; 2 bytes unknown). Rounded100.00% is INCOMPLETE. Residual1CC9C/1CDE5 code-versus-data ambiguity remains, with no ready lead or interrupted productive frontier. This workflow terminal handoff is not ROM classification completion or a blocked-success substitution.

Terminal result and limits: [terminal-handoff.md](terminal-handoff.md). Final acceptance report: [current-audit.json](current-audit.json). archive-options.json deliberately retains all additional final/historical Markdown; no disposable Markdown scratch omission is needed. Official audit process session53844 finished exit0; no live phase process remains. No archive execution, cleanup/deletion, publishing, commit or toolkit/input modification occurred. Goal accounting is completed only after the final requirement checks pass.

Final requirement check first returned workflow_terminal=false solely because R01-L01..L06 retained ready=true beside status=verified and their recorded accepted round02 results. Confirmed the complete89220-byte round02 acceptance, preserved the pre-repair lead file and failed requirement report, and cleared only those six stale ready flags. Each lead retains its prior ready result and an appended reconciliation/accepted-checkpoint record. No active proposal was discarded; R01-L07's two-byte unresolved subclaim remains separate. Evidence: final-acceptance/lead-ready-repairs.json and completion-attempt-001.json. Final requirement check rerun after this metadata repair; official source audit remains valid and unchanged.

## Final coordinator acceptance — workflow terminal incomplete

All19 requirement checks pass in analysis/current-audit.json. No ready lead remains after reconciling the six already-verified round02 flags; all prior outcomes and failed checks are preserved. Official final editorial/fidelity audit passes; maps/state/frozen checkpoint agree, ROM/source hashes remain current, exact two-byte complement and full current Markdown inventory verified. Terminal handoff: analysis/terminal-handoff.md. No remaining orchestration work or active phase frontier; no whole-ROM completion claim. Goal may now complete as the requested justified terminal-incomplete workflow, with2 Unknown bytes explicitly retained and no archive/cleanup/publishing action.

## Batch-only Manual fallback disassembly — pass-004-batch-fallback

ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).

Explicit user-authorized batch-only exception applied after independent current internal-map complement verification: physical [117916,117917) / [0x1CC9C,0x1CC9D), and [118245,118246) / [0x1CDE5,0x1CDE6), one byte each; two <=10 threshold. Both annotated generic Data of unknown meaning with Manual provenance using supported map.annotate. Prior redundant RTS versus padding/data ambiguity persists; this is not evidence-backed role inference. Code26737 / Data104351; Static120740 / Observed10346 / Manual2. Evidence-backed classified total remains131086; no full evidence-backed completion claim.

Header16 Data; bank0 Code12576/Data3808, banks1–4 each Code3/Data16381, bank5 Code116/Data16268, bank6 Code884/Data15500, bank7 Code13149/Data3235; all banks Unknown0. No new gameplay or redirect research; accepted prior role investigation reused within its limits. Full exact counts, authorization, annotations and preserved pre-change state/map/audit/terminal conclusions: [batch review](disassembly/batch-fallback-001/review.md), [summary](disassembly/batch-fallback-001/summary.json).

map.validate ok, state saved and current map exported; both SHA25601fbd61b4859c18d45fcf1fe542919df14e8be76ab42f69a03869e8bd0eb3742. Fresh pass-004-batch-fallback validates and rebuilds byte-identically: [verification](pass-004-batch-fallback-verification.json). Input ROM and prior commentary/source/symbol/reading unchanged. Full current-source commentary then final editorial/inventory audit pending; no commentary phase performed here. Existing unit-link repairs untouched. [Handoff](handoff-disassembly-batch-fallback-001.md).

Supported APIs: api.describe, map.load, map.annotate {region}, map.validate, map.save, map.export. Existing NES-DOC-001 only; no new toolkit issues found, no blocker; [report](toolkit-issues.md). No ROM/toolkit edits, archive/deletion/publish/commit. Prior historical records retained.

## Final batch coordinator acceptance — pass-004-batch-fallback

ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unclassified). Evidence-backed131086 plus user-authorized Manual generic Data2 at physical117916/118245, length1 each. The redundant RTS versus padding/data ambiguity and unknown meaning remain explicit. This is completion under the narrow batch exception, not a claim of full evidence-backed role recovery.

Sequential disassembly, full current-source commentary and final editorial review workers completed. Strict official audit finished exit0,ok=true,byte_identical=true,editorial_audit=true;10920 reviewed entries,5044 attachments of2090 logical units,15964 anchors and complete per-slice Code explanation coverage. Fresh checkpoint and commented-source verification pass; current source, reading copy, maps, state and frozen baseline agree. All28 coordinator requirement checks pass in current-audit.json with no unresolved blocker. Exact source/ROM hashes and review repairs are retained in final-acceptance-batch/final-review.md.

Final terminal conclusions: terminal-handoff.md. archive-options.json retains the final fallback record and every additional final/historical Markdown required by scripts/archive.py; no unrecognized Markdown and no disposable Markdown omission. No archive execution, cleanup/deletion, publication, commit, toolkit or ROM modification occurred. Earlier incomplete conclusions, audit attempts and source snapshots remain preserved. No live phase/audit process remains; official session76362 completed.
