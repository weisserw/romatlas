# Disassembly progress

Workspace initialized. Baseline and classification have not run.

## Round 001 disassembly — active

Initial baseline established with configured release binary. `python3 run.py inspect` confirms mapper 1, 131072-byte PRG and 131072-byte CHR, 262160 bytes total. `python3 run.py baseline` succeeded byte-identically; see `analysis/baseline-verification.json` and `generated/baseline`.

Live bounded work: `analysis/scratch/pass-v2.py` is driving reset/menu/start/directional+B scenarios, followed by three sourced pre-start stage-byte hypotheses. Selected evidence goes to `analysis/round-001`; `run-v2.log` preserves operation/frame results and API errors. Title is Arkista's Ring. Startup effective PRG slots [0,1,14,15], reset $E600 (physical $1E610); vectors inspected from physical $2000A..$2000F: NMI $F098, reset $E600, IRQ $F087. An initial wrong-offset discovery was rejected for unresolved context and did not annotate; an interrupted first route remains recorded in `run.log`. No classification saved yet; initial state/map still authoritative until final batch validation. No toolkit edits, input changes, deletion, archive or publishing performed.

Next: finish route and stage checks, inspect bank-aware CFG candidates, classify only supported instructions/CHR/header/vectors and checked uniform non-code ranges, then save/export and create fresh verified pass checkpoint. No exhaustion/completion claim.

Active correction: the tentative Arkista title identification was wrong (unrelated preexisting temporary PNG); actual uniquely converted workspace captures show Culture Brain intro/starfield. The erroneous external RAM hypotheses are superseded/rejected; patches do not demonstrate later-stage visits. See `analysis/gameplay.md`. Initial annotation attempt omitted required NES `id`, so API rejected all edits atomically; corrected API batch is now running with `id:0`. These were invocation mistakes, not toolkit bugs. Initial static vector roots were corrected to physical $1E610/$1F0A8/$1F097 before saved accepted CFG batch. Long intro is now advanced untraced with finite frame budgets; evidence remains resumable in `round-001/after-intro.snapshot` when saved.

Verified intermediate checkpoint: `pass-001` is now saved and byte-identical (`analysis/pass-001-verification.json`, `maps/pass-001.json`, `generated/pass-001`), map SHA256 a9d2d0ec9dd89cb834927d0e0e9cde4a8751c9fbf674b941d7e2164ac2656c05. Current map/state contain 2844 annotations. Actual title confirmed by `round-001/intro-more-2.ppm.png`: **Baseball Simulator 1.000**, Culture Brain ©1989. Earlier identification claims are superseded. TK-001 found: NES input requires `buttons` array, while shared docs use `button/pressed`; initial routes therefore exercised attract mode only. Corrected actual Start press now changes bank state and menu. Continuing bounded exhibition/other mode progression. See toolkit issue report.

Corrected-input exploration: title Start -> Exhibition/1Player -> Boston and Detroit -> Dome -> captured matchup's explicit Start prompt -> pitcher/lineup A confirmations. Selected captures: `round-001/team-choice.ppm.png`, `exhibition-2.ppm.png`, `pregame.ppm.png`, `pitcher-lineup-3.ppm.png` (PLAY BALL scoreboard). Corrected route is producing new contextual instruction roots; `actual-contexts-*.json` persists the first 4000 of 5811 at menu stage (remaining/final pages will follow). Finite batting attempt then release is in flight. Static inspection of the source-compatible strike handler establishes $A0 increments/compares 3, $A1 compares 4, $A2 increments/compares 2; practical shortcut hypothesis patches $A0=2,$A1=0,$A2=2 from clean pregame snapshot to seek a third-out/half-inning transition. This is artificial and no successful progression claim is made before frame/execution inspection. Ending/season contract still unresolved; other mode exploration remains useful.

Confirmed actual mechanics: `round-001/real-batting.ppm.png` shows live Boston batter against Detroit pitcher in top inning 1 after clean pregame snapshot + 300-frame initialization wait, then left+A for 90 frames. `edit-enter.ppm.png` confirms Edit's team-selection mode. Artificial counter patch capture shows batting field and two-out indicators; no successful half-inning change or ending was established. Saved final contextual roots include `artificial:true` in address contexts where appropriate; all accepted opcode fetches establish execution only in the indicated scenario, not natural player reachability. Bounded connected static analysis from these saved fetch roots is now merging a second code batch. Ending/season progression contract is a specific redirect question rather than a terminal claim.

Final merge interruption: the first per-instruction final classifier exited 143 before saving; pass-001 remained authoritative. All fetched contexts and CFG pages were already durable. Replacement merge reuses those exact artifacts, groups only contiguous complete instructions with identical effective views/provenance, and saves internal state plus current map every 20 accepted regions. No gameplay was replayed. `round-001/grouped-plan.json` records the full bounded frontier; `classify-grouped.log` records each persisted batch. Older no-longer-live writer ended before replacement start; idle inspection session closed.

## Round 001 disassembly — completed, ROM incomplete

ROM classification: 62.45% (163717 / 262160 bytes classified; 98443 bytes unknown).

Internal unique-byte accounting: Code 21,899; Data 10,746 (header, interrupt vectors, checked generic $FF runs); Image 131,072 (header-bounded CHR pattern ROM). Unknown is the exact complement, including gaps: `round-001/unknown-complement.json`. No explicit Unknown annotations or raw export fallbacks are counted. All 3,594 regions have evidence annotations. Structural classification, execution, semantic commentary and whole-game compatibility remain separate.

Current state `analysis/state.json` and `maps/current.json` validated. Fresh final checkpoint `maps/round-001-disassembly.json` / `generated/round-001-disassembly` verified byte-identical: `analysis/round-001-disassembly-verification.json`; map SHA256 `38c23b1d127cbeb734575c70ff102751d98f0811575fb8ae7876bbf701ce6bae`. Input SHA256 rechecked unchanged: `eeebc807e4a53336b871b4195eef25df0c80a558156742c3411a83225d4f909b`. Baseline and intermediate checkpoints preserved.

Execution evidence: 13,575,653 accepted Instruction events; 11,206 unique instruction keys, 13,983 contextual roots, 15,465 contextual edges, aggregate_dropped=0. Capacity zero intentionally retained no detail and dropped 118,487,372 detail events; bounded aggregate contexts were saved completely across final-contexts and tail-contexts JSON pages. Every saved ROM opcode root starts in classified Code (zero unclassified observed starts). Natural attract mode, corrected actual player routes and artificial counter observations are distinguished in scenarios.json and contextual `artificial` flags. Trace counters include branches/restores and are not a single forward-cycle timeline.

Pass stopping rationale: startup/vector discovery, a substantial bank-aware observed/connected code batch, generic uniform data, live Exhibition batting and representative Edit menus were exercised. Long manual innings are not repeated. The supported counter shortcut failed to establish half-inning/ending, so the needed inning/side dispatch contract and short-season AUTO route are concrete redirect work (L002), not terminal exhaustion. Inline bank-call structure L001 and PRG data L003 remain actionable. No initial ready leads existed; newly grounded leads are handed off for redirect.

Toolkit: one confirmed protocol/documentation input-schema mismatch, TK-001, with working full-buttons workaround; no blocking toolkit issue. See [toolkit-issues.md](toolkit-issues.md). Toolkit commit b9ae425dd6546741f2ec41e36bda9543910287de; no edits/builds. Disk at handoff: 6.4 GiB free, round evidence about 90 MiB, reusable scratch 48 KiB. No cleanup/deletion performed.

| Bank | Code | Data/image | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG 00 | 2068 | 0 | 6124 |
| PRG 01 | 0 | 663 | 7529 |
| PRG 02 | 3951 | 0 | 4241 |
| PRG 03 | 1229 | 149 | 6814 |
| PRG 04 | 1518 | 0 | 6674 |
| PRG 05 | 0 | 7833 | 359 |
| PRG 06 | 1607 | 247 | 6338 |
| PRG 07 | 343 | 978 | 6871 |
| PRG 08 | 0 | 1 | 8191 |
| PRG 09 | 0 | 0 | 8192 |
| PRG 10 | 900 | 0 | 7292 |
| PRG 11 | 0 | 0 | 8192 |
| PRG 12 | 2530 | 0 | 5662 |
| PRG 13 | 1211 | 0 | 6981 |
| PRG 14 | 2166 | 0 | 6026 |
| PRG 15 | 4376 | 859 | 2957 |
| CHR 00 | 0 | 8192 | 0 |
| CHR 01 | 0 | 8192 | 0 |
| CHR 02 | 0 | 8192 | 0 |
| CHR 03 | 0 | 8192 | 0 |
| CHR 04 | 0 | 8192 | 0 |
| CHR 05 | 0 | 8192 | 0 |
| CHR 06 | 0 | 8192 | 0 |
| CHR 07 | 0 | 8192 | 0 |
| CHR 08 | 0 | 8192 | 0 |
| CHR 09 | 0 | 8192 | 0 |
| CHR 10 | 0 | 8192 | 0 |
| CHR 11 | 0 | 8192 | 0 |
| CHR 12 | 0 | 8192 | 0 |
| CHR 13 | 0 | 8192 | 0 |
| CHR 14 | 0 | 8192 | 0 |
| CHR 15 | 0 | 8192 | 0 |


## Coordinator continuation — round 001 commentary active

Previous coordinator turn made progress: disassembly handoff accepted after current count/verification inspection, and a separate commentary worker started. Worker `/root/round1_commentary` was authoritatively confirmed running on continuation; no replacement or competing phase writer started. Current checkpoint remains `maps/round-001-disassembly.json` (38c23b1d127cbeb734575c70ff102751d98f0811575fb8ae7876bbf701ce6bae), classification 163717/262160 with 98443 unknown. The commentary baseline is prepared and latest `commentary/verification.json` confirms byte identity after audio-source edits. Audio physical $000018..$00105c is worker-reported reviewed; remaining classified banks and boundary inventory remain pending. No commentary completion accepted; redirect has not started. Continue this same worker/phase from its durable source and edit plan.

Coordinator inspected archive.py selection and editorial contracts without invoking archive. Provisional `archive-options.json` and `analysis/archive-document-selection.json` account for all current Markdown (zero unaccounted); preserve additional handoff, per-bank table and contract notes. No scratch Markdown currently exists to omit. Refresh selection after all final phase documents are written. No terminal handoff or goal completion claimed.

## Round 001 commentary — active, incomplete

Prepared `commentary/` from the frozen disassembly map (SHA256 `38c23b1d127cbeb734575c70ff102751d98f0811575fb8ae7876bbf701ce6bae`); initial preparation rebuilt byte-identically. Classification remains 62.45%, with 21,899 code bytes; no map/state edits. Current source has 3,582 code regions and 3,959 total inventory entries.

Read and annotated bank-zero audio code (physical `$000018..$00105c`, 2,068 classified code bytes) and fixed runtime banks (physical `$01c010..$020010`, 6,542 classified code bytes), including object dispatch, inline-argument helpers, PPU queue/streaming, palettes/metasprites, controller sampling, NMI critical-depth handling, MMC1 serialization, RAM access and positional checksum. Source comments explain observed mechanics; uncertain game roles and unclassified continuations remain explicit. Shared hardware/input/mapper symbols are also substituted in pending switchable-bank code. Detailed edit plans are local in `analysis/round-001-commentary/`; these scripts are one-shot historical edits, not rerunnable regeneration.

