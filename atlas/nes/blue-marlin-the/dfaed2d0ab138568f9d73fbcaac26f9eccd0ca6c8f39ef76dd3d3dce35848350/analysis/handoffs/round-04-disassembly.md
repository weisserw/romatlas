# Round 4 disassembly handoff (2026-11-20)

Scope: independent evidence-policy review of ready leads L-022 and L-023 (analysis/redirect/r03-a/proposals.json,
analysis/handoffs/round-03-redirect.md). Input: checkpoint pass-003 (sha256 703aba69...; = maps/current.json =
analysis/state.json at start). No gameplay or emulator runs, because no residual has a runtime question that could
discriminate it (S01-S16 were all visited earlier). No toolkit or ROM change.

## Re-checks
- Recipe step 1: re-ran r03-a refcheck.py and twins.py on pass-003. Output is byte-identical to the stored .txt
  (compared with cmp; the scratch copies were then removed).
- Independent control-flow cross-reference: analysis/evidence/round-04/xref.py -> xref.txt. It walks every Code
  region (14,886 instructions on pass-003) and looks for JMP, JSR, JMP-indirect and branch operands equal to any
  residual address ±1.
  - All 38 hits target the established neighbours $E0F7, $ADB7, $B2EB and $B344.
  - The one $B2E9 hit is a bank-13 `bne`, a different bank.
  - Zero hits on a residual itself. This confirms the r02-a and r03-a negative reference results.

## Decisions

### L-022: bank-15 NOPs, ACCEPTED
- Bytes: 0x1E106 ($E0F6) and 0x1E1C3 ($E1B3).
- Change: Unknown->Code, 2 bytes, Static, confidence 0.6, context [-,-,14,15].
- Governing text:
  - "Combine independent constraints: consumer semantics, control/stack contracts, encoded origins, independently
    established endpoints ... can jointly establish structure"
  - "legal decoding alone still does not prove code"
- Constraints combined:
  - (a) Exact 1-byte bounds are fixed independently by classified code on both sides:
    - $E0F6 sits between the StopLoop `jmp $E0F2` and the observed routine $E0F7.
    - $E1B3 sits between `rts` $E1B2 and the observed IRQ-vector handler $E1B4.
  - (b) Control-flow contract: each NOP is a 1-byte instruction whose only successor is an established instruction
    start (an observed routine entry). If decoded as code, it is coherent with its neighbours and creates no new
    boundary.
  - (c) No competing data interpretation exists:
    - no consumer or reference in any encoding (r03-a Q1/Q2, xref.txt);
    - no filler mechanism in the ROM (all 7,998 Padding bytes are $FF; the next starts are odd, so not alignment fill);
    - every other small inter-routine island is consumer-anchored (lonebytes.txt).
  - (d) Consistency: StopLoop NOP $E0F1 (L-020, region id 3312) was accepted with exactly this evidence class. It is an
    unreferenced $EA whose only relation is falling into known code at $E0F2. Treating $E0F6 and $E1B3 differently
    would be inconsistent. So $E0F1 is kept, not reopened.
- Limits: reachability is not claimed, and the bytes are not asserted unused. Whether they are intentional
  padding-NOPs or leftovers is not determined. In either case the source form is a `nop` instruction.

### L-023: bank-1 trailing RTS, REJECTED (bytes remain Unknown)
- Bytes: 0x02DC6 ($ADB6), 0x032FA ($B2EA), 0x03353 ($B343).
- Governing text:
  - "An unreferenced return opcode or plausible pointer sequence still lacks sufficient structure on its own."
  - "legal decoding alone still does not prove code"
  - "retain unknown where code versus data remains materially ambiguous"
  - "Lack of execution or successful rebuilding alone is insufficient."
