# Disassembly progress

Workspace initialized. Baseline and classification have not run.

## Coordinator log

Coordinator: goal.md sequence disassembly -> commentary -> redirect, one phase worker at a time.
Disk note (2026-11-25): ~2.7 GiB free on / at start; keep scratch bounded.

| Round | Phase | Status | Input checkpoint | Output checkpoint | Handoff | Notes |
|---|---|---|---|---|---|---|
| 1 | disassembly (attempt 1) | interrupted (API 529) after baseline | baseline (verified) | - | analysis/handoffs/round-1-disassembly.md | TK-001/TK-002 recorded; game appears to be Championship Bowling |

## Round 1 disassembly

ROM classification: 100.00% (65552 / 65552 bytes classified; 0 bytes unknown).

Status: incomplete as a project (structural classification covers every byte, but commentary/
readability and editorial gates have not run; many Data regions are generic and reachability of
1461 bytes of uncalled code is unresolved). Pass ended normally.

- Checkpoint: pass-001 (maps/pass-001.json, generated/pass-001, analysis/pass-001-verification.json):
  ok=true, byte_identical=true, map sha256 f73ff2c8...c35c3a (= analysis/state.json = maps/current.json).
- Map: 372 regions. Code 13859 (Observed 8077, Static 5782), Data 18548 (incl. iNES header),
  Text 116, Padding 261, Image (CHR-ROM) 32768.

| Physical bank | Size | Classified | Unknown |
|---|---|---|---|
| iNES header | 16 | Data 16 | 0 |
| PRG bank 0 ($8000-$9FFF) | 8192 | Code 2812, Data 5348, Text 32 | 0 |
| PRG bank 1 ($A000-$BFFF) | 8192 | Code 5680, Data 2512 | 0 |
| PRG bank 2 ($C000-$DFFF) | 8192 | Code 972, Data 7220 | 0 |
| PRG bank 3 ($E000-$FFFF) | 8192 | Code 4395, Data 3452, Padding 261, Text 84 | 0 |
| CHR banks 0-3 | 4 x 8192 | Image 32768 | 0 |

- Evidence: static discovery from vectors + RTS-dispatch table, gap probing, hidden-code scan,
  per-gap consumer evidence; see analysis/investigation.md. Execution coverage (separate metric):
  3868 natural instruction starts observed (8077 code bytes in Observed regions).
- Scenarios (analysis/scenarios.json): visited title, menu editing, 1P/2P selection, lane play,
  full 1P game through the 10th frame and final scorecard (natural); attempted strike, hi score
  (artificial RAM edit, unsuccessful), 3-4P selection, leaving the scorecard.
- Leads: produced L-001..L-008 (analysis/leads.json); none consumed (none existed).
- Toolkit issues: TK-001, TK-002 (kept), new TK-003 (doc mismatch, discover_context entry shape),
  TK-004 (missing feature: lightweight coverage-only tracing). See analysis/toolkit-issues.md.
