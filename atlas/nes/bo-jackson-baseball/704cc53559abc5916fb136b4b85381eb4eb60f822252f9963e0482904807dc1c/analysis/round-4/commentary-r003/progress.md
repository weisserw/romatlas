# Commentary progress

Round 1 active. Frozen baseline matches disassembly-r001; configured helper preparation verified byte identity. Source inventory: 966 classified code blocks in 231 contiguous spans, plus classified data/image and 260 unknown intervals.

Initial source reading completed for physical 0x4EA9–0x15C4F code spans (banks 2, 9, 10). Detailed mechanical notes are saved in analysis/commentary-notes-r001.tsv. Applying consistent symbols and source prose after the full inventory reading. All review scaffold entries remain pending until edits are verified; no readability completion claim. Next source frontier: bank 11, file 0x16010. BIT-skip alternate entries remain untouched. Final editorial audit reserved.

Source reading now covers banks 11 and 12 through physical 0x19E7D. Identified six-channel audio sequence/pattern machinery, bounded replay run records, actor destination/rectangle assignment and line-style movement. Individual state names remain tentative; mechanical explanations are being preserved. Next frontier: bank 13 at 0x1A010.

Bank 13 reading complete through 0x1BC78: settings/team/roster menus and pitcher/batter-stat parameter machinery. Sound/menu cursor entries and fragmented continuations are explicitly described without extending classified bounds. Next frontier: bank 14 at 0x1C2C8.

Initial whole-source code reading finished through bank 15, physical 0x1FFB7. All 231 contiguous spans have hand-authored mechanical summaries in analysis/commentary-notes-r001.tsv. Applying these to buildable source plus bank-aware control-flow labels and shared RAM/I/O symbols; data/unknown inventory notes follow. Readability still active, no redirect handoff.

Edited source checkpoint: all 231 code spans now have explanations, 1,458 descriptive routine/control labels and 231 shared hardware/state definitions. Every data/image/unknown interval has an adjacent note. First edited-source verification passed byte identity. Consistency pass corrected mixed-branch join names, added individual banked helper contracts and localized unresolved semantics. Review/source-location inventory and final fresh verification remain in progress.

## Round 1 current-source readability complete

Reviewed all current classified source: 20,869 code bytes in 966 structural code blocks / 231 contiguous spans, 71 Data regions and one CHR Image region. All 260 unknown intervals have honest local notes. No readability ranges remain. `review.json` retains its complete 1,298-entry physical scaffold; every entry is reviewed for this phase, with 313 unique routine/structure units and preserved bank context. Final editorial audit remains pending.

Source now has 1,458 routine/control labels, 231 shared RAM/hardware symbols, five bounded-domain constants and five supported alternate-entry constants. Explicit a:/z: widths, CPU aliases, physical identities and BIT-skip encodings are retained. Only 35 direct numeric control operands remain; their destination body/CPU-alias identity is not named in this execution view. Uncertain game meanings are local and summarized in uncertainties.md.

Verification after the coherent edit and consistency batches rebuilt the entire 393,232-byte image byte-identically with the configured binary. Fresh final verification and reading-copy refresh are recorded in verification.json and reading-after.asm. Text-only local checkpoint: checkpoints/r001; its existing local assets remain in source/. Whole-ROM classification is unchanged at 81.65%, with 72,166 bytes unknown. Full hardware compatibility and final editorial acceptance are not claimed.

Scope details and hashes: ../analysis/commentary-r001-scope.json. Routine notes: ../analysis/commentary-notes-r001.tsv. No toolkit issues found; no toolkit edits, classification edits, publication, archive, cleanup or deletion occurred.

## Round 2 current-source readability complete

Input `disassembly-r002` adds 67,974 classified bytes: 16,341 Code and 51,633 Data. The entire previous commentary tree, including frozen source/baseline/review and local assets, is preserved at `analysis/round-2/commentary-r001`; `checkpoints/r001` and the r001 handoff are unchanged. Active canonical baseline/state/manifest/before now match r002. Existing readable source was incrementally maintained: only formerly unknown binary slices received new code/data, and valid prior prose/labels remain.

