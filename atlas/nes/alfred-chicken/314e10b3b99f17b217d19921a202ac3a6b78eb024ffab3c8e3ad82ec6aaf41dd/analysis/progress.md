# Disassembly progress

Workspace initialized. Baseline and classification have not run.

## Round 1 disassembly — active
Baseline completed and byte-identical verified: `analysis/baseline-verification.json`, `maps/baseline.json`, `generated/baseline/`. Initial raw baseline remains unclassified.
ROM identified from rendered copyright screen as Alfred Chicken (Twilight/Mindscape, 1993), mapper 2, 128 KiB PRG and CHR RAM. NMI $C7E3, Reset $C5C6, IRQ $C84A, fixed banks 14/15. Initial vector discovery: 53 instructions, stopped at indirect dispatch; evidence `analysis/round1/initial-discovery.json`.
Live persistent configured-binary RPC daemon: `analysis/round1/session.py daemon`; reconnect via `PYTHONPATH=analysis/round1` and `client.rpc(method, **params)`. Sequential request/response files in `analysis/scratch/rpc/` (do not send concurrent requests). No state map yet saved. Emulator is at frame 235 on copyright screen; durable emulator checkpoint `analysis/round1/title.snapshot.json` (same-build local-only). Startup trace aggregates: 1181 unique physical instructions / 1450 contextual roots; detail truncation expected, aggregates not dropped (`startup-trace-summary.json`). Current trace has capacity 0, Instruction filter, enabled; aggregate counts retained. First run requested 180 frames with 10s timeout and reached 55 frames; subsequent 180-frame run succeeded with 60s budget. Input unchanged; no gameplay edits yet.
Pending: proceed into gameplay, inspect and annotate observed/static candidates, inspect data patterns and consumers, try progression shortcuts, export/validate/save and verified pass-001 checkpoint. No cleanup/deletion performed. No toolkit issue established.

Session recovery: original file RPC process stopped responding; restored `title.snapshot.json` in new daemon retained unified exec session 4058, now `analysis/round1/session2.py` and `analysis/scratch/rpc2/`. Client requests now atomically renamed to prevent partial JSON. `menu.snapshot.json` and `game.snapshot.json` preserve title/menu, not actual gameplay. At frame 841 title remained because shared documented button/pressed input schema silently does nothing in NES adapter; correct full `buttons` array now used (NES-001). `actual-level1` experiment pending. No cleanup performed; old requests retained. Source-specific gameplay hypotheses saved in `analysis/gameplay.md`.

## Round 1 batch 1 — verified pass-001, phase remains active
ROM classification: 4.85% (6360 / 131088 bytes classified; 124728 bytes unknown).
Internal state `analysis/state.json`; export `maps/current.json`; frozen map `maps/pass-001.json`; generated source `generated/pass-001/`; exact rebuild report `analysis/pass-001-verification.json`. Code 5343 bytes from 2400 nonoverlapping decoded instruction starts (observed opcode roots and static control-flow continuations); other data/text 1017 bytes includes parsed header, vectors, literal cartridge title and bounded maximal repetitive non-code runs. Root/listing evidence frozen in `analysis/round1/batch1-contexts.json`, `batch1-candidates.json`, `batch1-listing.txt` and initial discovery. Annotations describe limited structural claims; no fill purpose inferred. Full commentary phase not started.
Natural corrected Start reached Stage1 introduction at frame903. At1083 fade screen still running; durable `cheese-checked.snapshot.json` and matching frame/RAM saved. Corrected input route continues with right+A for180 frames. Original file daemons were externally terminated (143) before pending commands returned; no unsupported toolkit attribution. Switched to direct bounded subprocess batches (`game-batch.py`), each restoring map/snapshot, tracing, saving a new snapshot/frame/roots and candidates. Superseded transport files retained. Do not reconnect stale file daemon.
Toolkit findings: one nonblocking confirmed documentation/API validation issue NES-001 (NES input schema), detailed in `analysis/toolkit-issues.md`; no toolkit modifications. Broad later progression remains pending.

## Round 1 batch 2 — verified pass-002, broad progression active
ROM classification: 15.08% (19770 / 131088 bytes classified; 111318 bytes unknown).
Added 13410 code bytes (6465 instructions), inspected in `motion1-listing.txt`; no conflicting non-code boundaries or suspicious BRK/unofficial instruction patterns. `analysis/pass-002-verification.json` confirms byte identity. Natural gameplay reached cheese stage1 at frame1263 after right+A movement/jump; `motion1.snapshot.json`, frame, RAM, trace roots and discovery saved. Scenario and milestone checklist now maintained in `analysis/scenarios.json`.
Static consumer at fixed CPU $C679 loads stage descriptor pointer from bank0 $8263 indexed by RAM $93. Six pointer entries resolve stage descriptors; each first six bytes consumed into spawn/map variables. Stage2 practical shortcut is now running from checked frame1083 pre-load snapshot, patch RAM $93=1 and let normal initialization continue for240 frames with right+A. This is artificial state, not natural reachability. Other representative stages/boss/ending remain pending.

## Round 1 broad-stage batch — active continuation
Source-backed RAM $93 edits before the ordinary initializer reached distinct stage2 alphabet-block area, stage3 screw-platform industrial area, stage4 chain/spike area, and stage5 green-machinery/pipe area. Each held right+A, then released; actual frames/banks and opcode roots saved in per-stage `analysis/round1/stageN-*` evidence. Requested240frames/45s, bounded timeouts preserved partial progress around215–223frames for stages2–5. These observations are artificial-state reachability, not claims of natural completion. Stage6 descriptor test also saved and awaits interpretation; no out-of-domain selectors tested.
Static direct-call entry and mapper2 fixed-window write-continuation review added246 code bytes; proofs `static-call-root-proofs.json`, listing `staticcalls-listing.txt`. Added312 bytes of initial palette/table data and six descriptor pointer/header slices; full record tails remain unclaimed. Stage3–5 new candidate batches reviewed without non-code conflicts or suspicious instructions; accepted connected supported code. Additional final-balloon transition practical shortcut derived from physical0x14D22..0x14D33 (countdown $A8, playerstate$63=$16, balloon slot/latch) is being tested; it reached a black transition frame, and continuation is in progress. This is a normal state-consumer contract tested via exact RAM writes, not PC injection; see `balloon-transition-patches.json` for all writes. No ending/boss visit yet claimed. Current map will receive final fresh checkpoint after continuation and pending batch reviews.

## Round 1 pass-003 — verified; transition exploration continues
ROM classification: 31.97% (41904 / 131088 bytes classified; 89184 bytes unknown).
Fresh byte-identical checkpoint `maps/pass-003.json`, source `generated/pass-003/`, report `analysis/pass-003-verification.json`. Added stage3–6/transition code plus15423 generic non-code bytes. Patterned slice evidence `analysis/round1/patterned-data-review.txt`: C010:E010, D10:2410, 2910:3010, excluding already classified runs. Repeated sparse matrices, paired ascending records and bit patterns were inspected with current code/references; no full asset schema, original object extent or generator is claimed. Compressed/ambiguous adjacent portions retained unknown.
Selector$93=5 reached a sixth descriptor's dense spike room, not a demonstrated ending. Fifth descriptor (selector4) displayed green machinery/pipe area. Final-balloon state shortcut in stage1 reached star-field aerial bonus (frame1489), with actual fetched code and new rendering; continuation scripts preserve exact patches, state and roots. Equivalent source-backed final-balloon shortcut from stage5 reached a black transition with stage4 retained, map4, playerstate9 (different from stage1 bonus state23);60-frame continuation underway to verify boss/final state. No ending/credits claim.

## Round 1 disassembly phase complete — pass-004

ROM classification: 33.69% (44157 / 131088 bytes classified; 86931 bytes unknown).

Checkpoint `pass-004`: `analysis/state.json`, `maps/current.json`, frozen `maps/pass-004.json`, generated `generated/pass-004/game.asm`, byte-identical `analysis/pass-004-verification.json` (ok=true, byte_identical=true). Final map SHA2562889b50152817b3708815858dae2d84ce5029fc34f6184fce5dce659c64327bc. Input ROM SHA256 unchanged314e10b3b99f17b217d19921a202ac3a6b78eb024ffab3c8e3ad82ec6aaf41dd. Baseline and passes001–003 preserved. Full source commentary/readability phase not performed.

Natural reset/title/Stage1 cheese movement and jump visited. Source-backed pre-initialization RAM$93 selector edits visited stages2 alphabet blocks,3 screw platforms,4 chains/spikes,5 green machinery/collision, and sixth descriptor spike room. These are artificial-state observations. Final-balloon RAM contract (consumer physical14D22..14D33) exposed aerial bonus from stage1 and shooting flight from stage5; movement and B projectiles exercised. Later480-frame time advance plus90 traced frames showed enemy projectiles, without identifiable boss or ending. All exact presses/releases, budgets, patches, frames, effective banks, captures and durable snapshots are in `analysis/scenarios.json`, `analysis/gameplay.md`, and per-scenario `analysis/round1/` files. Do not call these natural later-stage reachability or full gameplay compatibility.

Structural evidence: vector roots and actual-context recursive decoding; direct JSR entries and UxROM fixed-window write continuations (`static-call-root-proofs.json`, `staticcall2-root-proofs.json`); six descriptor headers and pointer consumer; four24-word PPU variant tables under X=$60..$77 (`ppu-dispatch-proof.json`). All16 distinct target contracts coherently select PPU address/write handlers; newly supported360codebytes and192tablebytes accepted. Patterned generic non-code slices reviewed in `patterned-data-review.txt`; original asset schemas/whole-object extents remain unclaimed. All annotations preserve physical offsets, views, bank context and Static versus Observed provenance. Artificial reachability is separately identified by scenario evidence. Initial links now cite immutable batch1 copies. Trace details intentionally bounded/disabled; aggregate_dropped=0 for all saved scenario traces (`trace-integrity.json`).

Easy-gain gate: broad representative systems and supported stage/balloon shortcuts tried; startup repeated only to repair input schema, not as a coverage substitute. Runtime new gains declined to399 then162bytes in repeated shooting flight, without a new subsystem. Two bounded direct-callee/fixed-window passes declined to111bytes. The concrete PPU dispatch slice was investigated and accepted as a connected batch. Further useful work requires selector/return-contract/resource or boss-wave research, retained as redirect questions, rather than more long repeated flight. This is ordinary single-pass exhaustion, not terminal research exhaustion and not a full-ROM completion claim. No terminal rejection review is claimed/needed at this easy-gain handoff.

Next: separate full current-source commentary, then redirect using `analysis/leads.json`. Prioritize mixed resource boundaries (notably bank0 compressed intervals and bank1/2 graphics), remaining indirect actor dispatch contracts, and boss/wave/ending conditions from the final-later snapshot. No live server required: a fresh configured-binary serve needs map.load and emulator.start/snapshot.import; named snapshots are not persistent. `client.py`, `game-batch.py`, `review-batch.py` provide bounded direct-subprocess recipes. Old file-RPC transport attempts are historical and should not be reused.

Toolkit report `analysis/toolkit-issues.md`: one nonblocking confirmed documentation/API validation bug NES-001 (shared documented button/pressed schema silently clears NES input; workaround full buttons arrays). No other toolkit issues found in this pass; no toolkit edits. No archive, publishing, deletion or cleanup performed. At handoff free space8.9GiB, scratch17MiB, round1 evidence373MiB.

