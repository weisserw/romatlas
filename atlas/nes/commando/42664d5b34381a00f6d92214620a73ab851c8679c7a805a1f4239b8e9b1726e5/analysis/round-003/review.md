# Round003 independent structural adjudication

Reviewed all thirty R002-RESIDUAL proposals independently using fresh configured
binary decoding (review-decodes.json/txt), selected primary consumer/control paths
(primary-review.json/txt), raw backing bytes, existing map and current commentary.
Boundary-audit.json checks exact complement coverage, instruction endpoints, zero
existing overlap and encoded direct code targets conflicting with proposed data.
All roles are Static. No new natural reachability or execution coverage is claimed.

9AB6 parses exactly nine four-byte destination/count/first-tile records and a
zero-word terminator. The consumer starts explicitly here; next accepted routine
9ADC fixes its end. A70C FD..04 agrees with AC69 mask07 indexed reader and alias
boundary A714. A830 has explicit copy length1C and next twelve bytes have paired
A84C/A84D reads, X advancing twice to0C. These are independently bounded data.
AEF7 mode parameters and B501 sprite source are direct indexed inputs; their
existing unknown slices are classified without claiming complete selector domains.
C742 explicit source origin and four ascending offsets justify its narrow Data
prefix although a longer read overlaps code. CD01 text record and D94A..D94C
sprite parameters have consumers; accepted D94D remains excluded. E6F2 count04
and repeated hidden-Y FF/zero records support fourteen unknown Data bytes; skip
semantics explain why the fourth hidden record does not require retyping code.

B58B..B592: bytes 00 00 88 00 00 00 10 01 follow the prior pointer loader's PLA/RTS
and precede independent B593 code. Six zero/low-value cells and isolated88 form a
bounded parameter-like island. BRKs/DEY/BRKs followed by a branch into the next
instruction operand offer no coherent alternative continuation. Nearby loader
reads banked8000/8001 rather than this island. No specific consumer/schema is
established: accept generic Data, not named parameters or padding.

CF8B..CF90: 04 06 07 0A 0B 0C follows the scheduler's unconditional JMP D0E0 and
precedes independent CF91 initialization. Six strictly ascending small values and
no meaningful control/stack path discriminate non-code from the NOP/SLO/ANC
linear decode. No direct existing branch/call targets conflict. Accept generic
Data; no ordered selection-table semantics or intended execution inferred.

FA45..FA4C: CF FF FF 3F / 45 55 55 15 lies after transfer-descriptor RTS and before
independent FA4D JSR. Two bounded four-byte groups share repeated packed two-bit
interiors (FF/FF and55/55), with constrained edge bytes; decoded writes toFFFF and
5545 do not supply a supported routine contract. Accept generic Data based on
pattern plus boundaries, not a graphics/attribute schema or absence of execution.
These three decisions were separate from the direct-consumer data decisions.

Aliases A714/A71A, C014/C01A/C01D/C028/C037/C03A/C03D, CE0C/CE12 and F506/F509
are exact three-byte slots among accepted aliases, targeting independently supported
helper entries. FC99 nonzero LDY10 makes BNEFC9F unconditional and shares the
accepted transfer setup. E570 specializes E55D with one sprite. This is layout,
entry and semantic evidence beyond plausible addresses.

C19C repeatedly writes source03:04 toPPUDATA/palette shadow, incrementsY and
countsX before known scroll/control restoration. DA70 writesF0 to every fourth
OAM byte and decrementsX toRTS. FF70 reads PPUSTATUS and strobes4016 1->0 before
RTS. Explicit preconditions are sufficient for Static roles without known callers.
E82C initializes scratch1E and performs sixteen carry-linked shift/add iterations;
both conditional paths join the decrement, then RTS. No mapper operation occurs.

DC40 uses normal current-object/OAM contracts: initializeY10/sprite7A/motion and
field5=98; render/scroll/contact loops yield through C023, continue untilY70 then
apply direction motion; contact tail-jumps destructionDB8D. Yield continuation is
not ordinary fallthrough reachability. EE00 selects02A0 then scans seven32-byte
records; active records saveY, inspect field10=14 and balance PLA on both paths.
No-match advances20/decrementsX, match yields/rescans, exhaustion joinsEE30.
Supported control/stack contracts independently prove code. Natural spawn/selectors
remain unresolved. Fixed-only instructions use [null,null,14,15]; DC40 keeps
bank6 [12,13,14,15] sprite precondition; bank6 lower-window entries retain it.

All254 Code and143 Data bytes accepted,397 total. Broad later/final gameplay and
successful artificial ending presentation evidence are reused unchanged from
redirect/r002-contracts; no repeated replay is useful. Full current-source
commentary and final editorial audit remain pending after this disassembly pass.
