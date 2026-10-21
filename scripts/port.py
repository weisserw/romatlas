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
import port_research
import port_debug
import port_web

ROOT = phase.ROOT
TEMPLATES = ROOT / "scripts/port-template"
DISPOSITIONS = {"implemented", "merged", "replaced", "excluded"}
GOAL = ("Port this entire archived game into maintainable best-effort native Rust, following "
        "workflow.md and prompts/start.md. First survey representative source paths "
        "and write the high-level module/state/algorithm plan in architecture.md; maintain it "
        "during implementation. Use shared platform helpers. Assess uncertainties by consumer "
        "and port impact; implement reasonable source-informed assumptions and document them. "
        "Preserve the frozen reference and original ledger identities. Replace the scaffold "
        "and implement/account for the whole game, including ROM-backed startup assets, "
        "replacements/added slots, player settings, desktop/browser frontends and original battery saves where present. Validate native logic with focused unit "
        "tests and required format/static/build checks. Runtime fidelity validation is outside "
        "this porting goal: no native-versus-ROM frame/audio/state/scenario comparisons. "
        "Use the debugger only for a specific unresolved source question with a bounded "
        "experiment and stopping condition, never to validate the port. Complete necessary "
        "shared-library changes uncommitted, preserve unrelated work and log changes/checks "
        "for the separate reviewer. Finish architecture, accounting, required checks, "
        "BUILDING.md and handoff; run the administrative ledger/build audit and stop when "
        "it passes. Do not expand acceptance criteria or repeat validation for confidence. "
        "Unverified fidelity and documented implemented assumptions do not block completion; "
        "missing required implementations do. Report a concrete blocker if implementation "
        "cannot proceed. Fidelity/device auditing and release work require a later request.")
