# Commentary progress (rounds 3-4; current: pass-004-fallback)

## Scope of this pass
- maps/current.json and analysis/state.json changed (pass-002 -> pass-003: region 1251, 23:$B402
  now Code). Round-2 commentary preserved first at analysis/round-2/commentary/ (diff -r identical,
  25 files sha256-matched), then a fresh `commentary.py prepare` (byte_identical, 1232 regions).
- Source regenerated from commentary/before/game.asm with analysis/tools/round-003-commentary/
  (round-2 tools + names_ballr3.py loaded last; names.py rewrites stale Var_/Tbl_ tokens in
  prose; emit.py adds table lists and immediate-pointer consumers to data notes). All round-1/2
  names and comments carried by bank:CPU key; only the $B402 record changed structurally
  (its unknown note replaced by the NextLevelFrom4_DeadRts dead-RTS comment).
- Readability backlog closed this round:
  - RAM: all 76 address-style Var_xxxx names replaced with descriptive names and comments
    (0 Var_ left); RatingCounters range; MMC3 shadows; sound-driver bytes; ball/pitch work bytes.
  - ROM tables: all 81 Tbl_BB_AAAA equates named with format/role notes (0 Tbl_ left).
  - Data: the 41 generic blocks with 'format not established' now name their pointer consumers;
    21 have specific format notes (rating layout/weights, velocity tables, metasprites, etc.).
  - Routines: 247 routine headers rewritten or corrected (banks 22/23 runner and CPU input,
    23 umpire object, 26/27 fielder/ball helpers, 28 batter/pitcher/catcher/pitch physics,
    31 MMC3 and rating counters, scoreboard values); misnamed entries corrected, e.g.
    ObjSetVelocity* -> RunnerPathOffset/RunnerPosPlus/Minus, FielderAxisDelta -> FielderNearBase,
    FielderSetControl -> SetThrowTargetFromPad, PitchOutsideZone -> PitchInStrikeZone,
    *Noop entries that do work (LaunchBallPresetA/B, RunnerAnimStand, DrawBatterFrame0, ...).
  - L-016 folded into BatContact, BatterGroup2Dispatch, ApplyPitchOutcome and PitchOutcome.

## Verification
- commentary.py verify: ok, byte_identical true (commentary/verification.json).
- Reading copy refreshed: commentary/reading-after.asm.
- review.json: 1232 scaffold entries reviewed with summaries/source lines; 1939 units;
  editorial_audit pending (final acceptance only).

## Remaining / deferred
- 180 routine headers (under 32 characters) remain one-line mechanical descriptions (mostly bank 20/21 menu
  helpers, shared _Return/_Cont entry points, NMI no-op hooks); they are accurate but short.
- Local branch labels remain mechanical (@skip/@join/@loop) except a few named ones.
- Game meanings listed as tentative in uncertainties.md stay unconfirmed (no runtime work).
- Editorial audit not run (reserved for final acceptance).

## Final editorial acceptance (pass-003, resumed in place)
- Map/state unchanged since prepare, so the source was edited in place (no re-prepare). Edits were made
  as scripted exact replacements plus token renames, with review.json source lines remapped.
- Folded in redirect r003-a and L-016/L-018, and fixed misleading names and headers. Details are in
  commentary/editorial-audit.md.
- review.json: 1232 scaffold entries and 1937 units, all reviewed. 2 duplicate-equate units were
  removed (TeamRosterPtrs, TeamInfoPtrs). editorial_audit is reviewed.
- Audit: `commentary.py audit` ok, byte_identical true, reviewed_regions 1232, reviewed_units 1937,
  editorial_audit true (commentary/audit.json). Reading copy refreshed.
- Deliberately kept: terse but accurate helper headers and mechanical local branch labels.
- The 2 unknown bytes remain unknown, so whole-ROM classification is incomplete.

## Round 4: rebase onto pass-004-fallback and final editorial acceptance
- Map change: pass-003 -> pass-004-fallback adds exactly 2 one-byte Manual Data regions
  (1252 at phys 230960 = 28:$8620, 1253 at phys 235667 = 28:$9883), the former Unknown gaps.
  Classification: 393230 evidence-backed + 2 bytes user-directed manual fallback (0 unknown).
- Preserved the whole pass-003 commentary first at analysis/round-4/commentary/ (diff -r identical,
  27 files sha256-matched), then ran a fresh `commentary.py prepare` (byte_identical, 1232 regions).
- Carry (analysis/tools/round-004-commentary/carry.py): checked that the new before/game.asm differs
  from the pass-003 one only by comment lines at the 2 ranges (no directive or boundary change),
  then carried source/game.asm and review.json by physical range. Only the two old 'Unknown' notes
  were rewritten; review source lines were remapped (3167 references, 0 text mismatches).
- Inventory reconciled: 1232 scaffold entries (1230 carried with review credit, 2 now region-1252/1253
  Data instead of unknown-038630/039893, rewritten and reviewed); 1937 units carried unchanged.
- Fixed in the rewrite: the old $9883 note named the preceding routine PitchPhaseTest; the source
  label is CheckStrikeCall.
- uncertainties.md: the 2 bytes are manual-fallback Data, not Unknown; the stale pass-003 redirect
  question list (items resolved by r003-a) was reduced to the still-open items.
- Verification and audit results: see the round-4 section of commentary/editorial-audit.md and
  analysis/handoffs/final-editorial-pass-004.md.
