# Commentary uncertainties

- Bank 2 / bank 4 front-end and cutscene routines: names come from short reads and
  reference summaries; screen numbers (FrontEndScreen) are not mapped to on-screen screens.
  Script handlers B04_Script*_NN and B02_Task0605_NN are labelled by table index from
  unclassified tables (L-002) and may include false targets beyond the real table end.
- PlayState_NN labels: index = first table slot reaching the handler; the PlayState table
  extent is unproven (L-008).  NmiTask_NN likewise (L-007).
- Flag-bit meanings of BallFlags ($3B), AtBatFlags ($3D), PitchResult ($3E), PlayResult
  ($05E3) are partial; comments state them as tentative.
- Field units/projection (ProjectFieldPosition, table $9767) and physics constants are
  described mechanically; real-world units unknown.
- CPU AI (bank 6, bank 11) summarized from data flow only; decision rules not verified.
- Runtime confirmation of StrikeCount/BallCount/OutCount, VoiceSample, password fields
  and SeasonDay/SeasonWins is pending (L-009).
- Unknown ranges: 239 notes are generated from references/decode hints; 35 start with a
  legal instruction run ending in RTS/JMP (possible code, L-001); 12 follow a dispatcher
  JSR (L-002).  None were reclassified in commentary.

## Round 1 commentary resume (depth pass)

- Biorhythm (bank 4 entry 1): the date arithmetic (365-day years, leap years, 23/28/33-day
  remainders) is certain from the code; how the three values change play is not traced
  (L-011).  The year epoch (PlayDateYear + $3F vs birth-year byte) is inferred.
- Fielding control (bank 15): B = jump (no direction) / dive (with direction), A/B =
  throw with the d-pad choosing the base (Up 2nd, Down home, Left 3rd, else 1st) - read
  from bit tests; the field-side axis swap (SysFlags bit0) is mechanical.
- Runner flag bits (RunnerSlots byte 0): bit7 active, bit6 moving, bit4 forward,
  bit5 out, bit3 left early on a fly, bit2 batter; anim $80/$88 = tagged/forced out
  (tentative).  RunnerSetDirection0-3 compute positions; names kept for continuity.
- CPU batter/pitcher (bank 11): the state numbering and the start table U_B11_C5BB are
  partly in unclassified bytes; substitution kinds (0 relief, 1 pinch hitter, 2 pinch
  runner) are interpretations.  $0491-$0493 heights and U_B11_C8B0 remain mechanical.
- Line-score screen: columns read as runs/hits/errors by layout; HitsA/HitsB are
  incremented by CountTeamHits (confirmed), ErrorsA/B are never seen incremented here.
- Base coordinate table at bank 15 $EFF0 begins with a byte classified as code (RTS);
  it is documented as data in source and raised as L-010.
- About 289 raw RAM operands remain (mostly single-use or unknown-purpose bytes such as
  $3C, $0329, $0340-$034E, $06C7); they are explained in the surrounding routine comments
  where their role is known.
- Local branch labels: all address-only labels were replaced.  Labels in routines read in
  this pass have role names (@has_ball, @throw, @next_runner ...); elsewhere they carry
  structural names derived from the target (@loop_X, @done, @next, @test_X, @store_X).
