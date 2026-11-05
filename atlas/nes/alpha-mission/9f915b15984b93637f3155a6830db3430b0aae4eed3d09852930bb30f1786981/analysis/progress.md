# Disassembly progress

Workspace initialized. Baseline and classification have not run.

## Round 001 — disassembly active
Input checkpoint: initialized empty analysis/state.json (loaded). Baseline established:
[maps/baseline.json](../maps/baseline.json), [verification](baseline-verification.json), byte-identical true.
CNROM mapper 3, 32 KiB fixed PRG, 32 KiB CHR; entire physical image 65,552 bytes.
Validated NMI/reset/IRQ vectors $815D/$8042/$8293, effective PRG slots [0,1,2,3].
Static vector-root discovery yielded 673 instruction candidates, no diagnostics; inspection underway.
Startup reached Alpha Mission title at frame 147; first 180-frame trace request hit wall-time
budget and retained partial state. Zero detail capacity intentionally drops details while
aggregates remain complete (aggregate_dropped=0). Subsequent requests use smaller budgets.
Evidence: [disassembly-001](disassembly-001/), active file-polling RPC driver/server.

Active checkpoint: [disassembly-001-initial](disassembly-001-initial-verification.json), verified byte-identical.
54,534 / 65,552 bytes structurally classified at this intermediate checkpoint.
Progression: corrected NES input syntax to `buttons:[...]` (generic `button/pressed` silently ignored).
Earlier fire/start route names describe intended inputs only; those captures are unmodified attract mode.
Real player reached after two Start edges separated by released input at title.
Artificial stage initialization accepted scroll checkpoints $0070 (area6), $00FC (area12),
$011A (area13), using RAM $28=0,$29/$2A=scroll,$25=$CC,$DF=20; 180 initialization
frames plus 20 action frames with A+B+Left. Each accepted fetch trace has no aggregate drops.
Further discovery: independently bounded RTS directories (actor, stage commands, weapons,
armor and collision handlers) expand to 6,243 coherent official instruction candidates,
12,280 bytes. Inspection/annotation underway. Final base command found at scroll $0116;
representative boss and armor scenarios remain to be exercised.

## Round 001 — disassembly completed

Input checkpoint: initialized empty `analysis/state.json`; no prior classifications.
Output checkpoint: **[disassembly-001-final](disassembly-001-final-verification.json)**.
State [state.json](state.json), portable map [current.json](../maps/current.json),
checkpoint map [disassembly-001-final.json](../maps/disassembly-001-final.json),
generated [game.asm](../generated/disassembly-001-final/game.asm).
Earlier baseline and intermediate checkpoints remain preserved.

**ROM classification: 100.00% (65552 / 65552 bytes classified; 0 bytes unknown).**

Physical roles: 12,664 Code; 20,054 Data (including 16-byte header); 66 Text;
32,768 Image. 135 annotated regions, all with labels and structural evidence;
code regions have explicit CPU execution views and fixed PRG context [0,1,2,3].
All annotations use Static provenance. Runtime evidence supplements their contracts;
this does not claim every static instruction or table entry naturally executes.

| Bank | Physical interval (hex, end excluded) | Code | Data | Text | Image | Unknown |
|---|---|---:|---:|---:|---:|---:|
| Header | 00000–00010 | 0 | 16 | 0 | 0 | 0 |
| PRG 0 | 00010–02010 | 6024 | 2102 | 66 | 0 | 0 |
| PRG 1 | 02010–04010 | 6121 | 2071 | 0 | 0 | 0 |
| PRG 2 | 04010–06010 | 519 | 7673 | 0 | 0 | 0 |
| PRG 3 | 06010–08010 | 0 | 8192 | 0 | 0 | 0 |
| CHR 0 | 08010–0A010 | 0 | 0 | 0 | 8192 | 0 |
| CHR 1 | 0A010–0C010 | 0 | 0 | 0 | 8192 | 0 |
| CHR 2 | 0C010–0E010 | 0 | 0 | 0 | 8192 | 0 |
| CHR 3 | 0E010–10010 | 0 | 0 | 0 | 8192 | 0 |

Substantial supported gains: vector/state graph; actor/boss/weapon/armor/collision/
item RTS directories and connected control flow; renderer/PPU/audio/terrain routines;
finite-domain lookup arrays; repeated sprite/motion/terrain/PPU record blocks; hardware
CHR pattern banks. Exact per-range evidence is in state/map and
[inspection artifacts](disassembly-001/), with [data review](disassembly-001/residual-data-review.json)
and [tail pattern review](disassembly-001/data-pattern-review.json). Generic Data
means non-code is supported while constituent semantics/organization may remain unresolved.
Isolated helper contracts and natural reachability are separately recorded in leads.

Gameplay: title/attract progression, correct real-player Start route, artificial later
areas6/12/13, active first-boss fight, armor menu and Octo activation. Reproducible
inputs, releases, patches, bank context, budgets, results and durable snapshots are in
[scenarios.json](scenarios.json); researched controls/terminal-loop hypothesis in
[gameplay.md](gameplay.md). Distinct natural attract opcode starts:1,527 in saved
45-frame coverage; first-boss artificial scenario:1,747; all trace summaries have
aggregate_dropped=0. Zero detail capacity intentionally discards event details.
Execution coverage is separate from 100% structural classification; no exhaustive
natural execution or hardware/game compatibility claim is made.

Final-base environment and command were visited artificially ($E8=6,$ED=1), but
six destruction parts are missing from this shortcut's initialized state.240 more
firing frames made no progress. Ending/loop completion remains unobserved. Concrete
redirect lead D001-L05 records missing initialization prerequisites; repeat only after
resolving them. Middle-stage permutations were deferred after representative systems
and later milestones, not treated as proof of gameplay exhaustion.

