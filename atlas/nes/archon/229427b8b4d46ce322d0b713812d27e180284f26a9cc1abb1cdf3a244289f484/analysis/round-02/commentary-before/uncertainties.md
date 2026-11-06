# Commentary uncertainties

Carry forward relevant phase-1 ambiguities as their regions are reviewed.

Round01 reviewed limits:

- All 10,739 classified code bytes and 98,812 Data bytes are covered by local explanations. The 21,537 Unknown bytes remain unknown, including incomplete arena motion/hit arms, board movement/scoring/special-action bodies, audio command handlers and bounded table candidates. Source comments delimit every opaque span; no guessed instructions or new classification were introduced.
- Graphics/screen assets in physical banks1..6 are established non-code. Prior complete-bank bitmap inspection plus classified CHR/PPU consumers supports useful bank-level notes. Exact tile, map, palette, music and blank subrange boundaries are still unresolved; the generic Data annotation is retained without claiming all bytes are graphics.
- Piece types, movement modes, projectile behavior and palette routines have mechanical names. Specific piece/spell names, several type-dependent strength adjustments and the complete winner-display path remain uncertain. Pointer read aliases inside Unknown spans do not claim table extent.
- Inline upload/copy helpers have exact stack contracts. Additional two-/three-/four-byte records are explained as likely inline records but remain Unknown pending structural promotion. Static consumer understanding does not prove natural execution of every continuation.
- Physical1F684 is an observed alternate TXA entry overlapping the operand of BIT at1F683. RenderActorDirect=$F674 preserves this entry without changing the baseline region partition. CPU execution aliases and physical bytes remain distinct.
- Coordinate convention was checked against primary code: CursorRow=$61 and CursorColumn=$63; SelectedRow=$60 and SelectedColumn=$62; CandidateRow=$6B and CandidateColumn=$6C. ReadBoardCell uses Y as row-pointer index and A as column; BoardCellToActorPixels stores A to X and Y to Y. The initial commentary draft had these reversed and was corrected before handoff; prior disassembly coordinates were correct.
- CPU RAM is reused. Work0..Work5 are scratch; movement counters double as survivor counts; SecondActorTypeOrWork and several audio/terrain fields change role by phase. Do not turn these mechanical aliases into independent persistent objects in a port.
