# Round 5 coordinator handoff (2026-11-17): manual residual fallback -> final commentary/audit

- Checkpoint: maps/pass-005.json = maps/current.json = analysis/state.json (sha256 d09c3179...);
  analysis/pass-005-verification.json ok/byte_identical. Unknown complement: 0 bytes.
- 8 bytes (regions 5155-5158: b2 $80DA-$80DB, $9988-$998B, $9B48; b9 $A165) are generic Data, Manual provenance,
  by USER-DIRECTED FALLBACK only (not observed/proven; meaning unknown). See analysis/progress.md
  "Manual residual fallback". Never describe them as evidence-backed or invent semantics.
- Commentary: fresh prepare on pass-005; round-4 commentary preserved at analysis/round-004/commentary/ (immutable).
  commentary/source carried from round 4 (byte-identical verify ok); review.json carried by physical range
  (4,107 entries) + 4 new fallback entries reviewed. editorial_audit pending.
- Backlog for final acceptance (commentary/review.json):
  - The audit (scripts/commentary.py audit) requires every scaffold entry to be reviewed with a summary and valid
    source, and every Code entry to have >=1 reviewed unit (name, offset, length, status, summary, source).
  - Currently 1,356 Code entries have no units. Many are fragments of routines whose unit is recorded on a
    neighbouring entry, so split or duplicate spanning-unit inventory records onto them; do not invent new
    semantics.
  - Also close any remaining naming/comment gaps found in the source.
