# Redirect r02-a1 working evidence (input checkpoint pass-002)

Input: maps/current.json == maps/pass-002.json == analysis/state.json (sha256 9c36a833...).
1,691 unknown bytes in 99 runs (unknown_runs_pass002.txt, from unk.py). Research only: no
map/state/source changes. Proposal batch: proposals.json (built and checked by
build_proposals.py: no overlap with classified bytes, no overlap among proposals).

## Tools (read-only, rerunnable)
- unk.py: exact unknown complement from maps/current.json.
- refs2.py -> refs_pass002.txt: per run, operand references from classified code, raw words, immediate pairs.
- prevnext.py -> prevnext_pass002.txt: preceding classified instruction, linear decode, next kind.
- nb.py <addr..>: neighbouring regions with evidence strings.
- motion.py: decodes the 18 StepBgObjScript pointers (4-byte records, single 0 byte ends).
- codecheck.py + code_fragments.json: official opcodes, exact end alignment, control targets on classified starts.
- build_proposals.py -> proposals.json: the combined batch and the residual list.

## Key findings
1. Motion scripts (L-020 -> L-023): every pointer decodes to records ending with a 0 duration
   byte exactly at the run end. New: $F32E-$F346 is BgObj08's StageId==0 script (ldx #$2E /
   lda #$F3 at $F25E, beq on StageId); $EFA6-$EFAE has the same format but no pointer (generic).
   $EFE5 ($60 after always-taken bne at $EFE3) stays unknown.
2. Base-1 tables: many runs are tables whose consumer base is the last byte of the preceding
   terminal instruction (lda base,y with index >= 1). Pass-2's gap matcher attributed these
   reads to the code byte, so the runs were missed. Extents proven where independent:
   - b6 $8042-$809C: DrawAllSprites lda $8041,y, Y = ObjType ($00-$5B, L-010) -> exactly 91
     bytes; non-zero values $03..$24 (multiples of 3) all select jmp slots $80AD-$80CE.
     Type $4F -> $24 -> $80CE DrawFrameAndClip (previously "no known caller").
   - b6 $9746-$974F: next-stage ($973F,y) and $0682 ($9744,y) tables, Y = StageId $07-$0B.
     Values: 7->$0B, 8->$09, 9->$0A, $0A->$00, $0B->$00. $9745 ($60) is index 6, excluded by cpy #$07.
   - b0 $A2DC-$A2E7: three tables $A29C/$A2A0/$A2A4,y with Y = $40-$43 (cmp #$40 / bcc).
   - b0 $E86A-$E872: ldy #$1E / iny / cmp $E84B,y ascending search (06 46 66 B3 FF) and
     lda $E850,y for the same Y; $E867-$E869 (jmp $EE43) is never read as data.
   - b4 $B933-$B946: ldx $B8C3,y with ldy #$70 ... cpy #$84. This also corrects the L-021
     premise for $B8BD-$B8D0 (the table read never touches it): $B8BE-$B8D0 is a coherent
     camera-step routine joining $B8D5.
   - b4 $B97A-$B986: RAM-routine template copied with Y 0..$0C (cpy #$0D) to $0600+.
   - b2 $FC09-$FC0A: per-player target X ($60/$A0) read by cmp $FC09,x with X = player.
3. b6 $EEE0-$EF1A: DecodeGfxToPpu maps text byte & $7F through $EEC0,x; chars $20-$5A = exactly
   the run (A-Z -> $E6-$FF, 0-9 -> $D5-$DE). Bank 6 text is ASCII with bit-7 end flags.
4. Bank 5 $BB49-$BC46/$BC67-$BCD6/$C90F-$C916 lie inside stage maps (bases $BBCF stage $08 and
   $C8CF stage $03 from L-017's $A4D5/$A4E2 table) between classified zero-fill blocks.
5. Bank 2 $FC94-$FD33: pre-shifted 16-bit bit patterns (0C1F, 193E, 337C, 66F8 ...).
6. Stub tails (L-006): $FF6E-$FFA5 is one module copied into every bank; all unknown copy bytes
   match the bank-0 template except the jsr/jmp operands ($FF72/3, $FF7C/D, $FF8F/90, $FFA4/5).
   Module A ($FF6E, sta $FFB6 -> bank 3) and C ($FF84, ora #$03 -> bank 3) continue only in bank 3;
   module B's tail continues in the selected bank and its rts in bank 0; the NMI stub's jmp at
   $FFA3 runs only in bank 0 (stx $FFB3 with X = 0). Per-bank operands in banks 1,2,4,5,6,7
   point at the end of each bank's used area (fill), so those copies are not executed there.
7. Bank 5 $FF50-$FF6B: bank-switch snippets with bus-conflict-consistent immediates (lda #5 /
   sta $FFB8, ldy #6 / sty $FFB9), jmp $A12A (classified bank-5 label); no other bank has
   code at the continuation addresses: leftover code.

## Residual after the batch (14 bytes)
- b0 $8D92 ($60 after beq $8D6C / bne $8D7B), b5 $ACFA ($60 after jmp $AFB9),
  b4 $B979 ($60 after jmp $B987, before the RAM template), b6 $9745 ($60 after jmp, index 6 of
  the stage table that Y >= 7 never reads), b6 $EFE5 ($60 after always-taken bne): a lone,
  unreferenced $60 is equally a dead rts or an unread table/pad byte; no consumer, no branch.
- b6 $FE24-$FE2C: jsr $FE19 / stx OAMADDR / lda #$02 then $8D whose operand would overlap the
  classified ldx #$00 at $FE2D (sta $00A2 if entered at $FE2C). Either a patched leftover or
  an overlapping-instruction entry; no caller. Code-vs-data and bounds stay ambiguous.

## Runtime (bounded; server started/stopped by this attempt)
- ending.py (artificial: $0683=$FF while music played): no exit ($06A4 stays 0).
- ending2.py (no RAM edits from the ending snapshot): music flag $0301 reached 0 after ~900
  frames; Start -> StageId $00 within 60 frames -> Licensed by Nintendo -> title. No credits roll.
- twoplayer*.py: 2 PLAYER GAME A ($16 = 1) -> StageId $03 with both players; snapshot
  traces/r02a1_snap_2p_gameA.json; 240 traced frames: 3,277 ROM opcode addresses, 0 in unknown runs.
