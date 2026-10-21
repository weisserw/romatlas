#!/usr/bin/env python3
"""Original synthetic PC Engine CLI/RPC, bank identity, replay and rebuilding regression.
No commercial ROMs or game assets. Generated files live under ignored target/.
"""
import json, pathlib, subprocess, tempfile, shutil, hashlib, sys, os, zlib
ROOT=pathlib.Path(__file__).resolve().parents[1]
BIN=ROOT/'target/release/romatlas'
os.environ.setdefault('CARGO_TARGET_DIR',str(ROOT/'target/workflow-checks'))
def call(*args,ok=True):
 p=subprocess.run([str(BIN),*map(str,args)],capture_output=True,text=True)
 if ok: assert p.returncode==0,(args,p.stderr)
 else: assert p.returncode==1,(args,p.stdout);return json.loads(p.stderr)
 return json.loads(p.stdout)
class Rpc:
 def __init__(self,rom):self.directory=rom.parent;self.serial=0;self.p=subprocess.Popen([str(BIN),'serve','--rom',str(rom)],stdin=subprocess.PIPE,stdout=subprocess.PIPE,text=True);self.i=0
 def call(self,m,p=None,ok=True):
  p=p or {}
  snapshot_file=None
  if m=='emulator.snapshot.export' and 'path' not in p:
   self.serial+=1;snapshot_file=self.directory/f'snapshot-{id(self)}-{self.serial}.json';p=dict(path=str(snapshot_file))
  if m=='emulator.snapshot.import' and 'snapshot' in p:
   self.serial+=1;target=self.directory/f'import-{id(self)}-{self.serial}.json';target.write_text(json.dumps(p['snapshot']));p=dict(path=str(target))
  self.i+=1;self.p.stdin.write(json.dumps(dict(jsonrpc='2.0',id=self.i,method=m,params=p or {}))+'\n');self.p.stdin.flush();v=json.loads(self.p.stdout.readline())
  if ok:assert 'error' not in v,(m,p,v);return json.loads(snapshot_file.read_text()) if snapshot_file else v['result']
  assert 'error'in v,(m,v);return v['error']
 def close(self):self.p.stdin.close();assert self.p.wait()==0

def fixture(banks=4,prefix=False):
 data=bytearray([0xea])*(banks*8192)
 # Real MPR setup, RAM write, high speed; repeatedly read pad and increment RAM.
 code=bytes.fromhex('78 d8 a9 ff 53 01 a9 f8 53 02 a2 ff 9a d4 a9 2a 85 01 a9 02 8d 00 10 a9 00 8d 00 10 ad 00 10 85 02 e6 00 4c 12 e0')
 data[:len(code)]=code
 for n in range(0x1ff6,0x2000,2):data[n:n+2]=bytes([0,0xe0])
 return (bytes([0x17])*512 if prefix else b'')+data,len(code)
