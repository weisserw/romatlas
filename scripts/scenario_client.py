"""Persistent scenario authoring atop PortClient; no game state writes.

Bookmarks use advertised session-local native snapshots or reference snapshots.
Older native servers fall back to identity-checked replay checkpoints. Durable
recipes remain separate from fast in-memory bookmarks.
"""
from copy import deepcopy


class SetupFailure(AssertionError):
    """Setup was inaccessible or unobservable; this is not a game mismatch."""


class ScenarioSession:
    def __init__(self, client, *, native):
        self.client = client
        self.native = native
        self.bookmarks = {}
        self.recipe = []
        self.bookmark_recipes = {}
        self.recipe_valid = True
        self.fast_snapshots = None

    def _mutate(self, method, params=None):
        try:
            return self.client.request(method, params)
        except Exception:
            self.recipe_valid = False
            raise

    def input(self, actions):
        actions = list(actions)
        result = self._mutate('port.input' if self.native else 'emulator.input',
            {'actions': actions} if self.native else {'port': 1, 'buttons': actions})
        self.recipe.append(dict(kind='input', actions=actions))
        return result

    def bookmark(self, name):
        if not self.recipe_valid:
            raise SetupFailure("restore or reset after an indeterminate operation")
        if name not in self.bookmarks and len(self.bookmarks) >= 16:
            raise ValueError('at most 16 authoring bookmarks')
        if self.native:
            if self.fast_snapshots is None:
                self.fast_snapshots = 'port.snapshot.save' in self.client.request('api.describe').get('methods', [])
            if self.fast_snapshots:
                self.client.request('port.snapshot.save', {'name': name})
                self.bookmarks[name] = dict(fast_snapshot=name)
            else:
                self.bookmarks[name] = deepcopy(self.client.request('port.checkpoint.export'))
        else:
            self.client.request('emulator.snapshot.save', {'name': name})
            self.bookmarks[name] = name
        self.bookmark_recipes[name] = deepcopy(self.recipe)

    def restore(self, name):
        value = self.bookmarks[name]
        if self.native:
            if 'fast_snapshot' in value:
                result = self._mutate('port.snapshot.load', {'name': value['fast_snapshot']})
            else:
                result = self._mutate('port.checkpoint.import',
                    {'checkpoint': deepcopy(value), 'timeout_ms': 300000})
            if not result.get('ok'):
                raise SetupFailure('native bookmark import did not succeed')
        else:
            result = self._mutate('emulator.snapshot.load', {'name': value})
        self.recipe = deepcopy(self.bookmark_recipes[name])
        self.recipe_valid = True
        return result

    def until(self, *, conditions, ticks=600, queries=None,
              max_operations=100000, timeout_ms=1000):
        if self.native:
            method = 'port.run_until'
            params = dict(conditions=conditions, ticks=ticks, timeout_ms=timeout_ms)
        else:
            method = 'emulator.run_until'
            params = dict(conditions=conditions, queries=queries,
                          max_operations=max_operations, timeout_ms=timeout_ms)
        result = self._mutate(method, params)
        count = result.get('completed' if self.native else 'operations', 0)
        if count:
            self.recipe.append(dict(kind='advance', count=count, unit='tick' if self.native else 'operation'))
        if not result['matched']:
            raise SetupFailure(f'{method}: {result}')
        return result

    def reset(self):
        result = self._mutate('port.reset' if self.native else 'emulator.reset')
        self.recipe = [dict(kind='reset')]
        self.recipe_valid = True
        self.bookmarks.clear()
        self.bookmark_recipes.clear()
        return result

    def advance(self, count, *, unit=None):
        unit = unit or ('tick' if self.native else 'frame')
        if not isinstance(count, int) or isinstance(count, bool) or not 1 <= count <= 100000:
            raise ValueError('count must be 1..100000')
        if self.native:
            if unit != 'tick':
                raise ValueError('native authoring advances ticks')
            result = self._mutate('port.advance', dict(ticks=count, timeout_ms=300000))
            if result.get('completed') != count or not result.get('complete'):
                self.recipe_valid = False
                raise SetupFailure(f'incomplete native advance: {result}')
        else:
            if unit not in ('frame', 'operation'):
                raise ValueError('reference authoring advances frames or operations')
            result = self._mutate('emulator.step', dict(unit=unit, count=count,
                max_operations=1000000, timeout_ms=30000))
        self.recipe.append(dict(kind='advance', count=count, unit=unit))
        return result

    def export_recipe(self):
        if not self.recipe_valid or not self.recipe or self.recipe[0] != dict(kind='reset'):
            raise ValueError('reset explicitly before authoring a durable recipe')
        description = self.client.request('api.describe')
        if self.native and not description.get('identity'):
            raise SetupFailure('native recipe needs a build/ROM/asset identity')
        return dict(version=1, native=self.native, identity=description.get('identity'),
                    platform=description.get('platform'), steps=deepcopy(self.recipe))

    def replay(self, recipe):
        if recipe.get('version') != 1 or recipe.get('native') is not self.native:
            raise ValueError('recipe transport/version mismatch')
        steps = recipe.get('steps')
        if not isinstance(steps, list) or not steps or steps[0] != dict(kind='reset') or len(steps)>100000:
            raise ValueError('bounded recipe must begin at reset')
        # Validate every operation before changing live state. No arbitrary RPCs.
        for step in steps:
            kind = step.get('kind')
            if kind == 'reset':
                if step != dict(kind='reset'):
                    raise ValueError('invalid reset')
            elif kind == 'input':
                if not isinstance(step.get('actions'), list) or not all(isinstance(a,str) for a in step['actions']):
                    raise ValueError('invalid input')
            elif kind == 'advance':
                count = step.get('count')
                if type(count) is not int or not 1 <= count <= 100000 or step.get('unit') not in (
                        ('tick',) if self.native else ('frame','operation')):
                    raise ValueError('invalid advancement')
            else:
                raise ValueError('unsupported recipe operation')
        description = self.client.request('api.describe')
        if (recipe.get('identity') != description.get('identity')
                or recipe.get('platform') != description.get('platform')):
            raise SetupFailure('recipe transport identity changed; regenerate with source-backed expectations')
        if self.native:
            allowed = set(description.get('actions', []))
            if any(set(step.get('actions', []))-allowed for step in steps if step['kind']=='input'):
                raise ValueError('recipe contains unsupported native actions')
        for step in deepcopy(steps):
            if step['kind'] == 'reset':
                self.reset()
            elif step['kind'] == 'input':
                self.input(step['actions'])
            else:
                self.advance(step['count'], unit=step['unit'])
        return self.client.request('port.state' if self.native else 'emulator.state')
