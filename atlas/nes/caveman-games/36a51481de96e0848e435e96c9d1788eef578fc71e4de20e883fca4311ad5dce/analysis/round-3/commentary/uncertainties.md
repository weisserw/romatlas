# Commentary uncertainties

Carry forward relevant phase-1 ambiguities as their regions are reviewed.

## Round 1
- Fixed-bank $8000-$BFFF operands (e.g. EnterNextEvent's JSR $8009/$B95F) resolve to bank 0 only
  because MMC1_WritePRG #0 precedes them; other fixed-bank operands into that window stay numeric.
- IrqHandler's split handlers ($C036-$C0BD) and several small stubs in bank 7 were phase-1 Data that
  decode as code (L-007). Superseded in round 2: they are Code in pass-002 (see Round 2 below).
- $0492 (QuitEventFlag): set when a latched Select resumes from pause; event loops return when it is
  nonzero. "Quit event" is an inference from the control flow.
- $0490/$0491 (RoundCount/RoundIndex): attempt rounds in Mate Toss/Dino Vault, pairing index in
  head-to-head events (Fire Starting, Dino Race, Saber Race), bout index in Clubbing.
- Event names: bank 4 third actor called "Chaser" (presumably the sabretooth); bank 6 second actor
  "Actor2" (presumably the dinosaur). Controls of Dino Race, Saber Race and Dino Vault are described
  only as far as the input code shows (L-006 still open).
- Caves of Fame skips event indices 4 and 6 in its page cycle; reason not established. Superseded in
  round 3: index = EventId - 1, so 4 is Clubbing (no records, FameTitleOffsets entry 4 is a dummy) and 6
  ends the cycle.
- $82/$83, $0111/$0112 and Var04AD have unexplained initial values.
- Unknown residues: L-001 (bank 3 $A60C-$AEF9), L-008 (bank 2 $95E6, refines L-002), L-009 (bank 3
  $9179), L-010 (bank 5 $957C). Superseded in round 2: all classified in pass-002 (no Unknown bytes remain).

## Round 1 continuation (event-bank depth)
- Fire Starting: input codes map left/right to FsState 1 and up/down to state 2 (FS_InputToState); the
  game meaning (rubbing vs. blowing) is inferred from the computer patterns (FS_CpuRubPattern 2/1,
  FS_CpuBlowPattern 4/0). FsSfxFlagA/B overlay SongId/$0253-$0255 in this bank; why the sound
  driver bytes are reused is not established.
- Dino Race: action codes $40 (Right, club), $80 (B, jump) and $02 (Left, back up) come from
  DR_ReadRacerInput; DrState 1-4 = jump / confused / crashed / backing up are interpretations of the
  state handlers. DrBoostTimer replaces the old "DrFinished" reading (it is set by a successful club
  hit and forces speed 7). DR_AdvanceCourse is still statically inferred (L-004).
- Saber Race: "chaser" = presumably the sabretooth; runner states 1 jump, 5/$0B pose variants, 6 push,
  7 trip, 8 goal, 9/$0A celebrate, $0C pushed, 3/4 caught/out are read from the dispatcher. The old
  names SR_DrawRunners/SR_DrawChaser were swapped and are now SR_DrawActors/SR_DrawRunner.
- Clubbing: fighter 2 is drawn largely with background tiles (scroll and CHR bank per frame,
  CL_ScrollForPose) plus a 4-sprite overlay; fighter state numbers ($02-$04 attacks, $0A reeling,
  $0B/$0C clinch, $12/$13 falling, $1C knocked out, $1E frozen) are mechanical readings. The
  intimidation-phase move sequences (CL_IntimSeq1-4) are decoded only as "next code after previous".
- Dino Vault: DV_SkipComputer was renamed DV_PrepareRun (the code resets the scene for every run);
  DvVar043E/DvVar0440/DvBarRelX are only compared, their writer (DV_CheckClearance) scales scroll and
  height; the exact geometry is tentative. The second actor is presumably the dinosaur.
- Unreferenced Data bytes that decode as dead code: bank 3 $8984, bank 5 $8B53/$8CBD, bank 6
  $8DA8/$AE0C (lead L-011); classification unchanged.

## Round 2 (pass-002 carry-forward)
- No Unknown bytes remain. The former residues now read as: FS_MarkerXBase + FS_QueueMeterRow (bank 2),
  DR_SetVelocityBoth and DR_StaleForeignImage (bank 3), CL_Bracket6Pairs/CL_Bracket6Places (bank 5).
- Newly classified static routines with no known caller (reachability unknown, not claimed dead):
  FS_QueueMeterRow (which meter it drew is unknown; stack-argument entry inferred from the unbalanced
  PLA), DR_SetVelocityBoth, CL_IsState2to4 (meaning of states 2-4 as a group unknown), DV_FailBelow30
  (the variable compared against $30 is unknown), SetFrameTimerWhenIdle, WaitFrames3, RenderingOn,
  B7_QueueSfx.
- Raster IRQ split path (IrqHandler, IrqSplit* handlers) remains static-only and was never observed.
  (Corrected in round 3, L-015: banks 1, 4 and 5 execute CLI but have bare-RTI IRQ vectors, and IRQs
  were observed in banks 4 and 5; banks 0, 2, 3 and 6 vector IRQ to $C000 but never enable interrupts.) IrqSplitParam/IrqSplitDelay are 18
  bytes each but IrqHandler reads only the 7 even offsets 0-$0C; the other bytes' purpose is unknown.
- DR_StaleForeignImage: "left over from a build tool or earlier program" is an inference; the
  classification rests on the relocation/corruption evidence of L-001, not on that origin.
- $AEF9-$B01D in bank 3 (293 B) and the $CEDB-$CEE7 development-port stubs in bank 7 stay generic Data
  with boundary notes; no consumer identified.

## Round 3 (pass-003 carry-forward)
- Newly classified unreferenced code, explained as probable leftovers (dead status is static: no
  reference and skipped by a branch or not reached by fall-through): AttractTitle_UnusedExpire /
  AttractTitle_UnusedExpire2 / AttractAltScreen_UnusedExpire (bank 0, would end a screen's timer one
  frame later), FS_UnusedEmphasisCycle (bank 2, self-modifying, only meaningful when run from RAM),
  DR_DrawBothRacers (bank 3). Their original purpose is inferred.
- FameUnusedSelfJump (bank 0 $85B1, JMP to itself) stays data; whether it was filler or a stub is unknown.
- CL_Bracket3/CL_Bracket6/CL_PairFromWinners and several bank 0/4 ranges are now observed (L-016); no
  commentary claimed otherwise. AttractAltScreen and DrawMenuLowerScreen remain unexecuted (L-004).

