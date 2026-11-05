# Disassembly progress

Round 1 disassembly active. Verified baseline: [report](baseline-verification.json), byte-identical rebuild, ROM SHA-256 unchanged. Initialized mapper 1 (16 physical 8 KiB PRG banks, 16 CHR banks), total 262160 bytes.

Runner: analysis/round1/driver.py. First 180-frame traced request timed out at 30 seconds; server exited with runner. No successful gameplay snapshot yet. Resume with smaller frame batches and save state at each batch. No classifications accepted yet.

Update: first bounded gameplay batch succeeded through frame 122. Actual frame identifies The Adventures of Rad Gravity. Evidence in analysis/round1/{title,start,intro,play,play2}-state.json and .ppm; title-only route so far. Trace summary: 1100 unique physical instruction starts, 1166 contextual roots, aggregate_dropped=0; 9348734 dropped detail events with capacity=0 are deliberate, not missing aggregates. No snapshot from that exited server. Current live console: analysis/round1/console.py, exec session 61445; collecting 150-frame title snapshot, then supported Start/Continue menus. Manual and password hypotheses saved in analysis/gameplay.md.

Static/runtime batch selection: 2694 PRG code bytes accepted in 112 contiguous regions, no candidate conflicts. Header 16, CHR 131072 and vectors 6 bytes accepted by cartridge layout. Intermediate internal classified count 133788; saving/export after next live batch. Durable title.snapshot and selection.snapshot exist; source session remains 61445. Frame 214 still shows stars during title transition, so next experiment tests A/Start menu progression rather than assuming gameplay. Structural evidence and reachability remain separate.

Further progression: Start/Continue menu actually observed in start-again.ppm at frame 342; previous stars were an animated transition rather than blockage. Held Start 30 frames + release 60 advances animation; A +90 queued to enter game. Console remains live. New static candidate: bank 2/3 contains a long smooth byte-amplitude sequence around $80 ending in zero tail (bounded hex inspection); inspect consumer before claiming audio semantics, generic non-code classification remains plausible.

Correction to gameplay interpretation (primary source check): NES emulator.input requires buttons array, not generic protocol button/pressed keys. All prior action recipes silently cleared input, so title/selection frames are attract mode and do not prove accepted button transitions. NES-001 records this confirmed documentation/API mismatch. Fixed console helper uses {port:1,buttons:[...]}; actual-menu batch pending. Earlier observed opcode evidence remains valid natural no-input execution, but routes are corrected to idle attract mode. No prior phase handoff exists.

Correct input schema immediately reached ship bridge: actual-menu.ppm shows "HELLO RAD GRAVITY" on bridge monitor (natural route from idle attract state: Start 2 frames, release 2, wait60). Durable actual-menu.snapshot saved. Trace was disabled during this transition, so it establishes visited state visually, not opcode coverage for initialization. Re-enabled capture before A dialog advance.

Bridge-end batch saved (snapshot/context/coverage/frame/RAM). Connected closure adds 3311 code bytes, total code6005, no conflicts. Classification total137099 so far; unknown125061. Bridge dialog finished and teleport/transition particle animation observed; map-route batch queued. Inputs now actual arrays; natural progression evidence remains separated from manual guide hypotheses. Static PCM consumer has confirmed MMC1 serial write helper at $C73F storing A to $FFFF then successive shifts, so A=1 selects effective banks2/3.

Ship planetary navigation visited: cyberia-entry.ppm shows completed rotating planet view, character at console. Route from bridge-end used natural A plus wait; capture resumes after untraced transition. Landing A pending. Static data batch executing: bounded PCM slice plus inspected smooth residual and zero tail, exact uniform runs after conflict check; see analysis/round1/data_static.py and uniform-inspection.json. These are Static annotations, not observed sample reachability.

Static data batch accepted: Image131072, Data20499, Code6005 =>157576 classified bytes (60.11%). PCM consumer decode and maximal-run evidence saved. Round1 directory ~113 MiB including durable snapshots, bounded aggregate-only traces and capped discovery pages; retained all scratch as instructed. Cyberia landing includes a long character walk/teleport animation; cyberia-after-info still ship bridge with communicator message, so no false planet visit claim yet. Distinct late-game shortcuts remain pending password-buffer contract.

Cyberia visited naturally: planet-ready.ppm shows industrial platform area and enemies after final idle240 landing transition. Now exercising right+B30, right+A30, release and Start pause/inventory, with tracing enabled. This corrects previous "landing attempted" status; actual planet visit is confirmed visually and through fetched instructions. Next alternative: restore known Start/Continue menu snapshot and inspect password buffer (bounded later-stage shortcut work).

Cyberia-play closure accepted 5312 additional code bytes (no overlaps), plus prior mechanics690: code12007 total, data20499/image131072 =>163578 bytes classified. Screenshot shows changed HUD inventory icons after Start; movement/jump/attack route executed. Durable cyberia-play.snapshot retained. Password-screen experiment restoring start-again.snapshot (visible Start/Continue menu), Down/A with correct arrays, trace off through menu transition then five traced frames.

