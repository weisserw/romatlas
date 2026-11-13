# redirect-001 active research

Maps/state/source/input unchanged. Pass-001-extended: 194874 classified,
198358 unknown. All proposals Static; mapper5 execution unavailable.

Completed connected observations:
- L004: extended word storage EE8E:EEEC = selectors1..47; selector0 aliases
  accepted JMP operands EE8C:EE8E. First target EEEC gives independent endpoint;
  all47 targets align coherent 32-bit handlers EEEC:F1F8. Adjacent bitfield
  helpers F1F8:F2A4, no-op F2A4:F2A5 and VM-stack min/max/copy helpers F2A5:F35C
  fit accepted caller contracts. See helpers.json/txt and vm-fixedbank.json.
- L003: all25 fixed-bank JSR E42A stubs E003:E3BF have small negative/zero frame
  words and complete VM grammar. 824 reached representation bytes, no overlaps,
  no stop; seven trailing bytes E138:E13F are unreachable redundant jump/return
  representation, localized separately. Native sqrt helper E3BF:E42A is bounded
  8-round restoring integer square-root algorithm with negative input zero path.
- VM calls expose EE3F:EE71 register gateway via word FFF8=F8ED. F8ED:F917
  saves A/X/Y, clears result66/67, dispatches through F917:F957, restores and RTI.
  Exactly32 words precede independently accepted F957 code; all targets fit complete
  service contracts. Native service interval F9F8:FE6D contains inline tables
  FA75:FA7F (five fixed CPU-slot entry addresses) and FC55:FC5B (six-byte multiply
  constant). Other bytes are ordinary code; FA7F must be decoded freshly because
  sequential decode across FA75 table misaligns. FE8B:FE9A restores audio banks;
  FE9B:FF7A complete three-channel blocking record-player and PPU request setup.
- L002: under documented MMC5 reset-mode3 precondition, FC/FD select physical
  banks28/29 because PRG has32 banks. Reset literal setup, no direct STA/STX/STY
  5100 pattern, NMI-guard setter and reverse restoration agree. Hardware premise:
  https://www.nesdev.org/wiki/MMC5 . Static role proof does not require natural call.
  Audio native8000:8392, lookup bundle8392:8450 (last14FF), native8450:84A9,
  then already classified87FF. At8500:A5BC exactly20 positive length-linked song
  records, each 10-byte header with four relative track origins strictly internal;
  final continuation is FFFF, not another record. Consumer8450 and FDCf initialize
  same structure. A700:AF00 64 coherent32-byte instrument records match native
  A700+instrument*32 consumer8321; overall generic non-code role is clear even
  where full field/selector domain remains unresolved. See audio-records.json.
- Independent F35C:F398 30-word string-origin array: first pointerF398 and every
  target exact start of contiguous accepted English names throughF499. F499:F4A1
  four-word animal-name origins similarly precede stringsF4A1:F4BA. Extent follows
  encoded origin plus complete string-start agreement; no global selector claim.

Current productive frontier:
The VM grammar generalizes strongly beyond fixed bank. Correct physical CPU origin
is essential. Bank0 at8000: 2321 instructions/4539 script bytes, zero overlaps/stops;
bank1 at8000: 3777/6652, zero overlaps/stops; bank9 similarly3668/6630;
bank26 508/918. Exploratory pair0 incorrectly assumed bank1 atA000 and produced
28 overlaps; this falsifies that origin assumption, not grammar. Preserve evidence.
Banks2..8,10,11 have encoded JMP A003/nearby, so run them atA000; initial8000 scans
stop only at encodedAxxx/Bxxx targets, hence wrong origin. Banks12/13 and14/15
are true16KiB pairs8000/A000 (second bank starts continued bytecode), so scan pairs.
Bank16 and30 likelyC000; test against encoded local targets and barriers. No
blanket classification from signature inventory; package only independently
consistent regions, retaining native-call bank/preservation uncertainty separately.
After these bounded tests, inspect exact native targets outside established service
block only when consumer/calling contract gives credible progress; preserve frontier
if context interruption occurs. No exhaustion/terminal assessment yet.

## Later results / exact current frontier

All corrected VM contexts succeeded: banks0/1/9/26 at8000;2–8/10/11/16 atA000;
12+13 and14+15 paired8000/A000;30 atC000;31 atE000. Total988 stubs and57516
representation records. Zero overlaps/stops in every correct context. These are
structural encoding/branch/return constraints, not observed execution. Native-call
continuation is a representation claim; actual runtime bank preservation remains
unobserved. `proposals.json` is independently reviewable only, not an edited map.
Novel125525 bytes (118281Data,7244Code), zero accepted/proposed kind conflicts.
`unknown-ledger.json` retains exact current219-range complement198358 bytes.

Further connected native targets recovered from VM calls: bank0 8C27:8CE9 checksum
restore with four-byte magic8CCD:8CD1; bank6 B2B4:B357 checksum save with magic
B33B:B33F; bank1 9B3B:9BA7 linked-node aggregate; bank9 9BD8:9D02 bounded seven-way
hex-grid neighbor lookup with table9C03:9C11; bank11 B95C:BA59 bounded14x12 grid
filter and helper. Listings saved native-bank*. These are not yet added to proposal.
All call known gateway or local helpers; no unreviewed connected callee remains.

