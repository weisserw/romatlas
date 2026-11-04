# Disassembly progress

Workspace initialized. Baseline and classification have not run.

## Round 1 — disassembly active
Baseline established through `run.py baseline`; maps/baseline.json and analysis/baseline-verification.json confirm byte identity. Configured release binary retained. ROM 262160 physical bytes: 16-byte header, sixteen 8 KiB PRG banks, sixteen 8 KiB CHR banks (mapper 1). No classification committed yet. Active server subprocess is driven by analysis/scratch/session.py via bounded JSON file requests; no toolkit modifications.

Frontier: title identifies Adventures of Lolo II; initial startup/contextual fetches and vector discovery available. Durable title snapshot analysis/title.snapshot.json; corrected NES controller API uses `buttons:[...]`, not generic GB button/pressed examples. Initial apparent failed Start attempts therefore supplied no buttons; corrected Start triggered bank [12,13,14,15] transition. Gameplay transition exploration continuing. Snapshot analysis/game.snapshot.json is transitional black screen, not proof of level visit. Scratch captures/evidence will be promoted selectively before handoff. Map classification, broad progression, state save/export and pass-001 checkpoint remain pending.

Title correction and gameplay frontier: Adventures of Lolo III (western 1991 title), **not** II. See analysis/gameplay.md. Natural route: title snapshot at frame288, Start held2/released45 then released180 -> overworld; A2/release30 swapped character, Up20/release5/A2/release30; Right60/Up60/release10 -> first tree exterior by frame734; release120; Up100/release180 -> character at tree door (frame1149). A presses are now being tested. Known RAM contract still under investigation; later-state passwords provide a practical shortcut path. Static/observed discovery produced 3434 unique physical instruction starts with no overlapping boundaries and ~4922 execution views. Direct JSR callees followed under explicit effective bank contracts without assuming return banks. No code annotations applied yet; candidate artifact analysis/scratch/candidate-closure.json.

Intermediate verified checkpoint pass-001-intermediate: 156635 classified bytes, 105525 unknown; structural only, phase active. State analysis/state.json, portable maps/current.json, verification analysis/pass-001-intermediate-verification.json. Broad progression continues.

Password breakthrough: final King password accepted after correcting `$E9..$F8` to indices in 32-byte valid alphabet at physical `$53D2..$53F1`. ASCII `$600..$60F` is display only. Valid symbols `23456789BCDFGHJKLMNPQRSTVWYZ`, then plus `$D7`, heart `$D6`, diamond `$D5`, triangle `$D4`. Keyboard `$7A` x, `$82` y; END x187/y124. Entry count `$93=16`. First attempt wrote only ASCII; second used wrong 40-key alphabet indices; both rejected on END and are not stage visits. Corrected King bytes at `$E9`: 2,21,2,18,22,19,10,10,11,29,16,2,3,8,23,14; accepted transition to underground final castle overworld, visibly confirmed in analysis/scratch/kingworld2.png; durable analysis/kingworld2.snapshot.json. Artificial reachability only. Attempting final castle entry/combat next; then representative caves/underwater.

Underwater milestone verified: password `V4Y♦23G△5J♦+JNDN` accepted; Up60/release180 reached level09 pyramid exterior. Left10/Up90/release180 then Right40/Up40/A2/release10/Select2/release50 reached actual LVL09 RM1 puzzle with seaweed, skulls, hearts, ladders, underwater graphics; effective banks [10,11,14,15]. Further traced Up35, Right+A35, Select2/release30 exercised movement, action and restart. See analysis/underwater-actual.snapshot.json and analysis/scratch/underwater-play.png / underwater-mechanics.ppm. All this route is artificial after password RAM patches. Natural tutorial dialogue and artificial underwater runtime evidence are distinct.

King entry obstacle: accepted King password reliably shows underground final map, but Up60/release180/Up90/release180, alignment Left10/Up60/release180, A2/release5/Up100/release180, and fresh-snapshot Left60/Up60/Select2/release400 leave the map unchanged. No King combat/ending visit claimed. Stop input permutations here; missing fact is correct final-castle entrance/progression consumer contract, for redirect. A cave-boss password is the next representative alternative.

Connected static batch expanded to 6733 physical starts (10917 bank/CPU views), no overlapping instruction boundaries. Beyond direct JSR callees, added continuations after indexed stores only where the entire index domain remains below mapper address space (zero-page wrapping or absolute base <=$7F00). Added return continuations only with complete bounded decoded callee CFGs that include RTS and exclude unresolved control/calls, mapper-affecting stores and indirect writes. Proofs retain physical offset, CPU alias and all four effective slots; no inferred caller-return bank across unresolved routines. Artifacts/scripts in analysis/scratch/classify.py and pure_returns.py/pure-return-proofs.json will be promoted. Gameplay sample additions will be merged before final classification.

Static pre-annotation review tightened return summaries: bank-preservation alone is insufficient because inline-data consumers can rewrite return addresses. The initially proposed 438/487 caller continuations were not committed; reanalysis now additionally excludes all explicit stack operations and writes to stack RAM/aliases, and requires a reachable RTS. Candidate closure is regenerated from observed/vector roots without those earlier return proposals before deriving the stricter connected batch. Original safe indexed-store/direct-callee evidence remains valid; no speculative return continuation is retained merely from the earlier candidate dump.

## Round 1 — disassembly complete, ROM incomplete

ROM classification: 62.26% (163209 / 262160 bytes classified; 98951 bytes unknown).

Unique classified bytes: Code **13404**, Data **18733**, Image **131072**; unknown **98951**, across193 exact gaps. No explicit Unknown regions or raw export fallback are counted as classified. Header16, PRG131072 and CHR131072 are all included in denominator262160. Full physical counts and exact unknown complement: [counts](disassembly-001/counts.json), [unknown ranges](disassembly-001/unknown-ranges.json).

Final bank-aware static closure:6667 physical instruction starts,10926 execution views, no overlapping starts.363 strictly proven direct-call return continuations reached a fixed point; unresolved stack/inline-data/mapper/indirect cases stopped. Saved accepted fetch aggregates contain5203 unique physical ROM opcode starts (8264 CPU/bank roots normalized across provenance), with3087 naturally observed and4484 artificially observed starts,2368 shared. Natural decoded instruction bytes6258 and artificial8939 overlap; total Observed-classified bytes9916. This is execution evidence, separate from whole-ROM structural percentage. Static-only classified bytes153293 include CHR/header/data as well as code.

All696 regions have structural evidence; code regions carry explicit physical/CPU/bank execution views. This is not full source commentary/readability acceptance. Generated source still requires the coordinator's full current-source commentary phase.

Verified checkpoints: [raw baseline](baseline-verification.json), [intermediate](pass-001-intermediate-verification.json), and fresh final [pass-001](pass-001-verification.json), all `ok=true`, `byte_identical=true`. Final [state](state.json), [portable map](../maps/current.json), frozen checkpoint [map](../maps/pass-001.json), [source](../generated/pass-001/game.asm). Final map SHA256 `e88689f5101f8e2958738e0e2d68b78b51cea4c1b0dc391c30e753db179ed009`; input SHA256 unchanged `59f5a7d9bd4f89875465ecb55a7ef20abd3417f4c6a72b66da50652adb021a58`. Toolkit commit `3d0d1393149491777626f5a8e4461f7c49b2d6df`, configured binary hash `8ff56b0b1cf8d2a3aca5430c70d34280ae3db36fc6774919062db4ea63843f82`; no toolkit edits/rebuilds.

