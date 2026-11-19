# Final editorial acceptance — commentary rebased onto pass-004-fallback

Status: **complete** (final review and strict editorial audit passed).

## Scope
- Input: checkpoint pass-004-fallback (maps/current.json = maps/pass-004-fallback.json = analysis/state.json,
  sha256 c4dd4ae8a2e0e6d24f746f525c6d33d82f6526542ea54107462ab07fd9823dec). The map changed from pass-003 by
  exactly 2 one-byte Manual Data regions: 1252 (phys 230960, 28:$8620) and 1253 (phys 235667, 28:$9883).
- Rebase rule followed:
  1. The entire pass-003 commentary was preserved at analysis/round-4/commentary/ (diff -r identical,
     27 files sha256-matched).
  2. A fresh `commentary.py prepare` was run (byte_identical, 1232 scaffold regions); the frozen
     baseline/manifest now reference the pass-004-fallback map.
- Carry: analysis/tools/round-004-commentary/carry.py.
  - It confirmed that the regenerated before/game.asm differs from pass-003's only by comment lines at the
    2 ranges. No directive, boundary or bank change was found anywhere else.
  - It carried source/game.asm and review.json by physical range and remapped the review line references
    (3167 references, 0 text mismatches).
  - The round-3 emit/names pipeline was not needed, because no other generated text changed.

## Carried vs new review
- Carried with valid credit: 1230 scaffold entries and all 1937 units (unchanged ranges and text).
- New or rewritten (2 entries):
  - region-1252 and region-1253 (Data) replace unknown-038630 and unknown-039893.
  - Their source notes now say: generic Data by user-directed manual fallback; meaning unknown; the value
    is $60, which would decode as RTS; neither code nor data is proven; see L-015 and
    analysis/handoffs/manual-fallback.md. No label was added.
- Fixed: the old $9883 note used the stale routine name PitchPhaseTest; it now says CheckStrikeCall.
- Records updated:
  - commentary/uncertainties.md: the 2 bytes are now manual-fallback Data, and the stale redirect list
    was reduced to the questions still open.
  - commentary/progress.md: round-4 section with the reconciled inventory.
  - commentary/editorial-audit.md: round-4 section.
  - review.json: editorial_audit set to reviewed.

## Audit result
- `python3 /root/romatlas/scripts/commentary.py audit <workspace>` returned ok true, byte_identical true,
  reviewed_regions 1232, reviewed_units 1937 and editorial_audit true (commentary/audit.json).
- source/game.asm sha256 dd77d23dbd4b8f759139552739503812f3f0571b93c71367b51eee2a56d8c482.
- The reading copy was refreshed by audit: commentary/reading-after.asm, sha256
  4475dab8f267af7525aa85f8bc5ea3e894763c04ce026b7e184210a3e75f90dd.
- Before the audit, a separate `commentary.py verify` also returned ok and byte_identical true.
- The audit checks inventory and the rebuild only. The editorial judgment is recorded in
  commentary/editorial-audit.md.

## Residual uncertainties (commentary/uncertainties.md)
- The meaning and the code-vs-data role of 28:$8620 and 28:$9883 are unresolved; L-015 stays rejected.
- Object 10 is only tentatively the umpire.
- Wall sections 2-4 have no confirmed left/right assignment.
- Team 7's last starting-order pair is unexplained.
- The Safe/HitsAllowed counters, the clock at $03E5/$03E6, BootABFlag and the write-only bytes are unresolved.
- Terse helper headers, mechanical local branch labels and block-level bulk-data notes were kept on purpose.

## Classification (reported separately)
Classification: 393230 bytes evidence-backed + 2 bytes manual user-directed fallback (Data, meaning unknown; 0 unknown) — not fully evidence-backed.

## Toolkit
One minor missing feature was logged in analysis/toolkit-issues.md: there is no supported rebase/carry
command, and prepare refuses while commentary/ exists. It did not block this work; no toolkit files were
modified.
