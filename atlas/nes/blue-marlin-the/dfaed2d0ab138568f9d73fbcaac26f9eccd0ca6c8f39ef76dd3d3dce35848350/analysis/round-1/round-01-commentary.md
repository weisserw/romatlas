# Round 1 — commentary handoff

Status: **DONE** (current-source readability pass complete; final editorial audit not run, by design).

## Scope
- Baseline prepared from verified checkpoint **pass-001** (`maps/pass-001.json` = `maps/current.json`,
  `analysis/state.json`): `commentary/` created with `python3 /root/romatlas/scripts/commentary.py prepare`
  (frozen `baseline.json`, `phase1-state.json`, `before/`, `reading-before.asm`).
- All classified source covered: 626 code regions (24,646 bytes; banks 0, 1, 2, 4, 6, 7, 12, 13, 14, 15),
  1,005 data, 751 text, 10 padding, CHR image, plus explanatory notes for all 777 unknown ranges (17,738 bytes).
- Classification is unchanged (93.23%); this phase does not reclassify bytes.

## How the source is built
- `commentary/source/game.asm` (30,248 lines) is generated from `commentary/before/game.asm` +
  `commentary/notes/annotations.txt` (1.8k lines: RAM/constant symbols, routine headers, branch labels,
  data labels, operand expressions, bank overrides) by `analysis/tools/commentary_build.py build`.
  The tool only adds labels/comments/`.org`, substitutes symbols with identical values, and renders
  jump tables as `.word`/`.lobytes`/`.hibytes`; byte identity is proven by the toolkit verify.
- `commentary/review.json` filled by `analysis/tools/commentary_review.py` (scaffold untouched; all
  3,170 entries `reviewed` with summary + source line; 837 units = 464 documented routines/entry points
  + continuation units for split code regions; `editorial_audit` left `pending`).
- `commentary/progress.md`, `commentary/uncertainties.md` updated.

## Readability assessment
- No address-only routine/jump/data names remain (`sub_`/`loc_`/`dat_` count 0). Every call/jump/table
  target is named; 464 routines carry header comments (purpose, inputs/outputs, formats, bank context).
- ~330 RAM symbols (zero page, object arrays, PPU queue, clocks, boat, lure, wake, fish, fight model,
  script engine, sound driver), hardware/MMC3/button constants; MMC3 register indices and button masks
  substituted automatically where context is certain. Raw RAM operands remaining: 0.
- Remaining editorial gaps (non-blocking, listed for final cleanup):
  - ~280 mechanical local labels (`@skip`, `@skip2`, ...) inside documented routines (most others are
    condition-derived, e.g. `@no_a_held`, `@FishPull_positive`).
  - Fight fish-model term tables and fight events 1-9 (bank 1 $AD68-$BA9B) are named mechanically;
    game meaning tentative. Sound stream commands $C0-$E0 named from code paths.
  - Unknown-range notes are evidence-derived per range (text gap / code-plausible but uncalled /
    non-code-like / data-bank residue + static references); bank-0 $8B81-$8D15 has hand notes.
  - Fixed-bank operands into $8000-$BFFF stay numeric where the runtime bank is not established.

## Verification
- `python3 /root/romatlas/scripts/commentary.py verify <ws>` → `ok: true, byte_identical: true`,
  `commentary/verification.json` (fresh at handoff).
- Reading copy refreshed: `commentary/reading-after.asm` (`commentary.py read`).

## Examples (commentary/source/game.asm)
- `PpuQueue_Execute` (bank 14 $C28E, line ~21284): full queue command format; handler table rendered as
  `.lobytes/.hibytes`.
- `Script_Interpreter` / `ScriptCommandTable` (bank 15 $EFE9/$F013, line ~27939): all 75 commands named
  `ScrCmd_*` with operand layouts; `PrintText` text/control-code format.
- `Boat_Update` (bank 0 $8FE7, line ~2441), `Fish_CheckBite` (bank 0 $9CCF, line ~4851),
  `Fight_Outcome` (bank 15 $E542, line ~26316), `Sound_HandleCommand` (bank 2 $8006, line ~9505).