Fresh verification after the fixed-runtime batch passed byte-identically in `commentary/verification.json`; code source hash `f552894c9c2acce5ed7fde9617821afe27d479612cec970a59354a8b22f037ba`, symbol include hash `e43a1b88f135d70e7acdf4975d169697123b3d2da37b7249e911293b4f5771f8`. Frontier: review/annotate all classified switchable-bank code in physical `$004010..$01c010` (13,289 code bytes); finish source raw-boundary/data/CHR notes and inventory credit, verify again and refresh reading copy. No successful commentary handoff or redirect readiness claimed. Final editorial audit remains pending.

Commentary switchable-bank batch now saved: the entire remaining classified instruction inventory was inspected (13,289 code bytes, physical `$004010..$01c010`) and received 609 functional/control-flow comments plus scoped menu/presentation/game symbols. Fresh rebuild is byte-identical (`commentary/verification.json`, game.asm SHA256 `b2b455d7ac108b1018202cb09537d08b9e72385e855bc58ac8c04d4cef335b5a`). Header's misleading unclassified-audio note was removed. Current frontier is semantic consistency/gap check, raw/data/CHR source notes, compact units/inventory credit and final phase verification/reading refresh; phase still active/incomplete. Structural redirect question: CPU `$8ab3..$8abe` in physical bank pair 12/13 has explicit LDA table consumers, yet the current Code region includes its decoded ASL/PHP/NOP forms. Preserve current map pending bounded structural follow-up.

## Round 001 commentary — current-source readability complete

All currently classified source is now inspected and explained: Code 21,899, Data 10,746, CHR 131,072; all Unknown boundaries (98,443 bytes) have honest notes. Full 3,959-entry review scaffold retained; 267 meaningful semantic units, 1,159 renamed entry labels, 436 named numeric branch targets and 67 additional internal routine explanations. Source covers audio, UI/save/team setup, text/number rendering, presentation objects, role/automatic controls, match/batter/pitcher state and fixed hardware/runtime. Remaining original meanings are explicit, not deferred readability work.

Fresh final verification passes byte-identically: `analysis/round-001-commentary-verification.json` (also `commentary/verification.json`); game.asm SHA256 `0dbb4dcbb524a2935a45a14db7c99f6b933660942c9d9a45128116baa6450d57`, symbols SHA256 `01d0477718faffa94102931285af869c925f69fd3af600bd65841308692a4aa1`. Reading copy refreshed at `commentary/reading-after.asm`. Input hash unchanged; phase-1 state byte-identical to frozen commentary state; current map hash still `38c23b1d127cbeb734575c70ff102751d98f0811575fb8ae7876bbf701ce6bae`.

See `analysis/round-001-commentary-handoff.md` and `commentary/uncertainties.md`. Redirect: C001 table/code conflict at physical `$018ac3..$018acf` in banks `[12,13,14,15]`; C002 corrects L001's inline contracts (`$c03f` is one-byte audio, `$e672` is two-byte banked selector/PRG). No map changes, runtime coverage claims, toolkit edits, cleanup, archive, deletion or publishing. ROM still 62.45% classified; final editorial audit remains pending and was not invoked.

## Coordinator gate — round 001 commentary accepted; redirect active

Separate commentary worker completed all current-source readability and stopped. Coordinator inspected compact handoff/uncertainties, sampled source, and verified current source hashes against fresh byte-identical report, exact reading-copy freshness, frozen/current state+map identity, all 3959 scaffold entries reviewed and 267 semantic units. See `analysis/round-001-commentary-coordinator-check.json`; no claim that metadata alone proves prose quality. Final editorial audit remains pending. Separate `/root/round1_redirect` launched with only redirect prompt and compact checkpoint/handoffs. C002 corrects L001's conflated inline-call contract; C001 isolates twelve apparent table bytes currently Code for structural review. L002 progression and L003 data avenues remain actionable. Classification remains 62.45% (163717/262160, 98443 unknown). No terminal assessment accepted.

## Round 001 redirect — active attempt r001-static-contracts

Current-source commentary gate accepted. Attempt `analysis/redirect/r001-static-contracts/` preserves exact unchanged map complement: 365 intervals / 98,443 bytes, with physical 8 KiB banks, CPU aliases and neighbors (`unknown-ledger.json`). Initial `analysis.decode` request used the wrong context nesting and failed read-only; corrected API is top-level cpu_address plus context.prg_banks, demonstrated by audio-neighborhood.json. All map/state/source remain unchanged.

Corrected L001 premise: $C03F/$D0DE consumes one audio byte and resumes after it; $E672/$EFDA pops the wrapper's return, switches PRG, dispatches through $8000+2*selector, restores bank and returns to the wrapper's caller (no sequential fallthrough to the next wrapper). Static inventory now decodes every classified instruction using the configured toolkit's NMOS mode/length table; initial region-start-only inventory was superseded in-place while attempt active. Found 35 banked wrappers, 25 inline jump dispatch calls, 4 audio calls and 12 other fixed inline argument calls. `banked-targets.json` resolves 32 distinct handler entries with physical contexts; bounded 2,000-instruction static-only discoveries retained per handler. Next: inspect supported handler continuations/argument barriers, C001 incoming roots/domain, independent generic data neighborhoods and L002 progression contracts/runtime shortcuts. No classification proposals accepted yet; this is an active frontier, not exhaustion.

Redirect continuation: bounded static inventory now includes the C000/D179 two-byte record-pointer helper; failing to skip it produced two local invalid-entry artifacts, which localized the missing barrier and are retained in `menu-invalid.json` / `match-invalid.json`. Correcting that documented contract removes both invalid paths. All 25 currently classified inline jump consumers were inspected; 22 have exact consumer/pointer/first-handler frontiers, while mixed bank4 selector areas and fixed D148's sparse two-way table need narrower review. Candidate table bounds are hypotheses pending disassembly; no map annotation occurred. Following all supported handler continuations, rather than only the first ready entry, yields bank-specific CFG evidence in `connected-cfg-bank*.json`, with matched physical contexts and explicit inline barriers.

L002 runtime improvement: from clean real-batting.snapshot, patch $A2=3 and $AD=$88; 180 bounded frames produced phases 4 -> 6 -> 1 and changed $8E/$8F from 0/0 to $20/1 while $8D stayed 1. `half-inning-5.png` shows the between-inning scoreboard; saved `half-inning.snapshot`. Opcode coverage is artificial scenario evidence, not natural reachability. Initial 120-frame RPC timed out; one-shot script raised/exited (not an ongoing worker). Its log was preserved separately. Corrected runner retains RPC errors/state and uses 30-frame chunks. Subsequent clean-bottom snapshot shortcut sets $8D=9 and scores $0694/$0695=0/1; bottom-nine snapshot exported after genuine phase3 field initialization. A first outcome patch there settled in phase4 with $AD=$80 (low outcome overwritten), so terminal result is not yet established. Next: supported phase4 low-outcome completion patch, then inspect actual terminal logic; investigate Season AUTO route after match progression. Independent data lead: bank4-selected text table at physical $010030..$010096 has 51 pointers and perfectly terminated/aliased strings through $0101A6 under the $E85C consumer. Research and proposals remain active; no exhaustion claim.

Confirmed match terminal result: `terminal-finish.png` visibly says **GAME SET**. `bottom-nine.png` shows an initialized **9INBOT** field. Route: import half-inning.snapshot; preserve bottom-side $8E/$8F=$20/1, patch inning $8D=9 and scores $0694/$0695=0/1; advance 210 frames in 30-frame chunks until phase3 and save bottom-nine.snapshot. Initial $A2=3/$AD=$88 settled in phase4 because object updates rewrote outcome low bits. Import that terminal-attempt.snapshot, use bounded `emulator.run_until` (50,000 operation / 25s ceiling) for exact PC $85CA after the object update; matched in 6,337 operations with banks [12,13,14,15], stack $F0. Patch only $AD=$88 there, retaining legitimate stack; advance 300 frames in 30-frame chunks. Observed phase6 -> phase1, inning10/top side and signed match state $8B=$81, followed by GAME SET. `terminal-finish-runtime.json` retains exact calls/states and accepted artificial opcode fetch counts including physical $0185DA/$0185E8, side reset $0187F3 and inning increment $0187F9 (one each). No natural reachability or credits claim. Snapshots/captures retained.

Independent data investigation has productive container proof: OAM consumer $ED99..$EE6F temporarily selects PRG4 (effective [8,9,14,15]), uses set pointers at $8200 and packed frame pointers (bit6 horizontal flip, low14 target offset), then consumes count-prefixed four-byte components/redirects. `sprite-container.json` independently resolves 26 set-pointer cells, 501 packed frame-pointer cells at physical $010250..$01063A, 192 unique selected component records and 5,736 consumed bytes in nine intervals through $011E31. Every intervening gap independently partitions exactly into the same 1+4*n record format (including 21/22-component adjacent records), so no arbitrary semantic or runtime gate is needed. Candidate full component container and nearby uniform FB/FF filler will be packaged for review; generic fallback remains distinct from proven sprite/text layouts. Bank9's tile-like/repeated data and RTS run remain under practical consumer/boundary review. Season ordinary route reached Season->PLAY submenu; subsequent A confirmation exported season-play.snapshot and is ready for inspection. Map/state/source unchanged; ready batch still under construction, no terminal exhaustion claim.

Redirect active frontier update: combined static proposal now **39,218 unique unknown bytes** = 20,967 Code + 18,251 Data, plus exact C001 12-byte Code->Data correction (no classification-count gain). `proposal-batch.json` enumerates every candidate instruction/view/incoming path and consumer-sized/generic Data claim; these are proposals only. Residual if all accepted would be59,225; actual unchanged residual remains98,443. Configured binary independently decoded351 grouped runs with zero length/opcode faults. Exact graph audit has no Code/Data intersections and no overlaps into existing classified bytes. Four overlapping candidate operand bytes are intentional documented BIT-skip/LDA alternate entry interpretations at physical $01D0CA..$01D0D0, not conflicting code-versus-data claims; export choice still to document before handoff.

Practical generic-data review now also covers27 exact uniform00/FF runs outside the sprite/tile bank pair (498 bytes), after nearby/consumer-base checks and comparing all code proposals. No direct base reference established a stronger semantic role; generic Static Data is sufficient, with no fill/unused claim. `uniform-fallback-review.json` retains bounds, neighboring bytes and potential nearby indexed consumers. Bank9 profile supports generic data across its full8KiB, including the formerly ambiguous328-byte RTS run; repetitive four-tile/low-id layouts, attribute-like00/40/80/C0 runs and FB/FF suffix provide positive non-code structure. The four short RTS runs in the presentation bank remain ambiguous stubs versus data and are not proposed.

Conventional-return review was a materially improved static method: accepted caller fragments ending in ordinary JSR now supply explicit same-effective-bank return preconditions, exposing additional menu/presentation continuations. All newly found inline jump consumers were inspected and their coherent handlers followed. A local BNE from incremented queue cursor had a hypothetical wraparound fallthrough into the five-byte tile lookup $0083B8..$0083BD; actual indexed consumer and independent next routine bound that Data fragment, so graph now has an explicit barrier. No unsupported queue-overflow code claim remains. All currently found dispatch continuations are packaged together rather than artificially split into later rounds.

Season ordinary-input progression: Season->PLAY->Atlantic League->stadium confirmation->five-game schedule->team MANUAL/AUTO screen reached, with unique snapshots/captures. `season-auto-teams.snapshot` follows right/A/down across each of six team rows and ends at END; `season-auto-start` A confirmation is being inspected to determine whether selections entered AUTO simulation. All buttons use full arrays; no Season RAM patches yet. Match GAME SET remains confirmed. A separate bounded A-pitch route from initialized bottom-nine snapshot is also captured for representative pitching mechanics inspection. No gameplay or research exhaustion claim.