All 229 new contiguous native spans were hand-read with notes in `analysis/commentary-notes-r002.tsv`. All 195 newly classified Data regions have adjacent format/consumer or bounded uncertainty explanations, and all 79 remaining unknown intervals have honest local notes. Existing classified code/data/CHR credit was carried by exact physical range and inspected for new symbol/dependency consistency. No classified-source readability ranges remain.

The current full-ROM review scaffold has 8,657 entries and 737 unique meaningful routine/structure units, including 313 carried units. New explanations cover replay restoration/run decoding, ball/contact/probability and signed-motion helpers, actor target/state predicates, playoff bracket/setup/roster formatting, framed windows and stack-page text queues, background bootstrap/scroll transitions and finite audio streams/tables. Understood new RAM fields and control targets are used consistently in carried/new source. Eighteen additional data structures have descriptive source labels and separate bank-scoped CPU alias constants. Three new BIT alternate entries preserve their exact outer encodings.

Coherent integration and consistency checks each rebuilt the full ROM byte-identically using the configured release binary. Final fresh verification/reading refresh are recorded in `verification.json` and `reading-after.asm`; baseline verification and reading-before reflect r002. Final editorial audit remains pending for the coordinator, and was not run. Classification remains separate: 389,040 / 393,232 bytes classified (98.93%), including 37,210 Code, 89,686 Data and 262,144 Image; 4,192 bytes remain unknown.

Remaining game meanings/preconditions and concrete redirect constraints are in `uncertainties.md`; no readability work is deferred to redirect. Scope/hashes: `analysis/commentary-r002-scope.json`. No toolkit changes, classification changes, archive, publication, cleanup or deletion occurred.

## Round 3 active integration

Complete prior tree preserved at `analysis/round-3/commentary-r002`. All 80 new native spans have been hand-read and receive mechanical notes in `analysis/commentary-notes-r003.tsv`; all new/corrected data intervals have consumer/format explanations. Integration preserves prior physical source and replaces only new classifications plus the two-byte DBF4 prefix and three-byte roster template correction. Canonical baseline/state/manifest/before now reflect `disassembly-r003-final`. Readability review locations, consistency verification and local checkpoint remain in progress; no final editorial audit has run.

## Round 3 current-source readability complete

All current classified source reviewed, including 4185 additions, the two-byte DBF4 code prefix and the three-byte roster template correction. All eighty new native spans have hand-reading notes; eighteen newly classified Data regions and the retained sixteen-byte parameter block have local explanations. Valid prior review/name/prose credit is preserved; affected roster and DBF4 records are narrowed. The current inventory has 10345 entries and 835 unique meaningful units with refreshed source locations. No readability ranges remain.

The roster template now has its exact twenty-five-space/NUL consumer explanation; the spurious JSR2020 and RAM-upload question are removed. New explanations cover ball prediction/assignment filters, diagnostic records/stack restoration, tile-row transfer and attribute merging, replay/phase continuations, controller-response tables, overlay setup and menu timeout exits. The 906D cleanup explicitly requires a saved mode byte above the ordinary return address and does not prove 948B returns. Both residual unknown ranges have their exact competing interpretations.

Canonical baseline/state/manifest/before and frozen reading-before match `disassembly-r003-final`; full prior commentary remains at `analysis/round-3/commentary-r002`. Fresh configured-release baseline and maintained-source builds both verify byte-identically; `verification.json` and refreshed `reading-after.asm` describe the final source. Scope/consistency: `analysis/commentary-r003-scope.json` and `commentary-r003-consistency.json`; checkpoint `checkpoints/r003`, compact handoff `analysis/handoff-commentary-r003.md`. Classification remains separate: 393225 classified bytes and seven Unknown bytes at56D4/6 and1DF7F/1. Final editorial audit remains pending and was not invoked. No toolkit changes, redirect, archive, publication, cleanup or deletion.
