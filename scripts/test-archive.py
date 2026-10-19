#!/usr/bin/env python3
"""Prove archival fidelity, exclusions and failure preservation with a synthetic ROM."""
import hashlib
import json
from pathlib import Path
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]
BIN = ROOT / "target/release/romatlas"
SCRIPT = ROOT / "scripts/archive.py"


def call(*args, ok=True):
    result = subprocess.run(list(map(str, args)), capture_output=True, text=True)
    assert (result.returncode == 0) == ok, (args, result.stdout, result.stderr)
    return json.loads(result.stdout if ok else result.stderr)


def retained(root):
    return {str(p.relative_to(root)): p.read_bytes() for p in root.rglob("*")
            if p.is_file() and p.name not in {"game.o", "romatlas-rebuilt.gb"}}


with tempfile.TemporaryDirectory(prefix="romatlas-archive-") as temporary:
    base = Path(temporary)
    data = bytearray(0x8000)
    data[0x100:0x103] = bytes([0xc3, 0x50, 1])
    data[0x150:0x153] = bytes([0x00, 0x18, 0xfd])
    data[0x14d] = (-sum(data[0x134:0x14d]) - 25) & 255
    rom = base / "Owned synthetic.gb"
    rom.write_bytes(data)
    work = base / "workspace with spaces"
    call("python3", ROOT / "scripts/disassemble.py", rom, "--workspace", work, "--json")
    owned = work / "input/game.rom"
    state = work / "analysis/state.json"
    call(BIN, "store", "init", owned, "--output", state)
    call(BIN, "store", "annotate", owned, "--store", state, "--offset", "256",
         "--length", "3", "--kind", "code", "--provenance", "static", "--confidence", "0.8")
    call(BIN, "store", "export", owned, "--store", state, "--output", work / "maps/current.sym")
    call("python3", ROOT / "scripts/commentary.py", "prepare", work, "--json")
    destination = base / "atlas with spaces" / "gb" / "synthetic" / hashlib.sha256(data).hexdigest()
    save = ["python3", SCRIPT, "archive", work, "--name", "synthetic", "--output", destination]
    incomplete = base / "incomplete archive"
    partial = call("python3", SCRIPT, "archive", work, "--name", "synthetic",
                   "--output", incomplete)
    assert not partial["editorial_complete"]
    assert json.loads((incomplete / "archive.json").read_text())["editorial_complete"] is False
    partial_restore = base / "partial restored"
    assert call("python3", SCRIPT, "restore", incomplete, "--rom", rom,
                "--out", partial_restore)["byte_identical"]
    assert retained(partial_restore / "source") == retained(work / "commentary/source")
    source = work / "commentary/source"
    bank = source / "bank_000.asm"
    bank.write_text("; A synthetic entry with an intentional uncertainty (?).\n" + bank.read_text().replace("jp $0150", "jp SyntheticEntry"))
    (source / "synthetic_constants.inc").write_text("; Author-defined constant.\nDEF SyntheticEntry EQU $0150\n")
    game = source / "game.asm"
    game.write_text('INCLUDE "synthetic_constants.inc"\n' + game.read_text())
    review_path = work / "commentary/review.json"
    review = json.loads(review_path.read_text())
    for row in review["regions"]:
        row.update(status="reviewed", summary="Synthetic reviewed uncertainty", source=["bank_000.asm:1"])
        if row["kind"] == "Code":
            row["units"] = [{"name": "SyntheticEntry", "offset": row["offset"], "length": row["length"],
                             "status": "reviewed", "summary": "Synthetic entry", "source": ["bank_000.asm:1"]}]
    review["editorial_audit"] = {"status": "reviewed", "notes": "Synthetic complete editorial pass"}
    review_path.write_text(json.dumps(review))
    scratch = work / "analysis/raw-results.json"
    scratch.write_text(json.dumps({"bytes": list(data)}))
    (work / "analysis/toolkit-issues.md").write_text("EXCLUDED TOOLKIT ISSUE\n")
    extra = work / "commentary/conclusions.md"
    extra.write_text("Final conclusions reference analysis/raw-results.json, intentionally omitted.\n")
    scratch_doc = work / "analysis/scratch.md"
    scratch_doc.write_text("Temporary scratch document\n")
    call(*save, ok=False)  # Unknown final documents require explicit inclusion.
    save += ["--document", "commentary/conclusions.md", "--omit-document", "analysis/scratch.md"]
    complete_result = call(*save)
    assert complete_result["exact_source"] and complete_result["editorial_complete"]
    assert json.loads((destination / "archive.json").read_text())["editorial_complete"] is True
    archived = retained(destination)
    assert "commentary/conclusions.md" in archived
    assert "analysis/raw-results.json" not in archived
    assert "analysis/scratch.md" not in archived
    assert "analysis/toolkit-issues.md" not in archived
    assert not any(Path(name).suffix in {".asm", ".gb", ".rom", ".png", ".2bpp", ".o"} for name in archived)
    restore = ["python3", SCRIPT, "restore", destination, "--rom", rom]
    restored = base / "restored source"
    result = call(*restore, "--out", restored)
    assert result["exact_source"] and result["byte_identical"]
    assert retained(restored / "source") == retained(source)
    assert (restored / "reading-after.asm").read_bytes() == (work / "commentary/reading-after.asm").read_bytes()
    call(*restore, "--out", restored, ok=False)
    # Whole-directory replacement removes stale artifacts.
    (destination / "stale.txt").write_text("old")
    call(*save)
    assert not (destination / "stale.txt").exists()
    intact = retained(destination)
    # Binary assets may only be recovered from the original ROM/baseline.
    asset = source / "new.png"
    asset.write_bytes(b"unarchivable binary edit")
    call(*save, ok=False)
    assert retained(destination) == intact
    asset.unlink()
    # Failed replacements preserve the existing archive exactly.
    bank.write_text(bank.read_text().replace("jp ", "call ", 1))
    call(*save, ok=False)
    assert retained(destination) == intact
    wrong = base / "wrong.gb"
    wrong.write_bytes(bytes([1]) + bytes(data[1:]))
    rejected = base / "rejected"
    call("python3", SCRIPT, "restore", destination, "--rom", wrong, "--out", rejected, ok=False)
    assert not rejected.exists()
    # Integrity rejects edited metadata before generating any final output.
    editorial = destination / "editorial.json"
    editorial.write_text(editorial.read_text() + " ")
    call(*restore, "--out", rejected, ok=False)
    assert not rejected.exists()

print("archive exact-source/ROM fidelity, evidence exclusions, replacement and failure-preservation regressions passed")
