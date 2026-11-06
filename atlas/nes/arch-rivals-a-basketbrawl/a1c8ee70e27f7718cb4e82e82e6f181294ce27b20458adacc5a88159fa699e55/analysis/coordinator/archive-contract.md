# Archive document selection contract

Inspected `/root/romatlas/scripts/archive.py` FINAL and archive() document selection before terminal handoff. No archive executed.

NES uses JSON equivalents of built-in .sym entries. Additional final documents must be existing workspace-relative Markdown under analysis/ or commentary/, without parent traversal; toolkit-issues.md is explicitly excluded. Omitted documents may not overlap selected final artifacts. Every other Markdown file under analysis/ and commentary/ must be selected or explicitly omitted, except toolkit-issues.md, commentary/prompt.md, commentary/workflow.md, commentary/AGENTS.md and paths containing before or source.

At terminal handoff, inventory current Markdown, preserve conclusions and phase/rejection handoffs as additional documents, classify only disposable scratch as omitted, and write archive-options.json with documents and omitted_documents arrays. Validate selection without calling archive(), which writes an archive and is unauthorized.
