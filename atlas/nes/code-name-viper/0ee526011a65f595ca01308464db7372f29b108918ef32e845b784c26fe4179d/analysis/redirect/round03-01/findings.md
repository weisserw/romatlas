# Round03 redirect structural findings — ready combined batch

Input is the fully readable, freshly verified pass-003-final checkpoint. This phase
changes research and leads only. Current map/state SHA256 is
7b1032bc75e57ecef9741788fd7f1127b26c87fb506b6fba76d03cded25ad646:
241542 classified, 20618 unknown in119 intervals. The proposal is20570 distinct
unknown bytes:7431 Static Code and13139 Static Data. Accepted new bytes remain0.
Only if all proposals pass independent disassembly review would48 unknown bytes in
21 fragments remain. Exact checks, including unchanged ROM/map/state/commented
source identities, are in audit.json. No terminal completion or exhaustion claim.

Artifacts are local. code-proposals.json contains exact instruction bounds, CPU
addresses and effective banks. data-proposals-final.json contains111 exact slices,
bytes, hashes, per-slice role/rationale and available references. Some data slices
are generic containers; specific tentative role names are explanations, not claims
of complete schemas. residual-after-proposals.json is conditional bookkeeping,
never a replacement for the actual map complement in unknown-ledger.json.

## Changed reasoning and independent constraints

The prior round closed its selected rooted graph; its34 stops were not reissued.
This round started from the exact residual complement, inspected current commented
consumers and mixed bytes directly, then tested independent routine contracts and
narrow repeated data boundaries. A natural caller or concrete selected actor is not
required to establish a routine's structural role. Conversely, a pointer or valid
decoding does not establish that role. The supporting facts below combine coherent
field semantics, predicate-controlled transitions, bounded loops/records, conventional
stack flow, known helpers and explicit bank contracts. No natural reachability claim
is made. All classification proposals have Static provenance.

### Actor routine families: physical10010..14010, banks8/9/14/15

The224-entry split dispatch, retained actor metadata and accepted actor-update loop
were already understood. The new method examines the nontrivial remaining bodies as
routines under X = a valid actor slot. The stored entry words corroborate boundaries;
they are not the code proof. Standalone RTS table targets are excluded. Concrete
body facts for every root, including instruction offsets, memory reads/writes, calls,
return offsets and conservative context frontiers, are in routine-body-facts.json.
final-code-flow.json records the exact supporting decoded graph and service proofs.
The following connected groups supply the semantic evidence missing from a bare
root list:

* 10191..102B3 and1042A..10456: animation/state initializers and mirrored terrain
  movement. They select probes1/2 through C034, test terrain bit20, derive a paired
  signed speed via91BD/91BE and difficulty*2, write030C/030D, and update0301.
  Mirrored branches negate the same 16-bit speed with carry. Other entries gate on
  contact0406, write facing0409 and choose animation through92E5. C034's accepted
  explicit return restores8/9. X-indexed object pages and RAM temporary destinations
  match the independently documented allocator and integrator, rather than arbitrary
  memory operands.
* 10521..1082F: signed countdowns040F/040E and contact/slot gates. Timers expire to
  actual state writes, animation92E5 and stop-motion9062 calls. The slot allocator
  helpers9381/93BE return carry for success/failure; successful branches modify the
  allocated slot through Y or saved20 and set matching facing and velocities. Signed
  Y acceleration carries through030E/030F. State branches consistently return or
  enter another handler, rather than walking the intervening motion/count data.
* 10865..10AEE: completion0406 gates, direction/phase selection0506, explicitly
  bounded adjacent-slot occupancy checks, terrain/contact flags and state transitions.
  10935's masked phase selects four-byte vector fields into030C..030F. The global
  script-state entries gate96/C1, clear or set scratch flags, and tail-call the same
  accepted actor removal routine A44E. These are coherent different entry contracts;
  they do not assert that arbitrary actor classes use every entry.
* 10B21..1106A: counted oscillation, motion tables and reaction handlers. Timer/phase
  branches separate actual byte records from instructions. 10C33/10C87 mirror the
  earlier C034 terrain and paired-speed contract; 10CF4 allocates a secondary slot,
  maps0506 through8D16 into facing and even animation, then resets motion. 10E5F
  compares accumulated time with a difficulty threshold and loads the three adjacent
  vector fields8E85..87. 10F1A/10F3D/10F8C form a complete countdown/state-change
  family. 11007..11069 subtract the retained player coordinate, choose a signed
  velocity quadruple905A..5D and store it to030C..F. These paths terminate at RTS or
  meaningful helper tail calls, with ordinary balanced subroutine stack usage.
* 11315..117B0: shared acceleration/direction setters, coordinate-to-nametable
  conversion, allocation and child initialization. 11336's240-row wrapping, eight-pixel
  shifts,20/28 nametable selection and output26/27 closely match independent fixed
  helper D916. Slot scans advance by10 through known RAM slot records, return carry
  on failure, and on success copy current coordinates, set type98/state81, facing,
  motion enables and signed offset/velocity tables. Source fields and subsequent
  integrator consumers independently explain the records. Data islands9500/9518/
  953C,96D8/96F0/9708 and9728 are excluded from Code.
