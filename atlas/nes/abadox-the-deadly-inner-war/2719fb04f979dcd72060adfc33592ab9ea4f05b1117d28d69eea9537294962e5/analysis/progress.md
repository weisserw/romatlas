# Disassembly progress

Round 1 disassembly active. Baseline completed and byte-identical: [verification](baseline-verification.json), map maps/baseline.json, source generated/baseline. Input unchanged.

Historical initial frontier (bridge terminated; replay evidence retained): local RPC bridge started from analysis/round1/rpc_bridge.py; runtime was natural title screen Abadox (Milton Bradley/Natsume 1989), frame 125, mapper 1 effective banks [0,1,14,15]. Baseline is raw export, no classification yet. Initial reset $FFB0 mapped banks [0,1,14,15]; observed temporary startup banks [12,13,14,15]. Runtime context evidence analysis/round1/start-contexts.json, reset decode reset-decode.json; title screenshot title.png. RPC helpers and request/response are reusable scratch; cited JSON/screenshots are durable evidence. No files removed.

Next: classify validated header/CHR, inspected static reset paths and observed fetches; advance title into game and investigate broad progression/shortcuts. No terminal conclusion yet.

Round1 checkpoint pass-001a verified byte-identical. Classified 138420 bytes so far (52.80%): header16, vectors6, CHR131072, supported code3884, uniform generic PRG data3442. No conflicting code instruction boundaries in saved discovery batch. Runtime durable license-screen snapshot game-entry.snapshot.json; cheat-based representative stage/ending experiments currently running.

Round1 pass-001b verified byte-identical after gameplay classification: 6,006 new code bytes from 6,735 accepted contextual roots (3,405 unique physical instruction starts). Zero aggregate trace drops; detail intentionally discarded (capacity0), so durable aggregate contexts are evidence. Natural controller shortcuts visited stages1/2/4/6/7; this does not claim sequential ordinary progression. License snapshot and scenario recipes preserved in analysis/round1/gameplay.py, per-stage snapshots/screenshots/ram. Ending published recipe was rejected: it selects stage7. Static handler review supplied refined selector+A route, confirmed RAM$3D=7,$4F=1,$05AD=4 and a distinct animated spacecraft/planet scene. Stage6 second-half route confirmed $3D=6,$4F=1. Later-half boss and terminal-scene bounded followup currently active; no terminal handoff yet.

Additional supported structures: DMC sample backing $C000-$C37F and configuration quartets $C380-$C393 consumed by $862A; exact cheat comparison tables at $CEC4/$CECC and sentinel lookup $CED4. See dmc-structure.py. No toolkit issues found so far.

Current resumable frontier: no reliance on bridge or named in-memory saves. Durable license snapshot analysis/round1/game-entry.snapshot.json; successful natural stage snapshots stage1/2/4/6/7.snapshot.json and refined stage6-secondhalf.snapshot.json/ending-refined.snapshot.json. Terminal probe driver terminal-probes.py now completed boss6 case and is advancing ending cutscene. Exact restore/input/patch/budget recipe is in that driver; boss6 patched backing RAM$037C=1 after clean second-half snapshot, marking only this branch artificial. Latest saved map is analysis/state.json and maps/current.json, structurally beyond pass-001b with DMC/cheat data additions. Refined/terminal fetch batches not yet merged.


## Round 1 disassembly completed — incomplete ROM

ROM classification: 61.06% (160069 / 262160 bytes classified; 102091 bytes unknown).

Internal map contains 509 nonoverlapping evidence-backed regions: Code10830, Data18167, Image131072 bytes. Counts derive from the internal store, not raw export. Header16 and all16 CHR8KiB banks classified; each CHR bank8192 classified/0 unknown. PRG physical8KiB identities:

