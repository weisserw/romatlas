# Final editorial audit (pass-003 commentary)

Input: checkpoint pass-003 (maps/current.json and analysis/state.json unchanged since the round-3
commentary prepare). The commentary was resumed in place; nothing was re-prepared. Whole-ROM
classification stays at 100.00% (393230 / 393232): the 2 unknown bytes 28:$8620 and 28:$9883
remain unknown, so whole-ROM classification is **incomplete**. This audit does not change that.

## What was checked
- The redirect r003-a findings (analysis/handoffs/round-003-redirect.md, analysis/redirect/r003-a/notes.md)
  against every name and comment they affect: scoreboard figures, catcher/umpire objects, L-016
  (hit by pitch / brawl scene), play results and wall ids, the lineup model and L-018 (EDIT team tables).
- Equates for duplicate names of the same table, and for bank attributions that did not match the data.
- Routine headers that the round-3 handoff flagged as terse. Each was checked for a concrete error rather
  than rewritten for style. Most are accurate one-line helpers (setters, NMI no-ops, shared _Return entries).
- Headers stating "uncalled"/"no caller found" were compared with the static references in the source.
- Stale numeric aliases in prose ($0356/$0357/$0363/$03A1/$0364) and stale "tentative" wording on meanings that are now confirmed.
- review.json: every scaffold entry and unit has status reviewed, a summary and valid source line(s).

## Findings fixed
- Confirmed meanings renamed. BatterPowerStat -> BatterHomeRuns, BatterAvgStat -> BatterAverage,
  PitcherShownStat -> PitcherEra and PitchSpeedShown -> PitchSpeedMph. The scoreboard routines are now
  ScoreboardBatterHomeRuns, ScoreboardBatterAverage, ScoreboardPitcherEra, ScoreboardPitchSpeed and
  ScoreboardPitcherName. The bank 6 Old* copies were renamed to match.
- Name collision removed: HomeRuns/VisitorRuns (team run totals) -> HomeTeamRuns/VisitorTeamRuns, and
  ScoreboardHomeRuns/VisitorRuns -> ScoreboardHomeTeamRuns/VisitorTeamRuns.
- Catcher (object 2, confirmed): the PitchMarker* family became UpdateCatcher, CatcherInit,
  CatcherAwaitPitch, CatcherAfterCatch, CatcherPoseFromPitch, CatcherPoseColumn/Row, CatcherPoseTable and
  DrawCatcherFar. DrawBallShadow -> DrawCatcherSprite, BallShadowSpritePtrs -> CatcherSpritePtrs and
  ClearMarkerObject -> ResetCatcherState. Object 10 (umpire) stays tentative everywhere.
- L-016: the team banner names became brawl-scene names: InitBrawlScene, UpdateBrawlScene, BrawlSceneStep,
  BrawlStep*, BrawlScroll/SoundTable, BrawlTeamColours, SetBrawlTeamColours and @brawlScene.
  The comments on batter group 2, BatContact and PitchOutcome now state a confirmed hit by pitch.
- Play results and walls: PlayResult 1-4 is now documented as a home run over fence section 1-4, and
  WallPlayResult/WallHit document wall ids 1-4 (fence sections) and 5 (foul-side walls).
- L-018 / lineups:
  - RosterRowPtrs -> StartingOrderPtrs, a per-team starting-order table that also covers DH teams.
  - The duplicate equates TeamRosterPtrs and TeamInfoPtrs were removed, so only PlayerRecordPtrs and
    PitcherRecordPtrs remain. Their two duplicate review units were dropped.
  - The bank attribution of these equates was corrected to bank 21.
  - The InitRamTables comment and its three data blocks now identify the EDIT team records.
  - VisitorLineup/HomeLineup document the slot model.
