# Final acceptance handoff

## Scope
- Inputs:
  - checkpoint pass-003;
  - round-2 commentary, whose readability gate was met;
  - round-2 redirect attempt r2-a1, terminal for research.
- This pass did three things:
  1. applied lead L15 as commentary wording;
  2. ran the reserved final editorial audit;
  3. ran the final verification.
- No map or region change. The ROM is unchanged, and so are the toolkit and the configured
  binary (/root/romatlas/target/release/romatlas).
- Nothing was archived or published.

## Checkpoint
- These five files are identical, sha256 9e2a7bca32ad...aa22f2:
  - maps/current.json
  - maps/pass-003.json
  - analysis/state.json
  - commentary/baseline.json
  - commentary/phase1-state.json
- Classification: 100.00% (131088 / 131088 bytes; 0 unknown).
  - 1582 regions.
  - Code 31825 bytes, Data 99263 bytes.

## Verification outputs
- `romatlas map validate maps/current.json`: ok.
- `romatlas map validate maps/pass-003.json`: ok.
- `romatlas store validate --store analysis/state.json`: ok.
- `romatlas generate` with pass-003 into scratch/final-acceptance/pass-003-gen, then `verify`:
  ok, byte_identical=true. The earlier record is in
  [pass-003-verification.json](../pass-003-verification.json).
- `commentary.py verify`: byte_identical=true after each edit batch
  ([commentary/verification.json](../../commentary/verification.json)).
- `commentary.py audit`: **passed** ([commentary/audit.json](../../commentary/audit.json)).
  - 1582 reviewed regions and 1435 reviewed units.
  - editorial_audit=true.
  - Fresh byte-identical rebuild.
  - Reading copy refreshed (commentary/reading-after.asm).
- Summary with hashes: [analysis/current-audit.json](../current-audit.json).

## Audit result
The editorial audit passed. The full record is in
[commentary/editorial-audit.md](../../commentary/editorial-audit.md). Summary:

- **L15 applied.**
  - Password 25-bit packing layout.
  - Two unread constant fields, described mechanically.
  - Static writers for blackjack codes 1/2/3/8. Correction to the redirect note: code 3 is
    written by BlackjackSession $8052, not BjSettleHand.
- **Concrete inconsistencies fixed:**
  - 16 sub-table labels replace Label+offset operands;
  - the MenuCursorPos size was corrected, so the portrait animation bytes now read by name;
  - bank constants added on raw bank-select immediates;
  - comments added on raw cross-bank calls;
  - 2 header/label mismatches;
  - 8 stale prose names;
  - 7 undescribed tables.
- **Review inventory.** tools/cmtreview.py now takes data-table summaries from the consumer
  comment, so no region or unit keeps a generic "see the note above" summary.
- **Records updated:**
  - analysis/leads.json: L15 is applied, and a final pass was recorded;
  - commentary/progress.md;
  - commentary/uncertainties.md: L15 items moved to resolved.

## Archive-contract artifacts
- Present:
  - analysis/state.json, current-audit.json, scenarios.json, investigation.md, gameplay.md and
    progress.md;
  - commentary/baseline.json, phase1-state.json, manifest.json, review.json, audit.json,
    verification.json, progress.md, uncertainties.md and editorial-audit.md;
  - maps/current.json.
- Missing: **analysis/annotations.json**.
  - No toolkit command or workflow script produces it for NES, so it was not fabricated.
  - Recorded as TK-005 in analysis/toolkit-issues.md.
  - archive.py includes FINAL files only "where available".

## Remaining issues
- No blocking issues.
- Open leads, all non-structural:
  - L4 and L7: reachability;
  - L5 and L6: per-record data format semantics.
- Uncertainties are in commentary/uncertainties.md:
  - the FDS-BIOS origin hypothesis;
  - the $CFE6 remnant's reachability;
  - poker variable roles;
  - music envelope bits;
  - meanings of the unread constant fields.
- The ROM is not claimed to be fully understood, and no hardware compatibility is claimed.
- Archiving is left to the coordinator and the user.
