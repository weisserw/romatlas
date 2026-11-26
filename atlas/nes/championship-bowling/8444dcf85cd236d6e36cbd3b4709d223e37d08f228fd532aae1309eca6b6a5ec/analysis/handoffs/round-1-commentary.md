# Round 1 commentary handoff

- Phase: commentary, round 1, on checkpoint **pass-001** (maps/current.json = analysis/state.json
  = maps/pass-001.json, sha256 f73ff2c8...c35c3a). commentary/ was prepared fresh this round
  (no earlier commentary existed); frozen baseline: commentary/before/, baseline.json, phase1-state.json.
- Status: **current-source readability complete for all classified source** (all 372 scaffold
  entries, $0000-$FFFF file range incl. header and CHR). Final editorial audit / archive
  acceptance was intentionally **not** run (editorial_audit stays pending).
- ROM classification is unchanged and separate: 100.00% (65552/65552, 0 unknown), structural only.

## Verification

- `commentary.py verify`: ok=true, **byte_identical=true** (fresh run at handoff;
  commentary/verification.json, game.asm sha256 2a0fa2e1...). Every edit batch was verified.
- Reading copy refreshed: commentary/reading-after.asm (`commentary.py read`).

## Scope and readability assessment

Source: commentary/source/game.asm (single file, 10.7k lines; binary pieces via image.nes).
- Labels: 724 global + 687 local (`@name`); 0 address-only auto labels (L_/Sub_/Tbl_) remain.
  Some data labels are address-derived where the role is only mechanical (Stream_XXXX for
  sound streams, <Object>FrameNN for metasprite frames).
- Symbols: 249 equates - hardware registers, library and game RAM, sound-driver frame fields
  (SND_*), sound ids (MUS_/SFX_), ObjTable slot constants. Only `a:$0000,y` (Reset RAM clear)
  is left as a raw RAM operand. Immediate pointer pairs use #<Label/#>Label; PPU nametable
  addresses stay numeric and are described in routine comments.
- Comments: ~750 block-comment lines plus ~320 inline comments. Every routine has a header
  comment or belongs to a commented group (e.g. Add16/Sub16/Inc16/Dec16, SndCmd* handlers,
  FillNametable0-3); data tables have role/format notes or a boundary/uncertainty note.
- Code covered: game $8020-$CF84 (flow, pins in two views, ball/hook, meters, scoring,
  scorecard, title/menu/selection, celebrations, awards, high scores, screen decoders),
  sound driver $E02B-$E458, Reset/NMI and library $F1A0-$FF09, all phase-1 Static islands.
- Data made structural (byte-identical): SoundCmdJumpTable/NotePeriodTable/EnvelopeTable/
  SoundHeaderTable as .word; NoteDurationTable/OctavePeriodOffsets/PaletteSets as .byte;
  envelopes, 25 sound headers and ~200 streams labelled; screen descriptors as .word;
  22 ObjTable descriptor templates rendered field by field; 23 frame pointer tables as .word.
- Remaining generic .incbin data (sprite records, tile-pair lists, nametables) carry
  boundary/role notes; their internal splitting is lead L-010 (not a readability blocker).

## Examples

- NMI (F260): RenderOffState handshake, VRAM job queue order, sound tick.
- DrawObject (FB57): descriptor/record grammar ($FF/$FE/$FD).
- ScoreBall/ScoreFrames/StrikeBonus/SpareBonus (A757-AA75): FrameMarks nibble codes and
  running totals in FrameTotals.
- UpdateLanePins/ChoosePinDirection/MoveLanePin (8741-8883) and the close-up fall tables.
- UpdateChannel (E149): stream grammar; the $80-$EF `bcc *` trap explained.
- PinsetterObjTemplate (9F8B): explains the $9FFA word from lead L-002.

## Unresolved meanings (details: commentary/uncertainties.md)

Title-menu control mapping vs gameplay notes; P3/P4 controllers at runtime; exact on-screen
meaning of spin/power meters, celebration and award screens; tile $43 glyph; high-score
display timing; unused/unreferenced code and data (DebugActorFrameViewer, DrawBlock9FFB/A045,
SetFrameAreaAttrs, KnockDeckPinSideways, SetBowlerColors, FinalAwardsTest, UnusedBestScoreData,
CHR-RAM uploaders); ClearScores loop slip (`sta FrameTotals,y` counted by X).

## Structural questions for redirect (evidence-policy.md applied)

- L-010 (new, data format): split sprite-record and tile-pair blocks at the now-labelled frame
  tables and $FF terminators. Bounded, structure-only; ranges listed in analysis/leads.json.
- L-011 (new, annotation): $A26A is both the RTS of DrawLaneOverviewFromRom and the first byte
  read by `lda LaneOverviewTiles,x`; decide whether the map should note the dual use (keep code).
- L-009 (new, gameplay input): confirm title-menu Left/Right = players, Up/Down = lane.
- Updated: L-002 (word $9FFA is a frame pointer to $FE), L-003/L-004 (partial structure done in
  source), L-006 (high-score table identified statically), L-007 (static answer: bowlers 0/2 pad 1,
  1/3 pad 2), L-008 marked resolved-static (no $80-$EF command bytes in reachable streams).
- No change to structural classification is proposed by commentary itself.

## Records

- commentary/review.json: all 372 scaffold entries status=reviewed with summaries and
  game.asm source lines; 877 units (global labels; provenance-only code continuations recorded
  as `<routine>_part_<offset>` units). editorial_audit: pending (final acceptance only).
- commentary/progress.md, commentary/uncertainties.md updated.
- Tool: analysis/tools/commentary-round1/cm.py (view/xref/hex/apply/words/bytes/tmpl/frames/
  relabel/ptrs/fold/review). It edits game.asm in place; it does not regenerate source.
- Scratch: edit batches lived in /tmp (removed); no scratch .md files were created in analysis/
  or commentary/. No server process was started. No toolkit changes; no new toolkit issues
  (TK-001..TK-004 unchanged).

## Remaining gaps

- None for current-source readability: ALL classified source is covered.
- Not done by design: final editorial audit/archive acceptance; deeper decoding of generic data
  blocks (L-010); runtime confirmation of tentative meanings.
