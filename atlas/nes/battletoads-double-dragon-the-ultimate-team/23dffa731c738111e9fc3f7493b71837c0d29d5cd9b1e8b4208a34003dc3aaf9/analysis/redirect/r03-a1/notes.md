# r03-a1 redirect notes (input checkpoint pass-003, 16 unknown bytes, all L-027)

Research only; ROM, map, state and source unchanged. Address context: AxROM 32 KiB banks,
physical = 16 + bank*0x8000 + (cpu - $8000); prg_banks [4b..4b+3].

## 1. b0 $E536-$E537 (A9 02): frame-map tail vs `lda #$02` entry prefix to $E538

Prior barrier (round-03 disassembly): PlayerFrameMapA ($E4EB,y, Y < $5B) / PlayerFrameMapB
($E4F5,y) could index $E536 at Y=$4B/$41; frame domain unproven.

Retry record: target claim = role of $E536-$E537. Changed facts: (a) static linked-state
frame writer found; (b) consumer exercised for the first time (artificial latch). Expected
discriminator: whether the consumer index window is a table extent at all. Stop condition:
one bounded run per player/selector value.

Evidence:
- Static: player code $B8F8-$B91D (bank 0) runs when Obj_0539 bit 6 is set (the link value
  slot+$41 written by TryLatchOntoPlayer) and stores player frame $39, $3B or $3A via
  $B973 -> $BA3F (sta $03B6,x).
- Spawner: the only immediate ObjType $3A store is ObjType55_Rider (`lda #$3A / sta ObjType,x` at $F047) and it sets
  Obj_0575 = $A1, which makes ObjType3A skip the latch path (table-driven type stores, e.g. the carried-item type Obj_05B1+8, were not bounded). No type $3A/$55 record in the
  bank-3 spawn lists (spawn3a.py / spawn_types.txt; the list walker is approximate for
  StageId 6). Natural reachability of TryLatchOntoPlayer/MirrorPlayerAppearance stays
  unresolved; the experiment below is artificial.
- Artificial run (latch.py; snapshot traces/r02a1_snap_2p_gameA.json; patch: object slot 6
  := ObjType $3A, Obj_0575/052A/0539/0476/04A3/04FD/0548 = 0, ObjFrame $9A, position copied
  from the target player; 60 x 10-frame input cycles on that player's pad):
  * P1 ($05F0=0, $05F2=0): linked 600/600 frames; player frames $39 x205, $3A x65,
    $3B x37, $5B x292 (plus $08 once, the pre-link frame). Y at `lda PlayerFrameMapA,y`
    ($E4F7, traced sample): $39, $3B, $5B. (latch_p1_60.txt)
  * P2 ($05F0=3, $05F3=1, PlayerFrameMapB path): player $03B6 after the call = $97/$98/
    $99/$9A = B[$39]/($5B->$98)/B[$3A]/B[$3B]; Y at $E4F7 and $E511: $39, $5B.
    (latch_p2_60.txt)
  * Instruction events at $E4F7 (`lda PlayerFrameMapA,y`) with register Y = $5B (both
    runs) mean the consumer reads $E4EB+$5B = $E546, the operand byte of the classified
    `lda $04FD,y` at $E544 inside TryLatchOntoPlayer; the value is discarded by
    cpy #$5B / lda #$9C. Y = $39 reads $E524 (A[$39], data). The Read-event dump
    (latch_p1_60_readtrace.txt) mixes operand fetches with data reads, so it is not used
    for the $E546 claim.
- Consequences: the meaningful observed entries are $E524-$E526 (A) and $E52E-$E530 (B),
  inside the accepted 20-byte data block $E522-$E535. The consumer's index window is not a
  table extent (it is observed reading code at $E546 every linked frame with Y=$5B), so a
  read of $E536 would not have been evidence for data either. The frame-map-tail
  hypothesis therefore has no remaining support; it is not falsified for unobserved frames.
- Code hypothesis: `lda #$02` immediately before TryLatchOntoPlayer, whose first instruction
  is `sta $D1` with the documented entry contract A = hit-box id (the only caller does
  `lda #$03 / jsr $E538`). The prefix makes an alternate entry "TryLatchOntoPlayer with
  id 2" (same pattern as the accepted L-026 tier-B prefixes, e.g. b0 $EEED lda #$6F into
  $EEEF). Values $A9/$02 are outside the frame-id set of the data block
  {09,0A,0B,3E,4F,97,99-9D} (weak, values only).
