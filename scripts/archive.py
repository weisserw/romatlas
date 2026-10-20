#!/usr/bin/env python3
"""Archive reviewed metadata and restore exact commentary from an owned ROM."""
import argparse
import hashlib
import json
from pathlib import Path
import re
import shutil
import sys
import tempfile

import commentary as phase

ROOT = phase.ROOT
TOKEN = re.compile(r'"(?:\\.|[^"\\])*"|\s+|\$[0-9a-fA-F]+|%[01]+|[0-9]+|[A-Za-z_\.][A-Za-z_0-9\.]*|.', re.S)
IDENTIFIER = re.compile(r'[A-Za-z_\.][A-Za-z_0-9\.]*\Z')
FINAL = [
    "maps/current.sym", "analysis/state.json", "analysis/annotations.json",
    "analysis/current-audit.json", "analysis/scenarios.json",
    "analysis/investigation.md", "analysis/gameplay.md", "analysis/progress.md",
    "commentary/baseline.sym", "commentary/phase1-state.json",
    "commentary/manifest.json", "commentary/review.json", "commentary/audit.json",
    "commentary/verification.json", "commentary/progress.md",
    "commentary/uncertainties.md", "commentary/editorial-audit.md",
]
PRODUCTS = {"game.o", "game.gb", "game.gbc", "romatlas-rebuilt.gb", "game.map", "game.sym", "rebuilt.nes"}


def sha(data):
    return hashlib.sha256(data).hexdigest()


def files(root):
    result = {}
    for path in sorted(root.rglob("*")):
        if path.is_symlink():
            raise ValueError(f"symlinks are unsupported: {path}")
        if path.is_file() and path.name not in PRODUCTS:
            result[path.relative_to(root).as_posix()] = path.read_bytes()
    return result


def split_comment(line):
    quoted = escaped = False
    for index, char in enumerate(line):
        if char == '"' and not escaped:
            quoted = not quoted
        if char == ";" and not quoted:
            return line[:index], line[index:]
        escaped = char == "\\" and not escaped
    return line, ""


def recipe(baseline, final, platform="gb"):
    """Only editorial literals; all preexisting code/data comes from references."""
    tokens, lines = {}, {}
    texts = {}
    for name, data in baseline.items():
        if Path(name).suffix not in {".asm", ".inc"}:
            continue
        text = data.decode("utf-8")
        texts[name] = text
        position = 0
        for line in text.splitlines(keepends=True):
            lines.setdefault(line, [name, position, len(line)])
            position += len(line)
        matches = list(TOKEN.finditer(text))
        for index, match in enumerate(matches):
            tokens.setdefault(match.group(), [name, match.start(), len(match.group())])
            for width in (2, 4, 8, 12):
                if index + width <= len(matches):
                    end = matches[index + width - 1].end()
                    tokens.setdefault(text[match.start():end], [name, match.start(), end - match.start()])
    output = {}
    for name, data in final.items():
        if baseline.get(name) == data:
            output[name] = {"sha256": sha(data), "baseline": name}
            continue
        if Path(name).suffix not in {".asm", ".inc"}:
            raise ValueError(f"changed non-assembly asset cannot be archived: {name}")
        operations = []
        def copy(reference):
            if operations and "copy" in operations[-1]:
                previous = operations[-1]["copy"]
                if previous[0] == reference[0] and previous[1] + previous[2] == reference[1]:
                    previous[2] += reference[2]
                    return
            operations.append({"copy": list(reference)})
        for line in data.decode("utf-8").splitlines(keepends=True):
            if line in lines:
                copy(lines[line])
                continue
            code, comment = split_comment(line)
            constant = bool(re.match(r"\s*(?:DEF\s+)?[\w.]+\s+(?:EQU|SET)\b", code, re.I))
            if platform in {"snes", "nes", "sms"}:
                constant = constant or bool(re.match(r"\s*!?[\w.]+\s*=", code))
            include = bool(re.match(r"\s*(?:\.?INCLUDE|\.?INCBIN|INCSRC)\b" if platform in {"snes", "nes", "sms"} else r"\s*(?:\.?INCLUDE|\.?INCBIN)\b", code, re.I))
            matches = list(TOKEN.finditer(code))
            index = 0
            while index < len(matches):
                match = matches[index]
                width = next((width for width in (12, 8, 4, 2)
                              if index + width <= len(matches)
                              and code[match.start():matches[index + width - 1].end()] in tokens), 1)
                token = code[match.start():matches[index + width - 1].end()]
                index += width
                if token in tokens:
                    copy(tokens[token])
                elif token.isspace():
                    operations.append({"format": token})
                elif platform in {"snes", "nes", "sms"} and re.fullmatch(r'[^\w\s"]', token):
                    operations.append({"format": token})
                elif IDENTIFIER.fullmatch(token):
                    operations.append({"symbol": token})
                elif constant and re.fullmatch(r"\$[\da-fA-F]+|%[01]+|\d+", token):
                    operations.append({"constant": token})
                elif include and token.startswith('"'):
                    phase.local(Path("/archive-source"), token[1:-1])
                    operations.append({"include": token})
                else:
                    # Punctuation and existing operands can reference a substring
                    # even when token boundaries differ (e.g. $ff vs $ffff).
                    reference = next(([key, text.find(token), len(token)]
                                      for key, text in texts.items() if token in text), None)
                    if reference is None:
                        # A formatter may change numeric spelling ($100 -> $0100)
                        # or operators without changing bytes. Store no new data
                        # literals: spell those tokens with baseline character refs.
                        if not re.fullmatch(r"\$[\da-fA-F]+|%[01]+|\d+|[^\w\s\"]", token):
                            raise ValueError(f"unrepresentable source edit in {name}: {token!r}")
                        for char in token:
                            character = next(([key, text.find(char), 1]
                                              for key, text in texts.items() if char in text), None)
                            if character is None:
                                raise ValueError(f"unrepresentable formatting in {name}: {char!r}")
                            copy(character)
                    else:
                        copy(reference)
            if comment:
                operations.append({"comment": comment})
        output[name] = {"sha256": sha(data), "operations": operations}
    return {"version": 1, "baseline_sha256": {name: sha(data) for name, data in baseline.items()},
            "files": output}


