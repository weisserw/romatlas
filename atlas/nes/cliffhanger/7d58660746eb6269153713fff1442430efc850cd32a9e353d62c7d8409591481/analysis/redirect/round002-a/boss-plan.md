# Changed boss contact experiment
Prior attempt `../round001-a/boss.py` copied positions, reset mode0, then tried B/right+B/A+B and returnedtitle. No boss combat confirmed. Do not repeat that recipe.

New independent evidence: callback table type2D=>93B7, mode0A/18/11 selects facing helper9488; for equalX and facingbit20 set helpercarry succeeds. Actorfield10 positive and flags1E bitsC0 clear permit damage; field1D subtract4/6, underflow clearsfield10 sign and marksflag20. This provides a specific contact/damage contract, not arbitrary RAM sweep.

Import immutable saved stage14 snapshot, advance1frame, save session-local before edits. Copy full actor Xfraction/Lo/Hi and Yfraction/Lo/Hi into player fields, set playerflags20 (preserving otherbits), mode06FE=0A, actor650=1 damage threshold, actor651 clearC0 only; actor643=1 ensures positive damagegate. Run3frames withB, release2. Trace aggregate accepted fetches distinguishes actual callback from merepatchedvalues. Capture before/after andRAM. Restore before ending. A negative result stops this recipe; no arbitrary state/mode sweep. ROM unchanged; this is artificialstate evidence only.
