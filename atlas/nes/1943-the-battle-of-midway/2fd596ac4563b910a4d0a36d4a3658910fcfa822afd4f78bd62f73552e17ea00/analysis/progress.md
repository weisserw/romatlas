# Disassembly progress

Workspace initialized. Baseline and classification have not run.

## Round 001 — disassembly in progress
- Baseline completed and verified byte-identical: maps/baseline.json, analysis/baseline-verification.json, generated/baseline.
- ROM mapper 2, 131088 bytes; title capture identifies 1943, 1988 Capcom. Vectors NMI $C226, reset $C149, IRQ $C235 in fixed PRG16 bank 7 (physical 8 KiB banks 14/15).
- Live JSON-RPC subprocess held by Python REPL PTY session 21138, driver scratch/rpc.py. Every request/result logged locally in analysis/evidence/pass-001-rpc.jsonl. Resume after process loss with fresh serve + map.load and recorded input routes; snapshots not yet exported.
- Runtime startup at frame 79 reached title. Press/release Start over frames 79–81 enters long ship/prologue sequence, still prologue at frame 2273. Further Start press at frame 471 did not visibly skip it. Tracing disabled for frames 171–471, 473–2273 to bound overhead; captured intervals before/after retain aggregate roots. No drop in aggregates; detail ring overflow expected (753666 events at frame79), capacity now zero.
- Current frontier: initial runtime contexts (1945) seeded static discovery; 6619 contextual candidates / 1951 unique physical instruction candidates being decoded and retained in analysis/evidence/decoded-initial-candidates.json before acceptance. No classifications accepted yet; state still empty.
- Static tile inspection strongly identifies banks 0,2,3 as graphics and bank1 prefix as graphics, with bank1 latter half structured records. Tile previews scratch/bank0.png..bank3.png will be promoted if cited. Need practical consumer/boundary review and conflicting-code check before annotate.
- API parameter discrepancy: NES discovery uses max_instructions, not generic protocol's budget. Documented NES limit with concrete reproducer will be recorded in toolkit issues if confirmed documentation impact.
- Next: inspect candidate listing, accept observed/static supported instructions, annotate header/vectors and visual tile data, advance beyond prologue, identify RAM state/level/password contract and make representative later-stage/ending best-effort shortcuts, save/export/validate and fresh pass-001 checkpoint.

### Intermediate classification and input correction
- Accepted 5099 code bytes, 65558 data/image/header/vector bytes; internal map currently246 regions,70657 classified bytes (53.90% provisional). Saved analysis/state.json.
- Tile preview evidence promoted to analysis/evidence/bank0.png..bank3.png. Banks0/2/3 image, bank1 first4KiB image and rest generic data after repeated graphics/record inspection; no conflicting candidates in those banks. All image bytes remain local.
- Input correction NES-001: prior button/pressed requests were no-ops. Correct buttons list skipped attract to title at2299–2329; Start at2329–2332 entered upgrade transition. Upgrade screen at2735; Up/Down navigates, B allocates (A/right no effect); three B press/release pairs at3455–3467, Start3467–3470 entered launch. Full exact requests in evidence/pass-001-rpc.jsonl.
- Saved analysis/evidence/flight.snapshot at upgrade menu (2735; name historical, not combat) and stage1-entry.snapshot at3810 launch. Same-build snapshots contain ROM and remain local.
- At4410 launch reached mapper banks10/11, first combat path capture underway. API documentation issues NES-001 and NES-002 recorded; no toolkit edits.

### Broad progression and corrected RAM contract
- Natural combat verified at4450–4500, with movement/shooting/special input combinations and local combat snapshot. analysis/scenarios.json now preserves all emulator actions/releases/budgets and milestone checklist.
- $9C=11 shortcut failed: launch reset it to0. Do not treat old stage12-attempt*.ppm or stage12.snapshot as validated later level. Corrected inference from password consumer: $0715=(decoded_stage−1)*2, code physical $1A003–$1A011. Restored launch and patched$0715=22; selector persists, stream pointers change and flight code executes. Artificial midgame state, complete representative level12 visit not proven. final-combat snapshot uses$0715=47; final discriminator consumer compares#$2F. Enemy flight/GAME OVER reached, boss visit not proven.
- Direct ending entry$FA40 executes but stripes rather than valid ending; revised bounded full stage-result setup entry$F541, success values$0739=0,$0730=$30 and pause$30=0 now running. No natural ending/boss claim. Ending uncertainty is concrete redirect work if preparation remains unsuccessful.
- Additional generic data:519 exact uniform-run bytes and bank4 pointer/record suffix$11BA7–$14010 (9321B), sprite-layout records$11617–$11759 (322B). Additional observed code merge pending. Toolkit issues remain two documentation/API discrepancies, neither blocking.

