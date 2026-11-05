# Round1 disassembly handoff

Incomplete. ROM classification: 83.45% (218781 / 262160 bytes classified; 43379 bytes unknown).

Code15111;Data203670;1539 regions, all structurally evidenced. Checkpoint pass-001
validated and rebuilt byte-identically; input unchanged. [Verification](pass-001-verification.json),
[accounting](round1/accounting.json), [map](../maps/current.json),
[source](../generated/pass-001/game.asm), [state](state.json).

Controlled level1/options, natural pause skip to next mission; artificial guardian
indices2/8/11 and late normal9 exercised. Artificial ordinary completion tail reaches
bonus and visible escape ending. [Recipes/checklist](scenarios.json), [gameplay](gameplay.md).
Early common-input-schema requests were ignored and produced attract mode; these are
explicitly corrected. Failed stage12 cheat bypassed terminal gate; do not reuse it.

7943 physical opcode starts observed; aggregate drop0, detail intentionally disabled.
Static role and natural reachability kept separate. CHR and patterned map/tile data
classified generically; unresolved semantics do not become unknown classification.
Residual43379 bytes chiefly PRG8-15, plus528bytes per even level bank. Exact residuals
and per-bank counts in [progress](progress.md) and [unknown complement](round1/unknown-final.json).

Ready redirect: bank10 command dispatcher finite extent R1-L03; fixed object/render
indirect handlersR1-L04; audio stream/table boundariesR1-L05. Gameplay exit/guardian
contracts and ending-to-terminal continuationR1-L07. [Leads](leads.json). These need
research after commentary; no terminal exhaustion claim. All accessible startup and
observed connected batches were handled; source commentary/editorial work remains.

Toolkit:2 confirmed doc/API bugs, no blockers; [issues](toolkit-issues.md).
No toolkit changes, archive, deletion or publication performed.
