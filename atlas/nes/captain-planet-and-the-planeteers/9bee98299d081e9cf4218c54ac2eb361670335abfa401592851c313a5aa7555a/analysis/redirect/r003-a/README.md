# Redirect attempt r003-a (round 3, input checkpoint pass-003, unchanged)

Research only: ROM, map, state, source, toolkit and binary unchanged. r001-a/r002-a preserved.

## Files
- tools/consumers4.py - r002-a consumers.py with the 4 current residuals (adds bank 6 $A175/$A18B).
- tools/slots.py - live task-slot / CP-stage RAM dump (server).
- tools/shot.py - framebuffer PNG into a caller-local dir (copy of tools_local/shot.py with local output path).
- cov_r003a.json - 4605 executed physical opcode offsets (cumulative traced windows of the artificial routes below).
- l0boss_zone.json - snapshot: level 0 CP stage, boss-zone task $A015 (bank 8) created, EventPhase 1, 60 frames later.
- shots/ - l0t2 (level-0 victory pose after played target hits), l0d (WELL DONE), l1a (level-1 barrier + minion),
  l1c (level-1 cleared), l2c (level-2 immediate clear), l2d (next stage = Geo-Cruiser).

## Q1: the 4 Unknown lone-RTS bytes ($0C185, $0C19B, $1FB79, $1FDD0) - rejection review
Claim tested: each byte is (a) Static Code (dead/stub RTS) or (b) generic Data (padding/filler).
Checks (all on unchanged primary evidence):
1. consumers4.py: no abs/jmp-ind operand, no immediate lo/hi pair, no raw word for target or target-1 for any of the 4.
   Indexed hits are bounded known tables (bank 6 SolidRangeLo6/Hi6,y with y=Level 0-4; ShotStartOffsets6,y 10 bytes) and
   raw words at bank 13 $A113/$A11D are entries of bank 13's own ObjectAnimLists13 table (bank-13 targets, wrong bank).
   All game dispatchers (jmp (Ptr0) x4, JumpPtr0, RTS-trick QueuePaletteInline, task entry words) use interleaved
   words or inline data, so the raw-word search covers them; IrqHook is set only via VecSetIrqHookInline inline words.
2. Module-copy parallels: the 12-byte contexts before/after each byte occur nowhere else in PRG. Bank 3 has the
   PlaceShot routine (`a4 94 b9 .. .. 20 .. .. c8 b9 .. .. 20 .. .. a6 85 60`) followed by ordinary code, not by
   `60` + sound routines, so no shared-source module supplies a referenced counterpart.
3. Fill convention: every uniform run >=16 bytes in PRG is $00/$FF (or data values); no run of three $60 exists outside
   bank 10 data. $60 is not this ROM's fill value; the next routines ($A176, $A18C, $FB6A, $FDC1) are unaligned.
4. Runtime: none of the 4 bytes was fetched in any r003-a route either (cov_r003a.json, 4605 offsets, 0 outside Code).
Result: (b) has no support - a single $60 between code routines is not a repeated pattern, a uniform run, visibly
non-instruction content or a fill value. (a) rests only on legal decoding of one opcode in code context, which
evidence-policy.md explicitly calls insufficient for an unreferenced return opcode. Plausible code-side stories
(disabled-entry `rts` before the unreferenced bank-6 sound bodies; `@return: rts` remnant like bank 12 $BF0A whose
2nd RTS is a branch target) are analogies, not constraints on these bytes. No discriminating structural fact exists;
the pass-003 rejection is correct in scope. Keep Unknown.

## Q2: gameplay milestones (artificial routes; ordinary gameplay evidence NOT claimed)
Server: python3 tools_local/rpcd.py; RPC tools_local/rpc.py.
- Level 0 boss zone + played-hit target defeat: tools_local/pw.py 0; play.py "start:10 w:900 start:10 w:300 start:10 w:400 w:600";
  analysis/redirect/r002-a/tools/addtask.py 8 A015 F5=1; w:60 (= l0boss_zone.json). Slots: 8 $AD8D, 9 $AD39 (GoalTargetTask
  $AD25), 10 $AC50, 11 $ACC2, 12/16 $AA87; CP slot 17 bank 7 $AACC. ARTIFICIAL position patch: slot 9 ObjXLo $1B=$88,
  ObjXHi $2D=0, ObjYLo $3F=$60, ObjYHi $51=0 (target moved onto CP). Then REAL input "a:4 w:50" x6: each A press (special
  move, SpecialMoveTimer=8 -> CheckHitIfMode8) advanced ObjIndex 1..5, 6th hit set GoalHitCount=4 -> GoalTask
  MarkStageCleared + score 50 (GoalHitCount 5, StageEndFlag 2), target became debris (slot 9 -> bank 13 $B533), CP victory
  pose; ~700 frames later WELL DONE stage-clear screen (tasks $E360/$F1F1).
  Also artificial-only variant: GoalHitCount $0709=4 patch -> same outcome.
- Level 1 goal (Level1GoalTask) defeat: pw.py 2; same play tokens; addtask.py 8 A015 F5=1; w:120; write $0709=4 ->
  BarrierBuilderTask: 5 barrier parts (bank 8 $B732) + minion (bank 8 $B6A1) observed, GoalHitCount 5; write BossPartsHit
  $07A0=5 -> MarkStageCleared, StageEndFlag 2, ~900 frames -> stage-clear tasks $E360/$F1F1.
- Level 2 CP stage gameplay: pw.py 4; same play tokens -> green CP stage, traced "right+b:60 left+b:30 down+b:40 a:4 w:30 up:30"
  (Energy fell 193 -> 104: hurt mechanics). addtask.py 8 A015 F5=1 at MapColumn 3 -> GoalTask immediately set
  GoalHitCount 4 (level-2 rule: camera left of map column 4 counts as reached) -> victory pose; ~700 frames later
  the next stage (Geo-Cruiser, banks 1/12) started. Confirms the static GoalTask level-2 reading (artificial state).
- Coverage: 4605 executed PRG offsets, 0 outside Code; 153 Static Code regions gained executed opcodes (1744).
  L008/L009 ranges (incl. bank 6 $A175-$A1A0) not executed.
Deferred: barrier parts destroyed by power 2 (power-select RAM contract not established; outcome already observed via
patch); level-3 boss defeat by play (r002-a recipe; downstream path same MarkStageCleared13 family);
CpHurtCount>=$10 life-loss exception (semantic only, no byte depends on it).
