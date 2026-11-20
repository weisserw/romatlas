# Archive document selection

Inspected `/root/romatlas/scripts/archive.py`, `FINAL` and archive() document-selection
contract (lines345–404). Contract SHA256: `9dd430235e3f3f48dd345c90bc0d21a42456bed757307f090c588d5c28810bd3`.
No archive function or CLI was invoked. This is a local selection manifest only.

The contract selects default final artifacts, translates NES .sym paths to .json,
and recognizes toolkit-issues/current generated commentary instruction files without
copying them as additional final documents. It requires explicit --document or
--omit-document for other Markdown under analysis/commentary, excluding before/source.
Additional --document paths must be local Markdown under analysis/commentary and may
not be toolkit-issues.md; omitted paths must not collide with selected defaults.

archive-options.json uses exactly the requested documents and omitted_documents
arrays. All paths are workspace-relative and exist. No final conclusions, phase
handoffs, research/rejection records, final audit or historical progress/uncertainty
results are omitted. The omitted files below are generated instruction/scaffold copies,
with no unique analysis conclusions. They remain intact locally; omission authorizes
no deletion. The separately preserved commentary-generated-workflow.md is selected
because it is evidence for a reported documentation defect.

Inventory: 131 Markdown files; 91 additional final documents; 30 disposable scaffold omissions; 10 contract-recognized files; 0 before/source exclusions. Every unrecognized path is in exactly one array.

## Explicit additional final documents

- `analysis/archive-document-selection.md`
- `analysis/coordinator-completion-audit.md`
- `analysis/coordinator-rejection-review.md`
- `analysis/evidence/round-002/review.md`
- `analysis/evidence/round-003/review.md`
- `analysis/evidence/round-004/review.md`
- `analysis/evidence/round-005/review.md`
- `analysis/evidence/round-006/review.md`
- `analysis/evidence/round-007/review.md`
- `analysis/evidence/round-008/review.md`
- `analysis/evidence/round-009/review.md`
- `analysis/evidence/round-010/review.md`
- `analysis/evidence/round-011/review.md`
- `analysis/final-acceptance.md`
- `analysis/final-handoff.md`
- `analysis/handoffs/final-editorial-acceptance.md`
- `analysis/handoffs/round-001-commentary.md`
- `analysis/handoffs/round-001-disassembly.md`
- `analysis/handoffs/round-001-redirect.md`
- `analysis/handoffs/round-002-commentary.md`
- `analysis/handoffs/round-002-disassembly.md`
- `analysis/handoffs/round-002-redirect.md`
- `analysis/handoffs/round-003-commentary.md`
- `analysis/handoffs/round-003-disassembly.md`
- `analysis/handoffs/round-003-redirect.md`
- `analysis/handoffs/round-004-commentary.md`
- `analysis/handoffs/round-004-disassembly.md`
- `analysis/handoffs/round-004-redirect.md`
- `analysis/handoffs/round-005-commentary.md`
- `analysis/handoffs/round-005-disassembly.md`
- `analysis/handoffs/round-005-redirect.md`
- `analysis/handoffs/round-006-commentary.md`
- `analysis/handoffs/round-006-disassembly.md`
- `analysis/handoffs/round-006-redirect.md`
- `analysis/handoffs/round-007-commentary.md`
- `analysis/handoffs/round-007-disassembly.md`
- `analysis/handoffs/round-007-redirect.md`
- `analysis/handoffs/round-008-commentary.md`
- `analysis/handoffs/round-008-disassembly.md`
- `analysis/handoffs/round-008-redirect.md`
- `analysis/handoffs/round-009-commentary.md`
- `analysis/handoffs/round-009-disassembly.md`
- `analysis/handoffs/round-009-redirect.md`
- `analysis/handoffs/round-010-commentary.md`
- `analysis/handoffs/round-010-disassembly.md`
- `analysis/handoffs/round-010-redirect.md`
- `analysis/handoffs/round-011-commentary.md`
- `analysis/handoffs/round-011-disassembly.md`
- `analysis/handoffs/round-011-redirect.md`
- `analysis/redirect/round-001-a/research.md`
- `analysis/redirect/round-002-a/research.md`
- `analysis/redirect/round-003-a/plan.md`
- `analysis/redirect/round-003-a/rejection-review.md`
- `analysis/redirect/round-003-a/research.md`
- `analysis/redirect/round-004-a/plan.md`
- `analysis/redirect/round-004-a/rejection-review.md`
- `analysis/redirect/round-004-a/research.md`
- `analysis/redirect/round-005-a/research.md`
- `analysis/redirect/round-006-a/research.md`
- `analysis/redirect/round-007-a/research.md`
- `analysis/redirect/round-008-a/research.md`
- `analysis/redirect/round-009-a/research.md`
- `analysis/redirect/round-010-a/brk-correction.md`
- `analysis/redirect/round-010-a/rejection-review.md`
- `analysis/redirect/round-010-a/research.md`
- `analysis/redirect/round-011-a/rejection-review.md`
- `analysis/redirect/round-011-a/research.md`
- `analysis/round-10/commentary-round-009/progress.md`
- `analysis/round-10/commentary-round-009/uncertainties.md`
- `analysis/round-11/commentary-round-010/progress.md`
- `analysis/round-11/commentary-round-010/uncertainties.md`
- `analysis/round-2/commentary-generated-workflow.md`
- `analysis/round-2/commentary-progress-active-history.md`
- `analysis/round-2/commentary-redirect-questions.md`
- `analysis/round-2/commentary-round-001/progress.md`
- `analysis/round-2/commentary-round-001/uncertainties.md`
- `analysis/round-3/commentary-round-002/progress.md`
- `analysis/round-3/commentary-round-002/uncertainties.md`
- `analysis/round-4/commentary-round-003/progress.md`
- `analysis/round-4/commentary-round-003/uncertainties.md`
- `analysis/round-5/commentary-round-004/progress.md`
- `analysis/round-5/commentary-round-004/uncertainties.md`
- `analysis/round-6/commentary-redirect-questions.md`
- `analysis/round-6/commentary-round-005/progress.md`
- `analysis/round-6/commentary-round-005/uncertainties.md`
- `analysis/round-7/commentary-round-006/progress.md`
- `analysis/round-7/commentary-round-006/uncertainties.md`
- `analysis/round-8/commentary-round-007/progress.md`
- `analysis/round-8/commentary-round-007/uncertainties.md`
- `analysis/round-9/commentary-round-008/progress.md`
- `analysis/round-9/commentary-round-008/uncertainties.md`

