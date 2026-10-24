"""Runner contracts, independent of commercial ROMs and game fidelity."""
from pathlib import Path
import json
import subprocess
import sys
import tempfile
from types import SimpleNamespace
import unittest

from qa_harness import execute
from scenario_client import SetupFailure

ENTRY = {'id': 'synthetic', 'expectations': {'original': {'source': 'licensed-fixture'}}}


class HarnessTests(unittest.TestCase):
    def run_case(self, operation):
        with tempfile.TemporaryDirectory() as directory:
            evidence = Path(directory)/'fresh'
            result = execute(ENTRY, SimpleNamespace(run=operation), 'original', evidence)
            self.assertTrue((evidence/'result.json').exists())
            if result['status'] != 'passed':
                self.assertEqual((evidence/'retained.txt').read_text(), 'last observation')
            with self.assertRaises(FileExistsError):
                execute(ENTRY, SimpleNamespace(run=operation), 'original', evidence)
            return result

    def test_success_metrics_and_cleanup(self):
        closed = []
        def operation(run, entry):
            client = SimpleNamespace(serial=0)
            run.stack.callback(lambda: closed.append(True))
            with run.phase('setup'):
                run.clients['native'] = client
                client.serial += 2
            with run.phase('execution'):
                client.serial += 3
                run.check('authored expectation', lambda: None)
            return {'actual': 7}
        result = self.run_case(operation)
        self.assertEqual(result['status'], 'passed')
        self.assertEqual(result['requests'], {'native': 5})
        self.assertEqual([p['requests']['native'] for p in result['phases']], [2, 3])
        self.assertEqual(result['assertions'][0]['status'], 'passed')
        self.assertEqual(closed, [True])

    def test_suite_continues_and_profiles_are_explicit(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root/'adapter.py').write_text("def run(run, entry):\n"
                "    with run.phase('execution'):\n"
                "        def check():\n"
                "            assert entry['id'] != 'first', 'expected failure'\n"
                "        run.check('fixture contract', check)\n"
                "    return {'actual': 7}\n")
            manifest = dict(adapter='adapter.py', suites={'both': ['first', 'second']},
                tests=[dict(ENTRY, id=key) for key in ['first', 'second']])
            (root/'manifest.json').write_text(json.dumps(manifest))
            command = [sys.executable, str(Path(__file__).with_name('qa_harness.py')),
                '--manifest', str(root/'manifest.json'), '--suite', 'both',
                '--evidence', str(root/'evidence'), '--expectation']
            invalid = subprocess.run(command+['unwritten'], capture_output=True)
            self.assertEqual(invalid.returncode, 2)
            self.assertFalse((root/'evidence').exists())
            result = subprocess.run(command+['original'], capture_output=True)
            self.assertEqual(result.returncode, 1, result.stderr)
            summary = json.loads((root/'evidence/result.json').read_text())
            self.assertEqual([r['status'] for r in summary['results']],
                             ['game_mismatch', 'passed'])

    def test_empty_adapter_cannot_claim_success(self):
        with tempfile.TemporaryDirectory() as directory:
            result = execute(ENTRY, SimpleNamespace(run=lambda run,entry: {'claimed':'passed'}),
                             'original',Path(directory)/'evidence')
            self.assertEqual(result['status'],'setup_failure')
            self.assertIn('no named assertion',result['error'])

    def test_failure_classes_and_evidence(self):
        for stage, error, expected in [
            ('setup', AssertionError('identity'), 'setup_failure'),
            ('execution', SetupFailure('unmatched boundary'), 'setup_failure'),
            ('execution', TimeoutError('transport'), 'setup_failure'),
            ('execution', AssertionError('expected 60, actual 59'), 'game_mismatch')]:
            with self.subTest(stage=stage, error=error):
                def operation(run, entry):
                    with run.phase(stage):
                        def fail():
                            (run.evidence/'retained.txt').write_text('last observation')
                            raise error
                        run.check('contract', fail)
                result = self.run_case(operation)
                self.assertEqual(result['status'], expected)
                self.assertIn(str(error), result['traceback'])
                self.assertEqual(result['assertions'][0]['status'], 'failed')


if __name__ == '__main__':
    unittest.main()