Further redirect progress (actual, not terminal): four short presentation-bank $60 runs are now resolved by an independent bounded format check, replacing their prior ambiguous disposition. Physical $00CE41..$00CE97 contains two FF-terminated structures, each three PPU-address/count/payload rows with exact eleven-byte payloads; middle rows encode GAME SET and PLAY BALL. $60 is the blank tile payload. `presentation-text-records.json` records exact rows/bounds. Combined proposal before new audio findings is39,304 unknown bytes (20,967 Code /18,337 Data), plus C001's12-byte correction; no bytes accepted yet.

Representative pitching is verified visually: `bottom-nine-pitch.png` shows the pitcher in delivery and the ball travelling, after A30/release60 from the initialized9INBOT snapshot. Season route has advanced through all six visibly AUTO teams and the ordinary SEASON OPENER scoreboard. A300-frame no-input wait correctly found it awaiting confirmation; Start then passed through setup. Four A3/release30 confirmations revealed the actual additional choice: TOP AUTO, BOT AUTO, and GAME WATCH/SKIP, with explicit PUSH START BUTTON. `season-pair-auto.png` documents this. Next bounded test selects SKIP then Start, which is a materially better simulation method than merely waiting on opener/watch setup. No season results yet claimed.

New independent audio avenue: existing consumer inventory anchors song four-channel-list, effect, envelope and noise pointers. `audio-data-containers.json` now checks85 song-list pointer cells and85 exact eight-byte four-channel records (680 bytes), all channel pointers either ROM or zero inactive slots. Song id0 is ignored; its apparent base cell$1580 was correctly excluded, and effect index0 aliases the final song-list pointer. Table/record boundary and null-channel checks corrected an initial overly strict nonzero-pointer diagnostic. Four channel consumers, pointer tables and whole upper sequence-region byte profiles are saved. Upper region physical$0010A7..$003D79 consists of packed request descriptors and repetitive note/duration/control or fixed-parameter streams, followed by the already-established FF tail; no direct accepted CPU control edge enters it. This is positive consumer/pattern evidence for generic Data, separate from exact per-song layouts. Final audio proposals and alternate-entry export audit are being packaged; phase remains active.

Redirect graph closure correction: the earlier banked-target inventory included only initially classified wrappers. Newly supported wrapper entries revealed by connected handlers also need target resolution in this same pass. `expand_calls.py` found42 then11 additional banked wrappers plus new dispatches; `banked-targets.json` and configured-toolkit target discoveries now cover all current wrapper-selected targets. Following them materially expands menu, presentation and save/season helpers, rather than deferring their connected callbacks. Latest audit is65,929 proposed unknown bytes (34,612 Code /31,317 Data), with zero genuine code/data or existing-map overlaps and only documented BIT alternate entries. Three final new wrappers have just been followed, with no further inline dispatch/banked calls discovered; totals/decoder validation will refresh before handoff. Earlier52,071-byte snapshot remains a prior frontier, not accepted classification.

Audio container proposals are included: exact pointer/record cells, generic lower envelope/noise neighborhood,128-byte packed request prefix, and generic repetitive upper audio-sequence/parameter region through known FF-tail boundary. Positive consumer/format profile evidence is in audio-data-containers.json; no arbitrary per-song/command bounds or reachability claim. C001 remains exact twelve bytes only. All map/state/source/ROM hashes checked unchanged.

Season actual latest observation: TOP/BOT AUTO and WATCH/SKIP choice -> selected SKIP/Start -> starting-pitcher choices -> four A confirmations -> **PLAY BALL** scoreboard (`season-starting-lineup.png`). Simulation/season results still unclaimed. Newly followed menu helper graph identifies an actual persistent season progression writer: INC CPU$6008 at bank1 CPU$B9D4 (physical$0079E4); existing match control compares $6007/$6008. This supplies a concrete shortcut contract to inspect after first automatic-game transition, rather than patching the misleading/inadequately proven $069E/$069F names. API memory.write cannot mutate PRG-RAM directly; if needed an instruction-boundary edit must use the real writer with its bank/access/stack contract, not invented support.

Latest graph frontier: after all previously found banked wrappers were followed, no further banked wrapper calls remain unvisited. A further concrete bank-context review follows fixed continuations after explicit DesiredPrgSelection=4 or5 applications: fixed $C281/$C35E under [10,11,14,15], and $ED88/$C74B/$C5E0 under [8,9,14,15]. This exposes remaining AI/graphics helpers while preserving fixed-bank identities and stops at unresolved restoration calls. The previous audit before these additions was67,094 proposed bytes (35,727Code /31,367Data), decoder443 runs passed. Newly found AI dispatch tables and their finite/masked selector paths are being bounded/followed; final audit/decoder will refresh. No map changes. Current data barriers and intentional BIT overlap remain intact.

Season shortcut negative results are retained, not hidden: true stored season length is165 at PRG-RAM$6007, despite the earlier cursor displaying5GAME; Start had not committed that choice, so no five-game initialization claim. Direct late $87=1 patch during pending half-inning presentation stalled because timers stopped at1. Clearing the supported $84/$85 timer pair advanced to phases4/5, but did not establish a complete automatic game; initialization dependencies make late suppression inferior to committing SKIP before setup. Pre-setting UI index$056E was overwritten by selector updates and still produced $87=0. The discriminating next method intercepts the actual LDA$056E/STA$87 commit at bank1 CPU$9BD2, preserving normal stack and dependent setup; its bounded run_until result is being inspected. These are artificial routes; ordinary AUTO selections/PLAY BALL and separate match GAME SET/pitching captures remain valid. No Season terminal result claimed.

Current checkpoint before final callback-table closure: **71,115 proposed unknown bytes** =39,340Code +31,775Data; unchanged actual unknown98,443, conditional residual27,328. Configured decoder validates479 grouped runs with zero faults. Coherent AI-context continuation now includes finite type/event tables, separately addressed adjacent threshold/mask byte tables and the111-cell APU timer/noise lookup (95 descending timers ending0, then16$008F..$0080 values). A complementary BCS/BCC pair, previously explored as an impossible fallthrough into a small threshold table, is now modeled by carry logic; no invalid or unofficial-opcode paths remain. All dispatch cell bounds use independently targeted code/data frontiers or explicit bounded producers, not solely pointer values. `alternate-entry-audit.json` gives exact canonical physical$01D0C8..$01D0D0 /CPU$D0B8 Code and alternate roots$D0BB/$D0BE; earlier approximate D0CA summary is superseded by this exact artifact.

Season current negative outcome: run_until for commit CPU$9BD2 did **not** match under either Start or A from season-pair-auto.snapshot (100,000-operation bounds, ten emulated frames each; no supervisory timeout replacement). Pre-setting$056E was overwritten; late$87 patch plus timer fix reached game phases4/5 but no persistent season progress ($6008/$6009 stayed0/0 over bounded captures). Missing prerequisite is the active widget/port/action contract that actually reaches SKIP commit before initialization; direct flag edits bypass dependencies. No invented PRG-RAM write API, register-PC writer loop or arbitrary PRG-RAM alteration was used. Ordinary AUTO menus, initialized PLAY BALL, separate9INBOT pitching and artificial match GAME SET remain verified. Season terminal is unvisited; further unchanged waits or button sweeps have diminished prospect. Preserve actual165 stored length and all negative artifacts; do not claim five-game simulation.

Final concrete structural continuation now identified from the entire independently bounded38-entry five-byte object-descriptor table ($01C0D9..$01C197): eight fixed callback entries remain unreviewed, plus several context-dependent low-bank callback vectors. Endpoint is the independently rooted C187 wrapper, all records are mechanically consistent. Next bounded step follows all eight fixed callbacks under the game-object bank6 entry contract and their directly supported state tables. Low-bank callbacks lacking an effective selection contract stay unresolved locally rather than inventing a physical bank. This is productive same-phase research, not a terminal or empty-cycle handoff.

## Coordinator continuation checkpoint — redirect still active

At 2026-11-14 00:56 UTC, `/root/round1_redirect` is authoritatively running; previous continuation was a verified wait, not a blocker. Current `analysis/redirect/r001-static-contracts/proposal-batch.json` reports 71,115 unique proposed unknown bytes (39,340 Code /31,775 Data), leaving 27,328 only if independently accepted. Actual map remains at round-001 disassembly classification, 163,717/262,160 (62.45%) with98,443 unknown. Proposal totals are not accepted classification. Configured decoder validation has been refreshed in the same attempt. Current live frontier remains bank-context/dispatch-boundary review, final combined evidence packaging, and the bounded Season commit-path investigation. No completed redirect handoff or terminal rationale exists yet. Continue this same worker/attempt; no replacement writer started.

## Round 001 redirect — complete ready combined batch, nonterminal

Frozen attempt `analysis/redirect/r001-static-contracts/`; handoff `analysis/round-001-redirect-handoff.md`. Final master proposal: **83,752 unique unknown bytes** =51,013 Static Code +32,739 Static Data, plus exact12-byte C001 Code->Data correction. **Actual map unchanged:**163,717 classified /98,443 unknown. Conditional residual14,691 only if all proposals independently accepted. New ready leads L004/L005/L006, updated L001/L003 ready; L002 verified match progression with explicit unsuccessful Season remainder. Lead scopes overlap; count only master union.

All concrete wrapper/service continuations closed as a coherent graph:94 wrappers,89 distinct banked targets,68 bounded inline dispatches, nine matched PRG5 service-vector pointer/JMP pairs. This includes all three previously unentered A0B5/A169/ABCD service bodies, their player/fielding/AI state handlers and direct shared helpers. All eight additional fixed object callbacks were followed; low-bank object edges retain unresolved effective-selection contracts rather than guessed banks. Final configured binary check:559 grouped decoder runs, zero length/opcode faults, no invalid/unofficial paths and no genuine Code/Data/existing-map overlap. Intentional BIT/LDA operand overlap has exact canonical1D0C8..1D0D0 Code/export contract. Data includes the independently bounded38x5 descriptor table, consumer-linked audio/sprite/text containers, structured UI/PPU records and reviewed generic patterns. Exact counts, bank totals, evidence index and conditional residual spans in batch-summary.json; full per-byte claims/views/incoming paths in proposal-batch.json.

Season negative outcome is unchanged and preserved: no Season terminal, stored length165, missed actual SKIP commit, no season counter progress under late suppression/timer shortcuts, A/Start9BD2 bounded intercepts unmatched. Further unchanged sweeps have reduced prospect; missing active widget/port/action/parent-transition contract is explicit. Actual match GAME SET,9INBOT initialization and pitching delivery remain supported, with artificial-state provenance separated from ordinary input routes. No arbitrary PRG-RAM mutation or writer-PC loop was used.

Exact365 current unknown intervals retain bank-split physical parts and proposed-only dispositions in unknown-ledger.json; no raw-gap inference or exhaustive survey/exhaustion claim. All future changed-map work uses a new attempt; completed attempt/evidence and all prior records stay immutable. ROM, map/state and maintained source hashes rechecked unchanged; no cleanup/archive/publication/toolkit changes. No pending runtime experiment. Ready for independent disassembly of the combined batch, full current-source commentary, then redirect; final editorial acceptance still pending.

## Coordinator gate — redirect accepted for round002 independent disassembly

Inspected complete round001 redirect handoff, batch summary/deferred reasons and earlier decisive C001/alternate-entry evidence. `analysis/round-001-redirect-coordinator-check.json` confirms all frozen input hashes still current. Coherent proposal83752 unknown bytes (51013Code/32739Data) plus exact12-byte correction is READY, NOT accepted classification.559 configured-decoder runs are supporting checks, not semantic proof. Separate `/root/round2_disassembly` launched with only its phase prompt and compact handoffs; redirect worker finished, no competing writer. Frozen attempt remains immutable.

