# Disassembly progress

Workspace initialized. Baseline and classification have not run.

## Round 01 disassembly — active
Baseline established: maps/baseline.json and generated/baseline reproduce the ROM byte-identically; analysis/baseline-verification.json records hashes. ROM identified visually as Archon (Activision/Bullet-Proof, 1989), mapper 2, 128 KiB PRG and CHR RAM. Reset $C008 -> physical $1C018; NMI/IRQ $FB12 -> $1FB22. Initial bounded vector discovery found 102 instruction candidates; immutable results in analysis/round01-static-summary.json and round01-static-rpc.jsonl. Candidates are not yet accepted annotations.

Active emulator script analysis/round01_driver.py, PTY session 33769, fresh server with startup snapshot imported; log analysis/round01-rpc.jsonl. Startup snapshot analysis/round01-startup.snapshot persists 44 natural frames, context [2,3,14,15]. The first Start advances copyright splash to title, not yet board gameplay. Bounded 120–180 frame batches can take tens of seconds; timeouts preserve reached state and each scenario exports a snapshot and PPM. Reassert all buttons after imports. Earlier exploratory server session 20129 can be ignored; no persisted map changes there. No toolkit changes.

Update: initial driver finished at 699 attract-mode frames; individual button parameters were silently ignored by NES input adapter. This is NES-001 in toolkit-issues.md. Corrected input uses replacement buttons array; actual-start (Start 2 frames), actual-menu (release 30), oneplayer (A 2), options (release 30) now reach menu and selection naturally. Live interactive process is Python PTY 19451 running analysis/round01_session.py. The rpc helper logs every request/response durably in round01-session-rpc.jsonl; use complete buttons arrays. Snapshot r01-options.snapshot will be the newest milestone once complete. Static non-code inspection has local tile contact sheets r01-bank1-tiles.ppm through bank6 and quantitative round01-data-inspection.json; bank1 sample renders readable alphabet and icons. Planned classification accepts separately supported code discovery and generic non-code where exact semantic subdivisions remain unresolved.

Current checkpoint (later): active session remains 19451, corrected array inputs verified. Completed route title -> 1 player -> light side -> light first. Board draw is progressing (r01-boardloaded.ppm at frame996 shows board and pieces being introduced). Current latest clean snapshot r01-boardloaded.snapshot. Session evidence now includes 1930 distinct fetched ROM offsets in round01-coverage.json, and 2142 effective execution contexts in round01-contexts.json. Initial annotation batch was safely rejected for missing required id field; state remains empty (no classification loss). Corrected acceptance uses id:0 and will rerun without replaying experiments. Rejection schema log preserved as round01-annotation-schema-rejections.json. No toolkit code changed. The printed discovery execution_coverage flag is false even for known fetched roots, so acceptance derives Observed status from saved actual instruction-fetch coverage plus context roots, independently of this flag.

## Verified intermediate batch
ROM classification: 78.45% (102837 / 131088 bytes classified; 28251 bytes unknown).
Accepted 2206 bank-aware instructions (4503 bytes); all 6 large non-code bitmap banks (98304 bytes); header/vector/selector records (30 bytes). Internal map validates. New checkpoint maps/pass-001-intermediate.json -> generated/pass-001-intermediate, verified byte-identical (analysis/pass-001-intermediate-verification.json). Readability/editing remains subsequent commentary; classification incomplete.

Gameplay board piece introduction is lengthy; after natural menu setup, bounded untraced fast-forward batches visit continuing piece introduction (snapshots r01-boardwait and r01-boardready). Untraced steps establish state inspection only, not fetch coverage; trace resumed between batches. No RAM edits yet. Further static helper inspection found fixed inline consumers F862 (5-byte VRAM-buffer parameters), F8A5 (6-byte direct PPU upload descriptor), F9B6 (5-byte memcpy parameters). Each copies bytes from stacked return address and adjusts return by exact count. This supports precise inline Data barriers and continuation roots, without demanding natural callers. Connected enrichment prepared in round01_inline.py. The current latest pending transition is readyfinal, active process19451. No terminal/exhaustion claim made.

