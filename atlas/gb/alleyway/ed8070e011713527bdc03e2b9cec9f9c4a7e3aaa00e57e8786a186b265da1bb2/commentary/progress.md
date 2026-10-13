# Commentary progress

## Final batches 011–012 — audio and editorial acceptance

**Phase 2 complete:** all 161 scaffold entries and 1077 units are reviewed;
32768 bytes reviewed, 0 pending. Final human editorial findings are in
editorial-audit.md. The required closing `commentary.py audit /root/alleyway`
passed exit 0: ok=true, byte_identical=true, reviewed_regions=161,
reviewed_units=1077, editorial_audit=true. It freshly rebuilt without RGBFIX
changes and refreshed reading-after.asm. Results: audit.json and verification.json.
Source/ and its local assets remain the maintained buildable deliverable.

Completed audio: pulse1 effects, pulse2/wave music and loss noise. Earlier
planning's three-channel music assumption is superseded. Reviewed priority,
request locks, inherited phase counters, timed presets, byte-wrapping sweeps,
global routing, cue initialization, stream pointers/countdowns and token decode.
Documented 32 five-register presets, four noise registers, parallel 67-byte
frequency arrays, 44 durations, 24 valid bounded streams and 32 waveform samples.
Duration prefixes, rest, end and restart are explicit; requests 8..12 retain
restart ID 6. Tonal mute does not mute noise. Pulse music end writes NR12 rather
than NR22; its actual cross-channel effect is preserved and investigated.

Legacy key poll/selectors alias the installed DMA image at FF80/FF81 and have
no established callers. Their effects, write-only flags, unused preset rows,
shutdown intent and earlier material limits are reviewed in uncertainties.md.
Each has observable behavior, alternatives, methods and concrete reopening
evidence. No useful local discriminator remains outstanding. No new runtime
scenario, snapshot persistence, listening or execution-coverage claim was added.

Whole-source editorial checks found no generated code/branch names, numeric
direct control transfers or uninventoried labels. Added vector-fill anchors,
restart-slot units and paddle tile-table alias; compact region summaries cover
all mixed algorithms. All 1077 starts match linker symbols; source pointers name
exact labels; every code byte has unit coverage; unit ends match instruction/
region boundaries and ranges lie in ROM. Corrected one multiply-loop end to
046d. Corrected the auxiliary scanner's JR Z width: 4845 starts, including all
4488 distinct older captured starts plus 357 later-classified starts. Earlier
phase-2 4883 reports are superseded; no-reference conclusions/ROM bytes are
unchanged. This was an analysis-helper correction, not a toolkit defect.

Final ROM SHA-256:
ed8070e011713527bdc03e2b9cec9f9c4a7e3aaa00e57e8786a186b265da1bb2.
Final source hashes are recorded in audit.json/verification.json. All 161
scaffold identity/evidence records match frozen state; ROM and frozen/current
phase-1 map/state hashes match manifest; reading-before matches before/.
Graphics assets match ROM slices. Notices and local artifacts are preserved.
Toolkit unchanged: T001 is one existing nonblocking missing feature; no new
toolkit issue (../analysis/toolkit-issues.md).

This completes the commentary contract, not phase-1 classification: 31 Unknown
bytes at 0426-0445 remain unchanged. Per-bank kind totals and fixed CPU aliases
are in editorial-audit.md. No full-game execution/full hardware compatibility
claim is made. Preserve source/ and assets: stock .sym alone omits the commentary.

## Batches 008–010 — bonus results, UI, metasprites and graphics

Current inventory: 120 reviewed / 41 pending regions, 572 reviewed units.
Reviewed scaffold coverage is 28033 bytes; pending coverage is 4735 bytes,
exactly partitioning 32768. Bank0 is fully editorially reviewed, including its
preserved 31-byte phase-1 unknown. Remaining scaffold entries are precisely
the audio engine/tables at physical/CPU $6800-$7a7e in fixed bank1. A tonal-mute
helper there has a reviewed unit, while its larger audio region remains pending.
Whole-source editorial audit is still pending; this is not goal completion.

