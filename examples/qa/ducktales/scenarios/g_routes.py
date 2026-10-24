"""Source-backed G routes. Only ordinary gameplay uses source-owner completion.
Fresh reset/input establishes a gap replay. Terrain mutation and natural contact
health collection are distinct subscenarios; a culled reward is never collection.
"""
from common import ROOT,save,identity,SYMBOLS,sha

def finish_source_frame(pair,evidence,game,label):
    """Stop at the same source owner without transport polling or missed sleeps.

    Capability fallback preserves older debugger compatibility. Conditions retain
    the independently traced source contract (DC28, fresh delay1, composition).
    """
    if 'emulator.run_until' not in pair.reference_description.get('methods', []):
        return finish_source_frame_legacy(pair,evidence,game,label)
    assert game['mode']=='gameplay','owner completion is scoped to ordinary gameplay'
    target=game['timers']['composition']
    queries=[dict(name=name,method='emulator.memory',params=dict(address=SYMBOLS[symbol],length=1))
             for name,symbol in [('composition','CompositionTick'),('task','SchedulerSlots'),('delay','SchedulerDelay')]]
    queries.append(dict(name='caller',method='emulator.memory',params=dict(length=2),
                        indirect=dict(pointer_address=SYMBOLS['SchedulerSavedStackOrEntry'],base=256,offset=3)))
    conditions=[dict(query=name,pointer='/bytes',value=value) for name,value in
                [('composition',[target]),('task',[1]),('delay',[1]),('caller',[0x28,0xDC])]]
    result=pair.reference.request('emulator.run_until',dict(queries=queries,conditions=conditions,
                                                         max_operations=32768,timeout_ms=30000))
    save(evidence/f'{label}-source-completion.json',dict(version=4,target=target,**result))
    assert result['matched'],dict(reason='source owner setup inaccessible',label=label,result=result)
    pair.steps.append(dict(source_operations=result['operations'],reason=label,observer='server-until-v4'))
    if hasattr(pair.reference,'flush_recording'):pair.reference.flush_recording()


def finish_source_frame_legacy(pair,evidence,game,label):
    assert game['mode']=='gameplay','owner completion is scoped to ordinary gameplay'
    target=game['timers']['composition'];checkpoints=[];operations=0;pc=None
    # Source-only observer v3: a 256-operation batch can miss the short DC28
    # sleep immediately before NMI. Fine-step the fixed gameplay/Yield tail;
    # the established total CPU-operation bound remains 128*256.
    for batch in range(4097):
        raw=pair.reference.request('emulator.memory',dict(address=SYMBOLS['SchedulerSlots'],length=SYMBOLS['CompositionTick']-SYMBOLS['SchedulerSlots']+1))['bytes']
        composition=raw[SYMBOLS['CompositionTick']-SYMBOLS['SchedulerSlots']];task=raw[0];caller=None
        if task==1:
            saved=raw[SYMBOLS['SchedulerSavedStackOrEntry']-SYMBOLS['SchedulerSlots']]
            stack=pair.reference.request('emulator.memory',dict(address=0x100+saved+1,length=4))['bytes'];caller=stack[2]+256*stack[3]
        phase=dict(composition=composition,task=task,delay=raw[SYMBOLS['SchedulerDelay']-SYMBOLS['SchedulerSlots']],caller=caller,pc=pc);checkpoints.append(phase)
        if composition==target and task==1 and phase['delay']==1 and caller==0xDC28:
            if operations:save(evidence/f'{label}-source-completion.json',dict(version=3,target=target,operations=operations,checkpoints=checkpoints))
            if hasattr(pair.reference,'flush_recording'):pair.reference.flush_recording()
            return
        assert composition in [target,(target-1)&255],dict(reason='source composition advanced beyond expected actual owner',target=target,phase=phase)
        if pc is None and composition==target:pc=pair.reference.request('emulator.state')['main']['pc']
        fine=composition==target and pc is not None and (0xDBE0<=pc<=0xDC2B or 0xFFAC<=pc<=0xFFC9)
        count=1 if fine else (32 if composition==target else 256)
        assert operations+count<=32768,dict(reason='source foreground completion inaccessible',target=target,operations=operations,checkpoints=checkpoints)
        state=pair.reference.request('emulator.step',dict(unit='operation',count=count,max_operations=100000,timeout_ms=300000));operations+=count;pc=state['main']['pc']
        pair.steps.append(dict(source_operations=count,source_frames=state['frames'],source_cycles=state['cpu_cycles'],reason=label))
    raise AssertionError('bounded source owner polling inaccessible')


