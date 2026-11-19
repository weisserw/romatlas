# Round 001 commentary — current-source readability complete

All current classified source has descriptive names and functional explanations.
There are no pending current-source readability ranges or known naming/comment
backlog gaps. This completes commentary for the unchanged current map, not whole-
ROM classification or final editorial/archive acceptance. Final audit remains unrun.
Historical interrupted checkpoints -a and -b are preserved; do not reprepare source.

Scope: Code22423, Data12091, Image131072 =165586/262160 (63.16%) classified;
96574 unknown bytes retain honest physical-boundary/uncertainty notes. All1598
scaffold entries and1253 unit references are reviewed:591 existing meaningful
routine/structure records plus662 scaffold-fragment references, with
summaries/source locations. Exact remaining inventory is [] in
round-001-commentary-remaining.json. Valid prior review credit was retained.

C completed the remaining5756 Code bytes: fixed NMI/IRQ, OAM/CHR/video handshakes,
column/tile/palette transfers, serial controller edges and held state, guarded
bank0 sound calls, HUD decimal/health/row construction and scheduling; eight RTS
pointer adapters, actor path/probe/depth/terrain helpers and banked handler scans;
interval allocation/ticking, divide/multiply, mapper/collision wrappers, persistent
signature/counter, player response fragments, stage timer, candidate bounds and
OAM layer permutations. The entry physical122889 crosses PRG14/15 and retains its
full classified extent and fixed execution view. Prior actor/reset review remains.

Examples: F14Nmi and its request-bit branches explain frame service ordering;
F14PollControllers names held/previous/edge bytes; F15ClassifyDepthBand explains
seven thresholds and bands0..7; F15WriteMapperBank explains IRQ guard and I-flag
behavior; F15Divide24By8 describes big-endian quotient/remainder; the OAM routines
name forward/reverse allocation and layer permutations. Known RAM fields, branches,
record/count constants and bank-identity bytes have operand symbols.

Concrete dependency corrections: interval records are flags plus frame counts,
with duration supplied only by$03E5. Bank8/bank13/fixed source aliases and affected
inventory summaries now use byte frame durations, not16-bit intervals. Decimal
formatting consumes the remainder and big-endian input bytes. Player response
consumes input nibbles. Path initialization clobbers A, so its$FF completion test
is not described as a direct resolver-invalid check. Forward actor rendering
retries a zero motion byte; backward rendering advances. Literal active-type
comparisons against$1E/$1F omit bit7 masking; source explains their ineffective
equalities without silently repairing the game algorithm.

Unresolved meanings are localized in source and commentary/uncertainties.md:
numeric terrain/actions/path selectors, some HUD/persistent-counter gameplay roles,
and unclassified dispatch tables/continuations. These have mechanical names and
honest limits, not pending explanatory work or invented class boundaries. L007
remains a structural redirect question: physical$576/bank0 CPU$8566 is classified
BRK but also consumed by the alternate duty lookup. Classification stays unchanged.

Fresh supported verify passed after the final source edits, and read refreshed
commentary/reading-after.asm. ROM SHA256:
ad0ba0b8b4f6dc25d0244cf649a1a60e240c86218e4069dd40a3beff04fd9860.
Source game.asm SHA256:
6f972ebacb35a19a5ea85d6253ecbf4f360e0365469abc8c5a578b6b7a753342.
Source symbols.inc SHA256:
8b40973ed352e544f3ac054898288cda0265e32093af5ba69e2aa7b842e5fbb0.
Inventory summaries, source-tree-relative locations, unit bounds, per-Code unit
references and scaffold identity checked against the actual consumers. No pending
regions/units or Code entries without references remain.
Current map equals frozen baseline; current state equals frozen phase1-state.

Input ROM, maps/state/classification, before/ and toolkit remain unchanged. One
confirmed toolkit issue NES-001, zero suspected/missing-feature findings; buttons-
list workaround suffices. Disk7.2GiB free. No archive, cleanup, deletion, publication
or competing phase. Redirect can now proceed; reserve final editorial audit for
final acceptance after structural work and any resulting commentary refresh.

Metadata correction: all2189 original region/unit locations now use game.asm:line,
resolved relative to commentary/source, preserving line numbers. Both local helper
emitters were corrected. Read-only archive.editorial_status reports1598/1598 regions
and1253/1253 unit references valid; its sole remaining reason is the intentionally
pending final editorial audit. See round-001-commentary-metadata-check.json.
A fresh supported verify/read after metadata correction retained the source/ROM
hashes above. Reading-after.asm SHA256:
1aa4ee10f32f75c4eb6967b7948811879ba79a967718c0e66fa83f71c3280ca2.
