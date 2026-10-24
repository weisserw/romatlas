"""DuckTales showcase adapter. No accepted QA records or game state writes."""
import ast
import json
from pathlib import Path
import sys
import types

sys.path.insert(0, str(Path(__file__).resolve().parent))
SCENARIOS = Path(__file__).resolve().parents[1]/'scenarios'
sys.path.insert(0, str(SCENARIOS))
import common
import g_routes
import setup_cache
from scenario_client import SetupFailure


def instrument_j(run):
    """Keep every original assert and its lazy diagnostic; count successful executions."""
    path = SCENARIOS/'j_climb.py'
    source = path.read_text()
    records = {}
    def passed(line, text):
        key = (run.stage, line)
        if key not in records:
            records[key] = dict(name=f'j_climb.py:{line}', expression=text,
                                phase=run.stage, status='passed', executions=0)
            run.assertions.append(records[key])
        records[key]['executions'] += 1
    class Assertions(ast.NodeTransformer):
        def visit_Assert(self, node):
            record = ast.Expr(ast.Call(func=ast.Name(id='_assert_passed', ctx=ast.Load()),
                args=[ast.Constant(node.lineno), ast.Constant(ast.get_source_segment(source, node.test))], keywords=[]))
            return [node, ast.copy_location(record, node)]
    module = types.ModuleType('j_climb_integration')
    module.__file__ = str(path)
    module._assert_passed = passed
    tree = ast.fix_missing_locations(Assertions().visit(ast.parse(source, filename=str(path))))
    exec(compile(tree, str(path), 'exec'), module.__dict__)
    # Bounded owner searches establish observation boundaries, not game oracles.
    def boundary(operation):
        def call(*args, **kwargs):
            try:
                return operation(*args, **kwargs)
            except AssertionError as error:
                detail = error.args[0] if error.args else ''
                reason = detail.get('reason', '') if isinstance(detail, dict) else str(detail)
                if 'inaccessible' in reason:
                    raise SetupFailure(str(error)) from error
                raise
        return call
    module.finish_source_frame = boundary(module.finish_source_frame)
    module.finish_vertical_owner = boundary(module.finish_vertical_owner)
    return module


def setup_identity(provenance, expectation):
    keys = ['rom', 'executable', 'reference_executable', 'source', 'frozen_source', 'overrides']
    recipes = [SCENARIOS/'j_climb.py', SCENARIOS/'common.py', SCENARIOS/'g_routes.py',
               SCENARIOS/'j-source-contract.md', SCENARIOS/'routes/climb-vertical.json',
               Path(__file__), Path(setup_cache.__file__), common.TOOLKIT/'scripts/native_debug_client.py']
    return dict(version=1, expectation=expectation, build={key: provenance[key] for key in keys},
                recipes={str(path): common.sha(path) for path in recipes})


def restore_rope(pair, j, directory, provenance):
    metadata = json.loads((directory/'climb-entry-preconditions.json').read_text())
    for key in ['rom', 'executable', 'reference_executable', 'source', 'frozen_source', 'overrides']:
        if metadata['provenance'][key] != provenance[key]:
            raise SetupFailure(f'cached rope provenance mismatch: {key}')
    checkpoint = json.loads((directory/'climb-entry-checkpoint.json').read_text())
    result = pair.native.request('port.checkpoint.import', dict(checkpoint=checkpoint, timeout_ms=300000))
    if not result.get('ok'):
        raise SetupFailure('cached native import failed')
    pair.reference.request('emulator.snapshot.import', dict(path=str((directory/'climb-entry-reference-snapshot.json').resolve())))
    game = pair.game()
    if game != metadata['native'] or pair.reference_yield_return() != metadata['source_yield']:
        raise SetupFailure('cached rope canonical state or source owner differs')
    if (game['mode'] != 'gameplay' or game['player']['selector'] != 3
            or game['actors'][0]['position']['world_x'] != 896 or game['world']['camera']['x'] != 768):
        raise SetupFailure('cached rope is not the validated natural climbing entry')
    j.world_facts(pair, game, 'cached-rope-validation')
    common.save(pair.evidence/'rope-cache-restore-validation.json', dict(directory=str(directory),
        native=game, source_yield=metadata['source_yield'], hashes={name: common.sha(directory/name) for name in setup_cache.FILES}))
    return game


