# Investigation ledger

Follow workflow.md's completion and investigation contract.

## Round 1 disassembly (checkpoint pass-001)
Method: observed coverage/contexts from bounded scenarios (analysis/scenarios.json) + local
recursive descent (official 6502 opcodes; UxROM fixed bank $C000; $8000 bank from observed
context or static bank-select contract `ldx #n; jsr $F854`), inline-argument contracts and
consumer-decoded display lists. Toolkit static discovery unusable here (TI-003).

Key structural facts:
- Bank select $F854 (X = 16K bank; bus-conflict table $EC00-$EC07). Sound calls $F600/$F61E/$F640
  switch to bank 0 and call jmp-vectors $C030-$C039 -> bank 0 $A000-$A009; $C1AD ends with bank 6,
  so main-loop calls at $C05D..$C137 into $8000-$BFFF are bank 6.
- Inline-argument routines: $F06C (5 bytes: PPU addr lo/hi, src ptr, 16-byte tile count;
  returns jmp ($2C)), $F0F7 (2-byte display-list pointer; returns jmp ($31)). No other
  jsr target pops its return address (scanned all decoded callees).
- Display list format ($F0F7/$F14C): 6-byte records width,rows,PPU lo,hi,src lo,hi; $FF end.
- Jump tables: $C17D (3 entries, mode $0301), bank 0 $B025 (sound commands, entries 1..9).
- Lake banks 1-4: 30-word pointer header ($8000-$803B, first pointer $803C). Bank 0
  $8000-$8007: 4 lake map pointers. Bank 5: tile data (observed reads).

Outcome: 129652/131088 bytes classified; residual 1436 bytes in fixed bank = unreferenced
code-like routines and stray bytes (leads L-001, L-002). Data semantic refinement L-003.
Gameplay obstacles L-004. Static-only code L-005.

Checked and rejected/limited:
- Raw LE word search for gap starts: only $C8F0 (5), $F2B3 (2), $F479 (1) hits; not inspected
  yet (L-001).

## Round 1 redirect (r001-a7k2)
- Unknown complement fully dispositioned: analysis/redirect/r001-a7k2/bank7-unknown-ledger.md
  (code/table/generic proposals; weak residue $C040, $D76C, $E24A, $E30F).
- Pass-001 pointer search missed split-immediate pointers: $C38E/$C39B are Expand1bppList lists.
- Standing code generator: tools/standing_code.py; late-game/ending route in scenarios S-012..S-015.
- Negative: no caller for any bank-7 library routine by jsr/jmp, LE word, split immediate or
  rts-trick word; banks 1-4 word hits are data coincidences.

## Round 2 disassembly (pass-002)
Independent verification of ready leads (scratch checks, not retained; method reproducible
with analysis/redirect/r001-a7k2/tools/dis7.py and refsearch.py):
- L-008: re-parsed $C38E/$C39B with Expand1bppList $E5AF (6-byte records, $FF lead byte)
  and Expand1bppTiles $ED51 (8 source bytes per tile regardless of plane mode). Terminators
  at $C39A/$C3CB; record sources $8000,$8000,$8200,$81D8,$A9B5,$AE73,$B7E0,$B850 chain
  flush with independently bounded bank-5 objects. Accepted.
- L-001: every block linear-decodes with official opcodes only, ends exactly at rts/jmp/
  jmp(ind) or falls into classified code ($DB90 -> NegateMathA $DBC6); all branch/jsr/jmp
  targets are instruction starts in the batch or classified code; contracts re-read
  (UploadPalettes copy, DrawObject alt entry, UploadAttr sibling, inline-arg idioms of
  $F06C/$F0F7, MMC1 serial writers, BankSelect protocol). Tables: $F2F8 (x=$67 wraps at
  $20, values multiples of $20), $F3BB (8 compass dx/dy pairs), $F4B5 (pad nibble ->
  same direction numbering). refsearch re-run: only banks 1-4 data coincidences and
  intra-batch calls. Static code, no known caller; reachability unknown. Accepted.
- L-002: $CD strays at $F333/$F6FD/$F754/$F843 would overlap coherent or observed routine
  starts as code; $F6AD-$F6AF runs into signature text -> generic data. $C040: accepted as
  code (aligned jump-slot block; jmp ($FFFC) idiom classified in bank 6 $B77B). $D76C/$E24A/
  $E30F: accepted as code at low confidence (0.6): all three game-logic strays are exactly
  $60 following a terminal instruction, whereas the library strays are $CD; competing
  stray-data reading is not excluded by any consumer, so revisit if contrary evidence appears.
- L-006: consumer-bounded pointer tables kept; added tile-source extents from
  Expand1bppList and CopyTilesInline arguments ($8208 x1/$8218 x7 2bpp flush with $8288;
  $B8DE x28 flush with $BA9E; $AE73 x94 1bpp flush with $B163). Generic remainders:
  fish/lure frame tables, $AB35-$AB5C, $AC4D-$AE72, $B858-$B8DD, $BAD4-$BD8D, $BD98-$BFFF.
Outcome: 0 unknown bytes. Open (non-classification): data semantics (L-003), static-only
code observation (L-005), natural reachability of the bank-7 library routines.
