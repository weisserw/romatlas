# Starting a ROM disassembly

## 1. Set up and build the toolkit

Run these commands in the ROMAtlas repository on Ubuntu/Debian:

```sh
cd /root/romatlas
bash scripts/setup.sh
export PATH="$HOME/.cargo/bin:$PATH"
cargo build --locked --release
```

Run setup as your normal user with sudo access, or directly as root. It elevates
only system installation; do not wrap the whole script in sudo with an existing
user Rust installation. It installs pinned tools, Rust 1.90.0 and SDL2 development
libraries for native ports.
Setup is generally needed once. Rebuild after toolkit changes; neither step
needs repeating for every game.

## 2. Create a game workspace

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

## 3. Start the disassembly agent

Read the generated `goal.md` in the game workspace. It coordinates three single-pass
prompts in `prompts/`: `disassembly.txt`, `commentary.txt`, and `redirect.txt`.
The old printed provider-specific goal startup/resume workflow is deprecated.

Disassembly uses static analysis, normal gameplay driven with debugger inputs,
and bounded verification of supplied leads. It progresses through ordinary game
scenarios while they yield substantial new evidence or usable understanding, without
requiring an existing lead or restricting play to startup/basic inputs. It immediately stops **incomplete** when
no obvious substantial gain remains, even if difficult investigations are possible.
The full completion contract still requires evidence-backed classification of every
byte, annotations, valid maps and byte-identical rebuilding.

Commentary incrementally explains new material and affected dependencies, accepting
pending review. Significant hypotheses use ordinary tentative prose; no graded
uncertainty markers are required. Separate phase workers use compact handoffs.
Redirect searches commented source once for all available leads, using bounded debugger
experiments and memory/register edits where useful. Artificial states are recorded as
such, never as proof of natural reachability. `analysis/leads.json` preserves stable
lead IDs, evidence, verification recipes, statuses and attempt history. Later passes
must add information before reissuing a lead.

The coordinator repeats only for new or materially improved actionable leads. Without
those it stops incomplete and reports unknown counts and verified artifacts. Prior
commentary is preserved between rounds; generated source and assets stay local.

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

Read `goal.md` again in the same workspace. Resume the phase and round recorded in
`analysis/progress.md`; preserve state, commentary and lead history.

The coordinator preserves and updates `analysis/toolkit-issues.md`, merging
repeated findings and retaining evidence from earlier passes. These instructions
and the initial report are generated for new workspaces; previously created
workspaces are not updated automatically.

Keep ROMs, generated assembly, extracted assets, and traces local. The exported
stock `.sym` map is the shareable deliverable. A verified raw-byte baseline is
the starting point; it does not mean code/data classification is complete.

## 5. Prepare the commentary phase in the existing workspace

After structural analysis, prepare a separate editable assembly tree:

```sh
python3 scripts/commentary.py prepare /root/rom-work/game
```

This uses the existing `workspace.json`, `analysis/state.json` and
`maps/current.sym`. It validates ROM identity and both maps, generates a fresh
listing from the current `.sym`, and verifies a byte-identical rebuild. It then
creates `commentary/` without modifying phase-1 instructions, maps or evidence.
An investigated phase-1 unknown interval does not prevent starting this phase.
Preparation refuses an existing commentary directory; it never replaces edits.

The command points to `prompts/commentary.txt` (older workspaces use
`commentary/prompt.md`). Intermediate commentary reviews newly classified/changed
material and earlier interpretations affected by it, following dependencies as
needed. The first pass prioritizes useful known code/data and references into
unknown regions. Preserve valid prior review credit across rounds; changed boundaries
or contradictory evidence invalidate affected records, not the entire inventory.

Use descriptive symbols and the best coherent static interpretation. Write clear,
light prose: short functional comments with useful detail, avoiding compressed
address-heavy shorthand and mechanical opcode narration.
Runtime proof or original-author certainty is not required for semantic names.
Use ordinary tentative prose for significant hypotheses; graded question marks are
not required. Keep numeric identifiers where roles remain unknown, revise incorrect
interpretations, and replace redundant comments rather than stacking explanations.
Structural classification still requires its original evidence standards.

Stop when the affected scope is addressed, useful findings can feed redirect, or
further interpretation has diminishing returns. A verified partial handoff lists
scope, changed interpretations, unresolved questions and deferred work. Whole-ROM
review and complete semantic resolution are not intermediate pass requirements.
Final acceptance addresses pending review and known editorial issues, then runs
the strict audit before editorial completion or archive acceptance. Reuse valid
reviews; there is no mandatory whole-source reread or cosmetic rewrite. Revisit
reviewed material only for changed evidence, dependencies or concrete inconsistencies.

The coordinator launches sequential separate phase workers with compact handoffs,
checkpoint identities and only the relevant phase prompt. Workers consult durable
records as needed; logs do not all need to occupy coordinator context. If separate
contexts are unavailable, hand off to a separate session and report orchestration
pending. Do not run concurrent phase writers.

The files to read and compare are:

- `commentary/reading-before.asm`: frozen consolidated baseline from the current map.
- `commentary/source/game.asm` and its includes: authoritative editable, buildable source.
- `commentary/reading-after.asm`: consolidated reading copy of that source.
- `commentary/review.json`: full-ROM scaffold and agent-maintained semantic review inventory.
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

`instructions` prints the prompt location for a later session. `verify`
rebuilds the edited source and checks exact equality without `rgbfix` header
changes. `read` refreshes the after copy. Verify coherent editing batches and phase handoffs;
refresh the reading copy at handoff. Avoid tiny-edit rebuilds and repeated expected
completion-audit failures. `audit` is reserved for final cleanup/archive acceptance;
it checks the review inventory,
requires final editorial notes, performs fresh verification and refreshes the
reading copy. It checks coverage/status, not comment quality; the agent remains
responsible for useful and coherent explanations. Reviewed ambiguity can satisfy
phase 2 without declaring the phase-1 classification goal complete.

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
unknowns without claiming that editorial completion establishes classification.
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
codex -C /root/rom-ports/game
```

Ask the agent to read `goal.md` to start or resume the port. Preparation validates/restores the archived source, creates a separate
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
