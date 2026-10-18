#!/usr/bin/env python3
"""Fetch pinned MIT CPU tests and notices; no ROM downloads. Keep data local."""
from pathlib import Path
import json
import urllib.request

ROOT = Path(__file__).resolve().parents[1] / 'tools/snes-eval/cpu-tests'
SOURCES = {
    '65816': ('307c63f5673c8c6b861c52f71ddb89212a227480', 'LICENCE',
              ['a9.e', 'a9.n', 'c2.n', 'e2.n', 'fb.n', '80.n', '8f.n', '69.n']),
    'spc700': ('67d15f492b2740964abd4efd1229e0ec9c342228', 'LICENSE',
               ['e8', '8f', 'c4', '2f', '00']),
}
for cpu, (rev, license_name, cases) in SOURCES.items():
    dest = ROOT / cpu
    dest.mkdir(parents=True, exist_ok=True)
    base = f'https://raw.githubusercontent.com/SingleStepTests/{cpu}/{rev}'
    for remote, local in [(license_name, license_name)] + [(f'v1/{case}.json', f'{case}.json') for case in cases]:
        path = dest / local
        if not path.exists():
            with urllib.request.urlopen(f'{base}/{remote}', timeout=60) as response:
                data = response.read(32*1024*1024+1)
            if len(data)>32*1024*1024:
                raise SystemExit(f'oversized download: {remote}')
            path.write_bytes(data)
    (dest/'source.json').write_text(json.dumps(dict(repository=f'https://github.com/SingleStepTests/{cpu}', revision=rev, files=cases),indent=2)+'\n')
    print(cpu, rev, len(cases), 'files')
