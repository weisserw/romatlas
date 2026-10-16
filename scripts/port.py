#!/usr/bin/env python3
"""Prepare and maintain native Rust port workspaces from reviewed archives."""
import argparse
import json
import os
from pathlib import Path
import re
import shlex
import shutil
import subprocess
import sys
import tempfile

import archive as archived
import commentary as phase

ROOT = phase.ROOT
TEMPLATES = ROOT / "scripts/port-template"
DISPOSITIONS = {"implemented", "merged", "replaced", "excluded"}
GOAL = ("Port this entire archived game into readable native Rust, following AGENTS.md, "
        "workflow.md and prompts/start.md. Replace the demonstration scaffold. Account for "
        "all source behavior in ledger.json; implement every gameplay-relevant equivalent, "
        "favor ambiguous cases, and explain obvious exclusions. Load original assets from "
        "the user's ROM at startup, support replacements and added slots, provide player "
        "settings, and write useful headless tests. Implement necessary shared-library "
        "changes directly in ROMAtlas, preserve unrelated work, leave edits uncommitted "
        "and log requirements, rationale and validation for a separate reviewer. Continue "
        "through the whole game, not one playable checkpoint. Finish the ledger, checks "
        "and handoff; adapt BUILDING.md with platform build instructions and validation, "
        "and report untested desktop behavior. If blocked, preserve work and "
        "explain concrete unresolved behaviors and what would unblock them; never mark "
        "an incomplete port complete.")
RESUME = ("Read progress.md, ledger.json, differences.md, library-changes.md and existing "
          "code first. Preserve work; do not reinitialize. Resume the same objective "
          "and continue unresolved behavior and library work using prompts/start.md.")


def git(*args):
    return subprocess.check_output(["git", "-C", str(ROOT), *args], stderr=subprocess.PIPE)


def snapshot(destination):
    names = set(git("ls-files", "-z").decode().split("\0"))
    names.update(git("ls-files", "--others", "--exclude-standard", "-z").decode().split("\0"))
    inventory = {}
    for name in sorted(names - {""}):
        path = phase.local(ROOT, name)
        if path.is_file():
            target = destination / name
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(path, target)
            inventory[name] = phase.digest(target)
    return {"version": 1, "head": git("rev-parse", "HEAD").decode().strip(),
            "status": git("status", "--porcelain=v1").decode(), "files": inventory}


def records(review):
    rows = []
    for row in review["regions"]:
        entry = {"id": row["id"], "offset": row["offset"], "length": row["length"],
                 "kind": row["kind"], "original_summary": row["summary"],
                 "original_source": row["source"], "disposition": "pending", "summary": "",
                 "equivalents": [], "units": []}
        for index, unit in enumerate(row.get("units", [])):
            entry["units"].append({"id": f"{row['id']}/unit-{index}", "name": unit["name"],
                "offset": unit["offset"], "length": unit["length"],
                "original_summary": unit["summary"], "original_source": unit["source"],
                "disposition": "pending", "summary": "", "equivalents": []})
        rows.append(entry)
    return rows


def instructions(work):
    return (f"Read {work / 'goal.md'} to start or resume the port.\n"
            f"For a separate library reviewer, read {work / 'prompts/review.md'}.\n")


