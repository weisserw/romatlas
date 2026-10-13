# Commentary uncertainties

Carry forward relevant phase-1 ambiguities as their regions are reviewed.

## U-ZERO-0426 — reviewed structural ambiguity

Physical/CPU bank-0 $0426-$0444 contains 31 zeros between a non-fallthrough
joypad routine and a detached RET at $0445. Plausible meanings are unused zero
data, a reservation, or a 31-NOP dormant routine ending at that RET. The byte
behavior is identical; decoding as NOPs alone proves no intended purpose.

Phase-1 investigation inspected complete direct targets and 4830 instruction
boundaries, audited the sole state-table indirect jump, scanned immediate
references, rejected 32 little-endian and 10 big-endian raw pointer matches using
their instruction/graphics/command consumers, and found no applicable external
source metadata. A targeted zero-probe route (title, stage, launch, movement,
pause/resume, Start+Select reset) ran 257 frames / 4009009 operations: no
Instruction/Fetch/Read event in this interval, with zero detail/aggregate drops.
This is evidence only for that route. See ../analysis/investigation.md
U-ZERO-0426 and ../analysis/zero-probe-results.json.

This phase re-read neighboring key filtering and IE helpers without finding a
new consumer. Forcing execution only demonstrates legal NOPs; arbitrary play
has no identified discriminator. An original/source-derived reservation or
symbol record, or a newly evidenced natural access, would reopen the question.
No concrete useful local experiment currently distinguishes the alternatives.
The source retains raw bytes and a (?) comment; the phase-1 unknown kind is
preserved. Editorial review of this ambiguity does not complete phase 1.

## U-SERIAL — accessory identity (?) with established interface

Each VBlank resets a two-completion countdown and starts an internal-clock
serial transfer. The first completion stores a position sample; the second
compares its button byte with the preceding sample. Bit zero in the derived
mask denotes an old=1/new=0 edge. MovePaddle falls back to joypad for first-byte
values >= $f1; otherwise it subtracts $30 and clamps a position. State handlers
also accept second-byte bit-7 edges. SB is primed with $01 at initialization.

Static review of every $ff91/$ff92/$ff93 reference establishes the observable
position/button interface, but supplies no manufacturer/model identification.
Naming a specific accessory would exceed the evidence. Existing emulator
gameplay without a corresponding external serial peer cannot distinguish
accessory identities, and no device specification or matching peer is present
in this workspace. A known device/peer trace would reopen that identification;
there is no useful local runtime route that establishes it from this ROM alone.
This limit does not prevent reviewing the transfer and input algorithms.

## U-COLLISION-TAIL — trailing-Y response intent (?)

Leading Y probes reflect; trailing X probes use opposite-direction alignment
without negating velocity. Trailing Y probes instead jump to RET $0ecd.
The nearby $0ece helper would align Y against the current velocity sign, but
is a distinct detached routine, not a fallthrough from that RET. Static callers
and both axis probe implementations establish this actual asymmetric behavior.

Reasonable original intentions include disabling trailing-Y correction, leaving
a dormant counterpart, or a historical target error. The bytes alone cannot
identify which. No source error is asserted or fixed. A runtime collision can
show the already-known empty-tail behavior, but cannot distinguish original
intentions. A natural caller to $0ece or original source/version evidence would
reopen the relation; no specific useful local experiment identifies that intent.
Both units are reviewed with observable effects and (?) annotations.

## U-ANGLE-TAIL — unused direction choices (?)

The $100b-$101a sixteen-byte block contains angle indexes. RerollBallAngle is
the established consumer and masks the sequence to seven, reading only the
first eight bytes. The latter eight are compatible with the same 19-entry
velocity-record domain and resemble an alternate weighted choice set. No
operand/caller established in the static source selects $1013 or indexes beyond
seven. This phase checks each value against the velocity table record bound.

The exact intended use of the tail (alternate choices versus unused residue)
remains unsupported. Changing the mask or pointer would demonstrate valid angle
records, not original intent. No naturally sourced route currently reads the
tail; original source metadata or a new reference would reopen the question.
Source preserves the bytes and explicitly marks the interpretation tentative.

## U-UNREFERENCED-GRIDS — original use (?) unestablished

