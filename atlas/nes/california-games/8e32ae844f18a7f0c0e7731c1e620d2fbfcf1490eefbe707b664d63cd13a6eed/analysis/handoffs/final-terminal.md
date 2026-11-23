# Final terminal handoff (orchestration of goal.md) — FINAL, WITH USER-DIRECTED FALLBACK

Date: 2026-11-23 (updated). This supersedes the status line of the earlier terminal-incomplete record,
which is preserved verbatim below. It is a workflow terminal handoff ready for archive; it does NOT claim
evidence-backed whole-ROM classification.

## Result
- **Proven (evidence-backed) classification:** 131085 / 131088 bytes (Observed/Static), unchanged.
- **Manual user-directed fallback (reported separately):** 3 bytes, physical offset 16398 length 2
  (0x400E-0x400F, bank 0 $BFFE-$BFFF) and offset 114704 length 1 (0x1C010, fixed bank 7 $C000), marked
  generic Data with UNKNOWN meaning, provenance Manual, confidence 0.1 (regions 2509, 2510), under the user's
  explicit batch-only exception for <=10 residual bytes. Not an observed or statically proven data role.
  Prior ambiguity: they are the pieces of the observed cross-window instruction `lda $C149,x` that the toolkit
  cannot represent as Code (TK-04, lead L-R1-01); that evidence is retained (records below, progress.md).
- **Map coverage:** 131088 / 131088 bytes carry a region kind; 0 unclassified.
- **Checkpoint:** maps/final-fallback.json = maps/current.json = analysis/state.json (sha256 212cccb66dfad0ff…);
  analysis/final-fallback-verification.json ok, byte_identical true. ROM unchanged (sha256 8e32ae84…).
  Previous checkpoint maps/round03-final.json (aab2fb6c…) retained.
- **Commentary:** commentary/source/game.asm sha256 e5b27876792c597e…; round-5 commentary preserved in
  analysis/round-5/. Final review and strict editorial audit done (analysis/handoffs/final-commentary.md);
  `commentary.py audit` passed (ok, byte_identical, 2293 regions + 1237 units reviewed, editorial_audit true),
  re-run independently by the coordinator with the same result; reading copy commentary/reading-after.asm refreshed.
- **Remaining commentary limitations:** ~776 generated direction-only branch labels; tentative names listed in
  commentary/uncertainties.md (Round 6 section).
- **Blockers retained:** TK-04 (toolkit cannot express cross-window instructions) remains open; with authorized
  toolkit support the 3 fallback bytes should be reclassified as the observed Code they are.
- Archive: archive-options.json updated; archive not executed; nothing deleted or published.

---

# Earlier record (preserved verbatim)

# Final terminal handoff (orchestration of goal.md) — TERMINAL INCOMPLETE

Date: 2026-11-23. Workflow reached its documented terminal handoff: justified terminal
incomplete after the required rejection review. This is NOT whole-ROM completion and
does not mark ROM classification complete.

## Result
- ROM classification: 100.00% (131085 / 131088 bytes classified; 3 bytes unknown).
  (Rounded figure; 3 bytes remain unknown, so classification is incomplete.)
- Residual: physical 0x400E-0x400F (bank 0 $BFFE-$BFFF) + 0x1C010 (fixed bank 7 $C000):
  one observed instruction `lda $C149,x` spanning the switchable/fixed window boundary.
  The toolkit cannot represent a code region over physically non-adjacent bytes (TK-04,
  analysis/toolkit-issues.md); every non-code kind contradicts the observed fetch.
  Blocked on authorized toolkit support (lead L-R1-01).
- Final checkpoint: maps/round03-final.json = maps/current.json = analysis/state.json
  (sha256 aab2fb6c6b812036…), byte-identical rebuild.
- Commentary: commentary/source/game.asm sha256 a2b9634c2c312f97…; fresh verification
  analysis/round05-commentary-verification.json (ok, byte_identical); reading copy
  commentary/reading-after.asm; review.json 2293/2293 reviewed. Final editorial audit
  (commentary/editorial-audit.md, audit.json) deliberately not run: reserved for
  final acceptance/archive, which was not authorized here.
- Remaining editorial issues: ~843 generated structural branch labels (mostly SurfBuildWave,
  NameEntryFrame); tentative names listed in commentary/uncertainties.md.
- Gameplay: all 15 milestones in analysis/scenarios.json visited, including the natural
  compete-mode ending (FINAL RESULTS; game has no credits).

## Rejection review acceptance (coordinator)
Inspected analysis/handoffs/round-04-redirect.md (reusing round-03-redirect.md): premises
(observed opcode fetch at 0x400E, successor fetch at $C001, toolkit contract requiring
contiguous physical instruction bytes) are consistent with accepted evidence; no
alternative representation exists; the gap is a missing representation, not a missing
inference. No corrected inference or new test remains, so redirect was not relaunched.

## Round history
See the coordinator table in analysis/progress.md and phase handoffs in analysis/handoffs/.
Classification by round: R1 83.73% → R2 99.89% → R3 100.00% (3 B unknown); R4/R5
commentary-only (semantic leads L-R3R-01..09, L-R4R-01..02).

## Resume / next steps
- With authorized toolkit support for cross-window instructions: classify the 3 TK-04
  bytes, then run final editorial audit and byte-identical verification.
- Archive: archive-options.json prepared; archive not executed.
