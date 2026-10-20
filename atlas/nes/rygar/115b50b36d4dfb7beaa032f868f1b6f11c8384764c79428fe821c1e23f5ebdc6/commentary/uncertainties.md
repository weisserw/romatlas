# Commentary uncertainties and redirect questions

Classification is 131,088/131,088 bytes (100.00%); zero bytes are Unknown.
The exact empty complement is in [round 8](../analysis/disassembly-r8/unknown-ranges.json).
Seventy new unlabeled generic Data blocks have exact non-code bounds after pattern,
neighbor and conflicting-code inspection. Their format, consumer and original
allocation purpose may remain unresolved; these are semantic limits, not missing
classification. Raw preservation alone remains fidelity support, never data evidence.
Classified extracted CHR sources are separately explained by their upload consumer.
Equates identify CPU aliases, not additional physical ROM classifications.

## L001 — inline return dispatch and state domains

`source/bank_07.asm: DispatchInlineReturnTable` establishes an important format:
for selector A, the pair at return-address+1+2*A is **high then low of the
handler target minus one**. PHA stores high first, low second; RTS pops low first
and increments the restored address. This is distinct from the little-endian
scheduler pointer array. The dispatcher consumes the caller's return address.
`DispatchObjectBehavior` also pushes a packed state byte that nested enemy
handlers deliberately pop. `CheckWorldTriggers` discards another caller frame
when it schedules a transition. These contracts should guide bounded static
analysis and interpretation of handler returns.

Round 2 verifies 31 bounded payloads and their selected handlers, including two
parallel 33-type arrays. Initialized/typed state and stack contracts establish each
local extent; masks alone do not establish all representable selectors. Natural
selection and the exact game identity of many state handlers remain unresolved.
The original open question and review credit survive in the round-1 archive.

## L002 — graphics descriptors and sources

`UploadGraphicsDescriptor` consumes five fields relative to payload-queue end:
-5 repeat count, -4 destination low/chunk counter, -3 destination high,
-2 source-bank low three bits plus source-index upper bits, -1 source high.
The decoder uses a zero low source pointer plus Y as source index. Source and
destination high bytes advance by eight per invocation; eight-byte inner chunks
and a descriptor chunk counter govern transfer. `BuildAreaGraphicsDescriptors`
provides a producer: one parallel repeat field plus four descriptor bytes. A
negative fourth field continues. A nonnegative fourth field sets Work0=$FF, then
INC wraps zero and terminates the chain. This corrects the reversed round-1 prose.
Round 2 verifies 76 destination selectors, 63 repeat fields, 63 four-byte records
and the exact 48,752-byte source union; the 400-byte source gap is now separately classified as static NES 2bpp tile backing
(L012, analysis/disassembly-r7/review.md). Natural upload, an original standalone
asset declaration and original padding purpose remain unresolved.
The consumer low-three-bit count selects 2048-byte invocations followed by
(repeat or 256)*8 tail bytes. Natural selection of all chains is not claimed.

## L003 — audio descriptors and streams

The frame update advances eight logical tracks and pairs them onto four hardware
channels. A track record is sixteen bytes: stream pointer 0..1, envelope pointer
2..3, envelope cursor 4, control/attenuation 5, output control/level 6, period 7..8
(bit7 high means pending), stack cursor 9, and upper bytes used by the backward
stream-call/repeat stack. Separate four-byte counter arrays govern note/envelope
durations. Commands include envelope selection, attenuation, call/return,
repeat/end-repeat, upper control bits and a stop that unwinds the parser call.
Round 2 verifies twelve pitch words, 64 durations, eight envelope pointers and
142 duration/level pairs, ten command pointers, forty descriptors/79 track records
and 6,112 grammar bytes in 42 exact spans. Unconsumed separators and tail are generic non-code Data; their meanings remain unresolved. Source comments distinguish all duration tokens $80..$EF, pitched rest
class $0C and noise rest $10; zero is a note, not a universal rest. The first queued-sound track selector masks its sign bit;
subsequent negative selectors terminate the descriptor. Follow those distinctions
in L003 rather than imposing one universal terminator on every stream.

## Remaining semantic limits

- The object coordinate first axis is vertical (OAM Y consumer), second is
  horizontal (OAM X consumer). Fractions include packed contact edge flags, while
  object flag low bits retain coordinate extension. Do not port them as isolated
  clean Cartesian integers without preserving those effects.
- Object flags, state selectors, world flags and inventory bytes have partially
  understood bit fields. Clear observed mechanics and tentative game purposes
  are kept separate; exact enemy species and most animation state names remain
  unresolved. Numeric states are retained instead of invented game names.
- `$00..$0B` are scratch unions. Routine-local comments explain pointers, masks,
  counters and intermediate coordinates; they are not persistent global fields.
  `$0100` also serves a software stack and attribute shadow in different indexed
  ranges. Caller contracts and software-stack balance matter in a port.
- `Mind`, the health-like pair and status selection names follow the local UI
  consumers and gameplay context. The decimal place-value low/high planes are now bounded; exact gameplay names/scaling of every displayed quantity remain unresolved. Round 8 successfully exercised a spell in an artificial route (Mind 20 to 15, CD=10);
  earlier Mind=0 routes did not demonstrate spell use.
- Natural callers of the two new static entries remain unresolved. Every byte is
  structurally classified; generic Data does not assert a specific schema or
  consumer. Round 8 exercised five later areas, a spell, boss attack attempts and
  the visible terminal END through documented artificial state edits. Boss damage,
  victory, natural completion and full hardware compatibility are not established.

Full current-source readability and whole-ROM classification are separate gates.
Final editorial findings are recorded in editorial-audit.md; the audit result is
in audit.json. Generic Data retains uncertainty about meaning and original intent.

