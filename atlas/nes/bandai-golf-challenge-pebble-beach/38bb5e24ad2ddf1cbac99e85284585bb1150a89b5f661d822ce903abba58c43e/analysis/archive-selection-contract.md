# Archive document selection contract

Coordinator inspection: `/root/romatlas/scripts/archive.py`, `FINAL` and
`archive()` document selection (lines 17–25 and 375–404). This is preparation
only; no archive execution, cleanup or publication is authorized.

For NES, built-in `.sym` paths are replaced with `.json`. Built-in final
Markdown comprises `analysis/investigation.md`, `analysis/gameplay.md`,
`analysis/progress.md`, `commentary/progress.md`, `commentary/uncertainties.md`
and `commentary/editorial-audit.md`. Additional final Markdown must be explicitly
selected using workspace-relative paths under `analysis/` or `commentary/`.
Toolkit issues are explicitly excluded by the script's document contract.
`commentary/prompt.md`, `commentary/workflow.md` and `commentary/AGENTS.md`
are recognized infrastructure documents. Markdown within a `before` or `source`
path component is excluded from the unrecognized-document check.

Every other Markdown file recursively under `analysis/` and `commentary/` must
be selected as an additional final document or explicitly omitted. Selected
artifacts cannot also be omitted. Final conclusions, reasoning reviews, compact
phase handoffs and historical investigation explanations must remain selected;
only disposable scratch may be classified for omission. An empty omission list
is valid if there is no disposable Markdown.

Before terminal handoff the coordinator must refresh `archive-options.json`
with `documents` and `omitted_documents` arrays and check the current recursive
Markdown inventory against the above rules. This note itself is an additional
final document. Selection readiness does not establish editorial acceptance,
whole-ROM classification or archive execution.
