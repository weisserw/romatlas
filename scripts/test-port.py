#!/usr/bin/env python3
"""Exercise archive-to-port preparation, locked builds, headless use and accounting."""
import hashlib
import json
import os
from pathlib import Path
import subprocess
import tempfile

import port as port_tools

from behavior_test_support import finish_commentary, finish_port

ROOT = Path(__file__).resolve().parents[1]
BIN = ROOT / "target/debug/romatlas"
SCRIPT = ROOT / "scripts/port.py"


def call(*args, ok=True, env=None):
    result = subprocess.run(list(map(str, args)), capture_output=True, text=True, env=env)
    assert (result.returncode == 0) == ok, (args, result.stdout, result.stderr)
    return json.loads(result.stdout if ok else result.stderr), result


def inventory(root):
    return {str(path.relative_to(root)): hashlib.sha256(path.read_bytes()).hexdigest()
            for path in root.rglob("*") if path.is_file()}


call("python3",SCRIPT,"not-a-command",ok=False)
call("python3",SCRIPT,"prepare","missing-archive",ok=False)

with tempfile.TemporaryDirectory(prefix="romatlas-port-") as temporary:
    base = Path(temporary)
    data = bytearray(0x8000)
    data[0x100:0x103] = bytes([0xc3,0x50,1])
    data[0x150:0x153] = bytes([0,0x18,0xfd])
    data[0x14d] = (-sum(data[0x134:0x14d]) - 25) & 255
    rom = base / "synthetic owned.gb"
    rom.write_bytes(data)
    analysis = base / "analysis"
    call("python3",ROOT / "scripts/disassemble.py",rom,"--workspace",analysis,"--json")
    owned = analysis / "input/game.rom"
    state = analysis / "analysis/state.json"
    call(BIN,"store","init",owned,"--output",state)
    call(BIN,"store","annotate",owned,"--store",state,"--offset","256","--length","3",
         "--kind","code","--provenance","static","--confidence","0.8")
    call(BIN,"store","export",owned,"--store",state,"--output",analysis / "maps/current.sym")
    call("python3",ROOT / "scripts/commentary.py","prepare",analysis,"--json")
    review_file = analysis / "commentary/review.json"
    review = json.loads(review_file.read_text())
    for row in review["regions"]:
        row.update(status="reviewed",summary="Synthetic behavior or reviewed unknown",source=["bank_000.asm:1"])
        if row["kind"] == "Code":
            row["units"] = [{"name":"entry","offset":256,"length":3,"status":"reviewed",
                             "summary":"Synthetic jump","source":["bank_000.asm:1"]}]
    review["editorial_audit"] = {"status":"reviewed","notes":"Synthetic review complete"}
    review_file.write_text(json.dumps(review))
    finish_commentary(analysis)
    inherited_refinement = dict(id="R-UPSTREAM", status="reconciled", summary="Synthetic upstream refinement",
                                evidence=["Fixture reconciliation completed before archival."])
    (analysis / "commentary/refinements.json").write_text(json.dumps(dict(version=1, phase="commentary", entries=[inherited_refinement])))
    archive = base / "archive"
    call("python3",ROOT / "scripts/archive.py","archive",analysis,"--name","synthetic","--output",archive)
    initial_archive = inventory(archive)
    work = base / "port workspace with spaces"
    prepare = ["python3",SCRIPT,"prepare",archive,"--rom",rom,"--out",work]
    result, printed = call(*prepare)
    assert result["byte_identical_reference"] and result["scaffold"]
    inherited_behavior = json.loads((work / "behavior.json").read_text())
    assert inherited_behavior["items"][0]["id"] == "B001"
    assert inherited_behavior["items"][0]["status"] == "pending"
    assert (work / "analysis").is_dir()
    assert "/goal Port this entire" in printed.stderr and "/goal Resume" in printed.stderr
    assert "separate library reviewer" in printed.stderr
    assert str(work) in printed.stderr
    assert inventory(archive) == initial_archive and rom.read_bytes() == data
    assert (work / "port/Cargo.lock").is_file()
    build_guide = (work / "BUILDING.md").read_text()
    assert "synthetic-port.exe" in build_guide and "./target/release/synthetic-port" in build_guide
    assert "__GAME_NAME__" not in build_guide and "__TOOLKIT_PATH__" not in build_guide
    assert str(ROOT / "crates/romatlas-port") in build_guide
    assert '[BUILDING.md](BUILDING.md)' in (work / "README.md").read_text()
    for required in ["LIBRARY_PATH", "macOS", "Linux", "Windows", "--locked", "--exclude='target/'",
                     "path dependency", "No desktop runtime"]:
        assert required in build_guide, required
    for path in ["prompts/start.md", "prompts/resume.md", "workflow.md"]:
        assert "BUILDING.md" in (work / path).read_text()
    meta = json.loads((work / "workspace.json").read_text())
    before = json.loads((work / "review/toolkit-before.json").read_text())
    assert before["head"] and "crates/romatlas-port/src/assets.rs" in before["files"]
    assert any(row["kind"] == "Unknown" for row in json.loads((work / "ledger.json").read_text())["regions"])
    for path in ["AGENTS.md","prompts/start.md","prompts/resume.md","workflow.md"]:
        text = (work / path).read_text()
        assert "whole" in text.lower() or "entire" in text.lower()
    prompt = (work / "prompts/start.md").read_text()
    for required in ["startup","added", "uncommitted","headless","useful","borderline", "No check-in"]:
        assert required in prompt,required
    run = ["python3",work / "run.py"]
    frozen = inventory(work / "reference")
    call(*prepare,ok=False)
    assert inventory(work / "reference") == frozen
    call("python3",SCRIPT,"prepare",archive,"--rom",rom,"--out",ROOT / "build/disallowed-port",ok=False)
    assert not (ROOT / "build/disallowed-port").exists()
    assert call(*run,"headless","--frames","5")[0]["scaffold"]
    call(*run,"headless","--frames","-1",ok=False)
    assert call(*run,"check")[0]["ok"]  # Real fmt/clippy/test/SDL2 linking, no display.
    no_display = {k:v for k,v in os.environ.items() if k not in {"DISPLAY","WAYLAND_DISPLAY"}}
    rejected, _ = call(*run,"desktop",ok=False,env=no_display)
    assert "graphical session" in rejected["error"]
    call(*run,"audit",ok=False)
    assert json.loads((work / "checks/audit.json").read_text())["ok"] is False
    # The audit measures explained accounting, not semantic truth. Synthetic
    # exclusions are deliberately accepted without a runtime reachability proof.
    ledger_file = work / "ledger.json"
    ledger = json.loads(ledger_file.read_text())
    for row in ledger["regions"]:
        if row["original_treatment"] == "preserved":
            row.update(disposition="preserved", summary="Synthetic preservation reviewed for port fixture",
                       preservation=row["original_preservation"], equivalents=[])
        else:
            row.update(disposition="implemented", summary="Synthetic entry accounting exercise",
                       equivalents=["port/src/game.rs:Port"])
        for unit in row["units"]:
            unit.update(disposition="replaced",summary="Native control flow replaces synthetic jump",
                        equivalents=["port/src/game.rs:Port"])
    ledger["completion"] = {"status":"complete","notes":"Synthetic accounting exercise, not a real port"}
    ledger_file.write_text(json.dumps(ledger))
    # Source ledger completion cannot waive inherited behavior obligations.
    call(*run,"audit",ok=False)
    behavior_record = finish_port(work)
    audited = call(*run,"audit")[0]
    assert audited["behavior_items"] == 1
    assert json.loads((work / "reference/archive/commentary/behavior.json").read_text())["items"][0]["status"] == "accounted"
    behavior_record["items"][0]["id"] = "B-REPLACEMENT-CANNOT-ERASE-INHERITED"
    (work / "behavior.json").write_text(json.dumps(behavior_record))
    call(*run,"audit",ok=False)
    inherited = json.loads((work / "reference/archive/commentary/behavior.json").read_text())
    inherited["phase"] = "port"
    (work / "behavior.json").write_text(json.dumps(inherited))
    finish_port(work)
    assert audited["accounting_complete"] and audited["units"] == 1
    assert not audited["semantic_correctness_assessed"] and not audited["release_ready"]
    refinement_path = work / "refinements.json"
    pending_refinement = {"version":1,"phase":"port","entries":[inherited_refinement, dict(id="R-LATE",status="pending",
                          summary="New static asset extent",evidence=["Fixture pending correction"])]}
    refinement_path.write_text(json.dumps(pending_refinement))
    call(*run,"audit",ok=False)
    pending_refinement["entries"][-1].update(status="reconciled",evidence=["Synthetic asset recipe reconciled with the new extent."])
    refinement_path.write_text(json.dumps(pending_refinement))
    # Later static analysis may add a unit/behavior inside a formerly opaque row.
    # This validates accounting extensibility, not semantic correctness of a port.
    expanded_ledger = json.loads(json.dumps(ledger))
    opaque = next(r for r in expanded_ledger["regions"] if r["original_treatment"] == "preserved")
    opaque.update(disposition="replaced", summary="Newly understood fixture block",
                  equivalents=["port/src/game.rs:Port"])
    opaque["units"].append(dict(id="late-discovery", name="DiscoveredBlock", offset=opaque["offset"],
                               length=1, disposition="replaced", summary="Synthetic late interpretation",
                               equivalents=["port/src/game.rs:Port"]))
    expanded_behavior = json.loads((work / "behavior.json").read_text())
    expanded_behavior["items"].append(dict(id="B-LATE", summary="Late-discovered fixture behavior",
        status="accounted", evidence=["Synthetic static interpretation of a previously opaque region."],
        dependencies=["B001"], unresolved=[], regions=[opaque["id"]],
        equivalents=["port/src/game.rs:Port"], tests=["Synthetic fixture lifecycle check only."]))
    ledger_file.write_text(json.dumps(expanded_ledger))
    (work / "behavior.json").write_text(json.dumps(expanded_behavior))
    assert port_tools.validate_ledger(work, meta)["behavior_items"] == 2
    ledger_file.write_text(json.dumps(ledger))
    expanded_behavior["items"].pop()
    (work / "behavior.json").write_text(json.dumps(expanded_behavior))
    # Metadata upgrade never rewrites port code or the immutable reference.
    before_port = inventory(work / "port/src")
    before_behavior = (work / "behavior.json").read_bytes()
    migrated = call(*run,"upgrade")[0]
    assert migrated["preserved_port"] and migrated["audit_pending"]
    assert inventory(work / "port/src") == before_port
    assert (work / "behavior.json").read_bytes() == before_behavior
    assert inventory(work / "reference") == frozen
    assert (Path(migrated["backup"]) / "ledger.json").is_file()
    ledger["completion"]["status"] = "complete"
    ledger_file.write_text(json.dumps(ledger))
    old = ledger["regions"][0]["offset"]
    ledger["regions"][0]["offset"] = old + 1
    ledger_file.write_text(json.dumps(ledger))
    call(*run,"audit",ok=False)
    assert not json.loads((work / "checks/audit.json").read_text())["ok"]
    ledger["regions"][0]["offset"] = old
    unit_row = next(row for row in ledger["regions"] if row["units"])
    unit_row["units"] = []
    ledger_file.write_text(json.dumps(ledger))
    call(*run,"audit",ok=False)
    # Overriding the demonstration tile validates and loads a file, no window.
    assets = base / "custom assets"
    assets.mkdir()
    (assets / "tile.bin").write_bytes(bytes([0xff,0] * 8))
    (assets / "tiles.json").write_text(json.dumps({"version":1,"entries":[{"id":"demo-tile","file":"tile.bin"}]}))
    assert call(*run,"headless","--frames","1","--assets",assets)[0]["ok"]
    (assets / "tile.bin").write_bytes(b"bad")
    call(*run,"headless","--assets",assets,ok=False)
    # Frozen source and local ROM identity are checked before execution/build.
    reading = work / "reference/reading.asm"
    original = reading.read_bytes()
    reading.write_bytes(original + b"; changed\n")
    call(*run,"headless",ok=False)
    reading.write_bytes(original)
    (work / "input/game.rom").write_bytes(bytes([1]) + data[1:])
    call(*run,"headless",ok=False)
    wrong = base / "wrong.gb"
    wrong.write_bytes(bytes([1]) + data[1:])
    rejected_out = base / "rejected"
    call("python3",SCRIPT,"prepare",archive,"--rom",wrong,"--out",rejected_out,ok=False)
    assert not rejected_out.exists()
    # Quiet preparation still returns JSON but prints no instructions.
    quiet = base / "quiet"
    _, quiet_result = call("python3",SCRIPT,"prepare",archive,"--rom",rom,"--out",quiet,"--json")
    assert quiet_result.stderr == ""
    editorial = archive / "editorial.json"
    editorial.write_text(editorial.read_text() + " ")
    call("python3",SCRIPT,"prepare",archive,"--rom",rom,"--out",rejected_out,ok=False)
    assert not rejected_out.exists()

print("port preparation, archive fidelity, locked headless/desktop builds, overrides, accounting and rejection regressions passed")
