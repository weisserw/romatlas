# Localized residual review after proposals (not a terminal decision)

Current map remains42934 unknown bytes. If every independently reviewed proposal is accepted,12 bytes would remain. These are actual localized ambiguities, not schemas being withheld for lack of names. Main proposed code/data batch covers the surrounding bytes independently.

| Physical | Bank16:CPU | Byte | Prior supported instruction | Next supported instruction |
|---|---|---|---|---|
| $187A6 | 6:$8796 | $60 | $8791 jmp a:$ad5f | $8797 ldx a:$0413 |
| $1A46D | 6:$A45D | $60 | $A45A jmp a:$b1f8 | $A469 ldy #$00 |
| $1A488 | 6:$A478 | $60 | $A475 jmp a:$b1f8 | $A480 ldx z:$50 |
| $1B488 | 6:$B478 | $60 | $B475 jmp a:$b485 | $B479 lda a:$067e |
| $1B6DF | 6:$B6CF | $60 | $B6CE rts | $B6D0 lda a:$04e0 |
| $1C3E7 | 7:$C3D7 | $FF | $C3D4 jmp a:$c057 | $C3D8 lda z:$8a |
| $1DA44 | 7:$DA34 | $60 | $DA31 jmp a:$c221 | $DA35 lda #$05 |
| $1E390 | 7:$E380 | $60 | $E37D jmp a:$c221 | $E381 dec a:$050c,x |
| $1E3F7 | 7:$E3E7 | $60 | $E3E4 jmp a:$c221 | $E3E8 jsr a:$dfaf |
| $1E421 | 7:$E411 | $60 | $E40E jmp a:$e73f | $E412 lda #$10 |
| $1F2EC | 7:$F2DC | $60 | $F2DB rts | $F2DD lda z:$b3 |
| $1F853 | 7:$F843 | $60 | $F840 jmp a:$c057 | $F849 lda #$00 |

Primary evidence: input/game.nes at these exact offsets; final-proposal-graph.json for adjacent decoded instructions, data-proposal.json for nearby record contexts, and inline-tables.json for every consuming dispatcher found. No proposed branch or call targets these single bytes; they do not have a supported fallthrough from the preceding tail jump or terminating return. The special bank6:$8796 byte is named by repeated pointer-like header words, but no corresponding indirect-control consumer for those header slots has been established. Other header slots are data pointers, so header membership alone does not establish code.

Claim evaluated: annotate each$60 as a one-instruction returning routine, and the$FF as data padding. Decisive limitation: a bare RTS encoding, pointer value or absence of execution is insufficient under evidence-policy.md. Neighboring routine semantics demonstrate exact non-overlap, but do not select code versus an embedded scalar/padding byte. For$FF the neighboring immediate-command gateways prove their own boundaries and the next routine entry; they do not independently prove that this one byte is a reserved fill region. Strongest alternatives are intentional return stubs, a data/sentinel byte or alignment padding. None has a recovered consumer/entry contract that discriminates it.

Methods considered together: all supported direct call/branch successors,34 inline pointer prefixes and their coherent handler graphs, bank-command directories, direct data base references, boundary/record pattern inspection and current observed scenario contexts. These methods add no actual local entry for the residual singleton bytes. Artificially setting PC at an RTS would only test CPU mechanics; a successful return would not prove structural role or natural use. Brute-forcing states is not a useful revised test. A future recovered indirect-control contract or actual instruction fetch would be discriminating; it is currently a missing fact without a concrete targeted route. This does not reject any supported neighboring subinterval.

This is an ordinary ready-proposal handoff, not terminal exhaustion: disassembly must first accept/reject the combined batch and update map/source; final editorial/fidelity gates remain. Reassess any rejection locally against its decisive artifact, and reuse this singleton review only while its premises are unchanged.
