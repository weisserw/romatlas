# Disassembly 001 handoff

One pass complete; ROM incomplete. ROM classification: 61.39% (160941 / 262160 bytes classified; 101219 bytes unknown). Formula: 100*C/T, unique structurally classified physical bytes / entire file; U=T-C. Code17440, Data12429, Image131072. Exact per-bank counts and unknown intervals: disassembly-001/counts.json, disassembly-001/unknown.json.

Verified checkpoint pass-001: maps/pass-001.json; generated/pass-001/game.asm; analysis/pass-001-verification.json. Byte-identical true. Current state analysis/state.json; map maps/current.json. Input hash unchanged. Configured binary used, no toolkit edits/deletion/archive/publication. Server stopped; restart and map.load. Durable snapshots contain local ROM/state; keep local.

Game: Cyber Stadium Series: Base Wars, 1991 Ultra/Konami. Broad checklist and exact snapshot/patch/input recipes: scenarios.json; sourced controls: gameplay.md. Natural open-game pitching, pennant configuration/rosters/standings/ranking/change, parts shop/categories/weapons visited. Artificial $68/$69/$6A=[2,0,2] plus pitch reached batted-ball fielding; bounded first-base throw did not establish combat or later inning. Artificial $30/$34=$0F reached city/CONGRATULATIONS but graphics show partial initialization; clean ending and natural reachability unproved. No completed purchase claimed.

Structural evidence: 8532 map regions, every region has evidence metadata. Actual accepted fetch roots plus supported conservative static successors; explicit effective 8 KiB bank contexts, no uploaded-ROM-origin assumptions. 23 exact uniform PRG runs classified generic Data after adjacent-byte/bank-tail inspection; no accepted fetch overlaps. Header, CHR and six vector bytes classified independently. Scope of generic Data is non-code, not a specific table/fill purpose. All unreviewed gaps stay unknown.

Runtime coverage separately: 8241 physical instruction starts, 9536 contextual roots, aggregate_dropped=0. Detail intentionally bounded/dropped; cumulative snapshot branches are not a single chronological trace. Local evidence disassembly-001/batch*-contexts/candidates/diagnostics/conflicts.json and final-trace-summary.json.

Commentary should review full current source: names/comments are minimal and editorial gate is pending. Redirect has four concrete leads in leads.json: mode$20/substate$30 dispatch and RAM$0036 jump frontier; combat close-play state contract; clean ending entry near fixed C7FB (GG YENKGYAE is operand C7FC=$0F compare$00); residual PRG data/DPCM-like patterned spans. Follow connected evidence rather than treating initial byte intervals as ceilings. This is an ordinary easy-gain handoff, not terminal exhaustion.

Toolkit report: toolkit-issues.md, one confirmed documentation/API consistency bug (NES input uses buttons array; shared button/pressed silently ignored), no blockers; no suspected bugs or missing features. Final per-bank ledger and verification details in progress.md/investigation.md.
