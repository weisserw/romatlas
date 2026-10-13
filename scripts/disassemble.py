#!/usr/bin/env python3
"""Prepare isolated, local agent workspaces for owned Game Boy ROMs."""
import argparse
import hashlib
import json
from pathlib import Path
import shlex
import shutil
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[1]

AGENTS = """# ROM disassembly workspace

Read workflow.md, workspace.json, analysis/progress.md, and
analysis/toolkit-issues.md before work.
Read and maintain analysis/gameplay.md for sourced gameplay familiarity and
targeted experiment routes, following workflow.md's Game familiarization section.
The toolkit repository is {toolkit}. Read its project.md, docs/progress.md,
docs/protocol.md and docs/sym.md. Use its existing binary; this workspace is
for disassembling this game, not changing the toolkit.

Objective: classify ROM regions with evidence, export a portable stock .sym map,
and generate a bank-organized assembly listing that rebuilds byte-for-byte.
Investigate structure and semantics together; name and explain routines where
useful. Required annotations describe structural role and supporting evidence;
exhaustive explanations of every instruction are optional.
Do not stop after a raw-byte baseline: fidelity and classification completeness
are separate results. Report unknown bytes and unresolved bank contexts honestly.

Use python3 run.py inspect/baseline/serve/checkpoint as described in workflow.md.
Keep input/game.rom unchanged. Never download ROMs. Keep input, internal state,
traces, source and extracted assets local; only stock .sym is distributable.
Never commit the entire workspace. .gitignore is a guard, not authorization.

Use bounded execution, discovery and paged queries. Trace addresses are physical;
retain CPU aliases and effective bank context. Prefer analysis.discover_context.
Observed instruction fetches prove execution; reached targets and pointer values
do not. Unexecuted bytes may be classified using static or semantic evidence;
unsupported bytes stay unknown. Export's raw-byte gaps are not data evidence.
Inspect candidates before map.annotate. Split existing regions before inserting
smaller regions. Validate maps after edits. Save internal state with map.save
to analysis/state.json, and export stock .sym to maps/current.sym.

Record evidence, confidence, scenario input sequences, remaining unknown ranges,
and exact tested verification results in analysis/progress.md after each batch.
Snapshots are session-local; save replayable input sequences for later sessions.
Do not invent snapshot persistence or claim full hardware compatibility.

Maintain analysis/toolkit-issues.md as you encounter toolkit bugs or missing
features, following workflow.md's reporting requirements. Preserve findings
across sessions. Continue ROM analysis using workarounds unless blocked; do not
fix the toolkit in this workspace. At the end of every pass, summarize toolkit
findings and link the report, or explicitly say "no toolkit issues found" if empty.
"""

