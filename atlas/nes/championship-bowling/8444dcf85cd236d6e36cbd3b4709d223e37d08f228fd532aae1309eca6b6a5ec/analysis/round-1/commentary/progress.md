# Commentary progress

Prepared verified baseline (pass-001, map sha256 f73ff2c8...). All semantic review entries started pending.

## Round 1 commentary (in progress)

Method: commentary/source/game.asm is edited in place with the helper
analysis/tools/commentary-round1/cm.py (parses the source by CPU address; applies small
edit batches: labels, block/inline comments, operand symbols, equates; converts selected
.incbin ranges to .byte/.word). Every batch is followed by commentary.py verify.

Done so far (all verified byte-identical after each batch):
- Raw branch/jump/data addresses replaced by labels; the whole PRG is assembled under one
  `.org $8000` so data labels have CPU values. 0 auto-generated (L_/Sub_/Tbl_) labels remain.
- Symbols section: hardware registers, library and game RAM, sound-driver frame fields.
  Only 1 raw RAM operand remains (`a:$0000,y` in Reset's RAM clear, intentional).
- Library $F1A0-$FF09, Reset/NMI, sound driver $E02B-$E458 and its tables/streams,
  game code $8020-$CF84: routine labels, local branch labels, block comments.
- Sound data $E620-$F19F rendered as labelled envelopes, headers (.byte/.word) and streams.
- Tables converted to .word with labels: SoundCmdJumpTable, NotePeriodTable,
  EnvelopeTable, SoundHeaderTable, screen descriptors; immediate pointer pairs use
  #<Label/#>Label.

- Object descriptor templates rendered as fields; frame pointer tables as .word with
  per-frame labels; sound ids (MUS_/SFX_) and ObjTable slot constants defined.
- Renamed during the pass: PlayerHand -> PlayerCharacter (portrait index), PlayerBallColor
  -> PlayerBallWeight, BowlerObj -> ActorObj (bowler/pinsetter/sign girl), TileArgPtr/TileSrc
  -> VramDest/VramSrc; $C2-$C7 identified as HighScores (tentative).
- review.json: all 372 entries reviewed with summaries/sources, 877 units; editorial_audit pending.

Result: current-source readability complete for all classified source; deferred ranges: none.
Unresolved dependencies: leads L-009..L-011 (new), L-001..L-007 updated, L-008 resolved-static.
Verification at handoff: commentary.py verify ok=true, byte_identical=true; reading copy refreshed.
