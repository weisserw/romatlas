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

import behavior

ROOT = Path(__file__).resolve().parents[1]
INCLUDE = re.compile(r'^\s*INCLUDE\s+"([^"]+)"\s*(?:;.*)?$', re.I)

PROMPT = """# Phase 2: readable, fully reviewed assembly

Read commentary/AGENTS.md and commentary/workflow.md first. Work in this existing
game workspace; preserve all phase-1 evidence, maps and checkpoints. Read
the latest analysis/progress.md entries and relevant sections of
analysis/behavior.md, analysis/investigation.md, analysis/gameplay.md and analysis/toolkit-issues.md;
inspect analysis/state.json with bounded queries or local scripts. The source of truth for this
phase is commentary/source/, not generated/ or the portable .sym.

Reuse structural-pass semantic findings from the relevant investigation entries,
region metadata and batch records before re-deriving them. Check their evidence
and uncertainty against the actual source; earlier names are not unquestionable.
When explanation exposes a classification error or a newly resolved extent,
record old/proposed kinds and bounds, physical/bank context, evidence and affected
source/review units in analysis/investigation.md. Coordinate a single writer for
shared state. Snapshot the current state/map, apply the correction through normal
validated map APIs, save/export and verify a fresh classification checkpoint.
Do not rewrite commentary/baseline.sym, commentary/phase1-state.json or other
frozen baseline artifacts, and never regenerate over commentary/source. Reconcile
affected edited source manually, preserving names/comments and exact bytes; update
review units within the existing coverage scaffold and rerun commentary verification
and reading-copy generation. Record both checkpoint identities and reconciliation
status. If safe reconciliation is blocked, preserve the proposed correction and
explicit dependency rather than claiming it is applied or silently diverging.
Unresolved reconciliation is outstanding work, not editorial completion. Unknowns
remain honest; structural corrections may be discovered and resolved in this phase.

No context-management plugin is required. Summarize large inventories, traces and
test logs with local scripts or focused searches. Return counts, relevant excerpts,
failures and evidence paths; retain full evidence on disk. Read connected source
units and exact bytes/bank context before accepting interpretations. Use direct
tools for short reads and edits, bounded queries for large outputs, and concise
progress/next-action records. Retrieve historical details on demand. Avoid
unchanged repeat queries and redundant reports; never skip review or verification
to save context. Summaries are navigation aids, not evidence of correctness.

Objective: explain the implementation of released-game behavior sufficiently for
an assembly-literate human or agent to answer questions, modify it and build a
native implementation. Review relevant code, data, graphics, headers and padding;
opaque intervals may receive evidence-backed preservation treatment. Identify meaningful
routine entries and extents, alternate entries, shared tails, fallthroughs, loops
and branches; do not force every fragment into a conventional function model.

Use the conventions of well-written retro assembly: descriptive routine, local
branch, variable and constant names; short functional comments close to the code;
longer explanations only when complexity merits them. Small helpers may need just
a name and a few instruction comments. Larger routines may need purpose, important
inputs/results, side effects and algorithm notes, when useful to their reader.
Explain WHY and what happens in game terms (e.g. updates ball position), not opcode
translations (e.g. writes a register). Hardware explanations should describe the
effect, timing constraint or reason for a sequence. Do not add comment quotas or
boilerplate headers. Preserve upstream notices.

Thoroughly document data structures: record size, fields, signedness/endianness,
units, indexes, bounds, sentinels, pointer/bank interpretation, consumers and game
meaning where understood. Describe graphics blocks and their users without
requiring a comment for every pixel byte. Name RAM variables and constants and
replace numeric operands with them where useful; preserve their exact values.

Trust thoughtful static interpretation. Read callers, callees, neighboring code
and data consumers to establish coherent meanings; do not demand runtime proof
for every name. Accept the best coherent interpretation and move on. Mark material
tentative interpretations with (?) beside the relevant name/comment; put longer
alternatives in commentary/uncertainties.md only when useful. Contradictory evidence
requires revisiting a guess. Avoid unsupported certainty and invented specificity.

Use ROMAtlas debugging only when static reading leaves the role genuinely unclear
and a concrete bounded experiment could clarify it. Record inputs, budgets and
outcomes in the existing analysis records; retain physical/CPU/bank distinctions.
Do not require whole-game execution or debug every plausible interpretation.

Start from commentary/review.json: it inventories every physical byte using
phase-1 regions plus unknown gaps. These regions are a scaffold, NOT function
boundaries. Expand each entry's units with every routine, loop, shared tail,
significant branch and data structure found during reading. Give units stable
physical ranges/bank context, descriptive names, source locations, review status
and a short useful summary. Shared/overlapping units are allowed within the
scaffold; preserve its nonoverlapping full-ROM coverage. Revisit dependencies as
names and meanings become clearer. Keep statuses honest across interrupted runs.

Maintain commentary/behavior.json as the behavioral review inventory. Preparation
freezes inherited accounting, narrative findings and prior unknown dispositions in
handoff.json; missing legacy accounting starts pending, never accepted. The upstream
outcome remains historical: this phase may close inherited questions through static
analysis and must record the resolution. Every behavioral item needs source
locations and regions (scaffold IDs), linking systems across banks and shared
routines. Audit checks these links point to semantically reviewed regions.

Each review.json region has treatment=semantic or preserved, status=reviewed,
summary and source locations. Semantic Code and newly interpreted Unknown regions
need reviewed units. Preserved regions need preservation.rationale, evidence
(durable explanatory statements) and reopen_when. Known Code cannot use preserved
treatment to avoid semantic reading. Check inherited triage against the source,
record its limits, and do not repeat unchanged historical experiments. Unknown
purpose is acceptable when observable behavior is explained; an unresolved asset
selector affecting gameplay is an outstanding dependency, not editorial ambiguity.
Do not relabel opaque bytes just to finish. Preserve physical coverage as a scaffold,
not a requirement to recover every byte's historical meaning.

Keep commentary/progress.md current with behavioral review, region treatments,
refinements, source edits, uncertainties and actual verification. Maintain structural
corrections in refinements.json (id, status=pending/reconciled, summary, evidence)
and refinements.md with old/new bounds/kinds, bank context, affected behavior IDs,
map checkpoint and source reconciliation. Any outstanding correction blocks audit.

After batches, run the verification and reading commands in workflow.md. Before
completion, perform a whole-source editorial pass: check generic names, missing
structures, wrong boundaries, stale numeric aliases, contradictory interpretations,
misleading certainty and redundant comments. Every scaffold entry needs a reviewed semantic or preservation treatment and every
discovered unit must be reviewed, not just named. Ensure source pointers still match edits.
The script checks inventory coverage/status and byte identity; it cannot judge
comment quality. You are responsible for the editorial acceptance criteria.

Continue until behavioral review, region treatment and editorial work are done. No fixed pass count, comment
count or stagnant batch ends the goal. Documented, investigated semantic limits
are acceptable finished outcomes; missing review, broken builds, unresolved source
coverage or outstanding useful investigation are not. Report complete only with
the final audit, a fresh byte-identical verification, refreshed reading copy,
and links to source, inventory, uncertainties and toolkit findings. Do not fix the
toolkit. All ROMs, assembly, extracted assets and traces remain local.
"""

