# Archive document-selection contract

Coordinator inspected `/root/romatlas/scripts/archive.py`, `FINAL` and `archive()` (lines 345–404), before terminal handoff. No archive was executed.

The archive implicitly selects Markdown in `FINAL`: analysis/investigation.md, analysis/gameplay.md, analysis/progress.md, commentary/progress.md, commentary/uncertainties.md and commentary/editorial-audit.md. Additional final Markdown must be explicitly selected using workspace-relative analysis/ or commentary/ paths; toolkit issues cannot be explicitly selected. Every otherwise unrecognized Markdown file under those directories must be selected or explicitly omitted. The toolkit recognizes analysis/toolkit-issues.md and commentary/{prompt,workflow,AGENTS}.md separately, and excludes files nested in before/ or source/ from this unknown-document check. Selected final artifacts cannot be omitted.

At terminal preparation, archive-options.json must enumerate additional final documents and disposable scratch omissions. Preserve conclusions, phase handoffs, investigation history and review reasoning. Reconcile the final on-disk inventory after all workers finish; this note is itself a final coordinator document to select. Archive execution, cleanup, publishing and ROM classification completion are separate claims and are not authorized by this preparation.

## Final editorial verification contract

Coordinator additionally inspected archive.editorial_status and commentary.audit. The archive status helper reports editorial completeness but does not itself require it for archive creation. This orchestration goal nevertheless requires final current-source acceptance: every frozen-baseline scaffold region must have reviewed status, a nonempty summary and valid source locations; Code regions require reviewed units with valid ranges/names/source references; final editorial_audit must be reviewed with substantive notes. commentary.audit validates that inventory, runs fresh byte-identical verification, refreshes reading-after.asm and writes commentary/audit.json. Its green result checks inventory/fidelity, not adequacy of prose; targeted actual-source editorial review and explicit unresolved semantics remain necessary. No audit was executed during active disassembly, and no archive invocation is authorized.

## Round-2 preserved copies

Both analysis/round-2/commentary and commentary-working-prior hold complete previous commentary. Coordinator confirmed their progress/uncertainty and template Markdown copies are pairwise byte-identical. Archive options explicitly retain historical progress/uncertainties in both copies; copied prompt/workflow/AGENTS templates are explicitly omitted as duplicated workflow scaffolding. No files were removed. This is provisional document selection, to be rechecked at terminal handoff.

## Final reconciliation

Final contract-derived inventory check passed:46applicable Markdown files,27explicit
additional final/history selections,9omitted byte-identical copied workflow templates,
zero unrecognized/unaccounted files. Canonical templates remain. All paths are valid
workspace-relative Markdown; selected/omitted sets are disjoint and no selected final
artifact is omitted. Receipt:document-selection-audit.json. Archive options are now
final for this terminal handoff. No archive or cleanup was executed.
