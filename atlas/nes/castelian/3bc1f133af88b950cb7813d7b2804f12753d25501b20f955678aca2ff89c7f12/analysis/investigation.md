# Investigation ledger

## Round 1 disassembly (checkpoint pass-001)

Method: observed execution (bounded traced windows in scenarios S01-S11, analysis/scenarios.json)
plus a local static recursive-descent analyzer (analysis/evidence/round1/tools/static.py) seeded
from all observed instruction addresses with their effective PRG context, the CPU vectors, RTS
dispatch tables ($893A/$8946, $906F/$907B, $9F8B/$9F96, $9FA1/$9FAC, $83B2/$83AA, $C894/$C884,
$DD9D word table), jmp-indirect word tables ($F314, $F27D) and NMI handlers stored to $00/$01.
Bank context rules: switchable-bank code keeps its own 16K bank; fixed-bank code carries the bank
set by `lda #n / jsr $C00B` (SwitchPrgBank: and #7, tax, sta $C000,x with bus-conflict table
$C000-$C007). Static threads were rejected on BRK/illegal opcodes, 16K-window crossing or overlap;
0 conflicts with observed instructions. Static code is provenance Static (confidence 0.8), not
claimed reachable.

Data: classified only where non-code is clear: iNES header, 16K banks 0-2 (no execution ever
observed; tile bitplanes, nametable runs, leftover C64 assembler source text, zero/FF fill),
uniform $FF/$00 fills, ASCII strings with print consumer $FD17, dispatch/pointer tables with
identified consumers, $EB1D (i*16) table, bank-select table, CPU vectors.
Evidence for data reads: analysis/evidence/round1/rom-data-reads-sampled.json (sampled; each
chunk truncated at the 60 MB stream cap, TK-004). Note: 16K bank 3 is entirely read by the XOR
checksum loop at $BBE6, so reads there are not discriminating.

Observed execution coverage: 11,498 unique ROM instruction addresses (8,445 in natural
scenarios, rest only under RAM-patched scenarios), 17 RAM instruction addresses $06D5-$06FA
(RAM routine; upload origin not claimed). analysis/evidence/round1/observed-contexts.json.

Remaining unknown: analysis/evidence/round1/unknown-ranges.json (41,948 bytes, 67 ranges).
Open leads: L1-01..L1-08 in analysis/leads.json (bank 3 interior, bank 5 and 6 data/code split,
bank 4 tables, unresolved jmp ($0063)/($01EB) bank, small unreached fragments, $3F mode flag,
small tables).

Rejected/not pursued in round 1: none rejected. Gameplay in towers 2-8 added only ~57 new
instruction addresses over tower 1 (diminishing returns); further per-tower play not repeated.

## Round 1 redirect (attempt analysis/redirect/r1-a1, checkpoint pass-001)
Static consumer/extent analysis made every unknown range except 2 bytes ready
(ledger analysis/redirect/r1-a1/ledger.json; leads L1-01..L1-09 ready). Outcomes:
L1-05 bank resolved (explicit lda #$05 before the indirect jumps); bank 5 $A9E5-$BFE4
= bonus maps towers 4-7 (round-1 tower-map hypothesis rejected); bank 6 gaps chained
by consumers; bank 3 $BD38 tower palettes; bank 4 chain blocks are RTS-table entry 11
(new L1-09); $A019 jmp slot + $A0EC target; sound walker 2378/2382 bytes. Residual:
bank 3 $B6B0 and bank 4 $B685 (1 byte each, ambiguous). Attract mode reached by idling
~2700 title frames (S12). Handoff analysis/handoffs/round-1-redirect.md.

## Round 2 disassembly (input pass-001, output pass-002)
Method: restored analysis/state.json (pass-001, 495 regions); re-checked every r1-a1 ledger
proposal with analysis/evidence/round2/tools/verify_leads.py (range wholly unknown; no observed
instruction fetch inside proposed Data; consumer abs/indexed refs from same bank or fixed bank;
immediate lo/hi pointer pairs for the no-ref cases $BCCF/$BDA3/$B7A6/$BE41/$B6B1 strings;
every fixed-bank JSR/JMP into a proposed bank-3/4/5/6 Data window lands on Code of another
bank at the same CPU address, so none contradicts the data claim; linear decode of all Code
proposals ends exactly at the range end). Applied 75 proposals with
analysis/evidence/round2/tools/apply_leads.py (map.annotate; validate/save/export ok).
- Observed code (natural attract run S12, every instruction start fetched): bank 4 $909D-$9136,
  $9864-$98FD (L1-09); bank 5 $A533-$A5E1 (L1-05 drawers, prg_banks [10,11,14,15]); fixed
  $F11A-$F120, $F197-$F19D (stubs).
- Static code: bank 4 $A019 (jmp $A0EC slot) and $A0EC-$A0F9 (0.8); $B692-$B6B7 RAM-routine
  storage image (copied by $B686 to $06D5, executed there; NES uploaded views unsupported, so
  the view is the storage alias, branches relative); $E4B3-$E4CC and $E585-$E590 (0.6).
  Critical review of the two orphans: legal, closed, RTS-bounded decode between classified code;
  $E4B3 is the mirror twin of $E4A0 (same $05 direction test, $3D frame store, inc $FE) reading
  the classified frame list table $DD67/$DD6B; $E585 is an inc-capped-at-9 helper for $01F4
  (written at $E525). No RTS/pointer bytes for $E4B2/$E584 anywhere in ROM. Competing data
  interpretation has no support. Reachability unproven, not claimed dead.
- Data (Static 0.9): all other proposals (banks 3/4/5/6 and fixed-bank tables).
- Correction: leads.json phys bounds of L1-01 sub-ranges after $9BAE were off by 768; the ledger
  offsets (used) are correct.
- Residual unknown (L2-01, blocked): bank 3 $B6B0 ($60) and bank 4 $B685 ($EA), 1 byte each,
  unreferenced valid opcodes indistinguishable from pad bytes.
Result: 100.00% (131086 / 131088 classified; 2 unknown). Gameplay: S13 (2-player result) see
analysis/scenarios.json.

## Round 3 disassembly
- L2-02 verified: Observed provenance from artificial scenarios S13b (PLAYER 2 WINS) / S14 (HERO) applied
  to 1122 code bytes (7 full regions, 9 split into Observed/Static runs). Structure unchanged; natural
  reachability not claimed. Details analysis/evidence/round3/l2-02-changes.json; tool
  analysis/evidence/round3/tools/apply_l202.py. L2-01 unchanged (terminal). Checkpoint pass-003.
