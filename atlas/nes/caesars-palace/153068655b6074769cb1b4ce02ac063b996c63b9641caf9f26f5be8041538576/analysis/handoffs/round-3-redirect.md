# Round 3 redirect handoff (attempt r3-a1)

Outcome: READY LEAD, commentary only. **L-011** (medium) needs no map change. No structural lead survived review; see the
rejection review below. L-009 is now marked verified (incorporated by the round-3 commentary).
This redirect changed only research and lead records. ROM, map, state, source, commentary and the configured binary are
unchanged. state.json is still sha256 24099db1...

## Checkpoint identity
- Input: maps/round3-disasm.json = maps/current.json = analysis/state.json (sha256 24099db1a831...afbed3), 1021 regions.
  100.00% classified (131088/131088). Code 26,374 (19,753 Observed / 6,621 Static), Data 92,964, Padding 11,750.
- Proposed vs accepted bytes: L-011 proposes 0 bytes (commentary only). The exact unknown complement is empty (0 bytes),
  so the unknown ledger has no entries. The residual ledger below covers weak claims inside classified bytes.

## Methods and results (artifacts in analysis/redirect/r3-a1/; summary in findings.json)
All runtime states are artificial. They use r1-a1 `drv.enter(Y)`: load the casino-floor snapshot, run to 7:$C403, then
set pc=$C438 and y=Y. CHR, palette and OAM were read with immutable physical_memory reads. Captures are single frames.
1. **Clock alternate phase (tools/clock.py).** At 8:00 the normal phase shows the time. The alternate phase
   (ClockColon=0) shows "85" followed by a degree sign. ClockHourTable[8*6+0] is 85, so the table is the displayed
   temperature. Shots: clock-phase1.png, clock-phase0.png.
2. **Slot symbol ids (tools/symbols.py, plus an on-screen check).** I rendered each symbol record from CHR RAM, read
   palette sub 0, and compared ReelPos ($0617) against the strips at window offsets 0/2/4 with the framebuffer. For
   kinds 10 and 11 at ReelPos 0/6/12, every visible symbol matched.
   - Kind 9 (four-reel): 0 blank, 1 cherries, 2 red 7, 3 green 7, 4 black 7 (four of them is the jackpot).
   - Kind 10: 0 blank, 1 black 7 (three pay 1000), 2 red 7, 3 BAR, 4 double BAR, 5 triple BAR.
   - Kind 11: 0 watermelon, 1 pale oval (probably a lemon; seen only in the render, and never paid), 2 BAR, 3 seven,
     4 cherries, 5 orange, 6 plum, 7 bell.
   - The pay branches are consistent with these pictures. Kind 11 pays: 7s 200, BAR 100, melon 100, bell 18, plum 14,
     orange 10. A BAR on reel 3 completes a pair, and cherries pay 5 or 2.
3. **Pay lines (static reasoning plus the screen check in item 2).** Window offset 0 is the top row, 2 the middle and
   4 the bottom. One coin pays the middle line, two coins add the top line, and three coins add the bottom line.
4. **Sound ids $0D and $10 (static).** I enumerated all 54 PlaySound/PlaySoundAndSelectBank6 call sites.
   - Literal ids: $09-$0C, $0E, $0F, $11-$16, $80 and $83.
   - Computed ids: $04-$08 and $81-$82.
   - SoundEntry_Start is reached only from PlaySound and from the restart path, which never runs (r2-a2/L-009).
   - So $0D and $10 have script pointers but are statically unreferenced.
5. **7:$C0D3 block (tools/c0d3.py).** The mirrored half reverses tiles within each row, which gives the row widths
   (15 pieces = 3,3,3,2,2,2; 16 pieces = 4,3,3,2,2,2).
   - Rendered from sprite pattern table 0 (floor and slot CHR), it shows a rod with a round knob. The rod swings from
     vertical to horizontal and back over frames 0-1-2-3-2-1-0, and the second half repeats this mirrored.
   - This probably is an unused sprite slot-handle pull animation. That is an inference only: there is no writer, the
     alignment of the 17- and 13-piece rows is a guess, and the shipped game draws the handle in the background.
6. **Split hi/lo pointer-table scan (tools/splitscan.py, splitscan.json): new method, negative.** Earlier scans checked
   operands, immediate pairs and contiguous data words. This one looks for a low byte at p and a high byte at p±d
   (d ≤ 40) inside Data, aimed at every residual span [start-4, end). It is motivated by the split tables this game
   really uses (ReelStopTablePtrHiA/LoA, FourReelStopTablePtrHi/Lo), which the r2-a1 scans missed.
   - With the neighbour-entry filter there are no hits for 6:$85CE, 6:$B554, 7:$D0F0 or 7:$F398.
   - The few hits for $C0D3, $D0E4, $E195 and $F3DC are coincidences with inconsistent strides.
   - The 7:$E000 hits are $E0 Y values in the HotspotY table, not pointers.
