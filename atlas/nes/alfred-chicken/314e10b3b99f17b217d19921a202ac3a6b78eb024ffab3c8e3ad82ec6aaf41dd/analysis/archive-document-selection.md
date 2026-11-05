# Archive document-selection preparation — no archive executed

Read current /root/romatlas/scripts/archive.py FINAL and archive() document selection.
NES replaces default .sym paths with .json. Additional documents must be existing
workspace-relative Markdown under analysis/ or commentary/, excluding toolkit-issues.md.
Explicit omissions must not overlap selected final artifacts. Recognized special
exclusions are analysis/toolkit-issues.md and commentary/prompt.md, workflow.md, AGENTS.md.
The script excludes Markdown beneath any component named before or source from its
unrecognized-document check. No archive function or archive command was executed.

archive-options.json explicitly selects every other current analysis/commentary
Markdown containing result, uncertainty, investigation, correction, progress, handoff,
terminal review, acceptance or historical conclusion. Superseded terminal drafts are
preserved together with their explicit corrected/superseding handoffs, so history is
not mistaken for the current outcome. Final conclusions are never omitted to pass.

omitted_documents contains only nested AGENTS.md/workflow.md/prompt.md copies whose
bytes exactly equal the current recognized commentary instruction file. These are
disposable duplicate phase instructions, not unique findings or final results.
Historical progress/uncertainty documents remain selected. Every omitted file remains
intact locally; selection classification performs no deletion. There are no other
unrecognized disposable scratch Markdown documents in the current inventory.

archive-document-audit.json records every Markdown path and contract disposition,
missing/overlap/unaccounted checks and current archive-options.json hash. The final
independent current-audit.json checks that plan. Future new Markdown requires a new
inventory; the plan contains arrays for a future authorized caller to translate into
supported --document/--omit-document options, not an invented automatic options API.
All ROM/source/assets/state/traces stay local. No archive execution or cleanup occurs.
