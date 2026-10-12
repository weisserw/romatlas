#!/usr/bin/env python3
"""Run pinned source-built MIT Mooneye tests; no commercial ROMs."""
import argparse, json, pathlib, subprocess, concurrent.futures, re
root = pathlib.Path(__file__).resolve().parents[1]
parser = argparse.ArgumentParser()
parser.add_argument('--report', type=pathlib.Path, default=None)
selection = parser.add_mutually_exclusive_group()
selection.add_argument('--all-dmg', action='store_true', help='Probe all compatible DMG/MBC1/MBC5 tests; nonpasses fail the command')
parser.add_argument('--binary', type=pathlib.Path, default=root/'target/release/romatlas')
selection.add_argument('--all-cgb', action='store_true', help='Probe acceptance tests whose source verifies CGB/CGBABCDE, plus CGB boot tests')
selection.add_argument('--cgb-boot', action='store_true', help='Run the two supported CGB boot-state tests')
parser.add_argument('--match', help='Run selected tests whose path contains this text')
args = parser.parse_args()
if args.report is None:
    args.report = root/('build/hardware-cgb-probe.json' if args.all_cgb else 'build/hardware-probe.json' if args.all_dmg else 'build/hardware-gate.json')
assert subprocess.check_output(['git','-C',str(root/'tools/mooneye'),'rev-parse','HEAD'],text=True).strip() == '31510e12eea6286d36eea060a6adde755e1067aa'
paths = sorted((root/'tools/mooneye/build/acceptance').rglob('*.gb'))
paths += sorted((root/'tools/mooneye/build/emulator-only/mbc1').rglob('*.gb'))
paths += sorted((root/'tools/mooneye/build/emulator-only/mbc5').rglob('*.gb'))
cgb_paths=[root/'tools/mooneye/build/misc/boot_regs-cgb.gb',root/'tools/mooneye/build/misc/boot_div-cgbABCDE.gb']
if args.all_cgb:
    def verified_cgb(source):
        return any(re.search(r'\bCGB(?:ABCDE)?\b',line) for line in re.findall(r'pass:\s*([^\n]*)',source.read_text()))
    sources = sorted((root/'tools/mooneye/acceptance').rglob('*.s'))
    paths = [root/'tools/mooneye/build'/source.relative_to(root/'tools/mooneye').with_suffix('.gb') for source in sources if verified_cgb(source)] + cgb_paths
    assert all(p.is_file() for p in paths), 'missing source-built CGB tests'
elif args.cgb_boot:
    paths=cgb_paths
elif not args.all_dmg:
    paths+=cgb_paths
def eligible(path):
    name = path.stem
    if any(x in name for x in ['-S','-C','dmg0','-sgb','-mgb']) and 'GS' not in name:
        return False
    return True
if not args.all_cgb:
    paths = [p for p in paths if eligible(p)]
if not args.all_dmg and not args.cgb_boot and not args.all_cgb:
    gate = json.loads((root/'tests/mooneye-gate.json').read_text())
    paths = [p for p in paths if str(p.relative_to(root/'tools/mooneye/build')) in gate]
    assert len(paths) == len(gate), 'missing source-built gate tests'
if args.match:
    paths = [p for p in paths if args.match in str(p)]
def run(path):
    name = str(path.relative_to(root/'tools/mooneye/build'))
    try:
        r = subprocess.run([str(args.binary),'test-rom',str(path),'--timeout-ms','10000','--model','cgb' if args.all_cgb or path in cgb_paths else 'auto'],capture_output=True,text=True,timeout=15)
        if r.returncode == 0:
            result = json.loads(r.stdout)
        else:
            result = {'status':'error','error':r.stderr.strip()[-3000:]}
    except subprocess.TimeoutExpired:
        result = {'status':'process_timeout'}
    return {'test':name,**result}
with concurrent.futures.ThreadPoolExecutor(max_workers=2) as pool:
    results = list(pool.map(run, paths))
args.report.parent.mkdir(parents=True,exist_ok=True)
args.report.write_text(json.dumps({'suite_revision':'31510e12eea6286d36eea060a6adde755e1067aa','results':results},indent=2)+'\n')
counts = {}
for result in results:
    counts[result['status']] = counts.get(result['status'],0)+1
print(json.dumps({'tests':len(results),'counts':counts,'report':str(args.report)}))
for result in results:
    if result['status'] != 'passed':
        print(result['test'], result['status'], result.get('error',''))
raise SystemExit(0 if results and all(r['status']=='passed' for r in results) else 1)
