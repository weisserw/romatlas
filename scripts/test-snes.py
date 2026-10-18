#!/usr/bin/env python3
"""Production SNES CLI/RPC regression using only original synthetic ROMs."""
import hashlib
import json
import os
from pathlib import Path
import subprocess
import tempfile
import wave

ROOT = Path(__file__).resolve().parents[1]
BIN = ROOT/'target/release/romatlas'
ASAR = ROOT/'tools/asar-build/asar/bin/asar'


def call(*args, ok=True):
    r = subprocess.run([str(BIN), *map(str,args)], capture_output=True, text=True, timeout=60)
    if ok and r.returncode:
        raise AssertionError(f'{args}: {r.stderr}')
    if not ok:
        assert r.returncode == 1 and json.loads(r.stderr)['error']
        return
    return json.loads(r.stdout)


def fixture(work, layout, second_controller=False):
    text=(ROOT/'scripts/fixtures/snes/fixture.asm').read_text()
    values=dict(MAPPING='lorom',BASE='$008000',HEADER='$00ffc0',END='$01ffff',VECTOR='$00fffc',MODE='$20',SIZE='$06',SRAM='$700000')
    if layout=='hi':
        values.update(MAPPING='hirom',BASE='$c08000',HEADER='$c0ffc0',END='$c0ffff',VECTOR='$c0fffc',MODE='$21',SRAM='$206000')
    for k,v in values.items(): text=text.replace(f'@{k}@',v)
    name=layout+'-p2' if second_controller else layout
    if second_controller:text=text.replace('lda.w $4016','lda.w $4017')
    asm=work/f'{name}.asm';asm.write_text(text)
    rom=work/f'{name}.sfc'
    subprocess.run([str(ASAR),'--no-title-check','--fix-checksum=off',str(asm),str(rom)],check=True,capture_output=True,timeout=30)
    return rom


class Session:
    def __init__(self,rom):
        self.process=subprocess.Popen([str(BIN),'serve','--rom',str(rom)],stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True)
        self.id=0
    def rpc(self,method,params=None,ok=True):
        self.id+=1
        self.process.stdin.write(json.dumps(dict(jsonrpc='2.0',id=self.id,method=method,params=params or {}))+'\n');self.process.stdin.flush()
        reply=json.loads(self.process.stdout.readline())
        if ok:
            assert 'error' not in reply,(method,reply)
            return reply['result']
        assert 'error' in reply,(method,reply)
        return reply['error']
    def close(self):
        self.process.stdin.close();assert self.process.wait(timeout=10)==0,self.process.stderr.read()


