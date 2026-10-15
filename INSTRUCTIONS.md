# Starting a ROM disassembly

## Agent context management (all phases)

Recommended setup: disable context-mode in your agent's plugin settings and use
the context-saving guidance embedded in the generated prompts below. Preparing
a workspace does not change plugin settings. Disassembly, commentary and native
port prompts require no context-management plugin.

Process large traces, JSON, source inventories and test logs with local scripts
or focused searches; return counts, relevant excerpts, failures and evidence
paths instead of dumping complete files. Use bounded/paginated tool queries.
Inspect exact source, bytes and bank context before accepting a conclusion;
summaries are navigation aids, not proof. Keep full evidence on disk and concise
progress/next-action records for resumption. Read relevant older sections on
demand. Use direct tools for short reads and edits. Avoid repeated unchanged
queries, redundant reports and scripts for trivial one-line reads. Context saving
must never reduce investigation scope, verification or honest uncertainty.

## 1. Set up and build the toolkit

Run these commands in the ROMAtlas repository on Ubuntu/Debian:

```sh
cd /root/romatlas
bash scripts/setup.sh
export PATH="$HOME/.cargo/bin:$PATH"
cargo build --locked
```

Run setup as your normal user with sudo access, or directly as root. It elevates
only system installation; do not wrap the whole script in sudo with an existing
user Rust installation. It installs pinned tools, Rust 1.90.0 and SDL2 development
libraries for native ports.
Setup is generally needed once. Rebuild after toolkit changes; neither step
needs repeating for every game.

## 2. Create a game workspace

Build the optimized toolkit used by disassembly workspaces:
`cargo build --locked --release`. The initializer defaults to that release binary;
`--binary /path/to/romatlas` is an explicit override.

The default workspace parent is the current directory. Run from a directory
outside the toolkit repository, or pass `--root` explicitly as below, with the
path to your locally owned ROM:

```sh
python3 scripts/disassemble.py /path/to/game.gb --root /root/rom-work
```

This creates `/root/rom-work/game`, copies the ROM into it, and writes agent
instructions, start/resume prompts, and workflow commands. Choose a workspace
parent writable by your user, such as `--root "$HOME/rom-work"`.

You can prepare multiple games at once:

```sh
python3 scripts/disassemble.py /path/to/game1.gb /path/to/game2.gbc --root /root/rom-work
```

Each ROM gets its own directory named after its filename without the extension.
Names must be distinct, and existing workspaces are not overwritten.

For an existing workspace, use
`python3 scripts/upgrade-disassembly.py /path/to/workspace --apply` to install the
current prompts, cached session client and queue/report helper in place. Stop its
RPC bridge first. The upgrade backs up configuration and prompts, verifies the
ROM identity and preserves the classification state, maps and evidence history.
Read [workspace efficiency](docs/workspace-efficiency.md) for commands and rollback.
`prompts/goal.md` contains the updated objective. File upgrades do not themselves
edit an agent provider's stored goal, change its model, or resume a stopped agent.

## 3. Start the disassembly agent

Follow the startup instructions printed by the command. For example, run this
in your terminal:

```sh
codex --enable goals -C /root/rom-work/game
```

Once Codex opens, paste the printed `/goal ...` command into Codex's input and
press Enter. The goal tells the agent to read the workspace instructions,
identify code and data, export a mapping, and generate an assembly listing
that rebuilds byte-for-byte. Use a Codex version with goal support.

The objective is an evidence-backed account of released-game behavior and supporting
assets, with a portable map and byte-identical disassembly. Maintain a versioned
`analysis/behavior.md` inventory derived from research, static analysis and runtime
observations. Cover modes, mechanics, progression, terminal states, implementation
entry points, dispatch and loaders, evidence, validation and unresolved dependencies.
There is no classification-percentage target. Preserve unexplained bytes verbatim;
raw fidelity alone does not establish behavioral completion.

Triage every unknown interval for coherent code, references, banking, consumers,
formats and allocation boundaries. Prioritize behavioral dependencies and structural
families. Plausible disassembly is an investigation lead, not proof. Runtime must
answer a named reconstruction question; advancing through gameplay alone is not
progress. Preserve results and reopen conditions in `analysis/investigation.md`.

