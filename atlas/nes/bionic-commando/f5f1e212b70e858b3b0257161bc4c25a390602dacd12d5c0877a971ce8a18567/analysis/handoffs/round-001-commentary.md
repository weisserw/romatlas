# Round 001 commentary handoff

Status: complete readability across ALL current source; ready for redirect.
Whole-ROM classification is incomplete. Final editorial/archive acceptance pending.

Checkpoint: round-001-final-tables. Map SHA-256:
93cee40f9b4155b0230113cdb24f0bc550ddb2964c957f5fac942125a498cef8.
Map and internal state match frozen commentary baseline; no structural changes.
ROM classification: 61.42% (161031 / 262160 bytes classified; 101129 bytes unknown).

Scope: all 747 physical scaffold entries reviewed (298 Code / 233 Data / 16 Image /
200 Unknown), plus 298 compact code units with execution context and source links.
Maintained commentary/source/game.asm and symbols.inc contain descriptive routines,
resolved bank-safe control labels, 181 exact-value aliases and mechanical/semantic
source explanations. Opaque data retain original bytes, inspected boundaries and
honest unknown schema notes. No readability backlog or deferred source ranges.

Verification: final fresh commentary/verification.json has ok=true,
byte_identical=true and original ROM SHA-256. reading-after.asm refreshed after it.
game.asm SHA-256: 5fc68e7aa45317343e4b3404b80a9147295402d361f2436ca7bda78a959d6220.
symbols.inc SHA-256: 15809e9431d6c8cc289e6e077e10e5ee0c43909131bd13202d0ff78b12afb6b7.
Frozen before/, maps and evidence preserved. Final audit intentionally reserved.

Representative source locations (relative to commentary/):
- source/game.asm:500 palette fade: curve pointer, countdown, clamping and carry.
- source/game.asm:1495 audio command/priority initialization; :2575 stream timing,
  note decoding and channel records; named record fields in symbols.inc.
- source/game.asm:3512 grapple creation: direction tables, coordinate offsets and
  slot initialization; :3576 camera tracking, clamped displacement and deltas.
- source/game.asm:4016 reset; :4066 NMI/DMA/queues; :5848 eight-bit reversal;
  :5882 five-byte graphics descriptors; :6546 return-address inline dispatch.
- source/game.asm:7247 MMC1 five-write PRG serialization; :7488 packed PPU command
  format; :8004 pose-adjusted player collision intervals and carry contract.

Unresolved meanings / redirect: credits continuation and CE/CF prerequisites around
bank3 BB23/BB33; final cockpit projectile and collision contract; room/tile tuple
schemas, dialogue/metasprite boundaries and packed-font indexing. Source explains
known mechanics without assigning unsupported meanings. Numeric unresolved targets
enter unknown bytes or need switchable-bank contracts. Details and bounded next
checks: commentary/uncertainties.md; existing disassembly handoff/evidence remain
applicable. Unknown complement still analysis/evidence/round-001/unknown-ranges.json.

No toolkit edits, ROM/map changes, deletion/cleanup, archive or publication.
