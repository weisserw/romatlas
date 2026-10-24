"""Native-only gameplay scenario driver; no emulator or game-state writes.

Disk checkpoints remain identity-checked replay. bookmark/rewind use fast,
session-local snapshots on capable native servers. Games supply
semantic conditions and independent expectations. Captures require human/agent
review; writing an image alone is not presentation acceptance.
"""
import hashlib
import json
from pathlib import Path

from native_debug_client import PortClient
from scenario_client import SetupFailure


class NativeGameplay:
    def __init__(self, run, command, *, seconds=900, max_requests=5000):
        self.run = run
        self.client = run.stack.enter_context(PortClient(command, seconds=seconds,
            max_requests=max_requests, evidence=run.evidence/'native'))
        run.clients['native'] = self.client
        self.commands = []
        self.bookmarks = {}
        run.stack.callback(self.save_recipe)

    def save_recipe(self):
        (self.run.evidence/'native-inputs.json').write_text(
            json.dumps(self.commands, indent=2)+'\n')

    def state(self):
        return self.client.request('port.state')['game']

    def reset(self):
        self.client.request('port.reset')
        self.commands = [dict(kind='reset')]
        self.bookmarks.clear()
        return self.state()

    def restore(self, checkpoint):
        path = Path(checkpoint).resolve(strict=True)
        value = json.loads(path.read_text())
        result = self.client.request('port.checkpoint.import',
            dict(checkpoint=value, timeout_ms=300000))
        if result.get('ok') is not True:
            raise SetupFailure('native checkpoint import failed')
        self.commands.append(dict(kind='origin', checkpoint=str(path),
            sha256=hashlib.sha256(path.read_bytes()).hexdigest()))
        return result['game']

    def bookmark(self, name):
        result = self.client.request('port.snapshot.save', {'name': name})
        if result.get('ok') is not True:
            raise SetupFailure('fast snapshot save failed')
        self.bookmarks[name] = len(self.commands)

    def rewind(self, name):
        if name not in self.bookmarks:
            raise KeyError(name)
        result = self.client.request('port.snapshot.load', {'name': name})
        if result.get('ok') is not True:
            raise SetupFailure('fast snapshot restore failed')
        boundary = self.bookmarks[name]
        self.commands = self.commands[:boundary]
        for key, value in self.bookmarks.items():
            if value > boundary:
                self.client.request('port.snapshot.delete', {'name': key})
        # Bookmarks on an abandoned future branch must not be reused by this recipe.
        self.bookmarks = {k: v for k, v in self.bookmarks.items()
                          if v <= self.bookmarks[name]}
        return self.state()

    def input(self, actions):
        self.client.request('port.input', dict(actions=list(actions)))
        self.commands.append(dict(kind='input', actions=list(actions)))

    def advance(self, ticks):
        result = self.client.request('port.advance', dict(ticks=ticks, timeout_ms=300000))
        self.commands.append(dict(kind='advance', requested=ticks, completed=result.get('completed')))
        if result.get('complete') is not True or result.get('completed') != ticks:
            raise SetupFailure('incomplete native advance')
        return result['game']

    def until(self, conditions, *, ticks=600):
        result = self.client.request('port.run_until',
            dict(conditions=conditions, ticks=ticks, timeout_ms=300000))
        self.commands.append(dict(kind='until', conditions=conditions,
            limit=ticks, completed=result.get('completed'), matched=result.get('matched')))
        if result.get('matched') is not True:
            raise SetupFailure(f'native event not reached: {conditions}; {result.get("reason")}')
        return result['game']

    def capture(self, name):
        if Path(name).name != name or name in ('', '.', '..'):
            raise ValueError('capture name must be a basename')
        self.client.frame(name+'.ppm')
        (self.run.evidence/(name+'-state.json')).write_text(
            json.dumps(self.state(), indent=2)+'\n')
        return 'native/'+name+'.ppm'
