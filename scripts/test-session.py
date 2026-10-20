#!/usr/bin/env python3
"""Own synthetic ROM; persistent JSON-RPC input/snapshot/map/export workflow."""
import pathlib, tempfile, subprocess, json, shutil
root = pathlib.Path(__file__).resolve().parents[1]
with tempfile.TemporaryDirectory() as directory:
    work = pathlib.Path(directory)
    rom = work / 'session.rom'
    data = bytearray(0x8000)
    data[0x100:0x103] = bytes([0xc3, 0x50, 1])
    data[0x150:0x158] = bytes([0x3e, 0x42, 0xea, 0, 0xc0, 0x04, 0x18, 0xfd])
    data[0x250:0x256] = bytes([1, 0, 0x40, 1, 0x20, 0x40])
    data[0x28] = 0xe9
    data[0x310:0x313] = bytes([0xef,0xdf,0])
    data[0x300:0x302] = bytes([0xd3,0xdb])
    data[0x14d] = (-sum(data[0x134:0x14d])-25) & 255
    rom.write_bytes(data)
    with subprocess.Popen([str(root / 'target/release/romatlas'), 'serve'], stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True) as process:
        serial = 0
        def call(method, params=None, error=False):
            global serial
            serial += 1
            process.stdin.write(json.dumps({'jsonrpc':'2.0', 'id':serial, 'method':method, 'params':params or {}})+'\n')
            process.stdin.flush()
            response = json.loads(process.stdout.readline())
            assert response['id'] == serial, response
            if error:
                assert 'error' in response, response
                return response['error']
            assert 'result' in response, response
            return response['result']
        assert 'emulator.snapshot.load' in call('api.describe')['methods']
        call('emulator.start', error=True)
        call('rom.open', {'path':str(rom)})
        exported_file = work/'file-only.sym'
        file_only = call('map.export', {'path':str(exported_file),'include_sym':False})
        assert 'sym' not in file_only and file_only['bytes'] == exported_file.stat().st_size
        assert exported_file.read_text() == call('map.export')['sym']
        call('map.export', {'include_sym':False},error=True)
        call('map.export', {'path':str(work/'invalid.sym'),'include_sym':'no'},error=True)
        assert not (work/'invalid.sym').exists()
        aliases = [rom,work/'input-link',work/'input-hardlink']
        aliases[1].symlink_to(rom)
        aliases[2].hardlink_to(rom)
        alias_store = work/'alias-store.json'
        call('map.save',{'path':str(alias_store)})
        for alias in aliases:
            for method in ['map.export','map.save']:
                assert 'aliases the input ROM' in call(method,{'path':str(alias)},error=True)['message']
                assert rom.read_bytes() == data
            result = subprocess.run([str(root/'target/release/romatlas'),'map','init',str(rom),'--output',str(alias)],capture_output=True,text=True)
            assert result.returncode == 1 and 'aliases the input ROM' in result.stderr
            assert rom.read_bytes() == data
            result = subprocess.run([str(root/'target/release/romatlas'),'store','export',str(rom),'--store',str(alias_store),'--output',str(alias)],capture_output=True,text=True)
            assert result.returncode == 1 and 'aliases the input ROM' in result.stderr
            assert rom.read_bytes() == data
        assert call('rom.read', {'offset':256, 'length':3})['bytes'] == [0xc3,0x50,1]
        assert call('analysis.decode', {'offset':256, 'count':1})['items'][0]['instruction']['target'] == 336
        structure = call('analysis.structure', {'offset':256,'cpu_context':{'cpu_address':256,'fixed_bank':0,'switch_bank':1}})
        assert structure['length']==3 and structure['successors'][0]['offset']==336
        assert structure['references'][0]['context']['fixed_bank_asserted']==0
        call('analysis.structure', {'offset':256,'cpu_context':{'cpu_address':257,'fixed_bank':0}}, error=True)
        call('analysis.structure', {'offset':256,'cpu_context':{'cpu_address':256,'fixed_bank':1}}, error=True)
        contextual = call('analysis.discover_context', {'entrypoints':[{'offset':256,'cpu_context':{'cpu_address':256,'fixed_bank':0,'switch_bank':1}}],'budget':50})
        assert contextual['total']==5 and not contextual['budget_exhausted']
        assert all('gb.cpu_context' not in item['start']['context'] for item in contextual['candidates'])
        assert contextual['references'][0]['context']['source_cpu_address']==256
        ref_page=call('analysis.discover_context', {'entrypoints':[{'offset':256,'cpu_context':{'cpu_address':256,'fixed_bank':0,'switch_bank':1}}],'budget':50,'limit':1,'reference_offset':1})
        assert ref_page['references']==contextual['references'][1:2]
        assert ref_page['next_reference_offset']==min(2,contextual['total_references'])
        invalid_roots=[{'offset':address,'cpu_context':{'cpu_address':address,'fixed_bank':0,'switch_bank':1}} for address in [768,769]]
        diagnostic_all=call('analysis.discover_context', {'entrypoints':invalid_roots,'budget':5})
        diagnostic_page=call('analysis.discover_context', {'entrypoints':invalid_roots,'budget':5,'limit':1,'diagnostic_offset':1})
        assert diagnostic_all['total_diagnostics']==2
        assert diagnostic_page['diagnostics']==diagnostic_all['diagnostics'][1:2]
        assert diagnostic_page['next_diagnostic_offset']==2
        profile_context={'cpu_address':0x310,'fixed_bank':0,'switch_bank':1}
        profile={'image_sha256':call('rom.info')['sha256'],'offset':0x310,'cpu_context':profile_context,
                 'source_bytes':[0xef],'inline_bytes':[0xdf,0],'evidence':'synthetic all-arrival dispatcher fixture',
                 'all_arrivals':True,'returns_after_inline':False}
        profile_request={'entrypoints':[{'offset':0x310,'cpu_context':profile_context}],'budget':20,'use_observed':False,'call_profiles':[profile]}
        profiled=call('analysis.discover_context',profile_request)
        assert {c['start']['offset'] for c in profiled['candidates']}=={0x310,0x28}
        profile['all_arrivals']=False
        call('analysis.discover_context',profile_request,error=True)



        call('emulator.start')
        initial = call('emulator.state')
        call('trace.configure', {'enabled':True,'capacity':100})
        debug_trace=call('trace.summary')
        call('emulator.registers.set', {'registers':{'a':42,'pc':49152}})
        call('emulator.memory.write', {'address':49152,'bytes':[0x3e,99]})
        assert call('emulator.memory', {'address':49152,'length':2})['bytes']==[0x3e,99]
        edited = call('emulator.state')
        assert edited['operations']==initial['operations'] and edited['ppu_cycles']==initial['ppu_cycles']
        call('emulator.registers.set', {'registers':{'a':7,'sp':65536}}, error=True)
        call('emulator.memory.write', {'address':65535,'bytes':[1,2]}, error=True)
        assert call('emulator.state')==edited
        assert call('trace.summary')==debug_trace
        call('emulator.step', {'operations':1})
        assert call('emulator.state')['registers']['a']==99
        assert all(event['kind']!='Write' for event in call('trace.query')['items'])
        capture=call('emulator.framebuffer')
        assert capture['width']==160 and capture['height']==144 and len(capture['pixels'])==160*144*3
        disk_snapshot=work/'machine.snapshot.json'
        saved=call('emulator.state')
        call('emulator.snapshot.export', {'path':str(disk_snapshot)})
        call('emulator.reset')
        assert call('emulator.snapshot.import', {'path':str(disk_snapshot)})==saved
        assert call('emulator.memory', {'address':49152,'length':2})['bytes']==[0x3e,99]
        call('emulator.snapshot.export', {'path':str(disk_snapshot)}, error=True)
        envelope=json.loads(disk_snapshot.read_text())
        for field, invalid in [('format','future-format'),('model','cgb')]:
            previous=envelope[field]
            envelope[field]=invalid
            incompatible=work/'incompatible.snapshot.json'
            incompatible.write_text(json.dumps(envelope))
            call('emulator.snapshot.import', {'path':str(incompatible)}, error=True)
            assert call('emulator.state')==saved
            envelope[field]=previous
        envelope['image_sha256']='0'*64
        bad_snapshot=work/'bad.snapshot.json'
        bad_snapshot.write_text(json.dumps(envelope))
        call('emulator.snapshot.import', {'path':str(bad_snapshot)}, error=True)
        assert call('emulator.state')==saved
        envelope['image_sha256']=capture['image_sha256']
        envelope['snapshot']['bytes']=[0,1,2]
        bad_snapshot.write_text(json.dumps(envelope))
        call('emulator.snapshot.import', {'path':str(bad_snapshot)}, error=True)
        assert call('emulator.state')==saved
        call('emulator.reset')
        caps=call('emulator.capabilities')
        assert caps['platform']=='gb' and caps['clock']=='ppu-ticks'
        assert caps['units']==['operation','cycle','frame']
        before_typed=call('emulator.state')
        assert call('emulator.step', {'unit':'operation','count':0}) == before_typed
        call('emulator.step', {'unit':'operation','count':2,'max_operations':1}, error=True)
        assert call('emulator.state') == before_typed
        assert any(s['name']=='cartridge-rom' and s['size']==0x8000 for s in caps['address_spaces'])
        call('trace.configure', {'enabled':True,'capacity':100})
        call('emulator.run', {'operations':4})
        assert call('emulator.memory', {'address':0xc000, 'length':1})['bytes'] == [0x42]
        summary = call('trace.summary')
        assert summary['unique_instructions'] == 4
        events = call('trace.query')['items']
        collected = []
        offset = 0
        while offset < len(events):
            page = call('trace.query', {'offset':offset, 'limit':2})
            assert page['total'] == len(events)
            assert page['next_offset'] == min(offset + 2, len(events))
            collected.extend(page['items'])
            offset = page['next_offset']
        assert collected == events
        for offset, limit in [(len(events),2), (len(events)+10,2), (0,0)]:
            page = call('trace.query', {'offset':offset,'limit':limit})
            assert page['items'] == []
            assert page['next_offset'] == min(offset + limit, len(events))
        assert any(item['kind'] == 'Write' and item['context']['cpu_address'] == 49152 for item in call('trace.query')['items'])
        assert call('analysis.coverage')['total'] == 4
        edges = call('analysis.edges')
        assert edges['total'] == 4 and all(edge['provenance']=='Observed' for edge in edges['items'])
        context_discovery = call('analysis.discover_context', {'entrypoints':[{'offset':256,'cpu_context':{'cpu_address':256,'fixed_bank':0,'switch_bank':1}}],'budget':50})
        assert any(r['kind']=='observed-successor' for r in context_discovery['references'])
        assert all('gb.cpu_context' not in r['source']['context'] for r in context_discovery['references'])
        static_context = call('analysis.discover_context', {'entrypoints':[{'offset':256,'cpu_context':{'cpu_address':256,'fixed_bank':0,'switch_bank':1}}],'budget':50,'use_observed':False})
        assert all(r['kind']!='observed-successor' for r in static_context['references'])
        call('emulator.memory', {'address':256,'length':32})
        assert call('trace.summary') == summary
        assert call('emulator.physical_memory', {'space':'wram','offset':0,'length':1})['bytes'] == [0x42]
        assert call('trace.summary') == summary
        call('emulator.input', {'button':'a','pressed':True})
        call('emulator.snapshot.save', {'name':'held'})
        before = call('emulator.state')
        after = call('emulator.run', {'operations':30})
        call('emulator.input', {'button':'a','pressed':False})
        call('emulator.snapshot.load', {'name':'held'})
        assert call('emulator.state') == before
        assert call('emulator.run', {'operations':30}) == after
        # Snapshot input must replace live input, including both joypad rows.
        buttons = ['right', 'left', 'up', 'down', 'a', 'b', 'select', 'start']
        for index, button in enumerate(buttons):
            for other in buttons:
                call('emulator.input', {'button':other, 'pressed':False})
            row = 0x20 if index < 4 else 0x10
            call('emulator.memory.write', {'address':0xff00, 'bytes':[row]})
            call('emulator.input', {'button':button, 'pressed':True})
            expected = 0xc0 | row | (0xf ^ (1 << (index % 4)))
            def joypad():
                return call('emulator.memory', {'address':0xff00, 'length':1})['bytes'][0]
            assert joypad() == expected
            call('emulator.snapshot.save', {'name':'input-held'})
            input_file = work / f'input-{button}.snapshot.json'
            call('emulator.snapshot.export', {'path':str(input_file)})
            for method, params in [('emulator.snapshot.load', {'name':'input-held'}),
                                   ('emulator.snapshot.import', {'path':str(input_file)})]:
                for other in buttons:
                    call('emulator.input', {'button':other, 'pressed':other != button})
                call('emulator.memory.write', {'address':0xff00, 'bytes':[0x30]})
                call(method, params)
                assert joypad() == expected, (button, method, joypad(), expected)
                replay = call('emulator.run', {'operations':30})
                assert joypad() == expected
                call(method, params)
                assert call('emulator.run', {'operations':30}) == replay
            call('emulator.input', {'button':button, 'pressed':False})
            call('emulator.snapshot.save', {'name':'input-released'})
            call('emulator.input', {'button':button, 'pressed':True})
            call('emulator.snapshot.load', {'name':'input-released'})
            assert joypad() == 0xc0 | row | 0xf
            released_file = work / f'released-{button}.snapshot.json'
            call('emulator.snapshot.export', {'path':str(released_file)})
            call('emulator.input', {'button':button, 'pressed':True})
            call('emulator.snapshot.import', {'path':str(released_file)})
            assert joypad() == 0xc0 | row | 0xf
            if index == 0:
                # Reject the old core layout and roll back a partial core load,
                # preserving live input and counters in both cases.
                for failure in ['old-version', 'truncated']:
                    invalid = json.loads(input_file.read_text())
                    if failure == 'old-version':
                        assert invalid['snapshot']['bytes'][4] == 9
                        invalid['snapshot']['bytes'][4] = 8
                    else:
                        invalid['snapshot']['bytes'] = invalid['snapshot']['bytes'][:-8]
                    rejected_file = work / f'{failure}.snapshot.json'
                    rejected_file.write_text(json.dumps(invalid))
                    call('emulator.input', {'button':'left', 'pressed':True})
                    state_before_rejection = call('emulator.state')
                    input_before_rejection = joypad()
                    call('emulator.snapshot.import', {'path':str(rejected_file)}, error=True)
                    assert call('emulator.state') == state_before_rejection
                    assert joypad() == input_before_rejection
        cycle_start = call('emulator.state')
        cycle_end = call('emulator.step', {'unit':'cycle','count':100})
        assert 100 <= cycle_end['ppu_cycles']-cycle_start['ppu_cycles'] < 124
        frame_end = call('emulator.step', {'unit':'frame','count':1,'max_operations':100000,'timeout_ms':10000})
        assert frame_end['frames'] == cycle_end['frames']+1
        assert call('trace.summary')['counts']['Frame'] >= 1
        assert call('emulator.run', {'operations':1000001}, error=True)['code'] == -32000
        discovered = call('analysis.discover', {'entrypoints':[256],'budget':50})
        assert discovered['total'] == 5 and not discovered['budget_exhausted'], discovered
        assert any(r['kind']=='observed-successor' for r in discovered['references'])
        assert any(c['provenance'] == 'Observed' for c in discovered['candidates'])
        for candidate in discovered['candidates']:
            call('map.annotate', {'offset':candidate['start']['offset'],'length':candidate['length'],'kind':'code','provenance':candidate['provenance'].lower(),'confidence':candidate['confidence']})
        original = call('map.get')
        first = original['items'][0]
        r = first['region']
        edit = {'id':first['id'],'offset':r['start']['offset'],'length':r['length'],'kind':'code'}
        metadata = {'name':'AgentEntry','role':'entry routine','evidence':'synthetic execution trace','comment':'reviewed'}
        call('map.annotate', {**edit, **metadata})
        assert call('map.get')['items'][0]['metadata'] == metadata
        retained = call('map.get')
        for invalid in [{'role':7}, {'name':'invalid symbol'}, {'name':'.code:3'}, {'offset':257}]:
            call('map.annotate', {**edit, **invalid}, error=True)
            assert call('map.get') == retained
        call('map.annotate', {**edit, 'comment':None})
        del metadata['comment']
        call('map.annotate', edit)
        assert call('map.get')['items'][0]['metadata'] == metadata
        original = call('map.get')
        blocked_context = call('analysis.discover_context', {'entrypoints':[{'offset':0x250,'cpu_context':{'cpu_address':0x250,'fixed_bank':0,'switch_bank':1}}],'budget':1,'use_observed':False})
        assert blocked_context['visited']==1
        assert call('map.get') == original
        call('map.annotate', {'offset':257,'length':1,'kind':'data'}, error=True)
        assert call('map.get') == original
        temporary = call('map.annotate', {'offset':0x380,'length':1,'kind':'data','role':'temporary evidence'})['id']
        call('map.annotate', {'id':temporary,'offset':0x381,'length':1,'kind':'data'})
        assert next(item for item in call('map.get')['items'] if item['id']==temporary)['metadata'] is None
        call('map.remove', {'id':temporary})
        call('map.validate')
        call('map.save', {'path':str(work/'state.json')})
        sym = work/'session.sym'
        exported = call('map.export', {'path':str(sym)})['sym']
        assert '.code:3' in exported and 'sha256' not in exported
        call('disasm.generate', {'out':str(work/'source')})
        assert call('disasm.verify', {'generated':str(work/'source')})['byte_identical']
        call('rom.open', {'path':str(rom)})
        assert call('map.get')['total'] == 0
        call('map.load', {'path':str(work/'state.json')})
        assert call('map.get') == original
        assert 'AgentEntry' in exported and 'synthetic execution trace' not in exported
        assert call('map.export')['sym'] == exported
        pointers = call('analysis.pointers', {'offset':0x200,'count':4})
        assert pointers['candidate'] is None and pointers['total']==4
        assert all(r['kind']=='pointer-candidate' for r in pointers['references'])
        far = call('analysis.pointers', {'offset':0x250,'count':2,'layout':'bank8-word-le','page_offset':1,'limit':1})
        assert far['resolved_rom']==2 and far['entry_width']==3 and far['total']==2
        assert far['candidate']['length']==6 and far['candidate']['provenance']=='Static'
        assert far['references'][0]['source']['offset']==0x253
        assert far['references'][0]['target']['offset']==0x4020
        assert far['references'][0]['context']['encoded_bank']==1
        assert call('analysis.pointers', {'offset':0x100,'count':1,'layout':'bank8-word-le'})['conflicts']
        call('analysis.pointers', {'offset':0x3ffe,'count':1,'layout':'bank8-word-le'}, error=True)
        call('analysis.pointers', {'layout':'guess'}, error=True)
        call('analysis.pointers', {'offset':0x3fff,'count':1}, error=True)
        assert call('map.export')['sym'] == exported
        for method, name, size in [('map.import','oversized.sym',1024*1024+1),('map.load','oversized.json',64*1024*1024+1)]:
            oversized = work/name
            with oversized.open('wb') as file:
                file.truncate(size)
            error = call(method, {'path':str(oversized)}, error=True)
            assert 'exceeds' in error['message']
            assert call('map.export')['sym'] == exported
        community = '00:0100 Entry\n00:0100 .code:3\n00:0200 .byt:20:2\n00:0300 .asc:10\n01:4000 Tiles\n01:4000 .image:8:w8,pe4,1bpp\n00:c000 wBuffer\n00:ff00 rP1\n'
        call('map.import', {'sym':community})
        imported = call('map.export')['sym']
        for line in community.splitlines():
            assert line in imported
        image = next(item for item in call('map.get')['items'] if item['region']['kind']=='Image')
        image_edit = {'id':image['id'],'offset':0x4000,'length':8,'kind':'image'}
        call('map.annotate', {**image_edit,'role':'one-bit tiles','evidence':'explicit imported 1bpp directive'})
        assert call('map.export')['sym'] == imported
        before_rejection = call('map.get')
        call('map.annotate', {**image_edit,'name':'DuplicateTiles'}, error=True)
        assert call('map.get') == before_rejection
        call('map.save', {'path':str(work/'community.json')})
        call('rom.open', {'path':str(rom)})
        call('map.load', {'path':str(work/'community.json')})
        assert call('map.get') == before_rejection
        assert call('map.export')['sym'] == imported
        call('map.import', {'sym':'00:0200 .byt:20\n00:0201 .data:10\n'}, error=True)
        assert call('map.export')['sym'] == imported
        call('disasm.generate', {'out':str(work/'bad-budget'),'timeout_ms':0}, error=True)
        assert not (work/'bad-budget').exists()
        call('disasm.generate', {'out':str(work/'community-source')})
        (work/'community-source/game.o').write_bytes(b'partial cached object')
        assert call('disasm.verify', {'generated':str(work/'community-source')})['byte_identical']
        community_rom = work/'community.gb'
        community_rom.write_bytes(data)
        (work/'community.sym').write_text(imported)
        subprocess.run(['python3',str(root/'tools/mgbdis/mgbdis.py'),str(community_rom),'--output-dir',str(work/'community-stock')],check=True,stdout=subprocess.PIPE)
        subprocess.run(['make','gfx/Tiles.1bpp'],cwd=work/'community-stock',check=True,stdout=subprocess.PIPE)
        subprocess.run(['rgbasm','-D','wBuffer=$c000','-o','game.o','game.asm'],cwd=work/'community-stock',check=True)
        subprocess.run(['rgblink','-o','rebuilt.gb','game.o'],cwd=work/'community-stock',check=True)
        assert (work/'community-stock/rebuilt.gb').read_bytes() == data
        sym.write_text(exported)
        call('analysis.nonexistent', error=True)
        call('emulator.start')
        call('trace.configure', {'enabled':True,'capacity':0})
        stream_path=work/'events.jsonl'
        call('trace.stream.start', {'path':str(stream_path),'max_bytes':100000})
        call('emulator.run', {'operations':3})
        stream=call('trace.stream.stop')['stream']
        lines=stream_path.read_text().splitlines()
        assert stream['records']==len(lines)>3
        assert all(json.loads(line)['sequence']==index for index,line in enumerate(lines))
        call('trace.stream.start', {'path':str(stream_path)}, error=True)
        state = call('emulator.start', {'model':'cgb'})
        assert state['model'] == 'cgb'
        call('emulator.reset')
        assert call('emulator.state')['model'] == 'cgb'
        identity_state = call('emulator.state')
        identity_map = call('map.export')['sym']
        call('emulator.snapshot.save', {'name':'identity'})
        changed = bytearray(data)
        changed[0x2000] ^= 1
        rom.write_bytes(changed)
        rejected_output = work/'identity-rejected'
        for method, params in [('emulator.start',{}),('emulator.reset',{}),('disasm.generate',{'out':str(rejected_output)}),('disasm.verify',{'generated':str(work/'community-source')})]:
            assert 'ROM changed' in call(method,params,error=True)['message']
            assert call('emulator.state') == identity_state
            assert call('map.export')['sym'] == identity_map
        assert not rejected_output.exists()
        call('emulator.snapshot.load', {'name':'identity'})
        rom.write_bytes(data)
        process.stdin.close()
        assert process.wait(timeout=5) == 0, process.stderr.read()
    # Stock recipient uses no server, internal JSON, traces, or first assembly.
    shutil.rmtree(work/'source')
    (work/'state.json').unlink()
    subprocess.run(['python3',str(root/'tools/mgbdis/mgbdis.py'),str(rom),'--output-dir',str(work/'recipient')],check=True,stdout=subprocess.PIPE)
    subprocess.run(['make','game.o'],cwd=work/'recipient',check=True,stdout=subprocess.PIPE)
    subprocess.run(['rgblink','-o','rebuilt.gb','game.o'],cwd=work/'recipient',check=True)
    assert (work/'recipient/rebuilt.gb').read_bytes() == data
    print('PASS: persistent RPC replay, map edits, rejected conflict, state persistence, stock recipient round trip')