PHASE_AGENTS = """# Commentary phase instructions

Read workflow.md and prompt.md in this directory before editing source/.
This is phase 2 of an existing workspace. Follow this phase's editorial completion
contract. Editorial acceptance does not certify the parent disassembly's behavioral
accounting or change its recorded evidence and unknown-region dispositions.
Reviewed ambiguity and evidence-backed opaque preservation are allowed; missing
behavioral accounting and unresolved reconstruction dependencies are not.
Follow the parent workspace's ROM identity, local artifact, toolkit-reporting and
bounded runtime rules. Do not regenerate over source/ or edit before/.
"""


PROMPT += behavior.GUIDE

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
    def expand(path, active):
        if path in active:
            raise ValueError(f"cyclic assembly include: {path.name}")
        relative = path.relative_to(source)
        result = [f"; ===== BEGIN {relative} =====\n"]
        for line in path.read_text().splitlines(keepends=True):
            match = INCLUDE.match(line.rstrip("\r\n"))
            if match:
                included = local(source, match[1])
                if included.suffix.lower() == ".asm":
                    result.append(expand(included, active | {path}))
                    continue
            result.append(line if line.endswith("\n") else line + "\n")
        result.append(f"; ===== END {relative} =====\n")
        return "".join(result)
    return "; Reading copy; edit the buildable source tree, not this file.\n" + expand(source / "game.asm", set())


def scaffold(state):
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
        row.update(status="pending", treatment="pending", summary="", source=[], units=[])
    return rows


def commands(work):
    return f"python3 {shlex.quote(str(ROOT / 'scripts/commentary.py'))} {{command}} {shlex.quote(str(work))}"


