# Round03 disassembly handoff

## Round03 disassembly complete — classification full; editorial gates pending
ROM classification: 100.00% (163856 / 163856 bytes classified; 0 bytes unknown).

Accepted106 Static bytes in24 regions:78 Code,28 Data. R02-L1/L2 verified; exact independent review analysis/round03-review/review.md, fresh configured decodes and independent-audit.json. All prior map records byte-for-byte identical. Stored reset templates are structurally code; actual INC changes upper banks to14/15 and nextFFF9 fetch isFF, so no working reset/JMP flow claimed. Correction: postreset lower slots are selected pair[b-1,b], not necessarily0/1. Vector Data annotations correctly omit code-only views.

| Physical section | Code | Other classified | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG 00 | 3931 | 4261 | 0 |
| PRG 01 | 2978 | 5214 | 0 |
| PRG 02 | 0 | 8192 | 0 |
| PRG 03 | 7 | 8185 | 0 |
| PRG 04 | 6970 | 1222 | 0 |
| PRG 05 | 3664 | 4528 | 0 |
| PRG 06 | 0 | 8192 | 0 |
| PRG 07 | 7 | 8185 | 0 |
| PRG 08 | 0 | 8192 | 0 |
| PRG 09 | 7 | 8185 | 0 |
| PRG 10 | 0 | 8192 | 0 |
| PRG 11 | 7 | 8185 | 0 |
| PRG 12 | 0 | 8192 | 0 |
| PRG 13 | 7 | 8185 | 0 |
| PRG 14 | 7284 | 908 | 0 |
| PRG 15 | 0 | 8192 | 0 |
| CHR | 0 | 32768 | 0 |

Exact kinds/annotation counts: analysis/round03-counts.json. 1135/1135 regions have evidence. Valid saved analysis/state.json/maps/current.json SHAca0e8adc5134a79e32469916d8bf67dc033e26142cf3363b41abd5f4ed0644c0; fresh checkpoint maps/round03-disassembly-accepted.json, generated/round03-disassembly-accepted and analysis/round03-disassembly-accepted-verification.json byte_identical=true. ROM unchanged SHAd72122bb27376997493440c5a693eab566cb96e7150d5c059e2efaeadefeb1e1. An initial annotation request supplied code-only views to Data and was rejected atomically; no saved changes then. Premature checkpoint round03-disassembly preserves unchanged old map SHA93c0081fcdf9d4b49643f903e12e656fc94e073182258da7701ce4ee7ea82be1 and is NOT the accepted output; preserved rather than deleted. Corrected accepted checkpoint above is authoritative.

No new execution; last consolidated6033 physical opcode starts remains separate from full structural classification. Saved broad gameplay scenarios unchanged; no discriminating new replay needed. All new roles Static, no natural reachability claim. Existing commentary/source/game.asm unchanged SHAcc6041f9e700c2fd54aee479175e5c9706f977cc648c29354e7cdde6a92f5283, covering only prior checkpoint. Required next separate phase is full updated-source commentary preserving/carrying old work; final editorial audit and final byte-identical verification remain pending. No ROM completion claim.

Toolkit: existing NES-001 confirmed documentation/API mismatch; zero new issues, zero suspected or missing features, no blockers; analysis/toolkit-issues.md. Code-only view rejection was documented behavior, not a toolkit defect. No toolkit edits, archive, cleanup/delete, publishing or phase launch. No unknown residual/rejection to redirect. Worker writes finished.
