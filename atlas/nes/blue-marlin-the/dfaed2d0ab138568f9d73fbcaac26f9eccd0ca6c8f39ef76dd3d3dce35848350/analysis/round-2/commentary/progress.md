# Commentary progress

Round-1 history (pass-001) is preserved verbatim in analysis/round-1/commentary/progress.md.

## Round 2 (2026-11-20) — complete current-source pass on checkpoint pass-002

Baseline: the map changed (pass-001 -> pass-002), so the whole round-1 commentary was copied to
analysis/round-1/commentary/ (diff -r verified, with the round-1 tools and handoff) and a fresh
baseline was prepared with `commentary.py prepare` from maps/current.json = maps/pass-002.json and
analysis/state.json (frozen baseline.json, phase1-state.json, before/, reading-before.asm).

Carry-over: commentary/source/game.asm is still generated from before/game.asm +
notes/annotations.txt by analysis/tools/commentary_build.py; every annotation is keyed by bank:CPU
address, i.e. by physical position, so names/comments carried automatically. All annotations were
re-checked against both listings (analysis/tools/r02c/carry.py ->
analysis/evidence/round-02/commentary-carry-check.txt): invalidated and rewritten were the misaligned
L-009 label (now `fn 1:BA2A FightFish_SetThresholds`), the round-1 unknown-range notes for bank 0
$8B81-$8D15 and bank 15 $E5DC (now code/data), and the redirect-question comments on the fixed-bank
wrappers (Q2/Q3). review.json: entries whose physical range and kind are unchanged keep their review
as "carried from commentary round 1"; changed/new entries are reviewed in round 2.

Scope covered: all classified source of pass-002 (29,856 code bytes, all data/text/padding) plus
notes for the 25 unknown ranges (1,044 bytes).
- New code (~5,200 bytes) named and documented: bank-0 navigation windows (BiteAlert_Show,
  SeaEvent_BoatWarning/EngineTrouble/EngineBroken, Nav_TiredMenu, Nav_PauseStatusWindow,
  portrait helpers), Fight_InitGear; bank-1 fight-fish setup (FightFish_Init, PickType,
  RollSize, ComputeWeight, Init*); bank-12/13 script natives (keep, weigh-in, records,
  level-ups, rounds, title pictures, name and password entry, password pack/verify);
  fixed-bank helpers (Mul16x16, Div32by16, FormatWeight5/Rounded, script tests, far calls,
  title IRQ chains, drifting intro objects). Script-entry addresses that had no label
  (CallTest/hook targets inside round-1 code, e.g. $ED32, $EDA0, $EF49, $F7C6) now have names.
- Builder improvements: overlapping RAM symbols resolve to the most specific one (fixes the
  round-1 `ObjPriority+7` rendering of FishSpecies); `ramx` symbols for indexed accesses
  (IntroObjSpeed vs EntryRow at $D0); data/text region notes derived from round-2 evidence
  (bank-select audit, script parser, L-015, batch-B inspection, L-018) with repeats suppressed.
- Interpretations corrected from new evidence (round-1 defects): TimeOfDayScene -> HookedFishType
  (fish kind 0-8, 0-4 marlin; IsFishMarlin, FishType* tables), FishPullScale -> FishSize
  (weight = size*8 + 0..7), LureWearTable -> KeepFishType (+ KeepWeightLo/Hi), TripStats/TripStats2
  -> weight records (MaxWeight*/AvgWeight*/MarlinCount/OtherFishCount/BiggestCatch/
  LandedCatchCount), PlayerStat0760/0761/LineType -> SkillLevel/MusclePowerLevel/
  BodyStrengthLevel, DayCounters -> LevelProgress, Var049F -> FightSkillScore,
  SpritePaletteRam -> TextSlots and the InfoSprite_* "palette" helpers -> InfoText_* (they set
  window-text string pointers), NameSlotX/Y and ScriptNative_NameCursor -> password-entry names,
  Bank0_8BC4/Nav_RunArrivalScript/Nav_RunPauseScript -> Nav_ShowBiteAlert/Nav_RunTiredMenu/
  Nav_RunPauseStatus, generic `...2` names (Menu_HandleInputStart, Fight_RunEventScriptAlt,
  PassiveDir*TermsDirHigh/DirLow).

Readability counts (commentary/source/game.asm, analysis/tools/r02c/labstats.py): address-only
names 0; numbered generic locals 0 (no @skipN/@loopN/@returnN, no collision suffixes); global _N
suffixes 0; raw RAM operands 0; raw ROM operands 0; 1,189 globals, 660 locals (lone @loop 64,
lone @return 94); 680 documented routine headers; all 1,044 unknown bytes noted.
review.json: 3,268 scaffold entries reviewed (2,299 carried, 969 round 2), 1,054 units, 0 code
entries without units; editorial_audit pending (final acceptance only).

Verification: commentary.py verify -> ok, byte_identical true (commentary/verification.json).
Reading copy refreshed: commentary/reading-after.asm.

Deferred: final editorial audit; ~316 named globals without header are branch-promoted locals,
continuation labels or self-explanatory one-liners; Password_UnpackState is documented as the
inverse of Password_PackState without per-field prose.
