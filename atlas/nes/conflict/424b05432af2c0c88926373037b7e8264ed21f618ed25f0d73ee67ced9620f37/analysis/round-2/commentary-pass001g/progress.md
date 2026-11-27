# Commentary progress

Prepared verified baseline. All semantic review entries are pending.

## Round1 batch01 — ongoing commentary

Prepared from pass-001g/map SHA256 7c2b49e7c4417c10fe6a7b5a6f107cb5b48a40b0c46b734bf80c3d139c5c4fbd.
Reviewed 14 small mechanical contracts in physical bank0: class-mask filter,
count-prefixed command append, bounded choice and option cursors, pointer dispatch,
confirm flags, panel-row drawing, temporary side service, strip copy, paired option
index, two-digit display conversion, decimal split, and threshold/class lookup.
Descriptive entry/branch symbols replace numeric targets inside the same bank only.
Unresolved quantities are stated near the algorithms; no runtime interpretation invented.
All275 unknown intervals now carry exact boundary/uncertainty notes, and all16 CHR
inventory chunks have the shared planar layout and mapper visibility caveat.

490 scaffold entries reviewed;7310 pending. Full source readability NOT complete.
Exact pending region identities/ranges: analysis/commentary-round1/remaining.json.
The next sequential code frontier begins with pending physical000020..000035 and
0000AF..000258, then the rest of bank0 from000617; selected reviewed islands are
recorded in review.json and must be reused. Header and all PRG data review remain pending.
Scratch compact instruction listing and parsed instructions are in
analysis/commentary-round1/; the original source/baseline remain intact.

Fresh batch verification: byte_identical=true; source game.asm SHA256
777c74c22d82f388364a29925c70428bfbccf2f86a2e051afe942e431720f80a.
No audit run; reading-after still preparation copy and must refresh at final handoff.
No live runtime/map writer; no ROM, map, toolkit, archive or cleanup mutations.

## Round1 batch02 — ongoing

Added37 grouped contracts covering the frame save/restore, descriptor unpacking and
normalization, inline stack dispatch, dialogue token interpreter/substitution/
scroll/history/character delay, equipment-name copy, PPU shadow helpers, NMI
synchronization wrapper, sound/display queues and controller serialization/edges.
Hardware symbols now replace hardware-address operands throughout classified code.
Battle RAM aliases replace established field/buffer/scratch roles only in physical
bank0 execution code; other modes retain their separate meanings.

1015/7800 scaffold entries reviewed;6785 pending. Exact remaining.json refreshed.
Byte-identical verification passed after semantic batch and again after symbols.
No final audit; current source gate still incomplete. Next: remaining bank0 modal
settings, battle-choice callers and fixed record consumers, then subsequent banks.

## Round1 batch03 — ongoing

Bank0 classified code now has grouped contracts across every remaining accepted
fragment, including choice/action flow, side packing and modal settings. Raw holes
are explicitly localized; no implied whole-routine closure across unknown bytes.
Named the PPU stream decoder flags, repeated-fill/literal payload and palette exit,
and its pointer arithmetic including the full-page Y=0 case. Corrected the nested
modal delay interpretation before handoff: fall-through doubles the inner service
counts, so the four-call ladder performs16 cycles, not5.

1621/7800 scaffold entries reviewed;6179 pending. Bank0 code pending count is zero;
its data is still pending. Fresh byte-identical verify passed before the comment/
symbol correction; next batch verification will include the correction. No live
process remains. Exact remaining.json refreshed; next bank is physical004010+.

## Round1 batch04 — data readability complete, code ongoing

Reviewed all223 classified PRG/header Data/Text regions against grouped primary
format evidence (field-data-review, stage-data-review and accepted record proofs).
Source now explains equipment PPU packets, dialogue pointer aliases/token format,
map-expansion records with localized selector14/15 exceptions, counted field tile
messages, sprite/unit material and vectors. Printable runs show their local text;
uniform runs retain short boundary/purpose-unknown notes. No forced complete schema
for generic spans. Preserved the 23-byte parameter prefix after the RTS correction.

1844/7800 reviewed;5956 pending, all pending entries are Code. Exact remaining.json
refreshed. Fresh byte-identical verification passed including modal ladder fix.
Restored74 unknown boundary notes after data comment replacement; next verify
includes that comment-only restoration. Source readability gate remains incomplete.

## Round1 batch05 — tactical animation bank complete

Reviewed764 accepted Code regions in physical004010..008010: actor16-byte fields,
phase/timer/equipment handlers, command dispatch, alternating forward/reverse
metasprite rendering with clipping, status markers and sprite-zero split polling.
Unclassified handler/table holes remain explicit. Added mode-scoped actor-field
symbols; separated the slot-indexed command list from sixteen-byte actor records
sharing $7100. The same address has distinct aliases according to proven access.

