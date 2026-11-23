# Round 2 disassembly handoff

STATUS: COMPLETE (pass finished; ROM incomplete — 3 bytes unknown).

Scope: one disassembly pass consuming redirect r1-a7c2 ready leads L-007..L-014; state
restored with map.load (analysis/state.json from pass-001), all prior classifications kept
except region 1438 shortened by 2 B (L-014). Boss scenario (L-005/S8) re-attempted once.

Checkpoint: pass-002 — maps/pass-002.json, generated/pass-002,
analysis/pass-002-verification.json: ok=true, byte_identical=true,
map sha256 88ab22c692879ae492f6e3910e735172c7b3995429a5419d629f11d1aa61961e.
map.validate ok; maps/current.json exported; 991 regions.

ROM classification: 100.00% (262157 / 262160 bytes classified; 3 bytes unknown).
(Rounded; not completion.) Per-bank table: analysis/progress.md "Round 2 disassembly pass".

## Per-lead outcomes (all Static; none claimed reachable or unused)
| Lead | Outcome | Regions (physical) |
|---|---|---|
| L-007 | verified | Data 0x0BD0E Type12HitMotion, 0x1684B Type00MotionScript (17 B each) |
| L-008 | verified | Data 0x12100 BlinkArrowSprite, 0x13843 BlinkArrowSprite3, 0x12C7D ClearPanelsRect (3 B each) |
| L-009 | verified, conf 0.7 | Code 0x120A7 WaitButtonBlinkingReset (4 B) |
| L-010 | verified | Code 0x1C2BB(9) 0x1E948 InitMirroringVertical(22) 0x1F21D(35) 0x1F266(15) 0x1F5F6(20) 0x1F63B(10) 0x1FFEE(24); Data 0x20006(4) |
| L-011 | verified | Code 0x1EA83 DrawInlineScreen2400 (67), 0x1EB66 DecodeRleStream2 (81) |
| L-012 | verified | Data 0x1E8BB(5), Code 0x1E8C0(12, conf 0.75), Data 0x1E8CC(4) |
| L-013 | verified | Code 0x1F421(436); Data 0x1F5D5 AttrPaletteBits(16), 0x1F5E5 AttrClearMasks(4) |
| L-014 | verified | region 1438 -> 0x19FD8+92 code; Data 0x1A034 StageSelectLimits(2) |
| L-005 | attempted, deferred | ordinary play + health refills in 6-4 blocked at a wall; no boss |
Totals added: 735 code + 76 data = 811 B (+2 B code->data). Review notes: analysis/investigation.md.

## Unresolved
- L-015 (new, open, low): PRG14 $C43F-$C440 (0x1C44F-0x1C450, `01 00`) and $C587 (0x1C597, `60`).
  Compact rejection review in analysis/investigation.md (Round 2): no word, word-1 or immediate
  pointer-pair reference; code-prefix reading of `01 00` clobbers A, the input of
  PutMetatileInColumn; lone unreferenced RTS lacks structure. Reopen only with a consumer/caller.
- New code regions have no known caller (reachability unresolved, recorded per region).

## Deferred
- Bosses (S8/L-005): coverage-only value. Next practical try: stage 1 round 4 (simpler
  layout) with health refills, or decode spawn-record coordinates; TK-006 blocks PRG-RAM patching.
- L-006 semantics of generic data banks (readability, commentary).
- Commentary: new labels/regions in pass-002 need commentary (next phase).

Toolkit: no new toolkit issues found; TK-001..TK-006 unchanged (analysis/toolkit-issues.md).
Scratch: /tmp/ra2 (boss script/screenshot), /tmp/ra (round-1 scratch, 40 MB) — disposable.
