# Archive repair record (2026-11-20)

## Diagnosis

`scripts/archive.py archive` failed with "unrepresentable source edit in game.asm" on a quoted
`.byte "..."` string operand. The archiver stores the final source only as references into the
regenerated frozen baseline plus editorial literals (comments, symbols, formatting, constants of
EQU/= lines, include paths). A quoted data string that does not already occur in the baseline is a
new original-data literal, which the recipe deliberately refuses. The baseline renders these
regions as `.incbin` slices, while the commentary phase had respelled 113 name-table/text data
lines in `commentary/source/game.asm` as ASCII string literals (title/legal text, menu and record
screen text, fight messages, and one tile-index row read as text under the `Txt_Aaaa...` label).
A dry run of the toolkit's own `recipe()` confirmed these lines were the only obstacle.

## Edit

- Backup: `analysis/archive-repair-backups/source-game.asm.pre-repair` (sha256 3bb9ca97...127cec).
- In `commentary/source/game.asm` only, each quoted string operand on a `.byte` line was respelled
  as the equivalent comma-separated `$XX` byte list (ASCII code of each character; no escapes or
  non-printable characters were present). 113 lines changed; line count (14,676) and every line
  number are unchanged, so review.json source locations still point at the same lines.
- No label, comment, directive, region boundary, classification, review entry, frozen baseline,
  map, evidence or ROM byte was changed. `commentary/before/` and `commentary/baseline.json`
  are untouched. `commentary/verification.json` was refreshed by `commentary.py verify`
  (new source sha256 e662e2fb...1575).

## Verification

- `commentary.py verify`: ok, byte_identical true. `romatlas verify input/game.nes --generated
  commentary/source`: byte_identical true.
- Toolkit `recipe()` dry run against `commentary/before/game.asm`: succeeds; no quoted operand
  outside comments is absent from the baseline.
- Disposable probe archive with all `--document`/`--omit-document` options from
  `archive-options.json` (this file added as a final document): ok true, exact_source true,
  byte_identical true. The archiver's review_status reported editorial_complete true
  (767/767 regions, 1,048/1,048 units) from the existing review metadata; this repair does not
  claim a new editorial review of the respelled lines.

## Residual limitations

- The changed lines now show hex bytes instead of readable text; the descriptive `Txt_*` labels
  and preceding comments still identify each string. One untitled line (the HOT-B copyright row
  at $99EE) has no label. Final editorial completion was not required for this repair and the
  editorial content was not re-reviewed for prose; the earlier strict audit result applies to the
  pre-repair spelling.
- Residual research uncertainties are unchanged from `analysis/handoffs/final-acceptance.md`.