Coordinator scope correction: absence of a ROM-structure question alone does not justify skipping representative Ultra mechanics/manual fielding. These remain best-effort gameplay scope for round002; worker instructed to try a practical bounded route/state shortcut and retain actual outcomes/obstacles. This is a nonterminal handoff; Season negative results need not be replayed unchanged. Actual map remains163717/262160 (62.45%) with98443 unknown until independent annotations and fresh verification.

## Round 002 disassembly — active review and gameplay

Resumed exact round-001 checkpoint (SHA25638c23b1d127cbeb734575c70ff102751d98f0811575fb8ae7876bbf701ce6bae); no baseline reinitialization. Frozen redirect evidence retained. Independent raw-byte/effective-bank/incoming-source audit passes for full83,752-byte union, 94 banked wrappers, nine paired service vectors and68 dispatches; independent sprite redirect/gap replay covers203 records exactly. See analysis/round-002-disassembly/independent-review.json. Structural evidence review and annotation are ongoing; actual classification still62.45% until mutations accepted. C001 and BIT/LDA canonical segment remain exact planned exceptions.

Ordinary Ultra route currently reached team and stadium selection using full-buttons arrays, with captures and persistent snapshots in analysis/round-002-disassembly; actual Ultra team initialization/mechanics not yet established from that selection alone. Next bounded steps: verify matchup/live state, try B special hitting and double-Up special pitching, and representative manual field view/movement from initialized pitching snapshot. Prior failed Season experiments will not be repeated unchanged. Nonterminal pass, no exhaustion claim.

Round002 acceptance saved: **83,752 newly classified bytes**,51,013 Code and32,739 Data, plus exact C00112-byte Code->Data correction. `map.annotate` completed1,290 logged RPC operations including load/remove/prefix preservation/validation/save/export; analysis/round-002-disassembly/annotation-log.json records exact changes. Static provenance retained and canonical alternate entries explained in map comment. Commentary source untouched. Fresh checkpoint round-002-disassembly is generating/verifying while bounded gameplay experiments finish.

ROM classification: 94.40% (247469 / 262160 bytes classified; 14691 bytes unknown).

Actual Ultra-team AL-SW/Super matchup and PLAY BALL initialization captured (`ultra-pregame.ppm.png`, `ultra-live.ppm.png`), ordinary full-buttons route. B activation in live batting and B manual-field-view/movement/throw from artificial9INBOT are currently running under bounded trace; mechanics results pending capture inspection. No successful special hit/contact or fielding claim yet.


## Round002 disassembly — complete single pass; incomplete ROM, nonterminal

ROM classification: 94.41% (247503 / 262160 bytes classified; 14657 bytes unknown).

Accepted gain83,786 bytes: frozen master union83,752 (51,013 Static Code /32,739 Static Data), plus34 Observed Code bytes from15 accepted artificial opcode fetches during initialized9INBOT->Start/A defense-menu route. Counts: Code72,934; Data43,497; Image131,072. Exact C00112-byte correction changes role only. Structural review and independent audit: analysis/round-002-disassembly/structural-review.md, independent-review.json, annotation-log.json; new relative-period/audio APU branch physical0676..0698 has configured decoder and observed-annotation-log.json. Static acceptance is distinct from execution reachability; new audio fetches are explicitly artificial.

Final map/state validated and exported. Fresh verified checkpoint **round-002-disassembly-final** (generated/round-002-disassembly-final; maps/round-002-disassembly-final.json), verification analysis/round-002-disassembly-final-verification.json: ok=true, byte_identical=true. Map SHA2567dd308535281fec582e2294a84bb62b73b1d9cd258bf94823b9434f84a8bd0f4; immutable ROM SHA256eeebc807e4a53336b871b4195eef25df0c80a558156742c3411a83225d4f909b. Earlier successful round-002-disassembly checkpoint remains historical. Maintained commentary source unchanged and requires current-map carry-forward in next phase.

Leads L001/L003/L004/L005/L006 verified as the overlapping coherent supported batch, not summed separately. L002 match progression remains verified with prior Season limits. Natural Ultra team initialization and special pitching now visited (S009): icon selection/delivery and3-point consumption confirmed; no successful special hit claimed. Artificial defensive CHANGE/MOVE interface exercised (S008), B exit/12 released frames returned to live9INBOT; no exact reposition/live-contact catch claim. B-only field-view and mistimed Ultra half-inning patch negatives preserved; do not retry unchanged. Season stored165/failed SKIP-init contract remains unchanged. Runtime detail capacity0 intentionally drops detail, aggregate_dropped=0; accepted contextual summaries retained. Two early over-limit coverage queries were corrected to paginated limit1000; one bounded step timeout retained partial state, not claimed complete.

Active frontier:249 exact unknown intervals totaling14,657 bytes in round-002-disassembly/unknown-frontier.json; per-bank counts below/bank-counts.json. Largest mixed residuals include physical09506..0A010,0D62B..0DF19,0F810..0FDDD and fixed-bank fragments. No blanket gap classification. Low-bank object callbacks still need installing caller's effective selection; other mixed fragments need local entry/format bounds. Next commentary must carry forward current code/data roles and BIT alternate symbols D0BB/D0BE; then redirect may inspect concrete residual contracts. No terminal exhaustion or whole-ROM/readability completion claim. Current best-effort mechanics gaps are documented rather than converted to classification blockers.

Toolkit: existing confirmed input schema bug TK-001, workaround full-buttons arrays; no new toolkit issue. Preserve analysis/toolkit-issues.md. No toolkit edits, ROM edits, source replacement, cleanup/archive/deletion/publication.

| Bank | Code | Data/image | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG 00 | 2229 | 5588 | 375 |
| PRG 01 | 0 | 8192 | 0 |
| PRG 02 | 7692 | 238 | 262 |
| PRG 03 | 7339 | 211 | 642 |
| PRG 04 | 4144 | 306 | 3742 |
| PRG 05 | 0 | 8192 | 0 |
| PRG 06 | 4424 | 537 | 3231 |
| PRG 07 | 5546 | 982 | 1664 |
| PRG 08 | 0 | 8192 | 0 |
| PRG 09 | 0 | 8192 | 0 |
| PRG 10 | 7566 | 335 | 291 |
| PRG 11 | 7588 | 313 | 291 |
| PRG 12 | 7689 | 233 | 270 |
| PRG 13 | 6625 | 477 | 1090 |
| PRG 14 | 6955 | 565 | 672 |
| PRG 15 | 5137 | 928 | 2127 |
| CHR 00 | 0 | 8192 | 0 |
| CHR 01 | 0 | 8192 | 0 |
| CHR 02 | 0 | 8192 | 0 |
| CHR 03 | 0 | 8192 | 0 |
| CHR 04 | 0 | 8192 | 0 |
| CHR 05 | 0 | 8192 | 0 |
| CHR 06 | 0 | 8192 | 0 |
| CHR 07 | 0 | 8192 | 0 |
| CHR 08 | 0 | 8192 | 0 |
| CHR 09 | 0 | 8192 | 0 |
| CHR 10 | 0 | 8192 | 0 |
| CHR 11 | 0 | 8192 | 0 |
| CHR 12 | 0 | 8192 | 0 |
| CHR 13 | 0 | 8192 | 0 |
| CHR 14 | 0 | 8192 | 0 |
| CHR 15 | 0 | 8192 | 0 |

## Coordinator gate — round002 disassembly accepted; commentary active

ROM classification: 94.41% (247503 / 262160 bytes classified; 14657 bytes unknown). Exact final percentage emitted to user before commentary launch. Worker completed and stopped. Inspected final handoff and fresh byte-identical verification; current input hash, current/checkpoint map equality, checkpoint hash and state/map equality independently confirmed in `analysis/round-002-disassembly-coordinator-check.json`. Final map7dd308535281fec582e2294a84bb62b73b1d9cd258bf94823b9434f84a8bd0f4.

Separate `/root/round2_commentary` launched with only commentary prompt and compact checkpoints. Must preserve entire old commentary before preparing changed baseline, carry valid physical-range review/comments and complete all new/dependency-affected source readability. Preserve C001 Data correction and alternate BIT/LDA entries. Round001 readability credit remains valid only within unchanged scope; current-source gate pending. No terminal handoff; whole-ROM still incomplete.


## Round002 commentary — active, batch 01 verified

Entire prior commentary preserved at `analysis/round-002/previous-commentary/` before fresh baseline preparation. Current map/state frozen at SHA256 `7dd308535281fec582e2294a84bb62b73b1d9cd258bf94823b9434f84a8bd0f4`. Carried 3,581 exact-boundary Code blocks /21,864 bytes and their symbols; new/changed Code scope51,070 bytes. Bank0 new audio continuations and ALL bank1 new menu/save/team/editor/result/defensive-MOVE Code now have functional explanations (192 named entries in135 regions, including additional entry explanations in old grouped spans). Restored the C00123-byte prefix's two dependent labels; corrected its twelve-byte audio tables without stale assembly.

Fresh batch verification `commentary/verification.json`: ok=true, byte_identical=true (game.asm SHA256976f54a15161e740ee1cae44dfee2e4e3eb43f4815883d12de2abde7a2d91db4). No reading-copy refresh claimed yet. Active frontier: bank2 text/layout new code, then bank3 presentation, bank5 automatic-control, bank6 match/object, bank7 fixed additions; all new data, boundary/dependency updates and branch/RAM naming remain pending. Current-source readability is NOT complete. Original static/gameplay limits retained; artificial defensive/audio paths are identified explicitly. Final editorial audit pending. No map/toolkit/input changes.


### Batch02 — text/editor and raw data verified

All new bank2 Code has functional source explanations,57 named entries in42 regions (includes an entry in a retained grouped span). Text countdown/format restoration, budgeted editor increments, packed-role swap, schedule matchup and ten-row summary structures are explained. All996 raw blocks now explain their formats or bounded unresolved role:43,497 Data bytes,131,072 CHR and14,657 Unknown. Existing supported consumers and positive generic profiles were reused; this does not invent unknown classification. Added structured data names for inline parameters, target cells, channel pointers, text/sprite/screen formats and fields. Fresh byte-identical verification passes; current game.asm SHA256a81792fb2b8a5a1b0c9bbf3f97ca0882efaa9866dea677cc6231c0ef2239d292.

Active remaining Code frontier: banks3/5/6/7 and C001 prefix review inventory; branch/RAM naming, exact alternate D0BB/D0BE aliases, stale previous-boundary explanations and source-reference refresh still pending. Bank0/1/2 code and raw-format prose saved; no final source-readability handoff or audit claimed.

C003 found during active bank3 commentary: the new `$E68D->$FBA2->$FC1E` inline-copy contract consumes a two-byte descriptor pointer, while current physical `$00C590..$00C596` decodes across that pointer and the next call as DEC/STA. Exact bytes, consumer and bounded redirect in `commentary/uncertainties.md`. Frozen map preserved; no structural edits during commentary. Continue all pending source readability.


### Batch03 — presentation/records code verified

All new bank3 Code regions have functional explanations:105 valid named entries in42 regions, including a retained grouped entry. Presentation scripts/scrolling, signed object motion, field effects, standings and ranked record lists, wide arithmetic and static qualified-team progression are explained without a natural Season-terminal claim. C003 explicitly identifies the misleading current decode at physical $00C590..$00C596; its frozen bytes/classification remain unchanged for redirect. Fresh source verification passes byte-identically, game.asm SHA256c5a3105eb0e64d8bb200bfb115d93d0149b415afe04e8b991a8695f2ceb57439.

