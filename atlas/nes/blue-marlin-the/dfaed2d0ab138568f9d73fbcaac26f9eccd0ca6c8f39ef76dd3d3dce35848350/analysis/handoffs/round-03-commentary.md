# Round 3 — commentary handoff

Status: **DONE** (current-source readability complete for checkpoint pass-003; final editorial audit not
run, by design). Classification unchanged: 100.00% rounded, 262155/262160, 5 unknown bytes (not complete).

## Scope and carry-over
- Map changed (pass-002 -> pass-003, sha256 703aba69...). Round-2 commentary preserved first:
  `analysis/round-2/commentary/` (diff -r identical, game.asm sha256 3967c6f6...) plus round-2
  commentary_build.py, commentary_review.py, r02c tools and handoff (analysis/round-2/README.md).
  Then `commentary.py prepare` re-created commentary/ from pass-003 (fresh before/, baseline.json,
  phase1-state.json, reading-before.asm); notes/annotations.txt restored from round 2.
- Carry by physical position (bank:CPU annotations, analysis/tools/commentary_build.py). Check:
  analysis/tools/r03c/carry.py -> analysis/evidence/round-03/commentary-carry-check.txt. All round-2
  fn/lbl/cmt/op/xb/dat records still match; the 21 pass-002 unknown-range notes were invalidated
  (now code/data) and replaced by a round-3 block. Pre-existing names kept exactly (0 labels removed
  vs round 2; Fish_SetSprite_set_tile/_return and Snd_ReleaseVoice_silence/_mute pinned explicitly).
- review.json: scaffold 3,272 entries intact, all reviewed (3,242 carried from round 2 with the same
  range/kind, 30 reviewed in round 3); 1,079 units, all reviewed; editorial_audit pending.
- Builder changes (workspace tool only): redirect-added "Jump table:" regions render as symbolic
  `.word` (FishAheadHandlers); labelled data regions no longer carry the contradictory
  "consumer/format not identified" note (46 regions).

## New code/data documented (all Static; "statically inferred, no caller known"; never called unused)
bank 0 MapWin_HidePortrait, Bite_RollChanceAndHook + BiteChanceByLineLength/ByFreshness; bank 1
Fish_FixedToObjPos, Fish_CheckMapBounds, FishShadow_TryBiteAlt, Bite_RollChance, AltBite* tables
(op operands #<AltBiteRangeList ...), OffsetPair16/18_Add0/Sub4, Fish_CheckLandAhead, FishAhead_Dispatch,
FishAheadHandlers (16 .word), FishAhead_* stubs/probes/TestCell, Fish_SetSpriteAlt (+ FishTiles_Species0-3,
FishSpriteTables_Species2), FishShadow_PeekVelocity, FishShadow_MoveWhileAnimating,
FightFish_DrainDirAndFatigue (FatigueDrainFlag $05A5), FishFatigue_AddClamped, FishProbeDY entries 8-15,
FishParamTable entry 63; bank 2 Snd_SilenceVoiceReg0; bank 12 ScriptNative_RandomTest99; bank 13
ScriptSub_ShowAndWaitButton (ScreenOn/FadeIn/WaitButton $90/FadeOut/ScreenOff/ScreenReset/Return);
bank 14 ClearNametable1_Zero, Mul16x8, BitArray_Test/Set/Clear (BitArrayPtr $84), QueueWordDecimal_Pad9/Pad0/
_Body, FormatByte3_Pad9/Pad0/PadSpace/StorePad, FormatWeight5_Pad9/Pad0/_Body (#<NumberText operands);
bank 15 StopLoop, warm-boot signature bytes 15-16.

## Readability counts (commentary/source/game.asm, 34,937 lines; analysis/evidence/round-03/commentary-labstats.txt)
- Address-only names: **0**. Numbered generic labels: **0**. Global `_N` collision suffixes: 0.
  Raw RAM operands: 0. Unreviewed scaffold entries/units: **0**.
- Routine headers: 705 (round 2: 680); 1,255 globals, 686 locals; 343 named globals without own header
  (stubs/continuations covered by the owning header, e.g. FishAhead_* stubs, BitArray_Set/Clear).
- Unknown bytes 5/5 noted (bank 1 RTS $ADB6/$B2EA/$B343; bank 15 NOP $E0F6/$E1B3; lead L-021).

## Verification and reading copy
- `commentary.py verify` -> ok true, byte_identical true (commentary/verification.json, game.asm
  sha256 2a5b8906...), run after the final build and review update.
- `commentary.py read` -> commentary/reading-after.asm refreshed.

## Unresolved meanings (commentary/uncertainties.md)
FishState $A0 (set by Fish_CheckMapBounds, not handled by FishAI_Type0); FishAheadHandlers facing 8
= down+right (asymmetric); FatigueDrainFlag has no writer (code + byte search); BitArrayPtr never
written; OffsetPair16/18 operand roles; carried round-2 items (species per HookedFishType,
FishTypeAdjustSet writer, ObjX/ObjY naming on entry grids, fight-model quantities, sound semantics).

## Structural questions for redirect (evidence-policy.md)
- None new from commentary. L-021 (5 residual bytes) remains as stated by disassembly; commentary only
  documents them. Optional bounded check: the facing table of FishAheadHandlers vs FishMoveX/YHandlers
  (semantic, not structural).

## Deferred
- Final editorial audit (`commentary.py audit`) at archive acceptance.
- Optional: headers for the 343 header-less stubs/continuations; per-field prose in Password_UnpackState.

## Leads / toolkit
Consumed/produced: none (L-021 notes referenced). No toolkit issues found; analysis/toolkit-issues.md unchanged.