| PRG bank | Code | Data | Classified | Unknown |
|---|---:|---:|---:|---:|
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 0 | 5572 | 5572 | 2620 |
| 2 | 0 | 0 | 0 | 8192 |
| 3 | 432 | 0 | 432 | 7760 |
| 4 | 0 | 186 | 186 | 8006 |
| 5 | 0 | 128 | 128 | 8064 |
| 6 | 0 | 188 | 188 | 8004 |
| 7 | 0 | 448 | 448 | 7744 |
| 8 | 0 | 184 | 184 | 8008 |
| 9 | 0 | 652 | 652 | 7540 |
| 10 | 235 | 186 | 421 | 7771 |
| 11 | 305 | 1438 | 1743 | 6449 |
| 12 | 2032 | 32 | 2064 | 6128 |
| 13 | 80 | 0 | 80 | 8112 |
| 14 | 5006 | 939 | 5945 | 2247 |
| 15 | 2740 | 6 | 2746 | 5446 |

Checkpoint [pass-001 verification](pass-001-verification.json) validates portable maps/pass-001.json and current.json, generates generated/pass-001, and rebuilds byte-identically to input. Input SHA256 remains 2719fb04f979dcd72060adfc33592ab9ea4f05b1117d28d69eea9537294962e5. Exact unknown complement and per-bank metrics: round1/unknown-ranges.json, round1/classification-summary.json. Execution is separate: 3937 unique physical accepted instruction starts across 9633 contextual roots; this is not byte coverage or natural sequential reachability. Zero aggregate drops in saved runtime summaries; detail capacity0 intentionally discards individual events. Untraced waits are explicitly marked in recipes.

Current frontier: all drivers exited; restore durable snapshots with a new configured server and load analysis/state.json. Ending staff credits verified visually in round1/ending-later-result.png after corrected optional-A route. Stage6 artificial later branch saved boss6-later.snapshot.json, RAM$037C=1; recognizable enemy/cannon mechanics exercised but specific boss not established. Stage3/5 deferred with exact route and reason in scenarios.json. See leads R1-STAGE-DATA, R1-FIXED-CLOSURE, R1-BOSS-PROGRESSION. Ordinary easy-gain phase handoff, not terminal exhaustion: redirect should investigate banked consumers/dispatch contracts. No further unchanged gameplay replay justified; unknowns remain substantial.

Toolkit: **no toolkit issues found**; [persistent report](toolkit-issues.md). No toolkit changes, publication, deletion or workspace cleanup. Round1 local artifacts approximately118MiB; >10GiB free at final check. Compact phase handoff: round1/disassembly-handoff.md.

## Round 1 commentary active
Prepared commentary with documented toolkit script; frozen baseline maps/pass-001.json and phase1-state.json, independently maintained commentary/source/game.asm. Preparation rebuilt byte-identically. Reviewed the complete 7,700-line baseline mechanically: bank3 timed stage scripts; bank10 ending scene and credits; bank12 sound interpreter; fixed-bank startup/NMI, screens/cheats, metasprites, scrolling/collision and object engines. No source review credit claimed yet: naming and explanatory edits in progress across all 454 code fragments plus data/unknown scaffold. Current remaining edit scope is the full physical ROM inventory (641 entries); no classification changes. Final editorial audit remains pending.

Round 1 commentary coherent batch saved: all 641 scaffold entries reviewed for current-source readability, including 454 code fragments. Source now has 225 functional routine/structure groups, 419 CPU/RAM/hardware/table symbols and 809 named control-flow points. Group boundary handling includes routines inside a classification fragment (not merely fragment starts). All raw spans have honest physical-boundary/uncertainty notes. Fresh commentary verify passed byte-identically (game.asm SHA256 2a2806b0488e13316eeb024f7c40fa15783ce3c5859f80296fad67be0fde022e). Working through post-edit semantic consistency checks and descriptive branch/constant improvements; no pending classified ranges, but handoff withheld until these checks, final verification and refreshed reading copy. No classification/state/map changes.

## Round 1 commentary complete — ready for redirect
Full current-source readability completed across all 641 physical scaffold entries and all 454 classified code fragments: 225 functional code groups, 809 descriptive control/routine labels, 458 RAM/hardware/table/constant symbols, 247 meaningful review units including named structures. Data formats and every unknown raw span have honest boundary/uncertainty notes. No pending classified range/backlog. Sound, NMI/mapper, stage scripts, presentation/credits, scroll/terrain, metasprites, object/shot/allocation/collision mechanics explained in source. Credits aliases respect RAM reuse; palette writes corrected to palette colors; random generator purpose kept tentative. Final editorial_audit remains pending, intentionally not run.

