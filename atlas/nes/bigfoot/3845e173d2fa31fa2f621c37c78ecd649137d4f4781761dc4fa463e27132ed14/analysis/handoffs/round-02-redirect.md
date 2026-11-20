# Round 2 redirect handoff (attempt r02-redirect-a1)

Status: COMPLETE. The input checkpoint is pass-002 (100.00% classified, 0 unknown bytes). The map, source, ROM, state.json and toolkit were not changed; this round changed research and lead records only.
- Runtime sessions used `romatlas serve --rom` without `--store`.
- Where a RAM edit was used, it is listed with the result. All other input was ordinary joypad input.
- Evidence is in analysis/redirect/r02-redirect-a1/:
  - findings.json (G1-G14)
  - runs.json (parameters of every traced run)
  - static-hits.json
  - tools/ (rpc.py, cov.py, covcmp.py, scripts.py, autoplay.py; run with `python3 -I`)
  - frames/
- Snapshots added to traces/snapshots: snap-2p-race, snap-hill-start, snap-race-stop4, snap-secret-stop7, snap-nat-stop7-shop.

## Scope
1. Critical review of the weak classifications.
2. The bounded runtime checks that commentary asked for.
3. Broad gameplay: L-007 and L-008.

Static work was done on the source and ROM; runtime work used 11 bounded traced runs plus untraced play.

## Findings
- **G1 Bank 0 metatile terrain map.** $9C55-$A844 is one table of 191 x 16 bytes: a 4x4 terrain-class grid for each metatile. Evidence:
  - The consumer is SampleRaceTerrainTruck. Its pointer is $9C55 + id*16 + cell.
  - The values run 1..42, which index the 43-entry TerrainEffectA-D tables. Each of those tables is exactly 43 bytes ($9BA9/$9BD4/$9BFF/$9C2A).
  - 191 matches the metatile count in bank 3, and the table ends exactly at the code at $A845.
  - In a race trace, 265 reads with banks [0,1] landed in it.
- **G2 Bank 2 RaceSpriteLayouts.** It is a 51-word pointer table at $9F51-$9FB6, followed by 50 layouts of 64 bytes at $9FB7-$AC36. Each layout is 32 (tile, attribute) cells.
  - Pointer 50 would point at $AC37 and overlap the code at $AC38.
  - Every producer of RaceObjSprite stays at 49 or below, so layout 50 is never selected.
  - In the 2P race, 100k layout-aligned reads with banks [4,5] were recorded.
- **G3 Bank 2 tail.** The palette scripts run to $B0B6. $B0B7-$B0BE is TitleRevealA's 4-word pointer list. $B0BF-$B0E6 holds two 20-byte tile rows for NmiHandler13.
  - The boot trace read $B0B7-$B0BE, $B0C0-$B0D2 and $B0D4-$B0E6, all with bank 2 mapped.
- **G12 Fixed bank $FE12-$FE35.** This holds four secret button sequences, laid out as 9 rows x 4 columns (one column per sequence, last row $FF).
  - Sequences 0, 1 and 2 at the title screen start the tour at stop 0, 3 or 5. SELECT then adds 0-2 stops.
  - Sequence 3 in the shop gives free parts once.
  - Confirmed with ordinary input.
- **G4 Hill climb.** HillSpecial takes the values 0..12 in ordinary hill-climb play. The HillNmiJump handlers (bank 6 $8DA6-$9028) executed 237 instruction starts with banks [12,13].
  - The stop itself was selected by the round-1 shop patch.
- **G5 Coverage.** 99 Static code regions (5,628 B) were fetched as instructions. No executed opcode fell outside a Code region in any run, so there is no code/data contradiction.
- **Runtime checks:**
  - G6: ScriptOp1D is never used. A static parse of all 71 scripts covers 1,302 of 1,302 bytes, and opcodes 00, 16, 17, 1D and 1F never occur. $0014-$001F were not written in 900 race frames. The `jsr $0014` is dead.
  - G7: NmiHandler2's `jsr $8592` runs in bank 0 on the map screen (180 of 180 fetches).
  - G8: DrawObjectVScroll `$88F8,y` runs with bank 5 mapped in the hill climb.
  - G10: PlaySoundEffect ids seen were {0,3,7,8,9}; none was 10 or higher. SfxRecords has 10 real records, then an all-zero record 10; record 11 runs into code. The 48-word table extent stays as it is.
  - G9: PalettePtrTable slot 29 = $0491 = PaletteBuf, the palette built in RAM, selected with ScreenPaletteId $1D. Slot 21 ($0000) is never selected.
  - G11: bank 5 $8388-$8777 is 640 zero bytes followed by two identical 192-byte $85/$B5 blocks. Its role is still unknown.

