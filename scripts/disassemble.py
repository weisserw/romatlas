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

Read workflow.md, workspace.json, and analysis/progress.md before work.
The toolkit repository is {toolkit}. Read its project.md, docs/progress.md,
docs/protocol.md and docs/sym.md. Use its existing binary; this workspace is
for disassembling this game, not changing the toolkit.

Objective: classify ROM regions with evidence, export a portable stock .sym map,
and generate an unannotated bank-organized assembly listing that rebuilds
byte-for-byte. Semantic naming and explaining gameplay are outside this pass.
Do not stop after a raw-byte baseline: fidelity and classification completeness
are separate results. Report unknown bytes and unresolved bank contexts honestly.

Use python3 run.py inspect/baseline/serve/checkpoint as described in workflow.md.
Keep input/game.rom unchanged. Never download ROMs. Keep input, internal state,
traces, source and extracted assets local; only stock .sym is distributable.
Never commit the entire workspace. .gitignore is a guard, not authorization.

Use bounded execution, discovery and paged queries. Trace addresses are physical;
retain CPU aliases and effective bank context. Prefer analysis.discover_context.
Observed instruction fetches prove execution; reached targets and pointer values
do not. Unvisited bytes stay unknown; export's raw-byte gaps are not data evidence.
Inspect candidates before map.annotate. Split existing regions before inserting
smaller regions. Validate maps after edits. Save internal state with map.save
to analysis/state.json, and export stock .sym to maps/current.sym.

Record evidence, confidence, scenario input sequences, remaining unknown ranges,
and exact tested verification results in analysis/progress.md after each batch.
Snapshots are session-local; save replayable input sequences for later sessions.
Do not invent snapshot persistence or claim full hardware compatibility.
"""

WORKFLOW = """# Per-game workflow

Run your agent in this directory. Give it prompts/start.md for a first pass or
prompts/resume.md for a later pass. No particular agent provider is required.

1. Run `python3 run.py inspect`, then `python3 run.py baseline` once. Baseline
   creates maps/baseline.sym and generated/baseline and verifies the rebuild.
   It marks raw bytes for export only; it is not a classified disassembly.
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
maps/current.sym, and produce a verified checkpoint with code regions decoded
as assembly. Keep unknown bytes unknown internally. Do not stop at the baseline.
Record coverage, classification counts, unresolved targets, experiments and
verification in analysis/progress.md. No semantic recovery is required.
"""

RESUME = """Read AGENTS.md, workflow.md, workspace.json and analysis/progress.md.
Resume this ROM's structural disassembly. Load analysis/state.json into a fresh
ROMAtlas session if present. Review the last verified checkpoint and unresolved
regions. Replay recorded scenarios when execution evidence is needed; snapshots
from previous sessions are unavailable. Continue evidence-supported code/data
classification, save/export the updated map, and create a new verified checkpoint.
Report classification progress separately from rebuild fidelity.
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
   /goal Read AGENTS.md, workflow.md and prompts/start.md. Use ROMAtlas to identify evidence-supported code and data regions, export maps/current.sym and generate a byte-identical verified assembly listing. Continue beyond the raw-byte baseline; keep uncertain bytes unknown and record classification counts, unresolved regions and verification in analysis/progress.md.

3. For a later pass, open the same workspace and paste:
   /goal Read AGENTS.md, workflow.md and prompts/resume.md. Resume structural disassembly from analysis/state.json and analysis/progress.md, improve evidence-supported code/data classification, and save a new byte-identical verified checkpoint with remaining unknown regions documented.

Inside Codex, /goal shows status; /goal pause pauses and /goal resume resumes
an existing paused goal. These are Codex input commands, not shell commands.
If /goal is unavailable, use a Codex version with goal support.
""", file=sys.stderr, flush=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("roms", type=Path, nargs="+", help="local owned .gb/.gbc files")
    parser.add_argument("--root", type=Path, default=Path.home() / "rom-work",
                        help="parent for game workspaces (default: ~/rom-work)")
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
