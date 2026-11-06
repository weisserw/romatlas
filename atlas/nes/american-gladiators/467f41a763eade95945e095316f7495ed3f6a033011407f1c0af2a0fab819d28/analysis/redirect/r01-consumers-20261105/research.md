# Round 01 redirect research

Phase complete: return this combined **proposal** to independent disassembly review.
Input map/store checkpoint `round-01-disassembly` is unchanged, SHA256
`4c6a44876fb0b171e856cb54a96e453676de8b6ec8feb5da8b25957ddbc42cdf`.
Actual classification remains144024/262160 (54.94%),118136 unknown.
Proposals cover118135 unique currently unknown bytes:18895 code and99240 data.
No byte has been accepted in this phase. If every proposal survives review, one byte
remains unknown; that conditional count is not a completion or compatibility claim.

The exact per-instruction/per-record batch is [candidate-batch.json](candidate-batch.json).
It includes8234 candidate instructions,6994 data records, all physical identities,
execution views/contexts and evidence links. Repeated views/records overlap within the
same role; union counts avoid double counting. There are zero proposed code/data or
existing-map conflicts. Review/annotate only unknown portions, splitting existing
regions and preserving evidence provenance. Static inference is not Observed execution.
[unknown-complement.json](unknown-complement.json) is the exact **current** unknown
complement, split into bank-aware proposal/residual dispositions; it does not modify maps.

## Connected actors and inline records

The classified stack contracts resolve JSR parameter barriers: actor descriptors are
three bytes (callback low/high, mapper selector); animation/metasprite/rectangle and
NMI hooks take two bytes; text and palette arguments end at their consumer terminator.
[research.py](research.py), [records.json](records.json), [instructions.json](instructions.json)
and append-only decode.jsonl preserve bounded decoding and origins. Direct branch/call
paths, actor callbacks, continuations, associated helpers and data were followed together.

Callbacks select physical pairs0/1,2/3,4/5,8/9; the final event's explicit selector5
transition through$F595 reaches banks10/11. Sound wrappers at physical1E469/1F587/
1F592/1F59F have explicit pair6/7 overrides. Mapper-changing writes are barriers;
unknown lower slots remain null while fixed high code continues under[null,null,14,15].
No bank is inferred merely from a pointer or from its storage bank.

[structures.json](structures.json) and [structures.py](structures.py) preserve finite
animation records, optional extensions, zero-delay redirects, complete metasprite
width*height tile/attribute arrays and tile rectangles. RAM pointers and zero pointers
are not assigned fabricated ROM origins. [template-envelope-records.json](template-envelope-records.json)
includes bounded descending template copies to RAM; later runtime digit edits do not
change those templates' immutable source identity. Repeated unselected complete frames
are Static proposals, without a natural selector requirement. Partial record runs were
localized rather than rejected as whole gaps (residual-pattern-rejections.json,
final-followup-records.json).

## Sound graph and sample bounds

Sound directories$9A1D/$9B1D each have128 two-byte cells: eight-bit ASL supplies exactly
even offsets0..254. Every unique selected header has logical slot0..9. The command
handler pattern$9925..$99E5 has96 cells before the independently used envelope directory;
one is RAM$0088. This physical boundary does **not** prove a natural command domain.
No ROM code is assigned to the RAM target. Immediate callback writes and phase0..5
assignments support additional handlers independently of natural gameplay selection.

[sound.py](sound.py), [sound-command-contracts.json](sound-command-contracts.json),
[sound-records.json](sound-records.json) and [sound-call-frontier.json](sound-call-frontier.json)
recover all concrete directory/stream continuations under95 supported command contracts,
with zero unresolved parse barriers. Straight handlers have identical Y consumption on
all analyzed paths; documented preserving callees were checked. Nonlinear loop/jump/
call/return/random-call/wait contracts are explicit.

A corrected premise matters: random call$B056 is followed by pointer cells$B05A/$B05C.
Those selected streams tail-jump, so blindly parsing a call's following bytes produced
an impossible opcode from a pointer high byte. The final algorithm resumes a call's
following stream only if a selected substream has a supported return path. Its pointer
cells and targets remain data supported by the mask/base consumer. No parser failure
is inherited as a rejection of the surrounding non-code.

Parsed opcode$8F operands select envelope pairs0,2,..12; adjacent-pointer semantics
independently establish all fourteen pointer cells and bounded attack/release records.
The envelope reader supplies amplitude/step/target, hold counts and zero/zero stopping;
no original envelope generator is inferred.

Parsed$D1/$D2 operands and fixed handlers establish DMC descriptors0..8. Fixed slots
$C000/$E000 are physical14/15 in retained execution contexts; mapper selector3 only
chooses lower6/7. See [dmc-descriptors.json](dmc-descriptors.json), media.py and media-records.json.
The hardware start is start&$FFC0. The encoder's reverse shifts restore the **original**
low byte before length subtraction, so length is16*((end-original_start)>>4 &255)+1.
An audit corrected sample7's unaligned start: hardware reads$DC40..$E0D1, not through
nominal$E100. The47-byte gap$E0D1..$E100 is separately proposed as inspected patterned
non-code, never as DMC-read evidence. This correction is preserved in final-data-island-records.json.

## Graphics containers and event descriptions