- Handoff: analysis/handoffs/round-1-disassembly.md.
| 1 | disassembly (attempt 2) | done; reported to user: ROM classification 100.00% (65552/65552; 0 unknown) | baseline | pass-001 (byte_identical, map f73ff2c8...) | analysis/handoffs/round-1-disassembly.md | leads produced L-001..L-008; consumed none; TK-003/TK-004 added |
| 1 | commentary | done; all 372 regions reviewed; commentary verify ok/byte_identical (coordinator re-ran) | pass-001 | commentary/source (game.asm 2a0fa2e1...) | analysis/handoffs/round-1-commentary.md | leads L-009..L-011 produced; L-002/L-007/L-008 answered statically; final editorial audit pending |
| 1 | redirect | done; recommends another cycle | pass-001 + commentary r1 | (no map change) | analysis/handoffs/round-1-redirect.md | attempt analysis/redirect/r1-a1/; ready L-011, L-012 (2809 static->observed), L-013 (commentary fix); L-002/L-001 reachability deferred (no ROM refs) |
| 2 | disassembly | done; reported to user: ROM classification 100.00% (65552/65552; 0 unknown); code observed 11631 / static 2228 | pass-001 | pass-002 (byte_identical, map 1476b029...) | analysis/handoffs/round-2-disassembly.md | consumed L-011 (verified), L-012 (verified), L-013 (commentary only); produced L-014, L-015; 407 regions (11 split) |
| 2 | commentary | done; 407/407 entries reviewed; verify byte_identical (coordinator re-ran) | pass-002 | commentary/source (game.asm 4078124d...) | analysis/handoffs/round-2-commentary.md | r1 commentary preserved at analysis/round-1/commentary/; L-013, L-015 resolved; no new leads |
| 2 | redirect | done; recommends another cycle | pass-002 + commentary r2 | (no map change) | analysis/handoffs/round-2-redirect.md | attempt analysis/redirect/r2-a1/; ready L-016 (54 data regions -> 158 split/typed), L-014 (8 regions 144 B natural observed), L-017 (commentary); L-004 via L-016/17; L-001/L-002 deferred with rejection review |
| 3 | disassembly | done; reported to user: ROM classification 100.00% (65552/65552; 0 unknown); code observed 11775 / static 2084 | pass-002 | pass-003 (byte_identical) | analysis/handoffs/round-3-disassembly.md | consumed L-016, L-004, L-014 (8 regions); 407->511 regions; $F192-$F19F Data->Padding; TK-005 added |
| 3 | commentary | done; 511/511 reviewed; verify byte_identical (coordinator re-ran) | pass-003 | commentary/source | analysis/handoffs/round-3-commentary.md | r2 commentary preserved at analysis/round-2/commentary/; L-017 resolved; no new leads |
| 3 | redirect | done; recommends another cycle | pass-003 + commentary r3 | (no map change) | analysis/handoffs/round-3-redirect.md | attempt analysis/redirect/r3-a1/; corrected r2 rejection premise with computed-transfer census -> L-001/L-002 resolved-unreachable (still code); ready L-014 (11 regions 133 B), L-018 (commentary); L-010 deferred formatting-only |
| 4 | disassembly | done; reported to user: ROM classification 100.00% (65552/65552; 0 unknown); code observed 11917 / static 1942 | pass-003 | pass-004 (byte_identical) | analysis/handoffs/round-4-disassembly.md | consumed L-014 (verified); region 379 split -> 512 regions; L-001/L-002 evidence text updated |
| 4 | commentary | done; 512/512 reviewed; verify byte_identical (coordinator re-ran) | pass-004 | commentary/source | analysis/handoffs/round-4-commentary.md | r3 preserved at analysis/round-3/commentary/; L-018 resolved; no new leads |
| 4 | redirect | done; recommends another cycle | pass-004 + commentary r4 | (no map change) | analysis/handoffs/round-4-redirect.md | attempt analysis/redirect/r4-a1/; corrected r3 left-gutter premise; ready L-019 (region 49 $8A2D 3 B observed), L-020 (commentary: sign-girl digit, CONTROL gauge); 478 B static provenance-only with dispositions |
| 5 | disassembly | done; reported to user: ROM classification 100.00% (65552/65552; 0 unknown); code observed 11920 / static 1939 | pass-004 | pass-005 (byte_identical) | analysis/handoffs/round-5-disassembly.md | consumed L-019 (verified); evidence notes on 8 gated NMI-job regions + $F656; no type changes |
| 5 | commentary | done; 512/512 reviewed; verify byte_identical (coordinator re-ran) | pass-005 | commentary/source | analysis/handoffs/round-5-commentary.md | r4 preserved at analysis/round-4/commentary/; L-020 resolved; aim-range comment fixed |
| 5 | redirect | done; recommends another (small) cycle | pass-005 + commentary r5 | (no map change) | analysis/handoffs/round-5-redirect.md | attempt analysis/redirect/r5-a1/; corrected r4 premise at $A92B (natural $D4 mark); ready L-021 (region 386 +2 B observed + data-flow evidence notes), L-022 (commentary); $FC23 15 B deferred with rejection review |
| 6 | disassembly | done; reported to user: ROM classification 100.00% (65552/65552; 0 unknown); code observed 11922 / static 1937 | pass-005 | pass-006 (byte_identical) | analysis/handoffs/round-6-disassembly.md | consumed L-021 (verified); evidence notes only |
| 6 | commentary | done; 512/512 reviewed; verify byte_identical (coordinator re-ran) | pass-006 | commentary/source | analysis/handoffs/round-6-commentary.md | r5 preserved at analysis/round-5/commentary/; L-022 resolved; 2 comment defects fixed |
| 6 | redirect | done; recommends terminal (investigation) | pass-006 + commentary r6 | (no map change) | analysis/handoffs/round-6-redirect.md | attempt analysis/redirect/r6-a1/; corrected r5 $FC23 premise -> dormant by data flow; static ledger 1937 B = unreachable 1467 + dormant 461 + impossible 9; L-023 editorial follow-up |
| 6 | coordinator review | accepted terminal investigation | - | - | (this row) | Inspected rejection review + decisive artifacts (static_ledger.json sums to 1937 = state Static code; fc23_bounds.json per-object margins; census scope covers all stores/transfers on a fully classified map). No corrected inference remains; dispositions concern provenance only, classification already 100%. |
| final | editorial acceptance | done; audit ok, editorial_audit reviewed (512 regions/988 units); verify byte_identical (coordinator re-ran verify+audit; map validate ok) | pass-006 + commentary r6 | commentary/source game.asm 4bb03ed4... | analysis/handoffs/final-acceptance.md | L-010, L-023 closed; no open leads |
| final | coordinator terminal handoff | done | - | - | (below) | archive-options.json written (35 documents, 15 omitted instruction copies). No archive/cleanup executed. |

