# Pass-002 compact handoff

ROM classification: 99.99% (393174 / 393232 bytes classified; 58 bytes unknown).

New verified checkpoint maps/pass-002.json; state and maps/current.json match.
198300 novel bytes accepted after independent primary review, plus one Text→Code
RTS correction81169. All Static, natural reachability unobserved.
Review: analysis/disassembly/pass-002/review.md; exact counts/bank ledger and
complement: counts.json/unknown-ranges.json in that directory.
New generated/pass-002 is byte-identical; analysis/pass-002-verification.json.
Input unchanged. Commentary/source/game.asm preserved and now predates this
classification batch: full current-source commentary is next separate phase.

Consumed L002–L006 verified. Produced L007 with exact residual and concrete
reasoning-review recipe.58 bytes: nine4-byte JMP0000 bank prefixes17..24/27;
bank23 special11F2:1200 object14; bank30 C0E9 RTS1; fixedEBAC00 byte1;
fixedF9F2:F9F8 sixRTS. Review exact competing interpretations in prior
redirect gap-layout-review.md. Unknown due actual structural ambiguity, not
missing semantics/natural reachability. No terminal exhaustion asserted.

NES-M5-001 remains one documented missing feature,0 confirmed/suspected bugs;
runtime milestones remain unavailable; startup failure unchanged and not retried.
No toolkit edits, deletion, archive or publication.
