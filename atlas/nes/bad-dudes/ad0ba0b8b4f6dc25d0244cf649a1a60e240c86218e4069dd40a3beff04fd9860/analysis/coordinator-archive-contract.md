# Archive preparation contract

Coordinator inspected `/root/romatlas/scripts/archive.py`, especially `FINAL` and `archive()` document selection, on 2026-11-12. No archive was executed.

Before terminal handoff, create workspace-root `archive-options.json` with `documents` and `omitted_documents` arrays of workspace-relative Markdown paths. `documents` selects additional final Markdown under analysis/ or commentary/; toolkit issues cannot be selected. Preserve final phase handoffs, conclusions and rejection reviews needed to understand the result. `omitted_documents` explicitly accounts for disposable scratch only; no file is deleted by this preparation.

The script recognizes its FINAL inventory automatically, plus analysis/toolkit-issues.md, commentary/prompt.md, commentary/workflow.md and commentary/AGENTS.md. Markdown beneath a path component named before or source is excluded from its unrecognized-document scan. All other recursive Markdown in analysis/ and commentary/ must be selected or omitted. An omission cannot overlap a selected final artifact. Additional selections must exist, have .md suffix, have analysis or commentary as their first path component, and contain no parent traversal.

This document itself is a final coordinator record and must be selected. Final inventory and selection validation remain pending until the phase workflow reaches its terminal handoff. The script also performs source restoration and verification when archiving; inspection here does not claim those operations ran or authorize them.


Terminal preparation verification (2026-11-19): the actual archive.py contract was re-inspected. archive-options.json now selects 105 additional final Markdown documents and explicitly omits 27 preserved duplicate instruction templates. Every unrecognized Markdown path in analysis/ and commentary/ is accounted for; no selected/omitted path is missing or overlaps. Final rejection review, editorial conclusions and coordinator handoff are selected. See analysis/final-archive-selection-verification.json. No archive execution or cleanup occurred.
