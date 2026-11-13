# Commentary uncertainties

All classified code is Static fixed PRG bank 31, CPU $E000–$FFFF. No accepted
instruction fetches or gameplay observations exist because mapper 5 execution is
unsupported. Mechanical names describe native instruction behavior, not game-domain types.

- VM zero-page pairs: $00 address/scratch, $02 descending stack, $04 frame,
  $06 script, $08 primary value, $0C secondary operand. $0A/$0B retain signs
  during signed division. Aliasing is intentional; scratch fields have no global
  persistent type. Frame entry saves eight caller bytes in a nine-byte allocation.
- Opcode target arrays are independently bounded at 256 low/high entries. Table
  membership establishes a static handler contract, not natural opcode selection.
  VM native-call wrappers require matching native/VM stack conventions; a target
  address alone does not establish the callee contract or executable physical bank.
- Extended selector storage begins inside JMP operands at $EE8C. Remaining table
  extent and native $F1F8/$F22E/$F257/$F2A4 helpers are unresolved (L004). No additional
  classification was performed here. Inline jump-record and indexed-destination
  consumers are readable, but adjacent bytecode objects remain unknown (L003).
- PRG register literals $FC/$FD are not physical bank numbers. Audio CPU $8000
  and the unclassified return/restoration sequence require a PRG-mode contract (L002).
- Counter routines are mechanically two big-endian counters seeded $04D2 and
  incremented by two; random-selection purpose remains tentative. PPU/ExRAM transfer
  fields are named from their consumers. Unknown callees in NMI remain numeric.
- English text entries are printable slices, with source previews preserving
  newline and printf syntax. They do not recover complete adjacent control formats.
  Uniform data has checked exact bounds but no asserted padding/allocation purpose.
  CHR is tile-format storage; tile game meaning and effective PPU bank mappings are open.
- Every unknown physical span is explained as opaque. See the exact complement in
  ../analysis/disassembly/pass-001/unknown-ranges.json. Lack of semantics is not used
  to reclassify accepted generic data as unknown.

No unfinished readability range remains. These are semantic/structural questions
for redirect, with existing L002–L004 evidence retained; they are not terminal rejections.
