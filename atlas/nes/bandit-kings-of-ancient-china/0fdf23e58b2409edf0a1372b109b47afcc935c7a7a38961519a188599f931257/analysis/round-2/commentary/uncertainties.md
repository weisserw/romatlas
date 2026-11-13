# Commentary uncertainties

All evidence remains Static. Mapper 5 runtime is unsupported, so there are no
accepted instruction fetches or gameplay observations. Physical bank numbers and
CPU aliases are kept separate. The complete prior uncertainty record is preserved
in analysis/round-1/commentary/uncertainties.md; these are its current dependencies.

- The VM register pairs retain intentional aliases: address/scratch $00, descending
  stack $02, frame $04, script cursor $06, primary $08 and secondary $0C. The compiled
  bank20 ABI reuses $06 for local storage and has separate NativeLocals aliases.
  Scratch pairs and selected local fields are mechanical names, not persistent
  game-domain types. Native frames save eight caller bytes plus a local-save count.
- L003's stored VM representation is now classified and displayed as exact opcode
  and operand bytes with handler names, successors and CPU origins. This establishes
  stored control layout, not natural selection or mapper preservation across native
  calls. Dynamic/native bank contexts remain caller preconditions. Generic adjacent
  formatting bytes retain their bounded Data classification without invented fields.
- L004's extended targets and connected native helpers are now classified and named:
  32-bit arithmetic, bitfields, min/max, memory/stack transfer and banked copy.
  Selector zero overlaps native code; the accepted table has 47 following words.
  Width-zero bit masks and zero divisors are not guarded by their literal consumers.
  LogicalNot32 inverts the predicate in A without storing that inverted value back
  into the primary word; its exact higher-level return convention remains uncertain.
- L002's audio engine, length-linked song records and restoration paths are readable.
  Register literals FC/FD identify physical banks 28/29 only with MMC5 PRG mode 3
  and the recorded ROM geometry. That hardware precondition has not been observed.
  Song names, instrument identities and actual playback remain unresolved.
- Grid helpers mechanically operate on a 14 by 12 staggered coordinate grid, side
  flags, seven-byte linked records and word-valued costs. Cost units and the game
  role of individual terrain codes remain tentative. The $7575 word is a literal
  sentinel. Queue ordering and neighbor mechanics do not establish AI behavior.
- Bank17's 457 ten-byte sorted key/payload records have a concrete bank30 search
  consumer. The runtime upper-bound initialization at $79BE is unresolved; the
  measured prefix count does not prove that the search naturally selects all records.
- Generic data was inspected using accepted consumers, exact whole-interval layouts,
  pointer arrays, alphabets, repeated records and adjacent text/code boundaries.
  Where those only establish stored Data, source comments retain that limit. Encoded
  graphics/name grids need no invented field grammar. CHR tile roles and effective
  PPU bank selections remain unresolved. Printable Text is still a slice of possible
  larger controlled messages; previews do not claim a complete message object.
- The 58 Unknown bytes remain structural ambiguities: nine four-byte JMP0000 bank
  prefixes, the bank23 fourteen-byte transition object, one detached bank30 RTS,
  fixed EBAC $00 and six detached fixed-bank RTS bytes. The source explains each
  boundary. Exact physical ranges remain in analysis/disassembly/pass-002/unknown-ranges.json;
  L007 retains their competing interpretations and a bounded review recipe.

These are investigated limitations, not skipped readability work or a claim of
terminal ROM exhaustion. Final editorial/archive acceptance remains separate.