| Physical PRG8K bank | Code | Data/text | Classified | Unknown |
|---|---:|---:|---:|---:|
| header | 0 | 16 | 16 | 0 |
| 0 | 301 | 5176 | 5477 | 2715 |
| 1 | 215 | 2993 | 3208 | 4984 |
| 2 | 911 | 76 | 987 | 7205 |
| 3 | 243 | 0 | 243 | 7949 |
| 4 | 424 | 0 | 424 | 7768 |
| 5 | 2289 | 192 | 2481 | 5711 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 137 | 152 | 289 | 7903 |
| 8 | 0 | 128 | 128 | 8064 |
| 9 | 802 | 0 | 802 | 7390 |
| 10 | 3971 | 0 | 3971 | 4221 |
| 11 | 987 | 13 | 1000 | 7192 |
| 12 | 3318 | 0 | 3318 | 4874 |
| 13 | 1975 | 0 | 1975 | 6217 |
| 14 | 6070 | 0 | 6070 | 2122 |
| 15 | 5570 | 6 | 5576 | 2616 |


## Coordinator round 1 phase boundary — commentary active
Disassembly handoff `analysis/round1/handoff.md` accepted as ordinary single-pass easy-gain exhaustion, not terminal research exhaustion. Separate worker completed and stopped before commentary worker launch. Coordinator independently counted nonoverlapping internal classified intervals:44157 of131088 bytes,86931 unknown (33.69%). Current portable map SHA256 equals pass-004 verification map SHA256; input SHA256 unchanged. Exact required percentage printed to user before commentary.
Input checkpoint pass-004; next separate worker `/root/round1_commentary` is responsible for ALL current-source readability, fresh byte identity and reading-copy refresh. No prior commentary exists; prepare via documented workflow. Do not advance to redirect until its whole-current-source readability gate passes. Terminal acceptance and final archive-options inventory remain pending. Prior coordinator turn progressed through broad-stage evidence; this turn completed and independently checked the phase boundary.

## Round 1 commentary setup — active
Prepared commentary with documented `scripts/commentary.py prepare`; initial byte identity passed, 1230 full-ROM scaffold entries frozen. Working source 18148 lines, 985 classified code slices. Full current-source naming/commentary is pending; proceeding in physical order, preserving branch widths/bank contexts and all earlier evidence. Exact frontier in commentary/progress.md. No redirect or toolkit changes; final editorial audit reserved for final acceptance.

## Round 1 commentary phase complete — pass-004 source preserved
All current classified source received its first full readability pass:27213codebytes/12888instructions,16931data,13text;209unknown slices retain honest boundary notes. Comments cover245routine/control-flow contracts, consistent RAM/hardware/object/audio names and context-resolved branches/calls. No remaining classified-source edit gaps; meanings/incomplete structures remain explicitly uncertain. Full scaffold retained with1230reviewed entries/318contiguous units; final editorial audit pending. Examples and uncertainty links in `analysis/round1/commentary-handoff.md` and `commentary/progress.md`.
Fresh documented verify ok=true,byte_identical=true; reading-after.asm refreshed. Frozen final verification and all identity checks under `analysis/round1/commentary/`. ROM/current maps/internal state/frozen baselines unchanged. Classification remains44157/131088(33.69%),86931unknown. No new toolkit issues found in commentary; existing nonblocking NES-001 retained. No disassembly/redirect investigation, toolkit edits, final audit, archive, cleanup/deletion or publication. Next coordinator readability/identity gate, then separate redirect using original leads plus commentary questions.

## Coordinator round 1 phase boundary — redirect active
Commentary handoff `analysis/round1/commentary-handoff.md` accepted for full current-source readability, separately from final editorial acceptance. Coordinator inspected palette/controller symbols, PPU queue format/dispatch, NMI register/bank preservation and literal/back-reference decoder source explanations, plus uncertainties and whole-source assessment. All recorded source/reading/frozen map identities match current files; final verification is byte-identical. No classified-source remaining ranges reported. Commentary worker completed before separate `/root/round1_redirect` launch. Input checkpoint remains pass-004 (44157 classified,86931 unknown;33.69%). Final editorial audit remains pending; no terminal assessment accepted. Current continuation made progress: source edits/verification completed and sequential phase handoff checked. Next worker follows saved redirect questions and exact unknown complement under evidence-policy.md.

## Round1 redirect active — r1-20261105
Input pass-004 unchanged44157classified/86931unknown. Unique durable research directory `analysis/redirect/r1-20261105/`; method/frontier in research.md. Static flight substate consumer exposes paired actor3B/3C initializer and phase gates; practical RAM735=2 shortcut tested from final-later snapshot with B held45+120frames, capacity0 instruction aggregates, snapshots/frame/RAM saved. Artificial state only; visual/trace interpretation pending. Also inspecting direct decoder long-copy continuation, indirect actor contracts and mixed-resource boundaries independently. No map/source/toolkit edits, no terminal or exhaustion claim. Remaining productive connected work stays in this redirect.

Redirect r1-20261105 remains active. Native decoder crosscheck matches exact endpoints/output hash; exact map scan dimensions, six tileset streams, finite32 actor-graphics records and24map/actor configchains yielded coherent data candidates (no accepted bytes yet). Persistence matching simulation resolves1183cells/18maps with0failures. Flight shortcut reaches rendered mechanical encounter, normal combat initializer and health-triggered defeat states; later transition/ending pending. Source-based supporting/negative scopes and currentactor/body/rowtable frontier in `analysis/redirect/r1-20261105/research.md`. Map/state/source/ROM unchanged.

Redirect r1-20261105: artificial mechanical encounter completed through source-backed reward contact; ordinary terminal branch loadedstage5/map6. Exactmap6door/persistencegate then enteredmap2/player1C scriptedendingroom, rendered and characterwalking withallbuttonsreleased; finalscreen pendingboundedcontinuation. Independent wholemetasprite extentproof closes31families/243frameentries viaencodedrelativeorigins,counted6bytepieces andcomplete4458bytecoverage; apparent32ndslot rejectedlocally. Bank2rawgraphics32planes/64sourceorigins andframe/phase tables closeall7768remainingbytes inPRG8Kbank4. Currentproposeddata47052bytes/58unknownslices plus3389code(total50441),noneaccepted. Finalcapture/codepacketrefresh andcoherentready-lead handoff/dispositions remain; maps/source/ROM unchanged.

## Round1 redirect complete — coherent ready batch, pass004 unchanged
`analysis/redirect/r1-20261105/handoff.md` hands R1-L03/L04/L05 ready to independent disassembly.52340proposednewbytes:48136data in44unknownslices,4204codeunion,0accepted/0code-data conflicts. Classification remains44157/131088(33.69%),86931unknown. Exactunchanged ROM/map/state/commentary hashes in input-identity-at-handoff.json; new proposals require independent annotation/rebuild, then fullcurrent-source commentary. FullROM/editorial acceptance incomplete; this is not terminalexhaustion.
Rendered quest-is-over ending after artificial source-backed two-phaseencounter/reward and stage5door progression. `ending-finish.png`,snapshot,actualcontexts preserve screen; route/failedbranches in scenarios/research. Nativecompressedstream endpoint/hash crosscheck, encodedmapscan bounds,31metaspritefamily wholeextent,32patternplanes, finitegraphicresource schemas, persistence matches and localized failedrowtable domains support coherentbatch; exact primary artifacts/recipes/deferredreasons inhandoff. No furtherrepeat of unchangedflight/nativechecksneeded.
No new toolkit issues found; existingNES-001 remainsnonblocking. No map/source/toolkit edits, archive/publication, cleanup/deletion or goalstatus change. Free8.7GiB/scratch17MiB atfinalcheck. Next separate round2disassembly review; preserve completed redirectattempt immutable.

## Same round1 redirect active — audio supplement
The earlier r1-20261105 'complete' entry/handoff is a preserved preliminary coherentpacket, not the final phase boundary. Coordinator has not begun disassembly. Final residualsource review found actionableaudio descriptors/sequence grammars; continuing same redirect in unique `analysis/redirect/r1-audio-20261105/`, preserving firstattempt immutable. Nine supported songdescriptors alreadyparse42sequence lists/90streams,0unsupportedcommands; connected envelope/vibrato/effect/note domains under investigation. Finalcombinedhandoff will supersede phaseboundary only; noacceptedbytes/map/sourcechanges.

## Final round1 redirect phase complete — combined ready handoff
Audio supplement followed selectedsongs through42lists/90streams,75effectdescriptors,16FX/21musicenvelopes,9vibratos and explicitEndMusFXcontainerendpoint;4951additionaldata. Connectedstage-tailconsumer adds294data from6ballooncounts/48sixbyteexitrecords. Finalcombined **57585proposedbytes (53381data,4204code),0accepted**;0proposalconflicts/outsideunknown, allROM/map/state/commentaryidentities unchanged. Classificationremains44157(33.69%)/86931unknown. Finalhandoff `analysis/round1/redirect-handoff.md` -> `analysis/redirect/r1-audio-20261105/handoff.md`; preliminaryattempt preservedimmutable.
Noactive/interruptedexperiment. Quest-is-overterminalscreen visited artificially; finalcapturecontinuation completed. Concrete remainingactor/projectile/rowtable/audiosemantic limits recorded; no unsupportedblanketexhaustion, no terminalincompleteclaim. Nextindependentround2disassembly reviewsreadyL03–L06,thencheckpoint/rebuild/fullcurrent-source commentary; finaleditorialauditstillpending. No newtoolkitissues;NES-001retained. No toolkit/map/sourceedits, publishing, archive or cleanup/deletion; free8.7GiB,scratch17MiB.

## Coordinator round 1 redirect → round 2 disassembly boundary
Final combined handoff `analysis/round1/redirect-handoff.md` and `analysis/redirect/r1-audio-20261105/handoff.md` inspected and accepted as a coherent ready batch, not terminal exhaustion. Initial packet remains immutable; audio supplement closed the newly actionable independent avenue within the same redirect. Ready IDs R1-L03/L04/L05/L06 propose57585newbytes (53381data+4204code); zeroaccepted. Classified checkpoint remains pass-004:44157/131088 (33.69%),86931unknown. Both attempts retain primary artifact indexes, scoped counterexamples, recipes, exact dispositions and honest unresolved branches. Finalquest-over screen captured after completed bounded artificial progression; no live/interrupted experiment remains. No terminal-incomplete assessment accepted.
Redirect worker finished before separate `/root/round2_disassembly` launch. New worker independently validates roles/bounds/bank/stack contracts and uses reusable evidence without unchanged runtime replay. Preserve existing commentary for later round2 carryover and full-current-source readability. This coordinator continuation made progress by accepting the completed extended redirect and launching the next required separate phase. Terminal audit, final editorial acceptance and final archive document inventory remain pending.

## Round 2 coherent batch accepted — pass-005; residual review active
ROM classification: 77.61% (101742 / 131088 bytes classified; 29346 bytes unknown).
Independent code re-decode, CPU alias/effective PRG context audit and per-object structural boundary review accepted R1-L03/L04/L05/L06 ready union: +57585 bytes (53381 data,4204 code). Exact accepted regions, immutable input map, opcode audit and accounting in analysis/round2/. map.validate succeeds; pass-005 rebuild byte-identical. Artificial fetched provenance separated from static discovery. Commentary untouched; broad progression/quest-over evidence reused without replay. Continuing connected residual and repeated-pattern boundary review before easy-gain handoff. No new toolkit issues; existing NES-001 nonblocking.

## Round 2 disassembly complete — pass-007
ROM classification: 77.80% (101983 / 131088 bytes classified; 29105 bytes unknown).
Total pass gain57826 =4226code +53600data. Current kinds31439code,70531data,13text; header16classified. ReadyR1-L03/L04/L05/L06 verified and consumed. Beyond original union, explicit direct-edge audit found one additional22byte static persistence helper; six repeated non-code intervals add219genericData. Exact primary review/acceptance/accounting/handoff under analysis/round2/. No gameplay replay; existing representative later-stage/boss/reward/ending artificial routes reused. No naturalclear/credits claim. Finalmapvalidate success; pass-007 byte-identical rebuild. Checkpoints005/006 preserved. Commentary untouched; next commentary must preserve full existing version before fresh preparation.

