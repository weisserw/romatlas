# Round002 independent structural review

Input pass-001/current: 58908 classified, 6644 unknown; fixed CNROM CPU PRG
context [0,1,2,3]. Reviewed proposal.json against immutable decoder text, local
bytes, neighboring accepted instructions, callback consumers and stack/register/
record contracts. Fresh configured-binary analysis.decode checks all 2719 proposed
instruction lengths with explicit physical offset, CPU alias and bank context.
The acceptance script preserves each existing region and annotates exact new
instructions, not the envelopes or undifferentiated unknown spans.

Accepted 5463 Code and 1179 Data bytes. Primary lifecycle initializers and updates
share32-byte40..5F work records,5C OAM base and position/animation fields; their
movement, collision, animation, damage and render subpaths agree with existing
helpers. A9D1's AND0C/LSR supplies exactly four movement callback slots; every
handler updates compatible coordinate fields and returns/tailcalls correctly.
A615 completion's pointer dispatch has nineteen coherent effect continuations:
bounded health/capacity increments, ownership flags and three inventory bitplanes
independently support the code role beyond pointer plausibility. Condition tests
A929..A980 return carry from matching state/ownership/count checks. Natural full
selector domains remain unresolved separately.

Secondary initializers configure the same work/OAM fields and direction-dependent
velocity pairs, then join known routines. BAB7 points to twelve-byte records, not
instruction entries. B953's89 20 atB986 is a two-byte unofficial immediateNOP,
retained literally; following branch flags come from INY. BA92's twoPLA prefix
is accepted only under the explicit extra stacked-return contract before existing
BA94 dispatcher, without an actual-caller claim. Event callbacks operate on map
collision bits, actor allocation, OAM expansion and scroll transitions with coherent
continuations. Small independent PPU/OAM/grid/counter routines have specific
hardware/memory transformations, finite loops and balanced returns/tail exits;
legal decoding alone was not the criterion.

All nineteen proposed data intervals match ROM bytes exactly. Consumer indexing
and adjacent independently supported code discriminate the finite masks, parameters,
callback words, tile permutations, descriptors and lookup. B586 offsets plus repeated
FF-terminated packed-nibble signed-displacement streams agree with AB84/AB8B/ABE4.
BAA4..BC0E includes repeated twelve-byte records and distinct small streams/flags;
retaining Data does not assert each schema or natural index range. B37D..B3D4
is generic animation parameter data where complete index/schema is unproved.
9B6C is repeated four-byte00/04/02/84 content plus09/FF ending, not a strict
three-byte-record schema (wording in proposal says 'triples'); role/bounds remain
supported by9B32 consumer and next9B7A code. Do not turn that wording into a format
assertion. C8A0..C99F is exactly256 bytes because C896 uses arbitrary byteX from072F
and B574 independently uses the same base; generator/distribution is not claimed.
C466..C47B tile seed/control stream supports overlapping data reads atC469.
C47C..C4A1 is accepted as generic non-code because nineteen repeatedA2/C4 pairs,
no control entry, and neighboring endpoints discriminate non-code. It is not claimed
as a proven dispatch table merely because words resemble C4A2 pointers.

Independent direct-control audit reads branch/JSR/JMP operand bytes at all existing
and proposed instruction starts: every ROM direct target is an accepted/proposed
instruction boundary; zero direct targets enter new Data or either residual byte.
No proposed byte overlaps accepted bytes or another proposal. Inline barriers remain
intact. Fresh-decode.json, independent-audit.json and rpc.jsonl retain primary checks.
The first annotate call lacked required region id and was rejected without changes;
a followup with next_id was also rejected as an edit of nonexistent ID. Script now uses the documented implementation creation sentinel id=0. No toolkit defect inferred.

Exact two-byte residual: physical2991 CPUA981 and44B2 CPUC4A2, each60. Reused
localized residual-review.md: adjacent return/table-pointer patterns admit spare
RTS and data/padding interpretations. No established entry or stack contract
selects these specific bytes. Artificial execution would not discriminate intended
roles. Preserve unknown; do not call them dead/unreachable. No whole-ROM completion
or terminal redirect exhaustion claim.

Broad progression evidence/scenarios from round001 remains valid: natural title/
stage1, artificially initialized8/24/31/32 and fourth-loop ending were visited with
representative movement/shooting. Repeating unchanged sightseeing would not resolve
the two byte-role ambiguities. No new runtime evidence or Observed annotation.
No ROM/toolkit/commentary edits. Existing source/full review must be carried forward
by next commentary phase; generated/pass-002 is fresh structural source only.