WORKFLOW = """# Per-game workflow

Run your agent in this directory. Give it prompts/start.md for a first pass or
prompts/resume.md for a later pass. No particular agent provider is required.
Both prompts pursue the same whole-ROM objective. A pass is a checkpoint, not
goal completion. Follow the completion and investigation contract below.

1. Run `python3 run.py inspect`, then `python3 run.py baseline` once. Baseline
   creates maps/baseline.sym and generated/baseline and verifies the rebuild.
   It marks raw bytes for export only; it is not a classified disassembly.
   After inspection identifies the game and likely version, perform the Game
   familiarization step below before planning runtime scenarios.
2. Run `python3 run.py serve` as a persistent JSON-RPC subprocess. Use
   api.describe. A new session has empty analysis state. Load analysis/state.json
   with map.load when resuming. Do not import the raw baseline as classification.
3. Discover from entry offset 256 and valid interrupt/restart entry points after
   inspecting them. Supply effective CPU/bank context and bounded budgets to
   analysis.discover_context. Inspect candidates, references and diagnostics.
4. Start the emulator, inspect capabilities, enable bounded tracing, and run
   reproducible startup/menu/gameplay inputs. Save named snapshots for in-session
   branching. Record both presses and releases and execution budgets in
   analysis/scenarios.json. Clear traces before replay for a forward timeline.
   Restart/reset disables tracing; re-enable it. Inspect drops and budget stops.
5. Merge observed roots/edges with static discovery. Inspect data references and
   explicit analysis.pointers hypotheses. Confirm only supported regions with
   map.annotate, preserving confidence/provenance; leave other bytes unknown.
6. Call map.validate, map.save to analysis/state.json, and map.export to
   maps/current.sym (include_sym=false). These paths are relative to this workspace.
7. Run `python3 run.py checkpoint pass-001` (use a new name each time). It validates
   current.sym, generates from it, and verifies the new source directory. Each
   successful checkpoint saves a copy of the exact map used and a verification
   report. Keep the internal store and progress notes alongside these results.
8. Repeat for unknown regions and unresolved paths. End each pass with classified
   code/data/unknown byte counts from the internal store, uncertainty, execution
   scenarios exercised, verification evidence, and concrete next tasks.
   Include a summary of toolkit findings and a link to analysis/toolkit-issues.md;
   explicitly say "no toolkit issues found" when the report has no findings.

## Game familiarization

Consult available online manuals, walkthroughs, FAQs (such as GameFAQs), gameplay
guides and relevant technical resources after identifying the game and likely
platform/region/revision. Do not rely on training-set familiarity. Record concise,
actionable findings in analysis/gameplay.md with source URLs, applicable version,
uncertainty and confirmation status. Preserve and update this record on resume;
create it if absent. Record controls, menus, modes, progression and distinctive
mechanics; routes to later levels, bosses, bonus stages, endings and unusual states;
and passwords or built-in shortcuts that enable practical targeted experiments.
Do not download ROMs or extracted game assets as part of research.

Turn findings into specific runtime hypotheses, evidence sought and reproducible
input routes in analysis/scenarios.json; cross-link unresolved investigation entries.
External descriptions are hypotheses until confirmed against this ROM and do not
independently justify structural classification. Distinguish sourced instructions
from tested input sequences and observed results, including version mismatches.
Prioritize research that changes an experiment or resolves an obstacle, rather than
plot summaries or exhaustive background collection. If sources or browsing are
unavailable, record the limitation and continue local analysis; lack of a guide
alone does not justify terminating the whole investigation.

Revisit research when it could resolve a specific obstacle. Before exhausting a
gameplay-dependent investigation path, check for documented controls, passwords,
shortcuts and progression routes that could make the experiment feasible. New
actionable findings reopen affected ledger entries under the completion contract.

## Completion and investigation contract

Declare complete only when every physical ROM byte belongs to an evidence-backed
structural region, all required role/evidence annotations exist, the internal map
and exported stock map validate, and the final generated assembly rebuilds
byte-identically. Include headers, padding, unused banks and unreachable regions.
Do not decode arbitrary bytes as code, rename unknown to data, or use raw emission
as classification evidence. Semantic investigation is encouraged throughout;
exhaustive instruction commentary is not required. Keep annotations and evidence
in analysis/progress.md and the internal store where supported; use only supported
stock directives in the portable map. Execution coverage need not reach 100%.

Maintain analysis/investigation.md across sessions with stable entries for each
unresolved physical range and bank/context ambiguity. Record structural hypotheses,
evidence and local artifact links, competing interpretations, methods attempted,
results, remaining paths, and specific blockers. Split or merge entries as ranges
change, retaining history. Before declaring exhaustion, review every bank and every
unresolved range, including ranges never executed. For each entry assess:
- Static control flow and valid instruction boundaries.
- Incoming references and callers' behavior, including semantic interpretation.
- Pointer/table hypotheses and effective bank/CPU-window context.
- Runtime instruction/data accesses and targeted input/snapshot experiments.
- Structural patterns, neighboring regions and relevant format constraints.
Attempt applicable methods; justify each exclusion specifically. Absence of an
execution trace or apparent unreachability alone proves neither data nor
unknowability. Check available APIs/documentation before claiming a tool blocker;
record missing tools in toolkit-issues.md and consider bounded workarounds.

Runtime experiments must identify the unresolved hypothesis, evidence sought and
a concrete input/snapshot route. Record outcomes and alternatives. Continue play
when a concrete route could obtain useful evidence; do not repeat gameplay without
such a route. Bound individual calls, paginate scans and finish truncated queries.
These operational bounds are not a total investigation budget.

Progress means new evidence, justified classification, resolved references,
eliminated hypotheses or a demonstrated blocker. Repeated traces, cosmetic names,
unsupported classifications and unchanged hypotheses are zero progress. No fixed
total budget or number of stagnant passes authorizes stopping. A new experiment
must explain how it could distinguish hypotheses; speculative possibilities alone
do not justify endless retries. Reopen entries when discoveries elsewhere change
their evidence or make a blocked method feasible. Audit those dependencies before
the final exhaustion decision.

Stop incomplete only when every remaining path in every unresolved entry has a
concrete documented blocker or requires unsupported guessing. Evidence must
support exhaustion of applicable methods, not merely a claim of diminishing
returns, exponential effort or several unsuccessful passes. For combinatorial
obstacles, identify the alternatives, why available evidence cannot narrow them,
and why targeted experiments cannot distinguish them. Distinguish demonstrated
impossibility from uncertainty with available methods; do not claim universal
unknowability. If any concrete unblocked investigation path remains, continue.
This evidence-based rule is not a mathematical guarantee of termination.

At termination save state, export/validate the map and produce a final verified
checkpoint when possible. Report outcome complete or incomplete, checkpoint and
verification links, per-bank counts by internal kind (including unknown gaps),
required annotation coverage, unresolved ranges/context and investigation links,
and toolkit findings. Counts must cover the ROM exactly without overlap; report
execution coverage separately. A checkpoint or byte-identical rebuild alone never
satisfies the goal. Incomplete termination is an honest handoff, not goal success:
never mark an unfinished whole-ROM goal complete. If goal tooling has no incomplete
state, report the limitation and use only an available status whose conditions are
actually met; do not invent a status or pause without user authorization.

## Toolkit issues

Keep a separate, persistent report in analysis/toolkit-issues.md. Read it when
starting or resuming; create it if absent. Record findings when encountered,
not just at completion, so interrupted sessions retain them. Update existing
entries when evidence changes and avoid duplicates; never overwrite earlier
findings on resume. Keep ROM classification evidence in analysis/progress.md.

For each finding, record:
- A stable ID and short title.
- Type: confirmed bug, suspected bug, or missing feature.
- Affected toolkit command/API/component and toolkit version or commit if known.
- Evidence: the concrete analysis task, expected versus actual behavior for
  bugs, reproduction commands/API parameters where available, and local links
  to relevant traces, logs or analysis artifacts. If not reproduced, explain
  the uncertainty and keep the bug suspected.
- Impact on analysis speed, inference quality or reliability of conclusions.
- Workaround, or explicitly state that none is known and whether progress is blocked.
- Suggested fix or improvement, without implementing it during disassembly.

Missing features must be grounded in a concrete example from this ROM analysis
and explain how the functionality would substantially improve reverse engineering
performance or inference. Exclude speculative wish lists and cosmetic preferences.
Check toolkit documentation before treating a limitation as a defect. Distinguish
documented limits from newly demonstrated bugs; a documented limit can still be
a missing feature if it meets the evidence and substantial-impact requirements.
Missing execution coverage or an unresolved ROM region alone is not a toolkit bug.

Continue the disassembly using bounded workarounds unless an issue blocks it.
If blocked, record the blocker and save available progress before reporting it.
At every pass completion or blocked handoff, summarize the report's findings by
type, identify blockers and link the report. If there are no findings, explicitly
say "no toolkit issues found"; this describes the pass, not proof of no defects.

To resume, a new serve process needs map.load and emulator.start, then replay the
recorded inputs; named snapshots from the old process are unavailable. Inspection
uses the ROM identity captured at initialization. run.py rejects changed input.
Generated files remain local. A verified raw-byte listing is a fidelity baseline,
not evidence that classification is complete.
"""