## Round 1 commentary

- Checkpoint pass-001 (unchanged; classification still 100.00%, 0 unknown - structural only).
- Current-source readability: complete for ALL classified source (372 scaffold entries reviewed;
  0 auto/address-only code labels; routine/table/RAM/constant names and comments throughout).
- Verification: commentary.py verify ok=true, byte_identical=true; reading copy
  commentary/reading-after.asm refreshed. Final editorial audit not run (reserved).
- Leads: produced L-009, L-010, L-011; updated L-001..L-007; L-008 resolved statically.
- Handoff: analysis/handoffs/round-1-commentary.md.

## Round 1 redirect

- Input checkpoint pass-001 (unchanged; classification 100.00%, 0 unknown). No map/state/source/toolkit edits.
- Attempt dir analysis/redirect/r1-a1/ (README.md, tools/, evidence/). Handoff: analysis/handoffs/round-1-redirect.md.
- Milestones newly visited:
  - natural: leaving the final scorecard (B) -> results -> title; strikes/spares and strike
    celebrations; a full natural game of 235 with hi-score insertion; 4P title/selection/lane-2 play;
  - artificial: all three award scenes and hi-score slot insertions.
- Natural traces hit 65 Static code regions (2809 region bytes) -> ready lead L-012 (provenance upgrade).
- Ready leads: L-011 (annotation), L-012 (provenance), L-013 (commentary correction: SpinMeterUnused is OAM sprite 23).
- Resolved: L-006, L-007, L-009. L-002 verified-structure; L-005 superseded; L-001 deferred (no selector exists).
- Toolkit: no new issues (TK-001..TK-004 kept).
- Recommendation: another disassembly/commentary cycle.

## Round 2 disassembly

ROM classification: 100.00% (65552 / 65552 bytes classified; 0 bytes unknown).

Status: incomplete as a project (every byte has an evidence-backed structural classification, but
2228 Static code bytes have no observed natural execution, 1461 of them uncalled library/islands with
unresolved reachability; commentary/editorial gates are separate). Pass ended normally.

- Checkpoint: pass-002 (maps/pass-002.json, generated/pass-002, analysis/pass-002-verification.json):
  ok=true, byte_identical=true, map sha256 1476b029...4830fb (= analysis/state.json = maps/current.json).
- Map: 407 regions (372 + 35 from instruction-boundary splits). Code 13859 (Observed 11631, Static 2228),
  Data 18548 (incl. iNES header), Text 116, Padding 261, Image (CHR-ROM) 32768. No byte changed kind.

| Physical bank | Size | Classified | Unknown | Code Observed / Static |
|---|---|---|---|---|
| iNES header | 16 | Data 16 | 0 | - |
| PRG bank 0 ($8000-$9FFF) | 8192 | Code 2812, Data 5348, Text 32 | 0 | 2676 / 136 |
| PRG bank 1 ($A000-$BFFF) | 8192 | Code 5680, Data 2512 | 0 | 5268 / 412 |
| PRG bank 2 ($C000-$DFFF) | 8192 | Code 972, Data 7220 | 0 | 909 / 63 |
| PRG bank 3 ($E000-$FFFF) | 8192 | Code 4395, Data 3452, Padding 261, Text 84 | 0 | 2778 / 1617 |
| CHR banks 0-3 | 4 x 8192 | Image 32768 | 0 | - |

- Leads consumed: L-012 verified (Static -> Observed +3554 bytes; 87 regions upgraded whole, 11 split
  into 46 pieces), L-011 verified (map annotation on regions 94/95), L-013 checked (no map change needed;
  still ready for commentary). Produced: L-014 (residual natural coverage, low), L-015 (commentary
  runtime confirmations, ready).
- Scenarios (analysis/scenarios.json S16-S22): natural 300 perfect game + car award, natural 299 ->
  275 award, natural 267 -> 250 award, unskipped title animation, lane 4 with selection wrap paths and
  scorecard paging during aiming, 2P game to the end. Evidence: analysis/evidence/pass-002/.
- Execution coverage (separate metric): Observed code 11631 bytes; no region relies on artificial-only
  execution any more.