2608/7800 reviewed;5192 pending Code, beginning physical008040. Fresh byte-identical
verify passed6b7b0cae… including restored unknown notes and all batch05 code.
Corrected8 command-list operand aliases after that verify; next verify includes it.
One local edit initially referenced a branch just beyond a unit range; expanded
that unit over the already-classified fragment and reverified successfully.
No toolkit issue: this was source-edit bookkeeping, fixed before proceeding.

## Round1 batch06 — setup/state-machine code complete

Reviewed860 setup-bank Code regions physical008010..00C010. Named setup state,
PPU/CHR/palette/copy/input/frame-ready aliases; distinct from battle-side RAM uses.
Explained timed intro/reveal/scroll, sprite-zero CHR split, menu groups, secondary
options, stage-selection bounds/page transitions, highlight sprites/attribute edges,
state dispatch via constructed RTS and helper installation. Composite scaffold
regions retain identity but now include meaningful internal helper/loop labels.
Setup PPU record bit6 meaning is explicitly opposite to the tactical-bank consumer.

3468/7800 reviewed;4332 pending Code in banks3/4/6/7. Exact remaining.json refreshed.
Fresh byte-identical verify includes the prior8 command-list aliases and all setup
symbols/comments. Initial unit-bound/internal-label edit errors were localized and
fixed, with successful full rebuild before proceeding; no toolkit defect claimed.

## Round1 batch07 — audio bank code complete

Reviewed305 audio Code regions: eight logical tracks/four physical channels,
queue consumption and hardware shadow commits, durations/period/octaves, envelope
pairs with saturating attenuation, overlay priority, stream jump/call/loop commands,
stop semantics and inline dispatch. Added audio-specific track-field/pointer/shadow
and APU-register operands. Unsupported token-handler/table holes remain raw.

3773/7800 reviewed;4027 pending Code in banks4/6/7. Fresh byte-identical rebuild
passed after all audio symbols; exact remaining.json refreshed. Full source gate
still pending. No live process, toolkit/ROM/map mutation or cleanup.

## Round1 batch08 — field-command bank code complete

Reviewed359 bank4 Code regions: fixed21-byte row loading, encoded type pair lookup,
queued target record bounds/copy, synthetic axis steering/confirmation, six-neighbor
mask/capability gates, property lookup and32-unit turn-flag reset. Named field
coordinates, queue fields, packed strength/ammunition, comparison-code scratch and
mask source operands. Specific game command identities behind mask/table outputs
remain unresolved rather than named speculatively. Property byte $03BE is distinct
from center-neighbor code $03AE; corrected that distinction before verification.

4132/7800 reviewed;3668 pending Code in banks6/7. Fresh byte-identical verification
passed after all bank4 symbols and corrections; remaining.json refreshed. Review
source locations are recalculated from current physical-region markers after each
batch and will refresh again with exact semantic entries at final handoff.

## Round1 batch09 — field display/unit bank code complete

Reviewed886 field-bank Code regions with stable entry veneers, PPU row/column
flushing, counted tile-message/newline mechanics,24-piece portraits, action-panel
rows/cursors, world-unit coordinate search/selected-record writeback prefix, context
sprites, staged comparisons, neighbor eligibility, encounter descriptors and band
rendering. Added field-specific source/destination/message/record/OAM aliases.

The encounter constructor requires a coordinate-matching world target and traps
forever when none is found. Source now states this concrete precondition and keeps
artificial relocated-mirror evidence narrower. Staged comparison $03FA has exact
0->1->3 byte comparisons documented; objective/campaign meaning remains unresolved.

5018/7800 reviewed;2782 pending Code, all in fixed physical bank7. Fresh byte-identical
verify passed after symbols; exact remaining.json refreshed. Current-source scope
is still incomplete and no redirect handoff/audit is claimed.

## Round1 batch10 — fixed lower8KiB code complete

Reviewed662 accepted Code regions physical01C010..01E010: reset/interrupt DMA,
setup-to-field initialization, ordinary/queued field input loops, panel slide,
bank4 context wrappers, capability/action selection, neighbor encounter prefix,
factory response prefixes, cursor choices and stage/option parameter initialization.
Raw gaps are localized in each partial contract; calls/observed fragments are not
promoted to complete command handlers. Exact staged comparison/counter semantics
remain separate from any campaign-terminal claim.

5680/7800 reviewed;2120 pending Code, all upper fixed8KiB physical01E010+.
Fresh byte-identical verify passed17b85b31… after correcting one unit endpoint over
an already classified tail. Field operand aliases for fixed bank will be reconciled
with the complete fixed-helper contracts after the remaining code is explained.
Exact remaining.json refreshed; no live process or redirect handoff.

