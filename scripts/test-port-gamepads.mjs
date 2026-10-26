import assert from 'node:assert/strict';
import {gamepads, padState, capturePadBinding} from './port-web/browser-gamepads.mjs';
const pad = (mapping = '') => ({id:'USB controller', index:0, connected:true, mapping,
  buttons:Array.from({length:20}, () => ({pressed:false,value:0})), axes:[0,0,0,0,-1,0]});
let raw = pad();
const read = () => gamepads({getGamepads:() => [null,raw]});
assert.equal(read().length,1, 'unmapped controllers must remain visible');
raw.axes[0] = -.9;
assert.equal(padState(raw).axes.leftx,-.9);
raw.buttons[19].pressed = true;
assert.ok(padState(raw).buttons.includes('button19'));
assert.equal(padState(raw).axes.axis4,-1);
assert.ok(!('triggerleft' in padState(raw).axes));
let poll = capturePadBinding(read);
assert.equal(poll(),undefined,'held controls and idle -1 axes must not steal capture');
raw.axes[0]=0;raw.buttons[19].pressed=false;poll();
raw.buttons[19].pressed=true;
assert.deepEqual(poll(),{kind:'button',name:'button19'});
raw=pad();poll=capturePadBinding(read);
raw.axes[5]=-.9;
assert.deepEqual(poll(),{kind:'axis',name:'axis5',direction:-1});
raw=pad('standard');raw.buttons[0].pressed=true;raw.buttons[6].value=.8;
assert.ok(padState(raw).buttons.includes('a'));
assert.equal(padState(raw).axes.triggerleft,.8);
assert.deepEqual(gamepads({}),[]);
const duplicate=pad();
assert.equal(new Set(gamepads({getGamepads:()=>[raw,duplicate]}).map(p=>p.id)).size,2);
console.log('PASS: raw/standard pads, analog defaults, extra controls, neutral capture, duplicate IDs and absent API');

// Exercise the shared shell, including assignment routing and capture while paused.
const {readFileSync} = await import('node:fs');
const {runInNewContext} = await import('node:vm');
const elements = new Map();
const element = () => ({open:false, style:{}, dataset:{}, append(){}, replaceChildren(){}, addEventListener(){}});
const context = {
  gamepads, padState, capturePadBinding, console, structuredClone,
  navigator:{getGamepads:()=>[raw]},
  document:{getElementById:id=>{if(!elements.has(id))elements.set(id,element());return elements.get(id);},
    createElement:element, addEventListener(){}, hidden:false, hasFocus:()=>true},
  window:{addEventListener(){}}, requestAnimationFrame(){},
};
let shell = readFileSync(new URL('./port-web/shell.js',import.meta.url),'utf8')
  .replace(/^import .*;$/gm,'').replace('boot().catch(report);','');
raw=pad();
shell += `
assignments=['auto','none','none','none'];
keys.add('Z');
const initial=physical();
if(initial[0].keys[0]!=='Z')throw Error('Keyboard lost during pad merge');
if(pads().length!==1)throw Error('Raw pad filtered');
assignments=['none','none','none','none'];
draft={settings:{bindings:{a:[]}},assignments:[...assignments]};
capture='a';pollCapture=capturePadBinding(pads);
`;
runInNewContext(shell,context);
raw.buttons[19].pressed=true;
runInNewContext(`loop(0);if(draft.settings.bindings.a[0]?.name!=='button19')throw Error('Unassigned pad capture failed');`,context);
console.log('PASS: shared shell retains keyboard input and captures raw controls with all players unassigned');