Current progression frontier: board introduction completed naturally at frame2394 (r01-introend.snapshot), active board-selection loop return at D3E3. Local RAM inspection and source CFD2 prove board lookup row=$61, column=$63 via row pointers0591/059A. Previous positions60/62. Initial cursor columnFD was rejected by selection guard; saved r01-before-shortcuts.snapshot at2448. First artificial shortcut patches60..63=[4,4,1,1] and0615=0 (startup selection delay), then A3frames+release20. Actual knight selection graphics appear and bank switches[8,9,14,15], observed only in artificial state. Clean selected snapshot r01-knight-selected.snapshot. Arena experiment adds dark knight14 to row4col2 (RAM0472), patches61=4,63=2, A3+release45; remains board. Repeated A2+release40 after clearing0614 debounce still remains board. Final bounded selection-event test writes05ED=80 because D3E4 loop waits sign flag, then80frames; pending. These are explicit artificial hypotheses, not natural reachability claims. Do not repeat unchanged failed selector experiment. Source evidence r01-decode-d3c0/cb01/cfd2.json.

Artificial arena milestone verified: setting05ED=80 after the supported source/target patches entered a distinct combat arena (r01-force-selection.ppm). Fighter rendering confirmed at r01-arena-b.ppm and r01-arena-combat.ppm; exercised A+right60, B+up30, A+right60 frames. This provides actual fetch evidence in an artificial battle scenario; no natural battlefield route claim. Current-context audit round01-contexts-current.json contains fetches only in physical PRG banks0,1,14,15; mapping[8,9,14,15] was graphics loading, not execution in data banks. No classified graphics-region instruction-fetch conflict.

One practical terminal shortcut attempted: restore selected-knight snapshot, replace every dark board cell18..127 with80, set row4,col2 target and05ED=80; exact before/after in round01-ending-patch.json. First120frame budget timed out with movement animation still visible; one bounded80frame settle queued. Then a separate wizard/spell attempt restores clean pre-shortcut snapshot and selects row4,col0 wizard, avoiding contamination. Handoff will record unsuccessful transitions rather than extending unchanged experiments indefinitely.

## Round 01 disassembly pass complete — incomplete ROM

ROM classification: 83.57% (109551 / 131088 bytes classified; 21537 bytes unknown).

Exactly one disassembly pass completed. Physical unique-byte totals: 10739 Code, 98812 Data (including 16-byte header), 21537 unknown. All 5147 classified regions have structural evidence; 5116 Code regions have explicit valid CPU views. Labels/editorial commentary are not complete. Unknown complement is in round01-metrics.json; no raw-export fallback counted.

| Physical 8KiB PRG bank | Code | Data | Unknown |
|---|---:|---:|---:|
| 0 | 2983 | 0 | 5209 |
| 1 | 1516 | 206 | 6470 |
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
| 12 | 0 | 8192 | 0 |
| 13 | 0 | 8192 | 0 |
| 14 | 3454 | 136 | 4602 |
| 15 | 2786 | 150 | 5256 |

Header:16 Data bytes, counted separately from banks. Internal map validates. Final checkpoint [maps/pass-001-final.json](../maps/pass-001-final.json) generates [generated/pass-001-final](../generated/pass-001-final) and verifies byte-identically; [pass-001-final-verification.json](pass-001-final-verification.json) records unchanged ROM identity and map SHA256. Prior intermediate checkpoints preserved.

Execution coverage is separate:4270 distinct actual ROM instruction starts and8878 unique instruction bytes across composite natural and explicit artificial scenarios. Snapshot branches have nonmonotonic cycle histories and are not one forward timeline. Interior alternate entry physical1F684 (TXA) overlaps BIT at1F683; both actual fetch paths retained in map comment/evidence. No instruction fetch conflicts with Data (round01-fetch-conflict-audit.json). Context changes into graphics banks were not mistaken for execution there.

Progression: natural title/menu/light setup and strategy board visited. Artificial knight selection/adjacent-enemy encounter successfully entered arena; fighters and bounded movement/fire inputs observed. Enemy-removal terminal shortcut did not show a win/ending; wizard same-cell and B attempts did not show spells. No unsupported terminal or natural-arena claim. Scenarios and exact source-grounded RAM recipes persisted in scenarios.json, gameplay.md, ending-patch and RPC logs. Later terminal/spell predicates are ready redirect research, not repeated gameplay waiting. Alternate-side/demo choices remain deferred after representative board/arena and terminal attempts.

