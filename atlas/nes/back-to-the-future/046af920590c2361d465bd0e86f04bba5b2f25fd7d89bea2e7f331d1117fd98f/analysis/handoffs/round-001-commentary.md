# Round 001 commentary handoff

Current-source readability is complete. Redirect may proceed sequentially. Final editorial/archive acceptance remains pending; no final audit was run.

Checkpoint: `round-001-commentary-final`, recorded in `analysis/round-001-commentary-final-checkpoint.json`. Structural baseline remains `round-001-disassembly-final`. Internal state, portable map and both frozen commentary baseline files retain SHA256 `5176915eb58cb7ad441494c0df2f162baffc5b3e64b05100545c82f4c300b94c`.

Scope: all current classified source, all accepted data/CHR boundaries and all nineteen unknown intervals were read. `commentary/review.json` preserves the entire 112-record physical scaffold, now with reviewed summaries/source locations and 226 meaningful routine units in fixed PRG context [0,1,2,3]. `commentary/source/game.asm` supplies purpose/state/algorithm comments and 778 control-flow labels; `symbols.inc` declares 440 hardware, RAM, table and encoding symbols. Understood fields also use base-plus-offset operands. Unresolved game meanings retain mechanical descriptions or explicit raw scratch addresses. No current-source readability ranges are deferred.

Examples:

- Reset/main transitions and route history: `source/game.asm:28`, `RunRouteSelection`, `CommitRouteChoice`, `AdvanceRouteHistory` explain the selector dispatch, retry checkpoint and mixed $04/address history format.
- Player: `ResolvePlayerTileCollision` at line 815 describes corner order and property dispatch; `UpdatePlayerAnimationAndSprites` explains cursor/duration animation and signed vertical offset. Named fields distinguish candidate/world positions, camera history and sampled PPU tiles.
- Rendering: `MergeMetatileAttributes` at line 1900 explains preservation of unrelated two-bit quadrants. RLE/text consumers identify CHR-backed packets separately from visual tile art; raster routines preserve deliberate dummy reads/NOP timing.
- Object pool: `RunStreetObjectPool` at line 3849 documents ten parallel-array slots, script/behavior dispatch, signed fixed-point integration and OAM ownership. Classified commands explain per-sprite tile/attribute streams, child spawning, motion and wait payloads.
- Bonus/driving: cafe line 4870, school line 5818, dance line 6601 and driving line 7344 explain actual control/state mechanics. Driving comments explicitly retain the black-output/prerequisite uncertainty rather than claiming a working natural visit.
- Audio: `TickAudioEngine` at line 8639 explains four music plus three effect voices, sequence/pattern/instrument data, tie/duration behavior and effect channel priority.

Unresolved meanings and concrete structural questions are in `commentary/uncertainties.md`. Strongest redirect opportunities: player handlers/tables around physical $0b8c..$0ec1; street-object handlers and mixed $32fd..$39d3; shared bonus interpreter handler interval $5a0a..$625b; car interpreter gaps $559c..$55cb and $573a..$583f. Inspect static consumer/stack/control contracts and finite payload domains to split supported subranges. Pointer values do not establish entire intervals. Title's installed CPU $82ca handler remains in unknown $02da..$030e. Existing exact complement remains `analysis/round-001/unknown-ranges.json`.

Other limitations: original table segmentation, artistic tile meanings, tune names, some mode-shared scratch and perceived audio envelopes are not recovered. Generic accepted data has local boundary/inspection notes. Driving initialization and natural ending transition require targeted replay; direct ending entry is not completion proof. Prior artificial bonus-countdown observations do not establish a new game bug: the decrement helper checks zero, and ASCII timer versus unpacked decimal countdown contracts matter.

Verification: fresh `commentary.py verify` passed byte-identically using `/root/romatlas/target/release/romatlas`; preserved report `analysis/round-001-commentary-final-verification.json`. `commentary.py read` refreshed `commentary/reading-after.asm`. Final source SHA256: game.asm `0ba36906e5252fb5bfe7928e22fcd728d214afd93b9a745037904ae194073005`; symbols.inc `d27533cf7ed145918b0ab40044c65b171339a06f9e908de9fbe168086fca68b8`. Input ROM SHA256 remains workspace identity `046af920590c2361d465bd0e86f04bba5b2f25fd7d89bea2e7f331d1117fd98f`.

Classification is unchanged: 59496/65552 bytes (90.76%) classified; 6056 unknown. Source readability and classification are separate. Final editorial audit stays pending. Confirmed documentation issue NES-001 remains in `analysis/toolkit-issues.md`; no new toolkit issue was found. No toolkit edits, new runtime experiments, redirect, archive, publication or external transfer occurred. All source/assets remain local.
