# ROUND003 COMMENTARY continuation C — incomplete, resume commentary

Writer C stopped for sequential context continuation. This is **not** a successful commentary handoff; do not advance to REDIRECT. Classification remains **99.54% (260956 / 262160 classified;1204 Unknown)**. Final editorial audit remains pending and was not run.

Read prompts/commentary.txt and commentary/{AGENTS.md,workflow.md,prompt.md}. Use continuation A for preservation/baseline/carry provenance and continuation B for prior corrections. Preserve the full historical tree. Do not regenerate maintained source, prepare again, replay historical one-shot scripts, archive, clean/delete, change ROM/map/state/toolkit or publish local artifacts. Configured binary remains `/root/romatlas/target/release/romatlas`. C scripts are historical one-shot transforms too; **do not rerun them**. `checkpoint_c.py` is an idempotent inventory/anchor checker, not a source transform.

## Completed in C

- Batches014–016: **all29remainingbank14Code scaffolds/1008bytes**, with46new units across raster IRQ, NMI/video upload, HUD, contact, pause and player-life/saved-state continuations. Ordinary calls, synthetic RTS destinations and saved IRQ restoration are distinguished. Fixed-table operands use explicit CPU aliases, separate from physical `.incbin` labels.
- Batches017–018: **22bank6Code scaffolds/983bytes**, with37new units. Life-status/continue APIs and screens, choice/OAM handshakes, palette/screen-sequence tails, ending reset/nine-actor clear, terminal timer/input/sound-cycle, transfer descriptors and coordinate helper are explained. Existing good prose retained.
- **11dependency checks**: DEP009,016,099,104,108..112,115,119. Current inventory **47reviewed/92pending of139**. Filename `commentary-dependency-candidates-a.json` remains historical; its contents/status/anchors are current. These are candidates requiring concrete checks, not blanket errors.
- Corrected the stale claim that bank6 CHR setup writes register0 twice: loop includes STY and increments Y, so it writes registers0 and1 then2..5. Updated matching review-unit summary. Concrete stale gap claims in touched dispatch/HUD/contact/life material corrected.
- Life-switch restart at **C238** enters the bank6 screen-initialization call and skips life initialization at C235. Do not replace it with F14InitializePlayerSession. Continue setup's **AB58** is the palette-record submitter, distinct from AB49 command sequence. Both exact aliases passed byte verification.
- Kept B corrections: C018 unsigned Multiply8By16Entry; divider0216high/0217low; actor/playerX names; interleaved17+4-byte initialization; terrainX threshold fields. No classification/reachability changes.

## Exact remaining readability

`commentary-pending.json` and `commentary-pending-by-bank-c.json` are exact authoritative remaining IDs/ranges. **60Code scaffolds/3408bytes are wholly unread; no partial scaffold credit.** No pending Data/Image/Unknown notes. All valid prior review retained.

| Physical PRG bank | Pending Code scaffolds | Bytes |
|---|---:|---:|
|6|2|1147|
|7|29|1090|
|13|29|1171|

Bank6 remaining:
- region-1848 physical50302..50887, CPU A46E..A6B7,585bytes. Menu/session option, timed blink/briefing character/transfer graph. Entire scaffold unread; do not claim setup-only coverage. Several meaningful routines and branches need individual prose/names/units. Data at50887 already reviewed but consumer upkeep may revise interpretation.
- region-1880 physical55166..55728, CPU B76E..B9A0,562bytes. One/two-player choice/selection input and descriptor graph. Entire scaffold unread. Includes several helper entries and session flag semantics affecting bank14 life logic; follow scoped consumers/callers.

Then ALL29bank7 and29bank13 continuations, plus all92scoped dependencies. Prefer coherent graph batches; maintain exact partialsubranges if a large scaffold cannot complete. Add meaningful routine/control-flow/RAM/field/constant naming and comments, not a generic unit spanning each huge scaffold. Natural reachability remains distinct from structural mechanics. Keep honest uncertainty near unresolved game meanings/Unknown boundaries.

## Verification and inventories

Current2373scaffolds: **2313reviewed/60pending,2889units**. `commentary-scope-assessment-c.json` records fresh byte-identical verification, full partition, reviewed Code units and source-tree-relative metadata checks with0errors. These consistency checks do not establish full readability. `commentary-partial-coverage-c.json` has no pending partial ranges.

Map SHA256 `4f2424c84e630ea624df17aa8dfa32fbdd5cb1422ca3a095e04a5b9354b22f12`, checkpoint `round-003-disassembly-final`; unchanged. Source hashes:
- game.asm `14f6988a42ec0ac5dc96edcf718b0c39b3ffad9b998b87efa0c139e39c0742ca`
- symbols.inc `25791906769c6934a57eab0d8a90670b306cbdea6ad0c87be0ce36f57134c9a9`

All three progress paths contain C batch018/current exact frontier: commentary/progress.md, analysis/progress.md, analysis/round-003/commentary-active-phase.md. Source, assets, old trees and evidence remain local/preserved. No successful analysis/round-003-commentary-handoff.md was created.

Initial batch checks caught local naming errors (physical data labels used as CPU aliases; missing secondary entry symbol; C238 vs C235 and AB58 vs AB49). Corrected before fresh passing byte verification. No toolkit issue or map change was required.

Resume same phase sequentially. Fresh verify/read and complete ALL current-source readability before any successful commentary handoff. Final audit is reserved for terminal editorial acceptance; do not run it while intentionally pending.


## Superseded by completed ROUND003 readability handoff

ContinuationDcompleted allcurrent-sourceCode/dependencies and the scopegate. Do not resume the old pendingfrontier. See analysis/round-003-commentary-handoff.md and analysis/round-003/commentary-scope-assessment-final.json. Earlier continuation records remain historical and preserved. Finaleditorialacceptance remains separate/pending.
