# Round02 disassembly handoff

ROM classification: 99.94% (163750 / 163856 bytes classified; 106 bytes unknown).

Incomplete:106 physical bytes remain unknown in ten ranges. Accepted the full deduplicated95,839-byte redirect batch (14,834 Code/80,957 Data/48 Text); independent rationale analysis/round02-review/review.md, mechanical audit independent-audit.json and fresh primary decodes. No unsupported natural reachability or blanket legal-decode classification. L3..L8 verified; L9 deferred.

| Physical section | Code | Data/text/image | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG 00 | 3931 | 4261 | 0 |
| PRG 01 | 2971 | 5210 | 11 |
| PRG 02 | 0 | 8192 | 0 |
| PRG 03 | 0 | 8181 | 11 |
| PRG 04 | 6970 | 1222 | 0 |
| PRG 05 | 3648 | 4524 | 20 |
| PRG 06 | 0 | 8192 | 0 |
| PRG 07 | 0 | 8181 | 11 |
| PRG 08 | 0 | 8192 | 0 |
| PRG 09 | 0 | 8181 | 11 |
| PRG 10 | 0 | 8192 | 0 |
| PRG 11 | 0 | 8181 | 11 |
| PRG 12 | 0 | 8192 | 0 |
| PRG 13 | 0 | 8181 | 11 |
| PRG 14 | 7264 | 908 | 20 |
| PRG 15 | 0 | 8192 | 0 |
| CHR | 0 | 32768 | 0 |

Exact complement: analysis/round02-counts.json. Seven11-byte odd-bank reset-like signatures and three utility-like blocks9/5/15bytes remain. No unchanged absent-JSR scan or arbitrary-PC replay warranted; independent bootstrap bank/fetch or routine-role contract can reopen. This is phase completion, not terminal exhaustion.

Saved state analysis/state.json and map maps/current.json; verified fresh maps/round02-disassembly.json SHA93c0081fcdf9d4b49643f903e12e656fc94e073182258da7701ce4ee7ea82be1, generated/round02-disassembly, analysis/round02-disassembly-verification.json: map valid and byte_identical=true. ROM unchanged SHAd72122bb27376997493440c5a693eab566cb96e7150d5c059e2efaeadefeb1e1. All1111 annotated regions carry evidence; roles {'Code': 24784, 'Data': 104238, 'Image': 32768, 'Text': 1960}. Existing observations/scenarios unchanged; new annotations Static.

Next separate phase: full current-source commentary using generated/round02-disassembly. Preserve/carry prior commentary/source/game.asm (SHA010b8d9eb146c2089195468a490ede3cfc45bbd1e416d368246bf8e7d59d52eb) and historical commentary artifacts; do not overwrite their evidence. Prior readability gate covers only round01 source, so new full scope remains pending. Then redirect residual questions. No gameplay replays needed absent useful new discrimination.

Toolkit: one confirmed NES-001 documentation/API mismatch, zero suspected/missing features, no blockers or new findings; analysis/toolkit-issues.md. No toolkit changes, archive, cleanup/delete or publishing.
