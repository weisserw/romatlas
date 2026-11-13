# Redirect-003 compact rejection and terminal-incomplete review

Input pass-003; current-source readability complete per
[commentary handoff](../../handoffs/commentary-pass-003.md). Current accepted
393225/393232; exact complement 7 bytes. No proposal or accepted change this pass.
Reproduced complement directly from maps/current.json, not exporter raw gaps.
[primary.json](primary.json), produced by [inspect.py](inspect.py), contains exact
bytes, adjacent map regions/views, literal origins, original runtime rejection and
ROM/map/state/source identities. No map, state, source, ROM or toolkit mutations.
Disk 6.5 GiB free; analysis/scratch 4 KiB; no cleanup/deletion.

This review retrieved [pass-003 independent review](../../disassembly/pass-003/review.md)
and [redirect-002 primary](../redirect-002/primary.json) and proposal/review,
then inspected the current source excerpts below. Rejections are reassessed as
sufficiency judgments, not inherited vetoes. Historical proposals remain preserved.

## Bank31 six-byte claim, physical 260610:260616

CPU F9F2:F9F8, bank31 +19F2:+19F8, effective slots [null,null,null,31].
Claim actually needed: six singleton native no-op Code entries; alternative claim:
unlabeled generic Data. Neither claims natural selection or original compiler layout.
Decisive primary: [bank31-neighbors.txt](bank31-neighbors.txt),
[nmi-caller.txt](nmi-caller.txt), primary.json intervals[1]. Exact six bytes are
60 60 60 60 60 60. UploadPalette ends with RTS F9F0; supported NmiNoOp is RTS
F9F1 and has direct JSR in Nmi; independent address routine begins F9F8.

Independent constraints combined: accepted surrounding routines and NMI hook,
exact homogeneous seven-return span F9F1:F9F8, complete RTS stack/register contract,
and no intervening inline metadata/operand boundary. These establish exact physical
bounds and make six no-op entries plausible. However, the sole supported hook
returns immediately at F9F1; its control flow does not continue into the suffix.
Consequently adjacent hook identity does not assign executable role to any suffix
byte. JSR frames supplied at F9F2..F9F7 would each return correctly for both a set
of no-op entries and padding deliberately filled with RTS. Uniformity and six
identical decodes are one shared byte-pattern fact, not six independent contracts.
The strongest Code alternative remains a genuine bounded no-op family; lack of a
known caller is not its falsification, and no caller requirement is imposed.

For generic Data, repetition is admissible pattern evidence without a consumer,
schema or recovered meaning. Here the repeated value is itself the exact native
empty-procedure opcode, immediately beside the demonstrably used same-byte hook.
The finite run ending at F9F8 supports an extent but does not distinguish reserved
native entry slots from stored padding. This concrete competing native role makes
the code-versus-data uncertainty material. We do not require a fill generator,
semantic purpose, complete selector domain or runtime to accept generic Data.
Unlike the accepted zero-destination bank prefixes, these bytes do not break an
independently observed native entry-template relation: they exactly satisfy the
nearby one-byte native entry contract. Unlike accepted BRK signature EBAC, no
architectural skip identifies a mandatory inline data byte. Prior large uniform
FF/00 data runs do not transfer a non-code conclusion to this native return span.

Literal-origin result: only F9F5 occurs (physical257123), inside the accepted
low-dispatch array rather than a native target word. F9F1 occurs at physical259715,
the supported NMI JSR. This rules out that particular literal-word proof route;
it does not rule out computed/split targets or prove unused/data. No selected
index, table consumer or writer tied to these six suffix addresses was identified
in the existing VM/native graph or relevant source. A supported static entry-slot
organization would suffice without a live caller, but none is presently evidenced.
No useful changed-inference test survives: synthetic RTS tests and decoding cannot
discriminate the alternatives, and repeating literal search would be unchanged.
Retain these six bytes Unknown; no split subinterval has stronger evidence.

## Bank30 singleton claim, physical246009:246010

CPU C0E9, bank30+E9, slots [null,null,30,31], byte60.
Decisive primary: [bank30-neighbors.txt](bank30-neighbors.txt), primary.json
intervals[0]. FindTenByteKeyRecord's successful copy explicitly jumps to C0E8;
failed-search zeroing loop also ends at that RTS. C0EA independently starts
JMP(000C), the native secondary-address tail-call helper.

