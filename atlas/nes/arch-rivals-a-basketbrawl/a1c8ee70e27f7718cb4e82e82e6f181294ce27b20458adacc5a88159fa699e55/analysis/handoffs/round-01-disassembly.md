# Round 1 disassembly handoff

Status: productive pass complete, ROM incomplete. No terminal exhaustion claim.

ROM classification: 63.91% (83773 / 131088 bytes classified; 47315 bytes unknown).

Checkpoint: maps/pass-001.json, generated/pass-001, analysis/pass-001-verification.json. map.validate passed; rebuild byte_identical=true; configured binary and input unchanged. State analysis/state.json; export maps/current.json. Initial checkpoint preserved. Exact per-bank counts and unknown intervals: round-01/counts.json and round-01/unknown-ranges.json. Totals {'Code': 18680, 'Data': 1947, 'Image': 63146}; 720 evidence-bearing regions.

Scope/evidence: actual contextual ROM opcode roots, bank-aware static successors and direct JSR callee entry contracts; not decode-only speculative code. Static propagation stops at calls/mapper writes. Last two callee expansion batches yielded0 bytes. 3647 distinct ROM opcode starts; 59 RAM/other starts, no ROM origin inferred. Aggregates have no drops; capacity0 discards details intentionally.

Graphics/data: direct CHR-RAM upload consumers plus recognizable planar tiles and exact neighboring code boundaries support bank1/bank3 slices. Generic Data used only after pattern/reference/nearby-consumer inspection. Internal evidence preserved locally. See loaders, sheets, uniform-data.json and state annotations.

Gameplay: local game Arch Rivals, single match progression. Correct NES input is buttons array, **not** shared button/pressed schema (NES-001); early scripts are attract playback. Proper Start route reached selection/court naturally; movement/A/B sampled. Clock $059e-$05a1, quarter$05e6, divider$05d4 patches from clean court snapshot reached coaches, halftime, later coaches, FINAL SCORE; artificial provenance. Continuing halftime returned to quarter2 court03:44 and exercises. State17 post-final stays blank after bounded waits/A/Start/B; statistics/leaderboard unverified. Exact routes, patches/budgets and snapshots in scenarios.json/gameplay.md and cited scripts.

Lead outcomes: R01-PROGRESSION verified; R01-ENDSTATE blocked on a concrete initialization contract, not globally blocked; R01-DISPATCH and R01-RETURN ready for redirect. No original preexisting leads. Current source still needs full commentary pass. Redirect should inspect dispatch-table extents and bank-preserving return summaries, bank0/bank2 non-code consumers, and state17 score/entry dependencies; avoid repeating unchanged waits. Ready leads are useful recipes, not fixed range ceilings.

Toolkit: one confirmed doc/API-validation bug NES-001, workaround buttons arrays, no blocker; analysis/toolkit-issues.md. No toolkit changes. No archive/publish/cleanup. Disk8.6GiB free; round evidence215MiB and scratch16KiB.
