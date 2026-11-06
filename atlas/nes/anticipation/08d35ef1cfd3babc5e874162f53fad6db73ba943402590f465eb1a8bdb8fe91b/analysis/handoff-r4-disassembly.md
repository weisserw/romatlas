# Round 4 disassembly handoff

ROM classification: 99.96% (65525 / 65552 bytes classified; 27 bytes unknown).

Accepted R3-0013 Static code bytes0x1A4E..0x1A51 ($9A3E,slots[0,1,2,3]) as WriteSetupRectangleTwice. [Independent review](r4-disassembly/review.md), [fresh decode](r4-disassembly/decode.json), [complete bounds/stack/latch checks](r4-disassembly/review-checks.json). Two finite literal131x132 writes, source$A2FE..$E68C, second destination$00xx from clobbered X/A. First return resumes target setup, second returns to original caller. No same-coordinate, natural caller or purpose claim. Corrected primary comparison: ClearBothNametables calls$A3A7 then returns$A3A2 and falls through. All dependent consumer code/data already accepted. Lead verified with history.

[Fresh checkpoint](r4-disassembly-final-verification.json) byte_identical=true. Saved state/current/frozen map agree, prior regions preserved. [Metrics](r4-disassembly/final-metrics.json), [integrity](r4-disassembly/integrity.json). ROM unchanged; sourceb8380ee7f0580fb5bd0967bf3686b0ee2aa435261780beea853ce00e6a913d76 preserved independently. generated/r4-disassembly-final is separate. Commentary needs narrow3-byte prefix update plus dependent setup/uncertainty prose; preserve complete prior work.

Residual physical half-open/CPU:0x2707..0x2708/$A6F7(1),0x5E61..0x5E7A/$DE51(25),0x5E83..0x5E84/$DE73(1),slots[0,1,2,3]. Prior review of branch overlap, flag-lock/signature prefix and isolated RTS remains sound; no patterned generic non-code proof. No terminal exhaustion/editorial acceptance claimed. Per-bank classified/unknown:header16/0;PRG0 8192/0,PRG1 8191/1,PRG2 8166/26,PRG3 8192/0;CHR0..3 each8192/0.

All four levels,feature/drop-out,late drawing/answer and artificial WINNER evidence reused; natural typed victory unresolved. No replay or new useful runtime question. No new toolkit issue; existing [TK-001](toolkit-issues.md) unchanged. No toolkit edits/archive/cleanup/deletion/publication/commit. RPC stopped. Worker did not launch next phase.
