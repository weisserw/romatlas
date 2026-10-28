# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## Round 001 — disassembly complete; ROM incomplete

ROM classification: 90.50% (237228 / 262144 bytes classified; 24916 bytes unknown).

Input checkpoint: initialized workspace (0 classified); baseline established once at [baseline verification](baseline-verification.json). Output: [state.json](state.json), [portable map](../maps/current.json), [pass-001 map](../maps/pass-001.json), [source](../generated/pass-001/game.asm), [byte-identical verification](pass-001-verification.json). ROM SHA-256 remains 284cf8e186dcaa8174909c9fd4200f4e4dae325b2ad20e541814fc0355d5eee7.

| Bank | Code | Data | Classified | Unknown |
|---|---:|---:|---:|---:|
| 00 | 3364 | 8987 | 12351 | 4033 |
| 01 | 7184 | 16 | 7200 | 9184 |
| 02 | 4431 | 254 | 4685 | 11699 |
| 03 | 0 | 16384 | 16384 | 0 |
| 04 | 0 | 16384 | 16384 | 0 |
| 05 | 0 | 16384 | 16384 | 0 |
| 06 | 0 | 16384 | 16384 | 0 |
| 07 | 0 | 16384 | 16384 | 0 |
| 08 | 0 | 16384 | 16384 | 0 |
| 09 | 0 | 16384 | 16384 | 0 |
| 10 | 0 | 16384 | 16384 | 0 |
| 11 | 0 | 16384 | 16384 | 0 |
| 12 | 0 | 16384 | 16384 | 0 |
| 13 | 0 | 16384 | 16384 | 0 |
| 14 | 0 | 16384 | 16384 | 0 |
| 15 | 0 | 16384 | 16384 | 0 |

Annotation coverage: 237228 classified bytes have evidence and 237228 have comments. Editorial readability is pending commentary. Execution: 6904 unique accepted ROM opcode starts (14626 decoded bytes at those starts) in retained union; includes natural and artificial routes and is separate from classification. Static bytes are not claimed observed.

Natural Stage1 plus artificial Stage2/8/13, ending artwork, Quest puzzle and Endless field visited; exact patches/inputs/budgets and durable snapshots in [scenarios](scenarios.json). Broad progression checklist includes deferred Quest30/Quest ending/full credits and unavailable linked Versus. Runtime input schema failures and failed board/return-slot edits preserved with outcomes.

Evidence: [classification statistics](disassembly/round-001/classification.json), [exact unknown complement](disassembly/round-001/unknown-ranges.json), [code evidence](disassembly/round-001/all-code-evidence.json), [script inspection](disassembly/round-001/script-inspection.json), [asset containers](disassembly/round-001/asset-container-evidence.json), [sound/shared assets](disassembly/round-001/more-data-evidence.json), [RPC transcript](disassembly/round-001/rpc.jsonl). Data banks use generic aggregate structure, not invented exact original layouts.

Lead outcomes: R1-L001/002/003/006 verified; R1-L004 superseded (return slot overwritten); R1-L005 rejected only for its bounded single-cell experiment. R1-U001/002/003 concrete redirect avenues; R1-U004 commentary. No terminal exhaustion claim: easy startup, distinct gameplay and straightforward static continuation gains taken; deeper indirect dispatch/table contracts need redirect investigation. No supported proposal is withheld merely for lacking runtime use.

Toolkit: two confirmed documentation/API mismatches (TK-001 discovery budget, TK-002 input schema), zero suspected bugs, zero missing-feature findings; workarounds used, no blockers. See [toolkit report](toolkit-issues.md). Long-lived tool sessions received SIGTERM from unknown host cause; recovered via persisted maps/snapshots and short independent sessions, not attributed to toolkit. Toolkit and ROM unchanged.

## Rejection scope review (nonterminal handoff)

R1-L004 only falsifies using D486 as a persistent forced return because nested script calls overwrite it; full setup through C94E succeeds. R1-L005 only rules out prompt progression from CE14=1 over120frames; it does not reject consumer-backed board editing. Nonmonotonic asset bank8 offsets falsify strict monotone-directory model, not generic data or shared/reordered relative-offset consumers; bank8 independently accepted using bounded offsets, tile records and consumer contract (more-data-evidence.json). Unknown banks0–2 remain materially ambiguous code/data islands, not unknown merely for semantics. Concrete dispatch/record questions remain actionable for redirect.

## Redirect round001 attempt001

Combined proposal:17,961 Static code+6,952 Static data bytes, not yet classified. `redirect/round-001-attempt-001/handoff.md` and `proposal.json` are the independent disassembly input. All concrete connected/independent avenues were pursued through supported descendants; `methods.md` records the finite frontiers and closure. Initial missing caller for03B6 was an incomplete-root inference, corrected by accepted scene writer439D and typed stream1DA5. `rejection-review.md` localizes only43B2 two-byte ambiguity andBC36 single RET; no whole-gap rejection. Data semantics/genesis not required for generic classification. Actual map remains24,916 unknown until review.

Quest30/success route and later main-ending narrative visited artificially; tutorial was mistakenly hypothesized credits and corrected from visible text. Earlier counter trial with surviving old tasks was rejected only for that initialization state; clean-pool experiment confirmed the distinct tutorial system. No terminal Quest artwork/staff-roll claim. Scenarios preserve exact recipes and negative results. No new toolkit findings.

## Round 002 — disassembly complete; ROM incomplete