Remaining current-source Code: bank5 automatic control, bank6 match/object, bank7 fixed runtime, plus inventory for the restored C001 prefix. Continue same commentary phase. Branch/RAM/field/constant naming, D0BB/D0BE alternate aliases and selecting-cell explanations, stale prior raw-boundary dependencies and final source-location/reading-copy refresh remain pending. Raw payload/unknown-format prose is saved; current-source readability not complete and editorial audit not run.


### Batches04/05 — fixed runtime and changed dependencies verified

Reviewed new fixed-bank camera/map tile expansion, object initialization/drawing, motion projection, actor comparison, palette effects, wide arithmetic, inline-copy and PPU attribute upload.148 valid entries in115 regions received explanations. A verification mismatch caught and corrected an attempted symbolic alias for JMP ($8000); lower-bank indirect execution keeps its correct unresolved bank context. Fresh verification after correction and dependency batch05 is byte-identical. C001 prefix now retains sound prior names/23-byte control flow with a current explanation and review inventory. Canonical D0B8 BIT sequence has descriptive entry/tail prose, explicit alternate CPU aliases ObjectSelectState2C_D0BB/ObjectSelectState2D_D0BE, and selecting-cell offsets/indices beside the retained22-cell table.

Active Code frontier: bank5 automatic-control and bank6 match/object additions. New Code in banks0/1/2/3/7 and all raw Data/CHR/Unknown payload notes are saved. Still pending: branch/RAM/field/constant naming across new source, stale prior unclassified-continuation dependencies, final exact source-location refresh and fresh handoff verification/reading-copy refresh. C003 remains localized for redirect. Current-source readability is not complete; no editorial audit invoked.


### Batches06/07 — all new Code functional prose saved

Bank6 match/ball/pitch/point-selection code saved with220 valid entry explanations in158 regions. Bank5 automatic batting/pitching/defense, three-axis world flight, runner interactions, fielder motion/throw/contact mechanics saved with141 valid entry explanations in all91 new regions. Fresh byte-identical verification after bank5 passes, game.asm SHA2563a37013add3330630b350b6b17de9a376f1e1bb10ae0892266acc5a27e9fede0. This is functional source coverage; final readability finishing remains active.

Active frontier: consolidate descriptive branch/RAM/record-field/constant operands across new code, repair concrete stale prior raw-boundary comments, refresh all inventory/unit source references, verify current hashes/source and refresh reading-after.asm. C003 remains the exact localized structural decode question for redirect; no classification edits. No final readability handoff until finishing and fresh verification pass.


## Round002 commentary — current-source complete, verified handoff

All currently classified source and raw boundaries meet the current readability pass; no readability ranges deferred. Final editorial/archive audit remains pending and was not run.5,128 scaffold entries/1,625 meaningful units reviewed; preserved exact scaffold/effective views. All new Code/Data plus affected dependencies are explained, with2,291 new scoped control targets and RAM/record-field/constants. Exact prior source preserved at `analysis/round-002/previous-commentary/`; sound review/prose reused.

Fresh final verification is byte-identical and `reading-after.asm` refreshed. Source SHA256`687c97b04b3bf6c0c9f20428543007070ea3964c83bba113c93fba07a9ee86c5`; symbols SHA256`6b3605c46d4e4d02f673741b6a30aab2eff730ffe5ed2869b73289d13895db6b`. Input and map/state/baseline hashes unchanged; evidence in `analysis/round-002-commentary-verification.json` and `analysis/round-002/commentary-final-hashes.json`. Classification remains94.41%, with14,657 Unknown bytes.

Active next frontier is redirect: C003 exact inline-copy pointer/call boundary conflict at physical$00C590..$00C596; see current `commentary/uncertainties.md`. C001 corrected exact12-byte table and23-byte code-prefix dependency; D0BB/D0BE alternate aliases and selecting-cell explanations retained with real BIT mechanics. Gameplay/semantic limitations remain explicit. Compact complete handoff: `analysis/round-002-commentary-handoff.md`. No toolkit/input/map edits or publication.

## Coordinator gate — round002 commentary accepted; redirect active

Separate commentary worker finished. Coordinator inspected full compact handoff and C003 uncertainty; verified current source hashes against fresh byte-identical report, exact reading-copy content, frozen state/map identity, unchanged5128-entry scaffold partition and reviewed status.1625 meaningful units supplement source explanations; metadata alone does not establish prose quality. See `analysis/round-002-commentary-coordinator-check.json`. Preserved prior commentary remains local. Current-source readability complete; final editorial acceptance remains pending.

Separate `/root/round2_redirect` launched with its own prompt and compact checkpoints only. Exact map7dd308535281fec582e2294a84bb62b73b1d9cd258bf94823b9434f84a8bd0f4; classification94.41% (247503/262160;14657unknown). Investigate C003 inline-copy boundary contract and coherent residual bank/format avenues in a unique new attempt; prior completed redirect evidence immutable. No terminal assessment accepted; no requirement to repeat unchanged Season negatives.

## Round002 redirect — active r002-copy-contracts

Current map frozen SHA256 7dd308535281fec582e2294a84bb62b73b1d9cd258bf94823b9434f84a8bd0f4; 247503 classified,14657 unknown. Read redirect prompt, evidence policy and complete current-source commentary handoff. New evidence only under analysis/redirect/r002-copy-contracts; no map/state/source edits. Exact249-span residual ledger saved with physical8K partitions, alias candidates and neighbors.

C003 primary stack audit confirms E68D->FBA2 loads six descriptor bytes from a TWO-byte inline pointer; FC1E advances stacked return by3, restore jumps via($55). Six repeated call/pointer cells at C58D..C5AB, final JMP8473 closes to existing presentation handler. Descriptors begin C1EE and stride6. Investigating same-contract callers (current decoder inventory finds C7F4 fill contract and C58D copy; corrected cells expose five further calls). Independent bounded pattern/consumer inventory is saved in residual-pattern-primary.json. Strong opportunities include bank2 UI text/row/template containers, bank6 four-byte contact records, fixed threshold/quantized tables and bank3 six-byte numeric records; no exact generator requirement. F810..FDDA separates a12-byte pointer array and five49x6-byte numeric blocks, from unresolved3-byte suffix FDDA..FDDD. Missing-direct-targets.json finds only two unresolved direct absolute targets after current Code decoding; inspect E725 and F095 contracts before proposing. Active frontier: finish C003 proposal, inspect independent consumers/boundaries and code conflicts, package combined ready batch; not terminal/exhausted.

### Redirect active checkpoint — connected dispatch graphs

Bounded fixed PPU tables resolved from primary interpreter checks: F0..FE has15 control cells1F3AF..1F3CD;80..EF has7 token cells1F40B..1F419. All supported handlers and direct/shared continuations followed,631 candidate new Code bytes. The bank3 routines961B/9849 form coherent OAM palette/nibble memory editors with explicit PPU setup, frame/controller polling, scratch preservation and return; table cells37/38 corroborate these static entry contracts without asserting live callers. Followed all five four-mode inline dispatches, selector low bits maintained0..3, preserving inline barriers; bank3 connected graph now2053 candidate Code bytes. Mapper call9866 is supported by98xx saving incoming$50 and restoring it before E66F; precondition desiredPRG3 maintains banks[6,7,14,15]. Stops are existing MMC1 physical-write primitives and an indirect jump through caller data, not uninspected handlers.

Independent audio graph: consumer8258 pointer family has29 consistent cells ending at independently selected shared helper8292; effect commands1..4 have four cells86E8..86EF; command0 cell003E is deliberately excluded. Subdomains are supported static command contracts, not a claim that the full masked6bit/fourbit domains are valid. Followed all supplied handler continuations plus four accepted-instruction fallthrough gaps:283 candidate new Code bytes. Source displays and raw primary bytes saved; configured decoder validation and final proposal ledger still pending. Current map/source remain frozen; proposal bytes are not accepted classification.

### Redirect checkpoint — combined proposal validated, residual review active

Draft proposal now14248 unique unknown bytes (3358Code/10890Data), conditional409 unknown if independently accepted. Actual map remains247503/262160 (94.41%),14657Unknown; ZERO proposed bytes applied. Code validated through configured release binary:1615 instruction entries in91 bounded analysis.decode calls,zero decoder mismatches. Data evidence now160 exact interval reviews: bounded direct/new consumers, pointer/row/numeric records, independently delimited five49x6 mirrored-position tables, masked64-word quarter-wave table,101 random thresholds, fixed PPU attribute arrays/palettes/title metadata, and localized small patterned lookup fragments. Exact unknown ledger and proposed/unproposed complement updated; no full classification assertion.

Reviewed remaining accepted-instruction fallthrough/JSR continuation inventories: no missing relative targets. Fixed service JMP grid offers supported entry contracts without natural callers; followed new veneers and downstream palettes/transition wrappers. Explicit PRG3 writer before fixedCC13 now establishes the F810/B800 numeric-container consumer; previous nearby bank6 A500 references do NOT address fixedE500 (address-context error corrected in data review). NMI optionalFF selection gives explicit effective[14,15,14,15] for fixed continuation, not natural debug activation; incoming ROM flag disables ordinary entry. Bank3 memory editors remain static contract evidence, not claimed natural debug use.

Residual409 bytes are primarily small isolated RTS/JMP/PLA/PLP fragments, null-target wrapper placeholders and a repeated actor-comparison tail starting with operand-like2804 bytes at1E13D. Active review: localize the2-byte prefix and compare independently anchored predicate bodies; ensure short data fallbacks are supported by exact positive patterns/neighbors rather than missing semantics. Null-wrapper target0000 does not establish callable ROM targets; preserve history and distinguish stub pattern from Code role. Current artifacts: proposal-batch.json,pattern-data-review.json,configured-decoder-validation.json,remaining-jsr-continuations.json,branch-control-seeds.json. Static reachability and genuine gameplay limits unchanged. No toolkit/map/source/ROM edits.

## Round002 redirect — complete ready batch, nonterminal

Unique attempt r002-copy-contracts frozen; full handoff analysis/round-002-redirect-handoff.md. Actual map remains94.41%:247503classified/14657unknown. Proposed14478 unique unknown bytes (3562Code/10916Data), plus exact3Code-to-Data C003 correction; accepted0. Conditional179 unknown if independently accepted. LeadsL001,L003,L005,L006 reopened ready with new premise/evidence/recipes; historical entries preserved. Configured decoder1712 instructions/115boundedcalls/zero mismatches. Final input/map/checkpoint/state/source hashes unchanged (checkpoint-hashes.json).

All productive connected/independent PPU, audio, memory-editor, mirrored-position, API/terminal/null-veneer, numeric/PPU/contact/palette pattern and repeated short-helper continuations were investigated in one batch. Corrected null-target reasoning:0000 rejects pointed ROMcallee, not terminalwrapper or three-byte service-stubCode. Kept separate physical byteunion and supported effectivecontext views. Current ledger retains exact249-input-span complement; all conditional179 residual bytes have localized evidence-linked dispositions. Strong actor-prefix/tail candidate was split and primary-tested: two-byte2804 prefix remains ambiguous, and distinct tail helpercalls enter accepted instruction operands, failing offered equivalent-predicate contract (actor-tail-review.json). This is not a rejection based on missing natural caller and does not propagate to other ranges. InterstitialsingleEA and isolatedreturn/jump/branch fragments retain genuine code/data or entry-boundary ambiguity.

