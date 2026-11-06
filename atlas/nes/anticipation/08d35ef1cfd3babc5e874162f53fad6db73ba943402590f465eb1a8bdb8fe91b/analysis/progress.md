# Disassembly progress

Round 1 disassembly in progress (worker session). Baseline established once: [verification](baseline-verification.json), byte_identical=true. Input identity unchanged. Initial internal map loaded; classification annotations pending inspected startup candidates.

ROM: Anticipation, NES mapper 1; 32 KiB PRG + 32 KiB CHR; physical size 65552. Effective runtime PRG slots [0,1,2,3]. Reset vector $FFDD, NMI $DEE5, IRQ $DF24. Startup trace and reset/NMI discovery retained under [r1](r1/); startup route is being expanded into setup/puzzle gameplay. Session-local runtime is active; durable snapshot [setup](r1/setup.snapshot.json) preserves frame 223 with buttons released. Initial 180-frame traced request timed out at frame 67; subsequent batches record actual state. No structural claims inferred from timeout or mere pointer values.

Current phase has not finished; no pass checkpoint yet. Next: confirm setup options, enter gameplay, classify inspected observed/static code, header and CHR structural storage, and investigate clear non-code ranges. Gameplay research uses instruction booklet transcript https://www.world-of-nintendo.com/manuals/nes/anticipation.shtml (NES-AP-USA); sourced routes remain hypotheses until tested.

Intermediate checkpoint [r1-intermediate](r1-intermediate-verification.json) validates and rebuilds byte-identically. Internal classification now 35661 bytes, including 2871 observed code bytes, 16 header bytes, 6 vector bytes and 32768 CHR pattern bytes. Gameplay setup remains in progress; first option accepted; prompts have a multi-frame appearance delay.

Static connected graph review accepted 2185 additional code bytes (Static provenance); rejected only spurious candidate KIL at physical 0x2aa7, reached by impossible fallthrough from DEY/BNE loop followed by BEQ. Not a blanket rejection of surrounding routine. Evidence: r1/static-extension-review.json and r1/suspect-2a.json. Gameplay now reaches board level 1 and drawing puzzle (r1/puzzle-active.snapshot.json / .ppm). Artificial setup patch RAM $00AC=2 at frame968 changes the human count to two before supported game initialization (initial interpretation as computer count was corrected by inspecting $865F initialization); all subsequent gameplay explicitly artificial. Runtime slots remain [0,1,2,3]. Session-log.jsonl records exact inputs, budgets, failures and resulting frame state. Intermediate checkpoint remains r1-intermediate; newer internal map saved after static batch.

Progression correction: $AC is human count; $AD-1 is computer count. RAM $04CE onwards stores player kind (1 human, $80 computer) from inspected game initialization. Changed $04CF=$80 in a separate puzzle branch. Stage3 shortcut subsequently restored game-init snapshot and did not retain that second patch.

## Round 1 completed — incomplete ROM classification

ROM classification: 71.92% (47143 / 65552 bytes classified; 18409 bytes unknown).

| Physical section | Code | Data/image | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG 8 KiB bank 0 | 5158 | 1401 | 1633 |
| PRG 8 KiB bank 1 | 3071 | 922 | 4199 |
| PRG 8 KiB bank 2 | 3085 | 428 | 4679 |
| PRG 8 KiB bank 3 | 252 | 42 | 7898 |
| CHR 32 KiB | 0 | 32768 | 0 |

Code 11566, generic data 2809, CHR image 32768; all 5469 regions carry evidence. Provenance bytes: Observed 7643, Static 39500. Execution metric is separate: 5253 unique accepted instruction roots, 5818 contextual edges; trace aggregate_dropped=0. Detail drops intentional at capacity0. Physical instruction lengths determine classified bytes; instruction roots are not byte coverage.

[Final checkpoint](r1-disassembly-final-verification.json) validates and rebuilds byte-identically; [exact map](../maps/r1-disassembly-final.json), [generated assembly](../generated/r1-disassembly-final/game.asm). Current state analysis/state.json and maps/current.json match checkpoint. Baseline and intermediate checkpoints preserved. Input identity unchanged. [Metrics](r1/final-metrics.json), [unknown intervals](r1/handoff-unknown.json).

