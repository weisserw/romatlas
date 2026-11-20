# Round 2 — commentary handoff

Status: **DONE** (current-source readability complete for checkpoint pass-002; final editorial audit
not run, by design). Classification unchanged: 99.60% (1,044 unknown bytes in 25 ranges).

## Scope and carry-over method
- Map changed (pass-001 -> pass-002). The entire round-1 commentary was copied first to
  `analysis/round-1/commentary/` (diff -r identical; plus round-1 `commentary_build.py`,
  `commentary_review.py`, handoff and a README). Then `commentary/` was re-prepared with
  `commentary.py prepare` from maps/current.json = maps/pass-002.json (sha256 c6d84b24...) and
  analysis/state.json (frozen before/, baseline.json, phase1-state.json, reading-before.asm).
- Names/comments carried by physical position: `commentary/notes/annotations.txt` (all directives
  keyed by bank:CPU) re-applied to the new listing by `analysis/tools/commentary_build.py`.
  Every annotation was re-checked against both listings (`analysis/tools/r02c/carry.py` ->
  `analysis/evidence/round-02/commentary-carry-check.txt`). Invalidated and rewritten: the misaligned
  L-009 label ($BA2E -> FightFish_SetThresholds at $BA2A), the round-1 unknown-range notes for
  bank 0 $8B81-$8D15 and bank 15 $E5DC (now code/data), and the Q2/Q3 comments on the fixed-bank
  wrappers. 33 data labels whose bytes moved Unknown -> Data were reviewed and kept.
- review.json: scaffold (3,268 entries) intact; entries with unchanged physical range and kind are
  marked "carried from commentary round 1 (re-verified)" (2,299); new/changed entries reviewed in
  round 2 (969); 1,054 units; editorial_audit pending.
- New code (~5,200 bytes) and every other new region were read and annotated (see examples).

## Readability assessment (commentary/source/game.asm, 34,083 lines)
- Address-only names (sub_/loc_/dat_ + address): **0**. Numbered generic labels (@skipN/@loopN/
  @returnN or collision suffixes): **0**. Global `_N` suffixes: 0. Raw RAM operands: 0; raw ROM
  operands: 0. 1,189 global labels (680 with header comments), 660 local labels (64 lone @loop,
  94 lone @return). Remaining digits only in value-bearing names (QueueAttrAddress_23C0,
  ScriptNative_ScrollRight4, Mul8, FormatWeight5 ...).
- Unreviewed units: **0** (all scaffold entries and units status=reviewed; editorial audit pending).
- Unknown ranges: all 25 (1,044 bytes) carry specific notes (decoded shape, why unresolved, L-017).
- Data/text regions: every region has a label or a one-line role note derived from its evidence
  (bank-select audit, script parser, L-015, batch-B inspection, L-018 sound streams).

## Examples (game.asm)
- Bank 0 navigation windows: BiteAlert_Show, SeaEvent_BoatWarning / EngineTrouble / EngineBroken,
  Nav_TiredMenu, Nav_PauseStatusWindow, Nav_CommandWindow (window texts quoted in headers).
- Bank 1 FightFish_Init family (PickType, RollSize, RollBiggerSize, ComputeWeight, InitModel).
- Bank 12/13: Keep_* (keep/storeroom list), Weigh_* (scale count-up), Records_AddCatch (running
  average via Mul16x16/Div32by16), LevelUp_Skill/BodyStrength/MusclePower, Password_Generate/
  Verify/PackState (31 five-bit symbols, field layout in header), PasswordEntry_/NameEntry_ input.
- Fixed banks: Script_ResumeNative ($F363), ScriptTest_Random, FormatWeightRounded, title IRQ chains.
- Round-1 defects corrected from new evidence: TimeOfDayScene -> HookedFishType (fish kind;
  0-4 marlin), FishPullScale -> FishSize (weight = size*8+0..7), LureWearTable -> KeepFishType,
  TripStats -> weight records, PlayerStat0760/0761/LineType -> Skill/MusclePower/BodyStrength
  levels, SpritePaletteRam/InfoSprite_* -> TextSlots/InfoText_* (string pointers, not palettes),
  name/password cursor routines swapped names, ObjPriority+7 -> FishSpecies (symbol precedence).

## Verification and reading copy
- `python3 /root/romatlas/scripts/commentary.py verify <ws>` -> ok true, byte_identical true
  (commentary/verification.json, fresh at handoff).
- Reading copy refreshed: commentary/reading-after.asm (`commentary.py read`).

## Unresolved meanings (tentative wording in source; commentary/uncertainties.md)
Exact species per HookedFishType; FishTypeAdjustSet ($05AC) has no writer found (likely always 0);
FightFish_AddStaminaFar leaves Ptr2Hi unset; ObjX/ObjY naming vs the password/name grid (row drives
ObjXLo); GloveHeat/ReelHeat/FightGauge, fight-model quantities, sound command semantics (round 1);
FEV_ROD_BROKEN writer; Password_UnpackState only documented as the inverse of PackState.

## Structural questions for redirect (evidence-policy.md; lead L-017 notes updated)
1. Complete unreferenced routines adjacent to sibling code: bank 0 $811A (hide-object-1 sibling of
   MapWin_ShowPortrait), bank 14 $C9F8 (shift-add multiply, Mul8/Mul16x16 sibling), $CAB5 (bit-array
   helpers on ($84)), $DDA3/$DF76 (entry stubs of the decimal formatters next to
   QueueByteDecimal_Pad9/Pad0 and FormatWeight5), bank 1 $B424 (FishFatigue clamped add, variant
   of FightFish_AddFatigue). Question: do instruction-level contracts (prologue/epilogue, shared
   tail targets, sibling layout) justify Static code without a caller? Bounded check: confirm each
   decode ends in RTS/JMP to known code and its fall-in boundary is a known RTS/JMP.
2. Bank 12 $8A72 (`jsr Random / cmp #$99 / rts`) matches the ScriptTest_* shape; search unparsed
   bank-12 script blocks for a `$38 $72 $8A` operand (none found in the parsed set).
3. Bank 1 $A3ED-$A5DD cluster: table bytes 05 07 09 0B 0D 0B 09 07 then fish code, with observed
   data reads at $A3DC/$A3EC/$A51C: split the table prefix from code by consumer bounds.
4. Single bytes: RTS at $ADB6/$B2EA/$B343 and NOP at $E1B3 between routines (filler vs unreachable
   epilogues) — a sizing/role decision, no runtime evidence expected.
5. Bank 13 $A69B (02 0A 0C 90 0B 01 03 41): possibly bytecode of an unparsed script block.

## Deferred work
- Final editorial audit (`commentary.py audit`) at archive acceptance.
- Optional: per-field prose in Password_UnpackState; headers for ~316 self-explanatory or
  continuation globals; labelling hard-coded script pointers (lda #lo/#hi -> ScriptPtr) in the
  fixed bank with symbolic script labels.

## Leads
Consumed: none. Updated: L-017 (commentary notes). Produced: none (questions above extend L-017).

## Toolkit findings
No toolkit issues found in this phase (prepare/verify/read worked as documented);
analysis/toolkit-issues.md unchanged.
