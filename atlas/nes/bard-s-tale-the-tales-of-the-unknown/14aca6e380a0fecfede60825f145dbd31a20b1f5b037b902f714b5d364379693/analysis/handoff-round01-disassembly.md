# Round 01 disassembly handoff

ROM classification: 21.57% (28276 / 131088 bytes classified; 102812 bytes unknown).

Pass complete; ROM incomplete. Coordinator: print the exact classification line before starting commentary. Next: full current-source commentary, then redirect; this is not a terminal exhaustion claim.

- Code 17920, data 10356, unknown 102812. 704 regions, all evidence annotated; 692 Code regions all have explicit bank-aware views. Observed classification 15111 bytes includes artificial scenarios; Static 13165 bytes includes data. Structural classification and natural reachability remain separate.
- State: `analysis/state.json`; portable map: `maps/current.json`. Fresh verified checkpoint `maps/round01-disassembly.json`, generated source `generated/round01-disassembly`; report `analysis/round01-disassembly-verification.json` confirms byte identity. Input SHA256 unchanged. Baseline/mid checkpoints retained.
- Per-bank counts: `analysis/progress.md`, `analysis/round01/counts.json`. Exact 207-gap unknown complement: `analysis/round01/unknown-complement.json`. Durable execution union 7423 ROM opcode offsets and 855 non-ROM backing opcode identities; incomplete scenario coverage, separate from classification. Recorded trace summaries show zero aggregate drops; expected detail drops with capacity 0.

Supported batch: observed title/Guild/character/party/field/battle code; static reset/NMI/audio/menu/name/character/movement/battle alternatives; JSR callee destinations under pre-call context; safe direct indexed RAM-write continuations; seven 82-byte bank descriptors; 96-byte area-property noncode block; 9664-byte mixed dialogue/pointer/zero-suffix block. Primary artifacts: `round01/observed-decoded*.json`, `static2-inspected.json`, `callee-inspected.json`, `ramwrite-inspected.json`, `final-static-inspected.json`, `work-init-decode.json`, `dialogue-review.md`.

Correction worth retaining: seven 10-byte bank tails initially left unknown are purposeful dual code/vector reset bridges. CPU BFF6 under that lower bank pair executes SEI/INC FFFF, resetting MMC1 mode 3 while preserving selected lower pair; three BFF6 vector words also decode as INC zp,X and fall through into fixed C000 reset. Combined mapper/control-flow/vector constraints support structure without observed natural entry. `round01/reset-bridges.json`; don't inherit earlier unresolved-tail judgment as rejection.

Gameplay evidence and limits:

- Correct input schema is `{port:1,buttons:[...]}`; release with `buttons:[]` and execute a release interval before another edge. Earlier generic button/pressed requests had no input effect, preserved as idle routes.
- Valid Skara Brae field with Warrior AA follows Guild creation/addition/Leave Guild. Artificial cursor/name shortcuts make the subsequent lineage artificial. Reusable `round01/field-ready.json`; primary image `field-ready.png`.
- Early directPC C108 area1/15 attempts omitted PPU-disable and produced malformed Guild tiles. This rejects only that incomplete recipe. Source `leave-guild-decode.json` and `mapperinit-decode.json` establish EC93 PPU-disable followed by stack reset/C108. Corrected area bridges construct RTS to C108 via RAM01FE=07/01FF=C1, PC EC93/stackFD/P24, area0202 and0203=FF; see scripts/RPC logs.
- Corrected Cellar and M-Tower label/minimap initialization and Up movement observed: `transition1.json`, `transition15.json`, matching init/movement images. Darkness/late HUD drawing not fully understood. Castle 6 was externally interrupted during fade; no visit claimed.
- RAM $0340 threshold compares RNG $5A and sets encounter state $0200=3. FF plus Up produced Sorcerer groups; Fight, AA Attack and target selection completed. Enemy Arc Fire narration and AA HP12/action observed in latest `round01/combat-turn.json/.png`. Last 90 frames of narration yielded 42 new code bytes; stop repeating text/menu loops. Damage/victory/defeat not claimed.
- Final-area coordinate hypothesis x11,y14 from external ASCII map did not trigger Mangar/ending from11,13 facing north; `mangar-coordinate.json/.png`. No coordinate brute force. Ending/credits unvisited; no inferred completion.

Concrete redirect questions (details/status/retry conditions in `analysis/leads.json`):

1. Recover exact area event record/coordinate/selector contract from C816/C877/C8A8 consumers; align final-area boss/ending events before another shortcut. Reuse transition15 snapshot. Prior coordinate failure does not reject boss/event structure.
2. Prove ROM origins of PRG-RAM72D7/7462/771F handlers through upload pointer/length contracts; RAM execution alone is insufficient. Reset 128-byte copy is only a partial upload fact.
3. Follow graphics/decompression consumers D42D/EB1E and descriptor slots to separate mixed early-bank code and noncode intervals. Generic data is acceptable after bounded conflict/boundary inspection; don't require full semantics or classify whole mixed banks.
4. Resolve useful indirect targets/call return bank contracts; current static discovery stops conservatively. Safe indexed writes whose base + 255 < $8000 preserve PRG context and already yielded a connected accepted batch.

No toolkit issues found; `analysis/toolkit-issues.md`. No toolkit edits, archive, publication, cleanup or ROM mutation. All evidence retained locally. Commentary/readability/editorial review has not run; structural counts and byte identity do not establish those gates. Historical active notes retained at `round01/progress-active-history.md`.
