# Handoff: round 2, phase 1 (disassembly)

ROM classification: 100.00% (393222 / 393232 bytes classified; 10 bytes unknown).

The figure is rounded. 10 bytes are still unknown, so the classification is **incomplete**.

## Checkpoint and verification
- Input: maps/round1-disassembly-b.json (95.44%, 17,912 unknown), restored from analysis/state.json.
- Output: maps/round2-disassembly.json (map sha256 8b43ff3e...), generated/round2-disassembly.
  The verification report (analysis/round2-disassembly-verification.json) shows ok, byte_identical true.
- analysis/state.json was saved and maps/current.json was exported. map.validate reports ok.
  input/game.nes is unchanged and the toolkit was not modified.
- Tools and records are in analysis/round2/: build_r2.py, apply_r2.py, verify_props.py,
  island_ctx.py, stats.py, regions.json, changed-ranges.json, verify-props.json,
  island-review.txt, walk2-r2.json and stats.md.

## Per-bank counts
| bank | size | code | data | padding | classified | unknown |
|---|---|---|---|---|---|---|
| header | 16 | 0 | 16 | 0 | 16 | 0 |
| prg0 | 16384 | 8603 | 928 | 6853 | 16384 | 0 |
| prg1 | 16384 | 238 | 15897 | 249 | 16384 | 0 |
| prg2 | 16384 | 9272 | 6938 | 169 | 16379 | 5 |
| prg3 | 16384 | 397 | 12856 | 3131 | 16384 | 0 |
| prg4 | 16384 | 6229 | 2004 | 8151 | 16384 | 0 |
| prg5 | 16384 | 520 | 12617 | 3247 | 16384 | 0 |
| prg6 | 16384 | 3098 | 346 | 12939 | 16383 | 1 |
| prg7 | 16384 | 1792 | 3808 | 10784 | 16384 | 0 |
| prg8 | 16384 | 1648 | 12402 | 2334 | 16384 | 0 |
| prg9 | 16384 | 1646 | 12402 | 2336 | 16384 | 0 |
| prg10 | 16384 | 180 | 9827 | 6377 | 16384 | 0 |
| prg11 | 16384 | 3869 | 1040 | 11473 | 16382 | 2 |
| prg12 | 16384 | 2508 | 5422 | 8454 | 16384 | 0 |
| prg13 | 16384 | 1465 | 10230 | 4689 | 16384 | 0 |
| prg14 | 16384 | 247 | 14297 | 1840 | 16384 | 0 |
| prg15 | 16384 | 14024 | 2086 | 272 | 16382 | 2 |
| chr | 131072 | 0 | 0 | 0 | 131072 | 0 |

## Lead outcomes
- **L-012 (verified):** I reran walk2.py and got the same result: +6,112 code bytes, with no clashes,
  invalid decodes or conflicts. I rechecked each table extent against its own constraint.
  - Result: 227 code regions, split at jump-table handler and JSR entry points, all Static with
    confidence 0.9.
  - The table regions add 629 new bytes, and the $E37C selector bytes add 11.
  - Labels added: NmiTaskJumpTable, PlayStateJumpTable, FrontEndScreenJumpTable,
    CpuInputStateJumpTable, CpuSideModeJumpTable.
- **L-013 (verified):** 104 of 106 ranges accepted, adding 1,994 bytes of generic data. Two
  ranges were handled differently:
  - Bank 15 $FA14-$FA22 was proposed as data. I rejected that and classified it as Code
    (confidence 0.75). It is an LDA #n / JSR $FA28 chain that falls into the observed code at
    $FA23, and the $F95B fragment calls it.
  - The $EFF1 item is superseded by the L-010 table.
- **L-014 (verified):** all 81 ranges accepted as generic data (7,744 bytes).
- **L-015 (verified):** 35 of 36 code fragments accepted as Static code (1,368 bytes, confidence 0.6).
  Reachability is unresolved: they are not labelled Observed and not claimed to be unused.
  I rejected bank 2 $DAC9-$DACC (PLA/TAY/PLA/RTS) because it has no matching prologue and no
  reference.