- Toolkit issues: none new; TK-001..TK-004 kept (TK-004 slow tracing again dominated runtime). See
  analysis/toolkit-issues.md.
- Handoff: analysis/handoffs/round-2-disassembly.md.

## Round 2 commentary

- Checkpoint pass-002 (map unchanged by commentary; classification 100.00%, 0 unknown - structural only).
- Round-1 commentary preserved whole under analysis/round-1/commentary/; fresh baseline prepared on
  pass-002; round-1 source and review carried by physical ranges (98 region markers updated, 35 inserted
  for the 11 split regions; 46 new split entries reviewed).
- Current-source readability: complete for ALL classified source (407/407 entries reviewed, 909 units).
- Edits: L-013 spin-meter marker = OAM sprite 23 (SpinMarker*, HideSpinMarker, PlaceSpinMarker,
  SpinMeterArc); L-015 runtime confirmations (awards, results/hi-score flow, scorecard paging, title menu,
  controllers, $A26A). Both leads resolved.
- Verification: commentary.py verify ok=true, byte_identical=true; reading copy refreshed. Final
  editorial audit not run (reserved).
- Handoff: analysis/handoffs/round-2-commentary.md.

## Round 2 redirect

- Input checkpoint pass-002 (unchanged; classification 100.00%, 0 unknown). No map/state/source/toolkit edits.
- Attempt dir analysis/redirect/r2-a1/ (README.md, tools/, evidence/). Handoff: analysis/handoffs/round-2-redirect.md.
- Static: consumer-anchored split/naming plan for 54 generic Data regions (158 pieces, 142 named; new
  tables at $8A90, $B161, $C3EE, $C87A, $CD37, $BFDE); independent sound parse bounds envelopes /
  streams and finds $F192-$F19F unreached fill (Padding candidate); L-001/L-002 reference search
  repeated with immediate pairs: none (deferral kept, rejection review in the handoff).
- Natural runs: 3P game 300/278/257 with hi-score ranks 1-3 and all three awards; lanes 2/4/5 hook
  branches; title wraps. 8 Static code regions (144 bytes) fully covered.
- Ready leads: L-016 (map data split/naming), L-014 (provenance, 144 bytes), L-017 (commentary symbols/notes).
- Recommendation: another disassembly/commentary cycle.

## Round 3 disassembly

ROM classification: 100.00% (65552 / 65552 bytes classified; 0 bytes unknown).

- Input checkpoint pass-002 (state = current map, sha256 1476b029...830fb, 407 regions), restored with
  map.load; all classifications preserved. Output checkpoint **pass-003** (maps/pass-003.json = maps/current.json =
  analysis/state.json, sha256 d9f78600...afbd5cf, 511 regions); run.py checkpoint validate + generate + verify:
  ok=true, byte_identical=true (analysis/pass-003-verification.json).
- Kinds: Code 13859 (Observed 11775 / Static 2084), Data 18534 (incl. iNES header), Padding 275, Text 116,
  Image (CHR-ROM) 32768. Only kind change: $F192-$F19F (14 bytes) Data -> Padding.

| Physical bank | Size | Classified | Unknown | Code Observed / Static |
|---|---|---|---|---|
| iNES header | 16 | Data 16 | 0 | - |
| PRG bank 0 ($8000-$9FFF) | 8192 | Code 2812, Data 5348, Text 32 | 0 | 2692 / 120 |
| PRG bank 1 ($A000-$BFFF) | 8192 | Code 5680, Data 2512 | 0 | 5396 / 284 |
| PRG bank 2 ($C000-$DFFF) | 8192 | Code 972, Data 7220 | 0 | 909 / 63 |
| PRG bank 3 ($E000-$FFFF) | 8192 | Code 4395, Data 3438, Padding 275, Text 84 | 0 | 2778 / 1617 |
| CHR banks 0-3 | 4 x 8192 | Image 32768 | 0 | - |

- L-016 verified/applied: 54 generic Data regions -> 158 pieces (30 split, 142 named) after an
  independent ROM-level recheck of every named start (analysis/evidence/pass-003/verify_plan.json: 0 unverified,
  0 name collisions). New named tables include HookCurvePtrs/HookCurves ($8A90), BallWeightDigits ($B161),
  Award275PortraitLists ($C3EE), Award250PortraitLists ($C87A), SignGirlTileLists ($CD37).
- L-004 resolved via L-016: SoundEnvelopes $E620-$E70D, SoundHeadersAndStreams $E70E-$F191, Padding $F192-$F19F.
- L-014: 8 code regions (144 bytes) Static -> Observed from saved natural r2-a1 coverage (all instruction
  starts re-decoded and checked); residual ~0.62 KB stays deferred (provenance only).