Gameplay checklist and reproducible snapshot routes: [scenarios](scenarios.json), [familiarization](gameplay.md). Level1 board/drawing, artificial level3 and level4 geometry/puzzle selection, letter-input experiment, and WINNER terminal scene visited. Later-board drawing and feature/drop-out mechanics deferred; level2 is not separately visited. RAM patches and CPU entry contracts in session-log.jsonl and companion scripts. All post-edit observations artificial. Source commentary/editing is the next phase; code labels are still intermediate and the generated source is not editorially complete.

Ready for commentary/redirect; not orchestration completion or terminal exhaustion. Leads R1-001..R1-004 retain concrete consumer/dispatch/boundary questions. Direct-JSR static closure and broad representative progression exhausted this pass's easy gains; large mixed data/script intervals remain for connected redirect research. No unsupported bytes were classified simply from non-execution or successful rebuilding.

No toolkit issues found; [persistent toolkit report](toolkit-issues.md). No toolkit edits, publication, archive, deletion or commit performed. Final disk check retains ample space; all cited evidence local.

Phase RPC process stopped cleanly after save/export; next worker should start a fresh configured server and load analysis/state.json. Final ledgers scenarios.json and leads.json saved; no active writer remains.

## Coordinator boundary — round 1 commentary active

Disassembly handoff accepted: [handoff](handoff-r1-disassembly.md), checkpoint r1-disassembly-final. Coordinator independently recomputed unique classified bytes, checked unchanged ROM hash and exact current/checkpoint map hash, inspected byte-identical report, scenarios and lead ledger. Required 71.92% classification report printed to user before commentary. Previous disassembly worker/RPC stopped. Separate round1_commentary worker started with only phase prompt and compact handoff; full current-source readability is pending. Previous goal turn made progress (phase evidence and provisional archive options); this continuation completed the disassembly boundary and launched the next phase.

## Round 1 commentary — current-source readability complete

All 11566 classified code bytes reviewed in 280 meaningful routine/continuation units, with consistent RAM/hardware/table/control symbols and purpose/algorithm notes. All classified data/image storage and 124 unknown intervals explained locally; 5593-record scaffold preserved. [Compact handoff](handoff-r1-commentary.md), [scope](commentary-r1/scope.json), [source reading copy](../commentary/reading-after.asm).

Fresh [verification](r1-commentary-final-verification.json) is byte-identical, source SHA256 6301fb28ce2860160827f6cbf86f9ae6a8da7cb820f7c075f01df65891710f8b. Reading copy refreshed after final edit. Input hash unchanged; internal state/current/checkpoint maps remain byte-equal. Classification unchanged: 47143 classified, 18409 unknown. No current-source readability gaps pending; final editorial audit remains reserved for terminal acceptance. Unresolved packed streams, frame callback origins, residual boundaries and board feature routes remain redirect leads; uncertain game meanings have useful mechanical descriptions. No toolkit issues found; no toolkit changes or live RPC.

## Coordinator boundary — round 1 redirect active

Commentary handoff accepted: [handoff](handoff-r1-commentary.md). Coordinator checked current source SHA against fresh byte-identical report, confirmed reading copy equals toolkit reading() rendering, and compared meaningful-unit physical byte union to all 11566 classified code bytes (no missing or extra bytes). Scope notes/source samples describe mechanical behavior and uncertainty; no current-source readability gaps reported. Final editorial audit remains pending. Separate round1_redirect worker started after commentary stopped, with phase prompt and compact handoffs. Classification remains 71.92% (47143/65552;18409 unknown).

## Round 1 redirect — active consumer investigation

Attempt [r1-consumers-20261106](redirect/r1-consumers-20261106/plan.md), input r1-disassembly-final/source hash6301fb28ce2860160827f6cbf86f9ae6a8da7cb820f7c075f01df65891710f8b. No classification changes (47143 classified18409 unknown). Full residual hex ledger and compact source consumers inspected. New explicit contract continuation method addresses conservative mapper1 static stops after indexed RAM writes/calls; first proposal walk569 instructions1177 bytes, zero overlap conflicts, KIL0x2AA7 excluded. Bank3 catalog link independently established:503 puzzle counts, partitions93/112/114 then184 PRG pointers, from0x610D through0x7F3D. Stream extent/answer validation and remaining callback/sound/feature avenues continue in this same phase. Frontier and reproducible scripts in attempt directory; no toolkit edit or publication.

