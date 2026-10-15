"""Synthetic lifecycle fixtures; these records do not prove game semantics."""
import json
from pathlib import Path


def finish_commentary(work):
    phase = Path(work) / "commentary"
    review_path = phase / "review.json"
    review = json.loads(review_path.read_text())
    for row in review["regions"]:
        row["treatment"] = "semantic" if row["kind"] != "Unknown" else "preserved"
        if row["treatment"] == "preserved":
            row["preservation"] = dict(
                rationale="Synthetic lifecycle fixture's opaque ranges exercise preservation accounting, not semantic acceptance.",
                evidence=["The test deliberately leaves raw ranges to verify exact reconstruction without a classification target."],
                reopen_when="A behavioral dependency is identified by a real investigation.")
    review_path.write_text(json.dumps(review))
    path = phase / "behavior.json"
    record = json.loads(path.read_text())
    if not record["items"]:
        record["items"] = [dict(id="B001", summary="Synthetic entry behavior")]
    record["scope"] = dict(status="reviewed", summary="Synthetic lifecycle scope, not a commercial-game audit.",
                           evidence=["The fixture contains a jump and loop; this exercises record validation only."])
    regions = [r["id"] for r in review["regions"] if r["treatment"] == "semantic"]
    for item in record["items"]:
        item["resolutions"] = [dict(question=q, resolution="Synthetic fixture loop interpretation recorded.",
                                    evidence=["Fixture entry and loop inspected for this lifecycle test."])
                               for q in item.get("unresolved", [])]
        item.update(status="accounted", evidence=["Synthetic entry is identified at the fixture's declared code region."],
                    dependencies=[], unresolved=[], source=["bank_000.asm:1"], regions=regions)
    record["outcome"] = dict(status="complete", notes="Synthetic accounting validation only.")
    path.write_text(json.dumps(record))
    return record


def finish_port(work):
    path = Path(work) / "behavior.json"
    record = json.loads(path.read_text())
    ledger = json.loads((Path(work) / "ledger.json").read_text())
    regions = [r["id"] for r in ledger["regions"] if r["disposition"] in ("implemented", "merged", "replaced")]
    record["scope"]["status"] = "reviewed"
    record["outcome"] = dict(status="complete", notes="Synthetic accounting validation, not a real port.")
    for item in record["items"]:
        item.update(status="accounted", unresolved=[], regions=regions,
                    equivalents=["port/src/game.rs:Port"],
                    tests=["Synthetic lifecycle performs five headless scaffold frames; this is not gameplay validation."])
    path.write_text(json.dumps(record))
    return record