RESUME = ("Read architecture.md, progress.md, ledger.json, uncertainties.json, "
          "research/current.json if present, differences.md, library-changes.md and existing "
          "code first. Preserve work; do not reinitialize. Resume implementation under "
          "prompts/start.md's debugger and stopping boundaries; do not start a fidelity audit.")


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
            f"For a separate library reviewer, read {work / 'prompts/review.md'}.\n"
            f"For separately requested QA, read {work / 'debug/goal.md'}.\n")


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
    if manifest.get("version") != 1:
        raise ValueError("port preparation requires a version-1 archive")
    if manifest["platform"] not in {"gb", "snes", "nes", "sms", "gg", "genesis", "pce"}:
        raise ValueError("unsupported port archive platform")
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
        if review.get("version") != 1 or review.get("image_size") != manifest["image_size"]:
            raise ValueError("archive review metadata is inconsistent")
        editorial_status = review.get("editorial_audit", {}).get("status")
        if editorial_status not in {"pending", "reviewed"}:
            raise ValueError("archive editorial status is invalid")
        cursor = 0
        identities = set()
        for row in review["regions"]:
            if (type(row["offset"]) is not int or row["offset"] != cursor
                    or type(row["length"]) is not int or row["length"] <= 0
                    or row["id"] in identities or row.get("status") not in {"pending", "reviewed"}):
                raise ValueError("archive regions must validly partition the ROM")
            cursor += row["length"]
            identities.add(row["id"])
            for unit in row.get("units", []):
                if (unit.get("status") not in {"pending", "reviewed"}
                        or type(unit.get("offset")) is not int or type(unit.get("length")) is not int
                        or unit["offset"] < 0 or unit["length"] <= 0
                        or unit["offset"] + unit["length"] > manifest["image_size"]):
                    raise ValueError("archive unit review metadata is invalid")
        if cursor != manifest["image_size"]:
            raise ValueError("archive reviewed regions do not cover the whole ROM")
        phase.write_json(stage / "ledger.json", {"version": 1, "rom_sha256": manifest["rom_sha256"],
            "regions": records(review), "completion": {"status": "pending", "notes": ""}})
        phase.write_json(stage / "uncertainties.json", {"version": 1,
            "rom_sha256": manifest["rom_sha256"], "entries": port_research.seeds(review)})
        before = snapshot(stage / "review/toolkit-before")
        phase.write_json(stage / "review/toolkit-before.json", before)
        cargo = (f"[package]\nname = {json.dumps(name + '-port')}\nversion = \"0.1.0\"\nedition = \"2024\"\nrust-version = \"1.90\"\n\n"
                 "[workspace]\n\n[features]\ndefault = []\ndesktop = [\"romatlas-port/desktop\"]\nweb = [\"dep:wasm-bindgen\"]\n\n[lib]\ncrate-type = [\"cdylib\", \"rlib\"]\n\n[dependencies]\n"
                 f"romatlas-port = {{ path = {json.dumps(str(ROOT / 'crates/romatlas-port'))} }}\n"
                 "anyhow = \"1.0\"\nserde_json = \"1.0\"\nwasm-bindgen = { version = \"=0.2.129\", optional = true }\n")
        (stage / "port/Cargo.toml").write_text(cargo)
        (stage / "port/web-enabled").write_text("1\n")
        (stage / "port/rust-toolchain.toml").write_text('[toolchain]\nchannel = "1.90.0"\nprofile = "minimal"\ncomponents = ["rustfmt", "clippy"]\ntargets = ["wasm32-unknown-unknown"]\n')
        for filename in ["main.rs", "game.rs", "lib.rs", "config.rs"]:
            text = (TEMPLATES / filename).read_text().replace("__ROM_SHA256__", manifest["rom_sha256"]).replace("__GAME_NAME__", name)
            if manifest["platform"] == "snes":
                if filename == "game.rs":
                    text = text.replace(", gb,", ", snes,").replace("gb::tile_2bpp(", "snes::tile(")
                    text = text.replace('tiles.get("demo-tile").unwrap().bytes)?', 'tiles.get("demo-tile").unwrap().bytes, 2)?')
                    text = text.replace("Game Boy 2bpp", "SNES 2bpp")
                    text = text.replace("gb::GRAYSCALE", "[snes::color(0x7fff),snes::color(0x56b5),snes::color(0x294a),snes::color(0)]").replace(",76,68,", ",124,108,")
                else:
                    text = text.replace("(160,144)", "(256,224)").replace("TickRate::DMG", "TickRate::SNES_NTSC").replace("romatlas_port::settings::Settings::default()", "romatlas_port::snes::default_settings()")
            if manifest["platform"] == "nes":
                if filename == "game.rs":
                    text = text.replace(", gb,", ", nes,").replace("gb::tile_2bpp(", "nes::tile(")
                    text = text.replace("Game Boy 2bpp", "NES 2bpp")
                    text = text.replace("gb::GRAYSCALE", "[[255,255,255,255],[170,170,170,255],[85,85,85,255],[0,0,0,255]]").replace(",76,68,", ",124,116,")
                else:
                    text = text.replace("(160,144)", "(256,240)").replace("TickRate::DMG", "TickRate::NES_NTSC").replace("romatlas_port::settings::Settings::default()", "romatlas_port::nes::default_settings()")
            if manifest["platform"] == "sms":
                if filename == "game.rs":
                    text = text.replace(", gb,", ", sms,").replace("gb::tile_2bpp(", "sms::tile(")
                    text = text.replace("length: 16", "length: 32").replace("bytes.len() == 16", "bytes.len() == 32").replace("16-byte Game Boy 2bpp", "32-byte SMS 4bpp")
                    text = text.replace("gb::GRAYSCALE", "std::array::from_fn::<_,16,_>(|i| sms::color((i as u8)*4))").replace(",76,68,", ",124,92,")
                else:
                    text = text.replace("(160,144)", "(256,192)").replace("TickRate::DMG", "TickRate::SMS_NTSC").replace("romatlas_port::settings::Settings::default()", "romatlas_port::sms::default_settings()")
            if manifest["platform"] == "pce":
                if filename == "game.rs":
                    text = text.replace("length: 16", "length: 32").replace("bytes.len() == 16", "bytes.len() == 32").replace("16-byte Game Boy 2bpp", "32-byte PC Engine planar 4bpp")
                    text = text.replace(", gb,", ", pce,").replace("gb::tile_2bpp(", "pce::tile(")
                    text = text.replace("gb::GRAYSCALE", "std::array::from_fn::<_,16,_>(|i| pce::color((i as u16)*0x24, false))").replace(",76,68,", ",124,108,")
                if filename in {"main.rs", "lib.rs", "config.rs"}:
                    text = text.replace("(160,144)", "(256,240)").replace("TickRate::DMG", "TickRate::PCE_NTSC").replace("romatlas_port::settings::Settings::default()", "romatlas_port::pce::default_settings()")
            if manifest["platform"] == "genesis":
                if filename == "game.rs":
                    text = text.replace("length: 16", "length: 32").replace("bytes.len() == 16", "bytes.len() == 32").replace("16-byte Game Boy 2bpp", "32-byte Genesis packed 4bpp")
                    text = text.replace(", gb,", ", genesis,").replace("gb::tile_2bpp(", "genesis::tile(")
                    text = text.replace("gb::GRAYSCALE", "std::array::from_fn::<_,16,_>(|i| genesis::color((i as u16)*0x222, genesis::Brightness::Normal))").replace(",76,68,", ",156,108,")
                if filename in {"main.rs", "lib.rs", "config.rs"}:
                    text = text.replace("(160,144)", "(320,224)").replace("TickRate::DMG", "TickRate::GENESIS_NTSC").replace("romatlas_port::settings::Settings::default()", "romatlas_port::genesis::default_settings()")
            if manifest["platform"] == "gg":
                if filename == "game.rs":
                    text = text.replace(", gb,", ", gg,").replace("gb::tile_2bpp(", "gg::tile(")
                    text = text.replace("length: 16", "length: 32").replace("bytes.len() == 16", "bytes.len() == 32").replace("16-byte Game Boy 2bpp", "32-byte Game Gear 4bpp")
                    text = text.replace("gb::GRAYSCALE", "std::array::from_fn::<_,16,_>(|i| gg::color((i as u16)*0x111))")
                else:
                    text = text.replace("TickRate::DMG", "TickRate::GG_NTSC").replace("romatlas_port::settings::Settings::default()", "romatlas_port::gg::default_settings()")
            (stage / "port/src" / filename).write_text(text)
        # Pin dependencies before publication. Toolkit setup/build must have
        # resolved them; preparation does not require an online registry.
        result = subprocess.run(["cargo", "generate-lockfile", "--offline", "--manifest-path", str(stage / "port/Cargo.toml")], capture_output=True, text=True)
        if result.returncode:
            raise ValueError("cannot lock scaffold dependencies; build the toolkit first:\n" + result.stderr)
        subprocess.run(["cargo", "fmt", "--manifest-path", str(stage / "port/Cargo.toml")], check=True, capture_output=True, text=True)
        prompt = (TEMPLATES / "prompt.md").read_text()
        if manifest["platform"] == "gb":
            prompt += "\nGame Boy / Color: read " + str(ROOT / "docs/gbc.md") + ". Both use the gb platform and stock .sym/RGBDS. Inspect header byte $143: bit 7 selects native color; bits 7/6 mark CGB-only. Use gb::cgb for banked color rendering and palette RAM; keep DMG APIs for DMG behavior. CPU double speed does not double the LCD/APU cadence. The scaffold's grayscale tile is an arbitrary demonstration, not the game's rendering.\n"

        if manifest["platform"] == "pce":
            prompt += "\nPC Engine HuCard: read " + str(ROOT / "docs/pce-native.md") + ". Use independent MIT pce::Vdc, Vce and Psg helpers; ports own raster scheduling, music drivers and gameplay. CD/SuperGrafx are out of scope.\n"
        if manifest["platform"] == "genesis":
            prompt += "\nGenesis: read " + str(ROOT / "docs/genesis-native.md") + ". Use independent romatlas_port::genesis VDP, YM2612 and PSG helpers. Ports own raster scheduling, sound drivers and gameplay.\n"
        if manifest["platform"] == "gg":
            prompt += "\nGame Gear helpers: read " + str(ROOT / "docs/gg-native.md") + ". Use romatlas_port::gg for 12-bit CRAM/LCD Mode 4 and stereo PSG; Start is polled, and no FM unit exists. The native scaffold is not implemented gameplay.\n"
        (stage / "prompts/start.md").write_text(prompt)
        (stage / "goal.md").write_text("# Native port\n\n" + GOAL + "\n\nOn resume: " + RESUME + "\n")
        shutil.copyfile(TEMPLATES / "architecture.md", stage / "architecture.md")
        shutil.copyfile(TEMPLATES / "review.md", stage / "prompts/review.md")
        shutil.copyfile(TEMPLATES / "reference-updates.md", stage / "prompts/reference-updates.md")
        build_guide = (TEMPLATES / "BUILDING.md").read_text().replace("__GAME_NAME__", name).replace("__TOOLKIT_PATH__", str(ROOT))
        if manifest["platform"]=="pce":
            build_guide=build_guide.replace("game.gb", "game.pce")
        if manifest["platform"]=="genesis":
            build_guide=build_guide.replace("game.gb", "game.md")
        if manifest["platform"]=="gg":
            build_guide=build_guide.replace("game.gb", "game.gg")
        if manifest["platform"]=="sms":
            build_guide=build_guide.replace("game.gb", "game.sms")
        (stage / "BUILDING.md").write_text(build_guide)
        (stage / "AGENTS.md").write_text(f"""# Native port workspace

These rules apply to every task in this workspace. The user's requested task
selects the workflow; this file does not start or resume a goal.

- Use Rust 1.90.0 and Cargo.lock. Do not launch desktop graphics on this
  headless server; compile desktop code without opening a window.
- Never download or commit commercial ROMs or extracted assets. Keep ROMs,
  restored source/assets, build products and generated comparisons local.
- Preserve reference/, review/toolkit-before/, original ledger identities,
  deployment backups and unrelated existing changes. Keep QA records separate
  from implementation accounting and preserve superseded records.
- Use the shared Game APIs for desktop/browser rendering, audio, controls and
  original battery saves. Keep identity, dimensions, cadence and defaults in
  config.rs where present. Keep game rules shared across frontends.
- Toolkit: {json.dumps(str(ROOT))}. For edits there, follow that repository's applicable
  instructions. Leave shared-library edits uncommitted and log their changes
  and check results in library-changes.md; preserve pre-existing work.
- No agent launch, check-in or publication is authorized by this workspace.
""")
        (stage / "workflow.md").write_text("""# Port workflow

Implement the whole game as a maintainable best-effort native Rust port.
The restored byte-identical reference establishes source integrity only.
The arbitrary-tile scaffold implements no gameplay.

1. Read the full prompts/start.md contract and representative startup, gameplay,
   rendering and sound source paths. Write a short actionable architecture.md with
   modules, state ownership, interfaces, algorithms, scheduling and implementation
   order. Do not require exhaustive analysis or approval before implementing.
2. Implement the whole game using shared platform helpers. Maintain architecture.md
   when decisions change and fix concrete design problems after substantial
   features. Avoid monolithic code and cosmetic file splitting. No size quotas
   or repeated whole-project architecture audits are required. Write useful Rust
   comments as specified in prompts/start.md, keeping non-obvious reasoning,
   source-game quirks and implementation assumptions close to the relevant code.
   Use descriptive enums or scoped named constants for states and other meaningful
   values; avoid unexplained numeric IDs, flags, sentinels and gameplay parameters.
3. Account for original source behavior in ledger.json and assess uncertainties
   as consumers become relevant. Preserve IDs, ranges and original source records.
   Implement reasonable source-informed assumptions, documenting their basis and
   limits. A missing implementation remains incomplete; lack of fidelity proof
   for an implemented interpretation does not. ROM-backed data may remain
   structurally uncertain; preserving code bytes does not implement behavior.
4. Validate intended native logic with focused unit tests. Run python3 run.py check
   for formatting, clippy, tests and desktop/Wasm compilation. Recheck only after a
   relevant change, failure or concrete new defect. Document actual check results.
5. Finish accounting, assessed uncertainties, a coherent architecture and handoff
   (README.md, BUILDING.md, progress.md, differences.md, library-changes.md).
   Make one bounded maintainability pass, including unexplained semantic numeric
   literals and missing/stale code comments.
   Set completion.status=complete with
   notes describing this scope, run python3 run.py audit and stop when it passes.
   Do not create more work to fill the remaining goal budget.

Runtime fidelity validation is outside this porting goal. No native-versus-ROM
frame, screenshot, animation, audio, state or gameplay-scenario comparisons,
playthrough requirements or confidence-building replay loops are authorized.
Unit tests check intended logic; they need not establish original-game equivalence.
Debugger use is limited to a named unresolved source question that static reading
cannot reasonably answer. Record its needed observation and stopping condition;
perform a bounded experiment and return to implementation. Stop at the declared
bound even if unanswered; do not repeat inconclusive runs without new evidence.
Document a reasonable
assumption if inconclusive. Do not reopen it without new contradictory evidence.
A later fidelity/device audit requires an explicit user request. This boundary
applies to Game Boy, SMS, NES and SNES and to the entire generated instruction set.

Available commands (availability does not make each command a required phase):

```sh
python3 run.py instructions
python3 run.py check
python3 run.py audit
python3 run.py reference-init
python3 run.py reference-verify --revision findings-001
```

The audit checks accounting and builds, not gameplay fidelity. Do not run it while
ledger completion is pending. check retains logs in checks/latest.log and JSON
results. desktop and headless execution remain available for later user-requested
work; neither is a required implementation-validation step. Never launch desktop
graphics on this server. Unit tests initialize no display or audio device.

Ledger dispositions are implemented, merged, replaced or excluded, with a summary
and equivalents except for explained exclusions. Add discovered units with unique
IDs and source ranges. Favor implementing unclear behavior over dropping it.
Assess each uncertainty's consumers and consequences; distinguish established
source facts from implemented assumptions. Do not require complete classification,
source-proof exhaustion or a runtime trace for every assessment. The original
mapping goal retains its stricter classification/evidence requirements.

prompts/reference-updates.md describes optional source corrections. Use
reference-init only when a correction is useful; edit the working source/state
and evidence, then reference-verify a coherent correction batch. Its map and
byte-identical rebuild checks protect reference integrity, not native fidelity.
research/current.json selects adopted supplements; reference/ and original ledger
identities remain frozen. Never regenerate over maintained commentary or publish
an implementation assumption as confirmed source classification.

Keep progress.md for implementation status and concrete blockers, differences.md
for assumptions/approximations/unassessed fidelity, and library-changes.md for
shared changes, interface rationale, unit tests/checks and limits. Adapt BUILDING.md
to actual dependencies/executable/flags; other-OS/device recipes are optional
user instructions, not testing obligations. The initial toolkit snapshot and
hashes/status/HEAD in review/toolkit-before.json preserve pre-existing work for
separate review using prompts/review.md. No review agent is launched automatically.

Library changes remain uncommitted. A local path dependency is not frozen by
Cargo.lock; release pinning, packaging and publication are later work. Keep
ROMs, restored source/assets and generated products local. Source incompleteness
is not permission to silently omit required game behavior; report actual
implementation blockers without presenting unverified fidelity as a blocker.
""")
        (stage / "README.md").write_text(f"""# {name}: native Rust port workspace

Status: scaffold only. Implement the entire game before claiming completion.
See [BUILDING.md](BUILDING.md) for local macOS/Linux/Windows source builds, Android integration guidance and
transfer instructions. Adapt that guide for the finished port and tested facts.
The implementation goal uses focused unit tests and build/static checks. Runtime
fidelity/device auditing is separate later work. On a desktop with SDL2 installed,
`python3 run.py desktop` opens the window; F1 opens saved player settings.
`python3 run.py web` builds and serves the browser frontend (default host 0.0.0.0; use --host to change it).
Game logic and asset extraction stay shared; implement original battery saving
through the optional Game hooks. See BUILDING.md for storage and browser limits.
The executable accepts --rom, --assets, --settings and --headless (frame count).
After implementing the real collections, document replacement formats, adding a
level and game-rule source locations here. See examples/asset-extension.json for
an illustrative manifest: original ROM levels plus a replacement and added slot.
The illustrative manifest is documentation, not a working game-level loader.
Generated example content must be developer-authored; do not distribute originals.

For a separately requested debugging/QA run, tell the agent: "Read debug/goal.md."
That file links all required instructions and records. It is preinstalled; no
manual installation or automatic QA launch occurs during port preparation.
""")
        phase.write_json(stage / "examples/asset-extension.json", {"version": 1,
            "entries": [{"id": "level-01", "file": "levels/replacement-01.json"},
                        {"id": "level-16", "file": "levels/new-16.json"}], "order": []})
        (stage / "library-changes.md").write_text("# Library changes\n\nNo port-driven changes recorded yet. Record requirements, files, reusable contracts, focused unit tests/checks and limits. Runtime fidelity auditing is outside the implementation goal.\n")
        (stage / "progress.md").write_text("# Port progress\n\nPrepared reference and demonstration scaffold. Every source-ledger entry is pending. Read representative source and complete architecture.md before substantial implementation. No gameplay is ported.\n")
        (stage / "differences.md").write_text("# Differences and validation limits\n\nScaffold implements no gameplay or original audio. Record implemented assumptions and known differences as work proceeds. Runtime fidelity/device auditing is outside this goal; its absence does not block implementation completion.\n")
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
        (stage / ".gitignore").write_text("/input/\n/reference/\n/research/\n/review/toolkit-before/\n/checks/\n/port/target/\n/web-dist/\n/debug/artifacts/\n*.gb\n*.gbc\n*.sms\n*.gg\n*.md.bin\n*.gen\n*.smd\n*.rom\n__pycache__/\n")
        frozen = {str(p.relative_to(stage)): phase.digest(p) for p in (stage / "reference").rglob("*") if p.is_file()}
        frozen.update({str(p.relative_to(stage)): phase.digest(p) for p in (stage / "review/toolkit-before").rglob("*") if p.is_file()})
        frozen["review/toolkit-before.json"] = phase.digest(stage / "review/toolkit-before.json")
        phase.write_json(stage / "workspace.json", {"version": 1, "name": name,
            "toolkit": str(ROOT), "binary": str(binary), "platform": manifest["platform"],
            "uncertainty_tracking": 1, "rom": "input/game.rom", "sha256": manifest["rom_sha256"],
            "archive": str(archive), "archive_sha256": phase.digest(archive / "archive.json"),
            "frozen": frozen, "reference_verification": verification})
        # Install QA instructions in staging so every platform publishes the same
        # complete workspace atomically; preparation never starts QA.
        port_debug.install(stage)
        # Reserve the new name before publication; never replace another workspace.
        output.mkdir()
        try:
            stage.rename(output)
        except OSError:
            output.rmdir()
            raise
    return {"ok": True, "workspace": str(output), "sha256": manifest["rom_sha256"],
            "byte_identical_reference": True, "scaffold": True,
            "start_prompt": str(output / "prompts/start.md"), "review_prompt": str(output / "prompts/review.md"),
            "debug_goal": str(output / "debug/goal.md"), "debug_installed": True}


