# Archive document-selection contract

Inspected `/root/romatlas/scripts/archive.py`, particularly `FINAL` and `archive()` document selection (lines 379–404). No archive was executed.

The archive automatically selects the existing paths in `FINAL` (with `.sym` translated to `.json` for NES). Additional final Markdown must be explicitly selected in `documents`: paths must be workspace-relative under `analysis/` or `commentary/`, end in `.md`, contain no `..`, and exist. `analysis/toolkit-issues.md` is explicitly excluded from additional selection by the toolkit contract; retain it locally and report its findings in final handoffs.

`omitted_documents` records disposable scratch Markdown. A selected final artifact cannot be omitted. The contract separately recognizes `analysis/toolkit-issues.md`, `commentary/prompt.md`, `commentary/workflow.md`, and `commentary/AGENTS.md`; Markdown beneath a path component named `before` or `source` is excluded from the unrecognized-document scan. All other Markdown beneath analysis/commentary must be covered by default selection, explicit documents, or omitted_documents.

Before terminal handoff, generate and validate `archive-options.json` against the current tree. Preserve final conclusions, review rationale, phase handoffs and durable investigation history; do not omit results to satisfy selection. A selection-only check must not invoke archive(), publish, clean up, or claim classification completion. This document itself is a final coordinator record and must be selected.


## Final editorial readiness contract inspected

Also inspected archive.py `editorial_status()` and scripts/commentary.py `audit()` (lines356–393). The final inventory must match the frozen scaffold exactly; every region and meaningful unit needs reviewed status, a nonempty summary and valid source locations; each Code region needs units; units need valid names/ranges. `editorial_audit` requires reviewed status and nonempty notes. The audit then runs fresh verification, refreshes reading-after.asm and saves audit.json. Explained unknowns are permitted; neither inventory/editorial acceptance nor byte-identical verification establishes whole-ROM classification. The tool cannot judge semantic/prose correctness, so targeted review of known defects, changed evidence and dependencies is still required. Reserve this operation for final acceptance after current-source review and the terminal workflow rationale are ready. No archive operation or cleanup is authorized.