## Round 001 disassembly — completed incomplete, pass-001-final

ROM classification: 67.25% (88154 / 131088 bytes classified; 42934 bytes unknown).

Internal store: analysis/state.json; current ROM-free portable mapping: maps/current.json. Classification is 12434 code,22472 data,53248 image bytes; remaining42934 unknown. Header16 bytes included in full physical total. All provenance is observed actual instruction fetch or explicit Static inference; artificial scenarios remain artificial and do not establish natural reachability.

| PRG 8KiB bank | Code | Data/image | Unknown |
|---|---:|---:|---:|
| 0 | 0 | 8192 | 0 |
| 1 | 0 | 8192 | 0 |
| 2 | 0 | 8192 | 0 |
| 3 | 0 | 8192 | 0 |
| 4 | 0 | 8192 | 0 |
| 5 | 0 | 8192 | 0 |
| 6 | 0 | 8192 | 0 |
| 7 | 0 | 8192 | 0 |
| 8 | 1099 | 1906 | 5187 |
| 9 | 0 | 8192 | 0 |
| 10 | 2241 | 0 | 5951 |
| 11 | 321 | 0 | 7871 |
| 12 | 828 | 0 | 7364 |
| 13 | 2337 | 0 | 5855 |
| 14 | 3139 | 64 | 4989 |
| 15 | 2469 | 6 | 5717 |


### Evidence and verification
- Validated vector entries and effective mapper2 contexts, then actual title/menu/natural stage1 combat fetches and supported conditional/direct-call paths. Inspected official-instruction decoding and mapped bank views before acceptance. Classified image banks using complete tile raster inspection; generic data after bounded consumer/reference and record-pattern inspection; no observed fetch conflicts with data in combined captures.
- Evidence: analysis/evidence/decoded-initial-candidates.json, decoded-with-direct-callees.json, new-runtime-decodes.json, decoded-combat-all.json, decoded-final-static.json, decoded-final-callees.json, contexts-all.json, uniform-runs.json, bank0.png..bank3.png. Complete request/result history: pass-001-rpc.jsonl. Precise remaining intervals and per-bank counts: classification-final.json.
- Baseline and intermediate pass-001 retained. Final checkpoint maps/pass-001-final.json, generated/pass-001-final/game.asm, analysis/pass-001-final-verification.json: ok=true,byte_identical=true, map SHA256 1235bbba785ad78213dfc49a39546d54d1f07fd228cdc14e4b403d4e75d91ece. Generated diagnostics empty. Input SHA256 unchanged2fd596ac4563b910a4d0a36d4a3658910fcfa822afd4f78bd62f73552e17ea00.

### Pass outcome and next phase
- Natural stage1 shooting/movement/special inputs exercised; title, attract, upgrade, carrier launch visited. Upgrade screen B allocates points, then YES confirmation. Later-stage$0715 edits and final$2F state execute with distinct streams/enemies but do not establish full representative later-level/boss completion. Wrong$9C shortcut rejected after reset evidence.
- Ending direct$FA40 entry produced stripes. Corrected full result setup$F541 with $0739=0,$0730=$30 produced WHAT A PITY! and resumed flight; no ending/credits visit established. Both bounded experiments preserved. No additional arbitrary RAM/register trials; redirect should determine result flag/initialization contract or use validated password inputs.
- This is a normal single-pass handoff, not terminal exhaustion. Straight startup/runtime fetches, conditional continuations, direct-call entry contracts, and obvious graphics/uniform/record data have been processed. Remaining promising work needs dispatcher/table bounds and bank/selector contracts, so hand it to redirect instead of speculative code/data promotion. See leads.json and handoff-round-001-disassembly.md. Commentary has not run; full current-source readability remains pending.
- Toolkit report analysis/toolkit-issues.md:2 confirmed documentation/API inconsistencies (NES-001 input schema,NES-002 discovery budget),0 suspected,0 missing features,0 blockers. Workarounds used; toolkit unchanged.
- RPC server closed cleanly. REPL no longer owns a live server. Resume with configured run.py serve, map.load and replay/import snapshots; final-live.snapshot captures last artificial state, not a recommended clean natural baseline. For gameplay branch prefer combat1.snapshot or stage1-entry.snapshot.
- Disk at handoff:12GiB free, scratch240KiB before removal of duplicate PNG previews, cited local evidence about82MiB. No ROM/assets published; no archive/deletion performed. Selected captures/snapshots are retained locally.

