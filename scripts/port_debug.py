#!/usr/bin/env python3
"""Install native QA instructions or audit its records; never launch gameplay QA."""
import argparse
import hashlib
import json
from pathlib import Path
import shutil
import sys

ROOT = Path(__file__).resolve().parents[1]


def acceptance_contract():
    return dict(version=3, status='agreed', policy='two_full_successful_qa_passes',
        required_passes=2, failure_rule='fix_in_place_then_continue',
        restart_rule='never_restart_continue_current_pass',
        resource_ceiling=None, resource_limit_outcome='incomplete')


def audit(work):
    """Administrative acceptance checks; does not evaluate evidence truth."""
    work = work.resolve(strict=True)
    acceptance = json.loads((work/'debug/acceptance.json').read_text())
    if acceptance.get('policy') != 'two_full_successful_qa_passes' or acceptance.get('required_passes') != 2 or acceptance.get('status') != 'agreed' or acceptance.get('restart_rule') != 'never_restart_continue_current_pass':
        raise ValueError('expected the agreed two-pass acceptance contract')
    plan_bytes = (work/'debug/plan.json').read_bytes()
    plan = json.loads(plan_bytes)
    qa = json.loads((work/'debug/qa.json').read_text())
    if plan.get('version') != 1 or plan.get('status') != 'ready' or not isinstance(plan.get('revision'), str) or not plan['revision'].strip():
        raise ValueError('QA plan is not ready')
    def text(value):
        return isinstance(value, str) and bool(value.strip())
    coverage = set()
    for key, prefix in [('areas', 'area'), ('mechanics', 'mechanic'), ('transitions', 'transition')]:
        items = plan.get(key)
        if not isinstance(items, list) or not items or any(not text(v) for v in items) or len(set(items)) != len(items):
            raise ValueError(f'plan needs unique nonempty {key}')
        coverage.update(f'{prefix}:{item}' for item in items)
    checks = plan.get('checks')
    if not isinstance(checks, list) or not checks:
        raise ValueError('plan needs ordered checks')
    ids = []
    covered = set()
    for check in checks:
        if not isinstance(check, dict) or any(not text(check.get(k)) for k in ['id', 'expectation', 'setup']):
            raise ValueError('check needs id, expectation and reproducible setup')
        aspects = check.get('aspects')
        links = check.get('covers')
        if not isinstance(aspects, list) or not aspects or any(not isinstance(v, str) or v not in {'graphics','behavior','audio'} for v in aspects):
            raise ValueError('check needs applicable graphics/behavior/audio aspects')
        if not isinstance(links, list) or not links or any(not isinstance(v, str) or v not in coverage for v in links):
            raise ValueError('check has missing/unknown coverage links')
        ids.append(check['id']);covered.update(links)
    if len(set(ids)) != len(ids) or covered != coverage:
        raise ValueError('duplicate checks or uncovered plan items')
    if qa.get('version') != 1 or qa.get('status') != 'complete' or qa.get('blockers') != []:
        raise ValueError('QA is incomplete or has blockers')
    if qa.get('plan_sha256') != hashlib.sha256(plan_bytes).hexdigest():
        raise ValueError('QA results refer to a different plan; reconcile affected checks and plan hash without restarting')
    def artifact(name):
        if not text(name) or Path(name).is_absolute():
            raise ValueError('evidence/report must name a local workspace file')
        path = (work/name).resolve(strict=True)
        path.relative_to(work)
        if not path.is_file():
            raise ValueError('evidence/report is not a file')
    passes = qa.get('passes')
    if not isinstance(passes, list) or len(passes) != 2:
        raise ValueError('two complete successful passes are required')
    for number, result in enumerate(passes, 1):
        if result.get('number') != number or result.get('status') != 'passed':
            raise ValueError('passes must succeed in order, first then second')
        rows = result.get('checks')
        if not isinstance(rows, list) or [r.get('id') for r in rows] != ids:
            raise ValueError('each pass must execute the entire ordered plan')
        for row in rows:
            if row.get('status') != 'passed' or not text(row.get('build')):
                raise ValueError('failed/inaccessible/pending checks cannot count as passed')
            evidence = row.get('evidence')
            if not isinstance(evidence, list) or not evidence:
                raise ValueError('passing check needs evidence')
            for name in evidence:
                artifact(name)
    artifact(qa.get('report'))
    return dict(ok=True, passes=2, checks_per_pass=len(ids),
                acceptance='two_full_successful_qa_passes', semantic_correctness_assessed=False)


def install(work):
    work = work.resolve(strict=True)
    if not (work / 'workspace.json').is_file() or not (work / 'port/Cargo.toml').is_file():
        raise ValueError('expected an existing native port workspace')
    debug = work / 'debug'
    debug.mkdir(exist_ok=True)
    targets = {work/'DEBUGGING.md': ROOT/'docs/port-debugging.md',
               debug/'goal.md': ROOT/'scripts/port-debug-template/goal.md',
               debug/'plan.json': ROOT/'scripts/port-debug-template/plan.json',
               debug/'qa.json': ROOT/'scripts/port-debug-template/qa.json'}
    for target, source in targets.items():
        if target.exists() and target.name not in {'plan.json', 'qa.json'} and target.read_bytes() != source.read_bytes():
            raise ValueError(f'preserve existing instructions; reconcile explicitly: {target}')
    for target, source in targets.items():
        if not target.exists():
            shutil.copyfile(source, target)
    acceptance = debug/'acceptance.json'
    if not acceptance.exists():
        acceptance.write_text(json.dumps(acceptance_contract(), indent=2)+'\n')
    return dict(ok=True, workspace=str(work), goal=str(debug/'goal.md'),
                acceptance=json.loads(acceptance.read_text()), agent_started=False)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('command', choices=['install', 'status', 'audit'])
    parser.add_argument('workspace', type=Path)
    args = parser.parse_args()
    if args.command == 'install':
        result = install(args.workspace)
    elif args.command == 'audit':
        result = audit(args.workspace)
    else:
        result = json.loads((args.workspace/'debug/acceptance.json').read_text())
    print(json.dumps(result))


if __name__ == '__main__':
    try:
        main()
    except (ValueError, OSError, KeyError, TypeError, AttributeError) as error:
        print(json.dumps(dict(ok=False, error=str(error))), file=sys.stderr)
        raise SystemExit(1)
