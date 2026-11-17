# Round 1 commentary handoff - COMPLETE (routine-level readability + label naming, continuation #4)

Input checkpoint: pass-001 (maps/current.json = pass-001, analysis/state.json unchanged); commentary
baseline in commentary/ (before/ frozen, baseline-verification.json byte-identical).

## Continuation #4 (label naming) - status COMPLETE
- Address-only labels: 0 left.  All 268 Loc<bank>_<addr> branch targets and all Tbl<bank>_<addr> data labels
  (186 unique in source, incl. cross-bank uses of bank 8/31 tables) now have descriptive names from their consumers
  (commentary/notes/84_labels_names.py).  Multi-byte records are named once; the renderer prints +1..+3 offsets.
  Stale comment references to old auto names (Loc00_8204, Loc30_C43F, Tbl31_E81D, ...) were rewritten.  The only
  remaining match is the `Sub05_A123` example in the game.asm naming legend.  UnkNN_xxxx labels (unclassified
  bytes) are left for the structural pass (new lead L013).
- Local labels: 621 mechanical @skip/@loop/@return replaced by descriptive locals (2,255 -> 1,634) in the
  game-critical routines (commentary/notes/85_local_labels.py, helper LL()); see commentary/progress.md for the
  routine list.  The remaining mechanical locals are in display/menu helpers (e.g. FarDrawLineupScreen,
  RosterCursorInput, GridHighlight, DrawMatchCursors, Acquire*/Release* pickers) and simple loops.
- Tentative names are listed in commentary/uncertainties.md ("Continuation #4").
- Verification: commentary/verification.json ok=true, byte_identical=true (fresh, after the last edit);
  commentary/reading-after.asm refreshed (commentary.py read).
- Lead added: L013 (Unknown regions read by indexed loads in classified code -> classify as data; jump tables
  bank 24 $8E57 / bank 31 $EAC3).
- Render pipeline unchanged: `cd commentary && python3 tools/dupcheck.py && python3 tools/render.py`, then
  commentary.py verify/read.  render.py warns "label note ... is not an instruction start" for a wrong L() address.

## Earlier continuation summary (continuation #3; the label gap below is closed by continuation #4)
Status: COMPLETE for routine-level readability. Every classified routine in every bank now has a
descriptive name and a code-based explanation; no "Purpose not yet established" summary is left
(previously 544 across banks 2, 3, 5, 8-15, 24, 25). Fresh verification is byte-identical, and the reading
copy has been refreshed. Editorial gap still open for the final audit: intra-routine branch labels
are still mostly mechanical (2,364 local labels, about 2,250 of them @skip/@loop/@return, plus 268
Loc<bank>_<addr> branch targets). Descriptive labels exist only at key branch points. editorial_audit
stays pending in review.json (left for the final audit).

## Verification / artifacts
- commentary/verification.json: ok=true, byte_identical=true (fresh, after the last edit).
- commentary/reading-after.asm: refreshed via commentary.py read.
- Source: commentary/source/*.asm rendered by `python3 tools/render.py` (run from commentary/) from
  before/game.asm + notes/*.py. Run tools/dupcheck.py first, because duplicate global names break the build.
- review.json: 3,749 entries reviewed, 0 pending; 997 units, all reviewed (tools/review.py, READ_RANGES now
  all code banks; ranges are merged so regions crossing bank boundaries count).
- render.py gained BV(banks, addr, name, doc, size), a bank-specific RAM alias (listed in ram.inc under
  "Bank-specific aliases"). It is commentary tooling, not the toolkit.

## Scope covered in this continuation (notes 73-83)
- Banks 2/3 tail (computer time-out/menu automation), 12 (base runners, lead-offs, force/tag outs,
  pitcher and pitch flight, computer defence/throw decisions), 13 (fielder tasks, dive/jump catches,
  throws, ball physics and bounces, fence/home run, camera), 8 (new-game set-up, line-ups, at-bat panel,
  half-inning rules), 9 (scoreboard/marquee, pitchers of record, home-run list, season/league stats),
  10 (PointDistanceAngle, edited-team creation with random ratings, league menu/new league), 11 (standings,
  match selection, play/watch/simulate, stat leader pages), 24 (RELEASE/ACQUIRE PLAYER), 25 (SELECT
  PLAYER card, team-points ability purchase, team save + checksum), 14 (bat contact, hit
  elevation/direction/speed, pitcher fatigue, runner icons), 15 (palette sets, info screen, radar), 5
  (stadium field scroller).
- Corrections to earlier work: HomeStadium/StadiumChrSel became BatterForm/PitcherForm (player-record
  bytes), CallBank14A29B became CallBank15_A29B, the wrong "target in bank 13" comments at the bank 30 jsr
  $BA02/$BA53 sites were replaced, and the ScoreRun tally index was fixed (charged pitcher). Mode21ViewTeam
  is now documented as creating a team (probably MAKE TEAM).

## Examples
- bank12.asm CheckTags / TagOutRunner / RecordOut; CpuRunnerCanMakeIt; ReleasePitch -> PitchSpeed/PitchDirection.
- bank13.asm FielderTask (34-entry task dispatch), CheckCatches -> FielderCatchesBall, BallIntegrate (bounce).
- bank09.asm DrawPitchersOfRecord; bank11.asm RankStandings; bank25.asm SubtractPrice (decimal digit strings).

## Unresolved meanings (see commentary/uncertainties.md, continuation #3 section)
- Thresholds and plan tables of the computer base-running and throw AI. Some fielder task numbers.
- Team-points purpose (inferred), leader-page categories (numbered only), Mode 21 naming.
- Code that continues into unclassified bytes: bank 11 $A995/$AA1E/$AA7F/$AB11/$AFCE, bank 24 $8E57 handlers.

## Leads added (analysis/leads.json, pass commentary-001)
- L010: banks 23/27/29 CPU $BA02-$BAFF, $BB48-$BBA5, $BCAD-$BFF8 are byte-identical copies of the bank 5
  field scroller code (only $BB99 differs). Classify as code for stadium pairs 11/13/14.
- L011: small Unknown regions in banks 12/13 that are indexed-load tables (data labels placed in source).
- L012: supplements L008. Mode 40 is set by CheckLeagueFinished, and bank 11 $B421-$B50C precedes the
  classified ChampionScroll tail, so $B428/$B450 are probably code.
- Earlier leads L007-L009 are unchanged.

## Toolkit issues
- No new toolkit issue. TI-002 (no generated labels) still applies; the render.py workaround is extended.

## Next
- Redirect/structural pass on leads L007-L012, or a final editorial audit (descriptive branch labels in long
  routines, especially banks 12/13/25/2) before archive acceptance.
