# r04-a1 redirect notes (round 4)

Input checkpoint (state.before.sha): maps/round03-final.json = maps/current.json = analysis/state.json
aab2fb6c...; source commentary/source/game.asm e12e1cad...; ROM 8e32ae84.... Only emulator calls + read-only
static scripts; no map/state/source/ROM/toolkit change. Bridge: analysis/tools/rpcbridge.py (run.py serve).

Tools (this dir):
- tools/xwatch.py EVENT OFFS TOKENS: Instruction trace restricted to consumer instruction offsets, prints the
  pre-operation X/Y sets. No RAM/register writes. EVENT -1 = scratch/snap_menu.json, 1-6 = practice event
  (same entry sequence as r03-a1 evwatch.py).
- tools/static_checks.py: BMX/skating object-stream type check + in-code table alias extents vs map regions.
- tools/rdwatch.py: Read-trace attempt. Abandoned: rom Read events include opcode/operand fetches and carry
  no PC/registers, so they cannot separate data reads at code addresses. Kept only as a record.
Evidence: evidence/index-watches.txt (all watch outputs), evidence/static-checks.txt, evidence/batch2-recipe.sh.

## 1. Residual ledger (recomputed from maps/current.json)
Classified 131085/131088. Unknown = 0x400E-0x400F (b0 $BFFE-$BFFF, BD 49) + 0x1C010 (b7 $C000, C1).
This is the TK-04 cross-window instruction `lda $C149,x`. The premises are unchanged: same map hash, same
bytes, and the toolkit is unchanged. The r03-a1 rejection review (analysis/handoffs/round-03-redirect.md) is
reused; its decisive artifacts are L-R1-01 (observed fetch at 0x400E, successor at $C001) and the
decode_region contract (romatlas-nes mapping.rs). No corrected inference exists.

## 2. Bank-6 shared course code vs BMX variables ($A9-$C1) - RESOLVED (no overlap at runtime)
Question: CourseObjects / CourseObjectPalette (bank 6 $8980-$8B10, shared by scroll modes 1 and 2) write
CourseMarkerArmed $AE-$B1, CourseMarkerXHi $B6-$B9, CourseMarkerX $BE-$C1 and CoursePalA/B/Last $A9-$AB.
Do they write these slots in BMX (mode 1)?
- Static: markers need CourseObjType[id] == 1. Palette slots need a type in 1..$7F (type 0 goes to the
  attribute patch; a negative type exits at CourseColumnDone).
  - BMX object stream: $97C2 (ScrollColumnsMode1), 642 records up to the bank-6 code at $9F4A. Every
    record has type 0 (96 records) or $FF (546 records). None is in 1..$7F.
  - Skating stream ($A56B): 106 of 108 records are type 1.
  - CourseSpawnList ($AC/$AD) writes in CourseObjectTiles are gated by ScrollMode == 2.
- Conclusion: in BMX these routines never write $A9-$AB, $AE-$B1, $B6-$B9 or $BE-$C1. The overlap is a
  skating-only use of the same zero-page bytes.
- Runtime (evidence/index-watches.txt batch3/batch2):
  - BMX run: CourseObjects was entered 12 times and claimed 7 slots; CourseObjectPalette was entered 11
    times. There were 0 executions of the marker store (0x189F6 = phys 100854) and 0 of the palette-slot
    stores (101053 / 101070).
  - Skating control run: the marker store executed once (X=2).
- Commentary fix: CourseObjects' generated "Writes:" list names CoursePatternVramHi/CoursePatternMode ($B6/$BE,
  BMX-scope names); in this routine they are CourseMarkerXHi/CourseMarkerX (skating only).

## 3. Table addresses inside code - all are index-offset aliases; real extents established
Method: static index domain per consumer (gates, loop bounds, producer tables), packing against neighbouring
tables, and index-register watches at the consumer instruction. Full output: evidence/static-checks.txt.
No real extent touches a Code region. There is no classification conflict.