Lead outcomes: D01-VECTORS, D01-BITMAPS, D01-INLINE, D01-BOARD verified; D01-END and D01-SPELL blocked on specific transition contracts; D01-STACK-HELPERS and D01-GRAPHICS-FORMATS ready. This is a productive pass handoff, not terminal investigation exhaustion. Commentary should review full current source, then redirect should investigate connected helper/data and gameplay-state contracts.

Toolkit findings:1 confirmed documentation/API contract bug (NES-001),0 suspected bugs,0 missing features; no blocker. See [toolkit-issues.md](toolkit-issues.md). Toolkit unchanged; workaround complete buttons array confirmed. Input ROM hash unchanged; nothing published, archived or deleted.

Durable resume: start configured binary via run.py serve, map.load analysis/state.json; emulator.start then import desired local snapshot and reassert buttons array. r01-before-shortcuts.snapshot is natural board frontier; r01-knight-selected.snapshot and r01-arena-visited.snapshot are artificial. Named snapshots are not used as durable state. Old session IDs are historical; final maps and snapshot files are authoritative.

## Round01 commentary active
Disassembly handoff accepted after inspecting final verification and map identity; user shown exact 83.57% classification line. Input maps/pass-001-final.json and analysis/state.json. Prepared frozen commentary baseline with configured toolkit; initial rebuild byte-identical. Reviewing full current classified source, with no final audit. Archive selection contract inspected by coordinator; final options deferred until terminal artifacts exist. Current review frontier: bank0 combat initialization, random arena obstacles, projectile and actor updates through physical 0x112F inspected; edits and remaining bank0 through 0x4010 plus fixed bank 0x1C010..0x20010 and six bitmap banks pending. Current phase remains commentary.

Round01 commentary durable source batch: all5116 classified instructions inspected through physical0x1FEE8; all31 Data evidence records checked. Applied218 named routine/block explanations,666 descriptive routine/branch/call labels and265 RAM/hardware/table aliases across full source. All180 unknown spans have local boundary/role notes, including known inline-contract hypotheses without reclassification. Scaffold preserved with review summaries/source locations; meaningful unit inventory added. Verification in progress; final source correctness/readability cross-check and refreshed reading copy still required. Current phase commentary; no final audit.

## Round01 commentary complete
Full current classified-source readability completed, not just a repair subset: all5116 Code regions/10739 bytes and31 Data regions/98812 bytes reviewed; all180 Unknown spans/21537 bytes carry honest local boundary/role notes.218 routine/block explanations,666 descriptive routine/branch/call labels and280 RAM/hardware/table aliases applied.5327-record physical scaffold preserved;249 meaningful routine/Data units and per-region source locations recorded. No classified source readability ranges remain deferred.

Final fresh commentary verification passed byte-identically with unchanged input hash. Source SHA2561c1a9ecb022eac6839a608c3b801e79d9ee27c503f9f0f0d7733621c51c3efe1; reading-after.asm refreshed. Frozen baseline, before/, phase1 state/maps and historical evidence unchanged. Integrity record:round01-commentary-integrity.json. Handoff:round01-commentary-handoff.md; concrete redirect questions:round01-commentary-questions.md. Classification unchanged83.57% (109551/131088 bytes); current-source readability complete does not establish whole-ROM completion.

Consistency correction before handoff: initial commentary draft reversed board coordinates. Primary BoardCellToActorPixels accepts A=$63 for X/column and Y=$61 for Y/row; ReadBoardCell selects a stride9 row pointer with Y and indexes the column with A. Final aliases/prose correctly use $61=row,$63=column,$6B=candidate row,$6C=candidate column. Earlier disassembly convention was correct. Also distinguished empty power-square fallback from sole ownership of all five squares. A numeric array-offset edit briefly caused a rebuild mismatch; it was corrected and final verification passed.

Remaining uncertainties are semantic/structural Unknown gaps and asset subdivisions, documented locally. Redirect should investigate connected helper/record contracts, turn-boundary terminal predicates and special-action dispatch. Final editorial acceptance/audit remains pending; no audit, archive, publication, cleanup, deletion or toolkit change performed. All source/assets/state remain local. Ready for sequential redirect, not terminal exhaustion.