- Gameplay: no new play; milestone review S26 (all practical milestones visited earlier).
- Toolkit: TK-005 added (map.annotate needs id=0 for new regions; doc mismatch); TK-001..TK-004 kept.
  See analysis/toolkit-issues.md.
- Status: incomplete per gates only in the sense that commentary must re-sync to the 511-region map (L-017);
  structural classification has 0 unknown bytes. Handoff: analysis/handoffs/round-3-disassembly.md.

## Round 3 commentary

- Checkpoint pass-003 (map unchanged by commentary; classification 100.00%, 0 unknown - structural only).
- Round-2 commentary preserved whole under analysis/round-2/commentary/; fresh baseline prepared on pass-003;
  round-2 source carried by physical ranges (38 markers rewritten, 104 inserted, 6 .incbin lines split at new
  boundaries; no byte change).
- Current-source readability: complete for ALL classified source (511/511 entries reviewed, 987 units).
- Edits: L-017 resolved (raw table addresses -> HookCurvePtrs, BallWeightDigits, Award275/250PortraitLists,
  SignGirlTileLists, PerfectCharFrames0-3, SoundCmdDispatch-1; list tables split and documented; sound notes for
  unreferenced header/fragments and unused envelope ids); own comments for the new split pieces; L-014 wording.
- Verification: commentary.py verify ok=true, byte_identical=true; reading copy refreshed. Final editorial
  audit not run (reserved).
- Handoff: analysis/handoffs/round-3-commentary.md.

## Round 3 redirect

- Input checkpoint pass-003 (unchanged; classification 100.00%, 0 unknown). No map/state/source/toolkit edits.
- Attempt dir analysis/redirect/r3-a1/ (README.md, tools/, evidence/). Handoff: analysis/handoffs/round-3-redirect.md.
- The round-2 L-001/L-002 rejection review was corrected. It relied on an open-world reference search;
  a closed-world enumeration of computed transfers now shows the 1461 bytes lie outside the
  vector-rooted closure (statically unreachable). Their Code/Static classification is unchanged.
- The RNG contract was derived and modelled, and a natural route to CelebrationTilesA was predicted
  and confirmed.
- Natural 10th-frame spare/miss patterns and a slow roll were reached.
- Ready: L-014 (11 regions, 133 bytes Static -> Observed) and L-018 (commentary).
- Toolkit: no new issues (TK-001..TK-005 kept).
- Recommendation: another disassembly/commentary cycle.

## Round 4 disassembly

ROM classification: 100.00% (65552 / 65552 bytes classified; 0 bytes unknown).

- Input checkpoint pass-003 (511 regions) restored with map.load; all classifications preserved. Output checkpoint
  **pass-004** (maps/pass-004.json = maps/current.json = analysis/state.json, sha256 d0ea19c3...2db6137, 512 regions);
  run.py checkpoint validate + generate + verify: ok=true, byte_identical=true (analysis/pass-004-verification.json).
- Kinds: Code 13859 (Observed 11917 / Static 1942), Data 18534 (incl. iNES header), Padding 275, Text 116,
  Image (CHR-ROM) 32768. No kind change.

| Physical bank | Size | Classified | Unknown | Code Observed / Static |
|---|---|---|---|---|
| iNES header | 16 | Data 16 | 0 | - |
| PRG bank 0 ($8000-$9FFF) | 8192 | Code 2812, Data 5348, Text 32 | 0 | 2697 / 115 |
| PRG bank 1 ($A000-$BFFF) | 8192 | Code 5680, Data 2512 | 0 | 5533 / 147 |
| PRG bank 2 ($C000-$DFFF) | 8192 | Code 972, Data 7220 | 0 | 909 / 63 |
| PRG bank 3 ($E000-$FFFF) | 8192 | Code 4395, Data 3438, Padding 275, Text 84 | 0 | 2778 / 1617 |
| CHR banks 0-3 | 4 x 8192 | Image 32768 | 0 | - |

- L-014 verified/applied: 11 regions (133 bytes) Static -> Observed from r3-a1 natural coverage (instruction
  starts re-decoded and checked); region 379 split at $A84E: 9 bytes Observed, `jmp` (3 bytes, new id 512) Static.
  Observed +142 bytes.
- L-001/L-002: region evidence for the 23 runs (1461 bytes) now records resolved-unreachable (r3-a1 closure);
  structure still Code/Static.
- Residual Static code 1942 = 1461 unreachable (L-001/L-002) + 481 reachable residual branches (provenance only).
- Gameplay: no new play (S31 review; no cheap productive route). Toolkit: no new issues; TK-001..TK-005 kept
  (analysis/toolkit-issues.md).