def workspace(path, *, reference=True):
    work = path.resolve(strict=True)
    meta = json.loads((work / "workspace.json").read_text())
    if meta["version"] != 1 or Path(meta["toolkit"]).resolve() != ROOT:
        raise ValueError("unsupported port workspace/toolkit")
    rom = phase.local(work, meta["rom"])
    if phase.digest(rom) != meta["sha256"]:
        raise ValueError("workspace ROM identity changed")
    if reference:
        for name, digest in meta["frozen"].items():
            if phase.digest(phase.local(work, name)) != digest:
                raise ValueError(f"frozen reference changed: {name}")
        port_research.current(work, meta)
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
    return {"regions": len(actual), "units": len(seen) - len(actual),
            **port_research.validate_uncertainties(work, meta, ledger)}


def check(work):
    cargo = ["cargo", "--manifest-path", str(work / "port/Cargo.toml")]
    checks = [[cargo[0], "fmt", *cargo[1:], "--check"],
              [cargo[0], "clippy", *cargo[1:], "--all-targets", "--features", "desktop", "--locked", "--", "-D", "warnings"],
              [cargo[0], "test", *cargo[1:], "--locked"],
              [cargo[0], "build", *cargo[1:], "--features", "desktop", "--locked"]]
    if (work / "port/web-enabled").exists():
        checks.extend([
            [cargo[0], "clippy", *cargo[1:], "--lib", "--features", "web", "--target", "wasm32-unknown-unknown", "--locked", "--", "-D", "warnings"],
            [cargo[0], "build", *cargo[1:], "--lib", "--features", "web", "--target", "wasm32-unknown-unknown", "--release", "--locked"],
        ])
    results = []
    report = work / "checks/latest.json"
    report.parent.mkdir(parents=True, exist_ok=True)
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
    result = {"ok": True, "checks": results, "desktop_runtime_tested": False, "wasm_compiled": (work / "port/web-enabled").exists(), "browser_runtime_tested": False}
    phase.write_json(report, result)
    return result