Final documented commentary verification passed byte-identically after all source edits; reading-after.asm refreshed. game.asm SHA2565afc0ed8bf35642faf1e7d289b25203b7432e07ab959f8ccbfa557c1e74fdfb6; symbols.inc SHA25608ac756c05e712ea4ff75c0732ffc227566f6437782f3d7be813fabaf46f26d8. Map/state/current/frozen SHA256 remains1edea9a746d90fb8ced06dd7033eda0dd19805aa41ac28f530da516d81564cbc; input SHA256 unchanged2719fb04f979dcd72060adfc33592ab9ea4f05b1117d28d69eea9537294962e5. Classification remains160069/262160 (61.06%), unknown102091; reachability unchanged. No toolkit issues found; no runtime, toolkit modification, classification change, publication, cleanup or archive work. 11GiB free at handoff; commentary4.6MiB, round1 evidence118MiB.

Compact handoff: round1/commentary-handoff.md. Semantic limits: ../commentary/uncertainties.md. Next phase redirect should use stage descriptor/object/terrain/sound consumers and fixed entry leads, preserving evidence-policy distinctions. Full source remains independently maintained; preserve it before any later changed-baseline preparation.

Final-acceptance dependency: 254 Code fragment entries own no separate local unit because 225 coherent routine units are recorded at first fragments. All fragments are reviewed and have source/summary. The archive helper expects units on every Code entry; add clipped/reference unit inventory at final acceptance. This does not reopen readable source or authorize toolkit edits.

## Round 1 redirect active
Current-source readability gate accepted. Input checkpoint maps/pass-001.json; classified160069, unknown102091. Research directory analysis/redirect/r1-20261030-stage. Stage resource/object/layout, sound and fixed-entry avenues under static bank-aware investigation; maps, state and source unchanged.

Round1 redirect checkpoint: primary evidence and active frontier saved in redirect/r1-20261030-stage/findings.md. Static stage/layout graph yields independently bounded tile/attribute containers; object command/definition/scripts and sound descriptor/stream parsers saved with zero format errors. No classification accepted yet; connected handler continuations, terrain/tails, envelope/credits and fixed entries remain under investigation.

## Round1 redirect complete — coherent proposal ready for disassembly
Research handoff: redirect/r1-20261030-stage/handoff.md; final reconciliation and exact frontier dispositions in findings.md. All identified concrete connected/independent stage/object/layout/sound/fixed/credits avenues investigated reasonably in one redirect. Combined proposal102089 NEW bytes (Code8773/Data93316), zero proposed Code/Data conflicts; if independently accepted,262158 classified/2 unknown. Actual classification remains160069/262160,102091 unknown; map/state/current/source/ROM hashes unchanged. Exact accepted unknown complement and proposal dispositions in unknown-ledger.json. Isolated RTS candidates physical99897/$8629 and121601/$DAF1 retain unknown; opcode validity or bare pointers insufficient. This is a ready-proposal handoff, not terminal exhaustion or final acceptance. Explicit default-target counterexamples and corrected table/row/envelope bounds preserved; no direct full-map annotation. Leads updated with recipes/history. No toolkit issues found; no runtime, toolkit edits, publication, cleanup/deletion or archive work.

## Round 2 disassembly active — 2026-10-30
Input verified checkpoint: maps/pass-001.json; accepted baseline160069/262160,102091 unknown. Reviewing R1-COMBINED-STATIC-BATCH independently before annotation: primary consumer bounds, physical/effective-bank identities, instruction boundaries, JSR returns and MMC1 mode contracts. Proposal counts remain unaccepted. Existing commentary/source is preserved. Review frontier/artifacts: analysis/round2/.


## Round 2 disassembly completed — incomplete ROM

ROM classification: 100.00% (262158 / 262160 bytes classified; 2 bytes unknown).

The displayed percentage rounds to100.00%; two bytes remain unknown and completion is not claimed. Exact fraction262158/262160 = 99.999237107%. Accepted102089 new bytes (Code8773,Data93316), all Static; existing classifications/provenance preserved. Internal totals: Code19603,Data111483,Image131072;695 nonoverlapping regions. Header16/16; each of16 CHR8KiB banks8192 classified/0 unknown.

