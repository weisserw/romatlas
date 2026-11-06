# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## Round 1 — disassembly

Initial baseline and reset/NMI/IRQ static discovery followed by traced startup, setup, board, drawing and artificial later-board/ending experiments. Evidence is local under [r1](r1/), exact later commands in session-log.jsonl, persistent snapshots accompany cited frames. Initial state had no leads to verify. Direct-call closure was iterated until no meaningful new candidates remained; final residual KIL at physical0x2aa7 is a false flow continuation after DEY/BNE then necessarily taken BEQ, excluded rather than classified code. Other nearby routine bytes independently accepted.

Observed instructions are actual accepted fetch roots with explicit CPU alias and effective PRG slots. Static connected paths use the same context, with unresolved indirect/mapper effects retained. Data and CHR classifications use independent physical boundaries, pattern evidence and available consumers. Header/raw export fidelity does not classify gaps.

Stage shortcuts: inspected game initialization and promotion routines are saved in game-initialization.json, level-change-code.json, promotion-code.json, completion-transition.json. $AC=2 means two human slots, corrected after initialization inspection; $04CF=$80 enables AI for slot1 in one branch only. Later stage branches restore game-init snapshot, patch $0403/$0409 (position high nibble), $88, $8F/$90, $A3 and enter $86D4 with stackFF. Stage4 additionally sets $B7=1. Stage3/4 frames confirm levels and distinct geometry; blue puzzle selection executes, but neither later drawing nor feature/drop-out mechanics was fully exercised. Ending branch uses inspected $B326 terminal-check block with $A1=1,$B7=0,$03=0; WINNER frame confirms terminal scene. All are artificial-state observations, with no natural reachability claim.

Bounded static/data review: screen/tile records physical0x5c63–0x5e0f have PPU coordinate prefixes and high-bit terminated menu/answer text; following code independently bounded. Uniform FF0x7f6c–0x7f90 accepted as generic data, no asserted fill purpose. Full raw review [pattern-data-review.txt](r1/pattern-data-review.txt) supports generic non-code0x1a97–0x23aa: repeated indices, zero-margin tile rows, ramping tile codes and short command records; start referenced by indexed LDA at0x2c13/0x2ccf, subrange0x230e by PPU writer0x34a. No complete table schema inferred. This review was added during consolidation because generic classification does not require decoding every tile-record format.

One helper's partial map.get page caused overlap rejections; no accepted classifications were replaced. Corrected helper reads complete saved internal map. Intentionally zero detail trace capacity retained bounded aggregates without aggregate drops. No toolkit issues found; see [report](toolkit-issues.md).

Pass stopping is ordinary easy-gain handoff, not terminal exhaustion: remaining large mixed stream/routine intervals need consumer and boundary research; unexplored dispatch/state and graphics/script paths remain concrete redirect work. No exhaustive method rejection or whole-ROM completion claimed.

### Final round1 metrics

ROM classification: 71.92% (47143 / 65552 bytes classified; 18409 bytes unknown).

| Physical section | Code | Data/image | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG 8 KiB bank 0 | 5158 | 1401 | 1633 |
| PRG 8 KiB bank 1 | 3071 | 922 | 4199 |
| PRG 8 KiB bank 2 | 3085 | 428 | 4679 |
| PRG 8 KiB bank 3 | 252 | 42 | 7898 |
| CHR 32 KiB | 0 | 32768 | 0 |

Final verification: [r1-disassembly-final](r1-disassembly-final-verification.json), byte_identical=true, current map valid. Detailed metrics and exact residual complement linked from progress.

## Round1 redirect combined consumer/bank-contract investigation

Input r1-disassembly-final + completed current-source commentary, source hash6301fb28ce2860160827f6cbf86f9ae6a8da7cb820f7c075f01df65891710f8b. Unique attempt [r1-consumers-20261106](redirect/r1-consumers-20261106/plan.md), [primary interpretation](redirect/r1-consumers-20261106/consumer-review.md), [proposal](redirect/r1-consumers-20261106/proposal.json).

