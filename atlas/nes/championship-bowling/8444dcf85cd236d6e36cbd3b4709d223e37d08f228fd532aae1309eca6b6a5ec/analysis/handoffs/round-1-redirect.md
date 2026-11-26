# Round 1 redirect handoff

**Recommendation: another disassembly/commentary cycle.** The ready leads are L-011, L-012 and L-013.

## Scope and identity

- Input checkpoint: **pass-001**. maps/pass-001.json, maps/current.json and analysis/state.json are identical
  (sha256 f73ff2c8...c35c3a). Commentary round 1 is game.asm 2a0fa2e1....
- ROM classification is unchanged at 100.00% (65552/65552; 0 unknown).
  - Proposed byte reclassifications: none.
  - Accepted this phase: none. Redirect does not change the map.
  - Proposed provenance changes: up to about 2.8 KB of Static code → Observed (L-012).
- This phase did not modify the map, the state, the commentary source, the ROM or the toolkit.
- Attempt directory: analysis/redirect/r1-a1/.
  - README.md has the experiment table.
  - tools/ holds the scripts.
  - evidence/ holds the coverage dumps, three snapshots, montages and inputs.log with every press and RAM write.
- Server: started through tools/rpcd.py (socket /tmp/r1a1.sock) and stopped at the end.

## Findings

1. **Leaving the game-over scorecard (natural).**
   - ShowScorecard polls Left/Right to change the page and **B** to close. Start and A are not polled, which explains the pass-001 "stuck" scorecard.
   - Pressing B leads to ShowFinalResults: the "HI SCORE / A PRIZE WINNER!" results screen, which runs once per bowler.
   - After that, WaitAnyButton returns to the title loop (E1/E1t, S11).
2. **Strike/spare (natural).** Recipe from snap_lane1:
   - Hold Left for 60 frames (LaneBallX = 208).
   - Press A when HookCurveIndex reads 6. The released value is 7.
   - Press A when PowerLevel reads 16. Hold each meter press at least 6 frames.
   - Results: strikes in frames 1-3, with the StrikeStreak 1/2/3 celebrations. The same throw on a 9-pin leave makes a spare.
   - This is deterministic: traced and untraced runs gave identical results (E4/E4t, S12).
3. **Hi score (natural) and awards (artificial).**
   - A natural full game with the recipe scored **235**. B, then the results screen, inserted it at rank 1 as "AAA", with sound 9 and the name revealed letter by letter (E5, S15).
   - $C2-$C7 *is* the HighScores table. The pass-001 S8 patch failed only because the table is updated on the results screen, after the scorecard.
   - Artificial totals (300/275/250 for three bowlers) executed every UpdateHighScores insertion path and all three award scenes (E2/E2t, S13):
     - 300: a convertible car;
     - 275: "SPORTS NEWS / BCD TV";
     - 250: a beach.
4. **Title controls (L-009) and 3P/4P (L-007) confirmed at runtime (E3, S14).**
   - Right/Left change the number of players and Up/Down change the lane, each with wraparound.
   - Bowlers 1 and 3 use controller 1; bowlers 2 and 4 use controller 2.
   - A 4-player selection and a lane-2 start were reached naturally.
   - Snapshot: evidence/e3_4p_lane.json.
   - E6 (natural, traced): 4P lane-2 throws from e3_4p_lane.json, port following CurrentPlayer.
     - All four bowlers bowled: ActivePad alternated 0/1/0/1 and NextPlayer rotated 0→1→2→3.
     - The lane-1 strike recipe gives 8 then 4 on lane 2 (marks $8C/$4C/$4C/$40), since lane hook adjustments differ.
     - Files: evidence/e6t_4p_throws.png, e6t_coverage.json.
5. **L-011 ($A26A).**
   - `lda $A26A,x` (x = 0..$8F) reads a 144-byte table at $A26A-$A2F9.
   - The table's row 0 (60 BE BF 61 06 06) equals row 0 of LaneTopRows at $A396, which DrawLaneOverview uses to generate the same overview.
   - So the RTS opcode is deliberately shared as tile 0. Keep it as code and add an annotation.
6. **L-002 islands: structure supported.**
   - Each island's immediates name the data block that directly follows it, with sizes that match the copy (8x3 tiles + 3 attributes; 6 attribute bytes).
   - The $9FFA word is a frame pointer to the $FE byte just before code $9FFB.
   - An independent twin exists: frame table $BB33 holds $BB77, a $FE byte just before code $BB78, and that code ran naturally in E4t.
   - Reachability is unresolved, because no reference exists.