Redirect frontier2: [PRG catalog results](redirect/r1-consumers-20261106/prg-puzzle-records.json) now validate184/184 exact endpoints across7765 bytes plus32 direction bytes. RAM-safe continuation proposal1624 bytes. [Runtime log](redirect/r1-consumers-20261106/runtime-log.jsonl) confirms artificial LEVEL2/YELLOW PUZZLE and drop-out LEVEL4->LEVEL3 via ordinary board-square consumer; selected snapshots/frames retained. Full traced drop transition and separate category4 feature experiment remain active, as do sound-handler/callback closures and exact residual dispositions. No map/source change or final phase stop.

Redirect frontier3: combined static proposals now2926 bytes from explicit returning/bank-preserving contracts and installed callbacks877D/880F/A7EE, plus independently bounded data. [Final method plan](redirect/r1-consumers-20261106/plan.md) records indirect RAM pointer bounds and sound18-handler graph. Full traced drop transition and separate category4 late drawing/answer entry retained in runtime2-log.jsonl. Packaging exact proposed-vs-accepted bytes and all residual dispositions; no classification/map/source mutation. Localized orphan fragments retained ambiguous, not automatically generic data.

## Round1 redirect complete — combined proposals ready for independent review

[Handoff](handoff-r1-redirect.md). Accepted checkpoint/state/source unchanged:47143 classified,18409 unknown; source commentary/source/game.asm hash6301fb28ce2860160827f6cbf86f9ae6a8da7cb820f7c075f01df65891710f8b. [Fresh integrity/partition checks](redirect/r1-consumers-20261106/handoff-integrity.json) confirm unchanged ROM/current source, identical state/current/frozen maps, exact disjoint coverage of old unknown complement by proposed code/data and ambiguous remainder.

Combined proposal2959 Static code bytes1387 instructions,15410 Static data bytes,18369 total; zero accepted bytes this phase. Conditional remainder40 if all accepted; current classification remains71.92%, not99.94%. R1-001/002/003 and newR1-010/011 ready. R1-004 gameplay verified artificially: level2, real-table-selected drop-out4->3, category4 late drawing and A-buzz answer entry.41 new real instruction-fetch roots available with artificial flag and effective banks[0,1,2,3]. Current readability scope remains complete for accepted source; next disassembly acceptance will create new commentary scope.

Toolkit TK-001 missing feature documented, no confirmed/suspected bugs or blocker; no toolkit edits. All RPC stopped. Attempt38MiB and8.7GiB free; no cleanup/deletion, publication, archive or commit. Next coordinator may start independent disassembly; worker did not launch it. No terminal exhaustion/completion claim.

## Coordinator boundary — round 2 disassembly active

Accepted combined [round1 redirect handoff](handoff-r1-redirect.md), not its classification proposals. Coordinator verified proposed18369-byte disjoint partition against accepted unknown complement; remaining40 bytes exactly match four recorded fragments. Connected/independent investigations and actual deferral reasons inspected in handoff. Separate round2_disassembly worker started after redirect stopped, to independently review structural claims and save a fresh checkpoint. Input remains r1-disassembly-final, current-source commentary preserved. Previous goal turn was a verified wait; this turn completed redirect boundary and advanced workflow.

## Round 2 disassembly — independent review frontier

Accepted state remains r1-disassembly-final while fresh physical decode and explicit returning-path checks run. Semantic review of connected callback/audio/drawing graph, all50 data slices and three orphan contracts is recorded in [review](r2-disassembly/review.md). Independently recomputing184 packed-puzzle endpoints; no unchanged gameplay replay. Current commentary source preserved. Proposed18369 bytes remain unaccepted until checks complete;40-byte residual is localized structural ambiguity. Disk8.7GiB free; no deletion or toolkit edits.

## Round 2 disassembly completed — incomplete ROM classification

ROM classification: 99.94% (65512 / 65552 bytes classified; 40 bytes unknown).

