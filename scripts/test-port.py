#!/usr/bin/env python3
"""Exercise archive-to-port preparation, locked builds, headless use and accounting."""
import hashlib
import json
import os
import re
from pathlib import Path
import shutil
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]
BIN = ROOT / "target/release/romatlas"
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
    archive = base / "archive"
    call("python3",ROOT / "scripts/archive.py","archive",analysis,"--name","synthetic","--output",archive)
    initial_archive = inventory(archive)
    # Port preparation accepts an archive whose editorial review is still
    # incomplete, while retaining pending source regions in its accounting.
    partial_analysis = base / "partial analysis"
    shutil.copytree(analysis, partial_analysis)
    partial_review_path = partial_analysis / "commentary/review.json"
    partial_review = json.loads(partial_review_path.read_text())
    partial_review["editorial_audit"] = {"status":"pending","notes":""}
    partial_review["regions"][0].update(status="pending", summary="", source=[])
    for unit in partial_review["regions"][0].get("units", []):
        unit.update(status="pending", summary="", source=[])
    partial_review_path.write_text(json.dumps(partial_review))
    partial_archive = base / "partial archive"
    call("python3",ROOT / "scripts/archive.py","archive",partial_analysis,"--name","synthetic","--output",partial_archive)
    assert json.loads((partial_archive / "archive.json").read_text())["editorial_complete"] is False
    partial_work = base / "partial port"
    partial_result, _ = call("python3",SCRIPT,"prepare",partial_archive,"--rom",rom,"--out",partial_work)
    assert partial_result["byte_identical_reference"]
    partial_ledger = json.loads((partial_work / "ledger.json").read_text())
    assert any(row["original_summary"] == "" for row in partial_ledger["regions"])
    work = base / "port workspace with spaces"
    prepare = ["python3",SCRIPT,"prepare",archive,"--rom",rom,"--out",work]
    result, printed = call(*prepare)
    assert result["byte_identical_reference"] and result["scaffold"]
    assert "goal.md" in printed.stderr and "/goal " not in printed.stderr
    assert "Port this entire" in (work / "goal.md").read_text()
    assert "BUILDING.md" in (work / "goal.md").read_text()
    assert not (work / "prompts/resume.md").exists()
    assert "Preserve work; do not reinitialize" in (work / "goal.md").read_text()
    assert "separate library reviewer" in printed.stderr
    assert str(work) in printed.stderr
    assert inventory(archive) == initial_archive and rom.read_bytes() == data
    assert (work / "port/Cargo.lock").is_file()
    assert result["debug_installed"] and result["debug_goal"] == str(work / "debug/goal.md")
    for name in ["DEBUGGING.md", "debug/goal.md", "debug/acceptance.json", "debug/plan.json", "debug/qa.json"]:
        assert (work / name).is_file(), name
    debug_goal = (work / "debug/goal.md").read_text()
    assert "Execute or resume" in debug_goal and "Read debug/goal.md." in debug_goal
    assert "python3 run.py debug" in debug_goal
    for target in re.findall(r"\]\(([^)]+)\)", debug_goal):
        assert (work / "debug" / target).exists(), target
    assert json.loads((work / "debug/acceptance.json").read_text())["required_passes"] == 2
    assert json.loads((work / "debug/plan.json").read_text())["status"] == "planning"
    assert json.loads((work / "debug/qa.json").read_text())["status"] == "not_started"
    assert "Read debug/goal.md." in (work / "README.md").read_text()
    assert str(work / "debug/goal.md") in printed.stderr
    assert "/debug/artifacts/" in (work / ".gitignore").read_text()

    build_guide = (work / "BUILDING.md").read_text()
    assert "synthetic-port.exe" in build_guide and "./target/release/synthetic-port" in build_guide
    assert "__GAME_NAME__" not in build_guide and "__TOOLKIT_PATH__" not in build_guide
    assert str(ROOT / "crates/romatlas-port") in build_guide
    assert '[BUILDING.md](BUILDING.md)' in (work / "README.md").read_text()
    for required in ["LIBRARY_PATH", "macOS", "Linux", "Windows", "--locked", "--exclude='target/'",
                     "path dependency", "No desktop runtime"]:
        assert required in build_guide, required
    for path in ["goal.md", "prompts/start.md", "workflow.md"]:
        assert "BUILDING.md" in (work / path).read_text()
    meta = json.loads((work / "workspace.json").read_text())
    before = json.loads((work / "review/toolkit-before.json").read_text())
    assert before["head"] and "crates/romatlas-port/src/assets.rs" in before["files"]
    assert any(row["kind"] == "Unknown" for row in json.loads((work / "ledger.json").read_text())["regions"])
    uncertainty_file = work / "uncertainties.json"
    uncertainties = json.loads(uncertainty_file.read_text())
    unknown = [r for r in review["regions"] if r["kind"] == "Unknown"]
    assert [(r["offset"], r["length"]) for r in uncertainties["entries"]] == [(r["offset"], r["length"]) for r in unknown]
    assert all(r["status"] == "pending" and r["impact"] == "unknown" for r in uncertainties["entries"])
    assert "reference-updates.md" in (work / "prompts/start.md").read_text()
    assert "preserving code bytes" in (work / "prompts/review.md").read_text().lower()
    for path in ["prompts/start.md","workflow.md"]:
        text = (work / path).read_text()
        assert "whole" in text.lower() or "entire" in text.lower()
    prompt = (work / "prompts/start.md").read_text()
    for required in ["startup","added", "uncommitted","headless","useful","borderline", "No check-in"]:
        assert required in prompt,required
    # Check delivered coordinator/stage contracts as well as preparation.
    architecture = (work / "architecture.md").read_text()
    assert "state owners" in architecture and "algorithms" in architecture
    goal = (work / "goal.md").read_text()
    workflow = (work / "workflow.md").read_text()
    planning = (work / "prompts/plan.md").read_text()
    assert "subagent" in goal and "one-milestone" in goal
    assert "ONE TIME" in workflow and "--final" in workflow
    assert "runs it twice" in workflow and "begin-cycle" in workflow
    assert "stable ID" in planning and "future milestones" in planning
    assert "exactly one" in workflow and "awaiting QA" in prompt
    assert "future milestones stay pending" in prompt
    assert "deferred" in (work / "smoke-test.md").read_text()
    for path in ["goal.md", "workflow.md", "prompts/start.md"]:
        text = (work / path).read_text()
        assert "architecture.md" in text, path
        assert "stop when" in text or "after it passes, stop" in text, path
        assert "unresolved gameplay-affecting behavior prevents completion" not in text.lower(), path
    assert "implemented assumptions" in prompt
    assert "bounded maintainability pass" in " ".join(prompt.split())
    assert "Do not add more validation" in prompt
    assert "source-informed" in prompt and "file-count" in prompt
    references = (work / "prompts/reference-updates.md").read_text()
    assert "implementation assumption" in references and "This procedure is optional" in references
    reviewer = (work / "prompts/review.md").read_text()
    assert "architecture.md" in reviewer and "validation is outside this review" in reviewer
    assert "later explicitly requested device audit" in build_guide
    run = ["python3",work / "run.py"]
    frozen = inventory(work / "reference")
    # Corrections use an independently verified supplement; the frozen baseline
    # and original source inventory remain unchanged even after a map correction.
    assert call(*run,"reference-init")[0]["baseline_unchanged"]
    call(*run,"reference-init",ok=False)
    call(*run,"reference-verify","--revision","unexplained",ok=False)
    assert not (work / "research/current.json").exists()
    assert not (work / "research/revisions/unexplained").exists()
    working = work / "research/working"
    bank = working / "source/bank_000.asm"
    bank.write_text("; Synthetic port discovery: retain original startup jump.\n" + bank.read_text())
    call(BIN,"store","annotate",owned,"--store",working / "state.json","--offset","512","--length","1",
         "--kind","data","--provenance","static","--confidence","0.8")
    changes = json.loads((working / "changes.json").read_text())
    changes["changes"] = [dict(id="fixture-byte",kind="classification",offset=512,length=1,
        before="Unknown",after="Synthetic fixture data; source byte remains preserved",
        port_impact="No synthetic gameplay",evidence=["evidence/fixture.json"])]
    (working / "changes.json").write_text(json.dumps(changes))
    (working / "evidence").mkdir()
    (working / "evidence/fixture.json").write_text(json.dumps(dict(synthetic=True)))
    corrected = call(*run,"reference-verify","--revision","discovery-001")[0]
    assert corrected["byte_identical"] and not corrected["semantic_correctness_assessed"]
    selected = work / "research/current.json"
    selection = selected.read_bytes()
    published = work / "research/revisions/discovery-001"
    assert "Synthetic port discovery" in (published / "reading.asm").read_text()
    assert (published / "evidence/fixture.json").is_file()
    assert inventory(work / "reference") == frozen
    call(*run,"reference-verify","--revision","discovery-001",ok=False)
    # A broken assembly or foreign mapping neither adopts nor publishes a revision.
    text = bank.read_text()
    bank.write_text("INVALID_OPCODE\n" + text)
    call(*run,"reference-verify","--revision","discovery-002",ok=False)
    assert selected.read_bytes() == selection and not (work / "research/revisions/discovery-002").exists()
    bank.write_text(text)
    state_bytes = (working / "state.json").read_bytes()
    bad_state = json.loads(state_bytes)
    bad_state["image_sha256"] = "0" * 64
    (working / "state.json").write_text(json.dumps(bad_state))
    call(*run,"reference-verify","--revision","discovery-002",ok=False)
    assert selected.read_bytes() == selection and not (work / "research/revisions/discovery-002").exists()
    (working / "state.json").write_bytes(state_bytes)
    assert call(*run,"reference-verify","--revision","discovery-002")[0]["byte_identical"]
    assert json.loads((work / "research/revisions/discovery-002/revision.json").read_text())["parent"] == "discovery-001"
    adopted_reading = work / "research/revisions/discovery-002/reading.asm"
    saved_reading = adopted_reading.read_bytes()
    adopted_reading.write_bytes(saved_reading + b"; accidental edit\n")
    assert call(*run,"headless","--frames","0")[0]["ok"]
    call(*run,"audit",ok=False)
    adopted_reading.write_bytes(saved_reading)
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
        row.update(disposition="excluded",summary="Synthetic fixture has no gameplay",equivalents=[])
        for unit in row["units"]:
            unit.update(disposition="replaced",summary="Native control flow replaces synthetic jump",
                        equivalents=["port/src/game.rs:Port"])
    ledger["completion"] = {"status":"complete","notes":"Synthetic accounting exercise, not a real port"}
    ledger_file.write_text(json.dumps(ledger))
    # Finished behavior accounting cannot silently drop or waive unassessed gaps.
    result, _ = call(*run,"audit",ok=False)
    assert "uncertainty" in result["error"]
    for row in uncertainties["entries"]:
        row.update(status="excluded",impact="none",summary="Synthetic fixture has no gameplay consumer",
                   evidence=["Synthetic fixture generator"],validation=["Fixture bounds and accounting"],
                   validation_limits="No real gameplay assessed")
    uncertainty_file.write_text(json.dumps(uncertainties))
    first_uncertainty = uncertainties["entries"][0]
    first_uncertainty.update(status="preserved",impact="behavior")
    uncertainty_file.write_text(json.dumps(uncertainties))
    result, _ = call(*run,"audit",ok=False)
    assert "unknown behavior" in result["error"]
    first_uncertainty.update(status="excluded",impact="none")
    removed = uncertainties["entries"].pop()
    uncertainty_file.write_text(json.dumps(uncertainties))
    result, _ = call(*run,"audit",ok=False)
    assert "removed" in result["error"]
    uncertainties["entries"].append(removed)
    first_uncertainty.update(status="preserved",impact="data",
        summary="Synthetic accounting: original fixture values remain ROM-backed; no gameplay-equivalence claim")
    # A source-informed implemented assumption needs no new runtime-fidelity field
    # or comparison artifact to pass the administrative accounting contract.
    first_uncertainty.update(status="resolved", impact="behavior",
        summary="Implemented synthetic fixture interpretation; boundary behavior uses an explicit source-informed assumption",
        evidence=["Synthetic fixture consumer reasoning; interpretation is an implementation assumption"],
        validation=["Focused synthetic bounds unit test"],
        validation_limits="Original-game fidelity has not been independently assessed")
    uncertainty_file.write_text(json.dumps(uncertainties))
    audited = call(*run,"audit")[0]
    assert audited["accounting_complete"] and audited["units"] == 1
    assert audited["uncertainties"] == len(unknown)
    assert not audited["semantic_correctness_assessed"] and not audited["release_ready"]
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
    # Runtime/build commands need the ROM, not source-accounting artifacts.
    reading = work / "reference/reading.asm"
    original = reading.read_bytes()
    reading.write_bytes(original + b"; changed\n")
    assert call(*run,"headless","--frames","1")[0]["ok"]
    call(*run,"audit",ok=False)
    reading.write_bytes(original)
    reference_backup = work / "reference-backup"
    (work / "reference").rename(reference_backup)
    review_backup = work / "review-backup"
    (work / "review").rename(review_backup)
    shutil.rmtree(work / "checks")
    assert call(*run,"check")[0]["ok"]
    assert call(*run,"headless","--frames","1")[0]["ok"]
    call(*run,"audit",ok=False)
    reference_backup.rename(work / "reference")
    review_backup.rename(work / "review")
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
