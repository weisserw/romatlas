# Investigation ledger

## Round 1 disassembly (2026-11-22) - checkpoint maps/round01-final.json

Method summary (all tools in analysis/tools/, evidence in traces/):
1. Gameplay traces (aggregate, capacity 0) over boot, menus, all six practice events, high scores,
   compete registration and results transitions -> traces/round01-contexts.json (10,061 contextual
   opcode addresses; 9,779 ROM) and traces/round01-edges.json.
2. closure.py: observed fetches as roots; static successors (branches, JMP abs, JSR targets, fallthrough)
   with UxROM bank rules (fixed bank 7 at $C000; switchable bank = observed bank; any store to
   $8000-$FFFF or JSR from the fixed bank drops switchable context). Pruned static paths reaching BRK,
   unofficial opcodes or bank ends; suppressed fallthrough of forced-taken branches (LDA/LDX/LDY #imm + Bxx,
   CLC/BCC, SEC/BCS, CLV/BVC). Result: 579 code regions, 24,686 bytes (19,940 observed, 4,746 static).
3. readscan.py: ROM Read+Instruction streams in bounded windows (boot, menu, compete registration, each event
   load, trophy screen, banks 0/7 separately) -> traces/round01-reads-*.json. Read runs >= 8 bytes not
   overlapping fetched instructions/code -> Observed data. Short runs (<8, 524 bytes) rejected
   (traces/round01-rejected-short-read-runs.json) because dummy reads (after RTS/taken branch) and
   mis-banked sound-pointer reads were demonstrated (0x3A2F-0x3A32 is code read via sound routine $D75F).
4. Uniform runs >= 16 bytes and ASCII English text (data_plan.py) -> Static data.
5. dense.py (K=8): intervals >= 16 bytes where no offset starts an official-opcode chain >= 8 bytes before
   an unofficial opcode/BRK; excluding JSR/JMP/branch targets from classified code; 8-byte trim after code
   -> 617 Static data regions, 36,003 bytes.
6. RamRoutineImage 0xFC54-0xFCD9 (data, upload image of RAM code) and DefaultHighScores 0xFCDA-0xFD2F.

Structural vs reachability: Static code regions are control-flow successors of observed code and are not
claimed executed. Static data regions claim non-code only; no consumer/format claimed.

Known residuals / attempted leads: see analysis/leads.json L-R1-01..08.
- L-R1-01 blocked (toolkit TK-04): 0x400E-0x400F + 0x1C010 cross-window instruction.
- L-R1-02 verified: BIT-skip overlapping entries handled as one linear decode.
- Others ready for redirect.

Remaining unknown: 21,326 bytes in 784 gaps (traces/round01-unknown-gaps.json). Largest: 0x19073 (267,
bank 6), 0x15B8D (213, bank 5), 0x9F12 (209, bank 2, 8x$FF then code-like), 0xFA8D (206, bank 3).

## Round 2 disassembly (input maps/round01-final.json b4a8fa61; output maps/round02-final.json 53e08b7b)

1. Prerequisite fix: analysis/tools/closure.py forced_branch() now requires the preceding LDx#/CLC/SEC/CLV to be a
   decoded instruction start (same fix as r01-a1 closure2.py; pre-fix copy scratch/closure.py.r01bak, disposable).
   Re-running the closure on the final map yields no instruction outside classified Code except the 00 byte at
   bank 5 $82D2 (fallthrough of `iny; bne` loop at $82D0 after 256 iterations) which is the start of the
   L-R1R-06 x-adjust table (indexed-read base $82D2,x) - treated as data, not a conflict.
2. Lead verification (analysis/tools/r02_verify.py): all 25 L-R1R-01 code ranges decode with official opcodes only,
   no BRK, exact range ends, ending in RTS/RTI/JMP (22) or flowing into classified/proposed code (3); no overlap with
   classified bytes except the planned 0xC01E correction. Data proposals: no byte over classified or proposed code;
   15,192 unique unknown bytes (overlaps with existing Data skipped, existing classifications preserved).
   Spot checks: TABLES gaps $B755/$827F/$A861/$8208/$8265 bytes visibly tabular; code starts disassembled (scratch/dis.py).
   Applied by analysis/tools/r02_apply.py (Observed only where r01-a1 boot/surf-end fetch files list every instruction
   start of the sub-run; surf-end runs marked artificial-scenario). Checkpoint maps/round02-leads.json (96.30%).
