# Round2 independent disassembly review (pass-003 input)

Reviewed evidence-policy/workflow, compact redirect handoff, primary graph7 construction,
contracts3 checker and saved proof domains, inline-contracts/data-inspection and raw
bank-organized instruction listings. Separate re-decode of every proposed Code view
uses the configured release API; review_apply.py records exact results and rejects
physical overlaps. Graph membership and legal decoding alone are not the role proof:
accepted predecessors, explicit finite RTS dispatch domains, direct calls and audited
normal/inline return contracts supply entry/continuation evidence.

Primary $8006 shift/CMP48 provides36 words; $9A3A masks7F/CMP1F provides31 including
raw80->0; $C50C CMP6 provides6. High-then-low PHA followed by RTS uses word+1.
$CEAF saved-stack initialization words end at independently established $CF04 code;
six task slots are not six table words. $C4C3/$F922 read SP+3/+4, consume caller's
return+1/+2 and add2 with carry to saved return. Queue $FA6E overflow removes its
own return frame, then RTS exits the queue helper; both exits preserve its caller's
already advanced return. $C4E9 internal inline word is not its own caller operand.
Conservative contracts reject unknown stack/mapper/indirect writes and yields;
$CEDF only returns under immediately preceding LDY2/3/4. No arbitrary yield-return
or FA81 bank-restoration propagation is accepted.

Localized alias check: bank12 $8156 is KIL within supported instruction operand,
therefore excluded, not made a root. Bank10 $8221 stores1 to0462 and confirmed boss
context [10,11,14,15] supports the conditional fixed D68F transfer to bank10 $8156.
That veneer transfers to905A, preserves X/Y in84/85, enables two zone rows and calls
9073 (FA13 with two-byte inline907B), then restores and returns. This is an explicit
boss precondition, not a claim that0462 globally determines every mapper state.
Other unreferenced veneer entries have no inferred entry contract.

Data inspection: duration nibble gives16 bytes; envelope low nibble/ASL gives32
pointer bytes, independently bounded amplitude payload361 bytes. All34 score gaps
show repeated00/FE score vocabulary between independently supported handler bounds;
generic Data is sufficient without asserting complete stream organization. Packed
bank10 prefix ends at accepted veneer; bank12 sprite lookup bases and repeated
triplets are bounded by RTS9941 and dispatcher9A3A. PPU FA04 exact immediate-Y copies
and FA13 nonzero encoded width*rows+header delimit26 headered and184 copied bytes.
Caller inline words have exact2-byte stack-consumption bounds. No blanket Data
inference from absence of execution. Isolated sound8035 remains Unknown.

Actual fetched subset101 instructions/203bytes from saved ending/Continue80 contexts
is kept Observed with artificial scenario attribution; every other new Code remains
Static. Previous natural and artificial scenarios are reused; no gameplay repetition.
Remaining bank/stack/pointer barriers are concrete redirect frontier, not terminal
exhaustion. Existing commentary preserved for next phase; no regeneration over it.

Local apply validation caught two additional actual fixed-bank views at physical
118096/118099 ($CD40/$CD43): Static proposal context[12,13,14,15], actual fetched
context[10,11,14,15]. Both map the same physical bank14 bytes. Resume preserves
both views after fresh bank-aware decode; this is a context union, not a banked
alias assumption. Partial progress was durably saved and resumed without replacing
prior classifications. Local script assertion/fix is not a toolkit defect.
