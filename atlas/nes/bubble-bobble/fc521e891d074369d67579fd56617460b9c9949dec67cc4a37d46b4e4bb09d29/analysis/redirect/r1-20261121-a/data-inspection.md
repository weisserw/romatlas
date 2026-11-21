# Bounded non-code inspection and proposals

Exact Static intervals and roles: proposals.json data array. These are PROPOSALS,
not accepted annotations. All intervals checked against current classified bytes
and complete proposed code boundaries; no overlap. Input pass-003 still30724 Unknown.

*34 bank8 score blocks,4836bytes. Inspected every head/end and surrounding handler
  boundaries after following ALL36 dispatch handlers, including noise$8213. Every
  remaining proposed score block starts00 FE (duration command) and contains repeated
  00/control/sweep triples, little-endian phrase/note pointers, and packed timer-low /
  timer-high-duration pairs with00/FF controls. Accepted consumers$8176/$8217 and note
  readers$8096/$8102 explain this vocabulary and their read-only data role. Mixed
  words/notes/control formats do not constitute instruction streams. Exact intervals
  end at independently supported handler entry boundaries or terminal score boundary
  before accepted FF suffix. GenericData is the intended fallback: no asserted full
  original song layout, every phrase endpoint, or original serializer. Practical
  consumer/nearby-code inspection is complete; semantics do not block non-code.
*Duration table$803B16bytes: packed high nibble fourLSRs andY0..15 at$8133..$8136
  provides independent finite extent. The preceding6bytes$8035 remain Unknown:
  isolated LDA#0/STA$4015/RTS is plausible code but has no supported entry; absence of
  fetch does not make it Data.
*Envelope pointer table$828C32bytes: low control nibbleAND0F/ASL at$814C..$8157
  permits16 LE words. First payload origin$82AC independently separates table32bytes.
  Following361bytes through$8415 contain packed descending amplitude/zero-tail runs;
  consumer$815C..$8172 reads high/low nibbles and holds a terminating zero amplitude.
  Pointer origins and independent next dispatch-table boundary bracket this block.
  GenericData for payload, no exact per-envelope endpoint/generator claim.
*Three dispatch tables72/62/12bytes and task-init words10bytes: exact arithmetic,
  stack and endpoint arguments in inline-contracts.md. Task scan6slots is NOT a proof
  of six initializer words: sixth hypothetical word overlaps accepted$CF04 code.
*102 unique caller-inline words204bytes: each justified locally by its specific
  accepted/candidate caller and nested-frame word-consumer contract, including
  bank10 helper$9073's source word. No inference from skipped raw gaps alone.
*Bank10 prefix physical81936..82272 (336bytes): conspicuous repeated nibble-packed
  rows, same visible vocabulary as accepted banks0..6; ends at independently accepted
  JMP veneer CPU$8150.24*14 is consistent with known map consumer$CF04, but exact
  source-map identity is not needed or asserted. GenericData; not a named level claim.
*Bank12 physical104786..105034 (248bytes): lookup bases$9942/$994A/$9952 are read by
  accepted actor appearance code$9805/$987F. Adjacent origins divide8+8+16bytes;
  remaining216bytes show six36-byte repeated sprite triplet bodies. Word values
  point into these bodies; pair writer$9915 reads tiles/packed attributes. Independent
  accepted RTS$9941 and dispatch code$9A3A bound the whole block. GenericData with
  mechanical sprite lookup explanation; full actor selector domain/original grouping
  unresolved, so stronger per-kind schemas are not required or claimed.

Other residual heterogeneous code-bank gaps were not blanket-labelledData. Their
unknown complement and localized dispositions remain in unknown-ledger.json. Several
look code-like and sit after unproved bank/stack-return barriers; no pattern-only
non-code proposal extends across them. This is a useful combined handoff, not terminal
exhaustion or a whole-ROM completion claim.

## Independent PPU extent proof (final210-byte addition)

ppu-source-extents.json records every exact consumer/caller/pointer/context used:
FA13 reads source header then low nibble -> width, high nibble -> row count; row
loop starts payload at offset1 and consumes width*rows bytes. All5 selected headers
have nonzero fields (22/11/18), so no zero-count underflow ambiguity; total26bytes.
FA04 initializes one row, starts payload offset0 and consumes caller Y. The selected
calls have an immediately preceding accepted/candidate LDY#N, with no intervening
instruction. LE inline source pointers resolve in preserved effective banks. Their
sources are tile-encoded menu/title/story/door text (letter tiles0A..23, blank26,
punctuation25), not executable streams. Shared/overlapping copied sources are
merged only to their exact union,184bytes; no inferred original container tail.
No map/proposed-Code conflict, all intervals were previously Unknown. GenericData
with mechanical PPU text/row explanations is sufficient; no original text encoding
serializer or complete global graphics schema is claimed.
