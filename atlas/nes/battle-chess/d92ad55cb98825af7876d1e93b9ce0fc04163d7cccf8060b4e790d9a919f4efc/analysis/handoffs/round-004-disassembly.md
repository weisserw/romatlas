# Round004 disassembly handoff

ROM classification: 99.99% (262135 / 262160 bytes classified; 25 bytes unknown).

Fresh `pass-004-reviewed`, map/state SHA256 `8a795941fc84238feed1847b172e3cdfdaa272a53230c9950e98dd3401c2d55b`. Validation and byte-identical ca65 rebuild passed. Accepted19 Static Code bytes/13instructions; R003-L13/L14 verified. [Independent review](../evidence/pass-004/independent-review.md), [fresh primary decoder audit](../evidence/pass-004/independent-audit.json), [summary/per-bank counts](../evidence/pass-004/final-summary.json), [exact nine-interval25-byte complement](../evidence/pass-004/unknown-complement.json).

Primary claims localized: bank26 $8D21 initializes SP/D/EB then falls through to known initializer; no whole-body normal return. $8F24 suffix maps valid low-nibble indices0..7 to complement of independently known masks and returns with ordinary frame; preceding LSR/LSR/PLA remains unknown. Residual24bank26 +1bank30 bytes retain short NOP/RTS/constant-return and status/stack/table competing interpretations. No obvious supported unknown successor; no forced data. Ready phase handoff, not terminal exhaustion.

Next full current-source commentary then redirect. Old commentary/source/game.asm and symbols retained byte-for-byte; new map source/readability review pending, final editorial audit pending. Reuse round003 structural review and saved experiments; no unchanged runtime replay. Execution coverage unchanged, no new Observed bytes; no natural ending/full cinematic return or full compatibility claim.

No toolkit issues found; [report](../toolkit-issues.md). No ROM/toolkit/commentary edits, agents, cleanup/deletion, publication/archive. No live server. Storage inspected at handoff.