START = """Read AGENTS.md and workflow.md. Disassemble this workspace's ROM using
ROMAtlas. Establish the verified baseline, then continue with bank-aware static
discovery and bounded execution traces to identify code and data. Confirm only
evidence-supported structural regions. Save analysis/state.json, export
maps/current.sym, and produce verified checkpoints with code regions decoded
as assembly. Keep unsupported bytes unknown internally. Do not stop at the baseline.
After identifying the game/version, follow workflow.md's Game familiarization
section: consult online resources and record actionable, sourced routes in
analysis/gameplay.md before planning runtime experiments. Treat external claims
as hypotheses until confirmed against this ROM; continue locally if unavailable.
Record coverage, classification counts, unresolved targets, experiments and
verification in analysis/progress.md. Investigate semantics wherever useful and
annotate structural roles with evidence. Follow workflow.md's completion and
investigation contract, maintaining analysis/investigation.md. Continue toward
whole-ROM completion; a first pass or checkpoint is not goal completion. Stop
incomplete only under the documented exhaustion rule, never as successful completion.
Record encountered toolkit bugs and substantial, evidence-backed missing features
in analysis/toolkit-issues.md as you work, following workflow.md. Continue analysis
using workarounds where possible; do not fix the toolkit. At termination include a findings summary and report
link, or explicitly say "no toolkit issues found" if empty.
"""

