"""Lossless bounded source transport logging, format jsonl-zlib-base64-v1."""
import base64,hashlib,json,sys,zlib
from pathlib import Path
sys.path.insert(0,str(Path(__file__).resolve().parents[4]/'scripts'))
from native_debug_client import PortClient,MAX_MESSAGE
FORMAT='jsonl-zlib-base64-v1'
BLOCK_BYTES=65536
BLOCK_RECORDS=128

def pack_records(raw,count):
    encoded=base64.b64encode(zlib.compress(raw,6)).decode()
    return dict(direction='compressed_records',value=dict(format=FORMAT,records=count,raw_bytes=len(raw),sha256=hashlib.sha256(raw).hexdigest(),pieces={str(i//2000):encoded[i:i+2000] for i in range(0,len(encoded),2000)}))

def unpack_record(record):
    if record.get('direction')!='compressed_records':return [record]
    payload=record['value'];assert payload['format']==FORMAT
    assert 0<payload['records']<=BLOCK_RECORDS and 0<payload['raw_bytes']<=MAX_MESSAGE+65536
    pieces=payload['pieces'];assert set(pieces)=={str(i) for i in range(len(pieces))}
    assert all(len(part)<=2000 for part in pieces.values())
    encoded=''.join(pieces[str(i)] for i in range(len(pieces)));compressed=base64.b64decode(encoded,validate=True)
    decoder=zlib.decompressobj();raw=decoder.decompress(compressed,payload['raw_bytes']+1)
    assert decoder.eof and not decoder.unconsumed_tail and not decoder.unused_data
    assert len(raw)==payload['raw_bytes'] and hashlib.sha256(raw).hexdigest()==payload['sha256']
    records=[json.loads(line) for line in raw.splitlines()];assert len(records)==payload['records']
    return records

def read_records(path):
    with Path(path).open() as stream:
        for line in stream:yield from unpack_record(json.loads(line))

class CompressedPortClient(PortClient):
    """Recording only: actual RPC results, engine and hard I/O bounds unchanged."""
    def __init__(self,*args,**kwargs):
        self._records=[];self._record_bytes=0
        super().__init__(*args,**kwargs)
    def _record(self,value):
        raw=(json.dumps(value,separators=(',',':'))+'\n').encode()
        if self._records and self._record_bytes+len(raw)>BLOCK_BYTES:self.flush_recording()
        self._records.append(raw);self._record_bytes+=len(raw)
        if self._record_bytes>=BLOCK_BYTES or len(self._records)>=BLOCK_RECORDS:self.flush_recording()
    def flush_recording(self):
        if not self._records:return
        raw=b''.join(self._records);count=len(self._records)
        self._records=[];self._record_bytes=0
        # Parent _record still enforces the original8MiB log budget. Pieces
        # are <=2000 chars so its normal large-value hash does not erase data.
        super()._record(pack_records(raw,count))
    def close(self):
        try:
            if getattr(self,'log',None) is not None and not self.log.closed:self.flush_recording()
        finally:super().close()

if __name__=='__main__':
    import argparse
    parser=argparse.ArgumentParser();parser.add_argument('input');parser.add_argument('--output',required=True);args=parser.parse_args()
    with Path(args.output).open('x') as output:
        for record in read_records(args.input):output.write(json.dumps(record,separators=(',',':'))+'\n')
