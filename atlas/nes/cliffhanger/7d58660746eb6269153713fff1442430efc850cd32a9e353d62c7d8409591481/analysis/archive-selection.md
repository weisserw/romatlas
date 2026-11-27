# Archive document selection

The coordinator inspected /root/romatlas/scripts/archive.py, especially FINAL, additional-document validation and the unknown-Markdown check (lines379–404). This file and archive-options.json prepare a document selection only. No archive invocation, cleanup, publication or deletion is authorized or performed.

The contract automatically recognizes its fixed final documents, analysis/toolkit-issues.md and commentary's current AGENTS/prompt/workflow. Extra selected paths must be existing workspace-relative analysis/commentary Markdown, excluding toolkit-issues.md. It ignores Markdown beneath source/ and before/. Every other Markdown must be selected or explicitly omitted.

Selection policy: preserve all extra phase and final handoffs, coordinator decisions, investigation plans/results/frontiers, rejection/contract reasoning, counts/reviews and historical commentary progress/uncertainties. These explain both accepted evidence and the terminal incomplete assessment. Historical conclusions are retained rather than dropped for archive convenience. Current default final documents remain selected by the toolkit's contract. Toolkit issues remain local under its explicit special exclusion.

Omission policy: only duplicate AGENTS.md, prompt.md and workflow.md instructional copies in analysis/round-002,round-003,round-004/commentary-prior are disposable duplicate documents. Their current equivalents remain recognized in commentary/ and they contain no final result or investigation decision. Their historical progress.md and uncertainties.md are explicitly preserved. No file is deleted by this selection.

archive-options.json contains the exact documents and omitted_documents arrays. analysis/final-verification.json records an independent inventory check against the current archive.py contract, with every unrecognized Markdown accounted for and no selected final omitted. The options must be re-inventoried if any later Markdown is added. All internal state, source/assets and evidence remain local; the supported ROM-free portable map is the only sharing deliverable. Nothing here publishes it.

## Manual fallback retention update

The explicit user-authorized fallback record analysis/manual-residual-fallback/conclusions.md and its new phase/final handoffs are selected for retention alongside the prior terminal incomplete conclusions. The final fallback handoff supersedes the old disposition without erasing it. The round-005 commentary snapshot retains historical progress, uncertainties and editorial conclusions; only its three duplicate instructional files join the omitted instructions. All other new analysis/commentary Markdown is selected. No archive is executed. A fresh coordinator verification records exact document accounting after all final documents are written.
