# Terminal handoff — justified incomplete ROM analysis

The coordinator workflow has reached its documented terminal incomplete handoff.
ROM classification is **not complete**.

ROM classification: 99.98% (65541 / 65552 bytes classified; 11 bytes unknown).

There are 14,211 Code and 51,330 non-code bytes. Exact unknown physical intervals
(end exclusive, including the header) are $007C..$0080 (4), $0238..$023C (4),
$049E..$049F (1), $1219..$121A (1), $3B8A..$3B8B (1). PRG bank 0 has ten unknown
bytes and bank 1 has one; all other storage is classified. These are localized
code-versus-data ambiguities, not unknown meanings mislabeled as unknown bytes.

Authoritative structural checkpoint: **pass-003-reviewed**, map/state SHA256
90c4126c01fcd06e3dc687064fda1288f02ec1e022ec37049170ffe359a75c16.
[Per-bank counts](disassembly/pass-003/counts.json),
[structural verification](pass-003-reviewed-verification.json),
[portable map](../maps/current.json). Only the supported ROM-free portable map is
a sharing deliverable; internal state, source, assets, captures and traces stay local.

Three rounds ran separate sequential disassembly, commentary and redirect workers.
Round 1 established and expanded evidence; round 2 independently accepted the large
connected batch; round 3 accepted a corrected four-byte conditional initializer.
Their handoffs and historical commentary remain preserved. No competing phase
writers or background preparation remain. Current leads contain no ready lead.

Terminal reasoning: [round-3 rejection review](redirect/round-3/findings.md),
[primary excerpts](redirect/round-3/primary-source.txt),
[coordinator inspection](coordinator-rejection-review.md). The coordinator reviewed
the decisive bytes, boundary/consumer/stack constraints, competing interpretations,
split claims and negative-evidence scope. No useful corrected inference or bounded
discriminating method remains on these premises. Reopen with new evidence or a
justified changed inference; do not repeat unchanged gameplay or arbitrary-PC tests.
This is justified terminal incompleteness, not a blocker or whole-ROM success.

Readable source is complete across all currently classified storage:
[game.asm](../commentary/source/game.asm),
[reading copy](../commentary/reading-after.asm),
[current-source handoff](commentary/round-3/handoff.md).
Final acceptance reviewed all 167 scaffold entries and 417 units (360 routine,
57 data), fixed inventory path prefixes and passed a fresh byte-identical rebuild
with reading-copy refresh. [Editorial audit](../commentary/editorial-audit.md),
[audit result](../commentary/audit.json),
[acceptance handoff](final-acceptance/handoff.md).
Source SHA256 is 079bf436b43182359aed91c3d1f072da34b0f7a7971bc6358c037d01cf525d61.
Historical vector-address and B9 operand-width corrections are explicitly recorded;
the original records remain intact.

Broad exploration includes early/middle/last-course driving, final opponent/contact,
defeat-state continuation, all 21 ending text records and return to course 0,
and airborne jump, using documented artificial shortcuts and saved snapshots.
[Scenarios](scenarios.json), [runtime results](redirect/round-1/runtime-results.md).
Natural defeat, landing, unedited ordinary-timing full ending and directed hazard
permutations remain unclaimed; their deferred status and absence of a concrete
residual structural benefit were reviewed. No full compatibility claim follows.

The toolkit's scripts/archive.py document-selection contract was inspected read-only.
[archive-options.json](../archive-options.json) explicitly preserves additional final
Markdown results, handoffs, corrections and historical commentary context. There
are no disposable scratch Markdown documents among the unrecognized files, so
omitted_documents is empty. Every unrecognized analysis/commentary Markdown path
is selected. Standard artifacts and instruction/source exclusions follow the actual
contract; final conclusions were not omitted to bypass a gate.
[Completion audit](completion-audit.json) records the final current-state checks.

Input SHA256 remains
a6268aaed181bc2e58de599c89f3a70263feb069ff6c6b5fed926bea8f386c29.
Configured release binary was used; map/store validation and rebuild verification
passed. No toolkit issues found; [report](toolkit-issues.md).
No toolkit modification, archive execution, cleanup, workspace deletion, publication
or commercial ROM/assets download was performed. Artifacts are prepared locally
for a separately authorized archive operation.