- Status: incomplete only in that commentary must consume L-018 and re-sync the id-512 split; structural
  classification has 0 unknown bytes. Handoff: analysis/handoffs/round-4-disassembly.md.

## Round 4 commentary

- Checkpoint pass-004 (map unchanged by commentary; classification 100.00%, 0 unknown - structural only).
- Round-3 commentary preserved whole under analysis/round-3/commentary/; fresh baseline prepared on pass-004;
  source carried by physical ranges (12 markers rewritten, 1 inserted at $A84E; no byte change).
- Current-source readability: complete for ALL classified source (512/512 entries reviewed, 988 units).
- Edits: L-018 resolved (RNG/celebration contract; TILE_MISS "-", TILE_GUTTER "G", TILE_SPARE, TILE_STRIKE;
  L-001/L-002 routines reworded to "unreachable from the vectors, still code"; no pause in play); 10th-frame
  third-ball encoding and the region-512 defensive jmp; slow-roll note.
- Verification: commentary.py verify ok=true, byte_identical=true; reading copy refreshed. Final editorial
  audit not run (reserved).
- Handoff: analysis/handoffs/round-4-commentary.md.

## Round 4 redirect

- Input: checkpoint pass-004. Classification is 100.00% with 0 unknown; the map, state and source are unchanged.
- Attempt: analysis/redirect/r4-a1/. Handoff: analysis/handoffs/round-4-redirect.md.
- Recommendation: **another disassembly/commentary cycle**.
  - L-019: region 49 $8A2D Static -> Observed (3 bytes, natural left gutter E17), plus an "unreachable"
    evidence note on region 315 FillNametable2/3.
  - L-020: commentary (sign digit tiles, CONTROL label, left gutter, dormant gated jobs).
- Residual: 478 reachable Static bytes, provenance only, each group with a disposition. There are also the
  1461 bytes of L-001/L-002, which are unreachable.
- Gameplay: S32 (left gutter, bowler-2 announcement). No unvisited practical milestone remains.
- Toolkit: no new issues. The server was stopped and the scratch directory removed.

## Round 5 disassembly

ROM classification: 100.00% (65552 / 65552 bytes classified; 0 bytes unknown).

- Input checkpoint pass-004 (512 regions) restored with map.load; all classifications preserved. Output checkpoint
  **pass-005** (maps/pass-005.json = maps/current.json = analysis/state.json, sha256 951ffb73...c3d700e64f, 512 regions);
  run.py checkpoint validate + generate + verify: ok=true, byte_identical=true (analysis/pass-005-verification.json).
- Kinds: Code 13859 (Observed 11920 / Static 1939), Data 18534 (incl. iNES header), Padding 275, Text 116,
  Image (CHR-ROM) 32768. No kind or boundary change.

| Physical bank | Size | Classified | Unknown | Code Observed / Static |
|---|---|---|---|---|
| iNES header | 16 | Data 16 | 0 | - |
| PRG bank 0 ($8000-$9FFF) | 8192 | Code 2812, Data 5348, Text 32 | 0 | 2700 / 112 |
| PRG bank 1 ($A000-$BFFF) | 8192 | Code 5680, Data 2512 | 0 | 5533 / 147 |
| PRG bank 2 ($C000-$DFFF) | 8192 | Code 972, Data 7220 | 0 | 909 / 63 |
| PRG bank 3 ($E000-$FFFF) | 8192 | Code 4395, Data 3438, Padding 275, Text 84 | 0 | 2778 / 1617 |
| CHR banks 0-3 | 4 x 8192 | Image 32768 | 0 | - |

- L-019 verified/applied: region 49 ($8A2D `jmp $8A37`, left gutter clamp in UpdateLaneBall; 1 instruction start, 3 hits)
  Static -> Observed from r4-a1 E17 natural traced replay (no RAM/register edits). Whole region covered, no split.
  Observed +3 bytes. Script/log: analysis/evidence/pass-005/tools/apply.py, apply_log.json.
- Evidence text only (no kind/provenance change): 8 gated NMI-job regions (264 bytes) dormant by data flow;
  region 315 FillNametable2/3 ($F656, 6 bytes) unreachable (preceding bne always taken).
- Residual Static code 1939 = 1461 L-001/L-002 + 6 FillNametable2/3 (unreachable) + 472 reachable residual branches
  (provenance only; dispositions in analysis/handoffs/round-4-redirect.md).
- Gameplay: none (r4 S32 review: no unvisited practical milestone). Toolkit: no new issues; TK-001..TK-005 kept
  (analysis/toolkit-issues.md).
