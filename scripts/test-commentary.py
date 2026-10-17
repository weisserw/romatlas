#!/usr/bin/env python3
"""Exercise the commentary lifecycle with a local synthetic ROM only."""
import hashlib
import importlib.util
import json
from pathlib import Path
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]
BIN = ROOT / "target/release/romatlas"
SCRIPT = ROOT / "scripts/commentary.py"


def call(*args, ok=True):
    result = subprocess.run(list(map(str, args)), capture_output=True, text=True)
    assert (result.returncode == 0) == ok, (args, result.stdout, result.stderr)
    return result


with tempfile.TemporaryDirectory(prefix="romatlas-commentary-") as temporary:
    base = Path(temporary)
    data = bytearray(0x8000)
    data[0x100:0x103] = bytes([0xc3, 0x50, 1])
    data[0x150:0x153] = bytes([0x00, 0x18, 0xfd])
    data[0x14d] = (-sum(data[0x134:0x14d]) - 25) & 255
    original = base / "Synthetic game.gb"
    original.write_bytes(data)
    work = base / "existing work space"
    call("python3", ROOT / "scripts/disassemble.py", original, "--workspace", work, "--json")
    rom = work / "input/game.rom"
    state = work / "analysis/state.json"
    call(BIN, "store", "init", rom, "--output", state)
    call(BIN, "store", "annotate", rom, "--store", state, "--offset", "256",
         "--length", "3", "--kind", "code", "--provenance", "static", "--confidence", "0.8")
    call(BIN, "store", "export", rom, "--store", state, "--output", work / "maps/current.sym")
    preserved = {name: (work / name).read_bytes() for name in
                 ["AGENTS.md", "workflow.md", "analysis/state.json", "maps/current.sym"]}
    result = call("python3", SCRIPT, "prepare", work)
    assert json.loads(result.stdout)["byte_identical"]
    assert "prompts/commentary.txt" in result.stderr and "/goal " not in result.stderr
    prompt = (work / "commentary/prompt.md").read_text()
    for required in ["default is an intermediate research pass", "Preserve valid prior review credit",
                     "clear, light prose", "ordinary tentative prose", "diminishing returns",
                     "At final acceptance", "mandatory whole-source reread",
                     "Do not repeatedly rebuild", "Keep unreviewed work"]:
        assert required in prompt
    assert "(???)" not in prompt
    assert not (work / "commentary/instructions.txt").exists()
    phase = work / "commentary"
    before = (phase / "reading-before.asm").read_bytes()
    assert b"BEGIN bank_000.asm" in before
    assert b'INCLUDE "hardware.inc"' in before
    assert (phase / "source/game.asm").is_file()
    assert not (phase / "source/game.o").exists()
    assert "Do not relabel" in (phase / "prompt.md").read_text()
    call("python3", SCRIPT, "prepare", work, ok=False)
    assert all((work / name).read_bytes() == value for name, value in preserved.items())
    call("python3", SCRIPT, "audit", work, ok=False)
    bank = phase / "source/bank_000.asm"
    bank.write_text("; Enter the synthetic program.\n" + bank.read_text())
    call("python3", SCRIPT, "verify", work)
    call("python3", SCRIPT, "read", work)
    # A partial handoff verifies and reads successfully without weakening final acceptance.
    call("python3", SCRIPT, "audit", work, ok=False)
    assert not (phase / "audit.json").exists()
    assert b"Enter the synthetic program" in (phase / "reading-after.asm").read_bytes()
    assert (phase / "reading-before.asm").read_bytes() == before
    review = json.loads((phase / "review.json").read_text())
    assert sum(row["length"] for row in review["regions"]) == len(data)
    assert any(row["kind"] == "Unknown" for row in review["regions"])
    for row in review["regions"]:
        row.update(status="reviewed", summary="Synthetic test review", source=["bank_000.asm:1"])
        if row["kind"] == "Code":
            row["units"] = [{"name": "SyntheticEntry", "offset": row["offset"],
                             "length": row["length"], "status": "reviewed",
                             "summary": "Enter synthetic loop", "source": ["bank_000.asm:1"]}]
    review["editorial_audit"] = {"status": "reviewed", "notes": "Synthetic lifecycle acceptance"}
    (phase / "review.json").write_text(json.dumps(review))
    final = json.loads(call("python3", SCRIPT, "audit", work).stdout)
    assert final["byte_identical"] and final["reviewed_units"] == 1
    review["regions"][0]["length"] += 1
    (phase / "review.json").write_text(json.dumps(review))
    call("python3", SCRIPT, "audit", work, ok=False)
    # Broken edits must fail fidelity, even when a previous build succeeded.
    bank.write_text(bank.read_text().replace("jp ", "call ", 1))
    call("python3", SCRIPT, "verify", work, ok=False)
    rom.write_bytes(bytes([1]) + bytes(data[1:]))
    call("python3", SCRIPT, "read", work, ok=False)
    assert original.read_bytes() == data
    assert hashlib.sha256(original.read_bytes()).hexdigest() == json.loads((work / "workspace.json").read_text())["sha256"]

    # The reading copy follows includes, rather than sorting banks or omitting helpers.
    spec = importlib.util.spec_from_file_location("commentary", SCRIPT)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    source = base / "include checks"
    source.mkdir()
    (source / "game.asm").write_text('INCLUDE "helper.asm"\n')
    (source / "helper.asm").write_text('INCLUDE "game.asm"\n')
    try:
        module.reading(source)
        raise AssertionError("cyclic includes accepted")
    except ValueError:
        pass
    (source / "helper.asm").write_text('INCLUDE "../outside.asm"\n')
    try:
        module.reading(source)
        raise AssertionError("escaping include accepted")
    except ValueError:
        pass

print("commentary preparation, preservation, review audit, reading and fidelity regressions passed")
