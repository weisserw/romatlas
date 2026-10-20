#!/usr/bin/env python3
"""Run the shared Sega regression suite in Game Gear hardware/profile mode."""
from importlib.util import spec_from_file_location, module_from_spec
from pathlib import Path
spec = spec_from_file_location('sega_tests', Path(__file__).with_name('test-sms.py'))
suite = module_from_spec(spec)
spec.loader.exec_module(suite)
suite.PLATFORM = 'gg'
suite.SCREEN = (160, 144)
def suffix_profile():
    import json, tempfile
    with tempfile.TemporaryDirectory(prefix='romatlas-gg-selection-') as tmp:
        root = Path(tmp)
        rom = root / 'header.sms'
        rom.write_bytes(suite.fixture()[0])
        work = root / 'work'
        suite.script('disassemble.py', rom, '--workspace', work, '--json')
        metadata = json.loads((work / 'workspace.json').read_text())
        assert metadata['platform'] == 'gg' and metadata['rom'] == 'input/game.gg'
        assert (work / 'input/game.gg').is_file()
        assert json.loads((work / 'analysis/state.json').read_text())['format'] == 'romatlas-gg-map'
        assert suite.call('--platform', 'sms', 'inspect', rom)['platform'] == 'sms'

if __name__ == '__main__':
    suffix_profile()
    suite.main()
