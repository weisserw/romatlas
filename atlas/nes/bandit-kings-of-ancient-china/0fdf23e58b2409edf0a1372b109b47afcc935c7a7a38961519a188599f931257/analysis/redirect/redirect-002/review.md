# Redirect-002: localized structural review

Input pass-002:393174 classified/393232 total,58 unknown in13 ranges. Current-source
readability gate complete per commentary-pass-002. This is research only; no ROM,
map,state,source or toolkit mutation. Primary byte/neighbor evidence: primary.json,
reproduced by `python3 analysis/redirect/redirect-002/inspect.py`. Disk6.6GiB free;
shared scratch4KiB; no cleanup. Runtime unchanged MMC5 obstacle; no retry.

## Changed premises and discriminating checks

Earlier rejection text was retrieved from redirect-001/gap-layout-review.md and
assets.py, not inherited as a veto. Tested claims are structural; none claims a
natural caller, original generator or observed execution. Independent avenues:
prefix object boundary/template comparison, final numeric-object localization,
BRK/RTI continuation contract, and no-op family versus isolated return.

### Nine four-byte bank prefixes: propose generic Data,36 bytes

Physical139280,147472,155664,163856,172048,180240,188432,196624,221200,
length4 each; banks17..24,27 offset0. Entire bytes4C000000 are identical. Every
following+4 boundary begins independently accepted non-code payload. Comparison
against all32 bank starts shows functional native headers have nonzero supported
JMP destinations followed by known VM/native frames; these nine are the same
zero-destination marker before heterogeneous record/grid/text payloads. They form
a repeated bounded non-code prefix pattern, not a claim about a specific header
schema or consumer. Bank20 native procedures begin at+600, after its accepted data
prefix: that demonstrates mixed payloads, but does not contradict the observed
four-byte prefix pattern. Prior inference incorrectly propagated a bank-wide
counterexample to this independent narrow prefix claim.

Strongest alternative: a three-byte absolute JMP0000 plus a fourth byte of data.
This is a real legal decode. Unlike functional bank entry stubs, it supplies no
meaningful native entry relation; the identical zeroed four-byte pattern and
independently established following object boundaries favor generic stored data.
This is a pattern-based static inference under policy, not proof that0000 cannot
execute in some hypothetical RAM state. No original fill or invalid-entry meaning
is asserted. CPU aliases unresolved; no execution view required for Data.

### Bank23 special fourteen bytes: propose generic Data,14 bytes

Physical193026:193040,bank23 offsets11F2:1200. Complete object:
2C 00 4C 04 0C B0 04 01 00 0F 05 00 00 00.
Neighbor primary dump includes last seven accepted18-byte numeric records and
following FF block(122 bytes). Specific18-byte record proof remains rejected:
zero-column2 fails, and18-byte length crosses independently established FF suffix.
That negative result is confined to the schema. It does not establish native code.
Generic Data instead uses exact bounded numeric/zero-containing transition object,
its position immediately after255 records, and independent trailing FF boundary.
An interior4C040C at+2 could decode JMP0C04, but arbitrary interior legal decoding
without a supported entry/control contract is no stronger than the similarly
embedded4C in already accepted preceding numeric records (record249). Starting
at+0 instead decodes BIT4C00, unofficial NOP0C, BCS+4, ORA(00,X), SLO0005,
BRK,BRK: this lacks a coherent native routine/stack contract. No precise numeric
format or shared stride with preceding records is proposed. Unknown semantics
remain, separately from the generic non-code inference.

### Fixed EBAC signature: propose Data,1 byte

Physical256956,bank31+BAC,CPU EBAC under bank31 E000. Accepted dispatcher trap
EBAB is BRK; IRQ vector points FFD6, whose accepted handler is immediate RTI.
BRK's architectural return address is PC+2=EBAD; EBAD is independently accepted
JMP E46E. Therefore EBAC is the inline BRK signature/padding byte, skipped by the
supported trap/return control contract. Its value00 is immaterial. The prior
'isolated00 without consumer' rationale omitted this independent architectural
constraint. Annotate one-byte Data with this mechanical role; no observed fetch
or original meaningful signature value claimed. Source lines85509ff,88301ff.

### Fixed F9F2:F9F8: propose six singleton no-op Code entries,6 bytes

Physical260610:260616,bank31 offsets19F2:19F8; context[null,null,null,31].
Exact six60 bytes immediately continue accepted singleton RTS NmiNoOp at F9F1;
the next independent native procedure starts F9F8. The seven-byte homogeneous
return family lies between palette-upload routine's return F9F0 and F9F8's
argument-driven mapper/PPU address routine. Each singleton supplies the same
complete calling contract as accepted F9F1: ordinary6502 JSR return frame on
stack; RTS returns to caller+1, preserves registers/flags and changes only stack
consumption/PC. This is a bounded empty-procedure family inferred from the complete
uniform sequence and accepted neighboring empty-hook/native contracts, not merely
six arbitrary legal decodes. Entry addresses may be listed individually; callers
and original procedure grouping remain unobserved.

Strongest alternative: RTS-valued padding after one hook. That is observationally
compatible without historical source. The proposal concerns static executable
return contracts for these exact bytes, not proving original compiler functions.
Independent review should judge whether family/neighbor evidence suffices; it is
weaker than the BRK contract and should not be bundled inseparably with Data claims.
Literal origin search found no PRG wordsF9F2,F9F3,F9F4,F9F6,F9F7; F9F5 occurs as
adjacent low-dispatch bytes, not a pointer. Lack of callers is not the rejection
criterion and no caller prerequisite is imposed.

### Detached C0E9: retain Unknown,1 byte

Physical246009,bank30+E9,CPU C0E9,context[null,null,30,31],byte60. Preceding
FindTenByteKeyRecord routine already ends RTS C0E8; following C0EA is JMP(000C)
with independently supported native-operand tail-call contract. Unlike F9F1:F9F8,
this is only one duplicate terminal return, with no independently identifiable
empty-hook family. Isolated legal RTS with hypothetical JSR stack is insufficient;
Data also lacks a pattern/extent proof. Strongest alternative is empty native
procedure, competing with redundant terminal/fill byte. Raw little-endian C0E9
occurrences are VM Data and do not establish native origins. Retaining this byte
is localized, not a veto on the longer repeated family above. No useful new
runtime test exists on the configured MMC5-unsupported debugger; arbitrary RAM
execution would not discriminate original role. No terminal exhaustion claimed:
independent disassembly review of the combined supported batch is next.

## Continuations and scope

The trap continuation EBAD and IRQ RTI are already accepted; no novel downstream
bytes. Prefix payloads and neighboring numeric records/FF extents already accepted;
no missing count/selector is needed for generic Data. Source's unresolved game
meanings, dynamic bank mappings and generated/generic labels were checked against
these narrow objects; they provide no new structural target outside the exact
complement. Existing VM/native graphs and source clarify that literal pointer
occurrences can be VM operands or unrelated split-table adjacency. Broad gameplay
milestones remain unavailable in scenarios.json due to mapper5; no unexecuted RAM
patch is presented as a visit. No further concrete continuation was artificially
split into a new round. Proposed57 bytes remain unaccepted; actual58 unknown
unchanged. A separate review may accept a strict subset. No terminal decision is
requested before that independent structural assessment.