Independently accepted R1-001/002/003/010/011:2959 Static code bytes (1387 instructions) and15410 Static data bytes, connected batch18369. [Review](r2-disassembly/review.md) combines fresh physical instruction/flow checks,172 independently checked return routes and184/184 recomputed puzzle endpoints; [mechanical results](r2-disassembly/review-checks.json). Generic data uses consumer/neighbor and full-pattern inspection; semantic schema and natural reachability remain separate. Exact instruction inventory preserved in original proposal; new code grouped into contiguous regions. Data carries physical identity without execution views, which toolkit restricts to code. A first unsaved annotation attempt used views for Data, was rejected, and was rerun from unchanged saved state with code-only views; no toolkit defect.

| Physical section | Classified | Unknown |
|---|---:|---:|
| Header | 16 | 0 |
| PRG bank 0 | 8189 | 3 |
| PRG bank 1 | 8190 | 2 |
| PRG bank 2 | 8157 | 35 |
| PRG bank 3 | 8192 | 0 |
| CHR bank 0 | 8192 | 0 |
| CHR bank 1 | 8192 | 0 |
| CHR bank 2 | 8192 | 0 |
| CHR bank 3 | 8192 | 0 |

Exact residuals (half-open physical file offsets):0x1A4E..0x1A51 (3),0x2617..0x2618 (1),0x2707..0x2708 (1),0x5E61..0x5E84 (35). [Metrics](r2-disassembly/final-metrics.json), [competing interpretations](redirect/r1-consumers-20261106/ambiguous-fragments.json). Self-reentry, isolated opcode/instruction overlap and mixed self-locking control leave actual structural ambiguity; no rejected data merely for unknown meaning. No terminal exhaustion asserted.

[Fresh checkpoint](r2-disassembly-final-verification.json) validates and rebuilds byte-identically; saved state/current map agree. Input unchanged. Prior readable commentary/source/game.asm preserved at6301fb28ce2860160827f6cbf86f9ae6a8da7cb820f7c075f01df65891710f8b; new generated source lives separately under generated/r2-disassembly-final. Full current-source commentary now needs updating for newly accepted bytes. Representative progression in scenarios reused: all four levels, feature/drop-out, late drawing, answer entry and WINNER artificial scene; no unchanged replay. Natural winning route/full correct typed answer remain unresolved, not structural prerequisites.

Toolkit report: TK-001 conservative indexed RAM-store barrier remains missing feature; no new issues found. [Report](toolkit-issues.md). No toolkit modification, publication, archive, deletion or commit. One pass complete; next commentary then redirect for exact residuals and newly accepted source.

## Coordinator boundary — round 2 commentary active

Accepted [disassembly handoff](handoff-r2-disassembly.md). Recomputed unique classified bytes65512/65552 (99.94%),40 unknown; printed required report to user before commentary. Verified unchanged ROM identity and current map hash matches fresh byte-identical r2-disassembly-final report/checkpoint. Independent review artifacts cover instruction/return/domain/data evidence. Separate round2_commentary worker started after disassembly stopped, instructed to preserve entire prior commentary and carry valid editorial work by physical dependency. Full current-source readability pending.

## Coordinator boundary — round 2 redirect active

Accepted [commentary handoff](handoff-r2-commentary.md): inspected scope/examples, current source hash equals fresh byte-identical verification, reading copy equals toolkit rendering, all14525 classified code bytes have reviewed region coverage. Source readability complete; final editorial audit pending. Separate round2_redirect worker started after commentary stopped to investigate40-byte residual with accepted neighbors and evidence-policy rejection review if terminal. No whole-ROM completion claim.

## Round 2 redirect active — r2-local-contracts-20261106
Localized residual-contract review underway; no map/source changes. New method isolates closed PPUMASK toggle suffix from preceding self-locking counterexamples. Plan/frontier: redirect/r2-local-contracts-20261106/plan.md.