- Compact rejection review:
  - **Candidate claim:** each byte is a Static Code RTS ending FightFish_Update_fish_away, FightFish_Passive and
    FightFish_DirectionTerms, placed after the tail `jmp`.
  - **Decisive evidence:**
    - analysis/redirect/r03-a/twins.txt: the tail-call edit model is negative. There is no twin, and the sibling
      routines $B517 and $B535 end with the same `jmp ApplyDirection` but have no RTS.
    - refcheck.txt: all 18 other `JMP; RTS` followers in classified code are independently referenced.
    - analysis/evidence/round-04/xref.txt plus r03-a rtsdisp/splitcons/bankrel: no reference in any encoding.
  - **What these rule out:**
    - every tested reference route (direct, address-1, ±2, split lo/hi, immediate pair, script/hook, bank-relative);
    - every tested filler generator.
    - They do not supply positive structure for the RTS. They only remove competitors.
  - **Independent constraints considered together:**
    - exact bounds (shared by any 1-byte gap);
    - the ROM-wide absence of a filler mechanism.
  - **Why this is weaker than L-022:**
    - The RTS has no control or stack contract with any code. No predecessor can fall into it, because it follows an
      unconditional JMP. No successor is implied, because RTS does not flow onward. No caller is known.
    - So the only code-specific fact is the opcode value $60, which is exactly the case the quoted policy sentence
      calls insufficient.
    - The "no filler mechanism" census argues against one kind of data generator. It is not positive evidence of an
      instruction. A `.db`/leftover byte remains a concrete, untested-by-any-discriminator alternative.
    - There is no accepted precedent for an unreferenced RTS (refcheck.txt), so no consistency obligation applies.
  - **Strongest competing interpretation:**
    - Code: a dead epilogue left from a `jsr`->`jmp` edit. This is plausible but not established; the twin test was
      negative.
    - Data: a stray non-code byte with no generator identified.
    - Neither role has positive structure, so code versus data is materially ambiguous.
  - **Revised test that could change this:** any of the following would justify reopening:
    - a reference or consumer;
    - a second copy or analogue of the same routine ending `jsr X / rts`;
    - assembler or source-convention evidence, e.g. a demonstrable house style where every routine in this module is
      terminated by RTS regardless of a tail JMP. The current census contradicts that ($B517/$B535).
  - No further productive test is known. No runtime path exists, and every reference encoding has been searched.
    These 3 bytes are the terminal residual unless new evidence appears.

## Map changes / verification
- Applied with analysis/evidence/round-04/apply_l022.py (map.annotate; new region ids 3313, 3314); map.validate
  ok.
- Saved with map.save to analysis/state.json and exported with map.export to maps/current.json. Both have sha256
  4582999d1633428d4649053bfd38a65f0bc93eb15ee2b6491cbfe7281175979a.
- `python3 run.py checkpoint pass-004` produced maps/pass-004.json, generated/pass-004 and
  analysis/pass-004-verification.json. Result: ok, byte_identical true.

## Counts
ROM classification: 100.00% (262157 / 262160 bytes classified; 3 bytes unknown).
- Kinds:
  - Code 30,903 (Observed 20,357 / Static 10,546)
  - Data 78,807
  - Text 13,377
  - Padding 7,998
  - Image 131,072
  - Unknown 3
- Regions: 3,269.
- Per bank: analysis/evidence/round-04/perbank.md. Unknown bytes: bank 1 has 3; every other bank has 0. Bank 15 now
  has Code 6,416 and Unknown 0.
- Status: **INCOMPLETE**. The rounded 100.00% is not completion, because 3 unknown bytes remain.

## Remaining unknown ranges
| phys | bank / CPU | byte | status |
|---|---|---|---|
| 0x02DC6-0x02DC6 | b1 $ADB6 | $60 | L-023 rejected; terminal residual (review above) |
| 0x032FA-0x032FA | b1 $B2EA | $60 | same |
| 0x03353-0x03353 | b1 $B343 | $60 | same |

## Leads
- L-022: verified and applied.
- L-023: rejected. Retry only with one of the revised-test facts listed above.
- L-008: still deferred; no unknown byte depends on it.
- No new leads.

## Commentary follow-up
- Regions 3313 and 3314 are new 1-byte Static Code NOPs at $E0F6 and $E1B3.
- The commentary source should drop its "unknown" notes for these two bytes, and keep the notes for the 3 RTS bytes.

## Toolkit
No new toolkit issues found. TK-001..TK-004 are unchanged (analysis/toolkit-issues.md). The server was stopped.
