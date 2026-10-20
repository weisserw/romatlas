#!/usr/bin/env python3
"""SMS synthetic debugger, complete opcode families, reconstruction and agent lifecycle.
No commercial images, devices or extracted assets. Products remain local.
"""
import json, pathlib, subprocess, tempfile, hashlib, sys
from importlib.util import spec_from_file_location, module_from_spec
ROOT=pathlib.Path(__file__).resolve().parents[1]
BIN=ROOT/'target/release/romatlas'
PLATFORM='sms'
SCREEN=(256,192)
# Reuse transport only, never the NES platform implementation.
spec=spec_from_file_location('nes_test_transport',ROOT/'scripts/test-nes.py')
transport=module_from_spec(spec);spec.loader.exec_module(transport)
call=transport.call;Rpc=transport.Rpc

def fixture(banks=4,prefix=False,codemasters=False):
 data=bytearray(banks*16384)
 # PSG, enable display, read input and update RAM. All code stays in fixed 1 KiB.
 code=bytes.fromhex('f3 31 f0 df 3e 85 d3 7f 3e 10 d3 7f 3e 90 d3 7f 3e 40 d3 bf 3e 81 d3 bf db dc 32 01 c0 3a 00 c0 3c 32 00 c0 c3 18 00')
 data[:len(code)]=code
 data[0x38]=0xc9;data[0x66]=0xed;data[0x67]=0x45
 data[0x7ff0:0x7ff8]=b'TMR SEGA';data[0x7fff]=0x7e if PLATFORM=='gg' else 0x4e
 if codemasters:
  data[0x7fe6:0x7fe8]=b'\0\0'
  checksum=sum(int.from_bytes(data[i:i+2],'little') for i in range(0,len(data),2) if not 0x7ff0<=i<0x8000)&65535
  # checksum includes its own word: other words must sum to zero.
  data[0x7fe4:0x7fe6]=((-checksum)&65535).to_bytes(2,'little')
 return (bytes(512)+data if prefix else bytes(data)),len(code)

def script(name,*args,ok=True):
 p=subprocess.run(['python3',str(ROOT/'scripts'/name),*map(str,args)],capture_output=True,text=True,timeout=300)
 if ok:assert p.returncode==0,(name,p.stdout,p.stderr);return json.loads(p.stdout)
 assert p.returncode!=0,(name,p.stdout)

