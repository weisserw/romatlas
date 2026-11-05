# Round2 disassembly — final handoff, writer released

## Round2 authoritative final checkpoint — pass-002-reviewed

ROM classification: 98.88% (259219 / 262160 bytes classified; 2941 bytes unknown).

Final connected wrapper review accepted15 more Static bytes at physical1F66F..1F67E (CPU F65F). Complete save-latch/select3/callA400/restore wrapper has a balanced stack contract, demonstrated separately from unchanged-bank traversal. See round2-disassembly/wrapper.py. Previous pass-002-handoff remains preserved as an intermediate checkpoint. Final total added73727 bytes (10540 Code/63187 Data); totals27306 Code/231913 non-code. Map/state SHA256 c729ce4e85bf60f52a390ee488c35c291e915a0fa27ab927b23d8e0501adb176. Fresh maps/pass-002-reviewed.json, generated/pass-002-reviewed and analysis/pass-002-reviewed-verification.json validate and rebuild byte-identically. ROM input unchanged; existing commentary/source untouched.

All six ready leads verified; exact final per-bank counts and2941-byte complement saved. No terminal exhaustion claim. Full-source commentary then redirect remains appropriate; closed F65F wrapper is accepted, no longer a deferred candidate. No toolkit issues found. Writer released; no live server operations.

| Bank | Code | Data/header/image | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG 00 | 1674 | 6515 | 3 |
| PRG 01 | 173 | 8004 | 15 |
| PRG 02 | 0 | 8192 | 0 |
| PRG 03 | 686 | 7505 | 1 |
| PRG 04 | 1674 | 6515 | 3 |
| PRG 05 | 173 | 8004 | 15 |
| PRG 06 | 0 | 8192 | 0 |
| PRG 07 | 686 | 7505 | 1 |
| PRG 08 | 626 | 7492 | 74 |
| PRG 09 | 78 | 8114 | 0 |
| PRG 10 | 699 | 7361 | 132 |
| PRG 11 | 3500 | 4692 | 0 |
| PRG 12 | 4129 | 4050 | 13 |
| PRG 13 | 2926 | 5233 | 33 |
| PRG 14 | 5199 | 2338 | 655 |
| PRG 15 | 5083 | 1113 | 1996 |
| CHR 00 | 0 | 8192 | 0 |
| CHR 01 | 0 | 8192 | 0 |
| CHR 02 | 0 | 8192 | 0 |
| CHR 03 | 0 | 8192 | 0 |
| CHR 04 | 0 | 8192 | 0 |
| CHR 05 | 0 | 8192 | 0 |
| CHR 06 | 0 | 8192 | 0 |
| CHR 07 | 0 | 8192 | 0 |
| CHR 08 | 0 | 8192 | 0 |
| CHR 09 | 0 | 8192 | 0 |
| CHR 10 | 0 | 8192 | 0 |
| CHR 11 | 0 | 8192 | 0 |
| CHR 12 | 0 | 8192 | 0 |
| CHR 13 | 0 | 8192 | 0 |
| CHR 14 | 0 | 8192 | 0 |
| CHR 15 | 0 | 8192 | 0 |

Evidence and lead outcomes: reviewed original73605-byte combined package plus29Code/93Data connected bytes. Independent audits: round2-disassembly/review.json, extra-review.json, closure-candidates.json, wrapper.py. Every classified byte carries structural evidence. Static is not execution; Observed matches actual accepted artificial fetch contexts. Old commentary remains intact; new generated source requires full current-source commentary.

Retained residuals: stack-sensitive fixed/actor/sprite helpers (notably1EE5A PLA with incomplete entry convention), bank8 physical10139..10183 and bank10 composite14049..1434A continuations, isolated returns/placeholders, remaining late-copy fragments with ROM/branch dependency mismatches. Direct-control closure tested; no generic data inferred from absence of fetches. Redirect should investigate localized entry/stack/bank contracts after commentary, rather than unchanged runtime replay. Current scenarios already cover documented artificial finals, trophy/championship terminal and Start exit; free-throw/dunk actor initialization remains unresolved. No separate credits/natural tournament claim.

Per-bank totals above; final-counts.json and residual.json are exact machine-readable records. ROM incomplete and final editorial gate pending. No toolkit edits, cleanup, publishing or archival. [Toolkit issues](toolkit-issues.md): no toolkit issues found.
