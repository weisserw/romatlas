# Round03 primary rejection review — proposed terminal INCOMPLETE

Checkpoint pass-003-disassembly-20261031-b. Fresh exact internal-map complement,
neighbor records and CPU views are in residual-ledger.json; immutable ROM windows
and CURRENT source excerpts are in primary-rom-windows.json / primary-source-excerpts.txt.
No map annotation is proposed. Each claim is evaluated independently, not as a four-byte object.

| Exact half-open physical / CPU / effective slots | Exact role claim and independent constraints combined | Decisive primary evidence and negative scope | Strongest competitor and inference/test disposition |
|---|---|---|---|
| C7A6..C7A7 /8796 /6,7,14,15 | Empty callable RTS, or redundant terminator of BrawlWalkIn. Independently bounded preceding input/state operation and following three addressed eight-byte arrays fix one-byte extent, but neither assigns its role. | Current game.asm:16416..16491: BCC8795 and RTS8795 finish the walk-in; the candidate8796 is not that branch destination. Prior primary small-residual-inspection.json offset51110 gives consumers8797/879F/87A7 and actual bytes. This excludes a needed walk-in return and an ordinary field of those eight-byte arrays, not all possible code/data roles. | A separate empty helper or redundant source-level RTS remains plausible; scalar/inter-record byte equally remains plausible. Reinterpreting it as a preceding array cell would require unsupported index/domain change, not an independent extent proof. No specific local operation or copied source template distinguishes it. |
|14379..1437A /8369 /10,11,14,15 | Empty helper or redundant RTS after road animation tail transfer. Neighboring selector/state operations and known queue-tail return semantics establish boundaries. | Current game.asm:39324..39429: earlier BCC uses DrivingReturn;8366 JMP DrivingQueueRoadTiles already delegates finalization/return; next836A computes(4E&1)+3 and also tail-jumps. No open stack obligation is closed by8369. This rules out necessary fallthrough completion, not a separately callable empty handler. | A leftover explicit RTS after source JMP versus scalar/padding. Matching tail-call idiom makes redundant RTS plausible, but cannot distinguish intentional code from stored byte; no independent repeated multi-operation template or local no-op-handler slot exists. |
|1ACD0..1ACD1 /ACC0 /12,13,14,15 | Empty helper/redundant packet terminator. Independent fixed-suffix consumer, PPU queue writes and next lives routine establish exact separation. | Current game.asm:52073..52117: suffix62,66,AE ends JMP FinishPpuPacket, whose existing stack contract already returns. ACC1 begins LDA7/JSR script. The candidate cannot repair a missing suffix packet return; no adjacent table stride consumes it. This is local contract evidence, not proof of no alternate entry. | Redundant RTS remaining after tail-call optimization versus stored constant/padding. The accepted suffix operation ends before this byte; adding the RTS contributes no suffix semantics and no distinct operation. No test can distinguish these hypotheses by artificially entering ACC0: both predict RTS. |
|1EE00..1EE01 /EDF0 /null,null,14,15 | Empty helper/second split return. Independently established PPU split completion and road-profile state machine give exact boundaries; fixed bank15 removes local bank uncertainty. | Current game.asm:66924..67018: PPUCTRL store then RTS EDEF; next EDF1 tests road state, with state2 RTS EDF9. Thus EDF0 supplies neither missing split return nor the road-state exit. Low-bank identities do not affect these fixed-slot bytes. | Empty callable stub or duplicate return versus scalar/padding. Two adjacent60 bytes alone do not establish a copied routine/template; their equality is the opcode observation again, not an independent constraint. Artificial return-frame entry or more decoding cannot decide. |

The stored-byte alternative is a concrete competing interpretation, not an asserted
Data role. No long uniform run/noninstruction pattern supports generic Data here;
one scalar can equal60. Conversely, legal one-byte decoding and an ordinary-return
precondition are consistent with every60 constant and do not independently identify
an intended routine. This does not require a live caller: a bounded dispatch no-op
slot, independent copied template, or coherent specific operation could suffice.
None occurs in the inspected primary constraints. Reusing their absence is scoped
to the supported graphs, not a universal assertion of deadness.

## Acceptance-standard comparison and revised-method challenge

Retrieved analysis/disassembly/round03-20261031-a/review.md and round02
combined-contract-review.md rather than inheriting their conclusions. Accepted
C1E2 combines independently meaningful delegated state3/substate0 writes, local
restart/completion semantic family, two accepted exact terminal tails1E32F/1EAF2,
and complete ordinary-return/bank/boundary contract. It does not require a caller.
The four RTS candidates share only return mechanics with adjacent code, not a
particular independently established state operation. Treating every complete
return opcode as an input-independent helper removes the discriminating evidence
that justified C1E2. Matching redundant-return source style is not a bounded copied
format and cannot establish intended instruction versus stored scalar.

Considered corrected alternatives: extend C7A7 arrays backward; regard post-JMP
bytes as shared epilogues; infer an empty-handler family from all four60s; infer
linker padding; execute with a synthetic frame. Respectively these lack a supported
consumer/domain, contradict actual tail/branch contracts for necessity, merely
repeat one opcode pattern across unrelated objects, lack a noncode pattern, or
predict identical results under both roles. No relaxed threshold or caller mandate
is substituted. No new discriminating local inference/test survives this review.
Future evidence may reopen an individual byte, but no specific current selector,
writer, spanning layout or gameplay condition provides a bounded recipe for it.
This is justified workflow stopping, not proof that classification is impossible.