## Round 2 naming and remaining dependencies

Historical map label `QueueSecondarySound` at physical 118046 / CPU $CD0E is
now corrected to `FindFreeObjectSlot` in the portable map and source: instructions scan ordinary object flags and return
Y=slot or zero. Historical `UpdatePlayerObjectControl` at physical 118725 / $CFB5 is now
`QuantizeSignedPlayerVector` in the portable map and source: magnitude division and threshold search pack direction
with signs for velocity lookup. The two misleading map names were corrected in pass-008-editorial; earlier names
remain in immutable checkpoints and historical crosswalk records. Other address-based handler map names now
have local mechanical source names; see the round-2 unit catalog.

Round3 localizes the former universal animation-container rejection. Shared earlier streams are valid; type2 has14 supported local phases, type3 has12 and four excluded velocity bytes, type25 only its supported first group. Physical cells corresponding to $8126 and $82B2..$82B5 are now generic Data; expanded phase eligibility remains unproved. Sprite pointer/count-piece unions and15-area trigger/coordinate/override tables are now bounded independently. World geometry dimensions and origin normalization, Work8 animation transition widths, natural spawn selection, subtype eligibility and many enemy identities remain unresolved. These limitations do not leave classified source unexplained mechanically.

No new gameplay experiments or execution coverage were added during Round 2 commentary. No toolkit issues found;
[report](../analysis/toolkit-issues.md).

## Round 3 source reconciliation

Contact projection, player damage reaction, line drawing/contact, queue split tails and world-strip expansion now have supported bodies and mechanical source names. The two motion exits deliberately discard a caller frame. The world-strip consumer uses stride15/16 depending on orientation; this does not establish a universal geometry matrix container. The sprite union ends at $9954; its 44-byte tail is now generic Data. The 92-record spawn backing columns are now fully bounded; natural selection remains unresolved, and physical103264..103268 is generic Data, followed by the independently supported $9354 routine; the impossible-carry rejection concerned $9350 only. No new execution or natural reachability is claimed.

Remaining semantic questions concern bank4 normalized geometry dimensions, Work8 transition-width domains and eligibility of excluded animation cells. These do not block generic Data classification or current-source readability. Prior lead histories and rejection scopes remain preserved.

## Round4 backing roles versus stronger dimensions

State primary/result objects, all six 92-record spawn columns, independently bounded sentinel-associated fields, attack offsets, auxiliary fields and the available five-destination fallback prefix are now structural Data. This does not establish original transition row widths, natural Work8 values, every spawn subtype eligibility, or valid fallback page indices beyond the available prefix. Bank4 signed origins/payload, 246 intermediate nodes and four shared glyph views are bounded backing representations. Original per-area dimensions, row boundaries and normalized coordinates remain unresolved. Camera masks have four-byte rows across a 276-byte union; this does not impose universal per-area 64-byte views. The second-root minimum prefix is now supported:2525 finite query/render bytes and35 internal backing fields. Its 412-byte suffix, lower 216-byte FF suffix and 90-byte attribute zero suffix are now generic Data; no padding intent or table membership is claimed. Eight internal attribute omissions and action tile slots3/7 are supported backing fields, with natural selection unresolved. See analysis/disassembly-r4/review.md for localized claims and residuals.

## Round5 finite fields and backing slots

The second hierarchy has a2560-byte minimum backing prefix at CPU$9464..$9E64 (physical70772..73332). Query stride16 and render stride15 overlap;2525 bytes have finite selector/view evidence and35 internal fields have separately supported backing membership. This is not an original160-node declaration or natural full-coordinate domain. The 412-byte suffix is generic Data with unresolved meaning. Packed attribute roots bound185 bytes including eight internal omissions; the 90-byte zero suffix is generic Data with unresolved meaning. Action tiles at$9C3D..$9C45 form two four-slot orientation groups, including slots3/7; the separate$9C45 attribute field has six selected direction fields. Natural direction6/7 eligibility is not established. Destination initialization begins$8553; $8552 is the preceding RTS. See ../analysis/disassembly-r5/review.md.

## Round6 reserved coarse-result alignment slots

Twelve formerly Unknown one-byte result-root positions are reserved index-zero slots in encoded common-index backing views. Primary[0] is a search hint; the consumer selects only key/result indices1..N. Complete local keys, result suffixes, paired roots and repeated boundaries support the alignment role even without a read of index0. An original N-element array encoded with virtual base one byte before its allocation remains possible. The zeros are not selected stream ID0; original N+1 declarations and generator intent are unproved. Earlier uncertainty about the unread result-root slots is resolved only at this narrow structural level. Natural reads remain unclaimed. See ../analysis/disassembly-r6/review.md.

## Round 8 alternate entries and generic data

`UpdateTerrainBitplanesAlternate` at physical 103268 / CPU $9354 requires
three valid packed software-stack bytes, a decoded 17-cell strip and Work8 in
{0,2,4,6}, as supplied by the established strip decoder. Its static contract,
252-byte closed control flow, threshold bounds and plane-index bounds are
independently checked; no actual caller or natural entry is established.
`ScaleSignedByteBy16` at physical 116624 / CPU $C780 sign-extends A and enters
the existing shift loop. All 256 signed inputs establish its arithmetic role;
no natural caller is known. Both use effective PRG slots [12,13,14,15].

Generic Data replaces the old structural Unknown gaps. The original source
layout, fill intention, schemas and consumers remain unclaimed where unsupported.
The narrow source boundary notes reuse the documented round-8 pattern/reference
inspection; additional semantic research is not a readability prerequisite.
No new execution was performed during commentary. Gameplay evidence and exact
artificial recipes remain in analysis/scenarios.json and analysis/disassembly-r8.
