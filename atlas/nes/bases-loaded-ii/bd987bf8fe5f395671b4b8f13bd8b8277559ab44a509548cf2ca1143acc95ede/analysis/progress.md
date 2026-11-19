# Disassembly progress

Workspace initialized. Baseline and classification have not run.

## Coordinator status (2026-11-14)

- Round: 1 (not started)
- Current phase: 1 — prompts/disassembly.txt (pending; no worker launched)
- Input checkpoint: none (fresh workspace; maps/current.json is the initial map)
- Output checkpoint: none
- Leads consumed/produced: none
- ROM classification completion: not measured (baseline not run)

### Blocker: shell execution unavailable in this session

Every Bash invocation, including `echo hi`, fails before running the command with:

    apply-seccomp: write /proc/self/setgroups (nested userns is capability-restricted; caller must provide CAP_SYS_ADMIN): Permission denied

The OS-level command sandbox (bubblewrap) cannot create its user namespace on this
host, so neither the coordinator nor any phase worker can run the configured
romatlas binary (workspace.json) or run.py. Phase workers use the same Bash tool and
sandbox, so they would hit the same failure. No disassembly phase worker was launched,
and no workspace state, map or evidence was changed apart from this note.

Resume: once shell execution works (the host grants the sandbox the namespace
capability, or the sandbox configuration is changed by the user), start round 1 at
phase 1 (prompts/disassembly.txt) from the initial workspace state. archive-options.json
has not been written because there are no final documents yet.

## Coordinator status (2026-11-18) — blocker cleared

- Shell execution works again in this session; the 2026-11-14 seccomp/userns blocker no longer reproduces.
- Round 1, phase 1 (prompts/disassembly.txt): worker launched 2026-11-18 from the fresh initial state
  (no baseline, empty map, no leads). Handoff expected at analysis/handoffs/round1-disassembly.md.

## Round 1, phase 1 — disassembly pass (2026-11-18)

ROM classification: 95.44% (375320 / 393232 bytes classified; 17912 bytes unknown).

- Game: Bases Loaded II: Second Season (Jaleco), MMC1, PRG mode 2 (bank 0 fixed at $8000,
  16 KiB bank switched at $C000; observed contexts always prg_banks [0,1,2S,2S+1]).
- Baseline: `python3 run.py baseline` → maps/baseline.json, generated/baseline, byte-identical (raw only).
- Checkpoint (current): maps/round1-disassembly-b.json, generated/round1-disassembly-b,
  analysis/round1-disassembly-b-verification.json — ok, byte_identical true,
  map sha256 2b95d9dd93359eeb02fdbaaf26398dc18e4c29165fc77419a0c72eb38517ccac.
  Superseded same-pass checkpoint maps/round1-disassembly.json (95.33%; dispatcher heuristic
  later corrected, see investigation.md) is kept with its verification report.
- Internal store analysis/state.json saved; maps/current.json exported; map.validate ok (1,727 regions).
- Totals: Code 48,229 (30,270 Observed / 17,959 Static), Data 112,707, Padding 83,298,
  Image (CHR) 131,072, Text 14, Unknown 17,912. Execution coverage: 14,438 unique observed
  instruction starts (all in ROM) — a separate metric from classification.

