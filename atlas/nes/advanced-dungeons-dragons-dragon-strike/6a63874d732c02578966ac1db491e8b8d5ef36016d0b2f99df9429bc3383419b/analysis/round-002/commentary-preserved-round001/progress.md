# Commentary progress

Current-source readability is complete. The initial preparation and interrupted checkpoints below are preserved as history.

Latest checkpoint (cont04): all 26,229 Code bytes / 12,994 instruction records reviewed; no pending code ranges. All 13,488 scaffold records and 399 units are reviewed. Data/CHR/unknown-boundary review credit is preserved. Final editorial audit remains pending. See the completion entry at the end and analysis/handoffs/round-001-commentary-complete.md.

ROM and map/state identities are unchanged. This readability gate does not establish whole-ROM classification; 126,852 bytes remain structurally Unknown.

## Round 001 initial checkpoint (historical)

Current-source readability remains **in progress**, not ready for redirect. Baseline
unchanged: maps/current.json and analysis/state.json SHA256 c9f7a2f627454c733c0096ca352eab3da6fbf744c06f6c3153d106c2cfd4846e.

Reviewed code physical ranges (exclusive ends): 20..29, 15442..16130,
16404..16413, 31335..32023, 261421..261991. This is 1,964 code bytes in
888 instruction scaffold records. Both bank-pair audio drivers and APIs have
functional routine/branch labels, sound RAM arrays, CPU table aliases and comments
covering timed events, commands, presets, pitch and effect installation. The fixed
bank system tail covers stable controller sampling and press edges, selected input,
frame synchronization, PPU shadows, NMI dispatch/OAM/audio, queued PPU transfers,
and warm/cold reset initialization. Meaningful units are recorded in review.json.

All 134 Data records, the CHR Image record and 359 Unknown intervals now have local
source boundary/role/uncertainty notes. These reuse sound-data-review.json,
uniform-data-reference-review.json and prior exact-boundary inspections; generic
uniform payloads do not assert specific formats. Unknown intervals stay undecoded.
Header, bank identity records, vector words and CHR storage are explained.

Exact remaining code frontier is every Code entry with status=pending in review.json:
12,106 records / 24,265 bytes. All code outside the five physical ranges above
remains pending, including stage banks4..19, menus26..29, bank30, and bank31 before
261421. Shared hardware aliases have been introduced but this does not award code
review elsewhere. Next useful review: mapper/dispatch helpers around fixed bank
$F200..$F360, then scene initialization/transition and earlier fixed-bank routines.

Fresh verification after system batch: commentary/verification.json,
byte_identical=true, source/game.asm SHA256
afa10cfcd9a1e45968469fc578417789c87016dee0fa5b64ca4300846c40ab33.
Final editorial audit remains pending. Reading copy still needs refresh at next
checkpoint/handoff. No toolkit changes; existing NES-001 remains the only issue.

### Interruption checkpoint after connected fixed-bank batches

This worker is **interrupted by context scope**, not finished with commentary.
Continue the same phase; do not start redirect. Supersedes the earlier frontier:
reviewed code2,811 bytes /1,271 records; pending code23,418 bytes /11,723 records.
Exact pending complement is analysis/round-001/commentary-frontier.json (278
contiguous physical ranges) and pending Code entries in review.json.

New reviewed ranges: 258546..258618,258661..258680,258705..258938,
259129..259145,259145..259521,259523..259654. Together with the earlier five ranges,
these cover word arithmetic, PRG/CHR selection contracts, audio bank preservation
and effect priorities, queue drain, scene initialization/frame pacing, success
transition stack/carry contract, pause and indirect completion dispatch. Reused RAM
names now reflect the observed E6 priority/index role. Corrected the initial sound
carry comment: the second ASL establishes carry (clear for track values below64);
it is not an incoming caller carry precondition. Added command and Start constants.

Remaining by physical PRG bank in bytes:4=1387,6=794,8=758,10=933,12=830,
13=148,14=2182,16=1250,18=1195,26=676,27=310,28=1827,29=2048,30=3335,31=5745.
Next continue bank31 connected helpers: $F636 completion handlers, $F715/$F77D/
$F78D, $FA98/$FBEF/$FC5F/$FCCE and earlier fixed-bank state/actor/PPU consumers;
then bank30 and stage/menu bank units. Inspect consumers to refine mechanical RAM
aliases instead of guessing game meanings. The fixed system/scene source already
names unresolved outgoing contracts, so those are concrete dependency frontiers.

Fresh final verify/read both passed. Source game.asm SHA256
ecd59d5151001b85fd28444a1409b7f5a3e1b188499ac1db53f1fc30d7eebb44.
commentary/verification.json is current and reading-after.asm refreshed. ROM/map/state
hashes unchanged. Source locations in reviewed entries and units refreshed.
Final editorial audit remains pending. No new toolkit issues; existing NES-001 only.

