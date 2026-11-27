# Pattern classification review

Primary neighborhoods: uniform-neighbors.txt (physical file ranges, both boundaries).
Practical reference check: current accepted code/discovery references were searched for
conflicting instruction starts and overlapping operands; no such overlap occurred.
This does not claim absence of consumers or execution. Broad generic classification
uses byte structure, not non-execution. Consumer identification remains pending.

* $011F4-$01213 is32 copies$0F following bytes $3F,$00,$20, consistent with a
  PPU palette-address/length descriptor, then color groups containing $0F,$11.
  Classify bounded data; likely palette initialization, precise descriptor consumer
  unresolved. Do not widen to adjacent records.
* $03FA2-$0400B, $07A2E-$0800B, $0B11B-$0C009, $0F5F3-$1000B and
  $1BC77-$1C009 are long$FF runs following RTS or bounded record lists, stopping
  at reset/vector trailers. This is clear non-code; no specific padding generator
  or complete trailer structure is asserted. $08015-$0803F is a separate uniform
  run between reset-stub bytes and JMP entries; keep stubs/entries separate.
* $10E9B-$10EC6 zeros separate compact low-valued repeated records. $12A19-$12C0F
  zeros follow RTS and precede contiguous JMP entries. $12C55-$12C8F zeros separate
  JMP table and LDA-based routine. $1388C-$1400C zeros follow small signed/value
  records before bank trailer. BRK decoding of large zero blocks supplies no coherent
  execution structure; layout and exact transition boundaries support data.
* $180A9-$1810F$FF separates JMP entries and pointer-like words. In$189FE-$19D73,
  $AB,$CA,$AA,$FF constant runs are embedded in repeated two-dimensional-looking
  rows using narrow families $C0-$C8/$90-$9F, plus pointer-like separators.
  The$CA and$AA runs are129/65 and72/40 consecutive DEY/TAX opcode values, but
  surrounding repeated row patterns and differently-valued long slabs show non-code
  record/map-like content rather than instruction sequences. No timing-loop claim,
  exact tilemap schema or role is inferred from uniformity alone.
* Printable runs include military equipment names, action labels and full English
  battle messages with punctuation; other long alphabetic/pattern runs occur amid
  table/map-like data. Exact runs only are classified. Adjacent control bytes and
  larger string/table extent remain unknown. Text regions denote ASCII content;
  generic patterned runs remain Data. No complete text encoding or consumer claim.

Final fetch reconciliation correction: original printable run42DD-42F4 included a
leading RTS byte60 (printable backtick). Actual accepted fetch42DD and indexed
consumer82CE,Y prove routine return at42DD, table begins42DE. Split first byte as
Observed code; retain23-byte numeric/pattern table under consumer evidence. This
falsifies that single prefix classification, not surrounding non-code pattern. Old
region/rejection review preserved in fetch-conflict-review.json and prior checkpoints.
