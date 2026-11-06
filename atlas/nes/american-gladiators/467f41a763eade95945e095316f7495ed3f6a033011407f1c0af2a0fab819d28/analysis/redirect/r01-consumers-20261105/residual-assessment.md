# Residual assessment for next disassembly

This is a ready-proposal phase handoff, not terminal workflow exhaustion.
Current map still has118136 unknown bytes. The combined proposal covers118135;
only physical[$00CBDF,$00CBE0), one byte, is unproposed if all candidates are accepted.
Every actual current-unknown interval has a bank-aware disposition in unknown-complement.json.

Claim considered: classify$00CBDF=$60 as a one-byte sound return routine, or as data.
Effective CPU alias$8BCF uses lower physical banks6/7 with high14/15. Primary artifacts:
input/game.nes at$CBDF; commentary/source/game.asm around the FinishSoundSlotTick/
ContinueSoundSlotLoop boundary; research.py/instructions.json/barriers.json; complete
sound handler/callback roots and sound-command-contracts.json; candidate-batch.json.

The known preceding loop body already has its own conditional return/direct jump;
$8BCF is not a reached successor in the supported graph. The next supported routine
starts$8BD0. All supported sound handler cells and immediate callback roots were
inspected; none supplies this entry. That absence is not evidence of data or dead code.
A lone$60 has the same physical bytes whether it is an intentional alternate RTS,
a redundant copied epilogue or data/padding. No repeated typed-data layout, bounded
consumer or independent routine-family contract discriminates those interpretations.

What is rejected is the **role inference from the isolated opcode**, not the possibility
of static code and not code without a natural caller. Patching PC/stack to demonstrate
that RTS returns would only restate CPU semantics and cannot discriminate intended
storage role. No revised bounded test with a concrete supported prerequisite presently
survives. A newly found callback/table/root or independent copy/consumer contract can
reopen it. Do not repeat arbitrary execution or classify it just to reach100%.

No other candidate remains withheld for a missing natural selector, semantic name,
original generator or absence of gameplay fetch. Generic non-code and static routine
claims are already forwarded for independent review. Rejections should be localized
to their actual conflicting extent/premise, retaining unaffected proposals.
