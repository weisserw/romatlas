# Round 2 — disassembly handoff

- Scope: apply and verify the combined redirect batch (r1-a1 + r1-a2 proposals), merge the artificial-scenario
  observed code from both attempts, review lead L-005, then save, export and checkpoint. No new gameplay was run (see Deferred).
- Input checkpoint: pass-001 (98.19%, 4,750 B unknown).
- Output checkpoint: **pass-002**. Files: maps/pass-002.json and generated/pass-002. Verification
  [analysis/pass-002-verification.json](../pass-002-verification.json): ok=true, byte_identical=true,
  map_sha256 50c836beac3c1cf645bbdfcb60b216dc5ee2e68c972bad7ec58e4330558dc6e3.
- Internal store: analysis/state.json is identical to maps/current.json. It has 2,182 regions, and map.validate returns ok.
- Tools (analysis/tools/): r2_verify.py checks proposals. r2_build.py builds the edit list scratch/r2_edits.json
  (49 removals, 466 annotations). r2_apply.py applies the edits through map.remove/map.annotate, then runs validate/save/export.
  r2_l005.py does the boundary review, and r2_dis.py is a listing helper.

## Completion

ROM classification: 100.00% (262158 / 262160 bytes classified; 2 bytes unknown).
The unrounded value is 99.9992%. **Status: incomplete.** File 0x1C5AE-0x1C5AF is still unknown.

| Segment | Code | Data | Text | Image | Unknown |
|---|---|---|---|---|---|
| header | 0 | 16 | 0 | 0 | 0 |
| PRG 0 | 2241 | 5951 | 0 | 0 | 0 |
| PRG 1 | 0 | 8192 | 0 | 0 | 0 |
| PRG 2 | 2926 | 4146 | 1120 | 0 | 0 |
| PRG 3 | 2999 | 5193 | 0 | 0 | 0 |
| PRG 4 | 7407 | 785 | 0 | 0 | 0 |
| PRG 5 | 1524 | 6502 | 166 | 0 | 0 |
| PRG 6 | 245 | 7947 | 0 | 0 | 0 |
| PRG 7 | 1400 | 6792 | 0 | 0 | 0 |
| PRG 8 | 2040 | 6084 | 68 | 0 | 0 |
| PRG 9 | 0 | 8192 | 0 | 0 | 0 |
| PRG 10 | 0 | 8192 | 0 | 0 | 0 |
| PRG 11 | 0 | 8192 | 0 | 0 | 0 |
| PRG 12 | 6949 | 1243 | 0 | 0 | 0 |
| PRG 13 | 6772 | 1420 | 0 | 0 | 0 |
| PRG 14 | 3125 | 5065 | 0 | 0 | 2 |
| PRG 15 | 3495 | 4697 | 0 | 0 | 0 |
| CHR | 0 | 0 | 0 | 131072 | 0 |

Code totals 41,123 B: 28,236 B Observed provenance and 12,887 B Static.
Execution coverage is a separate metric. Unique opcode offsets come from pass-001 traces plus artificial r1-a1/r1-a2 traces.
No observed instruction byte from any of the three trace sets lies outside Code. The bank-8 crash trail is still excluded, as in pass-001.

## Verification of the proposal batch (all 33 groups accepted, 0 rejected)
- Every range was checked with r2_verify.py. Each range lies in one bank and covers only bytes that were unknown in pass-001.
  The one exception is C-R06, which replaces the 142-byte pass-001 Data region 0x1BCD9-0x1BD66.
  No range overlaps another group, and no data or text range contains a byte of any observed instruction.
- Code groups were re-decoded with bank windows assigned per group:
  - C-R06: bank 13 $A000, $BCB4-$BD56. It ends with JMP $EA8E at 0x1BD64. The tables at $BD57 follow.
  - C-R01: $C502. It falls into observed $C517.
  - C-K1X: $A000-$A02D. It ends with JMP $8116, followed by the tables it reads at $A02E/$A034.
  - C-B2: its 6 ranges end in RTS or land on observed instruction boundaries.
  - C-JOY: a $4016/$4017 reader ending in RTS, followed by $FF/$00 fill.
  - C-MISC: 4 ranges, each ending in JMP/RTS or falling into observed code.
  - None of these decodes contains an illegal opcode or a boundary conflict.
