# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## pass-001 disassembly — supported structural batch

Baseline and current checkpoint verified byte-identical. [Progress/counts](progress.md)
contains the per-bank code/data/unknown inventory; [raw counts](disassembly/pass-001/counts.json)
and [unknown complement](disassembly/pass-001/unknown-ranges.json) preserve exact scope.

- D001 (verified): header/section layout, reset $F500, NMI $F641, IRQ $FFD6,
  explicit physical bank31 view at $E000. Rooted control flow and hardware/stack
  contracts support 624 instructions, 1,353 unique code bytes. Reachability stays
  unobserved. $E42A consumes inline frame metadata; graph deliberately stops
  ordinary return continuation at $E0E6. Audio call $FE88 stops at unknown $8000.
- D002 (verified): 16 header bytes and 6 vector bytes classified Data; CHR
  131,072 bytes classified Image in sixteen physical 8 KiB banks. Header-defined
  separate PPU storage and evenly spaced per-bank tile samples show coherent
  graphics; [sample sheet](disassembly/pass-001/chr-samples.png).
- D003 (verified): 39 maximal uniform runs, 45,730 bytes, generic Static Data.
  [Boundary report](disassembly/pass-001/run-boundaries.txt) checked nearby
  text/table/native stubs. Some runs cross physical bank boundaries, deliberately
  without inventing source padding objects. Rooted code never intersects them.
  Uniformity/neighbor structure justifies non-code; no consumer or specific fill
  purpose is claimed.
- D004 (verified): 406 readable English message/menu/scenario slices, 14,162
  bytes including NUL terminators. [Candidates](disassembly/pass-001/text-candidates.json)
  were inspected for intelligible prose/substitutions. Printable prefixes can
  belong to control sequences; exact slice boundaries do not claim complete
  original string object boundaries. Uninspected surrounding bytes remain unknown.
- D005 (verified): two 8-byte sprite tile/X arrays $F6D6..$F6E6; consumer's
  Y=0, increment, CPY #8 bounds full extent, not merely sampled indices.
- R001 (blocked runtime only): emulator.start rejects mapper5 before any CPU
  fetch. See NES-M5-001 in [toolkit report](toolkit-issues.md). No repeated
  button sequences or speculative RAM/register edits were attempted. Progression
  checklist in [scenarios.json](scenarios.json) explains later/ending unavailable.

### Productive frontier and narrowly deferred claims

L001: interpreter $E46E byte fetch and $EC3F/$ED3F split dispatch tables have
163 distinct targets. [Interpreter listing](disassembly/pass-001/interpreter-listing.txt)
shows coherent local/frame/stack operations, 16-bit arithmetic and common
JMP $E46E exits; helpers preserve calling contracts. Table role and native handlers
are promising static classifications; complete per-target entry preconditions,
BRK semantics at $EBAB/$EBAC, and indexed target extent remain to be assessed as
a connected batch. Legal decoding or pointer values alone were not accepted.
Natural opcode selection is a separate question, not an acceptance prerequisite.
The extended $EE71 dispatcher scales a script byte and calls through $EE8C words;
do not assume its table's complete extent from apparent pointer runs alone.

L002: $FE6D writes $FC/$FD through bank helper X=1/2 and JSR $8000. Effective
physical bank-mode contract and bank restoration are the missing facts; exact
literal setup makes a bounded static test useful. No code claim made from pointer
or register values alone.

L003: recurring native stubs call $E42A followed by metadata and interpreted
bytecode. Classify supported script blocks as Data once their consumer/bounds or
clear non-native pattern is established; do not require semantic decompilation.
Full script grammar and native-call escape bank contracts require redirect review
before bulk propagation. None of these leads was rejected; this is an incomplete
phase handoff with useful next tests, not terminal exhaustion.

### Verification and metrics

ROM classification: 48.92% (192355 / 393232 bytes classified; 200877 bytes unknown).

499 regions, all Static; 100% of classified bytes carry evidence, no observed
coverage, no runtime trace. Internal map validation and portable map validation
passed; baseline and [pass-001 checkpoint](pass-001-verification.json) both
byte-identical. ROM SHA-256 unchanged. Current source readability and broader
editorial acceptance remain pending commentary.
Toolkit: 1 documented missing feature; no confirmed/suspected bugs; runtime blocked,
static classification unblocked.

## pass-001 connected extension — final checkpoint

L001 (verified, +2,519 bytes): reviewed all regular interpreter handler families
against shared VM preconditions and dispatch entries; accepted 2,005 native Code
bytes, 512 split dispatch bytes and 2 inline boot-frame bytes. This is stronger
than pointers/decoding alone: frame, stack, memory, arithmetic and common dispatcher
continuation contracts agree across the entire batch. Full 0..255 encoded-byte
index domain, exactly spaced low/high bases, and complete pair consistency support
array extents independently of natural opcode selection.

$EBAB BRK has IRQ RTI continuation $EBAD JMP $E46E. $EBAC remains locally unknown.
$EE71's final indirect JMP occupies through $EE8D; its own operand bytes alias
first zero entry at dispatch base $EE8C. Proposed code end $EE8C was correctly
rejected as truncated; final code end $EE8E accepted. No toolkit bug inferred.

[Accepted extension](disassembly/pass-001/interpreter-accepted.json) and
[final verification](pass-001-extended-verification.json). Initial pass-001 and
pre-extension pass-001-final checkpoints preserved historically.

Produced and verified L001 within this phase; no inherited leads consumed.
L002/L003 ready unchanged; L004 separates extended selector/table/helper review
with precise alias evidence. No exhaustive closure or terminal exhaustion claimed.

