# Round001 redirect findings — combined structural proposal

No classification change is made by this research. Input remains pass-001/current,
58908 classified and6644 unknown. proposal.json proposes **5463 Code +1179 Data =6642
bytes**, leaving two single-byte ambiguities if independently accepted. All CPU
contexts are fixed CNROM [0,1,2,3]; offsets include header. No new Observed claim.

## Connected graph followed

Primary record lifecycle is established by9F81..9FB0 copying32 bytes to40..5F,
9FE2 initializing,9FE9/9FEE reading B0E7 words and calling A086, and A041 reading
E2A7 words for later updates. Both arrays contain23 coherent words; their adjacent
first code/data boundaries independently separate them. E2A7 is already classified
Data; no reannotation needed. All23 initializer and update targets were inspected,
including repeated targets, and their direct control/call closure followed. They
share entity coordinates/OAM bases5C/5D, fractional position5E/5F, flags40/42,
animation50..54 and collision/displacement helpers. This produces supported state
machines, countdowns, directional movement, collision/death processing and OAM
composition rather than arbitrary linear decoding.

A9D1 provides exact four-entry movement extent: (42 AND0C)>>1 yields byte indices
0,2,4,6. Four handlers update5E/5F or OAM X byB0, share C33A coordinate helper and
return with proper stack balance. A903 contains19 condition callback words, ending
at its independently coherent carry-clear return A929. Tests inspect03D0 fields,
03DC ownership or count nonzero packed inventory slots; natural complete selector
domain is unresolved, unnecessary for bounded structural role.

A615 countdown completion selects A64D using doubled47. Nineteen words end at
A673, and **all nineteen** continuations were followed, including A781 (not merely
the originally noticed prefix). They alter health/capacity/lives/ownership and
inventory bitplanes with bounded checks and sound returns. Initializer/update
connections and downstream helper semantics independently support code; table
pointer plausibility alone was never the sufficient criterion. Complete natural
47 domain is not claimed.

Secondary updater B670 establishes9C=B80E before JSRBA94. Nine words terminate at
B820 first initializer, which supplies coherent state/OAM animation fields. All
unique initializers and continuations were followed. Data atBAB7 consists of16
pointers to twelve-byte parameter records, **not code**. Earlier misaligned BAB6
word speculation was rejected by raw bytes/consumer alignment before proposals.
The original query/replies remain in rpc.jsonl; research-initial.py preserves that
method. Followed B953/BA19/BA30/BA4F/BA06 and secondary movement helpers, rather
than ending at initializers. BA92 is a separate stack-unwind alternate entry:
discards two stack bytes then falls through dispatcherBA94; static precondition
requires an extra return address above the caller's continuation. No actual caller
or natural selection claimed.

BE8B..BECA selects BF58 callbacks using event low5 bits. Six bounded words end at
first callbackBF64; targets C07A(existingRTS),BF64,C083,C147,C11A,C13D have coherent
map/collision, actor spawn, scrolling and exit/transition contracts. All direct
continuations were followed, including BF64's sprite expansion and shared PPU
helpers. Full event-domain proof is not required to accept these bounded words and
coherent routines; values beyond5 remain unproved, not silently mapped.

Discovery visited8499 instruction/context starts with budget15000, exhausted=false.
Unresolved indirect diagnostic sites are these manually resolved dispatchers and
already understood split/audio dispatch sites. Data barriers at0D23/4B12 are
existing inline/stream boundaries and remain barriers. Candidate instruction
intervals do not overlap. No new or existing direct branch/JSR/JMP points into the
proposed Data ranges (boundary-audit.json). Unofficial opcode **89 20 atB986** is
decoded as immediate NOP; retain exact byte emission/mnemonic, not AND or BIT
repair. Its loop tests preceding INY's zero flag; do not invent frame-sentinel
semantics. All other proposed unknown instruction encodings have assembly forms.

## Independent useful avenues completed

Small routines have independently discriminating memory/control contracts:

- 8A93: ten PPU bytes after8AAD establishes destination; falls through accepted
  return. 8D7B and92B2 are short register presets into existing PPU/OAM loops.
- 92C6/92CF: two complementary frame-service/controller-bit waits, each balanced
  JSR/frame service and RTS, exact nine bytes each.9E38 was already classified:
  it sets bit10 at(70),0; no new bytes proposed there.
