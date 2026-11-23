# Round 2 redirect handoff (attempt r2-a2, continuation of r2-a1)

Outcome: READY LEADS (combined batch for one disassembly/commentary cycle): L-008 (high), L-007 (medium, amended), L-010 (medium, new), L-009 (medium, commentary only).
This redirect changed only research and lead records. The ROM, map, state, source, commentary and configured binary are unchanged. state.json is still sha256 6b48e02b...

## Checkpoint identity
- Input: maps/round2-disasm.json = maps/current.json = analysis/state.json (sha256 6b48e02b2ea8...e055), 928 regions, 100.00% classified (131088/131088). Code 26,288, Data 93,050, Padding 11,750.
- Source: commentary/source/game.asm (round 2, not modified).
- Proposed vs current bytes (nothing is accepted yet):
  - L-008 moves 86 bytes from Data to Code (7:$CFCF-$D024). Code would become 26,374 and Data 92,964.
  - L-007, L-010 and L-009 change labels, splits and comments only. Counts are unchanged.

## Methods (new in r2-a2; tools in analysis/redirect/r2-a2/tools/)
- **Static per-writer mapping of metasprites (msplits.py).** I mapped every `sta SprDef` and DrawSprite1x1At/Centered call site to its writer, piece count and SprFrame stride.
- **Runtime read-watch (watch.py, watch2.py).** I replayed the r1-a1 slot routes (machines Y=13,14,15,23) and the power-on to cashier route. Only Read events within one physical range were streamed, summarised, and then the stream was deleted.
- **Reader location (hunt.py/hunt2.py, then bsearch.py).** These save a snapshot every frame until the first watched read, then binary-search the CPU operation count to find the reading instruction.
- **Artificial screen decode (scr02.py).** I set A=2 at ShowCashStatus's `jsr ScreenDecode` (7:$DD99). Control run: A=$0C.
- All runtime states are artificial: they start from registers.set entries or a register patch. Results are in analysis/redirect/r2-a2/read-watch.json and shots/.
- Note: Read events include operand fetches and 6502 dummy reads, so a read is not automatically a consumer. Every claimed consumer below also has a static proof.

## Results
1. **Metasprite extents (deferred avenue 1, done; L-010).** All 13 SprDef values are immediates 0-$0C.
   - Record layout: each frame holds N attribute bytes followed by N tile bytes. Commentary currently says "tile numbers followed by"; that is wrong.
   - The 13 consumer-bounded records tile $C049-$C0D2 exactly. Each record ends at the next pointer.
   - Def 9 (wheel pointer) has 3 frames, but callers select only frames 0-1 (A&1 or 0). Def 12 (bounce ball) has 2 frames.
   - $C0D3-$C286 (436 bytes) holds 14 frames with variable piece counts: 15,16,17,13,17,16,15, then the same seven with attribute $40 and mirrored tiles. It ends exactly at code $C287.
   - No consumer reaches the 436-byte block: no SprDef value, pointer word, immediate pair or variable-count writer. It stays generic data, with a format note.
   - Strides: 3x2 uses frame*6, 4x2 and 5x2 use frame*8. These are smaller than their record sizes, so only frame 0 is meaningful, and all callers pass 0.
2. **Screen $02 (deferred avenue 2, done; L-009 f).** It is a developer placeholder text page: "Front of Caesar's palace with taxi and flashing lights. Lights will be made to flash by rewriting their char definitions." (shots/screen-02.png). No natural selector is known.
3. **The read-watch overturned 494 bytes of r2-a1 "no consumer" spans (L-007 amendments, analysis/redirect/r2-a2/l007-corrections.json).**
   - **$B678 was read.** bsearch located the reader at 6:$AD04 (ShowSlotWin): `lda (Ptr1F),y`, with Ptr1F taken from WinTextPtrs 6:$AD12. That table's words are $B5B4, $B678 and $B754, each a 60-byte 12x5 win text.
     - New records: FourReelWinText $B5B4-$B5EF, SlotWinTextA $B678-$B6B3, SlotWinTextB $B754-$B78F.
     - FourReelDenomText shrinks to 12 bytes ($B5A8-$B5B3). r2-a1's 36-byte extent was stride inference.
   - **$B59C-$B5A7 was read.** RefundFourReelCoins 6:$8E09 uses Y=(CoinsInserted-1)*4+$0C, so FourReelCoinTiles is 24 bytes (the commentary size was right). r2-a1's 12 was wrong.
   - **7:$D777-$D8BC (326 bytes) was read 7 times, at random offsets.** All 326 bytes are stop tables:
     - FourReelStripHi/Lo 6:$8ABD/$8AC1 point to $D777 and $D7B7. These are two 64-byte tables used by RandomReelStopFour (`and #$3F`).
     - ReelStripPtrHiA/LoA 6:$87A3/$87A6 point to $D7F7, $D839 and $D87B. These are three 66-byte tables used by RandomReelStopA (Mod66).
   - Cause of the r2-a1 miss: it scanned operands and immediate pairs, but not data-word pointer tables, and missed one +$0C offset path.
   - Effect: no-consumer bytes in L-007 scope drop from 582 to 88.
