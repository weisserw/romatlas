# Round 1 commentary handoff

Current-source readability complete; ROM classification incomplete at pass-004: 276715/393232 classified, 116517 unknown, 417 code bytes. Reviewed all 49 physical inventory intervals and eight meaningful units in commentary/review.json. All classified source now has descriptive mechanical naming/comments; every unknown interval has an honest physical-boundary note. No source backlog remains. Final editorial audit is pending, not invoked for this intermediate round.

Buildable maintained source: commentary/source/game.asm. Frozen baseline and state: commentary/baseline.json and phase1-state.json; before/ preserved. reading-after.asm refreshed. Fresh commentary/verification.json: byte_identical=true, input SHA256 24fb1cf2d308e07dbedadc3822ca07b2b48b7ebc1da0150e63203d2fdedfcc75, game.asm SHA256 57e194928bdd7c0dc11095c3ca4109b769b187adcd4840fa8a086d19e6d14596. Map/state SHA256 remain 13e76833ad4c8350ca4fffcdbc6ac693aa5a100b19bcc07180a8df40db5c5e81.

Examples: named NMI pending/cached paths, PPU and mapper shadows, interrupt register saves, reset vblank/zero-page/bank loops, partial clear and startup prefixes. Uniform PRG runs retain generic data purpose notes; CHR tile format and interrupt vector layout explained. No classification changes or runtime claims.

Redirect questions are concrete continuations from disassembly, with clarified local contracts in commentary/uncertainties.md: C117 continuation and reset clear range; C229 mapper helper from D394; E2CA dispatch target domain/table extent from D3; C084/C1DB interrupted/resulting bank context. NMI does not explicitly restore C000; verify before assuming it. Startup fields and uniform data semantics remain unknown, without blocking commentary.

TK-001 remains the one documented missing feature: mapper69 debugger admission blocks all runtime/gameplay experiments; see analysis/toolkit-issues.md. No toolkit/input edits, publication or archival action. Durable historical checkpoints preserved. Free disk 4.9GiB, small scratch only. This worker stops at commentary; coordinator selects the next phase.
