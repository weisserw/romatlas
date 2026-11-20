# Commentary progress

## Round 4 (pass-004)

Previous commentary (pass-003, round 3, complete) preserved in full at
analysis/round-3/commentary/ (copied, diff-checked, then removed before
`commentary.py prepare` ran on pass-004); rounds 1/2 stay at analysis/round-1/, round-2/.
This directory was freshly prepared from pass-004 (before/ byte-identical, 1,661 scaffold
regions). Pass 4 only added three Static regions (b0 $E536 code, b6 $FE24 code, b6 $FE2C
data) over former unknown runs; no other region changed, so bank:CPU note keys still name
the same physical bytes. notes/, tools/ and uncertainties.md were carried from round 3.

### Batch log
- B0: carried round-3 notes built and verified byte-identical on the pass-004 baseline
  (annot.py reported only the two stale unknown-run notes, 0:E536 and 6:FE24).
- B1: removed the stale notes from r3_data.txt; updated the $E522 note (next bytes are code);
  new notes/r4_code.txt: TryLatchOntoPlayer_Id2 ($E536), HideSpritesThenOamDma_Fragment
  ($FE24) with block comments, and a data note for $FE2C. Rebuilt, verified, fill_review.py
  (1661/1661 entries reviewed, 910 units), reading copy refreshed.

### Result
- Whole-source checks unchanged from round 3 (no address-only routine names; all data
  regions carry a note or the accepted automatic consumer/evidence note: 145).
- Verification: commentary.py verify ok=true, byte_identical=true
  (game.asm sha256 2894d99b...6d874576); reading-after.asm refreshed.
- editorial_audit pending (final acceptance only).
