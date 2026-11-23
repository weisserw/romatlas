# Commentary progress

## Round 1 commentary (2026-11-22) -- complete for current source

Baseline: prepared from maps/current.json == maps/round1-disasm-v2.json (sha256 3249...cdcdb), analysis/state.json
(manifest.json). before/ and reading-before.asm are the frozen generator output.

Method: source/game.asm is produced from before/game.asm by tools/apply.py using the name/comment database
tools/names.txt (labels, RAM/IO/ROM-table equates, routine header comments, inline comments, region notes,
explicit bank context for fixed-bank calls into $8000-$BFFF). Only labels, symbolic operands (explicit a:/z:
widths preserved) and comments are added. To revise commentary, edit names.txt and rerun
`python3 tools/apply.py && python3 tools/review.py`, then verify. tools/parse.py, graph.py, dump.py are the
reading helpers (dump.py BANK LO HI prints a compact control-flow listing).

Scope covered: all classified source -- 11,895 instructions in banks 0 (sound driver), 6 (game screens),
7 (fixed bank) and the bank 1-5 tail copies; every data/padding region has a role note; the single unknown
byte has an explicit note.
- 1,500 code labels: 363/363 JSR targets and all region entries named descriptively (0 address-only routine
  names); 458 routines/structures carry header comments (review.json units).
- Branch labels: ~225 condition-derived names (Routine_IfCondition), remaining forward targets
  Routine_SkipN / backward Routine_LoopN (mechanical, documented in the file header).
- 260 RAM/IO equates (no numeric RAM/IO operands remain), 130+ ROM table equates, sound-id comments on every literal PlaySound call.
- No numeric ROM operands remain (all table references use equates); 186 branch targets keep mechanical Routine_SkipN names.

Key interpretations introduced (see uncertainties.md for confidence):
game types and $C438 dispatch map (poker 3-7, blackjack 8-9, roulette 10-11, Big Six 12, slots 13-23, quit 0,
clock 1); chip economy (ChipCounts $A7-$AB = 100/25/10/5/1, BetChips $AC-$B0, ChipOverflow, AwardChips,
PayFromChips); NMI VRAM job queue; floor scroller; ScreenDecode nibble-RLE; in-game clock and progressive
jackpots; roulette bet ids and payouts; sound driver entry points and command table.

Verification: `commentary.py verify` byte_identical=true after the final edit batch (commentary/verification.json).
Reading copy refreshed: commentary/reading-after.asm.

Deferred / unresolved (not readability gaps): see uncertainties.md and the round-1 commentary handoff.
