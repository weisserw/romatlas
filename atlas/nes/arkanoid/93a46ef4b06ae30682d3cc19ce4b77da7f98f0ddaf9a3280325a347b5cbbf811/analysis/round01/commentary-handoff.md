# Round 1 commentary handoff

Complete current-source readability against pass-003; whole-ROM classification incomplete. Retain commentary/ as the sole maintained source; before/, baseline.json and phase1-state.json are frozen. Do not regenerate over source/.

Scope: all 49,167 classified bytes (15,189 code,17,594 data,16,384 CHR), plus honest explanation of the single unknown physical $009E / CPU $808E. review.json retains its complete 104-region scaffold, reviewed summaries/source locations and 185 meaningful units. No pending readability ranges. Final editorial_audit deliberately pending until terminal acceptance.

Primary source: commentary/source/game.asm; reading copy commentary/reading-after.asm. Functional prose/symbols cover startup/NMI, powerups/paddle/lasers, scoring, title/stage/ending state machines, motion and all four brick-collision directions, DoH, roaming enemies, sound interpreter and every data/CHR block. Example entry locations: Reset:360, NMI:469, CatchFallingCapsule:694, DispatchGameState:2879, LoadStageRecord:3255, ResolveBallBrickCollision:5506, StageRecords:8569, SoundSetOneShot:8881, SoundCall:8942. Shared pointer/scratch aliases and CPU versus physical labels are explicitly documented in the source introduction.

Fresh verification passed and reading copy refreshed. commentary/verification.json records byte_identical=true, input SHA256 93a46ef4b06ae30682d3cc19ce4b77da7f98f0ddaf9a3280325a347b5cbbf811, source SHA256 badcba5fdf8bdb771d927dd1ba95cd996e7357a19a3d13e4843b8e21793229b0. maps/current.json and analysis/state.json hashes remain equal to the frozen commentary manifest. No classification, toolkit or input changes.

Unresolved: $009E code/data identity; precise mixed-data subtable/song/species/artwork identities; peripheral hardware behavior; natural progression through all powers/two-player/warp/boss defeat. Some original corner-arbitration immediate comparisons look unusual; preserve exact behavior and investigate only with a discriminating bounded collision question. Details in commentary/uncertainties.md. Mechanical roles are documented despite unresolved game meanings.

Continue redirect with prior leads L-R1-001 residual byte, L-R1-002 progression and L-R1-003 data subdivision. Commentary adds no unsupported classification proposal. No toolkit issues found (analysis/toolkit-issues.md). No archive/publish/deletion/cleanup performed.