Gameplay checklist and exact natural/artificial routes/patches: [scenarios](scenarios.json), [research](gameplay.md). Natural tutorial and artificial underwater09 room1 visited; caves/final King/level17 shortcuts accepted to maps/exteriors but entry/ending obstacles documented. No boss or ending visit claimed. Do not repeat the same failed directions; investigate map-node/progression consumers and a specific discriminating route/state contract first.

Toolkit findings: **one confirmed documentation/API-contract discrepancy**, NES-DOC-001, no blocker; [report](toolkit-issues.md). Persistent debugger subprocesses ended143 twice; saved map/context artifacts survived. Final classification used direct bounded server batches and saved fetch roots, without gameplay replay. Old full trace summary unavailable; restart summary has aggregate_dropped0 and detail capacity0 (dropped detail expected), saved at analysis/scratch/restart-trace-summary.json. No claim of complete historical event capture.

Stopping rationale: startup/observed roots and connected safe-static closure exhausted under current contracts; representative gameplay/shortcuts supplied useful evidence but additional cave/King direction permutations stopped being discriminating. Missing indirect/inline-data/map-entry contracts belong to redirect, not speculative code/data annotations. This is an **ordinary incomplete easy-gain handoff**, not terminal rejection/exhaustion or full-ROM completion. Compact handoff: [disassembly-handoff](disassembly-handoff.md). No commentary/redirect/archive/publication/cleanup performed.

Per-bank physical accounting (Data/image includes all non-code structurally classified kinds):

| Space/bank | Code | Data/image | Classified | Unknown |
|---|---:|---:|---:|---:|
| header | 0 | 16 | 16 | 0 |
| PRG 00 | 1607 | 0 | 1607 | 6585 |
| PRG 01 | 0 | 3209 | 3209 | 4983 |
| PRG 02 | 498 | 112 | 610 | 7582 |
| PRG 03 | 0 | 1109 | 1109 | 7083 |
| PRG 04 | 0 | 0 | 0 | 8192 |
| PRG 05 | 81 | 1578 | 1659 | 6533 |
| PRG 06 | 0 | 0 | 0 | 8192 |
| PRG 07 | 0 | 1850 | 1850 | 6342 |
| PRG 08 | 0 | 0 | 0 | 8192 |
| PRG 09 | 0 | 2792 | 2792 | 5400 |
| PRG 10 | 2801 | 0 | 2801 | 5391 |
| PRG 11 | 1213 | 1822 | 3035 | 5157 |
| PRG 12 | 1835 | 0 | 1835 | 6357 |
| PRG 13 | 0 | 2839 | 2839 | 5353 |
| PRG 14 | 4638 | 0 | 4638 | 3554 |
| PRG 15 | 731 | 3406 | 4137 | 4055 |
| CHR 00 | 0 | 8192 | 8192 | 0 |
| CHR 01 | 0 | 8192 | 8192 | 0 |
| CHR 02 | 0 | 8192 | 8192 | 0 |
| CHR 03 | 0 | 8192 | 8192 | 0 |
| CHR 04 | 0 | 8192 | 8192 | 0 |
| CHR 05 | 0 | 8192 | 8192 | 0 |
| CHR 06 | 0 | 8192 | 8192 | 0 |
| CHR 07 | 0 | 8192 | 8192 | 0 |
| CHR 08 | 0 | 8192 | 8192 | 0 |
| CHR 09 | 0 | 8192 | 8192 | 0 |
| CHR 10 | 0 | 8192 | 8192 | 0 |
| CHR 11 | 0 | 8192 | 8192 | 0 |
| CHR 12 | 0 | 8192 | 8192 | 0 |
| CHR 13 | 0 | 8192 | 8192 | 0 |
| CHR 14 | 0 | 8192 | 8192 | 0 |
| CHR 15 | 0 | 8192 | 8192 | 0 |


Execution-view accounting note:10926 refers to candidate instruction views in the durable closure evidence. Exported code groups use a validated primary execution view; additional observed/static CPU aliases and bank contexts remain preserved in code-candidates.json and contextual-fetches.json rather than implying every group carries10926 views.

## Round 1 — commentary active

Input checkpoint pass-001; maps/current.json matches maps/pass-001.json. First commentary preparation created independent source and frozen before/baseline/state. Preparation verification passed byte identity. No prior commentary existed. Full classified-source readability remains active, separate from the 62.26% structural classification. Direct source review currently covers code physical457..2317 (audio fragments) and20812..23618 (password/UI fragments); named mechanics and summaries entered in source/review. Non-code/source gaps and remaining code43407..131072 are pending, as are consistent RAM/control-target symbols and a second readability check of these initial edits. Do not redirect from this partial frontier. All 696 classified regions and193 unknown gaps must be accounted for before success. No final editorial audit performed.

Commentary coherent batch2, **same phase active / incomplete**: all classified code before physical98672 now has direct mechanics prose and descriptive region names (6200/13404 code bytes); all18733 Data,131072 Image and98951 unknown bytes have reviewed structure/boundary notes. Exact remaining373 code intervals/7204 bytes: [commentary frontier](commentary-001-frontier.json), first98672..98681, then103703..106xxx and fixed banks14–15 through131078. Gameplay banks10/11 include room setup, movement/collision/entity handling, packed map format, palette animation and sprite output. Audio/password/gameplay RAM and hardware symbols installed;480 branch/call operands became bank-context-safe symbols and184 additional interior targets were named. All source constants retain exact values and address-size prefixes.

Fresh partial verification [report](commentary-001-partial-verification.json) passes `ok=true,byte_identical=true`, source SHA256`f6764a50ac6d5ad14aea61604a3890ad690781dba6c0fbf2d81e5640c9047399`. Refreshed commentary/reading-after.asm. Maps/current.json==maps/pass-001.json and input SHA256 unchanged. [Scope assessment](commentary-001-scope.json) explicitly sets readability_complete=false. No final audit or redirect transition. Bank12 physical98672..106147 additionally read provisionally; no review credit or source commentary installed yet. Continue bank12 from the frozen source reading, then fixed-bank code; inspect concrete dependencies and symbolic secondary entries before the final gate. Promising map-node hitbox/progression consumer recorded in commentary/uncertainties.md for eventual redirect, without pausing commentary.

Storage check:9.4GiB free; analysis/scratch14MiB. No deletion/cleanup, archive, publication, ROM or toolkit modification.

## Round 1 — commentary complete; ROM classification remains incomplete

Full current-source readability completed in the same phase, preserving the earlier6200-byte review and reviewing remaining7204 code bytes. All13404 Code/18733 Data/131072 Image bytes plus193 unknown-gap boundary notes are covered;889/889 scaffold entries reviewed, no pending classified source. [Scope](commentary-001-scope.json), [final frontier](commentary-001-final-frontier.json), [handoff](commentary-handoff.md). Historical partial frontier/scope/handoff retained separately. Final editorial acceptance is pending; no final audit/archive execution.

Source now explains audio/password, eight-actor and32-entity systems, map-node hitboxes/completion gates, script/software stacks and inline CPU-return rewriting, PPU decoding/queued writes, controllers/NMI, MMC1 registers/reset, palettes/attributes/dialogue and puzzle row compression. Descriptive routine/secondary-entry/control targets, RAM arrays/fields and hardware constants retain exact original values/address modes. Unresolved song/entity/text identities and progression meanings use mechanical or tentative prose. No invented code in unknown gaps; reachability and bank context remain separate.

