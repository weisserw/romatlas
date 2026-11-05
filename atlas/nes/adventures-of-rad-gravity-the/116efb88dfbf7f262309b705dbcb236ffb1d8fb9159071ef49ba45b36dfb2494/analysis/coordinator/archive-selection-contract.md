# Archive document selection contract

Inspected `/root/romatlas/scripts/archive.py` (`FINAL` and `archive`, lines 17–25 and 345–404). This is preparation only; no archive or cleanup has run.

The default selection retains investigation, gameplay, progress, commentary progress, uncertainties and editorial audit Markdown, together with configured final machine artifacts. NES substitutes `.json` for `.sym`. Additional final Markdown under analysis/ or commentary/ must be explicitly listed in `documents`; toolkit-issues.md cannot be added through this mechanism. Additional files must exist and remain workspace relative.

Every other Markdown under analysis/ and commentary/ must occur in `documents` or `omitted_documents`, except the recognized toolkit-issues.md, commentary/prompt.md, commentary/workflow.md, commentary/AGENTS.md and files below before/ or source/. A selected final artifact cannot be omitted. Omission is a selection decision, not permission to delete workspace files.

At terminal handoff the coordinator will enumerate actual Markdown, select final conclusions, phase handoffs and historical reasoning needed to understand the result, and classify only disposable scratch as omitted. It will write archive-options.json and verify coverage without invoking archive(). Selection alone does not establish editorial or ROM completion.
