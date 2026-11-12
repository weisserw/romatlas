# Static contract research (active)

Input checkpoint round-001-commentary-final; physical addresses include 16-byte header.
CPU = physical + $7FF0 throughout PRG; effective banks [0,1,2,3], mapper 3.
Linear probes preserve exact bytes and configured decoder output in condensed form.
They are not evidence of execution, and decoding alone is not the proposed proof.
No annotation has been made.

## Player, title and small helpers

Title setup source game.asm:312 installs $82CA and enables NMI. The unknown
$02DA..$030E saves A/X/Y, uses bounded delay, calls accepted PPU/palette/controller/audio
helpers, decrements FrameDelay, resets PPU latch/scroll, restores in reverse and RTI.
This is an interrupt-handler contract, with exact end before known $82FE.

The dispatch in AdvancePlayerAnimation (source:971) supplies ($BC),Y and table $8B7C.
Table words select $88B6/$88C2/$88CF: jump-to-script-address low minus base-low,
load loop count, decrement and either branch cursor or skip operand. Each updates
$03AE and jumps $87BB, no stack change. Unknown $08C6..$0913 is three handlers plus
$88E5 actor reset (zeros active/vertical fields, sets animation 5 and previous $FF,
tail $8757); targets are script-table entries 1,6,7,2. $0955..$0975 supplies two
one-byte commands, cursor++ and equipment/progression writes, tail $887C; table
entries 5,8. $0975..$099B and $09A6..$09CD are collision response handlers selected
by $8C50 table. They update $0389, conditional motion/hit state and join $86E2;
new shared helper $8AF2 ($0B02..$0B49) has direct JSR callers in these responses and
street hit handlers, gates by $03B2/$038C/$03B8, clears attachment/motion state,
resets projectile and returns. Thus the support is handler data/control contracts,
not solely pointer values.

$0FAC..$0FD3 is a complete rising-edge input function matching accepted PollStartRisingEdge:
mask ControllerHeld $64 by ROM $84A3, compare/store latch $04E0, return A=0/1.
$1C54..$1C6A adds input A to meter high $04B9, saturates at 3 and clears fractional
bytes $04B7/$04B8 on saturation. These two have no located live caller; their local
role/control/data contracts and sibling routines support static proposals, not
reachability. Reviewer should explicitly assess these weaker standalone contracts.

## Street object and projectile coherent graph

Accepted pool source:3849 dispatches script via ($21),Y, slot in $A4;
optional behavior uses X slot; collision loops dispatch responses with X slot.
$3025..$308B implements behavior 2 (direction toward player) and 3 (driving lane
spawn position/jitter, resets behavior and tail $AD1F). $314A..$3172 implements
script count/load-loop commands identical in mechanics to player/car. $31EA..$3297
implements signed-direction script branch; per-owned-sprite attribute stream;
wait until position threshold; removal; audio; driving palette trigger; collision
flag command. They retain slot/cursor and join $ACF5/$AD0B/$ACDB. No stack-manipulating
inline payload barrier occurs in these ranges.

$32FD..$3433 contains collision response families at $B2ED,$B304,$B327,$B336,$B345,
$B354,$B368,$B37C,$B390,$B3A7,$B406 plus called helper $B415. Their contracts clear
objects, affect player equipment/state, select script 9..11, reduce car speed,
or spawn car interaction objects; all tail $AB97, helper returns RTS. Table
$B44D has corresponding words (including $B403 entry, the common tail of $B3A7).

Projectile/object hit table $BE3A selects unknown $BC8C..$BD4D ($3C9C..$3D5D):
remove/hide object/projectile, optionally call $BDC5 with A=0..2, queue audio and
join $BBF0. $BD33 is a standalone JMP common tail selected for no-effect types.
$BDC5..$BDF4 ($3DD5..$3E04) initializes a replacement object using A=0..2 to index
three-byte script/effect tables at $BDF4/$BDF7; restores X from $A4 then RTS.
$BDFA..$BE06 ($3E0A..$3E16) is projectile-tile response: clear projectile and audio
then RTS, or JMP $BD9B scan continuation. Decode must restart at $3E0A; the broad
linear probe intentionally crosses six data bytes and loses instruction alignment.

## Car

Table $D72A selects four words ending at next independently referenced direction
lookup $D732. Unknown code $559C..$55CB holds jump/load-loop/decrement-loop handlers
at $D58C,$D598,$D5A5, which read ($1D),Y, store $0656 cursor/$065C count and rejoin
$D42E. No added return frame. Known frame handler $D4FD consumes fourteen tile bytes
and duration. Two animation scripts at $D76E/$D790 have repeated fourteen-tile
payloads/durations and loop commands; they are not CPU instructions.

## Bonus interpreter and callees

