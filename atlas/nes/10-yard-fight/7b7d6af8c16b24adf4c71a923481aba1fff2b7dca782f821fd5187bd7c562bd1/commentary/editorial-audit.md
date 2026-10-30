# Final editorial acceptance

Final acceptance passed for pass-003. Reused sound full current-source review, inspecting the complete 133-scope/308-unit inventory for pending status and revisiting concrete redirect findings and their helper/caller dependencies. No mandatory whole-source reread or cosmetic rewrite was needed. All scopes and meaningful inventoried units are reviewed; no pending readability ranges or concrete editorial defect remains. Mechanical audit validates inventory and rebuild fidelity, not semantic quality.

## Corrections and evidence

- game.asm:2512–2522: RestorePpuState reads PPUSTATUS and restores PPUCTRL and both scroll values. PPUMASK is written by the preceding palette writer, before the named entry. Corrected source and unit summary; preserved entry name and bytes.
- game.asm:8866 and actual jump at 8878: UploadFirst64OamBytes tails to RestoreRenderingMask (helper at 3276), restoring PPUMASK only. Corrected source, both inventory summaries and uncertainties.md. Caller still supplies OAMADDR; the loop copies exactly 64 bytes, and the immutable-NROM operand store has no source-page effect.
- Actual inventory: 308 units = 260 carried + 48 new, in 133 reviewed scopes. This explicitly supersedes the 309/49 prose in round02-commentary-handoff.md and the earlier section of commentary/progress.md. Historical records remain unchanged. analysis/round-02/commentary-accounting.json already had the correct counts.
- The historical tail name “RestorePpuState” in analysis/evidence/round02/review.md R01-R06 and round02 commentary handoff is superseded by this correction. CPU $945F is RestoreRenderingMask. The instruction/target and structural acceptance remain unchanged; this was a naming/effect discrepancy, not new code or a map correction. Redirect's round02-terminal-review/findings.md had identified it correctly.

## Verification and identities

Documented commentary.py audit passed, freshly rebuilt byte-identically, and refreshed reading-after.asm. commentary/audit.json records 133 reviewed regions and 308 reviewed units. Source SHA-256 db996732dfc53607ee766f63aef9abac5211503312a20e9d5df3ff7a2bdcfa54; reading SHA-256 4ebf3303fdae2b0e9e8aca360f143274582f49f63e16c740137b6bff6d59684f. analysis/current-audit.json records fresh configured-binary validation of current map and saved internal state, both ok, and hashes. Their SHA-256 remains a2c637a340703bcd0cbafb8a4e8106d75d5ddbfa82060fd22bd82d615ca09960; frozen commentary baseline/state match it. ROM hash remains 7b7d6af8c16b24adf4c71a923481aba1fff2b7dca782f821fd5187bd7c562bd1.

Classification remains 40,976/40,976, zero unknown: Code 13,174, Data 19,610, Image 8,192. No map/state/classification change or new checkpoint was necessary for comment-only edits; pass-003 and its verification remain preserved.

## Limits

Natural callers/selectors, exact football event meanings, some animation domains and scratch-field reuse remain explained uncertainties, not pending structural/readability work. Prior broad natural skill-level mechanics and artificial winner observations are reused; no new execution, natural win, absent credits, historical intent or full hardware compatibility is claimed. No new toolkit issues found; existing confirmed NES-INPUT-001 remains nonblocking with buttons-array workaround (analysis/toolkit-issues.md). No ROM/toolkit edits, archive, publication or deletion. All restored material stays local. Coordinator still handles archive options/document accounting and terminal workflow audit.