def run(run, entry):
    if run.expectation != 'original-fidelity':
        raise SetupFailure('No authored adapter for this expectation')
    pair = common.Pair.__new__(common.Pair)
    # Register cleanup before construction so partial transport startup also closes.
    run.stack.callback(lambda: hasattr(pair, 'stack') and pair.stack.close())
    with run.phase('setup'):
        try:
            common.Pair.__init__(pair, run.evidence, reference_request_limit=100000,
                                 compressed_reference=True)
        finally:
            for name in ('native', 'reference'):
                if hasattr(pair, name):
                    run.clients[name] = getattr(pair, name)
        run.stack.callback(lambda: common.save(run.evidence/'inputs.json', pair.steps))
        def retain_failure(exc_type, error, tb):
            if exc_type is not None:
                common.save(run.evidence/'observations-partial.json', getattr(pair, 'observations', []))
                try:
                    pair.capture('failure')
                except Exception as capture_error:
                    common.save(run.evidence/'failure-capture-error.json', dict(error=str(capture_error)))
            return False
        run.stack.push(retain_failure)
        j = instrument_j(run)
        provenance = common.identity()
        common.save(run.evidence/'provenance.json', dict(provenance=provenance,
            adapter_sha256=common.sha(__file__), harness_sha256=common.sha(common.TOOLKIT/'scripts/qa_harness.py'),
            source_contract_sha256=common.sha(SCENARIOS/'j-source-contract.md'),
            route_sha256=common.sha(SCENARIOS/'routes/climb-vertical.json')))
        for path in [SCENARIOS/'j_climb.py', SCENARIOS/'g_routes.py', SCENARIOS/'common.py',
                     SCENARIOS/'j-source-contract.md', SCENARIOS/'routes/climb-vertical.json', Path(setup_cache.__file__), Path(__file__), common.TOOLKIT/'scripts/qa_harness.py']:
            (run.evidence/path.name).write_bytes(path.read_bytes())
        cache_identity = setup_identity(provenance, run.expectation)
        cache_root = common.ROOT/'debug/cache/integration/J-climb-vertical'
        selected, cache_reason = setup_cache.select(cache_root, cache_identity)
        points = []
        if selected is not None:
            game = run.check('identity-checked cached natural rope restore',
                             lambda: restore_rope(pair, j, selected, provenance))
        else:
            keys = ['rom','executable','reference_executable','source','frozen_source','overrides']
            try:
                old = json.loads((j.SETUP/'gap-checkpoint-preconditions.json').read_text())
                stale = [key for key in keys if provenance[key] != old['provenance'][key]]
            except FileNotFoundError:
                stale = ['missing_checkpoint_metadata']
            for name in ('gap-checkpoint.json', 'gap-reference-snapshot.json'):
                if not (j.SETUP/name).is_file():
                    stale.append(f'missing:{name}')
            if stale:
                # Regenerate via the retained G natural input recipe, never edit identities.
                points = []
                game = run.check('regenerate natural gap recipe', lambda: g_routes.natural_gap(pair, points))
                common.save(run.evidence/'gap-checkpoint.json', pair.native.request('port.checkpoint.export'))
                pair.reference.request('emulator.snapshot.export', dict(path=str((run.evidence/'gap-reference-snapshot.json').resolve())))
                common.save(run.evidence/'gap-checkpoint-preconditions.json', dict(native=game,
                    source_yield=pair.reference_yield_return(), provenance=common.identity()))
                j.SETUP = run.evidence
            common.save(run.evidence/'checkpoint-selection.json', dict(stale_identity_fields=stale,
                selected=str(j.SETUP), regenerated=bool(stale)))
            points = []
            game = run.check('identity-checked gap restore and natural rope entry', lambda: j.natural_rope(pair, points))
            # Exported by natural_rope. Validate the staged pair before publication.
            with setup_cache.candidate(cache_root, cache_identity) as staged:
                setup_cache.copy_entry(run.evidence, staged)
                game = run.check('validate natural rope cache candidate',
                                 lambda: restore_rope(pair, j, staged, provenance))
                selected = setup_cache.publish(staged, cache_root, cache_identity, run.evidence)
        common.save(run.evidence/'setup-cache-selection.json', dict(result=cache_reason,
            key=setup_cache.cache_key(cache_identity), selected=str(selected), promoted=cache_reason != 'hit'))
    with run.phase('execution'):
        return run.check('source-backed climb and sixty-phase transition',
                         lambda: j.climb_vertical(pair, prepared=(game, points)))