RESUME = """Read AGENTS.md, workflow.md, workspace.json, analysis/progress.md and
analysis/toolkit-issues.md.
Resume the same whole-ROM disassembly objective. Load analysis/state.json into a fresh
ROMAtlas session if present. Review the last verified checkpoint and unresolved
regions. Replay recorded scenarios when execution evidence is needed; snapshots
from previous sessions are unavailable. Continue evidence-supported code/data
classification, save/export the updated map, and create a new verified checkpoint.
Report classification progress separately from rebuild fidelity.
Read analysis/investigation.md and reopen exhausted entries affected by new evidence.
Read and preserve analysis/gameplay.md. Follow workflow.md's Game familiarization
section, revisiting online resources for specific obstacles and checking documented
routes before exhausting gameplay-dependent paths. Confirm claims against this ROM.
Investigate semantics wherever useful and annotate structural roles with evidence.
Follow workflow.md's completion and investigation contract. A new checkpoint is
not goal completion; continue while concrete unblocked paths remain. Stop incomplete
only under the documented exhaustion rule, never as successful completion.
Preserve and update the toolkit issue report as you work; merge repeated findings
and follow workflow.md's evidence requirements for bugs and substantial missing
features. Continue analysis using workarounds where possible; do not fix the toolkit. At termination include
a findings summary and report link, or say "no toolkit issues found" if empty.
"""

TOOLKIT_ISSUES = """# Toolkit issues

No findings recorded yet; ROM analysis has not started.

Preserve and update this report across passes. Follow workflow.md's Toolkit issues
section for evidence requirements and entry fields. Use one entry per finding with
a stable ID; distinguish confirmed bugs, suspected bugs, and missing features.
"""

