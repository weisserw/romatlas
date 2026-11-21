# Commentary progress

Prepared verified baseline (commentary.py prepare from maps/current.json == maps/pass-002.json).

## Round 1 commentary pass (checkpoint name: commentary-001)

Scope: the whole current source (all 8 PRG banks, header, CHR) was brought to the naming/comment
standard in one pass. Classification was not changed (phase-1 regions and unknown bytes are
emitted exactly as before; only labels, symbols, comments and .org/.reloc wrappers were added).

How the source is produced (reproducible): `python3 analysis/commentary-tools/cmt.py build`
transforms the frozen `commentary/before/game.asm` with the annotation files in
`analysis/commentary-tools/notes/*.txt` (labels, header comments, inline comments, operand
overrides, RAM/constant symbols) into `commentary/source/game.asm`. `cmt.py review` rebuilds and
refreshes review.json (scaffold intact; summaries, source lines, units). Edit the notes, not the
generated source, when continuing this work.

Results:
- 1880 labels; 372 documented units (routines/tables with header comments); every JSR target and
  every table-dispatched entry has a descriptive name (no Sub_/Data_ placeholders remain).
- ~350 RAM symbols with sizes (zero page, object arrays, sound driver, streamer, bank-6 objects),
  hardware/MMC1 register names, button/form constants substituted by context.
- Unknown ranges keep notes stating what was checked and what they most likely are.
- review.json: 1359 scaffold entries all have status/summary/source; editorial_audit left pending
  (final acceptance is a separate gate).

Main interpretations established in this pass:
- Bank 1 = sound driver (vector $8000-$802F; PlaySfx/PlayMusic in bank 7), DMC sample player and
  metasprite tables; bank 7 $E000-$FFC0 = DPCM samples (DmcSampleTable).
- Bank 2-4 = scene tables + PPU stream engine (op table $B900, ScreenLoadStep RTS dispatch).
- Bank 5 = PCM speech/sound player for the ending (PlayPcmSequence) + one recording.
- Bank 6 = object system (vitamin shots, enemies, mints), pickups/sparkles, object sets and set
  scripts, room link tables ($A5C1-$ABC1).
- Bank 0 = game logic; jellybean flavour = blob form (Ladder..Vitablaster), per-form handlers.
- Bank 7 $C322-$C521 = RoomFloorPtrs (256 words) followed by floor lists.

Verification: `commentary.py verify` byte_identical true (commentary/verification.json, copied to
analysis/commentary-001-verification.json). Reading copy refreshed (reading-after.asm).

Deferred: redirect leads L-010..L-016 (classification of unknown table/sample ranges), deeper
semantics of bank-6 set scripts, exact music/stream extents, final editorial audit.
