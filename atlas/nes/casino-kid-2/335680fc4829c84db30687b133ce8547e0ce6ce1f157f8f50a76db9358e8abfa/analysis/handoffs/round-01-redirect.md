# Round 1 / redirect handoff (attempt r01-a1)

Status: **complete** (connected avenues closed or deferred with reasons; not terminal).
Checkpoint used: pass-001 (maps/pass-001.json == analysis/state.json, 87.31%, 16,641 B unknown).
No map/ROM/source changes. Research: analysis/redirect/r01-a1/ (tools/, unknown ledger, text_sel.txt,
frames/), runtime evidence traces/redirect-r01/ (gz snapshots + coverage/contexts).

## Ready leads (analysis/leads.json; expected unknown bytes, non-overlapping, total 11,783)
| ID | Target | Bytes |
| R1-L009 | CHR tile graphics sets banks 0/1/2 (LoadTileGraphics descriptor extents, contiguous tiling) | 2,790 |
| R1-L010 | Card tile blocks: 53-word table $C5CE + 53x37-byte 5x7 blocks to $CDE0 (supersedes L001) | 1,920 |
| R1-L011 | Bank 4 $8DDA-$90D0: jump table + handler code + chip-value jump table (exact closure) | 759 |
| R1-L012 | Bank 4 $913D-$91D0: jump table + hold/draw handlers (observed) + RunGridMenu descriptor | 148 |
| R1-L013 | Fixed-bank: $C197 continuation code (observed), OpponentGameTable, $EB6D job table, $E31B, $E93B stub, table tails | 73 |
| R1-L014 | TextListTable $CE1C + word lists + 20 VRAM structs to $CF51 | 227 |
| R1-L015 | DPCM samples (bank-6 DMC table $8C72) + duplicate tail $FC00-$FF2F | 272 |
| R1-L016 | Metasprite tile lists $DEFD-$DFD7 | 219 |
| R1-L017 | Bank 3 pictures 0..5 $A480-$BC7F + 44 palettes $BC80-$BF3F | 700 |
| R1-L018 | Bank 5 $82C2/$831B/$83ED code + $840E data (corrected always-taken-branch rejection; supersedes L003) | 224 |
| R1-L019 | Small consumer-anchored tables (b7 $D683,$ECDE,$F637,$F8B3; b2 $B0FD; b5 $952A,$A844) | 221 |
| R1-L020 | 79 game-encoded text windows, banks 2-6 (text_sel.txt) | 4,230 |
Each lead lists exact ranges, bank context, consumer evidence and a recipe (tools in r01-a1/tools:
gfx.py, desc.py `account(...)`, xref.py, textscan.py, leads_calc.py; m6502/analyze copied from round01).
Method finding for disassembly: round-1 rejection decoded fallthrough of always-taken branches
(lda #nonzero; sta..; bne) — desc.py implements the rule; rerunning round-1 rejection with it may
recover more transitively rejected nodes.

## Gameplay / progression (scenarios.json S05-S12)
- Opponent money RAM (R1-L004 verified): blackjack $07A9/$075B, poker $075B, King $06A0/$06DB (32-bit
  cents). Money patch + one won hand gives a natural defeat (GameOutcome 3, MarkOpponentResult).
- OpponentResultBits $0526-$0528 = FF FF 03 (written before the map is set up) unlocks every location
  and KING / Las Vegas (final). Poker vs Ailton played and won; roulette spin = Start (R1-L005 verified).
- Final match vs King reached (roulette stage); not won. Ending/credits seen only via artificial
  pc=$C1FD entry. Traces: $C197-$C1A2 and $914D-$91C2 executed; no executed byte is classified data;
  1,702+669+301 static code addresses newly observed (contexts in traces/redirect-r01).

## Deferred avenues (reason)
- Bank 6 music streams (~1.3 KB residual): need sound command operand-length table (decode
  B6_UpdateChannel handlers $F1-$FE) to walk streams from song headers at $8C8A. Credible next redirect
  or disassembly sub-task.
- Bank 2 attract BuildVramStrings blocks ($B6EF/$BACF) and bank 5 layout pointer tables ($BA02,$BB10,
  $BAB5): block walking (w,h,w*h) bounds remaining Data-Data windows; not done for time.
- Bank 6/5 name/label text below the 0.9 threshold ($A3C1-$A540, $BB2A, $A078): manual check.
- $C1B3-$C1B5 jmp $C1FD and $D209-$D219 remap fragment: no referrer; code-vs-data ambiguous.
- Winning the final match naturally: need the roulette result variable (bet on it or patch payout).
Residual ledger: analysis/redirect/r01-a1/unknown-ledger.md (4,858 B if all leads accepted).

## Recommendation
Run another **disassembly round** consuming R1-L009..R1-L020 (and re-run the rejection pass with the
always-taken-branch rule), then commentary (revise TextPointerTable -> card blocks, poker/roulette
handler names) and a redirect continuing with the bank-6 stream walk and bank-2/5 block walks.
