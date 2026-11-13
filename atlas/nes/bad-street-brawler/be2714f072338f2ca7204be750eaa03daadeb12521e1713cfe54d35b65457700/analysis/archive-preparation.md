# Archive preparation contract

Coordinator inspected `/root/romatlas/scripts/archive.py` (the `FINAL` list and `archive` document-selection logic) before terminal handoff preparation. No archive was executed.

`archive-options.json` must contain `documents` and `omitted_documents` arrays of workspace-relative Markdown paths. Additional selected documents must exist under analysis/ or commentary/, have suffix .md, contain no parent traversal, and exclude analysis/toolkit-issues.md. Built-in FINAL documents are already selected and cannot be omitted.

The recognition scan recursively checks Markdown under analysis/ and commentary/, excluding paths with a before/ or source/ component. It recognizes built-in selections, explicitly selected/omitted paths, analysis/toolkit-issues.md, commentary/prompt.md, commentary/workflow.md and commentary/AGENTS.md. Every remaining document requires an explicit disposition. Preserve final conclusions, rejection reviews and phase handoffs; omission is for disposable scratch only.

Final inventory and options are pending completion of the sequential phase workflow. Archive execution, cleanup and publication are not authorized.

## Preserved pass001 commentary snapshot

Round002 commentary retained the complete prior commentary tree under analysis/round-002/commentary-pass-001/commentary/. Select its progress.md and uncertainties.md as historical final conclusions. Explicitly omit only the duplicate prompt.md, workflow.md and AGENTS.md instruction copies from archive document selection; these remain local and do not contain final analysis results. Frozen before/source subtrees are excluded from the script document recognition scan by contract. This is selection preparation only, with no archive or cleanup execution.

Round003 additionally preserves the complete pass002 commentary snapshot under analysis/round-003/commentary-pass-002/commentary/. Its historical progress/uncertainties are explicitly selected; only its three duplicate phase-instruction copies are omitted from document selection, remaining local. No conclusions or handoffs are omitted.
