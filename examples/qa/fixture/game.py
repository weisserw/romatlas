"""Owned deterministic QA fixture: selectable movement speed, shared pause rules."""
import argparse
import json
import sys

parser = argparse.ArgumentParser()
parser.add_argument('--stride', type=int, choices=(1, 2), default=1)
parser.add_argument('--broken-pause', action='store_true')
args = parser.parse_args()
position = ticks = 0
held = previous = set()
paused = False
for line in sys.stdin:
    request = json.loads(line)
    method = request['method']
    params = request.get('params', {})
    if method == 'api.describe':
        result = {'identity': f'owned-fixture-stride-{args.stride}-pause-{not args.broken_pause}'}
    elif method == 'port.input':
        held = set(params['actions'])
        result = {'ok': True}
    elif method == 'port.advance':
        count = params['ticks']
        assert type(count) is int and 0 <= count <= 1000
        for _ in range(count):
            if 'pause' in held-previous:
                paused = not paused
            if 'right' in held and (not paused or args.broken_pause):
                position += args.stride
            previous = held.copy()
            ticks += 1
        result = {'completed': count, 'complete': True}
    elif method == 'port.state':
        result = {'game': {'position': position, 'paused': paused, 'ticks': ticks}}
    else:
        raise ValueError(method)
    print(json.dumps({'jsonrpc': '2.0', 'id': request['id'], 'result': result}), flush=True)