3. Residual gap screen (analysis/tools/r02_gapxref.py, r02_screen.py, r02_gapdata.py): per gap, static references from
   classified code (bank-7 references into $8000-$BFFF checked against every bank: all such targets are classified
   bank-0 code) and a code-plausibility screen: every start offset decoded; a chain is viable only if it reaches
   RTS/RTI/JMP or a classified code instruction start without unofficial/BRK, absolute operands in $0800-$1FFF,
   $2008-$3FFF or $4018-$7FFF (never used by any classified code), ROM writes other than STA $8000(,Y), or CF targets
   into Data/mid-instruction. 181 gaps (4,480 bytes) with no viable chain or only <=3-instruction incidental chains,
   no CF reference, and (if after code) a terminating previous instruction -> generic Data, Static 0.75.
   3 bank-6 gaps ($8FD4 89, $9E16 46, $9E5F 82) with 5-6 instruction chains starting on $60/$68 record bytes were
   accepted manually as continuations of the neighbouring record streams (tile strings / 3-byte records).
   This classifies all bank 6 course-area residuals as generic data; L-R1R-11 semantic format is no longer needed
   for classification.
4. Gameplay: S13 compete-some reached naturally (event selection, NEXT EVENT: FOOT BAG); traced 756 frames, 1,576 ROM
   fetch addresses, none on non-Code bytes. Remaining unknown fragments are unreferenced, so more play is unlikely to help.

Remaining unknown: 149 bytes in 18 gaps (lead L-R2-01): unreferenced code-like fragments (bank 0 $8229/$8A85/$8CED/
$87CB/$9B46/$BEA3, bank 2 $87DF zeros after non-terminating code, bank 7 $D7B9/$DEBF/$EEB5/$F072/$F14A/$F2BA),
bank 7 $F883 (45) and $FFDF-$FFF9 (27) with 5-6 instruction chains, bank 0 $8008 (ambiguous entry), TK-04 bytes
0x400E-0x400F/0x1C010.

## Round 3 disassembly

1. Ready leads L-R2R-01..04 re-verified and applied (analysis/tools/r03_apply.py; proposals from
   analysis/redirect/r02-a1/evidence/proposals.json). Checks performed: BinToBcdTable = BCD(0..99) exactly, next byte $A2;
   $EEAF-$EEB6 == PlayerNameOffsets $EFC9 (00 07 0E 15 1C 23 2A 31), consumer `ldx $EEAF,y` at $D10F with CurrentPlayer
   capped at 7 (TeamSelectFrame cmp #$07); FrameHandlerTable $87B2 last words -> $B9F1/$E2DA/$8A6D, ends before
   NameEntryFrame $87D0; BMX OAM copy `ldx #$1B / lda $BE8C,x / sta $0200,x` at $BDE8 bounds $BE8C-$BEA7;
   $F072-$F079 unofficial opcodes in every alignment; QueueSfx slot series $10/$14/$18/$1C; CallBank chain bytes
   A9 00 2C A9 01 2C ... A9 07 4C 69 FF. Region 578 re-split per recipe (Observed provenance kept on $F2BF-$F2F0;
   $F2BD-$F2BE fetch observation recorded in the new Static region's evidence).
2. Bank-0 $8008 (L-R3-01): sibling comparison of $8000-$802F in banks 1-6 shows an identical header structure
   (bus-conflict table, 3-byte JMP entry instructions, " (C) COPYRIGHT 1987 RARE LTD." text). This independent
   structural constraint, absent from the r02-a1 rejection, places bank 0's 20 93 D8 in the entry-0 instruction slot.
   Classified Code Static 0.55; the JSR-into-text continuation and lack of a caller are recorded; no reachability claim.
3. Result 131085/131088; only TK-04 bytes remain (0x400E, 0x400F, 0x1C010).
