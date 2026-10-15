#!/usr/bin/env python3
"""Upgrade local workspace tooling/prompts, with backups and unchanged ROM/map checks."""
import argparse
from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path
import shutil
import socket
import subprocess
import tempfile

import disassemble
import behavior


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def replace(path, text):
    path.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.NamedTemporaryFile(mode="w", dir=path.parent, delete=False) as file:
        file.write(text)
        temporary = Path(file.name)
    temporary.replace(path)


def upgrade(workspace, binary, apply=False):
    root = workspace.resolve(strict=True)
    if root == disassemble.ROOT or disassemble.ROOT in root.parents:
        raise ValueError("game workspace must be outside the toolkit repository")
    meta = json.loads((root / "workspace.json").read_text())
    rom = (root / meta["rom"]).resolve(strict=True)
    if root not in rom.parents or sha(rom) != meta["sha256"]:
        raise ValueError("workspace ROM identity/location mismatch")
    binary = binary.resolve(strict=True)
    inspected = json.loads(subprocess.check_output([str(binary), "inspect", str(rom)], text=True))
    if inspected["sha256"] != meta["sha256"]:
        raise ValueError("toolkit inspection identity mismatch")
    meta.update(binary=str(binary), toolkit=str(disassemble.ROOT), workflow_revision=4)
    files = {
        "workspace.json": json.dumps(meta, indent=2) + "\n",
        "AGENTS.md": disassemble.AGENTS.format(toolkit=json.dumps(str(disassemble.ROOT))),
        "workflow.md": disassemble.WORKFLOW,
        "prompts/start.md": disassemble.START,
        "prompts/resume.md": disassemble.RESUME,
        "prompts/goal.md": disassemble.GOAL,
        "run.py": disassemble.RUNNER,
        "analysis/session.py": (disassemble.ROOT / "scripts/disassembly_session.py").read_text(),
        "analysis/work.py": (disassemble.ROOT / "scripts/disassembly_work.py").read_text(),
    }
    # Seed new accounting without overwriting an existing inventory or its history.
    if not (root / "analysis/behavior.md").exists():
        files["analysis/behavior.md"] = disassemble.BEHAVIOR
    if not (root / "analysis/behavior.json").exists():
        files["analysis/behavior.json"] = json.dumps(behavior.new(meta["sha256"], "disassembly"), indent=2) + "\n"
    if not apply:
        return dict(ok=True, applied=False, workspace=str(root), files=list(files), binary=str(binary))
    with socket.socket() as probe:
        probe.settimeout(0.5)
        if probe.connect_ex(("127.0.0.1", 8766)) == 0:
            raise ValueError("port 8766 is in use; stop the workspace bridge before upgrading")
    protected = [rom, root / "analysis/state.json", root / "maps/current.sym",
                 root / "maps/current-compact.sym", root / "analysis/classification-audit.json",
                 root / "analysis/progress.md", root / "analysis/investigation.md",
                 root / "analysis/gameplay.md", root / "analysis/toolkit-issues.md",
                 root / "analysis/behavior.md", root / "analysis/behavior.json"]
    hashes = {str(p.relative_to(root)): sha(p) for p in protected if p.exists()}
    stamp = datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%S%fZ")
    backup = root / "analysis/setup-backups" / stamp
    backup.mkdir(parents=True)
    originals = {}
    for name in files:
        path = root / name
        if path.is_symlink():
            raise ValueError(f"refusing to replace symlink: {name}")
        originals[name] = sha(path) if path.exists() else None
        if path.exists():
            target = backup / name
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(path, target)
    manifest = dict(version=1, workspace=str(root), original_files=originals,
                    protected_hashes=hashes, binary=str(binary), binary_sha256=sha(binary))
    replace(backup / "manifest.json", json.dumps(manifest, indent=2) + "\n")
    installed = []
    try:
        for name, content in files.items():
            replace(root / name, content)
            installed.append(name)
        for name, expected in hashes.items():
            if sha(root / name) != expected:
                raise ValueError(f"protected file changed during upgrade: {name}")
        if (root / "analysis/state.json").exists():
            subprocess.run(["python3", "analysis/work.py", "refresh"], cwd=root,
                           check=True, capture_output=True, text=True)
        else:
            replace(root / "analysis/frontier.md", "No classification state yet. Establish the baseline, then run analysis/work.py refresh.\n")
    except Exception:
        for name in reversed(installed):
            if originals[name] is None:
                (root / name).unlink()
            else:
                shutil.copy2(backup / name, root / name)
        raise
    return dict(ok=True, applied=True, workspace=str(root), backup=str(backup),
                protected_files_unchanged=True, resume_prompt=str(root / "prompts/resume.md"),
                goal_prompt=str(root / "prompts/goal.md"))


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("workspace", type=Path)
    parser.add_argument("--binary", type=Path, default=disassemble.ROOT / "target/release/romatlas")
    parser.add_argument("--apply", action="store_true", help="apply after backing up setup; default previews")
    args = parser.parse_args()
    try:
        print(json.dumps(upgrade(args.workspace, args.binary, args.apply)))
    except (OSError, ValueError, subprocess.CalledProcessError) as error:
        raise SystemExit(json.dumps({"ok": False, "error": str(error)}))
