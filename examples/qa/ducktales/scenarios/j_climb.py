"""Natural Moon rope and actual cooperative sixty-phase vertical transition."""
from common import ROOT,save,identity,sha,SYMBOLS
from g_routes import finish_source_frame
import json
SETUP=ROOT/'debug/artifacts/pass-1/G-breakable-reward/attempt-009'


def finish_vertical_owner(pair,game,label,prior_irq):
    call=game['calls']['vertical'];wait=game['calls']['wait'];assert wait and wait['remaining']==1
    if call is None:
        assert wait['owner']=='ForegroundYield';expected_return=0xDC28
    else:
        assert wait['owner']=='VerticalYield'
        expected_return=0xC99C if call['counter']==0 else 0xCA08
    target=game['timers']['composition'];phases=[]
    if 'ne' in getattr(pair, 'reference_description', {}).get('conditional_operators', []):
        queries=[dict(name=name,method='emulator.memory',params=dict(address=SYMBOLS[symbol],length=1))
                 for name,symbol in [('composition','CompositionTick'),('task','SchedulerSlots'),('irq','InterruptTick')]]
        queries.append(dict(name='caller',method='emulator.memory',params=dict(length=2),
                            indirect=dict(pointer_address=SYMBOLS['SchedulerSavedStackOrEntry'],base=256,offset=3)))
        conditions=[dict(query=name,pointer='/bytes',value=value) for name,value in
                    [('composition',[target]),('task',[1]),('caller',[expected_return&255,expected_return>>8])]]
        conditions.append(dict(query='irq',pointer='/bytes/0',value=prior_irq,op='ne'))
        result=pair.reference.request('emulator.run_until',dict(queries=queries,conditions=conditions,
                                                              max_operations=32768,timeout_ms=30000))
        save(pair.evidence/f'{label}-source-owner.json',dict(observer='server-until',native_wait=wait,
             native_vertical=call,expected_return=expected_return,target_composition=target,**result))
        assert result['matched'],dict(reason='vertical owner inaccessible',label=label,result=result)
        observed=result['observations']
        phase=dict(composition=observed['composition']['bytes'][0],irq=observed['irq']['bytes'][0],
                   task=observed['task']['bytes'][0],caller=sum(b<<(8*i) for i,b in enumerate(observed['caller']['bytes'])))
        pair.steps.append(dict(source_operations=result['operations'],reason=label,observer='vertical-server-until'))
        return phase
    for batch in range(513):
        base=SYMBOLS['SchedulerSlots'];raw=pair.reference.request('emulator.memory',dict(address=base,length=SYMBOLS['CompositionTick']-base+1))['bytes']
        comp=raw[SYMBOLS['CompositionTick']-base];irq=raw[SYMBOLS['InterruptTick']-base];task=raw[0];caller=None
        if task==1:
            saved=raw[SYMBOLS['SchedulerSavedStackOrEntry']-base]
            stack=pair.reference.request('emulator.memory',dict(address=0x100+saved+1,length=4))['bytes'];caller=stack[2]+256*stack[3]
        phases.append(dict(composition=comp,irq=irq,task=task,caller=caller))
        if irq!=prior_irq and task==1 and caller==expected_return and comp==target:
            save(pair.evidence/f'{label}-source-owner.json',dict(native_wait=wait,native_vertical=call,expected_return=expected_return,target_composition=target,operations=batch*64,phases=phases))
            return phases[-1]
        assert comp in [target,(target-1)&255],dict(label=label,target=target,actual=phases[-1])
        if batch==512:raise AssertionError(dict(reason='vertical owner inaccessible',label=label,phases=phases))
        state=pair.reference.request('emulator.step',dict(unit='operation',count=64,max_operations=100000,timeout_ms=300000))
        pair.steps.append(dict(source_operations=64,source_frames=state['frames'],source_cycles=state['cpu_cycles'],reason=label))


def restore_gap(pair):
    metadata=json.load(open(SETUP/'gap-checkpoint-preconditions.json'));current=identity()
    for key in ['rom','executable','reference_executable','source','frozen_source','overrides']:assert current[key]==metadata['provenance'][key],key
    checkpoint=json.load(open(SETUP/'gap-checkpoint.json'));pair.native.request('port.checkpoint.import',dict(checkpoint=checkpoint,timeout_ms=300000));pair.reference.request('emulator.snapshot.import',dict(path=str((SETUP/'gap-reference-snapshot.json').resolve())))
    game=pair.game();assert game==metadata['native'] and pair.reference_yield_return()==metadata['source_yield']==0xDC28
    pair.compare_player('validated-gap',game);pair.compare_terrain('validated-gap')
    save(pair.evidence/'gap-restore-validation.json',dict(native_checkpoint=sha(SETUP/'gap-checkpoint.json'),source_snapshot=sha(SETUP/'gap-reference-snapshot.json'),native_identity=checkpoint['identity'],native=game,source_yield=0xDC28,provenance=current))
    return game