Password shortcut contract confirmed: on Continue screen, selecting initial0 writes RAM$0410=0 and increments$8D; selecting1 writes$0411=1. Handler physical$19663 stores selected index to$0410,X; Start validation checks all20 indices (<$20), transforms into$0432..$0443 and validates checksum. Alphabet exactly "0123456789BCDFGHJKLMNPQRSTVWXZ!$". Patched20 bytes at$0410 with Turvia password indices, pressed Start, waited180 (trace off except transition start/end). turvia-password.ppm shows a different green planet on ship console, so accepted password/state transition confirmed. Artificial=true in scenario; no natural reachability claim. A+420 landing wait queued, then traced movement/jump/attack30.

Turvia A+420 wait hit core-tick max_operations10,000,000 before420 frames, with partial state preserved at frame925 (turvia-budget-state.json, picture). No crash; still bridge walkout. step helper now splits requests into<=120-frame chunks, each bounded10M/300s, to avoid losing continuation after aggregate request exhaustion. Continue90 and inspect before claiming landing. No toolkit defect: documented operation budget limit.

Turvia confirmed visited: turvia-action.ppm shows upside-down player hanging beneath terrain with full late-game equipment/health. This is actual artificial-password gameplay, not merely console. Initial Turvia closure adds2452 code bytes, no conflicts. Representative movement/jump/weapon30 executed; additional opposite-direction action queued after settling. Next reuses password-screen.snapshot (pre-edit) for asteroid and final boss.

Turvia second action added60 code bytes only, then shifted to distinct asteroid scenario (no repetitive play). Current Code14519, Data20499, Image131072 =>166090 classified. Live late_route helper restores password-screen.snapshot before each patch, explicitly clears buttons, patches exactly$0410..$0423, presses Start, waits180, A, waits660 in <=120-frame/10M tick bounded chunks, captures5 then action30. Password state validation is source-backed and external spellings O normalized to displayed0. Asteroid route ongoing; final boss route next. Seven static direct-JSR targets saved in static-call-roots/support.json for bounded verification after gameplay; no fallthrough bank-preservation assumption.

Asteroid shortcut accepted, but actual asteroid-action.ppm remains ship dialog "I DETECT AN ABANDONED SHIP NEAR BY", not on-foot/recoil state. No false mechanics claim: status attempted, extra dialog advancement required. Asteroid closure only7 new bytes, so will do one practical dialog continuation then move on rather than repeat unchanged waits. Final-boss password route currently running from clean buffer snapshot. RAM comparison suggests bridge B1=$C0/B2=$29, Cyberia B1=0/B2=3 and Turvia B1=0/B2=$0B, but do not treat this as proven arbitrary stage-entry contract.

Endgame correction: final-boss-action shows Agathos conversation, and final-combat still shows last conversation text "PREPARE TO DIE A SLOW AND PAINFUL DEATH". The attempted A6+wait660 did advance dialogue but had not yet released combat, so no actual boss fight/ending claimed. Next restore final-continued.snapshot and one additional A (bounded follow-up) after asteroid continuation. Static direct-call batch and asteroid dialog continuation currently live; final-boss follow-up will be last gameplay shortcut unless it yields a concrete useful continuation.

Asteroid continuation succeeded: asteroid-recoil.ppm shows floating Rad among rocks after ship dialog A4+wait660; traced left+B30/right+B30 accepted. This provides representative asteroid-system instruction evidence; closure adds67 bytes (initial asteroid7 separately). Static call batch adds22 bytes (zero conflicts). PCM coherent routine exact physical[0x1C83B,0x1C87F) now classified Static Code68 after complete decode/endpoint check; banking effect on lower slots explicit, code fixed banks14/15 with lower context unknown. Natural entry remains unresolved. Final boss last-dialog A +120 and two30-frame combat input blocks underway, then phase will checkpoint and hand off remaining contracts rather than repeat low-yield play.

## Round 1 disassembly completed — incomplete ROM

ROM classification: 63.78% (167214 / 262160 bytes classified; 94946 bytes unknown).

Fresh map validation passed; analysis/round1-disassembly-verification.json confirms byte-identical generated/round1-disassembly rebuild. ROM hash unchanged. Map maps/current.json and state analysis/state.json saved. Handoff analysis/round1/handoff.md; counts analysis/round1/classification.json; exact unknown complement analysis/round1/unknown.json.

Code15643, Data20499, Image131072;715/715 regions have evidence,5 labels. Observed provenance12299 bytes; Static154915. Execution separately6487 physical ROM opcode starts,10370 all spaces,13411 contextual roots; aggregate_dropped0. Capacity0 dropped107926085 details deliberately, so no detailed timeline exists. Boss-engaged.ppm confirms dialogue gone and active boss/player damage; no ending/second-boss visit. Last boss action added17 bytes. Static direct-JSR continuation22 bytes, recheck0; PCM routine68 bytes accepted Static with unknown lower slots and fixed banks14/15.

Lead outcomes: R1-INPUT/R1-PCM/R1-CALLS/R1-PASSWORD verified; R1-ASSETS ready for exact bank/extent research; R1-ENDING blocked on defeat/transition contract (lead only, no workflow-wide blocker). Further unvisited planets are explicit deferred representative alternatives in scenarios.json. This is not terminal investigation exhaustion. Next commentary then redirect.

Toolkit findings:1 confirmed documentation/API bug NES-001;0 suspected bugs,0 separate missing features; no blockers. See analysis/toolkit-issues.md. No toolkit modifications, no scratch cleanup/publication.

