# Disassembly, archives and native ports

## 1. Set up and build the toolkit

Run these commands in the ROMAtlas repository on Ubuntu/Debian:

```sh
cd /root/romatlas
bash scripts/setup.sh
export PATH="$HOME/.cargo/bin:$PATH"
cargo build --locked --release
```

Run setup as your normal user with sudo access, or directly as root. It elevates only
system installation; do not wrap the whole script in sudo with an existing user Rust
installation. It installs pinned tools, Rust 1.90.0 and SDL2 development libraries for
native ports. Setup is generally needed once. Rebuild after toolkit changes; neither
step needs repeating for every game.

## 2. Create a game workspace

The default workspace parent is the current directory. Run from a directory outside the
toolkit repository, or pass `--root` explicitly as below, with the path to your locally
owned ROM:

```sh
python3 scripts/disassemble.py /path/to/game.gb --root /root/rom-work
```

The example uses Game Boy. Game Boy Color, NES, SNES, Master System, Game Gear, Genesis
and PC Engine HuCard ROMs use the same preparation command. The platform is detected
from the extension; `--platform` selects it explicitly. See the [platform
guides](docs/development.md#platform-and-api-references) for supported images, tools and
hardware limits.

This creates `/root/rom-work/game`, copies the ROM into it, and writes agent
instructions, start/resume prompts, and workflow commands. Choose a workspace parent
writable by your user, such as `--root "$HOME/rom-work"`.

You can prepare multiple games at once:

```sh
python3 scripts/disassemble.py /path/to/game1.gb /path/to/game2.gbc --root /root/rom-work
```

Each ROM gets its own directory named after its filename without the extension. Names
must be distinct, and existing workspaces are not overwritten.

## 3. Start the disassembly agent

Read the generated `goal.md` in the game workspace. It coordinates three single-pass
prompts in `prompts/`: `disassembly.txt`, `commentary.txt`, and `redirect.txt`. The old
printed provider-specific goal startup/resume workflow is deprecated.

Disassembly uses static analysis, normal gameplay driven with debugger inputs, and
bounded verification of supplied leads. It progresses through ordinary game scenarios
while they yield substantial new evidence or usable understanding, without requiring an
existing lead or restricting play to startup/basic inputs. It immediately stops
**incomplete** when no obvious substantial gain remains, even if difficult
investigations are possible. The full completion contract still requires evidence-backed
classification of every byte, annotations, valid maps and byte-identical rebuilding.

Every commentary invocation completes naming and comments across all currently
classified source, including older gaps, while preserving sound prior work. Whole-ROM
editorial acceptance remains a separate final gate. Significant hypotheses use ordinary
tentative prose; no graded uncertainty markers are required. Separate phase workers use
compact handoffs. Redirect searches commented source once for all available leads, using
bounded debugger experiments and memory/register edits where useful. Artificial states
are recorded as such, never as proof of natural reachability. `analysis/leads.json`
preserves stable lead IDs, evidence, verification recipes, statuses and attempt history.
Later passes must add information before reissuing a lead.

The coordinator repeats only for new or materially improved actionable leads. Without
those it stops incomplete and reports unknown counts and verified artifacts. Prior
commentary is preserved between rounds; generated source and assets stay local.

Run the disassembly agent in the game workspace. Keep `/root/romatlas` for building or
developing the toolkit. Initialization prepares the workspace; it does not launch Codex
or perform the disassembly itself.

The agent keeps a separate report at `analysis/toolkit-issues.md` as it encounters
confirmed bugs, suspected bugs, or missing toolkit features. Each finding includes the
affected tool, evidence or reproduction steps, impact, any workaround, and a suggested
improvement. Feature requests must cite a concrete analysis example and explain a
substantial improvement to reverse engineering speed or inference quality. Documented
limitations are distinguished from bugs. The agent continues analysis unless blocked and
leaves toolkit fixes for later.

At the end of each pass, including a blocked handoff, the agent summarizes findings and
links the report, or explicitly says "no toolkit issues found" when empty.

## 4. Continue later

Read `goal.md` again in the same workspace. Resume the phase and round recorded in
`analysis/progress.md`; preserve state, commentary and lead history.

The coordinator preserves and updates `analysis/toolkit-issues.md`, merging repeated
findings and retaining evidence from earlier passes. These instructions and the initial
report are generated for new workspaces; previously created workspaces are not updated
automatically.

Keep ROMs, generated assembly, extracted assets, and traces local. The exported portable
map is shareable: stock mgbdis `.sym` for Game Boy/Color, platform-specific ROM-free
JSON for all other supported platforms. A verified raw-byte baseline is the starting
point; it does not mean code/data classification is complete.

## 5. Prepare the commentary phase in the existing workspace

After structural analysis, prepare a separate editable assembly tree:

```sh
python3 scripts/commentary.py prepare /root/rom-work/game
```

This uses the existing `workspace.json`, `analysis/state.json` and the current portable
map (`maps/current.sym` for Game Boy/Color, `maps/current.json` otherwise). It validates
ROM identity and the analysis/map, generates a fresh listing from the current map, and
verifies a byte-identical rebuild. It then creates `commentary/` without modifying
phase-1 instructions, maps or evidence. An investigated phase-1 unknown interval does
not prevent starting this phase. Preparation refuses an existing commentary directory;
it never replaces edits.

The command points to `prompts/commentary.txt` (older workspaces use
`commentary/prompt.md`). Each commentary invocation reviews all currently classified
source, following dependencies as needed and catching up earlier readability gaps. The
first pass prioritizes useful known code/data and references into unknown regions.
Preserve valid prior review credit across rounds; changed boundaries or contradictory
evidence invalidate affected records, not the entire inventory.

Use descriptive symbols and the best coherent static interpretation. Write clear, light
prose: short functional comments with useful detail, avoiding compressed address-heavy
shorthand and mechanical opcode narration. Runtime proof or original-author certainty is
not required for semantic names. Use ordinary tentative prose for significant
hypotheses; graded question marks are not required. Keep numeric identifiers where roles
remain unknown, revise incorrect interpretations, and replace redundant comments rather
than stacking explanations. Structural classification still requires its original
evidence standards.

Hand off only after current-source readability is complete and a fresh rebuild passes.
An interruption leaves a resumable commentary phase; it does not permit skipping to
redirect. List changed interpretations, unresolved meanings and useful findings for
redirect. Complete semantic certainty is not required. Final acceptance addresses
pending review and known editorial issues, then runs the strict audit before claiming
editorial completion. Archives record whether that completion has been achieved. Reuse
valid reviews; there is no mandatory whole-source reread or cosmetic rewrite. Revisit
reviewed material only for changed evidence, dependencies or concrete inconsistencies.

The coordinator launches sequential separate phase workers with compact handoffs,
checkpoint identities and only the relevant phase prompt. Workers consult durable
records as needed; logs do not all need to occupy coordinator context. If separate
contexts are unavailable, hand off to a separate session and report orchestration
pending. Do not run concurrent phase writers.

The files to read and compare are:

- `commentary/reading-before.asm`: frozen consolidated baseline from the current map.
- `commentary/source/` (Game Boy entrypoint: `game.asm`): authoritative editable, buildable source.
- `commentary/reading-after.asm`: consolidated reading copy of that source.
- `commentary/review.json`: full-ROM scaffold and agent-maintained semantic review inventory.
- `commentary/progress.md` and `commentary/uncertainties.md`: progress and useful unresolved interpretations.

The reading copies expand `.asm` includes in source order, mark file boundaries, and
retain hardware/constants includes and binary-asset references. They are reading aids,
not independent build targets. Preserve the source tree and assets until archiving it in
step 6: **the portable map alone cannot reproduce comments**. Generation does not render
phase-1 role/evidence metadata as assembly comments. Never regenerate over the
commentary source tree.

From the toolkit repository, use:

```sh
python3 scripts/commentary.py instructions /root/rom-work/game
python3 scripts/commentary.py verify /root/rom-work/game
python3 scripts/commentary.py read /root/rom-work/game
python3 scripts/commentary.py audit /root/rom-work/game
```

`instructions` prints the prompt location for a later session. `verify` rebuilds the
edited source and checks exact equality using the platform backend (Game Boy
verification avoids `rgbfix` header changes). `read` refreshes the after copy. Verify
coherent editing batches and phase handoffs; refresh the reading copy at handoff. Avoid
tiny-edit rebuilds and repeated expected completion-audit failures. `audit` is reserved
for final cleanup/archive acceptance; it checks the review inventory, requires final
editorial notes, performs fresh verification and refreshes the reading copy. It checks
coverage/status, not comment quality; the agent remains responsible for useful and
coherent explanations. Reviewed ambiguity can satisfy phase 2 without declaring the
phase-1 classification goal complete.

All assembly, ROMs, extracted assets and reading copies remain local. Do not commit or
distribute a commercial game's generated workspace.

## 6. Archive the reviewed result

From this repository, archive the current commentary workspace:

```sh
python3 scripts/archive.py archive /root/rom-work/game --name game
```

The default destination is `atlas/<platform>/game/<full-rom-sha256>/` (`gb` for Game
Boy/Color). Use a lowercase, hyphenated game name; without `--name`, the workspace
directory name is used. `--output DIRECTORY` selects an explicit archive directory. An
existing archive is replaced in full only after the new result passes validation,
exact-source restoration and byte-identical rebuilding. Failure preserves the previous
archive. Directories without `archive.json` are never overwritten.

The command validates the review inventory and rebuilds the current source. It can
archive incomplete worktrees and records editorial completion explicitly. Final
editorial acceptance requires the strict commentary audit; archived phase-one unknowns
remain unknown regardless of editorial status. The archive contains the final portable
map, internal analysis/annotations, review inventory, final audits, and both phases'
final documentation, including uncertainties. Additional final Markdown documents
require `--document commentary/example.md` (repeatable); unrecognized documents are
reported rather than silently omitted. Use `--omit-document analysis/scratch.md` to
explicitly exclude an unrecognized scratch document. Scratch results, traces,
intermediate checkpoints, agent prompts and `analysis/toolkit-issues.md` are excluded.
Historical documents remain verbatim; references to omitted scratch evidence are
explicitly described as unavailable in the archive README.

`editorial.json` is a versioned reconstruction recipe holding comments, symbol edits,
constant definitions and formatting, with references into a regenerated baseline. It
preserves the final commentary separately from the portable map. The archive contains no
generated assembly, ROM, extracted graphics or build products. Baseline hashes detect
changes in generator/tool output, and every final source and asset has an expected hash.
Changed/new binary assets are rejected rather than included. New quoted data literals
that cannot reference baseline content are also rejected. Use the pinned tools installed
by `scripts/setup.sh`.

## 7. Restore readable assembly from an archive and the original ROM

```sh
python3 scripts/archive.py restore atlas/gb/game/FULL_ROM_SHA256 \
  --rom /path/to/original.gb --out /tmp/game-restored
```

The output must be a new directory outside this repository. It contains the exact
buildable final source tree in `source/` and its consolidated `reading-after.asm`. The
command checks archive hashes and the ROM's SHA-256, regenerates the frozen baseline
map, applies editorial metadata, checks every restored file hash, and rebuilds
byte-identically before publishing any output. The original filename used by the
workflow is reproduced internally, so your supplied ROM may have a different filename.
`--binary /path/to/romatlas` selects an alternate built binary. Success returns JSON
with `ok`, `exact_source` and `byte_identical` all true; errors return JSON on stderr
and exit 1. The original workspace is unnecessary.

Keep restored source, reading copies, ROMs and assets local. Test the archive lifecycle
with `python3 scripts/test-archive.py`, which uses a synthetic ROM only.

## 8. Prepare the native port phase

```sh
python3 scripts/port.py prepare atlas/gb/game/FULL_ROM_SHA256 \
  --rom /path/to/original.gb --out /root/rom-ports/game
codex -C /root/rom-ports/game
```

Ask the agent to read `goal.md` to start or resume the port. Preparation
validates/restores the archived source, creates a separate Rust project and behavior
ledger, and records the toolkit's initial files for review. It implements no gameplay
and launches no agent. Phase-one unknowns are allowed, but gameplay-relevant gaps must
be investigated by the port agent.

Read goal.md to invoke the fixed pipeline of sequential subagents: skeleton,
behavior planning, behavior contract, game implementation, then final QA. The
skeleton establishes identifiers, shared frontends and semantic debug/checkpoint
infrastructure without game/menu/title logic. Planning inventories all behavior
with source references. A separate contract worker for each area independently
traces source and writes unit tests with adjacent evidence before implementation.
Implementation completes the whole game until all contracts/checks pass. Final
whole-game QA executes two fresh passes using reusable scripts and validated
starting states, repairing failures in place without restarting. Preparation
launches no agents and does not migrate existing workspaces.

The agent implements every gameplay-relevant equivalent, accounting for merged,
modernized and obviously irrelevant original paths rather than translating every line.
Original assets load from the user's ROM at startup. File-backed replacements and new
slots allow original levels plus added levels. The shared library includes console
timing, selected platform effects, assets, headless helpers and an optional SDL2 shell
with persisted keyboard/controller/display/audio settings.

```sh
cd /root/rom-ports/game
python3 run.py headless --frames 120
python3 run.py check
python3 run.py audit
```

`check` runs formatting, Clippy, headless tests, desktop compilation and Wasm
lint/build checks. All source-backed contracts must pass before final QA; there
is no separate implementation gameplay smoke route. QA uses debug/scenarios/
scripts for every check, debugging and repair verification, executing both
required passes afresh. Do not run `python3 run.py desktop` on this headless
server; it is for a graphical desktop. F1 or controller Guide opens settings.
Do not write tests requiring unavailable devices. Desktop/other-platform runtime
behavior remains explicitly untested here. Audit checks source accounting/builds,
not gameplay truth; overall completion also requires the final QA record audit.

The agent may implement needed library changes directly in ROMAtlas, uncommitted, with a
log and relevant toolkit checks. Launch another agent manually from the port workspace
and give it `prompts/review.md` to review the actual diff against the initial toolkit
snapshot. You decide check-in. No release packaging or library publication is performed;
the development port uses an editable local dependency. See
[docs/porting.md](docs/porting.md) for the full contract and extension formats.

## 9. Run a browser port or invoke separate QA

From a prepared port workspace, `python3 run.py web --port 8000` builds and serves its
WebAssembly frontend. The default bind address is `0.0.0.0`; use `--host` to change it.
Press Start in the browser to enable audio and input. The server supplies your local ROM
for startup asset extraction; original assets are not bundled.

New port workspaces already contain the separate QA instructions. To start or resume QA,
ask an agent in the port workspace: `Read debug/goal.md`. Preparation does not start QA.
This workflow develops a game-specific plan and completes two continuous A-to-Z passes,
fixing issues as encountered. Repairs and plan changes preserve completed work and do
not restart pass numbering; incomplete runs resume in place. See the [debugging
guide](docs/port-debugging.md) for the full contract.
