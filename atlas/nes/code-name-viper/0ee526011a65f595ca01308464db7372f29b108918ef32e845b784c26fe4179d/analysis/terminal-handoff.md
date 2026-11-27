# Terminal handoff — justified incomplete classification

The coordinator workflow reached its documented terminal incomplete handoff after five sequential disassembly/commentary/redirect rounds, direct coordinator rejection review and final editorial acceptance. It is not blocked. Whole-ROM classification remains incomplete.

ROM classification: 99.99% (262144 / 262160 bytes classified; 16 bytes unknown).

Current checkpoint: [maps/pass-005.json](../maps/pass-005.json), [portable NES map](../maps/current.json), local internal state analysis/state.json. All three map identities are SHA2569ee8e27a9dbcb20283460614a5113e3869648a4d45ea5334215468301f82964e. Input ROM SHA2560ee526011a65f595ca01308464db7372f29b108918ef32e845b784c26fe4179d is unchanged. The supported portable map is the ROM-free mapping deliverable; source, ROM backing, snapshots and internal evidence remain local.

## Current source and verification

Maintained readable source: commentary/source/game.asm and constants.inc. Reading copy: [commentary/reading-after.asm](../commentary/reading-after.asm). All1560 scaffold entries,1551 classified regions and727 distinct meaningful units are reviewed with no readability frontier. There are1436 valid unit references, including overlapping units. Final supported audit passed: ok=true, byte_identical=true, editorial_audit=true. [Final editorial handoff](handoffs/final-editorial.md), [editorial assessment](../commentary/editorial-audit.md), [audit](../commentary/audit.json), [verification](../commentary/verification.json).

Final game.asm SHA25628816a872f2c3d663612900ea281ef4c89d553d0a9d1c047af0cc2409ec05293; constants.inc SHA256c9f0335a8b9400cb223f1fcea6fd2addb987ed794b8b572bf7b57f713e826d68. Instructions and classifications were unchanged by final editorial repairs. Historical editions are preserved under analysis/round-2 through round-5. A local migration defect lost pre-marker Data annotations;168 were restored and actual historical labels/source anchors independently checked. One intentional scene-entry rename is documented. Final review also repaired stale boundary prose and source-root-relative review paths.

## Exact residuals and stopping rationale

| Physical half-open range | Bytes | Competing structural roles |
| --- | ---: | --- |
| 6253..6255 | 2 | conditional scanner branch versus stored bytes |
| 21457..21460 | 3 | duplicate forwarding jump versus stored fragment |
| 21541..21542 | 1 | empty return versus literal/padding |
| 22519..22522 | 3 | zero-result helper versus literal fragment |
| 22872..22873 | 1 | empty return versus literal/padding |
| 31867..31869 | 2 | data/trailer versus BRK fragment |
| 49141..49142 | 1 | return fragment versus nonzero bank-tail literal |
| 66866..66867 | 1 | empty return versus literal/padding |
| 99608..99610 | 2 | redundant load prefix versus metadata |

[Coordinator primary rejection review](coordinator-rejection-review.md) accepts the scoped reasoning in [round05 rejection review](redirect/round05-01/rejection-review.md) after inspecting the decisive primary source, consumers and unchanged ROM bytes. [Exact ledger](redirect/round05-01/unknown-ledger.json) preserves bank/CPU context and neighbors. No useful corrected inference/test or ready lead remains on current premises. Rejections do not require natural execution, a live caller or complete routine exit universally; no tiny fragment was forced to generic Data to reach100%. The earlier shared-return objection was corrected and led to independently accepted32bytes before this terminal review.

Reopen for new evidence, a justified corrected inference/method or explicit reassessment; resume from this checkpoint and the exact rejection review, not a new baseline or unchanged automatic cycle. The unresolved bytes are structural uncertainty, not semantic naming gaps or a toolkit blocker. No full hardware/game compatibility, natural victory or exhaustive execution claim follows.

## Gameplay and retained evidence

Representative middle/late active mechanics, boss confrontation, supported artificial victory, all-clear, credits and visible The End were reached with saved routes. RAM/register edits, effective banks, explicit inputs and bounded budgets are recorded in [scenarios](scenarios.json) and [round01 redirect findings](redirect/round01-01/findings.md). Earlier failures remain preserved; subsequent reviews explain why unchanged replay does not discriminate the remaining static fragments. Other stage permutations/rare mechanics are deferred as broad best effort, not claimed exhaustive.

No toolkit issues found: zero confirmed/suspected bugs or missing features, no blocker. Persistent local report: analysis/toolkit-issues.md. The configured release binary was used; toolkit files were not changed.

## Archive readiness and scope

[archive-options.json](../archive-options.json) explicitly selects additional final Markdown and classifies only byte-identical historical instruction scaffolds for omission. The final recursive selection check follows the inspected scripts/archive.py contract; [document-selection audit](archive-document-selection.json) accounts for all current analysis/commentary Markdown. Final conclusions, primary rejection reasoning, handoffs, historical findings/progress and current editorial results are preserved. Omission selects disposable duplicate instructions only; no files were removed.

[Final requirement audit](final-requirements-audit.md), [current audit](current-audit.json) and [final verification](final-verification.json) record the checks. No archive execution, cleanup/deletion, toolkit edit, publishing or commercial ROM/asset download was performed. This handoff prepares the artifacts for a separately authorized archive operation; it does not start a background process.
