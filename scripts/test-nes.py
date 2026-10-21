#!/usr/bin/env python3
"""Original synthetic NES CLI/RPC, bank identity, replay and rebuilding regression.
No commercial ROMs or game assets. Generated files live under ignored target/.
"""
import json, pathlib, subprocess, tempfile, shutil, hashlib, sys
ROOT=pathlib.Path(__file__).resolve().parents[1]
BIN=ROOT/'target/release/romatlas'
def call(*args,ok=True):
 p=subprocess.run([str(BIN),*map(str,args)],capture_output=True,text=True)
 if ok: assert p.returncode==0,(args,p.stderr)
 else: assert p.returncode==1,(args,p.stdout);return json.loads(p.stderr)
 return json.loads(p.stdout)
def fixture(mapper=0,banks=1,chr_banks=1):
 header=bytearray(b'NES\x1a'+bytes([banks,chr_banks,mapper<<4,0])+bytes(8))
 prg=bytearray([0xea])*(banks*16384)
 # Set pulse/noise, palette, rendering, controller and RAM loop.
 code=bytes.fromhex('78 d8 a2 ff 9a a9 0f 8d 15 40 a9 bf 8d 00 40 a9 08 8d 01 40 a9 fd 8d 02 40 a9 08 8d 03 40 a9 3f 8d 06 20 a9 00 8d 06 20 a9 21 8d 07 20 a9 08 8d 01 20 a9 01 8d 16 40 a9 00 8d 16 40 ad 16 40 85 01 e6 00 4c 32 80')
 prg[:len(code)]=code
 for v in (0x3ffa,0x3ffc,0x3ffe): prg[-16384+v:-16384+v+2 if -16384+v+2 else None]=bytes([0,0x80])
 # For banks>1 always put vectors at end of physical PRG.
 prg[-6:]=bytes([0,0x80])*3
 return bytes(header+prg+bytearray(chr_banks*8192)),len(code)
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

