# Round 2 disassembly completed; ROM incomplete

round=2 phase=disassembly status=pass-complete; next=commentary.
ROM classification: 100.00% (131087 / 131088 bytes classified; 1 bytes unknown).
Rounded percentage does not establish completion. Code26849, data104238 including
16-byte header. Gain112032 (13318code,98714data). 906/906 classified regions have
evidence. 5964 proposed instruction records independently decoded;
all accepted structural proposals Static, no new execution/reachability claim.

| Physical 8 KiB bank | Code | Data | Unknown |
|---|---:|---:|---:|
| 0 | 0 | 8192 | 0 |
| 1 | 0 | 8192 | 0 |
| 2 | 0 | 8192 | 0 |
| 3 | 0 | 8192 | 0 |
| 4 | 0 | 8192 | 0 |
| 5 | 0 | 8192 | 0 |
| 6 | 0 | 8192 | 0 |
| 7 | 0 | 8192 | 0 |
| 8 | 0 | 8192 | 0 |
| 9 | 0 | 8192 | 0 |
| 10 | 0 | 8192 | 0 |
| 11 | 0 | 8192 | 0 |
| 12 | 7646 | 546 | 0 |
| 13 | 6833 | 1359 | 0 |
| 14 | 5409 | 2782 | 1 |
| 15 | 6961 | 1231 | 0 |

Accepted R1-Q01/Q02/L04 as the combined connected structural batch after independent
consumer, exact-boundary, bank/stack and data-pattern checks. No proposal slices
rejected; unproposed lone1CAF1/CAE1 remains unknown. Existing accepted regions all
preserved exactly. [Review](../round-02/review.md), [primitive audits](../round-02/review.json),
[counts](../round-02/counts.json), [lead history](../leads.json). Historical failed
methods remain preserved; no unchanged gameplay replays. Prior scenarios visited
stages0..7, inventory and ending with artificial/natural provenance kept separate.

Remaining1byte60 at physical1CAF1 CPUCAE1 is isolated RTS between NMI tailJMP and
IRQentryCAE2. No incoming edge/discriminating routine contract found. Keep unknown;
no dead/unused/code/data assumption. This local uncertainty does not reject coherent
neighboring engine APIs. No productive runtime/connected unknown target remains in
this pass; commentary then redirect should review this actual residual frontier.
No terminal exhaustion assertion by this disassembly pass.

Fresh map.validate ok=true; state saved analysis/state.json; export maps/current.json.
[Checkpoint](../../maps/round-02-disassembly.json) byte-equal current export;
[verification](../round-02-disassembly-verification.json) ok=true,byte_identical=true;
[source](../../generated/round-02-disassembly/game.asm). ROM SHA256
`8124ff905ef374e1e1eed36994961bc04dc005bb0081a1ba620b2ec95ca7650b` unchanged.
Map SHA256 `171e9c2d3ffa53f33df490ade67191a3b7d8342fca8f67167e635f4ad3b7948e`. Generated source is local reconstruction,
not a sharing deliverable. Existing commentary/source, reading-after and historical
checkpoints unchanged; new source now requires full commentary review.

Next commentary: carry forward all223 functional units and symbols from previous
source into expanded source, review all newly classified ranges and current source.
Correct decoder token prose:01..3Frepeat,40..7Fzero,81..FFliteral,00/80skip.
Honor physical1CEAF..1CEB1 exclusive (CPUCE9F/CEA0) inline pointer05A7 after
JSRD360->D611; CEA1 resumes code. No structural reclassification needed for that word.
Detailed roles/reachability limits and exact contracts are in review and redirect
r01-connected-20261127 data-contracts.md/residual-contracts.md. Preserve generic-data
honesty where identities or original subtable layouts remain unresolved.

No toolkit issues found; [persistent report](../toolkit-issues.md), zero blockers.
No ROM/toolkit changes, cleanup, deletion, archive, publication or commit. No live
server remains. Disk~3.4GiB before checkpoint; bounded evidence retained locally.