def main():
 parent=ROOT/'target/sms-tests';parent.mkdir(parents=True,exist_ok=True)
 with tempfile.TemporaryDirectory(dir=parent) as tmp:
  out=pathlib.Path(tmp);rom=out/f'synthetic.{PLATFORM}';data,length=fixture();rom.write_bytes(data)
  info=call('inspect',rom);assert info['platform']==PLATFORM and info['mapper']=='sega' and info['sha256']==hashlib.sha256(data).hexdigest()
  call('run',rom,'--operations',20,'--timeout-ms',1000)
  mapping=out/'mapping.json';call('map','init',rom,'--output',mapping);call('map','validate',mapping,'--rom',rom)
  s=Rpc(rom);v=s.call('emulator.start');assert v['context']['rom_banks']==[0,1,2]
  if PLATFORM=='gg':
   assert s.call('api.describe')['platform']=='gg'
   caps=s.call('emulator.capabilities');assert any(v['name']=='cram' and v['size']==64 for v in caps['address_spaces'])
   s.call('emulator.input',dict(buttons=['pause']),ok=False)
   s.call('emulator.input',dict(port=2,buttons=[]),ok=False)
   s.call('emulator.input',dict(buttons=['start']))
   s.call('emulator.step',dict(unit='frame',count=1,max_operations=100000,timeout_ms=10000))
   assert s.call('emulator.memory',dict(space='io',address=0,length=1))['bytes'][0]&128==0
   s.call('emulator.input',dict(buttons=[]))
   s.call('emulator.reset')
   s.call('emulator.memory.write',dict(space='io',address=6,bytes=[0x10]))
  initial=s.call('emulator.snapshot.export')
  for space,address,n in [('rom',0,16),('ram',0,16),('vram',0,16),('cram',0,16),('cart_ram',0,16),('cpu',0,16),('cpu',0xe000,16),('io',0xbf,1)]:s.call('emulator.memory',dict(space=space,address=address,length=n))
  assert s.call('emulator.snapshot.export')==initial,'peek mutated core'
  s.call('trace.configure',dict(enabled=True,capacity=1000));s.call('emulator.input',dict(buttons=['button1']))
  saved=s.call('emulator.snapshot.export')
  def segment():
   s.call('trace.clear');s.call('emulator.audio.clear');v=s.call('emulator.step',dict(unit='frame',count=2,max_operations=100000,timeout_ms=10000));return v,s.call('emulator.snapshot.export'),s.call('emulator.framebuffer'),s.call('trace.query',dict(limit=1000))
  first=segment();assert first[0]['frames']>=2 and (first[2]['width'],first[2]['height'])==SCREEN
  assert s.call('emulator.memory',dict(address=0xc001,length=1))['bytes'][0]&16==0
  s.call('emulator.snapshot.import',dict(snapshot=saved));assert first==segment(),'snapshot replay mismatch'
  wav=out/'audio.wav';s.call('emulator.audio.export',dict(path=str(wav)));assert wav.read_bytes()[:4]==b'RIFF' and any(wav.read_bytes()[44:])
  if PLATFORM=='gg':
   import struct
   stereo=list(struct.iter_unpack('<hh',wav.read_bytes()[44:]))
   assert any(left for left,right in stereo) and all(right==0 for left,right in stereo)
  assert s.call('analysis.coverage')['total']>0 and s.call('analysis.edges')['total']>0
  assert any(e['context'].get('origin')=='io' for e in first[3]['events'])
  s.call('trace.configure',dict(enabled=True,capacity=0));s.call('emulator.step',dict(count=20));assert s.call('trace.summary')['retained']==0 and s.call('analysis.contexts')['total']>0
  before=s.call('emulator.snapshot.export');s.call('emulator.snapshot.import',dict(snapshot=dict(before,rom_sha256='00'*32)),ok=False);assert s.call('emulator.snapshot.export')==before
  region=dict(id=0,offset=0,length=length,kind='Code',provenance='Manual',label='Reset',comment='Synthetic input and PSG loop; preserve image.sms and ROMAtlas SMS in commentary',views=[dict(cpu='main',cpu_address=0)])
  s.call('map.annotate',dict(region=region));s.call('map.save',dict(path=str(mapping)));old=s.call('map.get');s.call('map.annotate',dict(region=dict(region,offset=1)),ok=False);assert s.call('map.get')==old
  s.call('emulator.registers.set',dict(registers=dict(pc=0,af=0x1234,ix=0x5678)));assert s.call('emulator.state')['main']['af']==0x1234
  old=s.call('emulator.state');s.call('emulator.registers.set',dict(registers=dict(pc=65536)),ok=False);assert s.call('emulator.state')==old
  s.call('emulator.memory.write',dict(space='cpu',address=0xe002,bytes=[42]));assert s.call('emulator.memory',dict(address=0xc002,length=1))['bytes']==[42]
  s.call('emulator.memory.write',dict(space='vram',address=0x123,bytes=[99]));assert s.call('emulator.physical_memory',dict(space='vram',address=0x123,length=1))['bytes']==[99]
  s.call('emulator.memory.write',dict(space='io',address=0xbf,bytes=[0x23]));s.call('emulator.memory.write',dict(space='io',address=0xbf,bytes=[0x41]));s.call('emulator.memory.write',dict(space='io',address=0xbe,bytes=[0x55]));assert s.call('emulator.physical_memory',dict(space='vram',address=0x123,length=1))['bytes']==[0x55]
  assert s.call('emulator.translate',dict(address=0xe123))['physical']['offset']==0x123
  s.close()
  generated=out/'source';call('generate',rom,'--map',mapping,'--out',generated);assert 'ld sp, $dff0' in (generated/'game.asm').read_text();assert 'preserve image.sms and ROMAtlas SMS in commentary' in (generated/'game.asm').read_text();assert call('verify',rom,'--generated',generated)['byte_identical']
  (generated/'game.o').write_bytes(b'stale');call('verify',rom,'--generated',generated)
  (generated/'game.asm').write_text((generated/'game.asm').read_text().replace('ld sp, $dff0','ld sp, $dff1'));call('verify',rom,'--generated',generated,ok=False)
  # Every base, CB, ED, DD, FD and indexed-CB encoding, including signed displacements.
  stream=bytearray();samples=[]
  for op in range(256):
   samples.append(bytes([op,0,0,0,0,0]))
   for prefix in [0xcb,0xed,0xdd,0xfd]:samples.append(bytes([prefix,op,0,0,0,0]))
   for prefix in [0xdd,0xfd]:samples.append(bytes([prefix,0xcb,0xfe,op,0,0]))
  sample=out/f'opcode.{PLATFORM}';sample.write_bytes(data);rpc=Rpc(sample)
  # Batch all disassembly through one session by placing samples in fixed 8-byte records.
  records=bytearray(data)
  for n,b in enumerate(samples):records[n*8:n*8+len(b)]=b
  sample.write_bytes(records);rpc.close();rpc=Rpc(sample)
  for n,b in enumerate(samples):
   decoded=rpc.call('analysis.decode',dict(offset=n*8,count=1,cpu_address=0))
   assert not decoded['diagnostics'],(b,decoded)
   i=decoded['instructions'][0]['instruction'];stream+=b[:i['length']]
  rpc.close()
  allrom=out/f'all.{PLATFORM}';image=bytearray(data);image[:len(stream)]=stream;allrom.write_bytes(image)
  m=json.loads(mapping.read_text());m['image_sha256']=hashlib.sha256(image).hexdigest();m['regions']=[dict(region,id=1,length=len(stream),views=[dict(cpu='main',cpu_address=0,context=dict(rom_banks=[0,1,2]))])];m['next_id']=2
  allmap=out/'all.json';allmap.write_text(json.dumps(m));call('generate',allrom,'--map',allmap,'--out',out/'all-source');call('verify',allrom,'--generated',out/'all-source')
  # Actual mapper writes, fixed first KiB, CPU aliases, SRAM and bank events.
  banked=out/f'banked.{PLATFORM}';image=bytearray(data);image[:8]=bytes.fromhex('3e 03 32 ff ff c3 05 00');banked.write_bytes(image)
  rpc=Rpc(banked);rpc.call('emulator.start');rpc.call('trace.configure',dict(enabled=True));v=rpc.call('emulator.step',dict(count=2));assert v['context']['rom_banks']==[0,1,3];assert rpc.call('trace.summary')['counts']['BankSwitch']>0
  d=rpc.call('analysis.discover',dict(entrypoints=[dict(offset=0,cpu_address=0)],max_instructions=10,use_observed=False));assert d['total']>=1
  rpc.close()
  for name,program,count in [
   ('sram','3e 08 32 fc ff 3e 5a 32 00 80 3e 0c 32 fc ff 3e a5 32 00 80',8),
   ('video','3e 23 d3 bf 3e 41 d3 bf 3e 55 d3 be',6),
   ('disabled-cartridge','3e 48 d3 3e 00',3),
   ('codemasters-write','3e 03 32 00 80',2)]:
   image=bytearray(fixture(codemasters=name=='codemasters-write')[0]);program=bytes.fromhex(program);image[:len(program)]=program
   if name=='sram':image[0x100:0x102]=bytes([0,0x80])
   if name=='codemasters-write':
    image[0x7fe4:0x7fe8]=bytes(4);checksum=sum(int.from_bytes(image[i:i+2],'little') for i in range(0,len(image),2) if not 0x7ff0<=i<0x8000)&65535;image[0x7fe4:0x7fe6]=((-checksum)&65535).to_bytes(2,'little')
   path=out/f'{name}.{PLATFORM}';path.write_bytes(image);rpc=Rpc(path);rpc.call('emulator.start');rpc.call('trace.configure',dict(enabled=True));state=rpc.call('emulator.step',dict(count=count))
   if name=='sram':
    assert state['context']['ram_bank']==1
    pointer=rpc.call('analysis.pointers',dict(offset=0x100,count=1,context=state['context']))['hypotheses'][0]
    assert pointer['target']['space']=='cart_ram' and pointer['target']['offset']==16384
    assert rpc.call('emulator.physical_memory',dict(space='cart_ram',address=0,length=1))['bytes']==[0x5a]
    assert rpc.call('emulator.physical_memory',dict(space='cart_ram',address=16384,length=1))['bytes']==[0xa5]
   elif name=='disabled-cartridge':
    assert state['context']['cartridge_enabled'] is (PLATFORM=='gg')
    expected_space='rom' if PLATFORM=='gg' else 'cpu'
    assert any(row['address']['space']==expected_space and row['address']['offset']==4 for row in rpc.call('analysis.coverage')['items'])
   elif name=='video':assert any(e['kind']=='Write' and e['address']['space']=='vram' and e['address']['offset']==0x123 and e['value']==0x55 for e in rpc.call('trace.query')['events'])
   else:assert state['context']['rom_banks'][2]==3
   rpc.close()
  for name,image in [('prefix',fixture(prefix=True)[0]),('codemasters',fixture(codemasters=True)[0]),('large',fixture(banks=64)[0])]:
   path=out/f'{name}.{PLATFORM}';path.write_bytes(image);m=out/f'{name}.json';call('map','init',path,'--output',m);call('generate',path,'--map',m,'--out',out/f'{name}-source');call('verify',path,'--generated',out/f'{name}-source')
   if name=='codemasters':assert call('inspect',path)['mapper']=='codemasters'
  if PLATFORM=='gg':
   # Actual CRAM writes: low byte is latched until the high byte commits both bytes.
   image=bytearray(data);program=bytes.fromhex('3e 3e d3 bf 3e c0 d3 bf 3e 2f d3 be 3e 0a d3 be')
   image[:len(program)]=program;path=out/'palette.gg';path.write_bytes(image)
   rpc=Rpc(path);rpc.call('emulator.start');rpc.call('trace.configure',dict(enabled=True))
   rpc.call('emulator.step',dict(count=6))
   assert rpc.call('emulator.physical_memory',dict(space='cram',address=62,length=2))['bytes']==[0,0]
   latch=rpc.call('emulator.snapshot.export');rpc.call('emulator.step',dict(count=2))
   assert rpc.call('emulator.physical_memory',dict(space='cram',address=62,length=2))['bytes']==[0x2f,0x0a]
   events=rpc.call('trace.query')['events']
   assert {(e['address']['offset'],e['value']) for e in events if e['kind']=='Write' and e['address']['space']=='cram'}=={(62,0x2f),(63,0x0a)}
   rpc.call('emulator.snapshot.import',dict(snapshot=latch));rpc.call('emulator.step',dict(count=2))
   assert rpc.call('emulator.physical_memory',dict(space='cram',address=62,length=2))['bytes']==[0x2f,0x0a]
   rpc.close()
   # Explicit platform selects headerless/foreign-extension input for every workflow command.
   headerless=bytearray(data);headerless[0x7ff0:0x8000]=bytes(16);path=out/'headerless.rom';path.write_bytes(headerless)
   assert call('--platform','gg','inspect',path)['platform']=='gg'
   m=out/'headerless.json';call('--platform','gg','map','init',path,'--output',m)
   call('--platform','gg','map','validate',m,'--rom',path)
   call('--platform','gg','generate',path,'--map',m,'--out',out/'headerless-source')
   call('--platform','gg','verify',path,'--generated',out/'headerless-source')
   wrong=json.loads(m.read_text());wrong['format']='romatlas-sms-map';m.write_text(json.dumps(wrong))
   call('--platform','gg','generate',path,'--map',m,'--out',out/'wrong-source',ok=False)
   assert not (out/'wrong-source').exists()
  # More than 256 separate annotations must not consume WLA memory slots.
  many=out/'many.json';m=json.loads(mapping.read_text());m['regions']=[dict(region,id=n+1,offset=256+n,length=1,label=f'Part_{n}',views=[dict(cpu='main',cpu_address=256+n)]) for n in range(300)];m['next_id']=301;many.write_text(json.dumps(m))
  call('generate',rom,'--map',many,'--out',out/'many-source');call('verify',rom,'--generated',out/'many-source')
 with tempfile.TemporaryDirectory(prefix='romatlas-sms-workflow-') as tmp:
  work=pathlib.Path(tmp);rom=work/f'synthetic.{PLATFORM}';rom.write_bytes(fixture()[0]);workspace=work/'workspace'
  script('disassemble.py',rom,'--workspace',workspace,'--json');meta=json.loads((workspace/'workspace.json').read_text());assert meta['platform']==PLATFORM
  prompt=(workspace/'prompts/disassembly.txt').read_text();assert 'rom_banks' in prompt and 'whole-ROM' in prompt
  p=subprocess.run(['python3',str(workspace/'run.py'),'baseline'],capture_output=True,text=True);assert p.returncode==0,(p.stdout,p.stderr)
  script('commentary.py','prepare',workspace,'--json');script('commentary.py','audit',workspace,ok=False)
  script('commentary.py','verify',workspace);script('commentary.py','read',workspace)
  archive=work/'archive';script('archive.py','archive',workspace,'--output',archive,'--name','synthetic-sms');assert not list(archive.rglob(f'*.{PLATFORM}'))
  restored=work/'restored';script('archive.py','restore',archive,'--rom',rom,'--out',restored)
  port=work/'port';assert script('port.py','prepare',archive,'--rom',rom,'--out',port,'--json')['byte_identical_reference']
  frozen=(port/'reference/source/game.asm').read_bytes();script('port.py','reference-init',port)
  working=port/'research/working';source=working/'source/game.asm';source.write_text('; Clarify the synthetic SMS reset entry.\n'+source.read_text())
  changes=json.loads((working/'changes.json').read_text());changes['changes']=[dict(id='sms-reset-comment',kind='commentary',offset=0,length=1,before='Original reset note',after='Clarified SMS reset entry',port_impact='Document native initialization',evidence=['Original synthetic reset fixture'])];(working/'changes.json').write_text(json.dumps(changes))
  script('port.py','reference-verify',port,'--revision','sms-reset');revision=port/'research/revisions/sms-reset'
  assert (revision/'mapping.json').is_file() and json.loads((revision/'revision.json').read_text())['source_verification']['byte_identical'];assert (port/'reference/source/game.asm').read_bytes()==frozen
  script('port.py','check',port);script('port.py','headless',port,'--frames',2);script('port.py','audit',port,ok=False)
 print(f'PASS {PLATFORM}: synthetic debugger/replay/bank identity/opcode reconstruction/workspace/archive/native port')
if __name__=='__main__':main()
