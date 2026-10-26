import assert from 'node:assert/strict';
import {StateSnapshot} from './port-web/save-state.mjs';
class Runtime {
  constructor(){this.position=0;this.time=0;this.gain=1;this.boundaries=0;this.freed=false;this.advances=0;}
  advance(ms,input){this.advances++;this.time+=ms;this.position+=JSON.parse(input).direction*ms*this.gain;return Math.floor(ms);}
  configure(text){this.gain=JSON.parse(text).gain;}
  boundary(){this.boundaries++;}
  snapshot_json(){return JSON.stringify({position:this.position,time:this.time,gain:this.gain,boundaries:this.boundaries});}
  restore_snapshot_json(text){Object.assign(this,JSON.parse(text));}
  free(){this.freed=true;}
}
const history=new StateSnapshot(()=>new Runtime(),'test-build');
history.game.advance(16.25,'{"direction":1}');
history.game.configure('{"gain":2}');
history.game.boundary('{"direction":0}');
history.game.advance(17.125,'{"direction":-1}');
const slot=await history.capture();
history.game.advance(50,'{"direction":1}');
const restored=await history.restore(slot);
assert.equal(restored.runtime.position,16.25-17.125*2);
assert.equal(restored.runtime.time,33.375);
assert.equal(restored.runtime.gain,2);
assert.equal(restored.runtime.boundaries,1);
assert.equal(restored.runtime.advances,0,'snapshot restore must execute no simulation updates');
assert.equal(restored.ticks,33);
assert.equal(history.runtime.position,82);
assert.equal(history.runtime.freed,false);
const bad=JSON.parse(slot);bad.payload+=' ';
await assert.rejects(history.restore(JSON.stringify(bad)),/checksum/);
await assert.rejects(new StateSnapshot(()=>new Runtime(),'other-build').restore(slot),/different game build/);
await assert.rejects(history.restore('{"version":1}'),/old replay format/);
assert.equal(history.runtime.freed,false);
// Snapshot data contains current state only, even after more than the old limit.
for(let i=0;i<300000;i++)history.game.advance(0,'{"direction":0}');
const long=await history.capture();
assert.ok(long.length<1000);
assert.equal((await history.restore(long)).runtime.advances,0);
console.log('PASS: direct snapshots restore state without any replay, survive long sessions, and reject incompatible/corrupt data');

const {digest}=await import('./port-web/save-state.mjs');
const {createHash,randomBytes,webcrypto}=await import('node:crypto');
for(const size of [0,1,3,55,56,63,64,65,127,128,1024,1000000]) {
  const backing=randomBytes(size+6), bytes=backing.subarray(3,size+3);
  const expected=createHash('sha256').update(bytes).digest('hex');
  assert.equal(await digest(bytes,null),expected);
  assert.equal(await digest(bytes,webcrypto.subtle),expected);
  assert.equal(await digest(bytes.buffer.slice(bytes.byteOffset,bytes.byteOffset+bytes.length),null),expected);
}
console.log('PASS: SHA-256 fallback matches native crypto for empty, padding boundaries, subarrays and large binaries');
