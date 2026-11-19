# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## Round 01 initial and startup batches

- Header and eight exact FF intervals: structural generic data, not a fill-purpose claim. Boundaries independently scanned; each ends before the repeated reset stub/trailer; no code conflict. ASCII/metadata trailer and vector records separately classified. Evidence: scripts `scratch/classify_initial.py`, maps/pass-001-initial.json and verification report.
- Reset/NMI static snippets: decoder review `evidence/reset-static-review.json`; mapper/call uncertainty conservatively bounds discovery. Runtime reset25 independently supplies 927 physical opcode starts with effective views, now classified Observed; zero aggregate drops. `evidence/startup25-observed-review.json` ties every fetched instruction to decode and physical offset. Checkpoint pass-001-startup byte-identical.
- Bounded static continuation: 232 direct-control roots from observed instructions; 88 distinct new instructions across coherent alternate paths (sound driver, PPU stream, setup) reviewed in `evidence/startup-static-new-review.json` and annotated Static. Natural branch use not claimed. Indirect dispatch remains a barrier; do not linear-decode inline words after $D7A2 as instructions.
- Title progression: initial RAM-write recipes incorrectly used `offset`, whereas write API requires `address`; they patched RAM0. Corrected recipe has materially different actual addresses and was tested from clean snapshot. Title scroll shortened, but bank6 background animation still runs. `evidence/decode-ee11.json`, `evidence/decode-b767.json` and stack artifacts establish nested animation/frame-delay contracts. Next experiment uses explicit $EEAE entry and restored stack contract to finish title init. All edits artificial; no natural progression asserted.

No toolkit defect established. Long-lived tool processes twice ended 143 without stderr; this may be tool execution lifetime, not emulator behavior. Keep per-batch calls brief and export snapshots promptly. No artifacts removed.

### Controller correction

Earlier Start recipes used generic protocol's button/pressed shape; NES emulator.input actually expects full `buttons` array. It accepted the former as an empty array, so those were releases, not presses. This explains failure to enter naming despite visible menu. Corrected fresh Start experiment uses buttons:[start], then buttons:[]; old input experiments are superseded, not negative evidence about reachability. Toolkit issue NES-DOC-001 records mismatch. Earlier `offset` writes likewise corrected to `address`; those are an analysis-client error, not a new toolkit defect.

## Connected graphics streams

Graphics selector table $E633-$E648 consumer yields 17 bank-aware header/stream records, 10090 classified data bytes including pointer words/descriptor table. Exact RLE semantics were derived from $E6CD, including escaped 81 and count-minus-one repeats; mode at descriptor byte positions 0..2 uploads eight-byte tile halves twice. All records consume exact counts with no residual repeats. See evidence/graphics-parse-proposals.json, consumer decode-e600/e681/d741.json and scratch/graphics_parse.py. These static claims do not require runtime selection.

Connected $E667 consumer forces bank3 and 1024 output bytes to CHR RAM at PPU $1000, selecting thirteen area tile origins from initial bank3 table. All thirteen parse coherently; first twelve endpoints independently equal the next pointer. Final stream ends physical $F431, leaving $F431-$FE2E unclassified (initial expectation of ending at fill was false; localized to that stronger full-bank extent hypothesis). Fixed consumer count independently supports final stream boundary. Added 11263 data bytes, without treating trailing gap as automatically data. Evidence/stage-graphics-proposals.json, decode-e667.json and scratch/stage_graphics_parse.py.

Gameplay: corrected input reached names, then Greece region map showing ARCADIA (evidence/region-map-loading.png); Start confirms map and began stage loader. Current arcadia-scene capture is still black while stage data uploads; do not claim a visible gameplay visit yet. 686 new instruction starts observed in this initialization; next bounded continuation seeks visible loaded scene.

## Later-game and ending progression

Corrected blanking shortcut (scratch/visit_area_blanked.py) gives valid rendered areas1,4,5,8,11; screenshots `area01b/04b/05b/08b/11b-action.png` show distinct terrain/interior systems and enemies. Each restored clean Arcadia snapshot, asserted button state, loaded bank-aware entrance selector from four-field metadata, invoked documented source reinit after D300, and exercised right/A/B. Area11 has a large flying foe; identity/boss status unconfirmed and no defeat claimed. Initial unblanked tests are superseded only as display/precondition failures; their actual fetched code remains valid evidence.

Ending: bank0 physical $1954 JMP $EEE4 follows object-state countdown and sound-completion checks (decode-9910.json). $EEE4 immediately initializes display/graphics/scene, providing a supported no-register-argument artificial entry. Direct entry rendered gold-column rescue scene with flying figure (`ending-loading.png`); natural boss defeat not established. Scripted fade $B7E3 held execution after40 frames; stack `ending-fade-stack.json` confirms return $EF15. Restoring SP=$FF and resuming at $EF16 skips that fade, calls scene motion init, then waits for soundtrack. Credits suffix $EF33 is independently self-initializing and loops forever via $EF54; a bounded direct suffix experiment is underway to visit terminal credits without long music/fade waits. These shortcuts are explicit artificial scenarios, not natural completion.

