# Round 001 disassembly handoff — completed phase, incomplete ROM

ROM classification: 90.50% (237228 / 262144 bytes classified; 24916 bytes unknown).

Input: initialized workspace, no baseline/classification. Output: pass-001. One disassembly pass only; no commentary or redirect started.

Verified: maps/current.json validates; analysis/state.json saved and identity matches map. maps/pass-001.json generated into generated/pass-001/game.asm and rebuilt byte-identically; analysis/pass-001-verification.json has ok=true, byte_identical=true, map SHA256 74a10712737c0d2cda0e10eef09eba2267b61aff9ebb484baf8720a558680c14. Input ROM SHA256 unchanged. Baseline independently preserved.

Findings: 14,979 code bytes; 222,249 data bytes. Banks3–15 fully classified generic data aggregates with relative-offset asset/sound consumers and inspected patterns, not invented original source layout. Banks0/1/2 retain 4,033 / 9,184 / 11,699 unknown bytes. See classification.json and unknown-ranges.json for exact counts/complement. All classified bytes carry evidence and comments; editorial naming/readability awaits commentary. Accepted opcode evidence union: 6,904 unique starts, 14,626 decoded bytes, natural and artificial scenarios combined; static code is separate.

Gameplay: natural Stage1, artificial next Stage2, Stage8, final Stage13 and main ending artwork; Quest puzzle and Endless field exercised. Full setup pointer edits at C94E=1015/12A4 preserve initializer behavior. D47F=1/D482=1 forces result. D486-only route was overwritten and reached Stage2: legacy stage8-field snapshot name is misleading, ledger corrects it. CE14 single-cell loss attempt failed within120frames. See ../../scenarios.json and durable snapshots/captures here. Later Quest30/Quest ending and full credits deferred; linked Versus outside link scope. No endless forced ending attempts.

Lead IDs: R1-L001/002/003/006 verified, R1-L004 superseded, R1-L005 rejected with narrow scope. R1-U001/U002: concrete residual dispatch/table classification questions in banks0–2; R1-U003: Quest password/progression contract; R1-U004: full current-source commentary. See ../../leads.json. This is easy-gain pass exhaustion, not terminal investigation exhaustion; redirect has productive concrete work. Rejection scope review in ../../investigation.md.

Toolkit report ../../toolkit-issues.md: two confirmed API/documentation mismatches TK-001 (max_instructions vs budget), TK-002 (buttons array vs button/pressed); no suspected bugs/missing features or blockers. Workarounds used. Unexpected long-lived host SIGTERM unassigned; short local serve scripts and persisted snapshots avoided it. No toolkit modifications.

Evidence index: ../../progress.md; ../../investigation.md; ../../gameplay.md; ../../scenarios.json; rpc.jsonl; code-evidence.json (initial batch), all-code-evidence.json (final union), static-reviewed.json, script-inspection.json, asset-container-evidence.json, more-data-evidence.json, music-consumers.json, direct-call-discovery.json. Start a new server with map.load then emulator.start and snapshot.import; reassert intended buttons. Named snapshots do not persist. All ROM/state/assets/source remain local.