Final artificial-state structural review: analysis/evidence/artificial-code-review.json explicitly pairs additional debug-edited fetches with ordinary launch/selector and final-result caller/control/stack contracts. A reached address or altered PC alone was not used as structural proof. Handoff: analysis/handoff-round-001-disassembly.md. Final reviewed stage-result attempt displayed WHAT A PITY! and returned to flight; terminal ending screen remains unvisited. RPC server is closed.

## Round001 commentary in progress

Prepared frozen commentary baseline and verified initial byte identity. Inspected the complete 9,197-line current source and recorded 172 manually interpreted subsystem ranges in commentary/subsystems.json. All classified code has a subsystem explanation; added bank-qualified entry/branch symbols and established RAM/hardware aliases. Data/image and unknown includes have role/boundary notes. Remaining: build fidelity, semantic/symbol consistency review, inventory and refreshed reading copy. No successful handoff claimed yet.

## Round001 commentary complete

Full current-source readability reviewed: 798/798 physical scaffold entries (589 code,8 data,4 image,197 unknown); 173 subsystem explanations cover all12,434 classified code bytes. Source has bank-qualified entry/branch symbols, consistent RAM/object/hardware and consumer-table aliases, and explicit unknown-boundary notes. Meaningful mechanics include inline dispatch/allocator stack contracts, audio ownership and stream commands, 8.8 motion/collision, decimal carry/borrow, upgrade dependencies, PPU request priority and scrolling, gameplay/result sequencing. No remaining readability range deferred. Unresolved semantics and structural unknowns remain separate in commentary/uncertainties.md and classification-final.json.

Fresh commentary verification ok=true,byte_identical=true; final game.asm SHA256516546b9a822d3be95a01fc28c173266326ebdde5607bc19f1928157fca6da5b. Refreshed reading-after.asm. Frozen map/state equal current map/state; ROM unchanged. Full handoff analysis/handoff-round-001-commentary.md. Final editorial audit pending; no redirect/archive/publish/cleanup or toolkit edit performed. Existing toolkit issues2 confirmed,0 suspected,0 missing,0 blockers; no new commentary issue.

## Round001 redirect in progress
Unique attempt analysis/redirect/round001-20261030/; plan and primary rpc.jsonl retained. Read evidence-policy/workflow, compact commentary gate and R001–R004. Map/state/source and configured binary unchanged. Static inspection confirms command gateways set mapper4/5/6 then tail-dispatch through $9208/$A800/$8040; selected handlers must obey caller stack contracts, and inline dispatcher tables are non-fallthrough barriers. Investigating connected handlers plus independent residual audio/data boundaries. Current exact residual is unchanged 42934B; ledger preparation uses classification-final.json, not raw export gaps. No failed ending shortcut replay. Live research driver scratch/redirect/driver.py, Python PTY session70778, holds bounded run.py serve; recover from persistent artifacts if interrupted.

Redirect substantive checkpoint: command-directory and inline target graphs followed across enemy initializers/behaviors, formation states, shot/weapon/auxiliary states, audio commands and attract states. 34 inline tables now have localized candidate boundaries and selected coherent handler graphs; complete selector domains are recorded separately. Directory bank5 indices3/4 are data directories, excluded from code roots. New independent data evidence includes bank4 background container ($F10F/$F1DC consumers), audio duration/period/stream region, bank6 velocity profiles and text scripts. Found accepted-data contradiction at physical72615/CPUbank4:$9B97: divider's required RTS; preserve old evidence and propose one-byte correction in disassembly.