- Proposal: Code, Static, reachability unresolved (no caller), tier B (reviewer call).
  Limit: the linked frame domain is a sample plus one static writer; other player states
  writing $41/$42/$4B/$4C while linked are not excluded by exhaustive proof.

## 2. b6 $FE24-$FE2C (20 19 FE 8E 03 20 A9 02 8D)

Changed inference: idiom match. The bytes are `jsr HideSprite($FE19) / stx OAMADDR /
lda #$02 / $8D`, i.e. the first 9 bytes of the sprite-clear + OAM DMA idiom whose complete
form `8E 03 20 A9 02 8D 14 40` exists in bank 0 Nmi_Common ($FE0A-...) and bank 4 $C5F5
(`...inx/bne; stx OAMADDR; lda #$02; sta OAMDMA; rts`, directly after an identical
sprite-clear loop). In bank 6 the jsr target is the immediately preceding HideSprite
routine. The operand bytes `14 40` (+ `60`) are absent: RunRamColumnCode starts at $FE2D.
- $FE24-$FE2B (8 bytes, 3 complete instructions): code under both competing hypotheses
  (truncated leftover routine, or overlapping alternate entry). Proposal: Code, Static,
  uncalled (tier A).
- $FE2C ($8D): under the overlapping-entry hypothesis it is `sta $00A2` using the
  `A2 00` of `ldx #$00` at $FE2D; $A2 is one of the $9F-$A3 PPU-address bytes read by
  RunRamColumnCode (lda $009F,y), so storing 2 there is semantically arbitrary, while the
  idiom match says the intended operand was $4014. Best explanation: truncated opcode
  remnant. A map region cannot hold the overlapping 3-byte instruction. Proposal: generic
  Data (truncated opcode remnant), tier B; reviewer may keep it unknown.

## 3. Lone $60 bytes (b0 $8D92, b4 $B979, b5 $ACFA, b6 $9745, b6 $EFE5)

Re-reviewed against primary bytes and source (no runtime value):
- $9745: narrowed. The "unused index-6 table entry" alternative is inconsistent: the second
  StageNextTables table is read at base $9744 (+Y 7..$0B -> $974B-$974F), so its index 6
  would be $974A = the first table's index $0B entry; the packed tables have no index-6
  slot. Remaining alternatives: dead rts after `jmp LoaderCommonTail` vs pad byte.
- $B979: copy loop reads `lda $B97A,y` Y 0..$0C only, so the template does not include it.
- $8D92, $ACFA, $EFE5: no new reference or consumer.
For all five, remaining alternatives are dead `rts` (code) vs one unread pad byte (data);
both neighbours fixed by code/data on each side do not discriminate; no reader, no
reference, no runtime path. Policy: an unreferenced return opcode lacks structure on its
own. Kept blocked.

## 4. Gameplay
- 2P Game B (artificial: $16 := 2 on the 2P Game A snapshot, P2 moved onto P1, 30 x 20-frame
  attack inputs on both pads; gameb.py): 171 opcode addresses executed only with $16 = 2
  (bank 0 $848F-$84A3, $8CBD, $93FF, $A423-$A46E, $A5A5, $A61F-$A63F, $A6C7-$A719, ...);
  no unknown byte executed. $16 is set to 2 by the Game B menu path (InitTwoPlayerDefaults
  shows $16=2 as a preset); this is an artificial visit, not a menu route.
- Clinging enemy latch (section 1): artificial.
