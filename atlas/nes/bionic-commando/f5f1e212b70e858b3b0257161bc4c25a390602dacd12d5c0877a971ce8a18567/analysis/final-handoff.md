# Terminal handoff — orchestration complete, ROM incomplete

The eleven sequential disassembly → commentary → redirect rounds reached a justified
**terminal incomplete** investigation handoff. Final targeted editorial acceptance
passed. This completes the requested coordinator workflow; it does not complete
whole-ROM classification, natural gameplay or hardware compatibility.

ROM classification: 99.99% (262132 / 262160 bytes classified; 28 bytes unknown).

| Structural role | Bytes |
|---|---:|
| Code | 44915 |
| Data (including header/padding) | 117889 |
| Image | 99328 |
| Unknown | 28 |

Final checkpoint: `round-011-classified`. Current supported local portable mapping:
`maps/current.json` (`romatlas-nes-map`). Map/state SHA256:
`094b219910335a9fe0bdcc2d53f8e4fc87edbf0ea7fd660c4c3529fba993b7d8`.
Input ROM SHA256 unchanged:
`f5f1e212b70e858b3b0257161bc4c25a390602dacd12d5c0877a971ce8a18567`.
All source, ROM, state, assets, traces and snapshots remain local. No internal
state is being published or presented as a sharing deliverable.

Current source is `commentary/source/game.asm` and `symbols.inc`; refreshed reading
copy `commentary/reading-after.asm`. All 2107 spans and 2018 units are reviewed,
with no deferred readability ranges. Final source SHA256:
`af3fb0a1382c760c80bf0b1b50881e3dd7b5937a7934074a7e7094d5f2785502`.
Final symbols SHA256:
`66995f34d01f128a77fc190373cd1352cac45b2be6c3f266021fd99cc0cd35e4`.
Final editorial acceptance corrected PPU-ring symbols, inherited source-location
paths and stale current scenario/uncertainty summaries while preserving observations.

Fresh `commentary/audit.json` and `verification.json` report ok/byte_identical=true;
internal store and portable map validations passed. Evidence:
[editorial audit](../commentary/editorial-audit.md),
[final acceptance](handoffs/final-editorial-acceptance.md),
`analysis/final-acceptance.json` and companion validation records. These gates
establish fidelity and current readability, separately from unknown classification.

## Terminal reasoning and exact residuals

[Coordinator rejection review](coordinator-rejection-review.md) accepts the
[candidate-specific terminal review](redirect/round-011-a/rejection-review.md)
after checking actual bytes, current neighbors, context/copy artifacts and corrected
BRK/RTI evidence. The earlier round010 stop was rejected and reopened: actual IRQ
vector C0B2 reaches RTI, justifying the final Code1/padding Data1 gain. That trap is
no longer part of the rejection rationale.

The remaining isolated returns/jumps/TAX and partial copied bytes have genuinely
competing code/data interpretations. This is not a missing-semantic-label, caller,
whole-routine-return or natural-execution requirement. No credible revised method
or ready structural proposal remains at this checkpoint. Future independent
consumer/copy/boundary evidence can reopen a localized claim; unchanged forced
entry or previously exhausted searches should not be repeated automatically.

| Physical file offset (header included) | Unknown bytes |
|---|---:|
| $2E810 | 9 |
| $34231 | 3 |
| $34DFC | 1 |
| $359D3 | 1 |
| $38AF8 | 1 |
| $39319 | 1 |
| $39EDE | 1 |
| $3B612 | 4 |
| $3D62A | 1 |
| $3D6AF | 1 |
| $3DFD3 | 1 |
| $3E9B0 | 1 |
| $3E9EF | 1 |
| $3F448 | 1 |
| $3F9E5 | 1 |

Exact current ledger, bank/CPU contexts, neighbors and dispositions:
`analysis/redirect/round-011-a/residual-ledger.json` and
`analysis/evidence/round-011/unknown-ranges.json`.

## Gameplay, lead history and resumption

Natural startup/title/communications and representative artificial area1/area12,
neutral13, special96/98/101 mechanics were exercised. Recorded ending shortcuts
reached explosion, rescue and the terminal signature `2010.8.2 / Joe`. These are
artificial scenario observations, not natural final victory or full timed credits.
The room101 boss premise was corrected to the pre-final kind70/health39 encounter.

`R001-FINAL-SHOT` remains a blocked, resumable gameplay hypothesis with its corrected
identity, saved evidence and explicit new-contract prerequisite in `analysis/leads.json`.
It is not reported successful. No productive classification phase is left paused or
blocked. The historical broad resource blocker is explicitly superseded by verified
batches and the current exact complement. Preserve snapshots/routes and all attempt
history. Resume analysis only with specific new evidence or an explicit reassessment;
load the saved state with the configured binary, then use the corresponding compact
handoff/frontier rather than reinitializing or repeating old experiments.

[Phase index and progress](progress.md) records all eleven input/output checkpoints,
verification, consumed/produced lead IDs and outcomes. [Final redirect handoff](handoffs/round-011-redirect.md)
and [current commentary handoff](handoffs/round-011-commentary.md) remain durable.

Three nonblocking toolkit findings remain: discovery parameter documentation,
observed-provenance reporting and generated Game Boy wording. Workarounds and
primary evidence stay in `analysis/toolkit-issues.md`; no toolkit changes were made.
Configured release binary SHA256 remains
`8ff56b0b1cf8d2a3aca5430c70d34280ae3db36fc6774919062db4ea63843f82`.

## Archive preparation only

`archive-options.json` explicitly selects additional final Markdown and classifies
omitted generated scaffold copies. [Selection review](archive-document-selection.md)
accounts for every otherwise-unrecognized Markdown file under analysis/commentary
using the inspected toolkit archive.py contract. Final conclusions, all phase
handoffs and research/rejection history are selected; final results are not omitted
to bypass archive checks. Default final map/review/audit artifacts stay selected by
the toolkit contract. Additional local JSON research/captures are not implicitly
included by the document-selection interface; the selected Markdown retains their
conclusions and references, and all local originals remain untouched.

No archive, cleanup/deletion, publication, commercial-ROM download or toolkit edit
was performed or authorized. Future archive execution requires separate authorization;
its CLI uses repeated --document/--omit-document entries from the two arrays.
This preparation starts no background process. The workspace remains intact.
