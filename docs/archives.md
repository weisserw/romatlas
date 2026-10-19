# Analysis archives

`python3 scripts/archive.py archive WORKSPACE` writes
`atlas/<platform>/<game>/<full-rom-sha256>/` by default. `--name` supplies a
lowercase hyphenated game slug; `--output` selects the
archive directory. Existing archives are replaced after validation, with rollback
if publication fails. The directory must already contain `archive.json` to be
eligible for replacement. A failed archive/restore leaves no final new directory.

`python3 scripts/archive.py restore DIRECTORY --rom ROM --out OUTPUT` requires
an independently supplied matching ROM, this toolkit, and the pinned platform
assembler tools. For Game Boy these are mgbdis/RGBDS from `scripts/setup.sh`; SNES
uses the pinned Asar tool. It publishes `OUTPUT/source/` and
`OUTPUT/reading-after.asm`. The source files and generated assets match the final
commentary workspace exactly; reproducible assembler/linker products are excluded
from that comparison. Output must be new and outside the repository. Either
command accepts `--binary` to select a built ROMAtlas executable.

## Archive contents

- `archive.json`: format version, platform/name, ROM identity and filename, size,
  editorial completion status and pending-review reasons, explicitly omitted
  documents, and SHA-256 for all other archive files.
- `editorial.json`: versioned editorial reconstruction metadata described below.
- `maps/current.sym` (Game Boy) or `maps/current.json` (SNES): final portable map.
- `analysis/state.json`, `annotations.json`, `current-audit.json`, `scenarios.json`:
  final classification metadata, explanations, classification audit and scenarios,
  where available.
- `analysis/investigation.md`, `gameplay.md`, `progress.md`: final agent records.
- `commentary/baseline.sym` (Game Boy) or `commentary/baseline.json` (SNES), plus
  `phase1-state.json` and `manifest.json`: frozen phase-one baseline identity and
  structural interpretation used for the commentary phase.
- `commentary/review.json`, `audit.json`, `verification.json`: semantic review and
  final editorial/fidelity checks.
- `commentary/progress.md`, `uncertainties.md`, `editorial-audit.md`: final editorial
  documentation, where available.
- Additional final Markdown documents selected with repeatable `--document` paths
  relative to the workspace. Unknown Markdown documents are reported; use
  `--omit-document` to classify scratch documents explicitly.
- `README.md`: restoration instructions and the omission policy.

Original documents remain verbatim, including historical paths and links to
scratch evidence. The archive README identifies that evidence as unavailable.
Scratch requests/results/decodes/traces/scripts, intermediate maps/checkpoints,
workspace instructions/prompts, ROMs, generated assembly/reading copies, extracted
assets, object files and `analysis/toolkit-issues.md` are excluded. Incomplete
worktrees can be archived: ROM identity, maps, source reconstruction and a fresh
byte-identical source verification remain required. `editorial_complete` is false
when review entries, source locations or the final editorial audit are incomplete;
the review records and reasons remain in the archive. Port preparation requires a
complete reviewed archive. An archive is not a resumable runtime/debug workspace.

An editorial audit does not prove semantic correctness or complete phase-one
classification. Both phases' recorded statuses and uncertainties remain intact.

## Editorial recipe version 1

The recipe holds `baseline_sha256`, a filename-to-hash inventory of regenerated
baseline source/assets, and `files`, the exact final file inventory. An unchanged
file references its baseline filename. Changed/new `.asm` and `.inc` files hold
their final SHA-256 and an ordered sequence of operations:

| Operation | Meaning |
| --- | --- |
| `copy: [filename, start, length]` | Copy a UTF-8 decoded baseline substring; offsets/lengths count Unicode characters. |
| `comment: string` | Agent editorial comment, including its trailing newline where present. |
| `symbol: string` | Agent symbol edit or definition token. |
| `constant: string` | Numeric token in an agent-authored EQU/SET definition. |
| `include: string` | Quoted include/asset path edit within the source directory. |
| `format: string` | Whitespace formatting edit. |

Unchanged lines and token runs reference the baseline, including instructions and
ROM-derived data. Numeric formatting can use baseline character references;
preexisting string/data content is never stored as a new string literal. Comments
are recognized outside quoted strings. New quoted data absent from the baseline
and changed/new binary assets are rejected rather than copied. The recipe is not
a standalone source listing or a stock `.sym` extension.

Restoration copies the supplied ROM temporarily under its original workflow
filename to reproduce generated headers, generates from `commentary/baseline.sym`,
checks the complete baseline inventory, restores every final file and checks its
hash, validates include paths, then runs a fresh byte-identical rebuild. Archive
creation additionally compares the recovered inventory/content against the actual
final workspace. This does not depend on preserved `before/`, generated source,
scratch evidence, host-specific workspace paths or the original workspace itself.

`scripts/test-archive.py` tests the lifecycle using a synthetic ROM: exact complete
and incomplete source recovery, comments/constants/includes, reviewed unknown
regions, scratch/toolkit exclusions, extra documents, completion status,
replacement/stale-file removal, binary-asset rejection, failed-replacement
preservation, wrong-ROM rejection, archive integrity and existing-output rejection.