|PRG8K bank|Code|Data/text|Classified|Unknown|
|---|---:|---:|---:|---:|
|0|301|7843|8144|48|
|1|215|5475|5690|2502|
|2|911|4867|5778|2414|
|3|243|6580|6823|1369|
|4|424|7768|8192|0|
|5|2343|192|2535|5657|
|6|0|8192|8192|0|
|7|137|7495|7632|560|
|8|0|8190|8190|2|
|9|1973|3697|5670|2522|
|10|4474|403|4877|3315|
|11|3039|13|3052|5140|
|12|3318|4857|8175|17|
|13|1975|4950|6925|1267|
|14|6097|0|6097|2095|
|15|5989|6|5995|2197|

Easy-gain gate and next concrete actor/graphics/collision contracts in analysis/round2/handoff.md and review.md. ROM incomplete; full-current-source commentary and final editorial acceptance separate. No new toolkit issues found; existing confirmed nonblocking NES-001 retained in analysis/toolkit-issues.md. No toolkit edits, archive, publishing or cleanup/deletion.

## Coordinator round 2 disassembly → commentary boundary
Accepted `analysis/round2/handoff.md` as ordinary easy-gain pass completion, not terminal research exhaustion. Coordinator independently counted nonoverlapping internal intervals:101983/131088classified(77.80%),29105unknown; current portable map hash matches byte-identical pass-007 report, input ROM SHA unchanged. Required exact percentage printed to user before commentary. Ready L03–L06 consumed and supported extra connected/helper/pattern bytes accepted; total roundgain57826. Full perbank counts and next concrete actor/graphics/collision contracts retained.
Separate disassembly worker finished before `/root/round2_commentary` launch. New commentary worker must first preserve ENTIRE prior pass004 commentary version and identities under unique analysis/round-2/ path, then carry sound review/names/comments by physical ranges/dependencies into pass007 current source without overwriting sole old copy. Full current-source readability gate, fresh byte identity and reading-copy refresh remain required before redirect. No terminal assessment or final editorial acceptance yet. This continuation progressed through independent classification and verified phase transition. Archive-options final reinventory pending.

## Round 2 commentary active — preservation and carryover
Preserved ENTIRE previous commentary under `analysis/round-2/commentary-pass007-20261105/previous-commentary/` before fresh documented preparation. Identity manifest includes every prior file plus input/current map/state. Fresh pass007 baseline verifies byte-identical. All1021 prior classified map regions retain exact physical bounds/kinds; source prose/names/symbols and valid review carried by those ranges, retaining old snapshot intact. New source remains pending; exact pending scaffold ranges in `analysis/round-2/commentary-pass007-20261105/pending-ranges.json`. Carryover rebuild succeeds byte-identical; source locations still require refresh after all edits. No final audit or readability-completion claim.

Current classification101983/131088;31439code,70531data,13text,16header;29105unknown. Prior44157 classified preserved, additions57826 pending readability (4226code,53600data). Entire new classified scope plus changed unknown boundaries remains to review; prior uncertainty notes carried. Map/state/ROM unchanged. Same commentary phase continues.

## Round 2 full current-source commentary complete

Full prior version preserved before preparation at `analysis/round-2/commentary-pass007-20261105/previous-commentary/`; identity-manifest.json covers every file. All1021 prior classified ranges retain exact offset/length/kind/views. Carryover preserved names, prose, symbols and review credit; dependencies remain valid because prior targets and mapped views are unchanged. New baseline was prepared with the configured release workflow, never generated over maintained source. Frozen before/ remains unchanged.

Readability covers all31439code bytes,70531data bytes,13text bytes and16header bytes; unknown29105bytes retain honest boundary notes. New4226code bytes were read in physical order in compact-new-code.txt and explained through135 additional routine/control contracts, with139 contiguous code review units. New53600data bytes have accepted format/boundary explanations;399 specific data structures have descriptive physical backing labels and prose. Generic non-code slices retain bounded pattern/consumer uncertainty. All3648 scaffold records remain intact, reviewed for current-source readability;856 meaningful units include carried credit. Final editorial audit remains pending. No remaining classified-source readability ranges.

Names and symbols preserve exact instruction widths, constants and effective PRG contexts. Added aliases identify fixed actor slots and local encounter counters. Code explanations cover synthetic platform returns, allocation failure unwinding, line-motion error state, channel-specific bytecode grammar, bounded resource tables and nonreturning stack-reset transitions. Physical backing labels are explicitly distinguished from CPU addresses. Existing reviewed code was retained and numeric direct transfers into new targets now use context-resolved symbols.

Examples:
- `SetVariantPpuDestination`: source/game.asm:3335.
- `DispatchOscillatingPlatform`: source/game.asm:11409.
- `BuildActorLineMotion`: source/game.asm:15095.
- `QueueEncounterDialogueLine`: source/game.asm:6701.
- `RestoreSavedPlayerEntry`: source/game.asm:33331.
- `CopyLongDecompressBackReference`: source/game.asm:35747.

Fresh final verification succeeds: ok=true,byte_identical=true; ROM SHA256314e10b3b99f17b217d19921a202ac3a6b78eb024ffab3c8e3ad82ec6aaf41dd. Exact source hashes in verification.json and immutable final-verification.json. reading-after.asm refreshed after the final edit. Final identity check confirms input ROM/current map/internal state and entire preserved prior commentary unchanged. No map/classification changes or runtime replay.

Remaining meanings/questions are in uncertainties.md and this phase's handoff.md: natural actor/flight selector domains, indirect stack contracts, graphics helper boundaries, behavior/persistence prefixes and exact audio semantics remain research questions. Classification remains101983/131088(77.80%); this is commentary completion only, not full-ROM or final editorial acceptance. Initial pending-ranges.json is a preserved preparation frontier, superseded by this completed status and final-scope.json.

No new toolkit issues found. Existing confirmed nonblocking NES-001 remains in analysis/toolkit-issues.md. No toolkit edits, redirect/disassembly work, archive, cleanup/deletion, publishing or subagents performed.

## Coordinator round 2 commentary gate — corrective continuation required
Preliminary handoff `analysis/round-2/commentary-pass007-20261105/handoff.md` reported full readability and byte identity. Coordinator independently verified all26 preserved prior-commentary files against their original sizes/hashes and current scaffold3648reviewed entries. Primary source sampling nevertheless found a concrete alias-classification defect: CopyLongDecompressBackReference uses `lda z:BUTTON_START` for RAM$10, conflating an input bit constant with storage; generated names at SetVariantPpuDestination describe immediate$15/$B5 as RAM fields. Duplicate structural annotation prose repeats on instruction-sized slices despite an overarching functional explanation. Byte identity does not prove correct names.
Returned SAME `/root/round2_commentary` worker to audit this numeric-collision mechanism across all current/carryover aliases and branch names, remove redundant repeated slice notes, update affected source/review records and provide fresh verification/reading copy in a preserved corrective supplement. Preliminary handoff/evidence remain historical, not accepted phase completion. No redirect worker launched. Commentary remains current phase. This turn made progress by verifying preservation and identifying/returning concrete readability defects under the explicit handoff gate; no toolkit change or terminal assessment.
Correction to preceding coordinator example: BUTTON_START is$08 in symbols.inc, so the erroneous memory operand named RAM$08/work_byte, not$10. The immediate-versus-storage alias mechanism and required corrective audit remain unchanged. Use decoded operands and symbol definitions as authoritative.

## Round 2 commentary corrective supplement — complete

Coordinator gate reopened this same phase after the preliminary handoff: shared numeric lookup had assigned value-constant names to RAM operands and used immediate values as RAM roles in generated labels. The earlier completed packet remains historical and untouched; this supplement supersedes its readability gate. Correction snapshot and complete audit evidence: `analysis/round-2/commentary-pass007-correction-20261105/`.

Audited all14854 instructions by original addressing class, including carried source:7599memory operands,2636immediates,4619control/implied instructions. Corrected7 RAM$08 references from BUTTON_START to work_byte and10 RAM$04 references from PPU_INCREMENT_VERTICAL to scratch_ptr. These are scratch storage addresses, not controller/PPU bit values. Context notes explain their decoder, clipping-width and sprite-loop roles. No actual RAM$10 operand was involved in the coordinator's example; original decoded bytes prove RAM$08. Exact values/storage classes and bank-specific table aliases pass independent final validation with zero mismatches.

Corrected402 new generated immediate-site labels so immediate bytes no longer imply RAM identities; corrected17 related storage-derived labels. Prior immediate destination labels were checked against their construction: they describe actual following stores rather than naming the numeric immediate as RAM. Shared old symbolic immediate masks remain in immediate consumers. Removed122 repeated per-instruction structural provenance comments for platform/decompressor/persistence slices, retaining functional explanations once and all original structural evidence unchanged in maps. Inventory source locations refreshed; all3648 scaffold entries and856 units remain intact and reviewed for current-source readability, with final editorial audit pending.

Fresh final verify succeeds byte_identical=true,ok=true after all source edits; reading-after refreshed. Source SHA25617c7b649e9615f1f1e1ab9fe846c7bab80f59a6250f62e68455971f7345610d3; symbols SHA2568421360d7de39b052ca53736c3a8c1800abeb4a0b9f66a006a114dae3940a05c. Exact final hashes, review and verification copied into the corrective packet. Input ROM/current map/internal state identities unchanged. No classified readability gaps remain after this corrective audit. Classification remains101983/131088(77.80%),29105unknown; no structural inference changed.

No new toolkit issues found; defect was in the local commentary edit recipe, not the configured toolkit. Existing nonblocking NES-001 retained. No redirect, final audit, toolkit modification, cleanup/deletion, archive or publication.

## Coordinator round 2 commentary → redirect boundary
Corrective handoff `analysis/round-2/commentary-pass007-correction-20261105/handoff.md` accepted after primary source recheck. RAM$08 now work_byte; RAM$04 scratch_ptr, immediate$15/$B5 sites no longer claim RAM fields, redundant per-instruction notes removed. Complete14854-instruction addressing-class validation reports zero mismatches; current source hashes match fresh byte-identical verification and reading refreshed. Preservation manifest independently checked all26 prior files identical. Initial preliminary handoff/evidence remain historical; corrective supplement supersedes readability gate. Full current-source readability complete, final editorial acceptance still pending.
Same commentary worker finished before new separate `/root/round2_redirect` launch. Input remains pass-007:101983/131088classified(77.80%),29105unknown. New worker investigates remaining concrete static/indirect/resource/return contracts and evidence-policy dispositions; prior broad progression/quest-over route is reusable. No terminal assessment accepted. This continuation made progress by correcting concrete naming defects, verifying full corrective scope and completing required sequential phase boundary. Archive-options final inventory remains pending.

