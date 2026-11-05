# Remaining boundaries after hypothetical acceptance

Actual map remains unchanged: 21,076 unknown bytes. The authoritative proposal is `proposal-revised.json`; its 21,059 proposed unknown bytes leave these 17 bytes in ten intervals. This is a nonterminal proposal handoff, not an exhaustion or final completion decision.

| Physical offset | CPU interval | Bytes | Remaining ambiguity |
|---:|---|---:|---|
| 11661 | AD7D..AD7E | 1 | Extra RTS immediately after the RNG's own RTS; no independent functional distinction between unused return instruction and byte padding. |
| 16634 | C0EA..C0ED | 3 | Self-JMP after the text renderer's return; trap-code appearance alone does not distinguish a deliberate entry from incidental/padding bytes. |
| 20162 | CEB2..CEB5 | 3 | Self-JMP between an unconditional transfer and the palette block; same unresolved structural distinction. |
| 23310 | DAFE..DAFF | 1 | RTS after a software-stack helper's terminal JMP. |
| 23345 | DB21..DB22 | 1 | RTS after another software-stack helper's terminal JMP. |
| 23358 | DB2E..DB2F | 1 | RTS after a software-stack helper's terminal JMP. |
| 23380 | DB44..DB45 | 1 | RTS after a software-stack helper's terminal JMP. |
| 31864 | FC68..FC6A | 2 | Unofficial NOP plus RTS between instrument descriptors and the copy loop; could be an empty helper, alignment, or abandoned bytes. |
| 31885 | FC7D..FC80 | 3 | Apparent LDA $06 / LDX-immediate setup enters the accepted generic zero block at FC80. See boundary review below. |
| 31922 | FCA2..FCA3 | 1 | Isolated $20 after the accepted zero block and before music bytes; a standalone opcode interpretation cannot establish its role. |

The decisive bytes are in `residual-review.asm`, `residual-decodes.json`, `data-inspection.txt` and `narrowed-static-review.json`. These are not rejections based on missing callers. The reset veneer, DB95 wrappers and FC6A copy prefix were specifically promoted after combining independent endpoint, stack, PPU-register and loop-count constraints; earlier 46-byte residual records remain historical evidence.

FC7D boundary review belongs in this same disassembly batch: the apparent instructions are `LDA $06; LDX #$00`, with the immediate operand at FC80 currently classified as generic Data. The proposed 19-byte FC6A..FC7D tile-copy block matches EBB7 exactly and has independent hardware/loop semantics. Extending that prefix through FC81 would require splitting/reviewing the existing Data annotation, rather than silently treating its byte as both code and data. The research establishes this precise competing interpretation; it does not establish a complete intended routine or a return beyond the zero block. No arbitrary execution test would resolve original intent, and no natural caller is demanded as a prerequisite.

Other marginal stubs have no independently supported consumer, extended functional body, record role or discriminating runtime prerequisite after the source, parsed script calls, callbacks and adjacent contracts were inspected. The next phase can weigh these tiny claims independently; classification of the substantive batch does not require resolving them or proving them unused.
