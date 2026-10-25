"""Beta installation/baseline contracts; synthetic records are not gameplay QA."""
import hashlib
import json
from pathlib import Path
import tempfile
import unittest

import port_debug
import port_modernize


class ModernizeTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.work = Path(self.temporary.name)
        (self.work/'port/src').mkdir(parents=True)
        (self.work/'port/src/lib.rs').write_text('// owned synthetic baseline\n')
        (self.work/'port/Cargo.toml').write_text('[package]\nname="fixture"\n')
        (self.work/'port/Cargo.lock').write_text('version = 4\n')
        (self.work/'workspace.json').write_text('{}')
        port_debug.install(self.work)

    def write(self, name, value):
        (self.work/'debug'/name).write_text(json.dumps(value)+'\n')

    def complete_records(self):
        plan = dict(version=1,status='ready',revision='synthetic',areas=['room'],
            mechanics=['move'],transitions=['enter'],checks=[dict(id='fixture',
                expectation='synthetic administrative fixture',setup='synthetic',
                aspects=['behavior'],covers=['area:room','mechanic:move','transition:enter'])])
        self.write('plan.json',plan)
        (self.work/'observation.json').write_text('{}')
        (self.work/'report.txt').write_text('Synthetic audit fixture, not fidelity evidence')
        qa = dict(version=1,status='complete',blockers=[],report='report.txt',
            plan_sha256=hashlib.sha256((self.work/'debug/plan.json').read_bytes()).hexdigest(),
            passes=[dict(number=n,status='passed',checks=[dict(id='fixture',status='passed',
                build='synthetic',evidence=['observation.json'])]) for n in [1,2]])
        self.write('qa.json',qa)
        return qa

    def test_install_before_qa_does_not_start_or_mutate_game(self):
        source = (self.work/'port/src/lib.rs').read_bytes()
        qa = (self.work/'debug/qa.json').read_bytes()
        installed = port_modernize.install(self.work)
        self.assertFalse(installed['agent_started'])
        self.assertFalse(installed['pilot_started'])
        self.assertEqual(json.loads((self.work/'modernize/candidates.json').read_text())['candidates'],[])
        self.assertEqual((self.work/'port/src/lib.rs').read_bytes(),source)
        self.assertEqual((self.work/'debug/qa.json').read_bytes(),qa)
        self.assertEqual(port_modernize.check(self.work)['status'],'baseline_ready')
        self.assertFalse((self.work/'modernize/baseline.json').exists())
        port_modernize.baseline(self.work)
        self.assertEqual((self.work/'debug/qa.json').read_bytes(), qa)

    def test_full_records_capture_exclusive_baseline_without_claiming_truth(self):
        self.complete_records()
        port_modernize.install(self.work)
        result = port_modernize.check(self.work)
        self.assertTrue(result['ok'])
        self.assertFalse(result['semantic_correctness_assessed'])
        result = port_modernize.baseline(self.work)
        self.assertFalse(result['pilot_started'])
        baseline = json.loads((self.work/'modernize/baseline.json').read_text())
        self.assertIn('port/Cargo.lock',baseline['source']['files'])
        with self.assertRaises(FileExistsError):
            port_modernize.baseline(self.work)
        (self.work/'port/src/lib.rs').write_text('// changed fixture\n')
        self.assertNotEqual(port_modernize.source_identity(self.work)['sha256'],baseline['source']['sha256'])
        self.assertEqual(json.loads((self.work/'modernize/baseline.json').read_text()),baseline)

    def test_scoped_partial_and_stale_records_do_not_gate_baseline(self):
        qa = self.complete_records()
        self.write('acceptance.json',port_debug.acceptance_contract('scoped',False))
        self.assertTrue(port_modernize.check(self.work)['ok'])
        self.write('acceptance.json',port_debug.acceptance_contract())
        qa['passes'].pop()
        self.write('qa.json',qa)
        self.assertTrue(port_modernize.check(self.work)['ok'])
        self.complete_records()
        with (self.work/'debug/plan.json').open('a') as stream:
            stream.write(' ')
        self.assertTrue(port_modernize.check(self.work)['ok'])

    def test_smoke_only_and_missing_qa_allow_baseline_but_missing_source_blocks(self):
        self.write('acceptance.json', port_debug.acceptance_contract(smoke_only=True))
        self.assertTrue(port_modernize.check(self.work)['ok'])
        for path in (self.work/'debug').iterdir():
            path.unlink()
        self.assertTrue(port_modernize.check(self.work)['ok'])
        port_modernize.install(self.work)
        port_modernize.baseline(self.work)
        baseline = json.loads((self.work/'modernize/baseline.json').read_text())
        self.assertEqual(baseline['qa']['qa_files'], {})
        (self.work/'port/Cargo.lock').unlink()
        self.assertEqual(port_modernize.check(self.work)['status'], 'awaiting_source_baseline')
        with self.assertRaisesRegex(ValueError, 'blocked'):
            port_modernize.baseline(self.work)

    def test_symlink_install_cannot_write_outside_workspace(self):
        self.complete_records()
        outside = self.work/'outside'
        outside.mkdir()
        (outside/'goal.md').write_text('unrelated instructions')
        (self.work/'modernize').symlink_to(outside,target_is_directory=True)
        with self.assertRaisesRegex(ValueError,'symlink'):
            port_modernize.install(self.work)
        with self.assertRaisesRegex(ValueError,'symlink'):
            port_modernize.baseline(self.work)
        self.assertFalse((outside/'baseline.json').exists())

    def test_install_preserves_records_and_preflights_instruction_conflict(self):
        port_modernize.install(self.work)
        status = self.work/'modernize/status.json'
        status.write_text('{"custom":"retain"}')
        port_modernize.install(self.work)
        self.assertEqual(status.read_text(),'{"custom":"retain"}')
        goal = self.work/'modernize/goal.md'
        goal.write_text('custom instructions')
        changes = self.work/'modernize/changes.json'
        changes.unlink()
        with self.assertRaisesRegex(ValueError,'preserve existing instructions'):
            port_modernize.install(self.work)
        self.assertFalse(changes.exists())


if __name__ == '__main__':
    unittest.main()
