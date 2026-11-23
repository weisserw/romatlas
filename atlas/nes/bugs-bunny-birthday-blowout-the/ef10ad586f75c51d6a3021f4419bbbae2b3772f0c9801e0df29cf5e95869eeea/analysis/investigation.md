# Investigation ledger

## Round 1 disassembly (checkpoint pass-001, 2026-11-21)

### Method
- Toolkit static discovery is ineffective on this MMC3 ROM (TK-004), so a bank-aware
  recursive-descent script was used (analysis/evidence/round1-tools/sa.py, full.py),
  rooted at the validated vectors (NMI $ED9F, RESET $E8C0, IRQ $EC21, bank 15) and all
  observed opcode fetches. Every resulting region was annotated through `map.annotate`
  and validated by the toolkit decoder; checkpoint pass-001 rebuilds byte-identically.
- Bank model (confirmed by observed contexts): MMC3 PRG mode 0; banks 14/15 fixed at
  $C000/$E000; $8000/$A000 always hold a pair (2p, 2p+1) selected by $E964 (A=p).
- Supported entry contracts used:
  - Bank-call dispatchers $F780/$F790/$F7A0/$F7B0/$F7C0 (A = pair<<4 | index) ->
    $ECE4/$ED08/$ED2C/$ED50/$ED74 -> pointer tables $8000/$8020/$BE00/$A980/$9CF0 of
    pair p -> JMP ($0010). Every call site uses `LDA #imm` (checked); table extents
    bounded by entry-0 code adjacency or the 16-entry mask. Unused entries p4#5 $A3EF,
    p6#8 $9FC8, p2#2 $B192 were added as roots because they lie inside bounded tables.
  - Inline jump tables after `JSR $ECC7` / `JSR $FEBF` (pull return address, index*2):
    25 tables; extent = up to lowest in-window target (adjacent handler code); 5
    tables fixed by established code boundaries ($991E:9, $AB35:3, $BC8E:4, $BCA1:4, $B2FA:2).
  - Keyed handler table $B1BB (bank 5): (key, word) records terminated by $FF, used by
    $B19C-$B1B8 (JMP ($0075)).
  - Inline-parameter calls: `JSR $E9F7` + 8 CHR-bank bytes (returns via JMP ($0037));
    `JSR $EAB6` + RLE VRAM stream (FF c n / FF FF d / FF FF 00), never returns to caller.
    All 15 parsed streams end exactly where the next code begins (independent check).
- Consistency checks: 0 unofficial/KIL/BRK opcodes on any accepted path, 0 overlapping
  instruction boundaries, every observed fetch (10192 unique) already inside the static set.

### Results
- Code 40561 bytes (21041 in all-observed regions, 19520 static-only; 52.7% of static
  instructions observed executed). Structured data: 25 jump tables, 6 dispatcher tables,
  15 E9F7 blocks, 15 EAB6 streams, 1 keyed table, CPU vectors.
- Generic data (Static, unlabeled) for: whole banks 0,1,2,3,4,6; and gaps meeting one of:
  invalid/unofficial-opcode ratio >= 0.20 in linear decode with no JSR/JMP to any known
  routine; or read by static data operands (abs/abs,X/abs,Y) with ratio >= 0.10 or <= 32
  bytes; or manual inspection (palette bytes $BF34 b13, word list $DFC9, bank-end $FF fills,
  records $BF3B b9). Checked every generic block for JSR/JMP into fixed-bank instruction
  starts (all hits sit inside 6-byte records) and for 10-byte duplicates of live code
  (only $E8AB found; excluded -> unknown).
- Unknown 814 bytes = leads L-001 (orphan fixed-bank routines, 746 B), L-002 (small gaps,
  47 B), L-003 (stale duplicate, 21 B). See analysis/evidence/round1-unknown-ranges.json.

### Reachability notes (separate from structure)
- Observed: title, intro, all six stages (rounds 1-4 of stages 1-2, rounds 1 and 4 of
  stages 3-6), round clear + bonus board, ending/The End (RAM shortcuts, artificial).
- Not observed (static only): bosses, bonus game, game over, many object handlers.
- Generic data is not claimed unused; consumers are mostly unresolved.

### Rejected/abandoned in round 1
- Writing $043F=$80 to clear a round: overwritten by INC $043F at $C6E3 (abandoned).
- $043F=$04: door/scroll transition, led to death (abandoned; $08 works).
- $0422 as carrot counter: HUD unchanged (unconfirmed, L-004).

## Round 1 redirect (attempt r1-a7c2, input pass-001 + commentary round 1, 2026-11-21)

Records: analysis/redirect/r1-a7c2/ (unknown-ledger.md, evidence.json, tools/dis.py,
coverage/screens). Handoff: analysis/handoffs/round-1-redirect.md.

- Unknown complement re-queried (analysis.unknown): 19 ranges / 814 B, unchanged.
- Proposed (ready) 811 B: L-007 motion scripts 34 B data; L-008 bank-9 graphics
  headers 9 B data; L-009 $A097 prefix 4 B code; L-010 fixed-bank sibling helpers
  135 B code + 4 B data; L-011 $EA73/$EB56 twins 148 B code; L-012 $E8AB split
  (9 data + 12 code); L-013 $F411 block 436 code + 20 data. L-014: $A024-$A025
  code->data correction (0 net bytes).
