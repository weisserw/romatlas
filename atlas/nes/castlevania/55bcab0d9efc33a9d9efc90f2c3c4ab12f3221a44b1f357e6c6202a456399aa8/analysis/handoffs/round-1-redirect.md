# Round 1 redirect handoff

- Status: **COMPLETE** (ready leads produced; not terminal). Input checkpoint **pass-001**
  (maps/pass-001.json = maps/current.json, byte_identical). Map, state, ROM, source unchanged.
- ROM classification at input: 93.42% (122467 / 131088; 8621 unknown).
- Attempt dir: [analysis/redirect/r1-a1/](../redirect/r1-a1/) (scripts, `closure_out.json`,
  `proposals.json` with exact ranges, README.md with commands). Static analysis only; no
  emulator server started, no runtime experiments, scenarios.json unchanged.
- Proposed bytes: 8615 of 8621 unknown (8483 high/medium confidence + 132 low-priority
  dead-code fragments). Accepted bytes: 0 (disassembly decides). 6 bytes have no proposal.

## Avenues investigated

1. **L-001 bank-5 object handlers (8K bank 10)** - resolved statically.
   - Bank context at `RunObjects`/ObjectHandlerTable dispatch: main play selects bank 5 at
     $CEC6; State08/09/0B call RunObjects with the saved bank 6. Discriminating test
     (`entries.py`): in bank 6 most of the 19 banked entries land in Data or on
     unofficial opcodes/mid-instruction; in bank 5 every entry starts with the handler
     idioms (`jsr WaitObjDelay`, `lda ObjState,x`, `lda $0560`) of the observed $8008
     handler. Banked handlers play sounds through fixed wrappers $FCCC-$FCF3
     (`lda #id; jsr PlaySound; ldy #5; jmp BankSelectY`); no bank-5 code calls
     PlaySound/BankRestoreSaved directly. Bank 6 is not a valid handler context.
   - Closure (`closure2.py`, 35 roots: 22 table entries, StageSpecialHandler/ObjT10/$ED38
     stage dispatch targets, $FC7E/$FC95 targets): 2780 instructions, 0 bad opcodes,
     0 overlaps, 0 overlaps with classified Data/Padding, aligned with all existing bank-10
     code. 17 inline $CA1E tables (incl. existing data $8790 = 5 words; ObjT13 table
     $9191-$919C has 6 entries).
   - Residual 1168 B: each gap starts at a data operand of closure code; $9A35-$9E46 =
     3-byte table + the three attract **demo scripts** (DemoScriptPtrs $9A38/$9B64/$9CCF,
     [frames, pressed, held] triples ending [01,FF,FF]).
   - Bank-5 code also enters unknown fixed fragments $EC64, $ED38 (second stage dispatcher),
     $EE4B, $EF74, $FA90 (+32 B table $FAAB), $FCCC, and **classified Data $EBF6** (conflict).
2. **Coordinator check, 8K banks 1-9/11 whole-bank data: CONFIRMED.** All `ldy #n; jsr
   BankSelectY` sites select only 16K banks 0/4/5/6; the bank-4 selects only call
   fixed-bank routines (LoadStageChunks, UpdateStageScroll). The only variable select
   ($FD21 loader) copies list data through fixed $FD5A. No JSR/JMP from any code region
   targets 8K banks 1-9/11; the only indirect jump is JumpTableDispatch (tables are inline
   in code banks); 0 observed instruction starts in those banks (scratch/covF.json).
3. **Bank 6 $90A1-$934B (683 B)**: two unreferenced routines ($90A1-$90C9 writes $9005,x
   -> dead/leftover; $910A-$917C VramQueue writer), their tables ($90CA, $917D) and text
   screen 4 ($918D = TextScreenPtrs[4]; RLE streams end exactly at $934B).
4. **Fixed $D87B-$DA1E (420 B)**: item/spawner records; record-length model from
   LoadColumnContents over all 191 pointers tiles the range exactly; ItemRecordPtrsB has
   only 63 pointers.
