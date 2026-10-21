#!/usr/bin/env python3
"""ROM-free checks for compact transport and bounded noisy-child stderr."""
import json
from pathlib import Path
import sys
import tempfile
from native_debug_client import PortClient, MAX_STDERR, compact_record

payload = dict(pixels=[255]*262144, width=256, height=256)
record = compact_record(payload)
assert record['pixels']['omitted'] and record['pixels']['count'] == 262144
assert record['width'] == 256 and len(json.dumps(record)) < 300
assert compact_record([1, 2, 3]) == [1, 2, 3]
assert compact_record('x'*3000)['count'] == 3000
child = '''
import json, sys
sys.stderr.write('x'*(2*1024*1024)); sys.stderr.flush()
for line in sys.stdin:
    request=json.loads(line)
    print(json.dumps(dict(jsonrpc='2.0', id=request['id'],
                         result=dict(pixels=[255]*262144, width=256))), flush=True)
'''
with tempfile.TemporaryDirectory() as tmp:
    evidence = Path(tmp)/'probe'
    with PortClient([sys.executable, '-c', child], seconds=15,
                    max_requests=2, evidence=evidence) as client:
        result = client.request('port.framebuffer')
        assert len(result['pixels']) == 262144
    client.close()  # Idempotent after context cleanup.
    assert (evidence/'stderr.log').stat().st_size == MAX_STDERR
    assert (evidence/'transport.jsonl').stat().st_size < 1024
    rows = [json.loads(line) for line in (evidence/'transport.jsonl').read_text().splitlines()]
    assert rows[1]['value']['result']['pixels']['omitted']
print('compact transport and bounded stderr checks passed')