def instructions(work):
    return (f"codex --enable goals -C {shlex.quote(str(work))}\n\n"
            "/goal Read commentary/AGENTS.md, commentary/workflow.md and commentary/prompt.md. "
            "Complete behavioral implementation review and readable assembly commentary in "
            "commentary/source/. Follow the phase-2 completion contract, preserve phase-1 "
            "evidence and inherited obligations, review opaque preservation dispositions, "
            "resolve or bound behavioral dependencies, and finish with an editorial audit, "
            "byte-identical verification and refreshed reading copy.\n\n"
            "For later sessions, read the same prompt and resume commentary/review.json and "
            "commentary/progress.md. Use /goal resume only for an existing paused goal; "
            "do not replace an unfinished goal.\n")


def behavioral_handoff(work, meta):
    inherited = behavior.load(work / "analysis/behavior.json", meta["sha256"], "disassembly")
    notes = work / "analysis/behavior.md"
    queue = work / "analysis/work-queue.json"
    handoff = dict(version=1, behavior=inherited,
                   behavior_notes=notes.read_text() if notes.exists() else "No narrative inventory supplied.",
                   dispositions=[t for t in json.loads(queue.read_text()).get("tickets", [])
                                 if t.get("status") == "deferred"] if queue.exists() else [],
                   limitation="Preparation preserves upstream claims; it does not certify them. Recover missing legacy scope before completion.")
    return handoff


def commentary_workflow(work):
    command = commands(work)
    return """# Commentary workflow

This phase starts from the current phase-1 map in an existing workspace.
before/, baseline.sym and phase1-state.json are frozen comparison artifacts.
source/ is independently maintained, buildable assembly. Never regenerate over it.
The portable .sym carries structural directives and labels, not the final comments.
To reproduce commentary, preserve source/ and its local assets, not just .sym.
Unknown regions may retain evidence-backed preservation treatment without recovering
their historical meaning. handoff.json freezes inherited obligations and dispositions;
behavior.json and refinements.json track current review and structural corrections.
Review is behavioral plus editorial; a physical scaffold is preservation accounting.
See prompt.md for the shared ending contract, including incomplete outcomes.

""" + f"""Commands from any working directory:

```sh
{command.format(command='instructions')}
{command.format(command='verify')}
{command.format(command='read')}
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
scaffold. Each needs status=reviewed, treatment=semantic/preserved, a summary and
nonempty source locations. Preserved entries require preservation rationale/evidence/
reopen_when; known Code must be semantically reviewed. Behavior items need source
locations and regions (semantic scaffold IDs); missing inventory is pending work.
Add units for meaningful routines/loops/branches/shared tails/data structures:
name, offset, length, status, summary and source (list of file:line strings).
Units can overlap, but must lie within the ROM. Split a spanning unit's inventory
records across scaffold entries if useful. Every discovered unit must be reviewed.
Set editorial_audit.status=reviewed and notes to the final editorial findings.
Use uncertainties.md for tentative meanings. Unresolved behavioral dependencies
belong in behavior.json and block completion; refinements must be reconciled.
Keep all source and assets local. Preserve toolkit reports and upstream notices.
"""


def upgrade(work, meta, rom):
    """Back up metadata before migration; never rewrite source or frozen baselines."""
    phase = work / "commentary"
    manifest = json.loads((phase / "manifest.json").read_text())
    if manifest["rom_sha256"] != meta["sha256"] or digest(phase / "phase1-state.json") != manifest["state_sha256"] or digest(phase / "baseline.sym") != manifest["map_sha256"]:
        raise ValueError("frozen commentary baseline changed")
    if manifest.get("handoff_sha256") and not (phase / "handoff.json").is_file():
        raise ValueError("frozen behavioral handoff is missing")
    names = ["manifest.json", "review.json", "handoff.json", "behavior.json", "refinements.json",
             "refinements.md", "prompt.md", "AGENTS.md", "workflow.md", "instructions.txt", "audit.json"]
    if any((phase / name).is_symlink() for name in names):
        raise ValueError("refusing to migrate symlinked commentary metadata")
    backup_root = phase / "setup-backups"
    backup_root.mkdir(exist_ok=True)
    backup = Path(tempfile.mkdtemp(prefix="behavior-", dir=backup_root))
    for name in names:
        if (phase / name).exists():
            shutil.copyfile(phase / name, backup / name)
    try:
        if not (phase / "handoff.json").exists():
            write_json(phase / "handoff.json", behavioral_handoff(work, meta))
        elif manifest.get("handoff_sha256") and digest(phase / "handoff.json") != manifest["handoff_sha256"]:
            raise ValueError("frozen behavioral handoff changed")
        handoff = json.loads((phase / "handoff.json").read_text())
        if not (phase / "behavior.json").exists():
            write_json(phase / "behavior.json", behavior.inherit(handoff["behavior"], "commentary"))
        if not (phase / "refinements.json").exists():
            write_json(phase / "refinements.json", behavior.refinements("commentary"))
        if not (phase / "refinements.md").exists():
            (phase / "refinements.md").write_text("# Structural refinements\n\nRecord new findings and source/map reconciliation here.\n")
        review = json.loads((phase / "review.json").read_text())
        for row in review["regions"]:
            row.setdefault("treatment", "pending")
        review["editorial_audit"]["status"] = "pending"
        write_json(phase / "review.json", review)
        manifest.update(behavior_contract=1, handoff_sha256=digest(phase / "handoff.json"))
        write_json(phase / "manifest.json", manifest)
        (phase / "prompt.md").write_text(PROMPT)
        (phase / "AGENTS.md").write_text(PHASE_AGENTS)
        (phase / "workflow.md").write_text(commentary_workflow(work))
        (phase / "instructions.txt").write_text(instructions(work))
        write_json(phase / "audit.json", dict(ok=False, reason="Behavioral migration requires fresh scope/treatment audit."))
    except Exception:
        for name in names:
            if (backup / name).exists():
                shutil.copyfile(backup / name, phase / name)
            elif (phase / name).exists():
                (phase / name).unlink()
        raise
    return dict(ok=True, backup=str(backup), preserved_source=True, audit_pending=True)


