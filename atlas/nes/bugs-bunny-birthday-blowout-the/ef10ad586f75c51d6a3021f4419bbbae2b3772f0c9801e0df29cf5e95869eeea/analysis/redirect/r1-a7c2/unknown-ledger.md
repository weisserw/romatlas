# Unknown complement ledger — redirect round 1 (attempt r1-a7c2)

Source: `analysis.unknown` on analysis/state.json (checkpoint pass-001, map sha256
54a37984...), queried 2026-11-21 in this attempt: 19 ranges, 814 bytes (matches
analysis/evidence/round1-unknown-ranges.json). Physical offsets include the 16-byte
iNES header. Bank = 8 KiB PRG bank; CPU slot from the bank-pair/fixed-bank layout
(pair p = banks 2p/2p+1 at $8000/$A000; 14 at $C000, 15 at $E000).

Decoding: analysis/redirect/r1-a7c2/tools/dis.py (linear 6502 decode of ROM bytes).
Reference search: little-endian word search for every proposed entry and entry-1 over
all PRG (results in findings.md §6); no table or operand reference found except the
listed internal ones.

| # | Physical (incl.) | Len | Bank / CPU | Neighbours | Proposed role | Disposition | Lead |
|---|---|---|---|---|---|---|---|
| 1 | $0BD0E-$0BD1E | 17 | PRG05 $BCFE-$BD0E (slot $A000, pair 2) | after `Type12_Hit` RTS; before `Type12_Shoot` | motion script: 4 x (prevframe, frame, xvel, yvel) + $FF | READY data (Static) | L-007 |
| 2 | $120A7-$120AA | 4 | PRG09 $A097-$A09A (slot $A000, pair 4) | after StoryPage6Effect RTS; falls into `WaitButtonBlinking` $A09B | code `LDA #$00 / STA $58` = blink-counter reset prefix of WaitButtonBlinking | READY code (Static, no caller) | L-009 |
| 3 | $12100-$12102 | 3 | PRG09 $A0F0-$A0F2 | after DrawBlinkingArrow RTS; before DrawRoundMapReveal | metasprite 1x1 header $11, tile $F0, attr $00 (SrcPtr=$A0F0 in DrawBlinkingArrow) | READY data | L-008 |
| 4 | $12C7D-$12C7F | 3 | PRG09 $AC6D-$AC6F | after ClearPanels RTS; before EvaluatePrize | tile-rect-fill record w=2,h=2,tile $FF (SrcPtr=$AC6D -> QueueTileRectFill in ClearPanels) | READY data | L-008 |
| 5 | $13843-$13845 | 3 | PRG09 $B833-$B835 | after DrawBlinkingArrow3 RTS; before ShowLivesAndCarrots | metasprite 1x1 header $11, tile $F5, attr $00 (SrcPtr=$B833 in DrawBlinkingArrow3) | READY data | L-008 |
| 6 | $1684B-$1685B | 17 | PRG11 $A83B-$A84B (slot $A000, pair 5) | after Type00Motion RTS; before ObjType_01 | motion script: 4 records + $FF (Tmp5D=$A83B in Type00Motion) | READY data | L-007 |
| 7 | $1C2BB-$1C2C3 | 9 | PRG14 $C2AB-$C2B3 | after RNG routine RTS; before ScrollScreenTransition | code: JSR ScrollScreenTransition / LDA #$45 / JSR BankCall8000 / RTS | READY code (Static, no caller) | L-010 |
| 8 | $1C44F-$1C450 | 2 | PRG14 $C43F-$C440 | after column-row loop RTS; before PutMetatileInColumn | bytes `01 00`; no reference; code decode (ORA ($00,X)) would clobber the A input of the following routine | RESIDUAL unknown | - |
| 9 | $1C597 | 1 | PRG14 $C587 | after AttributeIndex RTS; before ApplyCameraScroll | lone `$60` | RESIDUAL unknown (policy: unreferenced RTS lacks structure) | - |
| 10 | $1E8BB-$1E8CF | 21 | PRG15 $E8AB-$E8BF | after ClearObjectSlots0500; before Reset $E8C0 | byte-identical copy of $E89A-$E8AA (table tail 5 B + routine 12 B) then $FF x4 | READY split: data 5 + code 12 + data 4 | L-012 (updates L-003) |
| 11 | $1E948-$1E95D | 22 | PRG15 $E938-$E94D | after JMP $E6F4; before InitMirroring $E94E | code: vertical-mirroring twin of InitMirroring | READY code | L-010 |
| 12 | $1EA83-$1EAC5 | 67 | PRG15 $EA73-$EAB5 | after AdvanceInlinePtr; before DrawInlineScreen $EAB6 | code: second-nametable ($2400) twin of DrawInlineScreen | READY code | L-011 |
| 13 | $1EB66-$1EBB6 | 81 | PRG15 $EB56-$EBA6 | after PutScreenByte tail; before ReadInlineByte | code: RLE decoder twin of DecodeRleStream/PutScreenByte using AttrBuffer1 ($0340); called by #12, re-enters live $EB47 | READY code | L-011 |
| 14 | $1F21D-$1F23F | 35 | PRG15 $F20D-$F22F | after scroll routine RTS; before ClearScreenAndShow | code: scroll/nametable setter variant (X from $2A, NT from $2B b0, Y=0) | READY code | L-010 |
| 15 | $1F266-$1F274 | 15 | PRG15 $F256-$F264 | after ClearScreenAndShow; before BeginScreenSetup | code: two entry variants JSR $F265/JSR $F276/JMP $F239 and JSR $F265/JMP $F236 | READY code | L-010 |
| 16 | $1F421-$1F5E8 | 456 | PRG15 $F411-$F5D8 | after FlushQueueIfFull; before FillAttributeBuffers | code $F411-$F5C4 (436 B) VRAM-queue / attribute helpers + data $F5C5-$F5D8 (20 B mask tables read at $F5A4/$F5B9) | READY code+data | L-013 |
| 17 | $1F5F6-$1F609 | 20 | PRG15 $F5E6-$F5F9 | after FillAttributeBuffers; before BeginAttrFill | code: single-buffer variants of FillAttributeBuffers | READY code | L-010 |
| 18 | $1F63B-$1F644 | 10 | PRG15 $F62B-$F634 | after RestoreXY; before RequestAttrUploads | code: LDA #$20/JMP RequestPpuUpdate; LDA #$10/JMP RequestPpuUpdate (single-buffer siblings of RequestAttrUploads #$30) | READY code | L-010 |
| 19 | $1FFEE-$20009 | 28 | PRG15 $FFDE-$FFF9 | after EnableWram $FFD2; before vectors $FFFA | code $FFDE-$FFF5 (24 B: two WRAM-mode twins of EnableWram, $C0 and $40) + data $FFF6-$FFF9 (4 B = two copies of word $ED9F, the NMI vector value) | READY code+data | L-010 |

Totals: READY 811 bytes (data 34+9+5+4+20+4 = 76; code 735), RESIDUAL 3 bytes (#8, #9).
Reachability: no static or observed caller for any READY code (structural claim only).