After identifying the game and likely version, the agent consults available online
manuals, walkthroughs, FAQs and technical resources. It keeps source links, version
uncertainty, controls, mechanics and actionable progression/password routes in
`analysis/gameplay.md`, translating useful findings into targeted runtime scenarios.
External claims need confirmation against the local ROM and do not independently
prove structural classifications. Research is revisited for specific obstacles,
including before exhausting gameplay-dependent paths. If resources are unavailable,
the agent records that limitation and continues local analysis.

Success requires every inventory item accounted for, relevant control/asset
uncertainties resolved or evidence-bounded, annotated accepted regions, validated
maps and a final byte-identical rebuild. Unknown intervals may remain only after
recorded triage shows no unresolved behavioral dependency, unexamined credible
hypothesis or concrete follow-up resolving a reconstruction question. Lack of
observed access is insufficient. Do not claim unexplained regions are proved unused.

Continue concrete actionable investigations. Stop **incomplete, blocked** when
required questions have only blocked investigations, or **incomplete, investigation
exhausted** when applicable methods leave no distinguishing experiment. Audit the
inventory and all unknown dispositions before stopping; report unmet conditions,
reopen triggers and a verified checkpoint where possible. Neither incomplete outcome
is success. Do not repeat unchanged exhausted work on resume. Provider goal-status
constraints still apply; file upgrades cannot change provider-stored goals.

Run the disassembly agent in the game workspace. Keep `/root/romatlas` for
building or developing the toolkit. Initialization prepares the workspace;
it does not launch Codex or perform the disassembly itself.

The agent keeps a separate report at `analysis/toolkit-issues.md` as it encounters
confirmed bugs, suspected bugs, or missing toolkit features. Each finding includes
the affected tool, evidence or reproduction steps, impact, any workaround, and a
suggested improvement. Feature requests must cite a concrete analysis example and
explain a substantial improvement to reverse engineering speed or inference quality.
Documented limitations are distinguished from bugs. The agent continues analysis
unless blocked and leaves toolkit fixes for later.

At the end of each pass, including a blocked handoff, the agent summarizes findings
and links the report, or explicitly says "no toolkit issues found" when empty.

## 4. Continue later

Open Codex in the same game workspace and use the printed resume goal, which
refers to `prompts/resume.md`. The agent loads saved analysis and continues
classification. In an existing Codex session, `/goal` shows goal status,
`/goal pause` pauses it, and `/goal resume` resumes a paused goal.

Keep an existing unfinished goal rather than replacing it with a checkpoint-sized
goal. The resume prompt pursues the same completion contract and preserves the
investigation ledger, reopening entries affected by new evidence.

The resume prompt preserves and updates `analysis/toolkit-issues.md`, merging
repeated findings and retaining evidence from earlier passes. These instructions
and the initial report are generated for new workspaces; previously created
workspaces are not updated automatically.

Keep ROMs, generated assembly, extracted assets, and traces local. The exported
stock `.sym` map is the shareable deliverable. A verified raw-byte baseline is
the starting point; it does not mean code/data classification is complete.

## 5. Prepare the commentary phase in the existing workspace

During classification, a stalled structural dependency can trigger targeted
subsystem explanation. Preserve names, behavior, invariants, bank-aware ranges
and supporting evidence in existing investigation/metadata records for reuse.
This makes the existing permission to investigate semantics actionable; it adds
no phase and does not require a whole-game commentary sweep during classification.

When ready for the full commentary review, prepare a separate editable assembly tree:

```sh
python3 scripts/commentary.py prepare /root/rom-work/game
```

This uses the existing `workspace.json`, `analysis/state.json` and
`maps/current.sym`. It validates ROM identity and both maps, generates a fresh
listing from the current `.sym`, and verifies a byte-identical rebuild. It then
creates `commentary/` without modifying phase-1 instructions, maps or evidence.
Unknown intervals and incomplete upstream accounting do not prevent preparation;
their behavioral obligations remain pending until resolved in the later phase.
Preparation refuses an existing commentary directory; it never replaces edits.