## Round 2 redirect — combined narrow proposals ready
Attempt [r2-local-contracts-20261106](redirect/r2-local-contracts-20261106/review.md) completed local semantic-contract review. R2-001 proposes1 Static-code byte0x2617 (same-system cursor advance); R2-002 proposes9 Static-code bytes0x5E7A..0x5E83 (closed PPUMASK toggle), separately from earlier lock and trailing RTS. No bytes accepted in redirect. Exact accepted complement remains40 bytes,65512/65552 classified. Prior whole-fragment locking counterexamples localized; reviewed competing interpretations and remaining boundaries. No changed live-caller prerequisite, map/source changes, runtime replay, toolkit edits or new issues. Source637ece766204ea0acf42e4909cc766e30f8f80b6e1093e4bee8e1dc10615fcb4 unchanged. Next independent disassembly review of combined10-byte proposal, then current-source commentary and redirect. Not terminal exhaustion; editorial audit pending.

## Coordinator boundary — round 3 disassembly active

Inspected combined [round2 redirect handoff](handoff-r2-redirect.md): two localized structural proposals totaling10 bytes, with primary decode/invariant and same-system idiom comparisons; actual remaining continuations/deferral reasons described. No terminal exhaustion claim. Separate round3_disassembly worker started after redirect stopped, to independently assess claims and fresh checkpoint. Accepted classification remains65512/65552,40 unknown; readable commentary preserved.

## Round 3 disassembly — independent local-contract review

Primary source neighbors, immutable identities, prior rejection scope and competing interpretations reviewed in [review](r3-disassembly/review.md). Both narrowed proposals supported:1 cursor INY byte and9-byte closed PPUMASK state machine, pending fresh mechanical checks and save. Connected paths add no unsupported extension; remaining30 bytes retain local structural ambiguity. Source637ece766204ea0acf42e4909cc766e30f8f80b6e1093e4bee8e1dc10615fcb4 untouched. Saved broad representative gameplay evidence reused. No toolkit changes/new issues.

## Round 3 disassembly completed — classification incomplete

ROM classification: 99.95% (65522 / 65552 bytes classified; 30 bytes unknown).

Accepted R2-0011 byte and R2-0029 bytes as Static code after [independent review](r3-disassembly/review.md), [fresh configured-release decode](r3-disassembly/decode.json) and [idiom/finite invariant checks](r3-disassembly/review-checks.json). The alternate cursor increment has independently accepted same-system INY/STY semantics and continuation; the PPUMASK suffix has an initialized, closed nonzero toggle invariant. Natural entry remains unresolved. Connected cursor successor paths already classified; toggle has no normal successor. No unsupported prefix/RTS extension. Prior rejection was narrowed to its actual conventional-loop claims. Lead statuses verified with history retained.

| Physical section | Classified | Unknown |
|---|---:|---:|
| Header | 16 | 0 |
| PRG bank 0 | 8189 | 3 |
| PRG bank 1 | 8191 | 1 |
| PRG bank 2 | 8166 | 26 |
| PRG bank 3 | 8192 | 0 |
| CHR bank 0 | 8192 | 0 |
| CHR bank 1 | 8192 | 0 |
| CHR bank 2 | 8192 | 0 |
| CHR bank 3 | 8192 | 0 |

Exact half-open physical residuals:0x1A4E..0x1A51 (CPU$9A3E,3),0x2707..0x2708 (CPU$A6F7,1),0x5E61..0x5E7A (CPU$DE51,25),0x5E83..0x5E84 (CPU$DE73,1), effective slots[0,1,2,3]. Double-write clobbered-coordinate contract, overlapping BPL boundary, earlier flag-lock prefix and isolated RTS retain structural ambiguity; no clear patterned generic data excluded for missing semantics. [Exact counts](r3-disassembly/final-metrics.json). No terminal exhaustion claim.

[Fresh verified checkpoint](r3-disassembly-final-verification.json) byte_identical=true; saved state/current/frozen map agree and all original regions preserved. Input unchanged. [Integrity](r3-disassembly/integrity.json) confirms preserved source SHA256637ece766204ea0acf42e4909cc766e30f8f80b6e1093e4bee8e1dc10615fcb4; generated/r3-disassembly-final is separate. Current readable source needs narrow10-byte update, especially former35-byte uncertainty and cursor preceding-byte prose. Prior source remains byte-identically verified on its previous map.