R003 materially revised ending test succeeded: restored clean natural combat1, reasserted $0715=47 and $0735..38=[1,1,1,1], entered full $F541 result initializer at instruction boundary SP=$FD; captured MISSION24 CLEAR / TOKAKU destroyed ending at frame4872. $0739=0 selects branch with final-phase discriminator; its prior semantic name 'Success' is misleading. Ending-revised.snapshot and exact RPC retained. Artificial initialization does not establish natural final boss/ending reachability. Full $F3FB per-phase init (both stage streams initialized) plus decimal resource999 now runs final47 waves; bounded camera-velocity edits advance supported streams and display ship/terrain encounters. Continued boss exploration underway; no arbitrary state sweep.

Static bank refinement: mapper changes invalidate only lower slots; fixed-bank instruction continuation can remain structurally classifiable with lower context unresolved. Current graph recipe stops unresolved lower calls, preserves inline-data barriers and explicit constant bank selection. Exploratory misdecodes retained in earlier graph files, superseded by barrier-aware review-candidate-graph; these are not proposed classifications. Maps/source/state still unchanged.

## Round001 redirect complete — ready for independent disassembly

Resumed durable round001-20261030 outcome without replaying completed experiments. Combined proposals:17234 unknown Code+25688 unknown Data=42922B; independent one-byte divider RTS Data→Code correction at72615. No annotations accepted here. Current pass-001-final remains88154/131088 classified,42934 unknown. Conditional residual12 singleton bytes has localized role ambiguity, not missing semantic names. Connected command/actor/helper/audio graphs and independent data containers are packaged together; no known promising actionable avenue deferred merely as independent. Final-phase encounters and victory screen were artificially visited; natural boss/ending completion unproved, unchanged waits diminished.

Fresh packaging-verification.json verifies exact197-range unknown complement, disjoint proposals, instruction non-overlap, physical bank identity and preservation hashes. Existing byte-identical commentary/checkpoint verification retained; no fresh runtime/build claimed. R001/R002/R004/R005 ready, R003 verified research; leads/scenarios updated. Handoff: analysis/handoff-round-001-redirect.md; primary contracts/outcome/residual-review in analysis/redirect/round001-20261030/. Next phase independently accepts/rejects annotations, verifies checkpoint, then full commentary and redirect. Not terminal exhaustion.

Old PTY70778 is unavailable; no live process claimed. Map/state/source/ROM/toolkit unchanged; no archive/publish/deletion/cleanup. Disk11GiB free,scratch156KiB. Toolkit report2 confirmed,0 suspected,0 missing,0 blockers; no new toolkit issues found.

## Round002 disassembly — complete pass, classification incomplete

ROM classification: 99.99% (131076 / 131088 bytes classified; 12 bytes unknown).

Single disassembly pass from pass-001-final accepted17,234 previously unknown Code bytes and25,688 previously unknown Data bytes, all Static, plus corrected exactly1 previously classified Data byte at72615 to Code. No proposal intervals rejected after independent review. Structural classification now29,669 Code+48,159 Data+53,248 Image=131,076. Data includes the16-byte iNES header; PRG Data alone is48,143. Execution coverage and natural reachability are separate. Exact unique-byte ledger: analysis/classification-round002-final.json.

| PRG 8KiB bank | Code | Data | Image | Classified | Unknown |
|---|---:|---:|---:|---:|---:|
| 0 | 0 | 0 | 8192 | 8192 | 0 |
| 1 | 0 | 0 | 8192 | 8192 | 0 |
| 2 | 0 | 4096 | 4096 | 8192 | 0 |
| 3 | 0 | 8192 | 0 | 8192 | 0 |
| 4 | 0 | 0 | 8192 | 8192 | 0 |
| 5 | 0 | 0 | 8192 | 8192 | 0 |
| 6 | 0 | 0 | 8192 | 8192 | 0 |
| 7 | 0 | 0 | 8192 | 8192 | 0 |
| 8 | 1885 | 6307 | 0 | 8192 | 0 |
| 9 | 0 | 8192 | 0 | 8192 | 0 |
| 10 | 2387 | 5805 | 0 | 8192 | 0 |
| 11 | 2185 | 6007 | 0 | 8192 | 0 |
| 12 | 3126 | 5065 | 0 | 8191 | 1 |
| 13 | 5594 | 2594 | 0 | 8188 | 4 |
| 14 | 7244 | 946 | 0 | 8190 | 2 |
| 15 | 7248 | 939 | 0 | 8187 | 5 |

