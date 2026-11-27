# Round1 commentary handoff

Current-source readability COMPLETE; ROM classification INCOMPLETE. All7800 scaffold
entries reviewed:7286 Code,223 Data/Text,16 Image and275 explained Unknown intervals.
All7525 currently classified regions now have readable source contracts/formats.
387 meaningful units,1245 named labels,358 equates. No current-source readability gap
or pending region remains. Final editorial audit remains pending and was not run.

Buildable local source: commentary/source/game.asm. Fresh verification:
commentary/verification.json, byte_identical=true. Source SHA256
7fefa470f60fbfb9b9f3c25176fd57ceb5161ae6adffb1740bed743a24b9f661.
Refreshed commentary/reading-after.asm SHA256
e7de6aebf85a2c04b2d6b67f97ad8ee214b976d997e7469bdedc9f4ff5a45a18.
All review source references now point to current semantic labels/scaffold locations.
Scope assessment: commentary-round1/scope-assessment.json; unique unit index:
commentary-round1/units.json. Exact remaining.json is empty.

Frozen pass-001g map and current map unchanged:
7c2b49e7c4417c10fe6a7b5a6f107cb5b48a40b0c46b734bf80c3d139c5c4fbd.
Counts unchanged: Code16908, Data30233, Text654, Image131072; Unknown83293/262160.
ROM classification remains68.23%. Input SHA256 unchanged424b05432af2c0c88926373037b7e8264ed21f618ed25f0d73ee67ced9620f37.
No new structural annotation or execution-coverage claim follows from commentary.

Examples: battle class masks, choice bounds and decimal conversion; tactical actor
fields, timed phases and alternating clipped metasprites; setup state/RTS dispatch,
stage bounds and CHR split; audio tracks/envelopes/overlay priority; queued target
routing; field messages/PPU strips,3x2 cells, attribute quadrants, unit records,
neighborhood sampling, movement costs and fixed MMC1/controller timing helpers.
Generic data retains useful boundary/uncertainty notes and known consumers. CHR
layout and physical-bank versus mapper-view distinction are explained once.

Concrete corrections: $C7AB neighbor selector is movement, proved by its
MoveSelectedUnitToNeighbor consumer; it is separate from battle entry $CA8B.
Renderer scratch$1A is pieces remaining during rendering, not actor slot. Modal
nested service ladder performs16 cycles through fall-through doubling. Setup PPU
packet bit6 literal/fill convention differs from the battle packet consumer.
154 cross-bank/remaining control aliases require a unique physical target under
ALL saved effective views plus matching CPU symbol value; evidence retained in
commentary-round1/call-symbol-links.json. The12 remaining numeric control operands
all point into structurally Unknown intervals. No classified direct target is numeric.

Redirect questions retain ordinary handoff status, not terminal exhaustion:
- R1-L01: UpdateStagedComparisonState ($01B20B..$01B24F) has exact0/1/3 comparisons,
  while CountUnitsForActivePlayer ($01F4FE..$01F53C) counts ownership bit6. Neither
  proves objective identity/campaign terminal sufficiency. Trace compared-byte
  writers and state consumers using corrected-world removal evidence.
- R1-L02: preserve stage14 overlap and stage15 third-group schema exception; compare
  supported natural/password initialization without rejecting generic non-code.
- R1-L03: remaining static-wrapper/handler continuations are localized by named
  accepted prefixes and explicit raw gaps. Reuse prior proof boundaries.
- R1-L04: factory response prefix ($01CB49..$01CB74) now has concrete property/
  capability/message gates documented; production body remains structurally raw.
- R1-L05 full current-source commentary is resolved for this frozen source.
Detailed uncertainty: commentary/uncertainties.md. Existing disassembly evidence and
reproducible scenarios remain authoritative. No new runtime experiment was needed.

Encounter construction traps atCPU$B390 when a coordinate-matching target is absent;
world commit has its separateCPU$F3D2 trap. Preserve tactical-only limits of original
relocated mirrors and reuse corrected-world evidence. No campaign-ending claim.

Toolkit: no new issue; existing confirmed NES-DOC-001 input schema mismatch remains
worked around, no blocker. See toolkit-issues.md. No toolkit edits. No live server,
map writer or tool handle. All ROM/state/source/assets remain local; no archive,
publication, deletion or cleanup. Disk3.0GiB free. Coordinator proceeds to redirect
only after inspecting this scope/verification handoff; final editorial acceptance is
reserved for its later gate.
