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
