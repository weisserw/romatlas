# Round002 disassembly handoff

**One disassembly pass complete; classification incomplete.** Fresh checkpoint `round-002-disassembly`, state/current saved, map.validate passed, rebuild byte-identical.

ROM classification: 98.71% (258766 / 262160 bytes classified; 3394 bytes unknown).

Gain89717 bytes: five independently verified/consumed R001 leads83231 (72971Data/10260Code), plus6448 bounded residual Data and38 direct-edge Code. All new regions Static; no natural selector/reachability inference. All616 input regions exactly preserved. Independent audit redecoded6707 exact CPU/bank views in78 batches, reconstructed15 geometry chains and checked presentation/text/animation/world record extents. No conflicts. Stage14 encoded82 columns accepted without nominal96 claim. [Review](../round-002/review.md), [audit](../round-002/audit-summary.json), [outcomes](../round-002/lead-outcomes.json).

Current counts23329Code/235437other classified,3394unknown in56 gaps. [Per bank](../round-002/bank-counts.json), [exact residuals](../round-002/unknown-ranges.json), [frontier](../round-002/frontier.md). Remaining useful static continuations: balanced PPU-only F26B now supports narrower fixed-bank presentation/text post-call review; CA00 setup entry; three small sound branch intervals;9324 callback,991D incoming pointer and D80F match/return prerequisites. Mixed9EBC code/data and TMR SEGA marker deserve localized inspection. No terminal exhaustion claim.

Natural level1 and artificial stages4/8/14/ending/credits already visited; recipes/status preserved in scenarios.json. No broad replay here; boss combat remains unconfirmed. No ROM patch or fresh observed-coverage claim.

`generated/round-002-disassembly/game.asm` is fresh fidelity output;89717new classified bytes await full current-source commentary. `commentary/source`, before snapshot and all editorial records untouched. Next coordinator phase commentary; preserve its rebase workflow. Frozen map `maps/round-002-disassembly.json`, verification `analysis/round-002-disassembly-verification.json` ok=true/byte_identical=true. ROMSHA unchanged7d58660746eb6269153713fff1442430efc850cd32a9e353d62c7d8409591481.

Existing NES-001 workaround remains; no new toolkit issues found. [Toolkit report](../toolkit-issues.md). No toolkit edits/archive/publishing/deletion.
