# Round 3 disassembly handoff (pass-003)

ROM classification: 99.99% (393207 / 393232 bytes classified; 25 bytes unknown). Status: incomplete.

Checkpoint: maps/pass-003.json -> generated/pass-003; analysis/pass-003-verification.json ok, byte_identical
(map sha256 c36cb8af12e91641bca1ee9ba4b4aaa9e16705f93460d73c6c69fec1b8a51a6f). Internal store analysis/state.json
(4,101 regions) = maps/current.json. No live server left running (scratch/p3/apply.py used one stdio serve session).

## What was applied (redirect batch r2-redirect-a, leads L023 + L024)
- Input pass-002 (1,727 unknown in 83 runs). Applied 1,702 unknown bytes (Code/Static 1,128, Data/Static 574) plus 23
  phase-1 Data bytes reclassified to Code (bank 25 $A033-$A03C; $B88C-$B88D and $B890-$B89A). 118 regions added, 2
  phase-1 Data regions removed and re-covered exactly (ids 3373 = 0x03203B+21, 3546 = 0x03389C+22):
  - 0x03203B+8 Data (table $A02B-$A032), 0x032043+10 Code (C19), 0x03204D+6 Data (inline FarCall Y=8 record $A03D-$A042).
  - 0x03389B+3 Code (JSR $C6FC at $B88B; C26b+C27a merged since the instruction straddles them), 0x03389E+2 inline Data,
    0x0338A0+11 Code (C27b), 0x0338AB+7 Data (table $B89B-$B8A1).
- All new regions Static. Code evidence states reachability not observed (0 starts in any traced scenario, incl. S23-S25);
  not claimed unused.

## Independent review (scratch/p3/review.py, plan.py, apply.py; plan scratch/p3/plan.json)
- Every item lies exactly in the pass-002 unknown complement (or, for the 23 reclassified bytes, in Data); items plus the
  25-byte residue = the full unknown set.
- Code: official opcodes only, linear decode ends exactly at each range end; every branch/JMP/JSR target is a classified
  or proposed instruction start (same bank, context pair, or fixed bank); non-terminal ends occur only at JSR $C6D6/$C6FC
  (2-byte inline) or JSR $C32D with LDY #$08 (6-byte record), each followed by the matching inline Data item; no
  inline-parameter JSR occurs mid-item.
- Data: no classified or proposed code branch/JMP/JSR lands inside any proposed Data item.
- Copy claims re-checked: C19 sequence + 6 inline bytes byte-identical to bank 24 $9803-$9812 code (classified).
- Weak items reviewed individually and accepted with reduced confidence:
  - C18 b24 $9F6E (conf 0.6): BPL +2 skips exactly LDA #0 onto RTS; bounded by Observed RTS $9F6D and the monotonic table
    $9F73; entry contract (caller's N flag) not demonstrated.
  - C02 b2 $81B9 (0.7): the classified SEC / JMP $81BA skips exactly this one CLC byte (carry-select entry pair).
  - C31 b25 $BAB3, C32 b30 $C0C5, C33 b30 $C0FD (0.7): short entries falling into classified code with matching idioms.
  - D03 b1 $BA1F (0.7, Data): every decode overlaps Observed code $BA24 or is a JSR into the RAM mirror.
- Label change: D62 ($CF53) left unlabeled; TeamBlockDstPtrs starts at $CF50 in the adjacent region, so a label at $CF53
  would be mid-table. Other proposal labels kept.
- Rejected items: none.

## Residue (analysis/pass-003-remaining.json, 6 runs, 25 bytes) - rejection review
| Range | Bank/CPU | Bytes | Decisive evidence | What it rules out / remaining reason |
|---|---|---|---|---|
| 0x0040EA-0x0040EB | b2 $80DA | 2 | two $60 bytes between RTS $80D9 and code $80DC; no reference | lone return opcodes lack structure; data alternative equally consistent |
| 0x005998-0x00599B | b2 $9988 | 4 | LDA #$80 / LDA #$00, then $998C reloads A; no reference, no copy | first load is dead; no discriminating constraint |
| 0x005B58 | b2 $9B48 | 1 | $60 between the EditedLineupPtrs end $9B47 and code $9B49 | could be a table pad byte or a stray RTS |
| 0x012175 | b9 $A165 | 1 | $60 after code ending $A164 (whose last bytes decode as an operand) | same as above |
| 0x03C0D0-0x03C0D4 | b30 $C0C0 | 5 | LDA $22 / STA $22 / RTS after data D59; no reference, no copy | effectless body; unlike C32/C33 it does not fall into a routine |
| 0x03F6D2-0x03F6DD | b31 $F6C2 | 12 | add-18 fragment with ADC $0D straight after STA $0C (carry/A misuse), between table $F6B6 and its duplicate $F6CE | could be a stale fragment of a duplicated block or data; no copy, consumer or reference found |
Strongest competing interpretation: all six are code-shaped fragments (stale build residue) rather than data. Code
claims would need a reference, an observed fetch, or a byte-identical classified copy. None exists, so the unknown
classification is kept. A revised test is possible only with new evidence, such as a pointer or fetch found in an unexplored state.

## Gameplay
Not driven this pass. Rounds 1-2 traced S1-S25: 11,221 starts in the last redirect, and the last two rounds found 0
fetches in residual bytes. The residue is 1-12 byte fragments with no consumer, so gameplay has no substantial prospect.
Scenario checklist analysis/scenarios.json unchanged (ending S06 visited; 6-team league completion deferred, L006).

## Leads
- Consumed / verified: L023 (173 bytes), L024 (1,529 bytes + 23 reclassified).
- Updated: L021 -> blocked. It holds the 25-byte residue. The missing fact is a reference or a discriminating structure.
- Produced: none.
- L006 remains deferred.

## Per-bank classified/unknown
b2 8185/7, b9 8191/1, b30 8187/5, b31 8180/12; all other PRG banks 8192/0; header + CHR fully classified.
Code 90,161 (Observed 53,697 / Static 36,464), Data 171,974 (incl. 16-byte header), Image (CHR) 131,072.

## Toolkit
No new toolkit issues found this pass (existing TI-001, TI-002 in analysis/toolkit-issues.md).

## Next
Commentary for the pass-003 regions (118 new). Redirect may decide whether the 25-byte residue warrants further work.
That work would need new evidence; otherwise this is a terminal incomplete state with exact residuals.
