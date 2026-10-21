#!/usr/bin/env python3
"""Synthetic Genesis debugger, dual-CPU source rebuild and agent lifecycle gate."""
import hashlib,json,pathlib,subprocess,tempfile
from importlib.util import spec_from_file_location,module_from_spec
ROOT=pathlib.Path(__file__).resolve().parents[1]
BIN=ROOT/'target/release/romatlas'
spec=spec_from_file_location('test_transport',ROOT/'scripts/test-nes.py')
transport=module_from_spec(spec);spec.loader.exec_module(transport)
call=transport.call;Rpc=transport.Rpc

def tool(name):
 local=ROOT/f'tools/m68k/bin/m68k-elf-{name}'
 return str(local) if local.is_file() else 'm68k-linux-gnu-'+name

def assemble(source,root,pc=512):
 (root/'fixture.s').write_text(source)
 subprocess.run([tool('as'),'-m68000',str(root/'fixture.s'),'-o',str(root/'fixture.o')],check=True,capture_output=True)
 subprocess.run([tool('ld'),'-Ttext',hex(pc),str(root/'fixture.o'),'-o',str(root/'fixture.elf')],check=True,capture_output=True)
 subprocess.run([tool('objcopy'),'-O','binary','-j','.text',str(root/'fixture.elf'),str(root/'fixture.bin')],check=True,capture_output=True)
 return (root/'fixture.bin').read_bytes()

def fixture(root):
 code=assemble('''
 move.w #0x2700,%sr
 move.w #0x8104,0xc00004
 move.w #0x8c81,0xc00004
 move.w #0x8f02,0xc00004
 move.w #0xc000,0xc00004
 move.w #0,0xc00004
 move.w #0x0e00,0xc00000
 move.w #0x8144,0xc00004
 move.w #0x0100,0xa11100
 move.w #0x0100,0xa11200
 lea 0x400,%a0
 lea 0xa00000,%a1
 moveq #15,%d0
copy: move.b (%a0)+,(%a1)+
 dbra %d0,copy
 move.b #0x85,0xc00011
 move.b #0x10,0xc00011
 move.b #0x90,0xc00011
 move.w #0,0xa11100
loop: addq.b #1,0xff0000
 bra.s loop
''',root)
 b=bytearray(32768);b[:8]=bytes.fromhex('00ffff00 00000200');b[0x100:0x110]=b'SEGA GENESIS    ';b[0x120:0x13a]=b'ROMATLAS SYNTHETIC GENESIS '[:26];b[0x1a4:0x1a8]=(len(b)-1).to_bytes(4,'big');b[0x1f0]=ord('U');b[512:512+len(code)]=code
 sound=bytes.fromhex('3e 90 32 11 7f 3a 00 80 32 00 10 3c 32 01 10 c3 05 00')
 # Copy the first 16 bytes, using a self-contained loop in the upload fixture.
 sound=bytes.fromhex('3e 90 32 11 7f 3a 00 80 32 00 10 3c c3 05 00 00')
 b[0x400:0x410]=sound;b[0x300:0x306]=bytes.fromhex('00000200 fe00');b[0x18e:0x190]=(sum(int.from_bytes(b[i:i+2],'big') for i in range(512,len(b),2))&65535).to_bytes(2,'big')
 return b,len(code),sound

def script(name,*args,ok=True):
 p=subprocess.run(['python3',str(ROOT/'scripts'/name),*map(str,args)],capture_output=True,text=True,timeout=900 if name=="port.py" and args and args[0] in {"check","audit"} else 300)
 if (p.returncode==0)!=ok and name=="port.py" and len(args)>1:
  log=pathlib.Path(args[1])/"checks/latest.log"
  if log.is_file():print(log.read_text(),flush=True)
 assert (p.returncode==0)==ok,(name,p.stdout,p.stderr)
 return json.loads(p.stdout) if ok and p.stdout.strip() else None

def region(offset,length,**kwargs):
 return dict(id=0,offset=offset,length=length,kind='Code',provenance='Manual',label='Reset',comment='Original synthetic Genesis setup.',views=[dict(cpu='main',cpu_address=offset)],**kwargs)