| PRG bank | Code | Data | Classified | Unknown |
|---|---:|---:|---:|---:|
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 39 | 8153 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 2254 | 5938 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 39 | 8153 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 39 | 8153 | 8192 | 0 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 39 | 8153 | 8192 | 0 |
| 10 | 657 | 7535 | 8192 | 0 |
| 11 | 703 | 7489 | 8192 | 0 |
| 12 | 2085 | 6106 | 8191 | 1 |
| 13 | 476 | 7716 | 8192 | 0 |
| 14 | 6877 | 1314 | 8191 | 1 |
| 15 | 6395 | 1797 | 8192 | 0 |

Independent review: [findings](round2/review-findings.md), [checks](round2/review.json), [accepted batch](round2/classification-batch.json). Rechecked4278 instruction records, exact physical bank views, record/pointer/terminator boundaries and code/data conflicts (none). Structural/static classification is separate from natural execution. Saved gameplay later-stage/credits routes reused; no unchanged replay. Two unknown isolated RTS offsets99897 ($8629,banks12/13/14/15) and121601 ($DAF1,fixed14/15) have no discriminating static entry or data constraint; retained for commentary/redirect.

Actual output checkpoint **pass-002-reviewed**: [verification](pass-002-reviewed-verification.json), map maps/pass-002-reviewed.json, generated/pass-002-reviewed; validated and rebuilt byte-identically. Current map SHA256 5e38346a7db9fab08897b82d33c5f1ba7c862ded185f9ddc714248eb1dbaf9c6. maps/current.json and analysis/state.json saved. `pass-002` is an unchanged-input historical checkpoint accidentally created after rejected map.annotate missing-id request; no annotation saved in that attempt. This is local orchestration error, not toolkit defect. Preserved all historical artifacts. Existing commentary/source SHA256 remains5afc0ed8bf35642faf1e7d289b25203b7432e07ab959f8ccbfa557c1e74fdfb6; commentary must catch up the accepted batch independently.

All ready structural lead IDs verified together; combined lead is not additive with component leads. R1-BOSS-PROGRESSION remains scenario-limited; no relation to residual byte questions established. New R2-ISOLATED-RTS records exact remaining uncertainty. Current phase: disassembly complete, commentary next; no terminal orchestration decision. Toolkit: **no toolkit issues found**; [persistent report](toolkit-issues.md). No toolkit edits, publication, archive or deletion.


## Round 2 commentary active

Entire prior commentary preserved at analysis/round-2/prior-commentary/. Fresh current baseline prepared and verified. Carried 509 exact unchanged classified physical ranges and prior 225 routine reviews/247 units; independently inspected all new code against current physical/effective-bank identities and applied 192 new mechanical groups plus continuation explanations. New-code batch byte-identical verification passed: game.asm 8d74cabd5baa27a12c52cc45ac545a9eba02b70507df632288df44d5fd57612e, symbols.inc 6755a63331cb7ff472f4738f2843f911a34fd3b1f2a9f1d854d407cfc775f60c. Data annotation/dependency wording/source-location refresh remains active; exact pending ranges in round2/commentary-pending.json. No successful handoff yet; final editorial audit pending.


## Round 2 commentary completed — ROM still incomplete

All697 scaffold entries and current classified source reviewed; no pending readability ranges. Entire previous phase retained at analysis/round-2/prior-commentary/. Carried509 exact physical classified ranges, preserving225 canonical routine reviews, and added192 functional groups plus current data-format notes/generic bounded fallbacks. Stale dependency claims corrected.957 unit records include417 canonical routine units and291 clipped Code-fragment references; all552 Code entries now own local inventory, resolving the previous bookkeeping dependency. Source locations refreshed; final editorial audit remains pending.

Fresh byte-identical verify passed; reading-after.asm refreshed16634 lines. game.asm SHA2566ae767a5b5100e77749728dd2fd2dd7b95f78aa8af113435a4c2595a3ab9f449; symbols.inc24ad79b978d7cf14067a9256848e5ecf890ace573924575f55b40634e0cc6112. Handoff: round2/commentary-handoff.md. Classification unchanged262158/262160; exact unknown ranges[99897,99898),[121601,121602) remain honestly explained, not forced to Code/Data. Next phase redirect; no terminal completion claim, toolkit edits, archive, cleanup/deletion or publication.