## Round01 redirect active — r01-contracts
Accepted current-source commentary gate (hash1c1a9ecb022eac6839a608c3b801e79d9ee27c503f9f0f0d7733621c51c3efe1). Unique research attempt analysis/redirect/r01-contracts, input maps/pass-001-final.json / analysis/state.json, classification unchanged109551,21537unknown. Exact frontier: additional stack-inline contracts/sites then supported connected continuations; next independent opportunities terminal turn boundary, special-action dispatch and graphics consumer bounds. Prior failures preserved; no arbitrary byte/avenue quota, no terminal assessment yet. Toolkit NES-001 remains known nonblocking; configured release only.

Redirect checkpoint01: analysis/redirect/r01-contracts/checkpoint-01.md. Productive connected graph expanded all seven computer-special + seven human-spell + seven effect handlers and eight selection handlers;9787 candidate instruction bytes and334 new inline Data bytes (not accepted/classified). Exact inline barriers have zero start conflicts and graph zero illegal stops after confirming F44F alternate4-byte entry. Revised terminal shortcut now visibly reaches 'THE GAME IS ENDED...' (terminal-2.png), after clearing dark PieceStrength as well as board and entering D183 turn boundary. Artificial only. Remaining work wizard mechanics, restart, graphics bounds, further concrete indirect dispatch opportunities and residual ledger; current phase redirect, map/source unchanged.

Redirect checkpoint02: wizard menu and teleport target-selection now visited via source-grounded artificial D510 consumer contract (wizard-revised.py), with saved fetch/context evidence and visible SELECT A SPELL / TELEPORT then WHICH ICON WILL TELEPORT. Terminal advanced to THE LIGHT SIDE WINS after bounded A+Start/release. Connected independent audio9 handlers and AI3 move handlers traced; final graph has10057 truly unknown candidate code bytes plus334inline bytes, zero illegal stops. Full details checkpoint-02.md; closing practical teleport interaction, graphics bounds and non-code data proposals before handoff. Same phase redirect.

Redirect latest: independent static routine contracts (independent-contracts.md) resolve original unexplained handlers without imposing callers as universal prerequisites; deliberate nontrivial RAM/hardware control-flow and accepted terrain-oscillator clone support these separately from reachability. Contracts graph candidate unknown11793Code+374inlineData;40 non-code pattern/consumer envelopes inspected and8205Data bytes proposed pending final exact boundary checks. Graphics finite actor domain refined0..19 via40-ID table and RNG&3+24elementalproducer;20exact0300byte chunks withinalreadyData banks4..6 (assets.md). Teleport source/destination05ED-only event edits did notcommit, mode86 persists; preserve these negative results, improve only through controller+debounce/source-coordinate writer contract. Terminal/wizardmenu milestones remainpositive. Currentphase redirect; no map/source changes.

## Round01 redirect complete — ready combined batch, incomplete ROM
Attempt analysis/redirect/r01-contracts completed after connected and independent structural avenues, exact graphics domain and representative terminal/spell mechanics. Current accepted state unchanged109551classified/21537Unknown,83.57%; currentmapvalid, state/checkpointSHA6998ac7003ed7a35d6a40f07209c20b127313fdbcd9afc2fdd4d54174a2e2bd5, commentaryhash unchanged1c1a9ecb022eac6839a608c3b801e79d9ee27c503f9f0f0d7733621c51c3efe1.

Authoritative review-proposals.json:5803Code regions12383bytes+96inlineData380bytes+47otherData8764bytes=21527uniqueunknownbytes proposed only. Potential10Unknown ifallaccepted; no structuralmapmutations inredirect. Zero internalinstruction/categoryoverlap, illegalgraphstops orData conflictswithnewartificialfetchstarts. Full exact180-rangeunknowncomplementledgerwithbankCPU/neighbors/evidence/dispositions saved. Independent structural routines have explicitcontracts (notjustvaliddecode); isolatedRTS/JMP/zerosremainunresolved. No universalcaller or naturallyproducedselector requirement imposed.

Gameplay now verifies complete artificialteleport: controllerA+0614/15zero+064A/Bsource/targetcoord writer alignment advancesmode86->81, accepts sourcepiece6row3col0, moves into animation then commitsboardold80/new06row4col2 after100boundedsettleframes. Screenshotteleport-complete-1.png and board/context/fetchrecords retained. Terminalartificialgameended->THELIGHTSIDEWINS verifiedseparately. Newscenariofetchunion1361starts,399candidateCodestarts,0Data conflicts; notnaturalprogressionclaim. scenarios.json preservespriorattemptshistory andnew successfulrecipes.