def runtime(work,rom,layout):
    s=Session(rom)
    try:
        assert s.rpc('api.describe')['platform']=='snes'
        s.rpc('emulator.start')
        s.rpc('trace.configure',dict(enabled=True,capacity=0))
        first=s.rpc('emulator.step',dict(count=1,timeout_ms=1000))
        coverage=s.rpc('analysis.coverage')['items']
        assert len(coverage)==1 and coverage[0]['address']['offset']==(0 if layout=='lo' else 0x8000)
        assert first['main']['pc']&0xffff==0x8001, 'next PC is not opcode coverage'
        s.rpc('emulator.input' ,dict(buttons=['B']))
        s.rpc('emulator.step',dict(unit='frame',count=2,timeout_ms=10000))
        assert s.rpc('emulator.physical_memory',dict(space='wram',address=0x100,length=3))['bytes']==[0x34,0x12,0x78]
        assert s.rpc('emulator.memory',dict(address=0x110,length=1))['bytes']==[1]
        assert s.rpc('emulator.physical_memory',dict(space='sram',length=1))['bytes']==[0x5a]
        assert s.rpc('emulator.physical_memory',dict(space='aram',address=0x200,length=3))['bytes']==[0x8f,0x6c,0xf2]
        assert s.rpc('analysis.coverage')['total']>0
        summary=s.rpc('trace.summary')
        assert summary['retained']==0 and summary['unique_contextual_roots']>0
        contexts=s.rpc('analysis.contexts',dict(limit=1000))['items']
        roots=[r['address'] for r in contexts if r['address']['space']=='rom' and rom.read_bytes()[r['address']['offset']] in (0xa9,0xa2)]
        assert roots
        root=roots[0]
        entry=dict(offset=root['offset'],cpu_address=int(root['context']['cpu_address']))
        static=s.rpc('analysis.discover',dict(entrypoints=[entry],max_instructions=8,use_observed=False))
        observed=s.rpc('analysis.discover',dict(entrypoints=[entry],max_instructions=8,use_observed=True))
        assert not static['candidates'] and observed['candidates'], 'observed width context lost at zero detail capacity'
        assert all(not c['execution_coverage'] for c in observed['candidates'])
        # Independent pages must reconstruct the same graph without changing its budget.
        params=dict(entrypoints=[entry, dict(offset=0, cpu_address=0)],
                    max_instructions=80, use_observed=True, limit=1000)
        whole=s.rpc('analysis.discover_context',params)
        assert whole['candidates'] and whole['references'] and whole['diagnostics']
        lists=[('candidates','offset','total','next_offset'),
               ('references','reference_offset','total_references','next_reference_offset'),
               ('diagnostics','diagnostic_offset','total_diagnostics','next_diagnostic_offset')]
        for method in ['analysis.discover', 'analysis.discover_context']:
            for key,offset_key,total_key,next_key in lists:
                collected=[]
                while len(collected)<whole[total_key]:
                    page=s.rpc(method,dict(params, **{offset_key:len(collected), 'limit':1}))
                    assert page[total_key]==whole[total_key]
                    assert page[next_key]==len(collected)+1
                    assert page['visited']==whole['visited']
                    assert page['budget_exhausted']==whole['budget_exhausted']
                    assert all(len(page[k])<=1 for k,_,_,_ in lists)
                    collected.extend(page[key])
                assert collected==whole[key]
            empty=s.rpc(method,dict(params,limit=0))
            assert all(empty[k]==[] and empty[t]==whole[t] for k,_,t,_ in lists)
            past=s.rpc(method,dict(params,offset=2**64-1,reference_offset=2**64-1,
                                   diagnostic_offset=2**64-1,limit=1))
            assert all(past[k]==[] and past[n]==whole[t] for k,_,t,n in lists)
            for invalid in [dict(limit=1001),dict(limit=-1),dict(offset='0'),
                            dict(reference_offset=-1),dict(diagnostic_offset=1.5)]:
                s.rpc(method,dict(params,**invalid),ok=False)
        before=s.rpc('emulator.state')
        peek_before=work/f'{layout}-peek-before.json';s.rpc('emulator.snapshot.export',dict(path=str(peek_before)))
        for space in ['rom','wram','aram','vram','cgram','oam','ipl']:
            s.rpc('emulator.physical_memory',dict(space=space,length=32))
        s.rpc('emulator.memory',dict(address=0x2140,length=1),ok=False)
        peek_after=work/f'{layout}-peek-after.json';s.rpc('emulator.snapshot.export',dict(path=str(peek_after)))
        assert json.loads(peek_before.read_text())['core_sha256']==json.loads(peek_after.read_text())['core_sha256']
        assert s.rpc('emulator.state')==before
        s.rpc('emulator.snapshot.save',dict(name='held'))
        snap=work/f'{layout}-state.json';s.rpc('emulator.snapshot.export',dict(path=str(snap)))
        s.rpc('emulator.audio.clear')
        after=s.rpc('emulator.step',dict(unit='frame',count=3,timeout_ms=10000))
        first=work/f'{layout}-first.wav';s.rpc('emulator.audio.export',dict(path=str(first)))
        s.rpc('emulator.input',dict(buttons=[]))
        s.rpc('emulator.snapshot.load',dict(name='held'))
        assert s.rpc('emulator.step',dict(unit='frame',count=3,timeout_ms=10000))==after
        second=work/f'{layout}-second.wav';s.rpc('emulator.audio.export',dict(path=str(second)))
        assert first.read_bytes()==second.read_bytes()
        with wave.open(str(first)) as wav:
            assert wav.getframerate()==48000 and wav.getnchannels()==2 and wav.getnframes()>1000
        assert any(first.read_bytes()[44:])
        picture=work/f'{layout}.ppm';s.rpc('emulator.framebuffer',dict(path=str(picture)))
        assert picture.read_bytes().startswith(b'P6\n')
        before=s.rpc('emulator.state')
        bad=json.loads(snap.read_text());bad['version']=0
        badpath=work/f'{layout}-old.json';badpath.write_text(json.dumps(bad))
        s.rpc('emulator.snapshot.import',dict(path=str(badpath)),ok=False)
        assert s.rpc('emulator.state')==before
        corrupt=json.loads(snap.read_text());corrupt['core'][0]^=1
        corrupt_path=work/f'{layout}-corrupt.json';corrupt_path.write_text(json.dumps(corrupt))
        s.rpc('emulator.snapshot.import',dict(path=str(corrupt_path)),ok=False)
        assert s.rpc('emulator.state')==before
        s.rpc('emulator.snapshot.import',dict(path=str(snap)))
        assert s.rpc('emulator.memory',dict(address=0x110,length=1))['bytes']==[1]
        s.rpc('emulator.step',dict(cpu='spc700',count=10,timeout_ms=1000))
        assert not s.rpc('emulator.state')['spc700']['mid_instruction']
        s.rpc('emulator.reset')
        assert s.rpc('emulator.physical_memory',dict(space='sram',length=1))['bytes']==[0x5a]
        s.rpc('emulator.step',dict(count=10,timeout_ms=1000))
        snapshot=s.rpc('emulator.state')
        s.rpc('emulator.registers.set',dict(registers=dict(pc=1<<24)),ok=False)
        assert s.rpc('emulator.state')==snapshot
        s.rpc('emulator.registers.set',dict(registers=dict(a=0x1234)))
        s.rpc('emulator.memory.write',dict(space='wram',address=0x120,bytes=[0x5a]))
        assert s.rpc('emulator.physical_memory',dict(space='wram',address=0x120,length=1))['bytes']==[0x5a]
        s.rpc('emulator.memory.write',dict(space='rom',address=0,bytes=[0]),ok=False)
        s.rpc('emulator.step',dict(unit='frame' ,count=1,max_operations=0),ok=False)
        s.rpc('trace.configure',dict(enabled=True,capacity=100))
        stream=work/f'{layout}-trace.jsonl';s.rpc('trace.stream.start',dict(path=str(stream),max_bytes=65536))
        s.rpc('emulator.step',dict(count=100,timeout_ms=1000))
        s.rpc('trace.stream.stop')
        assert stream.stat().st_size>0
        events=s.rpc('trace.query',dict(limit=100))['events']
        assert events and all(e['context']['artificial'] for e in events)
        s.rpc('emulator.memory.write',dict(space='sram',address=0,bytes=[0x55]))
        reset=s.rpc('emulator.reset');assert reset['artificial'] and reset['artificial_sram']
        assert s.rpc('emulator.physical_memory',dict(space='sram',length=1))['bytes']==[0x55]
        assert not s.rpc('emulator.start')['artificial']
    finally:s.close()


