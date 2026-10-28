# Disassembly progress

Workspace initialized. Baseline and classification have not run.

## Round 001 — disassembly (active)

Input: initialized workspace, no classification. Baseline now verified at maps/baseline.json and analysis/baseline-verification.json. Bounded runtime/static analysis underway; see analysis/disassembly/round-001/rpc.jsonl.

## Round 001 — disassembly complete; ROM incomplete

ROM classification: 90.50% (237228 / 262144 bytes classified; 24916 bytes unknown).

Input checkpoint: initialized workspace (0 classified); baseline established once at [baseline verification](baseline-verification.json). Output: [state.json](state.json), [portable map](../maps/current.json), [pass-001 map](../maps/pass-001.json), [source](../generated/pass-001/game.asm), [byte-identical verification](pass-001-verification.json). ROM SHA-256 remains 284cf8e186dcaa8174909c9fd4200f4e4dae325b2ad20e541814fc0355d5eee7.

| Bank | Code | Data | Classified | Unknown |
|---|---:|---:|---:|---:|
| 00 | 3364 | 8987 | 12351 | 4033 |
| 01 | 7184 | 16 | 7200 | 9184 |
| 02 | 4431 | 254 | 4685 | 11699 |
| 03 | 0 | 16384 | 16384 | 0 |
| 04 | 0 | 16384 | 16384 | 0 |
| 05 | 0 | 16384 | 16384 | 0 |
| 06 | 0 | 16384 | 16384 | 0 |
| 07 | 0 | 16384 | 16384 | 0 |
| 08 | 0 | 16384 | 16384 | 0 |
| 09 | 0 | 16384 | 16384 | 0 |
| 10 | 0 | 16384 | 16384 | 0 |
| 11 | 0 | 16384 | 16384 | 0 |
| 12 | 0 | 16384 | 16384 | 0 |
| 13 | 0 | 16384 | 16384 | 0 |
| 14 | 0 | 16384 | 16384 | 0 |
| 15 | 0 | 16384 | 16384 | 0 |

Annotation coverage: 237228 classified bytes have evidence and 237228 have comments. Editorial readability is pending commentary. Execution: 6904 unique accepted ROM opcode starts (14626 decoded bytes at those starts) in retained union; includes natural and artificial routes and is separate from classification. Static bytes are not claimed observed.

Natural Stage1 plus artificial Stage2/8/13, ending artwork, Quest puzzle and Endless field visited; exact patches/inputs/budgets and durable snapshots in [scenarios](scenarios.json). Broad progression checklist includes deferred Quest30/Quest ending/full credits and unavailable linked Versus. Runtime input schema failures and failed board/return-slot edits preserved with outcomes.

Evidence: [classification statistics](disassembly/round-001/classification.json), [exact unknown complement](disassembly/round-001/unknown-ranges.json), [code evidence](disassembly/round-001/all-code-evidence.json), [script inspection](disassembly/round-001/script-inspection.json), [asset containers](disassembly/round-001/asset-container-evidence.json), [sound/shared assets](disassembly/round-001/more-data-evidence.json), [RPC transcript](disassembly/round-001/rpc.jsonl). Data banks use generic aggregate structure, not invented exact original layouts.

Lead outcomes: R1-L001/002/003/006 verified; R1-L004 superseded (return slot overwritten); R1-L005 rejected only for its bounded single-cell experiment. R1-U001/002/003 concrete redirect avenues; R1-U004 commentary. No terminal exhaustion claim: easy startup, distinct gameplay and straightforward static continuation gains taken; deeper indirect dispatch/table contracts need redirect investigation. No supported proposal is withheld merely for lacking runtime use.

Toolkit: two confirmed documentation/API mismatches (TK-001 discovery budget, TK-002 input schema), zero suspected bugs, zero missing-feature findings; workarounds used, no blockers. See [toolkit report](toolkit-issues.md). Long-lived tool sessions received SIGTERM from unknown host cause; recovered via persisted maps/snapshots and short independent sessions, not attributed to toolkit. Toolkit and ROM unchanged.

## Round 001 — commentary complete; ROM incomplete

Current-source readability is complete against unchanged pass-001: all 14,979 classified code bytes (7,080 emitted instructions), 222,249 data bytes and honest notes for the 24,916 unknown bytes. All 575 review regions and 323 routine/block units are reviewed; no classified range deferred. Source now has substantive routine/algorithm/contract explanations, 693 control labels, 151 RAM aliases and separate task/sound/falling-pair/particle field names. Systems include scheduler, threaded scenes, VDP/CRAM/sprites, palette interpolation, banked assets/decompression, serial fragments, PSG voice sequencing, field/AI mechanics and Quest transitions. Aggregate data remains generic where exact layouts are unresolved; material uncertainty is explicit in source.

