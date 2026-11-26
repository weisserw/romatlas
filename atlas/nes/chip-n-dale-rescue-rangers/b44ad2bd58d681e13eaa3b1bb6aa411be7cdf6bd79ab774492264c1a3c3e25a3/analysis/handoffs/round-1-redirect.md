# Round 1 redirect - handoff (attempt r1-a1, COMPLETE)

## Scope and checkpoint
- Input: checkpoint **pass-001-final** (97.60%, 6303 unknown bytes in 121 ranges), commentary
  round 1 (commentary/source/game.asm). Map, state, ROM, source and binary unchanged; this
  attempt changed only research/lead records: analysis/leads.json, analysis/scenarios.json
  (S10-S14, M-boss/M-ending updated), analysis/gameplay.md, analysis/toolkit-issues.md (note only).
- Research: analysis/redirect/r1-a1/ (notes.md, unknown-ledger.json, runtime-new-fetches.json,
  frames end1/end2/l10boss.png, tools/). Snapshots: analysis/snapshots/r1-a1/{z0_boss,l10,l10_boss}.json.
- Proposed vs accepted: 0 bytes accepted (redirect does not classify); proposals below cover
  6301 of 6303 unknown bytes plus an 8-byte Data->Code correction.

## Findings
1. Commentary structural questions 1-3 resolved by bank-aware static descent (no conflicts):
   bank 6 type-$01 sub-handlers, bank 3 inline jump tables and their handlers, bank 2 Select
   pause screen ($8900, + $A597 sub-screen) and level-clear tally ($A410).
2. Bank 6 pick-up handler table at $8D52-$8D60 (5 lo/hi/sound entries) proves $8C91 and
   $8D2A/$8D4A are code (two of its five targets are already-known routines).
3. Fixed bank $FB19-$FC48 is a complete (unreferenced) demo-input recorder writing run-length
   joypad logs to $6000-$67FF; its last 8 bytes lie inside the current generic Data region at
   $FC41 - the one concrete counterexample to the 0.8 low-run generic-data rule.
4. Generic-data sanity check (coordinator concern): boundary and known-entry-call scans over
   all Data regions in code banks found no other conflict; data-bank holes are visibly
   tile/spawn/metasprite arrays (notes.md section 4).
5. Runtime (artificial, bounded): Select menu, tally, zone-0 boss, final boss and ending
   visited; proposed code fetched (S10 180/257 insns, S11 78/78), boss code and two sound
   commands newly fetched. RAM contract: PendingWarp $6C (door sections), BossHits $EE=5 +
   BossSlot $EF, LevelClearDelay $6D, level 10 clear -> ending "THE RESCUE RANGERS! - END -".

## Ready leads (next disassembly pass) and expected gains
| Lead | Content | Expected |
|---|---|---|
| L-007 | bank 6 type-$01 handlers + 3 tables, fixed $F0D3-$F0FA | 734 B (702 code) |
| L-008 | bank 3 inline jump tables + 5 sound command handlers | 160 B (76 code) |
| L-009 | bank 2 $8900 / $A410 / $A597 code + tables (+25 B filler) | 830 B (749 code) |
| L-010 | bank 6 pick-up handlers + table | 126 B (111 code) |
| L-011 | fixed demo-input recorder + BlockDraw tables; split Data at $FC41 | 320 B + 8 B correction |
| L-012 | 8 unreferenced complete routines (structure-only) + $CBA2 table | 183 B (review) |
| L-013 | 49 consumer-identified tables labeled by commentary | 1208 B |
| L-014 | visible-pattern data in data banks + filler candidates | 2740 B (273 candidates) |
Total if all accepted: 6301 B (-> ~99.999%); high-confidence subset L-007..L-011 + L-013 =
3,378 B. Exact sub-ranges, contexts and rationale per row: analysis/redirect/r1-a1/unknown-ledger.json.
Recipe: annotate per ledger row (Code Static/Observed with context, Data with label, generic
Data); validate, export, checkpoint; commentary for the new code (Select pause screen, tally,
recorder, pickup handlers, moving-item handlers, sound commands).

## Rejection review (compact)
- $803F bank 4 (1 B, $BC) - claim: filler. Decisive: any instruction starting there overlaps
  observed $8040. Rules out code-at-$803F; does not establish a data role. Competing: leftover
  byte of an edited routine. Kept unknown (single isolated byte; generic data acceptable if the
  reviewer prefers, by the overlap argument).
- $F62C fixed (1 B, RTS) - claim: code. Only evidence is a legal RTS after a JMP; per policy an
  unreferenced return opcode is insufficient. Kept unknown.
- L-012 pieces - strongest competing interpretation: dead code vs coincidental decode. Each
  range is exactly tiled, ends in RTS/JMP or falls into known code, uses project RAM symbols
  and known entries; coincidence is implausible, but no caller exists. Proposed as structure-
  only Code; weakest pieces $C268 (6 B) and $F686 (6 B) may stay unknown on review.
- Generic-data candidates ($AF53, $B9A5, $E047, $8088, $87FD, $8FE6, $9FF6, $FDC1, $FFB0,
  $88ED, $A40A): no consumer found; content is noise/unofficial opcodes or table patterns, no
  control-flow target inside. Proposed generic data; reviewer may keep unknown.
- L-005 (specific formats for 75 KiB generic data): deferred; semantic refinement, not a
  classification blocker; the only code-vs-data counterexample is in L-011.

## Remaining avenues / deferrals
- Other zone bosses (levels 1-9): same recipe (door section ids in notes.md section 3); not
  visited individually - diminishing prospect (boss handlers share helpers; S12/S14 already
  fetched the boss helper set), not a missing fact.
- 2-player manual start (M-2p): deferred; demo covered 2P logic.
- Natural callers for L-011/L-012 routines: raw JSR/JMP/word/immediate-pair scans negative;
  no further method without new evidence.

## Recommendation
Run another disassembly round (then commentary) on L-007..L-014; after it, a short redirect
should only need to re-check rejected/unknown residue (<= ~300 B).

## Toolkit
No new toolkit issues (analysis/toolkit-issues.md; TK-002 slowdown reconfirmed).
Scratch dir /root/rascratch-r1 removed at handoff; tools/bridge.py and rpc.py reference it
as the scratch socket location (recreate when rerunning).