ROM classification: 49.56% (194874 / 393232 bytes classified; 198358 bytes unknown).

Final 513 annotated regions; Code 3358, Data 46282,
Text 14162, Image 131072; all Static, evidence coverage 100% of
classified bytes. Final per-bank counts/unknown complement saved in same linked
artifacts. Internal+portable validation and byte-identical final rebuild passed.
Runtime progression unavailable due documented mapper5 feature boundary; full
current-source commentary and final editorial audit pending.

## redirect-001 — active productive research continuation

L002/L003/L004 materially advanced through connected handlers, consumers and native
contracts; L005 native bank20 split from residual triage and ready; L006 retains a
supported4570-byte record prefix and an active asset-layout frontier. Exact proposed
novel135828 bytes, plus accepted Text-prefix correction at81169. These are proposals,
not accepted classification. No toolkit or map/source edits. Primary compact result,
proof scope, rejected wrong-origin premise and precise remaining methods:
[active handoff](handoffs/redirect-001-active.md),
[research frontier](redirect/redirect-001/active.md),
[exact current unknown complement](redirect/redirect-001/unknown-ledger.json).
Current accepted194874, unknown198358; no exhaustion/terminal decision.

## Pass-002 — reviewed redirect batch accepted

ROM classification: 99.99% (393174 / 393232 bytes classified; 58 bytes unknown).

Exactly one disassembly pass. Consumed ready L002–L006 combined supported batch,
198300 novel bytes (185374 Data,12926 Code), plus physical81169 Text→Code RTS
correction. No other accepted classification changed. All additions Static,
reachability unobserved. Evidence and independent review:
[review](disassembly/pass-002/review.md), [reproducible checks/annotation](disassembly/pass-002/apply.py).
Exact per-bank and total counts: [counts](disassembly/pass-002/counts.json);
[58-byte complement](disassembly/pass-002/unknown-ranges.json).

Physical PRG banks17–22,24,27 each8188 classified/4 unknown; bank23
8174/18; bank30 8191/1; bank31 8185/7; all other PRG banks8192/0.
Header16/0; all16 CHR banks8192/0. Kind totals: Code16285, Data231656,
Text14161, Image131072;2785 regions.

Saved analysis/state.json and maps/current.json, validated and created NEW
[pass-002 checkpoint](../maps/pass-002.json); [verification](pass-002-verification.json)
confirms byte-identical rebuild with unchanged input SHA256. Map SHA256
b420bacc698c7a22659e5c1904aae320c7ba2eb6de461a4afbd3e11550476385.
Runtime progression and RAM shortcuts remain unavailable through documented mapper5
limitation; unchanged failed startup not repeated. Scenario checklist retained.
Toolkit report: [NES-M5-001](toolkit-issues.md), one documented missing feature,
zero confirmed/suspected bugs; no toolkit edits. Existing commentary untouched,
full current-source commentary is next. Produced L007 narrow residual review;
not terminal exhaustion or completion. No deletion, archive or publication.

Redirect-002 reviewed L007 directly against primary bytes rather than inherited
sufficiency judgments. New separable L008 batch proposes57 bytes; exact claims,
strongest alternatives and localized remaining1-byte ambiguity are recorded in
analysis/redirect/redirect-002/review.md. Accepted58Unknown remains unchanged;
independent disassembly assessment next, not a terminal exhaustion decision.

## pass-003 — localized primary review

[Review](disassembly/pass-003/review.md) reproduces primary residual evidence exactly,
accepts51 Static Data bytes and rejects the six-byte Code proposal narrowly.
Structural role and reachability evaluated separately; no observed execution asserted.
All accepted downstream objects already classified. Seven bytes remain materially
ambiguous: physical246009 (bank30+E9/C0E9) and260610:260616
(bank31+19F2/F9F2:F9F8). Hypothetical JSR preconditions/RTS semantics do not
resolve code versus redundant return/padding. Native neighbors and homogeneous
family considered jointly; no caller prerequisite imposed. Prior proposal/history
retained in L007/L008. Exact complement/per-bank counts and fresh byte-identical
checkpoint: [progress](progress.md), [verification](pass-003-verification.json).
No unchanged mapper5 retry, toolkit edit, source modification or publication.
Incomplete; commentary/redirect may inspect these exact residuals and this scoped
reasoning before any terminal decision. NES-M5-001 unchanged.

## Redirect-003 — primary rejection review; incomplete

Current-source readability gate complete, source SHA256 e0dd381ce63861b555fe848e41f5563daf7cafc9741f07f44a0edc8990b1f5e1. Accepted393225/393232;7Unknown in2 intervals, no proposal/new acceptance. Reviewed exact map complement, current native neighbors/NMI call and prior decisive artifacts; no useful revised inference/test survives for C0E9 or F9F2:F9F8. Code and generic Data alternatives assessed independently; no universal caller or data-semantics prerequisite. [Compact rejection review](redirect/redirect-003/review.md), [primary evidence](redirect/redirect-003/primary.json), [exact ledger](redirect/redirect-003/unknown-ledger.json). L007/L008 history appended; no ready lead. Gameplay later/ending milestones remain unavailable through original mapper5 rejection; no unchanged runtime retry. Terminal-incomplete assessment requires coordinator primary-artifact inspection. ROM,map,state,source,configured binary preserved; disk6.5GiB free,scratch4KiB,no cleanup/archive/publication. NES-M5-001 unchanged; no new toolkit issues found.