Fresh [commentary verification](../commentary/verification.json): ok=true, byte_identical=true, source SHA256 d477ed1d8290f871c17ea1915460e7ac76e00735f3db538dbad039947fc3b54e. [Reading copy](../commentary/reading-after.asm) refreshed; [buildable source](../commentary/source/game.asm). ROM, maps/current.json and analysis/state.json unchanged. Prior evidence, generated pass-001 and frozen commentary baseline preserved. Final editorial/archive audit remains pending separately; classification stays 90.50%.

[Compact commentary handoff](commentary/handoff.md), [scope review](commentary/scope-review.json), [review inventory](../commentary/review.json), [uncertainties](../commentary/uncertainties.md). R1-U001/U002 dispatch/table domains and R1-U003 Quest progression remain concrete redirect questions; commentary did not start redirect or make a terminal exhaustion claim. Existing toolkit findings remain two confirmed API/documentation mismatches TK-001/TK-002, no suspected bugs/missing features, no blockers or new findings; see [toolkit report](toolkit-issues.md). Disk: 9.5 GiB free, bounded local derived records, no new captures or trace logs.

## Round001 — redirect complete; combined proposal pending disassembly

Input/actual checkpoint remains pass-001:237,228 classified,24,916 unknown; zero annotations during redirect. [Handoff](redirect/round-001-attempt-001/handoff.md), [proposal](redirect/round-001-attempt-001/proposal.json), [exact complement/dispositions](redirect/round-001-attempt-001/unknown-ledger.json). Ready R1-R001/R002 propose17,961 Code+6,952 Data=24,913 bytes across banks0–2; if fully accepted only3 bytes remain. No acceptance/terminal exhaustion claim. R1-R003 retains isolated43B2..43B3 andBC36 pending independent structural evidence.

Connected/independent research:1,624 typed scene commands,38 field configuration occurrences,54 configured task occurrences; all concrete six-tile, sound, serial, orientation/field/AI, Quest, password/widget/particle and tutorial/name-entry task descendants followed. Exact candidate instructions, roots/edges, RAM/bank preconditions and generic-data boundary/role checks retained in attempt. Source scene writers corrected an initially overbroad withholding of reset stub03B6; typed command1DA5 now anchors it. [Compact rejection review](redirect/round-001-attempt-001/rejection-review.md), [decisive primary evidence](redirect/round-001-attempt-001/rejection-primary.json), [methods/history](redirect/round-001-attempt-001/methods.md). No speculative single-opcode labels and no withheld supported continuations.

Gameplay: Quest30 initializer/controls/success screen and parent ending setup entered artificially; stable terminal Quest artwork remains unconfirmed. Actual main ending continued960 frames through girl dialogue to complete kingdom-saved narrative. Initial credits interpretation of1F25 was rejected by visible tutorial text; selected rows/mascot system exercised with supported task-pool/text-delay edits, old task interference diagnosed. No staff roll confirmed. Exact scenarios and corrections appended to [scenarios](scenarios.json). ROM/map/state/source hashes unchanged; current readability and fresh commentary byte identity remain valid for pass001, final editorial/archive audit pending separately.

Toolkit: no new issues or edits; existing two confirmed mismatches TK001/TK002, zero suspected/missing features, no blockers. [Report](toolkit-issues.md). Research records local; lossless RPC archive and selected evidence retained, disk bounded.

## Round 002 — disassembly complete; ROM incomplete

ROM classification: 100.00% (262141 / 262144 bytes classified; 3 bytes unknown).

Input pass-001:237,228 classified/24,916 unknown. R1-R001/R1-R002 independently verified and accepted as a connected batch:17,961 Code+6,952 Data=24,913 added bytes, all Static. 199 exact code intervals (7,988 instructions) redecoded identically using the configured binary in asserted CPU/bank context [0,1,2]; all instructions have recorded root/edge closure. Reviewed typed scene writers/commands, allocator task contracts, word dispatch consumers, bank restoration, independent routine contracts and exact boundaries. All49 word-array manifests match ROM; all98 data intervals inspected in full with references, neighbors and pattern/consumer rationale. No proposed code/data bytes rejected; $92D0 five-byte ordered constants kept generic Data without the unsupported countdown-purpose inference. Earlier $03B6 reset-stub rejection was correctly narrowed by scene writer439D→cursor1D56→typed command1DA5.

