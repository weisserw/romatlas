# Coordinator archive document-selection contract

Inspected `/root/romatlas/scripts/archive.py`, `FINAL` and `archive()` document selection, during this orchestration. No archive execution is authorized or performed.

For NES, the script transforms `.sym` entries in `FINAL` to `.json`. Its default Markdown selection is analysis/investigation.md, analysis/gameplay.md, analysis/progress.md, commentary/progress.md, commentary/uncertainties.md and commentary/editorial-audit.md. Additional final documents must be workspace-relative Markdown files beneath analysis/ or commentary/, without parent traversal. analysis/toolkit-issues.md cannot be an additional selected document; the script specially recognizes it alongside commentary/prompt.md, commentary/workflow.md and commentary/AGENTS.md.

Every other Markdown file recursively beneath analysis/ and commentary/ must be explicitly selected through documents or classified through omitted_documents, except files with a before or source path component. Selected final artifacts cannot also be omitted. Historical plans/results, conclusions, rejection reviews and phase handoffs needed to understand the result should be selected; omissions are for disposable scratch, not inconvenient final results. The final archive-options.json must be checked against the current filesystem after all workers stop, rather than a stale intermediate inventory.

Archive preparation is not ROM completion. This workflow has three unresolved bytes after pass-008 and still requires commentary, redirect and the applicable coordinator stopping review. No archive, cleanup, publication or toolkit modification is authorized here.