[graphics-streams.json](graphics-streams.json) contains18 supported graphics roots,
each with an exact1024-byte output budget and the exact consumed source interval.
Zero/$80 controls skip; positive controls repeat; negative controls copy literals.
No whole raw gap is classified from a decompressor pointer alone.

[scene-streams.py](scene-streams.py) and [scene-stream-checks.json](scene-stream-checks.json)
establish19 unique containers from eight supported roots. The fixed reader takes a
mode and encoded source endpoint. `$F400` compares the source cursor to that endpoint
before starting another header. Every proposed compressed interval consumes exactly
its endpoint and produces whole30- or32-byte chunks under that mode, supplying
independent format/extent constraints. The combined physical union is35606 bytes.
Invalid subsequent headers end the claim (scene-stream-frontier.json); they do not
extend the compressed container into attribute reservoirs or bank tails.

The six-event description directory selects eight variants per event (RNG&7, then
ASL). All48 pointed rectangles have finite dimensions/cells under the established
working-pointer rectangle consumer. Split palette arrays have38 cells each, with
independent neighbor boundaries and complete shared resource-format consistency.
Their dual-use thunks are JSRE784, a terminated inline palette, then RTS. Additional
matching variants retain Static code/data roles even when natural invocation is unknown.
See [palette-descriptions.py](palette-descriptions.py), palette-checks.json and
final-followups.py. Array/layout bounds and natural selector domains remain distinct.

A25-row eight-byte motion/OAM directory has an independent endpoint: its lowest
pointed packed-coordinate path begins exactly after the last row. All thirteen targets
are11 bytes apart; the consumer emits10 packed X/Y deltas. The eleventh storage byte's
meaning remains unresolved without weakening its non-code role.

## Generic data and copied utility code

Before fallback, references, nearby code, established formats and repeated layouts
were inspected. Generic proposals cover clearly non-code packed/attribute reservoirs,
small state/coordinate arrays, tile/string patterns and sound fragments. They do not
invent original table schemas, full selector domains, fill purpose, unused/dead status
or semantic names. Primary per-range byte patterns and references are in
[tail-pattern-facts.json](tail-pattern-facts.json), [final-data-facts.json](final-data-facts.json),
residual-pattern-records.json and final-data-island-records.json. Independent review
should challenge these roles/bounds directly; raw exporter gaps are not their evidence.

Seven identical15-byte bootstrap/vector copies are split from data tails. Writing$80
to MMC1 at the mirrored STA changes the **actual** next opcode fetch to fixed bank15.
The copied JMP bytes have a Static copied-code role; they are not claimed observed
successors under an obsolete bank. Per-instruction views and this distinction are in
mirrored-reset-code.json. Canonical IRQ vector supports the one-byte RTI at$E46E.

Additional fixed/RAM utility entry contracts cover velocity negation, preserving-Y
sound-register writes, PPU/NMI waits and a decoded-pointer rectangle wrapper. Final
byte-reader helpers clone classified bank2 routines with explicit differences: the
final variant omits a dynamic-selector reload and sequence increment. Those bytes
were preserved literally. No caller, useful natural index or repaired behavior is
invented to justify the independently coherent helper-family structure.

## Supported broad progression

The old direct-PC/stack shortcut was retrieved and not repeated. From follow-0 snapshot,
parent slot11 is naturally waiting at$8B17 with savedY10. Set$0591 to3 (final event) or4
(ending),$0595=$1F,$0584=0, clear child10 flags$0554, and set pauseMode$0576=1. No PC/
stack/ROM patch is used. Clearing the child makes the naturally waiting parent fade,
remove actors and take its own completion transition. Earlier paused runs and an
ineffective adjacent$0577 write are retained as failed recipes, not successful visits.

[completion-scheduler.py](completion-scheduler.py) shows final-event Blaze portrait
and the terminal congratulations/password screen. [final-arena.py](final-arena.py)
confirms the portrait, settles and exercises Right+A+B in the Eliminator obstacle
arena. Selected local captures: scheduler-eliminator.png, scheduler-ending.png and
final-arena.png. Exact patches, buttons, frame/operation budgets, timeout states and
snapshot paths are retained in runtime.jsonl and per-scenario results. These are
artificially shortened supported transitions; natural reachability is not claimed.

## Phase stop and remaining frontier

Connected actor/data, sound, envelope, media, completion, container, description and
remaining concrete utility/pattern avenues were investigated in this phase rather
than scheduled as artificial extra cycles. No credible identified action remains
before **independent validation of this batch**. More semantic naming or natural
selector recovery would not improve structural decisions already supported by generic
data/finite contracts. Unclear copied/helper behavior remains explicit, not repaired.

The sole unproposed current-unknown byte is physical$00CBDF (bank6 CPU$8BCF, byte$60).
[residual-assessment.md](residual-assessment.md) gives the decisive role ambiguity and
why arbitrary execution or an unreferenced RTS is insufficient. It is not a whole-ROM
terminal exhaustion claim. Any disassembly rejection with narrower/new constraints
returns to redirect after full current-source commentary, preserving these records.

ROM, map/store, source, symbols and configured toolkit binary were unchanged. All
owned research servers/jobs stopped. No archive, cleanup, publishing or toolkit edits.
Toolkit: **no toolkit issues found**; no blockers. Final editorial audit and byte-identical
verification after accepted annotations remain later gates.
