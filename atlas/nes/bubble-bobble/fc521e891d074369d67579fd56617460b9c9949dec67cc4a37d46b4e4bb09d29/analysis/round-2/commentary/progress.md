# Commentary progress

Prepared verified baseline. All semantic review entries are pending.

## Round1 active mechanical review

Read all4258 classified instructions in physical order using
analysis/commentary-mechanics.txt. Durable routine explanations in
analysis/commentary-units.txt cover sound, boss motion, actor/player collision,
cooperative task stacks, stage records, NMI/PPU and mapper/controller helpers.
These are being applied to source with consistent symbols. All source region
scaffold entries still pending until transformed source is checked. Remaining:
apply explanations/symbols, inspect gaps and data, verify edited assembly, update
review scope, refresh reading copy, write final handoff. No phase completion yet.

Applied293 substantive routine/fragment explanations and740 contextual labels across
all4258 instructions. Replaced non-immediate memory operands with hardware/RAM,
record-field or explicitly unresolved CPU-address aliases, retaining encoded widths.
All266 unknown gaps now have local uncertainty notes; data/image/header/vector
regions carry format/boundary explanations. First coherent edited-source verification
passed byte-identically (source hash eaddcd9a0b4f558a4d5438b5394b3d42c58da5786700f8a1b3c99049987badb2).
Final review currently checks summary precision and established table names;
review records, reading-copy refresh and final handoff remain.

## Round1 commentary complete on current source

All4549 scaffold regions reviewed:4258 Code instruction regions (8970 bytes),
24 Data regions (91394 bytes),1 Image region (32768 bytes),266 Unknown gaps
(30724 bytes) with honest boundary notes. Distinct meaningful units:293, with740
routine/branch labels and321 exact RAM/register/table/address aliases. Unknown
ranges remain unknown, and table names describe consumer bases without invented
extents. The full classified-code subinterval list per meaningful unit is recorded
in review.json. No remaining source-review ranges; no classification changes.

Final verification passed byte-identically and reading-after.asm was refreshed.
Final source SHA256:e0bbb595a126222574d2b35d1162db4edeba5f61190a0282744e107e7c067d6a.
See verification.json. Input, state/map and frozen baseline hashes checked unchanged.
Final editorial audit remains pending; incremental readability does not establish
whole-ROM completion. Unresolved meanings and bounded redirect questions are in
uncertainties.md; handoff:analysis/round1-commentary-handoff.md.

No new toolkit issue, existing NES-001 unchanged and nonblocking. No live server,
publication, archive or cleanup/deletion. Disk3.5GiB free; scratch756KiB.