def reconstruction(work,rom,layout):
    mapping=work/f'{layout}.json';call('map','init',rom,'--output',mapping)
    data=json.loads(mapping.read_text())
    assert data['format']=='romatlas-snes-map' and not data['regions']
    source=work/f'{layout}-raw';call('generate',rom,'--map',mapping,'--out',source)
    assert call('verify',rom,'--generated',source)['byte_identical']
    # Opcode views read all operands from ROM. No values copied to the map.
    data['regions']=[dict(id=1,offset=0 if layout=='lo' else 0x8000,length=17,kind='Code',provenance='Static',
                          label='reset_console',comment='Enter native mode, establish stack and disable interrupt sources.',evidence='Synthetic program source contract',
                          views=[dict(cpu='main',cpu_address=0x8000 if layout=='lo' else 0xc08000,context=dict(e=True,m=True,x=True))])]
    # Stop at a known instruction boundary: sei/clc/xce/rep/sep/ldx/txs/stz = 14 bytes.
    data['regions'][0]['length']=14
    data['next_id']=2
    mapping.write_text(json.dumps(data))
    call('map','validate',mapping,'--rom',rom)
    out=work/f'{layout}-code';call('generate',rom,'--map',mapping,'--out',out)
    assert 'reset_console:' in (out/'game.asm').read_text()
    assert call('verify',rom,'--generated',out)['byte_identical']
    original=hashlib.sha256(rom.read_bytes()).hexdigest()
    linked=work/f'{layout}-alias.json';os.link(rom,linked)
    s=Session(rom)
    try:s.rpc('map.save',dict(path=str(linked)),ok=False)
    finally:s.close()
    assert hashlib.sha256(rom.read_bytes()).hexdigest()==original
    # Unsupported/ambiguous edits leave the loaded map intact.
    s=Session(rom)
    try:
        s.rpc('map.load',dict(path=str(mapping)))
        before=s.rpc('map.get')
        bad=dict(data['regions'][0]);bad['id']=0;bad['offset']+=1
        s.rpc('map.annotate',dict(region=bad),ok=False)
        assert s.rpc('map.get')==before
        discovered=s.rpc('analysis.discover',dict(entrypoints=[dict(offset=data['regions'][0]['offset'],context=dict(e=True,m=True,x=True))],max_instructions=8))
        assert discovered['candidates'] and all(not c['execution_coverage'] for c in discovered['candidates'])
        s.rpc('map.remove',dict(id=1))
        region=dict(id=0,offset=0x5000,length=16,kind='Padding',provenance='Static',evidence='Original synthetic zero-fill contract')
        assert s.rpc('map.annotate',dict(region=region))['id']==2
        s.rpc('map.remove',dict(id=2))
        assert s.rpc('map.annotate',dict(region=region))['id']==3
    finally:s.close()


