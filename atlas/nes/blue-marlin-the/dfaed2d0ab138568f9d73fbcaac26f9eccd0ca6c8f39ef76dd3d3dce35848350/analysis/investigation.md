# Investigation ledger

## Round 1 disassembly (2026-11-20)

### Hardware/bank model (static, from reset code)
- Reset $E000 (bank 15) writes $8000 = $06|$80 / $07|$80 (variable $43 = $80): MMC3 PRG mode 0, so CPU $C000 = bank 14
  and $E000 = bank 15 are fixed; R6 -> $8000 (shadow $41), R7 -> $A000 (shadow $42). Vectors: NMI $E124, RESET $E000,
  IRQ $E1B4 (bank 15). Observed contexts agree (e.g. [0,1,14,15], [12,13,14,15], [2,3,14,15] during sound calls).
- Far-call helper $E208 saves $41/$42, maps banks 2/3 and `jsr $8000` (sound driver entry, bank 2 starts with JMP table).
- RAM trampolines: $0033 = JMP ($34/$35 vector, default $E0E0 rts) and $0036 = JMP $E1C5 (IRQ disable). RAM execution
  observed at $0033/$0036 and $06E1/$06E4; not attributed to ROM (uploaded views unsupported).
- Joypad read $C025 (bank 14) with DMC-safe double read; pad state $46/$47, new presses $48.

### Inline-data call helpers (structural, static)
- $DD3E: `sta $b5; pla/pla` -> jump table of words following the JSR (index A). Entries begin at JSR+3.
- $DDEA: pops return address, consumes 7 inline bytes (y=1..7 -> $03C0..$03E0,x), returns after them.
- $DD6E/$DD79 -> $DDC4: consume 2 inline bytes (pointer -> $10/$11) and continue after them.
- $F366-style script entry and $F003 script-command dispatcher (table $F013, 75 entries; extent proven by the first
  handler $F0A9 immediately following the table).

### Code classification method
- Observed: opcode fetches from aggregate tracing (analysis/tools/covdump.py -> scratch/observed.json, merged).
- Static: analysis/tools/static_rd.py recursive descent from observed roots + vectors + resolved jump tables;
  rules: official opcodes only; a static block ending at BRK/unofficial opcode is discarded; JSR fallthrough is kept
  except for callees that begin with PLA (return-address consumers), with explicit inline-argument counts above;
  after JSR only the PC's own switchable slot keeps its bank; immediate MMC3 writes update the modelled slot;
  overlapping alternative decodings are rejected. Jump tables: `lda T,x/y; sta zp; lda T+1|H,x/y; sta zp+1; jmp (zp)`
  and $DD3E inline tables; extent ends at the first handler target located after the table, at known code, or at the
  first entry whose target is unresolvable/BRK/unofficial.
- Static regions carry provenance Static; reachability is not claimed for them.

### Data classification method (analysis/tools/classify_gaps.py)
Rules recorded per region in the evidence string: J (jump tables/inline args), T (ASCII text runs >= 8 bytes with
letters), P ($FF/$00 run to bank end), F (uniform runs >= 16), R (observed data reads from
analysis/tools/readtrace.py, clustered; neighbouring bytes only when no code-plausible decode exists at any offset),
N (unofficial/BRK opcodes at least every 16 bytes on all three decode alignments and no code-plausible decode),
W (consecutive 32-byte windows with <=8 distinct values, >=50% period-2 repetition, or >=2 unofficial/BRK opcodes on
all alignments, and no code-plausible decode in the run). "Code-plausible" = >=5 official non-BRK instructions
ending in RTS/RTI/JMP from some offset. Bytes failing all rules stay unknown.

### Leads / open questions -> analysis/leads.json
- Produced L-001..L-008 (fight win contract, day clock, Hawaii areas, unreached fixed-bank code, script format,
  RAM routines, data-bank residue, natural bite trigger). See analysis/leads.json.

### Round-1 outcome (checkpoint pass-001)
- Code 24,646 bytes (Observed 18,537; Static 6,109) in 626 regions; Data 67,641; Text 13,065; Padding 7,998;
  Image (CHR ROM) 131,072; unknown 17,738. Per-bank table: analysis/evidence/round-01/perbank.md.
- Remaining unknown is mostly (a) code-plausible but unreached gaps in banks 0/1/14/15 (e.g. $F74E-$F99B,
  $EEFD-$EF94, $F9FA-$FA26, bank 1 $976A-$991D), kept unknown because no caller/jump table/execution is known;
  (b) windows in data banks 8-13 where short plausible decodes block the conservative W/N rules.
- Static conflicts: 2 overlapping alternative decodings rejected (e.g. file 0x031f9, CPU $B1E9 in bank 1); the
  overlapping static path was not classified.
- Known limits: JSR-return assumption (callee returns to caller) is relied on for static fallthrough except for the
  listed inline-argument helpers; any other return-address consumer could make a static fallthrough wrong. Jump-table
  extents are heuristic (ends at first in-table handler target / known code / invalid target) and recorded per table.
- Evidence copies: analysis/evidence/round-01/{observed-coverage,data-reads,static_rd}.json.gz (gunzip into scratch/
  to rerun analysis/tools/static_rd.py, build_code_regions.py, classify_gaps.py, apply_map.py).

## Round 1 redirect, attempt r01-a (2026-11-20) — see analysis/redirect/r01-a/README.md and analysis/handoffs/round-01-redirect.md
- Attempted: L-009/L-010/L-011/L-012 bank-context checks (trace from bite/fight snapshots); L-001/L-002/L-003 progression
  patches (all visited); L-005 script parser; L-004 caller search (script operands + hook vectors); L-007 revised as a
  whole-bank bank-select contract (L-016).
