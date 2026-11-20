# Round 1 — redirect handoff (attempt r01-a)

Status: **DONE — ready leads L-004, L-005, L-009, L-010, L-012, L-013, L-015, L-016.** No classification changes were
made (map, state, source and binary unchanged); proposals go to the next disassembly pass.

## Scope and checkpoint
- Input: checkpoint **pass-001** (maps/pass-001.json = maps/current.json; 244,422 / 262,160 classified, 17,738 unknown),
  commentary handoff round-01-commentary.md, leads L-001..L-012.
- Research directory: `analysis/redirect/r01-a/` (README.md there lists tools, evidence and reproduction commands).
- Runtime: one `run.py serve` session via analysis/tools/rpcd.py, round-1 snapshots (scratch/snap_*.bin), aggregate tracing
  (capacity 0) with coverage merged to scratch/observed-r01a.json, promoted as `r01-a/observed-r01a.json.gz`. Server stopped.
- Verification of this phase: no map edits, so no new checkpoint; pass-001 verification remains the latest
  (analysis/pass-001-verification.json, byte_identical true).

## Milestones visited (analysis/scenarios.json S11-S14, S16; all artificial RAM-patch scenarios, traced)
| Milestone | Patch / route | Observed |
|---|---|---|
| S11 land a fish | fight snapshot, hold A, write $075D-$075F=0 for 2 frames | GOT HIM -> dorado weigh-in -> keep YES/NO -> sea |
| S12 end of day / results / password | sea, write $0755-$0757=59,59,15 | TIME IS UP -> WEIGHING/GO FISHING AGAIN/RESULTS -> password -> next day |
| S13 Hawaii | RoundCleared $079E-$07A1 nonzero, SELECT on Round_InfoScreen | HAWAII 1, KAWAIHAE, Hawaii sea (AreaIndex 4) |
| S14 ending | day-end menu, $079F-$07A1=$FF, best weight $0777=$01B0 | YOU HAVE WON ALL TOURNAMENTS -> password -> FIN screen |
| S16 fight event | fight, $05C4=4 | Fight_EventScript2 ($E521) runs script $E5DC, fight resumes |
Not attempted: S15 boat damage/return to port (lower priority). Gameplay notes: analysis/gameplay.md.

## Findings
1. **Fight entry runs in banks 0/1 (L-011 resolved).** Fight_BuildScreen $E6C1 executes with [0,1,14,15]; $ADC0 (bank 1)
   and $92A7 (bank 0) are executed fight-setup code, currently rule-R Data / unknown.
2. **Rule-R contamination.** Round-1 rule R ("observed data reads") labelled several executed instruction bytes as data
   (bank 0 $929F-$92A7/$92C8, bank 1 $ADC0, $ADF9-$AE11 JSR chain, $BA2A, bank 0 $8BF3-$8BF4 JSR operand + RTS ...).
   The readtrace windows ran code whose Instruction events were missing, so its opcode/operand fetches counted as
   data reads. 286 Data bytes are overlapped by the code proposal; remaining rule-R Data in code banks should be
   re-audited against the new code set (part of L-013).
3. **Script engine format solved (L-005).** `r01-a/script_parse.py` parses script bytecode, PrintText strings,
   switch tables and native islands from the 15 immediate ScriptPtr roots: 8,467 bytes with roles (script 4,494,
   text 3,810, switch tables 100, native 63); 1,277 of them are currently Unknown. Zero overlap with any existing Code,
   with the static code proposal, or with any observed opcode address.
4. **Script-driven native code (L-004 callers found).** `$38` CallTest operands (56 targets) and `$04/$05` frame/IRQ
   hook operands (12 vectors) are code entry points; with immediate IrqHookVec stores ($F7B2, $F7DF) and the default
   frame hook $E0E0 they reach the round-1 "uncalled" fixed-bank gaps ($EEFD-$EF94, $F74E-$F8CB, $F988-$F9AB,
   $F9FA-$FA2E, $E2A4-$E303 far-call wrappers into bank 13, $ECE4-$ED61 ...) and most of banks 12/13 native code
   (title/name entry/password/results routines, e.g. bank 13 $AC6C-$B1C7).
5. **Static prediction check.** All 11,151 r01-a observed contexts (day end, results, ending and Hawaii were traced after
   the static proposal was built) land on predicted instruction starts; 0 misaligned, 0 inside script/text roles.
6. **Whole-bank data (L-016).** R6/R7 audit: every bank-data write stores an immediate {0,2,4,6,$0C}/{1,3,5,7,$0D},
   AreaMapBank[AreaIndex] (8-11, 4) or a PLA restore. Banks 3, 5, 8-11 are mapped only for data access (sound far-call
   with bank 2 executing and its 33-entry jump table inside bank 2; PpuQueue_Execute copies; map wrappers whose
   910-instruction fixed-bank callee closure never transfers into $8000-$BFFF; frame hooks map 6/7 themselves before
   JSR $8000, e.g. $DE5B, $C881). Observed contexts [8,1,..], [9,1,..], [2,3,..], [4,5,..] show no fetch from those banks.
7. Easter-egg text "MEGUMI.MY.LOVE" at bank 15 $E0E1; $E0E0 is the default frame-hook RTS.

