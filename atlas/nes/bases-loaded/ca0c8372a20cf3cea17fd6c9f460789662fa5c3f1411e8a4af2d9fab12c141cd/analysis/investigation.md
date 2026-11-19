# Investigation ledger

## Round 1 disassembly (2026-11-19, checkpoint pass-001)

### Architecture established
- Bases Loaded (US NES, Jaleco/TOSE 1988; research.json). MMC1, 256 KiB PRG, 64 KiB CHR.
- Every 16 KiB PRG bank ends with an identical reset stub at $xFB0 (writes PRG=0, control=$02 =
  32 KiB PRG mode + vertical mirroring, CHR=0, then JMP $8000), an RTI at $xFDC, $FF fill and
  vectors. Hence the game runs in 32 KiB mode: eight 32 KiB banks, context
  prg_banks=[4n,4n+1,4n+2,4n+3]. Power-on executes bank 31's stub under [0,1,30,31].
- Common module $8000-$8353 in every 32 KiB bank: identical bytes in banks 0-2; banks 3-7 are
  relocated copies (banks 4-7 contain JSR $FFFF placeholders, bank 3 JSR $8241) — dead copies,
  since reset always selects bank 0. PRG switch helpers: $81AF (A bits 4-5 -> bank), $8242 /
  $817D / $8173 (via $077B-style values; also CHR); NMI save/restore at $82AE; NMI at $8354
  (per bank) dispatching through word tables indexed by $F1.
- 32 KiB banks 4-7 share an identical lower 16 KiB engine (8K banks 16/17 = 20/21 = 24/25 = 28/29)
  and carry DPCM voice samples in their upper 16 KiB.
- Inline jump-table dispatcher JSR $DB7E (bank 1): ASL/TAY/INY/PLA/PLA, words follow the JSR.
- Overlapping-instruction skip trick (`AE` = LDX abs skipping `LDA #$00`) at bank 0 $9E25/$9E26 and
  $9E3D/$9E3E: both entries observed; regions decode linearly from the earlier entry.

### Evidence and methods
- Observed code: aggregate trace coverage (15,593 instruction starts with contexts, scratch/obs.json)
  from scenarios S01-S09 (analysis/scenarios.json).
- Static code: own bank-aware traversal (scratch/static.py, run_static.py) because toolkit discovery
  stops at every JSR on MMC1 (toolkit-issues TK-002). Rules: follow JSR targets in caller context;
  no fallthrough after calls whose static closure writes $E000-$FFFF; stop at RTS/RTI/BRK/JMP
  indirect except recognised tables; don't follow the dead side of branches made constant by an
  immediately preceding LDA/LDX/LDY #imm, SEC/CLC/CLV; inline tables after JSR $DB7E bounded by the
  lowest entry target; LDA T,i/STA zp/LDA T+1,i/STA zp+1/JMP (zp) tables bounded by AND mask,
  lowest target and known code bytes; non-adjacent unmasked tables accept only observed targets;
  tables truncated at entries whose traversal reaches an invalid opcode (83E8 earlier; c40d).
  Final traversal reports 0 invalid opcodes, 2 known overlaps.
- DPCM: routine $BD00 (observed) writes $4012=(($077B&$0F)<<4), $4013=$F0 -> start $C000+n*$400,
  length 3841. In banks 4-7 upper halves the samples sit at $C000/$D000/$E000/$F000 and each slot's
  last non-$FF byte is exactly +$F00 (14 non-empty samples; bank 7 slots 2-3 empty) -> Data with
  labels DPCM_Bank*_Sample*, fills -> Padding.
- Generic data rule (scratch/gapdata.py): a linear sweep of the gap gives >=20% illegal opcodes,
  BRKs or operands outside RAM/PPU/APU/PRG-RAM/ROM (every classified code region scores 0%,
  DPCM data 30-60%), and no clean run of >=12 plausible instructions ending in RTS/RTI/JMP exists.
  74,756 bytes classified this way (confidence 0.75). Validation: 751 instruction starts newly
  observed after the rule was applied (VS mode, password entry, extra innings) fell in none of
  these Data regions.