- 9F32: alternate collision-grid coordinate packing ends by tailjumping existing
 9F12 lookup; shifts/masks preserve known packed-grid format, not just legal code.
- A35E/B57F: byte counter supplies two coordinate nibbles; bounded tile-position
  search wraps horizontal/vertical values and uses existing B054 collision test.
  A3E0/A3E9 setters/render wrappers share two adjacent three-byte parameters and
  9BD7 OAM helper. Complete entry value domains remain unresolved.
- AD1A/B05B/B05F: direction-selection/animation presets join established flags
  and movement code with consistent registers and balanced returns.
- B43D/B443: animation stream offset/pointer initializer and renderer using
  51..54, A2/A3 and existing B4B3 timing; B4DD shares established sprite-relative
  distance routine. B923 maps48 through BAA8 to work fields0A/0D/08 and returns.
- C27E collision-grid bit setter pairs with established C295 clearer; C326 writes
  four OAM attribute slots from inputA/X; C500 returns sprite-zero hit mask40;
  C516 hides sprite zero; C773 fills X bytes at(A0),Y and returns.
- C58C six-byte rectangular PPU descriptors advance88/89 through C560; C5BA rows
  advance sourceA2/A3 by width and destination8A/8B by32. C62A parses two-form
  descriptor stream, advancesA4/A5 through C65F and dispatches existing C523 or
  newly followed C5F7/C606 rectangle fill. C7EF copies rectangles fromA0/A1 to
  PPUA2/A3; C82E fills rectangles. Width/height counters use byte-wrap zero
  convention; callers/ordinary gameplay not required to establish code structure.
- C896 is a coherent scalar lookup helper. Full byte X loaded from072F selects
  exactly C8A0..C99F; B574 is a second consumer. This supplies a full256-byte table
  extent independently of naturally sampled indices or a generator hypothesis.

All candidate opcodes were decoded through configured immutable ROM reads;
unknown-decode.txt is sorted authoritative decoder output. Discovery output is a
review pool, not execution evidence or accepted map state. Proposed code instructions
with views are explicit in proposal.json; disassembly must independently confirm
contracts and exact boundaries before annotating.

## Remaining data and explicit commentary questions

Every residual non-code interval was inspected against references, nearby code and
patterns. proposal.json data_regions records physical/CPU bounds, raw bytes and
individual rationale. Evidence includes finite tile orientation masks, animation
streams withFF terminators, twelve-byte secondary records, coordinate offsets,
PPU masks/OAM descriptor, dispatch words and full256 scalar lookup. B37D..B3D4
can remain generic animation parameter data where exact schema/domain is unresolved.
B592..B62D visibly repeated packed-nibble motion streams are consumedAB8B/ABE4;
consumer reads signed magnitudes from each nibble and recognizesFF. Generic data
is sufficient for any unproved subdivision; don't invent game meaning.

C466..C469 four tile seeds overlap the beginning of the scrolling stream atC469:
C151/C153 setsA0=C469. Count/control pairs progress through C3A6 parser and finish
at zero countC47B. C47C..C4A1 are nineteen repeated A2C4 pairs; no direct code entry
was found. Propose generic Data from repeated pattern and bounded separation, with
no claim of a proven pointer-table consumer. C4A2 itself is excluded below.

Inventory:893F packs one bit from each03DA/03D9/03D8 intoBC, masked7 before8BF6.
A769/A76D/A771/A775/A779/A77D/A781 explicitly produce item values1..7, andA783
packs their bits into these planes. Therefore a global assumption of only0..5 is
unsupported. Six bytes8C36..8C3B are followed by established code A9 10 at8C3C;
for selector6/7 the attribute read accesses those existing instruction bytes. This
is a conditional data read of code storage, not authorization to retype code as
Data or claim natural6/7. Precise naturally allowed effect47 values remain open;
no further structural gain depends on this global semantic question.

9CA7: raw bytes25 FC unambiguously AND zero-pageFC, unlike29 FC at9C94. No direct
store toFC found among accepted/new instruction starts; indirect/indexed writes
or machine initial contents are not thereby excluded. Later ORA#2 combines this
byte mask with cursor flags. Preserve literal behavior and uncertainty; do not
silently convert to immediateFC. No supported natural value contract currently
justifies a new runtime experiment beyond confirming already certain AND mechanics.
