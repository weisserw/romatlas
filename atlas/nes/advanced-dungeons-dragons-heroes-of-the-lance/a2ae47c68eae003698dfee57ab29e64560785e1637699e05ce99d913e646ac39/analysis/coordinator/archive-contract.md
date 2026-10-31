# Archive preparation contract

Coordinator inspected `/root/romatlas/scripts/archive.py`, especially FINAL and archive() document selection (lines 16–25 and 378–404). No archive was executed.

For NES, built-in FINAL `.sym` paths become `.json`. Additional final Markdown must be explicitly listed in `archive-options.json` documents, relative to the workspace, beneath analysis/ or commentary/, without parent traversal. Toolkit issues cannot be an additional document. Omitted documents must not overlap selected finals. The script recognizes toolkit-issues.md and commentary/prompt.md, workflow.md and AGENTS.md separately; Markdown beneath source/ or before/ is excluded from its unrecognized-document scan. Every other Markdown under analysis/ and commentary/ must be built-in selected, explicitly selected, or explicitly omitted.

At terminal handoff, inventory the current filesystem again; retain phase handoffs, conclusions and investigation history needed to understand the result. Omit only disposable scratch. Validate selection without invoking archive or cleanup. This document is itself final coordinator evidence and must be selected.

## Editorial acceptance metadata

Also inspected `editorial_status()` (archive.py lines 274–343). It checks ROM/frozen baseline identity, exact scaffold identity, reviewed region/unit summaries and valid source locations, positive in-ROM unit ranges/names, at least one reviewed unit for every Code region, and a reviewed editorial_audit with nonempty notes. These metadata checks cannot assess prose quality or establish whole-ROM classification. Final readiness therefore needs the reserved targeted editorial review, current-source verify/read/audit artifacts, and an independent classification/termination assessment. The archive function calls source verification and map/store validation, but was not invoked.
