# Disassembly progress

Workspace initialized; round 1 baseline and classification are now established. See chronological records below and final counts at end.

## Round 1 disassembly — in progress

Baseline established once and byte-identical: [report](baseline-verification.json), [map](../maps/baseline.json), [source](../generated/baseline/game.asm). ROM hash remains workspace identity. Mapper 2, physical PRG banks 0–15 (8 KiB), no CHR ROM. At this historical point the classification map was not yet persisted; it has since been saved and verified. Raw baseline is not classification.

Validated vectors: reset $C000, NMI $C161, IRQ $C15F, fixed effective banks 14/15; initial low slots 0/1. Static vector discovery: 58 instruction candidates. Startup and title/menu inputs expanded to 1,489 context-specific discovery candidates (all fixed PRG storage $1C010–$1D577 so far); instruction decodes retained for review. [Discovery](evidence/discovery-observed-static.json), [decodes](evidence/candidate-decodes.json).

Local framebuffer identifies Color a Dinosaur (1993). Copyright, animated title and 4×4 dinosaur selection grid verified; [grid capture](evidence/grid3.png). Starting point-to-point mode is in progress. Start at frames 74 and 392 was during copyright/title setup and did not establish a completed menu transition; later Start at frames 512–522 followed by release and 50 frames reached the grid at frame572. All scenarios so far natural, no debugger patches. Current low banks 0/1 after selecting first dinosaur. Long startup steps timed out with partial advance; queried state and continued same server rather than restarting. Trace detail capacity zero intentionally drops details; bounded aggregate roots/edges remain intact (aggregate_dropped=0 at last inspection).

Remaining: painting, palette/freehand/alternate selection; finalize supported code and clear patterned data, then validate/export/verified checkpoint and exact counts. Boss/level/ending expectations are inapplicable to this coloring activity unless later evidence establishes such states. [Gameplay research](gameplay.md). No toolkit issues found so far; [persistent issue report](toolkit-issues.md).

Initial checkpoint [pass-001-initial verification](pass-001-initial-verification.json) succeeds byte-identically. Internal map has 78 regions / 61,669 classified bytes. ROM classification: 47.04% (61669 / 131088 bytes classified; 69419 bytes unknown).

At frame681 a full-size stegosaurus picture is visible; by frame712 point cursor and colored/patterned palette are visible after A, B and Select actions. [Picture](evidence/picture1.png), [palette](evidence/palette-paint.png); durable machine snapshot painting-snapshot.json is local only. Natural scenario, no RAM/register edits. Additional alternate/freehand route is being exercised before final classification batch.


## Round 1 final — incomplete

ROM classification: 98.99% (129764 / 131088 bytes classified; 1324 bytes unknown).

Internal map: 149 regions; 4156 code bytes, 125608 data bytes. Every classified region has structural evidence/provenance; all code regions have explicit physical-to-CPU views. One Reset label only: full-current-source commentary is the next phase. Classification is separate from semantic readability and actual fetch coverage. The26 unknown intervals are all in physicalbank14; [exact complement/counts](evidence/classification-final.json). No whole-ROM completion or full compatibility claim.

| Physical8KiB bank | Code | Data | Unknown |
|---|---:|---:|---:|
| header | 0 | 16 | 0 |
| 0 | 0 | 8192 | 0 |
| 1 | 0 | 8192 | 0 |
| 2 | 0 | 8192 | 0 |
| 3 | 0 | 8192 | 0 |
| 4 | 0 | 8192 | 0 |
| 5 | 0 | 8192 | 0 |
| 6 | 0 | 8192 | 0 |
| 7 | 0 | 8192 | 0 |
| 8 | 0 | 8192 | 0 |
| 9 | 0 | 8192 | 0 |
| 10 | 0 | 8192 | 0 |
| 11 | 0 | 8192 | 0 |
| 12 | 0 | 8192 | 0 |
| 13 | 0 | 8192 | 0 |
| 14 | 4156 | 2712 | 1324 |
| 15 | 0 | 8192 | 0 |