Changed method: rooting every known instruction reproduces only excluded KIL because mapper1 propagation stops at every indexed store/call. Exact RAM address-domain bounds plus independently recorded return/PRG-preserving paths allow continuation. Three missing installed callbacks and18 sound handlers followed as a batch; all independent clear data opportunities inspected too. Complete catalog/consumer model proves184 PRG puzzle stream endpoints; no inferred original generator. Orphan procedure contracts evaluated independently of actual callers; uncertainty localized to40 bytes instead of spreading missing callers across surrounding data.

New representative artificial runtime: separate initialized level2 board/drawing selection, stage4 real-table-selected drop-out tolevel3, category4 late drawing and A-buzz answer scene. Exact patches, buttons/budgets, restored snapshots, trace contexts and selected frames preserved; naturally winning/typing an entire correct answer not established. Existing winner milestone reused.

Outcomes: R1-001/002/003 ready, R1-004 verified, R1-005 code rejection retained with new bounded data interpretation underR1-010, newR1-010/011 ready.2959 code +15410 data =18369 proposed bytes; zero accepted by redirect.124 original unknown intervals receive exact evidence-linked dispositions. Four small ambiguous fragments retain specific code/data/control questions, not unknown merely due absent execution. This is a productive independent-review handoff, not a terminal assessment. No active RPC.

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

## Round 2 redirect — combined narrow proposals ready
Attempt [r2-local-contracts-20261106](redirect/r2-local-contracts-20261106/review.md) completed local semantic-contract review. R2-001 proposes1 Static-code byte0x2617 (same-system cursor advance); R2-002 proposes9 Static-code bytes0x5E7A..0x5E83 (closed PPUMASK toggle), separately from earlier lock and trailing RTS. No bytes accepted in redirect. Exact accepted complement remains40 bytes,65512/65552 classified. Prior whole-fragment locking counterexamples localized; reviewed competing interpretations and remaining boundaries. No changed live-caller prerequisite, map/source changes, runtime replay, toolkit edits or new issues. Source637ece766204ea0acf42e4909cc766e30f8f80b6e1093e4bee8e1dc10615fcb4 unchanged. Next independent disassembly review of combined10-byte proposal, then current-source commentary and redirect. Not terminal exhaustion; editorial audit pending.

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

## Round 4 redirect — terminal incomplete recommendation

ROM classification remains **99.96% (65525/65552)**;27 unknown bytes. Current-source readability complete, source SHA256dfdf77b5953921c6d64a6330e7db53e40639388df0e8ac8f8ba75ce2ed5bef1a, fresh commentary verification byte_identical=true reused unchanged. No proposed/accepted new bytes, no ready leads; historical leads preserved and empty-result pass appended.

[Primary rejection review](redirect/r4-rejection-review-20261106/rejection-review.md) retrieves decisive R1/R2/R3 artifacts and current neighbors, localizes each counterexample and compares independent contracts/competing interpretations. [Exact recomputed complement](redirect/r4-rejection-review-20261106/unknown-complement.json):0x2707..0x2708/$A6F7(1),0x5E61..0x5E7A/$DE51(25),0x5E83..0x5E84/$DE73(1), slots[0,1,2,3]. DE51 ordinary buffer loop is false; conditional terminal/abandoned code remains possible. Split DE51/DE59/DE65 alternatives lack independent source/field/role support; accepted toggle invariant cannot establish arbitrary preparation. Singleton overlap/RTS also remain ambiguous, with no positive non-code pattern. No caller/natural route/return or useful purpose made universal prerequisite. Forced entries would repeat decode mechanics; no credible revised discriminator remains.

All-level/feature/drop-out/late-answer/artificial-WINNER checklist reviewed, unchanged evidence reused. Full typed natural victory unconfirmed, extra credits unavailable; no new representative shortcut identified. Coordinator must inspect review and primary artifacts before accepting terminal stopping; this is incomplete and final editorial audit remains pending. [Integrity](redirect/r4-rejection-review-20261106/integrity.json): immutable ROM/source/map/state/binary hashes, exact partition; no classification/source edits, RPC, toolkit edits, archive, cleanup/deletion, publication or commit. No new toolkit issues; existing nonblocking TK-001 in [report](toolkit-issues.md). [Compact handoff](handoff-r4-redirect.md).