Fresh [final commentary verification](commentary-001-verification.json):ok=true,byte_identical=true, source SHA25683d224cc948e45460fdc28d4b585b0bbd187a35db3e139d3e5cca611471a57b5; commentary/reading-after.asm refreshed. input/game.nes hash unchanged; current map identical to pass-001. Structural classification remains62.26%/98951 unknown. Toolkit finding NES-DOC-001 unchanged/nonblocking; no toolkit edits. Redirect can now use the concrete map-node eligibility/entrance contract documented in commentary/uncertainties.md; commentary completion does not claim gameplay/whole-ROM/final editorial completion.

## Redirect round1 active — r1-node-and-residual
Input pass-001, commentary current-source complete; maps/state/source unchanged. Unique research directory analysis/redirect/r1-node-and-residual. Entrance investigation changes prior blind-input method: derive exact node partitions and rectangle coordinates from bank12$991D consumers, compare saved state completion nibbles, then place actor in an incomplete matched node and apply normal inputs. Initial snapshot/RAM inspection preserved in initial.json. Actor selection is transient at frame boundaries; use player object0/companion1 rather than treating saved SelectedObject8 as current player. Independent residual structural review follows this graph. Disk9.4GiB free, scratch14MiB; no cleanup performed.

Redirect entrance breakthrough: exact partitioned rectangle domain0..18 established. King/node18 is incomplete; position-only patch did not enter, cave/node5 position patch reached cave exterior and set flag80. Improved supported shortcut bypasses actor animation/collision: restore accepted map, release buttons, set RAM$014F=node and$014E=$20. Fixed$C0C6 sees transition flag20; $C0D0 uses selected node through ordinary dependent initialization, then actual execution changes banks[8,9,14,15]. King/node18 visibly LVL17 crowned boss, cave/node5 visibly LVL04 green boss, level17/node17 initialized. Evidence: analysis/redirect/r1-node-and-residual/transition-results.json, named transition snapshots/PNGs and transition.py. Neither patch nor password proves natural reachability. Combat sampling (A+Up30, A+Right30, A+Left30,release30) exercises live combat and gathers paged contexts/edges in combat-results.json. Ending frontier: actor script controller at bank8$803A/818E; predicates$AF7E/$AF96 inspect progression flags40/80 and node18; investigate actual victory writers before further artificial edits. Independent frontier: node coordinate/config/render arrays have domain19; inline PPU helper$D439 consumes exactly two inline pointer bytes and returns after them, exposing supported three-byte LDX/RTS tails and six4x4 render blocks. No map/state/source changes.

King ending investigation refined: bank8 callback$9348 uses$AF2A bullet1 proximity and$AFA0 removes bullet/decrements selected actor$90 (HP), switching script on zero; $AEC0 first checks player collision. Prior combat snapshot already has death flag$52 bit80 and an in-flight store (collision-trace.json begins$AF6F after that writer), so naive HP/flag edits there replay death/reset. Preserve those negative results: victory-results.json, victory-engaged-results.json, hit-results.json, hit-isolated-results.json, hit-cleared-results.json. Materially improved method starts clean transition,Up10/release1: king-clean-engage.snapshot.json has flag16 and active boss. Restore clean state, set HP$93=1, install stationary active bullet1 at boss3 X/Y with script channels disabled, move player away and disable unrelated projectiles. Actual callback decreases HP to0, then script switches$92F6->$9396->$939C and native callback$9348->$93B6 while HP resets10: confirmed second King phase, not ending. Evidence hit-clean-results.json and king-hit-clean17 frame/snapshot. Next supported continuation is another same-contract hit against phase2 callback$93B6, then inspect each changed phase/callback for decrement/victory contract rather than blindly editing ending PC. Static$AF7E at node18 returns carry CLEAR (branch lands$AF8B CLC), correcting a provisional predicate interpretation; node18 is deliberately excluded from ordinary flag40 completion predicate.

King multiphase continuation succeeded with validated repeated hit contract: callbacks$93B6->$94BB->$950E, animation callback$95A9, later$96A7 then$C8FC; phase HP reset10 is expected. phases-results.json records each hit and30-frame samples. King disappears, progression flags become$70. Exit consumer$B0D6 compares active playerY against$1C; positionX120/Y32 plusUp did not advance, but exact gate positionX120/Y16 did. Actual map controller selected transition to banks[4,5,14,15], visibly final rescue room with pink Lala (ending-gate-5.png). Rescue interaction follows from saved ending-gate5 snapshot. All artificial edits enumerated in scripts/results; natural reachability unclaimed.

Independent structural work now packaged for review: structural-proposals.json has44 claims/678 bytes (660 Data/18 Code), derived node-table19-domain, six4x4 PPU block bounds/inline pointer skip and return tails, fixed actor split dispatch arrays, independent audio period/register/stack data. observed-entry-roots.json currently3322 execution views/833 newly fetched unknown physical starts from bounded boss/late-room scenarios; new ending/rescue roots will be appended. These are proposals only; disassembly must independently validate and classify. Current map SHAe88689f5101f8e2958738e0e2d68b78b51cea4c1b0dc391c30e753db179ed009 unchanged.

Independent container proofs persisted: graphics-streams.json derives36 descriptors from first source$8090; decoder-exact36 streams tile35 successive endpoints with FF terminators and consistent screen output sizes. New14518 bytes; finalFF is already classified tail. Audio metadata inspection validates all24 music/49 effect counted lists, legitimate bank0 stream pointers/APU slots, aligned49 mask/priority arrays. Combined with period/register arrays and repeated control/event corpus, remaining bank0$89BD..B367 is proposed generic noncode (no exact individual-stream layout claim); evidence audio-*-lists.json, audio-pattern-samples.json. Structural-proposals current25862bytes before room proofs.

Further independent promising avenue pursued rather than stopping at graphics: fixed$C4F7 thresholds40/60/80/100/110 select PRG registers1/2/1/6/6; fixed$ED31 uses110 split low/high source pointers at$EDB6/$EE24 and decodes15 sixteen-cell rows. rooms.py models extra final run byte exactly (F0..FE lengths2..16; FF consumes count+2). All110 records decode fifteen exact16-byte rows,107 adjacent same-bank ends equal next source, no gaps/overshoots,15576 source bytes. Room-table/stream proposals being incorporated; endpoints and patterns, not mere pointer values, establish data. Ending autoplay progressed rescue/cutscenes through area13/14/15 and sunset scene, captured credits*.png; final credits/The End pending ongoing bounded autoplay.

Redirect ready combined handoff: THE END visibly reached in credits3-59.png after rescue/story autoplay; broad ending milestone visited artificially. Downstream room rendering investigation completed45-entry reverse mappings,15x240 tile/attribute arrays and86-byte palette schedule graph (boolean tile0F selector, explicit7/9 lengths, eleven4-byte palettes). Final static manifest303 claims/45444 bytes,3824 contextual roots/860 newly fetched unknown starts. Generic noncode audio blocks remain separately scoped from specific request/header formats. No proposed data/fetch conflicts; no overlapping proposals. Supported native callback contract roots and decisive negative traces retained. Full handoff analysis/redirect/r1-node-and-residual/HANDOFF.md; summary analysis/redirect-handoff.md. Current193-range/98951-byte ledger maintained from unchanged pass-001 map. This is ready for disassembly review, not terminal exhaustion; no accepted map/source/state edits in redirect. No new toolkit issues; NES-DOC-001 nonblocking. Attempt400MiB/scratch14MiB, disk9.0GiB free; no cleanup performed.

