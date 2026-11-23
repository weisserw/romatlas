# Round 2 - disassembly phase handoff

Status: FINISHED (single pass; easy gains exhausted). Classification incomplete: 149 bytes unknown.

## Scope
- Input: maps/round01-final.json (sha256 b4a8fa61...), state restored from analysis/state.json; baseline untouched.
- Applied the ready redirect leads L-R1R-01..10 after independent re-verification, then screened every residual gap.
- Workspace tool fix (prerequisite): analysis/tools/closure.py forced_branch() now has the r01-a1 closure2 fix.
  The fixed closure finds no new code outside the classified Code.

## Output checkpoint
- maps/round02-final.json sha256 53e08b7ba4231a38a737bbb86021112b6a9e2d287ea74e08841aad195b565bfa;
  generated/round02-final; analysis/round02-final-verification.json: ok, byte_identical true.
- analysis/state.json (exported to the same sha) and maps/current.json both match round02-final.
- Intermediate checkpoint: maps/round02-leads.json (70848b6d..., 96.30%, byte-identical).

ROM classification: 99.89% (130939 / 131088 bytes classified; 149 bytes unknown).

| Bank | Code | Data | Unknown |
|---|---|---|---|
| header | 0 | 16 | 0 |
| 0 | 11568 | 4785 | 31 |
| 1 | 651 | 15733 | 0 |
| 2 | 1274 | 15102 | 8 |
| 3 | 234 | 16150 | 0 |
| 4 | 103 | 16281 | 0 |
| 5 | 675 | 15709 | 0 |
| 6 | 1813 | 14571 | 0 |
| 7 ($C000) | 9659 | 6615 | 110 |

## Lead outcomes
- L-R1R-01 CODE: verified/accepted. 1,288 B code plus the correction at 0xC01E-0xC020 (Data to Code). The $800E JMP $BBFB
  fetch is observed in the boot trace. analysis/tools/r02_verify.py checked all 25 ranges: official opcodes, no BRK,
  each range ends in RTS/RTI/JMP or flows into code, and there is no overlap. Sub-runs are Observed only where the
  boot or surf-end fetch files list every instruction start. Surf-end runs are flagged artificial. The rest is Static 0.8.
- L-R1R-02..10 (BUSTAB, DISPATCH+MENU, SOUND, TABLES, MSPR5, GFX2, SCR3, B1, SURF6+STRIP6): verified/accepted as Data.
  Only still-unknown bytes were annotated, so existing classifications are preserved. No data byte lies over code.
  The TABLES spot checks were visibly tabular. Total 15,192 unique bytes.
- L-R1R-11 (bank 6 course scripts): superseded for classification. The round-2 gap screen
  (analysis/tools/r02_screen.py + r02_gapdata.py) classified all bank 6 residuals as generic Data. A manual
  record-pattern check covered $8FD4, $9E16 and $9E5F. The course command format is still untabulated, but that is a
  semantic question only.
- Residual gap screen overall: 4,697 B generic Data (Static 0.75). A gap was accepted when it had no control-flow
  reference from classified code in the effective bank, it had no viable instruction chain (or only <=3-instruction
  incidental ones), and any preceding code terminated. The bank-7 references into $8000-$BFFF all resolve to bank-0 code.
- L-R1-01: still blocked by TK-04.
- Rejected/kept unknown: the explicit list in r02_gapdata.py EXCL, with reasons.

## Leads produced
- L-R2-01 (analysis/leads.json): 149 residual bytes in 18 gaps.
  - Unreferenced code-like fragments: bank 0 $8229, $8A85, $8CED, $87CB, $9B46, $BEA3; bank 7 $D7B9, $DEBF, $EEB5,
    $F072, $F14A, $F2BA.
  - Bank 2 $87DF: 8 zero bytes after non-terminating code.
  - Bank 7 $F883 (45) and $FFDF-$FFF9 (27): look like bitmap/data but contain 5-6 instruction chains.
  - Bank 0 $8008: ambiguous entry.
  - TK-04 bytes.
  - Next step: search for callers via pointers or split immediates, or apply a dead-code policy.

## Gameplay
- S13 compete-some was visited naturally, reaching COMPETE IN WHICH EVENTS and then NEXT EVENT: FOOT BAG. The route is
  in analysis/scenarios.json. No fetches landed on non-Code bytes.
- All milestones are now visited except S12 (attempted earlier). The remaining unknowns are unreferenced, so more play
  is not expected to help.

## Unresolved / deferred
- L-R2-01 (above). TK-04 is a toolkit limitation.
- Commentary: re-run commentary for the changed ranges (many targets already have names in r01-a1 notes).

## Toolkit
- No new toolkit issues found. TK-01..05 are unchanged (analysis/toolkit-issues.md).

## Disk
- About 3.0G free. Scratch was cleaned of round-2 temporaries.
