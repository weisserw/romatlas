#!/usr/bin/env python3
"""Exercise an explicitly supplied local port's debug adapter, not ROM fidelity."""
import argparse
import json
from pathlib import Path
import select
import subprocess
import time
import tomllib


def exercise(work):
    meta=json.loads((work/'workspace.json').read_text())
    cargo=tomllib.loads((work/'port/Cargo.toml').read_text())
    binary=work/'port/target/debug'/cargo['package']['name']
    rom=work/meta.get('rom','input/game.rom')
    process=subprocess.Popen([str(binary),'--rom',str(rom),'--debug-stdio'],
        stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
    buffer=bytearray();serial=0
    def rpc(method,params=None,ok=True):
        nonlocal serial
        serial+=1
        process.stdin.write((json.dumps(dict(jsonrpc='2.0',id=serial,method=method,params=params or {}))+'\n').encode());process.stdin.flush()
        deadline=time.monotonic()+30
        while b'\n' not in buffer:
            remaining=deadline-time.monotonic()
            if remaining<=0 or not select.select([process.stdout],[],[],remaining)[0]:
                raise AssertionError(f'timeout waiting for {method}')
            block=process.stdout.read1(65536)
            if not block: raise AssertionError(f'port exited during {method}')
            buffer.extend(block)
            assert len(buffer)<=16*1024*1024
        line,_,rest=buffer.partition(b'\n');buffer[:]=rest
        reply=json.loads(line);assert reply['id']==serial
        assert ('error' not in reply)==ok,reply
        return reply.get('result',reply.get('error'))
    try:
        info=rpc('api.describe');assert info['protocol']=='romatlas-native-debug-v1'
        before=rpc('port.state');assert before['ticks']==0
        rpc('port.input',dict(actions=['not-a-real-action']),False)
        assert rpc('port.state')==before
        action='start' if 'start' in info['actions'] else next(iter(info['actions']))
        rpc('port.input',dict(actions=[action]));rpc('port.advance',dict(ticks=2,timeout_ms=10000))
        rpc('port.input',dict(actions=[]));rpc('port.advance',dict(ticks=5,timeout_ms=10000))
        checkpoint=rpc('port.checkpoint.export')
        assert checkpoint['segments']
        rpc('port.advance',dict(ticks=3,timeout_ms=10000));expected=rpc('port.state')
        expected_frame=rpc('port.framebuffer')['sha256']
        rpc('port.checkpoint.import',dict(checkpoint=checkpoint,timeout_ms=20000))
        rpc('port.advance',dict(ticks=3,timeout_ms=10000))
        assert rpc('port.state')==expected
        assert rpc('port.framebuffer')['sha256']==expected_frame
        bad=dict(checkpoint,identity='foreign-build')
        rpc('port.checkpoint.import',dict(checkpoint=bad),False)
        assert rpc('port.state')==expected
        audio=rpc('port.audio',dict(limit=128));assert audio['channels']==2 and audio['frames']>0
        rpc('port.reset');assert rpc('port.state')==before
        print(json.dumps(dict(ok=True,workspace=str(work),protocol=info['protocol'],checkpoint_replay=True)))
    finally:
        process.stdin.close()
        try: process.wait(timeout=5)
        except subprocess.TimeoutExpired:
            process.kill();process.wait()
        if process.returncode not in (0,None):
            raise AssertionError(process.stderr.read().decode())


if __name__=='__main__':
    parser=argparse.ArgumentParser(description=__doc__);parser.add_argument('workspace',type=Path)
    exercise(parser.parse_args().workspace.resolve())
