#!/usr/bin/env python3
"""Synthetic ROM: directive content, stock rebuild, and atomic rejection."""
import json
import os
import pathlib
import re
import shutil
import subprocess
import tempfile

root = pathlib.Path(__file__).resolve().parents[1]
cli = root / 'target/release/romatlas'

def call(*args, success=True, env=None):
    result = subprocess.run([str(cli), *map(str, args)], capture_output=True, text=True, timeout=120, env=env)
    assert (result.returncode == 0) == success, (args, result.stdout, result.stderr)
    return result

with tempfile.TemporaryDirectory() as directory:
    work = pathlib.Path(directory)
    rom = work / 'owned.gb'
    data = bytearray(0x8000)
    data[0x200:0x300] = bytes(range(256))
    data[0x300:0x320] = bytes((i * 73 + 19) % 256 for i in range(32))
    data[0x400:0x403] = bytes([0xc3, 0x00, 0x04])
    rom.write_bytes(data)

    # Stock's interior vector/header defaults must not classify exported gaps.
    # Test both default sets, explicit Unknown, and an empty classification store.
    for cgb in [False, True]:
        raw_rom = work / f'raw-{cgb}.rom'
        raw_data = bytearray([0xff] * 0x8000)
        raw_data[0x134:0x143] = b'RAW HEADER TEST'
        raw_data[0x143] = 0x80 if cgb else 0
        raw_data[0x147:0x14a] = bytes(3)
        raw_data[0x400:0x403] = bytes([0xc3, 0, 4])
        raw_rom.write_bytes(raw_data)
        store = work / f'raw-{cgb}.json'
        raw_sym = work / f'raw-{cgb}.sym'
        call('store', 'init', raw_rom, '--output', store)
        for classified in [False, True]:
            if classified:
                call('store', 'annotate', raw_rom, '--store', store,
                     '--offset', 0, '--length', 0x40, '--kind', 'unknown', '--confidence', 0)
                call('store', 'annotate', raw_rom, '--store', store,
                     '--offset', 0x400, '--length', 3, '--kind', 'code')
            before = store.read_bytes()
            call('store', 'export', raw_rom, '--store', store, '--output', raw_sym)
            assert store.read_bytes() == before
            raw_out = work / f'raw-{cgb}-{classified}'
            call('generate', raw_rom, '--map', raw_sym, '--out', raw_out)
            listing = (raw_out / 'bank_000.asm').read_text()
            statements = re.findall(r'^[ \t]+(\w+)\b', listing, re.MULTILINE)
            unexpected = set(statements) - {'db', 'jp'}
            assert not unexpected, unexpected
            assert statements.count('jp') == int(classified), statements.count('jp')
            assert 'db "' not in listing  # Header title must stay raw too.
            call('verify', raw_rom, '--generated', raw_out)
            assert (raw_out / 'romatlas-rebuilt.gb').read_bytes() == raw_data

    # All byte values in text, 1bpp and 2bpp assets, labels, and padding.
    sym = work / 'owned.sym'
    sym.write_text('00:0000 .data:200\n00:0200 .text:100\n'
                   '00:0300 Tiles\n00:0300 .image:20:w16,pe4\n'
                   '00:0320 .image:8:w8,pe4,1bpp\n'
                   '00:0328 .data:d8\n00:0400 Entry\n00:0400 .code:3\n'
                   '00:0403 .padding:3bfd\n01:4000 .data:4000\n')
    out = work / 'valid'
    call('map', 'validate', sym, '--rom', rom)
    call('generate', rom, '--map', sym, '--out', out)
    assert not (out / 'game.o').exists() and not (out / 'romatlas-rebuilt.gb').exists()
    call('verify', rom, '--generated', out)
    assert (out / 'romatlas-rebuilt.gb').read_bytes() == data

    for index, text in enumerate([
        '00:0200 .padding:100\n',  # Unequal bytes.
        '00:0200 .image:f\n',     # Incomplete tile.
        '00:0200 .code:1\n',      # NOP is valid; bad opcode below.
        '00:0300 .image:20:w16,p00\n',  # Palette destroys color identity.
        '00:0200 .text:100:cm=Missing\n',  # No charmap resource.
    ]):
        bad_rom = work / f'bad-{index}.gb'
        bad_data = bytearray(data)
        if index == 2:
            bad_data[0x200] = 0xd3
        bad_rom.write_bytes(bad_data)
        sym.write_text(text)
        bad_out = work / f'rejected-{index}'
        result = call('generate', bad_rom, '--map', sym, '--out', bad_out, success=False)
        assert json.loads(result.stderr)['ok'] is False
        assert not bad_out.exists()
        assert bad_rom.read_bytes() == bad_data
        assert not list(work.glob('.romatlas-input-*'))

    # Embedded palettes preserve 2bpp index order, including reversed colors.
    sym.write_text('00:0300 .image:20:w16,p1b\n')
    call('generate', rom, '--map', sym, '--out', work / 'palette')
    call('verify', rom, '--generated', work / 'palette')
    # Both bit depths preserve reversed palettes with the pinned tools.
    sym.write_text('00:0300 .image:20:w16,p1b,1bpp\n')
    call('generate', rom, '--map', sym, '--out', work / 'inverted')

    # Inject a corrupt result after the real linker succeeds. This proves
    # generation checks bytes, rather than trusting the process exit code.
    tools = work / 'tools'
    tools.mkdir()
    linker = tools / 'rgblink'
    linker.write_text('#!/usr/bin/python3\nimport pathlib, subprocess, sys\n'
                      f'result = subprocess.run([{shutil.which("rgblink")!r}, *sys.argv[1:]])\n'
                      'if result.returncode == 0 and "-o" in sys.argv:\n'
                      '    path = pathlib.Path(sys.argv[sys.argv.index("-o") + 1])\n'
                      '    data = bytearray(path.read_bytes()); data[0x300] ^= 1; path.write_bytes(data)\n'
                      'sys.exit(result.returncode)\n')
    linker.chmod(0o755)
    env = dict(os.environ, PATH=str(tools) + os.pathsep + os.environ['PATH'])
    result = call('generate', rom, '--map', sym, '--out', work / 'corrupt', success=False, env=env)
    assert 'byte-fidelity' in result.stderr and 'first mismatch' in result.stderr
    assert not (work / 'corrupt').exists()
    assert rom.read_bytes() == data
    assert not list(work.glob('.romatlas-input-*'))

print('PASS: content checks, image/text byte fidelity, and atomic rejection')
