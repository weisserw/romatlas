# Review the shared library changes from this port

Read workspace.json, library-changes.md, progress.md, differences.md and
ledger.json. Read uncertainties.json and research/current.json when present.
Inspect the adopted reference revision's changes/evidence against the frozen
source. Check that unknown fragments and semantic ambiguities have
consumer-specific port-impact assessments; preserving code bytes must not stand
in for implementing behavior. Verify original ledger identities and frozen
provenance remain intact, and local corrections have fresh map/source rebuild
evidence. A byte-identical revision does not prove its interpretation or the
native gameplay equivalent correct. Report missing implementations and weak
exclusions. An implemented, documented reasonable assumption does not require
runtime confirmation. Complete original classification and independent gameplay
fidelity are not port gates. The toolkit directory is in workspace.json. Compare
its current files against review/toolkit-before/ and the hashes/status/HEAD in
review/toolkit-before.json. That snapshot includes tracked and nonignored
untracked files from preparation; pre-existing changes are not automatically
attributed to this port agent.

Read architecture.md to understand intended module responsibilities and state
ownership. Review the actual port-driven diff. Verify shared utilities have
game-independent contracts or explicit platform-specific scope; game rules
belong in the port. Check ROM identity, bounded reads, asset
replacement/extension, data hygiene, headless separation, console cadence and
customization. Review focused unit tests and run relevant toolkit checks.
Review the recorded smoke-test.md scope/results and concrete unresolved failures.
Do not launch a second smoke test merely for review confidence. Broader fidelity
validation is outside this review; do not request full playthroughs, exhaustive
emulator/native comparisons or GUI/device checks.
Debugger use, if needed for a specific source question, follows
prompts/start.md's bounded inquiry rule. Do not expand review into an
original-game equivalence audit. A later fidelity audit requires an explicit
user request. Stop once concrete findings and the disposition are written; do
not repeat checks to accumulate confidence. Report regressions, unnecessary
abstractions, missing evidence and required fixes. Check the library-change log
accounts for the relevant diff and preserves unrelated changes. The review is
not an automatic check-in or release authorization.

Write review/findings.md with findings, validation and a clear disposition. The
user decides check-in. After review/check-in, a later release step can pin the
library commit. No agent is launched automatically by this workflow.