def render(plan, baseline, output):
    if plan["version"] != 1:
        raise ValueError("unsupported reconstruction version")
    if {name: sha(data) for name, data in baseline.items()} != plan["baseline_sha256"]:
        raise ValueError("baseline differs; use the documented pinned tools and original ROM filename")
    for name, entry in plan["files"].items():
        destination = phase.local(output, name)
        if "baseline" in entry:
            data = baseline[entry["baseline"]]
        else:
            pieces = []
            for operation in entry["operations"]:
                if len(operation) != 1:
                    raise ValueError("invalid reconstruction operation")
                kind, value = next(iter(operation.items()))
                if kind == "copy":
                    source, start, length = value
                    text = baseline[source].decode("utf-8")
                    if start < 0 or length <= 0 or start + length > len(text):
                        raise ValueError("invalid baseline reference")
                    pieces.append(text[start:start + length])
                elif kind in {"format", "symbol", "constant", "include", "comment"}:
                    if not isinstance(value, str):
                        raise ValueError("invalid editorial metadata")
                    pieces.append(value)
                else:
                    raise ValueError("unknown reconstruction operation")
            data = "".join(pieces).encode("utf-8")
        if sha(data) != entry["sha256"]:
            raise ValueError(f"restored source hash mismatch: {name}")
        destination.parent.mkdir(parents=True, exist_ok=True)
        destination.write_bytes(data)


def write_recipe(path, plan):
    """Keep each reference/edit on one line for compact, reviewable diffs."""
    header = json.dumps({key: value for key, value in plan.items() if key != "files"}, indent=2)
    entries = []
    for name, entry in plan["files"].items():
        if "operations" not in entry:
            entries.append("    " + json.dumps(name) + ": " + json.dumps(entry))
        else:
            edits = ",\n".join("        " + json.dumps(edit) for edit in entry["operations"])
            entries.append("    " + json.dumps(name) + ": {\n      \"sha256\": "
                           + json.dumps(entry["sha256"]) + ",\n      \"operations\": [\n"
                           + edits + "\n      ]\n    }")
    path.write_text(header[:-2] + ',\n  "files": {\n' + ",\n".join(entries) + "\n  }\n}\n")