4. **L-008 sanity check (still ready).** No indirect or RAM-copied path reaches $CFCF or $CFEE:
   - The only `JMP (ind)` is the sound driver's, and its targets are all in bank 0 below $8460.
   - No code anywhere does a JSR/JMP below $8000.
   - The word $CFCF appears only inside compressed screen data (and the internal JMP).
   - The routines use $0308-$0312, which the shipped game uses as AttrShadow, so they look vestigial.
   - The structural code claim stands as Static and "statically unreferenced", not dead.
5. **Commentary facts added to L-009:**
   - (g) ShowStartupScreen computes X=StartupParam>>1 at 7:$E174-$E178 and never uses it. The table right after it, $E195, fits a sprite-0 Y table. This is inference only.
   - (h) Metasprite record order and frame strides, as in item 1.

## Ready batch (recipes in analysis/leads.json)
- **L-008 (high):** $CFCF-$D024 becomes Code (Static 0.8, unreferenced). Shrink MetaspriteTable to $CFB5-$CFCE and split $D084-$D12B.
- **L-007 (medium):**
  1. Apply r2-a1/splits.json, with the six bank-6 records replaced by l007-corrections.json (run `python3 analysis/redirect/r2-a2/tools/l007fix.py`; it asserts identical coverage).
  2. Replace $D777-$D8BC with the five stop tables listed in the L-007 recipe.
  3. Rename the stop-table pointer equates.
- **L-010 (medium):** run `python3 analysis/redirect/r2-a2/tools/msplits.py` (it asserts the tiling and the code boundary). Annotate 13 labeled records plus one generic 436-byte block.
- **L-009 (medium):** commentary corrections (a)-(h).

## Residuals and deferred avenues
- **6:$85CE-$85ED (32 bytes).** A different method this time: a runtime read-watch over all four slot machines found zero reads, not even operand reads. Data-word scans of banks 6-7 also find no pointer. It stays generic, and further prospect is low.
- **7:$E195-$E1A3 (15 bytes).** The only runtime read is the RTS dummy read at $E194 (bsearch), and no consumer exists. The vestigial sprite-0 table idea goes into commentary only. No further method is available.
- **6:$B554-$B567 (20 bytes).** It was never read in the slot routes, and nothing points to it. Its layout looks like two more 10-tile SlotWinTextFrames rows beyond MessageFrame 0-2, which is a comment hint only.
- **7:$F398-$F39F (8) and $F3DC-$F417 (60).** No operand, immediate-pair or data-word reference exists. These are nametable/attribute helper tables. A read-watch over text and floor screens is possible, but the prospect is low because no code builds pointers into $F3xx with an offset.
- **7:$C0D3-$C286 (436 bytes).** The format is documented (L-010). No writer can consume variable-count frames, and only an unknown external engine could. Its role (a mirrored 7-frame animation, tiles $34-$69) could be named only by rendering it, and no layout data exists for it.
- **7:$D0E4-$D0EF (12) and the $D0F0 row table (L-008 split).** Unchanged from r2-a1: no consumer.
- **7:$E000-$E004 (5 bytes), the 6:$8000 leader byte and the 675-byte static fragment residue.** No new evidence.
- **Not watched:** the read-watch covered only the slot routes and startup. Data in other games was not watched because r2-a1 already gives it consumer bounds.
- **Recommended next step:** apply the batch in round-3 disassembly. In the next redirect, add a data-word pointer scan (words stored in Data regions that point into Data regions) to the static toolkit scripts. That scan is what found three of the corrections here.

## Rejection review
Not applicable: the outcome is ready leads, not terminal.

## Toolkit
TK-005: `emulator.step unit:"instruction"` fails with "unknown step unit" on NES, although docs/protocol.md documents it. Workaround: omit unit and pass operations.

## Disk / hygiene
Stream files were deleted after summarising, and the scratch logs were removed. Evidence in analysis/redirect/r2-a2/: about 60 KB of JSON and tools plus 2 PNGs. The serve daemon was stopped.
