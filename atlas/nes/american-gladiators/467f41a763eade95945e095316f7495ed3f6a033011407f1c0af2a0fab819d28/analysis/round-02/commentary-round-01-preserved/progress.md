# Round 01 commentary — current-source readability complete

Completed the entire classified baseline at round-01-disassembly: 10,632 code bytes,
2,320 data bytes and 131,072 CHR image bytes. All 278 contiguous code fragments were
read and given functional explanations; 689 code entries/branch joins are named and
explained. Shared definitions provide 268 RAM aliases plus hardware, button and flag
constants. Source is [source/game.asm](source/game.asm), with definitions in
[source/symbols.inc](source/symbols.inc) and refreshed [reading copy](reading-after.asm).

The original generator emitted one instruction per region. Commentary consolidates
those wrappers locally without changing physical order or classified boundaries.
Routine/branch explanations cover menus, password decoding, five-event selection,
completion/level transitions, fixed-point actor motion, collision probes and candidate
position updates, PPU/OAM queues, inline descriptor consumers, cooperative scheduler,
MMC1 operations and audio command/envelope/DMC mechanics. Event-local fields and
unresolved continuations retain explicit mechanical uncertainty.

review.json preserves the full 5,363-entry ROM scaffold, with current-source review
credit and 278 compact physical-fragment units. Unknown-range review means its honest
boundary explanation was checked; it does not classify its bytes. No classified source
range remains pending. Final editorial_audit remains pending for final acceptance.

Fresh verification rebuilt source byte-identically with the configured release binary:
[verification.json](verification.json), also saved as
[round-01-commentary-verification.json](../analysis/round-01-commentary-verification.json).
Source SHA-256: game.asm 1e2d4c449faa27dd7056c4984b26383b80d34a4a577b9c9c630a8c81d45deb92;
symbols.inc 084a25a4b2d1ac2c3c4200e2c2bffea9ac10cbed9db6d56ee2005a10174099e6.
Original map/store and frozen copies still match SHA-256
4c6a44876fb0b171e856cb54a96e453676de8b6ec8feb5da8b25957ddbc42cdf.
Input ROM hash is unchanged. Checkpoint details and scope assertions are retained in
[checkpoint.json](../analysis/round-01-commentary/checkpoint.json).

Classification remains 144,024 / 262,160 (54.94%), with 118,136 unknown bytes.
Execution coverage is unchanged from disassembly; no emulator runs were needed.
Specific unresolved meanings and bounded redirect checks are in
[uncertainties.md](uncertainties.md) and
[redirect-contracts.md](../analysis/round-01-commentary/redirect-contracts.md).
No toolkit issues found; no toolkit changes or final acceptance audit performed.
