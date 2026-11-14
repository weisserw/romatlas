# Archive document-selection contract

Coordinator inspected `/root/romatlas/scripts/archive.py` (`FINAL`, `archive`, and
`editorial_status`) during round 1. No archive or cleanup was executed.

At terminal handoff, `archive-options.json` must contain `documents` and
`omitted_documents` arrays of workspace-relative Markdown paths. Additional final
analysis/commentary conclusions, phase handoffs and rejection reviews belong in
`documents`; only disposable scratch documents belong in `omitted_documents`.
The selection must be recomputed after the final worker finishes.

The script automatically selects investigation, gameplay and progress under
analysis, and progress, uncertainties and editorial-audit under commentary, along
with its fixed map/state/review/verification artifacts. Additional `documents`
must exist, have `.md` suffix, reside under analysis or commentary, and contain no
parent traversal. `analysis/toolkit-issues.md` is excluded from additional selection
by the script and treated as known. Commentary prompt/workflow/AGENTS are known;
Markdown under `before` or `source` path components is exempt from the unknown
Markdown check. All other recursively discovered analysis/commentary Markdown
must be explicitly selected or omitted. Selected artifacts cannot also be omitted.

Archive readiness requires current frozen-baseline identities, complete review
inventory, source locations, final editorial audit and current byte-identical
verification to be assessed separately from ROM classification. The archive
script can report incomplete editorial status; that does not satisfy this goal's
current-source readability requirement. No archive invocation is authorized.
