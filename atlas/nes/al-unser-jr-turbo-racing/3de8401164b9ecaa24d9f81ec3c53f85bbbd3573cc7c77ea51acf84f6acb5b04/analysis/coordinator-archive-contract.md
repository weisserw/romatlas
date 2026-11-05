# Coordinator archive document-selection contract

Inspected `/root/romatlas/scripts/archive.py` before terminal handoff; no archive executed.

The `archive` function selects `FINAL` artifacts, adapting `.sym` names to `.json` for NES. Additional `documents` must be existing workspace-relative `.md` paths under analysis/ or commentary/, with no parent traversal, excluding analysis/toolkit-issues.md. `omitted_documents` cannot overlap selected artifacts. All recursively discovered Markdown in those directories must be selected, explicitly omitted, or recognized by the built-in exceptions (toolkit-issues.md, commentary/prompt.md, commentary/workflow.md, commentary/AGENTS.md); paths containing before or source are excluded from this scan.

At terminal handoff, generate archive-options.json with documents and omitted_documents arrays. Preserve final conclusions, phase handoffs and investigation records needed to understand the result; identify disposable scratch explicitly. Recheck the current Markdown inventory and contract before final acceptance. This record is itself a final additional analysis document to select. No archive, cleanup, deletion or publication is authorized.