Commentary findings can feed back into classification. Its prompt requires an
evidence-backed correction record, a single writer for shared maps/state, a
snapshot and validated map update, and a fresh classification checkpoint.
Reconcile affected edited source and review units while preserving comments,
then verify commentary again. Frozen commentary baseline artifacts must remain
unchanged; never regenerate over edited source. Track blocked reconciliation
explicitly. Structural and editorial completion remain separate outcomes.

The command prints a Codex startup command and a short `/goal` referring to
`commentary/prompt.md`. That prompt asks the agent to review the entire game,
identify routines, loops, shared tails and data structures, introduce meaningful
symbols, and explain functionality using conventional readable assembly style.
Comment detail scales with complexity; opcode translations and boilerplate
headers are discouraged. Static interpretation is the default, tentative meanings
use `(?)`, and debugging targets genuinely unclear behavior. Completion requires
behavior-linked semantic review, evidence-backed preservation treatment for opaque
regions, resolved or bounded dependencies, reconciled structural refinements, a
final editorial pass and a fresh byte-identical rebuild. It does not require
certainty about the historical purpose of every byte.

The files to read and compare are:

- `commentary/reading-before.asm`: frozen consolidated baseline from the current map.
- `commentary/source/game.asm` and its includes: authoritative editable, buildable source.
- `commentary/reading-after.asm`: consolidated reading copy of that source.
- `commentary/review.json`: full-ROM preservation scaffold with semantic/preserved treatments.
- `commentary/handoff.json`: frozen upstream behavioral claims, outcome, notes and dispositions.
- `commentary/behavior.json`: current behavioral review, source links and unresolved obligations.
- `commentary/refinements.json` and `refinements.md`: later structural findings and reconciliation.
- `commentary/progress.md` and `commentary/uncertainties.md`: progress and useful unresolved interpretations.

The reading copies expand `.asm` includes in source order, mark file boundaries,
and retain hardware/constants includes and binary-asset references. They are
reading aids, not independent build targets. Preserve the source tree and assets
until archiving it in step 6: **the `.sym` alone cannot reproduce comments**.
Generation does not render phase-1 role/evidence metadata as assembly comments.
Never regenerate over the commentary source tree.

From the toolkit repository, use:

```sh
python3 scripts/commentary.py instructions /root/rom-work/game
python3 scripts/commentary.py verify /root/rom-work/game
python3 scripts/commentary.py read /root/rom-work/game
python3 scripts/commentary.py audit /root/rom-work/game
```

`instructions` prints the startup/goal text again for a later session. `verify`
rebuilds the edited source and checks exact equality without `rgbfix` header
changes. `read` refreshes the after copy. `audit` checks the review inventory,
requires final editorial notes, inherited behavioral accounting, source/region links
and reconciled refinements, then verifies and refreshes the reading copy. It checks coverage/status, not comment quality; the agent remains
responsible for useful and coherent explanations. Reviewed ambiguity can satisfy
phase 2 when it leaves no unresolved behavioral dependency. Later static analysis
may close inherited questions; upstream recorded outcomes remain historical. See
[behavioral accounting](docs/behavioral-accounting.md) for schemas, all-phase endings
and backed-up `commentary.py upgrade` / `port.py upgrade` migrations.

All assembly, ROMs, extracted assets and reading copies remain local. Do not
commit or distribute a commercial game's generated workspace.

## 6. Archive the reviewed result

From this repository, archive a completed commentary workspace:

```sh
python3 scripts/archive.py archive /root/rom-work/game --name game
```

The default destination is `atlas/gb/game/<full-rom-sha256>/`. Use a lowercase,
hyphenated game name; without `--name`, the workspace directory name is used.
`--output DIRECTORY` selects an explicit archive directory. An existing archive
is replaced in full only after the new result passes validation, exact-source
restoration and byte-identical rebuilding. Failure preserves the previous archive.
Directories without `archive.json` are never overwritten.

