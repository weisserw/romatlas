"""Local uncertainty accounting and verified reference corrections for native ports."""
import json
from pathlib import Path
import re
import shutil
import tempfile

import commentary as phase


def seeds(review):
    return [{"id": f"source-{r['id']}", "offset": r["offset"], "length": r["length"],
             "source_region_ids": [r["id"]], "impact": "unknown", "status": "pending",
             "summary": "", "consumers": [], "ledger_ids": [r["id"]],
             "evidence": [], "validation": [], "validation_limits": "Not assessed."}
            for r in review["regions"] if r["kind"] == "Unknown"]


def validate_uncertainties(work, meta, ledger):
    path = work / "uncertainties.json"
    if not path.exists() and not meta.get("uncertainty_tracking"):
        return {"uncertainties": 0, "uncertainty_tracking": False}  # Older workspaces.
    data = json.loads(path.read_text())
    if data.get("version") != 1 or data.get("rom_sha256") != meta["sha256"]:
        raise ValueError("uncertainty ledger identity/version mismatch")
    review = json.loads((work / "reference/archive/commentary/review.json").read_text())
    expected = {r["id"]: r for r in seeds(review)}
    ledger_ids = {e["id"] for r in ledger["regions"] for e in [r, *r["units"]]}
    source_ids = {r["id"] for r in review["regions"]}
    size = phase.local(work, meta["rom"]).stat().st_size
    seen = set()
    for row in data["entries"]:
        identity = row["id"]
        if not isinstance(identity, str) or not identity or identity in seen:
            raise ValueError("missing or duplicate uncertainty ID")
        seen.add(identity)
        if identity in expected:
            old = expected.pop(identity)
            if any(row[k] != old[k] for k in ["offset", "length", "source_region_ids"]):
                raise ValueError("original uncertainty range identity changed")
        if type(row["offset"]) is not int or type(row["length"]) is not int or row["offset"] < 0 or row["length"] <= 0 or row["offset"] + row["length"] > size:
            raise ValueError("uncertainty range outside ROM")
        if row["status"] not in {"resolved", "preserved", "excluded"} or row["impact"] not in {"behavior", "data", "none"}:
            raise ValueError(f"unassessed port uncertainty: {identity}")
        if row["status"] == "preserved" and row["impact"] != "data":
            raise ValueError("preserving bytes does not implement unknown behavior")
        if row["status"] == "excluded" and row["impact"] != "none":
            raise ValueError("excluded uncertainty needs an explained no-impact assessment")
        if not isinstance(row["summary"], str) or not row["summary"].strip():
            raise ValueError("uncertainty assessment needs a summary")
        for key in ["consumers", "ledger_ids", "evidence", "validation"]:
            if not isinstance(row[key], list) or any(not isinstance(v, str) or not v.strip() for v in row[key]):
                raise ValueError(f"uncertainty {key} must be a list of nonempty strings")
        if not row["evidence"] or not row["ledger_ids"] or not set(row["ledger_ids"]) <= ledger_ids:
            raise ValueError("uncertainty needs evidence and valid behavior ledger links")
        if not isinstance(row["source_region_ids"], list) or any(not isinstance(v, str) or v not in source_ids for v in row["source_region_ids"]):
            raise ValueError("uncertainty needs valid original source region links")
        if not isinstance(row["validation_limits"], str) or not row["validation_limits"].strip():
            raise ValueError("uncertainty needs explicit validation limits")
    if expected:
        raise ValueError("original uncertainty entries were removed")
    return {"uncertainties": len(seen), "uncertainty_tracking": True}


def current(work, meta):
    pointer = phase.local(work, "research/current.json")
    if not pointer.exists():
        return None
    selected = json.loads(pointer.read_text())
    name = selected["revision"]
    if not isinstance(name, str) or not re.fullmatch(r"[A-Za-z0-9][A-Za-z0-9_-]*", name):
        raise ValueError("invalid reference revision name")
    root = phase.local(work, f"research/revisions/{name}")
    manifest = root / "revision.json"
    if phase.digest(manifest) != selected["sha256"]:
        raise ValueError("adopted reference manifest changed")
    data = json.loads(manifest.read_text())
    if data["rom_sha256"] != meta["sha256"] or data["archive_sha256"] != meta["archive_sha256"]:
        raise ValueError("adopted reference identity changed")
    for path, digest in data["artifacts"].items():
        if phase.digest(phase.local(root, path)) != digest:
            raise ValueError(f"adopted reference changed: {path}")
    return name


