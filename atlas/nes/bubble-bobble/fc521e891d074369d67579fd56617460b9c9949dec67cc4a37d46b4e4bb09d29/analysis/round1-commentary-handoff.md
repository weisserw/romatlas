# Round1 commentary handoff — current-source readability complete

Classification unchanged:133132/163856 bytes=81.249389708036% (81.25%). Code8970,
Data91394, Image32768;30724 unknown. This completes commentary on current source,
not whole-ROM classification or final editorial/archive acceptance.

Covered all4549 full-ROM scaffold intervals:4258 Code instruction regions,
24 Data regions,1 Image region and266 explained Unknown gaps. Source contains293
substantive mechanical routine/fragment explanations,740 routine/branch labels and
321 RAM/hardware/table/address aliases. Each classified code byte has a reviewed
owning unit; meaningful_units records exact classified subintervals so unknown
intervening bytes are not included in a code claim. No remaining source-review range.

Local source:commentary/source/game.asm; refreshed reading copy:
commentary/reading-after.asm. Frozen before/, baseline.json and phase1-state.json
are preserved. review.json retains the full scaffold and all covered unit contexts.
Final editorial_audit remains pending deliberately; no final audit/archive performed.

Examples: audio phrase/note pointers and envelope nibbles, bank10 boss sprite-pair
motion/burst accumulators, actor/player map probes and directional movement, six
cooperative task stacks and nonlocal yield, NMI/OAM DMA,24-row nibble map unpacking,
22-byte actor-record consumers, MMC1 LSB-first register writes, PPU queue format,
controller stable sampling/rising edges and caller-inline pointer stack contracts.
Unknown continuations and inferred visual meanings are stated locally. Generic
data boundaries retain the prior bank inspection and no unsupported table extents.

Fresh final commentary verification is byte-identical; see
commentary/verification.json. Final game.asm SHA256:
e0bbb595a126222574d2b35d1162db4edeba5f61190a0282744e107e7c067d6a.
Input SHA256 unchanged:
fc521e891d074369d67579fd56617460b9c9949dec67cc4a37d46b4e4bb09d29.
Map/state unchanged from pass-003 and frozen commentary baseline; no annotations
or toolkit changes. Analysis helpers and mechanical notes remain local.

Unresolved meanings and concrete bounded redirect checks are in
commentary/uncertainties.md. In particular investigate boss-zone hit continuations/
defeat and ending, stage record consumers, sound inline operands and heterogeneous
executable-bank gaps. Ending remains unvisited; no gameplay replay in this phase.
Existing analysis/leads.json remains authoritative; no terminal exhaustion claimed.

Toolkit report:existing NES-001 confirmed documentation/API validation bug; no new
issues in commentary, no blockers. See analysis/toolkit-issues.md. No publication,
archive, deletion, cleanup or live server. Disk checked at handoff (~3.5GiB free);
scratch remains under1MiB. Source and evidence stay local; only supported ROM-free
maps/current.json is a sharing deliverable.
