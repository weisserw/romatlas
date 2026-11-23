# Round 6 / redirect handoff (attempt r06-a1)

Status: **complete, proposal ready** (not terminal). Four ready leads, R6-L037..R6-L040. They are role and boundary
corrections only: 0 classification bytes change, and 1,338 B in 45 Padding regions become Data.

Checkpoint: pass-006 (maps/current.json == analysis/state.json, sha256 dd3b5f7b...4a6c), 131,088/131,088 classified,
0 unknown. Unchanged: map, state, ROM, commentary source and toolkit.
Research: analysis/redirect/r06-a1/ (findings.json, wmdata.txt, attrtables.txt, b5msg.txt, b6period.txt,
wmread_map_all_defeated.json, rdtrace_b4travel_negative.json, tools/). Scratch /root/rascratch-r6r has been removed and
my server stopped.

## Scope
- I looked for structural misroles that the round-5 structure scan could not see, because their consumers were not
  declared as structures in the commentary.
- Fixed-bank absolute operands into the switchable slot (tools/fixedrefs.py).
- Mechanical `*_Data_*` and `*SpriteAttrLookup_*` symbols.
- The Padding regions that remain (61), each checked against its readers and neighbouring structures.
- The weak claims in the coordinator handoff.

## Findings
1. **The bank-6 world-map data was misattributed (R6-L037, the main finding).**
   - symbols.asm calls Fixed_Data_A1FD..Fixed_Data_BDBB "bank 7 table". In fact the fixed-bank WorldMap* routines read
     them from bank 6, which MapWorldMapBank maps (zpBankRequest/zpBankHome = 6).
   - As a result, the bank-6 data notes wrongly say that no operand points into them. Round 5 also deferred the
     bank-6 Padding runs as "no reader".
   - $A000-$BDE6 is one contiguous chain of 20 structures, each anchored by a consumer: marker data, a 176-entry
     attribute table, plane/position tables, two 6x6 portrait layouts, pointer tables, and 14/15-B VRAM string records.
     It also holds 11 x 544 B portrait CHR and 11 location palettes. Every pointer/record/stride assertion passes.
   - The 34 Padding regions inside (944 B) are attribute zeros or blank CHR bytes.
   - A bounded read trace (map_all_defeated snapshot, about 570 frames) saw 1,514 bytes read across 13 of the
     structures, all with prg_banks [12,13,14,15]. It read 8 Padding regions in full as the CHR upload source.
2. **Sprite attribute lookup tables (R6-L038).** BuildMetasprite uses zpArg2 + tile code. Five bases have independent
   end bounds:
   - b5 $A60F: 256 entries, ending where the next base $A70F starts.
   - b5 $A70F: ends before the observed code at $A7FF.
   - b5 $BAC8: ends before the anchored $BAEA. Codes $00-$21 read 32 of its 34 bytes.
   - fixed $DFD8: 256 entries, ending before the observed code at $E0D8.
   - b4 $BD60: ends at the last statically read cell, $BDDF.

   Six Padding regions (286 B) are cells of these tables. This corrects r05 finding 4 ("editorial only").
3. **Bank-5 message-window lists (R6-L039).**
   - B5_UpdateBetDisplay reads the pointer table $844D. It, the lists and the [rows,width,tiles] blocks decode with no
     gap from $844D to $86BC.
   - Padding $85A8 and $85D2 (80 B) are the $FF tile payloads of two pointed 4x10 blocks.
   - This corrects the r05 deferral ("missing reader").
4. **B6_PeriodTableHi is 65 entries (R6-L040).** It runs $8C31-$8C71 and ends at the DMC table. The 28 Padding bytes are
   the zero high bytes of the semitone series.

## Ready leads
| ID | Target | Recipe | Padding->Data |
|---|---|---|---|
| R6-L037 | b6 $A000-$BDE6, 103 regions | `tools/wmdata.py input/game.nes maps/current.json`; one Data region per structure; commentary renames the Fixed_Data_* symbols to bank-6 labels and symbolizes #$3B/#$61,#$A2 | 944 B (34 ids) |
| R6-L038 | b5 $A60F-$A7FE, $BAC8-$BAE9; fixed $DFD8-$E0D7; b4 $BD60-$BDDF | `tools/attrtables.py`; one Data region per table | 286 B (2263, 2269, 2310, 2439, 2227, 2229) |
| R6-L039 | b5 $844D-$86BC | `tools/b5msg.py`; re-split into the pointer table, lists and blocks | 80 B (2241, 2242) |
| R6-L040 | b6 $8C31-$8C71 | `tools/b6period.py`; Hi table as one region | 28 B (2317) |

## Rejected / kept
These have no new evidence, or the evidence supports the current role:
- **b4 $BB30-$BB31 Code 0.5.** Accepted under R3-L027, and nothing new has been found. The competing reading (a
  2-byte orphan) is still neither supported nor disproved.
- **b2 $BB1F-$BB4A.** Stays generic data: it is unpointed and repeats row 3.
- **b7 $FBA0-$FBFF.** This is zero fill before the 64-aligned DPCM sample at $FC00. Its only possible reader is the
  inferred, never-run overrun to ChrRowAddrTable+32.
- **b3 $9C69-$A47F.** This is fill before the page-aligned NametablePicture0.
- **Bank tails, the $x003 stub gaps and b4 $BDE0+.** They lie beyond the last read cell.
- **$DFD8 read cells.** These were not derived statically (the headers are built in RAM). The claim rests on the extent
  bounds alone, at confidence 0.8. The reviewer may accept the other tables without it.
- **Travel-scene trace.** It did not reach the travel scene (0 reads), so I drew no conclusion from it. The static
  evidence is sufficient on its own.

## Remaining continuations (deferred, with reasons)
- Decoding the meanings of the world-map PlaneTableA/B fields (they look like distance and step pairs). This is
  semantic only and has no structural effect.
- Commentary meanings carried from round 6 (blackjack statuses, tier bytes, reaction sprites). These are editorial or
  semantic; none changes structure.
- After these leads, about 16 Padding regions remain. All are bank tails, stub gaps or alignment fill with a supported
  fill role.

## Recommendation
Run **disassembly round 7** to apply R6-L037..L040 as one batch (map edges/kinds only, 0 classification bytes). Then
refresh the commentary for the touched regions: the bank-6 world-map labels are the largest editorial change. Then
final editorial acceptance.
