#!/usr/bin/env python3
"""Parameter/error regression with a local synthetic ROM only."""
import json
import pathlib
import subprocess
import tempfile

root = pathlib.Path(__file__).resolve().parents[1]
with tempfile.TemporaryDirectory() as directory:
    work = pathlib.Path(directory)
    rom = work/'api.gb'
    data = bytearray(0x8000)
    data[0x100:0x103] = bytes([0xc3,0x50,1])
    data[0x150:0x153] = bytes([0,0x18,0xfd])
    data[0x14d] = (-sum(data[0x134:0x14d])-25)&255
    rom.write_bytes(data)
    with subprocess.Popen([str(root/'target/debug/romatlas'),'serve'],stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True) as process:
        serial = 0
        def call(method, params=None, rejected=False):
            global serial
            serial += 1
            process.stdin.write(json.dumps({'jsonrpc':'2.0','id':serial,'method':method,'params':{} if params is None else params})+'\n')
            process.stdin.flush()
            response = json.loads(process.stdout.readline())
            assert response['id'] == serial, response
            if rejected:
                assert 'error' in response and response['error']['message'], response
                return response['error']
            assert 'result' in response, response
            return response['result']
        description = call('api.describe')
        for method in description['methods']:
            assert call(method, [], True)['code'] == -32602
        assert call('missing.method',{},True)['code'] == -32601
        call('rom.open',{'path':str(rom)})
        call('emulator.start')
        assert description['max_request_bytes'] == description['max_response_bytes'] == 1048576
        assert len(call('rom.read')['bytes']) == 16
        assert len(call('analysis.decode')['items']) == 16
        assert call('rom.regions')['items'] == [{'offset':0,'length':32768}]
        assert call('map.get')['total'] == 0
        assert call('trace.query')['items'] == []
        capabilities = call('emulator.capabilities')
        assert capabilities['platform'] == 'gb' and capabilities['snapshots'] and capabilities['inputs']
        spaces = {space['name']:space['size'] for space in capabilities['address_spaces']}
        assert spaces['cpu'] == 65536 and spaces['cartridge-rom'] == 32768
        assert len(call('emulator.memory')['bytes']) == 16
        assert call('emulator.physical_memory',{'space':'cartridge-rom'})['bytes'] == list(data[:16])
        assert call('emulator.step')['operations'] == 1
        assert call('emulator.run')['operations'] == 1001
        call('emulator.snapshot.save',{'name':'defaults'})
        saved = call('emulator.state')
        call('emulator.step',{'unit':'cycle','count':1})
        assert call('emulator.state')['ppu_cycles'] > saved['ppu_cycles']
        assert call('emulator.snapshot.load',{'name':'defaults'}) == saved
        call('emulator.reset')
        state = call('emulator.state')
        mapping = call('map.export')['sym']
        trace = call('trace.summary')
        cases = [
            ('rom.read',{'offset':-1}),('rom.read',{'length':4097}),
            ('rom.regions',{'limit':1001}),('analysis.unknown',{'offset':'zero'}),
            ('emulator.start',{'model':'bad'}),('emulator.run',{'operations':1000001}),
            ('emulator.run',{'timeout_ms':0}),('emulator.step',{'unit':'bad'}),
            ('emulator.memory',{'address':65535,'length':2}),
            ('emulator.physical_memory',{'space':'bad'}),
            ('emulator.input',{'button':'bad','pressed':True}),
            ('emulator.input',{'button':'a','pressed':'yes'}),
            ('emulator.snapshot.save',{'name':'x'*129}),
            ('emulator.snapshot.load',{'name':'missing'}),
            ('trace.configure',{'enabled':'yes'}),('trace.configure',{'capacity':100001}),
            ('trace.configure',{'kinds':['bad']}),('trace.query',{'limit':1001}),
            ('trace.stream.start',{'path':str(work/'invalid.jsonl'),'max_bytes':-1}),
            ('analysis.coverage',{'limit':1001}),('analysis.edges',{'offset':None}),
            ('analysis.decode',{'count':4097}),('analysis.discover',{'budget':100001}),
            ('analysis.references',{'entrypoints':'bad'}),('analysis.discover_context',{}),
            ('analysis.pointers',{'layout':'bad'}),('analysis.structure',{'cpu_context':False}),
            ('map.get',{'limit':1001}),('map.annotate',{'offset':0,'length':0,'kind':'data'}),
            ('map.annotate',{'offset':0,'length':1,'kind':'bad'}),
            ('map.annotate',{'offset':0,'length':1,'kind':'data','confidence':2}),
            ('map.remove',{'id':999}),('map.import',{'sym':'zz:0000 Invalid'}),
            ('map.load',{'path':str(work/'missing.json')}),
            ('map.export',{'include_sym':False}),
            ('disasm.generate',{'out':str(work/'invalid-source'),'timeout_ms':0}),
            ('disasm.verify',{'generated':str(work/'missing-source')})]
        for method, params in cases:
            call(method,params,True)
            assert call('emulator.state') == state, (method,'state changed')
            assert call('map.export')['sym'] == mapping, (method,'map changed')
            assert call('trace.summary') == trace, (method,'trace changed')
        assert not (work/'invalid.jsonl').exists()
        assert not (work/'invalid-source').exists()
        assert rom.read_bytes() == data
        process.stdin.close()
        assert process.wait(timeout=5) == 0, process.stderr.read()
        print(json.dumps({'ok':True,'methods':len(description['methods']),'rejected_cases':len(cases),'state_map_trace_preserved':True}))