Final bounded reset-copy check adds112 bytes to ready batch: seven16KiB tails identically encode INC$BFF0/JMP$E207 plus ten-byte patterned footer containing BFF0 originword. Explicit mode3/effectivebank entry preconditions support static reset code while actual entry remains unclaimed; preserves distinct$BFF0/$FFF0 aliases. Final totals317 nonoverlapping Static claims/45556 bytes (45496Data/60Code),860 new unknown fetched starts. Handoff/leads/ledger/checks reconciled to final totals. Prior303/45444 summary is an intermediate checkpoint, preserved historically.

## Round 2 — disassembly active
Input pass-001 (62.26%, 163209 classified / 98951 unknown). Historical map/state preserved in analysis/disassembly-002/before-*.json; commentary and prior source remain untouched. Independent review frontier: 317 static proposals, finite graphics/room decoder contracts, node/PPU inline tables, audio generic noncode separately from schemas, reset mirror mapper contracts, then contextual fetched roots and strict connected code closure. Saved broad gameplay through THE END reused; no startup replay. Output checkpoint will be fresh pass-002. Disk9.0GiB free; scratch14MiB, no cleanup.

Round2 coherent accepted batch:317 proposals45556bytes independently checked and accepted; new contextual/static code2723bytes/1400 starts/125regions;32 inlinebytes plus26 audio register/mask bytes; connected data3532bytes (half-square512 exact floor(n²/2), reciprocal254, fourteen collision bitplanes, six31byte actor template fields, bounded printable dialogue1716bytes/two pointertables and supported PPU rectangles). Internal state/maps current saved and validated. Current ROM classification: 82.04% (215078 / 262160 bytes classified; 47082 bytes unknown). No completion claim; final exact counts/checkpoint pending. Small directly consumed numeric/direction/tile arrays now bounded review frontier; opaque script/native mixed ranges deferred to refreshed ledger. Saved broad gameplay through THE END reused without replay; old source/commentary preserved.

## Round 2 — disassembly pass complete, classification incomplete

ROM classification: 82.21% (215534 / 262160 bytes classified; 46626 bytes unknown).

Added52325 unique physicalbytes:2783 Code,49542 Data. Totals Code16187,Data68275,Image131072;169 unknownranges/46626bytes,1206 classifiedregions. Exact unique accounting and perbank counts: [counts](disassembly-002/counts.json); exact complement [unknownranges](disassembly-002/unknown-ranges.json). No unknown/raw export bytes count towardclassification.

All317 redirect proposals/45556bytes independently accepted after exactbyte/physicalbank/nonoverlap checks and consumer review. Independent graphics36/room110 reparsing reproduced endpoints; generic audio noncode distinct from individualstream/table semantics; resetcopies require explicit mode3/effectivebank/BFF0alias contract. Strict contextual/static callback/directcall/indexsafe/return closure has8079 physical instructionstarts/13482 views, no overlappingstarts or Data conflicts;1400 newstarts/2723bytes.476 stack-safe return summaries,22 narrowly bankedcall contracts (deduplicated contractrecords may include round repeats), inline-wordskip contracts preserve barriers. Observed provenance11583bytes totals actual accepted fetches; static203951bytes include code/data/image, and neither is wholeROM reachability.

Connected work beyond originalmanifest accepted4046 extraData:32 inlinewordsbytes,26 audio register/maskbytes,3532 exacthalf-square/reciprocal/collision/template/dialogue/PPU bytes,456 direction/glyph/angle/attribute numericbytes. All consumer and finiteformat reviews in disassembly-002; overlapping oldregions preserved. Broad saved gameplay checklist through THE END reviewed and reused without replay.

Fresh [pass-002 verification](pass-002-verification.json) ok=true,byte_identical=true; [frozen map](../maps/pass-002.json), [source](../generated/pass-002/game.asm), currentstate/maps saved and validated. MapSHA256 0f7d34a9b9671ad226a8c8fdc207d6028d4daa3033f0b008f9b8f10824d0aebb; inputSHA256 unchanged 59f5a7d9bd4f89875465ecb55a7ef20abd3417f4c6a72b66da50652adb021a58. Old commentary/sourcehash83d224cc948e45460fdc28d4b585b0bbd187a35db3e139d3e5cca611471a57b5 and all oldcheckpoints/history preserved. New source needs its own full commentaryreview; prior gate does not cover newbytes. No finaleditorial acceptance.

Lead outcomes: R1-NODE-DATA,GRAPHICS-CONTAINERS,ROOM-CONTAINERS,AUDIO-CORPUS,BOSS-ENDING-CODE,ACTOR-DISPATCH,RESET-MIRRORS verified; prior unresolvedPRG status superseded by freshledger, R2-MIXED-SCRIPTS-AND-CALLBACKS open research. Ready finitebatch/connected easygains exhausted for thispass; remaining opaque scripts/native mixedranges lack bounded interpreter/entry contracts. This is ordinary incomplete handoff, not terminal exhaustive rejectionreview. Unique [handoff](disassembly-002/HANDOFF.md); old disassembly-handoff preserved.

Toolkit: existing [NES-DOC-001](toolkit-issues.md) documentation/APIcontract issue remains nonblocking; no newissues, no toolkitedits/rebuild. Bounded subprocesses stopped; disk9.0GiB free, scratch14MiB, round2 durable evidence6.8MiB before finalnotes. No cleanup/deletion/archive/publication.

| Space/bank | Code | Data/image | Classified | Unknown |
|---|---:|---:|---:|---:|
| header | 0 | 16 | 16 | 0 |
| PRG00 | 1617 | 5917 | 7534 | 658 |
| PRG01 | 6 | 8186 | 8192 | 0 |
| PRG02 | 498 | 1086 | 1584 | 6608 |
| PRG03 | 6 | 8186 | 8192 | 0 |
| PRG04 | 0 | 0 | 0 | 8192 |
| PRG05 | 87 | 4623 | 4710 | 3482 |
| PRG06 | 0 | 8192 | 8192 | 0 |
| PRG07 | 6 | 8186 | 8192 | 0 |
| PRG08 | 341 | 10 | 351 | 7841 |
| PRG09 | 1300 | 2892 | 4192 | 4000 |
| PRG10 | 3154 | 138 | 3292 | 4900 |
| PRG11 | 1219 | 5651 | 6870 | 1322 |
| PRG12 | 1854 | 445 | 2299 | 5893 |
| PRG13 | 6 | 8186 | 8192 | 0 |
| PRG14 | 5315 | 823 | 6138 | 2054 |
| PRG15 | 778 | 5738 | 6516 | 1676 |
| CHR00 | 0 | 8192 | 8192 | 0 |
| CHR01 | 0 | 8192 | 8192 | 0 |
| CHR02 | 0 | 8192 | 8192 | 0 |
| CHR03 | 0 | 8192 | 8192 | 0 |
| CHR04 | 0 | 8192 | 8192 | 0 |
| CHR05 | 0 | 8192 | 8192 | 0 |
| CHR06 | 0 | 8192 | 8192 | 0 |
| CHR07 | 0 | 8192 | 8192 | 0 |
| CHR08 | 0 | 8192 | 8192 | 0 |
| CHR09 | 0 | 8192 | 8192 | 0 |
| CHR10 | 0 | 8192 | 8192 | 0 |
| CHR11 | 0 | 8192 | 8192 | 0 |
| CHR12 | 0 | 8192 | 8192 | 0 |
| CHR13 | 0 | 8192 | 8192 | 0 |
| CHR14 | 0 | 8192 | 8192 | 0 |
| CHR15 | 0 | 8192 | 8192 | 0 |