ROM classification: 63.78% (167214 / 262160 bytes classified; 94946 bytes unknown).

| Section | Bank | Code | Data | Image | Unknown |
|---|---:|---:|---:|---:|---:|
| header | - | 0 | 16 | 0 | 0 |
| PRG | 0 | 0 | 0 | 0 | 8192 |
| PRG | 1 | 0 | 179 | 0 | 8013 |
| PRG | 2 | 0 | 8192 | 0 | 0 |
| PRG | 3 | 0 | 8192 | 0 | 0 |
| PRG | 4 | 0 | 76 | 0 | 8116 |
| PRG | 5 | 0 | 1000 | 0 | 7192 |
| PRG | 6 | 862 | 0 | 0 | 7330 |
| PRG | 7 | 0 | 843 | 0 | 7349 |
| PRG | 8 | 0 | 0 | 0 | 8192 |
| PRG | 9 | 0 | 1241 | 0 | 6951 |
| PRG | 10 | 1306 | 0 | 0 | 6886 |
| PRG | 11 | 0 | 569 | 0 | 7623 |
| PRG | 12 | 2607 | 0 | 0 | 5585 |
| PRG | 13 | 1626 | 185 | 0 | 6381 |
| PRG | 14 | 6774 | 0 | 0 | 1418 |
| PRG | 15 | 2468 | 6 | 0 | 5718 |
| CHR | 0 | 0 | 0 | 8192 | 0 |
| CHR | 1 | 0 | 0 | 8192 | 0 |
| CHR | 2 | 0 | 0 | 8192 | 0 |
| CHR | 3 | 0 | 0 | 8192 | 0 |
| CHR | 4 | 0 | 0 | 8192 | 0 |
| CHR | 5 | 0 | 0 | 8192 | 0 |
| CHR | 6 | 0 | 0 | 8192 | 0 |
| CHR | 7 | 0 | 0 | 8192 | 0 |
| CHR | 8 | 0 | 0 | 8192 | 0 |
| CHR | 9 | 0 | 0 | 8192 | 0 |
| CHR | 10 | 0 | 0 | 8192 | 0 |
| CHR | 11 | 0 | 0 | 8192 | 0 |
| CHR | 12 | 0 | 0 | 8192 | 0 |
| CHR | 13 | 0 | 0 | 8192 | 0 |
| CHR | 14 | 0 | 0 | 8192 | 0 |
| CHR | 15 | 0 | 0 | 8192 | 0 |

Final consistency check: current map equals checkpoint byte-for-byte (SHA256 c7cd46d7d2b89577189137823edcae9f03fead4cba209a3a7793186aca96282e); JSON artifacts parse; input hash verified again. Console61445 closed after durable phase-final snapshot; new process requires map.load/emulator.start/snapshot.import and button reassertion. Final local round1 scratch size729 MiB, retained as requested. Full generated-source commentary/editorial review remains next phase; structural classification and verified rebuilding do not claim full readability or completion.

## Round 1 commentary — active

Input: maps/round1-disassembly.json / generated/round1-disassembly, SHA256 c7cd46d7d2b89577189137823edcae9f03fead4cba209a3a7793186aca96282e. Prepared commentary once from unchanged current map; baseline verification passed. Frozen commentary/before and reading-before preserved. Full inventory: 832 scaffold intervals, 698 Code fragments, 15,643 classified code bytes plus 20,499 Data and 131,072 CHR. No earlier commentary backlog exists outside this baseline. Reviewing the entire current source, including fragmented control-flow, hardware/RAM naming and unknown boundaries; final editorial acceptance remains reserved. Readability still active, not yet handed off.

### Commentary semantic review milestone (active)

Whole-source first mechanical layer verified byte-identical. Complete instruction inventory has now been read in bounded subsystem batches, including audio, unrolled sprite permutation, bridge/title/password, window/health/inventory/raster tasks, scheduler, asset decoder, camera cache, allocation/deletion, animation interpreter, movement, rectangle contacts, map probes and object scripts/text. Semantic refinements are being applied from this review; the current-source gate remains pending until all fragment dependencies/names and source explanations are reconciled. The first layer is not credited as semantic completion.

Concrete durable health/ending lead: CPU EB8D..EBFB physical[0x1EB9D,0x1EC0B) consumes source contact damage at RAM06C4[Y] and target health RAM0790[X]. Floor subtraction at0; player slot0 has RAM0088 cooldown set3C and difficulty damage shift RAM000A (minimum damage1). Nonplayer damage selects sound08 or52 for remaining/zero health. CPU E336 physical0x1E346 tests zero health in the script interpreter. Redirect should inspect boss snapshot active slots/script and use supported damage/death handling, not arbitrary PC edits. No boss defeat/ending is claimed.

Semantic review identified and is correcting initial broad aliases: RAM0748/0754 are left/right bounds,0730/073C top/bottom,0790 health,06C4 contact damage,079C vertical speed,06DC horizontal speed. Camera scroll at66/67 and packed column/cache state are distinct from object bounds. The title entry9362 is a choice-menu initializer, not the complete password initializer. A346 draws a fixed status frame; palette/pad upload starts atA394. These corrections are in the local semantic-review script, currently being applied/checked. Source/history/ROM preserved; final audit not run.

