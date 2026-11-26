# Redirect round 1, attempt r1-a1

Static, bank-aware research only (no emulator server started; ROM/map/state unchanged).
Run scripts from this directory (symlinks input/maps/scratch point to the workspace).

- lib.py: decoder over maps/current.json (op6502 table from scratch/).
- labels.py -> labels.json: label -> CPU address from commentary/source/game.asm.
- entries.py: ObjectHandlerTable banked entries decoded in bank 5 vs bank 6.
- closure2.py ROOTS: bank-5 static closure (context [10,11,14,15]; descends unknown fixed-bank
  fragments entered from bank-5 code; parses $CA1E inline tables). Command used:
  `python3 closure2.py 8008,83F3,8456,8389,90AA,8FD0,8787,824D,917C,8ADA,8BF9,8C51,8EB8,80C7,8644,82C5,89E2,94E6,935E,893F,863E,8DC6,8098,8910,8044,88B7,82A2,8E5D,872C,8C9A,922E,84FF,ED4B,ED25,F148`
  -> closure_out.json (2780 instr, 0 bad opcodes, 0 overlaps, 0 data/padding overlaps in bank 10).
- resid.py / drefs.py: residual bank-10 gaps and their data-operand anchors (resid.txt).
- proposal.py -> proposals.json: exact proposed ranges for L-001 and L-009..L-014.
- bankctx.py / b0check.py: bank-select and cross-bank transfer census (banks 1-9/11 cross-check).
- small.py: small-fragment context/reference survey. xref.py: raw word/JSR/JMP reference search.