## Round 2 — commentary active
Preserved complete prior commentary under analysis/round-2/commentary before fresh pass-002 preparation. Active frontier: transplant unchanged physical sections and review all added/split ranges plus changed dependencies. Prior review remains valid where mechanically identical; no readability completion yet.

## Round 2 — commentary complete; classification incomplete

Full current-source readability for pass-002 is complete: 16187 Code/68275 Data/131072 Image bytes and 169 unknown-gap notes. All 1375 scaffold entries reviewed, zero pending. Preserved old commentary under analysis/round-2/commentary; carried all 696 unchanged classified regions and reviewed all 510 additions plus changed consumers. Unique scope/verification/handoff: analysis/commentary-002/. Current source SHA256 c47a0895d2e576321ce33bf333b1e18691791edc4716cdfa2e3ee3810bec506d; fresh byte-identical verification passed and reading-after.asm refreshed. Maps/state/pass-002 and input identity unchanged.

New readable behavior covers boss HP/contact/exit predicates, signed actor spawn and angle arithmetic, puzzle power/timed-cell paths; new finite data structures and generic audio uncertainty are explained beside the data. Consistent CPU aliases preserve physical data labels. Mixed scripts/native callbacks remain a redirect frontier, not skipped commentary. ROM classification remains82.21%/46626 unknown; final editorial acceptance pending, no final audit. No new toolkit issue; NES-DOC-001 remains nonblocking. Disk9.0GiB free/scratch14MiB; no deletion/cleanup, toolkit edit, archive or publication.

## Active round2 redirect

Worker research continues in [r2-actor-contracts/PROGRESS.md](redirect/r2-actor-contracts/PROGRESS.md), input pass-002. Finite interpreter/native entry contracts and bounded script parsing; no classification edits or terminal assessment. Current2118 script records cover5503 unknown bytes without code conflicts; native/record validation and independent avenues remain active.

Round2 redirect active milestone: [proposal summary](redirect/r2-actor-contracts/proposal-summary.json) now39943 candidate unknown bytes (33669Data/6274Code), zero current-map conflicts; prospective6683 residual. Allocation producer and inline-mode1 contracts resolved; active remaining method/intervals in [PROGRESS](redirect/r2-actor-contracts/PROGRESS.md). Still research, not accepted classification or phase completion.

## Round2 redirect complete — ordinary-ready

Combined lead `R2-MIXED-SCRIPTS-AND-CALLBACKS` ready, attempt `r2-actor-contracts`. 46583 unique unknown bytes proposed Static (37716 Data/8867 Code), accepted0. Current map remains215534classified/46626unknown; prospective residual43bytes/15fragments. [Handoff](redirect/r2-actor-contracts/HANDOFF.md), [reconciled current169-range ledger](redirect/r2-actor-contracts/unknown-ledger.json), [preflight](redirect/r2-actor-contracts/proposal-preflight.json). Next: separate independent disassembly validation/classification, byte-identical checkpoint, full current-source commentary. Finaleditorialpending; no terminalexhaustionclaim.

## Round 3 — disassembly active
Input pass-002 preserved; full current commentary copied to analysis/disassembly-003/preserved-commentary before any refresh. Frontier: independently validate R2-MIXED-SCRIPTS-AND-CALLBACKS exact roles, bank aliases, starts and return/inline contracts. Classification remains82.21% (215534/262160;46626unknown). No annotation yet.

## Round 3 — disassembly pass complete; classification incomplete

ROM classification: 99.98% (262120 / 262160 bytes classified; 40 bytes unknown).

Added46586 unique Static bytes:8870Code/37716Data. Totals Code25057/Data105991/Image131072;1856classified regions,40unknownbytes/12ranges. All1206 oldannotations retained byte-for-byte includingIDs/evidence/provenance/views. Unique counts/perbank: [counts](disassembly-003/counts.json); exact physical complement+rawbytes: [unknownranges](disassembly-003/unknown-ranges.json). Observed11583bytes unchanged, Static250537bytes; execution coverage and natural reachability are separate fromclassification.

R2-MIXED-SCRIPTS-AND-CALLBACKS verified: all46583 proposedbytes independently accepted. Fresh configured-binary decode checked12315 proposedphysicalstarts/19496views; 6633script/context records and frame/composite/rectangle/palette/container formats independently reparsed against immutable ROM. All4236 proposednewstarts structurally connect to reviewed typed native/callback, finite interpreter/entity dispatch, manualdataflow and prioracceptedcode interfaces. These checks supplement finiteCPUstack/indirectRAM/bank/return contracts; legaldecode/pointers alone do not provecode. Native/manual semantic and exactbankreview: [contracts](disassembly-003/connected-contract-review.md), [formatcheck](disassembly-003/review-format-checks.json), [connectedgraph](disassembly-003/connected-structure-audit.json), [acceptanceledger](disassembly-003/accepted-claim-ledger.json).

Connected continuation beyond manifest: lowcommand21CF31 field0..1 plusactor0..7 limitsdestination0090..009F; operator0..2 dispatches coherentAND/OR/ADC bodies whose STA cannot touchmapper/liveframes/scratch. Their exactRTS tails118661/118668/118676 independently accepted (3bytes), freshdecoded. No currently parsedcommand21/naturaluse claimed. Tightened D7C8 boundedordinaryreturn precondition to endlowbyte!=FF; role remains coherentfillloop. D433/D439/D43F/D10B preserve inline2bytebarriers, C119 haspartiallowercontext until literals, selector5 means10/11, C228successcalls8150, A536 aliascorrect. Partialselector71665 boundssevenwords despiteencodedcount10. Primarycorrections/history retained.

Fresh [pass-003 verification](pass-003-verification.json) ok=true,byte_identical=true; [frozenmap](../maps/pass-003.json), [generatedsource](../generated/pass-003/game.asm). State/current/pass003 areidentical; mapSHA25677e3a1d0900145aa22eeab2afdf67423bfd07b4b8585bd0fd2ba11ec86139d20. InputROM hashunchanged. Full priorcommentary/source/review preservedunder disassembly-003/preserved-commentary; sourcehashc47a0895d2e576321ce33bf333b1e18691791edc4716cdfa2e3ee3810bec506d. Old1375reviewed/0pending gate appliesoldsource only; fullpass003commentary andfinaleditorial acceptance pending. No commentaryrefresh orfinalaudit performed.

Broadscenarios throughKingphases/rescue/story/THE END reviewed/reused; no startup/endingreplay ornewruntimecoverage. Singlepassready/connected batchcomplete; unresolved40bytes retainambiguousstructure, includingD07Dsame-bytepointerprefix. No speculativeorphanPC, exhaustive residualsweep or terminalexhaustionclaim. Next separatecommentary, thenredirect withfreshcomplement. [Handoff](disassembly-003/HANDOFF.md).

