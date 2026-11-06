# Archive document-selection contract inspection

Coordinator inspected `/root/romatlas/scripts/archive.py` (`FINAL` and `archive`, lines 345–404) before terminal handoff. No archive was executed.

The archive selects its built-in FINAL metadata, with `.sym` changed to `.json` for NES. Additional final Markdown must be explicitly supplied through `documents`. Such paths must exist under `analysis/` or `commentary/`, have `.md` suffix, contain no parent traversal, and exclude `analysis/toolkit-issues.md`. `omitted_documents` must not intersect selected artifacts. Every Markdown under those directories must be accounted for, except built-in selected documents, toolkit issues, commentary prompt/workflow/AGENTS, and files under source/before directories.

At terminal handoff, `archive-options.json` must preserve all final conclusions, phase handoffs and necessary investigation records through explicit selection; only disposable scratch may be omitted. Its completeness must be checked against the then-current filesystem. This note itself is a final coordinator record and should be selected.

Archive preparation is not execution permission. Internal state and restored source remain local; only the supported ROM-free portable mapping is a sharing deliverable.

## Final editorial review schema prerequisite

Additional read-only inspection of `archive.editorial_status` and `commentary.audit`
shows that each Code region requires a nonempty per-region `units` array. A compact
top-level `semantic_units` inventory plus per-region `unit_id` alone does not satisfy
these supported audit functions. Before final acceptance, preserve the compact review
credit but project the applicable reviewed semantic unit into each Code region's
`units` field, with valid name, physical offset/length, summary and source locations.
This is review metadata adaptation, not a need to reread unchanged sound commentary
or modify the toolkit. Final editorial audit still needs real targeted source review
and fresh byte-identical verification. Do not mark the pending editorial audit reviewed
merely to satisfy schema. Next commentary/final acceptance worker must address this.

## Document omission rationale

`archive-options.json` selects additional final analysis/commentary records explicitly,
including historical findings/frontiers/handoffs and preserved round001 progress and
uncertainties. The only omitted Markdown documents are the generated instruction
scaffolds `analysis/round-002/commentary-round-001/AGENTS.md`, `prompt.md` and
`workflow.md`. They duplicate the phase setup instructions already present in the
workspace/current commentary and contain no investigation result. They are disposable
setup copies, not omitted conclusions. Files remain on disk; this selection performs
no cleanup. Final audit/handoff documents must be added once written and the full
recursive inventory must then be revalidated.
