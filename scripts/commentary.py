#!/usr/bin/env python3
"""Prepare and maintain a local commentary phase in an existing ROM workspace."""
import argparse
import hashlib
import json
from pathlib import Path
import re
import shlex
import shutil
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[1]
INCLUDE = re.compile(r'^\s*INCLUDE\s+"([^"]+)"\s*(?:;.*)?$', re.I)

PROMPT = """# Incremental commentary and final editorial review

Read commentary/AGENTS.md and workflow.md. Work in commentary/source/, preserving
phase-1 maps/evidence and the frozen before/ baseline. Read the current checkpoint,
phase handoff and relevant analysis records; retrieve details only as needed.

Readable assembly is a required deliverable and the porting agent's source input.
Produce a fully commented and labeled listing: explain every meaningful routine,
control-flow block and data structure, with descriptive routine and branch labels,
RAM variable names, object-field offsets and constants used consistently in operands.
Comments explain purpose, algorithms, inputs/outputs, state changes and relevant
bank/format constraints; they need not paraphrase every instruction. When game
meaning is unknown, name the established mechanical role and explain the remaining
uncertainty nearby. Do not invent meanings or treat an address-only label or raw
address as an adequate explanation of an understood role.

Every commentary invocation must bring ALL currently classified source up to this
readability standard before handing off. Catch up the entire existing readability
backlog, including older uncommented code and concrete defects in reviewed material;
do not choose a small backlog slice. Unknown ROM regions need honest boundary/role
and uncertainty notes, not fabricated decoding or invented meanings. Unknown game
purpose does not excuse skipping explanation of established mechanics.
Preserve valid prior review credit and good existing prose; later invocations may
reuse that work and focus edits on changes, affected dependencies and remaining gaps.
Inspect the full current source inventory for gaps, without cosmetically rewriting
sound material. Report classification and source readability separately.

Use descriptive routine, branch, RAM variable and constant names wherever useful.
Accept the best coherent static interpretation from instructions, callers and
consumers; runtime proof and the original author's exact intent are not required.
Replace numeric operands with symbols when their roles are understood, preserving
exact values. Keep address-based identifiers where roles remain unknown. Explain
purpose and algorithms rather than translating instructions. Document formats,
units, bounds, bank context and consumers once, near the relevant structure.
Write clear, light prose: short functional comments in natural language, with
only the detail needed to understand purpose, behavior or a material assumption.
Avoid compressed address-heavy shorthand and mechanical opcode narration.
Rewrite existing explanations rather
than stacking duplicate old/new comments; remove redundant opcode paraphrases.

Use ordinary tentative prose for material hypotheses, e.g. 'Probably selects the
landing animation; also called during damage recovery.' Separate exact mechanics
from inferred purpose. No graded question-mark taxonomy or marker quota is needed;
preserve useful older uncertainty notes until affected evidence merits revision.
Record significant assumptions and concrete unresolved questions, not every possible
doubt. Revise names/comments when contradicted. Do not relabel phase-1 ambiguity
or weaken evidence-backed structural classification to finish commentary.

Use bounded debugging only for a concrete question likely to clarify useful behavior.
Diminishing returns on semantic research permit an explicit unresolved explanation,
not an early exit from naming/commenting the rest of the source. Publish redirect
questions as they arise, then continue commentary until the current-source scope
is complete. A small repair, useful finding or prepared lead is not a stop condition.
If interrupted by context/time limits, save exact remaining ranges and resume the
same commentary phase; report interrupted, not successful or ready for redirect.

Keep review.json's full-ROM scaffold intact. Add compact records for meaningful
routines/structures with physical ranges, bank context, source locations, status
and useful summaries. Do not inventory every loop/branch by default or duplicate
an explanation across progress, inventory and uncertainties. Keep unreviewed work
pending. Update commentary/progress.md with this pass's scope, names/interpretations
changed, unresolved dependencies, deferred ranges and actual verification results.
Publish concrete questions for redirect as soon as available; link supporting
source/ranges and suggest bounded checks without requiring a full lead dossier.

Verify after coherent source-edit batches and at phase handoff; refresh the reading
copy at handoff. Do not repeatedly rebuild after tiny comment edits or run the final
completion audit while deliberately leaving review pending. A verified handoff is successful only after the current-source readability scope
is complete; this does not establish whole-ROM classification or final acceptance.

At final acceptance, address remaining pending review and known editorial issues:
missing structures, stale numeric aliases, wrong boundaries, contradictory names,
redundant comments and misleading certainty. Reuse valid reviews; do not schedule
a mandatory whole-source reread or cosmetic rewrite. Revisit reviewed material only
when changed evidence, dependencies or a concrete inconsistency warrants it.
Every scaffold entry and meaningful discovered unit must then be reviewed; an
explained unknown may remain unresolved. Run audit for full inventory/editorial
acceptance plus a fresh byte-identical rebuild and refreshed reading copy.
The audit cannot judge semantic or prose quality. Preserve upstream notices;
all ROMs, generated source/assets and traces remain local. Do not fix the toolkit.
"""

