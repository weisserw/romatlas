# Round 6 / disassembly handoff

## Scope
One disassembly pass on pass-005 that applied the three ready redirect leads, R5-L034..R5-L036. These change only
boundaries and roles (Padding -> Data). No bytes changed classification and no code changed.
- analysis/state.json was restored. The baseline was not re-created.
- input/game.nes is unchanged, and there were no toolkit edits.
- I used the binary named in workspace.json through a temporary socket bridge. Scratch dir /root/rascratch-r6d has
  been removed and my server stopped. No other process was touched.
- No gameplay: no lead needed runtime evidence, and S01-S16 already cover the milestones.

## Checkpoint
**pass-006**:
- maps/pass-006.json (sha256 dd3b5f7bfcc2f7fc50246ea98d210c2ed528ce68dd03bee8bcfad56c5b274a6c)
- generated/pass-006
- analysis/pass-006-verification.json

## Verification
- map.validate ok.
- analysis/state.json == maps/current.json == maps/pass-006.json (same sha256).
- `run.py checkpoint pass-006`: ok=true, byte_identical=true.
- Byte-wise comparison with pass-005:
  - no overlaps;
  - the classified byte set is identical;
  - every Code region is identical;
  - the only kind change is Padding -> Data for 2,973 B.

ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).

Per bank (code / data / padding / unknown):

| Bank | Code | Data | Padding | Unknown |
|---|---|---|---|---|
| b0 | 3 | 16090 | 291 | 0 |
| b1 | 3 | 16142 | 239 | 0 |
| b2 | 1546 | 14407 | 431 | 0 |
| b3 | 4689 | 9425 | 2270 | 0 |
| b4 | 10102 | 5625 | 657 | 0 |
| b5 | 7124 | 8085 | 1175 | 0 |
| b6 | 3023 | 11845 | 1516 | 0 |
| b7 | 11436 | 4581 | 367 | 0 |
| header | 0 | 16 | 0 | 0 |

Regions went from 2,946 to 2,636 (1,614 code, 961 data, 61 padding). Observed code is unchanged at 25,671 B.

## Findings
Details are in analysis/investigation.md, "Round 6". The files are in analysis/tools/round06/:
- apply.py: the script, with ROM-derived assertions and a `--dry` invariant check;
- applied.jsonl and changes.json;
- new-regions.txt: every new region id and its range.

- **R5-L034 verified.** All bank-2 scene pointers, blocks and attribute rows matched the recipe. $BB1F-$BB4A stays
  generic data at confidence 0.7: it has no pointer and no valid header.
- **R5-L035 verified, including the optional re-split.**
  - I recomputed the graphics-set extents from the ROM descriptor tables. They equal all 45 GfxSet declarations in
    game.asm.
  - Each of the 39 distinct graphics extents and the 6 nametable pictures is now exactly one Data region.
  - All 65 Padding regions inside them (2,973 B) are now Data.
  - Regions 1877 and 1926 were narrowed to exactly the descriptor tables (b0 $8010-$809B, b1 $8010-$8037).
- **R5-L036 verified, with two corrections:**
  - The DPCM table record 5 ($FD40-$FDF0, 177 B) ends one byte after DpcmSample1 ($FC40-$FDE0). The map therefore has
    one region for their union, $FC40-$FDF0. The commentary's "lies inside" is wrong.
  - B2_MenuCursorXTable is 2 bytes, not the 4 bytes stated in the old region evidence.

## Leads
Consumed and verified: R5-L034, R5-L035, R5-L036. No new leads were produced.

## Regions changed (for commentary carry-forward)
Every new region is Data/Static. The commentary labels are already in game.asm, so carry them to the new edges.

**Bank 2.** The scene ids 3257-3269 replace 2054, 2055-2064 and 2796-2805.

| Range | Content | Name |
|---|---|---|
| $B0FD-$B13A (3270) | palette-block pointers, three 16x1 blocks, cursor X table (2 B) | B2_TitlePaletteBlocks; B2_MenuCursorXTable at $B139 |
| $B13B-$B156 (3271) | title metasprite | B2_TitleMetasprite |
| $B6EF-$B6FE (3257) | scene pointer table | B2_AttractScenePtrs |
| $B6FF-$BACE (3258-3265) | 8 picture blocks, one region each, 122 B | B2_AttractScenePictures (scenes 1u, 1l, 0u, 0l, 2u, 2l, 3u, 3l) |
| $BACF-$BAD6 (3266) | attribute pointers | B2_AttractSceneAttrPtrs |
| $BAD7-$BB1E (3267) | 3 attribute rows | B2_AttractSceneAttrRows |
| $BB1F-$BB4A (3268) | generic data, unpointed | none |
| $BB4B-$BB62 (3269) | scene-3 attribute row | none |

- Regions 2679 and 2046 were removed (replaced by 3270-3271).
- Graphics sets: ids 3283-3304 (bank 0), 3306-3313 (bank 1) and 3314-3322 (bank 2). Each region equals one GfxSet
  extent; shared extents carry all their set names, e.g. $01/$04.

**Bank 3.** NametablePicture0-5 are ids 3323-3328, one 1024-B region each, $A480-$BC7F.

**Bank 6.**

| Range | Name |
|---|---|
| $9F6B-$9F7E (3272) | B6_InstrumentPtrs |
| $9F7F-$9FFF (3273) | B6_InstrumentData |

Region 2861 now starts at $A000.

**Fixed bank.**

| Range | Name |
|---|---|
| $CE2E-$CE55 (3274) | VramStructLists |
| $CE56-$CF51 (3275) | ChipPanelVramStructs |
| $ECDE-$ECE7 (3276) | RouletteSpriteSetPtrs |
| $ECE8-$ED05 (3277) | RouletteSpriteSets |
| $ED06-$ED0D (3278) | FadeMaskTable |
| $FC00-$FC30 (3279) | DpcmSample0 |
| $FC31-$FC3F (3280) | gap, generic |
| $FC40-$FDF0 (3281) | DpcmSample1, plus the record-5 sample at $FD40 |

Region 2456 is now $FDF1-$FDFF.

Commentary should:
- remove the "Part of X / phase-1 windows do not follow" notes for these structures;
- describe the formerly-Padding runs as blank tiles/cells of the graphics set or picture.

## Toolkit
analysis/toolkit-issues.md: no new toolkit issues found. The TK-005 and TK-006 workarounds were used again.
