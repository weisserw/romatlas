# Round 4 redirect handoff (attempt r04-redirect-a1)

Status: COMPLETE. The input checkpoint is pass-004: 100.00% classified, 0 unknown bytes, map sha256 70b38672..., byte-identical.
- Only research and lead records changed. The map, source, ROM, state.json, scenarios and toolkit were not touched.
- Runtime work was 2 bounded Read-watch runs of 2,400 frames each, using `serve` without `--store` and no RAM edits. A 300-frame timing probe and two over-slow r03-style runs were dropped.
- Evidence is in analysis/redirect/r04-redirect-a1/:
  - findings.json (H1-H6)
  - runs.json
  - idxscan.json
  - tools/idxscan.py, tools/checks.py and tools/sparkwatch.py. Run them with `python3 -I`.
- Scratch /tmp/r4 is 68 KB, and there is 4.0 GB of free disk.

## What round 4 changed and why it reopens anything
The commentary found that MetatileRowOfTileRow (F:$D157) is read as `LaneTerrainBufOffset+2,x`. The operand's base sits *before* the table.

The r03 reference scan (refs.py) only accepted operands whose base was at or above the block start minus 1. So every "no consumer" disposition built on that scan had a systematic blind spot. That is a corrected premise under evidence-policy, not a repeat of the old method.

New method (tools/idxscan.py):
- Look at abs,x and abs,y operands in the same bank or the fixed bank whose base is in [block-255, block) and whose base region ends at or before the block.
- Then check each reader's index domain.
- A control run on F:$D157 recovers all 6 of its readers.

The other round-4 items (the LaneTerrainBuf RAM alias, RespawnSearchCount, symbolic constants, label renames) are RAM or editorial fixes. They add no caller, pointer or bank context for any ROM residual.

## Ready leads (all refine already-classified data; 0 new bytes; recipes in analysis/leads.json)
| Lead | Priority | Claim |
| --- | --- | --- |
| L-027 | medium | 3:$88B5-$89B4 (phys $0C8C5-$0C9C4) is one 256-byte 16x16 product table. Every cell satisfies T[a*16+b] = a*b. The previous table ends at $88B4 (last RowPpuOffsetsB word) and RaceBgPaletteInit starts at $89B5. ExpandMetatileTiles and ExpandMetatileAttrs read it as `MetatileTileIndex,x` with x = half*16 \| MetaCount, and an 8-bit X cannot pass $89B4. This explains 137 B that were "consumer not identified": 3:$894D and the first 36 B of $8991. The recipe merges regions 766-tail, 1814, 767, 1815 and the head of 768. |
| L-026 | low | 0:$9087-$90B1 (phys $01097-$010C1, 43 B) is the tail of MapTruckPath: 8 stops x 8 tiles. AnimateMapTruck rejects stops 8 and above (`cmp #$08`/`bcc`), uses x = stop*8 and copies 8 bytes. All 8 records follow the pattern (t, then t-1), and the pattern breaks at $90B2. The recipe splits region 1725 at phys $010C2. |
| L-025 | low | 0:$BE6F-$BE77 (phys $03E7F-$03E87, region 690) holds SparkStartY entries 15-23. StartRepairSparks uses Y = 11..0 for truck 0 and Y = 23..12 for any X other than 0, so all per-piece tables are 24 bytes. All 48 cells map consistently from DebrisVel to SparkStart. SparkVelY is 24 B, crossing the arbitrary split at $BE40. DebrisAttrs is read only for Y up to 23; $BDE8 and $BE00 are two unread 24-byte tables, which stay generic. Runtime: no truck-1 repair happened in either 2,400-frame race (1P and 2P, A held), so the tail was not observed being read. The extent rests on the static Y domain. |
| L-028 | low | Commentary only. The bytes after LaneNametableHi (F:$C723-$C726, bit masks read by SetTruckEndBit) and after TileReplaceLaneHi (F:$D450-$D453, row offsets read with y = 0..3) *are* read. The source comments and uncertainties.md wrongly say they are unread. |