| bank | size | code (observed) | data | padding | image/text | classified | unknown |
|---|---|---|---|---|---|---|---|
| chr | 131072 | 0 (0 obs) | 0 | 0 | 131072 | 131072 | 0 |
| header | 16 | 0 (0 obs) | 16 | 0 | 0 | 16 | 0 |
| prg00 | 16384 | 7948 (7135 obs) | 561 | 6853 | 0 | 15362 | 1022 |
| prg01 | 16384 | 238 (162 obs) | 14873 | 249 | 0 | 15360 | 1024 |
| prg02 | 16384 | 6168 (4928 obs) | 5258 | 169 | 0 | 11595 | 4789 |
| prg03 | 16384 | 397 (335 obs) | 10990 | 3131 | 0 | 14518 | 1866 |
| prg04 | 16384 | 5332 (749 obs) | 540 | 8151 | 0 | 14023 | 2361 |
| prg05 | 16384 | 382 (201 obs) | 12207 | 3247 | 0 | 15836 | 548 |
| prg06 | 16384 | 3044 (1369 obs) | 258 | 12939 | 0 | 16241 | 143 |
| prg07 | 16384 | 1792 (1413 obs) | 3524 | 10784 | 0 | 16100 | 284 |
| prg08 | 16384 | 1570 (1488 obs) | 12354 | 2334 | 0 | 16258 | 126 |
| prg09 | 16384 | 1568 (0 obs) | 12354 | 2336 | 0 | 16258 | 126 |
| prg10 | 16384 | 180 (91 obs) | 9175 | 6377 | 0 | 15732 | 652 |
| prg11 | 16384 | 2168 (1221 obs) | 715 | 11473 | 0 | 14356 | 2028 |
| prg12 | 16384 | 2508 (2106 obs) | 5251 | 8454 | 0 | 16213 | 171 |
| prg13 | 16384 | 1465 (808 obs) | 10005 | 4689 | 0 | 16159 | 225 |
| prg14 | 16384 | 247 (185 obs) | 12928 | 1840 | 0 | 15015 | 1369 |
| prg15 | 16384 | 13222 (8079 obs) | 1698 | 272 | 14 | 15206 | 1178 |

