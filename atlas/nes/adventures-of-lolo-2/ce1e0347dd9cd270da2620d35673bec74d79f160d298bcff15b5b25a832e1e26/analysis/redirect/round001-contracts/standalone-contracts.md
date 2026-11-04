# Conditional standalone routine contracts

Static provenance; natural selection unresolved. Banks [0,1,2,3]. These entries are not accepted from legal decoding alone: each has a supported input/output contract and independently matched consumer/shared-tail behavior. Exact instruction records are in graph.json, dispatch-entries.json and decode-cache.json. Independent disassembly must validate them.

- $4ACA: tile attribute updater, input A attribute X/Y coordinate, bounded pointer proof and saved coordinate return in scene-contracts.md.
- $4CAF: A/X source pointer; copies sixteen bytes to palette shadow $56..$65, then joins classified palette-update tail $CCD2. No mapper effects.
- $5259: queue reset entry copies tail $26 to head $25 and RTS; same ring indices are read by immediately following queue-space helper. No register/stack side effects beyond A.
- $5A11: note parameter reader loads current byte through audio pointer $39, masks five bits, sign-extends bit4 through known BIT mask $D5F4, returns Y=$FF as used by the related reader convention. No stream extent claim.
- $4C29/$4C34: parallel nametable-clear wrappers, JSR repeated PPU descriptor +2 inline bytes +RTS; descriptors and existing analogous clear wrapper establish role and exact continuation.
- $31D5: life display helper loads fixed screen position and shared lives $7A, tail jumps to accepted decimal display consumer $B1CE.
- $3288: bounded RoomTiles $0600+X scan X=$22..$CC for tile $0F; initializes $B2, increments only on match, RTS. It supplies the same scene counter used by immediate following periodic scene update.
- $1C35: A0..3 selects independent four-entry low/high palette arrays; preserves pointer pair across stack and tail jumps to sixteen-byte palette-copy consumer. Exact four palettes and split tables bounded by next code entry $1C8A.
- $2DAE: alternate source pointer $06F1 joins classified room-record reader; sets high byte $06 then clears per-room scratch before reading escaped room objects through pointer. No arbitrary RAM-to-ROM origin assertion.
- $2E83: room row renderer X=$F0..0 Y=$E0..0 step16; existing tile renderer $AEE9 consumes coordinates. Shared initialized room grid/PPU queue precondition. Connected following object loop uses the same object overlay.
- $1FAD: ending scene initializer clears scene slots via $8000, counted inline roots via $801A, then waits/transfers/renders with $8082 until progress $98 reaches $DA. Connected continuation independently supports ending scripts. Callback closure in scene-contracts.md.
- $4A31: initialized scene overlay flash/animation helper sets countdowns $06F1/$06F2, alternates validated palette loads, waits via PPU/audio frame service and returns after countdown reaches0. Direct calls and RAM counters supply more than opcode plausibility.
- $BDF: direct JSR target from known tutorial UI, A0..15 lookup digit remap then tile value, destination $016E+Y; increments Y and RTS. Supports formerly stopped caller continuation $B89 and its stage digit display mechanics.
- $2EC3: reads RoomTiles via existing coordinate-to-cell helper $AFC7, selects four metatile plane bytes via the same $B42B/$B51B/$B60B/$B6FB origins used elsewhere, builds RAM 2x2 descriptor at $016C and calls attribute transfer $D0E9 with pointer $016C. Inline pointer is data; RTS after it. Room/queue preconditions explicit.
- $4B14: unsigned16 A/X decimal decomposition; saves value to $19/$1A, repeats compare/subtract four big-endian split decimal divisors 10000/1000/100/10, writes quotient digits $15..$18 and leaves final remainder. Consumer-style repeated arithmetic and independent encoded divisor origins establish code and eight following table bytes; original caller unknown.
- $1BA3: transition alternate entry sets fade mode2 and waits20 frames via known helper, joins already classified stage-clear body at $1BAD. Stage transition environment required; no claim of natural caller.

Reset vector CPU $FFEF points physical $7FFF: SEI; LDA0; STA$8000 selects mode0; JMP$FF4F at physical $8005. Fixed bank3 remains mapped; slots0/1 unresolved until bootstrap. Two zeros before vectors are non-code bytes, no fill purpose claimed. Proposed reset prefix and stub use partial context [null,null,2,3], not assumed identity slots.
