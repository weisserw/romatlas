# r001-a7k2 — bank-7 unknown complement: static structure review

Input checkpoint: commentary-001 (maps/commentary-001.json; classification identical to
pass-001). Unknown complement: 1436 bytes, 24 blocks, all fixed PRG bank 7
(physical bank 7 = 8 KiB banks 14/15, CPU $C000-$FFFF, always mapped; no bank
ambiguity for any range below). File offset = CPU - $C000 + $1C010.

Tools (local, ROM-free output): tools/dis7.py (bank-7 linear decoder with commentary
labels), tools/refsearch.py (jsr/jmp/LE-word/immediate-pair reference search over all
PRG). Commands: `python3 tools/dis7.py C37B C3CB`, `python3 tools/refsearch.py C38E ...`.

## Method

1. Linear decode of every unknown block with two bytes of context on each side.
2. Reference search for every candidate entry: jsr/jmp abs, LE words anywhere, split
   `lda/ldx/ldy #lo .. #hi` pairs (gap <= 12), and rts-trick words (entry-1) in banks 6/7.
   Word hits inside lake-data banks 1-4 / bank-0 map data are byte coincidences in
   classified generic data (no consumer); they are not counted as references.
3. Contract check: RAM symbols vs the commentary symbol table, sibling routines with
   identical idioms, exact terminal instruction at the block end / fallthrough into a
   classified routine, inline-data barriers, table index domains.
4. Prior inspection reused: commentary decode notes on each block (game.asm block
   headers) and the pass-001 raw word search.

## NEW REFERENCE (missed by earlier passes)

