# Round027 redirect: inline origin supports a narrower empty-handler claim

Checkpoint:round-027-disassembly/current-source commentary complete. State SHA256
acc2cc210d08dc0bfa55a2fa9a2660a06de3647de5f55c12100d91d753e91a82.
65532/65536 classified; exact Unknown physical2f09,3420,34af,ad6a(bank2CPU6d6a).
Proposed1byte;accepted0. No classification/commentary/final acceptance performed.
Primary [audit](primary-audit.json), [physical source excerpts](source-excerpts.txt),
[changed-method plan](plan.md). ROM/state/map/source/toolkit/runtime unchanged.

## Corrected inference:3420 need not inherit the generic label's uncertainty

The R26/R27 assessments correctly note that33ae is genericData rather than a declared
whole directory, but use that annotation as if it denied independent consumer-origin
evidence. The accepted instructions provide that evidence directly:33a8 loads actor
field1,33aa calls ReadCurrentActorField,33ad executes RST00. RST00's POPHL consumes
its own return33ae as the table base. Thus33ae is independently the *origin* of this
word-selection consumer, regardless of any unsupported right endpoint. For supported
precondition current occupied actor.type=0, RST08 doubles0 and reads exactly33ae..33af,
which encode3420. This is a zero-offset structural relationship, not a plausible word
found by searching arbitrary bytes. It does not require proving the whole table's
extent, natural production of type0, or all possible runtime types.

UpdateCurrentActorAndCull/DispatchAndMoveCurrentActor supplies an ordinary CALL to
DispatchObjectRuntimeType. ReadCurrentActorField returns before RST00. RST00 removes
only its inline-pointer return; RST08 leaves that outer CALL return intact. JP(HL)
therefore enters3420 with a complete ordinary return contract. C9 implements the empty
handler, resuming the axis-integration code after CALL. Target3420 is fixedROM under
any switch bank. Previous word341e..341f is the accepted type56 wrapper pointer;3421
begins a separate callable bank wrapper. RET supplies the exact one-byte endpoint.
Occupancy at field0 and runtime type at field1 are separate; an occupied type0
precondition is neither demonstrated natural nor contradicted by the occupancy gate.
No artificial execution was performed or needed for this static claim.

Strongest alternative: the dispatcher is only intended for nonzero types, and its
zero-offset word/stored C9 is reserved coincidental storage. This is mechanically
possible, as a stale word is possible for accepted0728. But inline-origin ownership,
matching fixed-ROM word, inherited CALL stack and exact no-op endpoint jointly favor
an empty type0 slot. The earlier assertion that pointer and opcode alone supply all
support omits the independently established consumer origin. This is narrower than
claiming the entire57-word directory: no disputed intervening cell or outer endpoint
is needed. Return a ready one-byte StaticCode proposal for independent disassembly
review; preserve GenericData annotation until that review separately decides whether
to document word0 as a dispatch entry. No additional classified bytes are proposed.

## Terminal-tail epilogue comparison: three other bytes remain ambiguous

Physical2f09 follows JP3199;34af follows unconditional **JR**, not JP, back to34aa;
ad6a(bank2CPU6d6a) follows JP6c10. The target routines return directly to the original
caller's stack; none returns to the physical byte after the tail. All three have no
ordinary encoded target word anywhere in ROM (R26 raw audit), which excludes only
that pointer mechanism, not computed/RAM targets or any code role.

Compared every commented source unconditional JP/JR immediately followed by an
accepted RET, ignoring labels/comments between them. Exactly three analogous sites:
* physical30d6(bank0): ReturnFromPersistentPlacementScan follows JR back to the scan;
  independent JR NC to30d6 selects its return branch. Code provenance Observed.
* physical5934(bank1CPU5934): HoldTurningSoundActorFinalState follows JP of the prior
  handler; independently consumed state3 word at58b7 points5934. Code Observed.
* physical856b(bank2CPU456b): ReturnFromAudioPitchSequence follows the parser's back
  JR; independent JR Z selects it on terminatorFF. Code Observed.
The exact map records/provenance and source edges are preserved in the primary audit.
These examples establish adjacent terminal-tail/RET layout, but their RETs are separate
branch destinations/handlers. They supply no example of an accepted *detached unreachable
epilogue* justified by a repeated generation contract. Repetition of C9 after a tail
is the same byte/layout observation, not independent evidence of an original compiler
or assembler convention. The prior FI025 rejection did not test this specific template
comparison; this review does, and finds no supporting generation evidence.

Strongest alternatives for each are redundant unreachable routine epilogue versus stored
C9 between routines. Neighboring control/stack contracts do not distinguish them.
Classifying as StaticCode would require a supported layout/generation/entry relationship,
not necessarily a caller. GenericData is also unsupported: a lone C9 is not a non-code
pattern. Keep these three Unknown; no dead-code or natural-reachability claim.

## Connected and independent follow-through

Inspected dispatcher caller, occupancy/type distinction, zero-index consumer, selected
handler endpoint, continuation axis integration and neighboring bank wrappers together.
All connected continuations are already classified. The remaining generic runtime words
need no whole-directory reinterpretation to support the narrow first entry, and do not
provide pointers to the three detached returns. Ending state9 explicitly targets6d6b,
not6d6a; its retained credit-index/render-return contracts do not reopen that byte.
Current source's remaining unresolved questions/generic labels have no other Unknown
intervals to classify; their previously inspected consumer/format uncertainties do not
create a new residual-linked experiment. No unchanged arithmetic/table/audio/graphics
attempt or arbitrary gameplay state sweep was repeated.

Broad stage1..4 and ending/visible STAFF credits evidence remains in scenarios.json.
Boss arena and final held-A phase remain unvisited; neither has a newly identified
connection to the three isolated bytes. This is a ready proposal handoff, not terminal
gameplay exhaustion, and no stronger gameplay or fidelity claim follows from it.
A terminal incomplete stopping decision is not requested while this ready lead remains.
For later residual review, the compact three-byte rejection above records exact claim,
primary evidence, negative-result scope, combined constraints and strongest alternatives.
No discriminating revised test currently survives for those three; reopen on independent
layout contract or computed entry evidence, rather than another artificial RET call.

Existing confirmedT001 only;zero new toolkit issues,no blocker. Disk14GiB free;32KiB
research evidence, no bulk scratch. Internal evidence remains local.
