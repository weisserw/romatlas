# Round 001 commentary handoff

Full current-source readability complete. Next: coordinator-controlled redirect; final editorial acceptance/audit remains reserved. No live process.

Source commentary/source/game.asm, refreshed commentary/reading-after.asm. Full 632-entry scaffold retained and reviewed: Code472 entries/14,089 bytes, Data16/1,340, Image1/131,072, Unknown143/115,659. 120 purpose groups cover every Code fragment with reviewed clipped units and effective bank contexts; sixteen CHR-bank units. No classified-source naming/comment gaps or deferred ranges. Full inventory reconciliation analysis/round-001/commentary/scope.json; summaries/source locations commentary/review.json. editorial_audit intentionally pending.

Names/comments explain player and object metasprites, collision, spawn records, behavior dispatch, collection, fades, title/message screens, sound streams/envelopes/modulation/samples, MMC1/reset/main/NMI, HUD, room entries/streaming and movement. Descriptive routine/branch/RAM/field/buffer symbols and scoped CPU table aliases are used where roles are understood. Every Unknown interval has an honest boundary note. Actor identities, table extents, scene selector domains and generic Data meanings remain explicitly unresolved, not unread work. Detailed uncertainty commentary/uncertainties.md.

Fresh verify passed byte-identically; commentary/verification.json. Source SHA256 160f33f727d4ab34dc12aacf69ef0662040e3a58f2a887d1a997a42a907f3dcd. Reading copy refreshed afterward. Map unchanged SHA256 d9865903bf1c965ce68368ef12bb243fa04dcb03c7a6afaae1577c7b30dc3c5f; ROM unchanged f1ea177a4b51cc4a08799ac17ed5ace63f6e5701febdf310dfb891e6a77da926. Classification remains146501/26216055.88%,115659unknown.

Useful structural questions exposed without running redirect:
- CPU$C06E (physical$1C07E) serializes MMC1 control and falls through into $C081/$1C091 with A shifted four times. There is no RTS between these helpers: control writes also change/cache PRG. Model this actual effect when proving caller successors. CHR writers$C046/$C05A return; PRG$C081 invalidates low-window context.
- Source physical$1F405..$1F42E explicitly builds JMP$C21D at RAM$0104 and JMP$C524 at RAM$0314. Local trampoline origins are therefore established, refining earlier handoff wording. IRQ destination CPU$C524 / physical$1C534 remains one Unknown byte. Bound its entry/return contract before classification. The$0660 copy loop identifies 50 source bytes but copied body is still Unknown.
- Scene initializer/NMI dispatch tables retain established roles. Presentation tick dispatcher CPU$C525, room tick$CC89, PPU handler$D01D, spawn/graphics/exit consumers provide additional focused targets. Consumer-derived table names do not reclassify backing bytes or establish full selector domains.
- Earlier handoff claims Morticia emitter physical$4EA2..$4EA4, but those bytes remain Unknown in this exact map. Retain the observation for focused evidence review; do not count it as classified-source coverage.

All source groups were inspected in physical order. Intermediate duplicate-label build failure repaired, subsequent/final verifies passed. No new toolkit issue; NES-001 retained, no blocker. No emulator experiments, map/state changes, audit, redirect, archive, cleanup, publishing or toolkit edits. All source/assets stay local.