Verification: internal validation and portable validation pass; fresh generated
assembly rebuild is byte-identical. Final ROM SHA256 remains
`9f915b15984b93637f3155a6830db3430b0aae4eed3d09852930bb30f1786981`.
Final map SHA256:`71f91f57da5affd908db67474f695f700a6275f1130d1e261c14d63f50b9f736`.

Toolkit findings: one confirmed documentation/API-validation bug NES-001, no suspected
bugs or feature requests, no blockers; [toolkit-issues.md](toolkit-issues.md). NES input
requires `buttons` array; generic documented `button/pressed` silently clears input.
Earlier intended-input captures were corrected to attract-mode evidence. No toolkit edits.

Disk checked at handoff: about9.0GiB free; scratch12KiB before cleanup. Failed transport
requests were promoted to [transport-attempt.json](disassembly-001/transport-attempt.json)
and disposable scratch requests removed. Durable cited captures/snapshots/checkpoints
remain local. All per-script servers have exited; resume via common.py and explicit
snapshot import, with intended input reasserted.

This single disassembly pass met structural classification/fidelity requirements.
**Overall workflow remains incomplete pending full current-source commentary and
redirect/editorial review.** Routine labels remain mostly address-based; this pass
does not certify full source readability. No publishing, archive or toolkit modification.

## Round 001 — commentary completed

Input disassembly-001-final, unchanged map/state. Supported commentary preparation
and complete current-source readability are finished: all 135 ROM scaffold entries,
240 routine contracts, 437 control targets, 335 reviewed routine/structure units.
[Handoff](handoff-commentary-001.md), [reading copy](../commentary/reading-after.asm),
[review](../commentary/review.json), [final verification](commentary-001-final-verification.json).
Fresh source rebuild byte-identical; input and map SHA256 unchanged. Source SHA256
62718df9373e8ec7e1f7d06ad1f2c4d9487fafe6656ad042ff86a6a5db3d86b3.
No remaining readability ranges; final editorial audit deliberately remains pending.
D001-L05 final-base spawn prerequisites and standalone recorder ROM-write limits are
preserved for redirect. No new toolkit issues; existing NES-001 remains nonblocking.
No redirect execution, publishing, archive or toolkit edits during this phase.

## Round 001 — redirect active
Attempt redirect/r001-final-base. Reviewing progress-derived terrain records and final-base damage/loop contracts before a changed shortcut. Frozen map unchanged, zero unknown; editorial audit pending. Frontier: decode records for cells $010A–$011A; compare initialized object slots and spawn timing, then bounded supported replay.

## Round 001 — redirect completed

Input commentary-001-final / disassembly-001-final, unchanged map and maintained source.
Attempt [redirect/r001-final-base](redirect/r001-final-base/), [handoff](handoff-redirect-001.md).
D001-L05 resolved: five earlier terrain parts + generated sixth core; artificial accepted core defeat,
ordinary0002/difficulty+4 loop and special-source0102/difficulty+4 loop observed. Old failure retained.
Connected graph and independent concrete avenues dispositioned; no ready classification leads,
0 proposed/newly accepted bytes; exact unknown complement remains empty (65,552/65,552,100%).
[Rejection review](redirect/r001-final-base/rejection-review.md) requires coordinator inspection.
Final editorial audit pending, including integrating final-base semantics into stale source/uncertainty
notes and fresh verification. Natural full completion/credits remain unobserved. No empty disassembly
cycle warranted. Research phase complete; frontier is final acceptance review, not unfinished redirect.
ROM/map/state/source/binary preserved; selected evidence remains local, no publishing/archive/deletion.

## Round 001 — final editorial acceptance completed

[Final acceptance handoff](handoff-final-acceptance-001.md) integrates redirect findings
and closes editorial acceptance. Supported commentary audit passes byte-identically,
135 reviewed regions/337 reviewed units, editorial_audit=true; reading copy refreshed.
Source SHA2561d7f8cbac526c6224f3036d0695a40288817744f98b7af807a6013695e7f57a8.
Map/state/input unchanged. Zero pending readability/editorial ranges. Natural terminal
completion remains unobserved; artificial base defeat and both loops remain correctly
scoped. No archive/publishing/deletion/toolkit edits; coordinator retention/options and
completion review are separate next steps.

## Coordinator terminal handoff — round 001 complete

ROM classification: 100.00% (65552 / 65552 bytes classified; 0 bytes unknown).
Sequential phase workers and final acceptance have completed. [Final handoff](final-handoff.md),
[requirement audit](coordinator-completion-audit.json), [independent validation](coordinator-final-verification.json),
[coordinator rejection review](coordinator-rejection-review.md). Whole-ROM classification is
supported by physical evidence independently of orchestration completion. All 135 regions and
337 editorial units reviewed; current source builds byte-identically and reading copy matches.
Final structural checkpoint disassembly-001-final; final source hash
1d7f8cbac526c6224f3036d0695a40288817744f98b7af807a6013695e7f57a8.
No ready lead or unfinished phase remains. Natural complete play/credits are unobserved;
artificial base defeat and both loops are observed. Existing NES-001 is nonblocking.
Archive.py document contract inspected; archive-options.json selects every additional final
and historical Markdown, with no disposable Markdown omissions. [Selection check](archive-document-check.json)
has zero unaccounted paths. No archive, publication, toolkit modification or workspace cleanup
performed. Artifacts remain local and workspace intact; new evidence can reopen research.
Previous goal turn: no prior work beyond initialization; this turn made authoritative progress
through all phase artifacts and the terminal verification.
