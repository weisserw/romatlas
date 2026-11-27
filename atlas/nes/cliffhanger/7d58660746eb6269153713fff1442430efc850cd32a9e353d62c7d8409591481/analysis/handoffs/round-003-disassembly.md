# Round003 disassembly — verified, incomplete

ROM classification: 99.99% (262144 / 262160 bytes classified; 16 bytes unknown).

Accepted both ready R002 leads:3378 bytes (3305 Code,73 Data), all Static; no natural reachability assertion. Independent exact re-decode1664views/54bounded batches; no old-region/start/data conflicts. All5680 prior regions including comments/labels/evidence/views preserved exactly. Review and explicit contracts: ../round-003/review.md. Conditional D80F matching-record RAM precondition retained; D86F internal continuation expects two pointer bytes on stack; EF7C LDX2/two-PHA/three-iteration/six-PLA count independently confirmed. D58E itself is not PRG-preserving; D5B8 and EFB8 restoration paths were checked.

Authoritative checkpoint round-003-disassembly: state.json,current map and maps/round-003-disassembly.json saved; validation ok. ../round-003-disassembly-verification.json confirms byte_identical=true, ROMSHA7d58660746eb6269153713fff1442430efc850cd32a9e353d62c7d8409591481,mapSHA9c5fffec6ea42674ab4f0af7196a6835cd9c86ffd4c3519e4da8ad292be662ea. Generated source generated/round-003-disassembly/game.asm is verified, not commentary approved.

Exact residuals: physical100415+1 bank12 $882F TXA;108048+6 bank13 $A600 JSR/JMP;108281+5 bank13 $A6E9 INC/INC/RTS;111386+4 bank13 $B30A LDA/STA. All remain structurally ambiguous; no speculative classification. Detailed alternatives remain ../redirect/round002-a/residual-review.md; new review confirms boundaries and lack of independent discriminating role constraints, not a universal caller requirement. No extra credible connected easy gain established. This is not terminal exhaustion.

Per-bank/counts/unknowns: ../round-003/bank-counts.json,counts.json,unknown-ranges.json. Evidence: audit-summary.json,exact-decodes.json,rpc.jsonl,accepted-regions.json; lead history retained in ../leads.json and lead-outcomes.json. Total26634Code/235510other classified. PRG12 retains1unknown,PRG13 retains15unknown; all other banks/header complete.

No repeated gameplay; existing natural level1/artificial stages4/8/14,ending/credits evidence remains. Boss combat/completion remains unconfirmed from prior explicit negative experiment. No new toolkit issues; existing confirmed NES-001 nonblocking via full-button-list workaround (../toolkit-issues.md). No toolkit edits/archive/publish/deletion/downloads. Scratch unchanged minimal;3.4GiB disk available at finish.

Next coordinator phase: full current-source commentary for3378 newly classified bytes, preserving prior readability; then redirect if useful. Final editorial/fidelity gates are separate. Do not run commentary/redirect inside this completed disassembly pass.
