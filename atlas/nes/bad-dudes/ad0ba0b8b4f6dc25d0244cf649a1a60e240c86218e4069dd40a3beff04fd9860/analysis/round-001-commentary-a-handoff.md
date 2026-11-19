# Round 001 commentary — interrupted, resume this phase

Current-source readability gate is **not complete**. This is a context-capacity
checkpoint, not a successful commentary handoff or authorization to redirect.
Resume the same source and inventory without preparing/regenerating a baseline.

Classification remains 165586/262160 (63.16%): Code22423, Data12091,
Image131072; unknown96574. No maps/state/classification changed in commentary.

Readability reviewed3967 Code bytes: all current bank0 sound code, isolated
bank5 initializer physical[47542,47627), all current bank6 code, and fixed-bank
reset physical[130880,131058). All Data/Image entries have functional format or
honest uniform-run notes, and all unknown gaps have explicit physical/bank/role
uncertainty notes. Frozen baseline/before source preserved. Review inventory has
118 compact meaningful units; their physical ranges describe classified entry
blocks, not invented routine extents through unknown gaps.

Remaining: **1060 Code scaffold entries /18456 bytes**, first physical57360
(bank7). Exact authoritative ranges and IDs are in
[remaining inventory](round-001-commentary-remaining.json); every corresponding
commentary/review.json entry remains pending. Continue bank7 onward and revisit
reviewed material only for concrete defects or changed dependencies. No classified
code beyond bank6 is reviewed except the reset interval above. Most code in banks
7–9 and12–15 remains unnamed/unexplained.

Source: commentary/source/game.asm and symbols.inc. Reading copy:
commentary/reading-after.asm. Fresh supported verify passed byte-identically;
ROM SHA256 remains
ad0ba0b8b4f6dc25d0244cf649a1a60e240c86218e4069dd40a3beff04fd9860.
Source hashes: game.asm
6c2c8a2120cd9c7046d6fad3363eaa50d33cce4539ffddef3e82fc6ec475b6c6;
symbols.inc
e31e402d6fa85e47a499d6a025d3c4bce474cda0b4b60d0a4b4dd7dcac1e386d.
Reading copy refreshed after the last source edits. Review inventory edits afterward
only refined unit summaries; no source bytes changed. Final editorial audit unrun.

Examples: sound primary/overlay ownership and fractional clock; stream formats,
nonlocal parser exits, nested repeats and stream calls; 240 metatile indices
expanded into960 nametable tiles; 256 indices packed into64 attribute bytes;
OAM handshake state; collision cell/page/type/quadrant lookups and numeric
stage-specific substitutions. RAM symbols stay local where CPU aliases are reused.
Global hardware symbols preserve addressing modes. Do not globally replace sound
zero-page symbols into other bank code.

Structural question L007 is recorded in leads.json and commentary/uncertainties.md:
physical$576 / bank0 CPU$8566 currently Code BRK is also read as alternate duty
lookup data by$8515. Preserve current classification until redirect/disassembly
examines the entry contract and bounded table extent. Sound pointer/command tables
and intervening code gaps remain honestly unresolved; no unsupported semantics.

Toolkit unchanged. Existing NES-001 documentation/API mismatch has buttons-list
workaround; one confirmed bug, zero suspected, zero missing features, no blocker.
No final audit, archive, cleanup or publication. Persist continued progress in
commentary/progress.md and analysis/progress.md; only a complete current-source
review plus fresh verify/read may advance to redirect.
