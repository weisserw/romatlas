# Pass-003 independent structural review

Input: pass-002, 393174 classified / 58 unknown. Exactly one disassembly pass.
Read evidence-policy.md, workflow.md, relevant ledger/history, scenarios and toolkit
report. Reproduced redirect-002/inspect.py against immutable ROM; output exactly
matches primary.json (cmp). Reviewed current-source neighbors at EBAB/EBAD,
FFD6 and F9F1/F9F8; no current-source changes.

Accepted 51 Static Data bytes, reachability unobserved:

- Nine prefixes at 139280,147472,155664,163856,172048,180240,188432,
  196624,221200, four bytes each. Complete bank-head comparison reproduces
  4C000000 before independently established payload boundaries. Functional native
  entry templates elsewhere encode supported nonzero destinations and following
  frames. Repeated zero-destination marker and independent +4 object boundaries
  support generic stored data. Legal JMP0000 alternative is acknowledged; no
  specific consumer, header schema, invalid-entry behavior or bank-wide non-code
  assertion is made. Bank20's later native procedures do not contradict this slice.
  Reuses prior bounded consumer/layout inspection; no further supported semantics.
- 193026:193040, fourteen bytes. Verified exact preceding seven numeric records,
  255-record boundary and trailing FF extent. Rejecting the 18-byte schema remains
  sound but is narrower than the generic Data claim. The zero-rich transition
  object's numerical character and established neighboring object boundaries
  support non-code; interior JMP0C04 has no independent native relation and the
  same opcode byte appears inside accepted numeric record249. No new stride,
  meaning or complete record schema is asserted.
- 256956:256957 (bank31 +BAC, alias EBAC). Accepted BRK at EBAB pushes EBAD;
  vector FFD6 immediately RTIs, and accepted EBAD begins JMP E46E. This directly
  establishes inline BRK signature/padding data. Value00 and hypothetical execution
  elsewhere are immaterial. No observed execution claimed.

Rejected proposed six Code bytes 260610:260616 (bank31 +19F2:+19F8,
F9F2:F9F8 under [null,null,null,31]); retain Unknown. Reviewed homogeneous
seven-RTS span F9F1:F9F8, accepted NmiNoOp at F9F1, preceding palette return
F9F0 and following address routine at F9F8 together. Those constraints establish
bounds and executable surroundings, but do not distinguish six native entries
from RTS-valued padding following the one hook. Supplying each hypothetical
entry a JSR frame merely restates RTS semantics. Repetition and native neighbors
do not independently identify a return family: no selector/entry organization or
other structural relation separates six routines from redundant return fill.
This is a sufficiency judgment about role, not a universal caller requirement;
a static entry organization could suffice without runtime. Generic Data is also
unsupported for this localized span. Preserve original proposal and its alternative;
no historical grouping claim is required, but executable structural role remains
ambiguous. No unchanged decoder, RAM-stack test or mapper5 retry can discriminate.

Retain detached RTS at246009 (bank30+E9/C0E9, context [null,null,30,31]):
preceding C0E8 return and following C0EA indirect jump do not distinguish an empty
procedure from redundant terminal/fill byte. Prior scoped rejection remains valid.

All accepted neighboring continuations already classified. Remaining exact seven
bytes have no supported connected gain. Progression checklist reviewed: later
scenarios, battle and ending unavailable through documented mapper5 rejection;
no supported state-edit workaround. Existing gameplay evidence is reused, not
repeated. This review accepts a narrower batch, leaves classification incomplete,
and hands seven-byte residuals to commentary/redirect. It does not assert terminal
exhaustion or compatibility. Toolkit NES-M5-001 unchanged; no toolkit edits.