def prepare(work, meta, rom):
    destination = work / "commentary"
    if destination.exists() or destination.is_symlink():
        raise ValueError("commentary already exists; use instructions/read/verify/audit to resume")
    state_path = work / "analysis/state.json"
    state = json.loads(state_path.read_text())
    if state["image_sha256"] != meta["sha256"] or state["image_size"] != rom.stat().st_size:
        raise ValueError("phase-1 state does not match the ROM")
    rows = scaffold(state)
    handoff = behavioral_handoff(work, meta)
    inherited = handoff["behavior"]
    binary = Path(meta["binary"])
    call(binary, "store", "validate", rom, "--store", state_path)
    mapping = work / "maps/current.sym"
    call(binary, "map", "validate", mapping, "--rom", rom)
    with tempfile.TemporaryDirectory(prefix=".commentary-", dir=work) as temporary:
        stage = Path(temporary) / "commentary"
        stage.mkdir()
        shutil.copyfile(mapping, stage / "baseline.sym")
        shutil.copyfile(state_path, stage / "phase1-state.json")
        call(binary, "generate", rom, "--map", stage / "baseline.sym", "--out", stage / "before")
        verified = call(binary, "verify", rom, "--generated", stage / "before")
        if verified.get("byte_identical") is not True:
            raise ValueError("baseline is not byte-identical")
        # Verification build products are reproducible and need not become source assets.
        shutil.copytree(stage / "before", stage / "source",
                        ignore=shutil.ignore_patterns("*.o", "*.gb", "*.gbc", "game.map", "game.sym"))
        write_json(stage / "baseline-verification.json", verified)
        write_json(stage / "handoff.json", handoff)
        write_json(stage / "behavior.json", behavior.inherit(inherited, "commentary"))
        write_json(stage / "refinements.json", behavior.refinements("commentary"))
        (stage / "refinements.md").write_text("# Structural refinements\n\nNo corrections recorded. Preserve frozen baselines; record new findings and reconciliations here.\n")
        write_json(stage / "manifest.json", {"version": 1, "behavior_contract": 1,
                   "handoff_sha256": digest(stage / "handoff.json"), "rom_sha256": meta["sha256"],
                   "map_sha256": digest(stage / "baseline.sym"),
                   "state_sha256": digest(stage / "phase1-state.json"), "image_size": state["image_size"]})
        write_json(stage / "review.json", {"version": 1, "image_size": state["image_size"],
                   "regions": rows, "editorial_audit": {"status": "pending", "notes": ""}})
        (stage / "prompt.md").write_text(PROMPT)
        (stage / "AGENTS.md").write_text(PHASE_AGENTS)
        (stage / "workflow.md").write_text(commentary_workflow(work))
        (stage / "progress.md").write_text("# Commentary progress\n\nPrepared verified baseline. All semantic review entries are pending.\n")
        (stage / "uncertainties.md").write_text("# Commentary uncertainties\n\nCarry forward relevant phase-1 ambiguities as their regions are reviewed.\n")
        (stage / "reading-before.asm").write_text(reading(stage / "before"))
        (stage / "reading-after.asm").write_text(reading(stage / "source"))
        (stage / "instructions.txt").write_text(instructions(work))
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
    # A failed audit must not leave an old success report current.
    write_json(phase / "audit.json", {"ok": False})
    manifest = json.loads((phase / "manifest.json").read_text())
    if manifest["rom_sha256"] != meta["sha256"]:
        raise ValueError("commentary belongs to another ROM")
    if digest(phase / "phase1-state.json") != manifest["state_sha256"] or digest(phase / "baseline.sym") != manifest["map_sha256"]:
        raise ValueError("frozen phase-1 baseline changed")
    review = json.loads((phase / "review.json").read_text())
    expected = scaffold(json.loads((phase / "phase1-state.json").read_text()))
    if review["image_size"] != rom.stat().st_size or len(review["regions"]) != len(expected):
        raise ValueError("review inventory does not cover the baseline")
    if manifest.get("behavior_contract") != 1:
        raise ValueError("legacy commentary needs behavioral handoff migration; preserve source and recover behavior.json before claiming completion")
    if digest(phase / "handoff.json") != manifest["handoff_sha256"]:
        raise ValueError("frozen behavioral handoff changed")
    handoff = json.loads((phase / "handoff.json").read_text())
    units = 0
    def source_location(location):
        filename, line = location.rsplit(":", 1)
        path = local(phase / "source", filename)
        if not path.is_file() or not 1 <= int(line) <= len(path.read_text().splitlines()):
            raise ValueError(f"invalid source location: {location}")

    def reviewed(entry):
        if entry.get("status") != "reviewed" or not entry.get("summary", "").strip() or not entry.get("source"):
            raise ValueError(f"unfinished review: {entry.get('id', entry.get('name', '?'))}")
        for location in entry["source"]:
            source_location(location)
    for entry, original in zip(review["regions"], expected):
        if any(entry.get(key) != original[key] for key in ["id", "offset", "length", "kind"]):
            raise ValueError("review scaffold changed")
        reviewed(entry)
        if entry.get("treatment") == "preserved":
            if entry["kind"] == "Code" or entry.get("units"):
                raise ValueError("known/discovered code and units require semantic review")
            behavior.preservation(entry.get("preservation", {}))
        elif entry.get("treatment") != "semantic":
            raise ValueError("region treatment is pending")
        if entry["kind"] in ("Code", "Unknown") and entry["treatment"] == "semantic" and not entry.get("units"):
            raise ValueError(f"code region lacks reviewed units: {entry['id']}")
        for unit in entry.get("units", []):
            reviewed(unit)
            if not unit.get("name") or unit.get("offset", -1) < 0 or unit.get("length", 0) <= 0 or unit["offset"] + unit["length"] > review["image_size"]:
                raise ValueError("invalid review unit")
            units += 1
    current_behavior = behavior.load(phase / "behavior.json", meta["sha256"], "commentary")
    count = behavior.validate(current_behavior, handoff["behavior"], "commentary", source_location)
    semantic = {r["id"] for r in review["regions"] if r["treatment"] == "semantic"}
    for item in current_behavior["items"]:
        if not behavior.strings(item.get("regions")) or not set(item["regions"]).issubset(semantic):
            raise ValueError("behavior must link to semantically reviewed scaffold regions")
    refinement_count = behavior.validate_refinements(json.loads((phase / "refinements.json").read_text()))
    editorial = review.get("editorial_audit", {})
    if editorial.get("status") != "reviewed" or not editorial.get("notes", "").strip():
        raise ValueError("final editorial audit is pending")
    result = verify(work, meta, rom)
    (phase / "reading-after.asm").write_text(reading(phase / "source"))
    result.update(reviewed_regions=len(expected), reviewed_units=units, editorial_audit=True,
                  behavior_items=count, preserved_regions=sum(r["treatment"] == "preserved" for r in review["regions"]),
                  reconciled_refinements=refinement_count, behavioral_accounting=True,
                  semantic_correctness_assessed=False)
    write_json(phase / "audit.json", result)
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("command", choices=["prepare", "upgrade", "instructions", "read", "verify", "audit"])
    parser.add_argument("workspace", type=Path)
    parser.add_argument("--json", action="store_true", help="suppress printed startup instructions")
    args = parser.parse_args()
    work, meta, rom = workspace(args.workspace)
    if args.command == "prepare":
        result = prepare(work, meta, rom)
        if not args.json:
            print(instructions(work), file=sys.stderr)
    elif args.command == "upgrade":
        result = upgrade(work, meta, rom)
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
