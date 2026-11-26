# r04-a1 redirect notes (checkpoint pass-004; static only, no serve/emulator session)

Input: pass-004, 100.00% classified (262160/262160, 0 Unknown; unknown complement empty). Code identical to pass-003.
Map, state, source, ROM and toolkit not modified. Scripts in this directory run from the workspace root.

## Questions and results
1. Do the round-4 role-unknown blocks have consumers, or could any be code? (consumers.py, codeprobe.py, period.py)
   - consumers.py: abs/abx/aby/ind operands (base window s-255..e), branch targets, LD?-immediate pairs, data words.
     Every hit was resolved as a false positive: bank 7 JSR $B93E/$B959/$B95F run after JSR $FF3C with A=0
     (bank 0 sound-driver Code); bank 3 $8141 LDX #$04/LDY #$BE is a delay loop; $804F/$8652 are OAM tile
     immediates; $81AC is DR_CourseMap ($944B); bank 3 $943F/$9445,X are the 6-entry DR_LightMetatiles/DR_LightSfx;
     STA $9FFF writes are MMC1 register writes; data-word hits sit in metatile sets / the note-period table.
     No branch from Code into any block.
   - codeprobe.py: the longest clean official-opcode run is 13 instructions (bank 2 $9F63), 7 (bank 3), 5 (bank 0
     lists), 0 (bank 2 $FF fill). Linear decode: bank 2 $9C25-$A7E0 has 464 BRK and 583 unofficial in 1655 decodes;
     bank 3 $B375-$BEFF has 313 BRK and 685 unofficial in 1466. JSR/JMP triples to Code starts are $FF00/$C002
     coincidences inside $00/$FF runs. Not code.
   - period.py (new independent constraint): bank 3 $B375-$BEFF has mean popcount 1.0-1.3 at CPU address
     low nibble 0-7 and 7.1-7.5 at 8-F. This is an address-aligned 8-low/8-high bit pattern, typical of
     uninitialised memory images and incompatible with instruction streams. It strengthens the generic-data
     ("leftover fill") reading. The role stays unknown.
   - Bank 2 $9C25-$A7E0: $FF-separated small-value runs, then 8-byte bitplane-like groups (tile-bitmap look). No
     consumer. This is a CHR-ROM cart, so there is no CHR-RAM upload path. Stays generic Data.
2. DR_CourseMap extent (coursemap.py). **New proof (L-022).**
   - Only two MtileMapPtr writers exist in bank 3/7 Code: $81AC/$81B0 (#$4B/#$94) and $9421/$9425 (DR_BackgroundMap).
   - The DR_StreamTrack lookups are `lda (MtileMapPtr),y` (MetatilePtrFromMap $C4D4). Y is 8-bit:
     Y = (Dist>>5)&$FF (DR_ColumnFromDistance + DR_ColumnHalf) and (Y+$0F)&$FF.
   - So only $944B-$954A (256 B) is addressable.
   - Independent constraint: DR_CheckFinish ends the race at DrCourse >= $0F00. Dist = 2*course = $1E00, so
     Y=$F0, and the look-ahead column $FF is exactly the last byte of the window.
   - $954B-$9560 (22 B) is outside the window. It uses the same index vocabulary (a subset of the window's
     values; max 44). It is unreachable through this consumer, possibly an authoring overrun. Generic Data.
   - The window's max index is 75, so DR_BackgroundMetatiles records 0-75 are used. Records 76-103 are unchanged
     (stride evidence only).
3. L-021 checked.
   - Bank 7 $D881-$D922 is exactly MenuMetatiles records 41-49 (17 B each: 5 attr $05/$55 records with
     4 tiles, 4 full) plus 9 zero bytes.
   - $D899-$D8A2 are tiles of record 42 and $D8A3 is record 43's attribute ($55). The same tiles appear in bank 0
     TitleMetatiles records 43-49. Not prose, so Text -> Data is confirmed.
   - Bank 0 $A92E-$A94A is part of $FF-separated tile-number lists with 2x2-interleaved pairs (48 49 50 51 4A 4B 52 53 ...).
     Binary, so Data.
4. Small role hints (commentary-only; no map change):
   - Bank 4 $B348-$B369 = exactly 2 x 17 B after the 194-record SR set and before the $00 fill to $BEFF. Record
     195 has attr $55 (common in the set); record 194's attr $30 never occurs as a set attribute. Weak.
   - Bank 2 $9420-$943F matches the 16-byte FS pose-frame layout (2 frames, tiles $11-$18, Y $72/$7A). No
     FS_PoseFramePtrs entry points there; DrawMetasprite length comes from RAM params. Not established.
   - Bank 0 $A108-$A147 = TitleMapTop plus indices $17/$1F/$22/$27/$2C. With TitleTopMetatiles (14 records) these
     overrun the set. $2C falls in the TitleMetatiles tail (records 43-53). The set is not established.
   - Map nit: bank 2 region $ACDF-$AFFF starts with the $80 terminator of the last banked SFX stream
     (commentary B2_SfxStreamPtrs_17 = $ACDB-$ACDF). The $FF fill proper is $ACE0-$AFFF (one $DF at $AD90).
     Optional split.

## Not pursued (reasons)
- L-004 roots: reachability only. No new reference (consumers/anchors), no changed inference.
- S16 natural Games run: unchanged reason (all milestones visited; nothing depends on it).
- Runtime read-watch of $954B-$9560 through a Dino Race finish: optional. The 8-bit (ind),Y proof is decisive.
