# Commentary progress

Prepared verified baseline from checkpoint pass-001 with
`python3 /root/romatlas/scripts/commentary.py prepare <workspace>` (byte-identical).

## Method
source/game.asm is produced by the workspace-local helper tools/annotate.py from the frozen
before/game.asm plus the durable annotation records annotations/*.py:
common.py (RAM / cartridge-RAM symbols), fixed.py (fixed bank), bank0..bank6.py (16K banks).
The helper only adds symbols, comments, label placement and byte-identical re-expression
of data (.incbin splits with equates, .byte/.word for far-call triples and verified code
pointer tables).  Workflow: edit annotations, then
`python3 commentary/tools/annotate.py review` (rebuilds source/game.asm and refreshes the
review.json inventory), then `commentary.py verify` and `commentary.py read`.
`annotate.py listing B` prints a reading listing of 16K bank B (7 = fixed);
`annotate.py todo` lists globals still carrying address-only names (currently none).
Do not hand-edit source/game.asm without porting the edit into the annotations, or the
next build overwrites it.

## Round 1 pass (complete current-source scope)
- All 709 code regions in all banks: every global label (657 routines/entries) has a
  descriptive name; meaningful routines carry block comments; 837 branch targets have
  cheap-local labels (hand-named where useful, otherwise derived from the branch condition,
  e.g. `@if_GameMode_zero`, `@if_IsEnemyBlocked_carry_set`, `@loop`, `@return`, `@join`).
- RAM: 226 zero-page/RAM/cartridge-RAM symbols and 57 per-slot arrays (players, enemies,
  bombs, flames, items, sound) used consistently in operands; hardware and MMC1 register names.
- Data: far-call triples as `.byte bank / .word Target-1`; the 9 verified code-pointer
  tables as `.word` label lists; ~160 named data tables (equates at their bytes) with format
  notes; generic data blocks carry boundary/role-unknown notes in the bank headers or
  block notes.
- Unknown ranges (31 bytes): honest notes; four bank-5 ranges now have concrete consumer
  evidence (see uncertainties.md) - proposed to redirect, not reclassified here.
- review.json: all 978 scaffold entries status=reviewed with summaries/source lines and
  657 units; editorial_audit remains pending (final acceptance only).

## Main interpretations established
Game = Bomberman II.  Systems: kernel/NMI/VRAM queue/far calls (fixed), sound driver
(fixed $D800-$DD9B byte-code interpreter + bank 2 envelopes/effects/APU), normal-mode
enemies with 27 kinds and boss/cutscene scripts (bank 0), stage map builder (bank 4),
players/bombs/explosions/items/password/title/menus/intro/ending (bank 5), CHR graphics
(banks 1, 6), music data (banks 3, 4).
