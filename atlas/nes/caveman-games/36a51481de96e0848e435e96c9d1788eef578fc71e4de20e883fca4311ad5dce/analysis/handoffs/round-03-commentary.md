# Round 3 — commentary handoff

**Status: COMPLETE** (current-source readability for checkpoint pass-003). editorial_audit stays pending.
input/game.nes unchanged; toolkit unchanged; nothing published/archived; no processes left running.

## Scope / method
- pass-003 differs from the round-2 baseline (pass-002). Round-2 commentary preserved whole at
  analysis/round-2/commentary/ (diff -r identical); commentary/ re-prepared (`commentary.py prepare`, 5551 rows).
- Carried notes/, tools/, uncertainties.md by physical range (notes keyed by 16K bank + CPU address;
  build_source.py rejected none). review.json rebuilt: 857 code units reviewed (839 identical to round 2,
  4 re-read: AttractTitle, AttractAltScreen, CavesOfFame, FS_FlushPendingSfxX; 14 new); all 5551 rows reviewed.

## New / rewritten commentary
- bank0.asm: AttractTitle_UnusedExpire ($811A), AttractTitle_UnusedExpire2 ($817A), AttractAltScreen_UnusedExpire
  ($81C3) plus descriptive loop labels replacing B0_Entry811A_*; FameTitleOffsets / FameTitles / FameUnusedSelfJump
  ($8547-$85B3); CavesOfFame header (skip of index 4 = Clubbing explained).
- bank2.asm: FS_UnusedEmphasisCycle ($8363, self-modifying, ineffective from ROM).
- bank3.asm: DR_ActionRiderY/X 83-entry extents, DR_DrawBothRacers ($860E); merged duplicate pose-table comment.
- bank7.asm IrqHandler + uncertainties.md: L-015 fix (banks 1/4/5 CLI with bare-RTI vectors, IRQs seen in 4/5;
  banks 0/2/3/6 vector to $C000 but never enable IRQs). L-016 needed no text change.

## Verification
`commentary.py verify` -> ok, byte_identical true (commentary/verification.json, after last edit);
`commentary.py read` refreshed commentary/reading-after.asm.

## Unresolved (commentary/uncertainties.md)
- Original purpose of the new unused fragments (inferred leftovers); FameUnusedSelfJump role.
- Carried: IrqHandler static-only; IrqSplitParam odd bytes; generic data bank 3 $AEF9-$B01D, bank 2 $8A9D,
  bank 7 $CEDB-$CEE7; L-004 roots (AttractAltScreen, DrawMenuLowerScreen, ...); L-005/L-006 semantics.

## Leads
No new leads. L-015 -> verified (applied in commentary).

## Remaining gaps
None in current-source readability. Final editorial audit/archive acceptance remains separate.
