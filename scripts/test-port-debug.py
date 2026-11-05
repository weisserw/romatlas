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

    def test_pipeline_smoke_contract_requires_one_bounded_suite(self):
        self.pipeline_sanity()
        self.assertEqual(port_debug.audit(self.work)['passes'], 1)
        self.qa['passes'] = []
        self.write('qa.json', self.qa)
        with self.assertRaises(ValueError):
            port_debug.audit(self.work)

    def pipeline_sanity(self):
        self.write('acceptance.json', port_debug.acceptance_contract(smoke_only=True))
        self.qa['passes'] = self.qa['passes'][:1]
        def capture(name, data):
            (self.work/name).write_bytes(data)
            return dict(path=name, sha256=hashlib.sha256(data).hexdigest())
        identity = {key: capture(key, key.encode()) for key in ('rom', 'build', 'assets')}
        setup = capture('setup.py', b'# Synthetic deterministic capture recipe\n')
        self.sanity = dict(version=1, identity=identity, comparisons=[])
        self.sanity_results = dict(version=1, status='passed', comparisons=[])
        for aspect in ('rendering', 'scrolling', 'animation', 'audio'):
            reference = capture(aspect+'-original.raw', bytes([0, 0, 0, 255, 255, 255, 255, 255]))
            reference.update(producer='original-emulator', emulator='synthetic', recipe=setup)
            native = capture(aspect+'-native.raw', bytes([0, 0, 0, 255, 255, 255, 255, 255]))
            self.sanity['comparisons'].append(dict(id=aspect, aspect=aspect, setup=setup,
                expectation='Source-backed synthetic output', alignment='Frame 60 / sample 0',
                format='raw fixture bytes', reference=reference))
            self.sanity_results['comparisons'].append(dict(id=aspect, native=native,
                different_bytes=0, status='passed', review=dict(status='accepted',
                    criteria='Expected synthetic pattern', rationale='Both pixels match the independent fixture')))
        for row in self.qa['passes'][0]['checks']:
            row['build'] = identity['build']['sha256']
        self.write('qa.json', self.qa)
        self.write_sanity()

    def write_sanity(self):
        self.write('sanity.json', self.sanity)
        self.sanity_results['manifest_sha256'] = hashlib.sha256(
            (self.work/'debug/sanity.json').read_bytes()).hexdigest()
        self.write('sanity-results.json', self.sanity_results)

    def test_pipeline_rejects_self_baselines_stale_missing_failed_and_unreviewed_evidence(self):
        for failure in ('missing', 'stale', 'self', 'producer', 'review', 'failed',
                        'disabled', 'coverage', 'build', 'recipe', 'escape', 'manifest',
                        'missing-original', 'empty', 'size', 'metric'):
            with self.subTest(failure=failure):
                self.pipeline_sanity()
                policy = json.loads((self.work/'debug/qa-policy.json').read_text())
                policy['original_fidelity_sanity']['enabled'] = failure != 'disabled'
                self.write('qa-policy.json', policy)
                check = self.sanity['comparisons'][0]
                row = self.sanity_results['comparisons'][0]
                if failure == 'self': row['native'] = check['reference']
                if failure == 'producer': check['reference']['producer'] = 'native-port'
                if failure == 'review': row['review']['status'] = 'pending'
                if failure == 'failed': row['status'] = 'failed'
                if failure == 'coverage': self.sanity['comparisons'].pop()
                if failure == 'recipe': check['reference']['recipe']['sha256'] = '0'*64
                if failure == 'escape': row['native']['path'] = '../outside.raw'
                if failure == 'metric': row['different_bytes'] = False
                if failure in ('empty', 'size'):
                    data = b'' if failure == 'empty' else b'x'
                    (self.work/row['native']['path']).write_bytes(data)
                    row['native']['sha256'] = hashlib.sha256(data).hexdigest()
                    if failure == 'empty':
                        (self.work/check['reference']['path']).write_bytes(data)
                        check['reference']['sha256'] = hashlib.sha256(data).hexdigest()
                self.write_sanity()
                if failure == 'missing': (self.work/'debug/sanity-results.json').unlink()
                if failure == 'missing-original': (self.work/check['reference']['path']).unlink()
                if failure == 'stale': (self.work/row['native']['path']).write_bytes(b'stale')
                if failure == 'build': (self.work/'build').write_bytes(b'rebuilt')
                if failure == 'manifest': (self.work/'debug/sanity.json').write_text('{}')
                with self.assertRaises((ValueError, OSError)):
                    port_debug.audit(self.work)

    def test_pipeline_recomputes_difference_and_rejects_clear_only_renderer(self):
        self.pipeline_sanity()
        row = self.sanity_results['comparisons'][0]
        blank = bytes([0, 0, 0, 255]*2)
        (self.work/row['native']['path']).write_bytes(blank)
        row['native']['sha256'] = hashlib.sha256(blank).hexdigest()
        self.write_sanity()
        with self.assertRaisesRegex(ValueError, 'incorrect'):
            port_debug.audit(self.work)
        row['different_bytes'] = 3
        self.write_sanity()
        with self.assertRaisesRegex(ValueError, 'differs from original'):
            port_debug.audit(self.work)

    def test_intentional_difference_requires_independent_source(self):
        self.pipeline_sanity()
        check = self.sanity['comparisons'][0]
        row = self.sanity_results['comparisons'][0]
        data = bytes([1, 0, 0, 255, 255, 255, 255, 255])
        (self.work/row['native']['path']).write_bytes(data)
        row['native']['sha256'] = hashlib.sha256(data).hexdigest()
        row['different_bytes'] = 1
        check['intentional_difference'] = 'Source-backed rounding difference'
        self.write_sanity()
        with self.assertRaises(ValueError): port_debug.audit(self.work)
        check['difference_source'] = row['native']
        self.write_sanity()
        with self.assertRaisesRegex(ValueError, 'independent of captures'):
            port_debug.audit(self.work)
        check['difference_source'] = check['setup']
        self.write_sanity()
        self.assertTrue(port_debug.audit(self.work)['ok'])

    def test_complete_contract_and_installer_preserve_working_records(self):
        self.assertTrue(port_debug.audit(self.work)['ok'])
        before = (self.work/'debug/plan.json').read_bytes()
        before_qa = (self.work/'debug/qa.json').read_bytes()
        policy = json.loads((self.work/'debug/qa-policy.json').read_text())
        self.assertEqual(policy['gameplay']['execution'], 'native-only')
        policy['original_fidelity_sanity'].update(enabled=False, reason='Intentional widescreen')
        self.write('qa-policy.json', policy)
        result = port_debug.install(self.work)
        self.assertEqual(result['acceptance']['required_passes'], 2)
        self.assertEqual((self.work/'debug/plan.json').read_bytes(), before)
        self.assertEqual((self.work/'debug/qa.json').read_bytes(), before_qa)
        self.assertEqual(json.loads((self.work/'debug/qa-policy.json').read_text()), policy)
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

    def test_milestone_cycle_resume_archive_and_final_passes(self):
        result = port_debug.begin_cycle(self.work, 'm1')
        self.assertEqual(result['acceptance']['required_passes'], 1)
        archived = self.work/'debug/cycles/standalone'
        self.assertEqual(json.loads((archived/'qa.json').read_text()), self.qa)
        self.write('plan.json', self.plan)
        qa = copy.deepcopy(self.qa)
        qa['passes'] = qa['passes'][:1]
        self.write('qa.json', qa)
        self.assertEqual(port_debug.audit(self.work)['passes'], 1)
        self.write('qa.json', self.qa)  # Intermediate cycles must not run twice.
        with self.assertRaises(ValueError):
            port_debug.audit(self.work)
        qa['passes'][0]['checks'][0]['status'] = 'inaccessible'
        self.write('qa.json', qa)
        with self.assertRaises(ValueError):
            port_debug.audit(self.work)
        qa['passes'][0]['checks'][0]['status'] = 'passed'
        self.write('qa.json', qa)
        before = (self.work/'debug/qa.json').read_bytes()
        self.assertTrue(port_debug.begin_cycle(self.work, 'm1')['resumed'])
        self.assertEqual((self.work/'debug/qa.json').read_bytes(), before)
        with self.assertRaises(ValueError):
            port_debug.begin_cycle(self.work, 'm1', final=True)
        port_debug.begin_cycle(self.work, 'final', final=True)
        self.assertEqual((self.work/'debug/cycles/m1/qa.json').read_bytes(), before)
        self.write('plan.json', self.plan)
        self.write('qa.json', qa)
        with self.assertRaises(ValueError):
            port_debug.audit(self.work)
        self.write('qa.json', self.qa)
        self.assertEqual(port_debug.audit(self.work)['passes'], 2)
        with self.assertRaises(ValueError):
            port_debug.begin_cycle(self.work, 'm1')

    def test_incomplete_cycle_cannot_advance_or_erase_results(self):
        port_debug.begin_cycle(self.work, 'm1')
        before = (self.work/'debug/qa.json').read_bytes()
        with self.assertRaises(ValueError):
            port_debug.begin_cycle(self.work, 'm2')
        self.assertEqual((self.work/'debug/qa.json').read_bytes(), before)
        for invalid in ['../escape', '', '/absolute', 'space id']:
            with self.assertRaises(ValueError):
                port_debug.begin_cycle(self.work, invalid)

    def test_infrastructure_scope_allows_empty_categories_but_needs_checks(self):
        port_debug.begin_cycle(self.work, 'infrastructure')
        plan = copy.deepcopy(self.plan)
        plan.update(areas=[], transitions=[], checks=[plan['checks'][1]])
        self.write('plan.json', plan)
        qa = copy.deepcopy(self.qa)
        qa['plan_sha256'] = hashlib.sha256((self.work/'debug/plan.json').read_bytes()).hexdigest()
        qa['passes'] = [qa['passes'][0]]
        qa['passes'][0]['checks'] = [qa['passes'][0]['checks'][1]]
        self.write('qa.json', qa)
        self.assertTrue(port_debug.audit(self.work)['ok'])
        self.write('acceptance.json', port_debug.acceptance_contract('final', final=True))
        with self.assertRaises(ValueError):
            port_debug.audit(self.work)
        self.write('acceptance.json', port_debug.acceptance_contract('infrastructure'))
        plan['checks'] = []
        self.write('plan.json', plan)
        with self.assertRaises(ValueError):
            port_debug.audit(self.work)

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
