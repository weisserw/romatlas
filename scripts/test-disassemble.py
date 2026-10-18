#!/usr/bin/env python3
"""Exercise multi-game workspace setup and real verified listing checkpoints."""
import hashlib
import json
from pathlib import Path
import subprocess
import sys
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
    assert [Path(p).name for p in records[0]["prompts"]] == [
        "disassembly.txt", "commentary.txt", "redirect.txt"]
    assert not (work / "prompts/final-investigation.txt").exists()
    for path in records[0]["prompts"]:
        assert Path(path).is_file()
    # All phases use one policy, including the SNES profile. These assertions
    # verify instruction delivery, not an agent's reasoning or classification.
    evidence = (work / "evidence-policy.md").read_text()
    assert evidence == (ROOT / "scripts/workflow-template/evidence-policy.md").read_text()
    sys.path.insert(0, str(ROOT / "scripts"))
    import snes_workflow
    assert snes_workflow.profile(evidence) == evidence
    profiled = snes_workflow.files({
        "workspace.json": json.dumps(dict(rom="input/game.rom", sha256=identity)),
        "run.py": "",
        "evidence-policy.md": evidence,
        **{name: (work / name).read_text() for name in [
            "AGENTS.md", "workflow.md", "goal.md", "prompts/disassembly.txt",
            "prompts/commentary.txt", "prompts/redirect.txt"]},
    }, dict(size=len(data), sha256=identity))
    assert profiled["evidence-policy.md"] == evidence
    for name in ["AGENTS.md", "workflow.md", "goal.md", "prompts/disassembly.txt",
                 "prompts/commentary.txt", "prompts/redirect.txt"]:
        assert "evidence-policy.md" in (work / name).read_text(), name
        assert "evidence-policy.md" in profiled[name], name
    policy = " ".join(evidence.split())
    for required in ["Combine independent constraints", "not independent facts",
                     "particular inference depends on it", "Static provenance",
                     "does not automatically reject", "already classified or unknown cells",
                     "Do not fit arbitrary error bands", "independent role and extent evidence",
                     "not irrevocable vetoes", "materially different method even with unchanged ROM",
                     "strongest competing interpretation", "coordinator must inspect",
                     "Reuse a sound completed review", "stop incomplete"]:
        assert required in policy, required
    goal_policy = " ".join((work / "goal.md").read_text().split())
    for required in ["inspect the worker's compact rejection review",
                     "verifying hashes or counting previous attempts",
                     "Do not wait for a user", "stop incomplete rather than cycling"]:
        assert required in goal_policy, required
    disassembly = (work / "prompts/disassembly.txt").read_text()
    assert "Stop and report incomplete" in disassembly
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
                     "useful evidence"]:
        assert required in disassembly
    assert "Follow ready redirect leads" in disassembly
    assert "Investigation mode:" not in disassembly
    redirect = (work / "prompts/redirect.txt").read_text()
    for required in ["emulator.memory.write", "emulator.registers.set",
                     "Do not reissue", "additional evidence", "attempt history"]:
        assert required in redirect
    for required in ["Investigate connected behavior as a coherent batch",
                     "producing the first ready lead", "actual instruction fetches are not required",
                     "explain why any are deferred", "resume redirect",
                     "No minimum byte quota", "leave classification changes"]:
        assert required in redirect
    for required in ["not fixed byte-range ceilings", "Runtime fetch evidence is not mandatory",
                     "Reuse valid saved evidence"]:
        assert required in disassembly
    for required in ["checkpoint, not a phase stop condition",
                     "Continue with actionable", "independent avenues in this same redirect",
                     "outside selected scope", "same redirect phase"]:
        assert required in redirect
    redirect_policy = " ".join(redirect.split())
    for required in ["unique attempt directory", "never overwrite or discard",
                     "ready redirect lead is not required", "expected discriminating result",
                     "whole-table extent proofs", "terminal exhaustion",
                     "Every remaining range receives a disposition", "natural reachability",
                     "Trace missing selectors backward", "Preserve ROM, map",
                     "final editorial audit", "Ledger groups are bookkeeping",
                     "only prior method labels", "without an empty",
                     "analysis/redirect/<attempt-id>/", "legacy analysis/final-investigation/",
                     "every classification percentage", "no fixed hypothesis count"]:
        assert required in redirect_policy, required
    assert "prompts/final-investigation.txt" not in profiled
    for name in ["goal.md", "workflow.md"]:
        policy = " ".join((work / name).read_text().split())
        for required in ["no percentage threshold", "fixed pass quota", "On resume",
                         "Do not automatically", "unfinished legacy investigation as redirect",
                         "continue redirect directly", "without an empty",
                         "Do not relaunch planning",
                         "not a background process"]:
            assert required in policy, (name, required)
        assert "16384" not in policy and "0.05" not in policy
        assert "prompts/final-investigation.txt" not in policy
        assert policy == " ".join(profiled[name].split()) or name == "workflow.md"
    for name in ["prompts/disassembly.txt", "prompts/redirect.txt",
                 "goal.md", "workflow.md"]:
        contract = " ".join((work / name).read_text().split())
        for required in ["Structural classification and reachability are separate claims",
                         "require reachability evidence only when claiming reachability",
                         "not a universal prerequisite", "from the first disassembly pass",
                         "structural evidence and reachability status separately",
                         "When selected indices are the only evidence",
                         "independent extent proof", "does not establish intended code"]:
            assert required in contract, (name, required)
    commentary = (work / "prompts/commentary.txt").read_text()
    for required in ["ALL currently classified source", "Preserve valid", "prior review credit",
                     "previous commentary under", "ordinary tentative prose",
                     "not a small repair pass", "Reserve audit", "invalidate affected records only"]:
        assert required in commentary
    assert "(???)" not in commentary
    assert "do not assume missing comments mean there are" in redirect
    goal = (work / "goal.md").read_text()
    assert "continue redirect" in goal
    assert "more than closure of its first selected graph" in goal
    assert "do not advance to redirect" in goal
    assert "Context/time interruption is" in goal
    for required in ["separate", "worker context", "compact handoff", "Never run competing",
                     "orchestration pending", "targeted final acceptance checks", "do not require a"]:
        assert required in goal
    for required in ["Every redirect pass investigates", "expected byte gain is small",
                     "run redirect again", "concrete useful next step"]:
        assert required in " ".join(goal.split())
    for required in ["thoroughly but reasonably", "do not stop at an inventory",
                     "no fixed hypothesis count", "Do not chase rabbit holes",
                     "pursue slim chances", "materially different justified method",
                     "Keep each runtime call bounded", "leave classification changes"]:
        assert required in " ".join(redirect.split())
    assert "nominate up to three" not in redirect
    assert "automatically schedule one bounded investigation pass" not in goal
    workflow = (work / "workflow.md").read_text()
    assert "Every redirect investigates" in workflow
    assert "No separate investigation mode or fixed research quota" in " ".join(workflow.split())
    assert "without an immediate, obvious prospect" not in workflow
    assert "low versus medium" not in goal
    assert "Evaluate reasoning settings" not in goal
    assert "After every disassembly pass" in goal
    assert "explicitly print the ROM" in goal
    assert "recording it only in a file is" in goal
    assert goal.index("1. prompts/disassembly") < goal.index("2. prompts/commentary") < goal.index("3. prompts/redirect")
    assert "rejection review" in goal and "report incomplete" in goal
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
