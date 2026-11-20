# Active redirect findings (round 1, unchanged map)

C01 renderer $8316/$8317 is a real BCC after BCS; no fallthrough reaches
$8318 on the preceding carry branches. The first pointer selector zero aliases
these branch bytes; usable pointer table entries begin physical $328. First
$8316-based table ends at second consumer base $83F6, whose58 entries end at
first referenced record $846A. Both share142 distinct records; each first byte
count gives1+4N and every distinct record end is exactly next record origin,
through physical $115C. `metasprites.py/json` retains all selectors and boundaries.
Specific table layout requires caller selector contracts as a separate natural-use
claim; the complete joined interval $328..$115C is structurally non-code under
both pointer and count records, regardless of selector completeness. Actual
renderer $8316 branch and selector-zero dummy slot must remain code.

C02 template is physical $40BF..$40CF exclusive (16bytes includingFF), directly
copied by CPU809D untilFF; prior BNE8090 unconditional after LDA#10, next handler
starts80BF. Table CPU83DB/83DC physical43EB..43ED contains signed deltas +34/-2;
consumer QueueVerticalMapStrip uses Y direction. Subsequent code starts83DD.

C03 consumer DE1E selects direction*8 from actor direction and eight-byte records
atDF39: frames0..3, idle/motion timing4/5, frame limit6, attributes7. Values of
frame count are3 or4, matching <=4 frame slots. Eight consecutive records fill
DF39..DF79; second state-animation table startsDF79. First3 accepted bytes of
that table are wrongly Code; preceding F0C9 is unconditional after LDA#00/STA.
Direction bound still being traced; generic non-code conclusion doesn't require
all directions naturally used. Neighbour table/consumer review active.

Graphics consumer C818 loads little-endian pointers C89D+X and bank C8BB+(X/2).
Fifteen entries end at C8CA, which is the first fixed-bank stream. Token parser
checks FF termination,7F destination resets, high-bit literal length or positive
repeat count; all15 terminate normally, none use zero/80 degenerate counts.
Stream0 is fixed C8CA..C8F4, followed by yet-unreviewed nine bytes to C8FD routine;
entries in mapperbank4 include three exact adjacent groups. `graphics.py/json`
retains all tokens/destinations/physical contexts. Stream0 pointer resides fixed
bank even though bank table value0; initial parser error corrected in script,
never proposed/classified as a switchable address. Bank3 streams refine prior
generic data and extend its accepted prefix/tail minimally; inspect all conflicts.

Clock route: expiry-frontier was not permanently paused: 6C already0, match state6
(faceoff sequence). Clearing6C is redundant in this snapshot. Within60frames new
run reached stateA with timerzero, then stateB/C and later state5/6/7 with timer
reload20:00 and live19:48. Need period index writer and scene semantics before
calling this a later period. Changed result permits supported connected continuation;
prior identical waits were interrupted/misinterpreted rather than disproving gate.
`clock_probe.py` captured120 tracedframes, including contexts/coverage/edges;
`progression.py` continued1440 untracedframes and selected screenshots. Artifacts
are artificial because timer edits explicitly split chronology, even if unchanged.

Return contracts: `static_contracts.py/json` performs no mutations. Begins accepted
Code views except three proved data barriers; direct callees and normal successors,
following caller continuation only when a recursive graph has balanced PHA/PHP/
PLA/PLP, ordinary RTS, no unresolved indirect/mapper/stack writes, no TXS/RTI/KIL/
BRK/indirect JMP, and no inline C79A.171 initial ordinary proofs yield1283 new
instructions/2776 byte-count (per-context; unique physical must deduplicate before
proposing). All candidate mnemonics are official; independent disassembly review
still required. Fixed wrappers C2F1/C2F7/C304 remain contextual mapper barriers;
C3CB leaf queue-copy mechanics ordinary return proved, but saved-bank preservation
requires queue destination below7E0. Avoid silently calling all wrappers safe.

## Continued connected summary/intermission graph

D1C5 increments RAM34 period index; initial0->1 confirms second period. New expiry
of actual second period waits in bank1 9CC4 dispatcher66 state8: A2D8 intermission
arcade game. Screenshot period2-step8.png shows arcade display. Its countdown is
finite; a second observation batch (historical file labels period3, but its timer
patch occurred while stateB, not live) timed out the minigame and reached34=2,
state5/6 then live7 with19:31. Do not claim that intermediate patch was a third
expiry; it only allowed more minigame time. Actual third expiry experiment next.

Inspected all9 summary66 handlers: initialization0, sequential states, branch5,
state8 arcade and terminalFF. A2D8 dispatcher80 has6 entries; its input/game
handler chooses one of two dispatcher84 tables according to07FF, with19 or13
entries. Tables end at independent JSR/neighboring control boundaries and all
selected handlers form coherent countdown/input/slot-sprite/game-state code.
Read-only proposal generator follows their ordinary helpers and records blockers.
Added roots are conditional structural contracts, not a claim of natural selector
coverage. Every table target was followed, rather than stopping at first handler.
One embedded5-byte mask table A3FF..A404 (physical640F..6414) was caught from
AND A3FF,Y, adjacent next handler and unconditional BNE after LDA3/STA0651; it is
a data barrier, not executable SLO. New total after exclusion is recorded by
static-contracts.json. Independent disassembly must inspect graphs and banks.