## Round 2 redirect active — r2-20261105 static bank/stack contracts
Input pass-007 remains101983classified/29105unknown, source/map/state unchanged. Research in analysis/redirect/r2-20261105/research.md; exactunknownledger/inputhashes retained. PPU pending-command34slot parallel arrays and boundedpatch loops unlock connectedgraphics code/data. Corrected blanket indexedwrite barrier: absoluteRAMbase+$FF below$8000 cannotwrite mapper;51accepted-code fallthroughroots. Explicit balancedhelperanalysis checks allpaths, stackdepth, mapperpreservation anddata barriers;56 of71acceptedJSRunknownfallthroughs supported,15withheld (balanced-call-roots/failures). Separate manualbank-restoringwrapperpremises are FC26/FC3F,CE3B/CE61/CE78/CE8F,CEA6/CEBA andsourceproofs, notuniversalJSRrule. Actorstatewriters followedconcretely:46selectorsnowincludingnewinitializers/transitions; noarbitrarysweep. Currentreturn-flowgraph6975proposedunknowncodebytesbeforelatest3transitionroots; graphicsdata358bytesplusactortableclaimspending. Proposedbytesnotacceptedclassification. Frontier: finishfiniteactor/stack contracts, inspectfixedcollisionreturnfailures, thenlocalizedprefix/noncode review; no terminalexhaustion orhandoffyet. Configuredtoolkitunchanged; source confirmsconservativeindexedwrite handling; persistenttoolkitfindingassessmentpending. No unchangedprogressionreplay; prior artificialquest-overevidence reused.

Round2redirect laterbatch: existingJSRreturnreview64/71supported,7withheldafterexplicitbank-restoring/persistencecontracts; no-hitpathsfor8886/8A06supportconditionalsuccessorsalthoughhitpathsunwind. Exactimmediatebankselections146entries and20literalbanked-callbackcontracts includecontextduplicates;54concretestatewritersfixedpoint. Actor81threephasesclosedwithsavedcoordinatecleanup; player31fixedstatepointersfollowedunderunknownlowbanks. Currentgraph9667proposedunknowncodebytes,15572instructions<20000budget. Independentwaveform256newdata andbinaryactorflagprefix128additionalbytes pluslocalfinitepointer/flagarraysand35genericpatternsubintervals reviewed; exactfinalunionpending. Research.md andprimaryJSON updated. No classification/source/ROM/state/binary changes, no runtime replay. Remainingmixedprefixcode/indirectbankreturnsretainedlocally; readybatchpackaging/auditnext, no terminalexhaustionclaim.

Round2redirect final research extension: no-writer static actor sibling roles (26distinct entries), accepted15-state two-slot projectile family, conditional state3 dispatcherreturn, bank2 camera/numeric/PPU utilities and bank0 title/resource contracts followed in samephase. Current ordinary graph17634instructions/12802unknowncodebytes before independentgraphicsunion; callbackreturnroots reachedfixedpoint. Six24bit stride4 decimaldivisors and49byte camera-motion script have independentconsumer/endbounds. Bank0 encoded compressedresource38F6..3E99 terminates after1443input/1616output bytes under boundedlocaldecoder;85glyph bytes andtwo11bytePPUcommandrecords separatelybounded. New scripts/data included in nextpackageaudit; exactinputclassificationstill101983/29105. research.md nowcurrent. No runtime/map/source/binary edits; no terminalexhaustionclaim. Finalunion,ledgerdispositions,leadshistory andhandoff pending.

## Round 2 redirect complete — ready for independent disassembly
Final packet `analysis/redirect/r2-20261105/handoff.md`: **17727 proposed bytes (13509 code +4218 data),0 accepted**; current remains101983classified/29105unknown. Exact union206code/77dataslices,6347 independently decoded instruction views, zero prior-region/code/data conflicts. Full258interval bank-aware unknown ledger retains11378 hypothetical residual bytes if accepted. Static actor/projectile/graphics/return/bank utility paths and independent waveform/title/compressed resource/script/pattern objects investigated as one coherent batch. Conditional no-hit/decoder/audio paths explicitly localized; no universal live-caller/writer criterion. Methods reached stable roots/results; no budget interruption or terminalexhaustion claim. R1-L04/R1-L05 reopened with appended methods/history, original full entries preserved. `final-identities.json` confirms ROM/map/state/source/symbols/configuredbinary unchanged. No emulator replay/live server/toolkit edit/archive/publication/cleanup; free8.6GiB,scratch17MiB. NES-002 documented as nonblocking missingfeature; NES-001 retained. Nextphase independent disassembly, then fullcurrent-source commentary and redirect; final editorial acceptance pending.

## Coordinator round 2 redirect → round 3 disassembly boundary
Accepted `analysis/redirect/r2-20261105/handoff.md` as ready coherent research batch after reading its primary-artifact index, conditional contracts and scoped deferred-review.md. Graph reached stable methods with no pending queue/budget interruption; all selected actor/projectile/PPU/utility/resource/pattern and return avenues followed. Reopened stableR1-L04/L05 propose17727newbytes(13509code+4218data),zeroaccepted. These are proposals, not classification; input remains pass-007:101983/131088(77.80%),29105unknown. Coordinator independently verified all6 recorded protected identities, including configured binary, current map/state, ROM and commentary source/symbols unchanged.
Redirect worker completed before separate `/root/round3_disassembly` launch. New worker independently reviews exact bank/stack/conditional/data-boundary claims and supported connected continuations, then checkpoints/rebuilds; no universal normal-return/livecaller assumptions. Current corrected commentary remains untouched until next full-preservation/carryover phase. No terminal-incomplete/exhaustion assessment accepted. Previous coordinator continuation progressed through resource/return evidence and current continuation completes next required phase boundary. Toolkit issuesNES-001confirmedbug andNES-002missingfeature nonblocking; no toolkit modification/archive/cleanup/publication. Final editorial audit and final archive reinventory pending.


# Round 3 disassembly complete — pass-012

ROM classification: 94.04% (123275 / 131088 bytes classified; 7813 bytes unknown).

Code 45066; data/text 78209. New accepted bytes21292:13627code+7665data/text. Accepted entire independently checked17727byte r2 proposal (13509code+4218data), plus225 connected frame/pattern bytes,2435 localized patterned noncode bytes,118 reset/helper code bytes and787 literal text bytes. All Static; no additional execution/natural reachability claimed. Input unchanged; no runtime replay or baseline initialization. Existing broad artificial progression/quest-over scenarios remain reusable in analysis/scenarios.json.

Verification: final-validation.json passes; analysis/pass-012-verification.json ok=true,byte_identical=true; frozen maps/pass-012.json, generated/pass-012/game.asm; current map hash e5aa38e38ff8048be97991c8eec3ea63160afd2af2b79db97629a5748e838016. Exact internal counts/per-bank in final-stats.json; residual bank-split unknown-ledger.json totals7813. Historical pass-008 is a no-gain checkpoint from an insertion-schema retry; pass-009 original proposal,pass-010 connected225,pass-011 patterns/reset,pass-012 text remain preserved. Never infer final state from an earlier checkpoint hash.

Independent review: fresh configured-binary decode checked6347 effective-context views/6126instructionstarts, mapping, opcodes/lengths, starts and nonoverlap; independent-audit.json. Primary roots/role/return/bank/data evidence remains immutable in analysis/redirect/r2-20261105. Rechecked collision no-hit RTS against PLA/PLA hit exits; audio enabled-six-tick phase5; projectile state3 expiry; bounded decoder output and stream termination; concrete mapper contexts and literal callbacks. These are conditional path contracts, not all-path or arbitrary-caller safety. Role families combine accepted dispatch consumers, parameter semantics and closed control/bounds, not pointer values or legal decoding alone.

Connected review: connected-data-review.json records exact frame consumers/patterns/neighbors. residual-pattern-review.json localizes repeated small-value/sign-extension arrays after reference/neighbor/code-conflict review; generic data claims do not assert original schemas or generators. Mixed1C53A was explicitly split at reset SEI1C5D6, never swept wholesale as data. Reset helper8245 clears internal RAM pages07..00 skipping stack01; pointer bounds preserve mapper. Reset-vector prologue followed explicit bank0 selector and balanced helper/setters, stops after C9D7 call before unsupported return. reset-listing/reset-helper/reset-setters/reset-accepted preserve this. text-review.json contains69 exact readable/reversed language spans; command bytes/terminators were not inherited from strings.

Lead outcomes: R1-L04/R1-L05 verified for exact r2 batch; connected easy continuations accepted. Actual easy-gain gate: original ready graph is stable below its bound with no pending queue; additional obvious array/text/reset continuations reviewed rather than stopping at original ranges. Remaining mixed resources, scene callbacks and script command headers need local grammar/context/stack facts; no speculative inline tails, orphan JMP or legal-decode sweeps. This is ordinary easy-gain handoff, not terminal research exhaustion or full-ROM completion. Strong next questions: (1) Which concrete C9D7/C9C2 scene callbacks return to reset/title and fixedCDF2 tails, and under which bank/frame/stack contract? (2) Can residual bank1 projectile/collision bodies415F..4427 and bank9 scene/PPU text commands12C83..13F00 be bounded by newly readable callers? (3) Identify structural endpoints for bank5 B8DB..BFFF mixed resource and bank13 title/game-over payload; do not inherit compressed appearance as whole-object proof. (4) Resolve remaining short frame/script fragments and pointer/high-byte aliases independently, especially1C2CC and text command gaps. The fresh source supplies exact local context for commentary/redirect.

Next separate phase must preserve the ENTIRE current commentary directory/version before fresh preparation/carryover; no commentary edits performed here. Complete current-source readability remains pending, final editorial acceptance separate. Toolkit findings: existing confirmed nonblocking NES-001 and missing precision feature NES-002 retained in analysis/toolkit-issues.md; no new toolkit issues found. No toolkit edits, archive, publication, cleanup/deletion or subagents. ROM/source/commentary/configured-binary identity evidence in final-identities.json. Free disk checked at start8.6GiB,scratch17MiB; all cited evidence retained.

| PRG8K bank | Code | Data/text | Classified | Unknown |
|---|---:|---:|---:|---:|
| header | 0 | 16 | 16 | 0 |
| 0 | 334 | 7843 | 8177 | 15 |
| 1 | 636 | 7314 | 7950 | 242 |
| 2 | 2359 | 5030 | 7389 | 803 |
| 3 | 795 | 6841 | 7636 | 556 |
| 4 | 424 | 7768 | 8192 | 0 |
| 5 | 5600 | 744 | 6344 | 1848 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 338 | 7657 | 7995 | 197 |
| 8 | 0 | 8190 | 8190 | 2 |
| 9 | 2280 | 4063 | 6343 | 1849 |
| 10 | 6408 | 1771 | 8179 | 13 |
| 11 | 6029 | 1774 | 7803 | 389 |
| 12 | 3318 | 4857 | 8175 | 17 |
| 13 | 2502 | 5142 | 7644 | 548 |
| 14 | 6425 | 919 | 7344 | 848 |
| 15 | 7618 | 88 | 7706 | 486 |

## Coordinator round 3 disassembly → commentary boundary
Accepted compact `analysis/round3/handoff.md` as ordinary easy-gain pass completion, not terminal research exhaustion. Coordinator independently counted123275classified/7813unknown,94.04%, verified current portable map matches pass-012 byte-identical report and input ROM identity unchanged. Exact required percentage printed to user before next phase. Roundgain21292 includes full17727readybatch plus3565supported connected reset/data/text bytes. Full perbank residuals and source/grammar/callback questions preserved.
Disassembly worker completed before separate `/root/round3_commentary` launch. Worker must preserve ENTIRE correctedpass007 commentary directory/version and identities before freshpass012 prep, carry valid names/prose/review by ranges/dependencies, and complete expanded whole-current-source readability. Specific prior operand/label numericcollision defect and correction audit supplied to prevent recurrence across allnew/carried source. Fresh verification/reading copy and complete scope assessment remain required before redirect. Final editorial audit and terminal assessment pending; no archive/cleanup/toolkit edits/publication. This continuation progressed through independent validation/classification and next required sequential phase boundary.

# Round 3 commentary — active, pass-012

