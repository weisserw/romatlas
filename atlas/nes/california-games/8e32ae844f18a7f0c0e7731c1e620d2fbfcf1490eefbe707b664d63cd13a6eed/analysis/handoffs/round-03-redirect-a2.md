# Round 3 - redirect phase handoff (attempt r03-a2, continuation of r03-a1)

Status: **FINISHED** (nothing to resume). Recommendation: **one commentary cycle** that applies L-R3R-01..09 together
(disassembly step has no proposals). After it, unless TK-04 toolkit support is authorized, the next redirect should
report terminal incomplete (3 B) and final editorial audit can follow.

## Scope and checkpoint
- Input unchanged: maps/round03-final.json = maps/current.json = analysis/state.json (aab2fb6c...), 131085/131088
  classified, 3 B unknown (TK-04; r03-a1 rejection review stands, not redone). Source commentary/source/game.asm
  b70bdbb2...; ROM 8e32ae84... (all re-checked: analysis/redirect/r03-a2/state.before.sha).
- Only emulator calls; research records in analysis/redirect/r03-a2/ (NOTES.md, tools/, evidence/, leads.before.json).
  leads.json appended (L-R3R-05..09, L-R3R-02 attempt note, pass record). r03-a1 untouched.
- Proposed versus accepted bytes: 0 / 0.

## Findings (all commentary-semantic; exact names/comments in analysis/leads.json `proposed_names`)
| Lead | Content | Evidence type |
|---|---|---|
| L-R3R-05 (high) | BMX air/trick RAM contract ($A2-$A4,$A9,$9E/$9F,$BB,$C3,$C5,$CA-$D4). Corrections: `BmxLand` is the takeoff initialiser; BmxTrickBonusTable order is reversed vs the BmxTrickBonus0-3 labels/headers (Up->$EC06, Down->$EBFB, Left spin->$EBF2, Right spin->$EBE8); `BmxScoreTrick` = course finish + time bonus; `BmxUpdateTrick` = hazard-tile check ($D8-$E1/$F0-$F3 wipe-out, $29/$01 finish); air-score index 21 reads past both 21-entry tables (into CallJumpPtr's $6C) | static + 3 natural watches + 1 artificial ($A4 write); score 1510 confirms table order, 13802 shows the overrun |
| L-R3R-06 (medium) | Skating object = bouncing ball: X/depth/height/vertical speed/falling/anim fields; skater depth $98/$99; crash grace $D1; spawn-mask table $DBE8 is 4 B but indexed 0-15 (reads SurfStart code, never matching -> spawns only at CameraX $00-$3F) | static + artificial spawn image |
| L-R3R-07 (medium) | Surfing: $CB/$CC shark fin (1 in 4 at ride end, song $0C); state 5 aerial vars $C1-$C6,$D6; carve sequence $CD/$CE (+$20 to SurfPower $C7); SurfState2 carries SurfFrame's header by mistake | static + artificial fin screenshot |
| L-R3R-08 (medium) | Foot bag $DE-$E2 = seagull (X, Y, anim, timer, mode) + routine renames + hit messages | static + natural watch |
| L-R3R-09 (medium) | Half pipe trick input index (zone/stance/side/direction, 56 entries) and tables; $AB pending fall code, $BE message, $C1 side, $96 stance (0/9), trick runtime $A9/$AA/$BC/$BD/$BF/$C0. Corrects L-R3R-02: HalfPipeSaveState is gated by $AB, record 8 diverges only while a fall is pending | static + natural watches + fall screenshot |

## Remaining deferrals (with concrete reasons)
- Natural skating ball spawn / natural shark fin: reachability is static (direct jsr / branch); natural occurrence
  needs RNG matches (ball: CameraX $00-$3F and 1/256-1/32 per frame; fin: 1/4 at a state-4 time that bounded input
  variants did not move). Further attempts = RNG brute force; no naming value left.
- Surfing aerial (state 5) and BMX finish runtime confirmation: not reached in bounded watches; static control flow is
  complete and the proposals are worded from it. Reduced prospect (names would not change).
- Remaining unnamed overlay bytes (e.g. surf $C0/$CA/$D7/$D8, BMX $A0/$A1 ground/suspension, half pipe records'
  +5/$B1): roles are mechanical and already described by generated read/write lines; no additional connected question.
- No gameplay milestone remains unvisited (r03-a1: all 15 visited).

## Disk
~3.0G free; all r03a2 scratch logs deleted (evidence excerpts kept, < 100 KB). Bridge stopped.
