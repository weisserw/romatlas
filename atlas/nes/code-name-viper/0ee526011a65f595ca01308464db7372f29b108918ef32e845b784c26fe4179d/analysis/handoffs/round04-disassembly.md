# Round04 disassembly handoff

ROM classification: 99.98% (262112 / 262160 bytes classified; 48 bytes unknown).

One pass complete; whole ROM incomplete; overall goal remains active. L015/L016
verified. Accepted 20570 unique bytes = 7431 Static Code + 13139 Static Data in
186 regions (75 Code runs,111 Data slices). All 1353 previous annotations preserved
exactly. Primary independent role review: analysis/pass-004/review.md. Fresh 3292
contextual instruction views: fresh-code-review.json; exact data checks:
data-review.json; annotations: accepted.json, all within analysis/pass-004.

Internal/current/maps/pass-004.json SHA256:
db5ce160d67a8705fb7da671627035cdd170680a7af6bccb90756afcdb8c010e.
Fresh generated/pass-004 byte-identical verification:
analysis/pass-004-verification.json. ROM unchanged:
0ee526011a65f595ca01308464db7372f29b108918ef32e845b784c26fe4179d.
Code32786, Data97666, Text588, Image131072. All262112 classified bytes have evidence.
Exact per-bank counts: analysis/pass-004/classification-report.json. Exact residual
48 bytes in21 fragments with competing interpretations: analysis/pass-004/unknown.json.

Actor roles combine concrete fields, timer predicates, allocation carry, signed
motion, script records, helper calls and balanced returns under valid X slot and
banks8/9. Terrain16-word dispatcher and checkpoint50..53 graph accepted. C061/FA5B
maps10/11 for B70B, then explicitly restores2/3; no generic mapper permission.
Fixed hardware/decimal/velocity roles, finite PPU loops and86AE counted stack loop
accepted. F5E1 writes a single byte through a RAM destination, not a whole-RAM fill.
Generic Data retains inner-schema uncertainty. 12CE9..12CF9 is Data; slot search
12CF9 onward is Code. Source role proof is beyond pointer values/legal decoding.

IMPORTANT next commentary correction: physical1FE78/CPUFE68 is04, NOT EA in prior
prose. Exact metadata060A040406060604040404 immediately precedes FE69 A000. Narrow
generic Data; no stage10 domain claim. Preserve history and append the correction.

Residuals mostly isolated RTS and tiny load/tailcall stubs, plus F0ED, A42F,0009,
A907, and12 coordinate-copy bytes1E59A..1E5A6 falling into tile Data without an exit.
review.md/unknown.json preserve primary counterevidence and competing roles. This
is no blanket missing-caller rejection or terminal exhaustion assessment. Nine
graph stops are already Code and have no unknown immediately following their
current regions; connected-stops-review.json. No obvious connected easy gain was
left outside the accepted actor/terrain/helper/data batch. New discrimination may
help redirect. Do not repeat unchanged late/boss/ending routes already completed.
No new runtime observations. Observed Code10694 and Static Code22092 are provenance
counts, separate from complete execution coverage or natural reachability.

Next: full current-source commentary migration/review from generated/pass-004;
prior commentary untouched and186 new regions require readability coverage.
Final editorial audit reserved. Afterwards redirect the exact residuals under
evidence-policy.md; no automatic terminal or100% assertion. No toolkit issues found,
no blocker; analysis/toolkit-issues.md. No ROM/toolkit/source edit, baseline/replay,
cleanup/deletion/archive/download/publication. Disk3.1GiB free; pass artifacts2.9MiB.