- Status: structural classification complete with 0 unknown bytes; L-020 (commentary) remains ready.
  Handoff: analysis/handoffs/round-5-disassembly.md.

## Round 5 commentary

- Checkpoint pass-005 (512 regions). Map changed, so the whole round-4 commentary moved to
  analysis/round-4/commentary/ and `commentary.py prepare` made a fresh byte-identical baseline.
- Carry by physical ranges (analysis/tools/commentary-round5/carry.py): 1 marker rewritten (region 49).
- L-020 resolved (comment-only): left gutter clamp at $8A2D reached naturally; aim range $C4-$E4 (fixed a
  $C5 slip); CONTROL label; sign digit tiles; gated NMI jobs dormant by data flow; FillNametable2/3
  unreachable; commentary/uncertainties.md updated.
- Readability: ALL classified source covered (512/512 entries reviewed, 988 units); editorial audit reserved.
- `commentary.py verify` ok, byte_identical=true (fresh at handoff); reading copy refreshed.
  Handoff: analysis/handoffs/round-5-commentary.md.

## Round 5 redirect

- **Input:** checkpoint pass-005 (512 regions, 100.00% classified, 0 unknown).
- **Attempt directory:** analysis/redirect/r5-a1/.
- **Not changed:** map, state, source, ROM and toolkit.
- **Review of the round-4 dispositions:**
  - The $A92B premise was wrong: the first-ball-miss mark $D0 is legal.
  - E19 (natural; two left gutters from snap_lane1) gave FrameMarks $D4 and covered region 386.
- **Static checks turned 188 deferred bytes into dispositions:**
  - Fade slow paths (160 B): dormant by FadeDelay data flow.
  - Envelope restart, envelope carry and attenuation clamp (8 B): dormant by ROM sound data.
  - Three pointer carries (6 B): impossible.
  - DrawObject $FD jump (14 B): unused by the ROM sprite data.
- **Still open:** DrawObject top-edge clip $FC23 (15 B, provenance only).
- **Ready leads:**
  - L-021: region 386 Static -> Observed, plus evidence text;
  - L-022: commentary.
- **Gameplay:** S33. No unvisited practical milestone.
- **Recommendation:** another (small) disassembly/commentary cycle, then terminal. The rejection review is in
  analysis/handoffs/round-5-redirect.md.

## Round 6 disassembly

ROM classification: 100.00% (65552 / 65552 bytes classified; 0 bytes unknown).

- Input checkpoint pass-005 (512 regions) restored with map.load; all classifications preserved. Output checkpoint
  **pass-006** (maps/pass-006.json = maps/current.json = analysis/state.json, sha256 865f4d5a...ee973cf93, 512 regions);
  run.py checkpoint validate + generate + verify: ok=true, byte_identical=true (analysis/pass-006-verification.json).
- Kinds: Code 13859 (Observed 11922 / Static 1937), Data 18534 (incl. iNES header), Padding 275, Text 116,
  Image (CHR-ROM) 32768. No kind or boundary change.

| Physical bank | Size | Classified | Unknown | Code Observed / Static |
|---|---|---|---|---|
| iNES header | 16 | Data 16 | 0 | - |
| PRG bank 0 ($8000-$9FFF) | 8192 | Code 2812, Data 5348, Text 32 | 0 | 2700 / 112 |
| PRG bank 1 ($A000-$BFFF) | 8192 | Code 5680, Data 2512 | 0 | 5535 / 145 |
| PRG bank 2 ($C000-$DFFF) | 8192 | Code 972, Data 7220 | 0 | 909 / 63 |
| PRG bank 3 ($E000-$FFFF) | 8192 | Code 4395, Data 3438, Padding 275, Text 84 | 0 | 2778 / 1617 |
| CHR banks 0-3 | 4 x 8192 | Image 32768 | 0 | - |

- L-021 verified/applied: region 386 ($A92B `lda #$00`, frame-total @open after a first-ball miss; 1 instruction start,
  1 hit) Static -> Observed from r5-a1 E19 natural traced replay (no RAM/register edits). Whole region covered, no split.
  Script/log: analysis/evidence/pass-006/tools/apply.py, apply_log.json.
- Evidence text only (no kind/provenance change): fade 160 B dormant by FadeDelay data flow; sound 8 B dormant by ROM
  sound data; 3 pointer carries 6 B impossible; DrawObject $FD jump 14 B unused by ROM sprite lists; $FC23 top-edge
  clip 15 B unobserved/deferred.