## Round 1 commentary completed — current-source readable, ROM incomplete

All832 review intervals and698 code fragments covered. Full classified source has contextual mechanical explanations and consistent known roles:360 substantive entry explanations,1,184 control labels,481 RAM/base/member symbols plus hardware/CPU aliases. This follows the complete instruction-inventory semantic pass, not acceptance of the initial mechanical boilerplate. All15,643 Code,20,499 Data and131,072 CHR bytes reviewed;94,946 bytes remain unknown with boundary notes. No current-source readability backlog remains for this checkpoint.

Fresh commentary/verification.json reports byte_identical=true with unchanged ROM SHA256. Final game.asm SHA2569b37840aebe0b55bc870c06c900423eeb28e8fe805c6cc6a9d5d93330720f3ae; symbols.inc SHA256d1a844b7fb1d58c7c0cc3d99bb043be28ce2ba1dc0257bba8bc7a69b6b1368f2. commentary/reading-after.asm refreshed. Frozen before/baseline/state preserved; current map/state unchanged from round1-disassembly. Final editorial acceptance remains pending; no final audit run.

Handoff analysis/round1-commentary/handoff.md; coverage/readability-report.json, semantic-units.json and commentary/uncertainties.md explain reviewed scope and unresolved game meanings. Strong new redirect contract is ApplyContactDamage at physical[0x1EB9D,0x1EC0B), RAM0790 health/06C4 contact damage, player cooldown0088 and difficulty000A; script health test0x1E346. Use boss snapshot active-slot/script inspection and supported transition before claiming defeat. Asset/stage-table extents and ending remain unresolved.

Toolkit:1 confirmed documentation/API issue NES-001,0 suspected bugs,0 separate missing features; no blockers. See analysis/toolkit-issues.md. No toolkit/ROM changes, cleanup, archive or publication. Classification remains63.78% (167214/262160), not whole-ROM completion. Next redirect; terminal acceptance stays with coordinator.

## Round 1 redirect active

Input unchanged round1-disassembly c7cd46d7d2b89577189137823edcae9f03fead4cba209a3a7793186aca96282e. Unique attempt analysis/redirect/r1-assets-stage-damage; plan.md records asset/stage/boss and independent triage methods. Current-source commentary gate accepted; final editorial audit pending. Research only; maps/state/ROM remain unchanged.

Redirect milestone: stage-packed model follows consumer header/13-symbol columns; all34 adjacent128-column endpoints exactly match next pointers. Primary evidence analysis/redirect/r1-assets-stage-damage/{stage-setup.json,stage-tail.json,stage-packed-128columns.json}; results.md records model limits. Independent128-handler dispatch closure yields1044 proposed unknown code bytes; connected special-stage BDEE ending route remains active. Separate boss zero-health probes60frames each did not transition; negative result localized, not defeat rejection. No map/state mutation.

Redirect milestone: supported request from boss initialized special2C ship scene; requests from that checkpoint initialized2A framed waving-Rad ending and2B closing monologue (PNG/snapshots in attempt). Natural boss defeat not claimed. Ten template directories/8byte records validated with exact code barrier in mixed bank5. Full bounded mixed script/animation/text prefix inspection supports generic non-code proposals separately from exact schemas. Small-table/dispatch follow-up and final runtime continuation active; see attempt/results.md.

Redirect batch milestone: proposals.json now85,676 currently unknown proposed bytes (3,470Code/82,206Data), independent validation pending; zero data/code conflicts. Actual map/ROM unchanged, accepted classification63.78% (167214/262160),94946unknown. Exact split ledger unknown-dispositions.json; conditional residual9270 is proposal bookkeeping, not acceptance. The End visibly confirmed ending2a-settled.png; initialized closing monologue2B visited. Custom machine damage-response entry9121/D8=1 produces phase transition andresetsD8=20 by782frames; generic health shortcut was insufficient. ComplementaryDC>=4 source branch running as26887; completed jobs/outcomes andall template/audio/UI/task/JSR findings appended attempt/results.md. Same redirect; no terminal audit or map writes.

## Round 1 redirect completed — combined batch ready

Accepted classification remains **63.78% (167214 / 262160 bytes); 94946 unknown**. Map/current state/output checkpoint remains `maps/round1-disassembly.json`, SHA256 `c7cd46d7d2b89577189137823edcae9f03fead4cba209a3a7793186aca96282e`; ROM SHA256 unchanged. Redirect annotated nothing and changed no commented source.

Handoff: `analysis/redirect/r1-assets-stage-damage/handoff.md`. Combined proposal: **91600 currently unknown bytes (3780 Code, 87820 Data)**, independently unaccepted. Conditional residual3346 is bookkeeping only. Exact ranges/roles/provenance/views in proposals.json; original complement, bank/alias contexts, neighbors and dispositions in unknown-dispositions.json. Verification.json confirms zero code/data conflicts, unchanged ROM/map/state/source, existing byte-identical commentary verification, and zero retained actual ROM opcode roots omitted from the batch. Independent disassembly must validate and freshly rebuild accepted changes; then full current-source commentary is required.

