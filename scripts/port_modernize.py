#!/usr/bin/env python3
"""Install optional beta instructions and inspect its QA prerequisite; no agents."""
import argparse
import hashlib
import json
from pathlib import Path
import shutil
import sys

import port_debug

ROOT = Path(__file__).resolve().parents[1]
TEMPLATES = ROOT/'scripts/modernize-template'


def workspace(path):
    path = Path(path).resolve(strict=True)
    if not (path/'workspace.json').is_file() or not (path/'port/Cargo.toml').is_file():
        raise ValueError('expected a prepared native-port workspace')
    return path


def digest(path):
    sha = hashlib.sha256()
    with path.open('rb') as stream:
        for block in iter(lambda: stream.read(65536), b''):
            sha.update(block)
    return sha.hexdigest()


def source_identity(work):
    files = sorted((work/'port/src').rglob('*'))
    files += [work/'port/Cargo.toml', work/'port/Cargo.lock', work/'workspace.json']
    records = {str(path.relative_to(work)): digest(path) for path in files if path.is_file()}
    if not any(name.startswith('port/src/') for name in records) or 'port/Cargo.lock' not in records:
        raise ValueError('baseline requires native source and Cargo.lock')
    sha = hashlib.sha256(json.dumps(records,sort_keys=True,separators=(',',':')).encode()).hexdigest()
    return dict(sha256=sha, files=records)


def install(path):
    work = workspace(path)
    target = work/'modernize'
    sources = sorted(TEMPLATES.iterdir())
    if target.is_symlink():
        raise ValueError('modernize directory must not be a symlink')
    # Preflight all collisions before creating any files; preserve working records.
    for source in sources:
        destination = target/source.name
        if destination.is_symlink():
            raise ValueError(f'preserve existing symlink: {destination}')
        if destination.exists() and source.name == 'goal.md' and destination.read_bytes()!=source.read_bytes():
            raise ValueError(f'preserve existing instructions; reconcile explicitly: {destination}')
    target.mkdir(exist_ok=True)
    for source in sources:
        destination = target/source.name
        if not destination.exists():
            shutil.copyfile(source,destination)
    return dict(ok=True,workflow='modernization-beta',goal=str(target/'goal.md'),
                agent_started=False,pilot_started=False)


def check(path):
    work = workspace(path)
    try:
        acceptance = json.loads((work/'debug/acceptance.json').read_text())
        if not isinstance(acceptance,dict):
            raise ValueError('invalid QA acceptance object')
        if acceptance.get('policy') != 'two_full_successful_qa_passes' or acceptance.get('required_passes') != 2:
            raise ValueError('requires completed full two-pass whole-game QA, not scoped QA')
        if acceptance.get('version') == 4 and not acceptance.get('final'):
            raise ValueError('scoped milestone QA cannot open the modernization gate')
        qa = json.loads((work/'debug/qa.json').read_text())
        if not isinstance(qa,dict) or qa.get('status') != 'complete':
            raise ValueError('full whole-game QA is incomplete')
        audit = port_debug.audit(work)
    except (OSError,ValueError,KeyError,TypeError) as error:
        return dict(ok=False,status='awaiting_full_qa',reason=str(error),pilot_started=False,
                    semantic_correctness_assessed=False)
    files = ['debug/acceptance.json','debug/plan.json','debug/qa.json']
    report = json.loads((work/'debug/qa.json').read_text())['report']
    files.append(report)
    return dict(ok=True,status='qa_records_complete',audit=audit,
                qa_files={name:digest(work/name) for name in files},
                semantic_correctness_assessed=False,pilot_started=False,
                next_action='Verify current source/build/ROM/assets provenance and source backup before baseline capture; no pilot launched')


def baseline(path):
    work = workspace(path)
    gate = check(work)
    if not gate['ok']:
        raise ValueError('modernization blocked: '+gate['reason'])
    if (work/'modernize').is_symlink():
        raise ValueError('modernize directory must not be a symlink')
    if not (work/'modernize/goal.md').is_file():
        raise ValueError('install the beta workflow first')
    record = dict(version=1,status='baseline_recorded',qa=gate,source=source_identity(work),
                  semantic_correctness_assessed=False,pilot_started=False)
    # Exclusive creation: neither changed code nor a repeated command can replace it.
    with (work/'modernize/baseline.json').open('x') as output:
        output.write(json.dumps(record,indent=2)+'\n')
    return dict(ok=True,status='baseline_recorded',pilot_started=False,
                baseline=str(work/'modernize/baseline.json'))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('command',choices=['install','check','baseline'])
    parser.add_argument('workspace',type=Path)
    args = parser.parse_args()
    try:
        result = {'install':install,'check':check,'baseline':baseline}[args.command](args.workspace)
        print(json.dumps(result))
        return 0 if result['ok'] else 1
    except (OSError,ValueError,KeyError,TypeError) as error:
        print(json.dumps(dict(ok=False,error=str(error),pilot_started=False)),file=sys.stderr)
        return 1


if __name__ == '__main__':
    raise SystemExit(main())
