# Round 3 redirect handoff (attempt r03-a, 2026-11-20)

Scope: lead L-021 (5 unknown bytes) and the coordinator's four corrected-inference questions. Input checkpoint
pass-003 (maps/current.json = maps/pass-003.json, sha256 703aba695f36...; 262155/262160 classified). Research
only: no map, state, source, toolkit or ROM changes, and no emulator run. Evidence: analysis/redirect/r03-a/ (README.md
has the recipes). Leads produced: L-022 (ready), L-023 (ready). L-021 is superseded by them.

## Residuals (exact unknown complement, refreshed from maps/current.json)
| phys | bank / CPU (context) | byte | previous instruction | next start |
|---|---|---|---|---|
| 0x02DC6 | b1 $ADB6 [-,1,14,15] | $60 RTS | `lda #$64 / jmp LineOut_Add` (FightFish_Update_fish_away) | $ADB7 FightFish_ReinitBigger |
| 0x032FA | b1 $B2EA | $60 RTS | `jmp FightFish_PassiveDirection` (FightFish_Passive) | $B2EB |
| 0x03353 | b1 $B343 | $60 RTS | `jmp FightFish_ApplyDirection` (FightFish_DirectionTerms) | $B344 |
| 0x1E106 | b15 $E0F6 | $EA NOP | `jmp $E0F2` (StopLoop self-loop) | $E0F7 (observed) |
| 0x1E1C3 | b15 $E1B3 | $EA NOP | `rts` $E1B2 | $E1B4 IRQ vector handler |

## Q1. RTS-dispatch and address-1 references (rtsdisp.txt, splitcons.txt)
- Does the ROM use RTS dispatch at all? The only `pha / pha / rts` construct in classified code is $DE2D in
  CallWithInlinePtr. It pushes the caller's return address advanced past the inline argument bytes. It is not a
  dispatch table. Its return lands right after a `jsr helper + inline args`. Every residual is preceded by a
  decoded JMP/RTS that ends exactly at the residual, so no inline-argument return can land on one.
- Words equal to addr-2..addr+2 anywhere in PRG: the Code hits are byte pairs that straddle two instructions (no
  operand hits; this agrees with r02 targets.py). The data hits are in bulk generic data of banks 3/6/7/9 and in the
  bank-15 $FE07 read cluster. None has a consumer that treats it as a pointer.
- Immediate lo/hi pairs (LDA/LDX/LDY within 16 bytes, either order) for addr-2..addr+2: none.
- Split lo/hi tables at any separation were tested from the consumer side. I took 634 pairs of indexed absolute
  loads that use the same index register within 10 instructions, in every orientation, and evaluated indices
  0..255 in the bank context. 20 raw hits came out. All are from index 54 to 253, past the real table ends and into
  code or other tables, except $DAD9/$D8D9+68. Those are metatile tile tables copied to $0300 (PPU queue), not
  addresses. The two known split jump tables (sound commands in bank 2, PpuQueueHandlers $C2D6 in bank 14) are
  fully labelled and contain none of the residuals.
- What this rules out: any static reference to these addresses in the encodings tested (direct, address-1, split,
  immediate, ±2).

## Q2. Script operands and hooks (script_parse.json refs, bankrel.txt)
- I checked every r01-a script ref (native-call $38, hook $04/$05, palette, nametable, var, indexed tables): none
  is within 8 of a residual. Bank-1 native-call targets are $A15F..$AB11 only. The hook vectors are $DE39, $E1C5
  and $F777..$F9FA. The fixed-bank native calls near the NOPs are $E2A4/$E2CC/$E2F4.
- Script handlers read CPU addresses as words (rw). That direct and address-1 domain is already covered by the raw
  word scan.
- No code converts bank-relative values or PRG offsets into execution addresses. The bank-relative/offset word
  hits are small values in generic data (bankrel.txt), so they are noise.
- RAM hook writers: the immediate-pair scan (16 bytes) finds no store of a residual ±2.

## Q3. Twin/analogue endings and consistency census (twins.txt, refcheck.txt, lonebytes.txt)
- **Tail-call twin model, not supported.** No other copy of these routines exists (preceding-context matches stop
  at 3 bytes). No analogue ends `jsr X / rts` for the same X:
  - `jsr $EDA3` (b0 $9485, $9570) is followed by `pla`.
  - The JMP-ended analogues in the same module have no trailing RTS: FightFish_Classify (`jmp StaminaClass`),
    FightFish_PlayerActive (`jmp ComputePull`), ActiveDirectionAndFatigue, and the sibling direction-term routines
    that end `jmp ApplyDirection` at $B517 and at $B535 (followed by data $B538).
  - So a trailing RTS is not a module convention. The jsr-to-jmp edit model is possible, but no twin justifies it
    independently.
