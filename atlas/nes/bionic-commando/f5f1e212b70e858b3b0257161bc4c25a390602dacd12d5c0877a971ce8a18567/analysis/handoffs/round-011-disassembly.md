# Round011 disassembly handoff

## Round011 disassembly

ROM classification: 99.99% (262132 / 262160 bytes classified; 28 bytes unknown).

Accepted R010-BRK-RTI-SKIP: Static Code1 physical $3E010 (CPU $E000), Static Data1 BRK-discarded padding $3E011. Independently verified actual FFFE vector B2C0 -> accepted physical $3C0C2 RTI40 with fixed slots[unknown,unknown,30,31]. Hardware pushes E002/P and RTI restores PC/SP, architectural comparison flags and leaves A/X/Y unchanged; three residual stack writes and stacked B marker are explicit. Contract excludes NMI hijack/intervening interrupts, with I=1 sufficient for maskable IRQ exclusion; no natural caller, execution or timing-transparency claim. Decoder BRK length1 verified. Connected E002 filter/deactivation/scan and RTI already classified. Evidence: analysis/evidence/round-011/review.md, independent-check.json and fresh-decode.json.

Fresh internal-store complement:28 bytes/15 intervals; all32 per-bank counts and exact residuals in analysis/evidence/round-011/counts.json and unknown-ranges.json. Header16 classified. map.validate passed (validation.json); fresh checkpoint round-011-classified byte-identical (analysis/round-011-classified-verification.json). Map/state SHA256 094b219910335a9fe0bdcc2d53f8e4fc87edbf0ea7fd660c4c3529fba993b7d8. All existing regions, input ROM and prior commentary source/symbols preserved (preservation.json). Broad gameplay/scenarios unchanged: no changed RAM/entry/progression contract supports useful replay; prior artificial terminal signature is not natural cockpit victory/full timed credits. No new toolkit issues; existing [toolkit-issues.md](toolkit-issues.md) retained. Incomplete; next full current-source commentary then redirect. No toolkit/ROM/source edits, archive, deletion/cleanup, publication or subagents.

| Physical 8 KiB bank | Code | Other classified | Classified | Unknown |
|---|---:|---:|---:|---:|
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 0 | 8192 | 8192 | 0 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 1372 | 6820 | 8192 | 0 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 0 | 8192 | 8192 | 0 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 425 | 7767 | 8192 | 0 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 0 | 8192 | 8192 | 0 |
| 10 | 0 | 8192 | 8192 | 0 |
| 11 | 0 | 8192 | 8192 | 0 |
| 12 | 0 | 8192 | 8192 | 0 |
| 13 | 0 | 8192 | 8192 | 0 |
| 14 | 0 | 8192 | 8192 | 0 |
| 15 | 0 | 8192 | 8192 | 0 |
| 16 | 0 | 8192 | 8192 | 0 |
| 17 | 0 | 8192 | 8192 | 0 |
| 18 | 0 | 8192 | 8192 | 0 |
| 19 | 0 | 8192 | 8192 | 0 |
| 20 | 0 | 8192 | 8192 | 0 |
| 21 | 1190 | 7002 | 8192 | 0 |
| 22 | 0 | 8192 | 8192 | 0 |
| 23 | 598 | 7585 | 8183 | 9 |
| 24 | 2387 | 5805 | 8192 | 0 |
| 25 | 4 | 8188 | 8192 | 0 |
| 26 | 6179 | 2008 | 8187 | 5 |
| 27 | 6591 | 1601 | 8192 | 0 |
| 28 | 6689 | 1500 | 8189 | 3 |
| 29 | 7111 | 1077 | 8188 | 4 |
| 30 | 7652 | 537 | 8189 | 3 |
| 31 | 4717 | 3471 | 8188 | 4 |

Prior rejection of0035 superseded only for this exact two-byte structural claim. Other15 residual intervals retain existing evidence and unresolved code/data ambiguity; no blind generic Data acceptance. Preserve earlier histories. Disk3.8GiB free; scratch180KiB at pass start; bounded small evidence only.
