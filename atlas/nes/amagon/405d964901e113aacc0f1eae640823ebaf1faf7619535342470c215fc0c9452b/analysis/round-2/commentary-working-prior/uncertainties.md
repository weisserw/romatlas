# Current-source uncertainties

Readability is complete for the current classified fragments. Opaque source slices
remain explicit unknown storage; a readable rebuild does not classify them.

## Incorrect NMI-root premise in phase-1 evidence

Immutable physical bytes [131082,131088) are `FE D8 6C C0 F9 FF`.
Little-endian vectors are NMI **$D8FE**, reset **$C06C**, IRQ **$FFF9**.
The baseline annotation instead says NMI $FED8. Source prose and the expanded
vector fields correct that byte-order error; the frozen map/state are preserved.
Effective fixed CPU slots are physical PRG8K banks 14 and 15.

The classified fixed-tail ranges [130792,130876) ($FED8..$FF2C) and
[130877,130892) ($FF2D..$FF3C) total 99 bytes. These retain static classification,
but the first routine contains repetitive $41 values and unofficial opcodes,
and the second reaches KIL. Their ordinary behavior cannot responsibly be named.
The true $D8FE handler preserves registers, DMA-copies OAM, runs PPU/controller
services and bank-switches audio, consistent with an NMI contract.

Decisive provenance: analysis/round1-disassembly/rpc.jsonl line 23 submits
physical 130792 / CPU 65240 ($FED8) as a discovery entry. Lines 8543–8544 annotate
the two ranges above; line 8546 records the reversed NMI vector. The original
wrong-offset attempts at lines 3 and 17 yielded no candidates and are distinct.
See analysis/round1-commentary/vector-root-review.json for compact exact evidence.

Redirect should determine whether either range has independent accepted callers,
observed fetches or a non-vector entry contract. A wrong root invalidates that
root justification; it does not by itself prove data or reject unrelated fixed-bank
code. Compare the bytes with adjacent metatile patterns and consumers before
proposing a structural correction. Do not require a natural caller to classify
independently supported data. No classification was changed in commentary.

## Local semantic limits retained in source

* Audio voice parameter $0528 and some command behaviors remain mechanically named.
  Seven 32-byte voice records, period/envelope fields and local phrase stack are
  established; not every musical instrument or sound ID has a recovered game name.
* UI decimal quantization has clear repeated subtract-five/add-five mechanics;
  the precise display role remains tentative.
* Actor control records and boss timing are explained mechanically. Handler bodies
  separated by raw gaps are not claimed complete, nor given invented original names.
* Seven upload descriptors have supported five-field structure and tile sources,
  but natural selection is unresolved. Static image classification is separate.
* Generic uniform data preserves the prior bounded neighbor/reference/code-conflict
  inspection. Asset/fill purpose is unresolved and not required for readability.

No additional semantic research is needed to finish current-source commentary.
Further structural work belongs to redirect/disassembly; final editorial acceptance
remains a separate pending gate.
