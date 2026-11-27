# Round 2 independent structural review

Reviewed evidence-policy/workflow and exact redirect contracts/proposals against
frozen round1 map and unchanged input. Accepted 112032 new bytes: 13318 code,
98714 data. No proposal rejected; lone unproposed physical1CAF1 remains unknown.
Coverage fit was checked only after structural checks, not used as proof.

Independent decisive checks:
- Read decoded residual routines, including all stack-pop/push paths. D611 pops
  its own and caller return, adds2 to caller return, then pushes two argument bytes
  below its own return. D360 therefore consumes CE9F/CEA0 RAM pointer05A7; physical
  1CEAF..1CEB1 exclusive is data. CEA1 resumes LDX, not operand continuation.
  D7C5 retry subtract5 matches JSR+word. Alternate LDA#1 prefixes fall into established
  helper bodies. Decimal conversion loops/digit output and audio wrapper tailcalls
  establish contracts beyond legal decoding. B1B7 is coherent task API under bank6
  and CurrentTask<18 preconditions; no natural producer asserted.
- Read all mapper-select neighborhoods in connected graph. D714 selects temporary
  CHR4/5; D762 loads33 and D764 restores selection. F808 selects3 and F8E4..E8
  explicitly restores6. E4CD/E65D/E77E/E898 similarly restore6 after map/image
  helpers. FB29/FB33 restore6 after audio work. D639 writes selector-matching
  identity bytes; no guessed fixed-to-lower edges accepted. Lower context12/13
  is supported static calling precondition, not proof every runtime invocation meets it.
- Independently decoded every proposed code instruction with configured binary;
  checked exact instruction length, representability, all physical bounds, all
  overlaps/kinds and effective bank contexts before annotation. Saved decoded list.
- Recomputed all8 layout extents from width*height and actual pointers. Full sets
  equal26-byte definition starts, each palette<=2, next-root boundaries independently
  close table ends. Consumers read25tiles plus palette offset25. No selector-domain
  extrapolation needed.
- Rewalked all14 saved stage/screen/CHR streams directly from immutable bytes with
  count-based termination and independently coded token logic. CDCD repeats fixed
  source while CDEB increments per literal:01..3Frepeat,40..7Fzeros,81..FFliteral,
 00/80skip. Exact stored partial literal suffixes and music FF partner bytes have
  encoded/next-pointer boundaries; neither is claimed actually read.
- Rechecked animation sequence/image role union has no cross-role overlap or hole
  between81EE andB5A8. First sequence bounds246pointer entries; dimensions and
  repeated5-byte records plus independent terminal CHR root constrain extent.
  Raw CHR key formula agrees D698..D714; referenced128-byte source groups and
  explicit bootstrap support222groups rather than arbitrary bank sweep.
- Inspected every supplemental generic interval's actual byte patterns, references
  and adjacent code/data boundaries (review.json). Examples include repeated
  tile/action arrays, signed coordinate words, high-bit palette terminators,
  pitch period words/durations and uniform runs. References in lower CPU aliases
  were interpreted by bank; fixed E307/F99C references do not prove mapper6:8000.
  First20 mapper6 bytes have808C/80A0 path-record consumers; remaining12 tile-like
  bytes are bounded by8020 initializer. Generic role retained where exact subtable
  meaning/domain remains uncertain. DMC descriptors C068 encode C080/C420 and
  C440/CAA5: sample bodies, endpoint DAC bytes and interveningFF runs close at NMI
  CAA6. Adjacent palettes and numeric tables are data even without actor identities.

All new annotations Static with role/evidence and audit links. Old classifications
preserved, new proposals clipped against accepted bytes, same-claim adjacent
instructions merged only with sequential matching CPU view. 906/906 regions have
nonempty evidence. No newly observed execution; broad stage0..7/ending/inventory
scenarios already durable and unchanged, so not replayed.

Residual: physical1CAF1 CPUCAE1 is60 RTS between NMI tailJMP and IRQentryCAE2.
No incoming edge or discriminating multistep contract; plausible routine entry and
uninterpreted padding remain competing explanations. Adjacency/legal opcode do
not decide. Retain unknown, no claim dead/unused. Commentary must first cover the
expanded current source and correct decoder prose; terminal rejection assessment
belongs to subsequent redirect against this accepted frontier.