Ready IDs produced/improved: R1-ASSETS, R1-ENDING, R1R-SCRIPT-DISPATCH, R1R-TASK-UI, R1R-STAGE-CONNECTED. Historical input/PCM/call/password outcomes and every redirect attempt remain preserved. Connected resolver/template/packed/stage/damage work and independent script/task/UI/audio/lookup avenues were investigated together; all surfaced current-source indexed-ROM-data frontiers received proposals or limited claims. Precise generic-data and schema evidence are separated in model-proof.md; complete mechanical code/lookup listings are retained.

Broad progression now includes source-record Telos37/39 visits, a supported artificial two-contact/item machine clear, a distinct spaceship boss, a consumer-derived class28 rocket contact reducing its customDD to0, and the separate documented player boarding event entering initialized special2C. The closing monologue2B and fully displayed The End2A were independently visited. No natural fight or natural ending claim. Post-boarding automatic dialogue timing rejoins the already inspected ending system and has reduced structural prospect; no unchanged timing replay is required for this batch. Other unvisited permutations are not declared globally exhausted.

All runtime processes completed and closed. No active productive research interruption is being labeled exhausted. This is an ordinary ready-batch phase handoff, not terminal ROM completion or final audit; final editorial acceptance remains pending. Next: coordinator launches the separate independent disassembly worker. Toolkit report remains1 confirmed documentation/API issue NES-001,0 suspected bugs,0 separate missing features; no blocker or toolkit edit. No archive/publication/deletion/cleanup occurred; all local evidence retained.

## Round 2 disassembly active

Independently reviewing the combined redirect batch against ROM bytes, consumer contracts, corrected code listings and exact bank views. Input checkpoint maps/round1-disassembly.json; current state/map unchanged. New evidence and decisions persist in analysis/round2. Existing commentary and all historical artifacts preserved; no terminal exhaustion claim.

Round2 data batch persisted at analysis/round2/data-batch-state.json with162 new exact data segments,87820 novel Data bytes. Fresh independent tree/template/dispatch/model and view checks are in analysis/round2/independent-review.json. One unsupported KIL entry interpretation overlapping a valid JSR operand is rejected without changing the union; details in review.md. Code annotations and checkpoint verification are in progress.

## Round 2 disassembly completed — incomplete

ROM classification: 98.80% (259011 / 262160 bytes classified; 3149 bytes unknown).

Accepted 91797 unique new bytes:3977 Code (3780 redirect-proposed plus197 independently recovered direct continuations),87820 Data. Whole-ROM totals Code19620,Data108319,Image131072; unique counts exclude unknown raw export fallback. All prior regions preserved exactly. 2815 evidence-bearing regions; fresh checkpoint maps/round2-disassembly.json SHA256a50228a71f93e34e2c3da8fd3a35aeddaee01cc6f511437c6ed8bfc85c0f3e9a. analysis/round2-disassembly-verification.json confirms byte_identical=true, generated/round2-disassembly. State/current/checkpoint byte-identical; input unchanged.

Independent review: analysis/round2/review.md, independent-review.json, code-review.json/txt and accepted-continuations.json. Recomputed template records and sorted complete pointer domains (bank2 has a two-record permutation), packed/tree endpoints, all128 dispatch words, every code view and all conflicts. Unsupported operand-overlap KIL entry rejected at19C77; valid JSR still classifies that byte.39 ordinary-fallthrough continuation spans added197 bytes after complete local contract review. No runtime replay: prior representative late-game, machine/ship boss and initialized ending captures reused, preserving artificial/natural distinctions.

Ready leads R1-ASSETS,R1-ENDING,R1R-SCRIPT-DISPATCH,R1R-TASK-UI,R1R-STAGE-CONNECTED verified for their bounded structural batch; unresolved natural selection and original stream semantics remain separate. Exact residual complement: analysis/round2/unknown.json (35 gaps). Larger bridge/title/script gaps are concretely promising for redirect, but need inline-text/control boundaries and source entry context; unused/terminal claims are not made. This pass does not assert terminal exhaustion.

Retained actual opcode-root union:7282 physical starts,15612 contextual roots,all classified Code; this is separate from structural coverage and not a whole-game execution metric. See classification.json for artifact scope. Existing full-source commentary from round1 preserved byte-for-byte and remains valid for unchanged source; fresh generated source now needs separate full current-source commentary. Final editorial audit pending.

Toolkit:1 confirmed documentation/API issue NES-001,0 suspected,0 separate missing features,0 blockers; analysis/toolkit-issues.md. Configured release binary only; no toolkit, ROM, commentary, archive, publication, deletion or cleanup changes. Runtime review/apply processes closed.