RUNNER = '''#!/usr/bin/env python3
"""Local workspace commands; no shell expansion or agent-provider dependency."""
import argparse
import hashlib
import json
from pathlib import Path
import re
import shutil
import subprocess
import sys

ROOT = Path(__file__).resolve().parent

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("command", choices=["inspect", "baseline", "serve", "checkpoint"])
    parser.add_argument("name", nargs="?")
    args = parser.parse_args()
    if args.command == "checkpoint":
        if not args.name or not re.fullmatch(r"[A-Za-z0-9][A-Za-z0-9_-]*", args.name):
            parser.error("checkpoint requires a simple new name, e.g. pass-001")
        if args.name == "baseline":
            parser.error("baseline is reserved")
    elif args.name:
        parser.error("only checkpoint accepts a name")
    meta = json.loads((ROOT / "workspace.json").read_text())
    rom = ROOT / meta["rom"]
    if hashlib.sha256(rom.read_bytes()).hexdigest() != meta["sha256"]:
        raise ValueError("input ROM changed; create a separate workspace for another image")
    binary = meta["binary"]
    def call(*values, capture=False):
        result = subprocess.run([binary, *map(str, values)], cwd=ROOT,
                                check=True, text=True, capture_output=capture)
        return json.loads(result.stdout) if capture else None
    if args.command == "inspect":
        call("inspect", rom)
        return
    if args.command == "serve":
        call("serve", "--rom", rom)
        return
    name = "baseline" if args.command == "baseline" else args.name
    output = ROOT / "generated" / name
    saved_map = ROOT / "maps" / (name + ".sym")
    report = ROOT / "analysis" / (name + "-verification.json")
    if any(p.exists() or p.is_symlink() for p in [output, saved_map, report]):
        raise ValueError("checkpoint paths already exist; choose a new name")
    if name == "baseline":
        call("map", "init", rom, "--output", saved_map)
    else:
        shutil.copyfile(ROOT / "maps/current.sym", saved_map)
    call("map", "validate", saved_map, "--rom", rom)
    call("generate", rom, "--map", saved_map, "--out", output)
    result = call("verify", rom, "--generated", output, capture=True)
    if result.get("ok") is not True or result.get("byte_identical") is not True:
        raise ValueError("verification did not confirm byte identity")
    result.update(rom_sha256=meta["sha256"],
                  map_sha256=hashlib.sha256(saved_map.read_bytes()).hexdigest(),
                  map=str(saved_map.relative_to(ROOT)),
                  generated=str(output.relative_to(ROOT)))
    report.write_text(json.dumps(result, indent=2) + "\\n")
    print(json.dumps(result))

if __name__ == "__main__":
    try:
        main()
    except (OSError, ValueError, subprocess.CalledProcessError) as error:
        print(json.dumps({"ok": False, "error": str(error)}), file=sys.stderr)
        sys.exit(1)
'''


def initialize(source, destination, binary):
    source = source.resolve(strict=True)
    destination = destination.absolute()
    if destination.exists() or destination.is_symlink():
        raise ValueError(f"workspace already exists: {destination}")
    resolved = destination.resolve()
    if resolved == ROOT or ROOT in resolved.parents:
        raise ValueError("game workspaces must be outside the toolkit repository")
    if not source.is_file():
        raise ValueError(f"not a ROM file: {source}")
    destination.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(prefix=".romatlas-", dir=destination.parent) as temp:
        work = Path(temp) / "workspace"
        work.mkdir()
        for directory in ["input", "analysis", "maps", "generated", "prompts", "traces"]:
            (work / directory).mkdir()
        rom = work / "input/game.rom"
        shutil.copyfile(source, rom)
        inspected = subprocess.run([str(binary), "inspect", str(rom)],
                                   check=True, capture_output=True, text=True)
        metadata = json.loads(inspected.stdout)
        identity = hashlib.sha256(rom.read_bytes()).hexdigest()
        files = {
            "AGENTS.md": AGENTS.format(toolkit=json.dumps(str(ROOT))),
            "workflow.md": WORKFLOW,
            "prompts/start.md": START,
            "prompts/resume.md": RESUME,
            "analysis/progress.md": "# Disassembly progress\n\nWorkspace initialized. Baseline and classification have not run.\n",
            "analysis/scenarios.json": "[]\n",
            "analysis/investigation.md": "# Investigation ledger\n\nNo regions reviewed yet. Follow workflow.md's completion and investigation contract.\n",
            "analysis/gameplay.md": "# Gameplay research\n\nNo research performed yet. Follow workflow.md's Game familiarization section. Record source URLs, version uncertainty, actionable routes and local confirmation separately.\n",
            "analysis/toolkit-issues.md": TOOLKIT_ISSUES,
            "run.py": RUNNER,
            ".gitignore": "*\n",
            "workspace.json": json.dumps({"version": 1, "toolkit": str(ROOT),
                                          "binary": str(binary), "rom": "input/game.rom",
                                          "sha256": identity, "inspection": metadata}, indent=2) + "\n",
        }
        for name, content in files.items():
            (work / name).write_text(content)
        # Refuse replacement if another initializer created the destination meanwhile.
        destination.mkdir()
        # Retain a partial workspace for inspection on failure; never overwrite it.
        for item in work.iterdir():
            shutil.move(str(item), destination / item.name)
    return {"ok": True, "workspace": str(destination), "sha256": identity,
            "start_prompt": str(destination / "prompts/start.md"),
            "resume_prompt": str(destination / "prompts/resume.md")}