`$C38E` and `$C39B` ARE referenced: classified code at $C1C2 (`lda #$8E / sta Ptr96 /
lda #$C3 / sta Ptr96+1 / jsr Expand1bppList`, game.asm ~l.6373) and $C24A (same with
#$9B, ~l.6438). Expand1bppList consumes 6-byte records (PPU addr, src, mode, count)
until a $FF lead byte. Parsing from both pointers:
- $C38E: 2 records + $FF at $C39A -> list ends exactly at $C39A (13 bytes).
- $C39B: 8 records + $FF at $C3CB -> list ends exactly at $C3CB, the byte before
  GenerateFish ($C3CC, classified code).  (49 bytes)
Both lists are contiguous, terminator-bounded and end flush with the unknown block.
The earlier pass-001 search looked for LE words only, hence the miss.

## Per-block disposition (proposal for disassembly review; reachability separate)

| Range | Bytes | Proposed role | Key evidence |
|---|---|---|---|
| $C040-$C042 | 3 | code (Static), soft-reset stub `jmp ($FFFC)` | complete 3-byte instruction filling exactly to ResetMain $C043; sits in the aligned vector block ($C020 jmp, $C023 jmp, $C030-$C039 jmp slots, zero pad $C03C-$C03F); identical idiom at bank-6 GoHome `jmp (Vectors+2)`. No reference found (weak-moderate; disassembly may keep unknown) |
| $C37B-$C38D | 19 | code (Static) | byte-identical copy of UploadPalettes $E5E0-$E5F2 (RequestPaletteUpload 1/2 + wait on PalUploadPending), rts at $C38D; preceded by rts $C37A |
| $C38E-$C39A | 13 | data: Expand1bppList record list (consumer-backed) | referenced at $C1C2; $FF terminator at $C39A |
| $C39B-$C3CB | 49 | data: Expand1bppList record list (consumer-backed) | referenced at $C24A; $FF terminator at $C3CB |
| $C8F0-$C8F7 | 8 | code (Static), alternate entry of DrawObject | `lda #1 / sta $0317 / jmp $C900`; DrawObject $C8F8 does `lda #0 / sta $0317 / sta $0318` then falls into $C900 -> flag-variant entry sharing the body |
| $D76C | 1 | dead `rts` after `jmp FishWander` (see L-002 note) | 11 classified `jmp;rts` pairs exist, all with branch-targeted rts; this one is untargeted -> source-style epilogue; code vs data for a lone opcode remains weak |
| $DB90-$DBC5 | 54 | code (Static), signed 16/8 divide wrapper | jsr AbsMathA/Divide16By8/NegateMathA, sign via $98/$99 xor; last path `bpl $DBBF` else falls into NegateMathA $DBC6 (classified) |
| $DF57-$DF5A | 4 | code (Static) | `jsr ClampFishVelocity / rts`, after rts $DF56; FightPlayerInput starts $DF5B |
| $E24A | 1 | dead `rts` after `jmp PlaySfxTracked` | as $D76C |
| $E30F | 1 | dead second `rts` (after rts $E30E) | as $D76C |
| $ECB3-$ECC2 | 16 | code (Static): Inc16 MathA ($ECB3-$ECB9), Dec16 MathA ($ECBA-$ECC2) | two complete rts routines on MathA ($0D/$0E); NumberToDigits at $ECC3 |
| $ED08-$ED50 | 73 | code (Static), decimal-string -> 16-bit parser | uses MathA/MathB/DigitPtr ($15) and Multiply16 $EC46; preserves X; rts $ED50; Expand1bppTiles $ED51 |
| $EDD2-$EE78 | 167 | code (Static), inline-argument Expand1bppTiles variant + 3 helpers ($EE0A, $EE2F, $EE5D, $EE6B) | pla/pla return-pointer idiom (as CopyTilesInline $F06C), 6 inline bytes -> Exp1bppPpu..Count ($47-$4C), `jmp ($004D)` past args; internal jsr targets all inside block; ForceBlankBegin/End; rts $EE78 |
| $EF32-$EF98 | 103 | code (Static), NMI-style wrapping strip writer | job slot $35-$3A (gap between DlListPtr $33 and VramStripPending $3B; no live user) handles 32-column wrap to the other name table; mirrors NmiStripUpload $EFEA |
| $EF99-$EFE9 | 81 | code (Static), scroll+pixel -> name-table address | ScrollX/ScrollY ($1B/$1C) + $05/$06 -> $03/$04 with $2000/$2400 select via PpuCtrlShadow; rts $EFE9; NmiStripUpload $EFEA |
| $F264-$F270 | 13 | code (Static), UploadAttr for $2B40 | exact sibling of UploadAttr2000 ($F24A: src 0, $23) and UploadAttr2400 ($F257: src $40, $27) using src $40, hi $2B; RequestAttrUpload $F271 |
| $F2B3-$F2CD | 27 | code (Static), copy N tile/attr pairs into OamBuffer | A*4 slot index, ($05) source, count $02; rts |
| $F2CE-$F2F7 | 42 | code (Static), OAM rotation copy | indexes table $F2F8 with $67 (wraps at $20) |
| $F2F8-$F317 | 32 | data: 32-entry OAM offset table | consumer `lda $F2F8,x` with x=$67 in 0..$1F (code resets $67 at $20); table ends exactly at ClearOamBuffer $F318 |
| $F333 | 1 | generic data (stray $CD) | see CD note |
| $F334-$F377 | 68 | code (Static), inline-arg sprite palette setter | pla/pla, 2 inline bytes (slot, count), sets attr bits of N OAM entries, `jmp ($0003)` after +3 (same idiom as $F0F7 2-byte inline routine) |
| $F378-$F3BA | 67 | code (Static), object stepper | object fields $044F-$0456,y (ObjectIndex $1D); direction*2 indexes $F3BB/$F3BC |
| $F3BB-$F3CA | 16 | data: 8-direction (dx,dy) table | (0,-1)(1,-1)(1,0)(1,1)(0,1)(-1,1)(-1,0)(-1,-1); 8 dirs x 2 bytes, flush with $F3CB code |
| $F3CB-$F431 | 103 | code (Static), metasprite draw ($FF-terminated 5-byte records into OamBuffer) | rts $F431; ReadActiveJoypad $F432 |
| $F467-$F471 | 11 | code (Static), wait on $58 with $59=60 (seconds timer) | NOTE: current NMI never decrements $58 (NmiHook is a bare rts) -> would hang if called; library leftover |
| $F479-$F485 | 13 | code (Static), X*31 busy-wait | |
| $F486-$F4A0 | 27 | code (Static), immediate PPU read into $02C3 | |
| $F4A1-$F4B4 | 20 | code (Static), d-pad -> direction via table | same joypad select as ReadActiveJoypad |
| $F4B5-$F4C4 | 16 | data: d-pad nibble -> direction (0-7, $FF none) | index = pad&$0F (Right=1,Left=2,Down=4,Up=8): R->2, L->6, D->4, U->0, UR->1, DR->3, DL->5, UL->7: same direction numbering as $F3BB. Ends at ReadJoypads $F4C5 |
| $F4E7-$F4F9 | 19 | code (Static), wait release then press (non-Start/Select), store $5D/$5E | Random $F4FA |
| $F545-$F5FF | 187 | code (Static), palette fade-out ($F545,+$F5D7) / fade-in ($F568,+$F5B0) | internal jsr targets inside block; LoadBgPalette/LoadSpritePalette16; PlayMusic $F600 |
| $F6AD-$F6AF | 3 | generic data (CD 00 EC) | after `jmp JmpResetMain`; decoding runs into the signature text at $F6B0 (which itself starts with $CD) |
| $F6FD | 1 | generic data (stray $CD) | after `jmp NmiNormalFrame_sound` |
| $F754 | 1 | generic data (stray $CD) | after `rti` |
| $F76A-$F7A0 | 55 | code (Static), reversed-order OAM copy | rts $F7A0; ClearScreens $F7A1 |
| $F7DB-$F842 | 104 | code (Static), MMC1 init + serial register writers ($F7F3 $9FFF, $F807 $BFFF, $F81B $DFFF, $F82F $FFF9) | `inc $FFF9` reset idiom; 5-write lsr chains; harmless on UxROM (bus conflicts aside) |
| $F843-$F844 | 2 | generic data (CD EF) | $F843 CD decodes `cmp $A9EF` and then `ora ($85,x)` + illegal $5A, conflicting with the coherent routine at $F845 |
| $F845-$F853 | 15 | code (Static), MMC1-style BankSelect variant | `lda #1/sta BankSwitchBusy/txa/jsr $F82F/stx CurrentBank/lda #0/sta BankSwitchBusy/rts` = same protocol as BankSelect $F854 |

### CD separator note
$CD appears immediately after a terminal instruction at $F333 (after rts), $F6AD (after
jmp), $F6FD (after jmp), $F754 (after rti), $F843 (after rts), and as the first byte of
the classified signature text at $F6B0. No other `rts/rti/jmp`+$CD occurs in classified
code. At $F333/$F843/$F6AD the decode as an instruction runs into illegal opcodes or
text; no flow reaches any of them. Proposed: generic data (no specific role claimed).

### Byte totals (proposal)
- code (Static): 19+8+54+4+16+73+167+103+81+13+27+42+68+67+103+11+13+27+20+19+187+55+104+15 = 1296 (+3 $C040 optional)
- consumer-backed data lists/tables: 13+49+32+16+16 = 126
- generic data (CD bytes): 1+3+1+1+2 = 8
- dead rts bytes (code-vs-data weak): 3 ($D76C, $E24A, $E30F)
Total 1296+126+8+3+3 = 1436 (all unknown bytes accounted for).

Reachability: NONE of the code entries has a known caller (except the two data lists,
which are reached from classified code). Mark Static code with "no known caller";
do not label Observed. Several routines target MMC1 or a seconds timer the UxROM
build no longer services; they look like a shared library linked in full.
