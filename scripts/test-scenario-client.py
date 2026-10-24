"""Exercise persistent bookmarks, failure classification and replay identity forwarding."""
from scenario_client import ScenarioSession, SetupFailure

class Client:
    def __init__(self):
        self.calls = []
    def request(self, method, params=None):
        self.calls.append((method, params))
        if method == 'port.checkpoint.export':
            return {'identity': 'immutable-build', 'segments': [1, 2]}
        if method.endswith('run_until'):
            return {'matched': False, 'reason': 'tick_limit'}
        return {'ok': True}

client = Client()
scenario = ScenarioSession(client, native=True)
scenario.bookmark('approach')
scenario.input(['right'])
scenario.restore('approach')
assert client.calls[-1][1]['checkpoint']['identity'] == 'immutable-build'
assert client.calls[-1][1]['checkpoint']['segments'] == [1, 2]
try:
    scenario.until(conditions=[{'pointer': '/mode', 'value': 'gameplay'}])
except SetupFailure:
    pass
else:
    raise AssertionError('exhausted setup accepted')
reference = ScenarioSession(client, native=False)
reference.bookmark('before-jump')
reference.restore('before-jump')
assert client.calls[-1] == ('emulator.snapshot.load', {'name': 'before-jump'})
print('scenario client: passed')

class RecipeClient(Client):
    identity = 'original-build'
    partial = False
    def request(self, method, params=None):
        if method == 'api.describe':
            self.calls.append((method,params))
            return {'identity':self.identity}
        if method == 'port.advance':
            self.calls.append((method,params))
            return {'completed':0 if self.partial else params['ticks'], 'complete':not self.partial}
        return super().request(method,params)

client = RecipeClient()
session = ScenarioSession(client,native=True)
session.reset()
session.bookmark('origin')
session.input(['right'])
session.advance(2)
session.restore('origin')
session.input(['left'])
recipe = session.export_recipe()
assert recipe['steps'] == [{'kind':'reset'}, {'kind':'input','actions':['left']}]
client.identity = 'different-build'
before = len(client.calls)
try:
    session.replay(recipe)
except SetupFailure:
    pass
else:
    raise AssertionError('wrong-build recipe accepted')
assert all(method != 'port.reset' for method,_ in client.calls[before:])
before = len(client.calls)
try:
    session.replay(dict(recipe,steps=[{'kind':'reset'}, {'kind':'rpc','method':'emulator.memory.write'}]))
except ValueError:
    pass
else:
    raise AssertionError('arbitrary recipe RPC accepted')
assert len(client.calls) == before
client.partial = True
try:
    session.advance(2)
except SetupFailure:
    pass
else:
    raise AssertionError('partial advancement accepted')
try:
    session.export_recipe()
except ValueError:
    pass
else:
    raise AssertionError('incomplete recipe exported')
session.restore('origin')
assert session.export_recipe()['steps'] == [{'kind':'reset'}]
print('authoring recipes: abandoned branches, identity guard and partial-operation safety passed')
