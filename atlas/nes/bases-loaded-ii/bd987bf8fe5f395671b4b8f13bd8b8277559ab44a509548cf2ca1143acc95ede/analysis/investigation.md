# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## Round 1 disassembly pass (2026-11-18) — checkpoint maps/round1-disassembly-b.json

Structural evidence and reachability are recorded separately: Observed code = traced opcode
fetches; Static code = bank-aware traversal only (reachability unproven).

### Accepted structures
- **Mapper/bank model**: reset stub ($8001) writes MMC1 control $0A (PRG mode 2) and PRG bank
  15; all 2,181+ observed contexts are [0,1,2S,2S+1]. Every 16 KiB bank starts with byte $80 +
  a copy of the reset stub (62 bytes, identical prefix) and ends with $FF fill, a bank-ID byte
  at $FFDF (bank 14: at $FFF9) and vectors NMI $85F0 / RESET $8001 / IRQ $8694.
  Copies in banks 1-15 annotated Static code with power-on context [2b,2b+1,30,31].
- **FarCallBank $84BC** (bank 0): saves `lda $FFDF`, selects bank $51 via MMC1_WritePrgBank
  ($8566), reads entry word `$C03F+$54` of the target bank, `jmp ($004D)`, restores bank.
  Per-bank entry tables at $C03F (15 banks) proven by "lowest entry target == table end" or
  "observed instruction starts at table end"; their targets became static roots (bank 9 code
  is entirely static: 1,568 bytes, no observed execution in round-1 scenarios).
- **Far-call trampolines** `A9 bb 85 51 A9 ii 85 54 4C/20 BC 84` with (bank, index) inside a
  proven entry table → static code roots (39 previously unreached).
- **Jump-table dispatchers** $849F (zp $24-$28) and $9AA2 (zp $29-$2D): pull return address,
  index by A*2, `jmp (ind)`. 41 inline tables proven (38 + 3) by "lowest handler target ==
  table end" or observed code at table end; handlers became roots. 14 unproven → lead L-002.
- **Banks 8/9 block tables**: 12-word pointer table ($C695 / $C693) read with index clamp
  (`asl; cmp #$17; bcc; lda #0`) → 12 contiguous 1 KiB blocks to the start of $FF fill.
- **Generic data** (analysis/tools/classify_gaps.py): pieces >= 64 bytes in which every 32-byte
  window contains an illegal/KIL/BRK opcode under linear decode and overall rate >= 20%, after
  excluding code-like islands (>= 4 legal instructions ending RTS/RTI/JMP, plus 32 preceding
  bytes) — 97 pieces, ~87 KB. Uniform runs >= 32 bytes → Padding. Role/consumer of most blocks
  unidentified (lead L-004). CHR ROM = 16 Image regions; iNES header Data; footer title Text.

### Corrections within this pass
- First dispatcher heuristic treated any callee with an early PLA/TSX as non-returning; $89D0
  (TXA/PHA … PLA/TAX/RTS, balanced) was a false positive (30 call sites). Corrected to require a
  PLA/PLP with no prior push. All 41 proven tables use the genuine dispatchers ($849F/$9AA2), so
  no data claim depended on the false positive; regenerated map gained 467 code bytes
  (checkpoint round1-disassembly → round1-disassembly-b).
- Spot check found bank 9 $C313 routine + table swallowed by the first generic-data rule;
  tightened island rule (>= 4 instructions) so such code stays unknown (L-001).

### Static-code safeguards applied
No invalid opcodes, no instruction overlaps, no unresolved targets in the final traversal;
constant-flag branches after `ld? #imm` / SEC / CLC / CLV follow one side only; static BRK
rejected; no static instruction touches unusual address spaces ($0800-$1FFF, $2008-$3FFF,
$4018-$5FFF) or stores to ROM other than MMC1 registers. Callers of observed JSRs whose
return was never fetched: only $849F and $9AA2.

### Leads (analysis/leads.json)
L-001 code-like islands (ready), L-002 unproven jump tables (ready), L-003 progression RAM
contract (ready), L-004 data-block roles, L-005 bank-14 ID anomaly, L-006 commentary groundwork.

