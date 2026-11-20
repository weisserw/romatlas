# Round 7 commentary handoff (final acceptance)

Status: COMPLETE. `commentary.py audit` run once at the end: ok, byte_identical true,
1,400 regions and 1,330 units reviewed, editorial_audit true, reading copy refreshed
(commentary/reading-after.asm). Source sha256 game.asm 1ef05260..., ram.inc
fdf0b3c2..., hardware.inc 5a088100... (commentary/audit.json).

## Baseline / carry-forward
- maps/current.json and analysis/state.json differed from the round-6 baseline only in
  region 742 evidence text (bank 2 $85C6-$860F, "SongHeaders" -> "SongPatternBases").
- Whole round-6 commentary preserved untouched at analysis/round-6/commentary/
  (round-1..5 copies not touched). `commentary.py prepare` built a fresh baseline from
  pass-007; before/game.asm identical to round 6.
- notes/, tools/, progress.md, uncertainties.md carried; tools/review.py now diffs
  against round 6. annotate build: game.asm/ram.inc/hardware.inc/lines.json
  byte-identical to round 6; checknotes 0 problems; 0 generic labels.
- review.json: 1,399 entries carry earlier credit; region 742 re-reviewed (round 7).

## Editorial findings
- Round-6 open note "map evidence for 742 still says SongHeaders" resolved; names agree.
- Retired-name search (SongHeaders, SoundDataLow, ChampionPictureData, 1,329-byte
  InstrumentNoiseBytes) finds only historical "corrected" mentions in uncertainties.md.
- No remaining known editorial issues. No names/comments changed this round.

## Remaining uncertainties
Unchanged from round 6 (commentary/uncertainties.md): 0:$8F84, PieceTableUnreadA/B,
StoryTileRecords $9149/$9154 reads, LegalScreenTiles reader, 2:$AC37/$B059, 3:$AF42,
4:$A474, 5:$8388/$88F4, 6:$8400/$8701, F:$E9AC/$FF2C; per-song identities; partly
decoded record fields.

## Structural questions for redirect
None. Toolkit: no new issues; not modified.