def initialize(work, meta):
    destination = phase.local(work, "research/working")
    destination.parent.mkdir(exist_ok=True)
    if destination.exists() or destination.is_symlink():
        raise ValueError("reference working copy already exists; resume it")
    with tempfile.TemporaryDirectory(prefix=".reference-", dir=destination.parent) as temporary:
        stage = Path(temporary) / "working"
        shutil.copytree(work / "reference/source", stage / "source")
        shutil.copyfile(work / "reference/archive/analysis/state.json", stage / "state.json")
        (stage / "reading.asm").write_text(phase.reading(stage / "source"))
        phase.write_json(stage / "changes.json", {"version": 1, "rom_sha256": meta["sha256"], "changes": []})
        stage.rename(destination)
    return {"ok": True, "working": str(destination), "baseline_unchanged": True}


def verify(work, meta, rom, revision):
    if not revision or not re.fullmatch(r"[A-Za-z0-9][A-Za-z0-9_-]*", revision):
        raise ValueError("reference-verify requires --revision with a simple new name")
    working = phase.local(work, "research/working")
    target = phase.local(work, f"research/revisions/{revision}")
    if target.exists() or target.is_symlink():
        raise ValueError("reference revision already exists; use a new name")
    changes_bytes = (working / "changes.json").read_bytes()
    changes = json.loads(changes_bytes)
    if changes.get("version") != 1 or changes.get("rom_sha256") != meta["sha256"]:
        raise ValueError("reference changes identity/version mismatch")
    if not isinstance(changes.get("changes"), list) or not changes["changes"]:
        raise ValueError("reference revision needs an evidence-linked correction")
    seen = set()
    for entry in changes["changes"]:
        identity = entry["id"]
        if not isinstance(identity, str) or not identity or identity in seen:
            raise ValueError("missing or duplicate reference change ID")
        seen.add(identity)
        if entry["kind"] not in {"commentary", "classification"}:
            raise ValueError("invalid reference change kind")
        for k in ["before", "after", "port_impact"]:
            if not isinstance(entry[k], str) or not entry[k].strip():
                raise ValueError(f"reference change needs {k}")
        if type(entry["offset"]) is not int or type(entry["length"]) is not int or entry["offset"] < 0 or entry["length"] <= 0 or entry["offset"] + entry["length"] > rom.stat().st_size:
            raise ValueError("reference change range outside ROM")
        if not isinstance(entry["evidence"], list) or not entry["evidence"] or any(not isinstance(v, str) or not v.strip() for v in entry["evidence"]):
            raise ValueError("reference change needs evidence")
    manifest = json.loads((work / "reference/archive/archive.json").read_text())
    binary = meta.get("binary", str(phase.ROOT / "target/release/romatlas"))
    def call(*args):
        return phase.call(binary, "--platform", manifest["platform"], *args)
    target.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(prefix=".revision-", dir=target.parent) as temporary:
        stage = Path(temporary) / "revision"
        stage.mkdir()
        # Snapshot first: only these exact files are validated and adopted.
        shutil.copytree(working / "source", stage / "source")
        shutil.copyfile(working / "state.json", stage / "state.json")
        (stage / "changes.json").write_bytes(changes_bytes)
        if (working / "evidence").is_dir():
            shutil.copytree(working / "evidence", stage / "evidence")
        (stage / "reading.asm").write_text(phase.reading(stage / "source"))
        call("store", "validate", rom, "--store", stage / "state.json")
        mapping = stage / ("mapping.json" if manifest["platform"] in {"snes", "nes", "sms", "gg", "genesis"} else "mapping.sym")
        call("store", "export", rom, "--store", stage / "state.json", "--output", mapping)
        call("map", "validate", mapping, "--rom", rom)
        call("generate", rom, "--map", mapping, "--out", stage / "structural")
        structural = call("verify", rom, "--generated", stage / "structural")
        source = call("verify", rom, "--generated", stage / "source")
        if not structural.get("byte_identical") or not source.get("byte_identical"):
            raise ValueError("reference revision must rebuild byte-identically")
        # The structural build was checked independently, not adopted as commentary.
        shutil.rmtree(stage / "structural")
        artifacts = {str(p.relative_to(stage)): phase.digest(p) for p in stage.rglob("*") if p.is_file()}
        phase.write_json(stage / "revision.json", {"version": 1, "rom_sha256": meta["sha256"],
            "archive_sha256": meta["archive_sha256"], "parent": current(work, meta),
            "artifacts": artifacts, "structural_verification": structural, "source_verification": source,
            "semantic_correctness_assessed": False})
        pointer = Path(temporary) / "current.json"
        phase.write_json(pointer, {"revision": revision, "sha256": phase.digest(stage / "revision.json")})
        stage.rename(target)
        try:
            pointer.replace(work / "research/current.json")
        except OSError:
            shutil.rmtree(target)  # Only this unadopted, newly created snapshot.
            raise
    return {"ok": True, "revision": revision, "byte_identical": True, "baseline_unchanged": True,
            "semantic_correctness_assessed": False}
