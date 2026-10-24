"""Persistent native-only authoring: line-delimited JSON commands, compact replies.

Named snapshots are RAM-only. Export disk checkpoints for future sessions. This
is discovery tooling and never writes QA acceptance. No arbitrary RPC/state edits.
"""
import argparse
import json
from pathlib import Path
import sys
import time

from native_gameplay import NativeGameplay
from qa_harness import Run


def pointer(value, path):
    if path == '':
        raise ValueError('select specific observation fields, not the entire game')
    if not path.startswith('/'):
        raise ValueError('observations use JSON pointers')
    for part in path[1:].split('/'):
        part = part.replace('~1', '/').replace('~0', '~')
        value = value[int(part)] if isinstance(value, list) else value[part]
    return value


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--binary', type=Path, required=True)
    parser.add_argument('--rom', type=Path, required=True)
    parser.add_argument('--setup', type=Path)
    parser.add_argument('--evidence', type=Path, required=True)
    parser.add_argument('--observe', action='append', default=[])
    parser.add_argument('--seconds', type=int, default=3600)
    args = parser.parse_args()
    args.evidence.mkdir(parents=True, exist_ok=False)
    run = Run(args.evidence, 'discovery-only')
    start = time.monotonic()
    with run.stack:
        game = NativeGameplay(run, [str(args.binary.resolve()), '--rom', str(args.rom.resolve()),
            '--debug-stdio'], seconds=args.seconds, max_requests=100000)
        state = game.restore(args.setup) if args.setup else game.reset()
        game.bookmark('origin')
        fields = args.observe or ['/mode']
        def summary():
            return {field: pointer(state, field) for field in fields}
        print(json.dumps(dict(ready=True, setup_seconds=time.monotonic()-start, state=summary())), flush=True)
        valid = True
        for line in sys.stdin:
            try:
                if len(line) > 16384:
                    raise ValueError('authoring command exceeds 16 KiB')
                command = json.loads(line)
                operation = command['op']
                if operation == 'close':
                    break
                if not valid and operation not in ['state', 'restore']:
                    raise ValueError('restore a valid bookmark before continuing after a failed operation')
                started = time.monotonic()
                if operation == 'state':
                    state = game.state()
                elif operation == 'input':
                    game.input(command['actions']); state = game.state()
                elif operation == 'advance':
                    state = game.advance(command['ticks'])
                elif operation == 'until':
                    state = game.until(command['conditions'], ticks=command.get('ticks', 600))
                elif operation == 'book':
                    game.bookmark(command['name'])
                elif operation == 'restore':
                    state = game.rewind(command['name']); valid = True
                elif operation in ['export', 'capture']:
                    name = command['name']
                    if Path(name).name != name or name in ['', '.', '..']:
                        raise ValueError('artifact name must be a basename')
                    if operation == 'capture':
                        game.capture(name)
                    else:
                        path = args.evidence/(name+'-checkpoint.json')
                        checkpoint = game.client.request('port.checkpoint.export')
                        with path.open('x') as output:
                            json.dump(checkpoint, output)
                else:
                    raise ValueError('unknown authoring operation')
                print(json.dumps(dict(ok=True, seconds=time.monotonic()-started, state=summary())), flush=True)
            except Exception as error:
                valid = False
                print(json.dumps(dict(error=str(error), restore_required=True)), flush=True)
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