Entire corrected pass007 version preserved before preparation in analysis/round-3/commentary-pass012-20261105/previous-commentary; per-file size/SHA256 identity-manifest.json. Fresh preparation succeeded byte-identically. All3394 prior classified regions retain exact physical bounds/kinds/views; prose/symbols/review carried only at exact physical matches. New411 regions (13627code+7665data/text bytes) and changed dependencies remain pending; exact pending-ranges.json in phase packet. Current source readability is NOT complete. Final editorial audit pending. Next: read new code by functional family, explain mechanics and conditional return/bank/stack contracts, context-resolve labels, audit ALL addressing classes to prevent constant/RAM collisions, update full scaffold/units and verify/read at full-scope handoff. No classification changes, runtime replay or toolkit edits.

# Round 3 current-source commentary complete — pass-012

Preserved the entire correctedpass007 commentary before preparation at analysis/round-3/commentary-pass012-20261105/previous-commentary. identity-manifest.json covers all26 files; final checks show zero mismatches. All3394 prior classified map regions retain exact physical bounds/kinds/views, so valid prose, symbols and review credit were carried by physical match. Fresh pass012 source was independently prepared and verified before edits; frozen before/ remains intact.

Readability now covers all45066code and78209data/text/header bytes. New13627code bytes were read in compact-new-code.txt and explained with248 routine/control contracts;203 new data/text slices receive bounded format/role/uncertainty notes, including118 meaningful structures/text units. All3979 scaffold records remain intact and reviewed for current-source readability, with1222 units. Remaining classified readability ranges: none. Initial pending-ranges.json is preserved as the preparation frontier, superseded by final-scope.json. Final editorial audit remains pending;7813unknown bytes remain structurally unresolved.

Addressing-class audit checked all21035instructions:10567memory,4233immediate,6235control/implied; zero value/storage-class/bank-table mismatches. Immediate values never generated RAM-role labels. Memory$04/$08 retain scratch_ptr/work_byte rather than PPU/controller constants.44 new bank-specific CPU data aliases were checked across50 uses; physical Backing labels remain distinct from CPU aliases. Names preserve exact values and widths. Carried dependencies were rechecked through current contexts and byte-identical assembly.

Comments explain title glyph/PPU upload and numeric conversion, projectile orbit/profile/contact mechanics, actor response/line-motion/descriptor families, player recovery/slope/contact paths, actor81's saved-coordinate substates and reset setup. Conditional no-hit/hit unwinding, dispatcher state3 expiry, callback bank/stack restoration, bounded decoder output and TickAudio phase5 premises remain explicit. These do not assert arbitrary-input safety or natural reachability.

Examples:
- `ClearInternalRamExceptStack`: `commentary/source/game.asm:247`.
- `DrawTitleAttributesAndNumbers`: `commentary/source/game.asm:1016`.
- `Convert24BitValueToSixDigits`: `commentary/source/game.asm:6911`.
- `StepActorControlWordStream`: `commentary/source/game.asm:16322`.
- `UpdateActor81TimedSubstates`: `commentary/source/game.asm:27764`.
- `ResetAndInitializeTitle`: `commentary/source/game.asm:35220`.
- `ResolvePlayerLandingAndTileResponse`: `commentary/source/game.asm:42354`.

Fresh final verification after source edits: ok=true,byte_identical=true. reading-after.asm refreshed. Source SHA25621c25bc9114aa3a00826b5b6de745e600a9a0b7edb86d5f5465f113af4677c90; symbols SHA256edfffb2341c8a8c853dc337944392e6dd87c264dea4379fca067b2b87ea852c1. final-identities.json confirms input ROM, map/state and configured release binary unchanged, frozen map/state match current, and prior version intact. Final scope/review/verification are preserved in this phase packet. No classification changes or runtime replay.

Unresolved meanings and bounded next checks are in commentary/uncertainties.md: scene callbacks C9D7/C9C2 and fixed tails, residual projectile/contact code, bank9 text command grammar, mixed resource endpoints and physical1C2CC aliases. Generic noncode claims do not invent asset schemas; literal text does not validate adjacent commands. Classification remains123275/131088(94.04%),7813unknown; this is current-source commentary completion, not full-ROM or final editorial acceptance.

No new toolkit issues found. Existing nonblocking NES-001 and NES-002 remain in analysis/toolkit-issues.md. No toolkit edits, disassembly/redirect work, final audit, archive, cleanup/deletion, publication or subagents.

## Coordinator round 3 commentary gate — targeted naming continuation
Preliminary handoff `analysis/round-3/commentary-pass012-20261105/handoff.md` inspected with primary source samples (24-bit decimal conversion, actor word-stream grammar, actor81 substates, player landing/stack restoration). All current recorded hashes match and coordinator independently verified all26 preserved prior-version files identical. Source explanations substantively describe algorithms/preconditions, but understood six-cell digit buffer$038C remained numeric and proved actor81 substate entry addresses lacked distinct descriptive labels inside a shared region. Returned SAME commentary worker for targeted buffer/dispatch-entry naming correction and analogous concrete new gaps, preserving sound prior prose/review and preliminary evidence. Fresh verify/read/location refresh required. No redirect launched; final editorial audit remains separate. This continuation makes concrete source/readability progress rather than accepting review counts alone.

## Round 3 commentary corrective supplement — complete

Coordinator review found concrete naming gaps in the six-digit converter output and actor81 table entries. Resumed the same commentary phase. Preserved the complete preliminary current commentary in analysis/round-3/commentary-pass012-correction-20261105/preliminary-commentary with a per-file identity manifest; the original complete packet remains historical. Correction does not change classification.

Named decimal_digit_buffer at RAM$038C..$0391 consistently across all five absolute-indexed converter/queued/direct-PPU references; exact widths and bytes retained. Actor81 table bytes96B9C0B9D0B9 at physical179A0 independently confirm entries179A6/179D0/179E0 under[10,11,14,15]. Named their frame countdown, seeded hold and frame-restoration/retirement roles locally. Checked analogous packed-motion, line-motion, boundary-motion and guarded behavior-response entries, plus accepted actor/projectile role roots; added descriptive labels at sixteen additional established entry addresses, without treating invalid behavior selector0 or repeated suffix pointers as new selector proof. Specialized saved-coordinate, stack-unwind and bank contracts remain intact.

Refreshed all inventory source locations. All3979 scaffold records and1241units remain reviewed for current-source readability; final editorial audit pending. No remaining classified readability ranges. Fresh final rebuild succeeds ok=true,byte_identical=true and reading-after.asm is refreshed. Full21035-instruction addressing audit again passes with zero mismatches. Source SHA25637f29e235c8cb9bcf54f9c4c0c0ee03943b3c0c04fff3d2dd32d6e12f84adb83; symbols SHA256386ef08f13d7eaf564ce165a5a45e204008bf87c768b1b7eeab24d496937f934. Immutable input/map/state/configured binary identities match the disassembly handoff; preserved preliminary version identity checks pass.

Concrete source changes and proofs are in correct.py,entries.py,entry-audit.json,new-entry-units.json,scope.json and final-addressing-validation.json in the corrective packet. Current examples use Actor81CountDownFrames,Actor81HoldBeforeReturnFrames,Actor81RestoreFramesOrRetire and decimal_digit_buffer. Existing commentary/uncertainties.md remains the redirect question list; no semantic research question was replaced by fabricated certainty.

Classification remains123275/131088(94.04%),7813unknown. No new toolkit issues found; existing nonblocking NES-001/NES-002 remain in analysis/toolkit-issues.md. No toolkit changes, disassembly, redirect, final editorial audit, cleanup/deletion, archive or publication.

## Coordinator round 3 commentary → redirect boundary
Corrective supplement `analysis/round-3/commentary-pass012-correction-20261105/handoff.md` accepted after primary examples and current hash recheck. Digitbuffer$038C now consistently named across five uses; actor81's three encoded substates plus16 analogous proven entries now have descriptive labels/units. Full21035-instruction addressing audit zero mismatches; all3979 scaffold records/1241units complete for current readability. Fresh byte-identical source verification and reading copy current. Preliminary packet and full priorversion remain preserved; coordinator independently verified prior26files before correction. Final editorial audit remains pending; this is not ROM completion.
Commentary worker finished before separate `/root/round3_redirect` launch. Input unchanged pass-012:123275/131088(94.04%),7813unknown. New worker follows exact residual callback/scene/script/resource/bank/return questions with prior sound evidence, no unchanged gameplay replay. Required terminal rejection review and coordinator primary-artifact inspection remain mandatory if incomplete exhaustion proposed. No terminal decision yet. This continuation made progress through fullsource edits, targeted naming repair, verification and next required separate phase.

## Redirect r3 research checkpoint — SAME phase active

Accepted input unchanged pass012:123275classified/7813unknown. Latest code candidate union1776 bytes (`seventh-flow-candidates.json`); data proposal union2557bytes (`data-proposals.json`), overlap0; combined4333proposed, ZEROaccepted. Allprotectedfiles unchanged; no runtime replay/toolkitedit.

Connected scene/reset/projectile contracts, exact roots/stops and conditionalpredicates are durable in first..seventh-flow packets. ResetC9D7 is stage-tail loader, not scene wrapper; BAD5 boundeddecode704source/1408output terminated. Pause atphysical1CD7A has savedoldvideo+callerJSR; palettePHA and final4PLA restorepalette/video/discardJSR, JMP CPUC700 (intentionalunwind, noordinaryreturnclaim). Its fixedpostselector1CE09 usesunknownlowbanks [null,null,14,15], explicitCLC/validstreamPPUreturn predicate; CEEE restoresouterbank after literalcallback, not blanketcallbackbankpreservation. ViewportD318 callback has reviewed finitecopies plus cameraX/Ylow=0 QueueScrollChanges fastpaths and balancedstack; opens12D8B. New actor9Binitializer1BDA9 is coherent gatedallocation/fieldwrites/RTS; FA3D independentlyreturns47fallback whenfull (noallocationfailureunwind).

Data: `data-proof.py` parsespause/legal bank9 streams13D5F,13DC2,13E07; threebyteheaders, FFnextcommand/zeroend, everylow7count matchespayload andPPUaddressvalid. C2CC128byte residualsplitslowvaluedarrayprefix, fiveFFtileanimations andfivepointerpairs; apparentleading CD/FFpointer aliasespriorstream; F454/F464 independentlyconsume. B8DB1845 andbank13BF6B165 each havefullhexalternatingsparse/densequartets toindependentphysicalbankend; genericnoncode only, noassetobject/codec/fillclaim. Bank13A678186 hasframeindex/pointer/header/signedoffsetconsumer; fixedC010168 containsmapperbusconflictbytes/addressarrays.

Currentfrontier: `direct-control-target-review.json` findsNOremainingdirectactualbranch/calltarget inunknown amongcurrentacceptedcode (JSRsuccessor ispostcall, separatelyrecorded `direct-gap-leads.json`, NOTunconditionalroot). Ninepostcallbarriers remain: CPUC9C2 underbank3; F876 restorebank5motion; actor818Dmovementdispatchreturn; specialtileF80D conditionalcontext/stack; F3B5 mapselectorcontext; EB25 playercarrycontext; F81C restoredcallbacksuffix. These needindividualcontracts, notautomaticfallthrough. Largest bank9mixedgaps12E32/12F2B/13043/13093/1323F/13CC2 have boundeddispatch/scriptorigins unresolved; investigate finitecurrent-source selectors/initializer domains, notarbitraryPC. Remainingtextcontainermarkers17D1A..18000 andcompressedinter-objectzerogaps warrantgenericnoncode/grammarreview withoutinventedsemantics. Existing research-residual-ledger isat sixth-flow; refreshatnextproposalbatch. No terminalassessment orphaseexhaustionclaimed.

