# Round01 commentary handoff

Current-source readability phase complete; ROM classification remains incomplete.
No redirect/disassembly/audit/archive phase ran. Overall goal remains active.

Scope: all 864 scaffold intervals read and explained: 527 code regions (14592
bytes), 109 non-code regions (139549 bytes), 228 unknown intervals (108019 bytes).
All classified code belongs to 177 mechanical unit descriptions. Generic non-code
has bounded role/uncertainty notes; unknown intervals explicitly retain structural
ambiguity. No readability ranges are deferred. Final editorial acceptance remains
pending in review.json; it was not conflated with this phase.

Maintained source: commentary/source/game.asm plus constants.inc; frozen before/
and baseline remain intact. Examples: FindSpawnRecordList, CompareObjectRectangles,
RenderObjectMetasprite, MatchPasswordRecord, ProcessPpuTransferQueue,
AccumulatePitchModulation, HandleNmiAudioAndFrameCounters and
SelectAndRememberPrgPair. Object fields, RAM/hardware registers, audio fields and
consumer-view tables use consistent symbols, preserving explicit addressing width.
Physical/bank identity remains separate from CPU aliases; unresolved banked calls
stay numeric. Fragment suffixes distinguish continuations of explained units.

Fresh final verification: commentary/verification.json ok=true,
byte_identical=true. Reading copy refreshed afterward: commentary/reading-after.asm.
Source SHA256: 5cd4fc40747c05bb643b293d6b82166437774d80a067606ef3f62e62a2231394.
Constants SHA256: 14cfafe930621d90b84e8c3f302dfd4fb082b4016de4bb9ad43899975deddfeb.
ROM SHA256 unchanged: 0ee526011a65f595ca01308464db7372f29b108918ef32e845b784c26fe4179d.
Current/map/checkpoint/internal/frozen state all retain map SHA256
2c7c765e75c29d2c26d5f67ca313efbe26f748123cac6bc751eee40fdb0ad7f6.
Upstream checkpoint remains maps/pass-001.json, generated/pass-001/game.asm.
Readability checkpoint is the maintained commentary source, not that generated copy.

Redirect questions: commentary/uncertainties.md. New concrete findings:
1. Password matcher physical112978..113088 uses limit hexadecimal $54 (84),
   while accepted data is 54 decimal records. Inspect the adjacent candidate bytes
   and metadata accesses; do not expand classification from the loop bound alone.
2. FlushScrollBuffers physical117728..117738 reads ROM CPU$c6ab and tests bit3.
   Its purpose is unresolved; retain the literal condition rather than a RAM alias.
Continue existing L004 boss/ending, L005 late-map arrival and L006 residual formats.
State $1c/$1e calls to $f8ce are unresolved helper contracts, not mapper operations
or established endings. Audio identities, several player/terrain state meanings,
unknown table extents and original uniform-data purposes remain honestly uncertain.
These meanings do not create skipped readability work.

Review records: commentary/review.json and progress.md; compact unit descriptions,
source anchors and historical edit recipes: analysis/commentary-round01/.
Recipes are not idempotent; do not rerun over maintained source. On map changes,
preserve this commentary before preparing a fresh baseline and carry valid review
by physical ranges/dependencies as instructed.

No toolkit issues found; analysis/toolkit-issues.md remains the persistent report.
Disk checked: 3.5 GiB available; helper records approximately192 KiB. No toolkit
edit, ROM change, deletion, publishing or archive operation performed.