| Bank | Code | Data | Classified | Unknown |
|---|---:|---:|---:|---:|
| 00 | 5280 | 11104 | 16384 | 0 |
| 01 | 13814 | 2568 | 16382 | 2 |
| 02 | 13846 | 2537 | 16383 | 1 |
| 03 | 0 | 16384 | 16384 | 0 |
| 04 | 0 | 16384 | 16384 | 0 |
| 05 | 0 | 16384 | 16384 | 0 |
| 06 | 0 | 16384 | 16384 | 0 |
| 07 | 0 | 16384 | 16384 | 0 |
| 08 | 0 | 16384 | 16384 | 0 |
| 09 | 0 | 16384 | 16384 | 0 |
| 10 | 0 | 16384 | 16384 | 0 |
| 11 | 0 | 16384 | 16384 | 0 |
| 12 | 0 | 16384 | 16384 | 0 |
| 13 | 0 | 16384 | 16384 | 0 |
| 14 | 0 | 16384 | 16384 | 0 |
| 15 | 0 | 16384 | 16384 | 0 |

Exact residual: physical/CPU $43B2..$43B3 (bank1,00DC) and $BC36 (bank2,C9),3 bytes total. Neighboring serial/NMI and coordinate-helper code accepted independently. Read [decisive primary evidence](redirect/round-001-attempt-001/rejection-primary.json) and [localized rejection review](redirect/round-001-attempt-001/rejection-review.md): no independent word consumer or enclosing format for00DC; no callable contract for isolated RET. Competing code/data interpretations remain materially ambiguous. Corrected reset-stub premise does not generalize to these isolated bytes. Reopen only with independent consumer/entry/enclosing structure or a justified corrected inference; no unchanged replay or arbitrary state sweep warranted. This pass does not make a terminal workflow stopping decision.

Annotation coverage:262141 bytes with evidence and comments. Total code32,940 bytes; data229,201 bytes. Runtime evidence reused from prior progression and redirect; zero new execution/replays. Prior disassembly observed union6,904 opcode starts/14,626 decoded bytes remains separate (redirect observations not re-counted). Broad gameplay and ending evidence are in [scenarios](scenarios.json), including artificial Quest30 and continued main-ending narrative; no staff roll or stable Quest terminal artwork newly claimed.

Saved [internal state](state.json), [portable map](../maps/current.json), fresh [pass-002 map](../maps/pass-002.json), [generated source](../generated/pass-002/game.asm) and [verified checkpoint](pass-002-verification.json):ok=true,byte_identical=true. [Internal validation](disassembly/round-002/validation.json) ok. [Independent review](disassembly/round-002/independent-review.json), [data inspection](disassembly/round-002/data-inspection.json), [statistics](disassembly/round-002/classification.json), [unknown complement](disassembly/round-002/unknown-ranges.json). ROM identity unchanged; prior annotations, maps, source, commentary and investigation history preserved. Newly accepted source requires commentary; prior editorial verification covers pass-001 only.

Toolkit:no new toolkit issues found. Existing [report](toolkit-issues.md) retains2 confirmed documentation/API mismatches TK001/TK002,0 suspected bugs,0 missing features,0 blockers. No toolkit edits. Storage:bounded199 decode requests,49 table checks,297 annotations; concise digest review log and bounded annotation transcript, no captures/runtime traces;9.4GiB free.

## Round 002 — commentary complete; ROM incomplete

Current-source readability is complete for unchanged pass-002: all 32,940 code bytes (15,068 emitted instruction starts), 229,201 data bytes and honest notes for the three unknown bytes. All 687 scaffold regions and 739 meaningful units are reviewed, with no classified range deferred. All 388 unchanged prior classified sections retain review credit; new 17,961 code/6,952 data bytes reviewed and 330 mechanical contracts added. Complete prior commentary and analysis records preserved under analysis/round-002/commentary-pass001 and commentary-records-pass001 before fresh baseline preparation.

Source explanations now include complete accepted suspension/serial/sound arms, special falling objects, match/garbage scoring, AI neighbor weights, tutorial/scroll/name entry and Quest/password/planar tile feedback. Source-only interpretation corrections distinguish the $30DF score record, $9574 scroll schedule and $BED6 palette descriptors. All unknown semantics remain mechanically described; original asset subdivisions and actual linked-device behavior remain uncertain. Exact structural residual unchanged: $43B2..$43B3 and $BC36.