def main():
 parent=ROOT/'target/nes-tests';parent.mkdir(parents=True,exist_ok=True)
 with tempfile.TemporaryDirectory(dir=parent) as tmp:
  out=pathlib.Path(tmp);rom=out/'synthetic.nes';data,length=fixture();rom.write_bytes(data)
  info=call('inspect',rom);assert info['platform']=='nes' and info['mapper']==0 and info['sha256']==hashlib.sha256(data).hexdigest()
  call('run',rom,'--operations',20,'--timeout-ms',1000)
  mapping=out/'mapping.json';call('map','init',rom,'--output',mapping);call('map','validate',mapping,'--rom',rom)
  s=Rpc(rom)
  s.call('emulator.start');initial=s.call('emulator.snapshot.export');state=s.call('emulator.state')
  for space,address,n in [('rom',0,16),('ram',0,16),('oam',0,16),('palette',0,16),('cpu',0x8000,16),('ppu',0,16)]:s.call('emulator.memory',dict(space=space,address=address,length=n))
  s.call('emulator.memory',dict(address=0x2002),ok=False)
  assert s.call('emulator.snapshot.export')==initial,'inspection changed state'
  s.call('trace.configure',dict(enabled=True,capacity=1000));s.call('emulator.input',dict(buttons=['a']))
  saved=s.call('emulator.snapshot.export')
  def segment():
   s.call('trace.clear');s.call('emulator.audio.clear');v=s.call('emulator.step',dict(unit='frame',count=2,max_operations=100000,timeout_ms=10000));return v,s.call('emulator.snapshot.export'),s.call('emulator.framebuffer'),s.call('trace.query',dict(limit=1000))
  first=segment();assert s.call('emulator.physical_memory',dict(space='ram',address=1,length=1))['bytes'][0]&1==1
  assert first[0]['frames']>=2 and first[2]['width']==256 and first[2]['height']==240
  s.call('emulator.snapshot.import',dict(snapshot=saved));s.call('emulator.input',dict(buttons=['a']));second=segment();assert first==second,'snapshot replay mismatch'
  wav=out/'audio.wav';s.call('emulator.audio.export',dict(path=str(wav)));assert wav.read_bytes()[:4]==b'RIFF' and wav.stat().st_size>44 and any(wav.read_bytes()[44:])
  assert s.call('analysis.coverage')['total']>0 and s.call('analysis.edges')['total']>0
  s.call('trace.configure',dict(enabled=True,capacity=0));s.call('emulator.step',dict(count=20));assert s.call('trace.summary')['retained']==0 and s.call('analysis.contexts')['total']>0
  before=s.call('emulator.snapshot.export');bad=dict(before,rom_sha256='00'*32);s.call('emulator.snapshot.import',dict(snapshot=bad),ok=False);assert s.call('emulator.snapshot.export')==before
  region=dict(id=0,offset=16,length=length,kind='Code',provenance='Manual',label='Reset',comment='Original synthetic register and input loop',views=[dict(cpu='main',cpu_address=0x8000)])
  s.call('map.annotate',dict(region=region));s.call('map.save',dict(path=str(mapping)));before_map=s.call('map.get');s.call('map.annotate',dict(region=dict(region,offset=17)),ok=False);assert before_map==s.call('map.get')
  s.call('emulator.step',dict(count=1));s.call('emulator.registers.set',dict(registers=dict(pc=0x8000,a=7)))
  before_edit=s.call('emulator.state');s.call('emulator.registers.set',dict(registers=dict(pc=65536)),ok=False);assert s.call('emulator.state')==before_edit
  s.call('emulator.memory.write',dict(space='ram',address=2,bytes=[42]));assert s.call('emulator.memory',dict(address=2,length=1))['bytes']==[42]
  s.call('trace.clear');s.call('emulator.step',dict(count=1));assert s.call('emulator.state')['artificial'];assert s.call('analysis.coverage')['items'][0]['address']['context']['artificial']=='true'
  s.close()
  generated=out/'source';call('generate',rom,'--map',mapping,'--out',generated);assert 'lda #$0f' in (generated/'game.asm').read_text();assert call('verify',rom,'--generated',generated)['byte_identical']
  (generated/'game.o').write_bytes(b'stale');call('verify',rom,'--generated',generated)
  (generated/'game.asm').write_text((generated/'game.asm').read_text().replace('lda #$0f','lda #$0e',1));call('verify',rom,'--generated',generated,ok=False)
  # All 256 opcode encodings, deliberate absolute low operands and wrap branches.
  decoded=[];stream=bytearray()
  for op in range(256):
   sample=out/'opcode.nes';image=bytearray(data);image[16:19]=bytes([op,0x01,0x00]);sample.write_bytes(image)
   i=call('decode',sample,'--offset',16,'--count',1)['instructions'][0]['instruction'];stream+=bytes([op,1,0])[:i['length']];decoded.append(i)
  allrom=out/'all.nes';image=bytearray(data);image[16:16+len(stream)]=stream;allrom.write_bytes(image)
  allmap=out/'all.json';call('map','init',allrom,'--output',allmap);m=json.loads(allmap.read_text());m['regions']=[dict(region,length=len(stream),id=1)];m['next_id']=2;allmap.write_text(json.dumps(m))
  call('generate',allrom,'--map',allmap,'--out',out/'all-source');call('verify',allrom,'--generated',out/'all-source')
  # Mapper 2: genuine bank write from fixed window; repeated byte values cannot identify banks.
  banked=out/'banked.nes';b,_=fixture(2,4,0);b=bytearray(b);b[-6:]=bytes([0,0xc0])*3;b[16+3*16384:16+3*16384+8]=bytes.fromhex('a9 02 8d 00 80 4c 05 c0');banked.write_bytes(b)
  s=Rpc(banked);v=s.call('emulator.start');assert v['context']['prg_banks']==[0,1,6,7]
  s.call('trace.configure',dict(enabled=True));s.call('emulator.step',dict(count=2));v=s.call('emulator.state');assert v['context']['prg_banks']==[4,5,6,7]
  assert s.call('trace.summary')['counts']['BankSwitch']>0
  peek=s.call('emulator.memory',dict(address=0x8000,length=1));assert peek['bytes']==[b[16+2*16384]]
  s.call('map.annotate',dict(region=dict(region,offset=16+3*16384,length=8,views=[dict(cpu='main',cpu_address=0xc000,context=dict(prg_banks=[4,5,6,7]))])))
  discovered=s.call('analysis.discover',dict(entrypoints=[dict(offset=16+3*16384,cpu_address=0xc000,context=dict(prg_banks=[4,5,6,7]))],max_instructions=10));assert discovered['total']>=1
  s.close()
  # Baseline mapper construction and real MMC1/MMC3 register banking.
  for mapper,banks in [(1,4),(3,1),(4,4),(7,2)]:
   path=out/f'mapper-{mapper}.nes';image,_=fixture(mapper,banks,1);path.write_bytes(image);rpc=Rpc(path);rpc.call('emulator.start');rpc.close()
  for mapper,program,count,bank in [(1,bytes.fromhex('a9 00 8d 00 e0 a9 01 8d 00 e0 a9 00 8d 00 e0 a9 00 8d 00 e0 a9 00 8d 00 e0'),10,4),(4,bytes.fromhex('a9 06 8d 00 80 a9 05 8d 01 80'),4,5)]:
   image,_=fixture(mapper,4,0);image=bytearray(image);image[-6:]=bytes([0,0xc0])*3 if mapper==1 else bytes([0,0xe0])*3
   location=16+3*16384+(0 if mapper==1 else 8192);image[location:location+len(program)]=program
   path=out/f'bank-write-{mapper}.nes';path.write_bytes(image);rpc=Rpc(path);rpc.call('emulator.start');rpc.call('trace.configure',dict(enabled=True,capacity=0));v=rpc.call('emulator.step',dict(count=count));assert v['context']['prg_banks'][0]==bank,(mapper,v);rpc.close()
  # Large physical layout, unsupported mapper reconstruction, trainer/trailer retention.
  for name,image in [('large',fixture(2,64,0)[0]),('trainer',data[:6]+bytes([data[6]|4])+data[7:16]+bytes(512)+data[16:]+b'local-trailer')]:
   path=out/f'{name}.nes';path.write_bytes(image);mapping=out/f'{name}.json';call('map','init',path,'--output',mapping);call('generate',path,'--map',mapping,'--out',out/f'{name}-source');call('verify',path,'--generated',out/f'{name}-source')
 # Exercise the same generated workspace and native-port lifecycle as GB/SNES.
 with tempfile.TemporaryDirectory(prefix='romatlas-nes-workflow-') as temporary:
  work=pathlib.Path(temporary);rom=work/'synthetic.nes';rom.write_bytes(fixture()[0])
  def script(name,*args,ok=True):
   result=subprocess.run(['python3',str(ROOT/'scripts'/name),*map(str,args)],capture_output=True,text=True,timeout=900 if name=="port.py" and args and args[0] in {"check","audit"} else 300)
   if ok:assert result.returncode==0,(name,result.stdout,result.stderr);return json.loads(result.stdout)
   assert result.returncode!=0;return None
  workspace=work/'workspace';script('disassemble.py',rom,'--workspace',workspace,'--json')
  meta=json.loads((workspace/'workspace.json').read_text());assert meta['platform']=='nes' and meta['rom']=='input/game.nes'
  prompt=(workspace/'prompts/disassembly.txt').read_text();assert 'prg_banks' in prompt and 'whole-ROM' in prompt and '65816' not in prompt
  result=subprocess.run(['python3',str(workspace/'run.py'),'baseline'],capture_output=True,text=True);assert result.returncode==0,result.stderr
  script('commentary.py','prepare',workspace,'--json');script('commentary.py','audit',workspace,ok=False)
  source=workspace/'commentary/source';game=(source/'game.asm').read_text();(source/'game.asm').write_text('.include "constants.inc"\n'+game);(source/'constants.inc').write_text('native_constant = $1234 ; original synthetic annotation\n')
  script('commentary.py','verify',workspace);script('commentary.py','read',workspace);assert 'native_constant' in (workspace/'commentary/reading-after.asm').read_text()
  archive=work/'archive';script('archive.py','archive',workspace,'--output',archive,'--name','synthetic-nes')
  manifest=json.loads((archive/'archive.json').read_text());assert manifest['platform']=='nes' and not manifest['editorial_complete'];assert not list(archive.rglob('*.nes'))
  restored=work/'restored';script('archive.py','restore',archive,'--rom',rom,'--out',restored)
  assert (restored/'source/game.asm').read_bytes()==(source/'game.asm').read_bytes()
  port=work/'port';assert script('port.py','prepare',archive,'--rom',rom,'--out',port,'--json')['byte_identical_reference']
  script('port.py','check',port);script('port.py','headless',port,'--frames',2);script('port.py','audit',port,ok=False)
 print('PASS NES: synthetic inspect/RPC/replay/trace/bank identity/all-opcode exact reconstruction')
if __name__=='__main__':main()