def rebuild(archive, rom, output, binary):
    manifest = json.loads((archive / "archive.json").read_text())
    if manifest["version"] != 1 or phase.digest(rom) != manifest["rom_sha256"]:
        raise ValueError("archive version or ROM identity mismatch")
    for name, expected in manifest["artifacts"].items():
        if phase.digest(phase.local(archive, name)) != expected:
            raise ValueError(f"archive artifact changed: {name}")
    with tempfile.TemporaryDirectory(prefix="romatlas-restore-") as temporary:
        base = Path(temporary)
        supplied = base / manifest["rom_filename"]
        if supplied.parent != base:
            raise ValueError("invalid original ROM filename")
        shutil.copyfile(rom, supplied)
        before = base / "baseline"
        phase.call(binary, "generate", supplied, "--map", archive / ("commentary/baseline.json" if manifest["platform"] in {"snes", "nes", "sms"} else "commentary/baseline.sym"), "--out", before)
        render(json.loads((archive / "editorial.json").read_text()), files(before), output)
        phase.reading(output)  # Validate include traversal before invoking the assembler.
        for path in output.rglob("*"):
            if path.suffix in {".asm", ".inc"}:
                for line in path.read_text().splitlines():
                    code, _ = split_comment(line)
                    match = re.match(r'\s*(?:\.?INCLUDE|\.?INCBIN)\s+"([^"]+)"', code, re.I)
                    if match:
                        phase.local(output, match[1])
        result = phase.call(binary, "verify", supplied, "--generated", output)
        if result.get("byte_identical") is not True:
            raise ValueError("restored source is not byte-identical")
    return result


def replace(stage, destination):
    """Rollback on failed publication; the staged archive is already validated."""
    backup = stage.parent / "previous"
    if destination.exists():
        destination.rename(backup)
    try:
        stage.rename(destination)
    except OSError:
        if backup.exists():
            backup.rename(destination)
        raise


def editorial_status(work, meta, rom):
    """Report editorial completion without making it a prerequisite for archiving."""
    phase_dir = work / "commentary"
    manifest = json.loads((phase_dir / "manifest.json").read_text())
    if manifest["rom_sha256"] != meta["sha256"]:
        raise ValueError("commentary belongs to another ROM")
    state_path = phase_dir / "phase1-state.json"
    map_path = phase_dir / ("baseline.json" if meta.get("platform") in {"snes", "nes", "sms"} else "baseline.sym")
    if phase.digest(state_path) != manifest["state_sha256"] or phase.digest(map_path) != manifest["map_sha256"]:
        raise ValueError("frozen phase-1 baseline changed")
    review = json.loads((phase_dir / "review.json").read_text())
    expected = phase.scaffold(json.loads(state_path.read_text()))
    if review.get("image_size") != rom.stat().st_size or len(review.get("regions", [])) != len(expected):
        raise ValueError("review inventory does not cover the baseline")

    pending_regions = pending_units = regions_without_units = 0
    reviewed_regions = reviewed_units = 0

    def entry_complete(entry):
        if entry.get("status") != "reviewed" or not entry.get("summary", "").strip() or not entry.get("source"):
            return False
        try:
            for location in entry["source"]:
                filename, line = location.rsplit(":", 1)
                source = phase.local(phase_dir / "source", filename)
                if not source.is_file() or not 1 <= int(line) <= len(source.read_text().splitlines()):
                    return False
        except (ValueError, OSError):
            return False
        return True

    for entry, original in zip(review["regions"], expected):
        if any(entry.get(key) != original[key] for key in ["id", "offset", "length", "kind"]):
            raise ValueError("review scaffold changed")
        region_ok = entry_complete(entry)
        reviewed_regions += int(region_ok)
        if not region_ok:
            pending_regions += 1
        units = entry.get("units", [])
        if entry["kind"] == "Code" and not units:
            regions_without_units += 1
        for unit in units:
            valid_range = (type(unit.get("offset")) is int and type(unit.get("length")) is int
                           and unit["offset"] >= 0 and unit["length"] > 0
                           and unit["offset"] + unit["length"] <= rom.stat().st_size)
            unit_ok = entry_complete(unit) and bool(unit.get("name")) and valid_range
            reviewed_units += int(unit_ok)
            if not unit_ok:
                pending_units += 1
    editorial = review.get("editorial_audit", {})
    editorial_ok = editorial.get("status") == "reviewed" and bool(editorial.get("notes", "").strip())
    reasons = []
    if pending_regions:
        reasons.append(f"{pending_regions} regions are pending or have unresolved source locations")
    if pending_units:
        reasons.append(f"{pending_units} units are pending or invalid")
    if regions_without_units:
        reasons.append(f"{regions_without_units} code regions have no reviewed units")
    if not editorial_ok:
        reasons.append("final editorial audit is pending")
    return {
        "complete": not reasons,
        "reviewed_regions": reviewed_regions,
        "total_regions": len(expected),
        "reviewed_units": reviewed_units,
        "total_units": reviewed_units + pending_units,
        "editorial_audit": editorial_ok,
        "reasons": reasons,
    }


