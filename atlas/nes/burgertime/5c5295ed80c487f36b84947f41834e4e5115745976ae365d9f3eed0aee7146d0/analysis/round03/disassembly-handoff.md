# Round 3 disassembly handoff — user-authorized residual fallback

ROM classification: 100.00% (24592 / 24592 bytes classified; 0 bytes unknown).
This is map assignment under the explicit user exception: **24,588 evidence-backed
bytes plus 4 Manual fallback bytes**. Evidence-backed structural coverage remains
99.98%; the fallback does not prove that the four bytes are data or code.

Fresh pre-edit complement was exactly [{offset:93,length:1},{offset:13445,length:3}],
physical [0x005D,0x005E) and [0x3485,0x3488), within the authorized <=10-byte limit.
Only these ranges were annotated generic Data, unknown meaning, Manual provenance,
using supported map.annotate. The isolated RTS versus non-code and possible audio
JMP wrapper versus unframed data alternatives remain unresolved. No new role,
reachability, observed execution, or dead-code claim is made. Prior evidence and
rejections remain intact, including [coordinator review](../coordinator-rejection-review.md)
and [redirect review](../redirect/round02-residual-review/rejection-review.md).
The explicit batch fallback authorization overrides ordinary evidence-only policy
only for these exact residuals; no redirect research or runtime replay was performed.

| Section | Code | Data/image | Assigned | Unknown | Manual fallback |
|---|---:|---:|---:|---:|---:|
| Header | 0 | 16 | 16 | 0 | 0 |
| PRG0 | 6755 | 1437 | 8192 | 0 | 1 |
| PRG1 | 4552 | 3640 | 8192 | 0 | 3 |
| CHR0 | 0 | 8192 | 8192 | 0 | 0 |

All prior regions unchanged; state saved, current portable map exported, both
validated and equal. Fresh uniquely named **pass-003-user-fallback** checkpoint
[verified byte-identical](../pass-003-user-fallback-verification.json).
Map SHA256: 5c7af2c7047d731ad68f76e21ca6703a5bd06c0bb06d2cd60d91c7988a7f4f56.
[Fallback record](fallback-record.json), [RPC log](rpc-results.json),
[counts](counts.json), and [pre-edit state](pre-fallback-state.json) preserve the
authorization, ambiguity, precise changes and source/ROM preservation hashes.

Next separate phase: full current-source commentary and strict editorial audit;
preserve existing readable source and prior baseline, then reconcile the two new
regions and retain this fallback disclosure. Commentary/source was not regenerated
or edited by this phase. Recovery completion awaits those editorial checks.
Existing final-handoff/audit files remain historical, not current recovery signoff.
No toolkit issues found; [report](../toolkit-issues.md). Configured release binary
used exclusively; no toolkit/ROM edits, archive, deletion, cleanup or publication.
Disk at handoff: 3.3 GiB free; scratch 80 KiB. All cited evidence remains local.