ROM classification: 100.00% (262141 / 262144 bytes classified; 3 bytes unknown).

Input pass-001:237,228 classified/24,916 unknown. R1-R001/R1-R002 independently verified and accepted as a connected batch:17,961 Code+6,952 Data=24,913 added bytes, all Static. 199 exact code intervals (7,988 instructions) redecoded identically using the configured binary in asserted CPU/bank context [0,1,2]; all instructions have recorded root/edge closure. Reviewed typed scene writers/commands, allocator task contracts, word dispatch consumers, bank restoration, independent routine contracts and exact boundaries. All49 word-array manifests match ROM; all98 data intervals inspected in full with references, neighbors and pattern/consumer rationale. No proposed code/data bytes rejected; $92D0 five-byte ordered constants kept generic Data without the unsupported countdown-purpose inference. Earlier $03B6 reset-stub rejection was correctly narrowed by scene writer439D→cursor1D56→typed command1DA5.

| Bank | Code | Data | Classified | Unknown |
|---|---:|---:|---:|---:|
| 00 | 5280 | 11104 | 16384 | 0 |
| 01 | 13814 | 2568 | 16382 | 2 |
| 02 | 13846 | 2537 | 16383 | 1 |
| 03 | 0 | 16384 | 16384 | 0 |
| 04 | 0 | 16384 | 16384 | 0 |
| 05 | 0 | 16384 | 16384 | 0 |
| 06 | 0 | 16384 | 16384 | 0 |
| 07 | 0 | 16384 | 16384 | 0 |
| 08 | 0 | 16384 | 16384 | 0 |
| 09 | 0 | 16384 | 16384 | 0 |
| 10 | 0 | 16384 | 16384 | 0 |
| 11 | 0 | 16384 | 16384 | 0 |
| 12 | 0 | 16384 | 16384 | 0 |
| 13 | 0 | 16384 | 16384 | 0 |
| 14 | 0 | 16384 | 16384 | 0 |
| 15 | 0 | 16384 | 16384 | 0 |

Exact residual: physical/CPU $43B2..$43B3 (bank1,00DC) and $BC36 (bank2,C9),3 bytes total. Neighboring serial/NMI and coordinate-helper code accepted independently. Read [decisive primary evidence](redirect/round-001-attempt-001/rejection-primary.json) and [localized rejection review](redirect/round-001-attempt-001/rejection-review.md): no independent word consumer or enclosing format for00DC; no callable contract for isolated RET. Competing code/data interpretations remain materially ambiguous. Corrected reset-stub premise does not generalize to these isolated bytes. Reopen only with independent consumer/entry/enclosing structure or a justified corrected inference; no unchanged replay or arbitrary state sweep warranted. This pass does not make a terminal workflow stopping decision.

Annotation coverage:262141 bytes with evidence and comments. Total code32,940 bytes; data229,201 bytes. Runtime evidence reused from prior progression and redirect; zero new execution/replays. Prior disassembly observed union6,904 opcode starts/14,626 decoded bytes remains separate (redirect observations not re-counted). Broad gameplay and ending evidence are in [scenarios](scenarios.json), including artificial Quest30 and continued main-ending narrative; no staff roll or stable Quest terminal artwork newly claimed.

Saved [internal state](state.json), [portable map](../maps/current.json), fresh [pass-002 map](../maps/pass-002.json), [generated source](../generated/pass-002/game.asm) and [verified checkpoint](pass-002-verification.json):ok=true,byte_identical=true. [Internal validation](disassembly/round-002/validation.json) ok. [Independent review](disassembly/round-002/independent-review.json), [data inspection](disassembly/round-002/data-inspection.json), [statistics](disassembly/round-002/classification.json), [unknown complement](disassembly/round-002/unknown-ranges.json). ROM identity unchanged; prior annotations, maps, source, commentary and investigation history preserved. Newly accepted source requires commentary; prior editorial verification covers pass-001 only.

Toolkit:no new toolkit issues found. Existing [report](toolkit-issues.md) retains2 confirmed documentation/API mismatches TK001/TK002,0 suspected bugs,0 missing features,0 blockers. No toolkit edits. Storage:bounded199 decode requests,49 table checks,297 annotations; concise digest review log and bounded annotation transcript, no captures/runtime traces;9.4GiB free.

## Round 002 — redirect terminal incomplete

Current pass-002 remains262,141 classified/3 unknown; zero proposals or map/source changes. [Handoff](redirect/round-002-attempt-001/handoff.md), [current complement](redirect/round-002-attempt-001/unknown-ledger.json), [compact rejection review](redirect/round-002-attempt-001/rejection-review.md), [primary slices/hashes](redirect/round-002-attempt-001/primary.json). Reused prior sound review, assessed new current commentary and localized NOP/word/RET alternatives. Serial ring-base resemblance and isolated RET remain code/data ambiguous; natural entry/original layout not required. New score/scroll/palette corrections add no discriminating residual edge/format. No useful supported continuation or changed test remains; no automatic new cycle. Exact residual43B2..43B3 bank1 andBC36 bank2.

Current-source readability complete; existing fresh commentary verification byte-identical; final editorial/archive acceptance pending separately. Broad progression/terminal narrative and Quest30 attempts reviewed without unchanged replay. No new toolkit issues found; existing TK001/TK002 two confirmed mismatches, no blockers. ROM/map/state/source/toolkit unchanged; bounded research records only,9.4GiB free.
