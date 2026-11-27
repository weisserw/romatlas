# Archive document selection accounting

The coordinator inspected `/root/romatlas/scripts/archive.py`, especially `FINAL`
and `archive()` document selection. This is preparation only; archive was not run.
The contract automatically recognizes its final documents and four special paths;
all other Markdown under analysis/ and commentary/ must be explicitly selected or
omitted. Paths containing a before/ or source/ component are excluded by the contract.
For NES, automatic .sym selections become .json.

Additional final documents and historical conclusions/handoffs are selected for
preservation, including this ledger. The three omitted files below are duplicate
generated instruction scaffolds in the superseded commentary snapshot; they contain
no investigation conclusions or results. The entire snapshot remains local.
Toolkit issues are a contract-recognized special exclusion, retained locally.
No final findings or handoffs are omitted to satisfy archive.

| Workspace-relative Markdown | Disposition |
|---|---|
| `analysis/archive-document-selection.md` | additional final document: preserve |
| `analysis/final-completion-audit.md` | additional final document: preserve |
| `analysis/gameplay.md` | automatically selected final document |
| `analysis/handoffs/round-01-commentary.md` | additional final document: preserve |
| `analysis/handoffs/round-01-disassembly.md` | additional final document: preserve |
| `analysis/handoffs/round-01-redirect.md` | additional final document: preserve |
| `analysis/handoffs/round-02-commentary.md` | additional final document: preserve |
| `analysis/handoffs/round-02-disassembly.md` | additional final document: preserve |
| `analysis/handoffs/round-02-redirect.md` | additional final document: preserve |
| `analysis/investigation.md` | automatically selected final document |
| `analysis/progress.md` | automatically selected final document |
| `analysis/redirect/r01-connected-20261127/data-contracts.md` | additional final document: preserve |
| `analysis/redirect/r01-connected-20261127/residual-contracts.md` | additional final document: preserve |
| `analysis/redirect/r02-cae1-review-20261127/rejection-review.md` | additional final document: preserve |
| `analysis/round-01/commentary-readability-assessment.md` | additional final document: preserve |
| `analysis/round-02/commentary-readability-assessment.md` | additional final document: preserve |
| `analysis/round-02/prior-commentary/AGENTS.md` | duplicate instruction scaffold: omit |
| `analysis/round-02/prior-commentary/progress.md` | additional final document: preserve |
| `analysis/round-02/prior-commentary/prompt.md` | duplicate instruction scaffold: omit |
| `analysis/round-02/prior-commentary/uncertainties.md` | additional final document: preserve |
| `analysis/round-02/prior-commentary/workflow.md` | duplicate instruction scaffold: omit |
| `analysis/round-02/review.md` | additional final document: preserve |
| `analysis/terminal-handoff.md` | additional final document: preserve |
| `analysis/toolkit-issues.md` | recognized toolkit-report exclusion; keep local |
| `commentary/AGENTS.md` | recognized instruction scaffold |
| `commentary/editorial-audit.md` | automatically selected final document |
| `commentary/progress.md` | automatically selected final document |
| `commentary/prompt.md` | recognized instruction scaffold |
| `commentary/uncertainties.md` | automatically selected final document |
| `commentary/workflow.md` | recognized instruction scaffold |

## Batch exception refresh

The current selection preserves all additional final and historical Markdown, including batch fallback authorization/conclusions and the entire retained review history. Only duplicate instruction scaffolds in prior-commentary snapshots are omitted. No archive was run. The selection follows the inspected archive.py FINAL/special/before/source contract. Final independent verification checks exact accounting.

- `analysis/archive-document-selection.md`: preserve additional final/history document
- `analysis/batch-completion-audit.md`: preserve additional final/history document
- `analysis/batch-terminal-handoff.md`: preserve additional final/history document
- `analysis/final-completion-audit.md`: preserve additional final/history document
- `analysis/gameplay.md`: automatic or special contract path
- `analysis/handoffs/manual-fallback-disassembly.md`: preserve additional final/history document
- `analysis/handoffs/round-01-commentary.md`: preserve additional final/history document
- `analysis/handoffs/round-01-disassembly.md`: preserve additional final/history document
- `analysis/handoffs/round-01-redirect.md`: preserve additional final/history document
- `analysis/handoffs/round-02-commentary.md`: preserve additional final/history document
- `analysis/handoffs/round-02-disassembly.md`: preserve additional final/history document
- `analysis/handoffs/round-02-redirect.md`: preserve additional final/history document
- `analysis/investigation.md`: automatic or special contract path
- `analysis/progress.md`: automatic or special contract path
- `analysis/redirect/r01-connected-20261127/data-contracts.md`: preserve additional final/history document
- `analysis/redirect/r01-connected-20261127/residual-contracts.md`: preserve additional final/history document
- `analysis/redirect/r02-cae1-review-20261127/rejection-review.md`: preserve additional final/history document
- `analysis/round-01/commentary-readability-assessment.md`: preserve additional final/history document
- `analysis/round-02/commentary-readability-assessment.md`: preserve additional final/history document
- `analysis/round-02/prior-commentary/AGENTS.md`: omit duplicate instruction scaffold
- `analysis/round-02/prior-commentary/progress.md`: preserve additional final/history document
- `analysis/round-02/prior-commentary/prompt.md`: omit duplicate instruction scaffold
- `analysis/round-02/prior-commentary/uncertainties.md`: preserve additional final/history document
- `analysis/round-02/prior-commentary/workflow.md`: omit duplicate instruction scaffold
- `analysis/round-02/review.md`: preserve additional final/history document
- `analysis/round-03/prior-commentary/AGENTS.md`: omit duplicate instruction scaffold
- `analysis/round-03/prior-commentary/editorial-audit.md`: preserve additional final/history document
- `analysis/round-03/prior-commentary/progress.md`: preserve additional final/history document
- `analysis/round-03/prior-commentary/prompt.md`: omit duplicate instruction scaffold
- `analysis/round-03/prior-commentary/uncertainties.md`: preserve additional final/history document
- `analysis/round-03/prior-commentary/workflow.md`: omit duplicate instruction scaffold
- `analysis/terminal-handoff.md`: preserve additional final/history document
- `analysis/toolkit-issues.md`: automatic or special contract path
- `commentary/AGENTS.md`: automatic or special contract path
- `commentary/editorial-audit.md`: automatic or special contract path
- `commentary/progress.md`: automatic or special contract path
- `commentary/prompt.md`: automatic or special contract path
- `commentary/uncertainties.md`: automatic or special contract path
- `commentary/workflow.md`: automatic or special contract path

- `analysis/handoffs/manual-fallback-commentary.md`: preserve final full-source commentary and strict editorial acceptance handoff.