The command requires a fresh successful commentary audit. It preserves phase-one
unknowns, behavioral inventories, durable dispositions and refinement records.
Opaque preservation is not a claim of nonuse or exhaustive classification.
The archive contains the final portable map, internal analysis/annotations,
review inventory, final audits, and both phases' final documentation, including
uncertainties. Additional final Markdown documents require
`--document commentary/example.md` (repeatable); unrecognized documents are
reported rather than silently omitted. Use `--omit-document analysis/scratch.md`
to explicitly exclude an unrecognized scratch document. Scratch results, traces, intermediate
checkpoints, agent prompts and `analysis/toolkit-issues.md` are excluded.
Historical documents remain verbatim; references to omitted scratch evidence
are explicitly described as unavailable in the archive README.

`editorial.json` is a versioned reconstruction recipe holding comments, symbol
edits, constant definitions and formatting, with references into a regenerated
baseline. It preserves the final commentary separately from `.sym`. The archive
contains no generated assembly, ROM, extracted graphics or build products.
Baseline hashes detect changes in generator/tool output, and every final source
and asset has an expected hash. Changed/new binary assets are rejected rather
than included. New quoted data literals that cannot reference baseline content
are also rejected. Use the pinned tools installed by `scripts/setup.sh`.

## 7. Restore readable assembly from an archive and the original ROM

```sh
python3 scripts/archive.py restore atlas/gb/game/FULL_ROM_SHA256 \
  --rom /path/to/original.gb --out /tmp/game-restored
```

The output must be a new directory outside this repository. It contains the exact
buildable final source tree in `source/` and its consolidated `reading-after.asm`.
The command checks archive hashes and the ROM's SHA-256, regenerates the frozen
baseline map, applies editorial metadata, checks every restored file hash, and
rebuilds byte-identically before publishing any output. The original filename
used by the workflow is reproduced internally, so your supplied ROM may have a
different filename. `--binary /path/to/romatlas` selects an alternate built binary.
Success returns JSON with `ok`, `exact_source` and `byte_identical` all true;
errors return JSON on stderr and exit 1. The original workspace is unnecessary.

Keep restored source, reading copies, ROMs and assets local. Test the archive
lifecycle with `python3 scripts/test-archive.py`, which uses a synthetic ROM only.

## 8. Prepare the native port phase

```sh
python3 scripts/port.py prepare atlas/gb/game/FULL_ROM_SHA256 \
  --rom /path/to/original.gb --out /root/rom-ports/game
codex --enable goals -C /root/rom-ports/game
```

Paste the printed whole-game port goal. Use the printed resume goal for later
sessions. Preparation validates/restores the archived source, creates a separate
Rust project and behavior ledger, and records the toolkit's initial files for
review. It implements no gameplay and launches no agent. Phase-one unknowns are
allowed, but gameplay-relevant gaps must be investigated by the port agent.

The agent implements every gameplay-relevant equivalent, accounting for merged,
modernized and obviously irrelevant original paths rather than translating every
line. Original assets load from the user's ROM at startup. File-backed replacements
and new slots allow original levels plus added levels. The shared library includes
console timing, selected Game Boy effects, assets, headless helpers and an optional
SDL2 shell with persisted keyboard/controller/display/audio settings.

```sh
cd /root/rom-ports/game
python3 run.py headless --frames 120
python3 run.py check
python3 run.py audit
```

check runs formatting, clippy, headless tests and desktop compilation. Do not run
`python3 run.py desktop` on this headless server; it is for a graphical desktop.
F1 or controller Guide opens the settings menu. Do not write tests requiring
unavailable graphics/devices. Desktop/other-platform runtime behavior remains
explicitly untested here. Audit checks explained source accounting and builds,
not gameplay fidelity or semantic correctness.

The agent may implement needed library changes directly in ROMAtlas, uncommitted,
with a log and relevant toolkit checks. Launch another agent manually from the port
workspace and give it `prompts/review.md` to review the actual diff against the
initial toolkit snapshot. You decide check-in. No release packaging or library
publication is performed; the development port uses an editable local dependency.
See [docs/porting.md](docs/porting.md) for the full contract and extension formats.