## Connected flight/NMI/text closure — SAMEphase active

Latest eleventh-flow:3613proposedcodebytes. Combinedcode/datauniqueproposal7340, acceptedunchanged7813unknown; researchresidual473bytes in48bank-splitintervals, exact `research-residual-ledger.json`. Noclassification orobservedexecutionclaim.

Accepted28-entry FlightWaveHandlerPointers12DF1 suppliescoherent staticfamilyroots (`flight-wave-roots.json`), all28inspected/followed. Scene072F counter11wordtable130A6..130BC isindependentlyboundedbyfirsthandler; writersseed0/1, choose4/6/9/10 andadvance, coherenttimer/text/reward/actorbodies (`scene-counter-roots.json`). Localpackedtimer818D low-nibble0 immediateRTS andspecialtileF80D excludedC8..CD callbacknormalreturn openplayer/actorcontinuations. C9C2literalwrapperreturn nowcontext-independentonceexplicitcallbackoriginsresolved; savedbank restorationdistinctfromcallbackreturn.

NMI5wordtable1C835 lastmode4 CPUC84B hasexplicit sceneASL2->video_mode4 setup. ItsCF1Escriptprocessor selectsbank5 andconsumesliteralBD0A initializedby12C83. `story-text-proof.json`:167records/750bytes, FFsingleblankrow, positionbyte+zero-terminatedtext, FEterminalat18007. This converts genericintertextcontrolgaps intoconsumer-anchoredscriptdata; genericrecord notescorrectedfromlength-liketo position-like prefixes. CF1E andbounded6-OAM-row helpercode followedasoneconnectedbatch.

Independentdatareview addsallmatchingpatternedbanktails (3FCB,7EFE,FF73,13FE7), bank2descendingstep4vector, smallfixed/motion/thresholdvectors, zerointer-objectgapsafterbothneighborData/Textboundaries, andstorymarkerfragments; all exactrationales in extra-data-proposals.json. Noassetobject/generator/fillclaim inheritedfrompattern.

Currentfrontier islocalized473bytes: finish newlyboundedsceneheader/count/FF/zero records and11wordpointer/framearrays; typedsmallspawn/responseobjects andfixedNMI/camera tables; remainingconditionalmap-select/postcallbackcode at1E879,1EADC,1F84B andsmallactor174EF. TrulyambiguousorphanRTS723D/15F7B andJMPA6C4 stayunknownunlessindependententry/roleappears. No unchangedreplay. No phaseexhaustionclaimed.

## Round 3 redirect ready — pass-012 unchanged

Completed combined research batch at analysis/redirect/r3-20261105/handoff.md. Proposes7784unique newbytes(3781code+4003data), accepts0; acceptedclassification remains123275/131088(94.04%),7813unknown. Freshconfigured-binaryreview1776newinstructionviews:zeroopcode/length/text/mappingmismatch,zerocode/dataoverlap. Exactbank-splitacceptedunknownledger/dispositions andunknown-onlyproposalspans saved. Ifallaccepted,29bytesremain:723D1,A6C4three,15F7B1,1FFC7twentyfour. Compactscopeddeferredreview includesactualdecoderalternatealignment/operandtargetcounterexample, notblanketnocodeclaim.

Samephasefollowedall28flightwavehandlers,11scenecounterhandlers,nineflightsubstates,scene/reset/pause/collisionandNMIstoryconsumer plusindependentpattern/pointer/array/text/resourcequestions. Finalqueueempty; no scope/quota/firstready stop. Explicitconditionalreturn,callbackbankrestoreversusselectedmapbankandhitunwind contracts retained. Storyposition/textgrammar167records/750bytes endsFE18007. Genericnoncodeclaims do notinventwholeassetobjects/generators. StableR1-L04/R1-L05 reopenedready witholdacceptedhistorypreserved.

Next separatephaseindependentdisassembly ofcombinedbatch, thenfullcurrent-sourcecommentary; finaleditorialpending. Thisisnotterminalgoaldecision orwholeROMcompletion. ProtectedROM/map/state/source/symbols/configuredbinaryunchanged(final-identities.json). No newtoolkitissues; NES001/002 nonblockingretained. No runtime/native/gameplayreplay, toolkitedit, cleanup/deletion,archive,publishingorsubagents.

## Coordinator round 3 redirect → round 4 disassembly boundary
Accepted compact `analysis/redirect/r3-20261105/handoff.md` as coherent ready batch after reading detailed scoped deferred-review.md and primary-artifact index. ReopenedR1-L04/L05 propose7784newbytes(3781code+4003data),zeroaccepted. Input remains pass-012:123275/131088(94.04%),7813unknown; proposed29residual bytes are conditional accounting, not currentacceptedcount. Deferredreview separates two singletonRTS, isolatedJMP and24byte decoder-like region into tested claims/alternates, and restricts badJSR counterexample to unchanged-clone interpretation. No terminal-incomplete assessment accepted. Coordinator independently verified all6 protected identities unchanged, includingconfiguredbinary/ROM/map/state/commentary.
Redirectworker finished before separate `/root/round4_disassembly` launch. Independentworker validates roles/bounds/conditionalbankstack/data claims, may revisit/narrow deferredinterpretations under evidencepolicy rather than historyveto, preserves allpriorversions and currentcommentary, thenfreshcheckpoint/byteidentity. No forcedclassification from legaldecode or objectadjacency. This continuation progressed through finalizedresearch, preserved scopeduncertainty and nextrequiredsequentialphaseboundary. Finalcurrent-sourcecommentary, redirectstoppingreview, editorialaudit and terminalarchiveinventory remainpending.

## Round 4 disassembly active — independent review
Input pass-012 unchanged:123275classified/7813unknown. Reviewing7784byte ready batch against primary reset/pause/projectile contracts, exact contextual decoding and bounded data grammars. Frontier/evidence in analysis/round4/review.md; full current commentary protected by input-identities.json. No classification accepted yet; no runtime replay, toolkit edits or cleanup.


# Round 4 disassembly complete — pass-013

ROM classification: 99.98% (131059 / 131088 bytes classified; 29 bytes unknown).

Accepted7784newbytes:3781code+4003data, all Static. Exact current kinds: {'Code': 48847, 'Data': 81412, 'Text': 800}. R1-L04/L05 verified; all prior classified intervals preserved. Independent configured-binary opcode/length/text/mapping audit covers1776newinstructionviews. Data literal bytes and complete PPU grammar counts checked independently; story167records plusFE terminator. Evidence: review.md, independent-audit.json, new-code.txt, annotation-results.json and immutable analysis/redirect/r3-20261105 primary contracts/proofs.

Conditional stack/bank predicates retained: reset C9D7 stage-tail loader distinct from scene C9C2; explicit callback restoration separate from return; pause and projectile hit paths intentionally discard return frames; unknown map-bank slots remain null. No observed or natural-reachability upgrades. Existing broad artificial later-stage/boss/reward/quest-over evidence reused from scenarios; no unchanged runtime/native replay or baseline initialization.

Fresh map.validate passes; maps/current.json exported; analysis/state.json saved. maps/pass-013.json and generated/pass-013/game.asm verify ok=true,byte_identical=true in analysis/pass-013-verification.json. Map SHA25636f1a1c42910c33c11752baca0153176d41de82419e682d1647305681fde93c0. ROM/configured binary and ENTIRE prior commentary unchanged (protected-identity-check.json). Current commentary remains pass-012 and requires full preservation/carryover before next full-current-source phase. No commentary work performed here.

| Physical PRG8K bank | Code | Data/text | Classified | Unknown |
|---|---:|---:|---:|---:|
| header | 0 | 16 | 16 | 0 |
| 0 | 334 | 7858 | 8192 | 0 |
| 1 | 775 | 7417 | 8192 | 0 |
| 2 | 3031 | 5161 | 8192 | 0 |
| 3 | 1065 | 7126 | 8191 | 1 |
| 4 | 424 | 7768 | 8192 | 0 |
| 5 | 5600 | 2589 | 8189 | 3 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 372 | 7820 | 8192 | 0 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 3858 | 4334 | 8192 | 0 |
| 10 | 6408 | 1783 | 8191 | 1 |
| 11 | 6146 | 2046 | 8192 | 0 |
| 12 | 3335 | 4857 | 8192 | 0 |
| 13 | 2559 | 5633 | 8192 | 0 |
| 14 | 6938 | 1254 | 8192 | 0 |
| 15 | 8002 | 166 | 8168 | 24 |

Easy-gain gate: reviewed all connected ready roots/families and explicit return barriers; independent connected-closure.json combines accepted and proposed instruction references and finds zero direct control targets into residual29bytes, zero graph stops/fallthroughs there, pendingqueue0. No additional concrete supported continuation emerged. The four residual physical objects remain723D(1),A6C4(3),15F7B(1),1FFC7..1FFDF(24). Independently reviewed primary boundaries and alternate alignment/overlapping FDE6 call; no new discriminating code/data contract. Their uncertainty is localized, not inherited global rejection. See review.md and r3 deferred-review.md. Keep unknown pending a materially different contract/representation inference, not more legal decoding or speculative padding.

ROM incomplete; this is ordinary single-pass easy-gain completion, not terminal research exhaustion. Next separate full-current-source commentary must preserve the entire existing version first, then review all current classified source; final editorial acceptance and any terminal redirect review remain pending. No new toolkit issues found; existing nonblocking NES-001/NES-002 remain in analysis/toolkit-issues.md. No toolkit edits, cleanup/deletion, archive, publication or subagents. Handoff disk8.4GiB free, scratch17MiB; historical attempts preserved.


## Coordinator round 4 disassembly → commentary boundary
Accepted `analysis/round4/handoff.md` as ordinary easy-gain pass completion. Coordinator independently counted131059classified/29unknown(99.98%), checked currentmap matches pass-013 byte-identical report and inputidentity unchanged. Exact percentage printed to user before commentary. Full7784readybatch accepted after1776view/rawgrammar/conditionalcontract audit; fourlocalized residualobjects remain unknown withoutunsupportednarrowing. No terminal-incomplete decision yet.
Disassembly worker finished before new separate `/root/round4_commentary` launch. Worker must preserve ENTIRE correctedpass012 commentaryversion/identities beforepreparation, carry validcredit byphysicalranges/dependencies and complete allnew/affectedsource readability. Explicitaddress/grammar/unsigned-comparison corrections and prior operand/knownbuffer/dispatchentry namingdefects supplied topreventrecurrence. Current-source verification/readrefreshgate remains before nextredirect; finaleditorialaudit reserved until terminalacceptance. This continuation progressed through independentclassification/byteverification and nextrequiredsequentialphaseboundary. No toolkit/ROM edit/archive/cleanup/publishing. Finalarchive reinventory pending.

Round4 commentary active: entire corrected pass012 commentary preserved under analysis/round-4/commentary-pass013-20261105/previous-commentary, all-file identity manifest verified. Preparing pass013 baseline; exact physical-range carryover followed by all changed source/dependency review. Final editorial acceptance pending.

## Round4 pass013 commentary active — same phase

Entire previous corrected pass012 commentary preserved in analysis/round-4/commentary-pass013-20261105/previous-commentary; all-file identity-manifest.json verified before fresh preparation. Fresh baseline byte-identical. Exact physical-range carryover retained 3805 classified region records and their sound names/prose/review. New scope is3781code+4003data bytes,1765unique instructions/1776contextual views.

Completed: all new instruction listings reviewed;88 functional routine/control sections applied;138 new data fragments explained using47 meaningful structures and primary grammar/pattern/consumer evidence. New scene, flight, projectile, reset/pause, NMI/story and actor entry names are in source. Scoped hit/pause stack unwinds, explicit callback bank restoration, unsigned actor direction, story167position/blank records plusFE corrected. Full scaffold retained and source locations refreshed once; final refresh still required.