### cont02 durable checkpoint: fixed-bank rendering and record mechanics

Current-source readability remains incomplete; continue this same commentary phase.
Cumulative reviewed code: 5,725 bytes / 2,745 records. Pending: 20,504 bytes /
10,249 records, with exact 265-range complement in
analysis/round-001/commentary-frontier.json. Reviewed an additional 2,914 code bytes
without changing any classification, unknown boundary, map or ROM payload.

Named and explained completion carry predicates and guarded effects; ceiling-division
and three-sprite meter construction; OAM clearing; queue reservation and address
headers; direct nametable fill; render-record sorting; row-band sprite omission
rotation; palette block copies, queued transfers and fades; NMI CHR register/phase
dispatch; two-nametable 4x4-metatile rendering and attribute nibble merging; tile
coordinate/class queries; five-byte scene-record list scanning, clipping, timers,
action preparation and fixed/banked render dispatch. Unresolved helper meanings
remain explicit beside their calls. The fixed renderer at $E681 and action consumers
$DA10/$EB0D are still pending, as are the earlier bank30 and stage/menu backlogs.

Corrected concrete prior defects: literal operands had acquired unrelated RAM names
($00/$02/$03/$04 and audio command $E2); restored numeric literals and the actual
AUDIO_CMD_PARAMETER constant. Source operand constant sampling now shows only
intentional AUDIO_* and BUTTON_START symbols. Corrected sort direction to descending
and cache-predicate wording to distinguish unchanged view from clear-cache-on-movement.

Fresh verify passed byte-identical and reading-after.asm refreshed. Source SHA256
c4ddb789f896378a742a6ed8ddbe07581721782d2b6b611e07e31c1eb6144279.
Pending by physical PRG bank: 4=1387,6=794,8=758,10=933,12=830,13=148,
14=2182,16=1250,18=1195,26=676,27=310,28=1827,29=2048,30=3335,31=2831.
Final editorial audit remains pending; no toolkit edits or new toolkit issue.

### cont02 milestone: classified fixed31 code complete

Cumulative reviewed code is now 8,556 bytes / 4,221 records. Pending code is
17,673 bytes / 8,773 records across 259 exact ranges in commentary-frontier.json.
All currently classified physical-bank31 code is reviewed; unknown intervals remain
unknown. The additional units cover rectangular background patches, alternating
NMI row transfers and scroll limits, eleven-byte object allocation/compaction,
rectangle contact dispatch, lifetime/phase updates and movement script commands
$80..$83 with six-bit fractional stepping. This milestone is not phase completion.

Reconciled uncertainties.md: the earlier opaque PPU/CHR/palette helpers have
mechanical explanations; unresolved identities, bank-local APIs and fixed30
consumers remain. Documented exact nonstandard carry/category propagation rather
than silently normalizing it. Checked direction wrappers against the already
reviewed add/subtract helpers and corrected the wrapper names to their actual signs.

Fresh byte-identical verification and reading refresh passed at source SHA256
5b5cd11c99481379be2d8d1088fd79c5922d2108d21e8bcb412f0bf96a41a8f2.
Next: physical-bank30 pending3335 bytes, followed by all stage/menu pending code.
Final audit remains pending; no toolkit/ROM/map changes.

### cont02 interruption: direction and meter dependencies

This worker is interrupted by context scope; commentary is **not complete**.
Cumulative reviewed code9,140 bytes /4,520 records; pending17,089 bytes /8,474
records. Exact pending complement is256 contiguous ranges in
analysis/round-001/commentary-frontier.json and the pending Code scaffold records.
All classified bank31 is reviewed. Bank30 has2,751 pending bytes; other pending
bank totals are unchanged from the preceding checkpoint.

The latest connected bank30 batch explains the destructive twelve-direction point
quantizer, absolute vector/quadrant selection, restoring byte-quotient division,
scene meter subtraction/depletion and clamped addition, and bank-dependent scene
reference tile sampling. Some helpers referenced by classified source remain in
unknown intervals ($D880 for example); their meanings remain unresolved without
classifying those bytes. Current uncertainty notes replace the stale opaque list.

Fresh verify/read passed at source SHA256
f184d62efe5bbf7ee0120fa3caef8d90a611c090511158d60b2f2ecca4c96c98.
No final audit run. Continue same phase from fixed30 remaining ranges, then all
stage/menu pending code. Unique continuation handoff:
analysis/handoffs/round-001-commentary-cont02.md. Original handoff preserved.

### cont03 continuation checkpoint and interruption