Accepted UpdateBonusActorAnimation source:8222 loads shape metadata ($B0 count,
$B1 OAM base), current actor $AD, script pointer ($55), cursor Y; dispatch table
$DCAA starts with frame handler $DA53. All 22 apparent words through $DCD6 target
coherent handlers in $D9FA..$DC9C. These handlers use precisely those fields and
rejoin accepted $D9C1 dispatcher, $D9B2 animation reload, or $D9D7 RTS. Complete
register/stack contracts, cursor increments and sprite loops distinguish these from
accidental decoding. Inspect linear-5a0a.txt up to $5CAC. Handler $DC33 calls
$D096 ($50A6..$50C1), disabling actor and adjusting bounded game counter $86;
$DC3B calls $CA9F ($4AAF..$4ABB), disabling actor and hiding OAM via $CBD4.
Both have accepted actor/OAM contracts and RTS. $DC43 writes one attribute per
shape sprite; $DC0F writes one tile per sprite plus frame duration; $DA53 writes
one common attribute for count sprites, increments cursor by two, redispatches.
Other payload mechanics are explicit in probes: jump, duration, position pair,
velocity pair, mode flag/audio writes and game-specific interaction branches.

The exact code/data boundary is CPU $DC9C (file $5CAC): final handler tail JMP
$D9C1 is followed by DigitTiles used by DrawBonusCountdownDigits. Do not continue
linear decoding into those bytes.

## Non-code candidates and extent limits

$0B8C..$0EC1: handler words, direction lookups, multi-byte motion arrays, collision
pointer/property arrays, 81-byte zero run CPU $8D1F..$8D70, direction-to-animation
values, animation pointers/scripts, four-tile frame records and attribute bytes.
Consumers game.asm:643,660,673,727..745,827..829,891,949,958 and PlayerScriptSetFrame
anchor multiple independent subsections. Uniform run and repeated low-domain data
are clear non-code. Original subtable segmentation is not required for generic
Data; specific labels may be used for consumer-established starts only.

$3433..$39D3: parallel handler/pointer arrays ($B423..$B4E3), repeated signed sprite
coordinate/rectangle records, per-type sprite counts and animation pointers,
script payloads and spawn-list pointer domain. Source object projection, collision
and script consumers establish reads. Numeric repetition, bounded payload layouts
and next accepted record suffix separate the interval from the code ending RTS
$B422. Inspect possible conflicting code evidence before annotation; do not infer
whole-table selector domains merely from consecutive pointers.

$3E04..$3E0A are two three-byte helper lookups. $3E16..$3E68 are projectile tile
handler words, four-corner offsets, two order pointers/lists and object-hit words.
Their consumers and aligned adjacent entry boundaries support Data, with some
specific tables available. $573A..$583F includes car handler words, 11 direction
lookups, signed movement/speed pairs, two scripts, lane selections, palettes and
10 two-tile digit records. $5CAC..$625B includes digit lookup, bonus handler words,
11-entry shape pointer/count arrays, repeated coordinate/rectangle lists,
48 consecutive animation pointer words ending at first script $DE19, then repeated
variable-width animation payloads through final script ending $E24B. These are
non-code, with exact container boundaries independently anchored by final handler
and next accepted palette routine. Generic Data remains supported where original
subtable extent or shape pairing is unresolved; selected pointer values alone do
not establish a specific full table schema. None of these are proposed as fill.

Conflict check: final vector/observed-rooted discovery contains no direct control
references into the proposed non-code containers (data-conflict-check.json).
This is supporting negative evidence only; positive pattern/consumer evidence
establishes Data. Raw final bonus bytes $6180..$625B were inspected separately
because the bounded 1000-instruction linear probe ended at $618B; repeated script
records and final 05 00 00 02 60 05 10 sequence continue to accepted $E24B.

## Driving discrimination and bounded representative visit

New method compared two explicitly different entry contracts, recording palette
latches, NMI vector, stage/camera and car fields every 60 frames, rather than
repeating unchanged long gameplay. Main entry $81FB includes ShowStoryCard $97AD;
through 300 frames it still had CardNmi $984B, StageIndex 0 and card wait counters,
so those samples say nothing about driving initialization. Direct understood
RunDrivingMode $D1FC, same clean active snapshot/buttons[], initialized stage $11,
DrivingNmi $D669, scroll intro and car. At +240 frames the selected frame visibly
shows road/car/HUD and speed30 (driving-direct.ppm and PNG rendering). Exact server
requests/responses in runtime-log.jsonl; scripts record every patch and bound.

Important limitation: direct $D1FC with edited SP=$80 tests the routine body without
constructing a caller return frame. It is suitable while the mode waits, but after
its RTS it does not prove the main success/failure card transition. Future caller-
correct replay should enter CPU $8200 (the JSR $D1FC instruction immediately after
the story-card call), so hardware JSR supplies return $8203. Do not reuse the
post-return direct-entry state as a main-loop completion proof.

Representative controls from driving-direct.snapshot: buttons up/right for45 frames
moved X $6E->$8C and speed30->38; release recorded, selected steered frame retained.
Source AdvanceRouteRowAfterStripWrap proves $CD increments to StreetEndRow $0666
and clears ScrollEnabled/StreetActive. Targeted patch $CD=$FB discriminates this
supported end-latch condition with StreetEndRow=$FC; bounded observations confirmed
StreetActive=0. This is artificial progression evidence, not a natural ending.

Success logic has a concrete additional constraint: object/player hit response
$B3A7 clears $A9, requires CarSpeed >=$58 to set it, then allocates a script-$10
object; other branches leave it zero. Thus merely ending route or setting speed
without that interaction does not establish success. Further natural ending work
needs the type-12 object's spawn/position/timing route plus caller-correct return.
No unknown structural interval remains outside the combined proposal, so additional
natural-driving permutations have diminishing structural prospects in this pass.
