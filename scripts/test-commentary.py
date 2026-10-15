#!/usr/bin/env python3
"""Exercise the commentary lifecycle with a local synthetic ROM only."""
import hashlib
import importlib.util
import json
from pathlib import Path
import subprocess
import tempfile

from behavior_test_support import finish_commentary

ROOT = Path(__file__).resolve().parents[1]
BIN = ROOT / "target/debug/romatlas"
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
    # Start with incomplete upstream scope: neither preparation nor byte identity
    # may silently drop the pending question or promote it to accepted behavior.
    upstream_path = work / "analysis/behavior.json"
    upstream = json.loads(upstream_path.read_text())
    upstream["items"] = [dict(id="B-INHERITED", summary="Synthetic jump destination",
                             status="pending", evidence=["Fixture entry branches to a loop."],
                             dependencies=[], unresolved=["Explain the destination behavior."])]
    upstream["outcome"] = dict(status="incomplete_exhausted", notes="Synthetic unresolved target for handoff test.")
    upstream_path.write_text(json.dumps(upstream))
    preserved = {name: (work / name).read_bytes() for name in
                 ["AGENTS.md", "workflow.md", "analysis/state.json", "maps/current.sym", "analysis/behavior.json"]}
    result = call("python3", SCRIPT, "prepare", work)
    assert json.loads(result.stdout)["byte_identical"]
    assert "codex --enable goals -C '" in result.stderr and "/goal Read commentary/AGENTS.md" in result.stderr
    phase = work / "commentary"
    inherited = json.loads((phase / "behavior.json").read_text())
    assert inherited["items"][0]["id"] == "B-INHERITED"
    assert inherited["items"][0]["unresolved"] == ["Explain the destination behavior."]
    assert inherited["outcome"]["status"] == "pending"
    assert json.loads((phase / "handoff.json").read_text())["behavior"]["outcome"]["status"] == "incomplete_exhausted"
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
    finish_commentary(work)
    final = json.loads(call("python3", SCRIPT, "audit", work).stdout)
    assert final["byte_identical"] and final["reviewed_units"] == 1
    assert final["behavior_items"] == 1 and final["preserved_regions"] == 2
    completed_behavior = json.loads((phase / "behavior.json").read_text())
    broken = json.loads(json.dumps(completed_behavior))
    broken["items"][0]["id"] = "B-REPLACEMENT-CANNOT-ERASE-INHERITED"
    (phase / "behavior.json").write_text(json.dumps(broken))
    call("python3", SCRIPT, "audit", work, ok=False)
    assert json.loads((phase / "audit.json").read_text())["ok"] is False
    (phase / "behavior.json").write_text(json.dumps(completed_behavior))
    refinements = {"version": 1, "phase": "commentary", "entries": [dict(
        id="R001", status="pending", summary="Synthetic proposed correction", evidence=["Fixture correction is not applied."])]}
    (phase / "refinements.json").write_text(json.dumps(refinements))
    call("python3", SCRIPT, "audit", work, ok=False)
    refinements["entries"][0].update(status="reconciled", evidence=["Synthetic identity-preserving correction reconciled with source."])
    (phase / "refinements.json").write_text(json.dumps(refinements))
    call("python3", SCRIPT, "audit", work)
    # Metadata-only migration preserves source, reviews, baselines and current evidence.
    original_source = bank.read_bytes()
    current_review = json.loads((phase / "review.json").read_text())
    migration = json.loads(call("python3", SCRIPT, "upgrade", work).stdout)
    assert migration["preserved_source"] and migration["audit_pending"]
    assert bank.read_bytes() == original_source
    assert (phase / "behavior.json").read_text() == json.dumps(completed_behavior)
    assert json.loads((phase / "review.json").read_text())["regions"] == current_review["regions"]
    assert (phase / "reading-before.asm").read_bytes() == before
    assert (Path(migration["backup"]) / "review.json").exists()
    # Restore the reviewed editorial status after the migration audit.
    (phase / "review.json").write_text(json.dumps(current_review))
    call("python3", SCRIPT, "audit", work)
    missing_resolution = json.loads(json.dumps(completed_behavior))
    missing_resolution["items"][0]["resolutions"] = []
    (phase / "behavior.json").write_text(json.dumps(missing_resolution))
    call("python3", SCRIPT, "audit", work, ok=False)
    (phase / "behavior.json").write_text(json.dumps(completed_behavior))
    for field, value in [("dependencies", ["B-MISSING"]),
                         ("unresolved", ["A new relevant selector is unresolved"]),
                         ("regions", [next(r["id"] for r in current_review["regions"] if r["kind"] == "Unknown")]),
                         ("source", ["missing.asm:1"])]:
        invalid = json.loads(json.dumps(completed_behavior))
        invalid["items"][0][field] = value
        (phase / "behavior.json").write_text(json.dumps(invalid))
        call("python3", SCRIPT, "audit", work, ok=False)
    (phase / "behavior.json").write_text(json.dumps(completed_behavior))
    # Legacy metadata can be migrated without recreating source or losing review.
    legacy_manifest = json.loads((phase / "manifest.json").read_text())
    legacy_manifest.pop("behavior_contract")
    legacy_manifest.pop("handoff_sha256")
    (phase / "manifest.json").write_text(json.dumps(legacy_manifest))
    (phase / "handoff.json").unlink()
    legacy_review = json.loads(json.dumps(current_review))
    for row in legacy_review["regions"]:
        row.pop("treatment")
    (phase / "review.json").write_text(json.dumps(legacy_review))
    call("python3", SCRIPT, "upgrade", work)
    assert bank.read_bytes() == original_source
    assert all(r["treatment"] == "pending" for r in json.loads((phase / "review.json").read_text())["regions"])
    (phase / "review.json").write_text(json.dumps(current_review))
    # Frozen behavioral claims cannot be silently rewritten.
    frozen_handoff = (phase / "handoff.json").read_bytes()
    (phase / "handoff.json").write_bytes(frozen_handoff + b" ")
    call("python3", SCRIPT, "audit", work, ok=False)
    (phase / "handoff.json").write_bytes(frozen_handoff)
    # Unsupported preservation and broken behavior links fail without reclassification.
    invalid_review = json.loads(json.dumps(current_review))
    unknown_row = next(r for r in invalid_review["regions"] if r["kind"] == "Unknown")
    unknown_row["preservation"]["evidence"] = []
    (phase / "review.json").write_text(json.dumps(invalid_review))
    call("python3", SCRIPT, "audit", work, ok=False)
    (phase / "review.json").write_text(json.dumps(current_review))
    review = current_review
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
