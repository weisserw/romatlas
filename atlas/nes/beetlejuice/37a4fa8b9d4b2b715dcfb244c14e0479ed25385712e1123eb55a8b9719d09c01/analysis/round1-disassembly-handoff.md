# Round 1 disassembly handoff

Incomplete. ROM classification: 19.15% (25107 / 131088 bytes classified; 105981 bytes unknown).

Final verified checkpoint: maps/pass-001-final.json; generated/pass-001-final/game.asm; analysis/pass-001-final-verification.json. Map SHA256 b4cbd4690d6850d5155f0bc94dafcf48607ac1fab283289ff2d6c080043e9189. Internal state and maps/current.json match. Input remains original SHA256. Baseline and intermediate checkpoints preserved.

22647 code bytes, 2460 data bytes including 16-byte iNES header. All classified regions carry evidence/comments. Editorial readability is pending commentary. Observed code 11468 bytes; static code 11179 bytes. Runtime aggregate: 6591 accepted contextual opcode roots, 7176 contextual edges, no aggregate drops; capacity0 deliberately discards all detailed trace events. Aggregate includes artificial experiments; this is not whole-game execution coverage. Exact unknown complement is round1-counts.json.

| 8 KiB PRG bank | Code | Data | Unknown |
|---:|---:|---:|---:|
| 0 | 2217 | 20 | 5955 |
| 1 | 2295 | 10 | 5887 |
| 2 | 1109 | 47 | 7036 |
| 3 | 189 | 1062 | 6941 |
| 4 | 1571 | 18 | 6603 |
| 5 | 0 | 143 | 8049 |
| 6 | 0 | 176 | 8016 |
| 7 | 200 | 209 | 7783 |
| 8 | 438 | 61 | 7693 |
| 9 | 139 | 44 | 8009 |
| 10 | 76 | 2 | 8114 |
| 11 | 1654 | 261 | 6277 |
| 12 | 3459 | 56 | 4677 |
| 13 | 2738 | 20 | 5434 |
| 14 | 3959 | 52 | 4181 |
| 15 | 2603 | 263 | 5326 |

## Structural basis and rejected candidate

Validated NMI/reset/IRQ vectors in each AxROM bank. IRQ handler $FFAC reconstructs stacked PC minus1, consumes exactly two inline bank/offset bytes, masks first byte bit7 for tail-call semantics, changes mapper using $FB74,Y and invokes $8000 dispatcher; return restores caller bank. Bank-start JMP($0000) dispatcher has homogeneous following JMP stubs ending physically $00FD/$8079/$10082/$18052 (78/34/37/21 stubs). Consumer semantics, exact repeated instruction boundaries, endpoint change and coherent targets establish dispatch code role independently of natural selector reachability. Stub roots exclude targets below $8000. Three bounded waves followed supported direct JMP/branch chains and direct JSR target entry contracts in the same effective bank; calls/mapper writes remain conservative propagation barriers. No assumption that pointer values prove execution. Static code remains Static even if descended from observed roots.

Bank-aware candidate listings: round1-startup-inspection.json, round1-gameplay-new-inspection.json, round1-static-stubs-inspection.json; exact calls/responses in round1-rpc.jsonl. Accepted observed groups require an accepted opcode root for every instruction start in the same bank context. Inspection rejected false fallthrough $192D–$193A after LDX #$0A/BNE; accepted code only $1929–$192C. $192D–$1940 is generic data (indexed consumer at $1945) with larger table extent unresolved. Discovery does not understand this constant branch, so reuse barrier in future. All known accepted BRKs have two-byte Data operands, preventing accidental instruction discovery across inline dispatch data. Uniform zero/FF runs were bounded at exact byte changes and checked for code conflicts; generic Data does not claim original fill purpose.

## Gameplay and next research

Normal title/showtime/map/side-view room3; inventory and jumping/movement verified. active1.snapshot is preferred clean snapshot, frame1232, banks12–15, PC$8E14 (mid instruction), stack$26; finish one operation before register edits. Restore inputs explicitly. Full route and attempted shortcuts in scenarios.json; gameplay.md records external sources and version caveat (online level-select code belongs to Game Boy).

L-R1-ROOM: three artificial room9 variants ($55=9 plus PC$8D23/stack$26; PC$8D1E alternative; $0670=9 plus PC$8D23) kept rendered house despite RAM edit and actual execution. Rejected only as successful room visits, not as proof later states unavailable. $8D23 entry is reached from ordinary transition code, but NMI/coroutine scheduling and dependent initialization are unresolved; do not repeat same edits. Practical redirect: inspect $BB2A–$BB3D transition (physical $1BB3A etc), $8110 loader, dispatch at $DAAA/$DAE1, and saved task/stack contract to identify safe trigger/scheduler state. Later levels, distinct overhead mechanics, bosses, Afterlife and ending remain unvisited; no arbitrary RAM brute force.

L-R1-STATIC: routine prefixes/direct callees classified, but continuation after bank-neutral JSR and BRK is still underexplored. Prove transitive no-mapper-effect/BRK return contract for selected routines and resume connected static graphs. Known two-inline-byte barriers and exact effective banks must be retained.

L-R1-GFX: bank1 $92DE loader has bounded selector checks ($36/$24) and PPU output; tables near $9444–$9644 lead to compressed/high-entropy payload in physical $9800–$F900. Inspect actual consumer and independent payload boundaries to classify supported batch; do not infer all high-entropy bytes are data solely from entropy. The largest easy uniform tails are already classified.

This is easy-gain handoff, not terminal investigation exhaustion. Redirect should resolve these concrete prerequisites and connected data/code batches before another pass. No initial leads existed; IDs above preserve outcomes. Toolkit: one confirmed documentation/API mismatch NES-001, no blockers; analysis/toolkit-issues.md. No toolkit edits.
