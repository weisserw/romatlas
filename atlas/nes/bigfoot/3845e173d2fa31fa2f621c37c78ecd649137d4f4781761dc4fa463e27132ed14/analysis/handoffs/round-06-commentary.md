# Round 6 commentary handoff (final acceptance)

Status: COMPLETE. Current-source readability scope (pass-006, 100.00% classified) done;
final editorial review recorded. `commentary.py audit` passed once, at the end: ok,
byte_identical true, 1,400 regions and 1,330 units reviewed, editorial_audit true,
reading copy refreshed (commentary/reading-after.asm).

## Baseline / carry-forward
- Map changed (pass-006). Whole round-5 commentary preserved untouched at
  analysis/round-5/commentary/ (round-1..4 copies not touched).
- `commentary.py prepare` built a fresh frozen baseline from pass-006. notes/, tools/,
  progress.md and uncertainties.md carried; notes re-applied by bank/CPU address.
  checknotes initially flagged 5 comment keys at removed splits (0:$90ED/$9138/$9154,
  2:$8B45/$8B74); rewritten or deleted. Now 0 problems.
- review.json: 1,350 entries keep earlier credit; 50 entries whose source text changed
  (tools/review.py diff vs round-5 source) re-reviewed as round 6.

## Changes
- 0:$90B2 StoryTileRecords: 27 records (rows*16+width size byte + tile ids, unread
  separators), pointed to by StoryTextBlocks frame records, copied by
  NmiHandler10_TextQueue; $9149/$9154 not observed read (static extent).
- Bank 2 sound: ChannelOrderListPtrs $83D0 (SoundDataLow retired); SongChannelOffsets =
  absolute bases $83D0/$8430/$8490/$84F0; SongHeaders renamed SongPatternBases (was a
  wrong "song header" description: it is the pattern base address per request);
  SongPatternTables = pattern-offset table pointers; InstrumentTable 31 x 8 field
  layout (InstrumentNoiseBytes now a field label, "1329 bytes" removed); SoundOrderLists
  command format; SoundPatternArea $8C15-$9C85 (28 continuation notes repointed from
  the removed $8B45 note); NotePeriods 28 bytes. SoundTick/StartMusicChannel(s)/
  StartSfxChannel(s) comments, sound RAM comments and driver branch labels updated.
- 2:$B63D ChampionPictureData -> ChampionPalette2 (32-byte palette only);
  Bank2FillFF and F:$FF57 FixedBankFillFF documented as unread $FF fill.
- Note: phase-1 map evidence for region 742 still says "SongHeaders"; commentary name
  is SongPatternBases (map label is null there).

## Remaining uncertainties
commentary/uncertainties.md (round-6 section). Open: 0:$8F84, PieceTableUnreadA/B,
StoryTileRecords $9149/$9154 reads, LegalScreenTiles reader, 2:$AC37/$B059, 3:$AF42,
4:$A474, 5:$8388/$88F4, 6:$8400/$8701, F:$E9AC/$FF2C; per-song identities; partly
decoded record fields.

## Structural questions for redirect
None blocking; no map change proposed.

## Artifacts
commentary/source/{game.asm,ram.inc,hardware.inc}, review.json, audit.json,
verification.json, reading-after.asm, progress.md, uncertainties.md; notes edits in
r2_b012, b2, b0_3, r2_headers, r3_tables_06, bF_3, new r6_sound; tools/review.py now
diffs against round 5. Toolkit: no new issues; not modified.
