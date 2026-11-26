# Commentary progress

## Round 6 (checkpoint pass-006, 512 regions)

- Baseline: the map changed (pass-005 -> pass-006: region 386 $A92B Static -> Observed;
  evidence text added to fade, sound, page-carry, DrawObject $FD and $FC23 regions). No
  boundary or kind change. The whole round-5 commentary was moved to
  analysis/round-5/commentary/ and `commentary.py prepare` made a fresh verified baseline
  (byte-identical).
- Carry (analysis/tools/commentary-round6/carry.py, the round-5 method): round-5 game.asm
  copied, 1 region marker rewritten (region 386 provenance). No insert or .incbin split.
  Marker list equals before/game.asm; verified byte-identical before any edit.
- Edits, L-022 (batches/01-l022.txt): $A92B ($D first-ball miss counts as 0);
  AnnounceBowler is the only FadeDelay = $FF window; dormant notes on the stepped
  FadeInSpriteGroups body, BrightenGroups, TakeNextFadeGroup, FadeOutAll's instant path and
  BrightenColor's cap; sound envelope page carry, EnvelopeRestart (new header) and the
  attenuation floor; the three impossible page carries ($AAF7, $AB07, $BDEB); DrawObject $FD
  unused by ROM lists; $FC23 top-edge clip unobserved; header paragraph.
- Concrete defects fixed (batches/02-defects.txt): ShowLaneScreen said "instantly" although
  FadeDelay $0F makes a slow 15-frame-per-step fade; SetChannelVolume now mentions the
  floor at 0.
- review.json regenerated with cm.py review: 512/512 reviewed, 988 units; summaries change
  only for region-15 (ShowLaneScreen) and region-272 (EnvelopeRestart); region-512's
  hand-written summary carried. editorial_audit stays pending.
- Verification: `commentary.py verify` ok, byte_identical after the carry, after each batch
  and at handoff; reading copy refreshed.
- Deferred ranges: none for readability. Final editorial audit reserved for final acceptance.

Earlier rounds: analysis/round-5/commentary/progress.md and older copies.

## Final editorial acceptance (pass-006)

- Edits: $FC23 comment (L-023, dormant by data flow), file-header Static paragraph, DrawObject $FD
  note (no RAM-built lists) and a pointer to the record grammar for the per-frame includes (L-010
  closed by decision). review.json source lines shifted (+5 lines after the header), region-348
  summary updated, editorial_audit = reviewed. Details: commentary/editorial-audit.md.
- `commentary.py audit` ok, byte_identical, 512/988 reviewed; fresh verify byte-identical
  (game.asm sha256 4bb03ed410c63bc89e38178cadc7f7b849fec3fe2bbd418e1834175d6d2d1d13); reading copy refreshed.
