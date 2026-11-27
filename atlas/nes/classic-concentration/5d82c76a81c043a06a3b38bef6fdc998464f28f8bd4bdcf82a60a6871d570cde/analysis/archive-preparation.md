# Archive preparation contract

Coordinator inspected `/root/romatlas/scripts/archive.py`, especially `FINAL` and `archive()` document selection (lines 345–404), during round 001. No archive execution or cleanup is authorized or performed.

`archive-options.json` must provide workspace-relative `documents` and `omitted_documents` arrays. Additional selected documents must be existing `.md` files beneath analysis/ or commentary/, without parent traversal; analysis/toolkit-issues.md is explicitly excluded by the archive contract. Default FINAL documents are automatically selected. Toolkit issues and commentary/prompt.md, commentary/workflow.md, commentary/AGENTS.md are recognized exclusions. Markdown beneath before/ or source/ is excluded from the unrecognized-document check. Every other Markdown document needs explicit final selection or scratch omission. Selected final artifacts cannot also be omitted.

The final inventory must preserve final findings, current-source commentary assessments, phase handoffs and terminal reasoning. Historical investigation records remain local and should be selected when needed to understand the result. Scratch classification must not discard final results merely to satisfy archive checks. The contract check alone does not establish editorial acceptance, byte identity, ROM classification, or permission to archive.

Initial selection written to archive-options.json after round001 commentary. All current additional Markdown findings/handoffs are selected; no disposable Markdown scratch currently exists, so omitted_documents is empty. Redirect and subsequent phase documents must be incorporated and the inventory revalidated before terminal handoff. Final selection and inventory verification remain pending phase completion.

## Round002 preservation decisions

Both preserved pass001 commentary copies retain their progress and uncertainties as selected historical results. Their copied AGENTS.md, prompt.md and workflow.md are duplicate operational instructions, classified in omitted_documents as disposable preparation material; no conclusion/handoff is omitted. All copies remain untouched locally. Final terminal inventory remains pending.

## Final terminal selection

Terminal rejection review accepted by coordinator; final editorial audit passed. Final archive-options.json selects all additional conclusions, handoffs, review reasoning and historical progress/uncertainties. Nine omitted Markdown files are duplicate AGENTS/prompt/workflow instructions in preserved commentary copies; no final result is omitted. Final inventory in analysis/archive-document-inventory.json is computed against the current archive.py FINAL/default exclusions and source/before rule. Final selection is verified without executing archive or deleting local files.

## Authorized seven-byte fallback continuation

The coordinator re-read the current toolkit `scripts/archive.py` FINAL and archive() selection rules before this continuation's final handoff. No archive is executed. The final selection additionally retains the fallback disassembly, commentary and editorial handoffs, final fallback conclusions, requirement audit, and prior commentary progress/uncertainty/editorial records. Copied operational instructions alone remain classified as omissions; no final finding is omitted. The selected final fallback handoff records exact physical ranges, prior ambiguity and the user's explicit Manual Data authorization. The retained machine verification records distinguish map coverage from evidence-backed structural proof. All files remain local, including omitted duplicate instructions.