## Ready leads (analysis/leads.json; recipes there)
| ID | Claim | Physical ranges | Expected effect |
|---|---|---|---|
| L-009 | bank 1 $BA2A-$BA4C one routine (observed $BA2A/$BA2C) | 0x03A3A-0x03A5C | 4 Data -> Code, remove misaligned 0x03A3E decode |
| L-010 | bank 0 menu/event wrappers, context [0,1] | 0x00B96-0x00BAC, 0x00BD4-0x00CE0 (incl. Data 0x00C03-04), 0x00CED-0x00D25, 0x00114-0x00129, 0x00E41-0x00E63 | ~400 B code |
| L-012 | fight event script (Switch $05C4 + 15 words + End) | 0x1E5EC-0x1E60D | 34 B script data |
| L-013 | observed + static code batch (r01-a coverage) | r01-a/newobs.txt, r01-a/rd_delta.txt | with L-004/L-010: Unknown 4,918 + Data 286 -> Code |
| L-004 | script native-call / hook targets in banks 14/15 (+12/13) | r01-a/rd_delta.txt rows for $E2A4.., $ECE4.., $EEFD.., $F74E-$FA2E | included in L-013 totals |
| L-005 | script bytecode / text / switch tables / native islands | r01-a/script_parse.json runs | 1,277 Unknown -> Data/Text (+ refined labels for 6.8 KB Data/Text) |
| L-015 | small fixed-bank residues with consumers | 0x1E90C-0x1E92F, 0x1F6FE-0x1F707, 0x1FA2F-0x1FA36, 0x1F76E-0x1F786, 0x1E0F0 | 80 B |
| L-016 | banks 3, 5, 8, 9, 10, 11 never executed -> generic data | all Unknown in those banks | 7,389 B data |

Projection (r01-a/projection.txt + L-015 + L-016): unknown 17,738 -> ~4,080 if all are accepted (code/script batch
-6,189, L-015 -80, L-016 -7,389). Per bank after code/script batch: 0: 647, 1: 920, 12: 104, 13: 165, 14: 343, 15: 145.

L-015 details: $E8FC-$E91F continues AreaBgPalettes (all words $BAB3-$BB43, 18-byte palette stride, between classified
parts $E8E0-$E8FB and $E920; unofficial-opcode decode) -> generic data (exact table split unresolved: AreaIndex only
needs 13 words). $F6EE-$F6F7 TextCommandLengths `00 02 01 03 ff ff ff 02 ff 00`, consumer PrintText_QueueCommand,
end fixed by observed code $F6F8. $FA1F-$FA26 two 4-byte tables read by hook $F9FA with Y=(($28&$30)>>4). $F75E-$F776
`F8 14 0A F5 5E 07 F8 16 0A F5 C0 07 F8 18 0A F5 B8 07 F8 1A 0A F5 B3 05 00` PrintText string between code $F75D and
hook $F777 (consumer not located; Text/Data by format). $E0E0 RTS stored as FrameHookVec at $E0CE.

L-016 audit table (site: select, value source) is reproducible with the recipe in leads.json; examples: $C24E R6 #4 /
R7 #5 (PpuQueue), $CF90/$CFB2/$CFD4 R6 = AreaMapBank,y, $E21E/$E22E R6 #2 / R7 #3 (sound far-call), $E2B3/$E2DB R7 #$0D,
$EFAD/$EFBD R6 #$0C / R7 #$0D (RunScript); all others immediate 0-7 or PLA restores; $E18F loop and $F702/$F865 are CHR.

## Rejected / negative results
- Native islands: only 5 `$2F ... JSR $F363` islands exist (r01-a/native_islands.txt); other script-called native code
  is reached through `$38` targets instead.
- frontier.py: every existing Code region's control flow already lands in Code or classified inline data; no new
  frontier from the current map alone (new roots come only from scripts, hooks and observed traces).
- Bank 1 code-plausible residues ($A3F5-$A51B 303 B, $B424-$B475, $A57D-$A5DD, $A1B9-$A20D, $AEFF-$AF36): no JSR/JMP
  or little-endian word reference anywhere in PRG (direct search) — not proposed.

## Unresolved questions
- AreaMapBank has 13 entries ending in 4 (AreaIndex 12 -> bank 4?); bank 4 also holds executed code — not claimed.
- Exact split of the $E8E0-$E921 word table (AreaIndex domain is 13 words).
- Title script waits with `WaitButton mask 0` ($AC6A); how START leaves it (hook/stack reset) was not traced.
- Outcome $83 writer and FEV_ROD_BROKEN writer still unknown (commentary questions).
- Round-1 static conflicts at file 0x031F9 (bank 1 $B1E9) and 0x08013 (bank 4) remain.

## Deferred avenues (with reasons)
- Banks 0/1 residual (~1.5 KB after the batch): needs a split lo/hi table or RTS-dispatch search (fish-AI behaviour
  tables); direct references are absent (negative evidence above). Concrete next method; deferred for this attempt's size.
- Banks 2/4/6/7 residual (492/103/458/783 B): sound stream format (commands documented in commentary) and bank-6/7
  graphics/animation formats (CallBank6Gfx) would bound them; concrete, not started (time), no blocker.
- L-008 natural bite: artificial bite suffices for all fight experiments; no classification value now.
- S15 boat damage/return to port: not attempted (lower priority than the ending/Hawaii milestones).

## Recommendation
Run another **disassembly** cycle on the ready batch (largest gain so far, ~13.6 KB projected), then commentary for the
new code/scripts, then redirect continuing with the deferred residual-bank avenues above.

## Leads
Consumed/resolved: L-001, L-002, L-003 (milestones visited), L-011 (resolved), L-007 (superseded by L-016).
Updated: L-004, L-005, L-009, L-010, L-012 (ready), L-008 (deferred). Produced: L-013, L-015, L-016 (ready).

## Toolkit findings
No new toolkit issues in this phase; TK-001 (`buttons` input) and TK-003 (trace slowdown, ~2.5 fps observed again)
still apply. Report: analysis/toolkit-issues.md.
