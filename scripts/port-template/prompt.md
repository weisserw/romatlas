# Stage 4: implement the whole native game

Read workflow.md, prompts/common.md, architecture.md, behaviors.md, debug/api.md,
the behavior tests and their adjacent source traces. Implement the actual game
logic in the prepared files until all unit tests pass. Read the relevant original
source while coding; contracts and planning do not replace it. Preserve work;
do not reinitialize. A scaffold, first level or playable demo is not completion.

__CODING_STANDARDS__

## Implement the contracts and maintain the design

Implement all gameplay, actors, title/menu logic, display, sound, scene transitions,
asset preparation and other inventoried behavior. Keep modules cohesive, ownership
explicit and interfaces small. Add files where they improve logical organization.
Use modern native algorithms while preserving source-backed rules, cross-call
side effects and ordering captured by contracts. All logic must run headlessly
and expose the semantic state/events needed by unit tests and later QA through
the debug API. Extend debug/api.md and observations as systems become real.

When a test or plan contradicts source, reread the source/callers/data, document
the evidence and correct the affected contract in place. Retain trace/history
and area IDs. Do not weaken valid expectations, skip tests or substitute stubs
just to obtain green results. Add contracts for newly discovered required areas
before implementing them; reconcile the inventory without restarting the pipeline.

After substantial features inspect affected boundaries, fixing concrete coupling,
duplication and unclear algorithms before building on them. Update architecture.md
when decisions change. Finish with one bounded maintainability pass for semantic
numeric literals, bare addresses and missing/stale explanations; do not repeatedly
redesign a coherent implementation.

## Implementation and behavior accounting

Use ledger.json to account for all archived regions and meaningful units as
implemented, merged, replaced or excluded, with a behavior explanation and native
equivalents. Add discovered units while preserving original IDs and ranges.
Implement every gameplay-relevant equivalent in the whole game; modern methods may merge routines
or remove original mechanisms. Judge obvious irrelevant junk/unreachable paths
from source without a formal reachability proof. Favor implementing borderline
or unclear behavior. Lack of observed execution alone does not justify exclusion.

Incomplete disassembly/classification is allowed. Assess unknown ranges and
semantic ambiguities by their consumers and port impact as implementation reaches
them. Do not complete the whole-ROM mapping task as a port prerequisite. Prefer
source reading, data inspection and existing documentation. Where a reasonable
source-informed implementation is possible, implement it, record the assumption,
basis and residual uncertainty, and continue. Distinguish an implementation
assumption from an established fact about the original. Do not demand proof of
every interpretation or runtime confirmation of every branch.

For uncertain data, keep original values ROM-backed when a supported consumer can
interpret them; unit-test loading/bounds where useful. Preserving code bytes is
not native behavior. Do not silently skip handlers, zero unknown fields or label
behavior irrelevant to finish accounting. Missing required implementations remain
incomplete; implemented best-effort behavior with documented assumptions does not
remain blocked merely because fidelity has not been independently confirmed.
Use prompts/reference-updates.md for supported source corrections; its map and
byte-identical rebuild requirements protect reference integrity only. Never publish
an assumption as confirmed classification. Frozen reference/ stays unchanged.

## Checks and handoff

Run python3 run.py check for formatting, clippy, unit tests and desktop/Wasm
compilation. All behavior contracts must pass and all required behavior must be
implemented; green tests alone do not excuse missing game systems. Follow
prompts/common.md for bounded source-question experiments and relevant toolkit
regressions after library changes. Do not execute an additional gameplay smoke
route or full fidelity pass here; stage 5 owns runtime QA.

Complete behaviors.md links, ledger/uncertainty assessment (including implemented
assumptions), README.md controls/running/asset replacement/added-level/game-rule
instructions, BUILDING.md build and actual check results, differences.md and
library-changes.md. Provide an original-content-only extension manifest. Document
python3 run.py web, original battery saving and untested device/platform behavior.

Set ledger completion.status=complete and run python3 run.py audit; after it
passes, stop this subprocess and hand off to QA. Record changed systems, contract
results, debug schema and reproducible QA entry recipes in progress.md. Overall
completion still requires the short smoke test and initial exact-frame comparisons. Do not mark missing
behavior implemented. Leave shared-library edits uncommitted for separate review.
No check-in, publication or agent launch is authorized by this implementation
prompt. Release packaging and dependency pinning remain later work.