No terminal incomplete/exhaustion proposed; independent disassembly review is next, then complete maintained-source commentary and redirect. Original C001/BIT alternate entries and broad gameplay limits preserved. No ROM/map/state/source/toolkit edits, runtime repeats, deletion, publishing or archive. ExistingTK-001 remains; no new toolkit issues found. Do not rerun completed attempts' finalizers after map changes.

## Coordinator gate — round002 redirect ready for round003 disassembly

Redirect worker completed/stopped; frozen r002-copy-contracts retained. Inspected compact handoff, contracts and decisive actor-tail primary review. Hashes match current ROM/map/state/source (`analysis/round-002-redirect-coordinator-check.json`). Proposed14478 unknown bytes (3562Code/10916Data) plus exact3-byte C003 Code->Data correction remain unaccepted; conditional179 unknown is not current coverage or terminal exhaustion.115 configured decoder calls/1712 instruction entries support fidelity, not semantic proof alone.

Separate `/root/round3_disassembly` launched with only its phase prompt and compact checkpoints. Independent full-batch role/boundary/view/consumer review required; preserve aliases and physical union, frozen history and maintained commentary. Actual classification remains94.41% (247503/262160;14657 unknown) until accepted annotations/checkpoint. No terminal handoff accepted. Existing broad gameplay and failed Season methods may be reused without unchanged replays.

## Round003 disassembly — active independent batch review

Resumed exact state/checkpoint `7dd308535281fec582e2294a84bb62b73b1d9cd258bf94823b9434f84a8bd0f4`; actual247503 classified/14657 unknown. Reviewing entire frozen r002-copy-contracts union, not regenerating commentary. Primary consumer/stack contracts support two-byte C003 inline pointers and six descriptors; bounded audio/PPU dispatch domains, memory-editor graphs, fixed/banked/null veneers, mirrored-position and generic positive-pattern containers reviewed with separate structural/reachability claims. Automated independent raw/context/neighbor checks cover1712 instructions,1686 alias views,160 pattern intervals and33 veneers; three continuation source checks need existing-Code handling, and bank-start neighbor inspection excludes the header. No annotation yet. Frontier: finish independent audit, accept supported union through map.annotate, preserve residual179, validate/export/checkpoint and record exact outcomes. Prior gameplay routes and Season negatives reused unchanged.


## Round003 disassembly — complete single pass, incomplete ROM classification

Accepted the full independently reviewed frozen r002-copy-contracts master union:14478 unique new bytes,3562 Static Code/10916 Static Data. Exact C003 correction moves three previouslyCode bytes toData and retains nextJSR atC592. Final structural totals:Code76493,Data54416,Image131072. Physical bytes and supported CPU/effective contexts are distinct; aliases do not increase counts. Prior C001 correction and canonical BIT/LDA D0BB/D0BE alternatives retained. Independent review `analysis/round-003-disassembly/structural-review.md`, `independent-review.json`, `data-review.json`, `existing-continuation-boundaries.json`; mutation evidence `annotation-log.json` (335 RPC calls). All160 pattern intervals raw/neighbors checked,33 wrapper contracts,1712 instruction entries and1686 graph views inspected; reused clean configured release decoder115 calls. Three old continuation sources confirmed by actual canonical boundary walks and raw opcodes, not mere Code containment.

State/current validated and exported; fresh unique checkpoint `round-003-disassembly-final` verified byte-identically: `analysis/round-003-disassembly-final-verification.json`, `generated/round-003-disassembly-final`, `maps/round-003-disassembly-final.json`. Current map/state/checkpoint SHA25688b2a2d0856b4c936e59a043fa821c303fc91600e1b4183e3afb7711e12f9eac. ROM unchanged eeebc807e4a53336b871b4195eef25df0c80a558156742c3411a83225d4f909b. Maintained commentary source unchanged687c97b04b3bf6c0c9f20428543007070ea3964c83bba113c93fba07a9ee86c5; no source regeneration/mutation in this phase.

LeadsL001/L003/L005/L006 verified for overlapping full union; histories preserved. Broad gameplay/scenarios reused: Exhibition/Edit, natural Ultra pitching selection/delivery/points and artificial9INBOT pitching/defensiveMOVE/return/GAMESET. No unchanged Season replay, no new special-hit/catch/Season-terminal claim. Practical late-game shortcuts already attempted and documented in scenariosS005/S006/S008/S010; unresolved Season widget/parent-transition contract remains a separate semantic/gameplay question, not a reason to withhold structural claims.

Active next frontier: current-map commentary for all new/affected classifications and C003 explanation, then redirect over exact179 unknown bytes/39 spans (`analysis/round-003-disassembly/unknown-frontier.json`). Reused `residual-review.json` and `actor-tail-review.json` actual rejection scope: actor helper targets fail offered clone semantics by entering operands, while isolated returns/jumps/EA/branch fragments still lack discriminating roles; no blanket unknown-toData conversion or missing-natural-caller prerequisite. No positive continuation withheld for another round. This is an ordinary easy-gain handoff, not whole-ROM/readability/terminal exhaustion success. ROM classification remains incomplete.

Toolkit: no new toolkit issues found. Existing confirmed TK-001 and input-array workaround preserved in `analysis/toolkit-issues.md`. No toolkit/ROM edits, history deletion, cleanup/archive or publication.

ROM classification: 99.93% (261981 / 262160 bytes classified; 179 bytes unknown).

| Bank | Code | Data/image | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG 00 | 2520 | 5672 | 0 |
| PRG 01 | 0 | 8192 | 0 |
| PRG 02 | 7745 | 447 | 0 |
| PRG 03 | 7368 | 815 | 9 |
| PRG 04 | 4150 | 4042 | 0 |
| PRG 05 | 0 | 8192 | 0 |
| PRG 06 | 6425 | 1766 | 1 |
| PRG 07 | 5569 | 2618 | 5 |
| PRG 08 | 0 | 8192 | 0 |
| PRG 09 | 0 | 8192 | 0 |
| PRG 10 | 7614 | 539 | 39 |
| PRG 11 | 7600 | 576 | 16 |
| PRG 12 | 7710 | 482 | 0 |
| PRG 13 | 6636 | 1555 | 1 |
| PRG 14 | 7144 | 1032 | 16 |
| PRG 15 | 6012 | 2088 | 92 |
| CHR 00 | 0 | 8192 | 0 |
| CHR 01 | 0 | 8192 | 0 |
| CHR 02 | 0 | 8192 | 0 |
| CHR 03 | 0 | 8192 | 0 |
| CHR 04 | 0 | 8192 | 0 |
| CHR 05 | 0 | 8192 | 0 |
| CHR 06 | 0 | 8192 | 0 |
| CHR 07 | 0 | 8192 | 0 |
| CHR 08 | 0 | 8192 | 0 |
| CHR 09 | 0 | 8192 | 0 |
| CHR 10 | 0 | 8192 | 0 |
| CHR 11 | 0 | 8192 | 0 |
| CHR 12 | 0 | 8192 | 0 |
| CHR 13 | 0 | 8192 | 0 |
| CHR 14 | 0 | 8192 | 0 |
| CHR 15 | 0 | 8192 | 0 |

## Coordinator gate — round003 disassembly accepted; commentary active

ROM classification: 99.93% (261981 / 262160 bytes classified; 179 bytes unknown). Exact line emitted to user after completed pass and before commentary launch. Independent current ROM/checkpoint hash, current/checkpoint equality, state/map identity and fresh byte identity confirmed in `analysis/round-003-disassembly-coordinator-check.json`. Final checkpoint88b2a2d0856b4c936e59a043fa821c303fc91600e1b4183e3afb7711e12f9eac. Worker completed/stopped;179 residual bytes remain, no terminal exhaustion accepted.

Separate `/root/round3_commentary` launched with phase prompt and compact handoffs. Must preserve entire previous commentary before preparation, carry valid physical-range/alias review and complete new/affected source plus concrete stale dependencies. C003 correction resolves old decode caveat; all remaining179 Unknown require honest boundary notes. Current-source readability pending on changed map; prior credit retained by unchanged scope. Final editorial acceptance and terminal rejection review remain pending.

## Round003 commentary active

Full prior commentary preserved at `analysis/round-003/previous-commentary/` before new baseline preparation. Baseline byte-identical; 4,898 exact physical/kind records carried, 349 new/affected records active (3,776 Code bytes including C003 prefix). Exact frontier and remaining work in `commentary/progress.md`; plans saved at `analysis/round-003/plan.py`. Readability pending; no audit or handoff yet. Frozen classification 261981/262160; 179 bytes unknown, no whole-ROM claim.

Round003 commentary checkpoint: 213 new meaningful Code entries explained/named; code-batch byte-identical verification passes. Valid C003 prefix restored without stale trailing instructions. All 236 changed raw Data/Unknown boundaries now have checked format/uncertainty prose and 55 data labels. Remaining dependency review, concrete stale statements/alias checks, complete inventory references and fresh final verify/read are active in `commentary/progress.md`. No successful handoff claimed.

Round003 commentary dependency checkpoint: thirty-one concrete interior entries and sixteen service aliases resolved; explicit CPU aliases for physical data labels rebuilt byte-identically. New dispatch formats and all changed raw-boundary prose complete. C004 records CPU C05D -> CFB4 rendering-disable semantics (frozen research incorrectly said D0B8 initializer); no structural/map change. PPU coordinate-save quirk retained. Full inventory/source-location reconciliation and final verify/read remain active; no audit/handoff yet.

## Round003 commentary handoff — current-source readable, verified

All current source, including 349 new/affected records and concrete carried dependencies, meets this readability pass. 5,247 scaffold entries reviewed/current source links; 1,933 unique meaningful units retained/added. Fresh byte-identical verification and refreshed reading copy are complete. Source SHA256 4acccb9ce71d6c7479ddf89f0239e1a2d3c9ea65b8882797731d39737594c77a. Input unchanged; all frozen current map/state/baseline identities equal88b2a2d0856b4c936e59a043fa821c303fc91600e1b4183e3afb7711e12f9eac. Final audit not run: 2,719 readable Code rows still require meaningful-unit association/extent refinement at final inventory acceptance; exact metadata frontier saved. Classification remains 261981/262160,179 Unknown/39 spans; no whole-ROM/terminal claim. Next redirect receives C004 CPU C05D->CFB4 semantic correction and residual frontier. Details: `analysis/round-003-commentary-handoff.md`, `commentary/progress.md`, `commentary/uncertainties.md`.


## Round003 redirect — localized fragment contracts

Input map/state88b2a2d0856b4c936e59a043fa821c303fc91600e1b4183e3afb7711e12f9eac, source4acccb9ce71d6c7479ddf89f0239e1a2d3c9ea65b8882797731d39737594c77a. Actual261981/262160classified;179unknown/39spans unchanged. Unique attempt `analysis/redirect/r003-fragment-contracts/` revisits the short-unrooted inference using independent field/consumer/parallel-entry constraints, not unchanged runtime retries. L007 ready:53 Static Code bytes/10spans,22instructions,10configured decoder calls,0faults. Coherent action/priority veneers, fourth direction emitter, full height predicate, possession-gated collision/draw and modifier0/2 shared entries all reviewed with existing continuations. Each subclaim remains independently reviewable. Exact residual ledger and localized non-code/actor rejection scopes in contracts.md; no genericData fallback or first-ready stop. C004 appended research correction C05D->CFB4 rendering-disable, notD0B8 initializer. Gameplay checklist reused with actual GAMESET/artificial defense and naturalUltra pitch/point evidence; unchanged Season attempts not replayed. Research/lead records only; no ROM/map/state/source/toolkit edits. No new toolkit issues found; existing TK-001 nonblocking. Handoff `analysis/round-003-redirect-handoff.md`; nonterminal, no goal completion or terminal exhaustion claim.


