# Redirect r03-a2 notes (round 3 continuation)

Input: maps/round03-final.json aab2fb6c... (= current.json = state.json), source b70bdbb2..., ROM 8e32ae84...;
state.before.sha re-checked OK after the work. Only emulator calls + leads.json append (tools/add_leads.py).
Ledger unchanged from r03-a1 (3 B TK-04; rejection review not redone per coordinator).

Tools: tools/w2.py (evwatch.py derivative: chosen columns, optional recorded RAM writes 'wAAAA=VV'),
tools/skate_obj.py (artificial ball spawn image), tools/add_leads.py. Per-frame jsonl logs were disposable and deleted;
excerpts are in evidence/.

## Work log
1. BMX: static read of BmxPhysics..BmxRotate, BmxTrickBonus*, BmxCourseObjects, BmxFrame; r03-a1 bmx jsonl re-read per
   frame (Up sequence); new watches bmx-down (no write), down2/down3 (no write, score check), bmx-left (artificial $A4=4).
   Findings: BmxLand = takeoff init; bonus table order reversed vs labels (confirmed by score 1510); BmxScoreTrick =
   finish; BmxUpdateTrick = hazard check; air-score index 21 overrun (10802, artificial). -> L-R3R-05.
2. Skating: spawn gate + 4-byte mask table read with index 0-15 (static); artificial ball spawn -> bounce/depth contract.
   Natural spawn not pursued further (gate needs CameraX $00-$3F + RNG; skater falls early). -> L-R3R-06.
3. Surfing: $CB/$CC = shark fin (artificial screenshot); natural fin needs RNG 1/4 at state-4 entry; 9 input variants
   gave identical RNG (state 4 time fixed) -> stopped as brute force. Aerial/carve variables static. -> L-R3R-07.
4. Foot bag: $DE-$E2 = seagull (messages + natural watch). -> L-R3R-08.
5. Half pipe: trick start/end and fall-pending watches; save-state gate is $AB not PipeTrick. -> L-R3R-09.