PHASE_AGENTS = """# Commentary phase instructions

Read workflow.md and prompt.md before editing source/.
Every invocation completes readability work across all currently classified source.
Interrupted work must resume in commentary before redirect; a small repair is not completion.
Full editorial acceptance is reserved for final cleanup and archive preparation.
Neither partial nor full editorial review declares phase-1 classification complete.
Follow parent ROM identity, local artifact, toolkit-reporting and bounded runtime
rules. Do not regenerate over source/ or edit before/.
"""


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def write_json(path, value):
    path.write_text(json.dumps(value, indent=2) + "\n")


def local(root, relative):
    path = (root / relative).resolve()
    if path == root or root not in path.parents:
        raise ValueError(f"path must stay inside {root}: {relative}")
    return path


def workspace(path):
    work = path.resolve(strict=True)
    if work == ROOT or ROOT in work.parents:
        raise ValueError("game artifacts must remain outside the toolkit repository")
    meta = json.loads((work / "workspace.json").read_text())
    rom = local(work, meta["rom"])
    if digest(rom) != meta["sha256"]:
        raise ValueError("input ROM changed")
    return work, meta, rom


def call(binary, *args):
    result = subprocess.run([str(binary), *map(str, args)], capture_output=True,
                            text=True, timeout=330)
    if result.returncode:
        raise ValueError(result.stderr.strip() or "toolkit command failed")
    value = json.loads(result.stdout)
    if value.get("ok") is not True:
        raise ValueError("toolkit command did not return ok=true")
    return value


def reading(source):
    """Expand assembly includes in their actual order; retain hardware definitions."""
    source = source.resolve()
    marker = source / "mapping.json"
    is_snes = marker.is_file() and json.loads(marker.read_text()).get("format") in {"romatlas-snes-map", "romatlas-nes-map", "romatlas-sms-map", "romatlas-gg-map"}
    include_pattern = re.compile(r'^\s*(?:incsrc|\.include)\s+"([^"\n]+)"\s*(?:;.*)?$', re.I) if is_snes else INCLUDE
    def expand(path, active):
        if path in active:
            raise ValueError(f"cyclic assembly include: {path.name}")
        relative = path.relative_to(source)
        result = [f"; ===== BEGIN {relative} =====\n"]
        for line in path.read_text().splitlines(keepends=True):
            match = include_pattern.match(line.rstrip("\r\n"))
            if match:
                included = local(source, match[1])
                if included.suffix.lower() == ".asm" or is_snes and included.suffix.lower() == ".inc":
                    result.append(expand(included, active | {path}))
                    continue
            result.append(line if line.endswith("\n") else line + "\n")
        result.append(f"; ===== END {relative} =====\n")
        return "".join(result)
    return "; Reading copy; edit the buildable source tree, not this file.\n" + expand(source / "game.asm", set())


