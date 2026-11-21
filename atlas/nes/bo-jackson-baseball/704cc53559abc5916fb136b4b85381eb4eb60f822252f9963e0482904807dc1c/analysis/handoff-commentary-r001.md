# Commentary r001 handoff

**Current-source readability complete; ROM classification remains incomplete.** Final editorial/archive acceptance is reserved.

Input: frozen `disassembly-r001` map/source. The current map and internal store were not edited. ROM classification remains 321,066 / 393,232 bytes (81.65%): 20,869 Code, 38,053 Data, 262,144 Image, 72,166 Unknown. ROM SHA256 remains `704cc53559abc5916fb136b4b85381eb4eb60f822252f9963e0482904807dc1c`.

Readable buildable source: [game.asm](../commentary/source/game.asm), [shared symbols](../commentary/source/hardware-and-state.inc). Refreshed [reading copy](../commentary/reading-after.asm). Frozen before/ is intact; local text checkpoint is `commentary/checkpoints/r001`. Assets and snapshots remain local.

Covered **all** current source: 966 code annotation blocks in 231 contiguous spans across PRG banks 2 and 9–15; all 71 Data regions, the complete CHR Image region, and all 260 unknown intervals. Full 1,298-entry [review scaffold](../commentary/review.json) retained; current readability reviewed throughout with 313 unique meaningful units. No unread source ranges or readability gaps remain. Final editorial audit is explicitly pending and was not run.

There are 1,458 routine/control labels, 231 shared hardware/RAM symbols, five consumer-bounded constants and five supported alternate-entry constants. Explicit operand widths, mapper write/read distinctions and CPU aliases are preserved. Numeric control targets remain only where destination source-body/CPU-alias naming is unresolved (35 operands), without fabricated names or classification.

Examples: replay snapshot/run streams, six-channel audio commands/envelopes/APU ownership, scroll-edge tile/attribute construction, actor rectangle assignment and line-style movement, settings/team/roster menus, pitch/batting parameter bands, PCM playback, interrupt queues and metasprite expansion. Mechanical roles are explained even when game names/units are uncertain. The first tracking routine explicitly distinguishes horizontal easing from incrementing a nonzero vertical offset.

The BIT-skip alternate LDY entries at file 0x4F50/0x4F53, 0x4FAB/0x4FAE and 0x1C546 retain their original bytes and useful notes. The PCM record correction is preserved: stored length=count+3; playback begins at header+2 and leaves one stored tail byte. No unsupported full-table, original generator or natural reachability claims were added.

Fresh configured-binary [verification](commentary-r001-verification.json): whole image rebuilt byte-identically. Source SHA256: `game.asm` = `c984d9cebfb96bd1a15d4a98067737c59fe75b518710625a55df268aecd3b604`; `hardware-and-state.inc` = `8b846effff18057688cb47d0ed6bed721a219cf005791972a2b25ff61cf4e34e`. Reading copy refreshed after this source revision. Current map is byte-for-byte identical to maps/disassembly-r001.json. [Scope/hashes](commentary-r001-scope.json), [routine inventory](commentary-units-r001.json), [progress](../commentary/progress.md).

Unresolved meanings are localized in source and [uncertainties](../commentary/uncertainties.md): actor state/flag names, player-stat units/roles, individual audio commands/instruments, quantized projection formulas, settings/mode/series semantics, uniform data consumers and unknown ROM intervals. These are semantic/classification questions rather than remaining readability work.

Next phase may investigate the existing nonterminal [leads](leads.json): menu/PPU structures, bank 6–9 sprite records through actor consumers, bank 10 quantized tables, bank 11 audio formats and playoff series transitions. Named consumers now provide concrete bounded entry points. No terminal exhaustion assessment is made.

[Toolkit issues](toolkit-issues.md): **no toolkit issues found**; no blocker. No toolkit edits, classification edits, publication, archive, cleanup or deletion occurred.