def source_audio_globals(pair,game):
    base=SYMBOLS['AudioPriority'];raw=pair.reference.request('emulator.memory',dict(address=base,length=SYMBOLS['AudioSharedDelay']-base+1))['bytes']
    source=dict(priority=raw[0],secondary_mask=raw[SYMBOLS['AudioChannelMask']-base],secondary_cursor=raw[SYMBOLS['AudioSecondaryCursor']-base]+256*raw[SYMBOLS['AudioSecondaryCursorHi']-base],shared_delay=raw[SYMBOLS['AudioSharedDelay']-base])
    native={k:game['audio'][k] for k in source}
    # Composition alignment need not align NMI-driven effect delay during
    # heavy uploads. Return actual facts; callers scope comparisons to an
    # independently established sound occurrence/interrupt age.
    return dict(native=native,source=source,source_interrupt_tick=pair.memory('InterruptTick')[0],native_audio_frames=game['audio']['frames'])

def health(pair,game):
    native=[]
    for segment in game['status']['health']['segments']:
        if segment['y']==255:break
        native.append(segment['depleted'])
    raw=pair.memory('OverlaySprites',20);source=[]
    for i in range(0,20,4):
        if raw[i]==255:break
        source.append(raw[i+2])
    assert native==source,dict(native=native,source=source)
    return native


def natural_gap(pair,points):
    evidence=pair.evidence
    finish=lambda game,label:finish_source_frame(pair,evidence,game,label)
    pair.stage_entry(4);points.append(pair.standing())
    # Walk through the first real damage/release; preserve the shooter.
    pair.input(['right'])
    for frame in range(80):
        pair.advance(1);game=pair.game()
        finish(game,f'approach-jump-{frame}')
        points.append(pair.compare_player(f'approach-jump-{frame}',game))
        assert game['mode']=='gameplay' and game['player']['selector']!=13
        if game['actors'][0]['position']['world_x']>=150 and game['player']['selector']==0 and game['actors'][0]['vertical_velocity']==0:break
    else:raise AssertionError('first jumping floor prerequisite inaccessible')
    pair.capture('first-jump-floor')
    for jump in range(3):
        pair.input(['right']);pair.advance(1)
        finish(pair.game(),f'jump-{jump}-release')
        save(evidence/f'jump-{jump}-released-controls.json',dict(held=pair.memory('ControllerHeld')[0],pressed=pair.memory('ControllerPressed')[0],player=pair.player_facts(),yield_return=pair.reference_yield_return()))
        previous=pair.player_facts()
        released_return=pair.reference_yield_return()
        pair.input(['a','right'])
        for frame in range(90):
            pair.advance(1);game=pair.game()
            finish(game,f'jump-{jump}-{frame}')
            if frame==0:save(evidence/f'jump-{jump}-pressed-controls.json',dict(held=pair.memory('ControllerHeld')[0],pressed=pair.memory('ControllerPressed')[0],player=pair.player_facts(game),yield_return=pair.reference_yield_return()))
            points.append(pair.compare_player(f'jump-{jump}-{frame}',game))
            assert game['mode']=='gameplay' and game['player']['selector']!=13
            if game['actors'][0]['position']['world_x']>=294:
                pair.input([]);break
            if frame>2 and game['player']['selector']==0 and game['actors'][0]['vertical_velocity']==0:break
        else:raise AssertionError('jump did not reach source landing/gap')
        pair.capture(f'jump-{jump}-end')
        if game['actors'][0]['position']['world_x']>=294:break
    points.append(pair.standing())
    pair.capture('gap-standing');pair.compare_terrain('gap-standing')

    return pair.game()

