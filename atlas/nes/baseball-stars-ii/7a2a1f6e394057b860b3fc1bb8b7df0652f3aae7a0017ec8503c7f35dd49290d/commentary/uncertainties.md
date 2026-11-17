# Commentary uncertainties

Carry forward relevant phase-1 ambiguities as their regions are reviewed.

## Round 1

- FlushPpuBuffer selects the counted-run record format with `BIT $6040`; $6040 lies in the PRG-RAM save
  signature (bytes 0..255), so it holds $40 and the test is "address-high bit 6 set". Similarly
  `BIT $6080` in banks 24/25 tests the A button ($80). Reading these as constants depends on the signature
  being intact (InitGame rewrites it at power-on).
- $049C-$049F (BaseOrSubTable) has two uses: per-base runner table during plays (layout inferred from
  ForceRunnersOnWalk) and [action, outgoing, incoming] for the computer manager. Field meanings tentative.
- Object arrays: 14 slots per array ($39/$47/$55/$63/$71/$7F...). Slots 0-8 are the nine fielders
  (InitFieldObjects); the use of slots 9-13 (runners, batter, ball) is not fully established.
- At-bat interpretation (StrikeCount/BallCount/OutCount, sound ids for calls, JudgePitch) is from code
  reading and matches the observed game flow, but the meaning of SwingFlags bits is tentative.
- Stat record layout (8 bytes per batter: at-bats, hits, RBI, HR, average digits) is inferred from the
  update routines in bank 30 only; display code in bank 8/9 has not been read.
- Names in bank 31 $E371-$F2DD and bank 1 come from mechanical summaries, callers and nearby strings, not
  a full read; they are marked tentative in their headers where the evidence is thin.
- The sound driver command bytes (negative stream bytes, Loc00_8204) are not decoded.
- Game modes 16-19, 30, 31, 36, 37, 40, 41 have handler addresses in unclassified/data bytes (lead L008);
  FarCallTable entries 7 and 14 point at unclassified bytes that decode as code (lead L007).
- L001 premise correction: the word $BD1D at $FF28 belongs to ScreenDataTable (read by
  FarDrawScreenIndexed with an explicit data pair), not to FarCallTable, so bank 17 $BCAD-$BFFF is more
  likely screen data than code.

## Continuation #3 (banks 2/3 tail, 5, 8-15, 24, 25)

Corrections to the notes above (kept for history):
- Object slots are now established from bank 12/13: 0-8 fielders, 9 batter/batter-runner, 10-12 runners on
  first-third; ObjState ($0589) is the runner view of ObjTask (ObjTask+9).  Several per-object arrays mean
  different things for runners and fielders; render.py gained BV() bank-specific aliases so bank 12 shows the
  runner meaning (RunnerToBase, RunnerPathPos, ...) and bank 13 the fielder meaning (FielderDistLo, ...).
  Where a bank-12 line indexes a fielder through a runner alias the alias doc says so.
- SwingFlags acts as the ball-flight flag byte during plays (alias BallFlags in banks 12/13: bit 1 caught,
  bit 2 touched ground, bit 6 wall, bit 7 dead ball; bits 0/3-5 tentative).
- $0526/$0527 are the batter's and pitcher's form/handedness bytes (player record byte 8 / pitcher record
  byte 0), not stadium selectors: renamed BatterForm/PitcherForm; bit 7 = left-handed is tentative.
- Stat display code (banks 8/9/11) is now read; LeagueResults is 16 bytes per league team (wins against each
  opponent, games, wins, average digits).
- CallBank14A29B actually calls bank 15 (renamed CallLoadPaletteSet).
- The bank 30 auto comment "target in bank 13" at the jsr $BA53/$BA02 sites was wrong: the stadium pair from
  SelectStadiumPair is mapped (banks 5/23/27/29 field scroller).

Still tentative after reading:
- Computer base-running and throw heuristics (bank 12 CpuRunner*, CpuChooseThrow): the control flow is
  described, the exact meaning of the thresholds and plan tables ($85E5, $A0D2) is not.
