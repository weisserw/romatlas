# Archive document selection

Inspected toolkit scripts/archive.py FINAL and archive() selection code before
terminal handoff. Root archive-options.json contains explicit workspace-relative
Markdown documents and omitted_documents arrays. The accounting check reads FINAL
from the current script, applies NES .sym-to-.json substitution, recognizes the
same exclusions and checks every Markdown file recursively under analysis/ and
commentary/. It does not call archive() or create an archive.

All additional final handoffs, structural findings, independent acceptance,
coordinator conclusion/audit and historical commentary result/uncertainty records
are selected for preservation. Default final documents retain current progress,
investigation, gameplay, uncertainties, editorial audit and review metadata.
Three preserved historical instruction copies (AGENTS.md, prompt.md, workflow.md
under analysis/round-02/commentary-pass-002/) are classified as disposable duplicate
instruction scaffolding for archive omission. They contain no phase results and
remain untouched locally. Historical progress and uncertainties are explicitly
selected, not omitted. There are no other disposable Markdown scratch documents.

The archive contract itself recognizes but excludes analysis/toolkit-issues.md
and current commentary instructional files. Final handoff summarizes the issue
and workaround so interpretation survives that contract. No final conclusion or
handoff is omitted to satisfy the selector. Internal ROM, source, assets, scratch
and binary evidence remain local; selection is preparation only, with no execution
or cleanup. archive-document-audit.json records all categories and empty unaccounted
set. Supported ROM-free archive reconstruction is separate from publication;
no sharing was performed.