- $FF runs >= 16 bytes -> Padding (no data reference into any of them; checked).
- Header, CHR (Image, 8 x 8 KiB), vectors, stubs, "BASES LOADED" signature text.

### Lead outcomes
No input leads (first pass). Leads produced: L-001 .. L-008 (analysis/leads.json).

### Remaining unknown (14,557 bytes)
189 gaps, all inside code-bearing banks: either clean code-like islands (likely handlers behind
unresolved indirect jumps; e.g. $197DD+647, $1C062+387, $19DFE+274, $03C76+197, $03DAA+196,
$08601+193, $04010+188) or data with spurious islands ($11561+2395, $12BDE/$224BC/$2A4BC/$324BC/
$3A4BC +448, $0E8FF+1472). Full list: scratch/kept_unknown.json (offset, length, badness, islands).

### Uncertainties
- Static code regions (19,644 bytes) rely on the traversal assumptions above (callees return
  normally except $DB7E; $AA3D-style "pla pla rts" helpers conditionally return two levels).
- Jump tables with unmasked indices are bounded heuristically; L-001/L-005.
- Generic data has no semantic role yet (L-003).

## Round 2 disassembly (2026-11-19, input pass-001 -> checkpoint pass-002)

Input: redirect r1-a1 ready leads (analysis/redirect/r1-a1/proposals.json). Every lead was re-verified
before map.annotate (tools in scratch/: r2check.py, r2datacheck.py, r2audit.py, r2ram.py, r2control.py,
r2apply_leads.py, r2apply_l024.py; outputs r2check.json, r2audit.json, r2-unknown-after-leads.json).

### Lead outcomes
| Lead | Outcome | Bytes | Verification |
|---|---|---|---|
| L-015 | verified | 5,226 code (incl. 138 Data->Code) | probe.py rerun reproduced the 117 runs exactly; FarCall $817D return path re-read (JMP ($F2) returns to $819E, restores banks, RTS); AtBatStateTable words and FarCall sites of the $FFxx vector roots re-listed; each run decodes aligned, official opcodes only, ends RTS/JMP or abuts classified code |
| L-016 | verified | 250 data | table $B199-$B298 (128 words, ends at first handler $B299) re-annotated as one region |
| L-017 | verified | 2,252 data | consumers $9597 LDA $95A3,X; immediate pointers at $9511/$951C/$9551 |
| L-018 | verified | 1,472 data | base pointers set at $E82B-$E8B2 |
| L-019 | verified | 2,240 data | ORA abs,Y readers in each bank |
| L-020 | verified | 140 code | byte-identical to bank 0 $9E23-$9E45; JSR targets of module copies |
| L-009 | verified | 70 code | $8173 stub falling into FarCall (bank 1 copy already classified) |
| L-021 | verified | 28 code | same-bank continuation after SetPrgBank |
| L-022 | verified | 15 code | table $B920 handlers 2/3 |
| L-023 | verified | 541 data | consumer instructions re-found (21 ranges; $F0EE consumer in L-024 code) |
| L-024 | verified (reopened) | 2,427 code + 33 data | see below; $19CBC (1 byte) not accepted -> L-028 |

### L-024 reopening (evidence-policy "revisit reasoning")
Old premise: an entry (caller/pointer) is required. Corrected premise: static code needs structural
evidence for role, exact bounds and bank context; reachability is a separate claim. Independent
constraints combined per range: (1) bytes exactly fill a gap between classified regions, starting after a
classified RTS/JMP/RTI/table; (2) linear decode in the 32K bank context is instruction-aligned to the end,
official opcodes only, no BRK, ends in RTS/JMP or falls into a classified instruction start; (3) every
JSR/JMP/branch target is a classified instruction start or an instruction start inside the batch (chains:
$864D->$84A8->bank1 $FF00->$F074; $FF12->$8396->$857C; $A00A->$8442; $F697->$FF24->$CAFA->$CB63);
(4) absolute operands are RAM/PPU/APU or classified tables; (5) RAM operand addresses are variables also
used by classified code: all known except $07FF (2 ranges) and $06A4-$06B0 (2 ranges).
Competing interpretation (data decoding by chance) tested on 135 classified Data regions decoded the same
way: mean RAM-known ratio 0.55, and only 1 (a short word table) is clean with ratio >= 0.8.
Embedded non-code split out: $B73F-$B75A length-prefixed copy tables (read by the preceding routine),
$CB40-$CB44 sprite record (immediate pointer at $CB0C). Accepted Code/Static at confidence 0.75 with
"reachability unresolved" in evidence; not labelled Observed, not claimed dead.
Rejected for now: bank 3 $9CAC (phys $19CBC), a lone RTS between JMP $9C94 and live code $9CAD with no
reference (no word $9CAC/$9CAB, no branch) -> Unknown (L-028).

