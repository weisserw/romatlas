"""Maintain a compact investigation queue and reusable evidence-batch reports.

Installed as analysis/work.py. Never annotates maps or changes ROM content.
"""
import argparse
from datetime import datetime, timezone
import hashlib
import json
import math
from pathlib import Path
import re
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]
KINDS = ("Code", "Data", "Text", "Image", "Padding", "Unknown")


def read(path):
    return json.loads(path.read_text())


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def write(path, value):
    path.parent.mkdir(parents=True, exist_ok=True)
    text = value if isinstance(value, str) else json.dumps(value, indent=2) + "\n"
    with tempfile.NamedTemporaryFile(mode="w", dir=path.parent, delete=False) as file:
        temporary = Path(file.name)
        file.write(text)
    temporary.replace(path)


def audit():
    meta = read(ROOT / "workspace.json")
    rom = ROOT / meta["rom"]
    if sha(rom) != meta["sha256"]:
        raise ValueError("input ROM changed")
    path = ROOT / "analysis/state.json"
    subprocess.run([meta["binary"], "store", "validate", str(rom), "--store", str(path)],
                   cwd=ROOT, check=True, capture_output=True, text=True)
    state = read(path)
    size = state["image_size"]
    banks = meta["inspection"]["banks"]
    width = size // banks
    per_bank = [dict(bank=f"{bank:02x}", **{kind: 0 for kind in KINDS}) for bank in range(banks)]
    intervals = sorted((r["start"]["offset"], r["length"], r["kind"])
                       for r in state["regions"].values() if r["kind"] != "Unknown")
    gaps = []
    cursor = 0

    def add(start, length, kind):
        end = start + length
        while start < end:
            bank = start // width
            stop = min(end, (bank + 1) * width)
            per_bank[bank][kind] += stop - start
            if kind == "Unknown":
                gaps.append(dict(bank=f"{bank:02x}", offset=start,
                                 end_exclusive=stop, length=stop-start))
            start = stop

    for start, length, kind in intervals:
        if start < cursor or start + length > size:
            raise ValueError("invalid overlapping/out-of-image state")
        add(cursor, start-cursor, "Unknown")
        add(start, length, kind)
        cursor = start + length
    add(cursor, size-cursor, "Unknown")
    totals = {kind: sum(bank[kind] for bank in per_bank) for kind in KINDS}
    if sum(totals.values()) != size:
        raise ValueError("classification accounting does not cover the image")
    metadata = state.get("region_metadata", {})
    explained = sum(bool(metadata.get(str(id), {}).get("role")) and
                    bool(metadata.get(str(id), {}).get("evidence")) for id in state["regions"])
    return dict(state_sha256=sha(path), rom_sha256=meta["sha256"],
                region_count=len(state["regions"]), explained_regions=explained,
                per_bank=per_bank, unknown_ranges=gaps, totals=totals)