Fresh [commentary verification](../commentary/verification.json) ok=true,byte_identical=true, source SHA256 343b600fe825862809b98082d616c8b595f4ff8469a77dd33c9f8f038bc21496; [reading copy](../commentary/reading-after.asm) refreshed. [Compact handoff](commentary/round-002/handoff.md), [scope review](commentary/round-002/scope-review.json), [inventory](../commentary/review.json), [uncertainties](../commentary/uncertainties.md). ROM/map/state/toolkit unchanged. Final editorial/archive acceptance remains pending separately; no final audit or redirect run here.

Toolkit: no new toolkit issues found. Existing [report](toolkit-issues.md) retains TK-001/TK-002, two confirmed API/documentation mismatches, zero suspected bugs/missing features/blockers. No runtime/captures/traces; bounded local review artifacts and 9.4 GiB free.

## Round 002 — redirect terminal incomplete

Current pass-002 remains262,141 classified/3 unknown; zero proposals or map/source changes. [Handoff](redirect/round-002-attempt-001/handoff.md), [current complement](redirect/round-002-attempt-001/unknown-ledger.json), [compact rejection review](redirect/round-002-attempt-001/rejection-review.md), [primary slices/hashes](redirect/round-002-attempt-001/primary.json). Reused prior sound review, assessed new current commentary and localized NOP/word/RET alternatives. Serial ring-base resemblance and isolated RET remain code/data ambiguous; natural entry/original layout not required. New score/scroll/palette corrections add no discriminating residual edge/format. No useful supported continuation or changed test remains; no automatic new cycle. Exact residual43B2..43B3 bank1 andBC36 bank2.

Current-source readability complete; existing fresh commentary verification byte-identical; final editorial/archive acceptance pending separately. Broad progression/terminal narrative and Quest30 attempts reviewed without unchanged replay. No new toolkit issues found; existing TK001/TK002 two confirmed mismatches, no blockers. ROM/map/state/source/toolkit unchanged; bounded research records only,9.4GiB free.

## Coordinator — round002 terminal review accepted; workflow incomplete

ROM classification: 100.00% (262141 / 262144 bytes classified; 3 bytes unknown). Rounded percentage is not completion.

Inspected current redirect rejection-review.md and primary.json plus prior decisive rejection-primary.json. The two-byte 00 DC interval at physical $43B2..$43B3 is neither independently consumed as a ring-base word nor anchored as an alternate instruction entry; the isolated C9 at $BC36 has no independent callback/format contract. Neighboring routines and current commentary corrections do not discriminate these roles. Narrow NOP/prefix/RET alternatives were assessed without requiring natural reachability, original layout or complete semantics. Review supports localized ambiguity and no useful revised test; no unchanged automatic cycle is warranted. See [terminal handoff](redirect/round-002-attempt-001/handoff.md) and [rejection review](redirect/round-002-attempt-001/rejection-review.md).

Coordinator independently recomputed ROM/state/map/commented-source hashes against current primary.json: all match. Inspected pass-002 and commentary verification reports: both ok=true, byte_identical=true and identify those exact current hashes. Current-source readability is complete (687 regions/739 units); final editorial/archive acceptance remains pending, and whole-ROM completion is not claimed. No goal-complete assertion. Input and toolkit remain unchanged. Existing toolkit report has two confirmed API/documentation issues with workarounds, no blockers. Stop incomplete at pass-002; resume only for concrete changed evidence/inference or explicit reassessment.

## Final editorial/archive acceptance — accepted with three Unknown bytes

Explicitly authorized final audit passed: ok=true, byte_identical=true, editorial_audit=true, with a fresh rebuild and refreshed reading-after.asm. All 687 scaffold regions and 917 units reviewed; original 739 units retained plus 178 inspected continuation fragments. Repaired 1,426 inventory paths relative to commentary/source and clarified both Unknown boundary comments. Source SHA256 7b18886bbe72407735d34067fb1e6a949666458f5077971c27ba2124d767ef97. No classified source deferred.

Whole-ROM classification remains incomplete: 262,141/262,144 classified; physical $43B2..$43B3 bank1 and $BC36 bank2 remain Unknown. Independently checked exact unannotated complement in unchanged map/state and exact local slices in source; no new classification or research loop. Frozen artifacts/history/input/toolkit preserved; source/assets/internal evidence remain local. Final report, attempts, inventory repairs, archive manifest and handoff: analysis/commentary/final-audit/. Existing toolkit TK-001/TK-002 remain nonblocking; no new issue or toolkit edit.
