# Investigation ledger

## Round 1 disassembly (checkpoint pass-001)

### Method
- Observed coverage: 15,708 (offset, CPU address, PRG context) instruction starts and 15,021 edges from traced scenarios (analysis/scenarios.json). Contexts seen: pairs (0,1),(2,3),(4,5),(6,7),(8,9),(10,11),(12,13) with fixed (14,15).
- Static: local bank-aware recursive descent (toolkit discovery stops at JSR/indexed stores, TK-003) from observed starts, RESET $EC8F / NMI $F256 / IRQ $F259, and statically resolved dispatch roots:
  - FarCallIndexed $F81A + FarCallTable $F86A: 14 records (lo, hi, 16K bank, 0); all indices 0-13 passed as immediates by callers.
  - FarCallAddress $F855 (A=16K bank, X=lo, Y=hi) immediate call sites in bank 12 / fixed.
  - SetNmiHandler $F220 + NmiHandlerTable $F238 (15 words, bounded by NMI code at $F256).
  - Jump tables $F8EF (4), bank1 $AD59 (4, cmp #$04 bound), bank1 $B1F1 (32, bounded by next table $B231), bank12 split table $88F7/$8904 (13, bounded by code at $8911), NMI handler $FF36 selecting bank 6 -> $8BD4.
- Result: 16,443 instructions / 36,605 bytes, no invalid opcode/BRK, no overlapping decodes. All later-observed instructions (events + ending) were already in the static set (independent check of the static descent).
- Data: gaps between code split into generic data pieces where linear decode shows no code-like run (>=10-byte legal run with >=4 opcodes and a branch/RTS/JMP/JSR/abs store, or any I/O/mapper store), with 8-byte margins; consumers recorded in each region's evidence. Header (16 B) and CHR ROM (32 x 4 KiB Image) classified.
- MMC1_WriteChr0/1 ($EF25/$EF39) classified Static code by identity with reached sibling MMC1 writers.

### Remaining unknown (14,697 bytes) — see leads L-001..L-006, L-009
Per bank unknown: PRG00 814, PRG01 265, PRG02 2035, PRG04 929, PRG05 225, PRG06 239, PRG07 1163, PRG08 456, PRG09 903, PRG10 3835, PRG11 1522, PRG12 1747, PRG13 85, PRG14 93, PRG15 386. Mostly data that decodes as legal 6502 (text, tile maps, tables) plus a few unreferenced routine-like islands. Exact list: map.get/analysis.unknown on maps/pass-001.json.

### Limitations
- Generic data pieces are classified by code-absence heuristics plus consumers; tiny (<10 byte) unreached routines inside them would be misclassified — revisit if code evidence appears.
- Fixed-bank code views use prg_banks [null,null,14,15]; switchable code uses its 16K pair.

## Round 2 disassembly (checkpoint pass-002)
- Method: re-ran redirect r01 tools (unk.py, rle.py, cov5.py, rd.py) plus fresh bank-aware decodes of every island and each L-016/L-017 gap (left terminal instruction, right neighbour, byte pattern); analysis/disasm-r02/tools/plan.py asserts that the plan tiles exactly the 14,697 unknown bytes and that every Code piece decodes contiguously with no BRK.
- Outcomes: all ready leads verified and annotated (see progress.md Round 2). Code/data corrections: $8055-$805D Data->Code; L-016 items $8395-$8398, $8588-$8591, $8998-$89B1, $D827-$D828 judged code rather than tables (instruction idioms adjacent to known code, no indexed consumer).
- Structure splits: bank 5 per F3 layout, bank 6 hill tables per F5, bank 3 path lists per F6, bank 1 per RLE stream, bank 0 palettes/script tables per F7/F8.
- Remaining unknown: none. Open questions (not classification blockers): natural reachability of the Static islands and HillNmiJump handlers (HillSpecial $066A values 1-12); semantics of the bank 2 music format, bank 3 track data and the L-017 generic tables; palette pointer slots holding $0000/$0491.

## Round 3 disassembly (checkpoint pass-003)
- Method: static bank-aware recheck over input/game.nes (analysis/disasm-r03/tools/verify.py) of each lead's consumer instructions, pointer progressions, record counts and terminators; L-021 derived from saved r02 static-hits.json by comparing executed-start counts with each region's decoded instruction count (tools/l021.py). No emulator runs.
- Outcomes: L-018..L-022 verified and applied (see progress.md Round 3). Structure splits: region 675 -> 4 TerrainEffect tables + MetatileTerrainMap part 1; 758 -> 4 RaceSprOffset tables + RaceSpriteLayoutPtrs + RaceSpriteLayoutData part 1; 759 trimmed + $AC37 split; 762 -> 4 pieces; 962 labelled.
- Remaining open (non-classification): roles of generic 2:$B0E7-$B4A9, 2:$AC37, 4:$A474-$A873, 5:$8388 blocks; natural reachability of the 288 still-Static code regions (27 partly executed).

## Round 4 disassembly (checkpoint pass-004)
- Method: reran r03 checks (analysis/redirect/r03-redirect-a1/tools/l024.py, ntcompare.py) and static byte asserts in analysis/disasm-r04/tools/plan.py (fixed-bank reader bytes F:$D297/$D2A0/$D2AC, zero run, attribute bytes). No emulator runs; r03 Read-watch evidence reused.
- Outcomes: L-024 and L-023 verified and applied (progress.md Round 4). Structural claims (table extents) are static; reachability: EventGroundHeights observed read (artificial snapshot), legal screen reader unknown, $8388 zero run unread.
- Remaining open (non-classification): readers of 2:$B0E7-$B4E6 and 5:$8388-$8607; roles of 2:$AC37, 4:$A474-$A873; natural reachability of Static code regions.

## Round 5 disassembly (checkpoint pass-005)
- Method: ran analysis/redirect/r04-redirect-a1/tools/checks.py (all asserts pass) and independently re-searched the reader opcode bytes in the effective 16 KiB banks: 3:$8517/$8579/$8597/$859E `lda $88B5,x`; 0:$8EA6 `cmp #$08`/`bcc`/`jmp $8F1D` + 0:$8EDB `lda $9072,x`; 0:$BC71/$BC77 `lda $BE48,y`/`$BE60,y`, 0:$BCED `sbc $BE30,y`; fixed bank (16K bank 7) F:$C72B `ora $C723,y`, F:$D514 `lda $D450,y`. No emulator runs. Plan/apply: analysis/disasm-r05/{plan.json,tools/plan.py,tools/apply.py}.
- Outcomes: L-027, L-026, L-025 verified and applied as region merges/splits (0 kind changes, 0 new bytes); L-028 verified, map evidence notes only (the false "no reader" text exists only in commentary).
- Structural vs reachability: MulTable16x16 extent is static (product content + index form + endpoints); reads observed only below $894D. MapTruckPath extent static (guard + 8-record pattern). SparkStartY truck-1 tail ($BE6F-$BE77) never observed read; extent rests on the static Y domain 0..23.
- Remaining open (non-classification): 0:$90B2-$90EC and the other generic blocks listed in round-04-redirect.md; two unread 24-byte tables 0:$BDE8/$BE00 (inside region 689); natural reachability of Static code islands.

## Round 6 disassembly (checkpoint pass-006)
- Method: ran `python3 -I analysis/redirect/r05-redirect-a1/tools/checks.py` (L-029/L-030/L-031 all ok) and re-read the ROM bytes bank-aware: SongChannelOffsets 2:$8070 = $83D0/$8430/$8490/$84F0, each table entry 0 = $0000, last channel's last pointer $8C14; $8708 begins with order-list command bytes after the 31st 8-byte instrument record; 0:$90B2 record header $23 (2x3) + 6 tiles + separator, last record `11 6D` at $917E + separator $7F at $9180; 2:$B65D-$BFFF and F:$FF57-$FFF9 all $FF. No emulator runs (no runtime question; prior storywatch read evidence reused). Plan/apply: analysis/disasm-r06/{plan.json,tools/plan.py,tools/apply.py}; pre-edit state copy analysis/disasm-r06/state-before-r06.json.
- Outcomes: L-029, L-030, L-031 verified and applied (merges/splits/relabels; 0 kind changes, 0 new bytes; regions 1,401 -> 1,400).
- Structural vs reachability: StoryTileRecords extent is static (pointer fields + contiguous blocks); 25/27 records observed read naturally (r05 storywatch), $9149/$9154 not observed. ChannelOrderListPtrs/InstrumentTable/SoundOrderLists extents are static parses; no new reachability claim. Fill blocks: no reader, reachability not claimed.
- Remaining open (non-classification): generic residuals listed in round-05-redirect.md rejection review (0:$BDE8/$BE00, 2:$B059 spare pointers, small/uniform blocks, LegalScreenTiles reader, Static islands' natural reachability).

## Round 7 disassembly (checkpoint pass-007)
- Editorial only: region 742 (2:$85C6-$860F) evidence renamed SongHeaders -> SongPatternBases (per-request pattern base addresses; reader 2:801D LDA $85B0,y / 2:8022 LDA $85B1,y; commentary round 6 r2_b012 notes). Regex scan of all region texts for names retired in commentary rounds 5-6 found no other contradiction. No structural, kind or bound change; no runtime work.