def workflow(work, rom):
    def script(name,*args,ok=True):
        r=subprocess.run(['python3',str(ROOT/'scripts'/name),*map(str,args)],capture_output=True,text=True,timeout=300)
        if ok and r.returncode: raise AssertionError(f'{name}: {r.stderr}')
        if not ok: assert r.returncode != 0;return
        return json.loads(r.stdout)
    workspace=work/'snes-workspace'
    script('disassemble.py',rom,'--workspace',workspace,'--json')
    metadata=json.loads((workspace/'workspace.json').read_text());assert metadata['platform']=='snes'
    guidance=(workspace/'prompts/disassembly.txt').read_text()
    assert 'A runtime trace is not required' in guidance and 'fixed lengths' in guidance
    assert 'entry offset 256' not in (workspace/'workflow.md').read_text()
    subprocess.run(['python3',str(workspace/'run.py'),'baseline'],check=True,capture_output=True,timeout=60)
    # A reviewed synthetic reset routine, with honest unknown remainder.
    mapping=json.loads((work/'lo.json').read_text())
    for relative in ['maps/current.json','analysis/state.json']:
        (workspace/relative).write_text(json.dumps(mapping))
    subprocess.run(['python3',str(workspace/'run.py'),'checkpoint','pass-001'],check=True,capture_output=True,timeout=60)
    script('commentary.py','prepare',workspace,'--json')
    script('commentary.py','audit',workspace,ok=False)
    source=workspace/'commentary/source'
    game=(source/'game.asm').read_text()
    game='incsrc "constants.inc"\n'+game.replace('ldx.w #$1fff','ldx.w #reset_stack')
    (source/'game.asm').write_text(game)
    (source/'constants.inc').write_text('reset_stack = $1fff ; Native reset stack address\n')
    script('commentary.py','verify',workspace)
    script('commentary.py','read',workspace)
    assert 'reset_stack = $1fff' in (workspace/'commentary/reading-after.asm').read_text()
    review_path=workspace/'commentary/review.json'
    review=json.loads(review_path.read_text())
    for row in review['regions']:
        row.update(status='reviewed',summary='Synthetic reset contract' if row['kind']=='Code' else 'Unclassified bytes retained exactly; no structural claim.',source=['game.asm:1'])
        if row['kind']=='Code':
            row['units']=[dict(name='reset_console',offset=row['offset'],length=row['length'],status='reviewed',summary='Enter native mode, set stack and disable interrupt sources.',source=['game.asm:1'])]
    review['editorial_audit']=dict(status='reviewed',notes='Synthetic fixture reset routine has descriptive label and mechanical explanation; unknown ranges are explicit.')
    review_path.write_text(json.dumps(review))
    script('commentary.py','audit',workspace)
    archive=work/'snes-archive'
    script('archive.py','archive',workspace,'--output',archive,'--name','synthetic-snes')
    assert json.loads((archive/'archive.json').read_text())['platform']=='snes'
    assert not list(archive.rglob('*.sfc'))
    restored=work/'snes-restored'
    script('archive.py','restore',archive,'--rom',rom,'--out',restored)
    assert (restored/'source/game.asm').read_bytes()==(workspace/'commentary/source/game.asm').read_bytes()
    port=work/'snes-port'
    script('port.py','prepare',archive,'--rom',rom,'--out',port,'--json')
    assert 'snes::tile' in (port/'port/src/game.rs').read_text()
    assert 'gb::' not in (port/'port/src/game.rs').read_text()
    assert 'SNES_NTSC' in (port/'port/src/main.rs').read_text()
    script('port.py','check',port)
    script('port.py','headless',port,'--frames','2')
    script('port.py','audit',port,ok=False)


