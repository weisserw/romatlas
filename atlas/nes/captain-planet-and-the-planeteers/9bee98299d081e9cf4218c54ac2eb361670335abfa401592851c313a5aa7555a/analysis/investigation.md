# Investigation ledger

## Round 1 disassembly (pass-001, 2026-11-23)

### Hardware/bank model (static evidence)
- MMC3 PRG mode 0 throughout: the only $8000/$8001 writes are at $FFD7 (`sty $8000; iny; sta $8001`), callers pass Y=6 ($FFC5, R6->$8000),
  Y=7 ($FFCE, R7->$A000) or Y=0..5 (CHR, $FF9E). Bit 6 never set => $C000=bank 14, $E000=bank 15 fixed.
- Execution banks at $A000: 3, 6, 7, 8, 12, 13 (task $052B values / `lda #N; jsr $FFCE`). Banks selected only for data: R6 0,1,2,4,9,10,11 ($F906, $C1D4/$C1F0 via $0706/$0707),
  R7 5 (NMI sound call). No JSR/JMP from any discovered code targets $8000-$9FFF; all 112 task-entry words are >= $A000.
- Observed contexts never show execution in banks 0,1,2,4,5,9,10,11 (all scenarios). These 8 banks were annotated Data (whole bank; bank 0 split for the password table).
- Cooperative task system: $FEEE CreateTask (inline .word entry), $FF47 Yield, $FF63 Sleep(A), $FE99 Kill, dispatcher $FE9F (NMI) -> JMP ($0000) at $FEE0,
  per-task bank $052B,x. Inline-argument helpers: $FEEE/$FF57/$FD7A (+2 bytes), $FB19 (palette string to byte >= $80). $C000-$C129 is a JMP vector table; each code bank has
  JMP-stub import tables (bank 3 $AFC8, 6 $BB16, 7/8 $A000+, 12 $A019, 13 $A00F) and some $A000 entry tables.

### Method
- Toolkit discovery stops at every JSR on MMC3 (TI-001), so tools_local/trav.py (bounded recursive descent) was used:
  roots = vectors + observed contextual roots + task-entry words + JMP-stub/entry tables; JSR assumed to return except inline helpers;
  known-flag branches (lda #imm/clc/sec then branch) followed one way; unofficial opcodes/BRK stop a path (none remain on accepted paths); 0 overlap conflicts.
- Gap classification tools_local/gapclass.py: ASCII text runs -> Text, uniform runs >= 16 -> Data, indexed-load-referenced blocks with no plausible code chain -> Data; everything else left Unknown.

### Outcomes
- Code 36564 bytes (Observed/Static split per region), Data 73185, Text 2051, Image (CHR) 131072. Unknown 19288 (banks 8: 6757, 13: 7112, 3: 1438, 6: 1262, 15: 1227, 7: 758, 14: 422, 12: 312).
- Leads L001-L006 in analysis/leads.json. Assumptions needing review: same-bank task inference (L005), heuristic gap Data/Text (L004).

## Redirect round 1, attempt r001-a (analysis/redirect/r001-a/)
- Question: what executes in the bank 8/13 gaps, JMP-table targets (banks 3/6), fixed-bank task entries, and what the remaining non-code bytes are.
- Method: decode spawn/init entry tables (explicit TaskBank constants = entry contract), bounded static traversal (tools/trav2.py, pass-001 model), overlap check against
  the map, consumer/extent proofs for residual runs (resrefs.py, sprchain.py), runtime corroboration via task-table sampling and short traces on password routes (artificial).
- Outcomes: +~12.1 KiB static code with 0 instruction overlaps except 2 phase-1 Data regions that end/contain code (L004 counterexamples);
  every runtime task resume PC / observed fetch in unknown bytes coincides with traversal instruction starts; residual 6958 B classified by disposition in ledger.json.
  GameOverScreenSetup is the post-CP "WELL DONE / PASSWORD" screen; $F3D9 tasks are the credits sprites (both observed, artificial starts).
- Leads: L001-L005 ready (updated), L006 superseded, new L007 (data batch), L008 (fragments, low).

## Disassembly round 2, pass-002
- Applied redirect r001-a batch (L001-L004, L007, accepted part of L008) after independent checks of the entry contracts and table bounds
  (see analysis/leads.json pass-002 attempts). Tool: tools_local/pass002_annotate.py (input: trav2.py output with roots_all.json).
- Result: unknown 19288 -> 5 bytes. Code 48907, Data 80115, Text 2061, Image 131072.
- Remaining unknown (L009, low): bank 13 $BA5D-$BA5F 'jmp $B4C2' after an rts; bank 15 $FB69 and $FDC0 single $60 after an rts.
  Checked: no 16-bit references anywhere in PRG, no observed fetches; code vs data stays materially ambiguous (lone return/jump opcode).
- L008 accepted routines are Static code with reachability unknown (no caller found); they are complete, self-consistent routines or
  fall through into classified code, or well-formed slots of JMP-stub tables.
- Gameplay: L5 CP progression attempt did not reach the boss; boss/game-over still need the CP health/position RAM contract.

## Disassembly round 3, pass-003
- L009 decided with ONE criterion for all seven candidate bytes: unreferenced, never-fetched bytes are Static Code only if they carry an
  independent structural constraint beyond legal decoding (an operand resolving to an established code entry in the same effective bank,
  or a coherent multi-instruction routine contract). A lone single-byte RTS has none (evidence-policy.md: "an unreferenced return opcode
  ... lacks sufficient structure on its own"); the competing reading (1-byte data remnant with no reader) is equally unsupported, so it is
  materially ambiguous.
  - L009a bank 13 $BA5D-$BA5F `jmp $B4C2`: accepted, Static Code (operand = DebrisSpawner entry, duplicate of jmp at $BA59). Reachability not claimed.
  - L009b bank 15 $FB69, $FDC0: rejected, stay Unknown.
  - Consistency: bank 6 $A175 ($0C185) and $A18B ($0C19B), accepted in pass-002 on a nonexistent bank-3 parallel (rtsrts.py's bank-3 "ref"
    is bank 3's own bne to its own $A175), returned to Unknown; APU init routines $A176/$A18C stay Code.
  - consumers.py rerun: no hits. Reopen only with a reference/fetch or an independently justified source-pattern model.
- L010: region 1500 split; EmptySpriteDef8 ($1121E-$11222) labelled Data with consumer evidence. 0 B classification change.
- Unknown now 4 B: $0C185, $0C19B, $1FB79, $1FDD0 (all lone unreferenced RTS bytes between classified routines).