Reviewed bonus timer/results/point transfer and parameter records. Timer ticks
are foreground samples of frameCounter&31==0 (normally32 frames), not seconds.
Records are byte ticks plus big-endian award:95/500,90/700,85/1000,80/1500;
unsigned round-1 clamps to0..3. Bonus success animates ten-point transfers then
single points. Its score addition wraps at16 bits, unlike saturating brick
points. Decoded glyph pixels establish SPECIAL BONUS/PTS. and TRY AGAIN! text.
Copy helpers, loops, parameters and every command/sentinel have source records.

Completed bank1 entrance/exit, paddle pose transitions, loss/cycle animation,
window/HUD setup, captions, decimal sprite displays, left border and four-sprite
composer. Named anonymous entries and all meaningful branches/shared tails.
Documented 24 signed jump deltas, bounded frame sequences, selector map plus
three tile triples, and thirteen big-endian pointers to four relative sprite
records. Current/hi-score displays use four decimal digits plus a categorical
ten-thousands glyph; bonus-pool display uses four digits. Descriptive request
veneers cover effect/noise/music-cue mailboxes with last-write-wins semantics.
The fixed mute trampoline clears tonal registers/flags, not the noise channel.

Corrected an earlier call interpretation: all current speed-display calls land
on RET $4a29. The adjacent $4a2a tier-glyph writer is detached. Advancement
does not redraw it. Actual paths and competing original intentions are reviewed
in uncertainties.md U-SPEED-DISPLAY; no bug/fix was asserted. Also retained the
literal discarded scanline-cursor read in playfield setup, which does not clear
that cursor. Compact parent summaries now point to detailed unit records rather
than repeating the entire unit glossary.

Reviewed all remaining pre-audio data: twelve LE scroll-pattern pointers,
sixteen20-byte direction/period patterns, title/cycle/HUD command streams,
unreferenced grids, and full graphics blocks. All command lists parse to their
exact scaffold bounds/sentinel with mode/count/payload documented. The40-row
cross-bank grid is completely empty; the12-row grid has one type1 brick at
zero-based row11,column10. All33 descriptors and a fresh full classified-code
length/operand scan (4883 legal starts, no region-boundary crossings) have no
incoming sixteen-bit operand/descriptor into either grid. Original purpose
remains (?) with concrete reopening evidence in U-UNREFERENCED-GRIDS. No forced
execution or new gameplay route was used to manufacture a purpose.

Visually inspected all three local PNG atlases and checked each 2048-byte 2bpp
asset against its exact ROM slice. Documented128-tile blocks, scanline bitplanes,
blank slots (89/55/39), Mario/paddle/ball/font/title artwork and hardware ID
aliases. The8800 block is shared by signed BG IDs80..FF and high sprite IDs;
9000 serves positive signed BG tiles and is not a sprite fetch block. Assets
and upstream notices remain local and unchanged. Added concrete FF-fill and
fixed trampoline records without reclassifying phase-1 evidence.

Final verify exit0, byte_identical=true; read exit0 and reading-after.asm
refreshed. Original ROM SHA-256 remains
ed8070e011713527bdc03e2b9cec9f9c4a7e3aaa00e57e8786a186b265da1bb2.
Final bank_000.asm SHA-256 is
58ca14bfe0b8ee1cb6484fc9dc1008d120f07a413e79ca69262d80aefa292e08;
bank_001.asm is
10b35ce80477664c9e5ad634c9ce47ab4e968af4c26efedb2e4bff6733eb0934.
All572 unit starts match linker symbols and every source pointer matches its
named label. No invalid unit range, scaffold/evidence change or frozen/current
phase-1 map/state/ROM hash change was found. reading-before matches before/.
Scoped generic-label scan finds none in reviewed Code; the pending audio code
still needs semantic naming. No new runtime experiment/input sequence or
execution-coverage claim. Literal behavior/format confidence is high; original
intent limits remain marked (?). Existing toolkit T001 is nonblocking; no new
issue encountered (../analysis/toolkit-issues.md).

Next: complete audio UpdateAudio, dormant entry dispatchers, effects/noise,
three-channel music parsing, frequency/tempo tables and all cue streams/waveform.
Pay attention to $ff81 aliases inside the installed DMA image in detached code;
do not invent a normal input-register role. Then perform whole-source editorial
review, refresh all unit/source links and run final audit/verification/reading.