- Fielder task numbers $08-$21 in bank 13 FielderTask: names follow the handlers' behaviour; a few
  (back-up/cutoff, $1D pitcher cover) are inferred from movement targets only.
- Team points ($683D digit counter, AwardTeamPoints/PointsForLevel) - purpose inferred from the edit screens.
- Mode 21 (Mode21ViewTeam) generates an edited team; the round-1 name is kept, the doc says it is probably
  MAKE TEAM.
- Leader-page categories in bank 11 are numbered, not named (column header texts not decoded).
- Bank 11 $A995/$AA1E/$AA7F/$AB11 and $AFCE (leader printing / champion highlight continuations) are
  unclassified bytes following classified code - the routines that end there are described up to the gap.

## Continuation #4 (label naming)
- Tentative data names (meaning inferred from one consumer only): ReliefBiasByOrderPos, SlotTeamWords, PitcherStanceAlt,
  Obj13WorldPos/Obj13AnimByRegion (scoreboard/billboard sprite), TeamBlockSrcPtrs/TeamBlockDstPtrs, BoostThreshold,
  WatchCursorPos/MatchCursorPos* (X or Y not established), TeamColourTablePtr (which table it points at).
- Tentative branch names: RunnerCheckLeadOff (bank 12 $89AE), CpuDefenseChooseEntry (bank 2 $9967), the
  ScoreHomeRunners message classes (msg_normal / msg_tie_or_lead / msg_special), TeamHasEnoughPlayers count_group1/2
  (which roster group is counted), CheckAcquireCost len_first/len_second (which string is points and which is price).

## Round 2 (pass-002 baseline)
- Stadium field data (banks 4/5, 22/23, 26/27, 28/29): layout taken from the scroller and LookupFieldCell
  arithmetic; that the spare map columns $70-$7F hold LookupFieldCell's cell properties is shown for pair 2 only
  (LookupFieldCell always maps pair 2), the other pairs are assumed equivalent.  The at-bat metatile set/screen
  copies at $B380/$B780 of pairs 11/13/14 have no known reader.
- Screen records named after their static drawing routine (notes/97_round2_screens.py); 23 records reached only
  through ScreenDataTable keep index names (ScreenRecordNN) - the screens they show are not identified.  Bank 19
  $A6C0-$B9FF and bank 21 $B000-$BAC0 grid rows have no known ScreenDataTable entry.
- Unknown runs (83, 1,727 bytes): routine-like islands without entries are described mechanically only; their
  purpose (dead code vs. untraced state) is unresolved (L021).  Table pieces with a settled role are listed in L023.
- Pair-12 RAM aliases EditWorkBuf/EditWorkByte cover several short-lived uses; TradeSwapBuf is exact.
- Stale code copies (banks 11/15/17/19/21/25) are taken as build residue; no entry exists in their pairs.

## Round 3 (pass-003 baseline)
- Superseded: the round-1 note on bank 11 $A995/$AA1E/$AA7F/$AB11/$AFCE - these bytes are classified code
  now; $AFCE is LeaderValueDigits, a separate routine with no known caller, not the champion highlight.
- New pass-003 code (43 Code entries, all Static, no caller found by static or observed discovery): each
  routine is named after what it does mechanically. Purpose not established for MainMenuReenterAtCell
  (bank 3), GenRenumberRecords (bank 10: why RunsHome=$84 gates it), CheckRunnerAtThrowBase (bank 12),
  QueueRemoveTeamMarks (bank 24: what tiles $4E/$5E show), LineupCursorAdjust and DrawPlayerIcon (bank 25:
  meaning of player-record byte 5 "1"-"5"), DrawSideRosterSheet (bank 25: the ObjXLo<5 test).
- FormatMoneyLine (bank 25) reads $683D + 16*(SideTeam-8) + 5. That does not match the 12-digit
  team-points layout ($683D + 12*n), so the "dollar amount" reading and the source field are both tentative.
