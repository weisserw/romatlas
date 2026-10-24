"""Small game-owned integration runner; never writes QA acceptance records."""
import argparse
from contextlib import ExitStack, contextmanager
import importlib.util
import json
from pathlib import Path
import time
import traceback

from scenario_client import SetupFailure


def save(path, value):
    path.write_text(json.dumps(value, indent=2) + '\n')


class Run:
    def __init__(self, evidence, expectation):
        self.evidence = evidence
        self.expectation = expectation
        self.clients = {}
        self.phases = []
        self.assertions = []
        self.stage = 'setup'
        self.stack = ExitStack()

    def counts(self):
        return {name: client.serial for name, client in self.clients.items()}

    @contextmanager
    def phase(self, name):
        self.stage = name
        before = self.counts()
        start = time.monotonic()
        try:
            yield
        finally:
            self.phases.append(dict(name=name, seconds=time.monotonic()-start,
                requests={k: v-before.get(k, 0) for k, v in self.counts().items()}))

    def check(self, name, operation):
        """Record a named group of existing source-backed assertions, including failure."""
        record = dict(name=name, phase=self.stage, status='failed')
        self.assertions.append(record)
        try:
            value = operation()
        except Exception as error:
            record['error'] = str(error)
            raise
        record['status'] = 'passed'
        return value


def execute(entry, adapter, expectation, evidence):
    evidence.mkdir(parents=True, exist_ok=False)
    run = Run(evidence, expectation)
    start = time.monotonic()
    result = dict(schema_version=1, id=entry['id'], expectation=expectation,
                  expectation_contract=entry['expectations'][expectation],
                  status='setup_failure', evidence=str(evidence.resolve()))
    try:
        with run.stack:
            result['observations'] = adapter.run(run, entry)
            if not any(a.get('phase') == 'execution' and a.get('status') == 'passed' for a in run.assertions):
                raise SetupFailure('adapter executed no named assertion group; empty tests cannot pass')
        result['status'] = 'passed'
    except Exception as error:
        if isinstance(error, AssertionError):
            frames = traceback.extract_tb(error.__traceback__)
            if frames:
                frame = frames[-1]
                run.assertions.append(dict(name=f'{frame.filename}:{frame.lineno}',
                    phase=run.stage, status='failed', error=str(error)))
        result.update(status='game_mismatch' if isinstance(error, AssertionError)
                      and not isinstance(error, SetupFailure) and run.stage == 'execution'
                      else 'setup_failure', error=str(error), traceback=traceback.format_exc())
    finally:
        result.update(seconds=time.monotonic()-start, phases=run.phases,
                      requests=run.counts(), assertions=run.assertions)
        save(evidence/'result.json', result)
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--manifest', type=Path, required=True)
    select = parser.add_mutually_exclusive_group(required=True)
    select.add_argument('--test')
    select.add_argument('--suite')
    parser.add_argument('--expectation', required=True)
    parser.add_argument('--evidence', type=Path, required=True)
    args = parser.parse_args()
    if not __debug__:
        parser.error('assertions require Python without -O')
    manifest = json.loads(args.manifest.read_text())
    if args.suite and args.suite not in manifest['suites']:
        parser.error(f'unknown suite {args.suite}')
    ids = [args.test] if args.test else manifest['suites'][args.suite]
    tests = {e['id']: e for e in manifest['tests']}
    if (not ids or len(set(ids)) != len(ids) or len(tests) != len(manifest['tests'])
            or any(key not in tests or Path(key).name != key or key in ('.', '..') for key in ids)):
        parser.error('test IDs must be known, unique, nonempty basenames')
    entries = [tests[key] for key in ids]
    for entry in entries:
        if args.expectation not in entry['expectations']:
            parser.error(f'{entry["id"]}: undocumented expectation {args.expectation}')
    adapter_path = (args.manifest.resolve().parent/manifest['adapter']).resolve()
    spec = importlib.util.spec_from_file_location('game_qa_adapter', adapter_path)
    adapter = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(adapter)
    args.evidence.mkdir(parents=True, exist_ok=False)
    save(args.evidence/'manifest.json', manifest)
    results = [execute(e, adapter, args.expectation, args.evidence/e['id']) for e in entries]
    summary = dict(schema_version=1, status='passed' if all(r['status']=='passed' for r in results)
                   else 'failed', results=results)
    save(args.evidence/'result.json', summary)
    print(json.dumps(summary, indent=2))
    return 0 if summary['status']=='passed' else 1


if __name__ == '__main__':
    raise SystemExit(main())
