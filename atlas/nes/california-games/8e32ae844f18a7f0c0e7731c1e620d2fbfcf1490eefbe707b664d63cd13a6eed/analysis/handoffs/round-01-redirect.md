# Round 1 - redirect phase handoff (attempt r01-a1)

Status: FINISHED for this redirect pass. Recommendation: **(a) next cycle disassembly with ready leads**
(L-R1R-01..10). L-R1R-11 is the open frontier for the following redirect.

## Scope and checkpoint
- Input: maps/round01-final.json (sha256 b4a8fa61...), commentary/source/game.asm (c0b3595f...). Map, state.json,
  ROM, source and binary unchanged (current.json/state.json still b4a8fa61...). ROM classification still
  83.73% (109762 / 131088; 21326 unknown).
- Research directory: analysis/redirect/r01-a1/ (NOTES.md, tools/, evidence/, listings).
- Proposed for classification: **16,480 of 21,326 unknown bytes** (would give about 96.3% if all accepted)
  plus one correction (0xC01E-0xC020 Data to Code). The ranges, lead tags and rationale are in
  analysis/redirect/r01-a1/evidence/proposals.json. Rebuild it with `python3 analysis/redirect/r01-a1/tools/combine.py`
  (read-only; it prints the summary and checks for conflicts: 0 internal, 0 Data over classified Code).

## Ready leads (analysis/leads.json; every lead has a recipe; filter proposals.json by tag)
| Lead | Tag | Unknown bytes | Evidence / extent proof |
|---|---|---|---|
| L-R1R-01 | CODE | 1288 (+3 reclass) | Reset/boot code is observed in the power-on trace (evidence/boot-trace-fetches.json). The bank-1/2 score-screen code and frame handler $18 $B9F1 are observed in evidence/surf-end-trace-fetches.json. The remaining code is a static closure from CallBank literal-Y entries and bounded jump tables. The closure2.py forced-branch fix adds $D393 and two short pieces. |
| L-R1R-02 | BUSTAB | 56 | Bus-conflict tables 00..07 at $8000 of banks 0-6. |
| L-R1R-03 | DISPATCH+MENU | 128 | Surf, skate, sound-command, $EBE0 and bank 6 $9193 tables, plus MenuActionTable entries 13-28. Each table ends at its first target or at code. |
| L-R1R-04 | SOUND | 1385 | SongTempo/SongTable (14 songs), SfxTable (7), and a gap-free parse of every stream (evidence/sound-stream-walk.json). |
| L-R1R-05 | TABLES | 2111 | 86 gaps that start at an abs,X/Y operand base in classified code; the bytes are visibly table data (gaprefs-bytes.txt). |
| L-R1R-06 | MSPR5 | 5556 | Bank 5 metasprite tables and definitions, walked contiguously from 256+25 pointers (mspr-b5.txt). |
| L-R1R-07 | GFX2 | 595 | Bank 2 LoadGraphicsBlock tables (6 x 109) and the contiguous block sources $913F-$9F09. |
| L-R1R-08 | SCR3 | 1996 | Bank 3 LoadScreenBlock records (48 x 5) and their RLE/plain sources. The RLE length model matched a read trace exactly (456 bytes). Record 38 is excluded because it overruns into code. |
| L-R1R-09 | B1 | 1946 | Bank 1 planar animation store (all of it), index tables ending at code $830B, boot text and RLE screens, and the OAM image. |
| L-R1R-10 | SURF6+STRIP6 | 1419 | Bank 6 surf column sets end exactly at $BE78; the strip/metatile block ends exactly at code $85B9. |

Disassembly recipe: for each tag, check that the bytes are still unknown, then map.annotate with Static
provenance and the note text. Use Observed provenance only where the evidence fetch files list the start
offsets. Bank context is prg_banks [2b,2b+1,14,15]; bank 7 code is fixed and uses switchable context 0 where
it calls bank-0 helpers ($AB23/$AB43). Before running static discovery, fix analysis/tools/closure.py
`forced_branch`: the preceding CLC/SEC/CLV/LDx# must be a decoded instruction start
(analysis/redirect/r01-a1/tools/closure2.py). Then re-run commentary for the changed ranges. Many targets
already have names in the commentary notes (ShowEventScore frame handler, SndCmd 2/3/7/8/9, surf states 5-7, etc.).

## Other findings (intermediate evidence)
- Reset path: $FFB0 runs lda #0 / sta $8000 / jmp $86ED. The reset body ($86ED-$8747) inits the PPU and APU,
  calls ClearRam $F1DB, $F2D9, CallBank3 entry $0E -> $BBFB (RAM init: if the signature " PAUL & RACHEL " is
  missing, copy the default high scores and the two RAM copy routines), then PlaySong 0 and jmp $EF94.
- RAM routines: $0310 = plain 16-byte tile copy from bank Y. $042A = RLE tile copy (0/$FF marks a run: value, count).
- Bank0 $8008 bytes `20 93 D8` (jsr UpdateSound) have no CallBank0 Y=$08 caller and fall into the copyright text.
  They are left unknown (ambiguous; 3 bytes). $F2BA-$F2BC (CallBank0 entry) has no caller.
- Sound driver: command 8 ($D9D6) sets the flag without advancing Y. If used it would re-read the same byte;
  no stream uses it.
- Scenarios: FINAL RESULTS (compete ending) and the surfing end score screen were both reached through RAM
  patches (analysis/scenarios.json S14, S06b). The game has no separate credits: FINAL RESULTS times out to the
  main menu. Compete-some is still deferred (its code is already classified).

## Rejected / exhausted avenues (compact rejection review)
- Screen block record 38 ($BE, bank 4 $BA8C, 29 tiles). The decoded extent overlaps 16 bytes of the bank 4
  code at $BC4C. This rules out using consumer-read extents as proof wherever they touch code. Record 38 is
  excluded; the other records are bounded by data on both sides or chain contiguously
  (13->9, 14->13, 16->14, 17->16, 34->35->36->15, 31->39, 41->27).
- Record 2 ($0A, 191 tiles) overlaps record 5's source. It was kept because both lie entirely in Data/unknown
  with no code overlap. The competing interpretation (a deliberately composite load) still gives the same role,
  generic tile data.
- MSPR5 pointer entries 66, 96, 97 and 107 are not ROM addresses. They were treated as unused frames, not as
  evidence against the table: the min-target = table-end constraint and contiguous definition coverage hold
  independently.
- L-R1-05 (short read runs) needs no separate method: of 524 bytes, 248 were already classified and 310 are
  inside proposals.

## Deferred avenues and reasons
- L-R1R-11 frontier: about 4,846 unknown bytes remain after the proposals. About 3,800 are in bank 6 course
  scripts (pointer table $A1B4 -> $A1D4.., CourseReadSegment $87CB command set). They were deferred because the
  command lengths still have to be tabulated; this redirect ran out of budget before that, not out of method.
  Smaller residuals: bank 0 $B703-$B969 (BMX/half-pipe area, not yet inspected), bank 7 (477), bank 2 $82DA/$86F2
  (377), bank 4 (161). Resume with the L-R1R-11 recipe.
- L-R1-01 (cross-window instruction $BFFE) is still blocked on toolkit TK-04.

## Toolkit
No new toolkit issues. The forced-branch defect is in workspace tool analysis/tools/closure.py, not the toolkit.
