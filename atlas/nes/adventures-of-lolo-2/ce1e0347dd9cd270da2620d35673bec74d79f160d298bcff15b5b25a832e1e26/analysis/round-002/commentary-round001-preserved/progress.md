# Round 001 current-source commentary

The frozen baseline is `baseline.json`, `phase1-state.json`, `before/` and
`reading-before.asm`. Original classification/evidence and ROM are unchanged.
All 341 classified code blocks (7,306 bytes) have been read and given functional
names and behavioral explanations; five classified data/image regions and all 141
unknown spans have boundary/format or uncertainty explanations. Common RAM fields,
hardware registers and classified control targets use symbols consistently.

First coherent source batch passed byte-identical verification. Current work is
checking naming precision, shared RAM overlays and the full scaffold's source links.
Final editorial acceptance remains pending; no audit has been run. Concrete unresolved
script and bank-preservation questions will be recorded for redirect after this scope
check and a fresh verification/reading-copy refresh.

## Current-source scope complete

Reviewed all 487 full-ROM scaffold entries: 341 code regions, five classified
structures and 141 unknown spans. Retained 86 meaningful routine review units;
source contains 635 code labels and 202 RAM/register definitions. Names cover
script animation and signed fractional motion, password navigation/checksum,
room progression and hazards, direction-dependent probes, object movement,
OAM formats, inline PPU descriptors and circular queue, NMI/CHR service, and
note/envelope audio interpreters. Shared fields explicitly state their overlays.
No current classified-source readability backlog remains.

Fresh final verification passed (`verification.json`, `byte_identical: true`).
The reading copy was refreshed with the supported `read` command. Scope and hashes
are in `analysis/round-001-commentary-scope.json`; original state/current/checkpoint
maps are unchanged. Frozen baseline and prior disassembly evidence are preserved.
The source hash is `2c0a0e4b05d1833b8103df9afbd517c83a6923ea538d5b01e41fcb839a0cb309`.

Unresolved semantics and concrete redirect questions are in `uncertainties.md`.
Final editorial audit remains pending for final acceptance; no audit was run.
Whole-ROM classification remains 63.07%, with 24,206 unknown bytes.
No new toolkit issues arose; existing TK-001 is one missing feature, no bugs or blocker.
Disk inspection at handoff found 9.7 GiB available; commentary evidence is 124 KiB.
No cleanup, publication, archive, toolkit edits or redirect was performed.

### Coordinator gate metadata repair
Normalized every region/unit source reference to source-tree-relative `game.asm:line`,
as required by the commentary/archive resolvers. Validated all 573 filenames and line
numbers under `commentary/source`. Updated local annotation helpers to preserve that
convention and strengthened the scope check to validate both region and unit links.
Fresh verification remains byte-identical with the same source hash; reading copy
refreshed. Source and frozen baseline were unchanged. No final audit was run.

### Coordinator gate unit-coverage repair
The earlier inventory's 86 units omitted per-region unit metadata for 255 explained
code sub-blocks. Added reviewed units for those existing named/explained blocks,
using their exact physical extents, behavioral summaries and source labels. The
current inventory has 341 routine/sub-block units and zero code regions without
units; this supersedes the earlier 86-unit count. All 828 region/unit source paths
and line bounds were validated, as were unit extents inside their scaffold regions.
Fresh byte-identical verification and reading refresh passed; scope evidence updated.
Source, original scaffold boundaries and frozen baseline remain unchanged. No final
editorial audit was run.
