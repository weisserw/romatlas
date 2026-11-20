# Commentary uncertainties

Carry forward relevant phase-1 ambiguities as their regions are reviewed.

## C01: bank-zero sprite-pointer bytes currently classified as code

Physical $000328..$0003F1 (CPU $8318..$83E0, effective PRG banks [0,1,14,15])
contains repetitive STY/STA/SAX/NOP decoding, including repeated $88 high bytes.
The classified routine at physical $0001C2 reads a little-endian pointer table at
CPU $8316/$8317 indexed by twice the sprite selector. Thus $000326 onward has a
concrete competing pointer-data interpretation. The table's complete selector
bounds and the adjacent $83F6 pointer-table boundary are not yet established.
Frozen classification remains unchanged; this is not claimed reachable routine code.

Redirect: inspect the ROM bytes and selector-producing callers for $81B2, compare
pairs to sprite records and check actual instruction fetches independently of
static decoding. Reassess the full contiguous accepted Code range $0001C2..$0003F1,
retaining the real renderer through its RTS at CPU $8317 and separating supported
pointer data only after boundary review. A pointer value alone cannot prove its
referent is code. No new decode/discovery/classification was performed in commentary.

## C02: bank-one inline PPU command bytes decoded as code

Physical $0040BF..$0040CE (CPU $80AF..$80BE, banks [2,3,14,15])
overlaps the template read by the accepted copy loop at $809D from table $80AF
until its $FF sentinel. Frozen decoding contains ORA/EOR/XAA/NOP rather than an
established entry contract. The token-copy consumer supplies a concrete data
interpretation. Redirect should inspect through the sentinel and test fetched
instruction identities independently; preserve neighboring state handlers.

Physical $0043EB (CPU $83DB) is a frozen KIL immediately after an unconditional
continuation and also lies at the edge of a direction-indexed data table read by
the strip builder. No executable entry contract is claimed for this byte. Review
its table boundary with the same code-versus-inline-data check.

## C03: animation-table boundary currently decoded as an unofficial instruction

Physical $01DF49..$01DF4B (CPU $DF39..$DF3B, fixed physical banks14/15)
is frozen Code but begins exactly at the table base read by direction/frame
animation at physical $01DE9E and $01DEAE. It follows a branch back to the
sprite-publication path. No executable SLO entry contract is established.
Redirect should inspect direction/frame index bounds and separate the bounded
animation-table interpretation from any genuine fetched instruction identity.
The commentary preserves the original decode and map pending that decision.

## Meanings retained as unresolved after the current-source pass

- Shared-object movement probably belongs to the puck, but negative ownership,
  special collision states and goal/period gates are not fully named. In particular,
  the clock countdown is understood while the natural later-period route remains open.
- Menu indices have tested setup behavior, but individual team names, graphic
  template identities and complete selector-table bounds are not recovered.
- Sound engine mechanics cover channel arbitration, timers, pitch/envelope bytecode,
  DMC setup and speech selection. Sound IDs, instrument names and phrase transcripts
  remain unknown. Expansion-register accesses are retained without claiming mapper2
  expansion-audio hardware or emulator support.
- Numeric control-transfer operands into unknown gaps retain their CPU addresses;
  the comments explain missing entry/return/bank contracts. Unknown bytes are not
  decoded into invented routines. Fixed-bank direct calls into the switchable window
  retain numeric targets unless the call's explicit bank wrapper establishes context.
- Generic repeated-byte/pattern data retains exact boundary and prior consumer
  inspection evidence. No original asset partition, graphics schema or padding purpose
  is asserted beyond the evidence. These are understood limits, not skipped code review.

The source/readability phase is complete only with the fresh verification and
reading-copy results recorded in progress.md; full-ROM classification and final
editorial acceptance remain separate.

## Round 2

C01/C02/C03 structural conflicts were corrected in round02-disassembly-closure. Their old source decodings are preserved only in the prior commentary; current data must be described by its actual consumer format. Original uncertainty records above remain historical evidence.

## C04 — newly exposed bank-four delta table conflict

Physical $012853..$012857, CPU $A843 in banks[8,9,14,15], contains `20 00 E0 FF`: two signed little-endian deltas +$0020/-$0020. The accepted fighter-effect code at $A824/$A82D reads $A843,Y/$A844,Y with Y=0 or2. Its immediate predecessor $A83C is LDA#0;STA$05AC,X;BEQ$A838, so ordinary fallthrough cannot reach $A843. The frozen current map nevertheless calls its first three bytes StaticCode JSR$E000 and leaves its fourth Unknown. Preserve the exact source decoding but do not describe it as an established routine. A dual-use entry or separate root remains a competing explanation. Bounded redirect check: inspect accepted root/edge for physical012853 and the two consumers; do not infer a legitimate E000 caller from bytes that may be a table. Evidence:analysis/round-2/commentary/conflict-C04.json. No map edit made.

