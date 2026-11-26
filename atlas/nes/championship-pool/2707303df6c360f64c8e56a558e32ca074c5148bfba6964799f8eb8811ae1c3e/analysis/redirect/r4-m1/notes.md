# Redirect r4-m1 (round 4) - notes

Input checkpoint: pass-004 (byte_identical, map sha256 35ed6454...). ROM classification 100.00% (131088/131088; 0 unknown).
Scope: the three optional questions in analysis/handoffs/round-4-commentary.md, plus the terminal review.
Map, analysis/state.json, commentary source, input/game.nes and the toolkit were not modified.

Server: analysis/tools/rpcd.py daemon with ROMATLAS_SCRATCH=/tmp/ra-2707-r4r. Only its own PIDs (bridge, run.py
serve, romatlas child) were killed. The scratch directory was removed after copying three PNGs here.

## Runtime method (run_r4.py)
- `base`: emulator.start, run 1340, then the r3-p1 S8 route (run_r3.s8_route) -> snapshot aim0. It reproduced r3-p1
  aim0 exactly: frame 2740, GameCmd 1, GameMode 0 (tournament), GameType 0 (8-ball), TableView 1. No RAM edits.
- Each test loads aim0 and runs a ROM Read trace (capacity 0, streamed) limited to one physical range.
- All input is natural. There are no memory or register writes anywhere in r4-m1.

## Q2: SET SPIN panel attribute extent (region 998, bank 3 $9043-$90D2, phys $D053-$D0E2)
- Input: from aim0, hold Select 10 frames, then Select+B for 4 frames, then Select 10 frames (B released with Select
  held -> TableControlInput -> Far_SpinPanel). Then idle 60, Up 30, idle 30, A to close.
- Result (spin-panel-reads.json, r4_spin.png shows the SET SPIN window):
  - 105 reads, 88 distinct offsets, one contiguous run: phys $D053-$D0AA = CPU $9043-$909A.
  - That is SpinPanelRect (82 bytes: header 10x8 + 80 tiles, $9043-$9094) plus SpinPanelAttrs (6 bytes, $9095-$909A).
  - The 32-byte uniform run $909B-$90BA was not read. Neither was the rest of region 998 ($90BB-$90D2,
    MainMenuScripts etc.; not exercised by this input).
- Conclusion: the 6-byte attribute length is confirmed by a natural trace, so the commentary's "probably 6 bytes"
  becomes a fact. The run after it is outside the window reader's extent; its role is still unknown (meaning only).
- Classification impact: none. The whole range is Data in one generic region, and the reads fall inside it.

## Q3: BallMarkerSprite bank context (bank 4 $B209, phys $13219)
- Static contract, checked in the commentary source:
  - DrawBallMarker (fixed bank 7) is entered only by `jmp DrawBallMarker` in DrawBallSprite when Tmp09 != 0.
  - Tmp09 = 1 is set only in BuildSprites, for the CalledBall draw when GameCmd = $10.
  - DrawBallSprite is called only from BuildSprites, which begins with `jsr SelectBank4_Temp`.
  - DrawOverlayObject, DrawBallSprite, DrawPocketMarkers, EmitMetasprite, EmitSingleSprite, DrawBallShadowSprite and
    SetBallSpritePalette contain no bank write.
  - So bank 4 is statically mapped at $8000 whenever DrawBallMarker reads through SrcPtr.
- Runtime check (natural input): from aim0, hold Select, press A. This is a call-shot game, so BallInHandMode runs
  and sets GameCmd = $10 with CalledBall = 10 in the overview view. Four B presses followed. Trace range phys
  $13219-$14000 (the whole 3560-byte labelled block).
  - Evidence: ball-marker-reads.json. r4_mark0.png shows the ring marker drawn around the called ball.
  - 2079 reads, all with cpu_context prg_banks [8, 9, 14, 15], i.e. 16 KiB PRG bank 4 at $8000. No other context
    occurred.
  - $B209-$B20A was read. This word is a set pointer, and its target $B5D8 was read as a 13-byte record
    ($B5D8-$B5E4).
  - So "BallMarkerSprite" is a one-entry metasprite set-pointer word followed by other bank-4 data, not a single
    3560-byte sprite.
  - The other read runs in the block ($B23F-$B242, $B24A-$B24D, $B401-$B41A, $B7F1-$B7F2, $B825-$B826) are
    metasprite records reached through the other bank-4 set pointers (BallMarkerSet, overlay and phantom sets). They
    show that the block holds several metasprite records.
- Classification impact: none. The bank context agrees with the region's physical bank 4. PRG bank 4 is entirely Data
  and Padding (16369 / 15), so no code/data or boundary question arises.

## Q1: is the Speed Pool total a time? (static answer)
- GameFrame adds pending ClockTicks seconds to the current player's PlayerScore, stopping at 9999.
  - ShotClockTick counts 60 frames per tick.
  - For GT_SPEED_POOL, the setup sets ClockFrameCount = 60 and ClockTicks = 0.
  - The score heading for Speed Pool is MSG_HEAD_TIME ("TIME"), and UpdateScoreLine returns early for Speed Pool.
- The Challenge unlock rule increments WorldChampUnlock only when PlayerScore < 141, so lower is better there.
- So the Speed Pool total (the sum of rounds, via AddScoreToTotal) is elapsed seconds.
- ChallengeResult inverts only THREE BALL. Speed Pool group +$32 has five ascending thresholds (between about 290
  and 500 seconds; the exact values are in the ROM table) and counts how many the total exceeds. A slower total
  therefore ranks higher, and a fast one ranks NOVICE (r3-p1: a total of 7 gave NOVICE).
- Whether the developers meant this cannot be proved from the ROM. Mechanically it is a missing inversion: the same
  class of fix THREE BALL received. Record it as a likely original-game quirk.
- A natural three-round clear is not needed, because the mechanics are fully determined statically.

## Other bookkeeping
- L-008 (bank-6 screen-script labels) was applied by round-2 commentary (ScrLogo ... ScrIntermission), but
  leads.json still said "ready". It is now marked verified with a history entry. No content change was needed.

## Terminal review inputs
- Unknown complement: empty (0 bytes); no ledger groups remain.
- Code/data decisions that are still low-confidence: the r2-q1 kept-code set ($D9B0, $DE1C, $97B1, $FB32, gap
  fills).
  - The round-4 split-immediate scan found no reference into any of them.
  - The new r4 reads are all in bank-3 Data region 998 and bank-4 Data; none touch a Code region.
  - The premises are unchanged, so the review is reused.