## Findings useful for redirect (structural questions; see evidence-policy.md)
- **Q1 / L-009** Bank 1 $BA2A-$BA4C is one routine (`ldx #0; stx $05D8; lda $05B0 ...`, thresholds for
  `FightFish_IconState`); phase-1 has file 0x03A3A-0x03A3D as data and a static code region starting at
  file 0x03A3E (operand byte of `stx $05D8`), i.e. a misaligned 6-byte decode. Source label
  `FightFish_SetThresholds` explains it.
- **Q2 / L-010** Fixed-bank static wrappers `Nav_RunEvent0/1/2Script`, `Nav_RunCommandMenu`,
  `Nav_RunArrivalScript`, `Nav_RunPauseScript`, `Bank0_8BC4` (bank 15 $E92B-$E95E) JSR into bank-0
  unknown ranges file 0x00B91-0x00BAC, 0x00BD4-0x00C02, 0x00C05-0x00CE0, 0x00CED-0x00D25 (code-plausible;
  `CommandMenu` $8B9D follows $8B86). Needs bank-context confirmation by a short trace.
- **Q3 / L-011** `Fight_BuildScreen` ($E6C1, static) calls $ADC0 and $92A7, which are data under banks
  0/1 and text under 12/13 — fight-entry bank context unresolved.
- **L-006 resolved statically**: $06E1-$06E4 is a self-modifying "STA $40xx / RTS" APU helper written by
  `Sound_ResetAll` (bank 2 $8173-$8181) and `Snd_ShadowWrite`; entered by `JMP $06E1` ($88ED).
- **L-008**: natural bite paths are `Fish_CheckBite` (bank 0 $9CCF) and `FishShadow_TryBite`
  (bank 1 $A940); conditions documented in source.
- **L-001/L-002**: outcome codes ($80 LineOut 0, $81 LineOut >= $200, $82 LineStrength 0, $83 writer
  unknown) and the day-end condition (ClockHours >= 16) give bounded RAM-patch experiments
  (notes added to `analysis/leads.json`).

## Unresolved meanings (tentative wording used in source)
TimeOfDayScene values; LineType ($0762), PlayerStat0760/0761; FightCounterC5/C7; FightGauge; fight events
1-9; outcome $83 writer; sea events from BoatCrashCount; Sun_Update object identity; title/name-entry
script natives in banks 12/13; sound commands $FB/$FC; DayConditionValue ($0498).

## Deferred work
- Final editorial audit (`commentary.py audit`) at archive acceptance; replace remaining `@skip` labels.
- Re-run commentary after redirect/disassembly changes: carry annotations by physical range (the
  builder keys everything by bank:CPU address) and re-verify.

## Leads
Consumed: none. Produced: L-009, L-010, L-011. Annotated: L-001, L-002, L-005, L-006, L-008.

## Toolkit findings
No toolkit issues found in this phase (prepare/verify/read worked as documented);
report: analysis/toolkit-issues.md (unchanged, TK-001..TK-004 from disassembly).

## Continuation (2026-11-20) — readability gate follow-up

Status: **DONE** (classification unchanged at 93.23%; editorial_audit still pending by design).

### What changed
- **Branch labels.** `analysis/tools/commentary_build.py` local-label naming rewritten: names come
  from the branch condition (button tests, symbol zero/sign/compare tests including compare
  chains, bit tests via `and #m`, `lda #m / bit`, `lsr`/`asl` chains, carry after 16-bit adds,
  carry/zero results of called routines) or from what the target does (`@call_X`, `@goto_X`,
  `@store_X`, `@clear_X`, `@return_XX`, ...). Multi-source targets prefer a condition common to all
  sources; scopes with several loops get body-derived loop names; a lone `@loop`/`@return` stays.
  148 hand `lbl` names added in `commentary/notes/annotations.txt` (section "commentary round 1
  continuation") for the remaining generic cases (e.g. `@music_channel`/`@sfx_channel`/
  `@noise_drum_note` in the sound driver, `@multiplier_bit0_clear`..`bit7` in Mul8,
  `@world_piece_loop`/`@screen_hflip_done` in DrawMetasprite, `@right_pressed`/`@confirm_pressed`
  in AreaSelect, `@bite_roll_failed`, `@remainder_below_divisor`).
  Promoted locals now use the nearest preceding global as prefix (e.g. `FishAI_Type1_turn`
  instead of `Fish_RunBehaviour_turn_2`).
