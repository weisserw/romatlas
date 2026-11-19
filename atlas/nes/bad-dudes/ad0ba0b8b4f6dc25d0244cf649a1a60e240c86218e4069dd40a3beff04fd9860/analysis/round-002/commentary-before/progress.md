# Commentary progress — round 001, active

Prepared with supported commentary.py; baseline rebuild byte-identical. Current
source is game.asm plus symbols.inc. Frozen before/ remains untouched.

Reviewed classified Code physical [16,25), [36,111), [303,343), [348,447),
[455,980), and header [0,16). Sound API slots, initialization, compatibility
bitsets, scheduler, channel ownership and primary/overlay loader now have names,
branch labels and functional comments. Shared sound-state names apply only in
physical PRG bank 0; zero page in other banks retains numeric aliases. Hardware
register symbols apply across source. Later bank-zero helpers already use sound
RAM symbols but are NOT yet reviewed.

Exact pending scaffold entries are in ../analysis/round-001-commentary-remaining.json.
Remaining current-source review: all scaffold regions with status=pending,
including every unknown gap, all classified Code from physical 980 onward,
all non-header Data and all CHR Image. These ranges must be read/commented before
the gate can pass. Do not regenerate source or prepare a new baseline.

Batch verification: commentary.py verify passed byte-identical, original SHA256
ad0ba0b8b4f6dc25d0244cf649a1a60e240c86218e4069dd40a3beff04fd9860.
This is active work, not a successful commentary handoff. Editorial audit pending.

## Sound-driver and non-code batch

All current classified bank-zero sound code through physical2507 is now reviewed:
stream formats, command dispatch, envelope and period output, nested and single
repeat forms, one-level stream call/return, nonlocal stack exits and channel reset.
Every current Data/Image entry has a role/boundary explanation. Uniform-run
comments reuse the prior inspected-adjacent-boundary evidence; selector-related
bytes retain a deliberately limited prefix interpretation. CHR layout and hardware
vectors are explained. Every unknown gap now states its physical half-open range,
starting PRG bank and unresolved role; raw inclusion makes no classification claim.

Current remaining scope supersedes the initial paragraph: exactly 1113 pending
Code scaffold entries, listed in ../analysis/round-001-commentary-remaining.json.
First pending Code is physical47542; all other entries carry review credit.
The code at physical1398 (CPU$8566, BRK) conflicts with a duty-table read at$8518;
this is a new structural redirect question. Classification was preserved.
Fresh verify passed byte-identical after correction of a duplicate local symbol.
Reading-after.asm refreshed. Full gate is still pending, editorial audit unrun.

## Stage screen setup and reset batch

Reviewed isolated bank5 initialization physical[47542,47627), all classified
bank6 Code physical[49168,51643), and fixed-bank reset physical[130880,131058).
Added descriptive screen command/sequence and OAM-transfer state symbols locally
in bank6, named all established rendering helpers and meaningful branch blocks.
Explained 2x2 metatile expansion (240 indices ->960 tile bytes), attribute packing
(256 indices ->64 attribute bytes), stage-relative pointer records, literal CHR
mapper writes and RTS-based dispatch. Unknown continuation bytes remain explicitly
bounded and do not acquire invented control flow. Reset comments preserve the
precise RAM-clear coverage and unusual PPU address-only loop.

Fresh verify passed byte-identical. Exact pending Code inventory refreshed;
first pending physical51643. No classification changes and no editorial audit.

## Remaining bank6 batch

All currently classified Code in physical PRG bank6 is reviewed. Added status
screen tile-string/glyph/palette mechanics, OAM template transfer and clearing,
asynchronous event-mask sequence matching, and the stage collision lookup.
Collision symbols distinguish pixel probe X/Y, type results, cell identifiers,
page selection, subcell quadrants and numeric stage-specific substitutions without
inventing solid/hazard meanings. Comments retain unchecked table-domain contracts.
The first pending Code is now physical57360 (bank7). Exact inventory remains the
JSON frontier; prior 1113/1091 entry counts are historical checkpoints.
Fresh verify passed byte-identical. Full readability gate remains pending.

## Context-capacity interruption

Same phase must resume. Current reviewed Code3967 bytes; pending Code18456 bytes
in1060 exact scaffold entries, starting physical57360. All non-Code inventory
notes are reviewed. Added118 compact meaningful entry-block units and refreshed
all source locations. Fresh byte-identical verification and reading refresh passed.
See ../analysis/round-001-commentary-handoff.md for the compact resume contract.
This is an interrupted checkpoint, not successful completion or ready for redirect.

## Resumed bank7 stage/scroll batch — active