### Gameplay
No new runs: checklist S01-S11/S13 visited (S10/S11 artificial via r1-a1 patches), S12 attempted. With
1 unknown byte left and 0 L-024 fetches in the r1-a1 end-of-game/season/ending coverage, further play
could only add reachability evidence, not classification.

### Remaining unknown
1 byte: phys $19CBC (bank 3 $9CAC). Result: incomplete (1 unknown byte), byte-identical pass-002.

## Round 3 disassembly (2026-11-19, input pass-002 -> checkpoint pass-003)

Scope: ready redirect r2-a1 leads L-028 and L-029 (analysis/redirect/r2-a1/notes.md F1/F2, proposals.json).
Applied with scratch/r3apply_leads.py; state restored from analysis/state.json, all other regions preserved.

### L-028 — bank 3 $9CAC (phys $19CBC): verified, Code/Static 0.6
Independent checks (not just rerunning the redirect scripts):
- Linear decode $9C87-$9CB2 in 32K bank 3: FillNametableTile1 ends INC $00 / JMP $9C94 ($9CA9-$9CAB); $9CAC = RTS;
  $9CAD = LDA $69 (DrawBatterLine, Observed). The gap is exactly one byte, so no multi-byte record fits.
- Own scan over all bank-3 Code regions: no JSR/JMP/branch target and no absolute/indexed operand equal to
  $9CAB/$9CAC. PRG-wide LE word search for $9CAB/$9CAC finds only phys $3D96B (inside a DPCM sample, already classified).
- Consistency: a crude ROM-wide scan of $60 bytes after a classified JMP/RTS/RTI and before classified code found
  58 candidates. All are Code except this byte, including unreferenced round-2 L-024 null hooks. Rejecting it would
  apply a stricter criterion than the accepted ones; the redirect corrected the round-2 rejection premise, which treated
  the byte in isolation.
- Competing data reading: no consumer, no 1-byte inter-routine data precedent; fill in this ROM is $FF.
Judgement: code vs data is not materially ambiguous, so the byte is accepted. Confidence is 0.6, the lowest in the map,
because only structural and consistency evidence exists. Reachability is unresolved and the byte is not claimed live
or dead. No map label was set (other null hooks are named only in commentary); suggested name NullHook9CAC.

### L-029 — AnimationStepTimes / DrawStepFigure boundary: verified
- Consumer $A371-$A397: INC $057B / LDX $0579 / CMP $A398,X / ... / INC $0579 / CMP #$02. Sibling consumers at
  $A44C/$A4D0 have the same shape (CMP $A47C,X / CMP $A4FD,X, limit 4) and their tables are 5 bytes, each followed
  by a routine start. That makes three instances of the "timer; 5-byte table; routine" layout.
- $A39C ($40) as RTI cannot begin the JSR-style routine $A39D-$A3BB (ends RTS). No reference to $A39C/$A39D exists.
- New regions: Data $1A3A8+5 (0.9), Code/Static $1A3AD+31 view $A39D (0.75, reachability unresolved).
  Byte-identical; net 0 classified bytes (1 byte Code -> Data).

### Gameplay / other
No new runs, because no lead needed play and the checklist is essentially fully visited. L-031 is commentary-only and
left ready. No new toolkit issues.

### Remaining unknown
None: 327,696 / 327,696 classified, byte_identical true (pass-003). Completion still needs commentary for L-031 and
the changed regions, plus the final editorial audit.