## Round1 batch11 — fixed tile/scroll expansion contracts complete

Reviewed960 upper-fixed Code regions through physical01EAB8: three-level tile
pointer resolution, circular horizontal/vertical3x2-cell strip scattering,
two-pixel scroll animation and bounds, bulk half-row flush, three-slab eight-byte
stage expansion, attribute shadow transfer and world-unit overlays. Duplicate
coordinate/flag suppression is documented without inventing ownership/transport
meaning. Selector14/15 format exceptions remain localized in source.

6640/7800 reviewed;1160 pending Code, primarily attribute quadrant updates,
cursor/neighborhood sampling, movement commitment and fixed input/mapper helpers.
Fresh byte-identical verify passeda31d8372…; exact remaining.json refreshed.
Fixed-mode symbol consistency remains scheduled after helper contracts are complete.

Count correction: batch11 typed962/6642 initially; actual review.json delta is960,
with6640 reviewed and1160 pending. Counts below will be derived from inventory.

## Round1 batch12 — attributes, cursor and movement

Reviewed865 more Code regions with attribute quadrant merges/pointer algebra,
cursor repeat/scroll, seven neighbor PPU addresses and buffered tile sampling,
terrain-code lookup, movement ammunition costs, selected-coordinate update, world
commit trap, ownership-bit unit counts and unsigned multiplication. Reading the
movement consumer corrected the earlier neighbor-selector role: source now names
movement selection rather than implying a tactical encounter.

Inventory-derived counts: 7505/7800 reviewed;295 pending Code (fixed veneers and
input/mapper/PPU helpers). Fresh byte-identical verify passed after role correction;
fixed-field operand reconciliation remains before final phase verification.

## Round1 batch13 — full contract scaffold, consistency ongoing

All7800 scaffold entries now reviewed:7525 classified regions plus275 explained
unknown intervals. The last295 Code regions cover fixed veneers, controller edge
semantics, split/PPU timing, bank history, memory fill/reset and audio wrappers.
Fresh byte-identical verify passed8e62bf92… . This is contract coverage, not yet the
phase gate: fixed-field operand aliases, context-safe cross-bank symbols, resolved
helper role corrections and current semantic source references remain in progress.
Final fresh verify and reading-copy refresh follow those checks. No audit run.

## Round1 final consistency assessment

All7800 scaffold entries covered:7525 classified regions plus275 exact unknown
notes. The387 meaningful units have specific contracts/formats; source has1245
named labels and358 equates. Fixed-field/controller/mapper operands now match the
established roles. Renderer piece count is distinct from actor slot despite RAM
reuse; multiply/fill scratch has local aliases. Resolved fixed helper roles corrected
battle-side polling and NMI/control descriptions. Movement selection and tactical
encounter remain distinct. Seventy branch/block labels completed known target naming;
a misplaced setup packet label and one colliding label were corrected before verify.

Bank-aware control aliases use every saved effective PRG view and require a unique
physical target with matching CPU symbol value.154 such links are retained in
analysis/commentary-round1/call-symbol-links.json. The12 remaining numeric control
operands all target structurally Unknown bytes; no classified direct target is left
numeric. Source locations now point to current semantic labels and exact scaffold
markers, rather than stale preparation lines.

Inventory, map/input identities, known-control naming, unknown-note coverage and
source-reference checks passed in analysis/commentary-round1/scope-assessment.json.
No semantics-pending scaffolding remains. This is a full current-source readability
assessment; final editorial audit remains pending and was not run. Whole-ROM
classification remains68.23%, with83293 unknown bytes. No broader compatibility or
exhaustive gameplay claim follows. Final verification/reading refresh recorded below.

## Round1 commentary gate met

Final fresh verify: byte_identical=true, unchanged ROM SHA256
424b05432af2c0c88926373037b7e8264ed21f618ed25f0d73ee67ced9620f37.
Source game.asm SHA2567fefa470f60fbfb9b9f3c25176fd57ceb5161ae6adffb1740bed743a24b9f661.
reading-after.asm refreshed after verification; SHA256
e7de6aebf85a2c04b2d6b67f97ad8ee214b976d997e7469bdedc9f4ff5a45a18.
All7800 current references refreshed; exact remaining.json is empty. No classified
readability gap remains. Raw structural unknowns and semantic questions above remain.
Final editorial_audit is still pending by design; no audit/archive/publication run.

Compact coordinator/redirect handoff: analysis/commentary-round1-handoff.md.
No live server, map writer or tool handle remains. No runtime experiment was needed.
No ROM, structural map/state, toolkit or historical evidence was changed or removed.
No new toolkit issue; existing confirmed NES-DOC-001 remains worked around/unblocked.
Disk3.0GiB free; commentary47MiB, durable commentary-round1 evidence1.3MiB; no cleanup.
