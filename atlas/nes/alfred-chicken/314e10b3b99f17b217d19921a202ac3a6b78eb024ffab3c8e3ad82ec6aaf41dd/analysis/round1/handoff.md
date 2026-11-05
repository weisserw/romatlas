# Round 1 disassembly handoff — phase complete, ROM incomplete

ROM classification: 33.69% (44157 / 131088 bytes classified; 86931 bytes unknown).
Code27213; data16931; text13. Every count is from validated internal state, not baseline/export fallbacks. Exact209-range unknown complement: `analysis/round1/unknown-complement.json`; full accounting `analysis/round1/final-accounting.json`.

Checkpoint `pass-004`: `analysis/state.json`, `maps/current.json`, frozen `maps/pass-004.json`, generated `generated/pass-004/game.asm`, byte-identical `analysis/pass-004-verification.json` (ok=true, byte_identical=true). Final map SHA2562889b50152817b3708815858dae2d84ce5029fc34f6184fce5dce659c64327bc. Input ROM SHA256 unchanged314e10b3b99f17b217d19921a202ac3a6b78eb024ffab3c8e3ad82ec6aaf41dd. Baseline and passes001–003 preserved. Full source commentary/readability phase not performed.

Natural reset/title/Stage1 cheese movement and jump visited. Source-backed pre-initialization RAM$93 selector edits visited stages2 alphabet blocks,3 screw platforms,4 chains/spikes,5 green machinery/collision, and sixth descriptor spike room. These are artificial-state observations. Final-balloon RAM contract (consumer physical14D22..14D33) exposed aerial bonus from stage1 and shooting flight from stage5; movement and B projectiles exercised. Later480-frame time advance plus90 traced frames showed enemy projectiles, without identifiable boss or ending. All exact presses/releases, budgets, patches, frames, effective banks, captures and durable snapshots are in `analysis/scenarios.json`, `analysis/gameplay.md`, and per-scenario `analysis/round1/` files. Do not call these natural later-stage reachability or full gameplay compatibility.

Structural evidence: vector roots and actual-context recursive decoding; direct JSR entries and UxROM fixed-window write continuations (`static-call-root-proofs.json`, `staticcall2-root-proofs.json`); six descriptor headers and pointer consumer; four24-word PPU variant tables under X=$60..$77 (`ppu-dispatch-proof.json`). All16 distinct target contracts coherently select PPU address/write handlers; newly supported360codebytes and192tablebytes accepted. Patterned generic non-code slices reviewed in `patterned-data-review.txt`; original asset schemas/whole-object extents remain unclaimed. All annotations preserve physical offsets, views, bank context and Static versus Observed provenance. Artificial reachability is separately identified by scenario evidence. Initial links now cite immutable batch1 copies. Trace details intentionally bounded/disabled; aggregate_dropped=0 for all saved scenario traces (`trace-integrity.json`).

Easy-gain gate: broad representative systems and supported stage/balloon shortcuts tried; startup repeated only to repair input schema, not as a coverage substitute. Runtime new gains declined to399 then162bytes in repeated shooting flight, without a new subsystem. Two bounded direct-callee/fixed-window passes declined to111bytes. The concrete PPU dispatch slice was investigated and accepted as a connected batch. Further useful work requires selector/return-contract/resource or boss-wave research, retained as redirect questions, rather than more long repeated flight. This is ordinary single-pass exhaustion, not terminal research exhaustion and not a full-ROM completion claim. No terminal rejection review is claimed/needed at this easy-gain handoff.

Next: separate full current-source commentary, then redirect using `analysis/leads.json`. Prioritize mixed resource boundaries (notably bank0 compressed intervals and bank1/2 graphics), remaining indirect actor dispatch contracts, and boss/wave/ending conditions from the final-later snapshot. No live server required: a fresh configured-binary serve needs map.load and emulator.start/snapshot.import; named snapshots are not persistent. `client.py`, `game-batch.py`, `review-batch.py` provide bounded direct-subprocess recipes. Old file-RPC transport attempts are historical and should not be reused.

Toolkit report `analysis/toolkit-issues.md`: one nonblocking confirmed documentation/API validation bug NES-001 (shared documented button/pressed schema silently clears NES input; workaround full buttons arrays). No other toolkit issues found in this pass; no toolkit edits. No archive, publishing, deletion or cleanup performed. At handoff free space8.9GiB, scratch17MiB, round1 evidence373MiB.

| Physical PRG8K bank | Code | Data/text | Classified | Unknown |
|---|---:|---:|---:|---:|
| header | 0 | 16 | 16 | 0 |
| 0 | 301 | 5176 | 5477 | 2715 |
| 1 | 215 | 2993 | 3208 | 4984 |
| 2 | 911 | 76 | 987 | 7205 |
| 3 | 243 | 0 | 243 | 7949 |
| 4 | 424 | 0 | 424 | 7768 |
| 5 | 2289 | 192 | 2481 | 5711 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 137 | 152 | 289 | 7903 |
| 8 | 0 | 128 | 128 | 8064 |
| 9 | 802 | 0 | 802 | 7390 |
| 10 | 3971 | 0 | 3971 | 4221 |
| 11 | 987 | 13 | 1000 | 7192 |
| 12 | 3318 | 0 | 3318 | 4874 |
| 13 | 1975 | 0 | 1975 | 6217 |
| 14 | 6070 | 0 | 6070 | 2122 |
| 15 | 5570 | 6 | 5576 | 2616 |