Verification after instruction batch: ok=true,byte_identical=true (source6592889a77d3be7b945265f4ff6be0d4de5270c8fcadf2ec91c6ba65087020d8). Second coherent batch plus dependency corrections also verifies byte-identically (sourcefb34bbabe379a25e91a44aa39e714f3c8f9d7e0adc2aaa0a399b79331fa87877).36 bank-specific data aliases across65 uses recorded; full addressing-class audit remains pending.

Remaining SAMEphase: finish local RAM/buffer aliases and meaningful finite entry labels; audit every addressing class/bank alias including carried dependencies; remove stale multiline text/PPU/reset uncertainty prose; refresh truthful full review source locations/units; fresh final verify and reading copy. Unknown four objects retain29bytes and mechanical boundary/uncertainty notes. No phase handoff until all current-source checks finish. Final editorial audit remains pending and will not run here. Protected ROM/map/state/configured binary unchanged; no toolkit edits or new toolkit issues (NES001/002 remain nonblocking).

## Round4 commentary pass013 — full current-source scope complete

Current-source readability complete: all131059classified bytes,22800instructions;29unknown bytes remain with useful mechanical boundary notes. Classification99.98% is separate from readability and final editorial acceptance. No remaining classified-source readability ranges.

Entire corrected prior commentary preserved before fresh preparation under previous-commentary/ with all-file identity-manifest.json; all identities rechecked. Fresh pass013 baseline and exact physical-range carryover retained3805sound classified records. All new3781code+4003data bytes and affected dependencies reviewed; source includes88functional sections,51purposeful branch refinements,47data structures across138new fragments, and meaningful labels for all39entries in the28-wave/11-substate tables. Shared decimal_digit_buffer and prior finite actor entry names retained. Three newly recognized preexisting wave-entry suffixes named mechanically. Review scaffold5712records retained; all1374units have current source locations/status/summaries.

Corrected CPUC9D7stage-tail loader versus CPUC9C2/physical1C9D7scene wrapper; reset continuation uses explicit saved-pointer/bank restoration and bounded BAD5decoder. Story prefixes arepositions,FFblankrow,167records plusFEterminal; final8bytes outside records remain generic noncode. Pause pressed-edge loops are distinct from held input; final palette/video restoration and caller-return discard is intentional. Projectile hit four-PLAunwind and no-hitordinary return scopes remain explicit. Actor174EFuses unsignedborrow/equality relative-direction selection. No universal callback/bank/overflow/natural-reachability claim.

Addressing audit covers all22800instructions:11271memory,4714immediate,6815control/implied;zero value/storage-class/known-bank-table mismatches. Expanded bank audit checks122uses,including oldbank-specific and newly named structure aliases;zero bank-context mismatches. New RAMaliases describe saved scroll,scene timers and localstorypointer/row storage; memory$04/$08 never inherit PPU/controller constants. Immediate operands do not acquire RAM-role labels. Exact widths/values preserved by successful assembly.

Fresh final verification ok=true,byte_identical=true; reading-after.asm refreshed. Source SHA2562bf9e642c0cd579088df4f8d4d16acc184d73afd5ef2eba0a1337961d8fe80ab;symbols SHA2562bde586d909c8c2611e5211eeda2087b933f440af58bb6600ec3b587ba993867. Final identities match protectedROM/configured binary/workspaceconfig and pass013map/state; prior version intact. Proofs:final-verification.json,final-identities.json,full-scope-review.json,final-addressing-validation.json,expanded-bank-alias-validation.json,finite-dispatch-entry-review.json.

Examples (source/game.asm):
- MoveProjectileDownAcrossHeightProfiles: line2659.
- ChooseActorHorizontalRelativeDirection: line34884.
- RestorePausePaletteVideoAndUnwind: line46936.
- StepScrollingStoryScript: line47522.
- ReturnFromEmptyFlightWave: line45547.
- ContinueResetAfterStageTailLoad: line44479.

Current uncertainties and next questions: commentary/uncertainties.md. Four unknown objects723D(1),A6C4(3),15F7B(1),1FFC7..1FFDF(24) retain exact ambiguity; no fabricated classification. New evidence would need a discriminating typed entry/representation/overlap contract. Generic asset schemas, natural selectors and some audiovisual identities remain uncertain without skipping established mechanics. Prior known callback/grammar questions updated rather than repeated as unresolved.

Ready for separate redirect. Final editorial audit REMAINS PENDING until final acceptance after redirect; audit not run. ROM still incomplete. No new toolkit issues found; nonblockingNES001/NES002 retained in analysis/toolkit-issues.md. No toolkit edits, runtime replay, disassembly/redirect work, cleanup/deletion, archive, publication or subagents.

Round4 same-phase corrective naming audit active. Preliminary completed commentary preserved with every-file identities in analysis/round-4/commentary-pass013-correction-20261105/preliminary-commentary. Audit newly introduced local-role aliases against generated labels; preserve sound flight contexts. Final editorial audit remains pending.

## Coordinator round 4 commentary gate — final local-label consistency repair
Preliminary handoff and primary pause/reset/story examples inspected. Routine/data prose accurately separates pause unwinds, explicit reset resource restoration and storyposition grammar; full addressing/bankaliases pass. Coordinator independently verified all26 priorversion files identical. A newly local storypointer operand still had a generated branch/entryhint named ReadFlightDescriptorPtr, contradicting the established local alias. Returned SAME worker for narrowly scoped new-local-alias generatedlabel consistency correction, preserving sound source and preliminary packet; fresh verify/read/location refresh required. No redirect launched and no structural inference changed. This continuation progresses fullreadability/primarychecks plus explicithistoricalmilestone metadata, with finaleditorial and stoppingreview still pending.

# Round 4 commentary corrective supplement — complete

Resumed the same phase after coordinator found a stale generated flight-pointer hint in the story consumer. Preserved the entire preliminary completed commentary with an every-file identity manifest in preliminary-commentary/. The original completed packet remains unchanged.

Audited all 52 new-context memory operands using local story pointer/row/divider, scene timer/branch, saved scroll and viewport aliases. Corrected 43 generated labels: stale FlightDescriptorPtr/SpriteOffsetX/FineScrollY hints now name StoryScriptPtr/StoryRowIndex/StoryRowPhase; previously empty memory-role hints now identify their actual local storage. Purposeful entry labels and sound preexisting flight contexts were retained. This is naming only; no instruction, operand, structural or semantic change. Details and exact before/after names: local-alias-label-audit.json.

Refreshed inventory source locations. All 5712 scaffold records and 1374 units remain reviewed, with no classified readability gaps. Full 22800-instruction addressing audit passes; expanded 122-use bank-alias audit passes. Final verification ok=true, byte_identical=true; reading-after.asm refreshed. Source SHA256 b573399d3dfd5d7d7ff3a8af8c5fbdfc3141a6a63ae80fe3fcd340269f1d37d9; symbols SHA256 2bde586d909c8c2611e5211eeda2087b933f440af58bb6600ec3b587ba993867. Protected ROM/map/state/configured binary unchanged; preliminary preservation identities verified.

Classification remains 131059/131088 (99.98%), with 29 unknown bytes. Existing uncertainty notes remain current. Final editorial audit remains pending; no audit, structural research, redirect, toolkit edits, cleanup, archive, publication or subagents. No new toolkit issues; NES-001/NES-002 remain nonblocking.

## Coordinator round 4 commentary → redirect boundary
Corrective supplement `analysis/round-4/commentary-pass013-correction-20261105/handoff.md` accepted after primary storylabel/currenthash recheck. All52newcontext localaliasoperands audited;43stale/empty generatedhints corrected, soundflightcontexts retained. Full22800instruction/122bankaliasuse audits pass,5712scaffoldrecords/1374units current, freshbyteidentity/readrefresh complete. Coordinator independently verified prior26fileversion preserved; fullpreliminaryversion also preserved bycorrectivesupplement. No classifiedreadabilityrange remains; finaleditorialaudit pending.
Commentaryworker finished before new separate `/root/round4_redirect` launch. Input unchanged pass-013:131059/131088(99.98%),29unknown in4exactobjects. Newworker must applyevidencepolicy rejectionreview against decisiveprimaryartifacts, reusesoundunchangedreview ratherthan automaticretry, reopen onlyjustifiedcorrected/narrowed/integratedclaims, and provide terminalincomplete reasoning onlyif no usefulmethodsurvives. Coordinator primaryaudit mandatorybeforeanyterminalacceptance. No terminaldecisionyet. Thiscontinuation progressed source/data edits, targetedlabelconsistencyrepair, verification andnextrequiredphaseboundary. No archive/cleanup/toolkitedits/publishing; finalarchiveinventory stillpending.

## Round4 redirect — terminal incomplete proposed for coordinator review
Current pass013 remains131059classified/29unknown; zero proposals/ready leads. Exact residual ledger freshly recomputed against current map and ROM: analysis/redirect/r4-20261105/unknown-ledger.json. Primary linked rejection review separately evaluates isolated stubs/trampoline and fixed-fragment prefix/aligned suffix/mixed tail, scopes unchanged-clone counterexample and combines stack/control/representation constraints. No useful revised discriminating method survives; evidence-led ambiguity remains, not a caller/generator prerequisite. Handoff: analysis/redirect/r4-20261105/handoff.md. Coordinator primary review required before accepting stopping rationale. Full current-source readability accepted; final editorial pending. Protected identities unchanged; no toolkit issues added (NES001/002 remain nonblocking), no runtime replay/toolkit edits/subagents/archive/deletion/publication.

## Coordinator terminal-rejection gate — returned concrete corrected inferences
Read r4terminaldraft/rejectionreview plus decisive r3literal/local/aligned/target artifacts. Terminal NOT accepted. Combined-neighbor evidence was underweighted: orphanA6C4 duplicates the exact C7D4epilogue across a bounded30byte upload table and nextwriter; aligned1FFCCsuffix has knownbitreaderfield/shape correspondence plus actualbalancedLSR28,Xsubcall, so failedunchangedclone does not invalidate narrowerstatic bittransformrole. Returned exact questions to sameworker; worker agrees and nowproposes13Staticcodebytes pendingcoherentfollowthrough/independentclassification. Newleftboundary may affect mixedtail9/prefix5, so continue that connectedassessment beforehandoff. Originalterminaldraft preserved, newuniquesupplementrequested; analysis/coordinator-rejection-challenge.md recordsrationale. Acceptedclassificationunchanged99.98%/29unknown. Thisgoalturn makes substantivecorrectedreasoningprogress; no terminalsuccess or blockedclaim.

### Same-redirect corrected combined constraints — terminal proposal superseded
Coordinator challenged omitted local paired-epilogue/table layout and narrower decoder-shaped closed-stack transformation. Primary source comparison supports13Static code bytes atA6C4(3) and1FFCC(10), without original-use/correct-bitreader claim. See analysis/redirect/r4-20261105/combined-constraint-correction.md and corrected handoff.md. Initial terminaldraft preserved; no terminal gate requested. ReadyR4-L01/L02, accepted0, acceptedunknown29, predictedresidual16. Independent disassembly then full current-source commentary/redirect required. Protected identities unchanged.

### Same-redirect unique supplement: coherent22byte proposal
analysis/redirect/r4-20261105-supplement1/handoff.md supersedes preliminary stopping conclusion. Rootrequested original terminal handoff/review preservation honored; reconsideration isolated in unique supplement. Followed changed suffixRTSendpoint into adjacent9byte mixedtail; supported genericData after scoped content/boundary/conflict review, without schema/trap-exclusion prerequisite. ReadyR4-L01/L02/L03:13code+9data,accepted0,predicted7unknown. Prefix5 remains ambiguous. Independent disassembly then commentary/redirect required; no terminal gate/goalstatus.

