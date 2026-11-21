# Round 1 disassembly handoff — incomplete

ROM classification: 99.94% (24577 / 24592 bytes classified; 15 bytes unknown).

| Section | Code | Data/image | Classified | Unknown |
|---|---:|---:|---:|---:|
| Header | 0 | 16 | 16 | 0 |
| PRG bank0 | 6755 | 1436 | 8191 | 1 |
| PRG bank1 | 4541 | 3637 | 8178 | 14 |
| CHR bank0 | 0 | 8192 | 8192 | 0 |

Checkpoint [pass-001-final](../pass-001-final-verification.json): map validation
and byte-identical ca65 rebuild passed. [Saved map](../../maps/pass-001-final.json),
[current map](../../maps/current.json), [state](../state.json). Map SHA-256
75d8105e19a68e00cad655743bcbc11b76ec970b0aad9b97fafe83352f60c04f.
Input unchanged. No toolkit modifications.

Scope: one disassembly pass, baseline established once, vectors/main-state and
four indirect-handler families reviewed; 5243 structurally supported instructions,
11296 Code bytes,5089 Data bytes,8192 CHR Image bytes. All109 regions carry evidence.
Explicit NROM effective banks [0,1,0,1], physical header-inclusive offsets.
[Structural notes](structural-notes.md), [decoded code](decoded-expanded.json),
[exact counts](classification-counts.json), [investigation](../investigation.md).

Gameplay: title identifies BurgerTime. Natural startup/menu/first layout; artificial
stage3/stage6 with rendered distinct layouts and representative movement/pepper
inputs; artificial GAME OVER screenshot verified. Six-stage selector loops1..6
while total-stage counter grows; no separate ending/boss contract found. Exact
routes, unsuccessful early shortcut, supported patches and progression checklist
in [scenarios](../scenarios.json); [game notes](../gameplay.md). Persistent snapshots
in this directory. Trace-disabled settling is explicit.3863 unique accepted physical
instruction starts (8167 bytes), mixed natural/artificial, aggregate_dropped0;
[roots](final-contexts.json), [coverage](final-coverage-all.json). No fetch conflicts.

Residual physical intervals and redirect leads:
- R01-001 [0x005D,0x005E), CPU C04D: extra RTS after scrolling routine.
- R01-002 [0x3485,0x3488), CPU F475: JMP F493 audio trampoline hypothesis.
- R01-003 [0x3609,0x3614), CPU F5F9..F603: priority-test alternate audio slice.

None was rejected as dead data; legal decoding/pointer values alone insufficient.
Review supported alternate-entry/precondition and shared consumer semantics before
accepting or rejecting; a live caller is not universally required. [Leads](../leads.json)
contain concrete bounded next tests. This is easy-gain handoff, not terminal
exhaustion; no rejection audit claiming no future method was performed.

Next: commentary across full currently classified source, then redirect structural
review of the three small residuals. Do not replay completed routes without a new
question. Data roles/sub-table semantics can be refined editorially; generic
non-code status is complete where evidence already supports it.

No toolkit issues found; [report](../toolkit-issues.md). Disk3.3GiB free; selected
round evidence36MiB, scratch80KiB. Nothing published, archived or removed.
