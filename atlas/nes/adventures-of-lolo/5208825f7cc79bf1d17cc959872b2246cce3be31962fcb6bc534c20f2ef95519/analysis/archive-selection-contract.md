# Archive document selection contract

Coordinator inspected `/root/romatlas/scripts/archive.py` (FINAL and archive document-selection logic) on 2026-11-04. No archive command was run. Archive preparation does not authorize archive execution, publication or cleanup.

The script automatically selects its FINAL list (NES `.sym` entries become `.json`). Additional final Markdown must be explicitly selected through `documents` in `archive-options.json`, represented by future `--document` arguments. Disposable Markdown may be classified in `omitted_documents`, represented by `--omit-document`. This file is itself a final coordinator record and must be selected.

Additional documents must exist, be workspace-relative `.md` paths beneath `analysis/` or `commentary/`, contain no `..`, and cannot be `analysis/toolkit-issues.md`. A selected final artifact cannot also be omitted. The script recognizes toolkit issues and commentary prompt/workflow/AGENTS files specially, without selecting them for archival. It excludes Markdown beneath `before` or `source` directory components from the unrecognized-document check.

At terminal handoff, enumerate all Markdown recursively under analysis and commentary and compare against FINAL, explicit selections/omissions and these documented exceptions. Select final conclusions, phase handoffs and durable investigation records; omit only disposable scratch. Never omit final findings to satisfy selection checks. Recompute the inventory after the final phase so newly created documents are accounted for. This record describes selection only; source/editorial/map verification remains independently required.

Round 2 preservation inventory: explicitly select prior commentary progress and uncertainties under `analysis/round-2/previous-commentary/`; they retain historical conclusions and review scope. Classify only that preserved copy's duplicate AGENTS/prompt/workflow Markdown as omitted scaffolding: these are toolkit-generated phase instructions, not findings or handoffs, and the current instructions remain recognized in commentary/. No files are removed. Recheck the full inventory at terminal handoff.
