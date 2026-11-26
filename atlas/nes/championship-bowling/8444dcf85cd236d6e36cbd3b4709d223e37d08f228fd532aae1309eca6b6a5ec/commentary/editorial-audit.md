# Final editorial audit (checkpoint pass-006, commentary round 6)

Scope: final editorial acceptance of commentary/source/game.asm. Valid round-6 review
credit (512/512 scaffold entries, 988 units) is reused; no whole-source reread. Material
was revisited only where evidence changed (L-023) or a lead was open (L-010), plus
targeted greps for stale or contradicted wording.

## Findings and fixes

1. **$FC23 comment contradicted by later evidence (L-023), fixed.** PlaceSprite's Static
   path said "Nothing in the data rules it out". Replaced with the corrected mechanics
   (the path runs when Y + record offset leaves 0..255 at either edge; kept only when at
   most 7 pixels above the screen) and the disposition: dormant by data flow, every
   object's Y + offset stays within 9..232 (analysis/redirect/r6-a1/evidence/fc23_bounds.json).
   The file header's Static-branch paragraph was updated the same way.
2. **DrawObject $FD note incomplete, fixed.** Now states that no sprite list is built in
   RAM (every frame table pointer comes from ROM), so "unused by ROM lists" covers all lists.
3. **L-010 (sprite records as binary includes), decided and closed.** Every record list is
   already split at its frame label (one .incbin per frame, frame pointer tables as labelled
   .word), each group has a format note, and the 4-byte grammar (attribute, signed dx,
   signed dy, tile; $FF/$FE/$FD control bytes; sprite budget) is documented once at
   DrawObject. Expanding about 1,600 records (134 frame includes, 6,381 bytes) into .byte lines would add raw pixel offsets
   and CHR tile numbers with no new meaning, and fields have no symbolic values to
   substitute. Decision: keep per-frame includes; DrawObject's header now points readers
   to the grammar.
4. **commentary/uncertainties.md** updated: $FC23 moved to resolved; L-010 decision
   recorded; the remaining binary-data note narrowed to nametables and tile blocks.
5. **review.json**: source line references shifted for the 5 inserted comment lines
   (76 locations); region-348 summary now states the dormant disposition.

## Checks with no finding

- Stale wording grep (no known caller, uncalled, TODO, FIXME, unobserved, not seen, open
  possibility, top-edge): none left. "Purpose unknown" (SetFrameAreaAttrs) and "not
  checked" (selection objects) are deliberate, listed uncertainties.
- Raw address operands: only `sta a:$0000,y` in the Reset RAM clear (page 0 as a whole)
  and nametable addresses in PPU address tables; both intended.
- review.json: no summary mentions the old $FC23 wording.

## Result

editorial_audit = reviewed. Remaining semantic uncertainties are listed in
commentary/uncertainties.md; none affects classification or rebuilding.