Combined bounds, exact return opcode, preceding explicit terminal and following
indirect-call contract support a possible empty native procedure, but neither
neighbor transfers control to C0E9. Strongest competing interpretation is one
redundant return/padding byte. Unlike a reachable fallthrough instruction or
BRK signature, no adjacent architectural contract assigns this byte a role.
A hypothetical standard return frame is sufficient to execute it but does not
identify stored Code. Conversely its short repetition with C0E8 cannot establish
non-code while the same empty-procedure alternative remains concrete.

Literal C0E9 words at26889/77590/77606 are in already reviewed VM Data and do not
supply a native origin contract; their existence is not proof of Code, and lack
of a native caller is not proof of Data. This scoped result leaves computed use
unresolved. No independent format/consumer/entry organization presently selects
either interpretation. Retain one byte Unknown. This local result is independent
of the longer bank31 run; failure of one does not veto the other.

## Other identified avenues and useful-continuation triage

Reviewed commentary/uncertainties.md and the L001..L008 frontier against relevant
source and accepted review records. Earlier active entries in historical
investigation text are superseded by their retained verified attempts in leads.json.
No unfinished productive redirect frontier is being relabeled terminal.

- Dispatch/callee/native-escape and audio paths: L001..L005 complete structural
  batches already accepted. VM-byte dispatch and 47 extended target words have
  their own established bounds; numeric adjacency at257123 is not a new target.
  Dynamic bank mapping and natural use remain preconditions/observation questions,
  not unknown structure outside the exact complement. No new unsupported address
  arithmetic is promoted into a call-site hypothesis.
- Bank17 search upper-bound initialization at79BE may clarify natural record
  selection; its complete 457-record stored prefix and native consumer are already
  classified. C0E8's explicit common return remains independent of that bound.
  Tracing all writers has no present discriminating connection to C0E9.
- LogicalNot32's A-versus-primary return convention, grid cost/terrain meanings,
  audio identities, CHR effective banking and generic field meanings are honest
  semantic uncertainties in classified objects. None supplies an evidenced pointer,
  record extent or state transition into the seven residual bytes. Additional
  domain reconstruction would presently expand assumptions rather than resolve
  an identified structural decision. No complete semantics is required for Data.
- Generated/generic labels and nearby uncommented raw gaps were checked directly
  in the two source excerpts; no missed instruction body, metadata or consumer
  appears there. All supported neighboring continuations are already classified.
  Full current-source commentary review explicitly has zero unreviewed backlog;
  no exhaustive reread was repeated.

## Gameplay unavailable; terminal scope

Reviewed all scenarios.json milestones: startup attempted; menus, strategy turns,
battle/expedition, later scenarios/transitions and victory/defeat/ending unavailable.
The original accepted transport response is embedded verbatim in primary.json:
emulator.start rejects mapper5 before a CPU fetch. Toolkit docs/nes.md debugger
support explicitly lists0/1/2/3/4/7; RAM/register edits require a started emulator.
Therefore even a known stage/checkpoint/ending RAM recipe cannot run through this
supported binary. No RAM edit, snapshot, artificial visit or natural execution is
claimed. Renaming the mapper, changing the ROM header, or replacing the configured
binary would invalidate this experiment contract; no supported shortcut exists.
No unchanged startup retry or long input sequence was performed. Gameplay coverage
is unavailable, not completed, and no compatibility claim follows.

Conclusion for coordinator review: incomplete, no ready lead or useful revised
test currently survives. This is a reviewed terminal assessment of present
methods/evidence, not proof that future evidence can never classify these bytes.
A new independent native entry organization, non-code role constraint, meaningful
computed reference or separately authorized MMC5 support could reopen research;
none is an actionable unchanged retry. Proposed bytes0, newly accepted0; exact
unknown7 in2 intervals. NES-M5-001 unchanged; no new toolkit issues found.
Coordinator must inspect this review and decisive primary artifacts before
accepting terminality. Final editorial acceptance and archive options remain with
the coordinator; no archive, publication or cleanup performed.
