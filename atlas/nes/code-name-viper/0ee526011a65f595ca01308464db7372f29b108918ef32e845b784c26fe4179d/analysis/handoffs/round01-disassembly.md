# Round01 disassembly handoff — current source

Status: this one pass complete; ROM incomplete. Next coordinator phase: full current-source commentary, then redirect.

ROM classification: 58.80% (154141 / 262160 bytes classified; 108019 bytes unknown).

Code 14592, non-code 139549; 636 evidence-annotated regions. Exact residuals and per-bank counts: ../pass-001/unknown.json and ../pass-001/counts.json. Execution: 5776 physical starts, 12230 bytes; natural-scenario bytes 5692; keep separate from classification.

Current source: generated/pass-001/game.asm (all currently classified source). Exact map maps/pass-001.json; current internal state analysis/state.json, current portable map maps/current.json. Verification analysis/pass-001-verification.json byte_identical=true. ROM SHA 0ee526011a65f595ca01308464db7372f29b108918ef32e845b784c26fe4179d; map SHA 2c7c765e75c29d2c26d5f67ca313efbe26f748123cac6bc751eee40fdb0ad7f6.

Structural evidence: accepted contextual fetches plus bounded static direct successors with mapper/call stops; CHR/header/vector roles; ASCII bank markers; 54 six-digit password records and three metadata arrays; maximal uniform non-code intervals with exact endpoint checks. No code from pointer hypotheses alone. Most code regions still need meaningful labels/comments. Generic data meanings can remain unresolved honestly. Readability has not been audited in this phase.

Game Code Name: Viper. Exact scripts, local RPC log and durable snapshots: analysis/pass-001/. See analysis/scenarios.json, gameplay.md and investigation.md for route and evidence. Active stage3 forest firing confirmed; jump/climb/crouch results unconfirmed. Main replay checkpoint action-play.snapshot.json is artificial from password and understood delay skips. Its context [0,1,14,15], frame644. Initial clean password.snapshot.json is reusable across processes.

Ready redirect: L004 understand boss-room dialogue and boss victory/ending flags using boss.snapshot.json. L005 resume late7-preview.snapshot.json, enter actual post-initialization map countdown loop before patching; earlier early patch was overwritten. L006 inspect residual PRG data/dispatch/level structures with existing references and generic-data fallback. L001/L002 verified narrow slices; L003 external081629/217298 claims rejected for this ROM. Local corrected password table provides stage0..8, quest/difficulty domains. Do not require natural reachability for static structural claims.

No toolkit issues found; persistent analysis/toolkit-issues.md has no blocker. Configured binary unchanged; no archive/deletion/publishing/toolkit edit performed. This is an ordinary easy-gain handoff, not terminal exhaustion or overall goal completion. Preserve all existing evidence and map annotations.
