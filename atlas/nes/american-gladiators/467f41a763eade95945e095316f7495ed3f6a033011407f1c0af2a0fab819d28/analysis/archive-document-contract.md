# Archive document-selection contract

Coordinator inspected `/root/romatlas/scripts/archive.py` before terminal handoff. No archive execution is authorized or performed.

For this NES workspace, the script preserves its FINAL inventory with `.sym` replaced by `.json`. Additional final Markdown paths must be explicitly supplied through `documents` (`--document`), be workspace-relative under analysis/ or commentary/, and exist. Toolkit issues are excluded by the contract. `omitted_documents` (`--omit-document`) records disposable documents and may not overlap selected artifacts.

Every recursive Markdown file under analysis/ and commentary/ must be a default final document, explicitly selected, explicitly omitted, or a contract exemption: analysis/toolkit-issues.md; commentary/prompt.md; commentary/workflow.md; commentary/AGENTS.md; paths containing a before or source component. Additional final handoffs and conclusions must be selected, not omitted to satisfy validation.

At terminal handoff the coordinator will write archive-options.json and verify complete path accounting against the current inventory. This contract note is itself an additional final analysis document. No cleanup is authorized.