def save_pair_checkpoint(pair,name):
    save(pair.evidence/f'{name}-checkpoint.json',pair.native.request('port.checkpoint.export'))
    pair.reference.request('emulator.snapshot.export',dict(path=str((pair.evidence/f'{name}-reference-snapshot.json').resolve())))
    save(pair.evidence/f'{name}-preconditions.json',dict(native=pair.game(),source_yield=pair.reference_yield_return(),provenance=identity()))


def world_facts(pair,game,label):
    base=SYMBOLS['PaletteUploadPending'];raw=pair.reference.request('emulator.memory',dict(address=base,length=SYMBOLS['CameraXHi']-base+1))['bytes']
    def field(name):return raw[SYMBOLS[name]-base]
    source=dict(row=field('CurrentRoomRow'),camera_x=field('CameraX')+256*field('CameraXHi'),camera_room=field('CameraRoomIndex'),terrain_room=field('TerrainRoomCursor'),room_in_row=field('TerrainRoomInRow'),ring=field('TerrainColumnCursor'),split=field('SplitPpuLow')+256*field('SplitPpuHigh'),split_control=field('SplitTransitionControl'))
    native=dict(row=game['progression']['room_row'],camera_x=game['world']['camera']['x'],camera_room=game['world']['camera']['room'],terrain_room=game['world']['stream']['room'],room_in_row=game['world']['stream']['room_in_row'],ring=game['world']['stream']['ring'],split=game['transition']['split_address'],split_control=game['transition']['split_control'])
    assert native==source,dict(label=label,native=native,source=source)
    flags=pair.memory('ObjectStateFlags',16);ys=pair.memory('ObjectScreenY',16)
    for a in game['actors']:
        assert bool(flags[a['slot']]&0x80)==a['active'],dict(label=label,slot=a['slot'],source_flags=flags[a['slot']],native=a)
        if a['active']:assert ys[a['slot']]==a['position']['screen_y'],dict(label=label,slot=a['slot'])
    pair.compare_player(label,game);pair.compare_terrain(label)
    return dict(label=label,native=native,source=source,source_counter=field('VerticalTransitionCounter'),source_direction=field('VerticalTransitionDirection'),palette_pending=dict(native=game['presentation']['palette_pending'],source=field('PaletteUploadPending')),spawn=dict(native=game['world']['spawn'],source_lower=field('SpawnLowerCursor'),source_upper=field('SpawnUpperCursor'),source_special=field('SpecialSpawnControl')),source_actor_y=ys,source_flags=flags)


def natural_rope(pair,points):
    game=restore_gap(pair);lives=game['status']['lives'];up=False
    pair.input(['a','right'])
    for frame in range(450):
        if frame==8:pair.input(['down','b','right'])
        if not up and game['actors'][0]['position']['world_x']>=884:pair.input(['up','right']);up=True;pair.capture('rope-approach')
        pair.advance(1);game=pair.game();assert game['calls']['vertical'] is None,'entry route must expose climbing before crossing its threshold'
        finish_source_frame(pair,pair.evidence,game,f'rope-route-{frame}');points.append(pair.compare_player(f'rope-route-{frame}',game))
        assert game['mode']=='gameplay' and game['player']['selector']!=13 and game['status']['lives']==lives
        if frame%80==0:pair.capture(f'rope-route-{frame}')
        if game['player']['selector']==3:
            assert game['actors'][0]['position']['world_x']==896 and game['world']['camera']['x']==768
            pair.capture('actual-climb-entered');world_facts(pair,game,'actual-climb-entered')
            save_pair_checkpoint(pair,'climb-entry');return game
        assert game['actors'][0]['position']['world_x']<944
    raise AssertionError('proven bounded route did not reach actual rope')