Static continuation from all observed direct-control roots: 819 unique context roots, 63 distinct new instruction starts reviewed in evidence/all-static-new-review.json. Later observed overlap was skipped at commit time. Short coherent branch slices retained Static; indirect inline tables still barriers. Transition metadata: first three independent subtable origins give equal finite slices, fourth annotated only as supported same-domain prefix; no complete fourth-array extent claim. Added1848 data bytes (evidence/area-transition-prefixes.json).

## Round01 closeout

ROM classification: 31.93% (41854 / 131088 bytes classified; 89234 bytes unknown).

Final checkpoint pass-001-final verifies byte identity; exact per-bank table is in progress.md and evidence/round01-final-counts.json. Code9625/data32229. Whole-ROM/editorial completion not claimed. Gameplay checklist now records actual verified render/interaction dispositions; inventory visible; house attempts at RAM X80/96 produced zero new code and no dialogue. Unvisited systems and concrete missing contracts are handed to redirect, not globally rejected.

Evidence retention correction: second static expansion accidentally reused all-static-new-review.json/all-static-summary.json filenames. Earlier accepted42 annotations and their views remained in saved state; they were redecoded from unchanged ROM into all-static-initial-accepted-review.json, and the canonical review now combines those42 recovered accepted facts with111 second-expansion candidates. Earlier first expansion totals819 roots/70 candidate views/63 distinct starts are retained in this ledger; second expansion936 roots/111 views is frozen in all-static-round01b-review/summary.json. The accidentally named round01a files are also second-expansion copies; do not treat them as original first capture. Classification evidence is repaired, provenance not promoted, and primary runtime artifacts/ROM/history/checkpoints remain retained. Future captures require fresh names.

One confirmed toolkit documentation/API-validation finding, no blockers: toolkit-issues.md. No toolkit edits. No archive/cleanup/publish/commit performed. This ordinary easy-gain handoff leaves concrete source questions for full commentary then redirect; terminal exhaustion review is not asserted.

## Redirect round01 — coherent ready batch

Attempt analysis/redirect/round01-a01 investigated all five initial avenues, then connected graphics, dialogue, map/title and scene continuations. Final proposals89220 unique new bytes (17112Code/72108Data), not yet accepted; current map remains41854 classified/89234 unknown. Exact raw claims, zero-conflict union and complete complement dispositions are in proposals-raw.json, proposal-review.json and unknown-ledger-final.json. Six ready leads return together; four14-byte residuals have a localized primary-artifact review, not an exhaustion claim.

Major premise corrections:13th area-CHR pointer is special-room directory; audio uses bank6; command1A resumes after synchronization; zero table indices can alias JMP operands; area11 shared graphic lists invalidate a first-pointer-only extra entry. Exact record/stream endpoints, fields, bank contexts, static contracts and Observed artificial fetches remain separate. Door/NPC input contracts now tested successfully; no natural ending claim. See handoff-redirect-round01.md and attempt/contracts.md. No map/source/ROM/toolkit changes.

## Disassembly round02 — classification and remaining frontier

ROM classification: 99.99% (131074 / 131088 bytes classified; 14 bytes unknown).

Input pass-001-final; output pass-002-final. Accepted 89220 new unique bytes (17112 Code / 72108 Data), independently reviewed coherent R01-L01..L06 batch. Total Code 26737, Data 104337. Header16 Data counted separately from bank table.

|16 KiB bank|8 KiB bank pair|Code|Data|Unknown|
|---|---|---:|---:|---:|
|0|[0, 1]|12576|3808|0|
|1|[2, 3]|3|16381|0|
|2|[4, 5]|3|16375|6|
|3|[6, 7]|3|16375|6|
|4|[8, 9]|3|16381|0|
|5|[10, 11]|116|16268|0|
|6|[12, 13]|884|15500|0|
|7|[14, 15]|13149|3233|2|

Review: [structural reasoning](disassembly/round02/review.md), [independent byte/context validation](disassembly/round02/independent-validation.json), [accepted annotations](disassembly/round02/accepted-regions.json), [exact summary](disassembly/round02/summary.json). Old849-byte false AreaTiles_0C interpretation split into specific special-room directory/records/body roles; previous annotation retained. BIT/LDA overlap entries retained with exact-byte checkpoint representation and entry-views.json. Seven reset INCs Observed, suffixes Static instruction-template Data; copied sound code Static with explicit contracts and unresolved actual callers.

Existing ordinary/late-area, rescue/terminal-scene and successful NPC gameplay evidence reused; no unchanged replays. Execution coverage is separate: no new emulator exploration was performed this pass and no natural reachability promoted. All accepted Code has effective views; all classified bytes have evidence/provenance; semantic field knowledge and current-source editorial coverage remain separate. Current-source commentary must carry forward/review the new full map; commentary/source and before remain intact.