Resumed the prepared source in a fresh context; historical interrupted handoff
preserved as analysis/round-001-commentary-a-handoff.md. Reviewed 981 additional
Code bytes in 70 scaffold entries physical[57360,59458), excluding existing gaps.
Stage/scroll dispatch, timer-gated layer offsets/page toggles, tile-column requests,
stage initialization and bank5 ten-byte screen records now have local symbols and
functional explanations. Numeric fixed-bank calls await their own review; unknown
continuations and table bounds remain explicit. Fresh byte-identical verify passed.
Pending990 Code entries/17475 bytes, first physical59568. Same phase active.

## Bank7 complete — active bank8 frontier

All2280 classified bank7 Code bytes reviewed. Added reverse-scroll transitions,
coordinate-delta/page-boundary mechanics, previous row/column requests, metatile
expansion and quadrant attribute packing, stage graphics pointers, palette upload
and final-stage OAM template prefix. Inspection sharpened provisional names: the
low request nibble is a displacement, the special $BC45 entry updates final-stage
OAM, and $BE65 loads the stage palette. Rendering scratch aliases are kept local
where scroll variables reuse the same RAM. Defaults imply224 metatiles ->896 tiles
for this loader, with64 packed attribute bytes; unknown gaps remain untouched.
Fresh verify passed byte-identical before the final alias-only refinement, which
requires verification with the next coherent batch. Pending921 Code entries/
16176bytes, first65552(bank8). No structural/classification changes.

## Bank8 complete — active bank9 frontier

Reviewed all2579 current bank8 Code bytes/154 entries: stage-event dispatcher,
trigger-direction comparisons, class/type scans of nine21-byte actor records,
class-absence timers, position/counter waits, idle configuration and stage variants.
Contact code now explains shape pointer formats (four-/five-byte records), signed
saturating offsets, mirrored rectangles, inclusive overlap, contact response bits,
effect placement and timer activation. Axis A/B names preserve coordinate mechanics
without guessing visual orientation. Unknown tails/table domains remain explicit.
Fresh byte-identical verify passed, including final bank7 scratch alias refinement.
Pending767 Code entries/13597 bytes, first73780(bank9). Same phase active.

## Bank9 complete — active bank12 frontier

Reviewed all1562 classified bank9 Code bytes/122 entries. Actor-family handler
and state parameter ABI, motion-result dispatch, response flags, stage-dependent
direction, collision probes and child-actor placement have local names/comments.
Enemy family identities and shared fixed-bank helper semantics remain unresolved.
Separation helper is deliberately described asymmetrically: same-page lows use
absolute subtraction; different pages always actor-player and can be negative.
Fresh verify passed before this final wording/name correction; next batch will
verify the refinement. Pending645 entries/12035bytes first98320(bank12). Banks10/
11 have no classified Code; their reviewed non-code notes retain valid credit.

## Bank12 complete — active bank13 frontier

Reviewed all521 bank12 Code bytes/25 entries. Actor metasprite renderer now names
four-byte pointer records, header/tile and geometry streams, signed offsets and
per-sprite flip flags, clipping, tile/palette derivation and forward/reverse OAM
allocation. OAM consumers establish world/screen X/Y for its actor coordinates;
prior bank8/9 neutral-axis names remain mechanically valid and can be sharpened
with this dependency. Fresh byte-identical verify passed including bank9 final
separation correction. Pending620 Code entries/11514bytes first106512(bank13).

## Bank13 complete — active fixed-bank frontier

Reviewed all3328 classified bank13 Code bytes/157 entries. Added player action
priority, health/contact immunity, animation clock and three frame streams,
frame motion/damage overrides,16-byte stage movement records, collision cell/
quadrant alignment and moving-boundary mechanics, plus player metasprite output.
Scratch roles receive routine-local aliases where animation, collision and renderer
reuse zero page. Meaning of numeric animations/terrain types stays tentative;
unclassified branches/table domains remain explicit. Fresh byte-identical verify
passed after scratch alias refinement. Pending463 Code entries/8186bytes,
first114710(bank14); previously reviewed reset interval remains preserved.

## Fixed API/actor primitives and main stage setup — context checkpoint

Reviewed fixed API jump slots physical[114710,115253), all classified main stage/
session setup Code[115256,117352), and actor primitives[121328,122346). Added timer,
actor record/state/shape/update-flag, viewport/depth/facing/range contracts; stage
loop, lives/player copies, camera bounds/triggers, OAM/CHR request handshake and
stage runtime initialization. Fixed API names propagate into reviewed consumers.
Body inspection corrected bank9's provisional initialization wording: $C0B7
consumes contact flags, while $C0C3/$C0C6 change update flag3. OAM consumers also
sharpen prior neutral coordinate names to X/Y in banks8/9. Valid earlier work stays.

