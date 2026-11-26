# Round 2 commentary handoff

- Phase: commentary, round 2, on checkpoint **pass-002** (maps/current.json = analysis/state.json
  = maps/pass-002.json, sha256 1476b029...830fb; 407 regions).
- Status: **complete. Current-source readability covers ALL classified source**: all 407 scaffold
  entries, file range $0000-$FFFF, header and CHR included. No ranges remain.
- Final editorial audit / archive acceptance was intentionally **not** run; editorial_audit stays pending.
- ROM classification is unchanged and reported separately: 100.00% (65552/65552, 0 unknown), structural only.

## Baseline change handling

1. The map changed (pass-001 to pass-002).
2. The whole round-1 commentary directory moved unchanged to **analysis/round-1/commentary/** (source,
   review, frozen baseline, notes, reading copies). It is the only copy and must be kept.
3. `commentary.py prepare` then made a fresh frozen baseline. Its before/ is identical to
   generated/pass-002/game.asm, and it verified byte-identical.
4. Carried by physical ranges (analysis/tools/commentary-round2/carry.py):
   - round-1 game.asm was copied and its region marker comments rewritten to the pass-002 regions;
   - 98 markers changed: 87 Static→Observed, plus the first piece of each of the 11 split regions;
   - 35 markers were added at the instruction-boundary split points;
   - no assembly line changed, and verify was byte-identical right after the carry.
5. review.json was regenerated with the round-1 method (`cm.py review`, which reproduces round-1
   review.json exactly from round-1 source):
   - 353 entries carry unchanged round-1 text;
   - 8 were revised because their comments changed;
   - 46 entries are new split pieces, re-checked against the source: AdjustHookForLane, ScoreBall,
     ScoreFrames, WriteTenthFrameMarks, TitleMenu, StrikeCelebration,
     UpdateHighScores/RevealHighScoreName and FadeInSpriteGroups;
   - totals: 407/407 reviewed, 909 units.

## Edits this round

- **L-013 (resolved):**
  - SpinMeterUnused → SpinMarkerY/Tile/Attr/X ($065C-$065F, OAM sprite 23, the spin-meter marker);
  - HideSprite23 → HideSpinMarker (`ldy #SpinMarkerY-OamBuffer`);
  - SetSpinMeterValues → PlaceSpinMarker;
  - SpinMeterTable → SpinMeterArc (nine (X, Y) arc positions);
  - the LaneBall* equates are marked as sprite 22;
  - the "never read" claim is gone.
- **L-015 (resolved):** runtime confirmations added to:
  - CheckScoreAward (natural 300/299/267);
  - AwardPerfectGame (car), Award275 (SPORTS NEWS / BCD TV), Award250 (beach);
  - FinalResultsScreen (HI SCORE / A PRIZE WINNER!, after B on the final card);
  - UpdateHighScores (rank 1 natural; ranks 2/3 only in a patched run);
  - ShowScorecard (opens on 1-6 before frame 7, Left/Right paging, B closes, Start/A not polled);
  - TitleMenu (wraparound, runtime-confirmed), TitleBowlerThrow (unskipped animation) and NextPlayer
    (P1/P3 controller 1, P2/P4 controller 2).
  - HighScores/HighScoreNames are no longer tentative.
  - The $A26A shared byte is documented as deliberate (L-011).
- Other edits:
  - the header now explains the Observed/Static markers;
  - ShowPlayersDigit had no comment and now has one.
- A gap scan of global labels without their own comment found only labels covered by group comments
  (SndCmd*, FillNametable1-3, frame tables, and so on), apart from ShowPlayersDigit.
- commentary/uncertainties.md is rewritten into resolved and open lists. Six round-1 entries are
  resolved (L-007, L-009, high-score table, awards/scorecard, SpinMeterUnused, $A26A).
- commentary/progress.md is updated.

## Readability assessment

- One source file, commentary/source/game.asm (10.8k lines).
- 1411 labels; 0 auto or address-only labels; 1 intentional raw RAM operand (Reset RAM clear).
- Routine/table headers, RAM/constant equates and inline notes are unchanged in quality from round 1
  apart from the corrections above.
- Generic .incbin data (sprite records, tile-pair lists, nametables) keeps boundary/role notes.

## Verification

- `commentary.py verify` gave ok=true and **byte_identical=true**. It was run after each edit batch
  and fresh at handoff; see commentary/verification.json.
- The reading copy commentary/reading-after.asm was refreshed with `commentary.py read`.

## Unresolved meanings

Details are in commentary/uncertainties.md:
- left-handedness of character 0;
- "PND" ball-weight units;
- the CONTROL label for the spin meter (not checked against the screen data);
- the tile $43 glyph;
- celebration naming (TilesA not seen naturally);
- ClearScores `,y/,x` slip;
- a one-off ignored 4-frame B press;
- uncalled library code and islands (L-001/L-002);
- undecoded data blocks (L-003/L-004/L-010).

## Structural questions / leads

- No new structural question. Commentary proposes no change to the map.
- Leads:
  - L-013 and L-015 are resolved (leads.json, with a round-2-commentary pass entry);
  - L-014 (provenance only) and L-010 (data formatting) stay open;
  - no new lead IDs.

## Housekeeping

- The only Markdown files created this phase are this handoff and the archived round-1 copy under
  analysis/round-1/commentary/ (which contains its own AGENTS/workflow/prompt/progress/uncertainties .md).
- Fresh commentary/*.md files come from prepare.
- Scratch: /tmp files removed.
- No server was started.
- No toolkit change and no new toolkit issue.
- Disk: about 2.6 GiB free.
