# Archive document-selection contract inspection

Inspected `/root/romatlas/scripts/archive.py` FINAL and archive() document-selection logic (lines 16–25 and 322–410), 2026-10-30. No archive command was executed.

For NES, predefined `.sym` paths become `.json`. Default Markdown selections are analysis/investigation.md, analysis/gameplay.md, analysis/progress.md, commentary/progress.md, commentary/uncertainties.md, and commentary/editorial-audit.md. Additional final Markdown must be explicitly listed in archive-options.json documents, under analysis/ or commentary/, existing locally, with no parent traversal. analysis/toolkit-issues.md is excluded by the archive contract.

Every other Markdown under analysis/ and commentary/ must be selected or explicitly omitted, except commentary/prompt.md, commentary/workflow.md, commentary/AGENTS.md, analysis/toolkit-issues.md and paths with before/source components. Selected final artifacts cannot be omitted. Final conclusions, phase handoffs and rejection reviews must be preserved, including this contract record. Disposable scratch may be omitted only with an explicit reason; omission is selection metadata, not deletion.

At terminal handoff the coordinator will enumerate the then-current Markdown tree, write archive-options.json with documents and omitted_documents arrays, and verify exhaustive accounting without invoking archive(). Selection is deferred until all phase results exist.