All12 remaining intervals are single bytes: 100262 (bank6:$8796,$60), 107629 (bank6:$A45D,$60), 107656 (bank6:$A478,$60), 111752 (bank6:$B478,$60), 112351 (bank6:$B6CF,$60), 115687 (bank7:$C3D7,$FF), 121412 (bank7:$DA34,$60), 123792 (bank7:$E380,$60), 123895 (bank7:$E3E7,$60), 123937 (bank7:$E411,$60), 127724 (bank7:$F2DC,$60), 129107 (bank7:$F843,$60). Eleven return-opcode bytes and one $FF have materially ambiguous code/data roles despite surrounding supported routines/records; absence of entry or execution is not data proof. The localized residual-review remains applicable; no schema-only unknowns remain. Whole-ROM classification remains incomplete.

Evidence: analysis/disassembly/round002-20261030/review.md independently reviews dispatch consumption, stack/entry/mapper contracts and container boundaries; review-rpc.jsonl freshly decodes all13,847 connected instructions in238 bounded requests; decode-boundary-review.json verifies exact physical mapping, all34 inline word prefixes, no instruction overlap or code/data conflict; data-inspection.json inspects182 exact blocks and base consumers. Application split184 unknown data fragments and238 code fragments, preserving every prior region verbatim except id250's one-byte suffix correction. state-before.json preserves the old record including its superseded boundary premise. No statically inferred bytes marked Observed.

Fresh map.validate, map.save analysis/state.json, map.export maps/current.json, checkpoint maps/pass-002-final.json and generated/pass-002-final, analysis/pass-002-final-verification.json: ok=true,byte_identical=true. Map/state/checkpoint SHA25604757fd24b163af0a28eb86a6e072b7023fc4b9fb81642493b60ce7e9adc8f36. ROM SHA2562fd596ac4563b910a4d0a36d4a3658910fcfa822afd4f78bd62f73552e17ea00 unchanged. Previous map SHA2561235bbba785ad78213dfc49a39546d54d1f07fd228cdc14e4b403d4e75d91ece and commentary source SHA256516546b9a822d3be95a01fc28c173266326ebdde5607bc19f1928157fca6da5b preserved.

R001/R002/R004/R005 verified in this pass; R003 successful artificial final-phase encounters and MISSION24 CLEAR/TOKAKU victory already documented in redirect scenarios/snapshots/runtime-actions and reused without unchanged replay. Natural final boss/ending completion remains unproved. Prior failed stage/ending trials remain historical with localized scopes. Toolkit report analysis/toolkit-issues.md remains2 confirmed documentation/API inconsistencies,0 suspected,0 missing features,0 blockers; no new toolkit issues found and toolkit unchanged. RPC servers closed after bounded requests.

Next phase is full current-source commentary, migrating preserved commentary/source and frozen baseline into fresh generation. Newly classified code/data requires full readability coverage; old commentary success does not cover new source. Correct misleading result flag name and divider suffix physical comment. Redirect follows commentary and should review the exact12 singleton questions using evidence-policy.md before a terminal judgment. No archive/publish/deletion/cleanup or terminal exhaustion claim. Compact handoff: analysis/handoff-round-002-disassembly.md.

## Round002 commentary complete; redirect active

