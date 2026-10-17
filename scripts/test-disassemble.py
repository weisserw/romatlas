#!/usr/bin/env python3
"""Exercise multi-game workspace setup and real verified listing checkpoints."""
import hashlib
import json
from pathlib import Path
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
    assert "/goal " not in result.stderr
    assert str(workroot / 'Game one' / 'goal.md') in result.stderr
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
    assert Path(records[0]["goal_prompt"]).is_file()
    assert len(records[0]["prompts"]) == 3
    for path in records[0]["prompts"]:
        assert Path(path).is_file()
    disassembly = (work / "prompts/disassembly.txt").read_text()
    assert "Stop immediately and report incomplete" in disassembly
    assert "normal gameplay scenarios driven with debugger inputs" in disassembly
    assert "Do not limit play to startup or basic inputs" in disassembly
    assert "do not require a preexisting" in disassembly
    assert "abandon repetitive play" in disassembly
    assert "analysis/leads.json" in disassembly
    assert "ROM classification: X.XX% (C / T bytes classified; U bytes unknown)." in disassembly
    assert "100 * C / T" in disassembly
    assert "raw-byte export fallback do not count" in disassembly
    for required in ["emulator.memory.write", "emulator.registers.set",
                     "record exact patches", "natural gameplay reachability",
                     "same easy-gain stopping rule"]:
        assert required in disassembly
    redirect = (work / "prompts/redirect.txt").read_text()
    for required in ["exactly one", "emulator.memory.write", "emulator.registers.set",
                     "Do not reissue", "additional evidence", "attempt history"]:
        assert required in redirect
    commentary = (work / "prompts/commentary.txt").read_text()
    assert "(??)" in commentary and "(???)" in commentary
    assert "preserve the entire previous commentary" in commentary
    assert "revisit previous (?), (??) and (???) sections" in commentary
    assert "Previously reviewed status does not exempt" in commentary
    assert "inspect uncommented sections and generic/generated labels" in commentary
    assert "Silence must not stand for an investigated unknown" in commentary
    assert "do not assume missing comments mean there are" in redirect
    goal = (work / "goal.md").read_text()
    assert "After every disassembly pass" in goal
    assert "explicitly print the ROM" in goal
    assert "recording it only in a file is" in goal
    assert goal.index("1. prompts/disassembly") < goal.index("2. prompts/commentary") < goal.index("3. prompts/redirect")
    assert "no such leads remain" in goal and "report incomplete" in goal
    assert json.loads((work / "analysis/leads.json").read_text()) == {"version": 1, "passes": [], "leads": []}
    assert not (work / "prompts/start.md").exists()
    assert "every physical ROM byte" in (work / "workflow.md").read_text()
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