- Correction to round-1 method: rule R (observed data reads) is contaminated by instruction fetches of code executed in
  readtrace windows without matching Instruction events; R-labelled Data in code banks must be re-checked (L-013).
- Remaining unknowns after the proposed batch (projection): banks 0/1 code-plausible residue without references,
  banks 2/4/6/7 sound/graphics residue, small bank 12-15 residues. Deferred reasons in the handoff.

## Round 2 disassembly (2026-11-20) — checkpoint pass-002; tools analysis/tools/r02/
- Batch A (analysis/tools/r02/plan.py -> apply.py): ready leads L-009, L-010, L-013, L-004, L-005, L-012, L-015, L-016.
  Checks before applying: static_rd_r01a code set ⊇ every pass-001 Code byte with identical instruction starts except the
  misaligned static region 0x03A3E (L-009, removed); overlap only Unknown 4,918 + Data 286 (rule R 102, rule W 184),
  no Text/Padding/Image overlap; script_parse roles overlap code only on the 63 native-island bytes; L-015 consumers
  re-located; L-016 bank-select audit re-run (bankaudit.py: 77 sta/8 sty $8001 sites, all in known code; values immediates,
  AreaMapBank or PLA restores; hook vectors all fixed-bank; banks 2/4 code never transfers into $A000-$BFFF).
  40 Data regions overlapped by code were removed and their remainders re-added with a trim note; remainder $ADB6
  (lone RTS, contaminated-cluster neighbour evidence, no read, no reference) left Unknown.
- Rule-R re-audit (L-013): raudit.py (consumers) + rcontam.py (instruction-fetch signature = chains of >=3 official
  instructions with per-instruction-uniform read counts). 29 hits outside the new code, all linear stream copies of
  4-byte sprite records / tile streams (inspected); rule-R Data retained. $929F-$92A6 has consumers lda $929F,x/$92A3,x.
- Batch B (plan_b.py): bounded inspection of the remaining gaps: consumer search (gapcons.py), record-layout dumps
  (gapdump.py, period4.py), text grammars (ptext.py PrintText; maptext.py MapText window text, bank 0). Classified:
  banks 6/7 metasprite/pointer data, bank 4 graphics streams + string terminators, bank 2 sound data after the driver
  (last code byte $8A14), table/pointer data with exact consumers in banks 0/1/12-15, bank-0 MapText control bytes,
  footer "BLUE MARINE Ver1" + 10 bytes before the vectors, and 3-byte alternate entries $ED52 (script $38 operand)
  / $EE09 (branch lands on observed instruction start).
- Correction inside the pass: bank-0 strings use the MapText grammar (MapText_Control table $8235: F1 row,col; F3 call;
  F4-F9 skip,ptr; FA newline; FB margin; FC goto), not PrintText; evidence of 40 regions rewritten, all 42 re-checked.
- Left unknown (L-017): code-shaped but unreferenced residues (bank 1 $A1B9/$A3ED-$A5DD/$A66C/$A7D6/$A802/$A8B1/$AEFF/
  $B424, bank 0 $811A/$9D43, bank 14 $C085/$C9F8/$CAB5/$DDA3/$DF76, bank 15 $E0EF/$E1B3, bank 2 $87A4, bank 12 $8A72,
  bank 13 $A69B, lone RTS bytes). Bank 1 $A51C/$A3DC/$A3EC show genuine data reads inside the $A3ED code shape -> unresolved.
- L-018: banks 2/3 rule-T "Text" regions are sound stream bytes (role mislabel, non-code either way).

## Round 3 disassembly (2026-11-20) — checkpoint pass-003; tools analysis/tools/r03/
- L-019 verified/applied: bank-1 fish residues ($A1B9 Data; $A1BA-$A21B, $A3F5-$A4C4, $A4DC-$A53F, $A560-$A5DD, $A66C-$A67A,
  $A7D6-$A81C, $A8B1-$A8DD, $AEFF-$AF23, $B424-$B475 Code; $A3ED-$A3F4, $A4C5-$A4DB Data; $A540-$A55F kept as jump table).
  Rule-R barriers at 0x0221E/0x0252C/0x02811 voided by the dummy-read correction; rule-W block 0x0254D+64 split (its first
  3 bytes are `jmp ($0016)`, last 29 bytes are the jump-table target stubs).
- L-020 verified/applied: b0 $811A, $9D43; b2 $87A4; b12 $8A72; b14 $C085, $C9F8, $CAB5, $DDA3, $DF76; b15 $E0F1 Code;
  b13 $A69B script-bytecode Data; b15 $E0EF-$E0F0 WarmBootSignatureRom tail Data.
- Structural evidence vs reachability: all new code Static; no caller/pointer/selector known (r02-a ptrscan/targets);
  not claimed unused or dead.
- Remaining: L-021 (5 lone bytes), left unknown pending redirect re-test per coordinator note.

## Round 4 disassembly (2026-11-20) — checkpoint pass-004; evidence analysis/evidence/round-04/
- L-022 verified/applied: b15 NOPs $E0F6, $E1B3 -> Static Code (0.6). Combined: exact bounds by classified neighbours, NOP falls
  into an observed routine entry ($E0F7 / IRQ $E1B4), no reference (xref.txt, r03-a), no filler mechanism, consistency with $E0F1.
- L-023 rejected: b1 trailing RTS $ADB6/$B2EA/$B343 remain Unknown. Only code-specific fact is the opcode; no control/stack
  contract (after unconditional JMP, no caller); twin test negative (siblings $B517/$B535 end JMP without RTS); no accepted
  precedent of an unreferenced RTS. Reopen only on a reference/consumer, a `jsr X / rts` analogue, or module-convention evidence.
- ROM classification: 100.00% (262157 / 262160 bytes classified; 3 bytes unknown). Status INCOMPLETE.

