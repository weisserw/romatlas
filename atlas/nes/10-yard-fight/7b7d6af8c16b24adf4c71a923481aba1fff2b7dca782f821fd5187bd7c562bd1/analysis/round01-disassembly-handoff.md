# Round01 disassembly handoff

**Incomplete. ROM classification: 94.88% (38876 / 40976 bytes classified; 2100 bytes unknown).**

10-Yard Fight, 1985 Nintendo/Irem NES title; NROM fixed effective PRG banks [0,1,2,3]. Classified Code 11,290; Data 19,394; CHR Image 8,192. Per-bank and exact residuals: [counts](evidence/round01/counts.json), [19 unknown intervals](evidence/round01/unknown-ranges.json).

Final checkpoint **pass-002** validates and rebuilds byte-identically: [verification](pass-002-verification.json), generated/pass-002; portable map maps/current.json (same as maps/pass-002.json), SHA-256 `ce45be0835a8cfcc257414a9ba00a884e3bfb25b46c8c9474a2159b14b959e04`. Baseline/pass-001/history retained; input unchanged.

Vector/static discovery plus supported NMI dispatch roots prove connected code. Reviewed constant branches prevent false code at $A9A8 sprite records, $BFAB period words and $C181 audio streams. Bounded generic data review includes three whole noncode gaps and independent uniform FF suffix $4C34-$800A. [Data reasoning](evidence/round01/data-review.txt), [instruction review](evidence/round01/broad-code-review.txt), [ledger](investigation.md).

Normal corrected controller-array route visits all five skill levels, with representative kick/pass/movement input trials and 1,667 accepted natural opcode starts; no new code beyond existing static batch. Artificial $807A initializer also confirmed later-stage displays. Timer/score patch alone failed; understood $8149 timer-check entry reached generic 7–0 win screen artificially. No natural win or absent-credits claim. [Scenarios](scenarios.json), [gameplay](gameplay.md).

Toolkit: **one confirmed protocol/API mismatch, no blocker**: documented button/pressed calls silently ignored by NES, complete buttons arrays work. Earlier controller-play claims withdrawn and preserved as superseded. [NES-INPUT-001](toolkit-issues.md).

Next: commentary must cover all currently classified source; this phase supplies structural evidence, not full readability acceptance. Redirect should inspect entry/selector/stack contracts for remaining code-like islands, especially physical $11C2-$128B, $2466-$25E5, $2B08-$2C4E, $2C8B-$2DF2, $4C1E-$4C34. Residual gaps can mix code and data; valid decoding alone is insufficient. Specific table schemas/roles remain open. Normal gameplay closure ceased adding code; no terminal research-exhaustion claim. [Lead outcomes](leads.json).
