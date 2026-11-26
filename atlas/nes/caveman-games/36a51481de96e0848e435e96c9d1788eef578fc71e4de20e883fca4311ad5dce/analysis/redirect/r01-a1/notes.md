# Redirect attempt r01-a1 (round 1) — research record

Input checkpoint: pass-001b (maps/pass-001b.json == maps/current.json; analysis/state.json). No map/state change.
Tools in this directory (run from the workspace root): match.py, refscan.py, l001_reloc.py, reloc_control.py,
dino_race.py, saber_race.py, covdump.py. Exact unknown complement + dispositions: unknown_ledger.json.

## L-001: bank 3 $A60C-$AEF8 (file 0xE61C-0xEF08, 2254 unknown bytes plus 31 B of already-Data pieces)
Claim: the range is not code of this cartridge. It holds a stale, corrupted copy of a 6502 program that was
assembled for another address ($86EB-$8FD7) and memory map. Proposed role: generic Data (Static).
Evidence, from independent constraints:
1. No references. No classified Code in bank 3 or the fixed bank has an absolute operand in $A600-$AFFF
   (l001_refs_bank3_7.txt is empty; refscan.py A60C AEF9 3,7). The whole-ROM scan finds hits only in other
   16K banks, which use their own $A000 windows. No word table in bank 3/7 points to it as a jump-table entry
   (only noise words in generic data). No observed fetch in it (scratch/cov.json plus this attempt's
   cov_session2.json, which now includes a finished Dino Race).
2. A consistent relocation. A linear decode gives 66 JSR/JMP targets in $8000-$BFFF. Shifted by +$1F21,
   61 of them land inside the range and 47 land exactly on the byte after an RTS (l001_reloc.txt). The next
   best shift gets 17. As a control, real bank-3 code gets its best score at shift 0, and data/tail ranges
   score about 0 (reloc_control.txt). Examples: JSR $8F80 -> $AEA1, a wait-for-NMI-counter routine
   (PHA / LDA $0E / CMP $0E / BEQ / ... / PLA / RTS). Its NMI-like counterpart at $AEDA does PHA / INC $0E /
   LDA $2002 / scroll 0 / PLA / RTI.
3. It would malfunction in place. Unshifted, the targets land in unrelated Dino Race code: $8F80 is mid-routine
   (LDA $04A3,X inside the routine at $8F6C). The program keeps variables at $89D0-$89DA (STA/INC/DEC $89Dx,
   27 write instructions). After the shift these are its own embedded zero bytes at $A8F1-$A8FB. In this cartridge,
   writes to $8000-$9FFF are MMC1 serial-register writes. It also polls $7F00/$7F01 (LDA $7F00 / AND #1|#2 /
   BEQ, a handshake loop), the same development-port range as the unreferenced fixed-bank stubs at
   $CEDC-$CEE7 (STA $7F80/$7FC0/$7F40).
4. Corruption. Several bytes are replaced by a copy of the preceding byte, which breaks instruction streams.
   Examples: file 0xE7E7 `8D D9 D9` (expected `8D D9 89`), 0xE847 `AD D1 D1`, 0xE86B `8D D0 D0`, and
   wait-loop `F0 F0` (expected `F0 FC`) at 0xEEB6. The linear decode contains 26 undefined opcodes.
5. No clean copy elsewhere. Longest matches with the rest of PRG are 10 and 8 bytes (match.py).
Strongest competing reading: "Code of a foreign program". No CPU mapping of this cartridge makes it run as
written. Its addresses ($86EB+) are ROM here, and NES RAM ($0000-$07FF) cannot hold it at those addresses.
No copier references it. Classifying it as Code would only describe a hypothetical other machine, so non-code
(generic Data) is the supported role. The bytes' origin (an earlier tool or program build) is an inference;
the classification does not depend on it. Boundaries are already fixed by classified Data on both sides and
at the three inner pieces.
Recommended annotation: one Data region file 0xE61C-0xEF08 (it may merge the inner pieces $A68F/$A8F5/$AC8F),
Static, confidence about 0.85. Suggested label DR_StaleForeignImage. Comment: "6502 fragments assembled for
$86EB (+$1F21 relocation), corrupted, unreferenced; not executable here." Commentary may document the decoded
routines as remarks.