def prepare(archive, rom, output, binary):
    archive = archive.resolve(strict=True)
    rom = rom.resolve(strict=True)
    output = output.absolute()
    binary = binary.resolve(strict=True)
    if output.exists() or output.is_symlink():
        raise ValueError("port workspace already exists; choose a new directory or resume it")
    if output.resolve() == ROOT or ROOT in output.resolve().parents:
        raise ValueError("port workspaces and generated assets must remain outside the repository")
    manifest = json.loads((archive / "archive.json").read_text())
    if manifest.get("version") != 1 or manifest.get("editorial_complete") is not True:
        raise ValueError("port preparation requires a version-1 reviewed archive")
    if manifest["platform"] != "gb":
        raise ValueError("initial port scaffolds support Game Boy archives only")
    name = manifest["name"]
    if not re.fullmatch(r"[a-z0-9]+(?:-[a-z0-9]+)*", name):
        raise ValueError("invalid archived game name")
    output.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(prefix=".port-", dir=output.parent) as temporary:
        stage = Path(temporary) / "workspace"
        for directory in ["input", "reference", "port/src", "prompts", "review", "checks", "examples"]:
            (stage / directory).mkdir(parents=True, exist_ok=True)
        supplied = stage / "input/game.rom"
        shutil.copyfile(rom, supplied)
        if supplied.stat().st_size != manifest["image_size"]:
            raise ValueError("archive ROM size mismatch")
        # Snapshot metadata before rebuilding, so subsequent archive edits cannot
        # change the source or ledger behind this workspace.
        reference_archive = stage / "reference/archive"
        reference_archive.mkdir()
        shutil.copyfile(archive / "archive.json", reference_archive / "archive.json")
        for relative, expected in manifest["artifacts"].items():
            source = phase.local(archive, relative)
            if phase.digest(source) != expected:
                raise ValueError(f"archive artifact changed: {relative}")
            target = phase.local(reference_archive, relative)
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(source, target)
        source = stage / "reference/source"
        source.mkdir()
        verification = archived.rebuild(reference_archive, supplied, source, binary)
        for path in source.rglob("*"):
            if path.is_file() and path.name in archived.PRODUCTS:
                path.unlink()
        (stage / "reference/reading.asm").write_text(phase.reading(source))
        review = json.loads((reference_archive / "commentary/review.json").read_text())
        if review.get("version") != 1 or review.get("image_size") != manifest["image_size"] or review.get("editorial_audit", {}).get("status") != "reviewed":
            raise ValueError("archive editorial review is incomplete or inconsistent")
        cursor = 0
        identities = set()
        for row in review["regions"]:
            if type(row["offset"]) is not int or row["offset"] != cursor or type(row["length"]) is not int or row["length"] <= 0 or row["id"] in identities or row["status"] != "reviewed":
                raise ValueError("archive reviewed regions must partition the ROM")
            cursor += row["length"]
            identities.add(row["id"])
            for unit in row.get("units", []):
                if unit["status"] != "reviewed" or type(unit["offset"]) is not int or type(unit["length"]) is not int or unit["offset"] < 0 or unit["length"] <= 0 or unit["offset"] + unit["length"] > manifest["image_size"]:
                    raise ValueError("archive unit review is incomplete or outside the ROM")
        if cursor != manifest["image_size"]:
            raise ValueError("archive reviewed regions do not cover the whole ROM")
        phase.write_json(stage / "ledger.json", {"version": 1, "rom_sha256": manifest["rom_sha256"],
            "regions": records(review), "completion": {"status": "pending", "notes": ""}})
        before = snapshot(stage / "review/toolkit-before")
        phase.write_json(stage / "review/toolkit-before.json", before)
        cargo = (f"[package]\nname = {json.dumps(name + '-port')}\nversion = \"0.1.0\"\nedition = \"2024\"\nrust-version = \"1.90\"\n\n"
                 "[workspace]\n\n[features]\ndefault = []\ndesktop = [\"romatlas-port/desktop\"]\n\n[dependencies]\n"
                 f"romatlas-port = {{ path = {json.dumps(str(ROOT / 'crates/romatlas-port'))} }}\n"
                 "anyhow = \"1.0\"\nserde_json = \"1.0\"\n")
        (stage / "port/Cargo.toml").write_text(cargo)
        (stage / "port/rust-toolchain.toml").write_text('[toolchain]\nchannel = "1.90.0"\nprofile = "minimal"\ncomponents = ["rustfmt", "clippy"]\n')
        for filename in ["main.rs", "game.rs"]:
            text = (TEMPLATES / filename).read_text().replace("__ROM_SHA256__", manifest["rom_sha256"]).replace("__GAME_NAME__", name)
            (stage / "port/src" / filename).write_text(text)
        # Pin dependencies before publication. Toolkit setup/build must have
        # resolved them; preparation does not require an online registry.
        result = subprocess.run(["cargo", "generate-lockfile", "--offline", "--manifest-path", str(stage / "port/Cargo.toml")], capture_output=True, text=True)
        if result.returncode:
            raise ValueError("cannot lock scaffold dependencies; build the toolkit first:\n" + result.stderr)
        subprocess.run(["cargo", "fmt", "--manifest-path", str(stage / "port/Cargo.toml")], check=True, capture_output=True, text=True)
        prompt = (TEMPLATES / "prompt.md").read_text()
        (stage / "prompts/start.md").write_text(prompt)
        (stage / "goal.md").write_text("# Native port\n\n" + GOAL + "\n\nOn resume: " + RESUME + "\n")
        shutil.copyfile(TEMPLATES / "review.md", stage / "prompts/review.md")
        build_guide = (TEMPLATES / "BUILDING.md").read_text().replace("__GAME_NAME__", name).replace("__TOOLKIT_PATH__", str(ROOT))
        (stage / "BUILDING.md").write_text(build_guide)
        (stage / "AGENTS.md").write_text(f"""# Native port workspace

Read goal.md, workflow.md and prompts/start.md. The whole-game
completion contract applies; the demonstration scaffold is not a completed port.
Toolkit: {json.dumps(str(ROOT))}. Use its AGENTS.md for shared-library changes.
reference/ and review/toolkit-before/ are frozen. Change port/, ledger.json and
port documentation; implement/log needed shared-library edits directly in the
toolkit, uncommitted. Do not overwrite pre-existing changes. Never download or
commit commercial ROMs or extracted assets. Keep generated material local.
Use Rust 1.90.0 and Cargo.lock. Do not launch desktop graphics on this server.
Run python3 run.py check; run python3 run.py audit only after finishing the ledger.
No agent launch, check-in or publication is authorized by this workspace.
""")
        (stage / "workflow.md").write_text("""# Port workflow

This is a new phase from a reviewed archive. Reference source was restored and
rebuilt byte-identically. That proves source recovery, not gameplay equivalence.
The Rust scaffold displays an arbitrary tile; no gameplay has been implemented.

Commands from this directory:

```sh
python3 run.py instructions
python3 run.py check
python3 run.py headless --frames 120
python3 run.py desktop
python3 run.py audit
```

desktop is an explicit human command requiring a graphical session. Do not run
it on this headless server. check builds desktop code and tests without a display.
All build/test output is retained in checks/latest.log; results are JSON.

ledger.json preserves every original region and unit ID/offset/length. Set each
disposition to implemented, merged, replaced or excluded, with a summary and
equivalents (locations or named port/framework mechanisms). Exclusions need an
explanation but no equivalent. Add discovered units with unique IDs, source
offset/length, disposition, summary and equivalents. Whole-game completion requires
accounting for all source behavior, not a one-to-one translation or execution proof.
Favor porting unclear cases. Set completion.status=complete and notes at completion.
Audit checks accounting and builds, not semantic correctness. Unknown classification
is allowed at preparation; relevant unknown behavior must be resolved for porting.

Use progress.md for progress/blockers, differences.md for known behavior differences
and untested platform/device behavior, and library-changes.md for shared-library
requirements, actual changes, interface rationale, tests and unresolved findings.
BUILDING.md contains baseline platform recipes. Adapt it to the finished port's
dependencies/executable/flags, link it from README.md and record actual validation.
The initial repository files are copied to review/toolkit-before/, with hashes,
HEAD and pre-existing status in review/toolkit-before.json. Another agent reviews
the actual changes using prompts/review.md before the user checks them in.

The local library path dependency is editable and not frozen by Cargo.lock.
Reproducible release pinning/packaging comes after review/check-in in a later step.
All ROMs, restored source/assets and generated build products remain local.
""")
        (stage / "README.md").write_text(f"""# {name}: native Rust port workspace

Status: scaffold only. Implement the entire game before claiming completion.
See [BUILDING.md](BUILDING.md) for local macOS/Linux/Windows source builds and
transfer instructions. Adapt that guide for the finished port and tested facts.
Run `python3 run.py headless --frames 120` on this server. On a desktop with SDL2
installed, `python3 run.py desktop` opens the window; F1 opens saved player settings.
The executable accepts --rom, --assets, --settings and --headless (frame count).
After implementing the real collections, document replacement formats, adding a
level and game-rule source locations here. See examples/asset-extension.json for
an illustrative manifest: original ROM levels plus a replacement and added slot.
It is documentation, not a working game-level loader. Generated example content
must be developer-authored; do not distribute extracted originals.
""")
        phase.write_json(stage / "examples/asset-extension.json", {"version": 1,
            "entries": [{"id": "level-01", "file": "levels/replacement-01.json"},
                        {"id": "level-16", "file": "levels/new-16.json"}], "order": []})
        (stage / "library-changes.md").write_text("# Library changes\n\nNo port-driven changes recorded yet. For each change record requirement, files, reusable contract, validation and unresolved findings; explicitly report no findings if none.\n")
        (stage / "progress.md").write_text("# Port progress\n\nPrepared reference and demonstration scaffold. Every source-ledger entry is pending. No gameplay is ported.\n")
        (stage / "differences.md").write_text("# Differences and validation limits\n\nScaffold implements no gameplay or original audio. Desktop rendering/playback/controllers and other operating systems are untested. Replace this with specific findings as work proceeds.\n")
        (stage / "run.py").write_text(f"""#!/usr/bin/env python3
import runpy, sys
from pathlib import Path
toolkit = Path({str(ROOT)!r})
workspace = Path(__file__).resolve().parent
sys.path.insert(0, str(toolkit / "scripts"))
if len(sys.argv) < 2:
    sys.argv.append("instructions")
sys.argv = [str(toolkit / "scripts/port.py"), sys.argv[1], str(workspace), *sys.argv[2:]]
runpy.run_path(str(toolkit / "scripts/port.py"), run_name="__main__")
""")
        (stage / ".gitignore").write_text("/input/\n/reference/\n/review/toolkit-before/\n/checks/\n/port/target/\n*.gb\n*.gbc\n*.rom\n__pycache__/\n")
        frozen = {str(p.relative_to(stage)): phase.digest(p) for p in (stage / "reference").rglob("*") if p.is_file()}
        frozen.update({str(p.relative_to(stage)): phase.digest(p) for p in (stage / "review/toolkit-before").rglob("*") if p.is_file()})
        frozen["review/toolkit-before.json"] = phase.digest(stage / "review/toolkit-before.json")
        phase.write_json(stage / "workspace.json", {"version": 1, "name": name,
            "toolkit": str(ROOT), "rom": "input/game.rom", "sha256": manifest["rom_sha256"],
            "archive": str(archive), "archive_sha256": phase.digest(archive / "archive.json"),
            "frozen": frozen, "reference_verification": verification})
        # Reserve the new name before publication; never replace another workspace.
        output.mkdir()
        try:
            stage.rename(output)
        except OSError:
            output.rmdir()
            raise
    return {"ok": True, "workspace": str(output), "sha256": manifest["rom_sha256"],
            "byte_identical_reference": True, "scaffold": True,
            "start_prompt": str(output / "prompts/start.md"), "review_prompt": str(output / "prompts/review.md")}


