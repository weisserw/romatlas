# Round 5 / redirect handoff (attempt r05-a1)

Status: **complete, proposal ready** (not terminal). The 3 leads are boundary/role corrections only (0 classification bytes).

Checkpoint: pass-005 (maps/current.json == analysis/state.json), 131,088/131,088 classified, 0 unknown, unchanged.
Map, ROM, source, state and toolkit not modified. Research: analysis/redirect/r05-a1/ (findings.json, b2scene.txt,
padin.txt, padfix.txt, structcut.txt, innercuts.txt, smallstructs.txt, padread_roulette.json, tools/). Scratch
/root/rascratch-r5r removed; my server stopped.

## Scope
1. Bank-2 $B6EF.. scene-block bounds versus map regions.
2. Pre-acceptance consistency check: commentary-decoded structures whose region edges cut records, plus Padding
   inside decoded structures (new, systematic: every `Name ($A-$B)` / `Name ($A, N bytes)` declaration in game.asm
   (83 structures) compared with map region edges and kinds). Reused r04-a1 control-edge/pointer scans (no change).
3. Gameplay milestone review.

## Findings
1. **Bank 2 scene blocks exact** (R5-L034). $B6EF-$B6FE: 4 records of [upper, lower] word pointers. $B6FF-$BACE:
   eight BuildVramStrings blocks [w=$14, h=6, 120 tiles] = 122 B each, all 8 pointed (scenes 1,0,2,3 in address
   order). $BACF-$BAD6: 4 attribute-row pointers. Rows [22,1,22 B] at $BAD7, $BAEF, $BB07, $BB4B; $BB1F-$BB4A (44 B)
   is unpointed and has no valid header (generic data). Consumer: B2_AttractDrawScene $B2B5 -> BuildVramStrings
   $E502. Map: region 2054 (20 B) runs from the pointer table 4 B into block 0; 20 phase-1 32/64-B windows cut every
   block. Kinds are correct.
2. **Padding inside graphics** (R5-L035, the most significant): 65 Padding regions, 2,973 B (uniform $00/$FF runs
   from the phase-1 rule) lie fully inside GfxSet* extents (descriptor tile counts, R1-L009) or NametablePicture0-4
   (fixed 1024-B copies, R1-L017). They are uploaded blank tiles/cells: role Data, not filler. None straddles a
   structure edge. Also, 40+ GfxSet/picture starts are not region edges (R1-L009 annotated only unknown bytes).
3. **Other cut structures** (R5-L036): B2_TitleMetasprite (edge $B13D instead of $B13B), B6_InstrumentData
   (window crosses $A000), ChipPanelVramStructs (5 mid-struct edges; 20 structs tile $CE56-$CF51), RouletteSpriteSets
   (edge $ECFE mid-record; window spans FadeMaskTable), DpcmSample1 (last byte $FDE0 in the next window). The other
   36 structures with interior edges are GfxSet/Nametable/bank-2 (covered above) or "Contains" tables inside one
   region (not cut).
4. **BuildMetasprite zpArg2 "padding" bases** (editorial, no lead): StoreAttr reads (zpArg2),Y with Y = the
   arrangement byte, so zpArg2 is a per-tile-code attribute lookup base; the bytes read are base+code. A bounded Read
   trace (roulette_start, 60 frames, b5 $A60F+512) saw one read at $A77E (Data), none in padding. Final editorial
   should describe zpArg2 so; immediates may stay numeric.
5. **Gameplay**: S01-S16 cover title, story, travel, all three games, password, debug menu, final match, natural ending
   (S13), credits (S12 artificial) and game over (S16). No major milestone is missing; S15 stays deferred.

## Ready leads (analysis/leads.json)
| ID | Target | Recipe |
| R5-L034 | b2 $B6EF-$BB62 (phys 46847-47986) | `b2scene.py input/game.nes`; regions $B6EF+16, 8x122 from $B6FF (or one 976 B), $BACF+8, $BAD7+72, $BB1F+44 generic, $BB4B+24 |
| R5-L035 | 65 Padding ids in padfix.txt (b0-b3) | `padfix.py commentary/source/game.asm maps/current.json` (expect 65 / 2973 B, all inside); Padding->Data, preferably one region per GfxSet/picture (r01 gfx.py extents) |
| R5-L036 | b2 $B13B, b6 $9F7F/$A000, fixed $CE56-$CF51, $ECE8/$ED06, $FDE1 | `smallstructs.py`, innercuts.txt; re-split Data windows at the record edges |

## Rejected / deferred
- Bank-6 $A14D-$BDE7 and bank-5 $85A8/$85D2 Padding runs: they sit between unreferenced generic data with no
  decoded consumer or structure. The missing fact is a reader, so there is no basis for a role change.
- BuildMetasprite read domain sweep: tracing runs about 3 frames/s, and the result could change only padding kinds
  at unknown base+code offsets. Diminishing prospects; editorial note only (finding 4).
- Code/data conflicts: r04-a1 ctlcheck/immpair results still apply. No map change since then affects code.

## Recommendation
Run a short **disassembly round 6** that applies R5-L034..L036 as one batch: map edges/kinds only, 0 B, with
labels already in the commentary. Then refresh the commentary for the touched regions, then final editorial
acceptance. R5-L035 corrects about 3 KB of decoded graphics content that is misroled as filler in the map. Without
these leads, the remaining residuals are editorial (see round-05-commentary.md).
