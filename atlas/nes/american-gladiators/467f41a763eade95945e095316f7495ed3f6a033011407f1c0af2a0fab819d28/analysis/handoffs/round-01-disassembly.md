# Round 01 disassembly handoff

## Round 01 disassembly — completed pass, ROM incomplete

ROM classification: 54.94% (144024 / 262160 bytes classified; 118136 bytes unknown).

One disassembly pass completed. Baseline and latest checkpoint both validate and rebuild
byte-identically with configured release binary; ROM SHA-256 unchanged. Latest verification:
[round-01-disassembly-verification.json](../round-01-disassembly-verification.json);
[map](../../maps/round-01-disassembly.json), [current source](../../generated/round-01-disassembly/game.asm),
[internal state](../state.json). Earlier batch checkpoint preserved.

Classification: 10,632 Code; 2,320 Data (16-byte iNES header, 6-byte vectors and 2,298
uniform non-code bytes); 131,072 Image (complete iNES CHR tile allocation). Exact unique
interval union counted from internal map; raw export fallback is excluded.
Provenance bytes: {'Observed': 9995, 'Static': 134029}. Header/CHR allocation, decoded instructions, contextual
fetches, direct control-flow and uniform boundaries supply separate structural evidence.
Uniform role remains generic data: specific fill purpose/consumer not asserted.

Execution coverage is separate: 4739 unique accepted physical opcode starts,
6148 effective contextual roots across retained natural/password routes.
Trace summaries report aggregate_dropped=0; capacity-zero captures intentionally dropped
detailed events. These are coverage aggregates with replay/snapshot evidence, not exhaustive
instruction traces or all-ROM execution coverage. Artificial terminal roots excluded from
classification and these execution counts.

| Physical bank | Code | Data/image | Classified | Unknown |
|---|---:|---:|---:|---:|
| Header | 0 | 16 | 16 | 0 |
| PRG00 | 1753 | 1328 | 3081 | 5111 |
| PRG01 | 568 | 0 | 568 | 7624 |
| PRG02 | 1092 | 0 | 1092 | 7100 |
| PRG03 | 216 | 0 | 216 | 7976 |
| PRG04 | 1491 | 0 | 1491 | 6701 |
| PRG05 | 0 | 0 | 0 | 8192 |
| PRG06 | 2845 | 0 | 2845 | 5347 |
| PRG07 | 0 | 0 | 0 | 8192 |
| PRG08 | 0 | 0 | 0 | 8192 |
| PRG09 | 0 | 48 | 48 | 8144 |
| PRG10 | 0 | 0 | 0 | 8192 |
| PRG11 | 0 | 0 | 0 | 8192 |
| PRG12 | 0 | 0 | 0 | 8192 |
| PRG13 | 0 | 0 | 0 | 8192 |
| PRG14 | 0 | 87 | 87 | 8105 |
| PRG15 | 2667 | 841 | 3508 | 4684 |
| CHR00 | 0 | 8192 | 8192 | 0 |
| CHR01 | 0 | 8192 | 8192 | 0 |
| CHR02 | 0 | 8192 | 8192 | 0 |
| CHR03 | 0 | 8192 | 8192 | 0 |
| CHR04 | 0 | 8192 | 8192 | 0 |
| CHR05 | 0 | 8192 | 8192 | 0 |
| CHR06 | 0 | 8192 | 8192 | 0 |
| CHR07 | 0 | 8192 | 8192 | 0 |
| CHR08 | 0 | 8192 | 8192 | 0 |
| CHR09 | 0 | 8192 | 8192 | 0 |
| CHR10 | 0 | 8192 | 8192 | 0 |
| CHR11 | 0 | 8192 | 8192 | 0 |
| CHR12 | 0 | 8192 | 8192 | 0 |
| CHR13 | 0 | 8192 | 8192 | 0 |
| CHR14 | 0 | 8192 | 8192 | 0 |
| CHR15 | 0 | 8192 | 8192 | 0 |

Gameplay: title/player/start-password menus, default first event, sourced and confirmed
Level03/Lives10 password, and all five selectable events with representative input.
See [scenarios.json](../scenarios.json), [gameplay.md](../gameplay.md), captures follow-0..4.ppm
and matching durable snapshots under round-01/. Early and short-transition failures retained.
The first final/ending artificial test patched $0591/$0592, $0590, $0595 and entered
$89F2/$8B89 with stack $FF. Both froze; final screen remained menu, ending PC reached $0002.
The partial ending run ended before final snapshot; request/reply evidence is preserved.
Missing dependent entry/dispatch state prevents claiming successful final/ending visits.
No terminal artificial fetches justify intended-code annotations. Do not repeat unchanged.

Ready handoff: indirect bank-aware sound/engine dispatch, inline-parameter call formats,
completion/Eliminator initialization and residual PRG data/code boundaries. Details in
[leads.json](../leads.json), [investigation.md](../investigation.md), and
[unknown complement](../round-01/unknown-complement.json). Other level permutations and full
ending are deferred to concrete state-contract research; this is an easy-gain handoff,
not a terminal exhaustion assertion. Full source commentary/editorial audit remains.

Toolkit: **no toolkit issues found**. [Persistent report](../toolkit-issues.md).
