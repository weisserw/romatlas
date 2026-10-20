#!/usr/bin/env python3
"""Keep shared workspace crates free of platform implementations."""
import json,subprocess,pathlib
root=pathlib.Path(__file__).resolve().parents[1]
metadata=json.loads(subprocess.check_output(['cargo','metadata','--no-deps','--format-version','1','--locked'],cwd=root,text=True))
neutral={'romatlas-core','romatlas-analysis','romatlas-map','romatlas-agent','romatlas-disasm','romatlas-port'}
for package in metadata['packages']:
 if package['name'] in neutral:
  forbidden=[d['name'] for d in package['dependencies'] if d['name'] in {'romatlas-sms','smsgg-core','smsgg-config','z80-emu','dez80','romatlas-gb','romatlas-nes','nes-core','nes-config','mos6502-emu','ym-opll','romatlas-snes','snes-core','snes-coprocessors','wdc65816-emu','spc700-emu','arm7tdmi-emu','jgenesis-common'} or d['name'].startswith('mizu')]
  assert not forbidden,(package['name'],forbidden)
print('PASS: shared crates have no console core/adapter dependency')