Verification for all four: `python3 -I analysis/redirect/r04-redirect-a1/tools/checks.py $WS`. It asserts and needs no rerun of the emulator.

## Rejection review (evidence-policy; residuals after this pass)
Every round-3 review item was re-checked against what changed in round 4. The decisive artifacts were re-read: r03 findings.json, refs.json and runs.json, plus idxscan.json from this pass.

- **22 no-caller static code islands (650 B) and the low-confidence islands 0:$B1E2, 6:$A61E, 0:$8395 and F:$D827.**
  - Claim: code (retain).
  - Decisive evidence: r03 H1/H2 flow closure, plus the absence of any word, rts-trick or jsr/jmp reference.
  - Round 4 adds no reference. idxscan targets data reads, not control flow, and the indexed-base issue does not affect jsr/jmp targets.
  - Strongest competing interpretation: data. There is no reader or table structure, and the bytes decode contiguously between code with idiom matches.
  - Verdict: no discriminating test exists without a caller. Retain. The review premises are unchanged.
- **Unread generic blocks re-scanned with the corrected premise.** No indexed reader, or no reachable index domain, was found for:
  - 0:$8F84 (8 colour bytes)
  - 0:$90B2-$9180 (rest of the map/story area; the immediate-pair hits in r03 refs.json are coincidental `lda #$90/#$80` sound/delay constants)
  - 2:$83D4 (sound tables need indices of 31 or more; the note, pitch and channel nibble domains are 16, 16 and 6)
  - 2:$AC37 (L-019)
  - 2:$B059 (TitlePalettes needs an index of 204 or more against a domain of 3*16; slots 12)
  - 2:$B0E7 LegalScreenTiles (TitleRevealRowTiles rows are 20 bytes, per L-020)
  - 2:$B63D+, 3:$AF42, 4:$A474, 5:$8388, 5:$88F4, 6:$8400, 6:$8701, F:$E9AC, F:$FF2C
  - F:$FF57-$FFF9: the only hit is `$FF30,x` at F:$FECA, which needs an index of 39 or more, but the lane index is 0/1.
  - 0:$90B2-$9180: the map readers need indices of 64 or more, which their guards and loops exclude.

  The last two were checked in an extra run whose output was /tmp/r4/extra.json (scratch, not kept).

  These are already generic data, which the policy accepts. The competing code interpretation was rejected in r03 and remains so: no flow reaches them, and their content is zero runs or tile/colour/nibble patterns. Further watching gives only semantic gain with low prospect.
- **LegalScreenTiles reader.** There is no static reference, including the indexed-base window. It was unread in boot, title, race, 2P and ending runs. The title scenario shows no attract demo within about 820 frames (scenarios.json). A longer idle attract watch is the only remaining idea. It is semantic only, and its slow emulation (about 5 fps with a trace) gives it a poor cost/benefit. Deferred: diminishing prospect, and no structural question remains.
- **27 partly executed Static regions.** r03 H1 closure is unchanged. Retain.
- **Gameplay.** Coverage is unchanged (scenarios.json). A natural truck-1 crash and repair would observe the L-025 tail, but it did not occur in 2 x 2,400 frames. Forcing it would be artificial and is not needed for the static extent.

## Recommendation
Run one more short cycle. Disassembly applies L-027, L-026 and L-025: region merges and splits plus roles, 0 new bytes, 0 kind changes. Commentary then applies the renames and corrections from L-025 to L-028 and updates uncertainties.md: remove 0:$BE6F, 0:$9087-$90B1, 3:$894D/$8991 and the "unread tails" entry.

After that cycle, the coordinator may accept the next redirect as terminal unless disassembly review rejects a lead. The indexed-base scan has now been run over every unread generic block, and no other avenue in this review has a discriminating test.
