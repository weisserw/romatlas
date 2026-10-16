#!/usr/bin/env python3
"""Exercise multi-game workspace setup and real verified listing checkpoints."""
import hashlib
import json
from pathlib import Path
import shlex
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]
BIN = ROOT / "target/release/romatlas"


def call(*args, cwd=ROOT, ok=True):
    result = subprocess.run(list(map(str, args)), cwd=cwd, capture_output=True, text=True)
    assert (result.returncode == 0) == ok, (args, result.stdout, result.stderr)
    return result


with tempfile.TemporaryDirectory(prefix="romatlas-workspaces-") as temporary:
    base = Path(temporary)
    data = bytearray(0x8000)
    data[0x100:0x103] = bytes([0xc3, 0x50, 1])
    data[0x150:0x153] = bytes([0x00, 0x18, 0xfd])
    data[0x14d] = (-sum(data[0x134:0x14d]) - 25) & 255
    first = base / "Game one.gb"
    second = base / "Game two.gbc"
    first.write_bytes(data)
    second.write_bytes(data)
    current = base / "current directory"
    current.mkdir()
    default = call("python3", ROOT / "scripts/disassemble.py", first,
                   "--json", cwd=current)
    assert Path(json.loads(default.stdout)["workspace"]) == current / first.stem
    assert (current / first.stem / "input/game.rom").read_bytes() == data
    workroot = base / "work spaces"
    result = call("python3", ROOT / "scripts/disassemble.py", first, second, "--root", workroot)
    records = [json.loads(line) for line in result.stdout.splitlines()]
    assert len(records) == 2 and all(record["ok"] for record in records)
    assert result.stderr.count("Workspace ready:") == 2
    assert result.stderr.count("codex --enable goals -C ") == 2
    assert "/goal Read AGENTS.md, workflow.md and prompts/start.md." in result.stderr
    assert "/goal Read AGENTS.md, workflow.md and prompts/resume.md." in result.stderr
    assert "Continue beyond the raw-byte baseline" in result.stderr
    assert "not shell commands" in result.stderr
    assert f"codex --enable goals -C {shlex.quote(str(workroot / 'Game one'))}" in result.stderr
    quiet = call("python3", ROOT / "scripts/disassemble.py", first,
                 "--workspace", base / "quiet workspace", "--json")
    assert json.loads(quiet.stdout)["ok"] and not quiet.stderr
    work = workroot / "Game one"
    identity = hashlib.sha256(data).hexdigest()
    meta = json.loads((work / "workspace.json").read_text())
    assert Path(meta["binary"]) == BIN.resolve()
    assert meta["sha256"] == identity
    assert (work / "input/game.rom").read_bytes() == data
    assert (workroot / "Game two/input/game.rom").read_bytes() == data
    assert "Do not stop after a raw-byte baseline" in (work / "AGENTS.md").read_text()
    for prompt in ["prompts/start.md", "prompts/resume.md"]:
        content = " ".join((work / prompt).read_text().split())
        assert "completion and investigation contract" in content
        assert "not goal completion" in content
        assert "Stop incomplete" in content
        assert "semantics wherever useful" in content
        assert "Game familiarization" in content
        assert "analysis/gameplay.md" in content
        assert "against this ROM" in content
    workflow = (work / "workflow.md").read_text()
    assert "every physical ROM byte" in workflow
    assert "total budget or number of stagnant passes" in workflow
    assert "every remaining path in every unresolved entry" in workflow
    assert "Reopen entries" in workflow
    assert "apparent unreachability alone" in workflow
    assert "Execution coverage need not reach 100%" in workflow
    assert "required annotation coverage" in workflow
    assert (work / "analysis/investigation.md").exists()
    assert (work / "analysis/gameplay.md").exists()
    assert "source URLs" in (work / "analysis/gameplay.md").read_text()
    assert "GameFAQs" in workflow
    assert "do not\nindependently justify structural classification" in workflow
    assert "Before exhausting a\ngameplay-dependent investigation path" in workflow
    assert "continue local analysis" in workflow
    goals = [line for line in result.stderr.splitlines() if line.strip().startswith("/goal Read")]
    assert len(goals) == 4
    assert all("every physical byte classified with evidence" in goal for goal in goals)
    assert all("never marking this goal complete" in goal for goal in goals)
    assert all((work / path).exists() for path in ["prompts/start.md", "prompts/resume.md", "workflow.md"])
    call("python3", ROOT / "scripts/disassemble.py", first, "--root", workroot, ok=False)
    assert (work / "input/game.rom").read_bytes() == data
    invalid = base / "invalid.gb"
    invalid.write_bytes(b"not a ROM")
    call("python3", ROOT / "scripts/disassemble.py", invalid, "--root", workroot, ok=False)
    assert not (workroot / "invalid").exists()
    call("python3", ROOT / "scripts/disassemble.py", first,
         "--workspace", ROOT / "build/rejected-workspace", ok=False)
    assert not (ROOT / "build/rejected-workspace").exists()
    # Same stems must not silently merge different images.
    alias = base / "Game one.gbc"
    alias.write_bytes(data)
    call("python3", ROOT / "scripts/disassemble.py", first, alias,
         "--root", base / "duplicates", ok=False)
    assert not (base / "duplicates").exists()
    call("python3", "run.py", "inspect", cwd=work)
    call("python3", "run.py", "baseline", cwd=work)
    report = json.loads((work / "analysis/baseline-verification.json").read_text())
    assert report["byte_identical"] and report["rom_sha256"] == identity
    call("python3", "run.py", "baseline", cwd=work, ok=False)
    # A real code annotation survives export, assembly generation and verification.
    rom = work / "input/game.rom"
    store = work / "analysis/state.json"
    call(BIN, "store", "init", rom, "--output", store)
    call(BIN, "store", "annotate", rom, "--store", store, "--offset", "256",
         "--length", "3", "--kind", "code", "--provenance", "static", "--confidence", "0.8")
    call(BIN, "store", "export", rom, "--store", store,
         "--output", work / "maps/current.sym")
    call("python3", "run.py", "checkpoint", "pass-001", cwd=work)
    checkpoint = json.loads((work / "analysis/pass-001-verification.json").read_text())
    assert checkpoint["byte_identical"]
    assert (work / "maps/pass-001.sym").read_bytes() == (work / "maps/current.sym").read_bytes()
    assert checkpoint["map_sha256"] == hashlib.sha256((work / "maps/pass-001.sym").read_bytes()).hexdigest()
    assembly = list((work / "generated/pass-001").rglob("*.asm"))
    assert any("jp " in path.read_text().lower() for path in assembly)
    call("python3", "run.py", "checkpoint", "pass-001", cwd=work, ok=False)
    call("python3", "run.py", "checkpoint", "../escape", cwd=work, ok=False)
    call("python3", "run.py", "checkpoint", "baseline", cwd=work, ok=False)
    rom.write_bytes(bytes([1]) + bytes(data[1:]))
    call("python3", "run.py", "inspect", cwd=work, ok=False)
    call("python3", "run.py", "checkpoint", "pass-002", cwd=work, ok=False)
    assert not (work / "generated/pass-002").exists()
    assert first.read_bytes() == data

print("workspace setup, isolation, identity guards, baseline and code checkpoint passed")
