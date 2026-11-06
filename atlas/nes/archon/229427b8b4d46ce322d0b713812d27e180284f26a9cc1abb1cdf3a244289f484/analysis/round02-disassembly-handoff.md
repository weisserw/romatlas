# Round02 disassembly handoff

One pass completed from maps/pass-001-final.json. New accepted checkpoint: `maps/pass-002-final.json`, map SHA256 `309999c210ff7f476e6da91342d4cd4aab4a929fe8125ece4e8e7060d5f0c947`. `analysis/state.json` and `maps/current.json` are saved/validated. Fresh `generated/pass-002-final` rebuild is byte-identical; verification: `analysis/pass-002-final-verification.json`. Input ROM unchanged.

ROM classification: 99.99% (131078 / 131088 bytes classified; 10 bytes unknown).

Accepted 21,527 new bytes: 12,383 Code + 380 inline Data + 8,764 other Data. Total 23,122 Code / 107,956 Data. Fresh independent review checked 9,295 execution views, 14,644 decoded predecessor/inline edges, 171 call/context records for 96 new inline payloads and every non-inline Data envelope. No structural byte rejection; localized role wording was corrected. Existing generic Data was split to refine twenty $0300 actor graphics prefixes with no byte gain.

Review evidence: `analysis/disassembly/r02-review/findings.md`, `metrics.json`, `mechanical-summary.json`, `edge-summary.json`, `inline-check-summary.json`, `execution-metrics.json`, `fetch-conflict-audit.json`, `integrity.json`. Full proposals normalized in `normalized-annotations.json`; exact accepted definitions are in the current map. Progress/investigation/leads updated.

Commentary migration corrections to preserve: A6FB waits for nonzero input (A705 waits release); A33D clears 960 bytes; physical $006DA sequences alternate seven/eight bytes; $03C94..$03CA4 is two APU sets and $03CA4..$03CAC is script pointers/increment/count pairs; the 96-byte board marker envelope has unresolved internal subdivisions; DC69 has eight visible words but seven dispatched spell indices, with cancellation7 excluded. C317 copies $0300, whereas C381 copies $0800: only C317 proves the actor-prefix refinement extent. Source code/comments were not edited in this phase. Commentary/source/game.asm SHA256 remains `1c1a9ecb022eac6839a608c3b801e79d9ee27c503f9f0f0d7733621c51c3efe1`.

Ten Unknown bytes remain in seven intervals, ends exclusive: $00ABF..$00AC1, $00E87, $02481, $1DC0F, $1DC23, $1F08D..$1F090 and $1F9F7. Isolated zero/RTS/JMP bytes lack supported entry or non-code role; adjacency/legal decode does not settle them. This is incomplete, not terminal exhaustion. Next: full current-source commentary migration from the preserved prior commentary, then redirect reasoning review against the exact residuals. Final editorial audit remains pending.

Saved natural board and artificial arena/ending/Teleport routes were reused, avoiding unchanged gameplay. Composite execution is 4,686 ROM instruction starts / 9,753 bytes, zero Data conflicts; this is separate from structural classification and natural reachability. New regions retain Static provenance.

Toolkit: no new toolkit issues found; existing NES-001 is nonblocking, with complete replacement buttons array. See analysis/toolkit-issues.md. No toolkit changes, archive, publication, cleanup or deletion. The empty-label input normalization rejection was corrected and successful transactions restored through a validated annotation checkpoint; it was not a toolkit defect.
