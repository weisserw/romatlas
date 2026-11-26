# Round 3 — disassembly handoff

- Input: pass-002. Output checkpoint: **pass-003** (maps/pass-003.json == maps/current.json; generated/pass-003;
  analysis/pass-003-verification.json ok, byte_identical true; store analysis/state.json). input/game.nes unchanged.

ROM classification: 100.00% (262160 / 262160 bytes classified; 0 bytes unknown).

## Applied (all re-checked; none rejected). Script: tools/r03_apply.py
| Lead | Bank / CPU | File range | Change |
|---|---|---|---|
| L-013 | bank 3 $860E-$8614, ctx [6,7,14,15] | 0xC61E-0xC624 | Data -> Static Code 0.8: LDX #1 / JSR DR_DrawRacers / LDX #0, falls into DR_DrawRacers $8615 (suggested label DR_DrawBothRacers, unreferenced). DR_ActionRiderX = $85BB-$860D (83 entries). |
| L-014 | bank 0 $8547-$85B3, ctx [0,1,14,15] | 0x557-0x5C3 | Data $8547-$854C FameTitleOffsets (6 B); Text $854D-$85B0 FameTitles (5 x 20 B); Data $85B1-$85B3 `JMP $85B1` self-jump item (unreferenced). |
| L-017 | bank 0 $811A, $817A, $81C3 (8 B each) | 0x12A-0x131, 0x18A-0x191, 0x1D3-0x1DA | Data -> Static Code 0.7: LDA #1 / STA FrameTimer / STA FrameTimerHi, skipped by preceding BEQ +$0B, falls into the wait loop; unreferenced. |
| L-018 | bank 2 $8363-$8372, ctx [4,5,14,15] | 0x8373-0x8382 | Data -> Static Code 0.7: self-modifying PPUMASK emphasis cycler (STA $8364 = own operand); cannot work from ROM; unreferenced. |
| L-016 | 21 Static Code regions (banks 0, 4, 5) | e.g. 0x154A1 CL_Bracket3 (split), 0x1555A CL_Bracket6, 0x15598 CL_PairFromWinners | Provenance Static -> Observed (16 whole, 5 split; 310 B) from natural multi-player runs (S15). Split boundaries: 0x270, 0x983, 0x995, 0x11147, 0x154A1 regions. |

## For commentary (commentary/ not edited)
Changed ranges to (re)write: bank 0 $811A, $817A, $81C3, $8547-$85B3; bank 2 $8363-$8372; bank 3 $85BB-$8615
(DR_ActionRiderX extent note + new entry $860E). L-015 (IrqHandler CLI/IRQ-vector reason in bank7.asm header and
uncertainties.md) is still ready and commentary-only. L-016 changed only provenance/region splits; record CL_Bracket3/6
reachability (natural, multi-player) where the commentary says "not executed".

## Open (no classification effect)
- L-004: 7 roots unexecuted (reachability only). L-005: semantic roles in generic data (deferred).
- Verification extra: r02-a1 audit.py re-run on the new map: A/B/D/E/G 0; remaining F hits are coincidental bytes in
  dispositioned non-code. Toolkit: no new issues found. No processes left running.
