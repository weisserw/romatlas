#!/usr/bin/env python3
"""Synthetic integration regressions for caching, upgrades and investigation accounting."""
import hashlib
import importlib.util
import json
from pathlib import Path
import signal
import subprocess
import sys
import tempfile
import urllib.request
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[1]
BIN = ROOT / "target/debug/romatlas"
sys.path.insert(0, str(ROOT / "scripts"))
import disassemble


def module(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    result = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(result)
    return result


def rejects(fn):
    try:
        fn()
    except (ValueError, RuntimeError):
        return
    raise AssertionError("expected rejection")


with tempfile.TemporaryDirectory(prefix="romatlas-efficient-workspace-") as temp:
    base = Path(temp)
    data = bytearray(0x8000)
    data[0x100:0x103] = bytes([0xc3, 0x50, 1])
    data[0x150:0x153] = bytes([0x00, 0x18, 0xfd])
    data[0x14d] = (-sum(data[0x134:0x14d])-25) & 255
    rom = base / "test.gb"
    rom.write_bytes(data)
    work = base / "workspace"
    disassemble.initialize(rom, work, BIN)
    session = module("cached_session", work / "analysis/session.py")
    with subprocess.Popen([sys.executable, "analysis/session.py", "--port", "0",
                           "--journal", "traces/test.jsonl"], cwd=work,
                          stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True) as bridge:
        try:
            session.URL = bridge.stdout.readline().strip().split()[-1]

            def requests():
                with urllib.request.urlopen(session.URL, timeout=10) as response:
                    return json.load(response)["requests"]

            result = session.call("analysis.decode", offset=256, count=1)
            assert result["items"][0]["instruction"]["target"] == 336
            count = requests()
            assert session.call("analysis.decode", count=1, offset=256) == result
            # Disk cache also avoids an RPC in a separately loaded client module.
            other = module("second_client", work / "analysis/session.py")
            other.URL = session.URL
            assert other.call("analysis.decode", offset=256, count=1) == result
            assert requests() == count
            session.call("map.annotate", offset=256, length=3, kind="data",
                         role="synthetic barrier", evidence="test")
            assert session.call("analysis.decode", offset=256, count=1) == result
            discovery = session.call("analysis.discover_context", entrypoints=[dict(offset=256,
                                      cpu_context=dict(cpu_address=256, fixed_bank=0, switch_bank=1))],
                                     use_observed=False)
            assert discovery["total"] == 0  # Map-sensitive traversal was not cached.
            count = requests()
            rejects(lambda: session.call("analysis.decode", offset=256, count=4097))
            rejects(lambda: session.call("analysis.decode", offset=256, count=4097))
            assert requests() == count + 2  # Failures are never cached.
            rejects(lambda: session.call("rom.open", path=str(rom)))
            session.call("map.save", path="analysis/state.json")
            session.call("map.export", path="maps/current.sym", include_sym=False)
            helper = module("work_helper", work / "analysis/work.py")
            q = helper.refresh()
            assert q["totals"]["Unknown"] == len(data) - 3
            plan_path = work / "analysis/plan.json"
            proposal = dict(id=q["tickets"][0]["id"], hypothesis="synthetic data family",
                            experiment="inspect test fixture", expected_outcome="bounded data",
                            revisit_when="new caller", evidence=[], dependencies=[],
                            expected_bytes=4, estimated_minutes=1)
            plan_path.write_text(json.dumps(proposal))
            helper.plan(plan_path)
            session.call("map.annotate", offset=512, length=4, kind="data",
                         role="test bytes", evidence="fixture")
            session.call("map.save", path="analysis/state.json")
            session.call("map.export", path="maps/current.sym", include_sym=False)
            helper.refresh()  # Refresh must not erase the last reported accounting baseline.
            batch_path = work / "analysis/batch.json"
            batch_path.write_text(json.dumps(dict(id="test-batch", summary="Four proven bytes",
                                                  evidence=["analysis/plan.json"])))
            assert helper.record(batch_path)["newly_classified"] == 4
            # A deferred interval stays unknown and does not certify completion.
            q = helper.refresh()
            target = q["tickets"][0]
            deferred = dict(id=target["id"], status="deferred", result="Synthetic triage only",
                            revisit_when="new behavioral dependency")
            deferred_path = work / "analysis/defer.json"
            deferred_batch = dict(id="deferred-batch", summary="Review unknown disposition",
                                  evidence=["analysis/plan.json"], ticket_updates=[deferred])
            deferred_path.write_text(json.dumps(deferred_batch))
            rejects(lambda: helper.record(deferred_path))  # Evidence required.
            deferred["disposition_evidence"] = ["analysis/missing.json"]
            deferred_path.write_text(json.dumps(deferred_batch))
            rejects(lambda: helper.record(deferred_path))
            deferred["disposition_evidence"] = ["analysis/plan.json"]
            deferred_path.write_text(json.dumps(deferred_batch))
            helper.record(deferred_path)
            q2 = helper.refresh()
            assert q2["totals"] == q["totals"]
            assert next(t for t in q2["tickets"] if t["id"] == target["id"])["status"] == "deferred"
            assert target["id"] not in (work / "analysis/frontier.md").read_text()
            assert "Whole-ROM completion" not in (work / "analysis/progress.md").read_text()
            # Explicit planning reopens a deferred interval; relevance outranks size.
            proposal.update(id=target["id"], behavioral_relevance="B001 unresolved loader",
                            expected_bytes=0)
            plan_path.write_text(json.dumps(proposal))
            helper.plan(plan_path)
            assert helper.refresh()["tickets"][0]["id"] == target["id"]
            progress = (work / "analysis/progress.md").read_bytes()
            rejects(lambda: helper.record(batch_path))
            assert (work / "analysis/progress.md").read_bytes() == progress
            bad_report = work / "analysis/bad-verification.json"
            bad_report.write_text(json.dumps(dict(ok=True, byte_identical=True,
                                                  rom_sha256="wrong", map_sha256="wrong")))
            batch_path.write_text(json.dumps(dict(id="bad-batch", summary="Invalid verification",
                                                  evidence=["analysis/plan.json"],
                                                  verification="analysis/bad-verification.json")))
            rejects(lambda: helper.record(batch_path))
            assert not (work / "analysis/batches/bad-batch.json").exists()
            assert (work / "analysis/progress.md").read_bytes() == progress
        finally:
            bridge.send_signal(signal.SIGINT)
            bridge.communicate(timeout=15)

    # Offline hits still enforce local image identity; binary changes use a new namespace.
    original = (work / "input/game.rom").read_bytes()
    (work / "input/game.rom").write_bytes(b"changed")
    rejects(lambda: session.cached_decode(dict(offset=256, count=1), lambda: result))
    (work / "input/game.rom").write_bytes(original)
    meta_path = work / "workspace.json"
    meta = json.loads(meta_path.read_text())
    fake_binary = base / "different-toolkit"
    fake_binary.write_bytes(b"different decoder version")
    meta_path.write_text(json.dumps(dict(meta, binary=str(fake_binary))))
    changed = {"items": [], "next_offset": 123}
    assert session.cached_decode(dict(offset=256, count=1), lambda: changed) == changed
    session.MAX_ENTRIES = 2
    for offset in range(1000, 1005):
        session.cached_decode(dict(offset=offset, count=0), lambda: changed)
    assert session.cache_stats()["entries"] <= 2
    meta_path.write_text(json.dumps(meta))

    upgrader = module("upgrader", ROOT / "scripts/upgrade-disassembly.py")
    protected = ["input/game.rom", "analysis/state.json", "maps/current.sym",
                 "analysis/progress.md", "analysis/investigation.md", "analysis/behavior.md", "analysis/behavior.json"]
    before = {name: (work / name).read_bytes() for name in protected}
    (work / "analysis/behavior.md").write_text("Version: 2. Existing evidence must survive.\n")
    inventory_path = work / "analysis/behavior.json"
    inventory = json.loads(inventory_path.read_text())
    inventory["revision"] = 7
    inventory_path.write_text(json.dumps(inventory))
    before["analysis/behavior.json"] = inventory_path.read_bytes()
    before["analysis/behavior.md"] = (work / "analysis/behavior.md").read_bytes()
    old_runner = (work / "run.py").read_bytes()
    preview = upgrader.upgrade(work, BIN)
    assert not preview["applied"] and not (work / "analysis/setup-backups").exists()
    # Do not stop or depend on real workspace bridges using the default port.
    with patch.object(upgrader.socket.socket, "connect_ex", return_value=0):
        rejects(lambda: upgrader.upgrade(work, BIN, apply=True))
    with patch.object(upgrader.socket.socket, "connect_ex", return_value=1):
        upgrade = upgrader.upgrade(work, BIN, apply=True)
    assert upgrade["protected_files_unchanged"]
    assert (Path(upgrade["backup"]) / "run.py").read_bytes() == old_runner
    assert all((work / name).read_bytes() == content for name, content in before.items())
    assert "Operational bounds do not authorize" in (work / "prompts/goal.md").read_text()
    assert "Every behavior-inventory item" in (work / "workflow.md").read_text()
    assert json.loads((work / "workspace.json").read_text())["workflow_revision"] == 4
    # Legacy workspaces get an empty, explicitly unaudited inventory.
    (work / "analysis/behavior.md").unlink()
    with patch.object(upgrader.socket.socket, "connect_ex", return_value=1):
        upgrader.upgrade(work, BIN, apply=True)
    assert "Not yet audited" in (work / "analysis/behavior.md").read_text()
    assert hashlib.sha256((work / "input/game.rom").read_bytes()).hexdigest() == meta["sha256"]

print("decode cache isolation, RPC bypass, map barriers, accounting and backed-up upgrade passed")
