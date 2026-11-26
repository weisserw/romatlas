# Final editorial acceptance

- Checkpoint **pass-006** (maps/current.json = analysis/state.json = maps/pass-006.json =
  commentary/phase1-state.json, sha256 865f4d5a...ee973cf93; 512 regions). No map/state/ROM/toolkit change.
- Status: **complete**. Every completion gate is met (see below).

## What was fixed

- **L-023 (closed):** commentary/source/game.asm, PlaceSprite's Static path at $FC23 (physical
  31795..31810). The comment "Nothing in the data rules it out" is replaced. It now says the path
  runs when Y + record offset leaves 0..255 at either edge, keeps a sprite at most 7 pixels above the
  screen and hides others. It is dormant by data flow: every object's Y + offset stays within 9..232
  (analysis/redirect/r6-a1/evidence/fc23_bounds.json). The file-header paragraph on Static branches
  was updated to match. The DrawObject header now says no sprite list is built in RAM (every
  frame table pointer comes from ROM). commentary/uncertainties.md moves $FC23 to resolved.
- **L-010 (closed, editorial decision):** sprite record lists stay as 134 labelled per-frame
  .incbin includes (6,381 bytes, about 1,600 4-byte records). Frame pointer tables are already
  labelled .word, each group has a format note and the grammar is documented once at DrawObject.
  A sentence there now points readers to it. Per-record .byte expansion would add only raw pixel
  offsets and CHR tile numbers, with no symbolic field values and no new meaning.
- review.json: 76 source line references shifted for the 5 inserted comment lines;
  region-348 summary states the dormant disposition; editorial_audit recorded.
- Targeted greps for stale wording (no known caller, uncalled, TODO, FIXME, unobserved,
  not seen, open possibility, top-edge) and raw address operands found nothing else.
  Details: commentary/editorial-audit.md.

## Audit and verification

- `commentary.py audit`: ok=true, byte_identical=true, reviewed_regions=512, reviewed_units=988,
  **editorial_audit=true** (commentary/audit.json; review.json editorial_audit.status = reviewed,
  with notes and findings).
- Fresh `commentary.py verify` after the audit: ok=true, **byte_identical=true**, source
  game.asm sha256 **4bb03ed410c63bc89e38178cadc7f7b849fec3fe2bbd418e1834175d6d2d1d13**
  (commentary/verification.json). input/game.nes unchanged (sha256 8444dcf8...a5ec).
- Reading copy commentary/reading-after.asm refreshed (`commentary.py read`).

## Remaining semantic uncertainties (commentary/uncertainties.md)

Character 0 handedness; "PND" ball-weight units; celebration and selection-object naming
(objects A-D not matched to screen elements); portrait pose B; UnrefTilePairs_CDE7,
ScorecardData's first 16 bytes and UnusedBestScoreData; ClearScores `,y/,x` slip; one
ignored 4-frame B press; sound id meanings; purposes of unreachable routines/islands and
dormant NMI jobs (mechanics documented). None affects classification or rebuilding.

## Completion gates

| Gate | Result |
| --- | --- |
| Whole-ROM classification | 100.00% (65552/65552, 0 unknown) at pass-006 |
| Required annotations / review | 512/512 scaffold entries and 988 units reviewed |
| Valid maps | pass-006 generated source byte-identical (analysis/pass-006-verification.json); current = state = frozen baseline |
| Final commentary audit | passed, editorial_audit=true |
| Byte-identical verification | fresh verify passed (sha above) |

All gates are met. There is no open lead; L-010 and L-023 are closed. Archive was not run.

## Housekeeping

- Markdown created: analysis/handoffs/final-acceptance.md, commentary/editorial-audit.md.
  Edited: commentary/uncertainties.md, commentary/progress.md, analysis/progress.md.
- No server, commit, publish, archive or cleanup of workspace; scratch copies in /tmp removed.
