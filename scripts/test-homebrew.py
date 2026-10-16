#!/usr/bin/env python3
"""Pinned MIT source-built games: gameplay, replay, mapping and clean recipient rebuild."""
import pathlib,subprocess,tempfile,json,hashlib,re,argparse
root=pathlib.Path(__file__).resolve().parents[1]
parser=argparse.ArgumentParser();parser.add_argument('--binary',type=pathlib.Path,default=root/'target/release/romatlas');args=parser.parse_args()
suite=root/'tools/gb-asm-tutorial';revision='c931a929eac276156d65e0cb44cf422a7d19990b'
assert subprocess.check_output(['git','-C',str(suite),'rev-parse','HEAD'],text=True).strip()==revision
assert not subprocess.check_output(['git','-C',str(suite),'status','--porcelain','--untracked-files=no'],text=True).strip(), 'homebrew source checkout changed'
expected_hashes={'armada':'75fd77a7866fa811f237fdecb661f2305bcc2c1960e3a70b775f2709ac455ed0','unbricked':'c235cb25c228259c80d2c2930cdefc2c22abd48a1e9affeedeabc5e3e570fb85'}
results=[]
with tempfile.TemporaryDirectory() as directory:
 work=pathlib.Path(directory)
 for name,path in [('armada','galactic-armada/dist/GalacticArmada.gb'),('unbricked','unbricked/bricks/unbricked.gb')]:
  rom=suite/path
  assert hashlib.sha256(rom.read_bytes()).hexdigest()==expected_hashes[name], "rebuild pinned homebrew fixtures"
  with (work/(name+'.stderr')).open('w') as errors,subprocess.Popen([str(args.binary),'serve','--rom',str(rom)],stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=errors,text=True) as process:
   serial=0
   def call(method,params=None):
    global serial
    serial+=1;process.stdin.write(json.dumps({'jsonrpc':'2.0','id':serial,'method':method,'params':params or {}})+'\n');process.stdin.flush();response=json.loads(process.stdout.readline());assert response['id']==serial and 'result'in response,response;return response['result']
   def frames(count):return call('emulator.step',{'unit':'frame','count':count,'max_operations':1000000,'timeout_ms':30000})
   def memory(addr,length=1):return call('emulator.memory',{'address':addr,'length':length})['bytes']
   def input(button,pressed):call('emulator.input',{'button':button,'pressed':pressed})
   call('emulator.start');frames(20)
   if name=='armada':
    assert memory(0xc11c)==[0];input('a',True);frames(20);input('a',False);assert memory(0xc11c)==[1]
    for _ in range(30):
     frames(40);input('a',True);frames(40);input('a',False)
     if memory(0xc11c)==[2]:break
    assert memory(0xc11c)==[2],'story must advance into gameplay'
    address=0xc103
   else:address=0xfe01
   before=memory(address,2)
   call('trace.configure',{'enabled':True,'capacity':512})
   call('emulator.snapshot.save',{'name':'play'})
   input('right',True);first=frames(8);input('right',False);after=memory(address,2);assert after!=before,(name,before,after)
   raw=call('emulator.physical_memory',{'space':'oam','offset':0,'length':32})['bytes']
   call('emulator.snapshot.load',{'name':'play'});input('right',True);second=frames(8);input('right',False)
   assert first==second and memory(address,2)==after
   assert call('emulator.physical_memory',{'space':'oam','offset':0,'length':32})['bytes']==raw
   summary=call('trace.summary');assert summary['unique_instructions']>20 and summary['unique_edges']>20,summary
   discovery=call('analysis.discover',{'budget':5000,'limit':1000})
   candidates=[r for r in discovery['candidates'] if r['provenance']=='Observed'][:64];assert len(candidates)>=16
   for candidate in candidates:call('map.annotate',{'offset':candidate['start']['offset'],'length':candidate['length'],'kind':'code','provenance':'observed','confidence':1.0})
   sym=call('map.export')['sym'];metadata=work/(name+'.sym');metadata.write_text(sym)
   process.stdin.close();assert process.wait(timeout=10)==0
  # Separate recipient gets only local ROM + portable .sym, and uses stock mgbdis.
  recipient=work/(name+'-recipient');recipient.mkdir();owned=recipient/'owned.gb';owned.write_bytes(rom.read_bytes());owned.with_suffix('.sym').write_text(sym)
  generated=recipient/'source'
  subprocess.run(['python3',str(root/'tools/mgbdis/mgbdis.py'),str(owned),'--output-dir',str(generated)],check=True,capture_output=True,timeout=120)
  subprocess.run(['make','game.o'],cwd=generated,check=True,capture_output=True,timeout=120)
  subprocess.run(['rgblink','-o','rebuilt.gb','game.o'],cwd=generated,check=True,capture_output=True,timeout=120)
  assert (generated/'rebuilt.gb').read_bytes()==owned.read_bytes()
  results.append({'game':name,'revision':revision,'sha256':hashlib.sha256(owned.read_bytes()).hexdigest(),'observed_regions':len(candidates),'unique_instructions':summary['unique_instructions'],'unique_edges':summary['unique_edges'],'gameplay_input_changed_memory':True,'snapshot_replay':True,'stock_byte_identical':True})
(root/'build').mkdir(exist_ok=True);(root/'build/homebrew-results.json').write_text(json.dumps(results,indent=2)+'\n');print(json.dumps(results))