Toolkit: existing [NES-DOC-001](toolkit-issues.md) remainsnonblocking; no newissues. No toolkit edit/rebuild, ROMchange, publication, archive, deletion/cleanup. Subprocesses stopped; disk8.9GiBfree/scratch14MiB/disassemblyevidence17MiB.

| Space/bank | Code | Data/image | Classified | Unknown |
|---|---:|---:|---:|---:|
| header | 0 | 16 | 16 | 0 |
| PRG00 | 1638 | 6554 | 8192 | 0 |
| PRG01 | 6 | 8186 | 8192 | 0 |
| PRG02 | 1139 | 7053 | 8192 | 0 |
| PRG03 | 6 | 8186 | 8192 | 0 |
| PRG04 | 10 | 8182 | 8192 | 0 |
| PRG05 | 108 | 8084 | 8192 | 0 |
| PRG06 | 0 | 8192 | 8192 | 0 |
| PRG07 | 6 | 8186 | 8192 | 0 |
| PRG08 | 1179 | 7010 | 8189 | 3 |
| PRG09 | 1872 | 6314 | 8186 | 6 |
| PRG10 | 7497 | 694 | 8191 | 1 |
| PRG11 | 1693 | 6496 | 8189 | 3 |
| PRG12 | 2127 | 6062 | 8189 | 3 |
| PRG13 | 6 | 8186 | 8192 | 0 |
| PRG14 | 6688 | 1480 | 8168 | 24 |
| PRG15 | 1082 | 7110 | 8192 | 0 |
| CHR00 | 0 | 8192 | 8192 | 0 |
| CHR01 | 0 | 8192 | 8192 | 0 |
| CHR02 | 0 | 8192 | 8192 | 0 |
| CHR03 | 0 | 8192 | 8192 | 0 |
| CHR04 | 0 | 8192 | 8192 | 0 |
| CHR05 | 0 | 8192 | 8192 | 0 |
| CHR06 | 0 | 8192 | 8192 | 0 |
| CHR07 | 0 | 8192 | 8192 | 0 |
| CHR08 | 0 | 8192 | 8192 | 0 |
| CHR09 | 0 | 8192 | 8192 | 0 |
| CHR10 | 0 | 8192 | 8192 | 0 |
| CHR11 | 0 | 8192 | 8192 | 0 |
| CHR12 | 0 | 8192 | 8192 | 0 |
| CHR13 | 0 | 8192 | 8192 | 0 |
| CHR14 | 0 | 8192 | 8192 | 0 |
| CHR15 | 0 | 8192 | 8192 | 0 |

## Round 3 commentary — active

Current-source readability phase started on pass-003 (1856 classified regions, 12 unknown ranges). Full pass002 commentary preservation verified byte-identical; retain matching review credit, review new 8870 Code/37716 Data bytes and affected contracts. Preparation/current frontier: analysis/commentary-003/scope.json. Final editorial audit stays pending; no redirect handoff until full readability and fresh verification.

### Round3 commentary applied-source milestone

All 662 new scaffold entries (222 Code,428 Data,12 Unknown) now have current-source explanations, retaining 1206 matching old reviewed sections. Added distinct native/entity/interpreter routine and branch symbols, mode-local RAM aliases, bank-local data CPU aliases, record-level script operations and finite sprite/composite/PPU/replay layout notes. Corrected password candidate Y reset, channel-wait semantics, palette operand order, connected-gap dependencies and DMC prefix. Fresh applied-source rebuild byte-identical (source SHA256 9cb507770898f551ac529e642d13702b02f5da1036f32a65e34fc3f4fd5bbfa7). No unread physical source range remains; source-location/unit reconciliation, final phase verification, reading-copy refresh and complete handoff remain. Frontier: commentary-003/remaining-frontier.json. Final editorial audit pending; same commentary phase continues.

### Round3 commentary complete

All current-source readability complete on pass003:1856 classified regions+12 unknown notes,1868 reviewed/0pending,2026 meaningful units with current locations. All222 new Code/428 Data sections and changed dependencies explained;1206 valid matching old sections carried. Exact remaining readability frontier empty. Fresh configured-binary verification ok=true,byte_identical=true; reading-after refreshed. Source SHA2569cb507770898f551ac529e642d13702b02f5da1036f32a65e34fc3f4fd5bbfa7. Map/state/pass003/baseline identities match; ROM unchanged. [Handoff](commentary-003/HANDOFF.md), [verification](commentary-003/verification.json), [inventory](commentary-003/inventory-check.json), [frontier](commentary-003/final-frontier.json). Separate redirect may resume; classification remains99.98%(262120/262160),40unknown bytes/12ranges. Final editorial audit pending/not run; no terminal-exhaustion claim. Existing nonblocking NES-DOC-001 only; no new toolkit issue. No runtime replay/toolkit edit/rebuild/archive/publication/deletion/cleanup; local source/assets retained.


Round 3 redirect active: analysis/redirect/r3-residual-contracts/PROGRESS.md; exact 40-byte complement under local contract review, no map changes.


### Round3 redirect complete — ready batch

Inputpass003/current readability complete; attempt analysis/redirect/r3-residual-contracts. AddedreadyR3-RESIDUAL-CONTRACTS:34uniqueStaticbytes proposed (31Code/3Data),0accepted inredirect; current262120classified/40unknown unchanged. Sixphysicalclaims:phase-duration3bytes,queue-discardhelper5bytes,duplicatefailureepilogues8bytes,typedD07Dwrapper18bytes. Exactcurrentcomplement refreshed; prospective6unknown iff allaccepted. Candidate-specificindependent constraints and competinginterpretations inrole-proofs.md, proofs/recipes/ledger/primarylinks inHANDOFF.md. Earlier lackcaller/selector judgments localized/corrected; no runtime replay/artificialPC claims. All concrete identified residual avenues assessed; no productive connected continuation left deferred. Ordinaryreadyhandoff, notterminalexhaustion. Nextseparatedisassembly independently reviews combinedbatch, then fullcurrent-sourcecommentary/redirect. Currentmap/state/pass003 and sourcehashes unchanged; finaleditorialauditpending. ExistingNES-DOC-001 nonblocking, no newtoolkitissue; serversstopped; disk8.9GiBfree/scratch14MiB; no deletion/toolkitedit/rebuild/ROMchange/publication/archive.

## Round 4 — disassembly active

Independent review of R3-RESIDUAL-CONTRACTS against immutable physical bytes, current source and typed consumer contracts. Input pass003 and full current commentary preserved in disassembly-004; no annotations yet. Classification 262120/262160,40unknown. Finite roles and actual reachability reviewed separately; isolated returns remain ambiguous.

## Round 4 — disassembly complete; ROM incomplete

ROM classification: 100.00% (262154 / 262160 bytes classified; 6 bytes unknown).

Exact percentage 99.99771132133048%; rounded100.00does not mean complete. Added34Staticbytes31Code/3Data in6regions; totalsCode25088/Data105994/Image131072,1862regions. Preserved all1856oldregion objects byte-for-byte. Counts/perbank: [counts](disassembly-004/counts.json); complement [unknownranges](disassembly-004/unknown-ranges.json). Observed11583unchanged; no newruntime or naturalreachability evidence.