def print_codex_instructions(record):
    workspace = shlex.quote(record["workspace"])
    print(f"""
Workspace ready: {record['workspace']}

1. Run this in your terminal:
   codex --enable goals -C {workspace}

2. Once Codex opens, paste this into its input and press Enter:
   /goal Read AGENTS.md, workflow.md and prompts/start.md. Complete the whole-ROM disassembly and evidence-backed structural annotations under workflow.md's completion and investigation contract. Continue beyond the raw-byte baseline and individual checkpoints; investigate semantics wherever useful. Success requires every physical byte classified with evidence, required annotations, valid maps and a byte-identical rebuild. Maintain analysis/progress.md, analysis/investigation.md and analysis/toolkit-issues.md. Continue while concrete unblocked paths remain; exhausted investigation permits an incomplete handoff, never marking this goal complete. Report per-bank counts, verification, unresolved evidence and toolkit findings with report links, or say "no toolkit issues found" if empty. Do not fix the toolkit.

3. For a later session without an existing unfinished goal, open the same workspace and paste:
   /goal Read AGENTS.md, workflow.md and prompts/resume.md. Resume the same whole-ROM disassembly and evidence-backed structural annotations under workflow.md's completion and investigation contract. Restore analysis/state.json and review analysis/progress.md and analysis/investigation.md; reopen entries affected by new evidence. Success requires every physical byte classified with evidence, required annotations, valid maps and a byte-identical rebuild. A new checkpoint is not goal completion. Continue while concrete unblocked paths remain; exhausted investigation permits an incomplete handoff, never marking this goal complete. Preserve analysis/toolkit-issues.md; report per-bank counts, verification, unresolved evidence and toolkit findings with report links, or say "no toolkit issues found" if empty. Do not fix the toolkit.

Inside Codex, /goal shows status; /goal pause pauses and /goal resume resumes
an existing paused goal. These are Codex input commands, not shell commands.
If /goal is unavailable, use a Codex version with goal support.
""", file=sys.stderr, flush=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("roms", type=Path, nargs="+", help="local owned .gb/.gbc files")
    parser.add_argument("--root", type=Path, default=Path.cwd(),
                        help="parent for game workspaces (default: current directory)")
    parser.add_argument("--workspace", type=Path, help="exact directory; only for one ROM")
    parser.add_argument("--binary", type=Path, default=ROOT / "target/debug/romatlas")
    parser.add_argument("--json", action="store_true",
                        help="print JSON results only; suppress Codex startup instructions")
    args = parser.parse_args()
    if args.workspace and len(args.roms) != 1:
        parser.error("--workspace requires exactly one ROM")
    binary = args.binary.resolve(strict=True)
    destinations = [args.workspace or args.root / rom.stem for rom in args.roms]
    if len({str(p.resolve()) for p in destinations}) != len(destinations):
        parser.error("ROM filenames produce duplicate workspaces; use distinct names or --workspace")
    for source, destination in zip(args.roms, destinations):
        record = initialize(source, destination, binary)
        print(json.dumps(record), flush=True)
        if not args.json:
            print_codex_instructions(record)


if __name__ == "__main__":
    try:
        main()
    except (OSError, ValueError, subprocess.CalledProcessError) as error:
        print(json.dumps({"ok": False, "error": str(error)}), file=sys.stderr)
        sys.exit(1)
