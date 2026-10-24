"""Bounded native/reference execution and durable identity for whole-game QA.

Reference RAM reads use labels from the frozen symbols, only in this adapter;
no native RAM mirror or state edit is provided. Inputs are complete held sets.
"""
from contextlib import ExitStack
import os
import hashlib
import json
from pathlib import Path
import re
import sys
import wave
import struct

ROOT = Path(os.environ['ROMATLAS_GAME_WORKSPACE']).resolve()
TOOLKIT = Path(__file__).resolve().parents[4]
sys.path.insert(0, str(TOOLKIT / 'scripts'))
from native_debug_client import PortClient

NATIVE_BINARY = ROOT / 'port/target/debug/ducktales-port'
REFERENCE_BINARY = TOOLKIT / 'target/release/romatlas'
ROM = ROOT / 'input/game.rom'
BUTTONS = ('a','b','start','select','up','down','left','right')
MARKER_X = (0x20, 0x50, 0x90)
# Source D78F JSR YieldOneFrame saves return D791 before JMP land input.
LAND_FRAME_YIELD_RETURN = 0xD791
# FinishGameplayFrame DC26 JSR YieldOneFrame saves DC28.
GAMEPLAY_FRAME_YIELD_RETURN = 0xDC28
SYMBOLS = {name: int(address,16) for name,address in re.findall(
    r'^(\w+) = \$([0-9a-fA-F]+)$',
    (ROOT/'reference/source/symbols.inc').read_text(), re.M)}

def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()

def save(path, value):
    Path(path).write_text(json.dumps(value,indent=2)+'\n')

def source_identity():
    digest=hashlib.sha256()
    for path in sorted((ROOT/'port/src').rglob('*')):
        if path.is_file():
            digest.update(str(path.relative_to(ROOT)).encode()); digest.update(path.read_bytes())
    return digest.hexdigest()

def identity(assets=None):
    overrides = {} if assets is None else {str(p.relative_to(ROOT/assets)):sha(p) for p in sorted((ROOT/assets).rglob('*')) if p.is_file()}
    return dict(overrides=overrides,rom=sha(ROM),executable=sha(NATIVE_BINARY),reference_executable=sha(REFERENCE_BINARY),
                source=source_identity(),frozen_source=sha(ROOT/'reference/source/game.asm'),
                scripts={str(p.relative_to(ROOT)):sha(p) for p in sorted((ROOT/'debug/scenarios').glob('*.py'))})

class CompactPortClient(PortClient):
    """Hash repeated full summaries in transport logs, not actual RPC results.

    Full selected snapshots/trajectories/events are saved separately by checks.
    The inherited process, message, stderr and8MiB logging bounds remain intact.
    """
    def _record(self,value):
        if value.get('direction')=='response':
            reply=value.get('value',{})
            result=reply.get('result')
            if isinstance(result,dict) and isinstance(result.get('game'),dict):
                game=result['game']
                raw=json.dumps(game,sort_keys=True,separators=(',',':')).encode()
                events=game.get('events',{})
                summary=dict(omitted=True,json_bytes=len(raw),sha256=hashlib.sha256(raw).hexdigest(),
                             ticks=game.get('ticks'),mode=game.get('mode'),
                             event_next_sequence=events.get('next_sequence'),event_dropped=events.get('dropped'))
                # Copy each replaced container; mutating reply would alter the
                # actual state returned to assertions, defeating independent QA.
                value=dict(value,value=dict(reply,result=dict(result,game=summary)))
        super()._record(value)


