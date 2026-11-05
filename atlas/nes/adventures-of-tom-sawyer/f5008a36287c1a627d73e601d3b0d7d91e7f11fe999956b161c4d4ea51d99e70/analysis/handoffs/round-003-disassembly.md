# Round003 disassembly handoff

**Incomplete; commentary then targeted redirect required.**

ROM classification: 99.98% (262103 / 262160 bytes classified; 57 bytes unknown).

Checkpoint pass-003-final: byte-identical; map/state SHA256d04812ebf8f9948b33d274383decf2b38a6e0b28f79c2ce46f77dde2be7cbff5. Totals Code45274, Data85757, Image131072. New4481 Code/6070 Data, all Static; reachability separate. [Counts](../round-003/counts.json), [validation](../round-003/validation.json), [verification](../pass-003-final-verification.json), [primary independent review](../round-003/contract-review.md).

Original combined proposals reproduced exactly and decoded successfully; independent complete data review nevertheless found another embedded code block in bank9 BBD7..BFD0. Reject356 proposed Data bytes; accept299 coherent complete routines BD20..BE4B, retain57-byte BCE7..BD20 Unknown. Physical13CF7..13D30, effective context[8,9,14,15]. Branch/jump destinations BCE6/BCCF/BCAD lie in present template data, so neither legal decoding nor apparent obsolete-code similarity currently resolves intended structure. Neighboring template/pointer corpora remain Data. R003-BANK9-PARTIAL-SUFFIX contains concrete narrowed research recipe. No terminal exhaustion claim.

Review verifies original91-byte bank13 copy, local nonlocal exits, bounded channel pointers, CHR-only immediate mapper records, reset upper-slot aliases and standalone preconditions. Inherited A1DD-is-RTS wording corrected: it starts the recognizer reset arm. Bank9 text targets914A/915D are lower-slot pointers; don't use bank9 to infer their template dimensions. **Prior 'four-boss' summary was unsupported: actual evidence is representative flying boss only.** Six-stage/ending/flying-boss scenarios reused without new replay. Completed old records remain untouched.

Fresh generated/pass-003-final/game.asm needs full current-source commentary; prior commentary retained untouched. Whole-ROM final editorial/fidelity gates remain pending. No toolkit issues found; [report](../toolkit-issues.md). Disk9.1GiB; scratch4KiB; no toolkit edits, archive execution, cleanup/deletion or publishing.

[Exact57-byte residual and hypothetical decode/target bytes](../round-003/residual-evidence.json). A1DD arm resets F1 and returns at A1E1; no standalone RTS at A1DD.