Handoff analysis/round01-redirect-handoff.md; result analysis/redirect/r01-contracts/result.md; leads persistentpass/updatesrecorded. All originalproductiveavenues investigated andcombined, no firstleadstop. Other spell/gameplaypermutations reducedprospectforisolatedresiduals aftercompletehandlerinspection, no exhaustiveclaim. Nextphase disassembly independentproposalreview thenfullcurrentcommentary/redirect. Finaleditorialauditpending. Readybatchhandoff isnot terminalexhaustion, wholeROMcompletionorarchiveacceptance. Toolkit:no newissuesfound; priorNES-001nonblocking; ROM/toolkit/map/source/history unchanged; no publish/archive/cleanup/deletion.

Coordinator review guide: analysis/round01-redirect-review-guide.md gives the final batch in readable prose, preserving the detailed compact handoff unchanged. Research is complete; next action is independent disassembly review of review-proposals.json. No additional runtime test is pending.

Final handoff readability pass completed: analysis/round01-redirect-handoff.md and analysis/redirect/r01-contracts/result.md now use ordinary spaced prose and clear count distinctions. Research evidence, proposal counts and decisions are unchanged. Ready for independent disassembly review.

## Round02 disassembly active
Input state/checkpoint: analysis/state.json and maps/pass-001-final.json. Independently reviewing the authoritative redirect/r01-contracts/review-proposals.json against immutable ROM, inline stack-return contracts, bank-aware successor graph, independent routine contracts and bounded Data consumer/pattern evidence. Existing commentary remains unchanged for subsequent full-source migration. No classification has yet changed; fresh checkpoint and exact accounting will follow supported annotations. No toolkit changes or gameplay replay planned while saved evidence suffices.

Round02 review checkpoint: all 9,295 proposed execution views were freshly decoded and physically bank-mapped; 5,803 proposed Code regions / 12,383 bytes and all proposal intervals have no decode or overlap errors (disassembly/r02-review/mechanical-summary.json). Direct helper-window review confirms stacked return advances 2/3/4/5/6 and F44F shared entry. Local wording corrections are required: A6FB polls for nonzero input; A33D writes 960 tiles; sprite sequences alternate seven/eight bytes; physical 03CA4..03CAC is descriptor-like generic Data rather than a demonstrated third APU register set. Structural decision remains pending final connected-edge and consumer review. Original commentary untouched.

Round02 acceptance frontier: independent semantic/consumer review now supports the full connected 21,527-byte batch with localized explanation corrections, including a separate eight-byte audio script descriptor block. Edge audit checked 14,644 decoded parent/inline edges with zero exceptions. Fresh annotations are running through map.annotate; twenty existing $0300 actor prefix windows are refined using C317 alone (C381 instead copies $0800). Exact findings and reproducible review scripts: disassembly/r02-review/findings.md. Ten isolated stub/zero bytes remain unresolved; no terminal exhaustion claim. Fresh validation/checkpoint pending; commentary source remains untouched.

## Round02 disassembly complete — incomplete ROM

ROM classification: 99.99% (131078 / 131088 bytes classified; 10 bytes unknown).

Independently reviewed and accepted 21,527 formerly Unknown bytes: 12,383 Code, 380 stack-inline Data and 8,764 other Data. The accepted map now has 23,122 Code bytes and 107,956 Data bytes, including the 16-byte header. All 11,131 regions carry evidence; all Code has explicit validated execution views. Whole-ROM classification remains incomplete because ten bytes in seven exact intervals remain Unknown. No raw export fallback counted.

Fresh review decoded 9,295 proposal views and verified physical/effective banks, all intervals and 14,644 numeric predecessor edges; zero conflicts. 171 call/context checks establish 96 new inline records and exact return widths. Independent hardware/RAM contracts and finite dispatch consumers support static routines separately from natural reachability. Full Data byte/pattern/consumer inspection supports generic envelopes where precise schemas remain unresolved. Findings and localized wording corrections: [findings.md](disassembly/r02-review/findings.md). The APU suffix was split into its correct eight-byte script descriptor; sprite lengths, 960-byte clear, input predicate and DC69 domain claims were narrowed. Twenty already-classified 768-byte actor graphics prefixes were refined using C317 only; C381 independently copies 2048 bytes. No byte-role proposal required rejection.