- T-CRED: range 0x0AF3D contains one $61 glyph inside the $40-$5A credits charset. It is accepted as Text.
- Confidence for medium-strength groups: D-SND15 0.6, D-SNDX 0.65, orphan code C-B2/C-MISC 0.75. Other proposals 0.8–0.85 (Static).

## Applied
- L-007, R-06 correction. Bank 13 $BCB4-$BD56 (0x1BCC4-0x1BD66, 163 B) is now Code.
  - The fetched instruction runs are Observed (artificial: r1-a2 tags k42kill/k42near/k42pos).
  - The two unfetched runs are Static: $BD37-$BD3E and $BD43-$BD47.
- Artificial observed code merge: 1,032 opcode starts were fetched only in r1-a1/r1-a2 traces and fell inside pass-001 Static code.
  - All of them were on static instruction boundaries with matching CPU addresses.
  - 48 Static regions were split into Observed runs (artificial-scenario evidence, tags listed in each region's evidence) and the remaining Static runs. The original evidence and views were kept.
  - Code from these traces is recorded as artificial execution; it is not natural reachability.
- Data/Text by group:
  - L-008: code, 374 B.
  - L-009: sound, 503 B.
  - L-010: cutscene streams/metasprites, 1,036 B.
  - L-011: bank 14/15 tables, 424 B.
  - L-012: R-02 tables, 731 B.
  - L-013: 236 B.
  - L-014: area banks 6/7/8, 1,072 B.
  - L-015: 68 B.
  - L-016: 145 B.
  - L-017: 159 B.
  - Total: 4,748 B of the 4,750 B unknown in pass-001.
- L-005 review, verified with no change:
  - Every code region that ends without RTS/RTI/JMP before non-code is a JSR to one of these:
    - $C4F3 (21, inline args)
    - $C588 (12, inline tables)
    - $8792 (1)
  - The one exception is bank 13 $AD63, an always-taken BNE after ORA #$0B.
  - Re-running r1-a1 obsreads.py showed that the remaining "Observed reads" data regions contain no fetched instruction.

## Unresolved
- 0x1C5AE-0x1C5AF (bank 14 $C59E, `lda $F5` between jump engine `jmp ($0000)` and HideUnusedSprites $C5A0).
  - No reference and no fetch in any trace.
  - The bank-2 jump-engine copy is followed by different code ($87A8), so it gives no layout constraint.
  - The r1-a2 rejection review is still sound. Kept unknown as new lead L-018 (low; reopen only with a new constraint).
- Natural reachability of the orphan/alternate-entry code (C-B2, C-MISC, C-JOY, C-R01) is not established. Its classification is structural only.
- Exact semantic domains of several tables are approximate (r1-a2 note on the banks 9/11 screen model). This does not affect classification.

## Deferred
- No further gameplay this pass. Only 2 bytes remain unknown, and every trace set (pass-001, r1-a1 bosses/game-over/weapons, r1-a2 kind 42) left them unfetched.
  More play would add execution coverage only, not classification. S5 boss area clears (idx 6/16/17) are still partial (L-003).
- Commentary: the new regions have generic Static evidence only. Commentary should give labels and roles to the new code (C-B2/C-JOY/C-MISC/C-R01/R-06) and data groups, and refresh the source from pass-002.

## Leads
- Consumed: L-007, L-008, L-009, L-010, L-011, L-012, L-013, L-014, L-015, L-016, L-017.
- Updated: L-005 (verified).
- Produced: L-018 ($C59E residual).
- See [analysis/leads.json](../leads.json).

## Toolkit
- No toolkit issues found this pass.
- TK-001..TK-003 are unchanged in [analysis/toolkit-issues.md](../toolkit-issues.md). There are no blockers.
