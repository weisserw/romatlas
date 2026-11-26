# Commentary progress

## Round 3 (checkpoint pass-003, 511 regions)

- Baseline: the map changed (pass-002 -> pass-003), so the whole round-2 commentary was
  moved to analysis/round-2/commentary/ and `commentary.py prepare` made a fresh verified
  baseline (byte-identical).
- Carry (analysis/tools/commentary-round3/carry.py): round-2 game.asm copied, 38 region
  markers rewritten (8 Static->Observed code, $F192 Data->Padding, first pieces of split
  regions), 104 markers inserted at the new split points; 6 split points fell inside
  .incbin lines, which were split over the same bytes. Verified byte-identical before
  any edit.
- Edits (batches in analysis/tools/commentary-round3/batches/):
  - L-017: HookCurvePtrs/HookCurves (curves 0-8 as bytes), BallColorTable /
    BallWeightDigits / BallWeightPpuAddr, PerfectFramePtrs -> PerfectCharFrames0-3 and
    labelled perfect-game records, Award275/Award250 portrait list tables and pair lists,
    PlayerSignColors / SignGirlTileLists / SignGirlTiles, UnrefTilePairs_CDE7,
    SoundCmdDispatch-1 push, sound notes (SoundEnvelopes, SoundHeadersAndStreams,
    UnrefSoundHeader_EFC7, three UnrefSoundFragment_* labels, unused envelope ids).
  - Map labels adopted (HookCurvePtrs, HookCurves, PerfectCharFrames, SoundEnvelopes,
    SoundHeadersAndStreams, list tables); every map label now exists in source.
  - Own comments for ~90 split pieces (frame tables, sprite-record blocks, attribute
    bytes, scorecard/frame templates, screen descriptors and data blocks); stale
    combined-region notes and raw addresses removed ($8523, $9F95/$9F99, $86C1, L-003).
  - PortraitTilePtrs split into 8 lists (Portrait0-3 PoseA/PoseB; "highlighted" replaced
    by a tentative blink/expression note); selection-object frame labels added in
    TitleSpriteData.
  - Header and UpdateHighScores updated for L-014 (ranks 2/3 observed naturally).
  - 27 adjacent .incbin fragments merged (no byte change).
- review.json regenerated with the round-1/2 method (cm.py review): 511/511 reviewed,
  987 units; 375 entries unchanged from round 2, 2 revised, 134 new pieces.
- Verification: `commentary.py verify` ok, byte_identical after every batch and at
  handoff; reading copy refreshed.
- Deferred ranges: none for readability. editorial_audit stays pending (final acceptance).