## L-007: fixed bank 7 Data that is code
- IRQ split handlers. IrqHandler ($C000, already static Code) RTS-dispatches through 7 words at
  $C072-$C07F: $C08C,$C080,$C0A4,$C090,$C080,$C0A4,$C090. The targets (+1) $C081/$C08D/$C091/$C0A5 all
  decode cleanly. $C081 (LDA IrqScrollX1,Y / STA PPUSCROLL / LDA #0 / STA PPUSCROLL) falls through the NOP at
  $C08C into $C08D JMP $C036. $C091 and $C0A5 end in JMP $C036. $C036-$C047 (IrqSplitIndex += 2; if it
  reaches $0E, STA $4015 #$0F stops the DMC) falls into $C048, the established @irqDone epilogue
  (PLA/TAY/PLA/TAX/PLA/RTI). The single NOPs at $C080/$C090/$C0A4 sit directly before RTS-dispatch entry labels.
  NMI_Common at $C0BE has the same pattern (it starts with a NOP). Proposal: Code $C036-$C047 (file
  0x1C046-0x1C057) and $C080-$C0BD (0x1C090-0x1C0CD); Data words $C072-$C07F (0x1C082-0x1C08F). These bytes
  are currently inside the 76-byte Data region "IrqSplitHandlers" at $C072. No byte-identical copy exists
  elsewhere. Reachability: IrqHandler itself has never been observed, and only bank 1 executes CLI. Bank 1's
  IRQ vector is a bare RTI, so the split handlers are static-only, like their dispatcher.
- $F26A-$F277 (0x1F27A-0x1F287): byte-identical to the 14-byte QueueSfx body that is established code in
  banks 0-6 (file 0x317F, 0x59FA, 0xAB62, 0xF276, 0x11A1D, 0x16BD4, 0x1B2BD). It sits between fixed-bank
  sound-driver routines (RTS at $F269; the next routine at $F278). Proposal: Code, static, unreferenced (no
  JSR/JMP/word to $F26A or $F269). The duplicate-stub precedent applies.
- $C62F-$C634 (0x1C63F-0x1C644): LDA #$1A / STA PPUMASK / RTS, byte-identical to observed bank-0 code at file
  0xA1 (rendering on). It is adjacent to the observed rendering-off routine at $C635. Proposal: Code, static,
  unreferenced.
- $C61A-$C61B (0x1C62A-0x1C62B): LDX #$03, then falls into the observed $C61C wait loop. This is the same
  preset-X idiom as LatchBothPads ($C117, observed) and L-009. Proposal: Code, static, unreferenced.
- $C5F1-$C5F9 (0x1C601-0x1C609): wait until FrameTimer ($0625) = 0, then FrameTimer = A, RTS. It follows an
  RTS, and the observed routine at $C5FA follows it. The BNE targets its own first instruction. Proposal:
  Code, static, unreferenced. Weaker: no byte-identical twin, so the evidence is internal consistency plus
  the shared variable.
- $CEDB-$CEE7: a lone RTS after JMP $CC30, then three STA $7Fxx / RTS stubs. Keep as Data, or the
  disassembly phase decides. A lone RTS lacks structure. The stubs write the development-port range and are
  unreferenced. Low value, already classified.

## L-008 (refines L-002): bank 2 $95E6-$9613 (46 unknown)
- $95E6-$95E7 hold the X offsets read by FS_DrawMarker `ADC $95E6,X` (X = 0/1). Proposed role: Data.
- $95E8-$9613 decodes as 24 instructions with no gaps:
  LDA #$0C / JSR PpuBufStart($CD71) / LDA #$23 / JSR PpuBufPut / TYA / JSR PpuBufPut, then a 12-iteration
  loop (LDY #$0B .. BPL). Each iteration emits exactly one PpuBufPut: tile $29 for a full 8-px cell, else
  $21+remainder. That matches the 12-byte length header, an independent semantic check. The stack contract:
  the loop pulls one byte more than it pushes, so the meter value must be on the stack, not under a return
  address. Entry is therefore "PHA; JMP $95E8" (a tail call), and RTS returns to the caller's caller.
  No JSR/JMP/word reference exists in bank 2, and no byte-identical copy exists in the ROM.
  Proposal: Code, Static, unreferenced (stack-argument contract documented). If the disassembly phase rejects
  static code without a caller, the fallback is to leave it Unknown. It is not Data: the decode is clean and
  internally consistent.

## L-009: bank 3 $9179-$917E (6 unknown)
LDX #0 / JSR DR_SetVelocity($917F) / INX, then it falls into DR_SetVelocity. It follows an RTS at $9178.
This is the same idiom as LatchBothPads ($C117: LDX #0 / JSR LatchPad / INX, observed). No reference.
Proposal: Code, Static, unreferenced.

## L-010: bank 5 $957C-$9587 (12 unknown)
Extent proof: CL_Bracket6 indexes CL_Bracket6Pairs ($9570) by 2*RoundIndex for RoundIndex 0-8 (CMP #$09),
which covers $9570-$9581. It then calls CL_ApplyPlacements with pointer $9582, which reads NumPlayers bytes.
CL_Bracket6 is the 6-player entry (bracket table `.word ..., CL_Bracket5, CL_Bracket6`), so the list is
$9582-$9587, ending immediately before CL_PairFromWinners at $9588. Contents are consistent: pairs
80 81 / 83 82 / 43 42 / 40 41 / 85 86 / 45 46 and placements 84 44 87 47 88 48 (bit 7 = winner,
$4x = loser, low nibble = bout ≤ 8). Proposal: Data, split as $957C-$9581 (pair table tail) and $9582-$9587
(CL_Bracket6Places).

## L-011: dead-code-looking Data in banks 3/5/6 (already classified; no completion effect)
- bank 5 $8CBD-$8CC7 (0x14CCD, 11 B): a CMP/BEQ chain ending in RTS, a variant of the adjacent
  CL_IsGuarding idiom. The branches land on its own RTS.
- bank 6 $AE0C-$AE15 (0x1AE1C, 10 B): CMP #$30 / BCS $AE36 (an established label in the same routine) /
  STA DvVar043D / RTS. It sits after an unconditional JMP.
- bank 3 $8984-$8989: STA FrameTimerHi / JMP $8987 (jumps to itself, a hang).
- bank 5 $8B53 and bank 6 $8DA8: lone RTS bytes.
Optional proposal: reclassify the two multi-instruction fragments as unreferenced static Code. Keep the lone
RTS bytes and the self-jump as Data. Priority is low because only fidelity/readability changes.

## Gameplay (L-006 / S14): Saber Race and Dino Race finished naturally (practice, 1 player)
Controls were derived from the code:
- Dino Race: tap A (DrTapEnergy +$18 per press) and press B (jump) when DrObstDistHi($0478) = 0 and
  DrObstDistLo($0476) < $20. Right = club, Left = back up.
- Saber Race: alternate A every 2 frames (each A state change is one running step). Press B within 16 px
  before obstacles of kind 1 (SR_ObstacleStart/Kind at file 68995/68984; position = SrScrollX + runner X
  + $100).
Results:
- Dino Race: course reached $0F00 for both racers after about 6900 frames from dr_start; the results screen
  showed seconds. Snapshot: dinorace_results.json. Frame: frames/dr_after400.png.
- Saber Race: both runners reached the goal (SrP1Done/SrP2Done = 1, SrCourseEnd = 1) at about 2700 frames;
  the results screen appeared. Snapshot: saberrace_results.json. Frame: frames/sr_fin300.png.
- Coverage: 537 instruction starts not in the round-1 union (cov_session2.json; 345 from Dino Race and 192
  from Saber Race). All 537 lie inside Static-provenance Code regions, with 0 fetches outside Code, so the
  existing classification gets reachability confirmation. The 11 L-004 roots were still not executed
  (DR_AdvanceCourse at 0xD146 included).
Budgets: per-frame emulator.step calls; trace was off while playing and switched on near the finish
(capacity 0).