One intermediate rebuild detected an alias collision between viewport scratch$F2
and bound scratch$A8; separate routine-local aliases fixed it. Final fresh verify
passed byte-identically and reading-after.asm refreshed after all source edits.
Reviewed Code16667 bytes; pending291 Code scaffold entries/5756bytes, first117379
(bank14 NMI). Exact sorted ranges/IDs remain in analysis/round-001-commentary-
remaining.json. Pending code is fixed-bank NMI/IRQ/video/controller/HUD/sound work
and bank15 runtime helpers; reviewed actor primitives/reset must retain credit.
414 meaningful inventory units are reviewed. All Data/Image/unknown notes retain
prior credit. This context checkpoint is interrupted, not gate success: continue
same phase with the existing source and baseline. Final editorial audit remains
pending. No classification/toolkit changes, cleanup, archive or publication.

## Resumed C — fixed frame services reviewed, bank15 remains active

Resumed the existing prepared source without changing baseline or classification.
Reviewed current classified NMI/IRQ, video uploads/handshakes, serial controllers,
stage sound wrappers and HUD construction/scheduling through physical121191.
Named routine entries, meaningful branches, request latches, controller edges,
digit conversion state and HUD buffers. Raster code retains literal MMC3 values;
unclassified dispatch tables/continuations remain explicit. HUD quantities with
unproven gameplay meaning retain mechanical counter names. A duplicate equate
was removed after the first rebuild; fresh byte-identical verification then passed.
Same commentary phase remains active; exact remaining inventory is refreshed.

## C complete current-source review — verification passed, handoff preparation

All remaining3304 Code bytes are explained, including the physical122889 entry
crossing PRG14/15, eight pointer/RTS call adapters, actor path cursor/displacement,
terrain probes/depth bands, banked actor dispatch, byte timers, arithmetic, mapper
wrappers, persistent counter/signature, player response fragments, timer HUD,
movement bounds and OAM allocation/layer permutations. Pending scaffold inventory
is now empty; all22423 current Code bytes plus prior Data/Image/unknown notes are
reviewed. Meaningful routine units replace this pass's temporary fragment records.

Concrete dependency corrections: the interval record is flags plus remaining
frames, and only$03E5 supplies its duration. Prior bank8/bank13/fixed timer prose
and aliases now say frame counts, not16-bit intervals. Number conversion uses
big-endian input bytes and appends the division remainder. Player response uses
input nibbles, not terrain. Ordinary/special actor render scans compare active
negative type bytes literally against$1E/$1F without masking bit7; source explains
these ineffective equalities without inventing a repaired algorithm.

Fresh byte-identical verify passed after correcting the distinct$E686 return and
$E687 path-update entry aliases. Some path/response/render continuations and
consumer tables remain unclassified; numerical action/terrain identities remain
mechanically named. Final editorial audit remains pending for final acceptance.

## Current-source readability gate complete

All1598 scaffold entries and591 meaningful units are reviewed, with no pending
ranges. Current map/state exactly equal the frozen comparison artifacts. Fresh
supported verify passed byte-identically after the final literal-control-flow
corrections, and reading-after.asm was refreshed. Final game.asm SHA256:
6f972ebacb35a19a5ea85d6253ecbf4f360e0365469abc8c5a578b6b7a753342;
symbols.inc SHA256:
8b40973ed352e544f3ac054898288cda0265e32093af5ba69e2aa7b842e5fbb0.

Current-source readability is complete. Whole-ROM classification remains63.16%;
unknown96574 bytes and L007 remain for structural redirect. Final editorial audit
is pending for final acceptance. Compact complete handoff is preserved as both
analysis/round-001-commentary-c-handoff.md and the latest commentary handoff.

## Consumer metadata contract correction

Corrected all2189 original source locations to game.asm:line, relative to the
source tree, and fixed both local helper emitters. Preserved actual line numbers
and the1598-entry scaffold identity. Added662 bounded scaffold-fragment references
to existing explanations so every Code entry carries reviewed units; these are
references, not662 newly discovered routines. Total reviewed unit references1253.

Read-only archive.editorial_status now validates all1598 regions and1253 unit
references; only the reserved final editorial audit remains pending. Fresh
byte-identical verification and reading refresh passed with unchanged source/ROM
hashes. The metadata check records reading-after SHA256 1aa4ee10f32f75c4eb6967b7948811879ba79a967718c0e66fa83f71c3280ca2.