class Pair:
    def __init__(self, evidence, assets=None, *, reference_request_limit=20000, compressed_reference=False):
        self.evidence=Path(evidence)
        self.stack=ExitStack()
        command=[str(NATIVE_BINARY),'--rom',str(ROM),'--debug-stdio']
        if assets: command += ['--assets',str(ROOT/assets)]
        self.native=self.stack.enter_context(CompactPortClient(command,seconds=900,max_requests=20000,evidence=self.evidence/'native'))
        if compressed_reference:
            from transport_codec import CompressedPortClient
            reference_client=CompressedPortClient
        else:reference_client=CompactPortClient
        self.reference=self.stack.enter_context(reference_client([str(REFERENCE_BINARY),'serve','--platform','nes','--rom',str(ROM)],seconds=900,max_requests=reference_request_limit,evidence=self.evidence/'reference'))
        save(self.evidence/'reference-client-bounds.json',dict(seconds=900,max_requests=reference_request_limit,log_bytes=8*1024*1024,message_bytes=16*1024*1024,codec='jsonl-zlib-base64-v1' if compressed_reference else 'existing-compact-jsonl'))
        self.reference.request('emulator.start')
        self.reference_held=set()
        self.steps=[]
        self.native.request('port.input',dict(player=0,actions=[]))
        self.native.request('port.input',dict(player=1,actions=[]))
        for port in (1,2):
            self.reference.request('emulator.input',dict(port=port,buttons=[]))
        self.native_description=self.native.request('api.describe')
        self.reference_description=self.reference.request('api.describe')
        save(self.evidence/'api-identities.json',dict(native=self.native_description,reference=self.reference_description))
    def __enter__(self): return self
    def __exit__(self,*args):
        save(self.evidence/'inputs.json',self.steps)
        if hasattr(self,'observations'):save(self.evidence/'observations-partial.json',self.observations)
        if args[0] is not None:
            try: self.capture('failure')
            except BaseException as error: save(self.evidence/'failure-capture-error.json',dict(error=str(error)))
        return self.stack.__exit__(*args)
    def game(self):
        state=self.native.request('port.state')
        game=state.get('game')
        if game is None: raise AssertionError(f'missing canonical game summary: {list(state)}')
        assert game['schema_version']==2 and game['implementation']=='native_integration'
        return game
    def memory(self,name,count=1,offset=0):
        return self.reference.request('emulator.memory',dict(address=SYMBOLS[name]+offset,length=count))['bytes']
    def input(self,actions,side='both'):
        actions=set(actions)
        assert actions <= set(BUTTONS)
        if side in ('native','both'):
            self.native.request('port.input',dict(actions=sorted(actions)))
        if side in ('reference','both'):
            self.reference.request('emulator.input',dict(port=1,buttons=sorted(actions)))
            self.reference_held=actions
        self.steps.append(dict(input=sorted(actions),side=side))
    def advance(self,frames,side='both'):
        assert 0 < frames <= 600
        if side in ('native','both'):
            result=self.native.request('port.advance',dict(ticks=frames,timeout_ms=300000))
            assert result['completed']==frames and result['remaining']==0 and result['complete'] is True,result
        if side in ('reference','both'):
            before=self.reference.request('emulator.state')['frames']
            after=self.reference.request('emulator.step',dict(unit='frame',count=frames,max_operations=100000000,timeout_ms=300000))
            assert after['frames']-before==frames,after
        self.steps.append(dict(frames=frames,side=side))
    def edge(self,actions,frames=1,side='both'):
        self.input([],side);self.advance(1,side)
        self.input(actions,side);self.advance(frames,side)
        self.input([],side);self.advance(1,side)
    def capture(self,name):
        game=self.game(); save(self.evidence/f'{name}-native-state.json',game)
        self.native.frame(f'{name}.ppm')
        self.reference.request('emulator.framebuffer',dict(path=str((self.evidence/'reference'/f'{name}.ppm').resolve())))
        source_state=self.reference.request('emulator.state')
        save(self.evidence/f'{name}-reference-state.json',dict(execution=source_state,
            title_option=self.memory('TitleOption')[0],stage=self.memory('StageIndex')[0],
            player_state=self.memory('PlayerState')[0],progress=self.memory('StageProgressBits')[0],
            row=self.memory('CurrentRoomRow')[0],terrain_collision=self.memory('TerrainCollisionCache',128),
            terrain_attributes=self.memory('TerrainAttributeCache',128),composition=self.memory('CompositionTick')[0],
            actor_animation=self.memory('ObjectAnimation',16),actor_frames=self.memory('ObjectAnimationFrame',16),actor_ticks=self.memory('ObjectAnimationTick',16),
            oam=self.reference.request('emulator.physical_memory',dict(space='oam',offset=0,length=256))['bytes'],audio_channels=self.memory('AudioChannels',31*4),actor_flags=self.memory('ObjectStateFlags',16),
            actor_x=self.memory('ObjectWorldX',16),actor_pages=self.memory('ObjectWorldXHi',16),
            actor_y=self.memory('ObjectScreenY',16)))
        save(self.evidence/'inputs.json',self.steps)
        return game
    def begin_audio(self,label):
        self.native.request('port.audio.clear')
        self.reference.request('emulator.audio.clear')
        self.steps.append(dict(audio_interval_start=label))

    def audio(self,name,seconds=2):
        # Save a bounded selected tail in PCM16. Ring drops are explicit metadata;
        # callers checking an entire interval clear before that interval instead.
        metadata=self.native.request('port.audio',dict(offset=0,limit=0))
        assert metadata['dropped_frames']==0,'required native audio interval overflowed; clear/recreate interval before capture'
        total=metadata['frames']
        begin=max(0,total-int(seconds*metadata['sample_rate']))
        samples=[]
        for offset in range(begin,total,4096):
            page=self.native.request('port.audio',dict(offset=offset,limit=min(4096,total-offset)))
            samples.extend(page['samples'])
        assert len(samples)==(total-begin)*metadata['channels']
        with wave.open(str(self.evidence/'native'/f'{name}.wav'),'wb') as wav:
            wav.setnchannels(metadata['channels']);wav.setsampwidth(2);wav.setframerate(metadata['sample_rate'])
            wav.writeframes(struct.pack('<'+'h'*len(samples),*[round(max(-1,min(1,x))*32767) for x in samples]))
        save(self.evidence/f'{name}-audio-metadata.json',dict(**metadata,selected_start=begin,selected_frames=total-begin))
        original=self.reference.request('emulator.audio.export',dict(path=str((self.evidence/'reference'/f'{name}.wav').resolve())))
        save(self.evidence/f'{name}-reference-audio-metadata.json',original)
        assert original['dropped_stereo_frames']==0,'required original audio interval overflowed'
        return dict(**metadata,selected_start=begin,selected_frames=total-begin,
                    peak=max((abs(x) for x in samples),default=0))
    def audio_sequence(self,frames=60,max_alignment=64):
        def native_notes():
            return [dict(cursor=c['primary']['cursor'],duration=c['primary']['duration'],
                         period=c['primary']['period'],control=c['primary']['control'])
                    for c in self.game()['audio']['channels']]
        def reference_notes():
            raw=self.memory('AudioChannels',31*4)
            return [dict(cursor=raw[i]+(raw[i+1]<<8),duration=raw[i+2]+(raw[i+3]<<8),
                         period=raw[i+10]+(raw[i+11]<<8),control=raw[i+12])
                    for i in range(0,len(raw),31)]
        # Hardware bulk CHR copies can consume original interrupts; native omits
        # that hardware lag. Align actual note state, not title host frame index.
        original=reference_notes()
        for offset in range(max_alignment+1):
            if native_notes()==original: break
            self.advance(1,'native')
        else: raise AssertionError(dict(reason='cannot align source note sequence',native=native_notes(),reference=original))
        observations=[]
        self.native.request('port.audio.clear')
        self.reference.request('emulator.audio.clear')
        for frame in range(frames):
            self.advance(1)
            native=native_notes();original=reference_notes()
            assert native==original,dict(frame=frame,native=native,reference=original)
            observations.append(dict(frame=frame,notes=native))
        target=self.evidence/'source-audio-sequence.json'
        suffix=1
        while target.exists():
            target=self.evidence/f'source-audio-sequence-{suffix}.json';suffix+=1
        save(target,dict(native_alignment_ticks=offset,observations=observations))
        return dict(alignment_ticks=offset,compared_frames=frames)
    def title(self):
        # Independent semantic synchronization: native has no hardware warm-up,
        # whereas the original includes reset/vblank waits. Do not align frame0.
        for _ in range(100):
            if self.game()['transition']['menu']=='Title(Input)': break
            self.advance(1,'native')
        else: raise AssertionError('native title input inaccessible')
        for _ in range(180):
            if self.memory('TitleOption')[0]==1 and self.memory('ObjectStateFlags',2)==[128,128]:
                break
            self.advance(1,'reference')
        else: raise AssertionError('original title input inaccessible')
        self.input([])
        self.advance(1)
        assert self.game()['actors'][1]['position']['world_x']==MARKER_X[1]
        assert self.memory('ObjectWorldX',offset=1)[0]==MARKER_X[1]
    def reference_yield_return(self):
        # Saved task stack is reference-only provenance. YieldOneFrame pushes X/Y
        # above its JSR return; no native CPU/stack state is reconstructed.
        saved=self.memory('SchedulerSavedStackOrEntry')[0]
        stack=self.reference.request('emulator.memory',dict(address=0x100+saved+1,length=4))['bytes']
        return stack[2]+(stack[3]<<8)
    def lands(self):
        self.title();self.edge(['start'])
        for _ in range(160):
            if self.game()['transition']['menu']=='LandInput': break
            self.advance(1,'native')
        else: raise AssertionError('native land selection inaccessible')
        for _ in range(160):
            if (self.memory('ObjectAnimation',offset=2)[0]==0x6F
                and self.reference_yield_return()==LAND_FRAME_YIELD_RETURN): break
            self.advance(1,'reference')
        else: raise AssertionError('original land selection inaccessible')
        assert self.game()['progression']['stage']==0
        assert self.memory('StageIndex')[0]==0

    def stage_entry(self,stage=0):
        self.lands()
        for selected in range(stage):
            self.edge(['down'])
            native=self.game()['progression']['stage']
            original=self.memory('StageIndex')[0]
            assert native==original==selected+1,dict(native=native,original=original,expected=selected+1)
        self.edge(['a'])
        for _ in range(300):
            game=self.game()
            if game['mode']=='gameplay' and game['transition']['foreground']=='Yield': break
            self.advance(1,'native')
        else: raise AssertionError(dict(reason='native initial gameplay frame inaccessible',game=game))
        for _ in range(300):
            if (self.memory('GameplayDisplayMode')[0]==0x40
                and self.reference_yield_return()==GAMEPLAY_FRAME_YIELD_RETURN): break
            self.advance(1,'reference')
        else: raise AssertionError('original initial gameplay frame inaccessible')
        self.input([])
        return self.game()

    def player_facts(self,game=None):
        if game is None:game=self.game()
        actor=game['actors'][0]
        native=dict(action=game['player']['selector'],world_x=actor['position']['world_x'],
                    screen_y=actor['position']['screen_y'],x_fraction=actor['position']['x_fraction'],
                    y_fraction=actor['position']['y_fraction'],velocity=actor['vertical_velocity'])
        # Independent source RAM fields are scattered across the actor arrays.
        # Read their bounded contiguous span once instead of eight RPCs; no
        # native values participate in decoding the original machine state.
        base=SYMBOLS['ObjectScreenY']
        last=SYMBOLS['ObjectVelocityYInteger']
        raw=self.reference.request('emulator.memory',dict(address=base,length=last-base+1))['bytes']
        def field(name):return raw[SYMBOLS[name]-base]
        velocity=field('ObjectVelocityYFraction')+256*field('ObjectVelocityYInteger')
        if velocity>=32768:velocity-=65536
        reference=dict(action=self.memory('PlayerState')[0],
                       world_x=field('ObjectWorldX')+256*field('ObjectWorldXHi'),
                       screen_y=field('ObjectScreenY'),x_fraction=field('ObjectXFraction'),
                       y_fraction=field('ObjectYFraction'),velocity=velocity)
        return native,reference

    def compare_player(self,label,game=None):
        native,reference=self.player_facts(game)
        assert native==reference,dict(label=label,native=native,reference=reference)
        return dict(label=label,player=native)

    def standing(self):
        self.input([])
        for _ in range(120):
            game=self.game()
            if game['player']['action']=='standing' and game['actors'][0]['vertical_velocity']==0:break
            self.advance(1,'native')
        else:raise AssertionError('native initial floor standing inaccessible')
        for _ in range(120):
            if (self.memory('PlayerState')[0]==0 and self.memory('ObjectVelocityYFraction')[0]==0
                and self.memory('ObjectVelocityYInteger')[0]==0):break
            self.advance(1,'reference')
        else:raise AssertionError('original initial floor standing inaccessible')
        return self.compare_player('standing')

    def compare_terrain(self,label):
        terrain=self.game()['world']['terrain']
        for field,symbol in [('collision','TerrainCollisionCache'),('attributes','TerrainAttributeCache')]:
            original=self.memory(symbol,128)
            actual=terrain[field]
            assert actual==original,dict(label=label,field=field,
                differences=[dict(index=i,native=a,original=b) for i,(a,b) in enumerate(zip(actual,original)) if a!=b])
        return dict(label=label,collision_cells=128,attribute_cells=128)
