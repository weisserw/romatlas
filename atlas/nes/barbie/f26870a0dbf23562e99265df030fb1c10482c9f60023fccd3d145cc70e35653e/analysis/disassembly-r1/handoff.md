# Round 1 disassembly → commentary handoff

ROM classification: 71.05% (186253 / 262160 bytes classified; 75907 bytes unknown).

Incomplete. One disassembly pass, fresh verified checkpoint **pass-001-final**:
[verification](../pass-001-final-verification.json), [map](../../maps/pass-001-final.json),
[source](../../generated/pass-001-final/game.asm), internal [state](../state.json).
Code18465 / Data36606 / Image131072 / Text110. Header/CHR fully classified; PRG
per-bank counts in [counts](counts.json) and [progress](../progress.md).

Scope/evidence: 7188 first-batch decoded candidates plus1226 later accepted starts;
three discovery fallthrough tables corrected, all other supported instructions
annotated with explicit physical identities/views and observed/static provenance.
[Code review](code-review.json), [late code](late-code-review.json),
[contexts](contexts-all.json), [data boundaries](data-boundaries.json),
[trace summary](trace-summary.json). No natural reachability claim for artificial
state observations. All693 regions have evidence; semantic readability review
has not been done. Commentary must cover all current classified source.

Gameplay: natural corrected Start route visits sports-store scene16/stage4;
artificial scene13 bakery/platform,14 galleria,15 underwater,16 sports,17 music
initialized/exercised. Scenes5–11 include congratulations, soda exterior, bedroom,
shell, fountain and ballroom. Scene7 returned to title; ballroom advanced300
frames. Boss encounters and true ending/terminal flags unresolved, with concrete
stage-contract lead. [Scenarios](../scenarios.json), [gameplay](../gameplay.md),
[snapshot branches](shortcut-scenarios.json), [late branches](late-scenarios.json).

No initial leads; produced R1-001..003 in [leads](../leads.json). Redirect should
inspect remaining PRG graphics/table consumers (banks10/11), stage exit/boss flags,
and RAM upload/synthesis plus indirect dispatch contexts. Deeper bounds/contracts
require investigation; repetitive gameplay stopped after broad representative states.
Unknown bytes remain genuinely unresolved mixed code/data, not an inferred unused
category. Easy-gain stop only; no terminal rejection/exhaustion claim.

Toolkit report: [NES-001](../toolkit-issues.md), one suspected input-schema/API
validation issue, no blockers. NES input uses buttons array; button/pressed example
from shared GB docs silently releases NES input. Workaround verified. No toolkit
changes. Disk >6GiB free; all ROM/assets/state/snapshots/source remain local.