def large_discovery(work, original):
    rom=work/'large-discovery.sfc'
    data=bytearray(original.read_bytes())
    data[:8192]=bytes([0xea])*8192  # Original synthetic NOP graph, no assets.
    rom.write_bytes(data)
    session=Session(rom)
    try:
        params=dict(entrypoints=[dict(offset=i,context=dict(e=True,m=True,x=True))
                                 for i in range(100)],
                    max_instructions=4000,use_observed=True)
        first=session.rpc('analysis.discover_context',params)
        assert len(first['candidates'])==100 and first['total']==4000
        assert first['visited']==4000 and first['budget_exhausted']
        collected=[]
        while len(collected)<first['total']:
            page=session.rpc('analysis.discover_context',dict(params,offset=len(collected),limit=500))
            assert page['total']==first['total'] and page['budget_exhausted']
            collected.extend(page['candidates'])
        assert len({(c['offset'],json.dumps(c['view'],sort_keys=True)) for c in collected})==4000
        assert collected[:100]==first['candidates']
    finally:session.close()


def main():
    with tempfile.TemporaryDirectory(prefix='romatlas-snes-') as temporary:
        work=Path(temporary)
        for layout in ['lo','hi']:
            rom=fixture(work,layout)
            assert call('inspect',rom)['platform']=='snes'
            runtime(work,rom,layout);reconstruction(work,rom,layout)
            # Copier-header identity and preservation (not passed to cartridge).
            headered=work/f'{layout}.smc';headered.write_bytes(bytes([0x42])*512+rom.read_bytes())
            assert call('inspect',headered)['copier_header_size']==512
            call('run',headered,'--operations',10,'--timeout-ms',1000)
        p2=fixture(work,'lo',True);session=Session(p2)
        try:
            session.rpc('emulator.start');session.rpc('emulator.input',dict(port=2,buttons=['B']))
            session.rpc('emulator.input',dict(port=1,buttons=[]))
            session.rpc('emulator.step',dict(unit='frame',count=2,timeout_ms=10000))
            assert session.rpc('emulator.memory',dict(address=0x110,length=1))['bytes']==[1]
        finally:session.close()
        large_discovery(work,work/'lo.sfc')
        workflow(work,work/'lo.sfc')
        print('PASS: production SNES LoROM/HiROM RPC, snapshots, audio, SRAM, bounded traces, maps and exact reconstruction')

if __name__=='__main__':main()