Representative saved gameplay checklist reused: all four levels, feature/drop-out, late drawing/answer entry, artificial WINNER visited. Natural typed correct answer/win unresolved; no unchanged replay warranted. Toolkit: no new issues found; existing [TK-001](toolkit-issues.md) unchanged. No toolkit edits, archive, deletion, cleanup, publication or commit. RPC stopped. One pass complete; next commentary then redirect always follow; worker did not launch next phase.

## Coordinator boundary — round 3 commentary active

Accepted [disassembly handoff](handoff-r3-disassembly.md), recomputed unique65522/65552 classified (99.95%),30 unknown and printed required report to user. Unchanged input and exact current/checkpoint map checked against fresh byte-identical report. Separate round3_commentary worker started after disassembly stopped, to preserve prior commentary and update new10-byte scope/dependencies while retaining valid full-source credit. Terminal assessment pending.

## Coordinator boundary — round 3 redirect active

Accepted [commentary handoff](handoff-r3-commentary.md). Independently checked source hash against fresh byte-identical report, reading rendering and reviewed coverage of all14535 code bytes. Complete current-source readability; final editorial acceptance pending. Separate round3_redirect worker started after commentary stopped, to investigate residual30 bytes and provide evidence-policy rejection review if proposing terminal incomplete.

## Coordinator boundary — round 4 disassembly active

Inspected [round3 redirect handoff](handoff-r3-redirect.md): localized corrected double-write inference produces3-byte ready R3-001, other residual continuations investigated with concrete objections. Separate round4_disassembly started after redirect stopped for independent full mechanical contract review and fresh checkpoint. Accepted65522/65552,30 unknown unchanged until acceptance. Not a terminal exhaustion handoff.

## Round 4 disassembly completed — classification incomplete

ROM classification: 99.96% (65525 / 65552 bytes classified; 27 bytes unknown).

R3-001 accepted3 bytes as Static code at physical0x1A4E..0x1A51/CPU$9A3E under effective8KiB slots[0,1,2,3]. [Independent review](r4-disassembly/review.md), [fresh decode](r4-disassembly/decode.json), [checks](r4-disassembly/review-checks.json). Complete accepted rectangle consumer resets literal source$A2FE/header131x132 each invocation; source ends$E68C,17292 writes and20724 dummy reads per pass, exitX=0 and byteA. First RTS resumes setup for second valid$00xx destination, second returns to original caller. Address latch and stack contracts hold; no same-coordinate, observed entry or natural purpose claim. Corrected redirect shorthand: accepted ClearBothNametables JSR targets$A3A7, returns$A3A2 then falls through to body; same-system idiom still supports structure. All connected consumers already classified. Lead verified; previous history retained.

| Physical section | Classified | Unknown |
|---|---:|---:|
| Header | 16 | 0 |
| PRG bank 0 | 8192 | 0 |
| PRG bank 1 | 8191 | 1 |
| PRG bank 2 | 8166 | 26 |
| PRG bank 3 | 8192 | 0 |
| CHR bank 0 | 8192 | 0 |
| CHR bank 1 | 8192 | 0 |
| CHR bank 2 | 8192 | 0 |
| CHR bank 3 | 8192 | 0 |

Remaining exact half-open physical ranges:0x2707..0x2708(1),0x5E61..0x5E7A(25),0x5E83..0x5E84(1), CPU$A6F7/$DE51/$DE73 under same slots. Overlapping branch/storage ambiguity, scratch-lock/signature-mark prefix lacking independent structural discrimination, isolated RTS respectively. No clear generic non-code pattern; no unsupported extension. [Metrics](r4-disassembly/final-metrics.json). No terminal exhaustion claim.

[Fresh checkpoint](r4-disassembly-final-verification.json) byte_identical=true; state/current/frozen map equal, prior regions preserved. Input unchanged and current source SHA256b8380ee7f0580fb5bd0967bf3686b0ee2aa435261780beea853ce00e6a913d76 preserved; [integrity](r4-disassembly/integrity.json). New generated source remains separate. Commentary requires narrow3-byte update and dependent prefix/setup uncertainty prose. Representative all-level/feature/drop-out/drawing/answer/artificial-WINNER scenarios reused; no unchanged replay. Natural typed win unresolved independently of structural claims. No new toolkit issues; existing [TK-001](toolkit-issues.md) unchanged. No toolkit edits/archive/cleanup/deletion/publication/commit. RPC stopped; one pass complete, no next phase launched.