## Explicit disposable omissions

- `analysis/round-10/commentary-round-009/AGENTS.md` — generated phase instructions; no unique final result.
- `analysis/round-10/commentary-round-009/prompt.md` — generated phase instructions; no unique final result.
- `analysis/round-10/commentary-round-009/workflow.md` — generated phase instructions; no unique final result.
- `analysis/round-11/commentary-round-010/AGENTS.md` — generated phase instructions; no unique final result.
- `analysis/round-11/commentary-round-010/prompt.md` — generated phase instructions; no unique final result.
- `analysis/round-11/commentary-round-010/workflow.md` — generated phase instructions; no unique final result.
- `analysis/round-2/commentary-round-001/AGENTS.md` — generated phase instructions; no unique final result.
- `analysis/round-2/commentary-round-001/prompt.md` — generated phase instructions; no unique final result.
- `analysis/round-2/commentary-round-001/workflow.md` — generated phase instructions; no unique final result.
- `analysis/round-3/commentary-round-002/AGENTS.md` — generated phase instructions; no unique final result.
- `analysis/round-3/commentary-round-002/prompt.md` — generated phase instructions; no unique final result.
- `analysis/round-3/commentary-round-002/workflow.md` — generated phase instructions; no unique final result.
- `analysis/round-4/commentary-round-003/AGENTS.md` — generated phase instructions; no unique final result.
- `analysis/round-4/commentary-round-003/prompt.md` — generated phase instructions; no unique final result.
- `analysis/round-4/commentary-round-003/workflow.md` — generated phase instructions; no unique final result.
- `analysis/round-5/commentary-round-004/AGENTS.md` — generated phase instructions; no unique final result.
- `analysis/round-5/commentary-round-004/prompt.md` — generated phase instructions; no unique final result.
- `analysis/round-5/commentary-round-004/workflow.md` — generated phase instructions; no unique final result.
- `analysis/round-6/commentary-round-005/AGENTS.md` — generated phase instructions; no unique final result.
- `analysis/round-6/commentary-round-005/prompt.md` — generated phase instructions; no unique final result.
- `analysis/round-6/commentary-round-005/workflow.md` — generated phase instructions; no unique final result.
- `analysis/round-7/commentary-round-006/AGENTS.md` — generated phase instructions; no unique final result.
- `analysis/round-7/commentary-round-006/prompt.md` — generated phase instructions; no unique final result.
- `analysis/round-7/commentary-round-006/workflow.md` — generated phase instructions; no unique final result.
- `analysis/round-8/commentary-round-007/AGENTS.md` — generated phase instructions; no unique final result.
- `analysis/round-8/commentary-round-007/prompt.md` — generated phase instructions; no unique final result.
- `analysis/round-8/commentary-round-007/workflow.md` — generated phase instructions; no unique final result.
- `analysis/round-9/commentary-round-008/AGENTS.md` — generated phase instructions; no unique final result.
- `analysis/round-9/commentary-round-008/prompt.md` — generated phase instructions; no unique final result.
- `analysis/round-9/commentary-round-008/workflow.md` — generated phase instructions; no unique final result.

## Contract-recognized Markdown

- `analysis/gameplay.md`
- `analysis/investigation.md`
- `analysis/progress.md`
- `analysis/toolkit-issues.md`
- `commentary/AGENTS.md`
- `commentary/editorial-audit.md`
- `commentary/progress.md`
- `commentary/prompt.md`
- `commentary/uncertainties.md`
- `commentary/workflow.md`

Default final map/state/scenario/review/audit/verification artifacts remain selected
by the toolkit contract. Additional local JSON/captures are not implicitly selected
by this Markdown-only interface; selected final handoffs/reviews retain their
conclusions. All internal artifacts stay local. No archive execution, publication,
cleanup or deletion is authorized/performed. A separately authorized future operator
can supply repeated --document/--omit-document CLI arguments from these arrays.
