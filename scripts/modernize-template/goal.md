# Optional modernization beta — gated on completed full QA

Preparation installs this workflow only. It neither starts an agent nor authorizes
a pilot, candidate investigation or gameplay/code changes. This is not a sixth
stage of the fixed port pipeline and is not required for port completion.
Start it only after a separate user request to modernize this port and a completed
full whole-game QA sweep. Here, full QA means the existing agreed two-pass
whole-game contract, with both passes complete, issues resolved and the QA record
audit passing. A single check, partial pass, scoped milestone or synthetic tooling
test is insufficient. Never run QA automatically to open this gate.

## Entry and faithful baseline

Use the workspace (parent of modernize/) as the working directory.
Read workspace.json for the toolkit path, DEBUGGING.md, architecture.md,
behaviors.md, differences.md, uncertainties.json and the source/test contracts.
Paths in this paragraph are workspace-relative; this directory is modernize/.
Apply prompts/coding-standards.md (or established game standards), repository and
platform boundaries. Original ROM/source/assets and captures remain local.

From the toolkit run `python3 scripts/port_modernize.py check WORKSPACE`.
If it reports awaiting_full_qa, stop and leave the pilot unstarted. Do not reset,
finish, fabricate or weaken QA records to proceed. Verify that the full QA report,
scenario/build/ROM/assets/source provenance and impact rechecks actually cover the
current faithful implementation. The administrative audit cannot establish this.
Run applicable implementation checks/audit only after modernization is invoked.
Record their actual commands/results and any unavailable platform/device checks.

Then run `python3 scripts/port_modernize.py baseline WORKSPACE` to freeze hashes
of the current plan/results/report and native source/build configuration. Keep the
faithful implementation reproducible in local version control or an explicit local
source snapshot, with ROM/asset identities and pinned dependencies. The script
records hashes only; it does not make a source backup or certify semantic truth.
Preserve this baseline and original-fidelity expectations. Do not overwrite it
with the modernized implementation. After capture, set status.json to
baseline_recorded with its baseline path and keep pilot_started false until the
explicitly authorized candidate begins. Existing QA records continue to describe the
faithful baseline; they do not automatically certify a changed build.

## Select one bounded improvement

Populate candidates.json only when this workflow is explicitly invoked after the
gate. For each candidate record its ID, files/subsystem, current machinery, proposed
game-level abstraction, concrete clarity/flexibility benefit, source references,
observable rules to preserve, callers/state owners/update ordering, test IDs and
coverage gaps, coupling/risks and why the scope is bounded. Inspect the code before
ranking candidates; no subsystem is presumed safe from its name alone.

Prefer improvements that remove proven hardware bookkeeping or duplication and
make rules easier to understand. A direct drawing call can replace upload machinery
only when layering, coordinates, intentional flashing/palette/raster effects and
relevant timing still agree. Preserve wrapping/signed arithmetic, ordering and
historical quirks where observable behavior depends on them. Collision, movement,
scheduling or sound drivers may be strongly coupled; defer them when the semantic
contract is incomplete. Do not invent a generic engine or emulated RAM/CPU layer.
Renaming numeric fields alone is not proof that the architecture improved.

Choose one candidate within the user's scope. Establish targeted original-fidelity
integration tests and useful unit regressions first; independently verify any
missing expectations against source and the faithful reference. Scenario discovery
uses persistent branching, recorded recipes and validated setup caches. Missing
observations/setup are tooling gaps, not permission to guess game rules.
Record the starting source/build identity and a reversible checkpoint for edits.
Do not mix unrelated candidates or intentional gameplay enhancements into the patch.

## Implement, compare and retain or revert

Replace one bounded piece with the proposed abstraction. Keep public semantic debug
observations stable where practical so tests survive internal layout changes.
Update tests that assert obsolete implementation detail only with source-backed
reasons; preserve their behavioral coverage and original expectations. Never change
an expected value merely to make the new code pass. Intentional divergence requires
a separate explicit user request and documented variant contract, outside this
fidelity-preserving beta iteration; retain unaffected regressions in either case.

Run targeted fresh integration scenarios against the new build, compare relevant
original-game and faithful-baseline observations, and run affected unit/build/static
checks. Regenerate stale checkpoints from retained recipes; never bypass identity
guards. Select broader regressions from actual cross-system dependencies. Changes
to shared timing/state ownership or uncertain impact require the full relevant
integration suite; do not run two whole QA passes automatically after each edit.
A missing durable test remains a coverage gap, not evidence of equivalence.
Inspect relevant graphics/audio as well as structured assertions where affected.

If behavior differs, investigate source, test and implementation rather than
assuming any one is correct. Retain only a demonstrated improvement with passing
required checks. Otherwise revise or revert only the candidate's own edits,
preserving unrelated work, tests/evidence and the explanation. Stop incomplete
when no sound equivalence claim is available; do not broaden the rewrite to escape
a difficult result. Existing source uncertainty remains explicit.

Update changes.json and status.json with baseline/candidate/source/build identities,
actual edits and simplification benefit, test IDs/commands/results/evidence,
source-grounded contract corrections, remaining uncertainty and retained/reverted
outcome. Update architecture/behavior links and user documentation where affected.
Keep scope distinct from runtime performance optimization or feature development.

Default to one candidate, then report and stop; further iterations need requested
scope. Do not mark the whole game modernized or invent a percentage-complete goal.
No automatic commits, publication, deployments or migration of other games are
part of this workflow. A user-requested modernization commit must remain separate
from QA tooling and faithful-port fixes.
