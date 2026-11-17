# Final conclusions and terminal handoff (round 5, 2026-11-17)

This is the coordinator's final record.

## Classification

The two kinds of classification are reported separately.

- **Evidence-backed classification (Observed/Static):** 393,224 of 393,232 physical bytes (99.998%).
  - This was reached by rounds 1-4 of the disassembly, commentary and redirect workflow.
  - The round-4 terminal-incomplete rejection review was accepted under evidence-policy.md
    (analysis/handoffs/round-004-redirect.md).
- **User-directed MANUAL fallback:** 8 bytes, generic Data with unknown meaning, map regions 5155-5158.
  - physical 0x0040EA-0x0040EB (16618, 2 B), bank 2 CPU $80DA-$80DB
  - physical 0x005998-0x00599B (22936, 4 B), bank 2 CPU $9988-$998B
  - physical 0x005B58 (23384, 1 B), bank 2 CPU $9B48
  - physical 0x012175 (74101, 1 B), bank 9 CPU $A165
- **Unknown:** 0 bytes. The map has no unclassified complement.

### How the fallback bytes were classified

The user explicitly authorized a batch-only exception: if at most 10 residual bytes remained, they were to be
marked as generic Data of unknown meaning with Manual provenance. Before annotating, the complement was re-verified
from analysis/state.json and was exactly these 8 bytes.

Prior ambiguity, unchanged:
- Three runs are lone `$60` (RTS-valued) bytes after a final RTS or a table.
- $9988 is an LDA #$80 / LDA #$00 fragment that is overwritten at once by CpuDefenseChangeDrive.
- No reference, consumer, sibling copy, data read or opcode fetch was ever found.
- Code versus data was never decided.

This is **not** an observed or statically proven data role, and no semantics are claimed. If code or data evidence
appears later, reopen lead L021. Details: analysis/progress.md "Manual residual fallback".

### Completion status

Whole-ROM *evidence-backed* classification is NOT complete; 8 bytes rely on the manual fallback. What is complete
is the coordinator workflow, which ended in a justified terminal incomplete, followed by the authorized fallback.

## Checkpoint and verification

- maps/pass-005.json = maps/current.json = analysis/state.json (sha256 d09c3179...).
  analysis/pass-005-verification.json is ok/byte_identical.
- The earlier checkpoint was pass-004 (077d58cd...). Its map file was not recoverable after the rehydration; it is
  recorded in analysis/progress.md.

## Commentary

- Round-4 commentary is preserved at analysis/round-004/commentary/.
- Round 5 carried it forward by physical range. Handoff: analysis/handoffs/round-005-commentary.md.
  - 0 Code entries without units; 3,453 reviewed units.
  - 4,111/4,111 scaffold entries reviewed.
  - The source headers at the fallback sites state the manual fallback.
- The final review and strict editorial audit passed (analysis/handoffs/round-005-final-audit.md).
  - commentary/audit.json: ok, byte_identical, editorial_audit true.
  - A fresh `commentary.py verify` after the audit was byte_identical.

## Leads, gameplay and remaining limitations

- Leads (analysis/leads.json):
  - L021 is closed by manual fallback; reopen it on new evidence.
  - L006 is verified: gameplay reached the league-champion ending (no credits sequence).
  - There are no ready leads.
- Gameplay checklist: analysis/scenarios.json.
- Remaining readability limitations are listed in commentary/uncertainties.md. They include raw RAM operands whose
  role is unknown, the ScreenRecordNN index labels, and a "probably pair 6" note on one far call.

## Not done

No archive, cleanup, publish or toolkit edit was performed. archive-options.json lists the documents for retention.