def terrain_mutations(pair,points):
    evidence=pair.evidence
    finish=lambda game,label:finish_source_frame(pair,evidence,game,label)
    strikes=[]; reward=None;all_events=[]
    for strike in range(3):
        direction='right' if strike==0 else 'left'
        pair.input([direction])
        for frame in range(120):
            pair.advance(1); game=pair.game()
            finish(game,f'strike-{strike}-approach-{frame}')
            points.append(pair.compare_player(f'strike-{strike}-approach-{frame}',game))
            assert game['mode']=='gameplay' and game['player']['selector']!=13
            if game['actors'][0]['animation']['selector']==7:break
        else:raise AssertionError(dict(reason='gap strike pose unavailable',strike=strike))
        pair.capture(f'gap-strike-{strike}-before')
        fields=['ObjectHandler','ObjectStateFlags','ObjectRenderFlags','ObjectLocalCounter','ObjectAnimation','ObjectSpawnIdentity']
        save(evidence/f'gap-strike-{strike}-source-before.json',{name:pair.memory(name,16) for name in fields})
        before_driver=source_audio_globals(pair,game)
        save(evidence/f'terrain-strike-{strike}-source-driver-before.json',before_driver)
        if strike==0:
            assert before_driver['source']['priority']&0xF0<=0xC0
            assert before_driver['native']['priority']&0xF0<=0xC0
        pair.begin_audio(f'terrain-strike-{strike}')
        before=game['world']['terrain']['collision']; cursor=game['events']['next_sequence']
        pair.input([direction,'b']); changed=False; born=[];strike_events=[]
        for frame in range(90):
            pair.advance(1);game=pair.game()
            finish(game,f'strike-{strike}-swing-{frame}')
            points.append(pair.compare_player(f'strike-{strike}-swing-{frame}',game))
            assert game['mode']=='gameplay' and game['player']['selector']!=13
            entries=game['events']['entries'];assert entries[0]['sequence']<=cursor
            fresh=[e for e in entries if e['sequence']>=cursor];cursor=game['events']['next_sequence']
            all_events.extend(fresh);strike_events.extend(fresh)
            if any(e['kind']=='AudioCommand' and e['payload']['command']==0x13 and e['payload']['accepted'] for e in fresh):
                queue=pair.memory('AudioCommandQueue',8)
                save(evidence/f'terrain-strike-{strike}-source-audio.json',queue)
                assert 0x13 in queue,queue
            born.extend(e for e in fresh if e['kind']=='ActorSpawned')
            changed |= game['world']['terrain']['collision']!=before
            for actor in game['actors'][1:]:
                if actor['active'] and actor['handler']==0x13:
                    reward=dict(slot=actor['slot'],position=actor['position'],tick=game['ticks'])
            if frame>0 and game['player']['selector']==0:break
        pair.input([]);pair.capture(f'gap-strike-{strike}-after');pair.compare_terrain(f'gap-strike-{strike}-after')
        assert changed,dict(reason='actual terrain mutation missing',strike=strike)
        assert any(e['kind']=='AudioCommand' and e['payload']['command']==0x13 and e['payload']['accepted'] for e in strike_events), 'source swing sound missing'
        if strike==0:pair.audio('terrain-strike-sound')
        strikes.append(dict(strike=strike,changed=changed,reward=reward,born=born))
        save(evidence/'strikes.json',strikes)
        save(evidence/f'gap-strike-{strike}-source-after.json',{name:pair.memory(name,16) for name in fields})
        if strike==0:
            # Stop beyond the lower block's right edge, then settle on
            # lower grass before approaching that same block from right.
            pair.input(['right'])
            for step in range(36):
                pair.advance(1);game=pair.game();finish(game,f'lower-right-edge-{step}')
                points.append(pair.compare_player(f'lower-right-edge-{step}',game))
                if game['actors'][0]['position']['world_x']>=334:break
            pair.input([])
            for step in range(40):
                pair.advance(1);game=pair.game();finish(game,f'lower-grass-{step}')
                points.append(pair.compare_player(f'lower-grass-{step}',game))
                assert game['mode']=='gameplay' and game['player']['selector']!=13
                if game['actors'][0]['position']['screen_y']>=180 and game['player']['selector']==0:break
            pair.capture('lower-right-grass')

    births=[e for e in all_events if e['kind']=='ActorSpawned' and e['payload'].get('after',{}).get('handler')==0x13 and e['payload']['after']['animation']['selector']==0x24]
    assert len(births)==1,births
    born=births[0];actor=born['payload']['after'];slot=actor['slot']
    assert actor['position']['world_x']==312 and actor['position']['screen_y']==184 and actor['render']==0
    assert born['payload']['owner']=='actor_slot_4'
    source_before=__import__('json').load(open(evidence/'gap-strike-1-source-before.json'))
    assert source_before['ObjectSpawnIdentity'][4]==130 and source_before['ObjectLocalCounter'][4]&0x80
    assert source_before['ObjectStateFlags'][slot]&0x80==0 and source_before['ObjectRenderFlags'][slot]==0
    source_after=__import__('json').load(open(evidence/'gap-strike-1-source-after.json'))
    assert source_after['ObjectAnimation'][slot]==0x24 and source_after['ObjectStateFlags'][slot]==0
    assert not pair.game()['actors'][slot]['active'],'source render-zero recovery must be culled, never counted collected'
    assert any(e['kind']=='AudioDequeued' and e['payload']['command']==0x13 for e in all_events)
    save(evidence/'terrain-events.json',all_events)
    save(evidence/'terrain-birth-and-cull.json',dict(birth=born,source_before=source_before,source_after=source_after,collected=False))
    return dict(mutations=len(strikes),birth=born,collected=False)

