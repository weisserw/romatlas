# Connected field data span

CPU84FA-A886, physical1850A-1A896, bank6/effective slots[12,13,14,15].
Start follows exact terminal counted message, end precedes accepted drawing routine
A887. Current accepted code and candidate operands do not conflict. Uniform/ASCII
subregions from prior batch preserved. Generic non-code extent does not claim one
exclusive table/schema across the span.

Practical role/consumer inspection:
* AB5F reads24 sprite coordinate pairs from84FA/84FB with X stepping2 until Y=$60:
  exact48bytes end852A. AB7D reads pointer852A/852B by doubled selector. Its first
  pointer854C gives17 words; all17 pointer values advance48bytes, to884C/end887C.
  Sprite-style tile/attribute pairs form corresponding records.
* Nearby887C-8C4B contains pointer-led/low-alphabet icon rows, longAB background
  runs and repeated boundaries88/89/8C. 8C4C onward has pointer-led PPU records
  with destinations24xx/25xx and length/count bytes, plus further tile rows.
* B434 explicitly emits32-byte rows at9D34 and9D64. E6A3 reads map-selector
  pointers9E05/06. AA12 reads stage-selector pointers9FF7/8 and copies counted
  records to6000. Observed16-unit RAM block is reproduced by corresponding
  immutable eight-byte stage records aroundA000; every256-byte sample through
  terminalA886 remains repeated record/table content rather than instructions.
* 943A+ points into regular8-byte unit templates;98E0+ and9Bxx contain large
  uniform/row-pattern spans. CA/AA-valued runs were separately checked in
  pattern-review.md; their opcode decodability does not override record context.

No full semantic object partition or generator is claimed. Primary consumers,
repeated finite record layouts, densely repeated tile patterns, exact neighboring
code/message boundaries and matching runtime records jointly establish non-code.
Remain open to splitting/revision if future contradictory code evidence appears.