7. **Commentary correction (L-013).**
   - "SpinMeterUnused" $065C-$065F is OamBuffer sprite 23, the spin-meter marker. It is DMA'd through CopySpritesToDmaPage and hidden by HideSprite23 (`ldy #$5C`).
   - LaneBall* ($0658-$065B) is OamBuffer sprite 22.
8. **Coverage.**
   - Natural traces (E1t, E3, E4t, E6t) executed instructions in **65 Static code regions** (2809 region bytes, 922 instruction starts).
   - E2t (artificial) adds 4 regions (526 bytes), plus 247 starts in regions that natural runs also hit.
   - Region list: evidence/static_regions_observed.json.

## Ready leads (analysis/leads.json)

- **L-012 (high): Static → Observed provenance.**
  1. map.load analysis/state.json.
  2. For each region under `natural` in static_regions_observed.json:
     - if every instruction start in the region is covered, set provenance to Observed;
     - otherwise split the region at instruction boundaries (prg_banks [0,1,2,3]) and upgrade only the covered part.
  3. Source: the cited coverage files, filtered to drop `context.artificial == "true"`.
  4. Note the artificial-only regions in their evidence text and keep them Static.
  5. Validate, then checkpoint pass-002.
  - No code/data boundary change is expected.
  - Replay scripts are deterministic from the cited snapshots: tools/e1t.py, e3.py, `e4.py trace`, e6.py. Coverage: e1t/e3/e4t/e6t_coverage.json.
- **L-011 (low, annotation only):** add a note on the $A26A code region (phys 8826) and on data region phys 8827-8969 that the table starts at $A26A. Do not reclassify any byte.
- **L-013 (medium, commentary):**
  - Rename SpinMeterUnused to an OAM sprite-23 symbol and fix the "never read" claim.
  - Mark LaneBall* as sprite-22 fields.
  - Update uncertainties.md with the runtime confirmations:
    - L-007/L-009;
    - B closes the scorecard;
    - strike/spare marks;
    - the hi-score flow;
    - the award scene descriptions.

## Lead status changes

- Resolved:
  - L-006: strike/spare and hi score visited;
  - L-007 and L-009: confirmed at runtime.
- L-002: verified-structure; reachability unresolved.
- L-005: superseded by L-012.
- L-001: deferred (see below).
- L-003 and L-004: unchanged; their remaining work is formatting.
- L-010: open (commentary formatting). The recipe is refined: DrawObject also ends a frame when the sprite budget runs out, so some record lists have no $FF terminator.

## Deferred avenues (with reasons)

- **L-001 / L-002 reachability** (F3C9-FF09 library, DebugActorFrameViewer, DrawBlock9FFB/A045, SetFrameAreaAttrs, FinalAwardsTest, KnockDeckPinSideways, etc.).
  - A word-pattern search of PRG (entry and entry-1) finds no reference.
  - The ROM has no indirect dispatch other than the sound table.
  - None of these routines ran in any milestone reached this round: game end, results, hi score, awards, strikes/spares or 4P.
  - The missing fact is a selector, and none exists to trace. This is terminal for reachability, not for structure.
  - Their structural classification has no counterevidence.
- **Natural award scene (≥ 250).** The natural game reached 235, and the award code is already exercised artificially.
  - A natural 250+ game needs a better 10th-frame and spare recipe.
  - The only gain would be "natural" provenance for about 0.5 KB.
  - Optional, low priority. Retry by refining strike.py on 9-pin leaves (e.g. aim 218 → 208 with a hook).
- **Lanes 3-5 and the remaining celebration variants** (TilesA/B, Pair/Walk are chosen at random).
  - The code is shared and parameterised: AdjustHookForLane and the BallSpeedTable rows.
  - Coverage gain would be a few branch bytes.
  - Recipe: title Up ×N, then E4's recipe. The aim needs re-tuning per lane.
- **L-010 data splitting.** This is pure source formatting with no classification question, so it belongs to commentary.

## Toolkit issues

- No new toolkit issues; TK-001 to TK-004 are unchanged.
- TK-004 (slow tracing) again dominated this phase's runtime: about 3.5 traced frames/s.
- emulator.run_until with per-operation memory conditions was too slow for waits on the scale of frames. This matches its documented per-operation semantics, so it was not filed. Per-frame polling was used instead.

## Markdown files created this phase

- analysis/handoffs/round-1-redirect.md (this file)
- analysis/redirect/r1-a1/README.md
