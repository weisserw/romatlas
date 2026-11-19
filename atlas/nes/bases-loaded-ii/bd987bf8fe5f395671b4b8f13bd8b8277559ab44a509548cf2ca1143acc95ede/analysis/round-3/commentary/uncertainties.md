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

(The round-1 notes above are kept for history.  Superseded points: the PlayState, NMI task,
front-end screen, CPU input state and bank-4 script tables are now proven jump tables
(round-2 map, L-012), so their entries and extents are no longer in doubt; B02_Task0605_NN
handlers were renamed FrontEnd_<role>; the $EFF0 table is data (L-010 resolved); 239 unknown
ranges are down to 7 (10 bytes, L-016).)

## Round 2 commentary (map maps/round2-disassembly.json)

- Static-only code (L-015, ~1.4 KB) is explained mechanically with "no caller is known".
  Exceptions found while commenting: the force/tag-up fragments at bank 15 $F95B-$F982 and
  $F9DC-$FA13 are entries of the word tables ForceHandlers/TagUpHandlers (still generic
  data in the map), so they are table-reachable (lead L-017).
- Debug-looking code (L-018): NMI task 8 freeze reached only through the disabled
  DebugPauseCheck (AND #$00); DebugForceGameEnd ($C606); DebugEditStartDay/DebugEditWins
  editors in bank 2.  No natural route is known; descriptions are from the code only.
- Front end: which game events select screens $10 (lineup checks), $18 (batter
  substitution), $28 and $40/$41 (final score) is not traced.  The lineup-check pitcher
  step is permanently skipped (ReturnA2 = 2).  Position 9 in FielderCheck_Test, player
  categories (D_B02_E344) and the PositionRule tables are mechanical only.
- FinalScore_SeasonCheck: the 75-win threshold at day $81 and the best-of-7 reading of
  days $82-$88 are inferred from the comparisons; not confirmed at runtime.
- DrawPlayerStatsLine: record bytes $0A/$0B shown as two stats (probably home runs and
  batting average .150+); tile meanings not verified.
- CPU states $40-$43, $50-$56, $60-$66, $70-$79: mechanics are certain from the code;
  which side/menu each serves is inferred from the screens that set them.  "Right" is the
  pad-2 bit the CPU presses to move menu cursors; whether the menu reads it as a cursor
  step was not checked.  $06C5-$06C7 counters remain unnamed.
- PlayState $42, $46-$48, $4A-$52, $53-$66: handler mechanics are read from code;
  which PlayResult bits select them is partly inferred (e.g. $46 as ground-rule double).
- CpuSideModeJumpTable ($D08E) is rendered as .word in source but stays generic data in
  the map (L-017).
- Generic data: 81 L-014 islands carry a context note only (bank data area, probable part
  of the surrounding block); consumer-anchored L-013 tables carry "Used by" notes and, for
  the clear cases, a short role note.  Bank 1 sample boundaries are not mapped.
- RAM: about 330 raw RAM operands remain (unknown-purpose bytes); the CPU substitution
  bytes, $3C (PitchFlags, tentative) and the nametable-clear pointer were named.

## Round 3 commentary (map maps/round3-disassembly.json)

- Superseded: L-017 tables are now labelled in the map; L-016 bytes are classified code
  (stray returns, unreachable as far as known; confidence 0.5); no unknown ranges remain.
  The "$06C5-$06C7 counters" and most raw RAM operands are now named.
- Biorhythm effect (L-011): static only.  The read sites and the random-bound role are
  certain from code; how strongly each value changes play was not measured at runtime.
- PitcherStintMark ($06C4/$06C5, ex-DemoFlag): only direct $FF stores are known, which makes
  CpuPitcherDecision's stamina relief branch look unreachable (L-020).  Indirect writes were
  not excluded.
- PitcherWildness ($06C7) and PitcherRunsAllowed ($06C6) names describe the arithmetic
  (balls +1, strikes -2, hit batter + remaining balls; runs added per play); the names are
  interpretations.
- ReleaseOriginX/Y ($0399/$0398): mechanical names (ball start offsets and the bank-14
  figure origin); which on-screen figure DrawBatterSprites draws is not confirmed.
- PitchDriftAccum moves the 16-bit PitcherSprY/PitcherSprYFrac pair; whether that pair is
  really the pitcher sprite Y (round-1 name) or the drawn ball offset is unresolved.
- Screen-layout tables: 22 layouts are inferred from the table spacing and plausible values;
  ScreenLoadId's actual range was not traced.
- Remaining raw RAM operands (18 uses): $A9 (dummy read in InitPpuCtrl), $03DF-$03E2
  (PlayState_24 stores), $03FA-$03FE (ResetMarkerState defaults), $06D0, $0767, $0784,
  $0789 (sound/title bookkeeping); roles not established.
