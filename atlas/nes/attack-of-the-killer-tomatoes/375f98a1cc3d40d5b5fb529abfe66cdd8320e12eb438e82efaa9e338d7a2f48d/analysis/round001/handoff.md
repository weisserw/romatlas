# Round001 disassembly handoff

Status: phase complete; ROM incomplete.

ROM classification: 59.93% (157105 / 262160 bytes classified; 105055 bytes unknown).
Code15126; data/image141979; unknown105055. Observed code11045; static4081. All classified regions annotated with evidence. Counts/per-bank: counts.json and analysis/progress.md.

Resume authoritative internal analysis/state.json; portable maps/current.json. Exact verified map maps/pass-001-final.json (SHA256 a10e5d449f1da16d2fcb15bb5c2ee2514d66c7caae0b953ac076906bf7de2b54); source generated/pass-001-final; analysis/pass-001-final-verification.json reports byte_identical=true. Input SHA unchanged.

Game Attack of the Killer Tomatoes NES, likely USA. Natural street play; artificial sewer/factory/scaffolding/chamber, story/intro and explosion-scene visits. Boss and full credits not confirmed. Deterministic replay, patches, button releases/budgets, progression checklist and durable snapshot paths: analysis/scenarios.json; research/version caveats: analysis/gameplay.md. No need to replay completed waits.

L001–L004 verified; next L005 boss initialization, L006 ending conditions, L007 mixed CHR banks4..7 upload contracts, L008 indirect-dispatch/residual PRG. Concrete recipes and retry rules: analysis/leads.json. ROM data source bank context is critical; do not infer ROM execution from RAM roots. Tile sheet and review.json preserve supporting review. Ordinary easy-gain stop, not terminal exhaustion. Next phase commentary reviews all current classified source, then redirect investigates actionable leads. No toolkit issues found; analysis/toolkit-issues.md.