## Round004 disassembly — independent L007 review and verified checkpoint

ROM classification: 99.95% (262031 / 262160 bytes classified; 129 bytes unknown).

Accepted50 unique Static Code bytes/nine spans (F01,F02 both spans,F03,F05..F09); F04 three-byte isolated duplicate JMP retained Unknown. Exact role/control/stack/field/consumer/bank/boundary rationale: `analysis/round-004-disassembly/structural-review.md`; fresh configured decoder9 calls/21instructions/0diagnostics corroborates exact boundaries. No previous classified byte changed kind; no Data gain. Code76543, Data54416, CHR131072. Residual129bytes/30spans in `analysis/round-004-disassembly/unknown-frontier.json`. Complete connected accepted continuations reviewed; no ready fragment deferred.

Fresh checkpoint `maps/round-004-disassembly-final.json` SHA256 808eeb38ef5b3411a7764ae95b6c98939e707d53b791fc646322facbbfc3b026; `analysis/round-004-disassembly-final-verification.json` confirms byte_identical=true,ok=true. State/map/checkpoint identical; immutable ROM identity preserved. Maintained commentary source unchanged4acccb9ce71d6c7479ddf89f0239e1a2d3c9ea65b8882797731d39737594c77a; changed-map readability and final2719-row inventory metadata remain pending commentary, separate from classification/fidelity. C004 correct current semantics preserved.

Reviewed/reused broad gameplay S005 artificial9INBOT/pitch/GAMESET, S008 defensiveMOVE/exit, S009 naturalUltra pitch/point evidence and failed S006Season widget/parent routes. No changed practical shortcut arose; no unchanged replay or new natural/Observed claim. Nonterminal easy-gain stopping point, not terminal exhaustion. Coordinator flagged EA singleton inference for later localized review: regular6byte sibling spacing plus bounded5byte service payload may itself supply positive padding evidence; NOP-prefix possibility must be weighed rather than imposed as a universal veto. Not adjudicated beyond this supported batch.

Existing TK-001 confirmed input-schema bug remains nonblocking in `analysis/toolkit-issues.md`; no new toolkit issues found (0new confirmed/suspected/missing features). Toolkit unchanged. Disk5.9GiB free/scratch52KiB checked; no cleanup/deletion.

| Bank | Code | Data/image | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG 00 | 2520 | 5672 | 0 |
| PRG 01 | 0 | 8192 | 0 |
| PRG 02 | 7745 | 447 | 0 |
| PRG 03 | 7374 | 815 | 3 |
| PRG 04 | 4150 | 4042 | 0 |
| PRG 05 | 0 | 8192 | 0 |
| PRG 06 | 6425 | 1766 | 1 |
| PRG 07 | 5569 | 2618 | 5 |
| PRG 08 | 0 | 8192 | 0 |
| PRG 09 | 0 | 8192 | 0 |
| PRG 10 | 7644 | 539 | 9 |
| PRG 11 | 7614 | 576 | 2 |
| PRG 12 | 7710 | 482 | 0 |
| PRG 13 | 6636 | 1555 | 1 |
| PRG 14 | 7144 | 1032 | 16 |
| PRG 15 | 6012 | 2088 | 92 |
| CHR 00 | 0 | 8192 | 0 |
| CHR 01 | 0 | 8192 | 0 |
| CHR 02 | 0 | 8192 | 0 |
| CHR 03 | 0 | 8192 | 0 |
| CHR 04 | 0 | 8192 | 0 |
| CHR 05 | 0 | 8192 | 0 |
| CHR 06 | 0 | 8192 | 0 |
| CHR 07 | 0 | 8192 | 0 |
| CHR 08 | 0 | 8192 | 0 |
| CHR 09 | 0 | 8192 | 0 |
| CHR 10 | 0 | 8192 | 0 |
| CHR 11 | 0 | 8192 | 0 |
| CHR 12 | 0 | 8192 | 0 |
| CHR 13 | 0 | 8192 | 0 |
| CHR 14 | 0 | 8192 | 0 |
| CHR 15 | 0 | 8192 | 0 |


# Round004 commentary handoff

**All-current-source readability complete; fresh byte-identical verification passed and reading copy refreshed. ROM classification remains incomplete. Final editorial/archive acceptance remains pending.**

Frozen current/state/checkpoint/commentary baseline identity is `808eeb38ef5b3411a7764ae95b6c98939e707d53b791fc646322facbbfc3b026`; input remains `eeebc807e4a53336b871b4195eef25df0c80a558156742c3411a83225d4f909b`. Classification remains 262031/262160 (99.95%): Code76543, Data54416 including header, CHR131072, Unknown129 in30spans. Source readability is a separate completed gate, with no deferred readability ranges.

Preserved the entire prior commentary by moving it to `analysis/round-004/previous-commentary/` before fresh preparation. Carried5238 exact physical range/kind source and review records. Completed all9 new Code records/50bytes using the independent fragment/consumer contracts and current source. All5247 current scaffold entries are reviewed; the full partition exactly matches frozen state. Existing sound prose/review was reused; no full-source reread was required. Current audit source fields are normalized to `game.asm:line`, resolved against `commentary/source`, with every referenced file/line validated.

New named source explains unconditional team1 result selection; outer entries completing the five contiguous action veneers; the fourth cardinal direction mask and command/port writer; priority-order tail entries; a height/penetration predicate with complete carry partition; possession-gated interaction and shared drawing; base collision extents and modifier-two selection. Physical identities and effective banks remain intact; natural callers are not established. Five existing source blocks received concrete dependency edits: physical84707..84716,84927..84976,85350..85463,98282..98290,98294..98317. The source names the exact8D9E third-ordering entry rather than confusing it with8D56; verification caught and corrected the initial symbol-target mismatch before handoff. C001/C002/C003 resolutions and C004 C05D->CFB4 rendering-disable/NMI-bit2 service correction remain intact, as does the PPU coordinate-save asymmetry.

Reusable metadata repair added281 nontrivial reviewed named control-flow extents and associated1907 inherited Code rows. These derive from consecutive prior records explicitly reviewed within the same named block, optionally including its immediately abutting named entry; they stop at a different review scope or layout barrier. They do not assert original routine boundaries. Examples: AudioConsumeQueuedRequest physical31..48 processes/clears queued requests; MusicInitializeChannel148..200 initializes a$16-byte channel record; MusicReadNextEvent382..416 interprets events and resets relevant modulation state. Actual entry contexts/source lines are recorded in `analysis/round-004/metadata-extent-associations.json`. No dummy per-instruction units were added. Unique meaningful units now2223.

**Remaining final inventory metadata frontier:812 Code rows need meaningful-unit association/extent refinement**, down from2719. Their reviewed source remains readable; this is not a readability deferral. Exact ranges/summaries/current canonical source locations: `analysis/round-004/final-inventory-metadata-frontier.json`. Editorial status remains pending; final `audit` was not invoked. Continue legitimate functional/boundary review at final acceptance rather than fabricating units to satisfy schema.

Fresh final verification: `analysis/round-004-commentary-verification.json` and `commentary/verification.json`, ok=true,byte_identical=true. Source SHA256 `2e1d15500bb065d4660a258a956596844f0ebe60bb415cf8e38f190820093a93`; readable-symbols unchanged `810effce77c53f568fce2f1241b4964618f6c8a903d193a3104647e1bd378ef3`; reading-after SHA256 `3d9a024f8e0114e07105866b9f4768c8fede414aa1ffea8f0de13720b1797e05`. Exact identities/source inventory/readability counts are in `analysis/round-004/commentary-final-hashes.json`, `source-inventory-check.json` and `readability-counts.json`.

Remaining structural frontier stays `analysis/round-004-disassembly/unknown-frontier.json`. F04 isolated duplicate JMP remains Unknown, without rejecting the adjacent accepted family. Next redirect may adjudicate the materially revised EA-spacing/payload padding inference identified by coordinator; existing singleton ambiguity is not a universal requirement to exclude every executable interpretation. Actor/RTS/BIT-trailer and other short fragments remain localized. No terminal-exhaustion or whole-ROM completion claim.

No new gameplay replay was needed. Prior actual Ultra pitching/point cost, artificial initialized9INBOT/pitch/GAMESET and defensiveMOVE/menu-exit evidence remain; Season widget/parent-transition barrier, catch/contact and natural terminal limits remain. ExistingTK-001 is nonblocking; no new toolkit issues found. See `analysis/toolkit-issues.md`. No ROM/toolkit/map/state changes, archive, cleanup, deletion or publication. Final disk5.9GiB free; scratch52KiB unchanged. Historical source scripts are checkpoints, not safe blind replay recipes.

## Round004 redirect r004-wrapper-spacing

Input808eeb38; research only. Revised primary reasoning finds rooted NOP-prefix service family and two independently complete actor conditional-copy blocks; combined34 Static Code bytes/16spans proposed via L008/L009,0accepted. Exact primary evidence, local rejection scopes, gameplay review and deferred-avenue reasons: analysis/redirect/r004-wrapper-spacing/contracts.md; proposal-batch.json and unknown-ledger.json. No terminal/whole-ROM claim, no map/state/source/input/toolkit changes. ExistingTK-001 nonblocking; no new toolkit issues found.


## Round005 disassembly — independent L008/L009 review

ROM classification: 99.96% (262065 / 262160 bytes classified; 95 bytes unknown).

Accepted34 Static Code bytes in16spans:14 NOP entry prefixes W01 and two10byte actor coordinate conditional-copy blocks A01/A02. No Data gain, no prior classified kind changes. Code76577, Data54416 including header, CHR131072. Configured decoder16calls/22instructions/0diagnostics corroborates exact boundaries. Structural proof and localized negative scope: `analysis/round-005-disassembly/structural-review.md`; primary rooted caller115814->115195 and canonical actor123175 verified against immutable ROM. No candidate natural/Observed reachability claimed. Existing inline Data preserved.

Fresh byte-identical checkpoint `maps/round-005-disassembly-final.json` SHA256 289c2300d7444e697915ba512406de08776cdff0d5ddab5749b40510a2118dc4; verification `analysis/round-005-disassembly-final-verification.json`, ok=true,byte_identical=true. State/current/checkpoint identical; source remains2e1d15500bb065d4660a258a956596844f0ebe60bb415cf8e38f190820093a93 unchanged. Changed-map commentary pending; final812-row unit metadata frontier remains separate. C004 semantic correction intact.

Broad gameplay S005/S008/S009 and S006 Season barrier reused, without unchanged replay. This static finding supplies no improved transition/RAM contract; successful catch/contact, Season terminal and natural ending remain limited. F04 isolated duplicate JMP and helper-dependent actor residuals remain unknown. Ready batch/connected continuations reviewed; ordinary easy-gain stopping point, no terminal exhaustion claim. Exact95byte/18span frontier in `analysis/round-005-disassembly/unknown-frontier.json`. ExistingTK-001 nonblocking; no new toolkit issues found; see `analysis/toolkit-issues.md`. ROM/toolkit/history preserved; disk5.9GiB free/scratch52KiB; no cleanup/publication/archive.

