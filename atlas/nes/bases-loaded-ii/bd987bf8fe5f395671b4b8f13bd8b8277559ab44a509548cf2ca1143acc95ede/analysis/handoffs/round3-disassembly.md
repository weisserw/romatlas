# Handoff: round 3, phase 1 (disassembly)

ROM classification: 100.00% (393232 / 393232 bytes classified; 0 bytes unknown).

All bytes of the ROM are now classified with evidence-backed structural roles (0 unknown).
This does not by itself complete the project: commentary on the new map and the final
editorial audit are still outstanding and were not done in this pass.

## Checkpoint
- Input: maps/round2-disassembly.json (state restored from analysis/state.json).
- Output: maps/round3-disassembly.json = maps/current.json (sha256 bfba143a...),
  generated/round3-disassembly, analysis/round3-disassembly-verification.json
  (ok, byte_identical true). map.validate ok. input/game.nes unchanged.
- Apply script: analysis/round3/tools/apply_r3.py. Changes: analysis/round3/changed-ranges.json.

## Per-bank counts (changed banks; all banks now 0 unknown)
| bank | code | data | padding | unknown |
|---|---|---|---|---|
| prg2 | 9277 (+5) | 6938 | 169 | 0 |
| prg3 | 397 | 12856 (split only) | 3131 | 0 |
| prg6 | 3099 (+1) | 346 | 12939 | 0 |
| prg11 | 3871 (+2) | 1040 | 11473 | 0 |
| prg15 | 14026 (+2) | 2086 | 272 | 0 |
Full table: analysis/progress.md (round 3 section; stats via analysis/round2/tools/stats.py).

## Lead outcomes
- **L-016: verified, 7/7 ranges accepted** as Code, Static, confidence 0.5. Reachability is
  recorded as "no reference; unreachable as far as known". None is labelled Observed.
  - Each range follows an unconditional transfer and is bounded by established code.
  - Each byte value is an RTS or return idiom, not this ROM's fill.
  - No reader or pointer reaches any range. The round-3 word re-search agrees with r2.
  - The round-2 rejection premise ruled out reachability only, not the code role.
  - Per-range reasons are in analysis/investigation.md.
  - Note: bank 4 $D174-$D17B and bank 6 $C845-$C849 were already classified in round 2 and are
    not part of the 10 bytes.
- **L-017: verified (relabel only).** Regions 2081 ForceHandlers, 2082 TagUpHandlers and 1111
  CpuSideModeJumpTable keep the same bounds. The L-015 fragments 2199 and 2200 are now recorded
  as table-reached: provenance stays Static and confidence goes from 0.6 to 0.8. Regions 1928,
  1930 and 1931 are recorded as CpuSideModeJumpTable entries.
- **L-019: verified.** Region 517 was split into 2205 ScreenLoaderStepTable (phys 49240, 6 bytes,
  3 words) and 2206 generic data (phys 49246, 154 bytes).
- Still open (semantics only, no classification impact): L-004, L-005, L-009, L-011
  (commentary: biorhythm naming), L-018.

## Changed ranges for commentary
- New code regions (each is a lone dead return; comment it as unreachable as far as known):
  - b2 $D5FE (phys 38414)
  - b2 $DAC9-$DACC (39641, PLA/TAY/PLA/RTS)
  - b6 $CA2E (100926)
  - b11 $C286 (180886)
  - b11 $C9C4 (182740)
  - b15 $C8FE (248078)
  - b15 $D9F4 (252420)
- Relabelled tables: b15 $F92A ForceHandlers, b15 $F947 TagUpHandlers, b11 $D08E
  CpuSideModeJumpTable. Commentary already uses these names.
- New table: b3 $C048 ScreenLoaderStepTable. The rest of b3 $C04E-$C0E7 stays generic data.
- Evidence or confidence updates: regions 2199, 2200, 1928, 1930 and 1931.
- Commentary must be regenerated or brought onto maps/round3-disassembly.json. After that, the
  final editorial audit is still required.

## Toolkit
No toolkit issues found this pass (TK-001 to TK-005 unchanged; see analysis/toolkit-issues.md).
