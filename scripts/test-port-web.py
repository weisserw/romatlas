#!/usr/bin/env python3
"""Actual localhost Chromium/Wasm port smoke test using an authored synthetic ROM."""
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import threading
from urllib.request import urlopen
from urllib.error import HTTPError

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "scripts"))
import port_web


def run(*args):
    result = subprocess.run(list(map(str,args)),capture_output=True,text=True)
    if result.returncode:
        raise AssertionError((args,result.stdout,result.stderr))
    return json.loads(result.stdout)


def fixture(base):
    data=bytearray(32768)
    data[256:259]=bytes([195,80,1]);data[336:339]=bytes([0,24,253])
    data[333]=(-sum(data[308:333])-25)&255
    rom=base/'owned.gb';rom.write_bytes(data)
    analysis=base/'analysis'
    run('python3',ROOT/'scripts/disassemble.py',rom,'--workspace',analysis,'--json')
    binary=ROOT/'target/release/romatlas'
    run(binary,'store','init',analysis/'input/game.rom','--output',analysis/'analysis/state.json')
    run(binary,'store','export',analysis/'input/game.rom','--store',analysis/'analysis/state.json','--output',analysis/'maps/current.sym')
    run('python3',ROOT/'scripts/commentary.py','prepare',analysis,'--json')
    review=analysis/'commentary/review.json'
    records=json.loads(review.read_text())
    for row in records['regions']:
        row.update(status='reviewed',summary='Authored fixture data',source=['bank_000.asm:1'])
    records['editorial_audit']={'status':'reviewed','notes':'Synthetic browser fixture'}
    review.write_text(json.dumps(records))
    archive=base/'archive'
    run('python3',ROOT/'scripts/archive.py','archive',analysis,'--name','web-fixture','--output',archive)
    work=base/'future port'
    run('python3',ROOT/'scripts/port.py','prepare',archive,'--rom',rom,'--out',work,'--json')
    return work,rom


GAME = '''use anyhow::{Result,ensure};
use romatlas_port::{Game,assets::{Rom,MemoryAssets,Asset,Origin},graphics::Frame,input::InputState};
pub struct Port { value:u8,dirty:bool }
impl Port {
 pub fn load_assets(rom:&Rom,overrides:&MemoryAssets)->Result<Self>{
  let defaults=vec![Asset{id:"initial".into(),bytes:rom.read(0,1)?.to_vec(),origin:Origin::Rom{offset:0,length:1}}];
  let assets=overrides.collection("initial.json",defaults,|a|{ensure!(a.bytes.len()==1,"one-byte test asset");Ok(())})?;
  Ok(Self{value:assets.get("initial").unwrap().bytes[0],dirty:false})
 }
}
impl Game for Port {
 fn snapshot_json(&self)->Result<String>{Ok(serde_json::json!({"value":self.value,"dirty":self.dirty}).to_string())}
 fn restore_snapshot_json(&mut self,json:&str)->Result<()>{let v:serde_json::Value=serde_json::from_str(json)?;let n=v["value"].as_u64().ok_or_else(||anyhow::anyhow!("missing value"))?;ensure!(n<=255,"invalid value");let dirty=v["dirty"].as_bool().ok_or_else(||anyhow::anyhow!("missing dirty"))?;self.value=n as u8;self.dirty=dirty;Ok(())}
 fn reset(&mut self)->Result<()>{self.value=0;Ok(())}
 fn tick(&mut self,input:&InputState){if input.pressed("a"){self.value=self.value.wrapping_add(1);self.dirty=true;}}
 fn tick_players(&mut self,input:&romatlas_port::input::MultiInputState){self.tick(&input.primary);if input.players[1].pressed("a"){self.value=self.value.wrapping_add(10);self.dirty=true;}}
 fn render(&self,f:&mut Frame){f.clear([self.value,17,33,255]);}
 fn audio(&mut self,n:usize,_rate:u32,samples:&mut Vec<f32>){samples.extend(std::iter::repeat_n(0.1,n*2));}
 fn save_version(&self)->Option<u32>{Some(1)}
 fn load_save(&mut self,bytes:&[u8])->Result<()>{ensure!(bytes.len()==1,"invalid save");self.value=bytes[0];Ok(())}
 fn save_data(&self)->Result<Vec<u8>>{Ok(vec![self.value])}
 fn save_dirty(&self)->bool{self.dirty}
 fn save_acknowledged(&mut self){self.dirty=false;}
}
'''


