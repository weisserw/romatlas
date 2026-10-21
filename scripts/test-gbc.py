#!/usr/bin/env python3
"""Synthetic native-color cartridge: runtime, stock reconstruction and port workflow.
No commercial ROMs, extracted assets, display or audio devices.
"""
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]
BIN = ROOT / "target/release/romatlas"


def run(*args, cwd=None, env=None):
    result = subprocess.run(list(map(str, args)), cwd=cwd, env=env, capture_output=True, text=True)
    assert result.returncode == 0, (args, result.stdout, result.stderr)
    return result.stdout


def cli(*args):
    return json.loads(run(BIN, *args))


with tempfile.TemporaryDirectory(prefix="romatlas-cgb-") as temporary:
    work = Path(temporary)
    assembly = work / "fixture.asm"
    assembly.write_text('''SECTION "Entry", ROM0[$100]
    jp Start
SECTION "Header", ROM0[$134]
    db "COLOR FIXTURE"
    ds $143 - @, 0
    db $c0
    ds $147 - @, 0
    db $1b, 1, 3
    ds $150 - @, 0
SECTION "Program", ROM0[$150]
Start:
    di
    xor a
    ldh [$40], a
    ld a, 1
    ldh [$4f], a
    ld hl, $8010
    ld b, 8
.tile:
    ld a, $ff
    ld [hli], a
    xor a
    ld [hli], a
    dec b
    jr nz, .tile
    ld hl, $9800
    ld bc, $400
.attributes:
    ld a, 8
    ld [hli], a
    dec bc
    ld a, b
    or c
    jr nz, .attributes
    xor a
    ldh [$4f], a
    ld hl, $9800
    ld bc, $400
.map:
    ld a, 1
    ld [hli], a
    dec bc
    ld a, b
    or c
    jr nz, .map
    ld a, $80
    ldh [$68], a
    xor a
    ldh [$69], a
    ldh [$69], a
    ld a, $1f
    ldh [$69], a
    xor a
    ldh [$69], a
    ld a, $80
    ldh [$6a], a
    ld a, $e0
    ldh [$6b], a
    ld a, 3
    ldh [$6b], a
    ld a, 2
    ldh [$70], a
    ld a, $55
    ld [$d000], a
    ld a, 1
    ldh [$70], a
    ld a, $66
    ld [$d000], a
    ld a, $77
    ld [$c000], a
    ld a, $c0
    ldh [$51], a
    xor a
    ldh [$52], a
    ld a, 1
    ldh [$53], a
    xor a
    ldh [$54], a
    ldh [$55], a
    ld a, 2
    ld [$2000], a
    call $4000
    ld a, 1
    ldh [$4d], a
    stop
    ld a, $91
    ldh [$40], a
Steady:
    jr Steady
ProgramEnd:
SECTION "Banked code", ROMX[$4000], BANK[2]
Banked:
    ld a, $42
    ld [$c100], a
    ret
BankedEnd:
SECTION "Retain bank three", ROMX[$7fff], BANK[3]
    db 0
''')
    run("rgbasm", "-o", work / "fixture.o", assembly)
    run("rgblink", "-p", "0", "-n", work / "fixture.sym", "-o", work / "fixture.gbc", work / "fixture.o")
    rom = work / "fixture.gbc"
    data = bytearray(rom.read_bytes())
    data[0x14d] = (-sum(data[0x134:0x14d]) - 25) & 255
    rom.write_bytes(data)
    symbols = {line.split()[1]: int(line.split()[0].split(":")[1], 16)
               for line in (work / "fixture.sym").read_text().splitlines() if line and not line.startswith(";")}
    assert cli("inspect", rom)["cgb_flag"] == 0xc0
    # Identity/model selection depends on content, including workspace .rom names.
    content_rom = work / "game.rom"
    shutil.copyfile(rom, content_rom)
    with subprocess.Popen([str(BIN), "serve", "--rom", str(content_rom)], stdin=subprocess.PIPE,
                          stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True) as process:
        serial = 0

        def call(method, params=None, error=False):
            global serial
            serial += 1
            process.stdin.write(json.dumps({"jsonrpc": "2.0", "id": serial, "method": method, "params": params or {}}) + "\n")
            process.stdin.flush()
            response = json.loads(process.stdout.readline())
            assert response["id"] == serial, response
            assert ("error" in response) == error, response
            return response["error" if error else "result"]

        assert call("emulator.start")["model"] == "cgb"
        original = call("emulator.state")
        call("emulator.start", {"model": "dmg"}, error=True)
        assert call("emulator.state") == original
        caps = {s["name"]: s for s in call("emulator.capabilities")["address_spaces"]}
        assert caps["vram"]["size"] == 16384 and caps["wram"]["size"] == 32768
        assert caps["cgb-bg-palette"]["size"] == caps["cgb-obj-palette"]["size"] == 64
        call("trace.configure", {"enabled": True, "capacity": 200})
        state = call("emulator.run", {"operations": 13000, "timeout_ms": 10000})
        assert state["double_speed"] and state["switch_bank"] == 2, state
        coverage = call("analysis.coverage", {"limit": 1000})["items"]
        offsets = {row["address"]["offset"] for row in coverage if row["address"]["space"] == "cartridge-rom"}
        assert 0x8000 in offsets and 0x4000 not in offsets
        banked = next(row for row in coverage if row["address"]["offset"] == 0x8000)
        assert banked["address"]["context"]["bank"] == "2"
        edges = call("analysis.edges", {"limit": 1000})["items"]
        assert any(edge["target"]["space"] == "cartridge-rom" and edge["target"]["offset"] == 0x8000 for edge in edges)

        def physical(space, offset=0, length=1):
            return call("emulator.physical_memory", {"space": space, "offset": offset, "length": length})["bytes"]

        assert physical("wram", 0x100) == [0x42]
        assert physical("wram", 0x1000) == [0x66]
        assert physical("wram", 0x2000) == [0x55]
        assert physical("vram", 0x2010, 2) == [255, 0]
        assert physical("vram", 0x100) == [0x77]  # actual CGB DMA
        assert physical("cgb-bg-palette", 0, 4) == [0, 0, 31, 0]
        assert physical("cgb-obj-palette", 0, 2) == [0xe0, 3]
        call("emulator.physical_memory", {"space": "cgb-bg-palette", "offset": 63, "length": 2}, error=True)
        call("emulator.step", {"unit": "frame", "count": 2, "max_operations": 100000, "timeout_ms": 10000})
        framebuffer = call("emulator.framebuffer")
        assert framebuffer["pixels"] == [201, 0, 46] * (160 * 144)
        # Mid-frame snapshot: restore both displayed and partially drawn buffers.
        call("emulator.step", {"unit": "cycle", "count": 2000, "max_operations": 10000, "timeout_ms": 10000})
        call("emulator.snapshot.save", {"name": "color"})
        call("emulator.snapshot.export", {"path": str(work / "color.snapshot")})
        saved = call("emulator.framebuffer")
        summary = call("trace.summary")
        before = call("emulator.state")
        for _ in range(3):
            physical("cgb-bg-palette", 0, 64)
            physical("cgb-obj-palette", 0, 64)
        assert call("trace.summary") == summary and call("emulator.state") == before
        # Change palette through CPU registers; reading backing must bypass mode 3.
        call("emulator.memory.write", {"address": 0xff40, "bytes": [0]})
        call("emulator.memory.write", {"address": 0xff68, "bytes": [0x82, 0]})
        call("emulator.memory.write", {"address": 0xff69, "bytes": [0x7c]})
        call("emulator.memory.write", {"address": 0xff40, "bytes": [0x91]})
        call("emulator.step", {"unit": "frame", "count": 2, "max_operations": 100000, "timeout_ms": 10000})
        assert call("emulator.framebuffer")["pixels"] != saved["pixels"]
        call("emulator.snapshot.load", {"name": "color"})
        assert call("emulator.framebuffer") == saved
        first = call("emulator.step", {"unit": "frame", "count": 1, "max_operations": 100000, "timeout_ms": 10000})
        first_frame = call("emulator.framebuffer")
        call("emulator.snapshot.import", {"path": str(work / "color.snapshot")})
        assert call("emulator.framebuffer") == saved
        assert call("emulator.step", {"unit": "frame", "count": 1, "max_operations": 100000, "timeout_ms": 10000}) == first
        assert call("emulator.framebuffer") == first_frame
        assert call("emulator.reset")["model"] == "cgb"
        # Enhanced cartridges allow an explicit DMG override; its reset and
        # capability boundary must survive the additive CGB storage APIs.
        dual = bytearray(data)
        dual[0x143] = 0x80
        dual[0x14d] = (-sum(dual[0x134:0x14d]) - 25) & 255
        dual_path = work / "enhanced.gb"
        dual_path.write_bytes(dual)
        call("rom.open", {"path": str(dual_path)})
        assert call("emulator.start")["model"] == "cgb"
        call("emulator.snapshot.export", {"path": str(work / "enhanced.snapshot")})
        assert call("emulator.start", {"model": "dmg"})["model"] == "dmg"
        dmg = call("emulator.state")
        call("emulator.snapshot.import", {"path": str(work / "enhanced.snapshot")}, error=True)
        assert call("emulator.state") == dmg
        dmg_caps = {s["name"]: s for s in call("emulator.capabilities")["address_spaces"]}
        assert dmg_caps["vram"]["size"] == 8192
        assert dmg_caps["cgb-bg-palette"]["size"] == 0
        call("emulator.physical_memory", {"space": "cgb-bg-palette", "offset": 0, "length": 1}, error=True)
        assert call("emulator.reset")["model"] == "dmg"
        process.stdin.close()
        assert process.wait(timeout=10) == 0, process.stderr.read()

    # Native-color header variants use the same stock mapping and instruction set.
    for flag in [0x80, 0xc0]:
        data[0x143] = flag
        data[0x14d] = (-sum(data[0x134:0x14d]) - 25) & 255
        rom.write_bytes(data)
        store = work / f"{flag}.json"
        sym = work / f"{flag}.sym"
        cli("store", "init", rom, "--output", store)
        for offset, length in [(0x100, 3), (0x150, symbols["ProgramEnd"] - 0x150), (0x8000, 6)]:
            cli("store", "annotate", rom, "--store", store, "--offset", offset, "--length", length, "--kind", "code")
        cli("store", "export", rom, "--store", store, "--output", sym)
        with sym.open("a") as output:
            output.write("00:ff68 rColorIndex\n00:d000 wBankedScratch\n")
        generated = work / f"source-{flag}"
        cli("generate", rom, "--map", sym, "--out", generated)
        assert cli("verify", rom, "--generated", generated)["byte_identical"]
        # Clean recipient has only ROM + stock map; supply stock RAM EQU workaround.
        recipient = work / f"recipient-{flag}"
        recipient.mkdir()
        shutil.copyfile(rom, recipient / "fixture.gbc")
        shutil.copyfile(sym, recipient / "fixture.sym")
        run("python3", ROOT / "tools/mgbdis/mgbdis.py", recipient / "fixture.gbc", "--output-dir", recipient / "source")
        run("rgbasm", "-D", "rColorIndex=$ff68", "-D", "wBankedScratch=$d000", "-o", "game.o", "game.asm", cwd=recipient / "source")
        run("rgblink", "-o", "rebuilt.gbc", "game.o", cwd=recipient / "source")
        assert (recipient / "source/rebuilt.gbc").read_bytes() == data

    # Header text is metadata; a non-UTF8 byte must not block execution.
    data[0x134] = 255
    data[0x14d] = (-sum(data[0x134:0x14d]) - 25) & 255
    rom.write_bytes(data)
    assert cli("run", rom, "--operations", 1)["model"] == "cgb"
    analysis = work / "analysis"
    run("python3", ROOT / "scripts/disassemble.py", rom, "--workspace", analysis, "--json")
    assert "docs/gbc.md" in (analysis / "workflow.md").read_text()
    cli("store", "init", analysis / "input/game.rom", "--output", analysis / "analysis/state.json")
    cli("store", "annotate", analysis / "input/game.rom", "--store", analysis / "analysis/state.json", "--offset", 256, "--length", 3, "--kind", "code")
    cli("store", "export", analysis / "input/game.rom", "--store", analysis / "analysis/state.json", "--output", analysis / "maps/current.sym")
    run("python3", ROOT / "scripts/commentary.py", "prepare", analysis, "--json")
    archive = work / "archive"
    run("python3", ROOT / "scripts/archive.py", "archive", analysis, "--name", "color-fixture", "--output", archive)
    port = work / "native"
    run("python3", ROOT / "scripts/port.py", "prepare", archive, "--rom", rom, "--out", port)
    assert "docs/gbc.md" in (port / "prompts/start.md").read_text()
    assert json.loads((archive / "archive.json").read_text())["platform"] == "gb"
    env = dict(os.environ, CARGO_TARGET_DIR=str(ROOT / "target/cgb-scaffold"))
    run("cargo", "check", "--locked", "--manifest-path", port / "port/Cargo.toml", env=env)
    print("PASS: CGB banking/color/DMA/speed/replay, stock .sym/RGBDS rebuilds and archive/native preparation")