The already-classified data at physical/CPU $3e44-$4074 is a compatible40x14
cell grid with FF sentinel, straddling permanently mapped bank0/bank1. Another
compatible12x14 grid occupies $4333-$43db. This review checked every cell against
the established0..4 domain, exact row counts and sentinels, all33 descriptor
pointers, and the complete static decoded two-byte operands: neither range has
an incoming operand or descriptor. These are format/effect facts, not execution.

A spare or development layout is plausible, but no historical purpose is
asserted. No consumer-backed input route selects either grid. Forcing the loader
pointer only demonstrates known format compatibility, not original intent.
A new natural reference or original/version-matched source description would
reopen the purpose; no identified local experiment distinguishes these origins.
Source marks the limits and preserves the phase-1 Data classification.

Additional shape check: the 40-row grid has all560 cells zero; the twelve-row
grid has exactly one nonempty cell, type1 at zero-based row11,column10. Empty
and single-brick fixtures are reasonable interpretations of their effects,
while a historical test/development purpose remains tentative. A fresh length
scan of all classified Code regions covered4845 legal instruction starts with
no crossing region boundary and no incoming sixteen-bit operand in either
grid range. This supplements the older static-decoded capture and establishes
no new execution coverage.

The earlier auxiliary count4883 omitted the two-byte JR Z length. Final editorial
checking corrected that counting helper to4845 starts; the incoming-reference
result is unchanged. All4488 distinct starts in the older static-decoded capture
are included in this fresh scan, which also includes357 later-classified starts.

## U-SPEED-DISPLAY — return stub versus adjacent writer (?)

Current speed-display calls all target RET $4a29. The helper at $4a2a writes
sprite4 as Y98,X10,tile(tier+80),attributes0, but is a separate detached entry.
Static callers, exact branch target and neighboring writer were inspected;
prior commentary claiming that tier advancement redraws this glyph was corrected.

Deliberate display disabling and a historical entry difference are plausible.
A runtime call to the current target only confirms the already-known RET; a
forced call to the writer confirms its already-known writes. Neither establishes
original intent. Original source/version evidence or a naturally sourced caller
to $4a2a would reopen that relation. No bug or fix is claimed. Both units are
reviewed with observable effects, and source marks the intended relation (?).

## U-LEGACY-AUDIO — dormant input ABI and write-only flags (?)

The standalone poller at69aa stores active-high held/new-edge masks atFF80/FF81.
Four selector families consume those edge bits with priority0,1,3,2,4,5,6,7,
queuing effect, paired effect/music, noise or music IDs1..8. Normal UpdateAudio
does not call them; its caller is the VBlank trampoline. Current game startup
installs a DMA code image atFF80-FF8B, so these legacy slots alias instructions.
Classified callers and the normal input/DMA consumers establish the conflict;
calling the poller would overwrite that image, not exercise a normal sound menu.

A separate development sound-test harness or retained earlier input ABI is
plausible, but original integration is unproven. No caller-backed route exists,
and forcing the routines cannot identify their former context. Original source
or a natural harness caller would reopen that identity. DFF4's detached setters,
DFE3's mute-only byte and DFD6/DFF8's write-only cue markers likewise have known
writes but no read consumer in the classified code; no hidden voice/flag meaning
is invented. These code/data effects are reviewed, and source marks legacy intent.

## U-MUSIC-END — pulse-channel shutdown intent (?)

Pulse music writes NR21..NR24 (pulse2); rest explicitly clears NR22. On stream
token00 the end path74f9 instead clears pulse cue ID/stereo flag and NR12
(pulse1). Exact stream reader, rest/end paths, effect driver and mute helper
were compared. Actual cross-channel write is unequivocal; intentional pulse1
cleanup and a historical wrong-register target remain competing intentions.
No instruction was fixed. Runtime can demonstrate the same write/envelope
effects but cannot discriminate source intent; original/version-matched source
would reopen it. The end path does not explicitly mute pulse2's envelope.

## U-UNUSED-PRESETS — unreferenced compatible pulse rows

The final three five-byte rows at703d..704b have NR10..NR14 register grammar.
All timed effect starts/step leaves select earlier rows0..28; effects9..12
build their sweep registers inline. Current request and legacy selector paths
supply no caller for these last rows. Their bytes/effects and potential hardware
use are understood; an original named cue is not. Spare earlier effect stages
are plausible but not asserted. Forcing a row only plays its known register
values, without identifying original use; a source description/new consumer
would reopen that limit. No unresolved useful natural experiment is identified.
