# Round 2 — redirect handoff (attempt r02-a)

Status: **DONE — ready leads L-019, L-020** (1,039 of 1,044 unknown bytes proposed; 5 lone bytes left with a rejection review).

## Scope and checkpoint
- Input: checkpoint **pass-002** (maps/current.json = maps/pass-002.json, sha256 c6d84b24d5a969b1353e5443aca4b1ca2be098aa0ce21a7019dddadf53f5d425;
  1,044 unknown in 25 ranges). Started from analysis/handoffs/round-02-commentary.md (structural questions 1-5), L-017, L-008, L-006.
- No map/state/source/ROM changes. Research dir: analysis/redirect/r02-a/ (r01-a untouched). Tools there:
  r2dis.py (bank-aware linear decode with map kinds), absrefs.py, targets.py, ptrscan.py (reference searches),
  dummyreads.py (branch dummy-read audit), dummytest.py (runtime check), validate.py (structural check of all proposals),
  proposals.py -> proposals.json (exact ranges), ledger.md (unknown complement with dispositions).
- One `run.py serve` via analysis/tools/rpcd.py for the runtime check (stopped). Scratch untouched except reads.

## Key finding 1 — the "data-read" conflicts are 6502 dummy reads (method correction)
Round-1 readtrace.py recorded every ROM read outside the current instruction's bytes as a data read. A taken relative
branch that crosses a page performs a dummy read at (next-PC high byte | target low byte); those were recorded as data.
- Static audit (dummyreads.py -> dummyreads.txt): 63 page-crossing branch dummy addresses in classified Code;
  39 coincide with recorded "data reads", most on bytes that are themselves Code. Three rule-R Data singletons are
  explained entirely by it: 0x0221E ($A20E, branch $A2FB->$A30E), 0x0252C ($A51C, Fish_Wander `bne $A61C` at $A5E0),
  0x02811 ($A801, $A8EF/$A8F8->$A901).
- Runtime confirmation (dummytest-results.txt; snapshots sea / hawaii_sea, 30 frames, no patches, Read+Instruction
  trace on 0x2200-0x2A00): every read of those three bytes immediately follows the named branch instruction.
- Consequence: the "$A51C data reads inside the $A3ED cluster" conflict (L-017 note) and the barriers at $A20E/$A801
  are void. The $A3DC/$A3EC reads are genuine FishProbeDX/DY index-7 reads (outside the residual).

## Key finding 2 — the bank-1 $A3ED-$A5DD cluster is an older fish probe/bite code set
- $A3ED-$A3F4: FishProbeDY entries 8-15. Extent: index is FishFacing, whose writers bound it to 0-15 (AND #$0F,
  inc wrap at $10, dec wrap to $0F, reverse tables) and FishMoveYHandlers has 16 words; DX/DY are 16-byte tables.
- $A3F5-$A494: variant of FishShadow_TryBite ($A940) — |lure-shadow| distance, range list #$A4D7 / chance list #$A4DA,
  FishOnLine 1/0; $A495-$A4C4 chance roll from tables by LineChoice/LineLengthSetting/BaitFreshness (bank-0 twin $9D43).
- $A4C5-$A4DB: the operand tables of those routines (23 bytes; between RTS $A4C4 and code $A4DC).
- $A4DC-$A5DD: offset helpers, a FishFacing-indexed `JMP ($0016)` dispatch through the 16-word table $A540 (already Data)
  whose 16 targets are exactly the instruction starts of the stub block $A560-$A586, a land-cell check ($A587, same
  cells as Fish_CheckLandY) and four +/-$18 helpers ending at RTS $A5DD before observed Fish_Wander.
- All internal references close; validate.py: exact decode, every branch/JSR/JMP target a known or proposed instruction start.

