#!/usr/bin/env python3
"""Synthetic acceptance-record regressions; does not run native/game fidelity QA."""
import copy
import hashlib
import json
from pathlib import Path
import tempfile
import unittest

import port_debug


class ContractTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix='romatlas-qa-contract-')
        self.addCleanup(self.temporary.cleanup)
        self.work = Path(self.temporary.name)
        (self.work/'port').mkdir()
        (self.work/'workspace.json').write_text('{}')
        (self.work/'port/Cargo.toml').write_text('[package]\nname="synthetic"\n')
        port_debug.install(self.work)
        self.plan = dict(version=1, revision='synthetic', status='ready',
            areas=['room'], mechanics=['movement'], transitions=['start'],
            checks=[dict(id='start', covers=['area:room','transition:start'],
                         aspects=['graphics','behavior'], expectation='Enter the room', setup='Press Start'),
                    dict(id='movement', covers=['mechanic:movement'], aspects=['behavior'],
                         expectation='Move to the obstacle', setup='Hold right')])
        (self.work/'observation.txt').write_text('Synthetic administrative fixture, not fidelity evidence')
        (self.work/'report.md').write_text('Synthetic record audit only')
        self.write('plan.json', self.plan)
        digest = hashlib.sha256((self.work/'debug/plan.json').read_bytes()).hexdigest()
        self.qa = dict(version=1, status='complete', plan_sha256=digest, blockers=[], report='report.md',
                       passes=[dict(number=n, status='passed', checks=[dict(id=c['id'], status='passed',
                            build='synthetic-build', evidence=['observation.txt']) for c in self.plan['checks']])
                               for n in [1,2]])
        self.write('qa.json', self.qa)

    def write(self, name, value):
        (self.work/'debug'/name).write_text(json.dumps(value)+'\n')

    def test_complete_contract_and_installer_preserve_working_records(self):
        self.assertTrue(port_debug.audit(self.work)['ok'])
        before = (self.work/'debug/plan.json').read_bytes()
        before_qa = (self.work/'debug/qa.json').read_bytes()
        result = port_debug.install(self.work)
        self.assertEqual(result['acceptance']['required_passes'], 2)
        self.assertEqual((self.work/'debug/plan.json').read_bytes(), before)
        self.assertEqual((self.work/'debug/qa.json').read_bytes(), before_qa)
        self.assertFalse(result['agent_started'])

    def test_repairs_preserve_current_pass_and_unaffected_results(self):
        first_pass = copy.deepcopy(self.qa['passes'][0])
        self.qa['passes'][1]['checks'][1].update(
            build='repaired-build', evidence=['observation.txt'])
        self.qa['passes'][1]['checks'][1]['repair'] = 'Fixed during pass two; start unaffected'
        self.write('qa.json', self.qa)
        self.assertTrue(port_debug.audit(self.work)['ok'])
        self.assertEqual(self.qa['passes'][0], first_pass)
        contract = port_debug.acceptance_contract()
        self.assertEqual(contract['restart_rule'], 'never_restart_continue_current_pass')
        self.assertEqual(contract['failure_rule'], 'fix_in_place_then_continue')
        contract['restart_rule'] = 'holistic_code_or_plan_change_restarts_at_pass_one'
        self.write('acceptance.json', contract)
        with self.assertRaisesRegex(ValueError, 'acceptance contract'):
            port_debug.audit(self.work)

    def test_partial_failed_inaccessible_and_budget_limited_are_not_success(self):
        cases = []
        for status in ['failed','inaccessible','pending','inconclusive','deferred']:
            qa = copy.deepcopy(self.qa);qa['passes'][1]['checks'][0]['status'] = status;cases.append(qa)
        qa = copy.deepcopy(self.qa);qa['passes'].pop();cases.append(qa)
        qa = copy.deepcopy(self.qa);qa['passes'][0]['status'] = 'failed';cases.append(qa)
        qa = copy.deepcopy(self.qa);qa['passes'].reverse();cases.append(qa)
        qa = copy.deepcopy(self.qa);qa['passes'][1]['checks'].reverse();cases.append(qa)
        qa = copy.deepcopy(self.qa);qa['blockers'] = ['cannot enter room'];cases.append(qa)
        qa = copy.deepcopy(self.qa);qa['status'] = 'incomplete';cases.append(qa)
        qa = copy.deepcopy(self.qa);qa['passes'][1]['checks'].pop();cases.append(qa)
        for qa in cases:
            with self.subTest(qa=qa):
                self.write('qa.json', qa)
                with self.assertRaises(ValueError):port_debug.audit(self.work)

    def test_changed_plan_missing_coverage_and_missing_evidence_reject(self):
        changed = copy.deepcopy(self.plan);changed['revision'] = 'corrected-understanding'
        self.write('plan.json', changed)
        with self.assertRaisesRegex(ValueError, 'different plan'):port_debug.audit(self.work)
        changed['mechanics'].append('boss')
        self.write('plan.json', changed)
        with self.assertRaisesRegex(ValueError, 'uncovered'):port_debug.audit(self.work)
        self.write('plan.json', self.plan)
        (self.work/'observation.txt').unlink()
        with self.assertRaises(OSError):port_debug.audit(self.work)


if __name__ == '__main__':
    unittest.main()