* 124B9..12EAC: collision/terrain gates, timed animation, signed motion and child
  spawning. A64A increments a four-byte vector index by4, reverses horizontal words
  in its mirrored branch and stops at index60. A7BB/A9A8 mask retained0504 by07,
  fetch split-pointer streams, advance0505 by2 or3 and terminate/restart onFF; the
  fetched offset addresses four-byte velocity recordsB2EC..EF. ACE9 scans only
  slots50/80/B0 and creates type57/stateC9 with copied coordinates and animation1C;
  ADD5 scans60/70/90/A0/C0/D0, returns carry on failure and initializes type98/state81
  using direction-dependent paired offsets. AE76 applies bounded signed acceleration
  then either removes the actor or decrements its state. These concrete allocation,
  carry and record contracts establish considerably more than plausible opcodes.
* 12EDD..1357D: AECD first checks C6 bit1, scans four slots10/20/30/40 and compares
  signed coordinate distances; bounded direction3A&07 selects AF28/AF38 state bytes,
  with negativeFF rejected. AF48/B026 use a four-entry script grid, timed record
  advancement and saved coordinates; B09C/B116 apply terrain/contact and countdown
  motion changes. B387 uses retained0504&07, splitB330/B338 pointers and three-byte
  records into velocity/delay/animation state. B3F0..B4E1 form contact-gated state,
  timed movement and palette transitions; palette-slot FD02 is independently used by
  accepted palette installers. B462 loads signed low/high motion fromB498/B49A.
  No arbitrary RAM sweep or execution of every actor was used.

Dynamic metadata reads through0504/0506 do not alter PRG mapping. Their structural
routine role can therefore be established without proving an actual caller's actor
class. Where a specific table extent is claimed, masks, origin differences or next
independent native boundaries supply it. Otherwise the visibly repeated stored
bytes are only generic Data. This resolves the apparent actor-instance prerequisite
for structural classification while leaving actual selection explicitly unresolved.

### Player terrain/checkpoint graph: physical6520..66F5, banks2/3/14/15

Accepted A500 masks1E and loads A510/A511,Y before indirect JMP. This proves exactly
16 stored words,6520..6540, independently ending at first nondefault handler A530.
The selected bodies form a connected group, not one follow-up per handler. A530/
A53C gate low position nibbles; A56F/A573/A577 and A594/A598/A59C choose phase
variants, form the four checkpoint-key bytes50..53, clear velocity, install scene
state06 and initialize an effect slot. Calls C031 -> D916 convert coordinates into
nametable26/27. C061 -> FA5B explicitly maps10/11, invokes B70B's independently
reviewed six-byte checkpoint matcher and ends LDX02/JMPFEA4. Therefore this service
returns in2/3, precisely for this contract; no mapper-wide permission follows.

The returned7A/7B index RAM0770 and the known checkpoint categories; checked branches
consume/reduce an item, update counters, choose effect/message values through A68F/
A6B7, and end in the accepted9903 animation setter. Default A6CB was already Code.
The A6DF target writes player state1E and returns. The intervening stored metadata
6681..66DB is kept Data. Calls, branches and returned selectors are inspected in
final-code-flow.json and routine-body-facts.json. This is a new combination of
independent dispatcher, key-format, consumer and precise return-bank evidence.

### Independent fixed/scene/audio helpers

D8F0/D905 direction setters reproduce the actor-bank direction-to-flip mapping;
D916 world-coordinate conversion independently matches actor9326. They write RAM
and return without PRG changes. D858 has an explicit countdown backedge into accepted
D847 terrain handling, writes player state1D and movement fields at expiration, then
falls into the accepted D88F teardown. The current slot remains an explicit input.

The C004..C06D service grid has independently established three-byte JMP stride and
known meaningful implementations. Remaining stubs tail to reviewed PPU/HUD/coordinate/
score/checkpoint helpers; this supports executable trampolines rather than treating
stored words as Code. C000's NOP/self-JMP is a self-contained idle/trap loop immediately
before that service grid. No reset or natural caller is asserted.

E6F3/E6FA gate HUD updates by mode and timer, set high color bits only for nonblank
069B..D digits and return. E787 divides inputA into hundreds/tens/ones by finite
subtraction100/10, writes0680..2,Y and blanks leading zeroes with24. F680 fills
32 palette RAM bytes0180..19F with0F. F779 sets OAMADDR0 and triggers page02 DMA.
F7F9 andF82A each reset the PPU latch, initialize an exact nametable address, copy
1024 space bytes with X4/Y256 loops, then64 zero attributes at23C0/2BC0 and return.
Rendering/vblank compatibility is an entry precondition, not an observed invocation.
F8E6/F911/F91C wait through the accepted F931 vblank helper then update PPUMASK shadow
and, for the matching entries, hardware2001. F64A writes the supplied PPU address
A/X through2006. These tightly matching hardware operations and finite loop/stack
contracts support their roles independently of a live caller.

