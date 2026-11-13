# Archive document-selection contract inspection

Inspected `/root/romatlas/scripts/archive.py`, `FINAL` and `archive()` document-selection logic before terminal handoff. No archive command was executed.

The script automatically selects analysis investigation/gameplay/progress and commentary progress/uncertainties/editorial-audit Markdown along with its non-Markdown artifacts. Additional final Markdown under analysis/ or commentary/ must be explicitly selected. Toolkit issues and commentary prompt/workflow/AGENTS are recognized exclusions; paths under before/ or source/ are excluded from the unrecognized-Markdown check. All other recursively discovered Markdown must be selected or explicitly omitted. Selected final documents cannot be omitted; toolkit issues cannot be additionally selected. Relative paths must be local, with no traversal.

At final handoff the coordinator will inventory current files and write workspace-root archive-options.json with documents and omitted_documents arrays. Final findings, phase handoffs and rejection reviews must be selected; only disposable scratch may be omitted. This note itself is a final coordinator record to select. Selection preparation does not authorize archive execution, cleanup or publication.
