# Port the entire archived game

Read AGENTS.md, workflow.md, uncertainties.json, prompts/reference-updates.md,
reference/reading.asm and relevant archive documents
before coding. The commented, labeled reference is intended to explain routines,
branches, RAM variables and data formats. If it contains gaps, investigate and record
them rather than assuming classification or editorial status proves understanding.
Implement this entire game in readable native Rust. The scaffold
is an arbitrary ROM-backed tile demonstration, not gameplay or identified assets.
Replace it; do not stop at a working scaffold, first level or playable demo.

Use ledger.json to account for every archived region and meaningful unit. Record
implemented, merged, replaced or excluded, a behavior explanation, and port
equivalents. Add units as you discover them. Every gameplay-relevant behavior
needs an equivalent; source lines need not translate one-for-one. Modern methods
may merge routines or remove the need for an original mechanism. Judge obviously
irrelevant junk/unreachable paths from the commented source without a formal
reachability proof. Favor porting borderline/unclear behavior. Resolve relevant
unknown regions; editorial review does not prove complete gameplay understanding.

The disassembly may be unfinished, and classified source may still have uncertain
meanings or incorrect commentary. Begin useful porting without completing whole-ROM
classification first. Small gaps are not automatically harmless, and an Unknown
label does not mean the bytes are absent: the reference preserves them exactly.
Assess each seeded uncertainties.json entry as its consumers become relevant;
record CPU/bank context, callers/readers, competing interpretations, port impact,
evidence, useful tests and validation limits. Add uncertainties in classified
regions as you find them. Prioritize control flow, dispatch, pointer/bank fields,
record bounds and rare state transitions over storage-only questions. Continue
independent implementation while an investigation is pending; investigate deeply
when a concrete port behavior depends on it, rather than restoring an unlimited
whole-ROM classification task.

For uncertain data, keep original values ROM-backed where a supported consumer
can interpret them; document/test loading, bounds and affected behavior. Preserved
data can remain structurally uncertain. Raw instruction bytes are not a native
implementation: resolve needed logic, implement its equivalent and test meaningful
branches. Do not silently zero unknown fields, skip handlers, invent semantics or
label fragments irrelevant merely because no trace selected them. An explained
no-impact exclusion is allowed under the existing behavior-accounting contract.
Unresolved gameplay-affecting behavior remains incomplete, even if Rust builds.

When implementation reveals a supported correction or new interpretation, follow
prompts/reference-updates.md to update the local working disassembly/commentary,
validate a new reference revision and link it to uncertainties.json and ledger.json.
Preserve frozen reference/ and original ledger identities; add discovered units
instead of replacing historical records. On resume inspect research/current.json
and its verified reading/source supplement before trusting older commentary.

All original graphics, fonts, music, sound and gameplay data (including maps)
come from the matching user's ROM at startup through recipes. Do not bake ROM
bytes or decoded originals into Rust constants or distributable files. Support
validated file overrides and added named slots: original ROM-backed levels plus
additional file-backed levels must work together without a fixed level count.
Keep decoded originals and regenerated assembly local. Never download commercial
ROMs or commit ROMs/extracted assets. Developer-authored content may ship.

Use romatlas-port for common mechanisms and platform-specific effects. Keep game
rules and unusual tricks in the game when appropriate. Simulate at the original
console cadence independently of monitor refresh. Exact frame compatibility is
not promised; document known differences and aim for accurate gameplay.

You are authorized to implement necessary library changes directly in the toolkit
directory recorded in workspace.json. Read its AGENTS.md, project.md and
docs/progress.md. Leave edits uncommitted, preserve unrelated work, and distinguish
the initial state recorded in review/toolkit-before.json and review/toolkit-before/.
For each shared change log the game requirement, files, game-independent interface
rationale and tested evidence in library-changes.md. Two consumers are not required
to justify a reusable utility. Keep uncertain abstractions local. Complete needed
library work during porting; do not defer it merely by filing an issue. Record
unresolved findings and workarounds as well.

Supply persisted keyboard/gamepad mapping, controller selection, dead zones,
window/fullscreen/scaling and volume settings through a usable in-game menu.
Share simulation between desktop play and scripted headless tests. Headless mode
must initialize neither a window nor an audio device. Write a normal useful amount
of tests that run in this headless Linux environment. Do not write tests needing
unavailable displays, GPUs or physical controllers. Report untested desktop and
other-platform behavior. Missing desktop runtime validation alone does not block
completion. Compile the desktop target; do not launch it on this server.

Run python3 run.py check for formatting, clippy, tests and desktop compilation.
After toolkit changes run its required relevant checks, including portability and
dependency boundaries, and update its docs/progress.md with tested facts. Never
interpret tests or the ledger audit as proof of semantic correctness.

Update README.md with how to run the port, replace assets, add a level and find
game rules. Adapt the generated BUILDING.md to the finished port and link it from
README.md. Include macOS/Linux/Windows prerequisites and build/run commands,
source transfer excluding target/ binaries, the exact shared-library source and
lockfiles required, path-dependency relocation, and game-specific dependencies or
flags. Record actual validation separately from untested build recipes; never
copy another port's platform success as evidence for this one. Check upstream
platform setup when needed. Keep packaging/publication as later work.
Provide an original-content-only example extension manifest. Keep
progress.md and differences.md current. Completion requires a finished ledger,
assessed uncertainties with evidence and explicit residual limits,
implemented gameplay equivalents, useful passing tests and a documented handoff.
Set ledger completion.status=complete with notes only when this is true, and run
python3 run.py audit. Audit validates records and builds, not gameplay fidelity.
Leave shared-library edits for the separate reviewer; provide the actual diff,
requirements and validation. No check-in, publication or agent launch is authorized.

If blocked, preserve work, identify unresolved behaviors and concrete attempts,
and explain what would unblock them. Do not mark an incomplete port complete.
Release packaging and immutable library dependency pinning happen later, after
user review and check-in. A local path dependency and Cargo.lock do not freeze
the edited library source.