## Ready leads (all refine bytes that are already classified; 0 new bytes)
| Lead | Priority | Change |
| --- | --- | --- |
| L-018 | medium | Split region 675 at phys $01C65; role MetatileTerrainMap for $01C65-$02854; TerrainEffectA-D 4 x 43 B |
| L-019 | medium | Split region 758 at $09F61 and $09FC7; layouts through $0AC46; $0AC47 becomes a 1-byte generic region |
| L-020 | low | Split region 762 at $0B0C7, $0B0CF and $0B0F7 |
| L-021 | low | Provenance/evidence upgrade for the executed Static regions (static-hits.json) |
| L-022 | medium | Role/label SecretButtonSequences for region 962 |

The recipes are in analysis/leads.json. No new experiments are needed.

## Semantic notes for commentary
- Use G1 through G14 for names and comments:
  - MetatileTerrainMap
  - RaceSpriteLayoutPtrs / layouts
  - TitleRevealScriptPtrs / RowTiles
  - SecretButtonSequences and what each sequence does
  - SelectStartOffset (SELECT adds up to 2 stops)
  - HillSpecial reached naturally
  - the NMI handler banks
  - op $1D unused
  - palette slot 29 is RAM PaletteBuf
  - SfxRecords valid ids 0-10
  - the shop exit rule (G13)
  - bank 5 $8388 pattern
- Update commentary/uncertainties.md entries 1-5 to say they are resolved.

## Gameplay (analysis/scenarios.json)
- **Visited:**
  - Two-player mode: menu, map and 2P race.
  - HillSpecial sequence.
  - A later race (stop 4), patched, ending in '$BROKE!' and GAME OVER.
  - The secret stop-select.
  - The championship ending, reached naturally: stops 7 and 8 played with ordinary input only, through to the champion picture and back to title.
- **Not examined:** CPU forfeit/replacement and a finished 2P race. Neither has an unknown-byte prospect.

## Deferred avenues (reasons)
- **Bank 2 $B0E7-$B4A9 tail, 4:$A474-$A873 sample-like nibbles, 5:$8388 blocks, 2:$B4AA attribute-like bytes:** a static search found no consumer. These regions were not read in any of the traced title, race, event or ending runs. They are already generic Data, and no further method has a prospect of resolving them.
- **Bank 2 music format per song:** semantic only. Generic data is already sufficient.

## Rejection review (weak classifications; compact)
- **0:$B1E2 pla/tax (conf 0.5), 6:$A61E rts (0.5):**
  - The claim is that these are code residue between code.
  - Decisive evidence: no data consumer reads them (the round-1 global reference audit plus this round's coverage). They are surrounded by code, and the bytes match the txa/pha/pla/tax idiom and an empty rts stub.
  - The competing interpretation is data. Nothing supports it: no reader, no table structure.
  - Verdict: retain as code. No test discriminates further, because execution would require a caller that does not exist.
- **0:$8395, F:$D827 (0.6):**
  - These are if/else arms skipped by a preceding jmp.
  - The sibling arm F:$D822 executed in the hill climb, which confirms the routine's control flow.
  - Verdict: retain as code.
- **L-001 islands and the HillNmi handlers:** they keep their contracts. The HillNmi handlers are now observed (G4/L-021); the other islands are still unexecuted, which is consistent with debug/copy roles. No counterexample was found.
- **SfxPtrTable 48 entries:** the 48-word arithmetic progression has code on both sides, which bounds the table independently. Only the record semantics change (G10). Verdict: extent kept.
- The review exposed no code-to-data correction. Every proposal above is a boundary, role or provenance refinement.

## Recommendation
Run disassembly round 3 for the small correction batch (L-018 to L-022: region splits, roles and provenance). Then run commentary for the G-series notes and the final editorial audit. The project is not terminal, but after this batch the remaining work is only editorial.
