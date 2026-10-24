"""Local adapter instrumentation contracts; no game subprocess required."""
import sys
from pathlib import Path
sys.path.insert(0, str(Path(__file__).resolve().parents[4]/'scripts'))
from pathlib import Path
from tempfile import TemporaryDirectory
from qa_harness import Run
from scenario_client import SetupFailure
import integration_adapter as adapter

with TemporaryDirectory() as directory:
    run = Run(Path(directory), 'original-fidelity')
    module = adapter.instrument_j(run)
    class Pair:
        reference_description = {'methods': ['emulator.run_until']}
        def __init__(self): self.reference = self
        def request(self, *args, **kwargs): return {'matched': False, 'reason': 'tick_limit'}
    try:
        module.finish_source_frame(Pair(), Path(directory),
            {'mode': 'gameplay', 'timers': {'composition': 1}}, 'bounded')
    except SetupFailure:
        pass
    else:
        raise AssertionError('unmatched source boundary became a game mismatch')
    # The original first assertion still rejects an illegal wait and retains its line.
    try:
        module.finish_vertical_owner(None, {'calls': {'vertical': None, 'wait': None}}, 'bad', 0)
    except AssertionError:
        pass
    else:
        raise AssertionError('instrumentation weakened original assertion')
print('DuckTales adapter: boundary classification and original assertions passed')

with TemporaryDirectory() as directory:
    run = Run(Path(directory), 'original-fidelity')
    module = adapter.instrument_j(run)
    class ConditionalPair:
        reference_description = {'methods': ['emulator.run_until'], 'conditional_operators': ['eq','ne']}
        def __init__(self):
            self.evidence = Path(directory)
            self.reference = self
            self.steps = []
        def request(self, method, params):
            assert method == 'emulator.run_until'
            assert params['max_operations'] == 32768
            assert {'query':'irq', 'pointer':'/bytes/0', 'value':8, 'op':'ne'} in params['conditions']
            self.params = params
            return dict(matched=True, operations=11, reason='matched', observations={
                'composition':{'bytes':[7]}, 'task':{'bytes':[1]},
                'irq':{'bytes':[9]}, 'caller':{'bytes':[0x08,0xCA]}})
    pair = ConditionalPair()
    game = {'calls':{'vertical':{'counter':1}, 'wait':{'owner':'VerticalYield','remaining':1}},
            'timers':{'composition':7}}
    phase = module.finish_vertical_owner(pair, game, 'conditional-test', 8)
    assert phase == dict(composition=7, task=1, irq=9, caller=0xCA08)
    assert len(pair.steps) == 1 and pair.steps[0]['source_operations'] == 11
print('DuckTales adapter: fresh IRQ predicate and conditional vertical owner passed')