| Section | Bank | Code | Data | Image | Classified | Unknown |
|---|---:|---:|---:|---:|---:|---:|
| header | - | 0 | 16 | 0 | 16 | 0 |
| PRG | 0 | 0 | 8192 | 0 | 8192 | 0 |
| PRG | 1 | 0 | 8192 | 0 | 8192 | 0 |
| PRG | 2 | 0 | 8192 | 0 | 8192 | 0 |
| PRG | 3 | 0 | 8192 | 0 | 8192 | 0 |
| PRG | 4 | 0 | 8192 | 0 | 8192 | 0 |
| PRG | 5 | 0 | 8192 | 0 | 8192 | 0 |
| PRG | 6 | 873 | 7207 | 0 | 8080 | 112 |
| PRG | 7 | 0 | 8192 | 0 | 8192 | 0 |
| PRG | 8 | 0 | 8192 | 0 | 8192 | 0 |
| PRG | 9 | 0 | 8192 | 0 | 8192 | 0 |
| PRG | 10 | 1306 | 6886 | 0 | 8192 | 0 |
| PRG | 11 | 0 | 8192 | 0 | 8192 | 0 |
| PRG | 12 | 3706 | 2021 | 0 | 5727 | 2465 |
| PRG | 13 | 2126 | 5803 | 0 | 7929 | 263 |
| PRG | 14 | 7591 | 428 | 0 | 8019 | 173 |
| PRG | 15 | 4018 | 4038 | 0 | 8056 | 136 |
| CHR | 0 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 1 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 2 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 3 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 4 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 5 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 6 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 7 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 8 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 9 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 10 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 11 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 12 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 13 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 14 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 15 | 0 | 0 | 8192 | 8192 | 0 |

## Round 2 commentary active

Input current/state/map SHA256 a50228a71f93e34e2c3da8fd3a35aeddaee01cc6f511437c6ed8bfc85c0f3e9a; classification unchanged 98.80%,3149 unknown. Preserve the entire prior commentary as analysis/round2-commentary/prior-commentary before fresh preparation. Carry prose, symbols and review by exact unchanged physical intervals and dependencies. Reconcile every newly classified interval, then fresh verification and reading copy; final editorial audit stays pending. No runtime/classification/toolkit changes. Free disk8.5GiB; no scratch cleanup authorized.

## Round 2 commentary completed — full current-source readability

Input/output classification unchanged98.80% (259011/262160);3149unknown35gaps. All2850scaffold intervals reviewed with zero classified-source backlog, retaining745exact prior intervals/all15643old Code bytes and360substantive prior semantic entries. Entire prior commentary preserved in analysis/round2-commentary/prior-commentary before fresh baseline. New3977Code bytes (1984instructions/1938fragments) have105substantive routine/control explanations;162new Data regions/87820bytes have bounded format/consumer/uncertainty notes. Four affected old explanations updated for new resolver/PCM/special-stage evidence, consistent shared RAM/fixed table aliases reconciled, and unaffected prose/overviews retained. Unknowns honestly annotated; no classification/runtime claims added.

Fresh commentary/verification.json and analysis/round2-commentary/verification.json confirm byte_identical=true; input hash unchanged. Current/state/checkpoint and commentary baseline/state SHA256a50228a71f93e34e2c3da8fd3a35aeddaee01cc6f511437c6ed8bfc85c0f3e9a. reading-after.asm refreshed and independently compared to exact source expansion. readability-report.json checks full unchanged scaffold/evidence, counts, source locations and old prose carry. No final audit; editorial_audit remains pending for terminal acceptance. Full scope/findings/unresolved meanings/redirect frontiers in analysis/round2-commentary/handoff.md and commentary/uncertainties.md. Next separate phase: redirect.

Toolkit report unchanged1confirmed NES-001 documentation/API issue,0suspected,0separate missing features,0blockers; analysis/toolkit-issues.md. No toolkit/ROM edits,runtime servers,archive/publication,deletion or cleanup. Free disk8.4GiB; all history preserved locally.

## Round 2 redirect active — r2-inline-contracts
Input map/state a50228a71f93e34e2c3da8fd3a35aeddaee01cc6f511437c6ed8bfc85c0f3e9a,98.80%,3149unknown35gaps. Full current-source readability gate met. Unique attempt analysis/redirect/r2-inline-contracts/plan.md starts with bank12 bridge/message ordinary RTS versus inline-data contracts, then fixed-tail and independent helper gaps. Prior accepted attempt immutable by reference; no unchanged gameplay replay/map edits. Primary listings and frontiers will persist here.

Round2 redirect r2-inline-contracts completed substantive connected/independent research; primary results now in results.md, proposals.json, gap-decodes.json, proposed-instructions.json and validation.json. Proposed3146new bytes=2356Code+790Data across42exact disjoint intervals;1114new static instructions; no new code edge into proposedData, exact ends verified. Bank12 bridge dialogue/navigation graph followed through all known helpers/text/phase continuations, then independent audio JMP/handler tables, password/PPU/inventory/pause/fixed helpers and arithmetic fixed tail. Bank-switch instruction contexts explicitly segmented. Prior r1 batch immutable, no replay/map/source edits.

Exact35-gap disposition ledger unknown-dispositions.json: proposed3146, three unresolved isolated RTS bytes1C74D/1C74E/1E93D. Results.md records decisive local bytes, predecessor RTS/no fallthrough, complete routine/padding alternatives, direct-pointer/control negative inspection and why null-entry conditional decoding does not establish structural role. Not a universal live-caller requirement. Remaining semantic/natural-selection questions have no currently concrete structural discriminator. Ordinary ready batch returns to independent disassembly; not terminal exhaustion or final editorial acceptance. Accepted checkpoint/counts unchanged98.80%,3149unknown until review.

