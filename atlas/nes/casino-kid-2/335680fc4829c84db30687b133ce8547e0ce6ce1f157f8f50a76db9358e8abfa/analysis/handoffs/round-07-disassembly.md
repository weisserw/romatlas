# Round 7 / disassembly handoff

## Scope
One disassembly pass on pass-006. It applied the four ready redirect leads, R6-L037..R6-L040, as one batch. They change
only region edges and roles (Padding -> Data). No classification bytes changed and no code changed.
- analysis/state.json was restored. The baseline was not re-created.
- input/game.nes is unchanged, and there were no toolkit edits.
- I used the binary named in workspace.json through a temporary socket bridge. Scratch dir /root/rascratch-r7d has been
  removed and my server stopped. No other process was touched.
- No gameplay: no lead needed runtime evidence, and S01-S16 already cover the milestones.

## Checkpoint
**pass-007**:
- maps/pass-007.json (sha256 bea35cf51c638a8dcaf88031c31dd19c638381859c04c007811a6f4a4fb6be7f)
- generated/pass-007
- analysis/pass-007-verification.json

## Verification
- map.validate ok.
- analysis/state.json == maps/current.json == maps/pass-007.json (same sha256).
- `run.py checkpoint pass-007`: ok=true, byte_identical=true.
- Byte-wise comparison with pass-006:
  - no overlaps;
  - the classified byte set is identical;
  - every Code region is identical;
  - the only kind change is Padding -> Data for 1,338 B.

ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).

Per bank (code / data / padding / unknown):

| Bank | Code | Data | Padding | Unknown |
|---|---|---|---|---|
| b0 | 3 | 16090 | 291 | 0 |
| b1 | 3 | 16142 | 239 | 0 |
| b2 | 1546 | 14407 | 431 | 0 |
| b3 | 4689 | 9425 | 2270 | 0 |
| b4 | 10102 | 5731 | 551 | 0 |
| b5 | 7124 | 8263 | 997 | 0 |
| b6 | 3023 | 12817 | 544 | 0 |
| b7 | 11436 | 4663 | 285 | 0 |
| header | 0 | 16 | 0 | 0 |

Regions went from 2,636 to 2,534 (1,614 code, 902 data, 18 padding). Observed code is unchanged at 25,671 B.

## Findings
Details are in analysis/investigation.md, "Round 7". The files are in analysis/tools/round07/:
- refs.py: an independent operand/immediate-pair scan over the map Code regions;
- apply.py: the script, with ROM and reference assertions and a `--dry` invariant check;
- applied.jsonl and changes.json;
- new-regions.txt: every new region id and its range.

- **R6-L037 verified.** Every consumer base was found independently in fixed-bank code (WorldMapSetup,
  WorldMapMovePlane, WorldMapArrive, WorldMapStatusFrame and its twin, WorldMapDrawLocationInfo, WorldMapLockedMessage,
  WorldMapLocationMessage, WorldMapUpdateTitle/PlaceCursor, WorldMapDrawPanel). MapWorldMapBank sets bank 6, which gives
  prg_banks [12,13,14,15]. All the chain assertions pass. Result: 30 Data regions, 944 B Padding->Data.
- **R6-L038 verified, including $DFD8 (conf 0.8).**
  - Its 256-entry extent ends exactly before observed code $E0D8, at base+255.
  - All the values are in {0..3}, in tile-group runs.
  - Only the fill-vs-cell role changed; the bytes were already non-code.
  - Its read cells are still not derived statically. The other four tables are at conf 0.85.
- **R6-L039 verified.** The pointer table plus the lists and blocks tile $844D-$86BC with no gap. The $FF blocks
  $85A6/$85D0 are pointed from two lists.
- **R6-L040 verified.** B6_PeriodTableHi is 65 entries, $8C31-$8C71.

## Leads
Consumed and verified: R6-L037, R6-L038, R6-L039, R6-L040. No new leads were produced.