7. **Scenarios.** No unvisited milestone remains: all game types, info screens, the quit dialog, the endings and the
   broke ending were visited in r1-a1. I added S11 for the captures above.

## Ready batch
- **L-011 (commentary).** Clock temperature; symbol ids and pay-line rows (with optional label renames such as
  `_EvalLine0` -> `_EvalTopLine` and `_CheckSymbol4Pair` -> `_CheckCherries`); sound ids $0D and $10 unreferenced; the
  $C0D3 inference. The recipe is in analysis/leads.json. Disassembly passes it through unchanged. Commentary applies it
  and then reruns `commentary.py verify`.

## Rejection review (structural residuals; no structural lead proposed)
Every residual below is already classified, so none of them is an unknown classification. The question for each is
whether a stronger role, a different classification or tighter bounds is still supportable.

| Residual | Current claim | Decisive artifacts | What they rule out | Strongest competitor | Why no revised test remains |
|---|---|---|---|---|---|
| 6:$85CE-$85ED (32) | generic Data | r2-a2 read-watch.json (4 slot machines, 0 reads incl. operand reads); r2-a1/r2-a2 operand, immediate-pair and data-word scans; r3-a1 splitscan (0 hits) | Any consumer in slot routes, and any absolute/immediate/contiguous/split pointer in banks 6-7 | Old reel-symbol remap (values 0-7, rows of 8). Code is excluded: the bytes 00 01 02 ... decode as BRK/ORA garbage after the JMP that ends DrawReelB_Symbol | Kind-11 stops use ReelStopTableB via Mod20, and the strips are fixed 20-byte tables (checked this round), so no indexing path remains. Only an unknown computed pointer could reach it, and none is built in bank 6 |
| 6:$B554-$B567 (20) | generic Data | r2-a2 watch (b6tail run): never read; all scans negative | Reads during all slot play and pointer references | Two extra SlotWinTextFrames rows | Its consumer (frame index 0-2) is bounded by code, so a 4th or 5th frame is never selected. That is comment material only |
| 7:$C0D3-$C286 (436) | generic Data with format note | r2-a2 msplits (13 SprDef immediates tile $C049-$C0D2 exactly); r3-a1 render | Every metasprite writer: all are fixed-count, and none accepts 13-17 pieces | Unused sprite lever animation (r3-a1) | The role is now illustrated, but no consumer or extent beyond the already exact bounds exists. A label could be added only as an inference, which belongs in commentary (L-011) |
| 7:$D0E4 (12), $D0F0 (60) | generic Data next to the L-008 tables | L-008 tables read only by the unreferenced $CFCF code; scans negative (incl. split) | Live consumers | Further windowed-text tables of the vestigial printer | Their only plausible reader is itself unreferenced. More scans would repeat the same negative |
| 7:$F398 (8), $F3DC (60) | generic Data | Operand, immediate-pair, data-word and split scans negative | Absolute or pointer reference | Nametable helper tables | No code builds $F3xx pointers with an offset. The read-watch was judged low-prospect in r2-a2, and that premise is unchanged: there is no new writer evidence |
| 7:$E195 (15) | generic Data | r2-a2 bsearch: only the RTS dummy read | Live reader | Vestigial sprite-0 table indexed by the discarded X | Already in commentary as an inference. No caller can supply the index |
| 7:$E000-$E004 (5) | head of the bank-pointer table region | Table format of $E000-$E098 (4-byte records) | none new | Partial record | No new evidence. Splitting it would add nothing structural |
| 6:$8000 leader byte | Data 0.8 | r1-a1 trace (no fetch); r2 rescan of 11,895 instructions (no operand $8000) | Fetch and reference | NOP prologue | Analogy with the bank 0-5 leaders is the only discriminator. No experiment can separate an unentered NOP from data |
| 675 + 86 bytes of statically unreferenced code | Code Static 0.8 | r1-a1 fragment fixpoint; r2-a2 indirect-path check; r3 L-008 review | Known entry | Dead or vestigial code (not claimed) | The code-versus-data decision is already supported by control flow. Reachability needs an entry that no scan or replay found, so raising it to Observed would need artificial execution, which proves nothing |

Corrected premises considered: the r2-a1 scans missed split pointer tables, so this round added that scan; it found
nothing. The read-watch scope is limited to slot routes and startup. Residuals outside banks 6 and 7 slot code have
no bank-6-relative reader, so widening the watch would not discriminate.

## Recommended next step
Run one commentary cycle for L-011, which is commentary only. A disassembly pass is a pass-through with no map change.
After that, the structural work is at its limit under this review. What remains is final editorial audit and
verification.

## Toolkit
No new issues. TK-001, TK-002, TK-004 and TK-005 are unchanged (analysis/toolkit-issues.md).

## Disk and hygiene
- Evidence is about 130 KB: 10 PNGs, findings.json, splitscan.json and 4 tools.
- Scratch is empty, and the serve daemon is stopped.
- About 3.1 GB is free.