## Coordinator boundary — round 4 commentary active

Accepted [disassembly handoff](handoff-r4-disassembly.md), recomputed65525/65552 classified99.96%,27 unknown; required report printed before commentary. Input unchanged, exact current/checkpoint map agrees with fresh byte-identical report. Separate round4_commentary started after disassembly stopped to preserve prior work and reconcile3-byte prefix plus dependencies. Terminal review still pending.

## Coordinator boundary — round 4 redirect active

Accepted [commentary handoff](handoff-r4-commentary.md): current source hash matches fresh byte-identical verification, reading copy equals toolkit rendering, all14538 code bytes reviewed. Full current-source readability retained with narrow changes; final editorial audit pending. Separate round4_redirect started after commentary stopped for27-byte residual investigation/rejection review under evidence-policy. Classification remains99.96%, no terminal decision accepted.

## Round 4 redirect — terminal incomplete recommendation

ROM classification remains **99.96% (65525/65552)**;27 unknown bytes. Current-source readability complete, source SHA256dfdf77b5953921c6d64a6330e7db53e40639388df0e8ac8f8ba75ce2ed5bef1a, fresh commentary verification byte_identical=true reused unchanged. No proposed/accepted new bytes, no ready leads; historical leads preserved and empty-result pass appended.

[Primary rejection review](redirect/r4-rejection-review-20261106/rejection-review.md) retrieves decisive R1/R2/R3 artifacts and current neighbors, localizes each counterexample and compares independent contracts/competing interpretations. [Exact recomputed complement](redirect/r4-rejection-review-20261106/unknown-complement.json):0x2707..0x2708/$A6F7(1),0x5E61..0x5E7A/$DE51(25),0x5E83..0x5E84/$DE73(1), slots[0,1,2,3]. DE51 ordinary buffer loop is false; conditional terminal/abandoned code remains possible. Split DE51/DE59/DE65 alternatives lack independent source/field/role support; accepted toggle invariant cannot establish arbitrary preparation. Singleton overlap/RTS also remain ambiguous, with no positive non-code pattern. No caller/natural route/return or useful purpose made universal prerequisite. Forced entries would repeat decode mechanics; no credible revised discriminator remains.

All-level/feature/drop-out/late-answer/artificial-WINNER checklist reviewed, unchanged evidence reused. Full typed natural victory unconfirmed, extra credits unavailable; no new representative shortcut identified. Coordinator must inspect review and primary artifacts before accepting terminal stopping; this is incomplete and final editorial audit remains pending. [Integrity](redirect/r4-rejection-review-20261106/integrity.json): immutable ROM/source/map/state/binary hashes, exact partition; no classification/source edits, RPC, toolkit edits, archive, cleanup/deletion, publication or commit. No new toolkit issues; existing nonblocking TK-001 in [report](toolkit-issues.md). [Compact handoff](handoff-r4-redirect.md).

## Coordinator terminal-research boundary

Inspected round4 rejection review and decisive primary artifacts; [coordinator reasoning](coordinator-rejection-review.md) accepts terminal incomplete research assessment with27 structurally ambiguous bytes. No corrected inference/test remains on unchanged premises. Final editorial acceptance, fresh verification and archive document reconciliation remain required; orchestration not yet complete.

## Terminal handoff — justified incomplete, orchestration acceptance

[Terminal handoff](terminal-handoff.md) records final checkpoint r4-disassembly-final and full preserved result. ROM classification: 99.96% (65525 / 65552 bytes classified; 27 bytes unknown). Coordinator primary rejection review accepted terminal incomplete; no ready lead or credible changed test remains on current premises. Separate final editorial worker passed targeted judgment and supported fresh audit; current-source readability and byte identity accepted, no source correction needed. Archive document-selection contract inspected and options reconciled without archive execution. Final current-state verification is coordinator-final-verification.json. ROM classification remains incomplete; no compatibility/natural winning-route claim. Prior goal turn made progress through editorial acceptance and final audit. No blocked/resumable work is mislabeled success; research may reopen from saved artifacts on changed evidence.
