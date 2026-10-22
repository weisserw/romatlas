# Round 001 disassembly handoff

ROM classification: 48.39% (63432 / 131088 bytes classified; 67656 bytes unknown).

Single disassembly phase complete, structurally incomplete. Proceed to commentary; all currently classified source is new and needs full readability work. No commentary or redirect phase was started by this worker.

Checkpoint: [map](../../maps/pass-001-final.json), [verification](../pass-001-final-verification.json), [source](../../generated/pass-001-final/game.asm), [internal state](../state.json). Validation and whole-file byte-identical rebuilding passed; ROM SHA256 unchanged. Prior baseline and pass-001 preserved. Live server stopped after save/export.

Accepted 12,242 Code, 18,422 Data and 32,768 Image bytes, including header/vectors. Effective physical identities/CPU views preserved. Key readable anchors: fixed reset $FEE0, NMI $C000, graphics upload $C301-$C357, split list pointers $C358/$C363, descriptor lists $C36E-$C42B, mapper setter $FFDE, cooperative scheduler $FF38/$FF6A, bank12 object dispatcher $8000. Generic asset gaps have deliberately unresolved semantics, not unsupported invented layouts.

Review nuance: four bytes at physical82507..82510 were rejected as code because LDA #F8 / STA / BNE cannot fall through into table bytes. Discovery is a hypothesis; do not reimport rejected path. Static and observed provenance remain distinct; later Amazon fetches may be artificial after RAM007D=1. Unknowns remain raw source fallback and are excluded from completion.

Evidence and decisions: [investigation](../investigation.md), [accepted instruction inventory](classification-code-accepted.json), [graphics schema](graphics-structure.json), [uniform slices](uniform-data-review.json), [unknown complement](unknown-ranges.json), [per-bank counts](bank-counts.json). Baseline classification-free rebuild was independently verified before analysis.

Gameplay: clean [menu snapshot](../stage-select.snapshot) branches to all five starting lands; [scenarios](../scenarios.json) records held inputs/releases, budgets, observations, patches and progression checklist. Representative movement/jump/pogo, terrain and enemies exercised. Later Amazon reached ruins/vine terrain, then respawn; no boss/ending confirmed. RAM007D patch only persisted after initialization and does not prevent pit failure. Further long unchanged button play is low-yield without room/position/transition contracts.

After commentary, redirect should investigate connected actor-dispatch contracts and handlers, progression flags/room initialization for bosses/ending, and packed-data consumers in physical8KiB banks8/9. Concrete recipes and scoped prior outcomes: [leads](../leads.json). These need research, not blind pointer-to-code annotation. No terminal exhaustion claim.

Toolkit finding [NES-001](../toolkit-issues.md): shared docs describe incompatible input/discovery parameter forms; NES uses buttons arrays and max_instructions. Workaround confirmed; toolkit untouched. Disk had4.9GiB free; retained evidence and snapshots remain local. Only supported portable map is ROM-free sharing material.