[State](state.json), [portable current map](../maps/current.json), [frozen checkpoint map](../maps/pass-001.json), [generated source](../generated/pass-001/game.asm), [verification](pass-001-verification.json). map.validate succeeds; final rebuild is byte-identical. ROM SHA256 remains9ad49d6d2d4f100ee1f2118822a78bcea65b103f32f3485a89dcd05f84242540.

Structural evidence: reviewed vector/observed-connected6502 code, fixed bank14; banked data file[16,114704) shows planar/repeated graphic patterns, bank-start bus-conflict values0..7, long zeros and DOS utility string residue. Fixed code$CF35/$CF1A reads banked bytes through($59),Y into$2007. This supports generic non-code payload, not a uniform asset format or original provenance. [Bank review](evidence/banked-data-review.json). Tail[120184,131082) follows final sound RTS and shows numeric masks/runs, tile-index/bitplane patterns, repeated note-like records and zeros; sound references$EF47/$EF5A/$EF6D corroborate data use. [Tail review](evidence/fixed-tail-review.json). Generic data labels intentionally avoid unsupported precise schemas. Initial zero runs retain separate maximal boundaries.

Execution:2,015 distinct fetched physical instruction starts,2,643 contextual roots,2,881 contextual edges in final aggregate; these include the separately marked artificial$4D experiment and are not natural-only coverage.3,863,344 accepted fetch events; aggregate_dropped=0. Detailcapacity0 intentionally retains no event detail; dropped detail count6,146,629 is not aggregate loss. [Trace summary](evidence/trace-summary-final.json), [roots](evidence/contexts-final.json), [coverage summary](evidence/execution-coverage-final.json). Long initial requests timed out after partial progress; server state was inspected and same session continued, without reset.

Progression: copyright/title, sixteen-picture grid and stegosaurus/point palette visited naturally. B/Select changed palette selection; successful fill, Start-return, alternate picture and freehand movement remain unverified despite bounded attempts. RAM$4D=0 at frame851 was tested under saved clean snapshot and failed to move coordinates or change display through879; restored natural851 state and released buttons. [Exact scenarios/statuses](scenarios.json), [gameplay/manual](gameplay.md). Boss/level/ending assumptions inapplicable to available coloring-activity evidence.

Handoff: [ready leads](leads.json) cover direct fixed-bank gaps, concrete input/dispatcher gate investigation, and optional data-role refinement. No prior leads existed; this pass neither rejected nor exhausted these avenues. Coordinator requested handoff after coherent supported gains, so no terminal exhaustion claim/rejection audit is made. [Investigation ledger](investigation.md).

Toolkit: no toolkit issues found. No blockers or toolkit edits. [Issue report](toolkit-issues.md). Disk at final inspection 3.6GiB free; scratch48KiB and selected evidence29MiB; nothing deleted. All internal state/assets/captures/source remain local.

## Round 1 commentary — current-source readability complete

Prepared the supported independent commentary workspace against pass-001 (unchanged current map/state). Completed all current classified source: 4,156 code bytes / 2,021 instructions and 125,608 data bytes, plus honest boundary notes for 1,324 unknown bytes. All 175 scaffold entries and 155 functional units reviewed. Added meaningful dispatch, input, painting/flood-fill, PPU transfer, palette, menu and sound explanations with consistent RAM/hardware and consumer-table names. Static interpretation and unresolved semantics are explicit; no structural classification was changed.

Fresh [verification](../commentary/verification.json) is byte-identical; [reading copy](../commentary/reading-after.asm) refreshed. Input SHA256 unchanged. [Review inventory](../commentary/review.json), [phase progress](../commentary/progress.md), [questions C01–C03](../commentary/uncertainties.md), [handoff](round01-commentary-handoff.md). No deferred readability ranges; classification remains 98.99%, separate from readability. Final editorial audit remains pending and was not called. No toolkit issues found; no toolkit edits or publication.