def scaffold(state):
    if state.get("format") in {"romatlas-snes-map", "romatlas-nes-map", "romatlas-sms-map", "romatlas-gg-map"}:
        import snes_workflow
        state = snes_workflow.scaffold(state)
    rows = []
    cursor = 0
    size = state["image_size"]
    regions = sorted(state["regions"].items(), key=lambda item: item[1]["start"]["offset"])
    for identity, region in regions:
        start = region["start"]["offset"]
        end = start + region["length"]
        if region["start"]["space"] != "cartridge-rom" or start < cursor or end > size:
            raise ValueError("invalid or overlapping phase-1 regions")
        if start > cursor:
            rows.append({"id": f"unknown-{cursor:06x}", "offset": cursor,
                         "length": start - cursor, "kind": "Unknown"})
        rows.append({"id": f"region-{identity}", "offset": start,
                     "length": end - start, "kind": region["kind"],
                     "phase1": state.get("region_metadata", {}).get(identity, {})})
        cursor = end
    if cursor < size:
        rows.append({"id": f"unknown-{cursor:06x}", "offset": cursor,
                     "length": size - cursor, "kind": "Unknown"})
    for row in rows:
        row.update(status="pending", summary="", source=[], units=[])
    return rows


def commands(work):
    return f"python3 {shlex.quote(str(ROOT / 'scripts/commentary.py'))} {{command}} {shlex.quote(str(work))}"


def instructions(work):
    prompt = work / "prompts/commentary.txt"
    if not prompt.is_file():
        prompt = work / "commentary/prompt.md"
    return f"Read {prompt}\n"