def climb_vertical(pair, *, prepared=None):
    evidence=pair.evidence;points=[] if prepared is None else prepared[1];pair.observations=points;game=natural_rope(pair,points) if prepared is None else prepared[0];movement=[]
    def ordinary_step(label):
        pair.advance(1);g=pair.game()
        assert g['calls']['vertical'] is None,'short climb movement must stay on this rope before transition'
        finish_source_frame(pair,evidence,g,label);points.append(pair.compare_player(label,g));assert g['player']['selector']==3
        movement.append(dict(label=label,player=points[-1],native=g['actors'][0],camera=g['world']['camera']))
        return g
    pair.input(['up']);start=game['actors'][0]['position']['screen_y']
    for frame in range(16):game=ordinary_step(f'climb-up-{frame}')
    assert game['actors'][0]['position']['screen_y']==start-24
    pair.capture('climb-up');up_y=game['actors'][0]['position']['screen_y']
    pair.input(['down'])
    for frame in range(4):game=ordinary_step(f'climb-down-{frame}')
    assert game['actors'][0]['position']['screen_y']==up_y+6
    pair.capture('climb-down');save(evidence/'climb-movement.json',movement)
    pair.input(['up']);before=None
    for frame in range(50):
        if before is None and game['actors'][0]['position']['screen_y']<=60:
            before=game;pair.capture('before-vertical');save_pair_checkpoint(pair,'before-vertical')
        prior_irq=pair.memory('InterruptTick')[0];pair.advance(1);game=pair.game()
        if game['calls']['vertical'] is not None:
            finish_vertical_owner(pair,game,'vertical-entry',prior_irq);break
        finish_source_frame(pair,evidence,game,f'climb-threshold-{frame}');points.append(pair.compare_player(f'climb-threshold-{frame}',game));assert game['player']['selector']==3
    else:raise AssertionError('actual upward threshold did not enter legal vertical call')
    assert before is not None
    call=game['calls']['vertical'];assert call['direction']=='Up' and call['counter']==0
    entry=world_facts(pair,game,'vertical-entry');save(evidence/'vertical-entry-facts.json',entry)
    assert game['progression']['room_row']==before['progression']['room_row']-1==3
    assert game['world']['camera']['room']==before['world']['camera']['room']-8
    assert game['actors'][0]['position']['screen_y']<0x39 and game['actors'][0]['position']['screen_y']!=0x32
    assert game['actors'][2]['position']['screen_y']==entry['source_actor_y'][2]==0x32,'up E8C2 writes slot2, not player'
    assert game['transition']['split_control']==0x0C and entry['source_counter']==0
    pair.capture('vertical-entry');phases=[];entry_composition=game['timers']['composition'];saved=call['saved_cursor']
    for step in range(70):
        prior_irq=pair.memory('InterruptTick')[0];pair.advance(1);game=pair.game();finish_vertical_owner(pair,game,f'vertical-{step}',prior_irq)
        facts=world_facts(pair,game,f'vertical-{step}');call=game['calls']['vertical']
        phases.append(dict(step=step,tick=game['ticks'],call=call,facts=facts,composition=game['timers']['composition'],player=pair.player_facts(game)))
        save(evidence/'vertical-phases.json',phases)
        if call is None:break
        # Native issues a phase and increments its count before its Yield;
        # original INC runs after that Yield resumes:1..60 corresponds0..59.
        assert call['counter']==step+1 and facts['source_counter']==call['counter']-1
        assert game['transition']['split_control']==0x0C
        if call['counter'] in [1,12,13,31,59,60]:pair.capture(f'vertical-phase-{call["counter"]}')
    else:raise AssertionError('sixty-phase vertical call did not rejoin gameplay')
    assert len(phases)==61 and phases[-1]['facts']['source_counter']==60
    assert game['transition']['foreground']=='Yield' and game['transition']['split_control']==0
    assert (game['timers']['composition']-entry_composition)&255==60
    assert game['world']['stream']['ring']==saved['ring'] and game['world']['stream']['room_in_row']==saved['room_in_row']
    spawn=phases[-1]['facts']['spawn'];assert game['world']['spawn']['lower']==spawn['source_lower'] and game['world']['spawn']['upper']==spawn['source_upper']
    assert game['presentation']['palette_pending']
    pair.capture('vertical-rejoined');pair.input(['up'])
    for frame in range(4):
        pair.advance(1);game=pair.game();finish_source_frame(pair,evidence,game,f'upper-room-{frame}');points.append(pair.compare_player(f'upper-room-{frame}',game))
        assert game['progression']['room_row']==3 and game['calls']['vertical'] is None
    world_facts(pair,game,'upper-room-established');pair.capture('upper-room-established')
    save(evidence/'climb-vertical-trajectory.json',points)
    return dict(climb_up_ticks=16,climb_down_ticks=4,source_indexed_up_slot=2,vertical_motion_phases=60,entry_yield=True,room_before=4,room_after=3,gameplay_rejoined=True)
