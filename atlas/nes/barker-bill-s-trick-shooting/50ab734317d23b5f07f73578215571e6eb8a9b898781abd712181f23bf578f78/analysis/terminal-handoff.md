# Terminal handoff: orchestration complete, ROM incomplete

The coordinator workflow reached a justified terminal-incomplete handoff after three sequential disassembly/commentary/redirect rounds and a separate final editorial acceptance. No productive research is interrupted or waiting on an external process. This completes orchestration, not whole-ROM classification. New independent structural evidence or a justified changed inference may reopen a localized residual; unchanged planning/replay is not a next task.

ROM classification: 100.00% (196618 / 196624 bytes classified; 6 bytes unknown).

Exact classification is 99.99694849051998%; rounding does not establish completion. Classified code is 24,553 bytes; data/image is 172,065 bytes. Physical unknowns are:

| Physical | Byte | CPU alias | Effective 8 KiB PRG banks |
|---|---|---|---|
| $0403A | $00 | $802A | [2,3,6,7] |
| $041AC | $60 | $819C | [2,3,6,7] |
| $042E8 | $60 | $82D8 | [2,3,6,7] |
| $04999 | $60 | $8989 | [2,3,6,7] |
| $060CE | $60 | $A0BE | [2,3,6,7] |
| $0A1B2 | $60 | $A1A2 | [4,5,6,7] |

These are code-versus-data ambiguities, not missing game meanings. Tail jumps/returns establish preceding endpoints without choosing a following byte's role; inline-word and palette extents stop before their singleton; apparent raw address hits are frame bytes or immediate instructions. The accepted generic-data byte $02A21 had aligned consumer-target and closing record constraints that do not extend to these six. No useful discriminating test or ready lead survived the [localized rejection review](redirect/r03-20261113-a/rejection-review.md), [connected/independent avenue review](redirect/r03-20261113-a/findings.md), and [coordinator assessment](coordinator/terminal-rejection-review.md). Decisive primary artifacts and exact ledger remain local, linked in [redirect handoff](handoff-r03-redirect.md). Lead history remains in leads.json; R02-L006 is verified, R01-L005 records the unresolved six-byte frontier. An unresolved lead is not a running or externally blocked workflow.

Authoritative checkpoint: **pass-003-accepted**, not the preserved preacceptance pass-003-final. Current map/state/checkpoint SHA256 is `f7c49fce4ad15bbe7ad6b7d51354e5cf3aeb46760784f9902c148b9f198eaab1`. [Checkpoint verification](pass-003-accepted-verification.json) is byte-identical. Fresh configured-binary store and portable-map validation passed; [independent state/count checks](coordinator/final-state-verification.json) confirm exact residuals, evidence-bearing classified regions, matching map/store/checkpoint and unchanged input/binary. Input SHA256 remains `50ab734317d23b5f07f73578215571e6eb8a9b898781abd712181f23bf578f78`.

Current-source readability and final editorial acceptance are complete for all classified material: 951 reviewed entries, 298 functional reading scopes (944 contained fragment/unit records). Prior commentary and review credit are preserved across rounds. [Final editorial handoff](handoff-final-editorial.md), [editorial assessment](../commentary/editorial-audit.md), [fresh audit](../commentary/audit.json), and [coordinator current-source checks](coordinator/final-editorial-verification.json) record actual checks and repairs. Maintained source is commentary/source/game.asm, with refreshed commentary/reading-after.asm. Final source SHA256 is `241a4cbad07160804758c2e1fdfa57af79cfa550dc90ab6289456d7a3617cd1e`; fresh rebuild is byte-identical and the reading copy matches. Unknown singleton notes remain explicit. No editorial/readability scope is deliberately deferred.

Broad progression evidence includes all galleries, menu/bonus/results, high rounds and Bill's throwing/recovery; exact inputs, RAM shortcuts, bank identities and artificial provenance remain in scenarios.json and linked evidence. Natural Zapper hits are unavailable under the documented NES-001 missing feature; no finite ending, natural selection of every structure, or full hardware/game compatibility is established. [Toolkit report](toolkit-issues.md) retains one missing feature and zero confirmed/suspected bugs; it is not a structural-work blocker.

The archive document-selection contract was inspected in scripts/archive.py; [contract record](coordinator/archive-contract.md). Workspace-root archive-options.json explicitly selects additional final Markdown, including phase handoffs, rejection reviews, coordinator conclusions and preserved history. Its documents and omitted_documents arrays are checked against every contract-unrecognized Markdown file in analysis/ and commentary/. [Document audit](coordinator/archive-document-audit.json) records the complete current inventory. There is no disposable scratch Markdown to omit; no final conclusion was omitted to satisfy selection. Full internal state, captures, source and assets remain local. This prepares archive inputs only; it is not an archive or sharing operation.

No input mutation, toolkit edit, archive execution, cleanup, workspace deletion or publication was performed. Final completion requirements and their authoritative evidence are recorded in [completion audit](coordinator/completion-audit.json). Preserve this terminal assessment on unchanged premises; do not classify the six bytes merely to obtain exact 100%.