def refresh():
    current = audit()
    path = ROOT / "analysis/work-queue.json"
    old = read(path) if path.exists() else {"tickets": [], "history": []}
    previous = {t["id"]: t for t in old["tickets"]}
    tickets = []
    for gap in current["unknown_ranges"]:
        id = f"U-B{gap['bank']}-{gap['offset']:05x}"
        ticket = previous.pop(id, None)
        if ticket is None:
            parents = [t["id"] for t in old["tickets"]
                       if t["offset"] < gap["end_exclusive"] and gap["offset"] < t["end_exclusive"]]
            ticket = dict(id=id, status="untriaged", hypothesis="", experiment="",
                          expected_outcome="", evidence=[], dependencies=[], revisit_when="",
                          expected_bytes=gap["length"], estimated_minutes=15, parents=parents)
        elif ticket["end_exclusive"] != gap["end_exclusive"]:
            old.setdefault("history", []).append(dict(ticket, superseded_by=id))
            ticket = dict(ticket, status="needs_review", expected_bytes=gap["length"])
        if (ticket.get("dependencies") and old.get("state_sha256") != current["state_sha256"]):
            ticket = dict(ticket, status="needs_review", dependency_review_required=True)
        ticket.update(gap)
        tickets.append(ticket)
    history = old.get("history", []) + [dict(t, status="superseded_by_current_map") for t in previous.values()]
    # Behavioral relevance precedes byte gain; untriaged intervals still require review.
    tickets.sort(key=lambda t: (t["status"] in ("blocked", "waiting_dependency", "deferred"),
                               not bool(t.get("behavioral_relevance")),
                               -t["expected_bytes"] / max(t["estimated_minutes"], 1), t["offset"]))
    result = dict(version=1, state_sha256=current["state_sha256"],
                  totals=current["totals"], tickets=tickets, history=history)
    write(path, result)
    write(ROOT / "analysis/current-audit.json", current)
    if not (ROOT / "analysis/last-batch-audit.json").exists():
        write(ROOT / "analysis/last-batch-audit.json", current)
    lines = ["Current investigation frontier. Ranking is a hypothesis, not classification evidence.\n",
             "Full history remains in investigation.md and work-queue.json.\n",
             "Primary acceptance: analysis/behavior.md and workflow.md; byte totals are diagnostic.\n",
             "Deferred intervals remain unknown and require final disposition review.\n",
             f"State: {current['state_sha256']}\n",
             f"Unknown: {current['totals']['Unknown']} bytes in {len(tickets)} intervals.\n\n",
             "| Entry | Bytes | Status | Next experiment |\n| --- | ---: | --- | --- |\n"]
    for t in [t for t in tickets if t["status"] != "deferred"][:20]:
        experiment = (t["experiment"] or "Triage consumers, format and build-residue hypotheses").replace("|", "/").replace("\n", " ")
        lines.append(f"| {t['id']} | {t['length']} | {t['status']} | {experiment} |\n")
    write(ROOT / "analysis/frontier.md", "".join(lines))
    return result


def plan(path):
    proposal = read(path)
    for field in ["id", "hypothesis", "experiment", "expected_outcome", "revisit_when"]:
        if not isinstance(proposal.get(field), str) or not proposal[field].strip():
            raise ValueError(f"plan requires {field}")
    for field in ["evidence", "dependencies"]:
        if not isinstance(proposal.get(field), list):
            raise ValueError(f"plan requires {field} list")
    for field in ["expected_bytes", "estimated_minutes"]:
        if (type(proposal.get(field)) not in (int, float)
                or not math.isfinite(proposal[field]) or proposal[field] < 0):
            raise ValueError(f"invalid {field}")
    q = refresh()
    ticket = next((t for t in q["tickets"] if t["id"] == proposal["id"]), None)
    if ticket is None:
        raise ValueError("plan entry is not a current unknown interval")
    q["history"].append(dict(ticket, replaced_by_plan=True))
    for key in ["hypothesis", "experiment", "expected_outcome", "revisit_when", "evidence",
                "dependencies", "expected_bytes", "estimated_minutes"]:
        ticket[key] = proposal[key]
    if "behavioral_relevance" in proposal:
        if not isinstance(proposal["behavioral_relevance"], str):
            raise ValueError("behavioral_relevance must be a string")
        ticket["behavioral_relevance"] = proposal["behavioral_relevance"]
    ticket["status"] = "ready"
    write(ROOT / "analysis/work-queue.json", q)
    return refresh()


