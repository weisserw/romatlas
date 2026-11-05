# Round2 disassembly handoff

ROM classification: 100.00% (262152 / 262160 bytes classified; 8 bytes unknown).
Exact99.99694842844065%; INCOMPLETE. Code30338/Data231814; new15227Code+28144Data,
allStatic, accepted entire43371-byte exact proposal batch after independent review.
Every old annotation preserved exactly. Classification and natural reachability separate.

Actual current checkpoint **pass-003**: [map](../maps/pass-003.json),
[verification](pass-003-verification.json), [generated source](../generated/pass-003/game.asm).
byte_identical=true; map/state SHA4e8db6a04eb66e103f84903691c4c75900553daf826f7edf62392b2e1be2082a.
pass-002 is a preserved verified duplicate of pass-001 created by a shell sequencing
mistake after rejected annotation-ID requests; no classification gain there.
No history/source/checkpoint removal. ROM unchanged, configured binary retained.

[Review](round2/review.md), [exact complement/per-bank counts](round2/accounting.json),
[decoder audit](round2/decode-audit.json), [data audit](round2/data-audit.json),
[acceptance](round2/acceptance.json). Primary redirect evidence remains
[findings](redirect/r1-20261105-a/findings.md) and exact proposals/manifest there.
No accepted proposal rejected. All7227 proposed starts redecoded, all159 sprite
records/74 sequences/15 matched actor state domains/11 screen domains checked.
Full1KiB renderer identity is false on stage initialization tails; only instruction
slices match. Variable tails independently support generic record data. See review.
Only source-prefix-closures-v2/screen-irq-v3 authoritative; invalid roots, alias-only
reference claims and actor exploratory bank12 target remain excluded. Mapper slot
and nonlocal stack contracts retained; code is Static, no natural execution claim.

Four exact unknown physical half-open intervals, all bank14:
1D0FD..1D100 (CPU D0ED JMP CF65),1D4F5..1D4F6 (D4E5 RTS),
1D59B..1D59E (D58B JMP CF65),1DACE..1DACF (DABE RTS).
Isolated encoding/adjacency insufficient independent structure; no dead-code/data guess.
R1-L03/04/05/06/07/08/09 verified; residual question R2-L01 records next recipe.

Current old commentary remains preserved but does NOT cover all newly classified
source. Next complete full current-source commentary, then fresh redirect on actual
8-byte complement. Easy-gain phase complete; no terminal exhaustion/rejection review
claimed. Prior bounded broad progression and ending/final-score evidence reused;
no changed gameplay question called for repetition. Full editorial gate outstanding.
Toolkit:0new issues; existing2confirmed documentation/API mismatches,0suspected,
0missingfeatures, no blockers; [report](toolkit-issues.md). No toolkit edits,
publication/archive or evidence cleanup. Coordinator owns archive decisions.
