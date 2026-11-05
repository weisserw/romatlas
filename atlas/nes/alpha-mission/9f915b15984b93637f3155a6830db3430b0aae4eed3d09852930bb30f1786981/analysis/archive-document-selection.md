# Archive document selection preparation

Inspected /root/romatlas/scripts/archive.py FINAL and archive() selection contract (lines 16–25, 378–404). This document describes preparation only; archive() was never invoked.

NES converts built-in .sym selections to .json. Built-in final Markdown is retained automatically. Additional documents must be workspace-relative .md paths under analysis/ or commentary/, excluding toolkit-issues.md. Selected final artifacts cannot be omitted. All other Markdown under these directories must be selected or explicitly omitted, except toolkit-issues.md and generated commentary instructions (prompt.md, workflow.md, AGENTS.md); trees named before/source are excluded from this check.

archive-options.json selects all additional current conclusions, phase handoffs, bank counts, coordinator reviews, final acceptance and historical redirect plan/frontier/rejection records. No existing Markdown is classified as disposable scratch: the apparent plan/frontier files document the sole research attempt and its stopping rationale and are preserved. Consequently omitted_documents is empty. This is intentional retention, not an attempt to omit final results to satisfy archive validation. Non-Markdown scratch is outside this document-selection contract and no cleanup is performed.

The coordinator document-selection check independently compares the complete current Markdown inventory with these exact built-in selections/exemptions and archive-options.json. Its JSON report records every path and disposition; unaccounted paths must be empty before handoff. Final docs do not make local ROM/source/state artifacts public. Nothing is published or archived here.
