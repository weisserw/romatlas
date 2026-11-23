# Round 2 redirect handoff (attempt r2-a1)

Outcome: READY LEADS for another disassembly/commentary round: L-008 (high), L-007 (medium), L-009 (medium, commentary only).
This redirect changed only research and lead records. ROM, map, state, source, commentary and binary were not touched.

## Checkpoint identity
- Input: maps/round2-disasm.json = maps/current.json = analysis/state.json (sha256 6b48e02b...e055), 928 regions.
- Source: commentary/source/game.asm (round-2 commentary, byte-identical per commentary/verification.json; not modified).
- ROM classification: 100.00% (131088/131088). Code 26,288, Data 93,050, Padding 11,750. The unknown complement is empty.
- Proposed vs accepted:
  - L-008 proposes 86 bytes Data -> Code (7:$CFCF-$D024). Code would become 26,374 and Data 92,964.
  - L-007 adds labels and splits without changing the counts.
  - Nothing has been accepted yet; disassembly reviews the proposals.

## Methods
- Bank-aware static scans with new tools in analysis/redirect/r2-a1/tools/:
  - refs.py: absolute and indexed operands.
  - ptrs.py: LDA/LDX/LDY immediate lo/hi pairs within 5 instructions. This catches X/A PrintNumber tables and Ptr1F set-ups that the r1-a1 scans missed.
  - ctx.py: aligned disassembly context.
  - dis.py, hx.py: decode and hex helpers.
  - splits.py: builds and tiling-checks splits.json.
- No runtime experiments. Open questions 1 and 3 were already answerable from the source and from existing r1-a1 captures.

## Results
1. **Question 2 ($F3DE-$F8F7) answered.** The region splits as follows:
   - $F378/$F388: AttrShadowPtr0/1, 8 words each.
   - $F398-$F39F: 8 unreferenced bytes.
   - $F3A0-$F3DB: NametableRowAddr. It is 60 bytes, not 62; index is (29-row)*2.
   - $F3DC-$F417: an unreferenced descending nametable-1 row table.
   - $F418-$F8F7: **CardFaceTiles**, 52 cards x 24 tiles (4x6). BuildCardTiles sets Ptr=$F418+card*24 and DrawRandomCard limits card to 0-51. The table ends at code $F8F8.
2. **Bank 6 $B0EB-$BAC7 (2,525 bytes) fully tiled.** It splits into 53 consumer-anchored records (splits.json). Bounds come from:
   - loop limits;
   - modulo wraps: WrapWheelIndex mod 38, reel wraps 11/40/44;
   - finite selectors: MessageFrame 0-2, CoinMessageTimer 1-4, CoinsInserted 1-3, per-kind MachineDenom domains, HandMessageIndex 0/$12;
   - pointer immediates.

   Commentary equate sizes corrected: RouletteWheelFrames 196, SlotMessageFrames 210, FourReelCoinTiles 12, ReelSymbolTilesA 48, ReelSymbolTilesB 64, HoldCardTiles 24.

   New records: RouletteWheelNumberTiles (38x8), five 11-byte PrintNumber digit tables, three 32-byte coin-message frame sets, two coin-marker sets, two denomination texts, and two 6-byte chip-glyph sets (ChipGlyphPtr).

   Five spans have no consumer (188 bytes) and remain generic: $B554 (20), $B59C (12), $B5CC (36), $B678 (60), $B754 (60).
3. **Bank 7 $D6FD-$D9CA split.**
   - New FourReelStrip0/1 at $D70F/$D71A (11 entries each, from FourReelStripPtrs).
   - New ReelStripB0-2 at $D73B/$D74F/$D763 (20 entries each, from ReelStripPtrsB).
   - $D777-$D8BC (326 bytes) has no consumer.
   - The existing PowersOfTen, ReelStripA, ReelStopTableB and RandomSeedTable extents are confirmed.
4. **7:$DCF0-$DD7F is a 144-byte ClockHourTable**, not the 30-byte "ClockSecondsTable". The index is ClockHours*6 + (ClockMinutes mod 10), and the table ends at code $DD80. 7:$DEC8 computes mod 10, not /10, so hour 23 can read up to 4 bytes past the table. This looks like an original bug.
5. **Code-vs-data correction (L-008).** The MetaspriteTable region $CFB5-$D024 holds only 13 pointers ($CFB5-$CFCE). The remaining 86 bytes decode without gaps into two coherent routines that use PrintText/PrintNumber RAM: advance text cursor ($CFCF) and print one character ($CFEE). The run ends with JMP $CFCF exactly at the region end. No reference was found, so these are Static unreferenced code.

   Their position/width tables $D0C0/$D0CC/$D0D8 sit in the $D084 block. That block also holds the CharToTileTable entries for chars $1F-$5A, 12 unreferenced bytes, and an unreferenced ascending row-address table.
6. **Commentary facts (L-009).**
   - Songs $81/$82 are started by RebuildFloorWithMusic ($81 + RandomCursor&1).
   - Sfx $05-$09 are per-coin-count insert sounds (coins+4).
   - MusicRestartId is only ever written with 0.
   - Screens $10-$15 are entrance pictures with vehicles (r1-a1 shots).

## Ready leads (recipes in analysis/leads.json)
- L-008 (high): annotate 7:$CFCF-$D024 as Code (Static 0.8, unreferenced), shrink MetaspriteTable to $CFB5-$CFCE, split $D084-$D12B.
- L-007 (medium): apply the 76 records of analysis/redirect/r2-a1/splits.json. Run `python3 analysis/redirect/r2-a1/tools/splits.py` (it asserts tiling), re-check each cited consumer with `ctx.py`, then map.remove/annotate and verify the checkpoint.
- L-009 (medium): commentary corrections only.

## Residuals and deferred avenues
- **Metasprite definitions 7:$C049-$C286 (574 bytes).** The 13 start pointers are now exact. Frame extents still need the per-writer frame stride: which DrawSprite1x1/2x2/3x2/5x3 writer uses each SprDef, and the SprFrame multiplier. The data visibly holds 30-byte 5x3 frames (15 tiles + 15 attributes $00/$40) from about $C0D7. Next step: map each `lda #n / sta SprDef` call site to its writer, then partition $C049-$C286 by tiles x 2 x frame count. This was not done for context budget. It is a credible next redirect or disassembly item, not exhausted.
- **Screen $02.** It has no literal caller, and the InfoScreen computed ids cover $18-$1D. Recipe: a bounded runtime watch of A at the ScreenDecode entry, or an artificial decode with one capture. This is semantic only, since ScreenData02 is already typed.
- **6:$85CE (32) and 7:$E195 (15).** r2-a1 rescanned with the immediate-pair scan as well and again found no consumer. The negative result covers absolute/indexed/immediate-pair references only, so both stay generic.
- **Five spans with no consumer remain generic** (582 bytes in total, listed in splits.json). The $B678/$B754 60-byte 5x12 blocks parallel the per-kind slot text sets and may be unused win texts. No reader exists, so no role is claimed.
- **Slot symbol pictures.** Symbol tile records are now bounded (FourReelSymbolTiles 5, ReelSymbolTilesA 6, ReelSymbolTilesB 8). Naming them from the r1-a1 pay-table shots is commentary work.
- **6:$8000 leader byte and the 675-byte residue.** No new evidence; unchanged.
- **7:$E000-$E004 (5 bytes).** No consumer found.

## Disk / hygiene
Scratch dir removed. The new evidence is about 30 KB (splits.json plus tools). About 3.2 GB free.