## Batches 006–007 — raster scrolling, ball physics and paddle

Current inventory: 82 reviewed / 79 pending regions, 367 reviewed units.
Reviewed scaffold coverage is 15504 bytes; pending coverage is 17264 bytes,
partitioning all 32768 bytes. Bank-0 code through the paddle/ball sprite writers
is reviewed. Bonus results/presentation, remaining bank-1 UI/data, graphics and
audio remain unfinished; the whole-source editorial audit is still pending.

Completed close-to-source commentary, descriptive symbols, shared-tail/branch
records and RAM/HRAM fields for paired-cell tile queues, scrolling timers,
STAT raster bands, vertical-window discard, points/high-score updates, paddle
and wall contact, brick probes/removal, signed 8.8 integration/reflection and
speed/angle selection. Normal destroyed and solid cells report reflection;
bonus destruction removes cells while reporting no reflection. HP zero is a
noncounted solid; clearing an ID makes its retained HP irrelevant. Plane bounds,
row/column units, repeated tiles, record endian order and flag/counter semantics
are documented. Confidence is high in these literal effects and consumers;
original intent of the detached/asymmetric tails remains tentative.

Reviewed all 25 velocity tiers (19 four-byte records each), replacing raw bytes
with named tables and a byte-preserving big-endian BallVelocity macro. Pointer
table remains little endian. Magnitudes span 1..4 pixels per physics update in
1/8-pixel tier steps; consumers apply quadrant signs. Reroll selection consumes
AdvanceSequenceByte's returned A, broadening its established use beyond demo
stage choice. Reviewed the bounded eight-byte candidate set and documented the
unused compatible tail in uncertainties.md U-ANGLE-TAIL.

Reviewed incoming serve, serial/joypad paddle movement, rebound tables, sprite
composition and hit thresholds for vertical scrolling. Serve velocity uses
tier 1 direction 9, independent of the selected future tier 3/7; first paddle
rebound loads the latter. A/B select 5/1-pixel paddle steps, default 3; A and
Left respectively win simultaneous A/B and Left/Right. Narrow paddle uses three
overlapping sprites, not two. Shared clamp bounds differ by one pixel between
ordinary input and width-reset/demo helpers, and width-reset shifts X by four
even if already normal. Added normal/narrow impact tables and ten scroll-hit
thresholds. Corrected the prior ball-loss comment: FFC5=2 resets threshold
index, not a relaunch delay. Documented the unguarded ten-row reveal loop's
byte-cursor wrap for short layouts, preserving original behavior.

Investigated U-COLLISION-TAIL: trailing-Y probes jump to an empty RET, while
the nearby opposite-direction Y alignment helper is detached. Static callers,
counterpart X code and exact effects were inspected. Runtime could show the
already-known path but cannot discriminate original disabled-correction versus
historical-target intent; no unsupported bug claim/fix was made. This and the
unused angle tail are explicitly marked (?) and have concrete reopening evidence
in uncertainties.md. No new runtime experiment/input sequence or execution
coverage claim: this batch is static source/consumer review.

Final verify exit 0, byte_identical=true; read exit 0 and reading-after.asm
refreshed. ROM SHA-256 remains
ed8070e011713527bdc03e2b9cec9f9c4a7e3aaa00e57e8786a186b265da1bb2.
Final bank_000.asm SHA-256 is
19eab4fea82e5726f67d395517cb3667144dfe7ee00f60df3aba677cf96259f6.
All 367 unit starts match linker symbols and all source pointers match named
labels. All 161 scaffold identity/evidence records, original ROM and frozen/
current phase-1 map/state hashes are preserved; reading-before matches before/.
A scoped generic-label scan found none in reviewed code; this is not the final
whole-source editorial audit. No new toolkit issue; existing T001 is a
nonblocking missing feature (../analysis/toolkit-issues.md).

Next source group: bonus timer/results/presentation at $198c-$1b41 and its
parameter/command data, then the unreferenced cross-bank grid and bank-1
scroll patterns/UI/graphics/audio. Keep the 31-byte phase-1 unknown unchanged.

