# Round3 disassembly handoff — incomplete

ROM classification: 99.99% (262136 / 262160 bytes classified; 24 bytes unknown).

Fresh verified checkpoint: **pass-003b**, byte-identical true. Exact map SHA256
`d3932aa1bbf71f8200a2af01bac65ca0665f4f545b40d864d59dbb1805aba9e3`.
See [verification](pass-003b-verification.json), [summary/per-bank counts](round3/summary.json),
[independent structural review](round3/structural-review.md),
[fresh decode/format audit](round3/independent-review.json), [annotations](round3/applied.json).

R2-L02..08 verified:15344 Unknown->Code,22966 Unknown->Data; allStatic. Additional
$5836 Data->Code repair preserves16byte suffix. Effective bank5 overrides for
E006/E009/E00C retained. Existing Data/Text islands and Observed provenance preserved.
Partial lookup return/selector contracts remain explicit, no natural reachability claim.

Residual **24bytes/11intervals**: seven bank reset-template +4EAs; physical$982D..$9830,
$1E00D..$1E010,$1FFEF,$20000..$2000A. [Exact residuals](round3/unknown-complement.json),
[local ambiguity reasoning](redirect/r2-tactical-frontiers/residual-review.md).
No terminal exhaustion claim; a new source/commentary/redirect review may bring a
new structural constraint. Unknown meaning does not block already accepted genericData.

Commentary/source preserved intact SHA2569e2e25c7d996ddf316c0a6351ee5d1b7278f52437a55a6a96a5acd83b8e2fcb0;
old fullsourcegate does not cover newbytes. Next commentary must perform fullcurrent-
source review while preserving historical work. Saved broad gameplay/ending milestones
reviewed/reused with artificial vs natural provenance and prior exact budgets retained;
no new execution/RAM patches. NES-MAP-002 size-limit mismatch recorded; [NES-DOC-001](toolkit-issues.md)
remains. ROMunchanged; no toolkitbuild/edit, cleanup/delete/archive/publication/commit.
All phase handles closed. Only one disassembly phase executed.

| Physical 8KiB bank | Classified | Unknown |
|---:|---:|---:|
| 0 | 8191 | 1 |
| 1 | 8192 | 0 |
| 2 | 8191 | 1 |
| 3 | 8192 | 0 |
| 4 | 8188 | 4 |
| 5 | 8192 | 0 |
| 6 | 8191 | 1 |
| 7 | 8192 | 0 |
| 8 | 8191 | 1 |
| 9 | 8192 | 0 |
| 10 | 8191 | 1 |
| 11 | 8192 | 0 |
| 12 | 8191 | 1 |
| 13 | 8192 | 0 |
| 14 | 8189 | 3 |
| 15 | 8181 | 11 |

Checkpoint retry note: failed `maps/pass-003.json` is preserved. JSON whitespace
compaction reduced the map from18,225,559 to13,263,962bytes; independent parsed
objects are equal, so every evidence value/view/ID remains intact. This works
around the recorded16MiB file-load limit; future map.save/export may pretty-print
past that limit again. See [failure/compaction evidence](round3/checkpoint-failure.json).
Disk2.6GiBfree; scratch12KiB. Review78864, apply56963, failedcheckpoint57256 and
successfulcheckpoint46905 are all closed; no live server remains.