## Ready leads (details and exact ranges: analysis/redirect/r02-a/proposals.json; checks: validate.txt)
| Lead | Content | Byte effect |
|---|---|---|
| L-019 | bank-1 fish residues: $A1B9 Data (FishParamTable end, AND #$3F), $A1BA-$A21B Code, $A3ED-$A3F4 Data, $A3F5-$A4C4 Code, $A4C5-$A4DB Data, $A4DC-$A53F Code, $A560-$A5DD Code, $A66C-$A67A, $A7D6-$A81C, $A8B1-$A8DD, $AEFF-$AF23, $B424-$B475 Code | Unknown->Code 734, Unknown->Data 32, Data->Code 48 (rule-R 0x0221E-0x0222B 14, 0x0252C 1, 0x02811 1; rule-W 0x0254D-0x0254F 3 and 0x02570-0x0258C 29 — region 0x0254D+64 is split, 0x02550-0x0256F stays Data as the jump table) |
| L-020 | other banks: b0 $811A, $9D43; b2 $87A4; b12 $8A72; b14 $C085, $C9F8-$CA28, $CAB5-$CAF4, $DDA3-$DDAE, $DF76-$DF9A; b15 $E0F1-$E0F5 Code; b13 $A69B-$A6A2 Data (script subroutine), b15 $E0EF-$E0F0 Data (WarmBootSignatureRom bytes 15-16, cpx #$10) | Unknown->Code 263, Unknown->Data 10 |

Combined: Unknown 1,044 -> 5 (Code +1,045 incl. 48 from Data; Data +42 -48). Reachability: no caller/selector found
for any proposed routine (ptrscan.py: no words, word-1 or immediate pairs in banks 0/1/14/15; targets.py: no
branches/JSR/JMP); classify as **Static** code with "no known reference" notes, never Observed, never "dead".
Recipe: re-run `python3 analysis/redirect/r02-a/validate.py` on the input map; apply proposals.json with map.annotate
(Static provenance, rationale per entry; split Data regions 0x0221E/0x0221F, 0x0252C, 0x0254D+64, 0x02811);
fix the trimmed-region notes; label suggestions are marked "(suggested)"; checkpoint pass-003 + verify.

## Rejected / negative results
- Pointer-table reachability for the residues: none (ptrscan/targets outputs); not a barrier to static classification.
- Rule-R singleton re-audit in banks 0/1/14/15 (all 95 regions of 1-2 bytes, inspected by neighbours/bytes): no further region decodes as code or
  is solely dummy-read supported; the regular bank-14 singletons at $xx46/$xxF5 lie inside the $DAD9-family tables
  (indexed page-cross dummy reads within table data) — Data unchanged.
- $A69B as code: rejected (02 0A 0C 90 ... is valid script bytecode ending in Return).

## Unresolved (5 bytes) — compact rejection review
Candidates: RTS 0x02DC6 ($ADB6, after `lda #$64 / jmp $EDA3`), 0x032FA ($B2EA, after `jmp $B4D7`), 0x03353 ($B343,
after `jmp $B3C5`); NOP 0x1E106 ($E0F6, after the stop loop) and 0x1E1C3 ($E1B3, between RTS $E1B2 and the IRQ handler
$E1B4 = IRQ vector).
- Claim tested: each byte is code (dead epilogue/filler) or data.
- Decisive evidence: bytes and both neighbours (r2dis.py); targets.py/ptrscan.py show no branch/JMP/JSR/word reference;
  indexed operands with a base within 255 bytes below (absrefs.py) have bounded small indices ($AD58,y FishFacing 0-15;
  $B23A-$B247,y = $05B0*4+$05BF table rows; $E0E1,x x<16), so no known table consumer reaches them.
- What it rules out: a reachable entry from known code/tables; a known data consumer.
- Constraints considered together: exact boundaries (previous instruction ends at the byte, next routine starts right
  after) and opcode value. These fit a leftover RTS after a jsr->jmp tail-call edit or alignment-free filler NOP, but
  evidence-policy.md states an unreferenced return opcode lacks sufficient structure on its own; no consumer or pattern
  supports a data role either.
- Strongest competitor: filler/padding bytes (not executed, not read) — equally unsupported.
- Remaining reason: one-byte, value-only evidence; no revised test identified (no runtime path can reach them).
  If the coordinator accepts a "trailing unreachable instruction between contiguous routines" convention, classify as
  Static Code with that note; otherwise these 5 bytes are the terminal residual.

## Other leads
- L-017: superseded by L-019/L-020 (+5-byte residual above).
- L-006: resolved — the RAM routine source is established statically (commentary r1); no unknown bytes depend on it.
- L-008 (natural bite): remains deferred — every bite routine (Fish_CheckBite observed, FishShadow_TryBite static, the
  two variants now proposed) is classified/proposed; natural reachability adds no classification value. Recipe kept.
- Round-1 static conflicts 0x031F9 / 0x08013: already classified; no residual depends on them.

## Scenarios
All milestones S01-S16 visited (analysis/scenarios.json); no unvisited/attempted milestone remains. This pass only
re-ran the sea / Hawaii sea snapshots for the bounded dummy-read check (recorded under S05/S13 notes).

## Toolkit
No new toolkit issue: the dummy reads are correct 6502 behaviour reported by the trace; the defect was in the
workspace script analysis/tools/readtrace.py (filter only skipped reads within the current instruction). Recommended
fix for future read traces: also skip the branch page-cross dummy address (and indexed page-cross unfixed address).
