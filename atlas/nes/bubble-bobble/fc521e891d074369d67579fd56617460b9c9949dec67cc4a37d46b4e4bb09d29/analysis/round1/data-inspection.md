# First-pass non-code inspection

Physical offsets include the 16-byte header. Immutable input inspected directly;
all region annotations are applied through map.annotate with Static provenance.

PRG banks 0–6 each start with conspicuous repeated nibble-packed rows containing
00/11/22/33/44 and variants with single changed nibbles. The same row vocabulary
and repeated short rectangular layouts continue through their principal bodies.
Their later bytes contain repeated compact triples/records, coordinate-like values
and FF separators. Boundaries and every 8 KiB bank were inspected; byte frequency
summaries show 00/11/22/33/44 dominance plus FF. These patterns independently
support non-code. No instruction-fetch evidence occurs in these banks; absence
alone is not the rationale. Exact schemas, number of rows per level, pointer
consumers and original source grouping remain unresolved. Use generic Data,
not a stronger named table claim. Banks have the following maximal FF suffixes:

| Bank | Non-FF-body end (exclusive) | FF bytes |
| --- | --- | --- |
| 0 | 0x16DB | 2357 |
| 1 | 0x36DB | 2357 |
| 2 | 0x56D8 | 2360 |
| 3 | 0x76D8 | 2360 |
| 4 | 0x96DB | 2357 |
| 5 | 0xB6D8 | 2360 |
| 6 | 0xD6DB | 2357 |

Entire banks 7 (0xE010–0x10010), 9 (0x12010–0x14010) and 11
(0x16010–0x18010) are uniformly FF. Other unambiguous uniform suffixes:
bank 8 0x11903–0x12010 (1805), bank 10 0x15CDA–0x16010 (822), bank 13
0x1A5B8–0x1C010 (6744). Generic data suffices; no original padding purpose claimed.
Data annotation checks all ranges against accepted code intervals before insertion.

CHR 0x20010–0x28010 is independently defined by the iNES header as 32 KiB
pattern ROM (2048 planar 8x8 tiles), so Image is supported without requiring
all PPU indices to be selected. Header 0–16 and vectors 0x2000A–0x20010 have
independently bounded formats. No blanket data classification is made for
heterogeneous residuals in executable banks 8,10,12–15.