- Evidence: traced scenario coverage analysis/evidence/round1/cov_*.json (scenarios in
  analysis/scenarios.json), static traversal analysis/evidence/round1/walk.json, proposed
  regions analysis/evidence/round1/regions.json, unknown ranges
  analysis/evidence/round1/unknown_ranges.txt. Tools: analysis/tools/*.py
  (static_walk -> build_regions (+classify_gaps) -> apply_regions save; rerun reproduces regions.json exactly).
- Leads: produced L-001..L-006 (analysis/leads.json); none consumed (none existed).
- Toolkit: TK-001 (missing feature, substantial: MMC1 static discovery stops at JSR/indexed stores),
  TK-004 (missing feature: trace aggregation ~12x slowdown), TK-002/TK-003 (suspected doc mismatches);
  no blockers. See analysis/toolkit-issues.md.
- Status: INCOMPLETE (17,912 bytes unknown; no full-classification claim).
- Handoff: analysis/handoffs/round1-disassembly.md

## Coordinator: round 1 phase 1 done (2026-11-18)
- Output checkpoint: maps/round1-disassembly-b.json (generated/round1-disassembly-b); verified byte-identical.
- ROM classification: 95.44% (375320 / 393232 bytes classified; 17912 bytes unknown) — printed to user.
- Leads produced L-001..L-006 (L-001,L-002,L-003 ready). Handoff: analysis/handoffs/round1-disassembly.md
- Round 1 phase 2 (prompts/commentary.txt): worker launched. Handoff expected at analysis/handoffs/round1-commentary.md.

## Round 1 commentary (2026-11-18)
Commentary (INTERRUPTED: coverage complete, depth pass remaining) on maps/round1-disassembly-b.json: all classified source named/commented,
unknown ranges annotated; verify byte_identical true; reading copy refreshed.
Handoff: analysis/handoffs/round1-commentary.md.  Classification unchanged (95.44%).

## Coordinator: round 1 phase 2 commentary, part 1 (2026-11-18)
- Gate NOT met (interrupted): summary-level only in remaining ranges listed in analysis/handoffs/round1-commentary.md; ~600 helpers mechanical-only; local branch labels address-based.
- Verification: commentary.py verify byte-identical; reading copy commentary/reading-after.asm refreshed. Map unchanged (95.44%).
- Leads: consumed L-006; produced L-007, L-008, L-009. Toolkit issue TK-005 (non-blocking).
- Resuming phase 2 (same phase) with remaining-range handoff; continuation handoff expected at analysis/handoffs/round1-commentary-2.md.

## Coordinator: round 1 phase 2 commentary complete (2026-11-18)
- Gate met per analysis/handoffs/round1-commentary-2.md: all listed ranges commented, no mechanical-only summaries or address-only labels remain; review.json 1966/1966.
- Verification: commentary.py verify byte-identical; reading copy refreshed. Final editorial audit reserved for final acceptance.
- Leads produced: L-010 (bank15 $EFF0 code/data boundary), L-011 (biorhythm consumers). Map unchanged 95.44%.
- Round 1 phase 3 (prompts/redirect.txt): worker launched. Handoff expected at analysis/handoffs/round1-redirect.md.

## Round 1 redirect (attempt r1-redirect-a, 2026-11-18/19)
- Input checkpoint maps/round1-disassembly-b.json (95.44%; 17,912 unknown). No map/state/source changes.
- Method: survey of all 239 unknown ranges; jump-table extent proofs (20 tables) + bank-aware
  static walk (analysis/redirect/r1-redirect-a/tools/walk2.py): +6,112 code bytes, 0 clashes;
  consumer-anchored tables (2,024 B), pattern data (7,744 B), unreferenced routines (1,372 B);
  L-010 artifact proof; residual 20 bytes. Ledger: analysis/redirect/r1-redirect-a/unknown-ledger.json.
- Runtime (artificial patches): L-003 contract confirmed; batting, game end, mercy rule,
  post-season, ending credits visited (runtime.json, scenarios.json). Traced checks: 372 observed
  instruction starts inside proposed L-012 code, all matching walk instruction starts.
- Leads: produced L-012..L-015 (ready); updated L-001/L-002/L-007/L-008/L-010 (ready), L-003 verified,
  L-009/L-011/L-004/L-005 deferred notes.
- Toolkit: TK-004 addendum only.
- Recommendation: another disassembly cycle. Handoff: analysis/handoffs/round1-redirect.md

## Coordinator: round 1 phase 3 redirect done (2026-11-19)
- Handoff analysis/handoffs/round1-redirect.md; research analysis/redirect/r1-redirect-a/. Map unchanged (95.44%).
- Ready leads produced: L-012 (20 bounded jump tables + targets, consumes L-002/L-007/L-008), L-013 (inline tables), L-014 (data islands), L-015 (uncalled routines, medium/low), L-010 (bank15 $EFF0 -> data). L-003 resolved: later milestones (batting, game end, password, post-season, ending) reached via RAM patches; scenarios.json updated.
- Recommendation: another disassembly cycle. Round 2 phase 1 (disassembly) launched; handoff expected at analysis/handoffs/round2-disassembly.md.

## Round 2 disassembly (2026-11-19)
ROM classification: 100.00% (393222 / 393232 bytes classified; 10 bytes unknown).
(Rounded; 10 bytes remain unknown, so classification is NOT complete.)
- Restored analysis/state.json (= round1-disassembly-b), applied verified leads via
  analysis/round2/tools/build_r2.py + apply_r2.py (478 annotations, 0 rejected; 3 generic data
  regions removed and replaced by exact jump tables). map.validate ok; state saved; maps/current.json exported.
- Checkpoint: maps/round2-disassembly.json (sha256 8b43ff3e...), generated/round2-disassembly,
  verification analysis/round2-disassembly-verification.json: ok, byte_identical true.
- New bytes by lead: L-012 code 6,112 + tables 629 + selector 11; L-013 1,994 (+15 corrected to code);
  L-014 7,744; L-015 1,368; L-010 15 (+1 byte reclassified code->data); residual review 14.
- Lead outcomes: L-010/L-012/L-013/L-014/L-015 verified (L-013 one item rejected -> code; L-015
  one island rejected); L-001/L-002/L-007/L-008 superseded; produced L-016 (10 residual bytes).
- Per-bank counts (code includes all provenance; data includes Text):

| bank | size | code | data | padding | classified | unknown |
|---|---|---|---|---|---|---|
| header | 16 | 0 | 16 | 0 | 16 | 0 |
| prg0 | 16384 | 8603 | 928 | 6853 | 16384 | 0 |
| prg1 | 16384 | 238 | 15897 | 249 | 16384 | 0 |
| prg2 | 16384 | 9272 | 6938 | 169 | 16379 | 5 |
| prg3 | 16384 | 397 | 12856 | 3131 | 16384 | 0 |
| prg4 | 16384 | 6229 | 2004 | 8151 | 16384 | 0 |
| prg5 | 16384 | 520 | 12617 | 3247 | 16384 | 0 |
| prg6 | 16384 | 3098 | 346 | 12939 | 16383 | 1 |
| prg7 | 16384 | 1792 | 3808 | 10784 | 16384 | 0 |
| prg8 | 16384 | 1648 | 12402 | 2334 | 16384 | 0 |
| prg9 | 16384 | 1646 | 12402 | 2336 | 16384 | 0 |
| prg10 | 16384 | 180 | 9827 | 6377 | 16384 | 0 |
| prg11 | 16384 | 3869 | 1040 | 11473 | 16382 | 2 |
| prg12 | 16384 | 2508 | 5422 | 8454 | 16384 | 0 |
| prg13 | 16384 | 1465 | 10230 | 4689 | 16384 | 0 |
| prg14 | 16384 | 247 | 14297 | 1840 | 16384 | 0 |
| prg15 | 16384 | 14024 | 2086 | 272 | 16382 | 2 |
| chr | 131072 | 0 | 0 | 0 | 131072 | 0 |

- Unknown: b2 $D5FE (1), $DAC9-$DACC (4); b6 $CA2E (1); b11 $C286 (1), $C9C4 (1); b15 $C8FE (1), $D9F4 (1).
- Execution scenarios: none new this pass (redirect runtime/milestone evidence reused; see analysis/scenarios.json).
- Toolkit: no new issues (TK-001..TK-005 unchanged; analysis/toolkit-issues.md).
- Status: INCOMPLETE (10 bytes unknown). Handoff: analysis/handoffs/round2-disassembly.md

## Coordinator: round 2 phase 1 disassembly done (2026-11-19)
- Input checkpoint maps/round1-disassembly-b.json -> output maps/round2-disassembly.json (generated/round2-disassembly); byte-identical, map.validate ok.
- ROM classification: 100.00% (393222 / 393232 bytes classified; 10 bytes unknown) — printed to user (rounded; 10 unknown remain).
- Leads: L-012 accepted; L-013 104/106; L-014 all; L-015 35/36 (static, conf 0.6); L-010 accepted; L-001/2/7/8 superseded; produced L-016 (10 residual bytes: lone RTS x6 + 4-byte stack restore).
- Changed ranges: analysis/round2/changed-ranges.json (478 new regions + 3 retyped tables + L-010 trim). Commentary must regenerate against new map.
- Round 2 phase 2 (commentary) launched; handoff expected at analysis/handoffs/round2-commentary.md.

## Round 2 commentary (2026-11-19)
Commentary brought onto maps/round2-disassembly.json; round-1 commentary preserved in
analysis/round-1/commentary/.  All classified source commented; commentary.py verify ok
(byte_identical true); reading copy refreshed.  Leads L-017, L-018 added.  Handoff:
analysis/handoffs/round2-commentary.md.  Classification unchanged (10 bytes unknown).

## Coordinator: round 2 phase 2 commentary done (2026-11-19)
- Gate met per analysis/handoffs/round2-commentary.md; round-1 commentary preserved at analysis/round-1/commentary/. review.json 2208/2208; verify byte-identical; reading copy refreshed; editorial audit pending (final acceptance).
- Leads produced: L-017 (bank15 $F92A/$F947 and bank11 $D08E are jump tables, retype), L-018 (debug-like code). Unknown still 10 bytes (L-016).
- Note: stale round-1 edit.py process found and killed by worker.
- Round 2 phase 3 (redirect) launched; handoff expected at analysis/handoffs/round2-redirect.md.

## Round 2 redirect (r2-redirect-a, 2026-11-19)
- Checkpoint maps/round2-disassembly.json (unchanged; 10 bytes unknown). Records: analysis/redirect/r2-redirect-a/.
- L-016 reopened on newly combined constraints (gap census, $60 vs $FF fill, intra-routine
  positions, same-bank PLA/TAY/PLA/RTS epilogue idiom x4): proposed Code Static 0.5 (unreferenced), 10 bytes.
- L-017 verified with exact bounds (regions 2081, 2082, 1111; retype only). New L-019: bank 3
  ScreenLoaderFrame inline jump table $C048-$C04D inside generic region 517.
- L-011: biorhythm record bytes +$0C..+$0E read at 14 in-play sites as random-spread bounds
  (pitch speed/curve, batted ball, CPU batter). Runtime X1-X3 inconclusive (runtime.json).
- Handoff: analysis/handoffs/round2-redirect.md. Recommendation: disassembly cycle.

## Coordinator: round 2 phase 3 redirect done (2026-11-19)
- Handoff analysis/handoffs/round2-redirect.md; research analysis/redirect/r2-redirect-a/. Map unchanged (10 unknown).
- Ready: L-016 (reopened: 7 residual ranges proposed as static unreachable code, conf 0.5, with rejection review), L-017 (relabel 3 jump tables, exact bounds), L-019 (bank 3 $C048-$C04D jump table split out of region 517). L-011 static biorhythm consumer evidence (commentary-only). L-018/L-005 static-only.
- Round 3 phase 1 (disassembly) launched; handoff expected at analysis/handoffs/round3-disassembly.md.

## Round 3 disassembly (2026-11-19)
ROM classification: 100.00% (393232 / 393232 bytes classified; 0 bytes unknown).
- All bytes now carry an evidence-backed structural classification (0 unknown). Final acceptance
  (editorial audit / commentary on the new map) is NOT done in this pass.
- Restored analysis/state.json (= maps/round2-disassembly.json); applied L-016, L-017, L-019 via
  analysis/round3/tools/apply_r3.py. map.validate ok; state saved; maps/current.json exported.
- Checkpoint maps/round3-disassembly.json (sha256 bfba143a...), generated/round3-disassembly,
  analysis/round3-disassembly-verification.json: ok, byte_identical true. ROM sha256 unchanged.
- Leads: L-016 verified (7/7 ranges accepted, 10 B Code Static conf 0.5, unreachable as far as
  known); L-017 verified (retype/label only, reachability notes on 2199/2200/1928/1930/1931);
  L-019 verified (region 517 split). Details in analysis/investigation.md.
- Per-bank changes vs round 2: prg2 code 9272->9277 (unknown 0), prg6 3098->3099, prg11
  3869->3871, prg15 14024->14026; prg3 data split only. All banks 0 unknown.
- Execution scenarios: none new (no gameplay needed; static leads only).
- Toolkit: no toolkit issues found this pass (TK-001..TK-005 unchanged; analysis/toolkit-issues.md).
- Handoff: analysis/handoffs/round3-disassembly.md

## Coordinator: round 3 phase 1 disassembly done (2026-11-19)
- Output checkpoint maps/round3-disassembly.json; verification analysis/round3-disassembly-verification.json (byte-identical, validate ok).
- ROM classification: 100.00% (393232 / 393232 bytes classified; 0 bytes unknown) — printed to user.
- Leads: L-016 accepted (7 ranges/10 B static code conf 0.5, unreachable as known), L-017 relabel done, L-019 split done. Changed ranges analysis/round3/changed-ranges.json.
- Round 3 phase 2 (commentary + final editorial audit, since all bytes classified) launched; handoff expected at analysis/handoffs/round3-commentary.md.

## Coordinator: round 3 phase 2 commentary done (2026-11-19)
- Gate met per analysis/handoffs/round3-commentary.md; round-2 commentary preserved at analysis/round-2/commentary/. verify byte-identical; reading copy refreshed. Informational audit: inventory/review checks pass; only editorial-audit acceptance pending.
- Leads: L-011 closed; produced L-020 (PitcherStintMark only set to $FF; CPU stamina pitcher change possibly unreachable).
- Round 3 phase 3 (redirect) launched; handoff expected at analysis/handoffs/round3-redirect.md.

## Round 3 redirect (r3-redirect-a, 2026-11-19)
- Checkpoint maps/round3-disassembly.json unchanged (0 unknown). Records: analysis/redirect/r3-redirect-a/.
- L-020 refuted: PitcherStintMark $06C4/$06C5 is per-side pitcher stamina, decremented per pitch via
  bank-0 pointer table $919B (sta (Math0),y $925B); runtime decrement observed; artificial low value
  triggers CPU relief. Ready (commentary-only).
- L-021 (new, ready): region 1985 bank0 $919B-$91D4 exact stamina table layout (label/split, Data).
- L-022 (new, ready): password roll range 0-25 (commentary). CONTINUE password entry visited with
  real inputs (MS-password). Low-confidence code: no contradicting references.
- Handoff: analysis/handoffs/round3-redirect.md. Recommendation: commentary-only update (+ L-021 label
  split), then final acceptance.

## Coordinator: round 3 phase 3 redirect done (2026-11-19)
- Handoff analysis/handoffs/round3-redirect.md; research analysis/redirect/r3-redirect-a/. Map unchanged (0 unknown).
- L-020 refuted: PitcherStintMark is pitcher stamina, decremented via indirect store; relief observed artificially. No contradicting evidence for 42 low-confidence code regions. All later-game/ending milestones visited (password route via real inputs).
- Ready: L-021 (split/label bank0 region 1985 $919B-$91D4, map change), L-020/L-022 commentary-only.
- Round 4 phase 1 (disassembly, L-021) launched; handoff expected at analysis/handoffs/round4-disassembly.md.

## Round 4 disassembly (2026-11-19)
ROM classification: 100.00% (393232 / 393232 bytes classified; 0 bytes unknown).
- Restored analysis/state.json (= round 3 checkpoint); applied L-021 via analysis/round4/tools/apply_r4.py.
  map.validate ok; state saved; maps/current.json exported.
- Checkpoint maps/round4-disassembly.json (map sha256 e5ad3b69...), generated/round4-disassembly,
  analysis/round4-disassembly-verification.json: ok, byte_identical true. ROM sha256 unchanged.
- L-021 verified: bank 0 region 1985 ($919B-$91D4, 58 B) split into StaminaVarPtrs, StaminaLimitPtrs,
  StaminaLimitTable0..2 (Data, Static, 0.95). No byte-count change; prg0 code 8603 / data 928 /
  padding 6853, 0 unknown. All banks 0 unknown.
- Execution scenarios: none new (all milestones already visited; static lead only).
- Toolkit: no toolkit issues found this pass (TK-001..TK-005 unchanged; analysis/toolkit-issues.md).
- Handoff: analysis/handoffs/round4-disassembly.md

## Coordinator: round 4 phase 1 disassembly done (2026-11-19)
- Output checkpoint maps/round4-disassembly.json (byte-identical, validate ok). L-021 applied: region 1985 split into 5 stamina data regions.
- ROM classification: 100.00% (393232 / 393232 bytes classified; 0 bytes unknown) — printed to user.
- Round 4 phase 2 (commentary: new labels, L-020, L-022) launched; handoff expected at analysis/handoffs/round4-commentary.md.

## Coordinator: round 4 phase 2 commentary done (2026-11-19)
- Gate met per analysis/handoffs/round4-commentary.md; round-3 commentary preserved at analysis/round-3/commentary/. verify byte-identical; reading copy refreshed; audit informational (only editorial acceptance pending). L-020, L-022 consumed.
- Round 4 phase 3 (redirect) launched; handoff expected at analysis/handoffs/round4-redirect.md.

## Round 4 redirect (r4-redirect-a, 2026-11-19)
- Checkpoint maps/round4-disassembly.json unchanged (0 unknown). Records: analysis/redirect/r4-redirect-a/.
- Letter 25 decode resolved: alias of 0 in fields, rejected as check letter, valid as key (static model +
  real-input runtime, 4/4 match; MS-password-letter25 visited).
- Pitcher byte 5 domain = {0,1,2} statically (12 teams x 8 pitchers; one type-2 per team).
- New L-023 (ready, label/split only): bank 2 TeamPitcherPtrs $FBFD + 12 x 64-byte pitcher tables to $FF14.
- L-004/L-005/L-009/L-018 reviewed; deferred/rejected with reasons. Handoff: analysis/handoffs/round4-redirect.md.
  Recommendation: commentary-only update (+ optional L-023 label split), then final acceptance.

## Coordinator: round 4 phase 3 redirect done (2026-11-19)
- Handoff analysis/handoffs/round4-redirect.md; research analysis/redirect/r4-redirect-a/. Password letter-25 decode resolved with real inputs; pitch-type byte 5 static range 0..2.
- Ready: L-023 (bank 2 $FBFD-$FF14 team pitcher pointer table + 12x 8x8-byte pitcher tables; label/split only). Remaining items deferred with exhaustion review (no classification impact).
- Round 5 phase 1 (disassembly, L-023) launched; handoff expected at analysis/handoffs/round5-disassembly.md.

## Round 5 disassembly (2026-11-19)
ROM classification: 100.00% (393232 / 393232 bytes classified; 0 bytes unknown).
- Restored analysis/state.json (= round 4 checkpoint); applied L-023 via analysis/round5/tools/apply_r5.py.
  map.validate ok; state saved; maps/current.json exported.
- Checkpoint maps/round5-disassembly.json (map sha256 d669b4ac...), generated/round5-disassembly,
  analysis/round5-disassembly-verification.json: ok, byte_identical true. ROM sha256 unchanged.
- L-023 verified: bank 2 $FBDD-$FF14 (physical 48109..48932, former generic 2108/506/2109/507) split into
  generic 32 B + TeamPitcherPtrs (24 B) + TeamPitchers00..11 (64 B each); Data, Static. No byte-count change.
  Whole ROM: code 55746 / data 123102 / padding 83298 / text 14 / CHR image 131072; all banks 0 unknown.
- Execution scenarios: none new (static lead only; all milestones already visited).
- Toolkit: no toolkit issues found this pass (TK-001..TK-005 unchanged; analysis/toolkit-issues.md).
- Handoff: analysis/handoffs/round5-disassembly.md

## Coordinator: round 5 phase 1 disassembly done (2026-11-19)
- Output checkpoint maps/round5-disassembly.json (byte-identical, validate ok). L-023 applied (TeamPitcherPtrs + TeamPitchers00..11; 32 B generic data before).
- ROM classification: 100.00% (393232 / 393232 bytes classified; 0 bytes unknown) — printed to user.
- Round 5 phase 2 (commentary) launched; handoff expected at analysis/handoffs/round5-commentary.md.

## Coordinator: round 5 phase 2 commentary done (2026-11-19)
- Gate met per analysis/handoffs/round5-commentary.md; round-4 commentary preserved at analysis/round-4/commentary/. verify byte-identical; reading copy refreshed; review.json 2223/2223.
- Produced L-024 (bank 2 D_B02_FAE9 = 12 default-lineup pointer table; 32 B generic block is lineup tail) — label/split only.
- Round 5 phase 3 (redirect) launched with instruction to batch all remaining label/split refinements in one proposal to avoid one-lead cycles; handoff analysis/handoffs/round5-redirect.md.

## Round 5 redirect (r5-redirect-a, 2026-11-19)
- Static sweep of code-referenced generic Data; map unchanged. L-024 verified. New L-025 batch (57 label/split
  items, 8320 B, Data stays Data): bank-2 TeamRosterPtrs + 12 x 400 B rosters ($E811-$FAE8), default lineups,
  bank-14 close-up sprite pointer/frame tables ($C0FA-$CA49, 1-byte boundary fix), bank-3 layout re-split,
  4 named tables, 15 small pointer tables. Handoff analysis/handoffs/round5-redirect.md.
  Recommendation: one final disassembly+commentary cycle applying L-025, then final acceptance.

## Coordinator: round 5 phase 3 redirect done (2026-11-19)
- Handoff analysis/handoffs/round5-redirect.md; batch analysis/redirect/r5-redirect-a/batch-proposal.json (57 items, 8,320 B, label/split only; includes L-024; bank 14 $CA49 boundary moves 1 byte).
- Recommendation: one final disassembly+commentary cycle applying L-025, then final acceptance without another redirect (exhaustion review in handoff; skipped families stay generic data).
- Round 6 phase 1 (disassembly, L-025) launched; handoff analysis/handoffs/round6-disassembly.md.

## Round 6 disassembly (2026-11-19)
ROM classification: 100.00% (393232 / 393232 bytes classified; 0 bytes unknown).
- Restored analysis/state.json (= maps/round5-disassembly.json; pre-edit copy analysis/round6/state.before.json).
- Applied L-025 (includes L-024) via analysis/round6/tools/apply_r6.py: 57/57 items independently re-verified from
  ROM bytes and accepted, 0 rejected (8,320 B). 48 generic Data regions replaced by 57 labelled tables + 16 generic
  remainders; region 1290 start 232025 -> 232026 ($CA4A). Data stays Data; counts unchanged: code 55746 /
  data 123102 / padding 83298 / text 14 / CHR image 131072.
- map.validate ok; state saved; maps/current.json exported. Checkpoint maps/round6-disassembly.json
  (map sha256 a5b53604...), generated/round6-disassembly, analysis/round6-disassembly-verification.json:
  ok, byte_identical true. ROM sha256 unchanged.
- Execution scenarios: none new (static batch; consumers already classified/observed).
- Toolkit: no toolkit issues found this pass (TK-001..TK-005 unchanged; analysis/toolkit-issues.md).
- Handoff: analysis/handoffs/round6-disassembly.md

## Coordinator: round 6 phase 1 disassembly done (2026-11-19)
- Output checkpoint maps/round6-disassembly.json (byte-identical, validate ok). L-024/L-025 applied, 57/57 items accepted, label/split only.
- ROM classification: 100.00% (393232 / 393232 bytes classified; 0 bytes unknown) — printed to user.
- Round 6 phase 2 (commentary) launched; handoff analysis/handoffs/round6-commentary.md.

## Coordinator: round 6 commentary done; redirect skipped by reviewed exhaustion (2026-11-19)
- Round 6 commentary gate met (analysis/handoffs/round6-commentary.md); round-5 commentary preserved at analysis/round-5/commentary/; verify byte-identical; reading copy refreshed.
- Coordinator reviewed the round-5 exhaustion review (analysis/handoffs/round5-redirect.md §Exhaustion review): all bytes classified; remaining items are semantic-only extents of generic data (acceptable per goal policy) and previously reviewed deferred leads (L-005, L-009, L-018, natural stamina relief) on unchanged premises. Round 6 introduced no new evidence; no further redirect cycle.
- Final acceptance phase (editorial audit + verification) launched; handoff analysis/handoffs/final-acceptance.md.

## Final acceptance (2026-11-19)
ROM classification: 100.00% (393232 / 393232 bytes classified; 0 bytes unknown). The map is
unchanged (maps/round6-disassembly.json = maps/current.json = analysis/state.json, a5b53604...).
- map validate ok; store validate ok. Annotations: 2,248/2,248 regions carry evidence,
  provenance and confidence.
- `commentary.py audit` ok: byte_identical true, editorial_audit reviewed, 2,248 regions and
  1,883 units. Fresh `romatlas verify` of generated/round6-disassembly: byte_identical true
  (analysis/final-acceptance-verification.json). ROM sha256 unchanged.
- Verdict: whole-ROM complete. All gates pass. Residual uncertainties are semantic only.
- Handoff: analysis/handoffs/final-acceptance.md

## Coordinator: terminal handoff (2026-11-19)
- Final acceptance passed (analysis/handoffs/final-acceptance.md; analysis/final-acceptance-verification.json): 393232/393232 classified, 0 unknown; map/store validate ok; commentary audit passed with editorial audit reviewed; byte-identical rebuild; reading copy refreshed.
- ROM classification: 100.00% (393232 / 393232 bytes classified; 0 bytes unknown).
- Residual uncertainty is semantic only (commentary/uncertainties.md); deferred leads L-005, L-009, L-018 and natural stamina relief carry recorded reasons.
- archive-options.json written after inspecting /root/romatlas/scripts/archive.py selection contract: documents = all phase handoffs, redirect notes, round-2 stats, prior-round commentary progress/uncertainties; omitted = prior-round copies of commentary AGENTS/prompt/workflow (byte-identical duplicates of current toolkit-provided instructions). No archive executed, no cleanup.