## Round 1 redirect — round01-static-contracts

Input checkpoint pass-001; commentary current-source gate satisfied. Exact accepted complement unchanged26 intervals/1324 bytes in bank14. New static-contract method combines decodes with maintained source, finite table consumers and independent endpoints; no unchanged rejected experiment replayed. Proposed batch482 code+842 data bytes, including mixed-gap split and generic non-code fallback where format unresolved. See [attempt review](redirect/round01-static-contracts/review.md), [exact dispositions](redirect/round01-static-contracts/residual-ledger.json) and updated leads. No classifications accepted here. Independent runtime inquiry follows busy gate and200-frame Start delay; frontier recorded with runtime artifacts. No map/state/source/toolkit/input changes.

Round1 redirect completed coherent code/data and independent behavior batch. Correct NES input verified natural Start scheduled200-frame exit; artificial timer shortcuts then initialized menu, picture1/freehand, actual movement and visible fill. Fill returned idle at1378; final reusable checkpoint and exact logs are in attempt directory. No active runtime. [Handoff](round01-redirect-handoff.md). Preliminary wrong-schema probes preserved as invalid no-op inputs; one wrapper timeout-loss documented and corrected. No toolkit issues found. Structural next step is independent disassembly review of482code+842data proposed bytes, not terminal exhaustion.

## Round 2 disassembly — classification complete; commentary pending

ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).

Accepted482 code+842 data bytes from independently reviewed residual contracts. Internal map now contains4638 code/126450 data bytes, 183 regions, no unknown ranges. [Independent review](round02-disassembly/review.md), [fresh configured-toolkit decodes](round02-disassembly/fresh-decodes.json), [exact metrics](round02-disassembly/metrics.json). All previous regions preserved; new provenance Static with explicit fixed-bank code views [null,null,14,15]. Reachability remains a separate claim.

Correction to redirect CB16 description: BNE loops while frame_counter differs from saved A; usually returns immediately, and an intervening NMI may wait for byte recurrence. It does not wait until change. Coherent static polling code accepted; intended purpose/actual caller unresolved. Other isolated helpers independently supported by OAM, PPU-loop and flood-work contracts. C44D gap correctly splits48 cursor data+24 generic data+28 clear code. CB7B table run subdivided by consumers; CCAB32bytes retains generic planar data. No rejected intervals or forced unknown elimination.

| Physical8KiB bank | Code | Data | Unknown |
|---|---:|---:|---:|
| header | 0 | 16 | 0 |
| 0 | 0 | 8192 | 0 |
| 1 | 0 | 8192 | 0 |
| 2 | 0 | 8192 | 0 |
| 3 | 0 | 8192 | 0 |
| 4 | 0 | 8192 | 0 |
| 5 | 0 | 8192 | 0 |
| 6 | 0 | 8192 | 0 |
| 7 | 0 | 8192 | 0 |
| 8 | 0 | 8192 | 0 |
| 9 | 0 | 8192 | 0 |
| 10 | 0 | 8192 | 0 |
| 11 | 0 | 8192 | 0 |
| 12 | 0 | 8192 | 0 |
| 13 | 0 | 8192 | 0 |
| 14 | 4638 | 3554 | 0 |
| 15 | 0 | 8192 | 0 |

map.validate succeeded; fresh unique [pass-002 verification](pass-002-verification.json) confirms byte-identical rebuild. [Checkpoint map](../maps/pass-002.json), [current map](../maps/current.json), [state](state.json), [source](../generated/pass-002/game.asm). Input hash remains9ad49d6d2d4f100ee1f2118822a78bcea65b103f32f3485a89dcd05f84242540.