def synthetic_crc(data,target):
 # Solve the 32-bit influence matrix for four authored fixture padding bytes.
 data=bytearray(data);data[-4:]=bytes(4);base=zlib.crc32(data);basis={}
 for bit in range(32):
  sample=data.copy();sample[-4+bit//8]^=1<<(bit%8);value=zlib.crc32(sample)^base;mask=1<<bit
  while value:
   pivot=value.bit_length()-1
   if pivot not in basis:basis[pivot]=(value,mask);break
   other,encoded=basis[pivot];value^=other;mask^=encoded
 value=target^base;mask=0
 while value:
  pivot=value.bit_length()-1;other,encoded=basis[pivot];value^=other;mask^=encoded
 data[-4:]=mask.to_bytes(4,'little');assert zlib.crc32(data)==target;return bytes(data)
def main():
 parent=ROOT/'target/pce-tests';parent.mkdir(exist_ok=True)
 with tempfile.TemporaryDirectory(dir=parent) as tmp:
  out=pathlib.Path(tmp);rom=out/'test.pce';data,length=fixture();rom.write_bytes(data)
  info=call('inspect',rom);assert info['platform']=='pce' and info['mapper']=='linear' and info['sha256']==hashlib.sha256(data).hexdigest()
  call('run',rom,'--operations',20,'--timeout-ms',1000)
  mapping=out/'map.json';call('map','init',rom,'--output',mapping)
  s=Rpc(rom);s.call('emulator.start');initial=s.call('emulator.snapshot.export')
  for space,address,n in [('rom',0,16),('ram',0,16),('vram',0,16),('cram',0,16),('satb',0,16),('cpu',0xe000,16),('bus',0x1fe000,4),('io',0,4)]:s.call('emulator.memory',dict(space=space,address=address,length=n))
  assert s.call('emulator.snapshot.export')==initial,'inspection changed hardware'
  s.call('trace.configure',dict(enabled=True,capacity=2000));s.call('emulator.input',dict(buttons=['i']));s.call('emulator.step',dict(count=20))
  assert s.call('emulator.physical_memory',dict(space='ram',address=1,length=1))['bytes']==[42]
  # Exercise real PSG hardware writes and capture audio, not only native synthesis.
  s.call('emulator.memory.write',dict(space='bus',address=0x1fe800,bytes=[0,255]))
  s.call('emulator.memory.write',dict(space='bus',address=0x1fe804,bytes=[0xdf,0xff,31]))
  # Real palette port auto-increment and committed backing trace.
  s.call('emulator.memory.write',dict(space='bus',address=0x1fe402,bytes=[1,0,42,1]))
  saved=s.call('emulator.snapshot.export')
  def segment():
   s.call('trace.clear');s.call('emulator.audio.clear');v=s.call('emulator.step',dict(unit='frame',count=2,max_operations=200000,timeout_ms=10000));return v,s.call('emulator.snapshot.export'),s.call('emulator.framebuffer'),s.call('trace.query',dict(limit=100))
  first=segment();s.call('emulator.snapshot.import',dict(snapshot=saved));second=segment();assert first==second,'snapshot replay mismatch'
  wav=out/'audio.wav';s.call('emulator.audio.export',dict(path=str(wav)));assert wav.read_bytes()[:4]==b'RIFF' and any(wav.read_bytes()[44:])
  assert first[0]['frames']>=2 and first[2]['height']>0
  assert s.call('analysis.coverage')['total']>0 and s.call('analysis.edges')['total']>0
  assert s.call('emulator.physical_memory',dict(space='ram',address=2,length=1))['bytes'][0]&1==0
  s.call('trace.configure',dict(enabled=True,capacity=0));s.call('emulator.step',dict(count=20));assert s.call('trace.summary')['retained']==0 and s.call('analysis.contexts')['total']>0
  before=s.call('emulator.snapshot.export');s.call('emulator.snapshot.import',dict(snapshot=dict(before,rom_sha256='00'*32)),ok=False);assert before==s.call('emulator.snapshot.export')
  region=dict(id=0,offset=0,length=length,kind='Code',provenance='Manual',label='Reset',comment='Synthetic MPR/input loop',views=[dict(cpu='main',cpu_address=0xe000,context=dict(mpr=[None]*7+[0]))])
  s.call('map.annotate',dict(region=region));s.call('map.save',dict(path=str(mapping)));before_map=s.call('map.get');s.call('map.annotate',dict(region=dict(region,offset=1)),ok=False);assert before_map==s.call('map.get')
  s.call('emulator.registers.set',dict(registers=dict(pc=0xe000,a=7,mpr2=1)));before=s.call('emulator.state');s.call('emulator.registers.set',dict(registers=dict(a=8,pc=65536)),ok=False);assert before==s.call('emulator.state')
  for space,at,value in [('ram',3,42),('vram',20,55),('cram',2,77),('satb',4,24)]:
   s.call('emulator.memory.write',dict(space=space,address=at,bytes=[value]));assert s.call('emulator.physical_memory',dict(space=space,address=at,length=1))['bytes']==[value]
  s.call('emulator.memory.write',dict(space='cpu',address=0x4000,bytes=[123]));assert s.call('emulator.memory',dict(space='cpu',address=0x4000,length=1))['bytes']==[0xea],'ROM writes must not change backing'
  s.call('emulator.physical_memory',dict(space='bus',address=0,length=1),ok=False)
  discovered=s.call('analysis.discover_context',dict(entrypoints=[dict(offset=0,cpu_address=0xe000,context=dict(mpr=[None]*7+[0]))],max_instructions=12,use_observed=False));assert discovered['total']>0
  s.call('emulator.memory.write',dict(space='ram',address=10,bytes=[0,0x40]));pointers=s.call('analysis.pointers',dict(offset=0,count=1,context=dict(mpr=[None]*7+[0])));assert not pointers['hypotheses'][0]['code_proven']
  tg_saved=s.call('emulator.snapshot.export');s.call('emulator.start',dict(region='pce'));assert s.call('emulator.state')['region']=='pce'
  assert s.call('emulator.memory',dict(space='io',address=0x1000,length=1))['bytes'][0]&64
  s.call('emulator.reset');assert s.call('emulator.state')['region']=='pce';s.call('emulator.snapshot.import',dict(snapshot=tg_saved),ok=False)
  s.close()
  generated=out/'source';call('generate',rom,'--map',mapping,'--out',generated);assert 'tam #$01' in (generated/'game.asm').read_text();assert call('verify',rom,'--generated',generated)['byte_identical']
  # Stock recipient reconstruction uses the external tools directly.
  stock=out/'stock.pce'
  subprocess.run([str(ROOT/'tools/cc65/bin/ca65'),'--cpu','huc6280','game.asm','-o','stock.o'],cwd=generated,check=True,capture_output=True)
  subprocess.run([str(ROOT/'tools/cc65/bin/ld65'),'-C','layout.cfg','stock.o','-o',str(stock.resolve())],cwd=generated,check=True,capture_output=True)
  assert stock.read_bytes()==data
  (generated/'game.o').write_bytes(b'stale');call('verify',rom,'--generated',generated)
  (generated/'game.asm').write_text((generated/'game.asm').read_text().replace('lda #$2a','lda #$29',1));call('verify',rom,'--generated',generated,ok=False)
  # Every upstream HuC6280 opcode, including all extensions and undefined NOPs.
  stream=bytearray()
  sample=out/'op.pce';sample.write_bytes(data)
  for op in range(256):
   image=bytearray(data);image[:7]=bytes([op,1,0,0,0,1,0]);sample.write_bytes(image)
   i=call('decode',sample,'--offset',0,'--count',1)['instructions'][0]['instruction'];stream+=image[:i['length']]
  allrom=out/'all.pce';image=bytearray(data);image[:len(stream)]=stream;allrom.write_bytes(image)
  allmap=out/'all.json';call('map','init',allrom,'--output',allmap);m=json.loads(allmap.read_text());m['regions']=[dict(region,length=len(stream),id=1)];m['next_id']=2;allmap.write_text(json.dumps(m))
  call('generate',allrom,'--map',allmap,'--out',out/'all-source');call('verify',allrom,'--generated',out/'all-source')
  for banks,prefix in [(48,False),(64,True),(96,False),(128,False)]:
   path=out/f'layout-{banks}.pce';image,_=fixture(banks,prefix);path.write_bytes(image);mappath=out/f'layout-{banks}.json';call('map','init',path,'--output',mappath);call('generate',path,'--map',mappath,'--out',out/f'layout-{banks}-source');call('verify',path,'--generated',out/f'layout-{banks}-source');call('run',path,'--operations',10)
  pointer_rom=out/'pointers.pce';image=bytearray(data);image[0x100:0x108]=bytes.fromhex('00 e0 01 00 e0 00 e0 01');pointer_rom.write_bytes(image)
  rpc=Rpc(pointer_rom)
  assert rpc.call('analysis.pointers',dict(offset=0x100,context=dict(mpr=[None]*7+[0])))['hypotheses'][0]['target']['offset']==0
  for offset,layout in [(0x102,'bank8_first'),(0x105,'bank8_last')]:
   assert rpc.call('analysis.pointers',dict(offset=offset,width=3,layout=layout))['hypotheses'][0]['target']['offset']==8192
   assert rpc.call('analysis.pointers',dict(offset=offset,width=3,layout=layout,context=dict(mpr=[None]*7+[0])))['hypotheses'][0]['target'] is None
  rpc.close()
  # Known-board identification tested with forged checksums on authored data only.
  for mapper,size,checksum in [('populous_sram',512*1024,0xdb5f97b3),('sf2',2560*1024,0x33deb700)]:
   image,_=fixture(size//8192);image=bytearray(image)
   for bank in range(1,size//8192):image[bank*8192]=bank&255
   path=out/f'{mapper}.pce';path.write_bytes(synthetic_crc(image,checksum));assert call('inspect',path)['mapper']==mapper
   rpc=Rpc(path);rpc.call('emulator.start');rpc.call('trace.configure',dict(enabled=True,capacity=100));rpc.call('emulator.registers.set',dict(registers=dict(mpr2=0x40)))
   if mapper=='populous_sram':
    rpc.call('emulator.memory.write',dict(space='cpu',address=0x4000,bytes=[123]));assert rpc.call('emulator.physical_memory',dict(space='sram',address=0,length=1))['bytes']==[123]
   else:
    assert rpc.call('emulator.memory',dict(address=0x4000,length=1))['bytes']==[64]
    rpc.call('emulator.memory.write',dict(space='bus',address=0x1ff2,bytes=[0]));assert rpc.call('emulator.state')['context']['sf2_bank']==3
    assert rpc.call('emulator.memory',dict(address=0x4000,length=1))['bytes']==[192]
   rpc.close()
  # Accepted-fetch evidence excludes the incremental bytes of a block transfer.
  block=out/'block.pce';image=bytearray(data);program=bytes.fromhex('a9 f8 53 02 a9 01 53 04 73 00 40 20 20 10 00 4c 0f e0');image[:len(program)]=program;image[8192:8208]=bytes(range(16));block.write_bytes(image)
  rpc=Rpc(block);rpc.call('emulator.start');rpc.call('trace.configure',dict(enabled=True,capacity=1000));rpc.call('emulator.step',dict(count=5,max_operations=100,timeout_ms=1000))
  assert rpc.call('emulator.physical_memory',dict(space='ram',address=0x20,length=16))['bytes']==list(range(16))
  coverage=rpc.call('analysis.coverage');assert sum(v['hits'] for v in coverage['items'])==5
  stream=out/'events.jsonl';rpc.call('trace.stream.start',dict(path=str(stream),max_bytes=100000));rpc.call('emulator.step',dict(count=20));rpc.call('trace.stream.stop');assert stream.stat().st_size>0
  rpc.close()
 with tempfile.TemporaryDirectory(prefix='romatlas-pce-workflow-') as temporary:
  work=pathlib.Path(temporary);rom=work/'test.pce';rom.write_bytes(fixture()[0])
  def script(name,*args,ok=True):
   result=subprocess.run(['python3',str(ROOT/'scripts'/name),*map(str,args)],capture_output=True,text=True,timeout=900 if name=="port.py" and args and args[0] in {"check","audit"} else 300)
   if ok:assert result.returncode==0,(name,result.stdout,result.stderr);return json.loads(result.stdout)
   assert result.returncode!=0;return None
  workspace=work/'workspace';script('disassemble.py',rom,'--workspace',workspace,'--json')
  meta=json.loads((workspace/'workspace.json').read_text());assert meta['platform']=='pce' and meta['rom']=='input/game.pce'
  assert 'context.mpr' in (workspace/'prompts/disassembly.txt').read_text()
  result=subprocess.run(['python3',str(workspace/'run.py'),'baseline'],capture_output=True,text=True);assert result.returncode==0,result.stderr
  script('commentary.py','prepare',workspace,'--json');script('commentary.py','audit',workspace,ok=False)
  script('commentary.py','verify',workspace);script('commentary.py','read',workspace)
  archive=work/'archive';script('archive.py','archive',workspace,'--output',archive,'--name','synthetic-pce')
  manifest=json.loads((archive/'archive.json').read_text());assert manifest['platform']=='pce' and not manifest['editorial_complete'];assert not list(archive.rglob('*.pce'))
  restored=work/'restored';script('archive.py','restore',archive,'--rom',rom,'--out',restored)
  port=work/'port';assert script('port.py','prepare',archive,'--rom',rom,'--out',port,'--json')['byte_identical_reference']
  script('port.py','check',port);script('port.py','headless',port,'--frames',2);script('port.py','audit',port,ok=False)
  frozen=(port/'reference/source/game.asm').read_bytes();script('port.py','reference-init',port)
  working=port/'research/working';src=working/'source/game.asm';src.write_text('; Clarify the authored synthetic reset.\n'+src.read_text())
  changes=json.loads((working/'changes.json').read_text());changes['changes']=[dict(id='pce-reset-comment',kind='commentary',offset=0,length=2,before='Original reset note',after='Clarified reset entry',port_impact='Document initialization',evidence=['Original synthetic fixture'])]
  (working/'changes.json').write_text(json.dumps(changes));script('port.py','reference-verify',port,'--revision','pce-reset');assert (port/'reference/source/game.asm').read_bytes()==frozen
 print('PASS PC Engine: synthetic debugger, identity, replay, traces, all 256 opcodes, exact rebuild and archive/native workflow')
if __name__=='__main__':main()
