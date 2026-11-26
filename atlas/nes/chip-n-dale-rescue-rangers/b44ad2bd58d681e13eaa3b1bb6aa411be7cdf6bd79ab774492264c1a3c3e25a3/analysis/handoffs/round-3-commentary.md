# Round 3 commentary - handoff (COMPLETE, final editorial acceptance)

## Scope and checkpoint
- Input: checkpoint **pass-003** (maps/current.json == analysis/state.json, sha256 a47d91b2...).
  Classification: 262160/262160 bytes (0 unknown).
- It differs from the round-2 frozen baseline (8bc887dd...). So the whole round-2 commentary was
  first copied to **analysis/round-2/commentary/** (checked identical with diff -r). Round 1 is
  untouched at analysis/round-1/commentary/. Then `commentary.py prepare` built a fresh baseline:
  1031 scaffold entries, byte-identical.
- Carry-over: notes/, tools/, progress.md and uncertainties.md were carried over. Annotations are
  keyed by physical offset. The new listing differs from the old one in only two ways: 14 region
  headers changed from Static to Observed (L-016), and $F62C is now a 1-byte code region (L-015).
  All carried annotations were placed again. annotate.py now skips phase-1 region comment lines.
  This changes only the workspace-local tool; the toolkit was not modified.
- Covered: ALL classified source, plus final acceptance. I worked on the changed evidence and the
  known editorial issues. I did not reread the whole source.

## Audit / verification
- `python3 /root/romatlas/scripts/commentary.py audit <ws>`: **ok**. byte_identical true,
  reviewed_regions 1031, reviewed_units 681, editorial_audit true (commentary/audit.json).
  The audit also refreshed commentary/reading-after.asm.
- game.asm sha256 541eb90e..., symbols.inc sha256 b5bfcec9.... input/game.nes is unchanged
  (sha256 b44ad2bd...).
- review.json: every entry and unit is reviewed. editorial_audit is reviewed, with its findings
  recorded.

## Editorial fixes (examples)
- $F62C: the stale "unknown" note was replaced. PlayerBump_DeadRts is the vestigial CLC/RTS
  epilogue left after a JMP was spliced in. It is static code, and reachability is not claimed.
- All 28 raw base-offset operands are resolved:
  - MetaspritePtrLo/Hi (bank 5, 300 entries; ids 256+ use +256).
  - MultiplesOf7 and NotePeriods+1.
  - DirectionVelocityTable (4-byte Y/X velocity records).
  - Boss51WaveDropX (the waves table is read as drop-X/time pairs).
  - MetatileTL/TR/BL/BR/MetatileAttr. These are constants, because the tables exist once in each
    level data bank.
  - ResetStub_BankN+1 (the MMC1 reset INC).
  - The bank-4 SpawnEnteringObjects call.
- About 480 raw RAM operands are now named. Array sizes were added (Obj*, OAM, palette, VRAM
  lists, HUD counters, RandomSeed, ...). The sound engine RAM was decoded: Ch* channel-record
  fields, Trk* music fields, and Tempo*/Fade*/Sfx* zero-page variables. No raw RAM or ROM
  operands remain in instructions.
- Misleading names fixed: PlayerThrow_Launch/CheckEnd and ObjType64 index object fields with
  Y = PlayerCarry (slot|$80). They are now written Obj*-$80. Before, ObjScreenX,y was shown where
  the access really hit ObjActive.
- Sound engine:
  - EnvelopeSustain was renamed ChannelOutput and documented in four steps: volume/fade/tremolo,
    period/vibrato/packed-octave decode, slide, LFO.
  - Other renames: WriteChannelReg, ChannelSetDetune, ChannelSetSlideRate.
  - The effect-script field bits were corrected (the old "volume" field is the gate ratio).
  - The EnvelopePhaseTable order was corrected.
- Branch naming: the two-player camera decision tree in UpdateCameraX now has descriptive labels.
  The rule is documented: the player closer to the centre decides the scroll. ObjType00,
  Platform_TwoPlayers and NmiPpuUpdate were named too. ObjType00 now has a note that it clears
  player 2's carry when nobody holds the timed item.

## Remaining uncertainties (commentary/uncertainties.md)
- Game identity of most object types and of moving-item metasprites $23/$24/$25. The second HUD
  counter's item. The extra heart sprite.
- Audible meaning of sound commands 5-7 (FadeRate/FadeLevel are only tentatively a music fade).
  SfxResumeId's purpose. The packed period format was not checked against real pitches.
- The PlayerConfig byte tested by the pause screen. The 8-byte trailer before each bank's
  vectors. StreamColumnsArg ($1F) is written and never read.
- About 260 parent-anchored automatic branch labels (_SkipN/_LoopN) remain. They are acceptable
  as local names.
- Structure-only routines with no known caller keep "reachability not claimed" notes.

## Remaining gaps
- None in current-source readability or final editorial acceptance. Whole-ROM classification is
  complete (pass-003).
- No publishing was done. All source, assets and the ROM copy stay local. No new toolkit issues
  were found (analysis/toolkit-issues.md is unchanged).