def collect_natural_health(pair,points):
    evidence=pair.evidence;game=pair.game()
    initial=health(pair,game);previous_health=initial
    lives=game['status']['lives'];cursor=game['events']['next_sequence']
    contact_trace=[];events=[];stopped=False;audio_started=False
    pair.input(['a','right'])
    for frame in range(520):
        if frame==8:pair.input(['down','b','right'])
        if not stopped and game['actors'][0]['position']['world_x']>=690:
            pair.input(['down','b']);stopped=True;pair.capture('food-approach-stopped')
        before=game
        pair.advance(1);game=pair.game()
        finish_source_frame(pair,evidence,game,f'health-route-{frame}')
        points.append(pair.compare_player(f'health-route-{frame}',game))
        assert game['mode']=='gameplay' and game['player']['selector']!=13 and game['status']['lives']==lives
        current_health=health(pair,game)
        entries=game['events']['entries'];assert entries[0]['sequence']<=cursor
        fresh=[e for e in entries if e['sequence']>=cursor];cursor=game['events']['next_sequence'];events.extend(fresh)
        if 660<=game['actors'][0]['position']['world_x']<=740:
            if not audio_started:pair.begin_audio('natural-health');audio_started=True
            base=SYMBOLS['ObjectRenderFlags']
            raw=pair.reference.request('emulator.memory',dict(address=base,length=2048-base))['bytes']
            fields=['ObjectHandler','ObjectStateFlags','ObjectAnimation','ObjectAnimationFrame','ObjectAnimationTick','ObjectLocalCounter','ObjectLocalState','ObjectRenderFlags','ObjectSpawnIdentity']
            source={k:raw[SYMBOLS[k]-base:SYMBOLS[k]-base+16] for k in fields}
            native=[a for a in game['actors'][1:] if a['spawn_identity'] in [134,139]]
            for actor in native:
                slot=actor['slot']
                expected={'ObjectHandler':actor['handler'],'ObjectStateFlags':actor['state'],'ObjectAnimation':actor['animation']['selector'],'ObjectAnimationFrame':actor['animation']['frame'],'ObjectAnimationTick':actor['animation']['tick'],'ObjectLocalCounter':actor['local']['counter'],'ObjectLocalState':actor['local']['state'],'ObjectRenderFlags':actor['render'],'ObjectSpawnIdentity':actor['spawn_identity']}
                for key,value in expected.items():assert source[key][slot]==value,dict(frame=frame,field=key,slot=slot,native=value,source=source[key][slot])
            contact_trace.append(dict(frame=frame,tick=game['ticks'],player=pair.player_facts(game),health=current_health,protection=game['player']['protection'],native=native,source=source,source_interrupt_tick=pair.memory('InterruptTick')[0],native_audio_frames=game['audio']['frames']))
            save(evidence/'contact-trace.json',contact_trace);save(evidence/'health-events.json',events)
            target=game['actors'][12]
            prior=before['actors'][12]
            if target['active'] and target['handler']!=prior['handler']:pair.capture(f'health-handler-{target["handler"]}')
            if target['animation']['selector']!=prior['animation']['selector']:pair.capture(f'health-animation-{target["animation"]["selector"]}')
            if target['local']['counter']==29 and prior['local']['counter']==0:pair.capture('health-contact')
        if frame%40==0:pair.capture(f'health-route-{frame}')
        if sum(current_health)<sum(previous_health):
            # Compare to the real immediately preceding HP, since an ordinary
            # shooter may have caused additional damage during this route.
            prior_actor=before['actors'][12]
            assert prior_actor['active'] and prior_actor['handler']==0x13 and prior_actor['animation']['selector']==0x24
            assert not game['actors'][12]['active'] and contact_trace[-1]['source']['ObjectStateFlags'][12]==0
            sound=[e for e in fresh if e['kind']=='AudioCommand' and e['payload']['command']==0x15 and e['payload']['accepted']]
            assert sound,'actual food reward sound missing'
            queue=pair.memory('AudioCommandQueue',8);assert 0x15 in queue,queue
            states=[(x,next((a for a in x['native'] if a['slot']==12),None)) for x in contact_trace]
            delay=[(x,a) for x,a in states if a and a['handler']==0x14 and a['state']==0xB1]
            assert [a['local']['counter'] for x,a in delay]==list(range(29,0,-1)), 'source contact countdown must execute fully'
            assert delay[0][0]['protection']==0
            assert any(a and a['animation']['selector']==0x2A and a['state']==0xA2 for x,a in states)
            assert any(a and a['handler']==0x13 and a['animation']['selector']==0x24 for x,a in states)
            pair.capture('health-refilled');pair.compare_terrain('after-health-collection')
            driver_before=source_audio_globals(pair,game)
            save(evidence/'health-before-state.json',before)
            save(evidence/'health-collection.json',dict(frame=frame,before=previous_health,after=current_health,initial=initial,sound=sound,source_queue=queue,source_identity=134,slot=12,driver_before=driver_before))
            occurrence=max(e['tick'] for e in events if e['kind']=='AudioCommand' and e['payload']['command']==0x16)
            epoch=next(x for x in contact_trace if x['tick']==occurrence)
            source_age=(driver_before['source_interrupt_tick']-epoch['source_interrupt_tick'])&255
            native_age=driver_before['native_audio_frames']-epoch['native_audio_frames']
            save(evidence/'health-audio-epoch.json',dict(reveal16_queued_tick=occurrence,epoch=epoch,source_interrupt_age=source_age,native_interrupt_age=native_age))
            assert source_age==native_age,'exact driver comparison requires same actual16 interrupt age'
            assert driver_before['native']==driver_before['source']
            # Reveal16 (priorityD0) still owns pulse2/noise. Collection15
            # (priority60) is consumed but rejected by the original dispatcher.
            # Observe actual retained16 emission instead of forcing a15 stream.
            pair.input([]);pair.begin_audio('health-effect-after-collection')
            effect_trace=[];dequeued=[];post_cursor=game['events']['next_sequence']
            for effect_frame in range(30):
                pair.advance(1);post=pair.game()
                finish_source_frame(pair,evidence,post,f'health-effect-{effect_frame}')
                points.append(pair.compare_player(f'health-effect-{effect_frame}',post))
                assert health(pair,post)==current_health and post['status']['lives']==lives
                post_entries=post['events']['entries'];assert post_entries[0]['sequence']<=post_cursor
                post_fresh=[e for e in post_entries if e['sequence']>=post_cursor];post_cursor=post['events']['next_sequence']
                dequeued.extend(e for e in post_fresh if e['kind']=='AudioDequeued' and e['payload']['command']==0x15)
                raw=pair.memory('AudioChannels',31*4);voices=[]
                for voice,channel in enumerate(post['audio']['channels']):
                    offset=voice*31;pitch=raw[offset+16]
                    if pitch>=128:pitch-=256
                    source=dict(pitch_step=pitch,period=raw[offset+17]+256*raw[offset+18],control=raw[offset+19])
                    voices.append(dict(voice=voice,native=channel['secondary'],source=source))
                driver=source_audio_globals(pair,post)
                effect_trace.append(dict(frame=effect_frame,tick=post['ticks'],voices=voices,driver=driver))
                save(evidence/'health-effect-secondary-trace.json',effect_trace)
                save(evidence/'health-effect-dequeued.json',dequeued)
                assert ((driver['source_interrupt_tick']-driver_before['source_interrupt_tick'])&255)==driver['native_audio_frames']-driver_before['native_audio_frames']==effect_frame+1
                assert driver['native']==driver['source'],dict(effect_frame=effect_frame,driver=driver)
                for v in voices:assert v['native']==v['source'],dict(effect_frame=effect_frame,voice=v)
                if effect_frame==0:save(evidence/'health-postconsume-source-queue.json',pair.memory('AudioCommandQueue',8))
                if effect_frame in [0,4,19,29]:pair.capture(f'health-effect-{effect_frame}')
            # Output and actual facts survive any later contract assertion.
            pair.audio('health-collection')
            assert dequeued,'queued food command must actually be consumed by audio interrupt'
            first=effect_trace[0];prior=driver_before['source'];after=first['driver']['source']
            assert prior['priority']==0xD7 and prior['secondary_mask']==0x0A
            assert 0x60 < prior['priority']&0xF0
            assert after['priority']==prior['priority'] and after['secondary_mask']==prior['secondary_mask']
            assert after['secondary_cursor']==prior['secondary_cursor']==0xA49E
            queue_after=__import__('json').load(open(evidence/'health-postconsume-source-queue.json'))
            assert 0x15 not in queue_after
            assert first['voices'][1]['source']==dict(control=0x3F,period=0,pitch_step=0x10)
            noise=[x['voices'][3]['source'] for x in effect_trace]
            assert any(v['period']==8 and v['control']==0x3A for v in noise)
            assert any(v['period']==8 and v['control']==0x35 for v in noise)
            assert any(x['driver']['source']['priority']==7 and x['driver']['source']['secondary_mask']==0 for x in effect_trace)
            assert any(e['kind']=='AudioCommand' and e['payload']['command']==0x16 for e in events)
            save(evidence/'health-audio-priority-result.json',dict(food15='consumed but rejected:60<D0',before=driver_before,after=first['driver'],retained_effect16='actual pulse2/noise tail through termination',wav='health-collection.wav includes retained16 and later natural jump10, not emitted15'))
            return dict(collected=True,source_identity=134,frame=frame,before=previous_health,after=current_health)
        previous_health=current_health
    raise AssertionError('bounded natural contact-health route did not execute real recovery')