### Remaining unknown
17,912 bytes in 239 ranges (analysis/evidence/round1/unknown_ranges.txt); largest: bank 2 4,789,
bank 4 2,361, bank 11 2,028, bank 3 1,866, bank 14 1,369, bank 15 1,178, bank 1 1,024, bank 0 1,022.

## Round 1 commentary findings (2026-11-18)
- Correction: $0354 is the strike count (not balls); $0355 balls; $0356 outs.
- $07A6/$07A7 = umpire voice sample request/delay (bank 1 DMC player), not a boot flag.
- New leads L-007 (NMI task table), L-008 (PlayState table), L-009 (runtime confirmations).

## Round 1 redirect (r1-redirect-a)
- All 14 L-002 tables plus 6 new bank-2 tables have extent proofs (lowest in-range target,
  selector clamps, following branch target/byte data); their handlers add 6,112 code bytes.
- Remaining unknowns dispositioned: anchored tables (L-013), pattern data (L-014), unreferenced
  routines (L-015), 20 bytes unresolved. $EFF0 is a static-walk artifact after BCS/BCC (L-010).
- Progression contract confirmed; ending credits reached with RAM patches.
- Records: analysis/redirect/r1-redirect-a/, analysis/handoffs/round1-redirect.md.

## Round 2 disassembly (2026-11-19)
Input maps/round1-disassembly-b.json (95.44%); output maps/round2-disassembly.json (verified byte-identical).
Independent verification of redirect leads (records in analysis/round2/):
- L-012 accepted: walk2.py rerun reproduces 6,112 new code bytes, 0 clashes; every table extent
  rechecked against its constraint (lowest in-range target, AND/LSR-CMP clamp, BNE target $D169,
  FrontEndScreen 88 words + 88-byte parallel table ending exactly at handler $C27F, $E37C selector).
  Three round-1 generic data regions overlapping tables (b2 $C1E6, b15 $C088, b11 $C51B) were
  replaced by exact table regions (+ remainder generic data, b2 $C227-$C27E byte table).
- L-013 accepted except bank 15 $FA14-$FA22: that "data" is an LDA #n/JSR $FA28 chain falling
  into observed $FA23 and called (JSR $FA14/$FA19/$FA1E) from the $F95B fragment -> Code.
  $EFF1-$EFFF item superseded by L-010 (whole 16-byte table $EFF0-$EFFF).
- L-014 accepted (81 ranges); 13 with code-like clean suffixes reviewed: pointer lists, ASCII,
  metasprite piece records, uniform runs. verify-props.json lists overlap/ref/fall-through checks.
- L-015: 35/36 islands accepted as Static code (conf 0.6; reachability unresolved, not Observed).
  Patterns: sibling entries ($830F, $9ECF, $E6A0/$E6A5/$E6C2, $DDDC), vector block (b6 $C911/$C917),
  helper routines (negate, 16-bit increment, bin->dec b5 $ECD8, distance b0 $A1B5), debug/editor
  code (b15 $C606 end-game shortcut via controller bits in $55; b2 $E6CC/$E73D). Rejected b2
  $DAC9 PLA/TAY/PLA/RTS (no prologue/reference).
- L-010 accepted.
- Residual review: b4 $D174-$D17B and b6 $C845-$C849 accepted as low-confidence sibling code
  fragments (flow into established RTS, identical/sibling bodies); b0 $8420 classified as a
  1-byte filler (a JSR opcode there would overlap the observed instruction start $8421).
  Still unknown (10 B, lead L-016): lone RTS b2 $D5FE, b6 $CA2E, b11 $C286, $C9C4, b15 $C8FE,
  $D9F4 and b2 $DAC9-$DACC. Unreferenced return opcodes lack sufficient structure.
- No new gameplay run: remaining unknowns are 1-4 byte orphan returns with no reference; redirect
  runtime evidence (runtime.json, scenarios.json milestones incl. ending) reused.

## r2-redirect-a (round 2 redirect)
- L-016: rejection premise re-examined (it excluded reachability, not code role); combined
  constraints -> proposal Code/Static/0.5 for all 10 bytes; reference searches negative
  (ptrsearch.json, refs.py, indjmp.py). Remaining limit: cannot prove `rts` vs `.byte $60` authorship.