F5E1 writes a SINGLE byte1A to the RAM destination B2/B3 with Y0, postincrements the
pointer, returns below2000 and traps in a self-JMP if the high byte becomes20. It is
not a loop that fills all RAM. Only this explicit RAM-destination precondition permits
the otherwise conservative indirect-write continuation; no arbitrary-pointer permission.
The prior provisional root description calling it a fill loop is corrected here.

Audio86AE..86B4 is a seven-byte continuation of accepted86AA PHA/86AB JSR86B5:
PLA retrieves the saved extra-tick count, subtracts1, loops to PHA while nonzero and
RTS returns when complete. Existing31-byte RAM audio-record and854A inline-dispatch
contracts are reused (manual-code-claims.json, round02 audio-proof.json). Inline words
remain Data. No new score selector or natural playback claim.

Scene helpers B6A4 clear31/AF/B0 and call the accepted fade service; BBEE chooses
palette09 then tail-calls BBF5; BCDF initializes the text timer and loops through
BC6D/frame/sprite services until the text completion flag. Initialized text pointers
remain a precondition. The BFE2 continuation follows accepted C064's explicit
return to4/5; its following standalone RTS remains unknown.

## Non-code structure and exact residual review

111 data slices are individually documented in data-proposals-final.json. Major
independent groups are: repeated signed-coordinate/tile player graphics4020..4841;
four-byte bank5 resource records between accepted uniform zero regions; actor signed
motion vectors, phase/animation/count arrays and terminated streams between the
reviewed routines; fixed scroll PPU-address and0..31 index cycles; boolean/palette/HUD
records; and the split D973/D98B scene-strip pointers,24-byte offsets/counts and
repeated24-byte tile resources through E2DE. Accepted strip renderer F26D provides
independent consumer semantics; generic Data does not claim that every slot is used.
The first low-pointer byte shares prior helper RTS at1D983, so the new Data starts
1D984 and does not overlap Code. Bank-tail zero/20/0F patterns are localized, never
used to infer surrounding mixed code wholesale. Available reference origins and
indexed possible extents are preserved distinctly in residual-references.json.

Tiny finite consumers establish cursor arrays BC0B two bytes, B720 two bytes and
B887 three bytes. Projectile state20 is explicitly initialized at A18D and incremented
to21 by D817: F024 then reads the overlapping low/high grid to form D820. This proves
physical1F083 is the stored D8 high byte, without classifying following isolated RTS.

Important primary correction: physical130680/1FE78, CPU FE68, is04, not the EA stated
by old notes and current commentary. ROM bytes are060A040406060604040404 before the
accepted FE69 LDY00. The final04 continues the repeated even-bank metadata; decoding
opcode04 would consume the next accepted A0 opcode as operand. Narrow generic Data
is supported without asserting a naturally selected stage10 or enlarging a specific
stage-index domain. The commentary correction belongs to the next commentary phase.

The remaining48 bytes/21 fragments are preserved with exact bytes and scoped
competing interpretations in residual-dispositions.json. Most are lone RTS, very short
load/tail-call stubs, or isolated two-byte fragments. No pointer-only or opcode-only
proposal is made for them. Notably1E59A's12 plausible bytes (four instructions) store Y coordinates
but fall directly into proven tile text without a return/branch; a coherent complete
routine claim is not established. They do not justify re-running completed gameplay.
This is a ready batch, not terminal rejection review or a claim that all future
methods are exhausted.

## Verification and next phase

api.describe,119 residual decodes, current-source consumers, prior decisive artifacts,
new grouped contracts and4588 context-aware graph instructions were inspected with
the configured release binary. Static analysis only; no new runtime mutation,
replay, map/state annotation, source edit, toolkit edit, archive or publication.
Audit:3292 proposed instruction records, zero differing physical decodes, instruction
overlaps, Data overlaps or Code/Data conflicts. Proposed data boundaries were checked
against the EXACT internal-map complement. Initial provisional12CE9..12D49 was narrowed
to12CF9 when byte inspection found the slot-search routine; the provisional file is
retained. All later proposals exclude that code. Source/ROM identities remain exact.

Independent disassembly should load current state, re-decode proposals under their
explicit effective banks, inspect grouped contracts and known helper effects, then
annotate only accepted intervals with Static provenance. Verify current map and exact
rebuild. Review the48-byte remainder without inheriting unsupported caller prerequisites,
but do not classify tiny fragments solely to complete the ROM. Full current-source
commentary follows accepted classification; final editorial audit remains reserved.

No toolkit issues found. No toolkit blocker; limitations encountered were conservative
analysis barriers resolved only by the stated narrower contracts. Disk space and
attempt size are recorded at handoff; no evidence or history was removed.
