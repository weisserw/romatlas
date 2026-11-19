# Round 1 redirect handoff (attempt r01-a)

- Input checkpoint: pass-001 (maps/current.json == maps/pass-001.json, unchanged by this phase).
  Classification unchanged: 86.76% (341171 / 393232; 52061 unknown). Redirect changed research
  and lead records only (analysis/leads.json, scenarios.json, progress.md, toolkit-issues.md TK-004).
- Research directory: analysis/redirect/r01-a/ (tools/, evidence/, leads.before.json).
- Status: READY leads for another disassembly pass (not a terminal handoff).

## Method
1. Bank-aware static descent (tools/sa_pair.py + desc.py + batches.py), an improved copy of the
   round-1 analyzer: keeps the even/odd switchable pair across JSR (all PRG mapping goes through
   BankPushAndMapY, which maps Y/Y+1; only CHR registers are written elsewhere), models
   MapBanksN ($D10A-$D122) / BankPopAndRestore ($D156) as a pair stack, parses JumpEngine
   ($C294) inline tables and local `lda tbl,y .. jmp (zp)` tables whose base is the byte after the
   JMP; table extent = up to the first entry target, first already-reached instruction or classified
   byte. Overlaps/illegal opcodes are pruned; conflicts with existing map regions reported.
   One guarded path is excluded: $DCEF is never dispatched in pair 10/11 (StageState5_Event cmp #4/beq).
2. Root sets per lead (batches.py keys); `base` = only existing map code roots (finds unparsed
   entries of known dispatchers). Results: evidence/static-descent-r01a.json (ranges, tables, bad).
3. Bounded runtime checks (romatlas serve, same-build pre_stage.snap, artificial state writes):
   stage 3-3 boss (evidence/probe-stage33-boss.json) and ending via GameMode=7
   (evidence/probe-ending-gamemode7.json, frames/*.png). Scratch: /root/rascratch-r01a (pruned).

## Findings
- ObjHandlerTable $DE19 = 62 words filling the whole gap to $DE95; dispatch pair: loop 2 of
  UpdateAllObjects always 6/7, loop 1 by BossClass (0 -> 6/7, 6 -> 4/5, else 10/11). 59 roots in
  6/7 + 7 fixed-bank entries decode with 0 bad / 0 conflicts (10.5 KB). Stage 3-3: wave $1F sets
  BossClass 4 and ObjType $1F; 416 observed instruction fetches in bank-11 0x16F05-0x17546, all
  predicted by the static 10/11 descent from $AEF5 (0 unpredicted).
- Stage-event system: StageEventDispatch table $CE4C (16 entries, bank 5 code, ambient pair 4/5
  observed at StagePlayDispatch) + StageState5_Event table $CE99 (1-3 in 10/11, entry 4 $DCEF in
  4/5 - only 4/5 decodes $B0A2/$B0C6/$B105). Bank-5 event code contains the ending trigger
  (lda #7; sta GameMode at $B7B9) and StageIndex advances.
- Mode6_Dispatch ($C1A5) table -> $C1B5 (MapBanks10_11; jsr $84E4: continue/game-over code that
  sets GameMode 3/0 or jumps to Reset); Mode7_Dispatch ($C1BF) -> $C1CF jmp $F3C3 = ending code in
  bank 15 (local step table $F410, 22 entries). Artificial GameMode=7 shows the ending scenes
  (Penguin scene, Bat-signal/Catwoman) and the staff credits; step $05AF advanced 4 -> 16, then
  cycles 15/16 (credit pages) by ~3600 frames; 133 unknown fetches in banks 14/15, all predicted
  statically. Artificial state only (scenarios.json updated).
- Map error: Data region id 1066 (0x157DC, 134 B) ends with code $9844-$9851 (called by IntroStep $1A).
- Bank 0 gap is sound data anchored by SndHeaderTable/SndEnvelopeTable/SndVibratoTable inside it.
- Bank 11 $A043-$A2A1 = ObjType byte-pair tables + PPU text scripts; bank 14 $D58F-$D6B4 = map
  pointer/attribute lists; $C314 quarter-wave table (6-bit index); bank-ID bytes at $8000.
- Bank 12/13: code islands only at $91C2-$93B3 (StageScriptCmdTable, 14 entries; entry $9313 calls
  checksum $DDEA -> DamageDoubler) and $B48A-$B4D7; the remainder shows only data patterns.

## Ready leads (analysis/leads.json)
L001 (12.0 KB code + 124 B table), L007 (3.6 KB), L008 (1.1 KB), L009 (4.5 KB), L010 (14 B
correction, apply before L008), L011 (0.9 KB), L012 (9.6 KB, overlaps L009), L003 (~0.4 KB code +
~13.9 KB data), L004 (4456 B data), L013 (64 B), L014 (5 B), L015 (336 B), L016 (607 B).
Combined static code estimate (batch ALL): 27269 B new code; residual after ALL 24792 B, of which
bank 0 (L004) and banks 12/13 (L003) are data proposals (~18.8 KB). L002/L005 subsumed (L007/L008/
L012 and L001/L012). L006 stays open (natural ending route not played).
Recipe: `python3 analysis/redirect/r01-a/tools/batches.py <key> [nobase]`; review table extents in
the evidence JSON; classify as Static code; data leads via map.annotate with Static provenance.

## Rejected / blocked / attempted
- Wave $0B (BossClass 3, stage 3-3 part 2) not triggered by StageScriptPos=9 patch (BossClass stayed 0;
  record 1 format/pos assumption wrong). Static coverage of its handlers is via L001/L011 anyway.
- Dispatch of $DCF4 sub-table in pair 10/11 rejected (decodes bad at $B0A2/$B105; guard excludes it).
- No natural boss defeat / stage clear played: boss HP RAM contract not identified this pass.

## Deferred avenues (reasons)
- Small residual holes (4-180 B) in banks 4/5/6/7/10/11/14/15 next to new code: mostly inline tables;
  better resolved after L001-L012 are merged (refresh map, rerun lowwin.py/resid2.py) - depends on map change.
- Bank 7 $BF16-$BFCB (182 B), bank 6 $92CE (45 B), bank 7 $A33A (43 B), bank 15 $F850 (115 B),
  $FA6F (99 B): no reference found yet; revisit after merge with the refreshed reference scan (refs.py).
- Natural ending route (defeat final boss stage $15): needs boss-HP/defeat RAM (BossDefeat paths
  around $DF99/GroupDefeatedScore); next step: watch writes to ObjVar/HP bytes in stage $15 boss.
- Password screen (unchanged from round 1).