def workspace(path):
    work = path.resolve(strict=True)
    meta = json.loads((work / "workspace.json").read_text())
    if meta["version"] != 1 or Path(meta["toolkit"]).resolve() != ROOT:
        raise ValueError("unsupported port workspace/toolkit")
    rom = phase.local(work, meta["rom"])
    if phase.digest(rom) != meta["sha256"]:
        raise ValueError("workspace ROM identity changed")
    for name, digest in meta["frozen"].items():
        if phase.digest(phase.local(work, name)) != digest:
            raise ValueError(f"frozen reference changed: {name}")
    return work, meta, rom


def validate_ledger(work, meta):
    ledger = json.loads((work / "ledger.json").read_text())
    if ledger["version"] != 1 or ledger["rom_sha256"] != meta["sha256"]:
        raise ValueError("ledger identity/version mismatch")
    expected = records(json.loads((work / "reference/archive/commentary/review.json").read_text()))
    actual = ledger["regions"]
    if len(actual) != len(expected):
        raise ValueError("source ledger region inventory changed")
    seen = set()
    def validate(entry):
        identity = entry["id"]
        if not isinstance(identity, str) or not identity or identity in seen:
            raise ValueError("missing or duplicate source ledger ID")
        seen.add(identity)
        if entry["disposition"] not in DISPOSITIONS or not isinstance(entry["summary"], str) or not entry["summary"].strip():
            raise ValueError(f"unaccounted source behavior: {identity}")
        equivalents = entry["equivalents"]
        if not isinstance(equivalents, list) or any(not isinstance(e, str) or not e.strip() for e in equivalents):
            raise ValueError("equivalents must be nonempty strings")
        if entry["disposition"] != "excluded" and not equivalents:
            raise ValueError(f"missing port equivalent: {identity}")
        if type(entry["offset"]) is not int or type(entry["length"]) is not int or entry["offset"] < 0 or entry["length"] <= 0 or entry["offset"] + entry["length"] > phase.local(work, meta["rom"]).stat().st_size:
            raise ValueError("source ledger range outside ROM")
    for original, entry in zip(expected, actual):
        if any(entry[key] != original[key] for key in ["id", "offset", "length", "kind", "original_source", "original_summary"]):
            raise ValueError("original source region identity changed")
        validate(entry)
        originals = {u["id"]: u for u in original["units"]}
        for unit in entry["units"]:
            validate(unit)
            if unit["id"] in originals:
                old = originals.pop(unit["id"])
                if any(unit[key] != old[key] for key in ["offset", "length", "name", "original_source", "original_summary"]):
                    raise ValueError("original source unit identity changed")
        if originals:
            raise ValueError("original source units were removed")
    completion = ledger["completion"]
    if completion["status"] != "complete" or not isinstance(completion["notes"], str) or not completion["notes"].strip():
        raise ValueError("port completion is pending")
    return {"regions": len(actual), "units": len(seen) - len(actual)}


