# Static proposal contracts and corrected premises

Research only, checkpoint pass-001. CPU bank Bn denotes physical 32 KiB image n,
with effective 8 KiB slots [4n,4n+1,4n+2,4n+3]. Proposed nodes are Static, never
Observed merely because decoded. Current accepted map remains unchanged.

## Control flow

BRK in B0 uses B165 and common FFA2/FFB2/FFC7 return machinery. The stacked
PC is reduced by two and final RTS adds one, so continuation is BRK+1. The byte
following BRK is an instruction, not an inline selector. X/Y select service and
bank. Other banks' IRQ vectors use RTI, so this interpretation is local to B0.
Indexed stores to $0100+X remain within RAM ($0100..$01FF); ordinary decoding can
continue without assuming a mapper write. JSR continuations require callee return
and restored bank review; the walker deliberately retains that qualification.

B3 $8105..$8108 is data [00,0F,1E,2D], read at $80E2. $8103 BNE loops by advancing
X twice; apparent X-wrap fallthrough does not authorize decoding the offset table.
The previous graph includes a false BRK at $8105. walk-final.py removes this root;
all downstream proposal nodes still require independent review, not blanket acceptance.

## Dispatch extents

B0 $8AD9..$8AE8: AND #07 at $8AEB then ASL, two indexed loads at $8AEF/$8AF4,
therefore exactly eight little endian handler entries. All targets lead to coherent
sprite/update operations with converging shared helpers; natural selection separate.
B2 $905A..$90D9: $9038 AND #3F then ASL establishes exactly 64 entries. Two-byte
handler stored at $99/$9A or $9B/$9C and immediately consumed by $902E/$9031. This
proves finite directory structure without asserting every selector naturally occurs.
B0 $861E..$864B: 23 pointer entries end exactly at first target $864C. Consumer
$860D ASL uses A and reads both pointer bytes, but the selector's complete domain
is not established. Extent is directory adjacency plus all entries' compatible
handlers, a static proposal requiring independent review; do not claim mask proof.
B0 $A899..$A8B2 and B2 $80A3..$80BC: each has 13 complete entries ending at first
handler. B2 service0 consumes even RAM0B; bank0 writers and NMI transfer establish
shared selector mechanics. Domain/extent proof remains to be consolidated; these
hypotheses must remain separate from the mask-bounded directories above.
Bank1/2/3 front directories: bounded respectively by first routine $8012/$801C/$8014,
which means 9/14/10 entries, NOT bank1 10. Prior walker seeds bank1 index9 from
instruction bytes; remove that spurious root before any proposal acceptance.

## Additional bounded routines

B1 $8299,$82DE,$82FD,$833B are adjacent independent PPU loaders: set source $04/$05
and count $06/$07, reset PPU latch, set $2006, then JSR/tail-JMP $83FB counted RLE.
These setup/consumer/return contracts support static role independently of a known
natural caller. Exact individual ends must be decoded before annotation.
B1 $8599..$85B6 (inclusive) preserves Y on stack, copies 32 bytes ($06),Y to $2007 per X count,
advances $06/$07 by $20 and restores Y before RTS. No BRK inline-data assumption.
B2 $8253..$8262 stores destination X/Y in $10/$11, copies A bytes from ($0E),Y to
($10),Y using X as countdown, RTS. A=0 mechanically copies 256 bytes. RAM destination
precondition and known physical bank are sufficient structural contract; no natural
caller claimed.
B2 $9254/$92B5/$930F/$945D are separately supported by stored low/high pointer pairs
at $9567/$956B, $94CD/$94DF, $94B4/$94B8 and $9497/$949B. Follow their full handlers
and adjacent referenced tables as one connected proposal, retaining indirect-RAM
and indexed-address preconditions.

## Generic-data correction audit

noncode-control-targets.json checks every direct JSR/JMP edge in saved graph against
accepted non-code map regions. Only two edges conflict, B0 $808F/$81AB -> $E05A.
extra-0-e05a.json shows a short BRK service stub ending at $E06F; previous >=18
uninterrupted official opcode heuristic missed it. This audit is targeted, not a
proof that every generic data byte is correct. No second concrete conflicting
control target was found. Revisit only with a new concrete reference/contract.

## Progression gate

natural-threeframes-contexts.json identifies $82DA..$82EB as interlude gate.
$82E7 compares controller edge $3A to exactly $10 (Start), looping otherwise.
unpaused-run.json records release2/Start1/release2 then movement20: Start makes
$31=FF and $3A=10; unpaused.png shows actual soldiers/gameplay. Thus wait/NMI
freezing was a false premise. The first later-stage trial sent Start during loading,
then ended with $31=0. Revised experiment resumes their completed-load snapshots,
releases buttons, sends Start edge, then movement/fire; resume-milestones.py records
exact budgets and results. No ROM bytes patched; original $D2/$D3 patches retained
in milestones.py and snapshots. Artificial scenes do not prove natural reachability.

## Final localized boundary review

B3 $86F2..$86F4 physical100098..100100 is proven non-code lookup suffix: consumer
$8610 AND3/$8612 TAY/$8613 LDA $86F1,Y reads four entries [60,0C,4A,8E]. First
entry is dual-use with already established RTS $86F1; the three unknown bytes are
Data. Initial reference inventory looked only for bases inside the gap and missed
the base one byte before it. This is corrected consumer evidence, not pattern-only.

B0 $990C..$990D physical6428..6429 is proposed separately as a two-byte routine
prefix LDY $A4. Following known tail $990E increments Y, compares limit $A5, wraps
to0, stores cursor $A4 and JMP $9A9D. Existing caller $987D loads the same cursor
and $9884 enters $990E with Y live. Combined dataflow, complete cursor-update
contract, exact alignment and shared continuation support a reusable standalone
prefix $990C, without asserting a natural caller. Strongest alternative is an
unreferenced two-byte constant word adjacent to a shared tail; there is no read
consumer for that alternative. Independent disassembly must decide sufficiency;
this narrow proposal is not accepted merely because bytes decode.

The sprite0 relocation (four-byte RAM0200 record [16,73,2,32]) failed in all four
later-stage snapshots. Static overlap computation did not model full scrolling/
render timing and therefore was a discriminating probe, not proof a hit must occur.
A second method seeks an actual instruction-boundary PC946D/9470 and skips only
to9484, preserving entered subroutine stack/WaitNMI/RTS. All four trials perform
the supported edit, execute additional actor helpers, then revisit split wait;
none yields an independently confirmed sustained later combat/boss route.
Record partial artificial execution honestly. Further repeated skips or coordinate
sweeps lack a new prerequisite; next useful work would need the scene loader's
full sprite/background timing contract, with no immediate unknown structure gain.