- L-017 verified; L-019 found by survey of all JSR $849F/$9AA2 sites (only $C045 b3 and $D08B b11 untyped).
- L-011 static chain complete; runtime value-flow X1-X3 inconclusive, not repeated (sites already Observed).
- L-018/L-005 static checks only; L-004/L-009 not pursued (semantics only).

## Round 3 disassembly (2026-11-19)
- L-016 re-evaluated independently per range (evidence-policy "revisit reasoning"): the round-2
  rejection ("unreferenced return opcode, no matching prologue") rules out a known caller and
  reachability, not the code role. Combined constraints per range: (a) follows an unconditional
  transfer (JMP x5, RTS at b15 $D9F4, never-returning JSR JumpTableDispatch_24 + inline table at
  b11 $C286); (b) exact bounds fixed by established code on both sides (no bound ambiguity);
  (c) value is RTS / the PLA-TAY-PLA-RTS epilogue used by 4 classified bank-2 routines, never
  this ROM's fill ($FF 83,258 B, $00 40 B); (d) no reader: round-3 word re-search of addr/addr-1
  in owning bank + bank 0 found only the two known odd-straddle coincidences inside the bank-2
  $FBFD table (for $D5FE); r2 refs/indjmp searches reused. The competing "unread data" reading
  has no consumer, no fill role and no ROM precedent, so code vs data is not materially
  ambiguous; authorship (`rts` vs `.byte $60`) stays unprovable -> confidence 0.5. All 7 ranges
  accepted as Code/Static/0.5, reachability "no reference; unreachable as far as known"
  (regions 2207-2213). Not labelled Observed. b0 $8420 filler is unaffected ($20 is a 3-byte
  JSR opcode that would overlap the observed instruction at $8421, so it cannot be code).
- L-017 applied (regions 2081/2082/1111 labelled ForceHandlers/TagUpHandlers/CpuSideModeJumpTable;
  table entries re-checked as instruction starts; 2199/2200 corrected to table-reached Static, conf 0.8).
- L-019 applied: region 517 -> 2205 ScreenLoaderStepTable (6 B) + 2206 generic data (154 B);
  ScreenLoadStep ($44) writers re-checked in banks 0/3 (INC $C156, INC $C215, STA #0 $C228).
- Script: analysis/round3/tools/apply_r3.py; changes: analysis/round3/changed-ranges.json;
  pre-edit state copy analysis/round3/state.before.json.

## Round 4 disassembly (2026-11-19): L-021
- Restored analysis/state.json (identical regions to maps/round3-disassembly.json; pre-edit copy
  analysis/round4/state.before.json).
- L-021 verified independently, not by trusting F2: script asserts the 2 RAM words ($06C5/$06C4),
  3 table pointers ($91A5/$91B5/$91C5), first target = pointer-table end, 16-byte spacing ending
  at $91D4 (next byte = Observed LaunchPitchedBall $91D5) and the strict -2 descent of each
  table. Consumers re-read in source: LaunchPitchedBall LDA $919F,Y/$91A0,Y (Y = record byte 5 *
  2), LDA $919B,Y/$919C,Y twice (Y = (SysFlags&1)*2, index domain {0,2}), (Math4),Y with Y =
  RandomSpread result; bank 15 PlayState_22 $C4D3. No other operand reference into $919B-$91D4
  in banks 0/1 except the table's own pointer words (one coincidental byte pair at $9498 is not an
  absolute operand opcode).
- Region 1985 -> StaminaVarPtrs (4523+4), StaminaLimitPtrs (4527+6), StaminaLimitTable0/1/2
  (4533/4549/4565, +16 each); Data, Static, conf 0.95. Reachability: consumers Observed in r3
  redirect runtime; tables not claimed per-entry observed.
- Residual: byte-5 selector domain (0..2) not enumerated; extent proof does not depend on it.
- Toolkit: map kinds have no pointer-table type; specific role is carried by label/evidence (not a
  bug). No toolkit issues found.

## Round 5 disassembly (2026-11-19): L-023
- Restored analysis/state.json (identical regions to maps/round4-disassembly.json; pre-edit copy
  analysis/round5/state.before.json).
