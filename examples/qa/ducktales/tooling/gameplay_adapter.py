"""Player-facing native tests. Imports no paired emulator/source synchronization."""
import json
import os
import hashlib
from pathlib import Path
import sys

ROOT = Path(os.environ['ROMATLAS_GAME_WORKSPACE']).resolve()
sys.path.insert(0, str(Path(json.loads((ROOT/'workspace.json').read_text()).get('toolkit','/root/romatlas'))/'scripts'))
from native_gameplay import NativeGameplay
from scenario_client import SetupFailure


def run(run, entry):
    if entry['id'] != 'L-choice-reject' or run.expectation != 'faithful-gameplay':
        raise SetupFailure('native gameplay scenario/profile not implemented')
    checkpoint = ROOT/'debug/setups/choice-front/checkpoint.json'
    provenance = dict(adapter_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
        native_binary_sha256=hashlib.sha256((ROOT/'port/target/debug/ducktales-port').read_bytes()).hexdigest(),
        rom_sha256=hashlib.sha256((ROOT/'input/game.rom').read_bytes()).hexdigest(),
        checkpoint_sha256=hashlib.sha256(checkpoint.read_bytes()).hexdigest(),
        expectation=entry['expectations'][run.expectation])
    (run.evidence/'provenance.json').write_text(json.dumps(provenance,indent=2)+'\n')
    (run.evidence/'scenario.py').write_bytes(Path(__file__).read_bytes())
    with run.phase('setup'):
        game = NativeGameplay(run, [str(ROOT/'port/target/debug/ducktales-port'),
            '--rom', str(ROOT/'input/game.rom'), '--debug-stdio'])
        state = game.restore(checkpoint)
        def start_conditions():
            assert state['mode'] == 'gameplay'
            assert state['progression']['stage'] == 0 and state['progression']['room_row'] == 2
            assert state['actors'][0]['position']['world_x'] == 433
            assert state['status']['reward'] == [36,2,0,0,0,0]
        run.check('validated natural choice-NPC approach', start_conditions)
    before = state
    captures = []
    with run.phase('execution'):
        game.input(['right'])
        state = game.until([dict(pointer='/mode', value='dialogue')], ticks=60)
        game.input([])
        run.check('NPC contact opens the expected choice text',
            lambda: require(state['dialogue']['message'] == 15))
        state = game.until([dict(pointer='/dialogue/phase', value='AwaitInput')], ticks=600)
        captures.append(game.capture('choice-text'))
        # A page can require acknowledgment before the final choice. B is a
        # source-documented rejection; observe message completion rather than
        # predicting an exact glyph/frame count or polling source scheduler RAM.
        for page in range(8):
            game.input(['b']);game.advance(1);game.input([])
            state = game.advance(1)
            for _ in range(75):
                if state['mode'] != 'dialogue' or state['dialogue']['phase'] == 'AwaitInput':
                    break
                state = game.advance(8)
            else:
                raise SetupFailure('text page/restoration did not finish within bound')
            if state['mode'] != 'dialogue':
                break
            captures.append(game.capture(f'choice-page-{page}'))
        else:
            raise SetupFailure('choice did not finish within eight pages')
        # Once final acknowledgment starts panel restoration, finish it using
        # semantic mode/phase rather than treating its temporary waits as pages.
        state = game.until([dict(pointer='/mode', value='gameplay')], ticks=240)
        game.input([]);game.advance(2);state=game.state()
        def rejected():
            assert state['progression']['accepted_stages'] == before['progression']['accepted_stages']
            assert state['flags']['special_room'] == before['flags']['special_room']
            assert state['status']['reward'] == before['status']['reward']
            assert state['presentation']['suspend_depth'] == 0
            assert state['player']['selector'] != 15
        run.check('reject preserves money and progression and restores gameplay/HUD', rejected)
        captures.append(game.capture('choice-rejected-gameplay'))
    return dict(scope='L rejection subcase only; not full L acceptance',
        captures=captures, presentation_review_required=True,
        criteria=['Readable choice text in an intact panel',
                  'Panel closes and normal HUD/playfield return after rejection'],
        ticks_before=before['ticks'],ticks_after=state['ticks'])


def require(value):
    assert value