## Round 3 current dispositions

C01/C02/C03 were resolved in round 2. C04 was resolved by the accepted round 3 map: exactly physical $012853..$012857 is signed delta data; current source no longer contains a JSR $E000 there. The older records above are historical evidence, not current classification claims.

The current source explains all classified code/data and every exact Unknown interval. New data roles include 87 sound descriptors, generic command/pointer/nibble patterns, count-bounded hardware DMC spans and separately bounded bit-pattern remainders. Original sound/instrument names, phrase words, artwork identities, exact generic schemas and some selector domains remain unresolved without blocking mechanical explanations.

C340 is a sound-engine reset wrapper, distinct from the NMI seven-channel update entry. C31F non-speech return reasoning requires request <=$51, valid sound RAM, per-call S >=$B0, NMI off and no reentry. The high busy bit only gates the NMI banked service. Actual callers supplying all those conditions remain unproved. Speech $52..$55 is excluded from that proof because it can overwrite saved bank $07E1. Wrapper queue/saved-bank contracts stay conditional; C459 cursor <=$B4 is the checked local capacity domain.

Concrete preserved redirect frontier from round 3: physical $0052C5..$005386 needs an entry/origin to separate its six-byte prefix from apparent initialization; $01C755..$01C759 needs an alternate entry contract for the apparent STX/STY division setup; $01D0D9..$01D111 needs the $A882 effective-bank/return/queue contract. Later $C3F0 also needs queue capacity below $07E0 and nested-script selector domains. No structural map change or new runtime claim was made during commentary.

## Round 4 current dispositions

The earlier round 3 redirect frontier is resolved structurally by checkpoint round04-disassembly-closure. Physical $0052C5..$0052CB is three side-indexed parameter pairs and the following initializer/row-upload graph is classified code. Alternate arithmetic entries initialize the exact dividend/multiplicand scratch consumed by existing routines. Music command handlers and their local parameter/pointer/counter effects are classified; this supplies no universal stream termination proof.

Direct C34B speech requests one through four have a local, conditional contract: effective switchable physical bank pairs 4/5, 12/13 or 10/11, S >= $74, NMI off and no reentry. Positive step three prevents the final BNE from falling into descriptor-zero data. Descriptor words are a source pointer and remaining-byte count, not an end pointer. The nested C31F speech path still loses its original second bank-save slot and is excluded. Actual natural callers satisfying stack/interrupt conditions remain separate from these local proofs.

D0C9/CE64 local banked setup returns under selector zero/one and valid sprite publication fields. C3F0 selector seven directly sets completion without queue writes. These local contracts settle structural continuations without certifying arbitrary scene state, queue capacity or natural reachability. Non-speech C31F and queue-capacity contracts retained above remain conditional.

The exact current Unknown complement is nine bytes in four half-open physical intervals: $0080A5..$0080A7, $011B4F..$011B50, $0157FD..$0157FF and $01D01F..$01D023. The two $18/$60 pairs could be unrooted carry-clear return stubs or tail data; duplicate endings do not distinguish their roles. The isolated $60 lacks an entry contract. The final four small scalar bytes could be parameters or an unofficial instruction path. Existing neighboring control bypasses them, and bounded consumer/pointer inspection supplies no discriminator. Their exact context/evidence remains local in analysis/redirect/r03-residual-contracts/residual-review.json and analysis/round04/changed-neighbors.json. Do not classify from nonexecution alone.

Physical $001C7F..$001C81 is now Static Data: ActorTargetCheckCadenceMasks. The guarded $9C6E,Y reader and menu selector domain explain $3F/$1F as FrameCounter cadence masks. Selector zero bypasses the read, leaving the preceding RTS Code; the following TXA remains Code. This corrects the earlier no-consumer claim. Natural execution of the read and universal caller-domain validity remain unproved; see analysis/round05/review-notes.md.

All classified source has mechanical explanations, with original sound/phrase/artwork names, full generic-data schemas, some selector domains and actual caller preconditions unresolved nearby. No new execution/natural-route evidence is claimed. Independent residual-origin review belongs to redirect after this phase; final editorial acceptance remains separately pending.