## Round 2 redirect active
Input checkpoint pass-002-reviewed;262158 classified,2 unknown. Current-source readability697 entries complete; final editorial audit pending. Research directory redirect/r2-20261030-residual. Frontier: independently reassess isolated RTS structural claims, inspect current-source semantic/generic leads, review unvisited stage3/5 shortcuts and stage6 boss contract. Prior accepted investigations reused by reference; no source/map/state changes.

## Round 2 redirect completed — terminal incomplete proposed for coordinator review
Research/handoff: redirect/r2-20261030-residual/handoff.md; primary-static.json and rejection-review.md retain exact singleton claims, tail-call/stack constraints, all6 JMP/RTS comparisons, strongest Code/Data alternatives and why no independent role discriminator remains. No ready leads/new proposed bytes; actual checkpoint pass-002-reviewed unchanged262158/262160 classified,2 Unknown at99897/$8629 [12,13,14,15] and121601/$DAF1 fixed14/15. Rounded100.00% is not whole-ROM completion. Updated leads/scenarios preserve history. Current readability697 entries complete; verification remains current byte-identical; final editorial audit pending. Source/map/state/input hash preservation check passed. Stages3/5 controller shortcuts now exercised; stage6 source-backed artificial threshold/scheduler shortcut exposes active kind$40 formations and repeated$BEBE script loop, terminal defeat unconfirmed. No useful targeted residual method survives review; coordinator must inspect primary rejection rationale before accepting terminal incomplete. No toolkit issues found, no toolkit edits/ROM/source/map/state changes, archive/publication/cleanup/deletion. All experiment processes ended;40MiB attempt/11GiB free. Exact interrupted continuation distinguished from completed subsequent scheduler method in findings.md.


## Final editorial acceptance completed — ROM classification incomplete

Final targeted editorial review accepted all697 entries/957 units, preserving prior valid review credit and resolving no new structural bytes. Corrected obsolete ten-byte resource-descriptor dependency wording only. commentary.py audit passed, freshly rebuilding byte-identically and refreshing reading-after.asm; configured map/store validation and exact input identity passed. editorial_audit reviewed; no pending editorial acceptance work. Classification remains262158/262160 (99.999237107%), two Unknown bytes physical99897/121601. Compact handoff: final-editorial-handoff.md; primary audit notes: ../commentary/editorial-audit.md; exact reports/hashes: final-editorial/checks.json. Coordinator final requirements/doc-manifest audit remains outside this worker; orchestration goal not marked complete. No archive/restore, physical cleanup/deletion, toolkit edits or publication.


## Coordinator terminal handoff — orchestration complete, ROM incomplete

ROM classification: 100.00% (262158 / 262160 bytes classified; 2 bytes unknown). Exact99.999237107%; no whole-ROM classification completion. Final checkpoint pass-002-reviewed; Unknown physical99897/$8629 [12,13,14,15] and121601/$DAF1 fixed14/15.

Coordinator inspected final rejection reasoning and decisive primary bytes/neighbor contracts/adjacency comparisons plus completed progression results: coordinator-rejection-review.md. No unsupported prerequisite or corrected useful test remains; terminal incomplete accepted. No live experiment or external execution blocker is being treated as success. Stable residual leads remain resumable if independent evidence changes.

Separate final editorial acceptance passed after one stale descriptor comment correction: final-editorial-handoff.md; commentary/audit.json and verification.json freshly byte-identical, reading copy refreshed. All697 entries/957 units reviewed; no pending editorial acceptance. Configured map/store validation and unchanged input identity passed.

Final requirement-by-requirement and machine audits: completion-audit.md/completion-audit.json; terminal-handoff.md is the final concise result. Machine audit recomputes exact complement, checks current final source/review/verification/map hashes, validates all six phase handoffs plus final acceptance, and confirms zero ready leads. archive-options.json selects16 additional final/history Markdown documents and omits only3 byte-identical prior instruction copies; zero unrecognized applicable Markdown. scripts/archive.py document-selection contract inspected, not executed. No archive/restore, cleanup/deletion, publishing or toolkit modification authorized/performed. All ROM/source/assets/state/traces remain local.