- Wrong headers fixed:
  - DrawTeamRoster/RosterRowAddr/DrawRosterName/DrawRosterNumbers did not draw anything. They are now
    InitTeamLineups/LoadStartingOrder/BuildPitcherList/BuildBenchList, which build lineups.
  - ReadTeamTableByte was called "uncalled" and read pitcher records, so it is now ReadPitcherRecordByte.
  - CpuChooseBench's "pitcher variant" label was reversed: bit 1 is the pinch hitter, so it is now
    CpuChooseBench_PinchHit/CpuChooseBenchPinchHit.
  - CpuBatterSituation was a relief-pitcher check, so it is now CpuReliefCheck. CpuFlags bits are documented.
  - PitchSituation was a fatigue update, so it is now UpdatePitcherFatigue.
  - DrawStatBlockA-D/DrawSpecialValue are now DrawPanelAverage/HomeRuns/PitcherEra/Stamina and
    PitcherStaminaLeft.
  - WallZoneC_Wall4 was labelled "uncalled" but is used by the live dispatcher, so it is now WallZone_Wall4.
  - The bank 21 $AB88 data note wrongly listed a RAM-address routine as a consumer.
- Stale raw aliases in prose were replaced with symbols. The two unknown-byte notes now cite L-015 instead
  of "redirect question".

## Remaining explained uncertainties (commentary/uncertainties.md)
- Object 10 as the umpire is tentative.
- The left/right assignment of wall sections 2-4 comes only from the static line equations.
- The scoreboard "MPH" label tile reading is probable.
- The meanings of the Safe/HitsAllowed counters are not established.
- Clock $03E5/$03E6, BootABFlag and the write-only bytes are unresolved.
- Team 7's last starting-order pair (05, 4) is unexplained.
- About 180 accurate but terse helper headers and the mechanical local branch labels (@skip/@loop) were
  kept deliberately; this is not a cosmetic rewrite.
- Bulk data (bank 18 screens, bank 27 animation, bank 14 intro) is described at block level.
- The 2 unknown bytes (28:$8620, 28:$9883) remain unknown, with explanatory notes. Whole-ROM
  classification is incomplete.

## Audit tool
`python3 /root/romatlas/scripts/commentary.py audit <workspace>`. The result is recorded in
commentary/audit.json and summarized in analysis/handoffs/final-editorial.md. The tool checks inventory,
review status and the rebuild. It cannot judge prose quality; this document records the human-level
editorial review.

---

# Final editorial audit, round 4 (pass-004-fallback rebase)

Input: checkpoint pass-004-fallback (maps/current.json = analysis/state.json, sha256 c4dd4ae8...3dec).
The only change from pass-003 is the user-directed manual fallback for 2 bytes (regions 1252/1253,
Data, Manual, confidence 0.5, no label). The pass-003 section above remains valid for everything else.

## What was checked
- Generated baseline diff (pass-003 vs pass-004-fallback before/game.asm): only comment lines added at
  phys 230960 and 235667. No directive, label, boundary or bank context changed elsewhere, so the
  pass-003 review credit for the other 1230 entries and all 1937 units stays valid.
- Carried source line references: all 3167 point at the same text as before (0 mismatches).
- The two new Data entries: the notes state Data by user-directed fallback, meaning unknown, value $60
  (would decode as RTS), neither code nor data proven, with L-015 and analysis/handoffs/manual-fallback.md
  cited. No label is invented.
- Full-source sweeps: 0 address-style Sub_/Var_/Tbl_/region_ names; no 'format not established',
  'redirect question' or TODO notes; every global label has a header comment, except 26 shared
  _Cont/_Return entry points inside a commented routine, which were deliberately kept.
- No source comment still calls the 2 bytes unknown or uses the stale name PitchPhaseTest.

## Findings fixed
- The $9883 note referred to PitchPhaseTest; the routine is labelled CheckStrikeCall. Corrected.
- Both notes now say "manual fallback Data" instead of "Kept unknown", and no longer guess "probably a
  dead duplicate RTS" as the role, because no role is proven.
- uncertainties.md: the 2 bytes are re-described, and the stale redirect-question list (items resolved by
  r003-a) is reduced to the questions that are still open.

## Remaining explained uncertainties
- As listed for pass-003 above. In addition, the code-vs-data role and the meaning of 28:$8620/$9883 stay
  unresolved under the manual fallback.
- Classification: 393230 bytes are evidence-backed and 2 bytes are a manual user-directed fallback (0 unknown).
  This is not fully evidence-backed classification.
