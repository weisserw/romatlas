# Review the shared library changes from this port

No context-management plugin is required. Use local scripts and focused searches
for large diffs, inventories and test logs; return relevant excerpts, counts,
failures and evidence paths. Inspect actual changed code and dependencies before
concluding; summaries do not establish correctness. Keep full evidence on disk,
use direct tools for short reads, and record concise findings and next actions.
Retrieve older details on demand without reducing review scope or verification.

Read workspace.json, library-changes.md, progress.md, differences.md and ledger.json.
The toolkit directory is in workspace.json. Compare its current files against
review/toolkit-before/ and the hashes/status/HEAD in review/toolkit-before.json.
That snapshot includes tracked and nonignored untracked files from preparation;
pre-existing changes are not automatically attributed to this port agent.

Review the actual port-driven diff. Verify shared utilities have game-independent
contracts or explicit platform-specific scope; game rules belong in the port.
Check ROM identity, bounded reads, asset replacement/extension, data hygiene,
headless separation, console cadence and customization. Review tests and run
relevant toolkit checks. Do not require GUI/device tests unavailable on this host.
Report regressions, unnecessary abstractions, missing evidence and required fixes.
Check the library-change log accounts for the relevant diff and preserves unrelated
changes. The review is not an automatic check-in or release authorization.

Write review/findings.md with findings, validation and a clear disposition. The
user decides check-in. After review/check-in, a later release step can pin the
library commit. No agent is launched automatically by this workflow.
