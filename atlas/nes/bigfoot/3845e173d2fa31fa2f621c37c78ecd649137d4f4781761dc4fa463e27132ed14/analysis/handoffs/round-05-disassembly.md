# Round 5 disassembly handoff

Status: complete. The pass started from pass-004, restored analysis/state.json and did not re-baseline.

Checkpoint **pass-005** (maps/pass-005.json, generated/pass-005, analysis/pass-005-verification.json):
- map.validate ok
- byte_identical true
- map sha256 184532e6...

ROM classification: 100.00% (262160 / 262160 bytes classified; 0 bytes unknown).

Kinds are unchanged:
- Code 37,907 (26,456 Observed, 11,451 Static)
- Data 93,181
- Image 131,072

There are 1,401 regions. No emulator runs were needed.

## Lead outcomes (all verified)
| Lead | Map change (16 KiB bank:CPU, phys) |
| --- | --- |
| L-027 | New region 2025 **MulTable16x16**, 3:$88B5-$89B4 ($0C8C5-$0C9C4, 256 B), where T[a*16+b] = a*b. It is read as `lda $88B5,x` at 3:$8517/$8579/$8597/$859E with X = half*16 \| MetaCount. Region 766 is now $88AB-$88B4, the last 5 RowPpuOffsetsB words. Regions 1814, 767 and 1815 were removed. Region 768 now starts at $89B5 (RaceBgPaletteInit, 1858 B). |
| L-026 | Region 1725 is now **MapTruckPath**, 0:$9072-$90B1 ($01082-$010C1, 64 B = 8 stops x 8 tiles). The guard is `cmp #$08`/`bcc` at 0:$8EA6 and the reader is `lda $9072,x` at 0:$8EDB. Region 664 is now $8FFE-$9071. New generic region 2026 covers $90B2-$90EC (59 B, role unresolved). |
| L-025 | Region 689 is now $BD58-$BE2F (debris tables, two unread 24-byte tables at $BDE8/$BE00, and SparkVelX). Region 1742 is **SparkVelY** $BE30-$BE47. New region 2027 is **SparkStartX** $BE48-$BE5F. Region 690 is **SparkStartY** $BE60-$BE77. The Y domain is 0..23 (ldy #$0B/#$17). The truck-1 tail $BE6F-$BE77 was never observed being read; its extent rests on static evidence. |
| L-028 | This lead was commentary-only. The map evidence never made the false claim. The evidence for regions 932 and 938 now names the readers: `ora $C723,y` at F:$C72B (masks 01/02/04/08) and `lda $D450,y` at F:$D514 (offsets 0/6/12/18, y = 0..3). |

Evidence:
- analysis/redirect/r04-redirect-a1/tools/checks.py (passes)
- analysis/disasm-r05/plan.json and tools/plan.py / tools/apply.py

## For commentary round 5
The map labels MulTable16x16, MapTruckPath, SparkVelY, SparkStartX and SparkStartY are now map symbols.

Commentary needs to do the following:
- Reconcile these labels with the existing MetatileTileIndex and the other commentary names. MetatileTileIndex should be renamed or redescribed as the product table.
- Remove the "consumer not identified" notes at 3:$894D, $8975 and $8991.
- Fix the MapTruckPath wording (8 x 8 tiles per stop, not "four per frame"). Move the "map/story data unresolved" note to $90B2.
- Drop the note "Nine bytes ... unresolved" at 0:$BE6F. Note that SparkVelY spans the old $BE40 split.
- Name or fix the reads at LaneNametableHi+2 and TileReplaceLaneHi+2 (L-028).
- Update uncertainties.md: remove 0:$BE6F, 0:$9087-$90B1, 3:$894D/$8991 and the "unread tails" entry.

## Open (non-classification)
- Roles of 0:$90B2-$90EC and of the generic blocks listed in round-04-redirect.md.
- The unread tables at 0:$BDE8 and $BE00.
- Natural reachability of the Static code islands.

The round-4 redirect expected the next redirect to be terminal, and no lead was rejected in this pass.

Toolkit: no new issues (TK-001..TK-006, analysis/toolkit-issues.md).