Fresh residual bank triage is decisive: bank20 contains native6502 at physical
20*8192+16+~0x600 onwards, despite its JMP0000 bank prefix and graphics-like first
payload. Therefore do NOT generalize bank-prefix/data heuristics across opaque
banks. Native samples atoffset800/1000/1800 show coherent pointer/register code,
JSR B199, B35A, B833 and localAF62/B02B branches; likely CPUA000 view. Exact
native start, inline tables and end remain active next investigation. Other banks
17/18/19/21/22/23/24/27 samples are repeated indices, nibble maps, records or text,
but no whole-bank claim made. Need inspect their consumer/layout bounds after
bank20 mixed-code frontier; lack of semantics alone must not keep clear data unknown.

## Current resumable research checkpoint

Latest exact proposal: **135828 novel bytes** (122963Data +12865Code), plus one
accepted Text→Code correction at physical81169 (bank9 CPU9D01 RTS, accepted region221
MessageText_013D11 starts one byte too early). This is a primary control-flow-backed
boundary correction, not a new toolkit issue. `proposals.json` lists the conflict
explicitly; independent disassembly must split/adjust Text before Code replacement.
No map/source/state classification changes made here.

Current accepted scope stays194874/393232 and198358 unknown. Hypothetical residual
if every proposal accepted is62530; never report that as accepted/current unknown.

Bank20 native closure: A600:B86E exact4718 bytes, all9 framed procedures and21
inline call records accounted for. 4467 graph-covered bytes (including90 inline Data)
and251 independently inspected helper-family bytes; no unofficial decode or overlap.
External9757/9BD8 require a matching8000 bank contract (bank9 provides supported
roles); actual mapping is unobserved, and native graph claims no natural entry.

Bank17 localized Data A004:B1DE now supported: 457 strict ascending ten-byte
key+four-word payload records. VMbank30 C841 surrounding pushes select service3,
register2, bankF1; native FA7F/F973 gives physical17 atA000 under mode3. NativeC003
binary-search uses A004+10*index and copies8 payload bytes. First nonincreasing key
atB1DE switches to name/tile representation. No recovered runtime79BE upper-bound
initializer is claimed or required to identify this non-code prefix. Primary concise
record: bank17-records.json; proposal includes4570 novel Data bytes.

The current worker is saving a continuation checkpoint, **not declaring redirect
exhausted or requiring an empty phase cycle**. Concrete remaining frontier is L006:
finish bounded whole-layout/code-boundary triage of record/asset payloads in
banks17/18/19/21/22/23/24/27 and bank20 A003:A600 prefix. Do not generalize samples
into a whole-bank claim: bank20 already falsifies prefix-only inference. Promising
next tests: bank18 ascending pointers at physical bank18 offset~1800 imply fixed
0x3C stride; establish table starts/endpoints against encoded origins and consumer
copy contracts. Bank21 FF-separated short records and bank23/27 fixed records can
support generic Data after exact boundary review even without semantic recovery.
Remaining script gaps often contain formatted messages, tables and redundant VM
branches; reuse exact ledger and native target inventory rather than rescan all ROM.

All initial L002–L004 questions received connected and independent investigation;
remaining actionable asset frontier is explicit. Terminal rejection review is not
applicable to this interrupted productive checkpoint. No archival/publication,
cleanup/deletion, toolkit edit or runtime experiment occurred.

## L006 layout and script-gap continuation completed

Latest combined proposal is **198300 novel bytes** (185374Data +12926Code), plus
unchanged one Text→Code correction at81169. Accepted checkpoint remains194874
classified and198358 unknown. Hypothetical post-proposal residual is58; it is NOT
current unknown.2271 compact novel segments are regenerated in proposal-segments.json.

`asset-layouts.json`/`assets.py` and `asset-layout-review.md` record whole-byte
layout/boundary evidence. Bank18's exact100x60 objects have independent100-word
origins at1774/183C (A004+60*i,A022+60*i);100x8 palette-like objects have exact
origins at1C24 (B904+8*i). Bank23 has255 eighteen-byte records satisfying six exact
zero columns; bank27 has255 twenty-four-byte records with complete16-byte printable
name fields. Other complete asset layouts and explicit uniformFF suffixes support
generic Data. Bank20 prefix ends precisely at independently supported nativeA600.
No generic inference comes from a JMP0000 prefix.

Script-adjacent gap triage found230 hypothetical residual segments14085 bytes after
asset proposals. Whole-interval text/control evidence (`gap-text.json`) adds3217
Data bytes. Full mixed interval review (`gap-layouts.json`) adds10720Data bytes;
next-bank headers at bank1/16 ends are explicitly excluded. This includes formatted
English and multibyte diagnostic text, origin words, numeric tables and repeated
tile borders. Exact semantics are deliberately not prerequisites for generic Data.
`gap-control.json` adds90 bytes (native entry JMPs, localized redundant VM tails,
fixed13-byteF9AA:F9B7 register primitive). All JMP targets have independent Code
support; VM branch targets align established representation. No natural execution
is claimed. The complete existing VM/native graphs were reused, not replayed.

Practical remaining independent/connected frontier has been assessed.58 hypothetical
bytes are: nine4-byte JMP0000 headers in17..24/27 (36); special bank23 11F2:1200
numeric/native ambiguous object14; isolated bank30 C0E9 RTS1; fixedEBAC00 byte1;
sixRTS bytesF9F2:F9F8. Exact code-versus-data ambiguities and scoped negative
reasoning are documented in `gap-layout-review.md`; missing specific semantics is
not their barrier. No further concrete promising inference survives those local
constraints in this bounded batch. This is a ready supported-batch handoff for
independent disassembly, not terminal incomplete or global research exhaustion.

L006 is now ready in leads.json with appended history. Current unknown-ledger.json
still derives exact198358-byte complement from unchanged state; proposal dispositions
are separate. No map/source/state classification, toolkit edits, deletion, archive
or publication. Configured binary unchanged. Disk last checked6.6GiB free.
