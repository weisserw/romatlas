# Redirect attempt r002-a (round 2, input checkpoint pass-002, map sha256 295a9b18...)

Research only: ROM, map, state, source and binary unchanged. Prior attempt r001-a preserved.

## Files
- tools/m6502.py - opcode table (copy of tools_local/m6502.py).
- tools/deadtail.py - classified Code instructions following rts/rti/jmp with no static reference (survey).
- tools/rtsrts.py - every `60 60` pair inside Code, with reference/provenance of the 2nd RTS.
- tools/consumers.py - L009 consumer search: abs/indexed operands (base within 255 below), jmp-ind cells, imm lo/hi pairs, raw words.
- tools/xcheck.py - soundness: decoded control-flow targets landing in non-Code regions (result: none).
- tools/addtask.py - ARTIFICIAL: add one task (bank, entry) to the first free slot at the idle loop, other tasks kept.
- tools/chkpcs.py - check sampled task resume PCs against the map (kind/provenance/instruction start).
- tools/covcheck.py - dump analysis.coverage and check executed offsets against the map and L008/L009 ranges.
- cov_r002a.json - 4499 executed physical opcode offsets (artificial routes below; trace context marked artificial).
- l3boss_arena.json - snapshot: level index 3 (4th level) Captain Planet boss arena, EventPhase 3 (artificial route).

## Findings
1. L009 (5 Unknown bytes). consumers.py: no abs/indexed/indirect operand, imm pair or raw word (target or target-1)
   reaches bank 13 $BA5D-$BA5F, $FB69 or $FDC0. Next routines start at $BA60/$FB6A/$FDC1 (not aligned: no alignment
   padding role); value is not a fill value. $BA5D-$BA5F (4C C2 B4) is byte-identical to the `jmp DebrisSpawner`
   four bytes earlier (same routine). rtsrts.py: the ROM has 5 `rts rts` pairs in code areas: bank 12 $BF0A (2nd RTS is
   a branch target), bank 6 $A175 and $A18B (2nd RTS unreferenced, ACCEPTED in pass-002 as Static Code
   "Bank6StubReturn/Bank6StubReturn2"), and the two Unknown bank-15 bytes. The bank-3 "parallel layout" cited for
   Bank6StubReturn does not exist (banks 3/6 share only the $A003-$A089 header; bank 3 $A175 is the RTS ending a routine, followed by a byte table at $A176).
   => consistency problem: the same evidence status was accepted in bank 6 and rejected in bank 15.
2. Bank 8 $B20F-$B212 (`11 00 00 01`): bytes $B20E-$B212 = `00 11 00 00 01` form an empty 1x1 sprite definition
   (format [attr][rows<<4|cols][dx][dy][tile]; same bytes as EmptySpriteDef3/6/7/12/DebrisSpriteDef). Consumer:
   TransformFrames ($B8CC-$B92B, read by SetTransformFrame -> B8_SetSpriteDef) holds the word $B20E ten times.
   $B20E is also the shared empty spawn list of levels 3-4 (MapSpawnIndex = 00 22 41 72 72 -> $B19C+$72).
   Region 1500 is already Data (classification sound); only its role/label/extent description is wrong.
3. Soundness review of accepted heuristic classifications: xcheck.py found no decoded branch/jsr/jmp target inside
   any Data/Text region; all 20656 + 8772 observed roots of earlier rounds and all 4499 executed offsets of this
   attempt are Code instruction starts. Only inconsistency found: item 1 (bank 6 lone RTS stubs, 2 bytes).
4. L008 static-only routines: not executed in any route of this attempt (boss arena, boss clear + ending, game over,
   menu, continue). No new caller found; reachability stays unknown.
5. Gameplay RAM contract (from source, confirmed at runtime):
   Energy $0563 ($C6 full; >= $DC or underflow on hit -> CpPlayerDeath), RingLevel $0713 = remaining lives
   (4 after a continue; CpPlayerDeath / level-3 boss escape decrement; negative -> EndingFlag $0715 = $FF).
   EventPhase $F5, GoalHitCount $0709, BossZone tables bank 7 $B0DE ($B0CA creates task $A015 in CpStageBank).
6. Game over (artificial patch, natural sequence afterwards): EndingFlag $FF -> VecAfterPasswordEnding ->
   StartEndingSequence -> EndingTask shows Gaia "I AM VERY SAD, PLANETEERS... YOU ... THE PLANET IS DYING...";
   Start -> EndingPictureTask = "FINAL SCORE / CONTINUE / END" menu; CONTINUE restarts the stage with RingLevel 4,
   END returns to the Mindscape title. EndingTask is shared by the real ending (flag >= 0).
7. Bosses (artificial): bank 7 $B0CA creates task $A015 with TaskBank = CpStageBank = CreateInitialObjects(13),
   i.e. the boss/goal objects are created on entering the boss zone (label "Bank7SpawnItem" in commentary is the
   bank-7 alias, wrong effective target). Level 3 arena reached (Level3BossTrigger -> Level3BossTask, map bank $0B,
   EventPhase 2->3; boss vehicle). Observed: boss moving off right (ObjXHi=1, X>=$20) -> EventPhase 4, arena restart,
   RingLevel-1 (reading "pushed out" corrected to "boss escapes"). Level 4 boss: tasks $B68D/$B6B2/$B7E5/$B8B8/$B8CA
   live; GoalHitCount patched to 5 -> EndingFlag 2, CpVictoryPose (bank 7 $BB5E/$BB6B), ending Gaia text
   "EARTH IS SAVED! THANK YOU, PLANETEERS" + FINAL SCORE. Level 1 (2nd level) CP stage visited; its boss objects
   (bank 8 $A435/$A49F/$A8C5/$A8D4/$A8E3/$B3A3/$B4D3/$B5AA) started via the same $A015 task.
   All 26 sampled Static task PCs are instruction starts of classified code (no contradiction).

## Recipes (server: python3 tools_local/rpcd.py; JSON-RPC via tools_local/rpc.py)
- Game over: snapshot.import analysis/redirect/r001-a/runs/l5cp_start.json; write $0713=0, $0563=$DD; run ~600 frames
  (Gaia text); Start -> menu; Start = CONTINUE, Down+Start = END.
- Level-3 boss arena: tools_local/pw.py 6; play.py "start:10 w:900 start:10 w:300 start:10 w:400" then "w:1200";
  tools/addtask.py 13 A015 F5=1; run_until pc=$AD95 with prg_banks[1]=13, write $AE=$D0; ~100 frames -> arena
  (or import l3boss_arena.json).
- Level-4 boss / ending: import l5cp_start.json; tools/addtask.py 13 A015 F5=1; 300 frames; write $0709=5; ~1300 frames.
- Level-1 CP stage + boss objects: tools_local/pw.py 2; same play tokens as level 3; tools/addtask.py 8 A015 F5=1.
- Trace: trace.configure {enabled:true, capacity:0, kinds:[Instruction]}; ~3 fps; covcheck.py.
