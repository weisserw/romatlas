// Single-slot snapshots contain the current Rust state, never an input journal.
const MAX_BYTES = 4 * 1024 * 1024;
export class StateSnapshot {
  constructor(create, identity) {
    this.create=create;this.identity=identity;this.ticks=0;
    this.runtime=create();
    this.game=new Proxy(this.runtime,{get:(target,name)=>{
      const value=target[name];
      if(name==='advance')return (...args)=>{const count=value.apply(target,args);this.ticks+=count;return count;};
      return typeof value==='function'?value.bind(target):value;
    }});
  }
  async capture() {
    // Capture state and tick count synchronously before the checksum yields.
    const payload=this.runtime.snapshot_json(), ticks=this.ticks;
    if(payload.length>MAX_BYTES)throw Error('Snapshot exceeds the browser storage limit');
    const checksum=await digest(new TextEncoder().encode(payload));
    const json=JSON.stringify({version:2,identity:this.identity,savedAt:new Date().toISOString(),ticks,payload,checksum});
    if(json.length>MAX_BYTES)throw Error('Snapshot exceeds the browser storage limit');
    return json;
  }
  async restore(json) {
    if(typeof json!=='string'||json.length>MAX_BYTES)throw Error('Invalid or oversized snapshot');
    const saved=JSON.parse(json);
    if(saved.version!==2)throw Error('This checkpoint uses the old replay format. Create a new snapshot with Save State.');
    if(saved.identity!==this.identity)throw Error('This snapshot belongs to a different game build or audio configuration');
    if(typeof saved.payload!=='string'||!Number.isSafeInteger(saved.ticks)||saved.ticks<0)throw Error('Invalid snapshot');
    if(await digest(new TextEncoder().encode(saved.payload))!==saved.checksum)throw Error('Snapshot checksum mismatch');
    const candidate=new StateSnapshot(this.create,this.identity);
    try {
      candidate.runtime.restore_snapshot_json(saved.payload);
      candidate.ticks=saved.ticks;
      return candidate;
    }catch(error){candidate.runtime.free();throw error;}
  }
}

// Web Crypto is restricted to secure contexts. Use the identical SHA-256 on
// plain HTTP LAN origins so saved-state identities do not depend on transport.
export async function digest(bytes, subtle = globalThis.crypto?.subtle) {
  if (subtle) return hex(new Uint8Array(await subtle.digest('SHA-256', bytes)));
  const input = ArrayBuffer.isView(bytes)
    ? new Uint8Array(bytes.buffer, bytes.byteOffset, bytes.byteLength) : new Uint8Array(bytes);
  const data = new Uint8Array(Math.ceil((input.length + 9) / 64) * 64);
  data.set(input); data[input.length] = 0x80;
  const view = new DataView(data.buffer);
  const bits = input.length * 8;
  view.setUint32(data.length - 8, Math.floor(bits / 0x100000000));
  view.setUint32(data.length - 4, bits >>> 0);
  const k = [
    0x428a2f98,0x71374491,0xb5c0fbcf,0xe9b5dba5,0x3956c25b,0x59f111f1,0x923f82a4,0xab1c5ed5,
    0xd807aa98,0x12835b01,0x243185be,0x550c7dc3,0x72be5d74,0x80deb1fe,0x9bdc06a7,0xc19bf174,
    0xe49b69c1,0xefbe4786,0x0fc19dc6,0x240ca1cc,0x2de92c6f,0x4a7484aa,0x5cb0a9dc,0x76f988da,
    0x983e5152,0xa831c66d,0xb00327c8,0xbf597fc7,0xc6e00bf3,0xd5a79147,0x06ca6351,0x14292967,
    0x27b70a85,0x2e1b2138,0x4d2c6dfc,0x53380d13,0x650a7354,0x766a0abb,0x81c2c92e,0x92722c85,
    0xa2bfe8a1,0xa81a664b,0xc24b8b70,0xc76c51a3,0xd192e819,0xd6990624,0xf40e3585,0x106aa070,
    0x19a4c116,0x1e376c08,0x2748774c,0x34b0bcb5,0x391c0cb3,0x4ed8aa4a,0x5b9cca4f,0x682e6ff3,
    0x748f82ee,0x78a5636f,0x84c87814,0x8cc70208,0x90befffa,0xa4506ceb,0xbef9a3f7,0xc67178f2
  ];
  const hash = new Uint32Array([0x6a09e667,0xbb67ae85,0x3c6ef372,0xa54ff53a,0x510e527f,0x9b05688c,0x1f83d9ab,0x5be0cd19]);
  const words = new Uint32Array(64);
  const rotate = (x,n) => (x >>> n) | (x << (32-n));
  for (let offset=0;offset<data.length;offset+=64) {
    for(let i=0;i<16;i++) words[i]=view.getUint32(offset+i*4);
    for(let i=16;i<64;i++) {
      const x=words[i-15], y=words[i-2];
      words[i]=words[i-16]+(rotate(x,7)^rotate(x,18)^(x>>>3))+words[i-7]+(rotate(y,17)^rotate(y,19)^(y>>>10));
    }
    let [a,b,c,d,e,f,g,h]=hash;
    for(let i=0;i<64;i++) {
      const t1=(h+(rotate(e,6)^rotate(e,11)^rotate(e,25))+((e&f)^(~e&g))+k[i]+words[i])>>>0;
      const t2=((rotate(a,2)^rotate(a,13)^rotate(a,22))+((a&b)^(a&c)^(b&c)))>>>0;
      h=g;g=f;f=e;e=(d+t1)>>>0;d=c;c=b;b=a;a=(t1+t2)>>>0;
    }
    [a,b,c,d,e,f,g,h].forEach((value,i)=>{hash[i]+=value;});
  }
  return Array.from(hash,word=>word.toString(16).padStart(8,'0')).join('');
}
function hex(bytes) {return Array.from(bytes,b=>b.toString(16).padStart(2,'0')).join('');}
