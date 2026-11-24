# Commentary progress

## Round 1 commentary (2026-11-24) - checkpoint pass-002

Method: commentary/source/game.asm is BUILT by tools/cmt.py from the frozen
commentary/before/game.asm plus annotation specs commentary/notes/*.ann
(labels, block/inline comments, operand symbols, RAM/constant equates in ram.inc,
inline-parameter/jump-table rendering, pointer-immediate #</#> rendering, data-region
reader notes). Do not hand-edit source/ (the build refuses to overwrite hand edits
without --force); edit the .ann files and run:

    python3 tools/cmt.py build
    python3 /root/romatlas/scripts/commentary.py verify <workspace>
    python3 tools/cmtreview.py          # refresh review.json source lines / units
    python3 /root/romatlas/scripts/commentary.py read <workspace>

`python3 tools/cmt.py todo` lists code spans without a covering routine comment (0 now);
`python3 tools/cmt.py list BANK [A B]` prints a compact annotated listing.

Scope covered: all 925 code regions (banks 0-7) and all 568 data regions.
- bank 7: API jump table (Api_* names), reset/NMI, bank switching, text printer, menus,
  multi-byte arithmetic, FDS-style VRAM/PPU library, fades/wipes/shake, 36 sound effects.
- bank 6: title, password entry/decoder, intro, King match, broke screen, ending, music driver.
- bank 5: casino overworld (scrolling, metatiles, people AI, sprites), status/password
  screen and encoder, cheat passwords, debug screen, opponent encounters/progress.
- bank 4: poker session, betting, AI, drawing, hand evaluation (categories documented).
- bank 3: blackjack session (static-only, tentative names), inline copy/clear helpers.
- banks 0-2: reset stubs and graphics-data notes.
- RAM: ~230 global/bank-scoped symbols in ram.inc; constants for banks/NMI tasks.

Verification: commentary.py verify -> ok, byte_identical (commentary/verification.json);
reading copy refreshed (reading-after.asm). review.json: all 1493 scaffold entries
status=reviewed with source lines, 1140 units; editorial_audit left pending.

Deferred / improvement backlog (not blocking readability): descriptive names for the
remaining ~960 mechanical local branch labels; per-record decoding of data tables;
runtime confirmation of blackjack/debug/broke-screen semantics (see uncertainties.md).
