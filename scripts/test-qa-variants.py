"""Actual fixture variants: changed rule, baseline mismatch, unchanged regression."""
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile

root = Path(__file__).resolve().parents[1]
with tempfile.TemporaryDirectory() as temporary:
    for label, stride, profile, broken, expected in [
        ('original', '1', 'original-fidelity', False, ['passed','passed']),
        ('intentional-change-vs-original', '2', 'original-fidelity', False, ['game_mismatch','passed']),
        ('intentional-change', '2', 'double-speed', False, ['passed','passed']),
        ('unrelated-regression', '2', 'double-speed', True, ['passed','game_mismatch'])]:
        evidence = Path(temporary)/label
        env = dict(os.environ, ROMATLAS_QA_FIXTURE_STRIDE=stride,
                   ROMATLAS_QA_FIXTURE_BROKEN_PAUSE='1' if broken else '0')
        result = subprocess.run([sys.executable, root/'scripts/qa_harness.py', '--manifest',
            root/'examples/qa/fixture/integration.json', '--suite','all', '--expectation',profile,
            '--evidence', evidence], env=env, capture_output=True, text=True)
        summary = json.loads((evidence/'result.json').read_text())
        assert [r['status'] for r in summary['results']] == expected, (label, result.stderr, summary)
        assert result.returncode == (0 if all(s=='passed' for s in expected) else 1)
        print(label, expected)
print('QA variants: explicit changed expectations and unaffected regressions passed')
