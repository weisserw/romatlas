# Final orchestration handoff — final acceptance with user-directed fallback (2026-11-18)

Supersedes analysis/handoffs/final-terminal.md as the current terminal handoff (that file is retained as history of the
pass-006 terminal-incomplete decision and its rejection-review inspection).

## Result
- Checkpoint: maps/pass-007.json (sha256 d0dd6e1f91d04845b83dd6c8c795603f08456c4aafef2aa83be775e1d3942bdf)
  == maps/current.json == analysis/state.json; map.validate ok; generated/pass-007 verified
  (analysis/pass-007-verification.json ok=true, byte_identical=true). input/game.nes unchanged (sha256 a71e4cc8...).
- ROM classification: 100.00% (262160 / 262160 bytes classified; 0 bytes unknown).

## Proven classification vs manual fallback (reported separately)
- Evidence-backed: 262,159 B — Code Observed 10,804; Code Static 51,489; Data Static 68,794; Image (CHR) Static 131,072.
- User-directed Manual fallback: 1 B — phys $15B18 (offset 88856, length 1), PRG bank 10 CPU $9B08, region 3989,
  kind Data, provenance Manual, confidence 0.5, no label. Generic data with UNKNOWN MEANING by explicit user batch
  exception (at most 10 residual bytes). It is not an observed or statically proven data role. Prior ambiguity
  (L-R3-01): lone unreferenced `$60` after `jmp PlaceGridCursor` and before table GridRightWrap; leftover `rts` or pad
  byte both remain consistent; no reference, consumer or execution. Rejection review: analysis/handoffs/round3-redirect.md.
  Therefore every-byte *evidence-backed* classification is NOT claimed; whole-ROM structural coverage is complete only
  with this one manual fallback byte.

## Commentary and final acceptance
- Round 5 commentary/final acceptance: analysis/handoffs/round5-commentary-final.md. Round-4 commentary preserved in
  analysis/round-4/commentary/ (rounds 1-3 untouched). review.json 1225/1225 regions, 1411 units reviewed;
  editorial_audit reviewed. `commentary.py audit` ok=true, byte_identical=true (commentary/audit.json).
  Coordinator re-ran `commentary.py verify` afterwards: ok=true, byte_identical=true; `commentary.py read` refreshed
  commentary/reading-after.asm.

## Remaining open (documented, not classification blockers)
- Meaning of $9B08 (manual fallback above).
- Whether the overlapping $A07B entry (TK-003) is intentional; reachability of Static no-known-caller fragments
  (L-R4-01, L-R5-01). See commentary/uncertainties.md, analysis/toolkit-issues.md (TK-001..TK-003).

## Archive readiness
archive-options.json updated (documents / omitted_documents per scripts/archive.py). No archive, cleanup or publish run.
