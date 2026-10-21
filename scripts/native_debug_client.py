"""Bounded persistent native-port client for agent-authored scenarios (Unix host)."""
import json
import os
from pathlib import Path
import select
import signal
import subprocess
import time

MAX_MESSAGE = 16 * 1024 * 1024


class PortClient:
    def __init__(self, command, *, seconds, max_requests, evidence):
        if not 0 < seconds <= 86400 or not 0 < max_requests <= 100000:
            raise ValueError('declare positive bounded seconds and max_requests')
        self.evidence = Path(evidence)
        self.evidence.mkdir(parents=True, exist_ok=False)
        self.log = (self.evidence/'transport.jsonl').open('x')
        self.errors = (self.evidence/'stderr.log').open('xb')
        try:
            self.process = subprocess.Popen(command, stdin=subprocess.PIPE,
                stdout=subprocess.PIPE, stderr=self.errors, start_new_session=True)
        except BaseException:
            self.errors.close();self.log.close();raise
        self.deadline = time.monotonic()+seconds
        self.max_requests = max_requests
        self.serial = 0
        self.buffer = bytearray()
        self.log_bytes = 0

    def __enter__(self):
        return self

    def __exit__(self, *_):
        self.close()

    def _record(self, value):
        text=json.dumps(value,separators=(',', ':'))+'\n'
        self.log_bytes += len(text.encode())
        if self.log_bytes > 128*1024*1024:
            raise RuntimeError('transport evidence reached 128 MiB budget')
        self.log.write(text);self.log.flush()

    def request(self, method, params=None):
        try:
            if self.serial >= self.max_requests or time.monotonic() >= self.deadline:
                raise TimeoutError('declared native experiment budget exhausted')
            self.serial += 1
            request=dict(jsonrpc='2.0',id=self.serial,method=method,params=params or {})
            raw=(json.dumps(request)+'\n').encode()
            if len(raw)>MAX_MESSAGE:
                raise ValueError('request exceeds 16 MiB')
            self._record(dict(direction='request',value=request))
            # Bounded nonblocking writes prevent an unresponsive child from
            # defeating the deadline with a large checkpoint import.
            fd=self.process.stdin.fileno();os.set_blocking(fd,False)
            sent=0
            while sent<len(raw):
                remaining=self.deadline-time.monotonic()
                if remaining<=0 or not select.select([], [fd], [], remaining)[1]:
                    raise TimeoutError('native process did not consume request')
                try: sent+=os.write(fd,raw[sent:sent+65536])
                except BlockingIOError: continue
            while b'\n' not in self.buffer:
                remaining=self.deadline-time.monotonic()
                if remaining<=0 or not select.select([self.process.stdout],[],[],remaining)[0]:
                    raise TimeoutError(f'native process exceeded deadline during {method}')
                block=os.read(self.process.stdout.fileno(),65536)
                if not block: raise RuntimeError(f'native process exited during {method}; see stderr.log')
                self.buffer.extend(block)
                if len(self.buffer)>MAX_MESSAGE: raise ValueError('response exceeds 16 MiB')
            line,_,rest=self.buffer.partition(b'\n');self.buffer[:]=rest
            reply=json.loads(line)
            if reply.get('jsonrpc')!='2.0' or reply.get('id')!=self.serial:
                raise ValueError('unexpected native response identity')
            self._record(dict(direction='response',value=reply))
            if 'error' in reply: raise RuntimeError(reply['error']['message'])
            return reply['result']
        except (TimeoutError, BrokenPipeError, ValueError):
            self.close();raise

    def frame(self, name):
        """Capture a PPM viewable by local image tools; RGBA alpha is discarded."""
        if Path(name).name!=name: raise ValueError('frame name must be a basename')
        result=self.request('port.framebuffer')
        data=bytes(result['pixels']);width=result['width'];height=result['height']
        if len(data)!=width*height*4: raise ValueError('invalid native framebuffer')
        rgb=bytearray()
        for offset in range(0,len(data),4):rgb.extend(data[offset:offset+3])
        path=self.evidence/name
        with path.open('xb') as output:output.write(f'P6\n{width} {height}\n255\n'.encode()+rgb)
        return path

    def close(self):
        if self.process.poll() is None:
            self.process.stdin.close()
            try:self.process.wait(timeout=1)
            except subprocess.TimeoutExpired:
                try:os.killpg(self.process.pid,signal.SIGKILL)
                except ProcessLookupError:pass
                self.process.wait()
        self.process.stdin.close();self.process.stdout.close();self.errors.close();self.log.close()