def prepare(work, meta, rom):
    destination = work / "commentary"
    if destination.exists() or destination.is_symlink():
        raise ValueError("commentary already exists; use instructions/read/verify/audit to resume")
    state_path = work / "analysis/state.json"
    state = json.loads(state_path.read_text())
    if state["image_sha256"] != meta["sha256"] or state["image_size"] != rom.stat().st_size:
        raise ValueError("phase-1 state does not match the ROM")
    rows = scaffold(state)
    binary = Path(meta["binary"])
    call(binary, "store", "validate", rom, "--store", state_path)
    is_snes = meta.get("platform") in {"snes", "nes", "sms", "gg"}
    map_name = "baseline.json" if is_snes else "baseline.sym"
    mapping = work / ("maps/current.json" if is_snes else "maps/current.sym")
    call(binary, "map", "validate", mapping, "--rom", rom)
    with tempfile.TemporaryDirectory(prefix=".commentary-", dir=work) as temporary:
        stage = Path(temporary) / "commentary"
        stage.mkdir()
        shutil.copyfile(mapping, stage / map_name)
        shutil.copyfile(state_path, stage / "phase1-state.json")
        call(binary, "generate", rom, "--map", stage / map_name, "--out", stage / "before")
        verified = call(binary, "verify", rom, "--generated", stage / "before")
        if verified.get("byte_identical") is not True:
            raise ValueError("baseline is not byte-identical")
        # Verification build products are reproducible and need not become source assets.
        shutil.copytree(stage / "before", stage / "source",
                        ignore=shutil.ignore_patterns("*.o", "*.gb", "*.gbc", "game.map", "game.sym"))
        write_json(stage / "baseline-verification.json", verified)
        write_json(stage / "manifest.json", {"version": 1, "rom_sha256": meta["sha256"],
                   "map_sha256": digest(stage / map_name),
                   "state_sha256": digest(stage / "phase1-state.json"), "image_size": state["image_size"]})
        write_json(stage / "review.json", {"version": 1, "image_size": state["image_size"],
                   "regions": rows, "editorial_audit": {"status": "pending", "notes": ""}})
        (stage / "prompt.md").write_text(PROMPT)
        (stage / "AGENTS.md").write_text(PHASE_AGENTS)
        command = commands(work)
        (stage / "workflow.md").write_text("""# Commentary workflow

This phase starts from the current phase-1 map in an existing workspace.
before/, baseline.sym and phase1-state.json are frozen comparison artifacts.
source/ is independently maintained, buildable assembly. Never regenerate over it.
The portable .sym carries structural directives and labels, not the final comments.
To reproduce commentary, preserve source/ and its local assets, not just .sym.
Phase-1 unknown regions may remain unknown when their ambiguity has been reviewed
and explained. Every invocation completes current-source readability; interrupted
work resumes in this phase before redirect.
Final editorial completion is a separate acceptance gate; neither declares
phase-1 classification complete. See prompt.md for the full contract.

""" + f"""Commands from any working directory:

```sh
{command.format(command='instructions')}
{command.format(command='verify')}
{command.format(command='read')}
# Final editorial cleanup / archive acceptance only:
{command.format(command='audit')}
```

verify rebuilds source/game.o and compares the linked ROM with the original without
rgbfix header changes. Do not use make's default game.gb target as fidelity proof.
read refreshes reading-after.asm by expanding .asm includes in source order; it
retains hardware/constants includes and binary-asset references. It is a reading
copy, not a separate build target. reading-before.asm is frozen at preparation.
audit checks full-ROM inventory, reviewed entries/units and editorial notes, then
performs a fresh verification and refreshes the reading copy. It cannot measure
whether explanations are good: the agent must perform that editorial review.

review.json entries partition the physical ROM. Do not remove or change that
scaffold. Preserve valid review credit. Pending classified-source readability
work prevents a successful commentary handoff; interruptions must resume this phase. At final acceptance each needs status=reviewed, a summary and source locations.
Add compact units for meaningful routines and data structures:
name, offset, length, status, summary and source (list of file:line strings).
Units can overlap, but must lie within the ROM. Split a spanning unit's inventory
records across scaffold entries if useful. Every discovered unit must be reviewed at final acceptance.
Keep editorial_audit pending until final cleanup; then record reviewed and findings.
Verify coherent edit batches and handoffs; refresh the reading copy at handoff.
Do not repeatedly invoke the final audit during an intentionally partial pass.
Use uncertainties.md for investigated limitations; ambiguity is not skipped work.
Keep all source and assets local. Preserve toolkit reports and upstream notices.
""")
        (stage / "progress.md").write_text("# Commentary progress\n\nPrepared verified baseline. All semantic review entries are pending.\n")
        (stage / "uncertainties.md").write_text("# Commentary uncertainties\n\nCarry forward relevant phase-1 ambiguities as their regions are reviewed.\n")
        (stage / "reading-before.asm").write_text(reading(stage / "before"))
        (stage / "reading-after.asm").write_text(reading(stage / "source"))
        if is_snes:
            if meta.get("platform") == "gg":
                import gg_workflow as snes_workflow
            elif meta.get("platform") == "sms":
                import sms_workflow as snes_workflow
            elif meta.get("platform") == "nes":
                import nes_workflow as snes_workflow
            else:
                import snes_workflow
            for name in ["prompt.md", "workflow.md"]:
                file = stage / name
                file.write_text(snes_workflow.profile(file.read_text()) + snes_workflow.GUIDANCE)
        # Refuse concurrent preparation; retain other workspace files untouched.
        destination.mkdir()
        for item in stage.iterdir():
            shutil.move(str(item), destination / item.name)
    return {"ok": True, "commentary": str(destination), "byte_identical": True,
            "regions": len(rows), "reading_before": str(destination / "reading-before.asm")}