## Batches 004–005 — state graph and consumer-backed stage data

Current inventory: 70 reviewed / 91 pending regions, 230 reviewed units.
Reviewed scaffold coverage is 11176 bytes; remaining scaffold coverage is
21592 bytes, exactly partitioning the original 32768-byte ROM. Partial reviewed
units also exist in pending code regions; these counts do not imply that their
whole scaffold entries have been accepted.

Reviewed DispatchGameState, the 16-entry pointer table, startup/title/demo,
stage entry/scroll flags, launch, play/pause, loss, completion and cycle
celebration flow. Replaced pointer bytes with byte-identical DW labels, added
state constants, descriptive branch/shared-entry names and HRAM/RAM field
symbols. Added partial reviewed game-over/resume and audio-request gate units
without marking the larger physics/audio regions complete. Reserve-ball count
excludes the current ball. Title/demo timers count frame-counter wraps, not
fixed seconds. Audio suppression clears new request IDs, not existing channels.

Correction to batch 003: the add-65 helper's returned A is consumed by the demo
stage chooser even though there is no independent load of its stored byte.
Renamed it AdvanceSequenceByte/hSequenceByte, documented the return value and
updated the inventory summary. Demo selects A & 31 and rejects bonus layouts.
This is deterministic variation, not a source of independent entropy.

Read the stage/HP loader, paired-row rendering, horizontal/vertical scrolling,
destructible counting, score/high-score and extra-ball consumers. Applied source
comments/names and detailed unit records for LoadStageGrid, destructible count
and threshold award/loading. Remaining scrolling/rendering/physics helpers in
region-32 are still pending for their complete source/unit editorial treatment.

Reviewed and reformatted all 24 referenced layouts into 14-cell rows with a
single FF sentinel. Cells are unsigned: 0 empty, 1..3 destructible, 4 solid with
zero completion HP; rows represent four pixels. Documented the 15 six-byte
brick records (occupancy tile IDs, packed points/HP, minimum speed tier, audio
category), including unused default slots. Rewrote all 33 stage records as
flags plus named little-endian layout pointers; ordinary index range is 0..31,
and the final repeat record has no identified state route. Extra-ball thresholds
are ten big-endian words (1000..9000, FFFF), awarded on strict score exceedance
with reserve count capped at nine. The unreferenced cross-bank grid remains
pending, along with the remaining sprite/graphics/audio structures.

Static observations for the next batch: cell and HP planes at C000/C400 each
hold 840 cells (60x14); current layouts have at most 40 rows. Two logical rows
form one tile row; queue builders duplicate 14 tiles horizontally for 112-pixel
wrap. Twenty scrolling-row offsets/patterns/countdowns are at CA00/CA14/CA28.
STAT service schedules four-line bands at LYC=4*next+7, restoring border SCY/SCX
after 20 bands. Vertical scrolling discards the row below the new visible window
and decrements only destructible cells. These findings still need their full
close-to-source commentary and individual shared-tail/branch review records.

Final checks after both edit groups: verify exit 0, byte_identical=true; read
exit 0, reading-after.asm refreshed. ROM SHA-256 remains
ed8070e011713527bdc03e2b9cec9f9c4a7e3aaa00e57e8786a186b265da1bb2.
Final bank_000.asm SHA-256 is
fc2bcbee180893b2a33f1d1d4f5c95f4c1d7c4aac2214d9ce1dda7820fed3c83.
All 230 unit starts match linker symbols and all source pointers match their
named labels. All 161 scaffold identity/evidence records match frozen state.
ROM, frozen/current phase-1 state and map hashes are preserved; reading-before
still matches before/. No runtime experiment or new execution claim was added.
Whole-source editorial audit remains pending. No new toolkit issue encountered;
T001 remains a nonblocking missing feature in ../analysis/toolkit-issues.md.

Prepared verified baseline. All semantic review entries are pending.

## Batch 001 — startup and frame/transfer infrastructure

