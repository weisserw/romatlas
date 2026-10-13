# Starting a ROM disassembly

## 1. Set up and build the toolkit

Run these commands in the ROMAtlas repository on Ubuntu/Debian:

```sh
cd /root/romatlas
bash scripts/setup.sh
export PATH="$HOME/.cargo/bin:$PATH"
cargo build --locked
```

The setup script requires root privileges; use `sudo bash scripts/setup.sh` if
needed. It installs the pinned tools, including Rust 1.90.0.
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

Follow the startup instructions printed by the command. For example, run this
in your terminal:

```sh
codex --enable goals -C /root/rom-work/game
```

Once Codex opens, paste the printed `/goal ...` command into Codex's input and
press Enter. The goal tells the agent to read the workspace instructions,
identify code and data, export a mapping, and generate an assembly listing
that rebuilds byte-for-byte. Use a Codex version with goal support.

The objective spans the entire physical ROM, across sessions and checkpoints.
The agent investigates structure and semantics together, annotating each region's
structural role and supporting evidence. Detailed instruction commentary is optional.
Success requires every byte to be classified with evidence, required annotations,
valid maps and a byte-identical rebuild. Raw bytes preserve fidelity but do not
establish classification; execution coverage is a separate measure.

There is no universal total investigation budget. The generated workflow requires
an investigation ledger at `analysis/investigation.md`, reviewing static flow,
references/callers, pointer and bank hypotheses, runtime experiments and structural
patterns for every unresolved range. Applicable methods must be attempted and
exclusions justified. The agent continues while concrete unblocked paths remain,
reopening entries when new evidence changes them. Repeated traces or cosmetic
annotations do not count as progress.

After identifying the game and likely version, the agent consults available online
manuals, walkthroughs, FAQs and technical resources. It keeps source links, version
uncertainty, controls, mechanics and actionable progression/password routes in
`analysis/gameplay.md`, translating useful findings into targeted runtime scenarios.
External claims need confirmation against the local ROM and do not independently
prove structural classifications. Research is revisited for specific obstacles,
including before exhausting gameplay-dependent paths. If resources are unavailable,
the agent records that limitation and continues local analysis.

Exhausted investigation permits an **incomplete** handoff only when all remaining
paths have specific evidenced blockers or require unsupported guessing. Unreachable
code alone does not establish unknowability. This rule does not mathematically
guarantee termination. The final report includes per-bank classification counts,
annotation coverage, verification and unresolved evidence links. An incomplete
handoff or a verified checkpoint must never mark the whole-ROM goal complete.

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

The command prints a Codex startup command and a short `/goal` referring to
`commentary/prompt.md`. That prompt asks the agent to review the entire game,
identify routines, loops, shared tails and data structures, introduce meaningful
symbols, and explain functionality using conventional readable assembly style.
Comment detail scales with complexity; opcode translations and boilerplate
headers are discouraged. Static interpretation is the default, tentative meanings
use `(?)`, and debugging targets genuinely unclear behavior. Completion requires
full review, documented investigated uncertainties, a final editorial pass and a
fresh byte-identical rebuild, rather than certainty about every original intent.

The files to read and compare are:

- `commentary/reading-before.asm`: frozen consolidated baseline from the current map.
- `commentary/source/game.asm` and its includes: authoritative editable, buildable source.
- `commentary/reading-after.asm`: consolidated reading copy of that source.
- `commentary/review.json`: full-ROM scaffold and agent-maintained semantic review inventory.
- `commentary/progress.md` and `commentary/uncertainties.md`: progress and useful unresolved interpretations.

The reading copies expand `.asm` includes in source order, mark file boundaries,
and retain hardware/constants includes and binary-asset references. They are
reading aids, not independent build targets. Preserve the source tree and assets
to rebuild the commented game: **the `.sym` alone cannot reproduce comments**.
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
requires final editorial notes, performs fresh verification and refreshes the
reading copy. It checks coverage/status, not comment quality; the agent remains
responsible for useful and coherent explanations. Reviewed ambiguity can satisfy
phase 2 without declaring the phase-1 classification goal complete.

All assembly, ROMs, extracted assets and reading copies remain local. Do not
commit or distribute a commercial game's generated workspace.
