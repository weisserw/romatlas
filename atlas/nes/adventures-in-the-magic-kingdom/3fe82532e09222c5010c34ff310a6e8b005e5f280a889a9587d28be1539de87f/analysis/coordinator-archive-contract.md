# Archive preparation contract inspection

Coordinator inspected `/root/romatlas/scripts/archive.py`, especially FINAL and archive() document selection, on 2026-10-31. No archive was executed.

For NES, the built-in FINAL selection converts `.sym` paths to `.json`. Additional final Markdown must be workspace-relative under analysis/ or commentary/, exist, contain no parent traversal, and exclude analysis/toolkit-issues.md. Omitted documents must not overlap selected final artifacts. All recursive Markdown in analysis/ and commentary/ must be recognized by the default selection, explicitly selected, or explicitly omitted, except analysis/toolkit-issues.md and commentary/{prompt,workflow,AGENTS}.md and paths containing before or source.

Before terminal handoff, write archive-options.json with documents and omitted_documents arrays. Preserve final conclusions, phase handoffs and historical investigation explanations needed to understand the outcome. Inspect the then-current inventory and validate selection without calling archive(). This inspection alone does not establish archive readiness, editorial acceptance, or ROM completion.

## Historical commentary selection

Round-2 preservation at analysis/round-2/commentary-pass-001-preserved is retained locally in full. Its handoff, progress and uncertainties are explicitly selected in archive-options.json. Its AGENTS.md, prompt.md and workflow.md are duplicate generated instruction templates, explicitly omitted from future archive document selection; they contain no final analysis conclusions. This metadata selection removes no local files and runs no archive. Recheck the entire inventory at terminal handoff.

Round 3 preservation: select the pass-002 commentary handoff, progress and uncertainties from the full local snapshot. Its duplicate generated prompt/AGENTS/workflow templates are explicit archive omissions; no analysis conclusions or local files are removed. Recheck final inventory after later phases.