The same commentary phase remains incomplete: cumulative 21,368/26,229 classified
code bytes reviewed (10,664 instruction records), with 4,861 bytes /2,330 records
pending in 64 exact ranges. All currently classified code in physical banks 4,6,8,
10,12,13,14,16,18,30,31 is now reviewed, in addition to the prior sound audio work.
Only banks 26=676,27=310,28=1,827,29=2,048 bytes remain pending. The full pending
complement is analysis/round-001/commentary-frontier.json and review.json's pending
Code records. All Data/CHR/unknown-boundary review credit remains preserved.

Added scene-reference status/input/orientation controls, feedback palette updates,
signed spawn offsets, six-bit fractional motion and bounded view following; five-byte
render records, composite sprite descriptors, clipping/attribute processing and the
61-sprite global budget; scene marker contact effects; bank resource setup, actor
update gates/distance/tile sampling; scene-specific special actor dispatch/contacts,
wave/multipart actor setup, direction-based typed spawns and recorded-input mechanics.

Shared stage helper mechanics were carried only after same-CPU-position instruction
byte comparisons with reviewed bank4. Bank-local labels, effective pairs and unknown
callee limitations remain explicit; equal address bytes do not establish callee
semantics. Evidence: commentary-cont03-shared-evidence.json. Bank14's two list-builder
payloads also match bank4 exactly, while their unknown middle boundaries remain unchanged.
Restored 226 provenance headers after detecting that source-comment carry inherited
bank4 provenance; frozen baseline provenance is now reflected in source throughout.
Corrected multiplier prose: object descriptor bit7 selects twelve times the low nibble;
bank14's reviewed scaling path computes rounded three quarters of the incoming byte.

Fresh byte-identical verify passed and reading-after.asm refreshed at source SHA256
8e4fad73bf4dea657880ef8bad356743148430d7050e97ebe8f9555d9b93c51f. No final audit, redirect, archive, cleanup,
deletion, toolkit/ROM/map changes or publication. Context scope interrupts this worker;
resume the SAME phase from the remaining bank26..29 ranges. Unique handoff:
analysis/handoffs/round-001-commentary-cont03.md. One-shot cont03 scripts are preserved
in analysis/round-001/ and must not be rerun.


### cont04 current-source readability completion

Completed the remaining 4,861 Code bytes / 2,330 records in physical banks26..29,
adding 102 meaningful units while preserving all earlier work. The exact frontier
now records zero pending bytes/records/ranges. All 26,229 currently classified Code
bytes (12,994 records), all 134 Data records, the CHR Image record and all 359 Unknown
intervals have reviewed explanations. All 13,488 scaffold records and 399 units are
reviewed; editorial_audit stays pending for terminal acceptance. Classification is
unchanged: Code26,229 Data109,079 Image262,144 Unknown126,852 (75.81% classified).

Menu source now names/explains cursor row selection and OAM strips, one-color PPU
updates, palette stepping, player-count/password menu flags, sprite-zero CHR splits,
four/twelve-direction matchers and carry contracts, direct PPU RLE image loads,
scene/reference/actor setup, overview markers and password entry/validation. The
password codec explains its 24 packed bits, position-dependent alphabet conversion,
XOR check/mode trials, bounds and destructive work buffer. Unknown continuations
remain undecoded and are stated at each interrupted control-flow boundary.

Concrete connected corrections: PrepareRendering no longer calls its reviewed
callees unresolved; scene palette writes use the destination pointer left by
CopyPaletteBlockToSlot instead of assuming a caller-installed scene-handler pointer;
menu coordinate-high/sort-key scratch is distinct from actor-contact scratch; scene
placement, blank-letter counts, password checksum and range bits use contextual
names. Removed three new unused equates whose bank27 prefixes obscured bank26
cursor targets. Shared bodies were inspected independently here; callee semantics
were not transferred merely from identical CPU call addresses. Special-key cursor
stores are separated from letter stores by unknown three-byte tails; no accumulator
value or control flow is assumed across those gaps. Prior stage/audio
review credit was reused without a mandatory whole-source reread.

Fresh final byte-identical verify passed, followed by reading-after.asm refresh.
Source SHA256 7c2412a800002f18dbeae885737e0aefad239220b4e2624dd47a8c43651b2130.
Input ROM hash6a63874d732c02578966ac1db491e8b8d5ef36016d0b2f99df9429bc3383419b;
map/state both c9f7a2f627454c733c0096ca352eab3da6fbf744c06f6c3153d106c2cfd4846e.
No final audit, redirect execution, archive, cleanup, deletion, publication or
ROM/toolkit/map mutation. One-shot cont04 edit scripts are local historical records
and must not be rerun. commentary-cont04-checkpoint.py is a non-source-mutating
frontier/source-location refresher. Complete handoff: analysis/handoffs/round-001-commentary-complete.md.
