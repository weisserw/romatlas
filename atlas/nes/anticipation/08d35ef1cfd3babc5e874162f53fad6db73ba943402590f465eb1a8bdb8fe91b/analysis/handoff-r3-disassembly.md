# Round 3 disassembly handoff

ROM classification: 99.95% (65522 / 65552 bytes classified; 30 bytes unknown).

Accepted both R2 proposals:1 Static code byte0x2617 (cursor INY),9 Static code bytes0x5E7A..0x5E83 (initialized closed PPUMASK toggle). Independent [review](r3-disassembly/review.md), [fresh decode](r3-disassembly/decode.json), [checks](r3-disassembly/review-checks.json), [metrics](r3-disassembly/final-metrics.json). Static contracts only; no observed caller/natural reachability claim. Connected supported paths reviewed; no extra ambiguous extension. Leads R2-001/002 verified, history retained.

[Fresh checkpoint r3-disassembly-final](r3-disassembly-final-verification.json): byte_identical=true. Saved state/current/frozen map agree; all previous regions preserved. [Integrity](r3-disassembly/integrity.json). Input unchanged. Current source preserved independently at SHA256637ece766204ea0acf42e4909cc766e30f8f80b6e1093e4bee8e1dc10615fcb4; generated/r3-disassembly-final separate. Commentary update required for10 newly classified bytes and dependent uncertainty prose; preserve prior readable work.

Residuals physical half-open/CPU aliases under slots[0,1,2,3]:0x1A4E..0x1A51/$9A3E(3),0x2707..0x2708/$A6F7(1),0x5E61..0x5E7A/$DE51(25),0x5E83..0x5E84/$DE73(1). JSR prefix is double write with clobbered second coordinates, not infinite recursion; BPL overlaps accepted LDA boundary; earlier prefix lacks independent complete contract despite real flag-lock mechanics; final RTS isolated. No clear non-code pattern. Redirect may review residual contracts independently; no terminal exhaustion claimed.

Per-bank classified/unknown:header16/0; PRG0 8189/3,PRG1 8191/1,PRG2 8166/26,PRG3 8192/0; each CHR0..3 8192/0. Broad saved scenarios all four levels, feature/drop-out, late drawing/answer and artificial WINNER reused; natural correct-answer/win unresolved. No unchanged replay. No new toolkit issue; existing [TK-001](toolkit-issues.md) workaround unchanged. No toolkit edits/archive/deletion/cleanup/publication/commit. RPC stopped. Next commentary then redirect; no next worker launched.