R3-RESIDUAL-CONTRACTS verified/consumed ready=false; six independent role proofs accepted after fresh contextualdecode and phase/branch/queue/typed-record checks. [Review](disassembly-004/role-review.md), [acceptance](disassembly-004/accepted-regions.json). Connected consumer/renderer/entity/queue graphs alreadyclassified; five loneRTS andzero retain separate ambiguity. No forcedcompletion/terminalexhaustion claim. Prior broadgameplay throughTHEEND reused without replay.

Fresh [pass004verification](pass-004-verification.json):ok=true,byte_identical=true; state/current/pass004 identical mapSHA2569b4691491780172c94178a7c5a1fad550a10f91c0b0f71302ee664d76c486382. ROMunchanged. Full commentarypreserved [here](disassembly-004/preserved-commentary/); oldsourcehash9cb507770898f551ac529e642d13702b02f5da1036f32a65e34fc3f4fd5bbfa7. Newcurrent-sourcecommentary andfinaleditorial acceptance pending; no commentary/redirect run inthispass. [Handoff](disassembly-004/HANDOFF.md).

Toolkit: existingNES-DOC-001nonblockingdocumentationissue only; no newtoolkitissues found. No toolkitedit/rebuild,ROMchange,publication,archive,deletion/cleanup. Configuredserver stopped; assets/source/evidence local; disk8.9GiBfree,scratch14MiB.

| Space/bank | Code | Data/image | Classified | Unknown |
|---|---:|---:|---:|---:|
| header | 0 | 16 | 16 | 0 |
| PRG00 | 1638 | 6554 | 8192 | 0 |
| PRG01 | 6 | 8186 | 8192 | 0 |
| PRG02 | 1139 | 7053 | 8192 | 0 |
| PRG03 | 6 | 8186 | 8192 | 0 |
| PRG04 | 10 | 8182 | 8192 | 0 |
| PRG05 | 108 | 8084 | 8192 | 0 |
| PRG06 | 0 | 8192 | 8192 | 0 |
| PRG07 | 6 | 8186 | 8192 | 0 |
| PRG08 | 1179 | 7010 | 8189 | 3 |
| PRG09 | 1878 | 6314 | 8192 | 0 |
| PRG10 | 7497 | 694 | 8191 | 1 |
| PRG11 | 1693 | 6499 | 8192 | 0 |
| PRG12 | 2129 | 6062 | 8191 | 1 |
| PRG13 | 6 | 8186 | 8192 | 0 |
| PRG14 | 6711 | 1480 | 8191 | 1 |
| PRG15 | 1082 | 7110 | 8192 | 0 |
| CHR00 | 0 | 8192 | 8192 | 0 |
| CHR01 | 0 | 8192 | 8192 | 0 |
| CHR02 | 0 | 8192 | 8192 | 0 |
| CHR03 | 0 | 8192 | 8192 | 0 |
| CHR04 | 0 | 8192 | 8192 | 0 |
| CHR05 | 0 | 8192 | 8192 | 0 |
| CHR06 | 0 | 8192 | 8192 | 0 |
| CHR07 | 0 | 8192 | 8192 | 0 |
| CHR08 | 0 | 8192 | 8192 | 0 |
| CHR09 | 0 | 8192 | 8192 | 0 |
| CHR10 | 0 | 8192 | 8192 | 0 |
| CHR11 | 0 | 8192 | 8192 | 0 |
| CHR12 | 0 | 8192 | 8192 | 0 |
| CHR13 | 0 | 8192 | 8192 | 0 |
| CHR14 | 0 | 8192 | 8192 | 0 |
| CHR15 | 0 | 8192 | 8192 | 0 |

## Round4 commentary active
Pass004 full current-source review: preserve 1856 unchanged classified sections; review six additions (34 bytes), affected dispatcher/renderer/phase/queue dependencies and six remaining unknown notes. Previous commentary verified identical to disassembly004 frozen copy and moved intact to analysis/commentary-004/prior-active. Final editorial audit pending.

## Round4 commentary complete — pass004
All 1862 classified regions plus six unknown-gap notes reviewed (1868 reviewed, zero pending); 2032 meaningful units have refreshed current locations. Carried 1856 valid classified reviews; explained 31 new Code and3 new Data bytes. Corrected dispatcher table classification and D07D narrow typed-wrapper contract, preserving general selector/natural-entry uncertainty. New phase duration CPU alias is bank10/11-specific. Fresh configured-binary verification ok=true, byte_identical=true; reading-after refreshed. SourceSHA428802d744ac32fad35012a1e4f1e3039551ea10ff07ae79cc8059c040967911. ROM/state/maps unchanged. Classification remains incomplete:262154/262160, sixunknown. Full current-source readability complete; final editorial audit pending. See analysis/commentary-004/HANDOFF.md. Existing NES-DOC-001 nonblocking; no new toolkit issues. No runtime replay/toolkit edits/cleanup/publication.

## Round4 redirect active — r4-script-boundary
Inputpass004 sixunknown; examining corrected genericData inference101752 independently of precise endcap role. Five RTS remain separate. Frontier/status: analysis/redirect/r4-script-boundary/status.json. No map/source/ROM edits.

## Round4 redirect ready — r4-script-boundary
New R4-SCRIPT-BOUNDARY-DATA proposes1genericData byte101752 using four independent script-control stop analogues and exact neighboring script bounds. No selected stop/endcap claim. Five RTS separately reviewed and retainedunknown. Current sixunknown unchanged; no terminalassessment. Recheck passed identity/bytes/complement. See analysis/redirect/r4-script-boundary/HANDOFF.md and role-review.md. No new toolkitissues; NES-DOC-001 nonblocking. No runtime/map/source/ROM/toolkit edit.

## Round 5 — disassembly active
Independent bounded review of R4-SCRIPT-BOUNDARY-DATA; input pass004 has262154classified/6unknown. Full current commentary frozen in disassembly-005/preserved-commentary before any future refresh. No runtime replay needed; broad progression through THE END reused.

## Round 5 — disassembly complete; ROM incomplete

ROM classification: 100.00% (262155 / 262160 bytes classified; 5 bytes unknown).

Exact percentage 99.9980927677754%; rounded100.00% does not establish completion. Added1 Static Data byte101752; totals Code25088/Data105995/Image131072,1863 regions. All1862 prior region objects/IDs/provenance/views/evidence preserved. Observed11583 unchanged, no new runtime/reachability claims. [Counts/per-bank](disassembly-005/counts.json), [complement](disassembly-005/unknown-ranges.json).

R4-SCRIPT-BOUNDARY-DATA verified/consumed ready=false after independent primary byte/parent/corpus review. Generic noncode only; no stop/endcap/padding/container/natural-entry claim. [Review](disassembly-005/role-review.md), [checks](disassembly-005/independent-checks.json). Five RTS67407/68116/70230/86712/120565 remain ambiguous. Connected graphs already classified; broad gameplay through THE END reused without replay. Single pass ends at easy-gain exhaustion; no terminal redirect assessment.

Fresh [pass005 verification](pass-005-verification.json) ok=true,byte_identical=true; state/current/pass005 mapSHA2569ed86bef8db138cb02991ec2a718f4febe4c972a607a962a32558b900843601a. ROM unchanged. Prior full commentary preserved byte-for-byte [here](disassembly-005/preserved-commentary/), sourceSHA256428802d744ac32fad35012a1e4f1e3039551ea10ff07ae79cc8059c040967911. New current-source commentary and final editorial audit pending. [Handoff](disassembly-005/HANDOFF.md).

