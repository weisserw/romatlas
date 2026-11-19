# Final orchestration handoff — terminal INCOMPLETE (2026-11-18)

Workflow reached its documented terminal stop: justified terminal incomplete after rejection review.
This is NOT whole-ROM completion. ROM classification is not complete (1 byte Unknown).

## Final state
- Checkpoint: maps/pass-006.json (sha256 ac37c43fff6441132a9919788f6769764a5c76829235748e4edc86cbc982f073)
  == maps/current.json == analysis/state.json. Map validates.
- ROM classification: 100.00% (262159 / 262160 bytes classified; 1 bytes unknown) — exact 99.9996%.
  Code 62,293 B (10,804 Observed), Data 68,794 B, Image (CHR) 131,072 B. Per-bank table: analysis/progress.md.
- Exact residual (refreshed by coordinator from analysis/state.json after pass-006): phys $15B18, 1 byte ($60),
  PRG bank 10 CPU $9B08 (context [10,11,14,15]), between NameGrid_Right's `jmp PlaceGridCursor` and table
  GridRightWrap ($9B09). Lead L-R3-01 (deferred).
- Verification: analysis/pass-006-verification.json (map->generate, byte_identical=true);
  commentary/verification.json (commentary source rebuild, byte_identical=true). input/game.nes unchanged
  (sha256 a71e4cc8...).
- Commentary: current-source readability complete on pass-006 (analysis/handoffs/round4-commentary.md):
  0 auto labels, 0 address-only locals, 0 raw RAM operands, 3414 names, review.json 1225/1225 regions reviewed,
  1405 units; reading copy commentary/reading-after.asm refreshed. editorial_audit remains PENDING by design
  (final editorial audit is reserved for whole-ROM final acceptance; `commentary.py audit` not run).
  Prior commentary rounds preserved in analysis/round-1/, round-2/, round-3/.

## Rounds (details in analysis/progress.md)
- R1: pass-003 96.38% (9483 U) -> commentary -> redirect r1-redirect-01 (menus, league champion, ~9215 B leads).
- R2: pass-004 99.91% (245 U) -> commentary -> redirect r2-redirect-01 (L-R4-01 240 B, corrected premise).
- R3: pass-005 (5 U) -> commentary -> redirect r3-redirect-01 (L-R2-04 corrected representation, L-R5-01).
- R4: pass-006 (1 U) -> commentary (complete). Redirect not relaunched: see coordinator review below.

## Coordinator rejection-review inspection (evidence-policy.md "Challenge a terminal decision")
Review inspected: analysis/handoffs/round3-redirect.md "Rejection review: b10 $9B08".
- Claim tested: phys $15B18 is Code (stray rts) or generic Data. Decisive evidence: GridRightWrap base fixed at
  $9B09 by `lda $9B09,x` with X bounded 0..4 -> byte is outside the table (extent proof, not index-only);
  no word/branch/fall-through reference (round-2 entry-mechanism searches: words, split immediates, RTS dispatch,
  $C178/$FF80 tables — scope covers this byte); never fetched in S01-S18.
- Competing interpretations (leftover rts after jsr->jmp edit vs pad byte) are both consistent; there is no
  accepted in-ROM precedent for either (r3 deadafter.py: no unreferenced post-transfer instruction in classified
  code; all 20 jmp+$60 pairs are branch targets). evidence-policy.md explicitly states an unreferenced return
  opcode lacks sufficient structure on its own, and a single byte gives no repeated-pattern basis for generic data.
- Assessment: counterexample scope is correct, no unsupported prerequisite, consistent with how L-R5-01 (3 B with an
  operand constraint) was accepted. Premises are unchanged by Round 4 (pass-006 edits were in banks 12/13 only).
  Remaining discriminating test: a reference/consumer/execution of $9B08 — none exists in source or traces.
  Review is sound; per goal.md, stop incomplete rather than cycling.
- Gameplay: scenarios.json S01-S18; season-end CHAMPION screen reached naturally; no credits; no unvisited
  milestone connected to the residual.

## Open (non-classification) items
- L-R1-02 (partial, unbounded $C178 tables — all in classified regions), L-R1-05 (indirect jumps; no residual impact).
- Reachability of Static no-known-caller fragments (L-R4-01, L-R5-01) unresolved; the $A07B overlap is documented
  in region evidence (TK-003). Toolkit issues TK-001..TK-003: analysis/toolkit-issues.md.
- To finish whole-ROM completion later: resolve $9B08 with new evidence, then final editorial audit
  (review.json editorial_audit), `commentary.py audit`, archive.

## Archive readiness
archive-options.json lists documents/omitted_documents per scripts/archive.py contract. No archive run here.
