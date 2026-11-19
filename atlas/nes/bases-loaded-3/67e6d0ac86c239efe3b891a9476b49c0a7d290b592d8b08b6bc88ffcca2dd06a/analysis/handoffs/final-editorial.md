# Final editorial acceptance — commentary (pass-003)

Status: **complete** (the editorial acceptance gate passed). Whole-ROM classification is still
**incomplete**: 100.00% (393230 / 393232); the 2 unknown bytes 28:$8620 and 28:$9883 remain unknown (L-015).

## Scope
- Resumed the pass-003 commentary in place. maps/current.json and analysis/state.json are unchanged
  since prepare, so nothing was re-prepared. The ROM, map, state, toolkit and before/ are untouched.
- Folded in redirect r003-a (analysis/handoffs/round-003-redirect.md, analysis/redirect/r003-a/) and
  L-016/L-018.
- Applied the final-acceptance fixes for contradictory or stale names, misleading certainty, wrong
  headers and a duplicate-equate cleanup. There was no whole-source reread and no cosmetic rewrite.

## Main fixes (full list: commentary/editorial-audit.md)
- Scoreboard: BatterHomeRuns, BatterAverage, PitcherEra and PitchSpeedMph, with matching Scoreboard*
  routines. Team run totals were renamed HomeTeamRuns/VisitorTeamRuns to avoid a clash with home runs.
- Catcher (object 2): the PitchMarker* routines became Catcher* routines. DrawBallShadow -> DrawCatcherSprite.
- L-016: the team-banner routines became BrawlScene* routines. Hit by pitch is stated as confirmed.
- PlayResult 1-4 = home run over wall section 1-4; wall id 5 = foul-side walls.
- L-018:
  - RosterRowPtrs -> StartingOrderPtrs (DH teams 6-11).
  - TeamRosterPtrs/TeamInfoPtrs were merged into PlayerRecordPtrs/PitcherRecordPtrs.
  - InitRamTables and its blocks are documented as EDIT team data.
  - The lineup slot model is documented.
- Wrong headers or names corrected:
  - lineup builders (formerly "Draw*Roster*")
  - ReadPitcherRecordByte (was "uncalled" ReadTeamTableByte)
  - the CPU relief/pinch-hit variants (the pitcher label was reversed) and CpuReliefCheck
  - UpdatePitcherFatigue (was PitchSituation)
  - the stat panel routines (were DrawStatBlockA-D)
  - WallZone_Wall4 (was mislabelled as uncalled)
  - the bank 21 $AB88 consumer note

## Audit result
`python3 /root/romatlas/scripts/commentary.py audit <ws>` returned ok:
- byte_identical true; source game.asm sha256 510b20cf...e4a5
- reviewed_regions 1232; reviewed_units 1937; editorial_audit true
- commentary/audit.json written; reading copy commentary/reading-after.asm refreshed

## Residual uncertainties (commentary/uncertainties.md)
- Object 10 as the umpire is tentative.
- The side mapping of wall sections 2-4 is static only.
- The "MPH" label tiles reading is probable.
- Team 7's last starting-order pair is unexplained.
- Safe/HitsAllowed counters, clock, BootABFlag and the write-only bytes are unresolved.
- Terse helper headers and mechanical @-labels were kept.
- The 2 unknown bytes stay unknown.

## Toolkit
No issues found. Scratch /tmp/fe67 was removed.