- **L-010 (verified):** the code region at 0x3EFFE is trimmed to 2 bytes, and $EFF0-$EFFF
  (0x3F000, 16 bytes) is now one Data table.
- **Residual review:** three additions.
  - Bank 4 $D174-$D17B and bank 6 $C845-$C849 are now Code (confidence 0.5). Both are sibling
    fragments that flow into an established RTS.
  - Bank 0 $8420 is a 1-byte Data filler. A JSR opcode there would overlap the observed
    instruction start at $8421.
- L-001, L-002, L-007 and L-008 are marked superseded because their content was applied through
  L-012 to L-015. New lead: L-016, the 10 residual bytes.

## Changed ranges (for commentary)
The complete list is in analysis/round2/changed-ranges.json: 478 new regions, plus 3 removed
regions and the L-010 trim. Bytes added per bank:
- bank 0: Code 655, Data 367
- bank 1: Data 1024
- bank 2: Code 3104, Data 1801
- bank 3: Data 1866
- bank 4: Code 897, Data 1464
- bank 5: Code 138, Data 410
- bank 6: Code 54, Data 88
- bank 7: Data 284
- bank 8: Code 78, Data 48
- bank 9: Code 78, Data 48
- bank 10: Data 652
- bank 11: Code 1701, Data 453
- bank 12: Data 171
- bank 13: Data 225
- bank 14: Data 1369
- bank 15: Code 805, Data 566
- Removed round-1 generic data regions, now replaced by exact tables plus a remainder:
  - bank 2, 0x81F6 ($C1E6-$C25E). This is now the FrontEndScreen word table and the parallel
    byte table $C227-$C27E.
  - bank 15, 0x3C098 ($C088-$C147), now part of PlayStateJumpTable $C048-$C147.
  - bank 11, 0x2C52B ($C51B-$C59A), now part of CpuInputStateJumpTable $C49B-$C59A.
- Main new code areas:
  - bank 2: front-end screen handlers spread over $C27F-$E73C (including $D72F.. and
    $DFCF-$E697), and the L-015 editors $E6CC/$E73D.
  - bank 11: CPU pad states $C8D0-$CD91 and $D1AA-$D319.
  - bank 15: PlayState handlers $CC08-$CEEB, $CFEB.., the mercy handler $D31B, and the debug
    shortcut $C606.
  - bank 4: script handlers $C605-$CB63.
  - bank 0: NMI tasks $82AF/$8305/$8708/$877C (L-012), plus the L-015 routines $9D4D and $A1B5.
  - bank 5: binary-to-decimal routine at $ECD8.
- The commentary source (commentary/source/game.asm) still reflects the round-1 map. Commentary
  needs to regenerate from the new map and add comments for the new regions.

## Unresolved questions
1. **L-016:** 10 bytes remain unknown.
   - Lone RTS bytes: bank 2 $D5FE, bank 6 $CA2E, bank 11 $C286 and $C9C4, bank 15 $C8FE and $D9F4.
   - Bank 2 $DAC9-$DACC (PLA/TAY/PLA/RTS).
   - Each is a valid instruction with no reference, so code versus data is genuinely ambiguous.
     Reopen only if a caller or an observed fetch turns up.
2. **Natural callers for the L-015 fragments** and the two residual fragments are unknown. This
   includes whether the bank 15 $C606 debug shortcut can be reached through the controller bits
   in $55.
3. **Roles of the L-013/L-014 generic data** (player records, DMC spans, metasprite pieces) are
   largely unnamed. This is commentary or redirect semantics work and does not change
   classification.
4. **No new gameplay scenario this pass.** The remaining unknowns are orphan returns that
   gameplay cannot resolve. The redirect's runtime evidence and milestone recipes in
   analysis/scenarios.json were reused.

## Toolkit
No new issues were found. TK-001 to TK-005 are unchanged; see analysis/toolkit-issues.md.
