# Terminal workflow handoff

**Whole-ROM structural classification and current-source editorial acceptance complete.**

ROM classification: 100.00% (40976 / 40976 bytes classified; 0 bytes unknown).
Code 13,174; Data 19,610 (including header/vectors); CHR Image 8,192.
All four physical 8 KiB PRG banks, CHR and header are fully classified.
This conclusion follows independent structural review and final acceptance; it is
not inferred from completion of the coordinator goal.

Final checkpoint **pass-003**: maps/current.json and analysis/state.json SHA-256
`a2c637a340703bcd0cbafb8a4e8106d75d5ddbfa82060fd22bd82d615ca09960`.
[Checkpoint verification](pass-003-verification.json), [map/state validation](current-audit.json),
[coordinator verification](coordinator-verification.json), [requirement audit](coordinator-audit.md).
Input SHA-256 remains `7b7d6af8c16b24adf4c71a923481aba1fff2b7dca782f821fd5187bd7c562bd1`.

Readable local source: [game.asm](../commentary/source/game.asm),
[reading copy](../commentary/reading-after.asm). Final source SHA-256
`db996732dfc53607ee766f63aef9abac5211503312a20e9d5df3ff7a2bdcfa54`.
All **133 regions and 308 units** reviewed. [Final editorial handoff](final-editorial-handoff.md)
and [editorial audit](../commentary/editorial-audit.md) document corrections and
fresh byte-identical rebuild/reading refresh. Historical 309/49 counts and two
wrong PPU helper descriptions are explicitly superseded, with history retained.

Sequential phase records: [R1 disassembly](round01-disassembly-handoff.md),
[R1 commentary](round01-commentary-handoff.md), [R1 redirect](round01-redirect-handoff.md),
[R2 disassembly](round02-disassembly-handoff.md), [R2 commentary](round02-commentary-handoff.md),
[R2 redirect](round02-redirect-handoff.md). Six structural leads R01-R01 through
R01-R06 were independently verified: player dispatch/handlers and inline data;
seven-sprite routines/template; bounded text streams/sentinel; score and presentation
helpers; demo audio guard; 64-byte OAM upload with immutable-ROM store caveat.
Current leads.json retains all attempt history; no ready leads remain. Primary
contract reasoning and exact recipes are preserved in
[redirect findings](redirect/round01-contract-review/findings.md) and
[independent acceptance](evidence/round02/review.md). Final corrections take precedence
where old descriptions conflict; instruction bytes/classification did not change.

All five skill levels visited through normal menu/controller routes; supported
artificial timer-check shortcut reached a win screen. See scenarios.json,
[gameplay](gameplay.md) and [terminal review](redirect/round02-terminal-review/findings.md).
Natural win, every state selector, precise event semantics and separate-credits
absence are not established. These are explained limits, not unknown bytes or
interrupted work. No full hardware compatibility claim.

Toolkit: one confirmed nonblocking input API mismatch, NES-INPUT-001;
complete buttons arrays are the tested workaround. [Report](toolkit-issues.md).
No toolkit changes. All ROM/source/assets/state/captures remain local.

Archive instructions only: [selection rationale](archive-selection.md) and root
archive-options.json explicitly account for final/historical Markdown and duplicate
instructional copies. archive-document-audit.json checks the actual archive.py
selection contract without executing archive. No archive, cleanup, deletion or
publication was performed or is authorized by this handoff.
