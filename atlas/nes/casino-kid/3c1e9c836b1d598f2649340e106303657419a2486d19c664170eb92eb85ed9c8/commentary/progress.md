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

## Round 2 commentary (2026-11-24) - checkpoint pass-003

Baseline: maps/current.json changed (pass-002 -> pass-003, 1493 -> 1582 regions). The whole
round-1 commentary is preserved in analysis/round-1/commentary/ (plus analysis/round-1/cmt.py,
cmtreview.py). commentary/ was re-prepared from pass-003 (`commentary.py prepare`). The notes/*.ann
specs were then carried over by address: they are keyed by bank + CPU address or physical
offset, so the Observed/Static re-splits needed no changes. Stale anchors were fixed: data
notes at $93ED (bank 5), $87F0 (bank 6) and $CFF6 (now code).

tools/cmt.py additions (local tool, not the toolkit):
- DL (labels inside a data region; the .incbin is split there)
- VR (routine-local RAM aliases)
- DN (default note for a bank's unreferenced data)
- data-label symbols in inline parameter words
- a duplicate-local-label check
tools/cmtreview.py now also adds review units for labelled data tables.

Changes:
- Region absorption:
  - all 21 pass-003 data labels adopted (cheat/preset passwords, Bj*/Pk* tables);
  - $CFF6 is code: commented remnant, labels @remnant_check_3f / @remnant_push_char;
  - the three "unlabelled" text spans got consumer-anchored labels (BjUiStrings,
    PkText* strings).
- Lead corrections:
  - L9: ROYAL STRAIGHT.
  - L10: blackjack result codes, payout renames, BjHitAssist, BET ALL MONEY / FINAL HAND
    menu, new Bj RAM names.
  - L11: debug A/B; Dbg* aliases.
  - L12: GAME OVER.
  - L13: 30-byte strings, 28 compared.
- Symbols:
  - every raw RAM operand now has a name. Notable new names: OpponentProgressBits,
    PokerOppMoneySaved, BjDealerMoneySaved, PasswordBytes/Nibbles, the 22 music driver
    arrays, ending/password buffers, FDS_* registers, StackPage, and the bank 7 FxTmp0-5;
  - every ROM table operand has a label (124 DL + 76 D labels: casino, status, encounter,
    debug, ending, King, music, SFX step tables and others). The only raw ROM operands left
    are the cross-bank entry slots ($8010/$8013/$8016/$8019/$801C), which carry comments,
    and the reset idiom `inc $8001`.
- Branch labels: all 966 mechanical @loop_/@skip_/@case_ labels have descriptive names
  (0 left), plus KingEffect* globals.
- Several wrong round-1 notes were corrected:
  - StatusCursorInit / PokerCursorInit were called "unused", but they are copied via
    PushLocals/PopLocals;
  - PasswordMoneyLimit was described as "no reference";
  - BjDealerName is 9 bytes, not 13.

Verification: `commentary.py verify` -> ok, byte_identical (commentary/verification.json).
Reading copy refreshed. review.json: 1582/1582 entries reviewed with source lines, 1419
units (routines + labelled data tables). editorial_audit is left pending (final acceptance
only).

## Final acceptance: editorial audit (2026-11-24, checkpoint pass-003)

Scope:
- Applied lead L15.
- Ran the reserved final editorial audit over the full current-source inventory. Round-2
  review credit was reused; only concrete inconsistencies were revisited.
- Edits were made only in commentary/notes/*.ann and tools/cmtreview.py. The source was
  regenerated with `tools/cmt.py build`.

Changes (details in commentary/editorial-audit.md):
- L15:
  - password 25-bit field layout (PackMoneyAndProgress / UnpackPasswordFields);
  - BjDealerRecordTable +10 and PkOpponentTriples +0 described as unread constants;
  - static writers of blackjack codes 1/2/3/8. Code 3 comes from BlackjackSession $8052,
    which corrects the redirect note.
- 16 new sub-table data labels replace Label+offset operands: TitleFlashPtrsA/B,
  TitleMenuFramePtrs, TitleSpriteFrames2, SceneChrSets, ScenePaletteIds, PasswordGridDesc,
  PasswordMessagePtrs, PwMessageFramePtrs, CursorDescAddrs, CursorOamSlots, CursorTilePtrs,
  CursorSizes, CursorAttrs, BetAmountPtrs and BetTierTable.
- MenuCursorPos is now 10 bytes, so the portrait animation bytes use their own names.
- Bank constants and comments were added for the remaining raw bank-6 and bank-3 calls.
- Two comment headers now match their labels, and 8 stale prose names were replaced.
- Notes were added for 7 undescribed tables.
- tools/cmtreview.py now takes data-table summaries from the consumer comment.

Verification:
- `commentary.py verify`: byte_identical=true after each batch.
- `commentary.py audit` passed: 1582 regions and 1435 units reviewed, editorial_audit=true,
  fresh byte-identical rebuild, reading copy refreshed (commentary/audit.json).