def main():
    class Parser(argparse.ArgumentParser):
        def error(self, message):
            raise ValueError(message)
    parser = Parser(description=__doc__)
    parser.add_argument("command", choices=["prepare", "instructions", "check", "headless", "desktop", "audit", "reference-init", "reference-verify", "web-build", "web", "debug"])
    parser.add_argument("path", type=Path, help="archive for prepare; port workspace otherwise")
    parser.add_argument("--rom", type=Path)
    parser.add_argument("--out", type=Path)
    parser.add_argument("--binary", type=Path, default=ROOT / "target/release/romatlas")
    parser.add_argument("--host", default="0.0.0.0", help="web server bind host (default: 0.0.0.0)")
    parser.add_argument("--port", type=int, default=8000, help="web server port (0 selects a free port)")
    parser.add_argument("--frames", type=int, default=120)
    parser.add_argument("--assets", type=Path)
    parser.add_argument("--settings", type=Path)
    parser.add_argument("--json", action="store_true")
    parser.add_argument("--revision", help="new name for a verified local reference revision")
    args = parser.parse_args()
    if args.command == "prepare":
        if args.rom is None or args.out is None:
            raise ValueError("prepare requires --rom and --out")
        result = prepare(args.path, args.rom, args.out, args.binary)
        if not args.json:
            print(instructions(args.out.absolute()), file=sys.stderr)
    else:
        work, meta, rom = workspace(args.path, reference=args.command in {
            "audit", "reference-init", "reference-verify",
        })
        if args.command in {"web-build", "web"}:
            bundle = port_web.build(work)
            if args.command == "web":
                port_web.serve(work, rom, bundle, args.assets, args.port, args.host)
                return
            print(json.dumps({"ok": True, "bundle": str(bundle)}))
            return
        if args.command == "debug":
            if args.settings:
                raise ValueError("debug uses logical input; --settings is not supported")
            command = ["cargo", "run", "--quiet", "--locked", "--manifest-path", str(work / "port/Cargo.toml"),
                       "--", "--rom", str(rom), "--debug-stdio"]
            if args.assets:
                command.extend(["--assets", str(args.assets.resolve(strict=True))])
            result = subprocess.run(command, cwd=work)
            if result.returncode:
                raise ValueError("native debug process exited with code " + str(result.returncode))
            return
        if args.command == "instructions":
            print(instructions(work))
            return
        if args.command == "check":
            result = check(work)
        elif args.command == "reference-init":
            result = port_research.initialize(work, meta)
        elif args.command == "reference-verify":
            result = port_research.verify(work, meta, rom, args.revision)
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