def main():
 with tempfile.TemporaryDirectory(prefix='romatlas-genesis-') as temp:
  root=pathlib.Path(temp);data,length,sound=fixture(root);rom=root/'synthetic.md';rom.write_bytes(data)
  info=call('inspect',rom);assert info['platform']=='genesis' and info['checksum_valid'] and info['sha256']==hashlib.sha256(data).hexdigest()
  call('run',rom,'--operations',20,'--timeout-ms',1000)
  s=Rpc(rom);started=s.call('emulator.start');assert started['video']['registers'] and len(started['audio']['ym2612']['last_writes'])==512;s.call('trace.configure',dict(enabled=True,capacity=1000));s.call('emulator.input',dict(buttons=['a','start','x','mode']));s.call('emulator.step',dict(count=1000,timeout_ms=10000));assert s.call('emulator.state')['z80']['pc']>0
  snap=s.call('emulator.snapshot.export')
  def segment():
   s.call('trace.clear');s.call('emulator.audio.clear');state=s.call('emulator.step',dict(unit='frame',count=2,max_operations=1000000,timeout_ms=10000));return state,s.call('emulator.snapshot.export'),s.call('emulator.framebuffer'),s.call('trace.query',dict(limit=1000))
  first=segment();assert first[2]['width']==320 and first[2]['height']==224
  s.call('emulator.snapshot.import',dict(snapshot=snap));assert first==segment(),'deterministic replay'
  wav=root/'capture.wav';s.call('emulator.audio.export',dict(path=str(wav)));assert any(wav.read_bytes()[44:])
  assert s.call('analysis.coverage')['total']>0 and s.call('analysis.contexts')['total']>0
  assert any(e['context'].get('cpu')=='z80' for e in s.call('trace.query',dict(limit=1000))['events'])
  s.call('trace.configure',dict(capacity=0));s.call('emulator.step',dict(cpu='z80',count=5,max_operations=10000,timeout_ms=1000));assert s.call('trace.summary')['retained']==0
  before=s.call('emulator.snapshot.export');s.call('emulator.snapshot.import',dict(snapshot=dict(before,rom_sha256='0'*64)),ok=False);assert before==s.call('emulator.snapshot.export')
  s.call('emulator.memory.write',dict(space='cpu',address=0xe00005,bytes=[42]));assert s.call('emulator.memory',dict(address=0xff0005,length=1))['bytes']==[42]
  assert s.call('emulator.translate',dict(address=0xff0005))['physical']['offset']==5
  before=s.call('emulator.snapshot.export');s.call('emulator.memory',dict(space='io',address=0xc00004,length=2));assert s.call('emulator.snapshot.export')==before
  s.call('emulator.registers.set',dict(registers=dict(d3=123)));assert s.call('emulator.state')['main']['data'][3]==123
  s.call('emulator.registers.set',dict(cpu='z80',registers=dict(ix=0x1234)));assert s.call('emulator.state')['z80']['registers'][4]==0x1234
  pointer=s.call('analysis.pointers',dict(offset=0x300,count=1,width=4))['hypotheses'][0];assert pointer['target']['offset']==512 and not pointer['code_proven']
  pointer=s.call('analysis.pointers',dict(offset=0x304,count=1,width=2,base=0x400))['hypotheses'][0];assert pointer['target']['offset']==512
  discovery=s.call('analysis.discover',dict(entrypoints=[dict(offset=512)],max_instructions=10,timeout_ms=10000,use_observed=False));assert discovery['total']>0
  entry=region(512,length);s.call('map.annotate',dict(region=entry));old=s.call('map.get');s.call('map.annotate',dict(region=dict(entry,offset=514)),ok=False);assert s.call('map.get')==old
  # Static Z80 ROM view requires explicit bank context; a RAM trace alone is insufficient.
  z=dict(entry,offset=0x400,length=16,label='SoundDriver',views=[dict(cpu='z80',cpu_address=0x8400,context=dict(z80_bank=0))]);s.call('map.annotate',dict(region=z))
  mapping=root/'mapping.json';s.call('map.export',dict(path=str(mapping)));s.close()
  source=root/'source';call('generate',rom,'--map',mapping,'--out',source);assert 'move' in (source/'game.asm').read_text();assert 'ld ' in (source/'z80-2.asm').read_text();
  import commentary
  assert 'BEGIN z80-2.asm' in commentary.reading(source)
  call('verify',rom,'--generated',source)
  (source/'game.o').write_bytes(b'stale object');call('verify',rom,'--generated',source)
  text=(source/'game.asm').read_text();(source/'game.asm').write_text(text.replace('moveq #15','moveq #14'));call('verify',rom,'--generated',source,ok=False);(source/'game.asm').write_text(text)
  for layout in ['prefix','swapped','smd','ssf']:
   encoded=bytearray(data);prefix=0
   if layout=='prefix':encoded=bytes(512)+encoded;prefix=512
   if layout=='swapped':encoded=bytes(v for pair in zip(data[1::2],data[::2]) for v in pair)
   if layout=='smd':
    encoded=bytearray(512);encoded[8:11]=bytes([0xaa,0xbb,6]);prefix=512
    for block in [data[:16384],data[16384:]]:encoded.extend(block[1::2]);encoded.extend(block[::2])
   if layout=='ssf':encoded[0x100:0x110]=b'SEGA SSF        ';encoded.extend(bytes(0x500000-len(encoded)))
   path=root/f'{layout}.md';path.write_bytes(encoded);m=root/f'{layout}.json';call('map','init',path,'--output',m);meta=json.loads(m.read_text());e=dict(entry,id=1,offset=512+prefix,views=[dict(cpu='main',cpu_address=512,context=dict(rom_banks=list(range(8))))]);meta.update(regions=[e],next_id=2);m.write_text(json.dumps(meta));generated=root/f'{layout}-source';call('generate',path,'--map',m,'--out',generated);call('verify',path,'--generated',generated)
  # Every base 68000 instruction family plus effective-address encodings is
  # assembled independently before adapter disassembly and exact reconstruction.
  families = """
  .word 0xa000,0xf000
  abcd %d0,%d1
  add.b %d0,%d1
  adda.l %a0,%a1
  addi.w #0x1234,%d0
  addq.l #7,%a0
  addx.w %d0,%d1
  and.l %d0,%d1
  andi.b #0x55,%d0
  asl.w #3,%d0
  asr.b %d1,%d0
  bchg #5,(%a0)
  bclr %d0,(%a1)
  bset #3,(%a2)
  btst %d1,(%a3)
  chk.w %d0,%d1
  clr.l (%a0)
  cmp.b %d0,%d1
  cmpa.w %a0,%a1
  cmpi.l #0x12345678,%d0
  cmpm.w (%a0)+,(%a1)+
  dbra %d0,.
  divs.w %d0,%d1
  divu.w (%a0),%d0
  eor.b %d0,%d1
  eori.w #0x1234,%d0
  exg %d0,%a0
  ext.l %d0
  illegal
  jmp 0x123456
  jsr (%a0)
  lea 8(%a0),%a1
  link %a0,#-16
  lsl.l #1,%d0
  lsr.w (%a0)
  move.b (%a0)+,-(%a1)
  move.w #0x1234,0xff0000
  move.l 12(%a0,%d0.w),%d1
  movea.l %d0,%a0
  movem.l %d0-%d3/%a0-%a3,-(%sp)
  movem.l (%sp)+,%d0-%d3/%a0-%a3
  movep.w %d0,4(%a0)
  moveq #-1,%d0
  muls.w %d0,%d1
  mulu.w #17,%d0
  nbcd (%a0)
  neg.l %d0
  negx.w %d1
  nop
  not.b %d0
  or.w %d0,%d1
  ori.b #0x11,%d0
  pea 0x123456
  reset
  rol.l %d0,%d1
  ror.w #2,%d0
  roxl.b #1,%d0
  roxr.w (%a0)
  rte
  rtr
  rts
  sbcd %d0,%d1
  seq (%a0)
  stop #0x2700
  sub.l %d0,%d1
  suba.w %a0,%a1
  subi.b #5,%d0
  subq.w #1,%d0
  subx.l -(%a0),-(%a1)
  swap %d0
  tas (%a0)
  trap #3
  trapv
  tst.l %d0
  unlk %a0
  """
  stream=assemble(families,root);sample=bytearray(data);sample[512:512+len(stream)]=stream;path=root/'families.md';path.write_bytes(sample);m=root/'families.json';call('map','init',path,'--output',m);meta=json.loads(m.read_text());meta.update(regions=[dict(region(512,len(stream)),id=1)],next_id=2);m.write_text(json.dumps(meta));call('generate',path,'--map',m,'--out',root/'families-source');call('verify',path,'--generated',root/'families-source')
  pal=bytearray(data);pal[0x100:0x110]=b'SEGA MEGA DRIVE ';pal[0x1f0]=ord('E');pal_path=root/'pal.md';pal_path.write_bytes(pal);pal_rpc=Rpc(pal_path);state=pal_rpc.call('emulator.start');assert state['video']['timing_mode']=='Pal' and 49<state['video']['nominal_fps']<51
  state=pal_rpc.call('emulator.step',dict(unit='frame',count=1,max_operations=1000000,timeout_ms=10000));cycles=state['cpu_cycles'];state=pal_rpc.call('emulator.step',dict(unit='frame',count=2,max_operations=1000000,timeout_ms=10000));assert state['cpu_cycles']-cycles>2100000;pal_rpc.close()
  # Actual executed mapper writes, not inferred bank identity from byte values.
  bank_code=assemble('move.b #8,0xa130f3\nbra.s .\n',root)
  banked=bytearray(0x500000);banked[:len(data)]=data;banked[0x100:0x110]=b'SEGA SSF        ';banked[512:512+len(bank_code)]=bank_code;path=root/'banked.md';path.write_bytes(banked);s=Rpc(path);s.call('emulator.start');s.call('trace.configure',dict(enabled=True));v=s.call('emulator.step',dict(count=1));assert v['context']['rom_banks'][1]==8;assert s.call('emulator.translate',dict(address=0x80000))['physical']['offset']==0x400000;assert s.call('trace.summary')['counts']['BankSwitch']>0;s.close()
  # New agent profiles preserve the established classification/editorial gates.
  workspace=root/'workspace';script('disassemble.py',rom,'--workspace',workspace,'--json');assert json.loads((workspace/'workspace.json').read_text())['platform']=='genesis';prompt=(workspace/'prompts/disassembly.txt').read_text();assert 'whole-ROM' in prompt and 'z80_bank' in prompt
  subprocess.run(['python3',str(workspace/'run.py'),'baseline'],check=True,capture_output=True,text=True)
  script('commentary.py','prepare',workspace,'--json');script('commentary.py','audit',workspace,ok=False);script('commentary.py','verify',workspace);script('commentary.py','read',workspace)
  archive=root/'archive';script('archive.py','archive',workspace,'--output',archive,'--name','synthetic-genesis');assert not list(archive.rglob('*.bin'))
  script('archive.py','restore',archive,'--rom',rom,'--out',root/'restored')
  port=root/'port';assert script('port.py','prepare',archive,'--rom',rom,'--out',port,'--json')['byte_identical_reference'];script('port.py','check',port);script('port.py','headless',port,'--frames',2)
  assert 'GENESIS_NTSC' in (port/'port/src/config.rs').read_text();assert 'genesis::tile' in (port/'port/src/game.rs').read_text()
  frozen=(port/'reference/source/game.asm').read_bytes();script('port.py','reference-init',port);working=port/'research/working';src=working/'source/game.asm';src.write_text('| Clarify the original synthetic reset.\n'+src.read_text());changes=json.loads((working/'changes.json').read_text());changes['changes']=[dict(id='genesis-reset-comment',kind='commentary',offset=512,length=2,before='Original reset note',after='Clarified reset entry',port_impact='Document initialization',evidence=['Original synthetic fixture'])];(working/'changes.json').write_text(json.dumps(changes));script('port.py','reference-verify',port,'--revision','genesis-reset');assert (port/'reference/source/game.asm').read_bytes()==frozen
 print('PASS Genesis: dual CPU/debugger/replay/maps/layout rebuilds/agent/archive/reference/native scaffold')
if __name__=='__main__':main()