def record(path):
    batch = read(path)
    id = batch.get("id", "")
    if not re.fullmatch(r"[A-Za-z0-9][A-Za-z0-9_-]*", id):
        raise ValueError("batch id must be a simple new name")
    if not batch.get("summary") or not batch.get("evidence"):
        raise ValueError("batch needs a summary and local evidence paths")
    for name in batch["evidence"]:
        candidate = (ROOT / name).resolve()
        if ROOT not in candidate.parents or not candidate.is_file():
            raise ValueError(f"missing/non-local evidence: {name}")
    report_path = ROOT / "analysis/batches" / (id + ".json")
    if report_path.exists():
        raise ValueError("batch already recorded; preserve its history")
    previous_path = ROOT / "analysis/last-batch-audit.json"
    previous = read(previous_path) if previous_path.exists() else None
    q = refresh()
    current = read(ROOT / "analysis/current-audit.json")
    for update in batch.get("ticket_updates", []):
        ticket = next((t for t in q["tickets"] if t["id"] == update["id"]), None)
        if ticket is None:
            # A classified/split ticket belongs to history, not a fabricated new gap.
            q["history"].append(dict(update, recorded_in=id))
            continue
        if update.get("status") not in ("ready", "needs_review", "blocked", "waiting_dependency", "deferred"):
            raise ValueError("invalid ticket status")
        if not update.get("result") or not update.get("revisit_when"):
            raise ValueError("ticket result and revisit condition are required")
        if update.get("status") == "deferred":
            if not isinstance(update.get("disposition_evidence"), list) or not update["disposition_evidence"]:
                raise ValueError("deferred ticket requires disposition_evidence paths")
            for name in update["disposition_evidence"]:
                candidate = (ROOT / name).resolve()
                if ROOT not in candidate.parents or not candidate.is_file():
                    raise ValueError(f"missing/non-local disposition evidence: {name}")
        # Evidence existence is checked; the helper cannot judge semantic sufficiency.
        q["history"].append(dict(ticket, updated_in=id))
        if update.get("status") == "deferred":
            ticket["disposition_evidence"] = update["disposition_evidence"]
        ticket.update({k: update[k] for k in ["status", "result", "revisit_when"]})
        if "experiment" in update:
            ticket["experiment"] = update["experiment"]
    verification = None
    if batch.get("verification"):
        verification = read(ROOT / batch["verification"])
        if (verification.get("ok") is not True or verification.get("byte_identical") is not True
                or verification.get("rom_sha256") != current["rom_sha256"]
                or verification.get("map_sha256") != sha(ROOT / "maps/current.sym")):
            raise ValueError("verification is not for the current ROM/export; keep it historical")
    report = dict(batch, recorded_at=datetime.now(timezone.utc).isoformat(),
                  audit=current, verification_current=verification is not None,
                  newly_classified=(previous["totals"]["Unknown"]-current["totals"]["Unknown"]
                                    if previous else None))
    report_path.parent.mkdir(exist_ok=True)
    write(report_path, report)
    write(ROOT / "analysis/work-queue.json", q)
    marker = f"\n## Batch {id}\n"
    delta = (marker + f"\n{batch['summary']}\n\n"
             + f"Evidence/accounting: [batches/{id}.json](batches/{id}.json). "
             + f"Unknown: {current['totals']['Unknown']}; newly classified: {report['newly_classified']}. "
             + f"Current export byte identity verified: {report['verification_current']}.\n"
             + "Behavioral accounting and unknown dispositions determine completion; byte totals are diagnostic.\n")
    for name in ["progress.md", "investigation.md"]:
        target = ROOT / "analysis" / name
        old = target.read_text() if target.exists() else ""
        write(target, old + delta)
    write(previous_path, current)
    refresh()
    return {"ok": True, "report": str(report_path), "newly_classified": report["newly_classified"]}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("command", choices=["refresh", "plan", "record"])
    parser.add_argument("file", nargs="?", type=Path)
    args = parser.parse_args()
    if args.command == "refresh":
        q = refresh()
        print(json.dumps({"ok": True, "unknown_bytes": q["totals"]["Unknown"],
                          "tickets": len(q["tickets"]), "frontier": "analysis/frontier.md"}))
    elif args.file is None:
        parser.error("plan/record require a JSON file")
    elif args.command == "plan":
        plan(args.file)
        print(json.dumps({"ok": True, "frontier": "analysis/frontier.md"}))
    else:
        print(json.dumps(record(args.file)))


if __name__ == "__main__":
    try:
        main()
    except (OSError, ValueError, KeyError, subprocess.CalledProcessError) as error:
        raise SystemExit(json.dumps({"ok": False, "error": str(error)}))
