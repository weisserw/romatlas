"""New title-menu branch discovery and recipe replay; no gameplay modification.

Expected markers come from ChangeTitleOption/TitleOptionMarkerX in frozen source:
initial option1 X80, right option2 X144, left option0 X32. The right branch is
abandoned; the exported left branch must reproduce exactly from reset.
"""
import argparse
import os
import json
from pathlib import Path
import sys

ROOT = Path(os.environ['ROMATLAS_GAME_WORKSPACE']).resolve()
sys.path.insert(0, str(Path(__file__).resolve().parents[4]/'scripts'))
from native_debug_client import PortClient
from scenario_client import ScenarioSession

parser = argparse.ArgumentParser()
parser.add_argument('--binary', type=Path, required=True)
parser.add_argument('--evidence', type=Path, required=True)
args = parser.parse_args()
with PortClient([str(args.binary),'--rom',str(ROOT/'input/game.rom'),'--debug-stdio'],
                seconds=60,max_requests=60,evidence=args.evidence) as port:
    session = ScenarioSession(port, native=True)
    session.reset()
    session.until(conditions=[dict(pointer='/transition/menu',value='Title(Input)')],ticks=180)
    session.bookmark('choice')
    def choose(direction):
        session.input([direction]);session.advance(1)
        session.input([]);session.advance(1)
        return port.request('port.state')['game']
    right = choose('right')
    assert right['actors'][1]['position']['world_x'] == 144
    session.restore('choice')
    left = choose('left')
    assert left['actors'][1]['position']['world_x'] == 32
    recipe = session.export_recipe()
    assert not any(step.get('actions') == ['right'] for step in recipe['steps'])
    replayed = session.replay(recipe)['game']
    assert replayed == left, 'exported branch did not replay to identical semantic state'
    (args.evidence/'recipe.json').write_text(json.dumps(recipe,indent=2)+'\n')
    (args.evidence/'result.json').write_text(json.dumps(dict(status='passed',right_marker=144,
        left_marker=32,abandoned_branch_excluded=True,replay_identical=True,requests=port.serial,
        source=['ChangeTitleOption','TitleOptionMarkerX']),indent=2)+'\n')
    print('DuckTales new branch authoring, rewind and durable replay passed')
