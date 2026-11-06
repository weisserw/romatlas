# Round 1 commentary handoff

Current-source readability complete; ROM classification incomplete.

Scope: all 6,483 code bytes / 36,148 data bytes and honest boundary notes on 88,457 unknown bytes. All 514 scaffold entries reviewed; 213 functional-unit records, 222 used constants. No classification/state/map change. commentary/source/game.asm is the maintained readable source; constants.inc supplies shared RAM/hardware and CPU table aliases. Frozen before/ and baseline/state preserved. reading-after.asm refreshed after the last edits.

Fresh final verification: commentary/verification.json, byte_identical=true. ROM SHA256 f7e9e6981d3446128fe59efb141a483e2c83d11100a246df66701c6d69b2c1e3. Final game.asm hash c88c5f27be166e18c9de57fb8d6d54d34252674ff8d1c1378ce4922fd68409b4; constants.inc hash 2749bd852f02b084ea9a68e4125de874be6dbc50ecb5433b4efdf661d56d91fb. Whole-file rebuild performed using configured release binary via supported commentary script. Final editorial/archive audit is pending and was not run.

Examples: start_task/yield_current_task/schedule_next_task explain saved task stacks and runnable $80 state; queue_rectangle_transfer and NMI explain address/count/data ring records and modulo-128 indices; emit_sprite_pieces explains signed coordinate rejection/OAM capacity; arbitrate_audio_channels explains six software voices/four APU groups; stage upload scripts explain count/source/destination records and stack-unwinding exit; primary_object_task explains ordinary entry and exit animation without claiming playable controls.

Redirect: R01-PROGRESSION remains concrete. Primary task $F200..$F273 (physical $01F210..$01F283) branches on $94, initializes object zero on the zero path, animates to screen X $60, then runs the $F7-to-zero exit counter and terminates task $06 with mode $FF. Understand the alternate $F273 path, task $04/$06 entry contracts and scheduler state transitions before another complete scene/player transition experiment. $E73C is scene setup alone. Reuse existing snapshot/scenarios; do not repeat unchanged long input runs.

Other unresolved meanings/structural gaps: commentary/uncertainties.md. Sprite stream optional/skip continuations, audio command handlers, descriptor fields/domains, remaining stage/script/table backing and connected static successors can support redirect. CPU table aliases name consumer mechanics but do not classify backing bytes or assert complete extents. Graphics backing segmentation and uniform-data purpose remain unresolved. No skipped current-source readability ranges; unknown bytes remain structurally unknown.

Evidence and progress: commentary/progress.md, commentary/review.json, analysis/progress.md and prior disassembly handoff. No toolkit issues found (analysis/toolkit-issues.md). No toolkit modifications, archive, publication or cleanup. All ROM-derived artifacts remain local. Disk 8.5GiB free; scratch44KiB.