- Method change vs round 1: L-001/L-003 were blocked on "no reference"; this pass
  argues structure from sibling contracts, byte identity with live routines,
  boundary-exact jumps into live code and consumer-anchored table extents
  (prompt: reachability is a separate claim). L-003 split by source role.
- Negative results: no word / word-1 / computed BankCall8000 reference to any
  proposed entry; pair 6 entry 8 ($9FC8 stage select) has no caller (35 immediate
  selectors, none $68).
- Residual unknown 3 B: $C43F-$C440 (`01 00`), $C587 (`60`) — reasons in evidence.json.
- Gameplay: bonus game visited ($042A carrots, L-004 verified); game over visited;
  boss deferred (PRG-RAM not writable, TK-006; teleport broke the scene).

## Round 2 disassembly (input pass-001 state, output checkpoint pass-002, 2026-11-21)

Method: independent review of ready leads L-007..L-014 against ROM bytes (linear decode
analysis/redirect/r1-a7c2/tools/dis.py, consumer/neighbour disassembly, byte diffs of
twins, reference searches), then map.annotate (analysis/evidence/round2-tools/apply_r2.py).
All provenance Static; none labelled Observed or claimed reachable/unused.

| Lead | Outcome | Independent checks this pass |
|---|---|---|
| L-007 | verified, 34 B data | Consumer $D8FA reads (5D),y y=0..3, CMP #$FF terminator, +4 stride; producers set $5D/$5E=$BCFE / $A83B immediately before JSR $D8FA; $BD0F is a BPL target (code start). |
| L-008 | verified, 9 B data | SrcPtr set to $A0F0/$B833 before JSR DrawMetasprite $C114 and $AC6D before QueueTileRectFill; each follows an RTS; no other reference. |
| L-009 | verified (conf 0.7), 4 B code | Falls into $A09B, which bank 8 calls at $9A1D/$9A61 (JSR $A09B) without touching $58; $58 is read (AND #$20) and INC'd by $A0BA. No reads of $A097 in pair-4 context (word hits are unrelated `85 97` ZP stores). Accepted as an alternate-entry prefix; lowest confidence of the batch. |
| L-010 | verified, 135 B code + 4 B data | Decoded each block: $C2AB, $E938 (twin of $E94E, #$00/#$47), $F20D (variant of $F1E0 setter), $F256/$F25F (JMP $F239/$F236 = live instruction starts), $F5E6/$F5F0, $F62B/$F630 (same JMP $EC8A list as live #$30/#$04/#$18), $FFDE/$FFEA (twins of $FFD2). $FFF6 `9f ed 9f ed` = NMI word copies, AHX only -> data. |
| L-011 | verified, 148 B code | Byte diff: $EA73 vs $EAB6 differs only at $EA90 (#$24) and JSR target; $EB56 vs $EAF9 differs only at JSR targets, branch offsets and STA $0340 (vs $0300), plus JMP $EB47. |
| L-012 | verified, 5+4 B data, 12 B code (conf 0.75) | $E8AB-$E8BB identical to $E89A-$E8AA; copy's BCC targets its own $E8B3; $FF x4 ends at Reset $E8C0. |
| L-013 | verified, 436 B code + 20 B data | Full decode $F411-$F5C4: RTS-terminated routines, closed internal call graph, JMP $F394 into live code; Y index for $F5C5 is ($11 0-3)<<2 | 2 bits of $22 -> 0..15; Y for $F5D5 is $11 (0-3); tables end at live $F5D9. Split into AttrPaletteBits (16) + AttrClearMasks (4). |
| L-014 | verified (0 net) | $A020 LDY #1/BNE always taken; $A01C LDY #0/BEQ always taken; CMP $A024,y at $9FFC; region 1438 shortened to 92 B, data region StageSelectLimits added. |

Residual review (reuses redirect r1-a7c2 evidence.json residuals, plus a new check):
- $C43F-$C440 `01 00` (PRG14). Claim tested: code or data. Added check: no immediate
  pointer pair (LDA #$3E/$3F/$40 with #$C4 within 12 bytes) or word $C43E-$C440 anywhere in PRG.
  Rules out a pointer-based consumer in ordinary form; the code-prefix reading still clobbers
  A (input of PutMetatileInColumn). Strongest competitor: generic 2-byte data. Remaining
  reason: no consumer, no format, not a uniform/non-instruction pattern -> stays unknown.
- $C587 `60`: no word $C586/$C587 or immediate pair; unreferenced RTS lacks structure
  (evidence-policy.md) -> stays unknown.
Revisit only if a consumer/caller of $C43F or $C587 appears (e.g. new traced gameplay).

Gameplay: S8 boss re-attempted with a materially different method (ordinary play with
health refills instead of teleport, per redirect recipe): stage 6 round 4, right+jump/mallet
for 25x60 frames, $0428=$0C/$0429=3 each chunk -> stuck at a wall at PlayerX 467 / CameraX
$0100 (conveyor-platform route needs real platforming). No boss. Deferred; value is coverage
only (boss code already classified statically).