R1-CODE-GAPS and R1-DATA-SUBDIVISION verified; R1-INPUT-MODE-CONTRACT reused verified runtime5/6 evidence unchanged. Existing scenarios already cover title/grid, point palette, alternate picture/freehand movement and completed visible fill; artificial timer shortcuts remain qualified. No distinct conventional level/boss/ending mechanism established. No repetitive runtime experiments. Historical failed shortcuts and redirect records preserved.

Classification contract complete; full current-source readability needs separate commentary carry-forward of frozen pass-001 commentary plus new1324bytes, and editorial gate remains pending. No commentary, redirect or final audit run in this pass. No toolkit issues found; [persistent report](toolkit-issues.md). No toolkit changes, deletion/cleanup, archive or publication.


## Round 2 redirect review

Pass-002 complement independently refreshed:131088/131088classified,4638code/126450data,183regions,zero unknown. No structural proposals/accepted bytes or map/source changes. Source-led residual review found a concrete editorial direction mismatch in four map-scan continuation labels/comments; R2-EDGE-ENTRY-EDITORIAL is ready for final editorial correction, not disassembly. Source consumer proves current-cell entry edges left/right/top/bottom for direction1/0/3/2. Equality-poll correction retained. Bounded target-reference search, descriptor/attract consumers and source-contract overflow counterexample recorded; no natural reachability or universal safety claims. Detailed claim/rejection review and actual stop rationale: [review](redirect/round02-residual-review/review.md); [identity/complement](redirect/round02-residual-review/identity-and-complement.json). Broad representative gameplay reused with explicit timer-shortcut qualification; no distinct ending system established. No unchanged runtime retry, toolkit change, cleanup or final audit. Next: editorial correction and final acceptance; residual authored semantics remain incomplete.


## Final editorial acceptance

Resolved R2-EDGE-ENTRY-EDITORIAL: the repeated-map continuations now name current-cell left/right/top/bottom entry edges (directions1/0/3/2), consistently in source and review summaries. Reused complete valid review credit and inspected concrete dependencies/remaining pending defects. First supported audit exposed inherited source/game.asm references resolved relative to commentary/source; normalized inventory references to game.asm:line, preserving locations. Rerun audit passed:183 reviewed records,189 reviewed units, editorial_audit=true, fresh byte_identical=true; reading-after.asm refreshed. Source SHA256 e647da4e484e72914fcb115ce526f571b3a84311453d3e0759d30278f9c098ac. See commentary/editorial-audit.md, commentary/audit.json and analysis/final-editorial-handoff.md. CB16 equality, static-versus-natural reachability and timer shortcuts remain qualified; unresolved authored semantics/safety remain documented. No source instruction/data, structural map/state, input or toolkit changes; no archive/cleanup/publication. No editorial work remains pending.

## Coordinator terminal handoff — workflow complete

Current phase: terminal handoff; round 2 complete. Input/output checkpoint pass-002 unchanged through final commentary, redirect and editorial acceptance. All four lead IDs verified; none ready, blocked or rejected pending. Full classification and final editorial acceptance are independently satisfied, not inferred from orchestration status.

ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).

[Terminal handoff](terminal-handoff.md), [requirement audit](coordinator/final-audit.md), [current verification](coordinator/final-verification.json), [final source/editorial handoff](final-editorial-handoff.md). Supported final audit183regions189units, editorial_audit=true, byte-identical rebuild; independent store/map validation passes. Source hash e647da4e484e72914fcb115ce526f571b3a84311453d3e0759d30278f9c098ac. Input unchanged. Remaining semantic/coverage limits remain documented; no full compatibility claim. No toolkit issues found.

archive-options.json and coordinator/document-selection.json account for all additional Markdown using the inspected archive.py selection contract. No archive execution, deletion/cleanup, toolkit change or publication. No active worker/runtime or pending productive investigation remains. Historical phase-in-progress/deferred statements above are superseded by their chronological outcomes and this terminal record.