## Coordinator reconsidered round 4 redirect → round 5 disassembly boundary
Originalterminaldraft NOT accepted; correctedjointconstraints produced coherent22byte readybatch under analysis/redirect/r4-20261105-supplement1/handoff.md, combined-constraint-correction.md and latermixed-tail-review.md. Lastreview supersedes13only section's tailwithheld premise. Proposed13StaticCode+9StaticData,zeroaccepted; inputremains pass013131059/13108899.98%/29unknown. Parentreadcorrectedreasoning/strongestalternatives and independentlyverified6protectedidentitiesunchanged. A6C4family-specificpairedPPUepilogue and1FFCCactualbalancedindexedbittransformrole are notnaturalreachability/unchangedcloneclaims; supportedRTSendpoint thennarrowsfollowing9bytesgenericnoncode withoutoriginalschema. Predicted7residual remainsconditional. Existingterminaldraft/rejectionprimary historypreservedunchanged andsupersededexplicitly.
Redirectworker finished before separate `/root/round5_disassembly` launch. Newworker independentlyvalidates22bytes/alternatives, retainsphysicalbank/context/alternateoperandentrycontracts throughsupportedmetadata, preservesexistingCodealignmentandallcurrentcommentary. No unsupportedoverlapAPI, no forcedclassificationof7residuals, no runtime/global replay. Newbytegain is smallbutconcreteuseful; fullsequencecontinueswithno minimumquota. Thisturn made substantiveprimaryrejectionreview/correctedinference/proposalprogress andrequiredphaseboundary; no terminalsuccessaccepted. Finaleditorial andfinalarchiveinventoryremainpending.

## Round 5 disassembly — pass-014

ROM classification: 99.99% (131081 / 131088 bytes classified; 7 bytes unknown).

Accepted R4-L01/L02/L03 independently:22 bytes (13 Static Code +9 Static Data). Total code 48860; data/text/header 82221. Exact residuals723D(1),15F7B(1),1FFC7..1FFCC(5), physical half-open. Review/evidence: analysis/round5/review.md and rpc.json. Preserved every existing accepted region unchanged; no source edits. Operand-byte FDE6 alternate entry recorded on new transform annotation; retain primary INC56/PLP/RTS representation and explain numeric call explicitly in subsequent commentary. No unchanged-decoder, intended-X or natural-use claim.

| Physical bank8K | Code | Data/text | Classified | Unknown |
|---|---:|---:|---:|---:|
| header | 0 | 16 | 16 | 0 |
| 0 | 334 | 7858 | 8192 | 0 |
| 1 | 775 | 7417 | 8192 | 0 |
| 2 | 3031 | 5161 | 8192 | 0 |
| 3 | 1065 | 7126 | 8191 | 1 |
| 4 | 424 | 7768 | 8192 | 0 |
| 5 | 5603 | 2589 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 372 | 7820 | 8192 | 0 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 3858 | 4334 | 8192 | 0 |
| 10 | 6408 | 1783 | 8191 | 1 |
| 11 | 6146 | 2046 | 8192 | 0 |
| 12 | 3335 | 4857 | 8192 | 0 |
| 13 | 2559 | 5633 | 8192 | 0 |
| 14 | 6938 | 1254 | 8192 | 0 |
| 15 | 8012 | 175 | 8187 | 5 |


Map validation and pass-014 byte-identical rebuild pass (analysis/pass-014-verification.json). Connected continuations closed at accepted fixed helpers/returns; no concrete additional easy gain emerged. No runtime scenarios replayed: prior broad artificial progression/quest-over coverage reused (analysis/scenarios.json). Seven bytes remain structurally ambiguous; see review.md for scoped alternatives. Single pass complete; ROM incomplete, next full-current-source commentary then redirect; final editorial audit pending. Existing NES-001/NES-002 nonblocking (analysis/toolkit-issues.md); no new toolkit issues found. No toolkit edits, source/commentary edits, deletion, archive, publication or subagents. Disk8.3GiB free/scratch17MiB.

## Coordinator round 5 disassembly → commentary boundary
Accepted `analysis/round5/handoff.md` as ordinaryeasygainpasscompletion. Independently counted131081classified/7unknown(99.99%), checkedcurrentmaphashmatchesbyteidenticalpass014 andROMidentityunchanged. Exactpercentageprintedbeforecommentary. All22correctedproposalbytes accepted;13StaticCode+9StaticData; everypriorregion/commentarypreserved. OperandbyteFDE6actualentrycontractrecordedwithoutalteringprimaryacceptedINC/PLP/RTSalignment. Remainingexactunknown723D1/15F7B1/1FFC7..1FFCC5; no terminaldecision.
Disassemblyworkerfinishedbeforeseparate `/root/round5_commentary` launch. WorkerpreservesENTIREcurrentversionbeforefreshprep/carryover, reusessoundreviewandexplainsnewfamilyepilogue/bittransform/genericdata+overlapaliaswithaccurateRAM/CPU/bankclasses, refreshesfullreview/verify/read. Finaleditorialauditreserveduntilafterredirect. Thiscontinuation progressed independentclassification/verification andnextrequiredphaseboundary; no archive/cleanup/toolkitedits/publication. Finalarchiveinventorypending.

## Round5 commentary — complete current-source readability

Pass014131081/131088classified99.99%,sevenunknown unchanged. Entire corrected pass013 commentary preserved with every-file identities before fresh preparation at analysis/round-5/commentary-pass013-preserved. Exact-range sound source/review carryover retained; all5714scaffold entries and units reviewed with current source locations. New13code+9genericdata bytes and affected PPU/decoder/operand-entry dependencies explained. PrimaryINC56/PLP/RTS alignment unchanged; alternate CPUFDE6 equate supports actual indexed zero-page bit transformation with modular X/alias/carry mechanics and no intended-use claim. All22806instructions addressing checks and122bankalias uses pass. Fresh verification byte-identical; reading copy refreshed. Full handoff analysis/round5/commentary-handoff.md; verification/identities/fullscope under analysis/round-5/commentary-pass014. No classified readability gaps; unknown7 still unresolved. Finaleditorialauditpending, not run. Ready separate redirect; NES001/NES002 nonblocking, no newtoolkitissues; protectedROM/map/state/binary unchanged. No toolkit edits, runtime replay, classification research, archive, cleanup/deletion, publication or subagents.

## Coordinator round 5 commentary → redirect boundary
Accepted `analysis/round5/commentary-handoff.md` after primarysourcecheck ofdetachedepilogue,actualSTA/ROL/overlapJSR/ROL/RTStransform andpreservedprimaryINC/PLP/RTSalignment. CPUFDE6equate/comment explicitlypointsintooperandbyte;Xzero-pagewrap/aliasing/writtenfields/finalflags describedwithoutintendedselectororunchangeddecoderclaim. Generic9tailandremainingprefix5notconfused. Full22806instruction/122bankaliaschecks+freshbyteidentity/readrefresh pass; coordinatorindependentlyverifiedall26priorfilesidentical andcurrentverifiedsourcehashesmatch. No currentclassifiedreadabilitygap; finaleditorialauditpending.
Commentaryworkercompletedbeforeseparate `/root/round5_redirect` launch. Inputpass014131081/13108899.99%,7unknown. Newworker mustreviewremaining3objectsagainstcurrentestablishedboundaries andscopedprimaryevidence, notreviveold29byteclaim ormisscoordinatorcorrections; no universalcaller/orgeneratorcriteria. Terminalincompleteonlyifactualcompetinginterpretationspersistwithoutusefulcorrectedmethod, thenparentprimaryauditmandatory. No terminaldecisionyet. Thisturn progressed fullsource/commentaryverification andnextrequiredphaseboundary; finalarchiveinventorypending.

## Coordinator round 5 redirect stopping gate — terminal incomplete research accepted
Read fresh r5rejectionreview and decisiveprimaries/currentliteralmapgapbytes, independentlyrecomputed7unknownandchecked31protectedidentitiesunchanged. ExactPHP/RTS syntheticstackeffect and code/datarepresentation alternatives remainmaterial; singletonRTSs isolatedbyrealneighborbounds butnoindependentrole. No universalcaller/generator/priorrejectionveto; accepted13Code+9Data corrections retained. No usefulcorrectedinference/discriminatingtest survives; no liveinterruptedfrontier/toolblocker. AcceptterminalincompleteRESEARCHstop undergoal/evidencepolicy; ROM stillincomplete. Detailedreasoning in analysis/coordinator-terminal-review.md. No furtherunchangedplanningcycle authorizedbytheseunchangedpremises. Finaleditorialacceptance stillrequired: fixthreeactualstaleproseissues andcomparablecontradictions, reusevalidreviewcredit, freshaudit/verify/read, finaldocumentinventory. GoalNOTcompleteyet.

## Final editorial acceptance in progress — 2026-11-05
Targeted source corrections and final inventory repair complete; fresh documented commentary audit running. Evidence and immutable pre-final commentary: analysis/final-editorial/20261105-acceptance. No classification changes; seven unknown bytes remain after accepted terminal-incomplete research review.

Final editorial audit PASS: ok=true, byte_identical=true, editorial_audit=true;5,714 reviewed scaffold entries and5,564 reviewed bounded units. Fresh reading-after.asm refreshed. Independent validation confirms11,278 valid source locations, every prior unit retained, exact scaffold unchanged, unchanged instruction/directive values except one label spelling, and protected ROM/map/state/configured binary unchanged. Evidence: analysis/final-editorial/20261105-acceptance/handoff.md. ROM classification remains terminal incomplete with seven unknown bytes.

## Final coordinator terminal handoff — orchestration complete, ROM incomplete
All required work complete for the documentedterminal-incomplete workflow outcome. Researchstop accepted after scopedprimaryrejectionreview and a coordinatorchallenge thatfirstrecovered22supportedbytes. Finaleditorialacceptance nowPASS: actualcommentary.py audit byte_identical/ok/editorial_audit true,5714regions/5564unitrecords; allpriorreviewcreditpreserved, stalefactualprose and metadatafixed, readingrefreshed. No classifiedreadabilitygaps; sevenbytes remaingenuinecode/dataambiguity, notblockedorinterruptedwork.
Independent configuredstore/map validationspass. analysis/verify-terminal-handoff.py passes fullcurrentphysicalaccounting/annotationviews/source+ROM+checkpointidentities/frozenbaseline/scaffold+unitlocations/editorialdecision/no-readyleads/documentselection; consolidated analysis/current-audit.json. Current checkpointpass014:131081/131088classified99.99%,7unknown,Code48860/Data81421/Text800. Exactresidual723D1/15F7B1/1FFC7..1FFCC5. ROM/configuredbinaryunchanged; allsource/assets/state/traceslocal.
Finalarchivecontractreinspection/inventorydone, archive-options.json selects65additionalfinal/historicalMarkdown andomits18byte-identicalduplicatephaseinstructions. All98Markdowninventorypathsaccounted underexactscriptdefaults/special/before-source exclusions; no missingselected/unaccounted/overlap, nofinalresult omitted. analysis/archive-document-audit.json recordsplanhash/dispositions. Noarchiveexecution,cleanup/deletion,publication,toolkiteditorwholeworkspacecommit. Finalresult/evidence scope: analysis/terminal-handoff.md and analysis/completion-audit.md. No requiredworkremains fororchestration; ROMclassificationexplicitlyINCOMPLETE. Futurelocalizedresearchrequiresconcretemateriallychangedmethod/evidenceoruserreassessment, notautomaticunchangedretry.
