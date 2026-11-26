# Round 2 disassembly handoff

## Scope

- One disassembly pass. Input checkpoint: pass-001 (state.json = current.json, sha256 f73ff2c8...).
- The map was restored with map.load from analysis/state.json. All classifications were kept, and the baseline was not re-run.
- Work done:
  - consumed the ready leads L-011, L-012 and L-013;
  - ran natural gameplay to extend reachability evidence.
- The pass ended normally. The serve process (socket /tmp/r2.sock) was stopped. Scratch space (/tmp/r2scratch) was used only for working copies.
- Not touched: the commentary/ directory, the ROM and the toolkit.

## Checkpoint and verification

- **pass-002**:
  - maps/pass-002.json
  - generated/pass-002
  - analysis/pass-002-verification.json
- Verification: ok=true, byte_identical=true.
- Map sha256 1476b0299cd4f699d8490af33ee936d81843acada8c9d590ce7468a8754830fb. It is identical to analysis/state.json and maps/current.json.
- ROM classification: 100.00% (65552 / 65552 bytes classified; 0 bytes unknown).
  - The project is still incomplete: 2228 Static code bytes have no observed natural execution, and the commentary/editorial gates are separate.
- The map has 407 regions.
  - Code is 13859 bytes: **Observed 11631** (was 8077) and **Static 2228** (was 5782).
  - Data 18548, Text 116, Padding 261, Image 32768.
- Per-bank figures are in analysis/progress.md under "Round 2 disassembly".

## Findings

1. **L-012 (verified).** tools/apply.py did the following:
   - Decoded every Static code region with prg_banks [0,1,2,3]. Each decode ended exactly at the region end, and every covered offset was an instruction start.
   - Upgraded regions whose instruction starts were all covered to Observed: 87 regions.
   - Split 11 partially covered regions at instruction boundaries into 46 pieces.
   - The coverage used was natural only (`context.artificial` entries excluded): round 1 E1t/E3/E4t/E6t plus this pass's E7t/E7t2/E9a/E9b/E8/E10t.
   - No byte changed kind. The per-region log is analysis/evidence/pass-002/apply_log.json.
2. **Award code reached naturally.** In round 1, the award code ran only in E2t, a RAM-patched run. This pass reached it with natural inputs:
   - Award275 $C288, Award250 $C7AB, $C3D5 and $E1F7.
   - So no region now depends on artificial-only execution.
   - E2t still shows only that the code executes, not that it is naturally reachable.
3. **Natural milestones** (analysis/scenarios.json):
   - S16: a natural **300 perfect game** and the car award. The per-ball recipe list is in gameplay.md. Snapshots: e7_frame10_start.json and e7_scorecard_300.json.
   - S17: a natural 299 → the 275 award (SPORTS NEWS).
   - S18: a natural 267 → the 250 award (beach).
   - S19: the unskipped title animation (power-on, no input).
   - S20: lane 4, with:
     - a non-default bowler;
     - the name/weight/cursor wrap paths;
     - B during aiming → scorecard pages 1/2.
   - S21: a 2P game to the end, with results.
4. **L-011 (verified).** Evidence notes were added to code region 94 (it ends with the $A26A RTS) and to data region 95. No byte was reclassified.
5. **L-013.** No map evidence or label carries the SpinMeterUnused claim, so the map needed no change. The fix remains a commentary task.

## Unresolved

The remaining Static code is 2228 bytes in 67 regions:

- **1461 bytes: L-001/L-002.** These are the uncalled library routines (F3C9-FF09 etc.) and the code islands.
  - They have no ROM references and no selector exists.
  - Reachability is deferred; their structure is supported.
- **About 0.77 KB: small unobserved branches (L-014).** These include:
  - hi-score insertion at rank 2/3 ($BD33, $BD6C);
  - lane 5 hook branches;
  - residual 10th-frame mark/score branches;
  - CelebrationTilesA (a random choice);
  - TitleMenu branches.
- Lane 5 itself was not visited. The title lane input reached lane 4 because presses during the title animation were ignored.

## Deferred

- **L-014** gives provenance only. Its recipe is in leads.json: two natural games over 200 in one session, title Up ×5 after the animation, and repeated strikes for the random celebration.
- **L-001/L-002:** do not repeat without new evidence.
- **L-003/L-004/L-010:** formatting and semantics only, for commentary.

## Leads

- Consumed: L-011 (verified), L-012 (verified), L-013 (map side checked; still ready for commentary).
- Produced:
  - L-014 (open, low): residual natural coverage.
  - L-015 (ready, medium): commentary runtime confirmations — natural awards/thresholds, the title animation, scorecard paging during aiming, and the 2P end.

## Toolkit

- No new issues. TK-001..TK-004 are kept; see analysis/toolkit-issues.md.
- TK-004 (slow tracing) again dominated runtime, at about 25 minutes per traced results sequence. A note was appended to TK-004.

## Files

- Evidence: analysis/evidence/pass-002/
  - coverage dumps, snapshots and montages;
  - inputs.log, logs/ and tools/;
  - apply_log.json.
- Updated:
  - analysis/progress.md
  - analysis/investigation.md
  - analysis/scenarios.json
  - analysis/leads.json
  - analysis/gameplay.md
  - analysis/toolkit-issues.md
- The only Markdown file created this phase is this handoff.