Read the phase-2 contract and existing phase-1 records. Began source review at
physical $0000-$0301 in bank 0: vectors/header, reset memory bounds, interrupt
preservation, VBlank completion latch, LCD shadow synchronization, serial byte
exchange and the four VRAM command modes. Added functional comments in
source/bank_000.asm. In particular, retained the literal reset clear bounds and
documented the nonstandard big-endian VRAM destination and zero-count wrapping.

Inventory remains honest: 0 reviewed / 161 pending. This batch's source reading
is provisional until callers/consumers, descriptive symbols and corresponding
unit records are completed. Serial game meaning and the bank-1 frame service
remain dependencies for later review. No phase-1 artifacts or map were edited.
Cross-references establish that $7ff0 is VBlankAudioTrampoline -> UpdateAudio;
replaced the numeric call with that existing symbol. MovePaddle reads serial
sample $ff91; multiple state handlers test bit 7 of the derived $ff93 falling-edge
mask. The external serial device/protocol interpretation remains tentative.

`commentary.py verify /root/alleyway` passed with byte_identical=true and ROM
SHA-256 ed8070e011713527bdc03e2b9cec9f9c4a7e3aaa00e57e8786a186b265da1bb2.
`commentary.py read /root/alleyway` passed and refreshed reading-after.asm.
Both commands were repeated after the symbolic audio call edit (results below).
Final repeat: verify exit 0, byte_identical=true, bank_000.asm source SHA-256
6e676c0a08edf3074a1d3ed59612fb163c18ab3bf341302bd15f71a482fa6e6e;
read exit 0, reading-after.asm refreshed. Whole-source editorial audit remains
pending; this bounded batch does not establish phase-2 completion.
No new toolkit issue was encountered; existing T001 (missing annotation metadata
support, nonblocking) remains documented in ../analysis/toolkit-issues.md.


## Batches 002–003 — infrastructure inventory, input and arithmetic

Replaced numeric infrastructure labels/references with descriptive frame, LCD,
VRAM, memory-clear, DMA, input-filter, decimal and multiply symbols. Added
game_constants.inc for shared HRAM state and updated both banks' references.
Reviewed the actual two-mode detached rectangle grammar, graphics copy blocks,
OAM clear alternate entry and DMA image. Added routine/loop/shared-tail records
without changing the 161-entry full-ROM scaffold.

ReadJoypad produces active-low held/new-press masks plus an active-high press
latch. Numeric sprite builders consume the decimal converter's five unpacked
HRAM digits. The add-65 sequence has no other read consumer in the ROM, so
comments explicitly avoid treating it as game randomness. Start-held plus
Select-new-press requests title restart. Serial samples supply a position and
button-edge interface; accessory identity remains marked (?) and investigated
in uncertainties.md. The 31-byte zero interval is editorially reviewed using the
preserved phase-1 investigations, without reclassifying its unknown kind.

Current inventory: 37 reviewed / 124 pending; 100 reviewed units. Whole-source
editorial audit remains pending. Final verification/read for this batch follows
below. Previous verify/read runs after both applied edit groups passed exit 0,
byte_identical=true; ROM SHA-256 remains
ed8070e011713527bdc03e2b9cec9f9c4a7e3aaa00e57e8786a186b265da1bb2.
Frozen baseline/source notices and all phase-1 maps/checkpoints remain intact.
No new toolkit issue encountered; T001 remains a nonblocking missing feature.

Final batch verification: verify exit 0, byte_identical=true; read exit 0 and
reading-after.asm refreshed. Verification is saved in verification.json.
bank_000.asm SHA-256 is
4af5b3009e4dec9ed62cf8c101198867aca8a1f7566a139849761e6806de5f61.
Added individual cartridge-header field records and compacted erased restart
slots into DS directives. Linker symbol export checked all 100 unit starts and
found no mismatch after correcting STAT to $0260 and serial to $0272. Every
unit source pointer resolves to its exact named label. Compared the scaffold's
id/offset/length/kind/phase1 fields against frozen state: all 161 preserved.
ROM, frozen/current phase-1 map and state SHA-256 values match the manifest;
reading-before.asm still exactly matches expansion of before/.

Next review begins at DispatchGameState ($053c), its pointer table ($054c),
and the title/demo/stage/launch/pause state graph. No whole-source audit has yet
been claimed or recorded as reviewed.
