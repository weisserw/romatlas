"""Authored variant expectations; never infer expectations from fixture output."""
import os
from pathlib import Path
import sys

sys.path.insert(0, str(Path(__file__).resolve().parents[3]/'scripts'))
from native_debug_client import PortClient


def run(run, entry):
    with run.phase('setup'):
        command = [sys.executable, str(Path(__file__).with_name('game.py')),
                   '--stride', os.environ.get('ROMATLAS_QA_FIXTURE_STRIDE', '1')]
        if os.environ.get('ROMATLAS_QA_FIXTURE_BROKEN_PAUSE') == '1':
            command.append('--broken-pause')
        port = run.stack.enter_context(PortClient(command, seconds=10, max_requests=10,
                                                  evidence=run.evidence/'transport'))
        run.clients['native'] = port
        description = port.request('api.describe')
    with run.phase('execution'):
        if entry['id'] == 'movement':
            port.request('port.input', {'actions':['right']})
            port.request('port.advance', {'ticks':1})
            actual = port.request('port.state')['game']
            expected = {'original-fidelity':1, 'double-speed':2}[run.expectation]
            def check():
                assert actual['position'] == expected, {'expected':expected, 'actual':actual}
            run.check('one tick movement under explicitly selected contract', check)
        else:
            port.request('port.input', {'actions':['pause','right']})
            port.request('port.advance', {'ticks':3})
            actual = port.request('port.state')['game']
            def check():
                assert actual['paused'] and actual['position'] == 0, actual
            run.check('pause blocks movement in every variant', check)
    return dict(identity=description['identity'], actual=actual)
