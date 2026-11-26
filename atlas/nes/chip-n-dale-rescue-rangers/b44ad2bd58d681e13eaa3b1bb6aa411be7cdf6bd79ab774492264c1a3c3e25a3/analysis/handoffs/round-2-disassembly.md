# Round 2 disassembly - handoff (pass 2, COMPLETE; ROM classification INCOMPLETE)

## Scope and checkpoint
- Input: checkpoint pass-001-final (97.60%, 6303 unknown) + redirect r1-a1 ready leads L-007..L-014
  (analysis/handoffs/round-1-redirect.md, analysis/redirect/r1-a1/unknown-ledger.json).
- Output: checkpoint **pass-002** - maps/pass-002.json, generated/pass-002,
  analysis/pass-002-verification.json (ok=true, byte_identical=true). analysis/state.json saved,
  maps/current.json exported and validated (`romatlas map validate`).
- ROM classification: 100.00% (262159 / 262160 bytes classified; 1 bytes unknown).
  Exact 99.9996%; one byte (fixed $F62C, physical 0x1F63C) remains unknown, so the ROM is not complete.

## Verification
- Evidence and scripts: analysis/evidence/pass-002/ (check.json, plan.json, tools/).
- Each candidate code range was decoded with toolkit `analysis.decode` using the ledger's
  prg_banks context. Each one splits exactly into instructions, uses only official opcodes, and
  ends with RTS/RTI/JMP or runs into classified code. Every branch, JSR and JMP target is an
  instruction start.
- All words in the split/inline handler tables point to instruction starts: MovingItem 7,
  Pickup 5, SoundCommand 8, EffectParam 5, MusicCommand 25, EnvelopePhase 5.
- A successor scan over all 556 Code regions found no control-flow target inside any new Data range.
- Provenance: 197 B Observed, because every instruction in those regions was fetched in r1-a1
  S10/S11. The rest is Static. Code reached through a table or caller has confidence 0.95.
  Unreferenced structure-only code (L-011, L-012) has confidence 0.75 and reachability is not
  claimed.
- No new emulator runs. Static evidence was enough, and the r1-a1 runtime fetches were reused.
  Progression milestones (bosses, final boss, ending, Select menu, tally, game over) were already
  visited (analysis/scenarios.json).

## Lead outcomes (analysis/leads.json)
| Lead | Outcome | Bytes |
|---|---|---|
| L-007 bank 6 moving-item handlers + tables + $F0D3/$F0E7 | verified | 702 code + 32 data |
| L-008 sound inline jump tables + command handlers | verified | 76 code + 84 data |
| L-009 Select pause $8900, tally $A410/$A487, sub-screen $A597 | verified | 749 code + 81 data |
| L-010 pickup handlers + lo/hi/sound table | verified | 111 code + 15 data |
| L-011 DemoInputRecorder $FB19-$FC48 (+ Data->Code fix $FC41-$FC48; Data id 1582 now starts $FC49) | verified (structure-only) | 304 code + 24 data |
| L-012 unreferenced helpers + $CBA2 table | verified (structure-only, 0.75) | 173 code + 10 data |
| L-013 consumer-identified tables (commentary labels) | verified | 1208 data |
| L-014 pattern data / filler | verified | 2740 data |
| $803F bank 4 (unknown-keep) | reopened via the r1-a1 overlap argument and accepted as 1-B filler data (0.6) | 1 |
| $F62C fixed (unknown-keep) | kept unknown | 1 |

## Findings
- New named code: SelectPauseScreen, LevelClearTally / LevelClearTallySetup / LevelClearTallyWait,
  SelectSubScreen, MovingItemHandler23/24/25/27, PickupHandler1/3/4, ProbeMoveHelperA/B,
  PpuCtrlMaskNmiOff / PpuCtrlSetNmiOn, FillMemoryHelper, InlinePointerDispatch,
  PlaceCarriedObjectAlt, DemoInputRecorder, TaskFreeHelper / TaskExitHelper,
  PlatformAdvanceRightEdge. Commentary data/code labels were reused where commentary already
  defined them, with regions split at those labels.

## Unresolved questions
- $F62C: a legal RTS after JMP $F721. No $F62B/$F62C word exists anywhere in PRG, so it is
  still unclear whether this byte is code or filler. Classify it only with new evidence, such as
  an RTS-trick dispatch or an observed fetch.
- Natural callers for the L-011 and L-012 routines are still unknown (raw scans in r1-a1 were
  negative).

## Deferred work
- Commentary: write full comments for the new code (Select pause screen and sub-screen, tally,
  moving-item and pickup handlers, sound commands, demo recorder) and review the generated
  names.
- Optional: L-005 format refinement of generic data. Optional: visit the zone 1-9 bosses using
  the r1-a1 warp recipe; the expected prospect is execution coverage only.

## Toolkit
- TK-004 is new: a confirmed doc/API mismatch where `map.annotate` needs `id`, and `id:0`
  creates a region.
- TK-001 (confirmed bug), TK-002 (suspected performance bug) and TK-003 (missing feature) are
  unchanged.
- None of these issues blocks work. Details are in analysis/toolkit-issues.md.