| Symbol (base) | Index domain (evidence) | Real bytes |
|---|---|---|
| TeamLogoAddrHi $8EDE | X=$0A-$12, observed (team screens, both players) | $8EE8-$8EF0 |
| TeamLogoAddrLo $8EE7 | same | $8EF1-$8EF9 |
| PipeFrameFlags $96CF | rider frame $0D-$5F (observed $0D-$4E) | $96DC-$972E |
| PipeFallFramesA $9CC1 | X 4..12 (static PipeBodyStoreAngle) | $9CC5-$9CCD |
| FootBagDigitTiles $A215 | ASCII '0'-'9' (observed $30-$35) | $A245-$A24E (= FootBagDigitSprites) |
| SurfFaceTileIndex $B701 | 2..7 (static; observed 2-4) | $B703-$B708 (+2 triangular offsets 02 04 07 0B 10 16 into SurfFaceTiles $B709) |
| AnimTileIndex $8258 (bank 1) | frame $0D-$5F (round-2 L-R1R-09; observed) | $8265-$82B7 |
| FrameBoxAddrs $87AA (bank 2) | X=1..$25 (ldx #1 .. cpx #$26) | $87AB-$87CF = exact Observed data block 34747-34783 |
| SkateLeanFrames $D66C | X = SkateFrames values $D0-$E5 | $D73C-$D751 (values are SkateFrames indexes) |
| TextSpecialSlot $EE29 | X=$3E-$4D (cpx gates) | $EE67-$EE76, ends right before HighScoreOffsets $EE77 |

How the half pipe rider-frame domain $0D-$5F was established:
- The five per-frame tables have the same stride, $53 = 83 entries: Flags, DY ($9722), DX ($9775),
  AnimTileIndex and AnimTilePageGroup ($82AB).
- They have $FF sentinel entries at identical frames: 40-41 and 64-66.
- Each table, starting at base+$0D, packs end-to-start with the next.
- AnimTilePageGroup ends exactly at the code at $830B.
- The observed frames were 13-78.

Further labels that are also aliases and are not marked so in the source:
- TeamCursorOam: real bytes $8EFA-$8EFD (X 4..1).
- SpriteListStart: real bytes $8EFE-$8F06 (X 11..19 = loader block + 1).
- PipeFrameDY: real bytes $972F-$9781.
- PipeFrameDX: real bytes $9782-$97D4.
- PipeBackwardsBoard: real bytes $97D5-$97DD (board frame 4..12).
- PipeFallFramesB: real bytes $9CCE-$9CD6 (9 entries, not 13). It is followed directly by PipeTrickRates $9CD7.
- PipeRiderAngleFrame: real bytes $97DE-$97E6.
- PipeWallX: real bytes $97E7-$980D (positions $5E-$84).
- FootBagLetterTiles: real bytes $A22B-$A244, ASCII A-Z.
- AnimTilePageGroup: real bytes $82B8-$830A.

Comments in the current source that are wrong:
- The FootBagLetterTiles note says "$A215 + code ... entries $41+ = $A256". The digit path is only taken for
  codes below $41.
- The PipeFallFramesA constant says "angles past 8 read PipeFallFramesB". Indexes 9-12 are A's own bytes
  $9CCA-$9CCD.
- PipeFallFramesB says "13 bytes".
- The TextSpecialSlot constant says "lines $28+". The domain is $3E-$4D.
- The TeamLogo constant comment says "index (team number)". The index is graphics block $0A-$12, logo k = X-$0A.

Correction made during this pass: $97D5-$97DD is not unexplained. An operand scan found `lda PipeBackwardsBoard,x`
($97D1, HalfPipeDrawSkater $93AE) with X = board frame 4..12, so its real bytes are $97D5-$97DD. The values
0A 09 08 07 06 05 04 0C 0B are a mirror mapping: 4->10 ... 10->4, 11<->12.
- This closes the bank-0 packing: DX ends at $97D4, then PipeBackwardsBoard, PipeRiderAngleFrame and PipeWallX
  follow with no gaps.
- It also independently confirms the rider-frame maximum $5F.

## 4. Tentative names (half pipe records, PipeReact*, BmxLift/BmxWheelA/B, SurfRideFlag, DiskUncaught)
Deferred: naming only. Each is already described mechanically in the source with explicit uncertainty.
- A short watch cannot discriminate the semantic alternatives: wheel vs bike part, or spectator vs flag.
- The data flow is fully described, so the expected gain is wording only.

## Bookkeeping
Proposed / accepted bytes: 0 / 0. Scratch: scratch/r04r (batch logs/scripts only, <30 KB; jsonl deleted).