- Residual Static code 1937 = 1461 L-001/L-002 + 6 FillNametable2/3 (unreachable) + 264 gated NMI jobs + 188 r5
  dispositions + 3 $A84E (impossible) + 15 $FC23 (open, provenance only).
- Gameplay: none. Toolkit: no new issues; TK-001..TK-005 kept (analysis/toolkit-issues.md).
- Status: structural classification complete with 0 unknown bytes; L-022 (commentary) ready.
  Handoff: analysis/handoffs/round-6-disassembly.md.

## Round 6 commentary

- Checkpoint pass-006 (512 regions). Map changed, so the whole round-5 commentary moved to
  analysis/round-5/commentary/ and `commentary.py prepare` made a fresh byte-identical baseline.
- Carry by physical ranges (analysis/tools/commentary-round6/carry.py): 1 marker rewritten (region 386).
- L-022 resolved (comment-only): $A92B first-ball miss; FadeDelay $FF only in AnnounceBowler, stepped fades
  dormant; sound restart/carry/attenuation floor unused by ROM data; three impossible page carries; DrawObject
  $FD unused by ROM lists; $FC23 top-edge clip unobserved (open). Defect fixed: ShowLaneScreen "instantly".
- Readability: ALL classified source covered (512/512 entries reviewed, 988 units); editorial audit reserved.
- `commentary.py verify` ok, byte_identical=true (fresh at handoff); reading copy refreshed.
  Handoff: analysis/handoffs/round-6-commentary.md.

## Round 6 redirect

- **Input:** checkpoint pass-006 (512 regions). ROM classification is 100.00% (65552/65552, 0 unknown) and stays
  unchanged. Attempt: analysis/redirect/r6-a1/.
- **$FC23 reviewed, last open provenance item (L-023).** Two r5 premises were corrected:
  - the path covers any sprite Y outside 0..255, not only the top edge;
  - object Y is bounded by ROM templates and bounded writers.
  - The per-object census gives Y + offset within 9..232, so the path is **dormant by data flow**. It stays
    Code/Static.
- **Static ledger:** all 1937 Static Code bytes are dispositioned (static_ledger.json).
  - Unreachable: 1467
  - Dormant: 461
  - Impossible: 9
- **Recommendation: terminal.** Structural classification is complete. The next step is final editorial acceptance
  (commentary audit), which must fix the $FC23 comment (L-023) and handle L-010.
- No server started, no map/state/source/toolkit change. Handoff: analysis/handoffs/round-6-redirect.md.

## Final editorial acceptance

- Checkpoint pass-006 (512 regions), classification 100.00% (65552/65552, 0 unknown); no map/state change.
- L-023 applied: $FC23 comment now describes the out-of-range Y path (either edge) as dormant by data flow
  (Y + offset within 9..232); header paragraph and DrawObject $FD note (no RAM-built sprite lists) updated;
  uncertainties.md moved $FC23 to resolved. L-023 closed.
- L-010 closed by editorial decision: sprite record lists stay one labelled .incbin per frame (134 lists),
  grammar documented at DrawObject (commentary/editorial-audit.md).
- `commentary.py audit`: ok, byte_identical, 512 regions / 988 units reviewed, editorial_audit=true
  (commentary/audit.json). Fresh verify byte-identical, source sha256 4bb03ed4...1d13; reading copy refreshed.
  Handoff: analysis/handoffs/final-acceptance.md.

## Coordinator terminal handoff (2026-11-25)

- ROM classification: 100.00% (65552 / 65552 bytes classified; 0 bytes unknown) at checkpoint pass-006
  (maps/pass-006.json, analysis/pass-006-verification.json, byte_identical). Code 13859 B (Observed 11922 / Static 1937,
  every Static byte dispositioned in analysis/redirect/r6-a1/evidence/static_ledger.json); remaining non-code is data,
  text, padding and CHR Image.
- Workflow reached terminal: 6 rounds of disassembly -> commentary -> redirect; round-6 redirect rejection review
  inspected and accepted by coordinator; final editorial acceptance done (analysis/handoffs/final-acceptance.md).
- Commentary: all 512 regions reviewed; commentary.py audit ok with editorial_audit true; verify byte_identical
  (game.asm sha256 4bb03ed4...). Semantic-only open meanings remain in commentary/uncertainties.md.
- Toolkit findings: analysis/toolkit-issues.md (TK-001..TK-003 doc/API mismatches, TK-004 missing coverage-only trace mode,
  TK-005 map.annotate id doc mismatch); no blockers.
- Archive preparation: archive-options.json lists final handoffs, redirect attempt READMEs and per-round commentary
  progress/uncertainty records as documents; omits only the round-N copies of the generated commentary
  AGENTS/prompt/workflow instructions. Archive was not run.
