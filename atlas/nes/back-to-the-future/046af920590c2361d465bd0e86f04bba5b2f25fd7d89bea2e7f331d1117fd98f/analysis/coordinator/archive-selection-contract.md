# Archive document selection contract

Coordinator inspected `/root/romatlas/scripts/archive.py` (`FINAL`, lines 17–25;
`archive`, lines 345–404) on 2026-11-12. No archive command was executed.

The archive automatically selects the fixed `FINAL` list; NES `.sym` entries become
`.json`. Additional final Markdown must be explicitly selected with `documents`:
workspace-relative existing `.md` paths under analysis/ or commentary/, excluding
analysis/toolkit-issues.md. Additional paths cannot contain `..`.

Every other Markdown file recursively under analysis/ and commentary/ must be
selected or explicitly listed in `omitted_documents`, except toolkit-issues.md,
commentary/prompt.md, commentary/workflow.md and commentary/AGENTS.md, and paths
containing a before or source component. A selected final artifact cannot be omitted.
The archive script's automatic exceptions do not imply permission to discard local
history. Final conclusions, research results, handoffs and rejection review must be
selected when outside the fixed list; omit only disposable scratch documents.

Before terminal handoff, inventory the current filesystem again and write
archive-options.json with documents and omitted_documents arrays. Verify that the
arrays cover every unrecognized Markdown file and preserve the documents needed to
understand the result. The initial contract inspection alone is not final selection
verification. Archive execution and cleanup are not authorized.
