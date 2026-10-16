#!/usr/bin/env python3
"""Own synthetic bytes only. Exercise stock tools in a clean recipient directory."""
import hashlib, pathlib, shutil, subprocess, tempfile
root = pathlib.Path(__file__).resolve().parents[1]
cli = root / 'target/release/romatlas'
def run(*args, cwd=None):
    subprocess.run([str(a) for a in args], cwd=cwd, check=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
with tempfile.TemporaryDirectory() as work:
    work = pathlib.Path(work)
    # Includes every byte value, invalid opcodes, nonzero STOP operands, and invalid checksums.
    rom = work / 'original.gb'
    data = bytearray((i * 73 + 19) % 256 for i in range(0x10000))
    data[0x147:0x14a] = bytes([1, 1, 0])
    data[0x100:0x103] = bytes([0xc3, 0x50, 0x01])
    data[0x150:0x152] = bytes([0x18, 0xfe])
    rom.write_bytes(data)
    sym = work / 'original.sym'
    store = work / 'analysis.json'
    run(cli, 'store', 'init', rom, '--output', store)
    run(cli, 'store', 'annotate', rom, '--store', store, '--offset', 256, '--length', 3, '--kind', 'code')
    run(cli, 'store', 'annotate', rom, '--store', store, '--offset', 336, '--length', 2, '--kind', 'code')
    before = store.read_bytes()
    rejected = subprocess.run([str(cli), 'store', 'annotate', str(rom), '--store', str(store), '--offset', '257', '--length', '2', '--kind', 'data'], capture_output=True)
    assert rejected.returncode == 1 and store.read_bytes() == before
    run(cli, 'store', 'export', rom, '--store', store, '--output', sym)
    assert '.code:3' in sym.read_text() and 'code_00_0100' in sym.read_text()
    assert 'sha256' not in sym.read_text()
    store.unlink()
    run(cli, 'generate', rom, '--map', sym, '--out', work / 'first')
    run(cli, 'verify', rom, '--generated', work / 'first')
    first = {p.name: p.read_bytes() for p in (work / 'first').glob('*.asm')}
    shutil.rmtree(work / 'first')
    recipient = work / 'recipient'
    recipient.mkdir()
    shutil.copy(rom, recipient / 'original.gb')
    shutil.copy(sym, recipient / 'original.sym')
    run('python3', root / 'tools/mgbdis/mgbdis.py', recipient / 'original.gb', '--output-dir', recipient / 'source')
    run('make', 'game.o', cwd=recipient / 'source')
    run('rgblink', '-o', 'rebuilt.gb', 'game.o', cwd=recipient / 'source')
    assert (recipient / 'source/rebuilt.gb').read_bytes() == data
    assert first == {p.name: p.read_bytes() for p in (recipient / 'source').glob('*.asm')}
    print('PASS: clean stock-tool portability, deterministic source, exact 4-bank bytes', hashlib.sha256(data).hexdigest())
