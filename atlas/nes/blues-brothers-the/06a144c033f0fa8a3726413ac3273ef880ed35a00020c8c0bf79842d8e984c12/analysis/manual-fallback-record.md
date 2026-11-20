# Final Manual fallback record

The user explicitly authorized a batch-only exception for at most 10 residual physical ROM bytes: classify the validated remaining complement as generic Data with unknown meaning using Manual provenance. This authorization overrides further structural proof only for the listed residual bytes. It is not an observation or static proof of a data role.

The authoritative pre-annotation complement was exactly 4 bytes: physical offset 119404 length 1 (fixed 8 KiB PRG bank 14, CPU $D25C), and offset 120325 length 3 (CPU $D5F5–$D5F7). All contain $60. The supported map.annotate API applied exactly two Data/Manual regions. See [precheck](manual-fallback/before-counts.json), [action log](manual-fallback/rpc.jsonl), and [disassembly handoff](manual-fallback-disassembly-handoff.md).

Prior evidence left isolated return stubs versus return-valued data materially ambiguous. The [retained rejection review](redirect/round03-residual-review/rejection-review.md) and [historical terminal handoff](terminal-handoff.md) remain valid records of that uncertainty. Neither interpretation was established by the fallback, and the bytes acquire no invented consumer, format, reachability or game meaning.

The resulting map covers 131088/131088 physical bytes (100.00%), with 19070 Code, 112018 Data and zero unclassified bytes. Of those, 131084 bytes retain prior evidence-backed classification and 4 bytes are this separately disclosed Manual fallback. Unknown meaning is distinct from an Unknown region kind. This achieves coverage under the authorized exception, not evidence-backed whole-ROM structural proof.

Input ROM bytes remain unchanged. State, portable map and fresh byte-identical checkpoint are documented in the [fallback disassembly handoff](manual-fallback-disassembly-handoff.md). Current-source commentary and final editorial acceptance are separate gates, recorded in their subsequent handoffs. No archive, deletion, publication or toolkit change is authorized or performed.