- **Fight events.** The fight event script at bank 15 $E5DC (still unknown bytes) decodes as
  `ScrCmd_Switch SeaEventFlag` + 15-word table; its bank-12 targets print texts that match the
  trigger code. FightEvent_1..9 renamed (PropellerCut, Resting, Jump, RamBoat, TailWalk, Shark,
  LineBitten, BiggerMarlin, RunAfterRest) with exact trigger conditions; `FEV_*` constants for
  all 14 values used at every writer; FightCounterC5/C7 -> GloveHeat/ReelHeat; FightGauge,
  Fight_RodBend, Fight_ReelApply and the RollEvent carry quirk documented.
- **Fight fish model.** Helpers/tables renamed by the value they update (FightFish_DirectionTerms,
  FatigueTermsByRod, FatigueTerms, PassiveDirection/Fatigue/Stamina/Strength,
  StaminaTerms(ByRod), StrengthTerms(ByRod); DirectionTermTables, FatigueTermTables, ...) with
  mechanical headers and a model overview at FightFish_Passive; two original-code quirks noted.
  Game meaning of Strength/Stamina/Fatigue/Direction stays inferred (stated in source).
- **Sound driver.** Every stream command $C0-$E0 has a one-line operand/effect comment;
  misnomers fixed: Snd_SweepUpdate -> Snd_EnvelopeUpdate (music pulse volume envelope),
  SndSweepState/SndEnvState -> SndEnvPhase/SndEnvLevel, SndTempoScale -> SndVolumeScale,
  SndScale -> SndGateScale, Snd_WriteVolumeReg -> Snd_WriteSweepReg, SndCmdD4 -> SetSweep,
  Snd_KeyOff -> SndCmdCB_KeyOff; SndRegFlags bit meanings documented.
- Unknown-range notes: a hand `rgn` note now replaces (not stacks on) the automatic note.

### Label counts (commentary/source/game.asm)
- 557 local labels; `@skip*` 0; numbered `@skipN/@loopN/@returnN` 0; lone `@loop` 45 and lone
  `@return` 81 (each the only one in its scope). Remaining digits only in value-bearing names
  (e.g. `@LineTension_below_28`, `@multiplier_bit3_clear`). Numbered global `_N` suffixes: 0 (the bank-15 `Fight_Update_2`
  clash was renamed `Fight_StepAndCheck`).

### Verification
- `commentary.py verify` -> ok true, byte_identical true (commentary/verification.json, source
  sha256 3662d8b4...). Reading copy refreshed (`commentary.py read` -> commentary/reading-after.asm).
- review.json regenerated by analysis/tools/commentary_review.py: 3,170 scaffold entries
  unchanged and reviewed, 864 units; editorial_audit pending.

### Remaining gaps
- None blocking current-source readability. Mechanical condition names remain where only the
  mechanics are known (e.g. `@Ptr2Hi_result_zero`, `@CalcLo_bit6_set` in Map_StreamColumn
  $D231, `@MapIndexTmp_bit0_set` in Map_StreamRow $D397).
- New redirect lead **L-012**: classify bank 15 $E5DC-$E5FD (file 0x1E5EC-0x1E60D) as the fight
  event script (Switch + table + End); confirm bank 12 at $8000 during the fight (shares the
  trace with L-011). FEV_ROD_BROKEN ($0A) writer not found.

### Toolkit findings
No toolkit issues found in this continuation (verify/read worked as documented);
analysis/toolkit-issues.md unchanged.
