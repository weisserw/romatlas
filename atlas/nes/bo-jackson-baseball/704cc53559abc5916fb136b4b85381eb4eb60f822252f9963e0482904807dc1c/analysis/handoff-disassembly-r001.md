# Disassembly r001 handoff

**One pass finished; ROM incomplete.**

ROM classification: 81.65% (321066 / 393232 bytes classified; 72166 bytes unknown).

Final checkpoint: [portable map](../maps/disassembly-r001.json), [current source](../generated/disassembly-r001/game.asm), [verification](disassembly-r001-verification.json). Map validation and whole-image byte-identical rebuild passed. Current store/export: `analysis/state.json`, `maps/current.json`. ROM unchanged (`704cc53559abc5916fb136b4b85381eb4eb60f822252f9963e0482904807dc1c`). Map hash `ba7762de5c6a4cad48861b06d427da49bfddccbe788b19b0f9d72f32dd588fad`.

| PRG 8 KiB bank | Code | Data | Classified | Unknown |
|---:|---:|---:|---:|---:|
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 0 | 8192 | 8192 | 0 |
| 2 | 1704 | 5319 | 7023 | 1169 |
| 3 | 0 | 213 | 213 | 7979 |
| 4 | 0 | 3039 | 3039 | 5153 |
| 5 | 0 | 7983 | 7983 | 209 |
| 6 | 0 | 1004 | 1004 | 7188 |
| 7 | 0 | 1208 | 1208 | 6984 |
| 8 | 0 | 749 | 749 | 7443 |
| 9 | 468 | 953 | 1421 | 6771 |
| 10 | 3730 | 338 | 4068 | 4124 |
| 11 | 1321 | 73 | 1394 | 6798 |
| 12 | 2672 | 225 | 2897 | 5295 |
| 13 | 3092 | 388 | 3480 | 4712 |
| 14 | 3397 | 72 | 3469 | 4723 |
| 15 | 4485 | 89 | 4574 | 3618 |

Header16 Data; CHR262144 Image (32 reporting chunks ×8192, each fully classified). Overall20869 Code/38053 Data/262144 Image.260 remaining intervals: [exact complement](r001/unknown-final.json).

1038/1038 regions have structural evidence and every Code region explicit CPU/bank views;42 labels. Execution is separate:7567 physical opcode starts /24618 contextual roots,6312 natural and5300 artificial starts with overlap. Decoded instruction-byte union16745. [Metrics](r001/metrics-final.json).

Useful structural findings:
- Eight packed DAC samples, file0x10–0x4EA8,20,121 bytes. Direct callers anchor every start; stored record size count+3. Consumer initializes playback at header+2 and leaves one stored tail byte; corrected exact-consumption explanation retained in [investigation](investigation.md), not silently overwritten. End0x4EA9 is observed code entry CPU$CE99 in bank2.
-26 team rosters /10,882 bytes, encoded24/25-player counts,21-byte team headers and16-byte player records; Baltimore names match rendered lineup. [Records](r001/rosters.json).
- Intentional alternate LDY entries inside BIT operands at file0x1C546,0x4F50/53 and0x4FAB/AE. Outer linear encoding kept with comments. Preserve these while editing source.

Gameplay and replay: [scenarios.json](scenarios.json) gives complete button/budget routes, artifact links, exact state-edit protocol and milestone checklist. Natural startup, teams, pitcher roster, anthem, pitching, fielding, settings Music/Icons toggles, Play Offs entry. Artificial half-inning/batting, verified ninth-inning third out, handshake and credits. Clean base `r001/after-anthem-snapshot.json`; successful late route `r001/ninth-inning-snapshot.json`; credits `r001/final-out-attempt-snapshot.json`. No full natural game/championship claim. Final-score billboard not separately captured after bounded epilogue checks; later credits added0 code. Full series/password behavior deferred.

Recovery: use a new configured serve process, map.load, emulator.start, snapshot.import, then reassert buttons. Snapshots are same-build/local and contain ROM state. Early helper interruption lost only live emulator state; durable roots were retained. Subsequent experiments use direct JSON-RPC subprocesses and durable exports. No live server is required to resume. `r001/rpc.jsonl` and `r001/rpc-direct.jsonl` preserve literal requests/results; bulk snapshots/frames remain in separate files.

Next: full current-source commentary, including all20,869 classified code bytes, not only the final batch. Useful names exist for reset/interrupts, packed PCM player/mapper helper, sample records and rosters; most routine/variable/branch labels remain to be supplied. Keep supported literal addresses, effective-bank aliases, operand widths and BIT-skip comments. Source rebuild fidelity is verified; full readability/editorial completion is pending.

Redirect has concrete nonterminal avenues in [leads.json](leads.json): remaining bank3/menu/PPU data, bank6–9 sprite-like records and their bank14 consumer, bank10 word tables/known mixed code, bank11 music command structures, and playoff series state. These need consumer/extent contracts; clear non-code slices may be generic data under evidence-policy.md without solving all semantics. No terminal exhaustion claim or unchanged-rejection veto applies.

[Toolkit report](toolkit-issues.md): **no toolkit issues found**; no blocker. Configured binary only; no toolkit edits, publication, archive or deletion.
