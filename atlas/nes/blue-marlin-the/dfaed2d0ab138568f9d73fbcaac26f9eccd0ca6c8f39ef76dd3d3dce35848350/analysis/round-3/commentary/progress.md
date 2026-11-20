# Commentary progress

Round-1 history is in analysis/round-1/commentary/progress.md; round-2 history (pass-002) is preserved
verbatim in analysis/round-2/commentary/progress.md.

## Round 3 (2026-11-20) — complete current-source pass on checkpoint pass-003

Baseline: the map changed (pass-002 -> pass-003, map sha256 703aba69...). The whole round-2
commentary was copied to analysis/round-2/commentary/ (diff -r identical; with the round-2 builder,
review tool, r02c helpers and handoff) and a fresh baseline was prepared with `commentary.py prepare`
(before/, baseline.json, phase1-state.json, reading-before.asm now frozen at pass-003).

Carry-over: source/game.asm is generated from before/game.asm + notes/annotations.txt by
analysis/tools/commentary_build.py; annotations are keyed by bank:CPU (physical position).
analysis/tools/r03c/carry.py compared every annotation with the pass-002 and pass-003 listings
(analysis/evidence/round-03/commentary-carry-check.txt): all fn/lbl/cmt/op/xb/dat records still hit
the same instruction/data; the 21 unknown-range notes of pass-002 became invalid (code/data now) and
were replaced by the round-3 block in annotations.txt. Unchanged: 3,242 review entries carried from
round 2 (same physical range and kind, re-verified); 30 new/changed entries reviewed in round 3.

New in round 3 (all Static code, described as statically inferred with no known caller):
- bank 0: MapWin_HidePortrait, Bite_RollChanceAndHook (+ BiteChanceByLineLength/ByFreshness).
- bank 1: Fish_FixedToObjPos, Fish_CheckMapBounds, FishShadow_TryBiteAlt, Bite_RollChance,
  AltBite* tables, OffsetPair16/18 helpers, Fish_CheckLandAhead + FishAhead_Dispatch/FishAheadHandlers
  (16-word jump table rendered as .word)/stubs/probes/TestCell, Fish_SetSpriteAlt (+ FishTiles_Species0-3),
  FishShadow_PeekVelocity, FishShadow_MoveWhileAnimating, FightFish_DrainDirAndFatigue, FishFatigue_AddClamped.
- bank 2 Snd_SilenceVoiceReg0; bank 12 ScriptNative_RandomTest99; bank 13 ScriptSub_ShowAndWaitButton
  (script bytecode); bank 14 ClearNametable1_Zero, Mul16x8, BitArray_Test/Set/Clear,
  QueueWordDecimal_Pad9/Pad0/_Body, FormatByte3_Pad9/Pad0/PadSpace, FormatWeight5_Pad9/Pad0/_Body;
  bank 15 StopLoop; WarmBootSignatureRom bytes 15-16.
- New RAM symbols: FatigueDrainFlag ($05A5, no writer), BitArrayPtr ($84).
- Fixes: preserved observed names that the new headers would have re-prefixed (Fish_SetSprite_set_tile/
  _return, Snd_ReleaseVoice_silence/_mute now explicit); FormatWeight5 immediates now #<NumberText/#>NumberText;
  46 labelled data regions no longer say "consumer/format not identified" (builder change).
- Unknown bytes (5): each has a specific note (bank 1 RTS $ADB6/$B2EA/$B343, bank 15 NOP $E0F6/$E1B3; L-021).

Verification at handoff: `commentary.py verify` ok, byte_identical true (verification.json);
`commentary.py read` refreshed reading-after.asm. editorial_audit stays pending (final acceptance).
Deferred: final audit; optional headers for ~343 self-explanatory/continuation globals.
