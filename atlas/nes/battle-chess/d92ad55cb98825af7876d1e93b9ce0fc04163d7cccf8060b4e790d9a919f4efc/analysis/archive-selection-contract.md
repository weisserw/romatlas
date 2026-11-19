# Archive document selection preparation

Coordinator inspected `/root/romatlas/scripts/archive.py`, `FINAL` and lines 375–404, read-only. No archive execution or cleanup is authorized or performed.

The archive's default selection includes progress, investigation, gameplay and standard commentary review/uncertainty/editorial records. Additional Markdown must be explicitly selected through `documents`; unrecognized scratch Markdown must be accounted for through `omitted_documents`. Toolkit issues and commentary prompt/workflow/AGENTS are special recognized exclusions. Markdown inside source/before is excluded from this document-accounting scan. Additional selections must be workspace-relative existing Markdown in analysis/ or commentary/, excluding toolkit issues.

`archive-options.json` is a preparation manifest, not an invocation. Its arrays correspond to the contract's additional `--document` and `--omit-document` selections. It is checked against the final inventory before terminal handoff. Phase handoffs, research outcomes/frontiers and final coordinator conclusions are selected; no final result is omitted to bypass validation.

Omission rationale: analysis/scratch/bank-table.md is a disposable formatting intermediate whose complete table is already preserved in analysis/progress.md and analysis/investigation.md. The file remains local; omission does not delete it.

Historical investigation frontiers, plans, corrected/rejected reasoning, independent phase reviews and preserved commentary progress/uncertainty/instruction records are retained as provenance. They are not scratch merely because newer results supersede them. Standard final documents use the toolkit's default selection; its special recognized exclusions remain local and are not silently treated as additional selectable documents.

Status: final preparation. `analysis/archive-selection-verification.json` records the exact contract digest, complete Markdown accounting, selected/omitted paths and missing/unaccounted checks. This preparation executes no archive code and creates no archive destination. See [final handoff](final-handoff.md) and [completion audit](completion-audit.md).