- **Consistency of trailing RTS.** All 18 other `JMP abs; RTS` pairs in classified code have an independently
  referenced RTS (a branch target), or for $E0E0 the default FrameHook via an immediate store. No unreferenced
  trailing RTS has been classified, so the treatment of RTS bytes is consistent so far.
- **Consistency of NOPs, and an inconsistency.** Real NOP instructions in classified code are only the reset NOPs
  $E034/$E035 and the StopLoop at $E0F1/$E0F2. The other $EA bytes in Code are operand bytes. **$E0F1 was accepted
  in L-020 (pass-003) as Static Code.** It is an unreferenced NOP whose only structural relation is that it falls
  through into known code (the loop at $E0F2). $E0F6 (falls into the observed routine $E0F7) and $E1B3 (falls into
  the IRQ-vector handler $E1B4) have exactly that evidence class. They are now treated differently, which is
  inconsistent. The NOP before the NMI handler ($E124) does not exist, so this is not a "NOP before handlers"
  convention. The only `RTS; $EA` elsewhere ($C2D6) is consumer-anchored (PpuQueueHandlersLo).
- **Filler/padding mechanism.**
  - All 7,998 Padding bytes in the ROM are $FF.
  - Every other small non-Code island between code routines has a consumer: inline call arguments, tables read by
    adjacent code, TwoChoiceCursorX, observed-read clusters.
  - No other unexplained gap between routines exists anywhere in the ROM.
  - The next routine starts at $ADB7, $B2EB and $E0F7, which are odd addresses, and nearby routine starts are
    unaligned. That rules out alignment fill.
  - So no mechanism in this ROM produces non-code filler bytes between routines, and none of the residuals has the
    fill value.

## Q4. Policy assessment (evidence-policy.md text)
- Data: "Clear repeated byte patterns, uniform runs or visibly non-instruction content" is required to support
  generic data. A single $60/$EA has none of these. No consumer exists (r02 absrefs; Q1/Q2 above). A data role is
  unsupported.
- Code: "legal decoding alone still does not prove code" and "An unreferenced return opcode ... still lacks
  sufficient structure *on its own*". The proposal does not rest on the opcode alone. It combines constraints that
  are independent of the byte's value:
  - (a) A ROM-wide census: there is no non-code filler mechanism, fill is $FF, there is no alignment, and every
    other inter-routine island has a consumer. This is the policy's "encoded origins" constraint: the only
    supported generator for a byte here is a source instruction.
  - (b) Exact boundaries, set independently by the classified routines on both sides.
  - (c) For the NOPs, the accepted $E0F1 precedent, which has the same evidence class. Consistency requires either
    accepting them or reopening $E0F1.
  - Concrete competitors tested and rejected: alignment fill, inline argument, table head/tail (bounded indices),
    RTS-dispatch target, script/hook target, split-table entry. What remains is "an unexplained non-code byte with
    no generator". That is not a concrete interpretation that keeps code vs data materially ambiguous.
- Limits:
  - Reachability is not claimed. The RTSs are unreachable after the tail JMP. The NOPs have no known entry.
  - The tail-call edit history is not established (the twin test was negative).
  - Confidence is modest: 0.6 for the NOPs, 0.55 for the RTSs. The RTS case is weaker because there is no
    precedent of an accepted unreferenced RTS.
  - Disassembly review may reject L-023 by reading the policy sentence as requiring a twin or consumer. In that
    case, keep the 3 RTS bytes unknown as the terminal residual with this review.
  - L-022 should then still be decided consistently with $E0F1.

## Ready leads (recipe in analysis/redirect/r03-a/proposals.json)
- **L-022** (Unknown->Code 2, Static, 0.6): 0x1E106 ($E0F6) and 0x1E1C3 ($E1B3), bank 15. Comment: "NOP falling
  through into $E0F7 / IRQ handler; no reference; same class as StopLoop $E0F1; reachability not claimed".
- **L-023** (Unknown->Code 3, Static, 0.55): 0x02DC6 ($ADB6), 0x032FA ($B2EA), 0x03353 ($B343), bank 1, context
  [-,1,14,15]. Comment: "unreachable RTS after tail JMP; no reference in any encoding; no filler mechanism in ROM".
- Recipe:
  1. Re-run r03-a/refcheck.py and twins.py on the then-current map. Expect unchanged output.
  2. Annotate each as a 1-byte Code region with map.annotate: Static provenance, the comment above, and no label
     (or a local `@unreachable_rts` / `@nop` in source).
  3. Build/verify the checkpoint (byte-identical). Expected 262160/262160 classified.
  4. The final editorial audit remains required.
- If rejected: record the decisive objection against Q4 above. The residual is then terminal with this review. No
  further test is available: no runtime path exists, and every encoding has been searched.

## Other leads / scenarios / toolkit
- L-008 stays deferred; no unknown byte depends on it.
- Scenarios S01-S16 are all visited; no new run was made.
- No toolkit issue. No server was started.