| Bank | Code | Data | Unknown |
|---|---:|---:|---:|
| 0 | 5795 | 2394 | 3 |
| 1 | 3552 | 4639 | 1 |
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
| 12 | 0 | 8192 | 0 |
| 13 | 0 | 8192 | 0 |
| 14 | 6698 | 1492 | 2 |
| 15 | 7077 | 1111 | 4 |

Header:16 Data bytes, outside the 8 KiB banks. Exact residuals and physical/CPU identities: [metrics.json](disassembly/r02-review/metrics.json). Unknown intervals (ends exclusive): $00ABF..$00AC1, $00E87, $02481, $1DC0F, $1DC23, $1F08D..$1F090 and $1F9F7. Isolated return/JMP/zero content lacks a supported entry or non-code role; adjacent accepted structure does not settle it. No dead-code or terminal exhaustion claim.

Saved scenarios were reused: representative natural board plus explicitly artificial arena, ending/winner and completed Teleport already satisfy practical progression review; no unchanged gameplay repeated. Composite saved execution is 4,686 distinct ROM instruction starts / 9,753 instruction bytes (execution-metrics.json), separate from structural classification and natural reachability. New annotations remain Static.

State saved to analysis/state.json, current map exported to maps/current.json and validated; fresh pass-002-final generated source and byte-identical verification passed. [Verification](pass-002-final-verification.json), [integrity](disassembly/r02-review/integrity.json). Input hash unchanged. Existing commentary SHA256 remains 1c1a9ecb022eac6839a608c3b801e79d9ee27c503f9f0f0d7733621c51c3efe1; full current-source commentary migration is the next phase, followed by redirect. No final editorial audit performed.

Lead outcomes: D01-STACK-HELPERS and D01-GRAPHICS-FORMATS verified by this review; D01-END and D01-SPELL preserve their successful artificial scenario evidence. Residual role questions remain localized for the next commentary/redirect review; no unsupported inference added.

Toolkit findings: no new toolkit issues found; existing NES-001 is one confirmed documentation/API contract bug, nonblocking with complete buttons arrays. Zero suspected bugs or missing features. See [toolkit-issues.md](toolkit-issues.md). Empty labels in proposal input were normalized to null after an expected schema rejection; successful annotate transactions were recovered into a validated local checkpoint before continuation. This was input normalization, not a toolkit defect. Toolkit unchanged; no publication, archive, cleanup or deletion. Free space about 8 GiB; all cited evidence retained.

## Round02 redirect started — r02-residual-review
Current-source readability gate accepted; reviewing seven exact residual intervals (10 bytes), corrected cancellation mechanics and computer distance-table naming. No classification changes. Plan: analysis/redirect/r02-residual-review/plan.md. Historical evidence retained; configured binary/ROM unchanged.

Round02 redirect checkpoint01: primary consumer proof supports physical $00ABF..$00AC1 as two-byte Static Data (proposal only). Fresh decode and exact operand/branch/word checks saved in analysis/redirect/r02-residual-review; checkpoint-01.md records changed rejection premise, exact selector and current frontier. Continuing remaining eight-byte alternatives plus independent source/editorial leads; no first-lead phase stop. No map/source/ROM edits.

## Round02 redirect complete — ready combined review batch
Accepted checkpoint unchanged131078/131088 (10unknown). D02-PROJECTILE-Y-ADJUST proposes2StaticData bytes via exact accepted bounded indexed consumer; D02-DISTANCE-CONSISTENCY supplies existing60byte map label/evidence correction plus precise source editorial repairs (0byte gain). Other6intervals/8bytes retain localized ambiguity with primary-artifact rejection review, not caller-as-universal-rule. Exact ledger, fresh configured-binary decodes, reference hypotheses, clone comparison, per-bank counts and integrity hashes saved in analysis/redirect/r02-residual-review. Leads/pass history appended; handoff analysis/round02-redirect-handoff.md. Next independent disassembly then full commentary/redirect; final editorial audit pending. No new toolkit issues; existing NES-001 nonblocking. No map/state/ROM/source/toolkit edits; historical evidence retained. Not terminal exhaustion and not whole-ROM completion.

## Round03 disassembly complete — incomplete ROM

ROM classification: 99.99% (131080 / 131088 bytes classified; 8 bytes unknown).