def check(work):
    cargo = ["cargo", "--manifest-path", str(work / "port/Cargo.toml")]
    checks = [[cargo[0], "fmt", *cargo[1:], "--check"],
              [cargo[0], "clippy", *cargo[1:], "--all-targets", "--features", "desktop", "--locked", "--", "-D", "warnings"],
              [cargo[0], "test", *cargo[1:], "--locked"],
              [cargo[0], "build", *cargo[1:], "--features", "desktop", "--locked"]]
    results = []
    report = work / "checks/latest.json"
    phase.write_json(report, {"ok": False, "checks": []})
    with (work / "checks/latest.log").open("w") as log:
        for command in checks:
            log.write(shlex.join(command) + "\n")
            log.flush()
            result = subprocess.run(command, cwd=work / "port", stdout=log, stderr=log)
            results.append({"command": command, "ok": result.returncode == 0})
            phase.write_json(report, {"ok": False, "checks": results})
            if result.returncode:
                raise ValueError(f"{command[1]} failed; see {work / 'checks/latest.log'}")
    result = {"ok": True, "checks": results, "desktop_runtime_tested": False}
    phase.write_json(report, result)
    return result


def main():
    class Parser(argparse.ArgumentParser):
        def error(self, message):
            raise ValueError(message)
    parser = Parser(description=__doc__)
    parser.add_argument("command", choices=["prepare", "instructions", "check", "headless", "desktop", "audit"])
    parser.add_argument("path", type=Path, help="archive for prepare; port workspace otherwise")
    parser.add_argument("--rom", type=Path)
    parser.add_argument("--out", type=Path)
    parser.add_argument("--binary", type=Path, default=ROOT / "target/release/romatlas")
    parser.add_argument("--frames", type=int, default=120)
    parser.add_argument("--assets", type=Path)
    parser.add_argument("--settings", type=Path)
    parser.add_argument("--json", action="store_true")
    args = parser.parse_args()
    if args.command == "prepare":
        if args.rom is None or args.out is None:
            raise ValueError("prepare requires --rom and --out")
        result = prepare(args.path, args.rom, args.out, args.binary)
        if not args.json:
            print(instructions(args.out.absolute()), file=sys.stderr)
    else:
        work, meta, rom = workspace(args.path)
        if args.command == "instructions":
            print(instructions(work))
            return
        if args.command == "check":
            result = check(work)
        elif args.command == "audit":
            phase.write_json(work / "checks/audit.json", {"ok": False})
            counts = validate_ledger(work, meta)
            result = check(work)
            result.update(counts, accounting_complete=True, semantic_correctness_assessed=False,
                          release_ready=False)
            phase.write_json(work / "checks/audit.json", result)
        else:
            if args.command == "headless" and not 0 <= args.frames <= 1_000_000:
                raise ValueError("headless frames must be in 0..1000000")
            command = ["cargo", "run", "--quiet", "--locked", "--manifest-path", str(work / "port/Cargo.toml")]
            if args.command == "desktop":
                if sys.platform.startswith("linux") and not (os.environ.get("DISPLAY") or os.environ.get("WAYLAND_DISPLAY")):
                    raise ValueError("desktop requires a graphical session; use headless on this server")
                command.extend(["--features", "desktop"])
            command.extend(["--", "--rom", str(rom)])
            if args.command == "headless":
                command.extend(["--headless", str(args.frames)])
            if args.assets:
                command.extend(["--assets", str(args.assets.resolve(strict=True))])
            if args.settings:
                command.extend(["--settings", str(args.settings.absolute())])
            ran = subprocess.run(command, cwd=work, capture_output=True, text=True)
            if ran.returncode:
                raise ValueError(f"{args.command} execution failed: " + ran.stderr.strip())
            if args.command == "headless":
                print(ran.stdout, end="")
            else:
                print(json.dumps({"ok": True, "desktop_exited": True, "messages": ran.stderr.strip()}))
            return
    print(json.dumps(result))


if __name__ == "__main__":
    try:
        main()
    except (OSError, ValueError, KeyError, TypeError, subprocess.SubprocessError) as error:
        print(json.dumps({"ok": False, "error": str(error)}), file=sys.stderr)
        sys.exit(1)
