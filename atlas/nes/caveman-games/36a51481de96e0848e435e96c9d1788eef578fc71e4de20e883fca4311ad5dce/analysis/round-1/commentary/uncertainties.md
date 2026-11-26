# Commentary uncertainties

Carry forward relevant phase-1 ambiguities as their regions are reviewed.

## Round 1
- Fixed-bank $8000-$BFFF operands (e.g. EnterNextEvent's JSR $8009/$B95F) resolve to bank 0 only
  because MMC1_WritePRG #0 precedes them; other fixed-bank operands into that window stay numeric.
- IrqHandler's split handlers ($C036-$C0BD) and several small stubs in bank 7 are phase-1 Data that
  decode as code (L-007). Comments describe them without changing classification.
- $0492 (QuitEventFlag): set when a latched Select resumes from pause; event loops return when it is
  nonzero. "Quit event" is an inference from the control flow.
- $0490/$0491 (RoundCount/RoundIndex): attempt rounds in Mate Toss/Dino Vault, pairing index in
  head-to-head events (Fire Starting, Dino Race, Saber Race), bout index in Clubbing.
- Event names: bank 4 third actor called "Chaser" (presumably the sabretooth); bank 6 second actor
  "Actor2" (presumably the dinosaur). Controls of Dino Race, Saber Race and Dino Vault are described
  only as far as the input code shows (L-006 still open).
- Caves of Fame skips event indices 4 and 6 in its page cycle; reason not established.
- $82/$83, $0111/$0112 and Var04AD have unexplained initial values.
- Unknown residues: L-001 (bank 3 $A60C-$AEF9), L-008 (bank 2 $95E6, refines L-002), L-009 (bank 3
  $9179), L-010 (bank 5 $957C).

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