Separate commentary worker completed full pass-002-final readability gate; coordinator accepted after handoff, scope, source samples and hash inspection.1036 scaffold entries and719 semantic units cover29669 Code/48159 Data/53248 Image;12 unknown bytes have explicit uncertainty. Prior commentary preserved under analysis/round-001/commentary. Source SHAaf584d13044200e6a0139e6c23264e570c81980bfe13e07334a6d6f4b74d980a; reading copy SHA3ffb2ca04b2f0c1baa2595a416a0855a0784f6013b907421a305afec29b68eb7; fresh commentary/verification.json ok=true,byte_identical=true. No remaining readability gap; final editorial audit pending. Details: analysis/handoff-round-002-commentary.md, commentary/coverage-round002.json, analysis/commentary-round002-readability-gate.json. Map/state/checkpoint/frozen identities unchanged04757fd24b163af0a28eb86a6e072b7023fc4b9fb81642493b60ce7e9adc8f36. Classification99.99% (131076/131088;12 unknown), not complete.

Separate Round002 redirect worker is active, inspecting exact singleton role claims and prior rejection premises under evidence-policy. No terminal decision accepted. Phase workers remain sequential; source and structural classification are now frozen during research. No toolkit edit, archive, publish, deletion or cleanup.

## Round002 redirect — terminal-incomplete assessment for coordinator review

Unique attempt analysis/redirect/round002-20261030/ starts at accepted pass-002-final and full current-source commentary gate (source SHAaf584d13044200e6a0139e6c23264e570c81980bfe13e07334a6d6f4b74d980a). Fresh audit.py reads current internal map, ROM, neighboring regions, primary supported graph and raw pointer candidates; exact complement remains12 singleton bytes/131088, classified131076 (99.99%). No proposals/accepted annotations or new ready leads. All prior accepted leads and histories retained. No classification, map, state, source, ROM or toolkit edit.

Localized rejection-review.md independently challenges header-named bank6:$8796, redundant RTS suffixes, adjacent return entries and isolated$FF. Distinguishes exact supported header/data/command bank contracts: five$8796 header words are outside actual$8040 command directory, indexed$8008 consumer fixes X=0, fixed$8000/$8003 audio calls select bank5, background header consumers select bank4. Repeated return-shaped suffixes and known neighboring return contracts do not independently discriminate intentional stubs from filler/scalars; lack of entry alone is neither data proof nor a universal caller requirement. Generic-data fallback assessed on each exact singleton and lacks affirmative non-code pattern/consumer. No surrounding classified records rejected. Primary artifacts, competing interpretations, actual negative scopes and specific insufficiency are linked in review and residual-ledger.json.

Scenarios reviewed against prior exact runtime actions: natural stage1 plus artificially initialized final waves, ship/terrain, large-aircraft encounters and MISSION24 CLEAR/TOKAKU/victory cover representative later systems/terminal screen. Natural final defeat remains unproved; unchanged camera$0618 waits/special sequence have diminished prospects. No replay or arbitrary sweep. Full current-source semantic questions provide no concrete new residual role/consumer. No interrupted productive investigation remains; stop assessment is subject to coordinator rejection review, not ROM completion. Final editorial/fidelity acceptance remains separate. Editorial worker should clarify ambiguous positive-result wording at commentary/source/game.asm:20466–20467 using accurate StageRepeatRequested explanation/evaluator at20677; source left unchanged.

Handoff analysis/handoff-round-002-redirect.md. Preservation hashes freshly match ROM, internal/current/checkpoint maps and source. Existing byte-identical checkpoint/commentary verifications retained; no fresh build/runtime claim. Disk11GiB free; scratch156KiB; no archive/publish/deletion/cleanup. Toolkit issues remain2 confirmed documentation/API inconsistencies,0 suspected,0 missing features,0 blockers; no new toolkit issues found.

## Terminal handoff — justified incomplete, final acceptance verified

Separate final acceptance worker completed targeted editorial corrections, supported final audit and fresh byte-identical verification; reading copy refreshed.1036 regions/719 semantic units reviewed;1415 embedded audit-unit records. SourceSHA46a6ee4e604c016f48f6e05caf2de1b9faf38ef5648a492020cdd52929ba054d. Store and portable map freshly validate with configured binary. Coordinator independently inspected and accepted localized redirect rejection rationale; no ready leads or productive interrupted research remains. Classification remains99.99% (131076/131088;12unknown), explicitly INCOMPLETE. Workflow termination is not whole-ROM completion or blocked status. Full handoff analysis/terminal-handoff.md; final artifact/document accounting and completion audit linked there. No toolkit edit/rebuild, archive, publish, deletion or cleanup.