## Round 2 redirect complete — combined batch ready
analysis/redirect/r2-inline-contracts/handoff.md hands3146proposed bytes(2356Code/790Data),42intervals,1114instructions to independent disassembly. New ready IDs R2R-AUDIO-CONTRACTS112,R2R-BRIDGE-INLINE2127,R2R-FIXED-CONTRACTS156,R2R-FIXED-TAIL132; materially improved R1R-TASK-UI619 returned ready with prior accepted history preserved. No old lead reissued unchanged. proposals.json/context_segments and results.md are authoritative recipe/rationale; gap-listing linear text disassembly is not code evidence.35gap dispositions exact; conditional residual3isolatedRTS bytes as localized in results.md. Map/state/current unchangeda50228a71f93e34e2c3da8fd3a35aeddaee01cc6f511437c6ed8bfc85c0f3e9a; accepted98.80%,3149unknown. No terminal exhaustion/editorial claim. Existing commentary byte-identical verification valid,full current-source readability retained; preservation.json records hashes. No active servers/runtime sessions/toolkitchanges/ROMchanges/cleanup/publication. Next separate disassembly worker.

## Round 3 disassembly active
Input r2-inline-contracts/handoff.md; checkpoint a50228a71f93e34e2c3da8fd3a35aeddaee01cc6f511437c6ed8bfc85c0f3e9a. Independent review of ordinary return, text, audio-table and bank-switch contracts underway. Prior commentary/history retained. Exactly one disassembly phase.

## Round 3 disassembly completed — incomplete

ROM classification: 100.00% (262157 / 262160 bytes classified; 3 bytes unknown).

Rounded percentage is not completion. Unique whole-ROM totals Code21976, Data109109, Image131072; header included in Data. Accepted3146new bytes (2356Code/790Data),42proposal intervals split47regions at explicit bank-change boundaries;1114static instructions. All prior regions/evidence preserved exactly. 2862regions, 2862 evidence-bearing. Review analysis/round3/review.md independently checks allocator/text/scanner ordinary-return contracts, explicit text records, audio JMP and split handler entries, inventory dispatch, coherent CAF6 reservoir and C807 bank wrapper, and bounded arithmetic non-code tail. Fresh decode primary analysis/round3/independent-decode.json; checks/log alongside it.

State/current/fresh maps/round3-disassembly.json SHA256869ab8e8ec00b1c2902b1ad0aeb3379a741b6d162ebaba24cf881e57274388a9; map validated, generated/round3-disassembly verified byte_identical=true in analysis/round3-disassembly-verification.json. Input ROM unchanged. Existing commentary game.asm/symbols.inc hashes match redirect preservation. New generated source now needs full current-source commentary; final editorial audit pending. No terminal exhaustion claim.

Verified R2R-AUDIO-CONTRACTS,R2R-BRIDGE-INLINE,R2R-FIXED-CONTRACTS,R2R-FIXED-TAIL and pending R1R-TASK-UI structural ranges; natural use/selector identities remain separately unresolved. Exact unknown complement analysis/round3/unknown.json: physical1C74D..1C74F and1E93D..1E93E, all60. Null-entry versus separator/padding remain indistinguishable without another independent constraint; absent callers alone was not a rejection. Keep3unknown and forward review to separate commentary then redirect. No additional supported connected unknown interval remains outside that localized ambiguity; broad progression/scenarios and late/ending artificial observations reused without unchanged replay.

Execution metric unchanged: retained union7282physical opcode starts/15612context roots; scope retained primary artifacts, separate from structural classification, not whole-game coverage. No new runtime observation. Toolkit unchanged1confirmed NES-001 documentation/API issue,0suspected,0separate missing features,0blockers; analysis/toolkit-issues.md. Buttons-array workaround. Configured release only; no toolkit/ROM/commentary edits,archive/publication,deletion/cleanup. All processes closed.

| Section | Bank | Code | Data | Image | Classified | Unknown |
|---|---:|---:|---:|---:|---:|---:|
| header | - | 0 | 16 | 0 | 16 | 0 |
| PRG | 0 | 0 | 8192 | 0 | 8192 | 0 |
| PRG | 1 | 0 | 8192 | 0 | 8192 | 0 |
| PRG | 2 | 0 | 8192 | 0 | 8192 | 0 |
| PRG | 3 | 0 | 8192 | 0 | 8192 | 0 |
| PRG | 4 | 0 | 8192 | 0 | 8192 | 0 |
| PRG | 5 | 0 | 8192 | 0 | 8192 | 0 |
| PRG | 6 | 985 | 7207 | 0 | 8192 | 0 |
| PRG | 7 | 0 | 8192 | 0 | 8192 | 0 |
| PRG | 8 | 0 | 8192 | 0 | 8192 | 0 |
| PRG | 9 | 0 | 8192 | 0 | 8192 | 0 |
| PRG | 10 | 1306 | 6886 | 0 | 8192 | 0 |
| PRG | 11 | 0 | 8192 | 0 | 8192 | 0 |
| PRG | 12 | 5513 | 2679 | 0 | 8192 | 0 |
| PRG | 13 | 2389 | 5803 | 0 | 8192 | 0 |
| PRG | 14 | 7762 | 428 | 0 | 8190 | 2 |
| PRG | 15 | 4021 | 4170 | 0 | 8191 | 1 |
| CHR | 0 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 1 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 2 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 3 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 4 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 5 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 6 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 7 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 8 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 9 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 10 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 11 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 12 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 13 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 14 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 15 | 0 | 0 | 8192 | 8192 | 0 |