- L-023 verified independently (analysis/round5/tools/apply_r5.py asserts from ROM bytes before
  editing): 12 little-endian words at $FBFD = $FC15 + 64*i; first target = pointer-table end;
  12 x 64-byte tables end at $FF14, next byte = Static code region 508 ($FF15, physical 48933,
  context prg_banks [0,1,4,5]); byte 5 of every record in {0,1,2}, exactly one type-2 per team
  (matches 3-entry StaminaLimitPtrs); $FC2D = team 0 record 3 (demo pitcher copy).
- Consumers re-read: BuildPitcherRecord $D5FF and FindFreePlayer_DrawEra $D37D (LDA $FBFD,Y /
  $FBFE,Y with Y = EditTeam*2, then (Tmp0),Y with Y = pitcher*8 [+6]); InitDemoRosters_Fielding
  $C7F5 (LDA $FC2D,X, X=7..0). No other label/operand references into $FBDD-$FF14 in source.
- Regions 2108/506/2109/507 -> 2219 generic (48109+32, $FBDD-$FBFC, role unresolved: small values
  < $18, no reference), 2220 TeamPitcherPtrs (48141+24), 2221..2232 TeamPitchers00..11 (48165+64k).
  Data, Static, conf 0.95 (generic 0.8). One region per team (each a pointer target) rather than
  the recipe's single 768-byte block. Reachability: consumers Observed in earlier rounds; per-table
  reads not claimed observed.
- Unresolved: record bytes 0-4 and 7 meaning; $FBDD-$FBFC role. No toolkit issues found.

## Round 6 disassembly (2026-11-19): L-025 batch (includes L-024)
- Restored analysis/state.json (identical regions to maps/round5-disassembly.json; asserted by the apply script).
- Every batch item re-asserted independently in analysis/round6/tools/apply_r6.py (not by re-running the redirect
  script) and cross-checked to the exact 57 ranges/labels of batch-proposal.json; consumer index domains re-read
  from classified code with analysis/round6/tools/refs.py:
  - Bank 2 rosters: TeamRosterPtrs words = $E829+400t; consumers $D635/$D6DF (Y = EditTeam*2), $DFF2/$FF2E
    (+player*16); all 300 records have byte $0D 1-12, $0E 1-31; TeamRoster11 ends at $FAE9.
  - Lineups (L-024): $FB01+21t, positions exactly 1-9, players 8-24 distinct, starters 0-7, end at $FBFD.
  - Bank 14 close-up tables: consumer is bank-14 DrawBatterSprites ($C050/$C0C5, context [0,1,28,29]); side index
    {0,2}, form index PREC_FORM*2, frame index (SwingTimer>>1)&$3E. Frame targets never point into the tables;
    the lowest CloseupExtraSpriteFrames target is exactly $CA4A, which proves the 1-byte boundary move of
    region 1290 (232025 -> 232026). The remainder $CA4A.. stays generic Data (metasprite records, not labelled).
  - Bank 3 layout: six bases referenced by the loader, 44 bytes apart, last ends at Observed code $C156; PpuAddr
    words are $2000/$2400, ChrBank words < $10, pointer tables point >= $C156.
  - Named tables: PitcherRecordPtrs -> $04B3/$04A3; PitcherFormTiming (CMP $8BC3,Y, Y = PREC_FORM 0-8);
    PitcherFormChrBank split from the 2-byte handedness table $D516 (index 0/1); BaseCoordTable label only.
  - 15 small tables: StrikeCount/BallCount domains {0,2,4}; ($06BB>>2)&2; roster byte 3 & $FE <= 30
    (D_B11_C1BF 16 words); self-bounds (lowest target = table end) for D05F/D0B9/ECA3/DEA5/F699/CCA2 and
    stride-exact pointer runs for CD0D (24)/D079/C1BF/C30F/C36D/C3E9/CAA6.
- Reachability: unchanged claims (consumers Observed/Static as before); tables not claimed per-entry observed.
- Unresolved: D_B07_CD0D has 24 stride-exact entries while the team-slot consumer index seen covers 0..11
  (extent from structure, not domain); D_B06_CC82 index domain not traced (extent from the referenced parallel
  table and lowest target). No toolkit issues found.
