# Round 002 disassembly handoff

Status: single disassembly pass complete; ROM incomplete. ROM classification: 100.00% (262157 / 262160 bytes classified; 3 bytes unknown).

Accepted6381 code+79575 data new bytes; total19471 code+242686 non-code. All503 earlier regions preserved verbatim;7797 distinct code views re-decoded using configured binary;3360 scene columns and screen/event/descriptor bounds checked independently. [Review](round-002-disassembly/review-result.json), [exact counts](round-002-disassembly/counts.json), [evidence and per-bank table](progress.md). New annotations Static, natural reachability unresolved. Broad prior gameplay and artificial terminal win/dismissal reused without replay.

Checkpoint [maps/round-002-disassembly.json](../maps/round-002-disassembly.json), state/current SHA256 d6af065d7fd00334045ea58046b2704e3d4a51caa5f3b6be8db0abc7461ea863. [Fresh verification](round-002-disassembly-verification.json): byte_identical true, empty generation diagnostics. ROM unchanged.

Only residual physical$3327..$332A CPU$B317 [0,1,14,15],4C4DB3. ORPHAN-JUMP-001 remains blocked on independent structural entry/origin evidence; bypassing predecessor and legal instruction shape leave code/data ambiguous. Reuse [focused reasoning](redirect/r001-static-streams/deferred-review.md). No terminal exhaustion claim; coordinator must review any terminal decision after commentary/redirect.

Next full updated-source commentary: generated/round-002-disassembly/game.asm is new source; maintained commentary/source/game.asm unchanged SHA256 cf1851d19476ee60e2409f7ac72cf136d5080616cf4964682e23502bc03b794c. Use supported rebase workflow and preserve all prior functional comments/symbols/history; do not overwrite maintained source. Then redirect can review residual with current evidence policy.

Toolkit existing NES-001, zero new findings, no blocker; [report](toolkit-issues.md). No toolkit changes, archive, deletion, cleanup or publication.
