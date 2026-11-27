# Archive document selection contract

The coordinator inspected `/root/romatlas/scripts/archive.py`, especially `FINAL`
and `archive()` document selection, before terminal handoff. No archive command
was run. `archive-options.json` is a preparation artifact, not authorization to
archive, publish, clean up or delete anything.

For NES, the script substitutes `.json` for default `.sym` paths. Additional
`documents` must be existing workspace-relative Markdown paths under `analysis/`
or `commentary/`, without traversal; toolkit-issues.md cannot be selected. Selected
final artifacts cannot also be omitted. The script recursively accounts for all
Markdown in those two trees, except paths containing `before` or `source`.
Default final paths and its four explicitly recognized instruction/issue paths
need no extra selection. Recognized does not necessarily mean copied: the toolkit
issue report and current commentary instruction files are excluded by contract.

The additional selection preserves every substantive phase handoff, redirect
finding, independent disassembly review, historical commentary progress and
uncertainty record, final acceptance and coordinator conclusions. Historical links
may reference local JSON, captures or source that the archive does not retain;
these Markdown records preserve conclusions and reasons, not the entire internal
investigation. All original evidence remains in this local workspace.

The only explicit omissions are redundant historical commentary instruction
snapshots (`AGENTS.md`, `prompt.md`, `workflow.md` under `commentary-prior`). These
are disposable copies of preparation instructions, not results or unique evidence.
They remain on disk. Historical progress and uncertainties are selected, not
omitted. No final result is omitted to satisfy document recognition.

At terminal handoff the coordinator refreshes the arrays against a recursive
filesystem enumeration, independent of gitignore, and records zero unaccounted
Markdown documents in the final verification artifact. No archive execution or
cleanup is part of this workflow.