| Bank | Code | Data/image | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG 00 | 2520 | 5672 | 0 |
| PRG 01 | 0 | 8192 | 0 |
| PRG 02 | 7745 | 447 | 0 |
| PRG 03 | 7374 | 815 | 3 |
| PRG 04 | 4150 | 4042 | 0 |
| PRG 05 | 0 | 8192 | 0 |
| PRG 06 | 6425 | 1766 | 1 |
| PRG 07 | 5569 | 2618 | 5 |
| PRG 08 | 0 | 8192 | 0 |
| PRG 09 | 0 | 8192 | 0 |
| PRG 10 | 7644 | 539 | 9 |
| PRG 11 | 7614 | 576 | 2 |
| PRG 12 | 7710 | 482 | 0 |
| PRG 13 | 6636 | 1555 | 1 |
| PRG 14 | 7158 | 1032 | 2 |
| PRG 15 | 6032 | 2088 | 72 |
| CHR 00 | 0 | 8192 | 0 |
| CHR 01 | 0 | 8192 | 0 |
| CHR 02 | 0 | 8192 | 0 |
| CHR 03 | 0 | 8192 | 0 |
| CHR 04 | 0 | 8192 | 0 |
| CHR 05 | 0 | 8192 | 0 |
| CHR 06 | 0 | 8192 | 0 |
| CHR 07 | 0 | 8192 | 0 |
| CHR 08 | 0 | 8192 | 0 |
| CHR 09 | 0 | 8192 | 0 |
| CHR 10 | 0 | 8192 | 0 |
| CHR 11 | 0 | 8192 | 0 |
| CHR 12 | 0 | 8192 | 0 |
| CHR 13 | 0 | 8192 | 0 |
| CHR 14 | 0 | 8192 | 0 |
| CHR 15 | 0 | 8192 | 0 |

## Round005 commentary complete — redirect next

Input/output structural checkpoint unchanged:
289c2300d7444e697915ba512406de08776cdff0d5ddab5749b40510a2118dc4.
Classification99.96% (262065/262160), residual95bytes/18spans. Preserved prior
commentary and completed ALL current-source readability:5232 exact reviews carried,
16newCode spans/34bytes plus3split Unknown spans reviewed;5251rows total with no
readability deferrals. W01/A01/A02 contracts consumed for explanations, no new lead
produced. C001..C004 and PPU asymmetry preserved. Fresh commentary verification
byte-identical and reading refreshed. Final editorial frontier812Code rows remains
pending;2239meaningful units, no final audit. Handoff
analysis/round-005-commentary-handoff.md; verification
analysis/round-005-commentary-verification.json; inventory
analysis/round-005/source-inventory-check.json. No new toolkit issues, TK-001
nonblocking. No map/state/ROM/toolkit edits, archive, cleanup or publication.

## Round005 redirect r005-actor-patterns

Checkpoint289c2300; exact95byte/18span complement preserved. Corrected needed-claim review distinguishes recognizable field/mask/control machine-code fragments from invalid whole-handler contracts. L010 proposes62 Static Code bytes/3spans;0accepted here. Primary bytes, canonical comparison, malformed target scope, weaker subclaims, other residual dispositions and broad gameplay review: analysis/redirect/r005-actor-patterns/contracts.md, actor-primary.json, unknown-ledger.json. No useful connected continuation omitted; ordinary ready handoff, no terminal or ROM-completion claim. Map/state/source/input/toolkit unchanged; existing TK-001 nonblocking, no new toolkit issues found.


## Round006 disassembly — independent L010 actor pattern review

ROM classification: 99.99% (262127 / 262160 bytes classified; 33 bytes unknown).

Accepted62 Static Code bytes in3spans P01/P02/P03, physical123199..123229,123239..123259,123269..123281, bank15 effective[null,null,14,15]. Code76639/Data54416/CHR131072. Prior annotated kinds preserved. Structural rationale and weaker subclaim review: analysis/round-006-disassembly/structural-review.md. Immutable ROM/context/decode checks: independent-review.json; configured release25instructions/0diagnostics. Repeated field/test/mask/local branch topology supports recognizable fragments; malformed contemporary helper destinations remain explicit. No functioning whole handler, relocation, obsolete origin, natural entry or Observed claim. Orphan2804 remains unknown. Exact62byte diff/33unknown/16span audit and per-bank counts retained.

Fresh maps/round-006-disassembly-final.json SHA9693711a84a622e470d5e1ae492735ca2404fffe6b7778d316054494c7c21637; analysis/round-006-disassembly-final-verification.json confirms ok=true,byte_identical=true. State/current/checkpoint identical, input ROM and commentary source unchanged SHA95d55c01310dcf6570dfc279f45016dbd8ecb068a4f94bbb7fa695b20d13a136. Full current-source commentary next; final812-row metadata frontier pending.

Broad progression S005 actual artificial GAMESET/S008 MOVE/S009 natural Ultra reused; Season widget/parent transition and failed S010 limits unchanged. No new practical RAM/transition contract, no unchanged replay. Remaining isolated transfers/returns, numeric trailer,E608 split and orphan retain localized ambiguity; no automatic extension. Ordinary easy-gain handoff, not terminal exhaustion. Existing TK-001 nonblocking; no new toolkit issues found, see analysis/toolkit-issues.md. Disk5.9GiB free; no scratch batch, cleanup,archive,publication or toolkit changes.

| Bank | Code | Data/image | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG 00 | 2520 | 5672 | 0 |
| PRG 01 | 0 | 8192 | 0 |
| PRG 02 | 7745 | 447 | 0 |
| PRG 03 | 7374 | 815 | 3 |
| PRG 04 | 4150 | 4042 | 0 |
| PRG 05 | 0 | 8192 | 0 |
| PRG 06 | 6425 | 1766 | 1 |
| PRG 07 | 5569 | 2618 | 5 |
| PRG 08 | 0 | 8192 | 0 |
| PRG 09 | 0 | 8192 | 0 |
| PRG 10 | 7644 | 539 | 9 |
| PRG 11 | 7614 | 576 | 2 |
| PRG 12 | 7710 | 482 | 0 |
| PRG 13 | 6636 | 1555 | 1 |
| PRG 14 | 7158 | 1032 | 2 |
| PRG 15 | 6094 | 2088 | 10 |
| CHR 00 | 0 | 8192 | 0 |
| CHR 01 | 0 | 8192 | 0 |
| CHR 02 | 0 | 8192 | 0 |
| CHR 03 | 0 | 8192 | 0 |
| CHR 04 | 0 | 8192 | 0 |
| CHR 05 | 0 | 8192 | 0 |
| CHR 06 | 0 | 8192 | 0 |
| CHR 07 | 0 | 8192 | 0 |
| CHR 08 | 0 | 8192 | 0 |
| CHR 09 | 0 | 8192 | 0 |
| CHR 10 | 0 | 8192 | 0 |
| CHR 11 | 0 | 8192 | 0 |
| CHR 12 | 0 | 8192 | 0 |
| CHR 13 | 0 | 8192 | 0 |
| CHR 14 | 0 | 8192 | 0 |
| CHR 15 | 0 | 8192 | 0 |

## Round006 commentary — complete

Input/output checkpoint9693711a84a622e470d5e1ae492735ca2404fffe6b7778d316054494c7c21637 unchanged. L010 accepted fragment evidence consumed; no new leads produced. All5252 current source entries reviewed (5248 exact carries,3 new Code fragments/62bytes,1 split Unknown prefix); 2242 meaningful units. Corrected2 copy-join dependency notes; malformed targets and orphan prefix explicit. Fresh byte-identical verification passed, reading refreshed: analysis/round-006-commentary-verification.json. ROM99.99% (262127/262160),33unknown/16spans. Final812row metadata frontier pending separately; no final audit. Compact scope/limitations/links: analysis/round-006-commentary-handoff.md. Next phase redirect. No new toolkit issue; TK-001 remains nonblocking. No ROM/map/state/toolkit edits or cleanup/publication.


## Round006 redirect — residual review, terminal incomplete recommendation

New attempt `analysis/redirect/r006-residual-review/`; input checkpoint/current/state9693711a84a622e470d5e1ae492735ca2404fffe6b7778d316054494c7c21637, readable source19180bcbe08750c7974381bb8beeefbd9e6c9a57bb9759906b90d8a967eab69a. Corrected Code-role criterion from actor acceptance tested against isolated transfers/returns; positive generic-Data patterns and independent bounds considered locally. Fresh exact33byte/16span ledger and limited accepted-Code direct-edge audit saved; no direct edges found, not an unreachability conclusion. Source-lead scan inspected dispatch/table/bank/semantic uncertainties and generic labels; actual source constraints and primary bytes weighed in compact rejection-review.md. No supported proposal or justified new discriminating test survives. Recommend stop incomplete pending coordinator independent inspection, not ROM/goal completion; no empty disassembly cycle. Proposed0/accepted0; Code76639, non-CHRData54416,CHR131072,Unknown33.

Broad gameplay primary review rechecked actual GAMESET/Ultra/MOVE images and failed Season shortcut/timer/Start/A-intercept logs; no new active-widget/parent/entry contract justifies unchanged replay or arbitrary RAM. Season terminal/live contact/credits remain unvisited. Current-source readability complete; final812Code-row metadata frontier and archive acceptance remain pending. No new toolkit issues found; existing confirmed nonblockingTK-001 retained. No ROM/map/state/source/toolkit/history edits, cleanup or publication. Durable review/primary links and exact residuals in `analysis/round-006-redirect-handoff.md`.

## Final editorial acceptance — audit running

Terminal structural recommendation remains INCOMPLETE (33unknown/16spans); map/state9693711a84a622e470d5e1ae492735ca2404fffe6b7778d316054494c7c21637 unchanged. Final editorial worker preserved prior metadata in analysis/final-editorial/round-006 and resolved all812 empty Code-row associations plus28 partial existing unit extents. All4272Code rows/76639Code bytes covered by associated reviewed functional units; 2811unique units/6082associations. Split misplaced initialization/dispatch explanation at111791/111807; no instruction edits. Editorial notes reviewed in commentary/editorial-audit.md. Single supported commentary.py audit launched after gap resolution, currently checking references; final result/hashes and handoff pending. Do not rerun while active or report final acceptance success before its result.

## Final editorial acceptance — passed

Single supported audit completed ok=true,byte_identical=true,editorial_audit=true; reading refreshed. 5252reviewed regions,4272Code rows/76639Code bytes fully associated,2811unique meaningful units/6082reviewed associations. All812empty-row plus28partial-extent metadata gaps resolved; source initialization/dispatch comment corrected without instruction changes. Exact hashes, preserved metadata, methods and limitations: analysis/final-editorial-handoff.md. Map/state unchanged9693711a84a622e470d5e1ae492735ca2404fffe6b7778d316054494c7c21637. Structural terminalINCOMPLETE33unknown/16spans/99.99% remains; root objective/document-selection acceptance pending. No new toolkit issues; no archive/cleanup/publication.


## Coordinator terminal handoff — round006

Structural terminal INCOMPLETE accepted after compact rejection review and decisive primary inspection. ROM classification:99.99% (262127/262160),33Unknown bytes/16spans; no ready leads or justified useful revised test. Final checkpoint maps/round-006-disassembly-final.json SHA9693711a84a622e470d5e1ae492735ca2404fffe6b7778d316054494c7c21637. Final source/editorial audit passed byte-identically; all5252reviewrows and76639Codebytes covered bymeaningful reviewedunits, readingcurrent. Rootfinalchecks analysis/final-coordinator-verification.json; requirementaudit analysis/coordinator-acceptance.json. Finalhandoff analysis/final-handoff.md. Archiveoptions final:70Markdownaccounted,45additionalselected,15verifiedduplicateguidancecopies omittedfromselection butretainedlocally,0unrecognized. Orchestrationobjectivefulfilled throughdocumentedterminalincomplete, notwholeROMcompletion. Noarchive/cleanup/deletion/publication/toolkitedits.
