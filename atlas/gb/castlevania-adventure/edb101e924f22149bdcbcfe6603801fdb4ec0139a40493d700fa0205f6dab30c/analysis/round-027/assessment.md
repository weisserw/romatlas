# Round027 independent disassembly review

Accepted R26-MAINSTATE7-EMPTY-HANDLER: physical0728 length1 Static Code,
fixedROM bank0 CPU0728. Primary review is in primary-review.json. Current-map
region71 is established Static Data044f..0462 with explicit ten-handler role;
its word7 at045d..045e is2807. Independently decoded CALL01a2→0433 leaves01a5
on stack. At0443 the selector is loaded; CP03/JRNC044e means7 bypasses synthetic
PUSH0266 at044a..044d. RST00 at044e pushes044f; POPHL at0000 removes that word;
RST08 selects directory word7 and returns to JP(HL) at0002. Thus the only retained
continuation is01a5. C9 at0728 implements an empty handler with no extra precondition.
0727 terminates the preceding setter;0729 begins CALL07e6 of the separate completion
routine. Exact endpoint1 follows from RET. Table decoding past044f and subsequent
inline words in diagnostic decode output is not instruction evidence.

The alternative storedC9 with accidental/stale directory member is mechanically
possible, but accepted directory ownership, dispatcher semantics and stack contract
together support this handler beyond legal decoding/pointer resemblance. This
corrects the older blanket no-contract rationale. Natural selector7 and actual
fetch are unclaimed; provenance remains Static.

Connected state6/8/nested handlers are already classified. Reused sound Round026
redirect primary review of remaining2f09,3420,34af,bank2CPU6d6a: no changed premise
or productive revised test. No global type0 exclusion:3420 still has a candidate
word33ae, but independent directory ownership is unresolved. The other three have
terminal preceding paths and no encoded target words; word absence does not exclude
computed targets. A singleC9 supplies neither generic non-code pattern nor routine
entry contract. Preserve all four Unknown. See ../redirect/R26-20261027-dispatch-contracts-v1/assessment.md.

Broad initialized stages1..4, ending and visible credits from Round026 remain useful;
reviewed scenarios and gameplay records, with no new useful residual-linked route.
No startup replay or arbitrary artificial call repeated. Boss defeat/final held-A
phase/natural later-game reachability remain unclaimed. This is single-pass easy-gain
exhaustion, not a final editorial or archive acceptance decision.

Configured binary map annotation/save/export/validation and conflicts checks pass;
fresh round-027-disassembly checkpoint byte-identical65536bytes. Commentary source
preserved for separate phase. ExistingconfirmedT001 only;zero new toolkit issues,
no blocker. ROM,config,release and source hashes unchanged. Internal evidence local.