5. **$FF06-$FFF9 (244 B)**: DPCM sample $FF40-$FFF0 (PlayDmcSample $FD/$0B -> 177 B), $FF
   fills, and an unreferenced routine storing JoyPressed/JoyHeld pairs (probably a demo
   recorder).
6. **$CF5F-$CF7B (29 B)**: 2 B of trigger box record X=3 + unreferenced debug stage-skip
   routine (branches land on existing RTS $CF5A/$CF11).
7. Small fragments (L-003/L-013): operand-anchored data (bank 0 24-25, 405-406; $C453;
   $DEDF; bank 6 $87A0) and boundary-aligned dead entries (list in leads.json L-003).
8. Commentary questions: Q1 -> L-011; Q2 -> L-013/L-001; Q3 -> L-001 tables; Q4 -> bank-6
   code at $82A2/$84FF/$88B7/$8C9A is independently supported (observed $82A2; $849A and
   $8867 are entries of the bank-6 $F3B6 table; $8C9A is State0F fall-through), so no
   misclassification; the addresses only coincide with bank-5 targets. Q5/Q7 -> L-009/L-010.
   Q6: bank-6 $838A is an observed start; in bank 5 $838A is mid-handler, so a lag NMI with
   bank 5 mapped would misbehave (latent game issue, no classification impact).

## Ready leads (for the next disassembly pass)

| ID | Range | Proposal | Bytes |
|---|---|---|---|
| L-001 | bank 10 all unknown + fixed $EC64/$ED38/$EE4B/$EF74/$FA90-$FACA/$FCCC | code 5518+194, tables 226, data 1168+32 | 7138 |
| L-014 | $EBF6-$EBFC Data->Code; bank-10 $9E47-$9E48 Padding->Data | corrections | 0 |
| L-009 | $D87B-$DA1E | data (item records) | 420 |
| L-010 | bank 6 $90A1-$934B | code 156, data 527 | 683 |
| L-011 | $FF06-$FFF9 | DPCM 177, fill 22, code 45 | 244 |
| L-012 | $CF5F-$CF7B | data 2, code 27 | 29 |
| L-013 | 5 small fragments | data | 14 |
| L-002 | bank 6 (umbrella for L-010 + $87A0) | - | (689) |
| L-003 | 14 dead-entry fragments | code if reviewer accepts alignment evidence | up to 87 (low) |

Recipe: annotate from `analysis/redirect/r1-a1/proposals.json` ("ranges" = L-001 bank 10
with context prg_banks [10,11,14,15]; "other" = per-lead ranges with roles/notes); re-run
`closure2.py` against the new map to confirm no overlaps. Code is Static; record
reachability separately (most new fixed/bank-6 routines have no known caller).

## Rejected / blocked / deferred

- L-007 (bosses/ending): deferred - no proposal depends on it (bank-5 context settled
  statically); still the optional runtime confirmation for L-001 and an open progression
  checklist item (needs the room/checkpoint RAM contract).
- L-004/L-005/L-006: semantic only (0 B), not pursued this round.
- L-008: blocked on toolkit (TK-NES-004), unchanged.
- No proposal: lone unreferenced RTS bytes $C405, $C916, $CA5B, bank 13 $A1E4 and bank 6
  $9B8B `sec; rts` (R3) - an isolated return opcode has insufficient structure. 6 B.

## Recommendation

Run **another disassembly round** (then commentary + redirect) on these leads; expected
classification after acceptance ~99.9% (8483 B high/medium) up to 99.995% with L-003.

## Unresolved questions

- Whether State08/09/0B ever run with banked-type objects alive (would execute bank-6 bytes).
- Callers of the unreferenced routines ($90A1, $910A, $CF61, $FF06): probably removed
  development/FDS-era code; reachability unknown.
- Demo script termination: last triple [01,FF,FF] presumably ends the demo via Start.