Toolkit: existing [NES-DOC-001](toolkit-issues.md) confirmed documentation bug remains nonblocking; no new toolkit issues. No toolkit edit/rebuild, ROM change, deletion/cleanup, archive or publication. Configured server stopped; all artifacts local; disk8.8GiBfree/scratch14MiB.

| Space/bank | Code | Data/image | Classified | Unknown |
|---|---:|---:|---:|---:|
| header | 0 | 16 | 16 | 0 |
| PRG00 | 1638 | 6554 | 8192 | 0 |
| PRG01 | 6 | 8186 | 8192 | 0 |
| PRG02 | 1139 | 7053 | 8192 | 0 |
| PRG03 | 6 | 8186 | 8192 | 0 |
| PRG04 | 10 | 8182 | 8192 | 0 |
| PRG05 | 108 | 8084 | 8192 | 0 |
| PRG06 | 0 | 8192 | 8192 | 0 |
| PRG07 | 6 | 8186 | 8192 | 0 |
| PRG08 | 1179 | 7010 | 8189 | 3 |
| PRG09 | 1878 | 6314 | 8192 | 0 |
| PRG10 | 7497 | 694 | 8191 | 1 |
| PRG11 | 1693 | 6499 | 8192 | 0 |
| PRG12 | 2129 | 6063 | 8192 | 0 |
| PRG13 | 6 | 8186 | 8192 | 0 |
| PRG14 | 6711 | 1480 | 8191 | 1 |
| PRG15 | 1082 | 7110 | 8192 | 0 |
| CHR00 | 0 | 8192 | 8192 | 0 |
| CHR01 | 0 | 8192 | 8192 | 0 |
| CHR02 | 0 | 8192 | 8192 | 0 |
| CHR03 | 0 | 8192 | 8192 | 0 |
| CHR04 | 0 | 8192 | 8192 | 0 |
| CHR05 | 0 | 8192 | 8192 | 0 |
| CHR06 | 0 | 8192 | 8192 | 0 |
| CHR07 | 0 | 8192 | 8192 | 0 |
| CHR08 | 0 | 8192 | 8192 | 0 |
| CHR09 | 0 | 8192 | 8192 | 0 |
| CHR10 | 0 | 8192 | 8192 | 0 |
| CHR11 | 0 | 8192 | 8192 | 0 |
| CHR12 | 0 | 8192 | 8192 | 0 |
| CHR13 | 0 | 8192 | 8192 | 0 |
| CHR14 | 0 | 8192 | 8192 | 0 |
| CHR15 | 0 | 8192 | 8192 | 0 |


## Round5 commentary complete — pass005

Full current-source readability complete: 1863 classified regions plus five honest one-byte unknown notes;1868 reviewed,zero pending,2032 meaningful units. Reused all1862 prior classified reviews and their prose/symbols, after entire inventory and source-boundary checks. Updated only new generic noncode101752 and current uncertainty inventory. Exact adjacent script boundaries and same-bank analogues support Data; command, literal or padding role and natural selection remain unresolved. No new runtime needed; broad progression through THE END retained.

Fresh configured-binary verify passed ok=true,byte_identical=true; reading-after refreshed and checked exact. MapSHA9ed86bef8db138cb02991ec2a718f4febe4c972a607a962a32558b900843601a;sourceSHA1eb0117282ff62bb8d9bd57521e9e3bf9e4e7fc32d0ac8230a754ba52cc3aba3. ROM unchanged. Classification incomplete262155/262160 (100.00% rounded;99.9980927677754% exact),unknown RTS67407,68116,70230,86712,120565. Readability/fidelity do not establish whole classification or final editorial acceptance. Audit pending,not run. No unread ranges or deferred commentary backlog. Prior active preserved intact at analysis/commentary-005/prior-active; instruction copies preserved. No toolkit edits/build,ROM edits,deletion,publication/archive. See analysis/commentary-005/HANDOFF.md.

Phase input/output pass005; leads consumed R4-SCRIPT-BOUNDARY-DATA structural result, no new leads produced. Next separate phase redirect. Scope/inventory/identity/verification:analysis/commentary-005/. Disk8.8GiB free; no cleanup performed.

## Round5 redirect — terminal incomplete pending coordinator review

Input/output unchanged pass005,mapSHA9ed86bef8db138cb02991ec2a718f4febe4c972a607a962a32558b900843601a. Full readability gate1868reviewed/0pending,2032units and fresh commentary005 byte-identical verification reused; final editorial audit pending. Exact262155/262160 classified,5unknown;100.00%rounded is not complete. Unknown67407,68116,70230,86712,120565 each60.

Unique attempt analysis/redirect/r5-return-contract-review. New bounded native JMP+RTS corpus has four accepted examples with exact conditional side exits; compare actual AFDC/D7FA/drawing ordinary stack contracts and IRQ RTI contract with accepted rich duplicate clauses. Each candidate's strongest Code and Data interpretations, independent constraints, scoped negatives and specific structural insufficiency are recorded in rejection-review.md with primary.json source/byte evidence. No naturalcaller universal veto. Positive noncode format proof also absent. Existing connected helpers, script and independent uncertainty avenues consumed; no ready lead or promising discriminating method remains. Proposed0,accepted0; R5-ISOLATED-RETURN-ROLES rejected/nonready preserves this outcome, previous leads unchanged. Broad progression through THE END reused.

Worker terminal assessment requires coordinator reasoning+primary-evidence inspection; do not autoaccept or run empty cycles. Handoff analysis/redirect/r5-return-contract-review/HANDOFF.md. ROM/map/state/source/binary unchanged; no runtime,toolkit edits/build,deletion,publication/archive. Storage8.7GiBfree,scratch13.12MiB retained.

## Final editorial acceptance — pass005

Human editorial review complete, valid full current-source credit retained: 1868 reviewed entries, 2032 reviewed units, zero pending. Inspected affected contracts and five honest unknown notes; assembly unchanged. Final configured audit passed ok=true,byte_identical=true,editorial_audit=true with fresh rebuild and reading refresh. Independent archive editorial_status complete=true,reasons=[]; no archive executed. Source/map/ROM hashes unchanged. First audit source-location-base mismatch repaired in metadata after backup and recorded as COMMENTARY-LOC-002; normalized paths all retain real source line targets. Final evidence: analysis/final-acceptance/HANDOFF.md; commentary/editorial-audit.md. Classification remains262155/262160, five unknown bytes67407,68116,70230,86712,120565; terminal structural uncertainty unchanged. Coordinator terminal handoff pending. No toolkit/ROM edits, archive/publication or cleanup/deletion.


## Coordinator terminal handoff — pass005

Workflow terminal incomplete after mandatory coordinator inspection of rejection-review.md and primary bytes/control/stack evidence. No ready lead or useful changed discriminator remains identified. Classification 262155/262160; five unknown offsets67407,68116,70230,86712,120565; 100.00% rounded, exact99.9980927677754%. This is not whole-ROM completion. Final editorial acceptance passed1868 reviewed entries/2032units, zero pending; fresh byte-identical audit, exact reading copy and configured map/store validation passed. Review source paths normalized after backup; no assembly/map/ROM edits. See analysis/terminal-handoff.md and analysis/current-audit.json. archive-options.json prepares explicit final/scratch Markdown selection only; no archive/cleanup/publication. Histories and local source/assets/evidence retained.