def main():
    subprocess.run(["node", str(ROOT / "scripts/test-port-gamepads.mjs")], check=True)
    subprocess.run(["node", str(ROOT / "scripts/test-port-states.mjs")], check=True)
    try:
        from playwright.sync_api import sync_playwright
    except ImportError:
        raise SystemExit('Run bash scripts/setup-web-tests.sh then tools/web-tests/bin/python scripts/test-port-web.py')
    os.environ.setdefault('CARGO_TARGET_DIR',str(ROOT/'target/port-web-tests'))
    with tempfile.TemporaryDirectory(prefix='romatlas-web-') as temporary:
        base=Path(temporary);work,rom=fixture(base)
        # First compile the delivered future-workspace scaffold, not just a test entrypoint.
        port_web.build(work)
        (work/'port/src/game.rs').write_text(GAME)
        bundle=port_web.build(work)
        assert not any(p.suffix in {'.rom','.gb','.bin'} for p in bundle.rglob('*'))
        assets=base/'overrides';assets.mkdir()
        (assets/'initial.json').write_text(json.dumps({'version':1,'entries':[{'id':'initial','file':'value.bin'}]}))
        (assets/'value.bin').write_bytes(bytes([20]))
        with port_web.server(work,rom,bundle,assets,0) as server:
            assert server.server_address[0]=='0.0.0.0'
            thread=threading.Thread(target=server.serve_forever,daemon=True);thread.start()
            url=f'http://127.0.0.1:{server.server_port}/'
            assert urlopen(url+'rom').read()==rom.read_bytes()
            assert urlopen(url+'game_bg.wasm').headers['Content-Type']=='application/wasm'
            assert b'export function gamepads' in urlopen(url+'browser-gamepads.mjs').read()
            try:urlopen(url+'../workspace.json');raise AssertionError('unexpected source exposure')
            except HTTPError as e:assert e.code==404
            with sync_playwright() as playwright:
                browser=playwright.chromium.launch(headless=True,args=['--no-sandbox'])
                context=browser.new_context();page=context.new_page();errors=[]
                page.on('pageerror',lambda error:errors.append(str(error)))
                page.goto(url);page.wait_for_function("!document.getElementById('start').disabled")
                pixel="Array.from(document.getElementById('screen').getContext('2d').getImageData(0,0,1,1).data)"
                assert page.evaluate(pixel)==[20,17,33,255] # server override + Rust extraction path
                page.click('#start');page.wait_for_function("Number(document.getElementById('screen').dataset.ticks)>0")
                # Page defaults: centered display, explicit reset, and no focus pause.
                box=page.locator('#screen').bounding_box()
                assert abs(box['x']+box['width']/2-page.viewport_size['width']/2)<2
                assert page.locator('h1 + p').inner_text()=='This is a native recompilation of the original game in Rust, running in WebAssembly. You must click the Start button below to start the game engine and enable audio and input in the browser.'
                assert page.locator('#pause + #reset').inner_text()=='Reset (F3)'
                for kind in ['blur','visibilitychange']:
                    previous=int(page.locator('#screen').get_attribute('data-ticks'))
                    page.evaluate("""kind => {
                        Object.defineProperty(document,'hasFocus',{value:()=>false,configurable:true});
                        Object.defineProperty(document,'hidden',{value:true,configurable:true});
                        (kind==='blur'?window:document).dispatchEvent(new Event(kind));
                    }""",kind)
                    page.wait_for_function(f"Number(document.getElementById('screen').dataset.ticks)>{previous+2}")
                    assert page.locator('#status').inner_text()=='Running'
                    page.evaluate("delete document.hasFocus;delete document.hidden")
                page.click('#reset');page.wait_for_function("!document.getElementById('start').disabled")
                assert page.locator('#status').inner_text().startswith('Ready')
                assert page.evaluate(pixel)==[20,17,33,255]
                page.click('#start');page.wait_for_function("Number(document.getElementById('screen').dataset.ticks)>2")
                page.keyboard.down('z')
                page.wait_for_function("document.getElementById('screen').getContext('2d').getImageData(0,0,1,1).data[0]===21")
                page.keyboard.up('z')
                page.wait_for_function("Object.keys(localStorage).some(k=>k.includes(':battery:'))")
                save=page.evaluate("JSON.parse(localStorage.getItem(Object.keys(localStorage).find(k=>k.includes(':battery:'))))")
                assert save['bytes']==[21] and save['rom_sha256']==hashlib.sha256(rom.read_bytes()).hexdigest()
                page.click('#settings');page.locator('#volume').evaluate("el => {el.value='0.3';el.dispatchEvent(new Event('input',{bubbles:true}));}");page.locator('#height').fill('600');page.click('button[type=submit]')
                page.reload();page.wait_for_function("!document.getElementById('start').disabled")
                assert page.evaluate(pixel)==[21,17,33,255]
                page.click('#settings');assert page.locator('#volume').input_value()=='0.3'
                assert page.locator('#bindings label').evaluate_all("els=>els.map(el=>el.firstChild.textContent.trim().replace(/:$/,''))") == ['a','b','start','select','up','right','down','left']
                page.click('#cancel')
                saved_settings=page.evaluate("localStorage.getItem(Object.keys(localStorage).find(k=>k.includes(':settings:')))")
                for expected in [
                    'a b x y l r start select up right down left',
                    'a b c x y z start mode up right down left',
                    'button1 button2 pause up right down left',
                    'button1 button2 start up right down left',
                    'i ii run select up right down left',
                ]:
                    actions=expected.split()
                    page.evaluate("actions=>{const key=Object.keys(localStorage).find(k=>k.includes(':settings:'));const saved=JSON.parse(localStorage.getItem(key));saved.settings.bindings=Object.fromEntries(actions.map(a=>[a,[]]));localStorage.setItem(key,JSON.stringify(saved));}",list(reversed(actions)))
                    page.reload();page.wait_for_function("!document.getElementById('start').disabled")
                    page.click('#settings')
                    assert page.locator('#bindings label').evaluate_all("els=>els.map(el=>el.firstChild.textContent.trim().replace(/:$/,''))") == actions
                    page.click('#cancel')
                page.evaluate("saved=>localStorage.setItem(Object.keys(localStorage).find(k=>k.includes(':settings:')),saved)",saved_settings)
                page.reload();page.wait_for_function("!document.getElementById('start').disabled")
                assert page.locator('#integer_scale').count()==0
                assert page.locator('#width').count()==0
                box=page.locator('#screen').bounding_box()
                assert abs(box['width']/box['height']-4/3)<.01
                page.keyboard.press('F4')
                page.wait_for_function("document.getElementById('state-status').textContent==='State saved.'")
                page.click('#start');page.wait_for_function("document.getElementById('status').textContent==='Running'");page.keyboard.down('z')
                page.wait_for_function("document.getElementById('screen').getContext('2d').getImageData(0,0,1,1).data[0]===22")
                page.keyboard.up('z')
                page.keyboard.press('F5')
                page.wait_for_function("document.getElementById('game-message').textContent.includes('State restored')")
                assert page.evaluate(pixel)==[21,17,33,255]
                page.keyboard.press('F1')
                page.wait_for_function("document.getElementById('status').textContent==='Running'")
                # Rebind a to Q through the actual settings UI and persist it.
                page.click('#settings');page.locator('#bindings label').filter(has_text='a:').first.locator('button').first.click();page.keyboard.press('q');page.click('button[type=submit]')
                page.click('#start');page.wait_for_timeout(100);page.keyboard.down('q')
                page.wait_for_function("document.getElementById('screen').getContext('2d').getImageData(0,0,1,1).data[0]===22")
                page.keyboard.up('q')
                # Pause has no time debt or stuck input after resume.
                page.click('#pause');before=page.locator('#status').inner_text();page.wait_for_timeout(200);assert page.locator('#status').inner_text()==before
                page.click('#start');page.wait_for_timeout(100);assert page.evaluate(pixel)[0]==22
                # Browser standard gamepads reach independent player slots.
                before_ticks=int(page.locator('#screen').get_attribute('data-ticks'))
                page.evaluate("""() => { window.testPads=[0,1].map(index=>({id:'Synthetic pad',index,mapping:'standard',axes:[0,0,0,0],buttons:Array.from({length:17},()=>({pressed:false,value:0}))}));Object.defineProperty(navigator,'getGamepads',{value:()=>window.testPads}); }""")
                page.wait_for_function(f"Number(document.getElementById('screen').dataset.ticks)>{before_ticks+2}")
                page.evaluate("window.testPads[1].buttons[0]={pressed:true,value:1}")
                page.wait_for_function("document.getElementById('screen').getContext('2d').getImageData(0,0,1,1).data[0]===32")
                page.evaluate("window.testPads[1].buttons[0]={pressed:false,value:0}")
                # Quota failures are visible and leave dirty data pending.
                page.evaluate("() => {window.originalSetItem=Storage.prototype.setItem;Storage.prototype.setItem=function(){throw new DOMException('test quota','QuotaExceededError')};}")
                page.keyboard.down('q');page.wait_for_function("document.getElementById('error').textContent.includes('Battery save failed')");page.keyboard.up('q')
                page.evaluate("() => {Storage.prototype.setItem=window.originalSetItem;}")
                page.wait_for_function("JSON.parse(localStorage.getItem(Object.keys(localStorage).find(k=>k.includes(':battery:')))).bytes[0]===33")
                assert not errors,errors
                # Raw controllers stay visible and can be rebound while unassigned.
                raw_context=browser.new_context();raw_page=raw_context.new_page()
                raw_page.on('pageerror',lambda error:errors.append(str(error)))
                raw_page.add_init_script("""window.rawPad={id:'Unmapped USB pad',index:0,connected:true,mapping:'',axes:[0,0,0,0,-1,0],buttons:Array.from({length:20},()=>({pressed:false,value:0}))};Object.defineProperty(navigator,'getGamepads',{value:()=>[window.rawPad]});""")
                raw_page.goto(url);raw_page.wait_for_function("!document.getElementById('start').disabled")
                raw_page.click('#settings')
                assert raw_page.locator('#players select').first.locator('option').count()==3
                for select in raw_page.locator('#players select').all():select.select_option('none')
                binding=raw_page.locator('#bindings label').filter(has_text='a:').first.locator('button').first
                binding.click();raw_page.wait_for_timeout(100)
                assert 'Press a key' in binding.inner_text() # idle -1 axis must not capture
                raw_page.evaluate("window.rawPad.buttons[19]={pressed:true,value:1}")
                raw_page.wait_for_function("document.getElementById('bindings').textContent.includes('button19')")
                raw_page.evaluate("window.rawPad.buttons[19]={pressed:false,value:0}")
                raw_page.locator('#players select').first.select_option('auto')
                raw_page.click('button[type=submit]');raw_page.click('#start')
                raw_page.wait_for_function("Number(document.getElementById('screen').dataset.ticks)>2")
                raw_page.evaluate("window.rawPad.buttons[19]={pressed:true,value:1}")
                raw_page.wait_for_function("document.getElementById('screen').getContext('2d').getImageData(0,0,1,1).data[0]===21")
                raw_page.evaluate("window.rawPad.buttons[19]={pressed:false,value:0}")
                raw_page.click('#settings');binding.click()
                raw_page.evaluate("window.rawPad.axes[5]=-.9")
                raw_page.wait_for_function("document.getElementById('bindings').textContent.includes('axis5')")
                raw_page.evaluate("window.rawPad.axes[5]=0")
                raw_page.click('button[type=submit]');raw_page.click('#start')
                raw_page.wait_for_timeout(100)
                raw_page.evaluate("window.rawPad.axes[5]=-.9")
                raw_page.wait_for_function("document.getElementById('screen').getContext('2d').getImageData(0,0,1,1).data[0]===22")
                assert not errors,errors
                raw_context.close()
                context.close();browser.close()
            server.shutdown();thread.join()
        # The public future-workspace command builds and starts the same server.
        process=subprocess.Popen(['python3',str(work/'run.py'),'web','--host','127.0.0.1','--port','0'],stdout=subprocess.PIPE,stderr=subprocess.DEVNULL,text=True)
        try:
            import select
            ready,_,_=select.select([process.stdout],[],[],120)
            assert ready,'web command did not announce startup'
            announced=json.loads(process.stdout.readline())
            assert announced['host']=='127.0.0.1'
            assert announced['ok'] and urlopen(announced['url']+'rom').read()==rom.read_bytes()
        finally:
            process.terminate();process.wait(timeout=10)
        # Server rejects symlink escape before exposing any bytes.
        (assets/'value.bin').unlink();(assets/'value.bin').symlink_to(rom)
        try:port_web.asset_routes(assets);raise AssertionError('accepted escaping symlink')
        except ValueError:pass
    print('Wasm scaffold, localhost ROM/assets, Chromium frame/input/audio startup, 4:3 display, F1–F6 controls, full snapshots, settings and battery error checks passed')

if __name__=='__main__':main()