## Round 3 commentary — full current-source readability complete

Full current-source readability complete. All 2864 scaffold intervals are reviewed; zero classified-source backlog. Classified 262157/262160 (100.00% rounded), with 3 unknown bytes explicitly incomplete. Code21976, Data109109, CHR131072. No map or runtime evidence changes.

Entire prior commentary preserved at analysis/round3-commentary/prior-commentary before fresh preparation. 2815 exact classified physical/kind/evidence intervals retain sound prior prose/review and all19620 old Code bytes. New47 intervals/1114instructions reviewed under67 substantive units; seven stale gap caveats, the prior UI/navigation data structure and affected references reconciled. Shared RAM aliases distinguish bridge, password, column editing and inventory blink uses; audio_fade_enabled now connects producer and consumer. New dialogue and all five nested navigation records have explicit readable representation. Remaining game identities are localized in uncertainties.md.

Fresh configured-release commentary verification: byte_identical=true; reading-after.asm refreshed from the exact verified source. analysis/round3-commentary/readability-report.json checks scaffold/evidence identity, full scope/source links, prior prose carry, hashes and exact reading expansion. Source hashes and map identity are recorded in that report and handoff. Final editorial_audit remains pending; no final audit command ran. No deferred classified source ranges.

Separate redirect is next for the three isolated RTS alternatives. C807 jumps RAM0000, not003E; initialized target/use unresolved. CAF6 coherent static routine has no caller claim. Patterned tail has no specific consumer claim. These do not force residual classification. Toolkit: sole confirmed NES-001 documentation/API issue;0suspected/0missing features/0blockers; no new toolkit issues found. No ROM/toolkit edits, runtime server, archive/publication or cleanup. All artifacts local.

Compact handoff: analysis/round3-commentary/handoff.md. Fresh verification: commentary/verification.json; exact reading copy: commentary/reading-after.asm.

## Round3 redirect — residual reasoning review; coordinator inspection pending

Attempt analysis/redirect/r3-residual-contract-review. Checkpoint unchanged869ab8e8ec00b1c2902b1ad0aeb3379a741b6d162ebaba24cf881e57274388a9;262157/262160classified(100.00%rounded),3unknown explicitly incomplete. No new bytes/proposals, no runtime or unchanged scans. Primary local comparison independently separates script table-selectedE92C from residualE92D and treatsC73D/C73E separately; bounded padding alternatives retain material code/data ambiguity. Compact rejection-review.md includes exact claims, primary links, negative scope, combined constraints, competing interpretations and remaining insufficiency. No supported revised inference/test or pending productive avenue identified. leads.json pass appended empty added/updated IDs; previous histories unchanged. Terminal incomplete only proposed, coordinator must inspect reasoning before accepting.

Current-source readability complete2864intervals, exact reading/byte-identical verification preserved, final editorial audit pending. Source:3570 stale structurally-unresolved text caveat recorded for final editorial correction, source unchanged. Progression checklist and retained later/ending/contact/boarding evidence reviewed without unchanged replay; artificial/natural distinction retained. NES-001 only,0blockers,no new toolkit issues. No maps/ROM/toolkit/source edits,servers,archive/publication,deletion/cleanup. Handoff analysis/redirect/r3-residual-contract-review/handoff.md; primary-local.json,identities.json,unknown.json.

## Final editorial acceptance active

Separate terminal editorial gate after coordinator accepted justified incomplete investigation stop. Preserve valid full-source review credit; inspect concrete stale bridge text, stage-directory and no-op naming issues plus current uncertainty frontiers. No map classification or toolkit changes. Pre-final source/review/reading/verification preserved under analysis/final-acceptance/20261104T233017Z/pre-final.

## Final editorial acceptance completed

Current-source editorial accepted: all2864intervals/2876units reviewed; corrected Cyberia text, stage-template directory/CPU aliases, table-selected no-op name and duplicate C807 prose. Reused complete sound prior review credit. Fresh supported audit byte_identical=true, exact refreshed reading expansion, fresh map/store validation passed. ROM incomplete262157/262160 with3unknown physical1C74D,1C74E,1E93D; no classification changes. Evidence/frozen identities preserved. Handoff analysis/final-acceptance/20261104T233017Z/handoff.md; substantive findings commentary/editorial-audit.md. Coordinator final selection/archive preparation remain separate; no archive/publication/toolkit/ROM edits/deletion/cleanup.

## Terminal coordinator handoff — justified incomplete

ROM classification:100.00% (262157 /262160bytes classified;3bytes unknown). Rounded percentage is not completion. Terminal rejection rationale accepted after direct primary ROM/dispatch/trampoline inspection; no ready leads, pending ranges or interrupted productive research. Final editorial audit accepted2864regions/2876units with fresh byte-identical source/reading and valid map/store. Actual ROM/current source/checkpoint identities and exact disjoint internal coverage independently checked in analysis/coordinator/final-verification.json. Final handoff analysis/final-handoff.md and requirement-by-requirement analysis/coordinator/completion-audit.md separate workflow completion from incomplete ROM classification. archive-options.json explicitly selects final/historical Markdown and omits only duplicate instruction scaffolding; all files retained locally. No archive/publication,toolkit/ROM changes,deletion or cleanup. Orchestration ready for completion; classification remains incomplete.