def verify(work, meta, rom):
    source = local(work, "commentary/source")
    result = call(meta["binary"], "verify", rom, "--generated", source)
    if result.get("byte_identical") is not True:
        raise ValueError("commented source is not byte-identical")
    result.update(rom_sha256=meta["sha256"], source_sha256={
        str(p.relative_to(source)): digest(p) for p in sorted(source.rglob("*"))
        if p.is_file() and p.suffix in {".asm", ".inc"}})
    write_json(work / "commentary/verification.json", result)
    return result


def audit(work, meta, rom):
    phase = work / "commentary"
    manifest = json.loads((phase / "manifest.json").read_text())
    if manifest["rom_sha256"] != meta["sha256"]:
        raise ValueError("commentary belongs to another ROM")
    if digest(phase / "phase1-state.json") != manifest["state_sha256"] or digest(phase / ("baseline.json" if meta.get("platform") in {"snes", "nes", "sms", "gg"} else "baseline.sym")) != manifest["map_sha256"]:
        raise ValueError("frozen phase-1 baseline changed")
    review = json.loads((phase / "review.json").read_text())
    expected = scaffold(json.loads((phase / "phase1-state.json").read_text()))
    if review["image_size"] != rom.stat().st_size or len(review["regions"]) != len(expected):
        raise ValueError("review inventory does not cover the baseline")
    units = 0
    def reviewed(entry):
        if entry.get("status") != "reviewed" or not entry.get("summary", "").strip() or not entry.get("source"):
            raise ValueError(f"unfinished review: {entry.get('id', entry.get('name', '?'))}")
        for location in entry["source"]:
            filename, line = location.rsplit(":", 1)
            path = local(phase / "source", filename)
            if not path.is_file() or not 1 <= int(line) <= len(path.read_text().splitlines()):
                raise ValueError(f"invalid source location: {location}")
    for entry, original in zip(review["regions"], expected):
        if any(entry.get(key) != original[key] for key in ["id", "offset", "length", "kind"]):
            raise ValueError("review scaffold changed")
        reviewed(entry)
        if entry["kind"] == "Code" and not entry.get("units"):
            raise ValueError(f"code region lacks reviewed units: {entry['id']}")
        for unit in entry.get("units", []):
            reviewed(unit)
            if not unit.get("name") or unit.get("offset", -1) < 0 or unit.get("length", 0) <= 0 or unit["offset"] + unit["length"] > review["image_size"]:
                raise ValueError("invalid review unit")
            units += 1
    editorial = review.get("editorial_audit", {})
    if editorial.get("status") != "reviewed" or not editorial.get("notes", "").strip():
        raise ValueError("final editorial audit is pending")
    result = verify(work, meta, rom)
    (phase / "reading-after.asm").write_text(reading(phase / "source"))
    result.update(reviewed_regions=len(expected), reviewed_units=units, editorial_audit=True)
    write_json(phase / "audit.json", result)
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("command", choices=["prepare", "instructions", "read", "verify", "audit"])
    parser.add_argument("workspace", type=Path)
    parser.add_argument("--json", action="store_true", help="suppress printed startup instructions")
    args = parser.parse_args()
    work, meta, rom = workspace(args.workspace)
    if args.command == "prepare":
        result = prepare(work, meta, rom)
        if not args.json:
            print(instructions(work), file=sys.stderr)
    elif args.command == "instructions":
        if not (work / "commentary/prompt.md").is_file():
            raise ValueError("run prepare first")
        print(instructions(work))
        return
    elif args.command == "read":
        output = work / "commentary/reading-after.asm"
        output.write_text(reading(work / "commentary/source"))
        result = {"ok": True, "reading_after": str(output)}
    elif args.command == "verify":
        result = verify(work, meta, rom)
    else:
        result = audit(work, meta, rom)
    print(json.dumps(result))


if __name__ == "__main__":
    try:
        main()
    except (OSError, ValueError, KeyError, TypeError, subprocess.SubprocessError) as error:
        print(json.dumps({"ok": False, "error": str(error)}), file=sys.stderr)
        sys.exit(1)
