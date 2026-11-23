# Investigation ledger

## Round 1 disassembly (2026-11-23, checkpoint pass-001)

### Architecture findings (static, bank-aware; AxROM 32 KiB bank b = 8 KiB PRG banks 4b..4b+3)
- Bank 0 is the main bank: reset stub $FFEE (sei; lda #0; sta $22; sta $FB1C; jmp $8000)
  exists in every bank and selects bank 0; ColdStart at bank0 $8000. Bank-0 NMI vector $ADE9;
  banks 1/2 NMI vector $FFB7 (save regs, push bank, select bank 0, body at bank0 $FFC4
  jsr $B560, restore bank with sta $FB1C,x, epilogue $FFCC rti in the original bank);
  bank 3 NMI = IRQ = $FFF9 (rti).
- Frame loop: code ends a frame with `ldy #lo; ldx #hi; jmp $AE10` (stores Y/X to $20/$21,
  enables NMI, spins at $AE23); NMI (bank0 $ADE9) resets S=$FF and `jmp ($0020)`.
  12 such sites give 9 distinct resume targets (static roots).
- Far call: `lda #bank; sta $23; lda #index; jsr $FFD2`. Stub stores index to $18 (pointer
  $18/$19, $19=$80 set at ColdStart), swaps $22/$23, selects the bank via sta tbl,x, then
  `jsr $8000` = `jmp ($0018)` in the target bank, i.e. index selects a 3-byte JMP entry.
  73 call sites, all in bank 0; tables bank1 $8003-$801B, bank2 $8003-$8030 (two unused
  `jmp ($0018)` slots at $8021/$8027), bank3 $8003-$802D.
- Bank-select tables (bytes $00..$13 = index) per bank: 0:$FB1C, 1:$FE8B, 2:$C7C0, 3:$FEDC.
  Stores `lda #v; sta table+n` keep the same bank when (v & byte) & 3 == current bank.
- RAM code: bank0 $F796-$F805 copied to $0700 (RLE decoder used by graphics loader),
  bank2 $C730-$C794 copied to $079B (unrolled VRAM copy). Both stored as Data (uploaded
  views unsupported by the toolkit); lead L002.

### Method
- Toolkit static discovery stops at every JSR / indexed store on mapper 7 (toolkit issue
  TK-002), so a local AxROM-aware recursive descent was used
  (analysis/evidence/round-01/tools/trav.py): branches, jmp/jsr targets, jsr fallthrough
  (no callee starts with pla/tsx; no non-returning callee found), stops at rts/rti/jmp(ind)/brk
  and at stores to $8000+ unless the resulting bank is provably unchanged. Dispatch tables
  (`lda T,x; sta zp; lda T+1,x; sta zp+1; jmp (zp)`) were accepted only with an extent proof
  (closure at a target/code start, or index mask). No invalid opcode, BRK or overlap occurred
  in 35,085 traversed bytes; every observed ROM instruction start (9,478) coincides with a
  static instruction start (0 conflicts).
- Every code region was validated by toolkit map.annotate decoding with explicit
  prg_banks context.
- Data: every remaining gap was checked for code-like linear decodes (>=6 instructions,
  >=85% common opcodes, ending in rts/rti/jmp) and for any static/observed entry. Only the
  L001 fragments, the two RAM routine images and a few false positives inside obvious
  tables/graphics were flagged; flagged gaps are kept Unknown (152 bytes). Data blocks are
  split at code-referenced addresses and uniform runs >= 16 bytes; specific roles were only
  claimed with extent evidence (vectors, header, bank-select tables, 3 jump tables, graphics
  descriptor table, message pointer table and strings, RAM routine images).

### Execution coverage (separate from classification)
Observed instruction starts / static instruction starts: bank0 6373/9717, bank1 345/653,
bank2 220/1871, bank3 2540/4271; 90 RAM PCs. Scenarios S01-S06 in analysis/scenarios.json
(missions 2-8 and NMI-resume screens are artificial RAM patches).
Evidence: analysis/evidence/round-01/coverage.json.gz, camp*.log, frames/.

### Unknown residue (152 bytes) - lead L001
0:$844C, 0:$9843-$9845, 0:$98C6-$98CC, 0:$9A1A-$9A53, 0:$9E30-$9E31, 0:$B091-$B092,
0:$E0B7, 2:$A1EB-$A1F0, 3:$9DBD-$9DC9, 3:$A067-$A0A1. All decode as code next to
classified code but have no known entry; code-vs-dead-code decision left to redirect.

### Lead outcomes this round
No prior leads. Produced L001-L005 (analysis/leads.json).

## Round 2 disassembly pass (2026-11-23) - checkpoint pass-002

Scope: ready redirect leads L006, L007, L008 only (static; no new gameplay - progression to the
ending already visited in S01-S10; L002 still blocked on toolkit RAM execution views, not retried).

### Verification method
- analysis/evidence/round-02/tools/verify_frags.py (local decoder) -> frag-verify.json: each range
  decodes exactly to its end with official opcodes; branch/jmp/jsr targets are internal starts or
  established Code; previous/next regions confirmed. Toolkit analysis.decode with explicit
  prg_banks context re-checked lengths before map.annotate (tools/apply_l006_l008.py).
- Byte identity rechecked: 3:$9E4A[68] == 0:$D518 == 2:$84E9; 3:$9DBD[13] == 3:$8901 == 3:$9D03.
- Bypass jumps rechecked: 0:$9A13 lda $47/bne $9A54/jmp $9A54 skips $9A1A-$9A53; 0:$B08E jmp $B093
  skips $B091; 0:$9E2D jmp $9E32 skips $9E30; 2:$A1E8 jmp $A1F1 skips $A1EB.

### Classification changes (all Code, provenance Static; reachability: no known entry, not Observed)
| Range | Bytes | Label | Conf | Lead |
|---|---|---|---|---|
| 0:$9843-$9845 | 3 | NoCaller_JmpGameOverFade | 0.8 | L006 |
| 0:$98C6-$98CC | 7 | NoCaller_LaunchActiveCheck | 0.8 | L006 |
| 0:$9A1A-$9A53 | 58 | BypassedTerrainPaletteBlock | 0.8 | L006 |
| 0:$9E30-$9E31 | 2 | NoCaller_StoreFramePhase | 0.75 | L006 |
| 0:$B091-$B092 | 2 | BypassedStarResetZero | 0.75 | L006 |
| 2:$A1EB-$A1F0 | 6 | NoCaller_FrameLeftClamp | 0.8 | L006 |
| 3:$9DBD-$9DC9 | 13 | NoCaller_ShadowDrawB3 | 0.85 | L006 |
| 3:$A067-$A0A1 | 59 | NoCaller_BulletHitCheckB3 | 0.85 | L006 |
| 3:$9E4A-$9E8D | 68 | MulSignedB3 (was Data) | 0.85 | L008 |
| 0:$844C, 0:$E0B7 | 1+1 | (rts after tail jmp) | 0.6 | L007 |
| 3:$91B9 | 1 | (rts after tail jmp; was Data) | 0.6 | L007 |

Result: 0 unknown bytes. Code 35306, Data 95782 (incl. 16-byte header). Weakest claims: the three
1-byte RTS decisions (pattern-based; no runtime discriminator possible).
Remaining non-classification items: L002 (RAM routine images kept as Data in ROM; toolkit lacks
uploaded execution views), L005 (data semantics for commentary).

## Round 3 disassembly (pass-003) - L009 BlockSubstTables extent

- Input: pass-002 state (scratch/state-before-pass003.json). One pass applying ready lead L009 only
  (static; reuses r02-a runtime evidence, no replay). Gameplay not repeated (S01-S12 reach the ending).
- Verification: toolkit analysis.decode of LoadBlockSubst 1:$C73A (prg_banks 4-7): lda $0669 (Mission)
  / asl a x4 / tay / ldx #0 / lda $D600,y / sta $05E1,x / iny / inx / cpx #$10 / bne / rts => reads
  $D600+16*Mission..+15; Mission domain 0-7 (8-entry mission tables) and r02-a E3c observed exactly
  those 16 reads for m=0..7 (dummy page-cross reads attributed to $C720 excluded). Content: records
  4-7 == 0-3, records 3/7 all zero. 3:$D76F-$D7EE == 1:$D600-$D67F (128 B). 3:$D76C-$D76E = c6 60 60
  (sta $C6 / rts / lone rts). Artifacts: analysis/evidence/round-03/inspect_l009.txt, tools/.

### Region changes vs pass-002 (byte totals unchanged except 1 B Data->Code)
| Removed (pass-002) | New (pass-003) |
|---|---|
| 1:$D600+48, $D630+25, $D649+39, $D670+16 (all Data) | Data 1:$D600-$D67F BlockSubstTables (128 B, conf 0.9) |
| 3:$D76E+49, $D79F+25, $D7B8+39, $D7DF+16 (all Data) | Code 3:$D76E (1 B lone rts, conf 0.55, L007-consistent) + Data 3:$D76F-$D7EE BlockSubstTablesCopyB3 (128 B, conf 0.85, no reader) |

Result: 482 regions, 0 unknown. Code 35307, Data 95781 (incl. 16-byte header). Checkpoint pass-003
byte-identical (map_sha256 08865b21...76a3). L002 still blocked (toolkit), L005 semantics-only.
