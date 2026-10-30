# Archive document selection contract

Coordinator inspected `/root/romatlas/scripts/archive.py` before terminal handoff; script identity is recorded in `analysis/archive-contract.json`. No archive was executed.

`FINAL` (lines 16–26) supplies standard metadata and documents, translating `.sym` to `.json` for NES. Additional final Markdown must be explicitly selected with `documents`; it must exist under `analysis/` or `commentary/`, contain no `..` components, and cannot be `analysis/toolkit-issues.md`. `omitted_documents` explicitly classifies scratch; selected final artifacts cannot be omitted. The archive rejects unrecognized Markdown recursively in those directories, except paths containing `before` or `source`, toolkit issues, and commentary prompt/workflow/AGENTS documents (lines 354–381).

At terminal handoff the coordinator will write workspace-root `archive-options.json` with both arrays and verify that every unrecognized Markdown path is accounted for. Preserve substantive findings, rejection reviews, historical investigation results and phase handoffs as additional final documents; omission is for disposable scratch only. This document itself is final coordination evidence and must be selected. Selection preparation does not authorize archive execution or cleanup.

Round002 preservation selection retains prior commentary progress and uncertainty conclusions from the frozen commentary-prior snapshot. Its duplicate prompt/workflow/AGENTS instruction copies are classified as disposable administrative copies in omitted_documents; no source, conclusion, handoff or investigation result is omitted for this reason. These are selection entries only, with no deletion or archive execution. Final recursive accounting remains required after terminal records exist.
