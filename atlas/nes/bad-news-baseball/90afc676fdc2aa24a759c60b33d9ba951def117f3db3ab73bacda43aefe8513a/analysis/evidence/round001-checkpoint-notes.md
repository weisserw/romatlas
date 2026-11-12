# Round 1 disassembly checkpoint

Status: incomplete. Exactly one substantive disassembly pass completed. No persistent server remains; replay scripts start the configured release binary and import durable local snapshots.

ROM classification: 61.45% (161103 / 262160 bytes classified; 101057 bytes unknown).

Code 16314; Data 13717; Image 131072. 518 regions all carry evidence; 173 unknown intervals. Observed instruction bytes 12715, unique starts 6199; natural 5144 starts, artificial 5449 starts (overlapping sets). Static code3599 bytes is not execution coverage.

Checkpoint: maps/pass-001.json; generated/pass-001; analysis/pass-001-verification.json confirms byte-identical ca65/ld65 rebuild. map/state exported and validated. ROM SHA256 90afc676fdc2aa24a759c60b33d9ba951def117f3db3ab73bacda43aefe8513a.

| Physical bank | Code | Data | Image | Classified | Unknown |
|---|---:|---:|---:|---:|---:|
| header | 0 | 16 | 0 | 16 | 0 |
| PRG00 | 503 | 0 | 0 | 503 | 7689 |
| PRG01 | 0 | 1508 | 0 | 1508 | 6684 |
| PRG02 | 0 | 0 | 0 | 0 | 8192 |
| PRG03 | 0 | 3023 | 0 | 3023 | 5169 |
| PRG04 | 1940 | 0 | 0 | 1940 | 6252 |
| PRG05 | 0 | 1167 | 0 | 1167 | 7025 |
| PRG06 | 562 | 408 | 0 | 970 | 7222 |
| PRG07 | 0 | 1050 | 0 | 1050 | 7142 |
| PRG08 | 1102 | 0 | 0 | 1102 | 7090 |
| PRG09 | 0 | 252 | 0 | 252 | 7940 |
| PRG10 | 4574 | 0 | 0 | 4574 | 3618 |
| PRG11 | 1367 | 5223 | 0 | 6590 | 1602 |
| PRG12 | 3287 | 18 | 0 | 3305 | 4887 |
| PRG13 | 11 | 1046 | 0 | 1057 | 7135 |
| PRG14 | 2962 | 0 | 0 | 2962 | 5230 |
| PRG15 | 6 | 6 | 0 | 12 | 8180 |
| CHR00 | 0 | 0 | 8192 | 8192 | 0 |
| CHR01 | 0 | 0 | 8192 | 8192 | 0 |
| CHR02 | 0 | 0 | 8192 | 8192 | 0 |
| CHR03 | 0 | 0 | 8192 | 8192 | 0 |
| CHR04 | 0 | 0 | 8192 | 8192 | 0 |
| CHR05 | 0 | 0 | 8192 | 8192 | 0 |
| CHR06 | 0 | 0 | 8192 | 8192 | 0 |
| CHR07 | 0 | 0 | 8192 | 8192 | 0 |
| CHR08 | 0 | 0 | 8192 | 8192 | 0 |
| CHR09 | 0 | 0 | 8192 | 8192 | 0 |
| CHR10 | 0 | 0 | 8192 | 8192 | 0 |
| CHR11 | 0 | 0 | 8192 | 8192 | 0 |
| CHR12 | 0 | 0 | 8192 | 8192 | 0 |
| CHR13 | 0 | 0 | 8192 | 8192 | 0 |
| CHR14 | 0 | 0 | 8192 | 8192 | 0 |
| CHR15 | 0 | 0 | 8192 | 8192 | 0 |

Evidence: round001-counts.json and round001-unknown.json record exact physical complement. Startup/menu/match/mechanics/password runtime fetches retain CPU aliases and effective PRG-slot contexts in *-contexts.json/*-decoded.json. Generic data rationale and exact boundary neighbors: uniform-data.json. Specific tables: PasswordAlphabetGlyphs 0x16ADF..0x16B1F, MatchPhaseDispatchWords 0x18033..0x18045. Static continuations: static-full-new.json/static-final-new.json with bounded discovery reports. An unobserved BRK candidate at0x159DD remains unknown pending consumer-precondition review. No raw-export fallback bytes count as classified.

Progression: natural introduction/title/team/lineup/pitcher selection and batting/pitching/pause/batter-substitution visited. Artificial supported half-inning entry reaches results and password; accepted final-game password reaches Chicago vs Toronto. Ninth-inning top display visited. Final-game win shows GAME11 and subsequent engine7 victory parade after1200 untraced empty frames+30 traced frames (ending-wait.ppm.png). Ending visited; credits completion not claimed. Ninth count/outs patch did not establish an actual side transition; retain as attempted. Alternate all-star/spectator/girls modes and sustained fielding remain deferred, not declared unreachable. See scenarios.json and gameplay.md.

No initial ready leads existed. Verified phase leads D001 startup, D002 natural match, D003 half-inning results, D004 password, D005 ending, D006 ninth; exact recipes/outcomes in leads.json. New redirect avenues: engine7 script/ending consumers, code/data in untouched bank2 and paired data banks, all-star/girls state routes, nonuniform pointer/roster/audio structures adjacent to existing code. Further work needs consumer/bank-bound proof rather than longer repeated initial play. This is an easy-gain handoff, not terminal exhaustion or a completion claim.

Toolkit: no toolkit issues found; see analysis/toolkit-issues.md. Initial mailbox process termination and rejected socket creation were local harness/infrastructure events, not reproduced toolkit defects. Bounded direct subprocess/snapshot replay worked. Trace detail capacity0 intentionally drops detail while aggregate_dropped=0 in retained summaries. Disk at handoff ~6.7GiB free; scratch112KiB before final records, evidence~119MiB; cited evidence retained.
