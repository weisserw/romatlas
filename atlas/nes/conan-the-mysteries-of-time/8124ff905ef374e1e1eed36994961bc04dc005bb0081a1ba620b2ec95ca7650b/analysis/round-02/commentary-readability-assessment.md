# Round 2 current-source readability assessment

The complete current classified source is readable: 731 code regions (26,849 bytes,
12,426 instruction starts), 175 data regions (104,238 bytes including header), plus
an explicit unresolved note for physical $1CAF1. No classified readability ranges
remain pending. This is separate from whole-ROM classification and final acceptance.

The entire prior commentary tree is preserved under prior-commentary/. The new
baseline was prepared from the frozen round-02 map; 639 unchanged classified regions
and 223 functional contracts carried useful review credit by physical identity.
The expanded source has 361 functional contracts with 841 exact code-fragment
attachments. All 907 review scaffold entries retain their physical identity/bounds
and have source locations and useful summaries. These counts inventory coverage;
they do not substitute for the prose assessment below.

New explanations cover the connected path/phase encounter, three synchronized
channels, carried-player behavior, linked object lifetimes, randomized item slots,
cell substitutions, collectible windows and stage transitions. Actor/action identities
remain conservative: RunPlayerCarryEncounter, RunLinkedActionE8 and
RunAction7BChild explain established mechanics and retain numeric actions, without
inventing enemy names. Local aliases identify path cursors, channel selectors,
readiness flags, linked slots, composite phase and record cursors; shared scratch
and object-field reuse remains explicit. New branch destinations use their owning
routine and established field/helper role. Existing useful routine/block explanations
and coherent hardware/array symbols were preserved.

Fixed API additions explain inline argument ownership and stack adjustment, bounded
rectangle decompression with runs continuing across rows, strict image-rectangle
intersection, bottom-left tile-image rendering, image-cache retry by rewinding five
bytes, task clearing, decimal conversion and music task lifecycle. The ground-status
polarity was checked: one means falling, zero support. TestSupportAcrossObject
restores temporary X offsets but the underlying probe can snap Y; its return is one
if any probe reports falling. GetPlayerActionDamage has no local bounds clamp.

Data explanations distinguish mapper selections and effective physical banks,
rectangular layouts, mutable-ID translation, 26-byte metatiles (25 tiles plus palette),
counted CHR versus implicit-1024-output screens, raw eight-tile CHR cache groups,
five-byte animation records versus sprite images and nametable images, music
note/duration pairs, palette terminators, stage spawn records and DMC descriptors.
Generic uniform storage retains honest purpose uncertainty. Older uniform fragments
inside newly supported containers receive their parent format without changing the
historical structural scaffold. Consumer/reference and format evidence was reused
from the connected structural audits, with no new asset guessing or runtime claims.

Changed evidence invalidated prior assertions about unknown intervening bodies,
unclassified table extents and the spawn-record backing. Those assertions were
replaced in source, shared-symbol notes and uncertainties.md. Decoder explanations
now consistently assign $01..$3F repeat, $40..$7F zero, $81..$FF literal and $00/$80
skip. The $40 do-while edge emits 128 zeroes unless output is exhausted; none of
6554 saved token records contains it, so accepted bounds are unaffected. Stored
unread literal suffixes and second $FF music markers are distinguished from reads.
NMI $CAB7 reentry skip/$CABA callback/$CABD default-body distinction is preserved.
CPU $CE9F/$CEA0 is explicitly the inline $05A7 RAM-image pointer; $CEA1 resumes code.

Remaining uncertainty is localized in source/uncertainties.md: exact game identities,
score units, natural selector use, fourth zero decimal scale, generic fill purpose
and the isolated unknown byte. No source research was cut short or left pending.
The final editorial audit is deliberately pending for the coordinator's targeted
acceptance gate, not invoked as a proxy for prose quality.

Fresh final verify passed ok=true, byte_identical=true. Reading copy refreshed.
The original ROM hash is unchanged, state matches phase1-state, map matches baseline
and round-02 checkpoint, and map validation succeeds. Full hashes are in
../round-02-commentary-verification.json. No ROM/toolkit edits, structural mutation,
cleanup, deletion, publication, archive or commit occurred. Helper scripts and all
source/assets remain local. A duplicate new layout-fragment symbol was found during
the first data build, corrected locally and followed by successful fresh rebuilds.