- 25 unknown bytes (6 runs) stay unknown. They are RTS-valued bytes (bank 2 $80DA-$80DB, $9B48; bank 9
  $A165), an effectless dead load (bank 2 $9988) and two effectless or faulty code-like fragments (bank 30
  $C0C0 LDA/STA ChrBankTemp, bank 31 $F6C2 Temp0C += 18 with a broken carry). Each has a source note.
- Inline words at bank 2 $8C52/$8CA0 (inside PitcherListDrawAlt) are still classified as code. They are
  reported as lead L025.
- New RAM aliases are limited to banks where every use was checked: OamWriteOffset ($048A; DigitOutIndex
  in bank 9), RosterSetupSide/Index/FillCandidate (bank 8 $074A-$074C), TeamPointsPtr (bank 25 $055A),
  InfoDigits (bank 15 $0777). Other multi-use scratch (e.g. $0540 in bank 2, $074A/$074B in bank 11,
  zero page $BA/$C0/$C1/$DB) keeps raw operands because its role changes between routines.
- Round-3 branch names (notes/99d_round3_locals.py) describe the local control flow they were read from.
  Names such as @select_7/@select_8 (bank 24 option rows) and @zone_0/@zone_2 (bank 13 BallZone) name
  values whose game meaning is not established.

## Round 4 (pass-004 baseline)

- Superseded: the round-3 items above on bank 30 $C0C0, bank 31 $F6C2 and the bank 2 inline words. Pass-004
  classified $C0C0 and $F6C2 as Static code (LoadChrBankTemp, AddRecordStrideTemp0C), and split the inline
  words $8C52/$8CA0 out of PitcherListDrawAlt as data (L025 consumed).
- LoadChrBankTemp (bank 30 $C0C0) only leaves A = ChrBankTemp. Whether it is a stub of the CHR-set helpers or
  dead code is open. No caller is known and no fetch was observed.
- AddRecordStrideTemp0C (bank 31 $F6C2) is meant to add 18 to Temp0C/0D, but its high-byte ADC is defective,
  which suggests it is dead code. No caller is known.
- PitcherListDrawAlt builds the four-character value "d.dd" from record bytes 4/5. Reading it as the
  earned-run average is a guess based on the format.
- 8 unknown bytes remain in 4 runs (bank 2 $80DA-$80DB, $9988-$998B, $9B48; bank 9 $A165). Their round-3 notes
  still hold and were re-checked: no reference and no fetch in S1-S27 (L021).
- The 21 regions whose evidence changed (11 now Observed, 10 partly observed) agree with their existing
  routine names and comments. None of the prose depended on them being static-only.

## Round 5: residual bytes classified by user-directed manual fallback (2026-11-17)

The 8 bytes formerly left unknown (b2 $80DA-$80DB, $9988-$998B, $9B48; b9 $A165) are now generic Data with
Manual provenance only because the user explicitly authorized this batch exception for <=10 residual bytes.
Their code-versus-data role is still unresolved: no reference, consumer, read or fetch was ever found. Do not
treat them as observed or statically proven data, and do not infer a meaning.

## Round 5 final audit (2026-11-17)

- PendingPitcherChange = 2 means that a forced pitching change is due. The flag is set to 1 by a pinch hitter or
  runner for the pitcher, and Mode9HalfInningEnd advances it to 2. The earlier "computer-vs-computer play" reading
  is withdrawn. Still not traced: which side the forced A press for side 1 corresponds to in every inning state.
- Bank 30 EndOfPlay's jsr $8274 is assumed to run with pair 6, which is set on the CheckPlayOver and SimEndAtBat
  paths. The FinishPlayRedraw and HomeRunTrot callers were not checked.
- The 8 fallback bytes (regions 5155-5158) are unchanged. They are a user-directed manual fallback, not backed by
  evidence, and no meaning is claimed for them.
