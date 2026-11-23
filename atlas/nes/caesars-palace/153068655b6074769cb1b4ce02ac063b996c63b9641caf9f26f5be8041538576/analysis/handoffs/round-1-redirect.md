# Round 1 redirect handoff (attempt r1-a1)

Outcome: READY LEADS for another disassembly/commentary round: L-003, L-006, L-002 (high), L-001, L-004 (medium), L-005 (low).
Redirect changed research/lead records only (ROM, map, state, source and binary untouched;
analysis/state.json and maps/current.json sha256 324985b8... verified unchanged at handoff).

## Checkpoint identity
- Input: maps/round1-disasm-v2.json = maps/current.json = analysis/state.json (sha256 324985b8e9b5df8fb4833fc4a4e3f8a5470ebce62c73a226e619940bb27cdcdb),
  commentary/source/game.asm (round-1 commentary, byte-identical verify).
- ROM classification: 100.00% (131087 / 131088; exact unknown complement = 1 byte, physical 0x18010, 16K bank 6 CPU $8000, value $EA).
- Proposed vs accepted: proposals below change 1 unknown byte to Data (L-003) and correct 25 Padding bytes to Data (L-006); nothing accepted yet (disassembly reviews).

## Unknown-complement ledger
| Physical | Bank/CPU | Neighbours | Disposition |
|---|---|---|---|
| 0x18010 (1 byte) | 16K bank 6 $8000 | before Code 6:$8001 TableCommonInput; preceded by bank-5 tail template | L-003 ready: Data (bank leader byte), evidence below |

## Methods and results
1. Gameplay/milestones (L-001). Shortcut: from analysis/snapshots/casino-floor.snap.json, run_until pc==$C403
   (UpdateCasinoFloor, JSR'd from CasinoFloorLoop), registers.set pc=$C438 (DispatchGameScreen), y=hotspot.
   Stack contract matches the natural JMP. All untraced except one bounded traced replay (tools/cov.py).
   Visited: video poker (deal/hold/draw), blackjack (bet, deal, stand, win payout), roulette (bet, spin),
   Big Six (bet, spin), slot kinds 9/10/11 and the $500 machine (coins, spin; a $10 win), clock, restroom,
   pay-table info screens, cash status, quit dialog YES/NO, EndingTally with all six ratings (ChipCounts/
   ChipOverflow patches) and the broke path GameOverBroke -> EndingTally -> MainInit.
   Betting contract (round-1 obstacle): pick a chip on the chip bar first (Y $B0-$DF), then A on the bet
   spot; slots: A on the coin slot (X $B0-$C7, Y $70-$7F) then A on the handle (X $E0-$F7). Details and
   screenshots: analysis/scenarios.json S5-S9, analysis/redirect/r1-a1/shots/.
2. L-003 static review: no code operand equals $8000; all non-literal transfers enumerated (one bounded
   bank-0 JMP (ind), no RTS/RTI dispatch, no RAM execution, no BRK, vectors $C000/$FFB2/$FFB2); all
   switchable banks place content at $8001 (sound table, sprite set, first screens 4/9/16/23, bank-6
   TableCommonInput); banks 0-5 leader bytes are already Data. Traced replay of all milestones (14,166 contexts): no fetch at 0x18010.
3. L-004 fixpoint reachability (tools/frag2.py -> fragment-split.json): 967 of the 1642 "unreferenced
   fragment" bytes (confidence 0.7) are reachable from established callers (GameOverBroke/QuitDialog/
   EndingTally $C687-$C7D7, DrawCardTiles/NextPpuRow $FB71-$FC45, 6:$8BDA-$8CD2, ...). Residue 675 bytes in
   24 runs has no possible entry given the transfer enumeration; still structurally code.
