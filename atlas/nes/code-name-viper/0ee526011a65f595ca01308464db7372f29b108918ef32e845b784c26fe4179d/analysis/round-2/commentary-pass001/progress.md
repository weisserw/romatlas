# Round01 commentary — current-source phase complete

Read the entire 10,110-line prepared source, preserving frozen before/ and baseline.
All 527 classified code regions (14592 bytes) now belong to177 explained mechanical
units. Source covers entry contracts, control flow, algorithms, state changes,
object/metasprite/PPU transfer layouts, room/camera operations, audio records,
interrupt/bank restoration and honest unresolved continuations.

Added 950 generated-in-phase routine/local labels,51 concrete branch refinements,
1050 symbolic control references, consistent hardware/RAM/object symbols and54
consumer-table/audio-field aliases. Labels retain suffixes for bank-aware fragment
identity; source prose explains shared units rather than treating every discovered
fragment as a routine. Unknown execution views remain numeric.

All 109 classified non-code scaffold entries have boundary/role notes; every 228
unknown interval has a fidelity/uncertainty note. The864-entry inventory remains
intact and reviewed for current readability. Final editorial audit remains pending.
Source anchors and unit records are in review.json; compact unit/location artifacts
are in analysis/commentary-round01/. No ranges remain unread or deferred for
current-source readability. Whole-ROM classification remains incomplete.

Concrete review corrected spawn eligibility ($88 is rejected), distinguished
reset-stage/object-clear service vectors, and retained the$f8ce call as an unknown
helper rather than a mapper operation. Unresolved meanings and two new concrete
questions are in uncertainties.md. No classification, ROM, maps or toolkit changed.

Final edited-source verification: verification.json, ok=true and
byte_identical=true; ROM SHA256
0ee526011a65f595ca01308464db7372f29b108918ef32e845b784c26fe4179d.
Source/game.asm SHA256
5cd4fc40747c05bb643b293d6b82166437774d80a067606ef3f62e62a2231394;
constants.inc SHA256
14cfafe930621d90b84e8c3f302dfd4fb082b4016de4bb9ad43899975deddfeb.
Reading-after.asm refreshed after that verification. No final audit invoked.
Disk checked: 3.5 GiB available; commentary helper artifacts 192 KiB; no cleanup
needed or evidence deleted. No toolkit issues found.