def breakable_reward(pair):
    points=[];pair.observations=points;evidence=pair.evidence
    gap=natural_gap(pair,points)
    checkpoint=pair.native.request('port.checkpoint.export')
    source_path=evidence/'gap-reference-snapshot.json'
    pair.reference.request('emulator.snapshot.export',dict(path=str(source_path.resolve())))
    save(evidence/'gap-checkpoint.json',checkpoint)
    save(evidence/'gap-checkpoint-preconditions.json',dict(native=gap,source_yield=pair.reference_yield_return(),provenance=identity()))
    terrain=terrain_mutations(pair,points)
    # Restore both independent same-build replays to the already tested gap.
    # The mutation obligation has just executed, and collection is a separate
    # natural contact-health subscenario rather than a claim about culled food.
    pair.native.request('port.checkpoint.import',dict(checkpoint=checkpoint,timeout_ms=300000))
    pair.reference.request('emulator.snapshot.import',dict(path=str(source_path.resolve())))
    assert pair.game()==gap,'same-build replay must restore full canonical state'
    assert pair.reference_yield_return()==0xDC28
    pair.compare_player('restored-gap',gap);pair.compare_terrain('restored-gap')
    pair.steps.append(dict(paired_replay_restore=True,native_checkpoint_sha256=sha(evidence/'gap-checkpoint.json'),source_snapshot_sha256=sha(source_path),native_identity=checkpoint['identity'],source_yield=0xDC28))
    save(evidence/'restore-validation.json',pair.steps[-1])
    recovery=collect_natural_health(pair,points)
    save(evidence/'terrain-health-trajectory.json',points)
    return dict(terrain=terrain,recovery=recovery,paired_gap_replay_validated=True)
