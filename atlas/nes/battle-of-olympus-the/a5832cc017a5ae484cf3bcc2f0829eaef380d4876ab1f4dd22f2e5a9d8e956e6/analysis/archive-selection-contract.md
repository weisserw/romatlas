# Archive document selection contract

Coordinator inspected `/root/romatlas/scripts/archive.py`, `FINAL` and `archive()`
on 2026-11-19. No archive execution is authorized or performed.

Before terminal handoff, `archive-options.json` must contain `documents` and
`omitted_documents` arrays of workspace-relative Markdown paths. Additional final
analysis/commentary records go in `documents`; only disposable scratch goes in
`omitted_documents`. Preserve final conclusions, phase handoffs, rejection reviews
and investigation history needed to understand the result.

The script automatically selects Markdown in `FINAL`: analysis/investigation.md,
analysis/gameplay.md, analysis/progress.md, commentary/progress.md,
commentary/uncertainties.md and commentary/editorial-audit.md. It also recognizes
analysis/toolkit-issues.md and commentary/{prompt,workflow,AGENTS}.md separately.
Markdown nested below a path component named `before` or `source` is excluded from
the unrecognized-document scan. Every other Markdown file recursively below
analysis/ or commentary/ must be selected or explicitly omitted. Toolkit issues
cannot be passed as an additional document; selected final artifacts cannot be
omitted. Additional documents must exist, end in `.md`, begin with analysis/ or
commentary/, and contain no `..` path component.

The coordinator must recompute the inventory from the terminal current worktree;
this early contract inspection is not evidence of terminal inventory coverage.