4. L-002 data formats (tools/screens.py, records.py -> data-records.json, screens.json): consumer-matching
   decoders give exact extents for 63 records (85,306 bytes) that tile all content of data banks 0-5 with
   zero gaps: 30 nibble-RLE screens (CHR + 960 nametable + 64 attribute + 16 palette bytes, $FF end), the
   bank-1 sprite set, 4 songs, 19 sfx scripts, 1bpp font, floor map (header, 28 blocks x 64 tiles, 28 x 4
   attributes, 72 row ids), floor CHR (239 tiles), floor palette/aux/size. This settles the reduced-confidence
   "banks 1-5 generic data" claim: every content byte is consumed by ScreenDecode/LoadSpriteAnimSet; no
   room for hidden code. It also exposed L-006 (25 bytes wrongly Padding).
5. L-005: commentary equates provide consumer-sized boundaries for several fixed/bank-6 tables; one
   inconsistency (FourReelPayTable size vs FourReelColumns $8D09). Metasprite definitions not decoded.
6. Coverage delta (tools/delta.py -> coverage-delta.json): traced replay (~11k frames, 3,809 s) produced 14,166
   observed ROM contexts (round 1: 5,377); 5,288 instruction starts not observed in round 1, all inside 88
   Static Code regions (49 bank 6, 38 fixed, 1 bank 0). No fetch in any Data/Padding region (supports L-002),
   none in the L-004 unreferenced residue, none at 0x18010. Reached fragment parts observed: $C679 region
   (121 starts: GameOverBroke/QuitDialog/EndingTally), $FB10 region (81: DrawCardTiles), 6:$8BD7 (79), $CC4C (35).

## Ready leads (recipes in analysis/leads.json)
- L-003 (high): annotate 0x18010 Data, Static 0.8, "bank-6 leader byte"; expected 131088/131088 classified.
- L-006 (high): phys 10976-10995 -> Data (FloorBlockAttrs); phys 32493, 48887, 65049, 81243, 88610 -> Data
  (screen $FF end markers); shrink adjoining Padding. Classification total unchanged.
- L-002 (high): split banks 0-5 Data into the 63 records of data-records.json with labels/format notes, confidence 0.95.
- L-001 (medium): import obs-r1a1.json.gz as Observed provenance for newly executed Static code.
- L-004 (medium): split 0.7 fragments into reachable (0.9 with caller list) and unreferenced residue (0.8, explicit "no possible entry" evidence).
- L-005 (low): equate-bounded table splits; fix FourReelPayTable size.

## Deferred avenues (with reasons)
- Natural floor walking to every hotspot: not needed; S4-S6 already prove the floor route and the
  dispatcher contract is exact (diminishing value).
- Slot pay-table / reel-strip semantics (FourReelPayTable, ReelSymbolTiles*): pay-table screens now
  captured (shots/info-slotB.png, info-slot4.png) and could drive semantic labels in commentary; no
  classification consequence, so left to commentary.
- Metasprite definition extents ($C049-$C286, $D084-$D12B via MetaspriteTable $CFB5): frame size depends
  on the DrawSprite* writer (1x1, 2x2, 5x3 ...); needs per-SprDef writer mapping; generic data is already
  sufficient - deferred for diminishing prospects.
- 6:$85CE (32 bytes) and 7:$E195 (15 bytes) consumers: no absolute/indexed operand within 255 bytes below
  either table; no consumer found (negative result recorded).

## Toolkit
TK-002 updated with new measurements (traced stepping ~15-30x slower). No new toolkit bugs.

## Reproduction
Tools in analysis/redirect/r1-a1/tools/ (static: ops.py, frag2.py, screens.py, records.py, eqs.py, delta.py need no
server). Runtime: `mkdir -p scratch; python3 analysis/redirect/r1-a1/tools/daemon.py &` (serves run.py serve on
/tmp/romatlas-1530.sock), then run enter.py/slot.py/poker.py/bj.py/roul.py/b6.py/ending.py/msgs.py/cov.py from the
tools directory. Scratch (snapshots, frames) was deleted after promoting cited screenshots to shots/.
