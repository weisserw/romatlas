# Round 3 commentary handoff

- Phase: commentary, round 3, on checkpoint **pass-003** (maps/current.json = analysis/state.json
  = maps/pass-003.json, sha256 d9f78600...afbd5cf; 511 regions).
- Status: **complete. Current-source readability covers ALL classified source**: all 511 scaffold
  entries, file range $0000-$FFFF (physical 0-65551), header and CHR included. **No ranges remain.**
- Final editorial audit / archive acceptance was intentionally **not** run; editorial_audit stays pending.
- ROM classification is unchanged and reported separately: 100.00% (65552/65552, 0 unknown), structural only.

## Baseline change handling

1. The map changed (pass-002 -> pass-003: 54 Data regions split into 158 pieces, $F192-$F19F
   Data -> Padding, 8 code regions Static -> Observed).
2. The whole round-2 commentary directory moved unchanged to **analysis/round-2/commentary/** (source,
   review, frozen baseline, notes, reading copies). It is the only copy; analysis/round-1/commentary/ is
   untouched.
3. `commentary.py prepare` made a fresh frozen baseline (before/ = generated pass-003 source), verified
   byte-identical.
4. Carry by physical ranges (analysis/tools/commentary-round3/carry.py, adapted from round 2):
   - round-2 game.asm copied; 38 region markers rewritten to pass-003 text, 104 markers inserted at the
     new split points;
   - 6 split points fell inside an .incbin line ($8AA2, $BFF6, $C88A, $CD37, $CD47, $CDE7); those lines
     were split over the same bytes;
   - marker order checked against before/game.asm; verify byte-identical right after the carry.
5. review.json regenerated with the round-1/2 method (`cm.py review`): 375 entries keep their round-2 text
   unchanged, 2 revised (TitleSpriteData, EnvelopeTable), 134 new split pieces checked against the
   edited source. Totals: 511/511 reviewed, 987 units.

## Edits this round (batches: analysis/tools/commentary-round3/batches/)

- **L-017 (resolved):**
  - code operands: `#<HookCurvePtrs/#>HookCurvePtrs` (ApplyHookAndAdvance), `BallWeightDigits`
    (ChooseBallWeight), `Award275PortraitLists` (Award275), `Award250PortraitLists` (Award250),
    `SignGirlTileLists` (AnnounceBowler), `SoundCmdDispatch-1` (UpdateChannel return push);
    PerfectFramePtrs now `.word PerfectCharFrames0..3`;
  - tables split and commented: HookCurvePtrs + HookCurve0-8 (mirror-symmetric around straight curve 4);
    BallColorTable / BallWeightDigits (7, 9, 11, 13, 15) / BallWeightPpuAddr; PerfectObj0Frames,
    PerfectCharFrames0-3 and labelled perfect-game records (PerfectFrameHidden, PerfectChar*Frame1/2,
    PerfectObj0Frame1); Award275/Award250 list pointers + Char0-3 Pairs A/B (26 / 22 pairs);
    PlayerSignColors, SignGirlTileLists, SignPlayer1-4 Tiles A/B (20 tiles);
  - sound: SoundEnvelopes / SoundHeadersAndStreams labels and notes; UnrefSoundHeader_EFC7 (channels 4-7
    -> single End bytes), UnrefSoundFragment_ED45/EDD8/EF83; envelopes $1C-$28 never selected; duration
    indices 1-27 only; no $80-$EF command byte.
- Map labels adopted: every map label now exists in source (checked by script).
- Own comments for about 90 split pieces (frame tables, sprite-record blocks, attribute bytes, frame/
  scorecard templates, selection/title/celebration/award templates, screen descriptors and data blocks).
  Stale combined-region notes and raw addresses in comments removed ($8523, $9F95/$9F99, $86C1,
  "not split further, lead L-003").
- Other: PortraitTilePtrs split into Portrait0-3 PoseA/PoseB lists ("highlighted" replaced by a tentative
  blink/expression note); SelectionObjA-D frame labels inside TitleSpriteData (object A's frame runs on
  into object D's records); header and UpdateHighScores updated for L-014 (ranks 2/3 observed
  naturally); 27 adjacent .incbin fragments merged (no byte change).

## Readability assessment

- One source file, commentary/source/game.asm (11.3k lines, 802 global labels).
- 0 auto or address-only labels apart from deliberate unknown-role labels (UnrefTilePairs_CDE7,
  UnrefSound*_xxxx, Stream_xxxx); 1 intentional raw RAM operand (Reset RAM clear); no raw ROM addresses in
  code operands or pointer words.
- Gap scan of global labels without their own comment: only labels covered by a group comment (frame
  records, curves, envelopes/streams, SndCmd*, library groups), as in round 2.
- Routine/table headers, RAM/constant equates and inline notes from rounds 1-2 are carried unchanged
  except for the corrections above.
- Sprite records, RLE nametables and the bowler record block remain labelled .incbin with format notes
  (L-010, formatting only).

## Verification

- `commentary.py verify`: ok=true, **byte_identical=true** after the carry, after each edit batch and
  fresh at handoff (commentary/verification.json, source sha256 67b63802...4a51 at last run).
- Reading copy commentary/reading-after.asm refreshed with `commentary.py read`.

## Unresolved meanings

Details in commentary/uncertainties.md: character 0 handedness; "PND" units; CONTROL label for the spin
meter; tile $43 glyph; celebration naming (TilesA not seen naturally); which selection object is which;
portrait pose B; sign number tiles; UnrefTilePairs_CDE7, ScorecardData's first 16 bytes and
UnusedBestScoreData; ClearScores `,y/,x` slip; one ignored 4-frame B press; sound id meanings; uncalled
library code and islands (L-001/L-002).

## Structural questions / leads

- No new structural question; commentary proposes no map change.
- L-017 resolved (leads.json, round-3-commentary pass entry). L-010 updated (list/pointer tables now
  formatted; per-record sprite lists remain, open/low). L-001 deferred, L-014 deferred residual (provenance).
- No new lead IDs.

## Housekeeping

- Markdown created this phase: this handoff only, plus the moved round-2 copy under
  analysis/round-2/commentary/ (its own AGENTS/workflow/prompt/progress/uncertainties .md). commentary/*.md
  come from prepare (progress.md and uncertainties.md rewritten).
- Tools: analysis/tools/commentary-round3/ (cm.py copy, carry.py, batches/). Scratch /tmp/r3 removed.
- No server started; no toolkit change; no new toolkit issue.