Accepted 2 new Static Data bytes at $00ABF..$00AC1, ProjectileOriginYAdjustmentByDirection, through exact sign-selected consumer/boundary review. 23122 Code / 107958 Data; header16 Data included. Corrected region11047 label AND stale evidence to ComputerArenaDistanceParameters, preserving bounds and original evidence history (0 byte gain). $0678 derives from $067A+3. [Independent findings](disassembly/r03-review/findings.md), [metrics](disassembly/r03-review/metrics.json).

| Bank | Code | Data | Unknown |
|---|---:|---:|---:|
| 0 | 5795 | 2396 | 1 |
| 1 | 3552 | 4639 | 1 |
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
| 12 | 0 | 8192 | 0 |
| 13 | 0 | 8192 | 0 |
| 14 | 6698 | 1492 | 2 |
| 15 | 7077 | 1111 | 4 |

Residual ends-exclusive intervals: $00E87..$00E88,$02481..$02482,$1DC0F..$1DC10,$1DC23..$1DC24,$1F08D..$1F090,$1F9F7..$1F9F8. Primary evidence and combined-constraint review retain material Code/Data ambiguity, without a universal live-caller prerequisite. Previous rejection records preserved; no speculative classification, terminal exhaustion or completion claim.

Saved board/arena/ending/Teleport evidence and progression checklist reused. No unchanged gameplay repeated; no new execution observation claimed. State saved and map exported/validated; fresh [pass-003-final verification](pass-003-final-verification.json) confirms byte identity. An unchanged pass-003 checkpoint created after an input-schema rejection is retained as history; final is authoritative. ROM and existing commentary hashes unchanged: [integrity](disassembly/r03-review/integrity.json).

D02-PROJECTILE-Y-ADJUST verified; D02-DISTANCE-CONSISTENCY map work verified, with four stale scope comments and new table/count prose deferred to next full commentary phase. Existing commentary deliberately unmodified. No new toolkit issues found; existing NES-001 nonblocking via complete buttons arrays, see [toolkit-issues.md](toolkit-issues.md). No toolkit edits, publication, archive, cleanup or deletion. About8GiB free. Next full current-source commentary then redirect; final editorial audit pending.

## Round03 redirect complete — proposed terminal incomplete
Unique attempt analysis/redirect/r03-terminal-review retrieved decisive primary endpoint/clone/control/stack artifacts and checked corrected current source. Exact map complement refreshed: six intervals/eight bytes,131080/131088 classified. Prior localized ambiguity reasoning remains sound; zero proposals/no ready leads, no changed-method runtime test justified. Compact rejection-review.md links decisive artifacts for mandatory coordinator scrutiny; handoff analysis/round03-redirect-handoff.md. leads.json records empty result. No caller-as-universal-prerequisite, speculative classification, empty workflow cycle or whole-ROM success claim. Final editorial audit pending. Existing broad gameplay evidence reused. No new toolkit issues (NES-001 nonblocking), no map/state/source/ROM/configured binary edits, historical artifacts retained. Disk7.9GiB free; no cleanup/deletion/archive/publication.

## Coordinator final acceptance active
Round03 commentary completed with full carried/current review and refreshed reading copy; round03 redirect completed with zero ready leads. Coordinator inspected decisive primary residual decodes and clone comparison and accepted justified terminal incomplete structural reasoning in coordinator-rejection-review.md. This is not whole-ROM completion. Final targeted editorial assessment recorded in commentary/editorial-audit.md and review.json. Store and portable map freshly validate. Final commentary audit is running in exec session 72329; poll this exact handle rather than restarting on an observation timeout. Final completion verifier prepared at analysis/verify-final-handoff.py. Final terminal handoff, refreshed document reconciliation and completion audit remain pending; no archive/cleanup authorized.

## Documented terminal handoff — justified incomplete
Final commentary audit completed successfully: 11138 reviewed records, byte-identical source and refreshed reading copy; no pending editorial scope. Exact source hash f4769b7dc065c06d4a0ed46bf5b13ec72e9c0b8c4d4a53980f81ab2d894fc9fa. Final state/checkpoint pass-003-final; ROM classification: 99.99% (131080 / 131088 bytes classified; 8 bytes unknown). Coordinator primary-artifact rejection review accepted; no ready lead or useful revised experiment remains. Terminal handoff: terminal-handoff.md. Workflow endpoint is incomplete ROM analysis, not classification success. Document selection and final invariant proof are recorded in archive-options.json and final-verification.json. No archive, publish, toolkit modification or cleanup.
