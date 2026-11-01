# Localized remaining-byte review (not terminal acceptance)

Input map and counts in residual-ledger.json are freshly computed from the internal
map, not raw export gaps. Source/ROM primary windows are preserved beside this file.
The three-byte candidate is reopened separately in combined-contract-review.md.
This record reviews the four one-byte subclaims without inheriting a whole-gap veto.

| Exact physical interval, CPU/banks | Combined primary constraints and actual negative scope | Strongest competitor / disposition |
|---|---|---|
| C7A6..C7A7,8796,[6,7,14,15] | C793 walk-in forces rightward input, calls the active player helper, compares X to80, sets player state1 then returns at C7A5. Its BCC targets that previous return, not C7A6. Following C7A7..C7BF are three separately addressed eight-byte mask/motion arrays (8797,879F,87A7), beginning FF,20,00. No remaining field or stack operation needs the second60. | Empty callable RTS versus stored scalar/inter-record byte. A duplicate return is a possible source artifact but not an independently established copied template. Neither generic data nor Code gains discriminating support. Unknown. |
| 14379..1437A,8369,[10,11,14,15] | Previous routine ends at 14379 with JMP DrivingQueueRoadTiles; next entry836A independently forms `(4E&1)+3` and tail-jumps to the same queue helper. Earlier conditional exits use DrivingReturn, not this byte. The60 does not close an open instruction, conditional continuation or stack imbalance. | An unused explicit RTS after a tail call is plausible original source organization; an unused scalar/padding is equally viable. Neighboring road semantics give no role to an empty helper. Unknown. |
| 1ACD0..1ACD1,ACC0,[12,13,14,15] | Previous HUD fixed-suffix unit writes62/66/AE to the PPU queue then tail-jumps FinishPpuPacket; the next ACC1 independently queues a script and converts lives. The JMP supplies packet finalization/return; the60 supplies no missing return contract. No adjacent typed record requires this byte. | Redundant unreachable RTS source artifact versus constant/padding. Field/queue semantics support surrounding routines, not the isolated opcode. Unknown. |
| 1EE00..1EE01,EDF0,[null,null,14,15] | Sprite-zero split completes PPUCTRL write then RTS at1EDFF. EDF1 starts a separately supported road-profile script state test; its own state2 return is EDF9. No preceding branch requires EDF0, and no indexed/table extent covers it. Fixed bank15 is known; low bank uncertainty is irrelevant locally. | Empty helper/second return versus scalar/padding. Two equal bytes do not establish the multi-operation copied reset template accepted elsewhere. Unknown. |

Known caller/selector graph and byte-reference results are reused from round01
small-residual-inspection.json, final-audit.json, brawl/driving/fixed graphs and round02
independent review; absent edges are scoped to inspected routes. No universal
missing-caller premise is used: a nontrivial independent semantic/template contract
could suffice. Unlike C1E2, these bytes carry no particular delegated state effect
and lack contextual evidence for a distinct operation. The policy explicitly warns
that an unreferenced return opcode is insufficient alone. Four isolated60 bytes
at unrelated boundaries do not imply a common emitted-template format or generic
non-code run. No new consumer/extent/stack constraint emerged from direct context.

A broader opcode scan or artificial entry would repeat non-discriminating decoding.
Arbitrary gameplay permutations have no specific route to these entries. A future
supported empty-handler table selection, independently bounded copied template or
specific spanning branch would reopen only the affected byte. No presently concrete
experiment provides that missing role constraint. This is a localized disposition,
not terminal workflow exhaustion; the C1E2 ready proposal must receive review first.