def archive(workspace, destination, name, binary, documents=(), omitted_documents=()):
    work, meta, rom = phase.workspace(workspace)
    binary = binary or Path(meta["binary"])
    meta = dict(meta, binary=str(binary))
    review_status = editorial_status(work, meta, rom)
    phase.verify(work, meta, rom)
    (work / "commentary/reading-after.asm").write_text(phase.reading(work / "commentary/source"))
    phase.call(binary, "store", "validate", rom, "--store", work / "analysis/state.json")
    is_snes = meta.get("platform") in {"snes", "nes", "sms"}
    phase.call(binary, "map", "validate", work / ("maps/current.json" if is_snes else "maps/current.sym"), "--rom", rom)
    state = json.loads((work / "analysis/state.json").read_text())
    platform = meta["platform"] if is_snes else state["platform"]
    # The current adapter identifies itself as gb; the archive hierarchy stays
    # platform-qualified rather than baking this into the reconstruction format.
    if platform not in {"gb", "snes", "nes", "sms"}:
        raise ValueError(f"unsupported archive platform: {platform}")
    slug = name or work.name
    if not re.fullmatch(r"[a-z0-9]+(?:-[a-z0-9]+)*", slug):
        raise ValueError("ROM name must be a lowercase hyphenated slug; use --name")
    destination = (destination or ROOT / "atlas" / platform / slug / meta["sha256"]).absolute()
    if destination.is_symlink() or (destination.exists() and not destination.is_dir()):
        raise ValueError("archive destination must be a directory, not a symlink")
    resolved = destination.resolve()
    if resolved == work or resolved in work.parents or work in resolved.parents or resolved == ROOT or resolved in ROOT.parents:
        raise ValueError("archive destination must not overlap the workspace or replace the repository")
    if destination.exists() and not (destination / "archive.json").is_file():
        raise ValueError("refusing to replace a directory that is not an archive")
    destination.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(prefix=".archive-", dir=destination.parent) as temporary:
        base = Path(temporary)
        stage = base / "archive"
        stage.mkdir()
        included = []
        selection = [name.replace(".sym", ".json") if is_snes else name for name in FINAL]
        for document in documents:
            if document == "analysis/toolkit-issues.md" or Path(document).suffix != ".md" or Path(document).parts[0] not in {"analysis", "commentary"} or ".." in Path(document).parts:
                raise ValueError("--document accepts final analysis/commentary Markdown, excluding toolkit issues")
            phase.local(work, document)
            if not (work / document).is_file():
                raise ValueError(f"additional final document not found: {document}")
            selection.append(document)
        for name in selection:
            source = phase.local(work, name)
            if source.is_file():
                target = phase.local(stage, name)
                target.parent.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(source, target)
                included.append(name)
        # Additional final documents are explicit, rather than guessed from scratch filenames.
        for document in omitted_documents:
            phase.local(work, document)
            if document in selection:
                raise ValueError(f"cannot omit a selected final artifact: {document}")
        known_docs = set(selection) | set(omitted_documents) | {"analysis/toolkit-issues.md", "commentary/prompt.md", "commentary/workflow.md", "commentary/AGENTS.md"}
        unselected = [p.relative_to(work).as_posix() for folder in ["analysis", "commentary"]
                      for p in (work / folder).rglob("*.md")
                      if p.relative_to(work).as_posix() not in known_docs
                      and not any(part in {"before", "source"} for part in p.relative_to(work).parts)]
        if unselected:
            raise ValueError(f"unrecognized documents; use --document for final documents or --omit-document for scratch: {unselected}")
        before = base / "baseline"
        phase.call(binary, "generate", rom, "--map", stage / ("commentary/baseline.json" if is_snes else "commentary/baseline.sym"), "--out", before)
        final = files(work / "commentary/source")
        write_recipe(stage / "editorial.json", recipe(files(before), final, platform))
        (stage / "README.md").write_text("# Archived ROM analysis\n\n"
            "Final maps, review metadata and agent documents are retained verbatim.\n"
            "Phase-one classification and phase-two editorial completion are separate;\n"
            "see their audit records for remaining unknowns. Scratch evidence, toolkit\n"
            "issue reports, ROMs, generated assembly and extracted assets are omitted.\n"
            "References to omitted scratch files in historical documents are unavailable.\n\n"
            "editorial.json stores editorial metadata and baseline references, not a\n"
            "standalone assembly listing. Restore with the original ROM and pinned tools:\n\n"
            "```sh\npython3 scripts/archive.py restore PATH_TO_THIS_DIRECTORY --rom ORIGINAL_ROM --out /tmp/restored-game\n```\n")
        phase.write_json(stage / "archive.json", {"version": 1, "platform": platform,
            "name": slug, "rom_sha256": meta["sha256"], "rom_filename": rom.name,
            "image_size": rom.stat().st_size, "editorial_complete": review_status["complete"],
            "review_status": review_status,
            "omitted_documents": list(omitted_documents),
            "artifacts": {p.relative_to(stage).as_posix(): phase.digest(p)
                          for p in sorted(stage.rglob("*")) if p.is_file()}})
        restored = base / "restored"
        restored.mkdir()
        rebuild(stage, rom, restored, binary)
        if files(restored) != final:
            raise ValueError("restoration differs from the authoritative final source tree")
        replace(stage, destination)
    return {"ok": True, "archive": str(destination), "byte_identical": True,
            "exact_source": True, "editorial_complete": review_status["complete"],
            "review_status": review_status, "documents": included}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest="command", required=True)
    save = sub.add_parser("archive", help="validate and replace a metadata archive")
    save.add_argument("workspace", type=Path)
    save.add_argument("--output", type=Path)
    save.add_argument("--name")
    save.add_argument("--binary", type=Path)
    save.add_argument("--document", action="append", default=[], help="additional final Markdown document relative to workspace")
    save.add_argument("--omit-document", action="append", default=[], help="explicitly classify an unrecognized document as scratch")
    restore = sub.add_parser("restore", help="recover exact source into a new local directory")
    restore.add_argument("archive", type=Path)
    restore.add_argument("--rom", type=Path, required=True)
    restore.add_argument("--out", type=Path, required=True)
    restore.add_argument("--binary", type=Path, default=ROOT / "target/release/romatlas")
    args = parser.parse_args()
    if args.command == "archive":
        result = archive(args.workspace, args.output, args.name, args.binary, args.document, args.omit_document)
    else:
        output = args.out.absolute()
        if output.exists() or output.is_symlink():
            raise ValueError("restore output already exists; choose a new directory")
        if output.resolve() == ROOT or ROOT in output.resolve().parents:
            raise ValueError("restored source and assets must remain outside the repository")
        output.parent.mkdir(parents=True, exist_ok=True)
        with tempfile.TemporaryDirectory(prefix=".restore-", dir=output.parent) as temporary:
            stage = Path(temporary) / "restored"
            source = stage / "source"
            source.mkdir(parents=True)
            result = rebuild(args.archive.resolve(strict=True), args.rom.resolve(strict=True), source, args.binary)
            # The reading copy is a local convenience alongside the buildable tree.
            (stage / "reading-after.asm").write_text(phase.reading(source))
            stage.rename(output)
        result.update(ok=True, exact_source=True, source=str(output / "source"),
                      reading_after=str(output / "reading-after.asm"))
    print(json.dumps(result))


if __name__ == "__main__":
    try:
        main()
    except (OSError, ValueError, KeyError, TypeError, phase.subprocess.SubprocessError) as error:
        print(json.dumps({"ok": False, "error": str(error)}), file=sys.stderr)
        sys.exit(1)