## Regions changed (for commentary carry-forward)
Every new region is Data/Static. The commentary labels are at CPU addresses, so carry them to the new edges by address.

**Bank 6, world map (3329-3358).** All 103 old regions in $A000-$BDE6 were removed.

| Range | Id | Name | Commentary note |
|---|---|---|---|
| $A000-$A02B | 3329 | B6_WorldMapMarkerPtrs | |
| $A02C-$A133 | 3330 | B6_WorldMapMarkerSprites | |
| $A134-$A14C | 3331 | B6_WorldMapMarkerTiles | |
| $A14D-$A1FC | 3332 | B6_WorldMapMarkerAttrs | |
| $A1FD-$A210 | 3333 | plane table A | Fixed_Data_A1FD |
| $A211-$A224 | 3334 | plane table B | |
| $A225-$A23A | 3335 | location (Y,X) | |
| $A23B-$A260 | 3336 | portrait layout A | symbolize #$3B/#$A2 at $F4C9/$F684 |
| $A261-$A286 | 3337 | portrait layout B | symbolize #$61/#$A2 at $F4BE/$F679 |
| $A287-$A2B2 | 3338 | info pointers | |
| $A2B3-$A314 | 3339 | info blocks | |
| $A315-$A340 | 3340 | name pointers | |
| $A341-$A48A | 3341 | name strings | |
| $A48B-$A49A | 3342 | locked-message pointers | |
| $A49B-$A512 | 3343 | locked-message strings | |
| $A513-$A536 | 3344 | location-message pointers | |
| $A537-$A644 | 3345 | location-message strings | |
| $A645-$A65A | 3346 | portrait CHR pointers | |
| $A65B-$BDBA | 3347-3357 | portrait CHR, 11 x 544 B (one region per location) | |
| $BDBB-$BDE6 | 3358 | location palettes | Fixed_Data_BDBB |

Commentary should:
- rename Fixed_Data_A1FD..Fixed_Data_BDBB, which symbols.asm wrongly calls "bank 7 table", to bank-6 labels;
- drop the "no operand points into it" notes for bank 6.

**Attribute tables (R6-L038).**

| Range | Id | Name |
|---|---|---|
| b5 $A60F-$A70E | 3359 | B5_SpriteAttrLookup_A60F |
| b5 $A70F-$A7FE | 3360 | B5_SpriteAttrLookup_A70F |
| b5 $BAC8-$BAE9 | 3361 | B5_SpriteAttrLookup_BAC8 |
| fixed $DFD8-$E0D7 | 3362 | SpriteAttrLookup_DFD8 |
| b4 $BD60-$BDDF | 3363 | B4_SpriteAttrLookup_BD60 |

**Bank 5, message windows (R6-L039).** These replace Bank5_Data_844D/845F.

| Range | Id | Content |
|---|---|---|
| $844D-$845E | 3364 | pointer table |
| $845F-$86BC | 3365-3372 | one region per list with its following blocks: $845F, $846C, $84F1, $8540, $859D, $85FA, $8657, $8660 |

Record format: list = [count, count x (src word, col, row)]; block = [rows, width, tiles].

**Bank 6, sound (R6-L040).**
- B6_PeriodTableHi is $8C31-$8C71 (3373).
- Region 2316 was narrowed to $8BDC-$8C30 (NoteValueTableA + PeriodTableLo).

Describe the formerly-Padding runs as attribute-0 cells, blank CHR rows, $FF blank tiles and zero high period bytes.

## Remaining
18 Padding regions remain. All are bank tails, stub gaps or alignment fill (see the r06-a1 "Rejected / kept" list).
Semantic open items are unchanged:
- the meanings of the world-map PlaneTableA/B fields;
- $BB1F-$BB4A;
- the $BB30 caller;
- the "MONEY" $9291 reader;
- DMC records 5+.

Next: commentary round 7 (carry the labels above), then redirect/final editorial acceptance.

## Toolkit
analysis/toolkit-issues.md: no new toolkit issues found. The TK-005 and TK-006 workarounds were used again.
