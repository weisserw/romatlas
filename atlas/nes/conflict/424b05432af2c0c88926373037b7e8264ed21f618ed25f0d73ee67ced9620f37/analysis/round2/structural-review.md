# Round2 independent structural review

Input pass-001g. This review distinguishes structural role, successful artificial
execution, and natural reachability. Fresh configured-binary decoding of all4531
proposed instructions matched opcode text/length; all execution views resolve to
exact physical bytes, with no instruction/data or accepted-map overlap. See
independent-decode.json and alignment-review.json. This is an alignment check, not
the sole code proof. Primary contracts below establish the connected structure.

F7EA saves A plus X/Y shadows and selects PRG4; F7F6 restores the saved selector
and registers. The exact three-JSR wrappers therefore execute middle calls under
[8,9,14,15], with final RTS under incoming view. No propagation of caller bank6
through bank4 callees was accepted. F2E1 prepares eight tile bytes with bank4's
92A2 dictionary, calls FB17 to restore bank6, and F322 consumes bank6's9D84
properties. Separate views/roots are necessary. FFD7 selects PRG2 before JSR8033.
The FFF* mapper serializers are not treated as unconditional bank-preserving
callees: setup controls1E/1F and field1C/1D/1E preserve mode3, CHR serializers do
not select PRG; FB09/FB17 require explicit changed/restored contexts.

Reviewed bounded indirect stores: setup20/21 points0234 and8bitY stays<=0333;
OAM28/29 points02xx and8bitY stays below0400; fieldAF/B0 is the64C1 attribute
shadow andY0; worldA5/A6 points6001+8n withY0..7; field62 points6666+33k,
k0..15 with8bitY remaining below6955. These are local preconditions, not arbitrary
RAM-pointer permissions. Production count<32 and8byte shifts stay cartridge RAM;
PHA/TYA preservation around display calls is balanced. The artificial production
route's observed INC6000 and exact old/new record assertions corroborate mechanics.

C749 indirect dispatch is code because its independently decoded producer and all
five handlers establish the control contract; exact universal nested pointer-table
partition remains unproved. Ending state05 domain0..5 and secondary34 domain0..6
are bounded by initializers and transition tests; target-minus-one versus absolute
pointer forms are kept distinct. Setup audio helper's40byte source is Data because
it is copied to6000 and executes there; no ROM observation is inferred from RAM
execution. Proper initializers/stack and explicit result/timer/page patches separate
saved observed ending fetches from natural campaign victory.

Bank5 stage directories and row windows independently meet each next directory
boundary; stage2 reuses a prior row and stage14 has overlapping stride5 windows.
Final stream has2*52 payload indices and next pattern directory bounds its end;
this does not assert exclusion of all later third-slab consumers.47sixbyte tile
patterns have independent directory stride/end constraints. Patterned tail uses
seven-value alphabet and repeated pairs: generic non-code, no fill schema.

Audio dispatch tables each have eight entries with producer masks. Period/duration/
track/envelope/request directories have consumer domains and adjacent endpoints.
185stream roots traverse the bounded audio grammar without bad pointers or budget
exhaustion; A8FD alone remains generic inter-stream data, not parsed token/reached
code. Envelope payload is value/duration data without inventing loop semantics.
UniformFF suffix is generic data. Proposal0x1000C's stale vector-mirror role is
REJECTED: immutable bank3 footer words areFFFF/C000/FFFF. Its four unknown bytes
are accepted only as generic non-code footer within the uniformFF/data terminus.
Bank5 C01B/C000/C01B words independently match that mirror claim. PPU packets use
consumer81D9 flag/count grammar, including count0=256, exact FF terminator and
separate OAM source. No fixed-width text claim is invented.

Footer0x1FFEF: fresh decode confirms RTS immediately afterFFD7's PRG2/JSR8033.
8033 jumps97A7; outcome paths end at reset, including endings' reset sinks. Thus
normal return fetch is unsupported. Adjacent backtick/ASCII interpretation also
fails to independently establish this leading byte as non-code: its only old proof
was membership in a printable run. Strong competing interpretations remain an
unreachable conventional RTS tail or accidental printable data. Withdraw only
this byte to Unknown; retain0x1FFF0..0x1FFFF printable spaces/CONFLICT data.
Neither legal decoding alone nor lack of execution settles it. No replay would
resolve intended structure without a different contract/reference.

Saved gameplay already covers natural setup/field and artificial later/final map,
combat/death/world return, actual password comparator, factory/pending production/
world birth, staff credits/END/restart. Reusing accepted saved fetch roots adds
real artificial-scenario observation without unchanged replay. No new natural
campaign, arbitrary AI-state sweep or every-credit-page claim. Ready connected
continuations were reviewed as a batch, not stopped at original lead ranges.

## Connected residual continuation review

After accepting the ready batch, fresh direct-edge inspection identified18edge
entries/12unique targets (edge-frontier.json), so the pass continued. Valid omitted
branch paths in bank4 and restored-context C446/C4AA wrapper returns were followed;
8078's independent safe-return summary and CF83's ten-byte zero-page copy supported
further continuations. No speculative A088 return at981D is claimed: the ending
controller is a nonreturn/reset sink. A8FD SEC/BCS makesA900 fallthrough infeasible.
Its actual A8E0 domain0..8 plusA8E5/A8E9 offset/sentinel consumer instead proves122
Data bytes to independentA97A code boundary. Directory/list overlap and shared
suffixes are intentional local facts; no disjoint nine-byte directory inferred.

New A5FD/A5B4 flow revealed the producer required for the indirect-store contract.
Existing A678 loader independently reads16pointers95F0, count and21byte rows into
6546. Immutable fixed-row-proof.json verifies159rows,954destination/shift triples,
all6112..642B and shifts0/2/4/6; maximum copied length253 fits8bitindex.32directory
bytes,3339rowbytes,16countbytes and16FFseparators exactly cover3403bytes endingA33B.
Every next pointer independently bounds its predecessor; the final next-directory
origin is independently consumed byAE71/AE76. Only flags are mutated in this
contract, preserving row pointerfields. ThusA5F0 stores cannot alter mapper state;
its six-field loop and caller stack continuation are supported under this explicit
loader-populated precondition. A5FD's unmatched RTS leaves two pushed bytes, so
this is not an unconditional balanced-return contract: matched-record paths return
balanced, and classification does not assert every artificial input is valid.

The nextA33B directory also yielded417Data bytes:16pointers and123counted x/y/value
triples, including count1 FF/FF/FF sentinel rows. All16endpoints meet next pointer;
finalA4DC is independently typed code. ConsumerAE61 explicitly loads4 beforeE06F,
so selecting4 preserves caller[8,9,14,15]; E072 restores previous4. This narrow
caller-specific argument resolvesAE66 continuation without grantingE06F a global
bank-preserving contract. Compare03B8&1F /03B9 then store03BC explains the triplet
role. See stage-triples-proof.json and exact aligned portion ofadjacent-decode.json;
initial deliberately unaligned bytes in itslinearpeek are not Code proposals.

Final connected follow-on:236instructions/501Codebytes,3942Data bytes; all Static.
Continuation traversal has no remaining localstops. Residual981D requires an
unsupported returning-ending contract;1FFEF remains materially ambiguous. No
other proposed direct edge exposes a productive unresolved bank-preserving path.
This is ordinary easy-gain exhaustion for this pass, not terminal research review.