Residual14 bytes at BE28..BE2E,FE28..FE2E,1CC9C,1CDE5 remain unknown under localized primary review; this is ordinary easy-gain exhaustion, not terminal exhaustion or ROM completion. Full current-source commentary then redirect are still required. Toolkit report: one existing confirmed documentation/API-validation issue NES-DOC-001, zero new findings, no blocker; [report](toolkit-issues.md). No toolkit/input/archive/commit/publishing changes.

## Disassembly round03 — verified narrow suffix batch

ROM classification: 100.00% (131086 / 131088 bytes classified; 2 bytes unknown).

Incomplete despite rounding: Code26737, Data104349, Unknown2. Accepted12 Static generic Data bytes at BE28..BE2E and FE28..FE2E; independent counted graphics origin, literal-output endpoints, same-position copies and boundary/context review jointly distinguish the tiny-code alternative. No bank2/3 live consumer or original generator claim. R01-L07 verified; prior history retained. Residual1CC9C/1CDE5 remain competing redundant return versus padding/data; no forced classification.

Evidence: [independent review](disassembly/round03/review.md), [measured validation](disassembly/round03/independent-validation.json), [accepted regions](disassembly/round03/accepted-regions.json), [exact counts/provenance](disassembly/round03/summary.json). Provenance totals Static120740 / Observed10346. Header16 Data separate from banks below.

|16 KiB bank|Effective8 KiB pair|Code|Data|Unknown|
|---|---|---:|---:|---:|
|0|[0, 1]|12576|3808|0|
|1|[2, 3]|3|16381|0|
|2|[4, 5]|3|16381|0|
|3|[6, 7]|3|16381|0|
|4|[8, 9]|3|16381|0|
|5|[10, 11]|116|16268|0|
|6|[12, 13]|884|15500|0|
|7|[14, 15]|13149|3233|2|

State/current saved and validated; map/state SHA256 d60123b8f0dcffb8ab9cd1c972680b1e87509085ece9ae40282f705fd6196bdc. Fresh checkpoint pass-003-final verification linked in handoff. Existing source, symbols, reading and ROM hashes unchanged. Accepted full-map/source and representative gameplay/late-region/NPC/terminal-scene evidence reused; no unchanged runtime replay or natural reachability promotion.

Next phase full current-source commentary must retain all prior authored commentary, cover the12 new Data bytes, and close all seven exact stale-comment defects in redirect/round02-a01/editorial-followups.json. Current-source and final editorial gates pending. One existing toolkit documentation issue NES-DOC-001, no new toolkit issues found, no blockers; [report](toolkit-issues.md). No toolkit edits, input edits, archive execution, cleanup/deletion, publishing or commit. No terminal-exhaustion claim.

## Batch-only Manual fallback disassembly — pass-004-batch-fallback

ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).

Explicit user-authorized batch-only exception applied after independent current internal-map complement verification: physical [117916,117917) / [0x1CC9C,0x1CC9D), and [118245,118246) / [0x1CDE5,0x1CDE6), one byte each; two <=10 threshold. Both annotated generic Data of unknown meaning with Manual provenance using supported map.annotate. Prior redundant RTS versus padding/data ambiguity persists; this is not evidence-backed role inference. Code26737 / Data104351; Static120740 / Observed10346 / Manual2. Evidence-backed classified total remains131086; no full evidence-backed completion claim.

Header16 Data; bank0 Code12576/Data3808, banks1–4 each Code3/Data16381, bank5 Code116/Data16268, bank6 Code884/Data15500, bank7 Code13149/Data3235; all banks Unknown0. No new gameplay or redirect research; accepted prior role investigation reused within its limits. Full exact counts, authorization, annotations and preserved pre-change state/map/audit/terminal conclusions: [batch review](disassembly/batch-fallback-001/review.md), [summary](disassembly/batch-fallback-001/summary.json).

map.validate ok, state saved and current map exported; both SHA25601fbd61b4859c18d45fcf1fe542919df14e8be76ab42f69a03869e8bd0eb3742. Fresh pass-004-batch-fallback validates and rebuilds byte-identically: [verification](pass-004-batch-fallback-verification.json). Input ROM and prior commentary/source/symbol/reading unchanged. Full current-source commentary then final editorial/inventory audit pending; no commentary phase performed here. Existing unit-link repairs untouched. [Handoff](handoff-disassembly-batch-fallback-001.md).

Supported APIs: api.describe, map.load, map.annotate {region}, map.validate, map.save, map.export. Existing NES-DOC-001 only; no new toolkit issues found, no blocker; [report](toolkit-issues.md). No ROM/toolkit edits, archive/deletion/publish/commit. Prior historical records retained.
