# Round 3 - disassembly phase handoff

Status: FINISHED for this single pass. Overall result is INCOMPLETE: 3 bytes are still unknown, and all 3 are blocked by the toolkit (TK-04).

## Scope
- Input: maps/round02-final.json (sha256 53e08b7b...), restored from analysis/state.json, which was byte-identical to it. The baseline was not reinitialized, and existing classifications were kept.
- Work done: ready leads L-R2R-01..04 from analysis/handoffs/round-02-redirect.md, plus the coordinator's $8008 sibling-bank check.
- Not done: gameplay. No lead needed it. S12 (natural ending) is still `attempted` and S14 (ending, artificial) is still `visited` in analysis/scenarios.json.
- Tool: analysis/tools/r03_apply.py. It annotates through the RPC bridge analysis/tools/rpcbridge.py, which uses the configured binary /root/romatlas/target/release/romatlas.

## Output checkpoint
- maps/round03-final.json, sha256 `aab2fb6c6b8120365d94d57a881bad3c0da726c7f5ccf5126a5504d8367b297b`.
- maps/current.json and analysis/state.json are identical to it (same sha256).
- Verification: `python3 run.py checkpoint round03-final` passed validate, generate and verify, with `ok: true` and `byte_identical: true` (analysis/round03-final-verification.json). generated/round03-final/diagnostics.json is `[]`.
- input/game.nes is unchanged (sha256 8e32ae84...).

ROM classification: 100.00% (131085 / 131088 bytes classified; 3 bytes unknown).

The rounded 100.00% does not mean the work is complete: 3 bytes are still unknown.

| Area | Code | Data | Unknown |
|---|---|---|---|
| header | - | 16 | 0 |
| bank 0 | 11585 | 4797 | 2 |
| bank 1 | 651 | 15733 | 0 |
| bank 2 | 1274 | 15110 | 0 |
| bank 3 | 234 | 16150 | 0 |
| bank 4 | 103 | 16281 | 0 |
| bank 5 | 675 | 15709 | 0 |
| bank 6 | 1813 | 14571 | 0 |
| bank 7 | 9683 | 6700 | 1 |
| total | 26018 | 105067 (incl. header) | 3 |

## Lead outcomes (details in analysis/leads.json)
- **L-R2R-01: verified, Code 29 B, Static, confidence 0.75.**
  - $8229: `lda #$60` falls into UploadStatusRow. The preceding WaitSprite0Hit ends in RTS.
  - $8CED: same shape as FootBagSummaryDone, ending in `jmp ClearStatusLineY`.
  - $D7B9: `ldy #$18 / jmp QueueSfx`. It completes the slot series $10/$14/$18/$1C.
  - $F150: a self-closed fill loop. It sits between BinToBcdTable, which ends at $F14F, and WriteAttributeRamp at $F15A.
  - $F2BA: CallBank0. I followed the recipe:
    - Removed Observed region 578.
    - Annotated $F2BA-$F2BE (`lda #$00 / bit $01A9`) as Static. Its evidence text notes that $F2BD-$F2BE were observed as opcode fetches.
    - Re-annotated $F2BF-$F2F0 as Observed with region 578's original evidence.
  - None of this code has a caller or selector, and no reachability is claimed.
- **L-R2R-02: verified, Code 9 B, Static, lower confidence 0.6.**
  - Three lone JMPs: $8A85 to IdleLoop, $DEBF to SurfUpdate, $FFF7 to ResetVector.
  - Each operand is an exact established entry. Each sits on an instruction boundary after a terminator or the irregular data block, and fills the space exactly up to the next code or the vectors.
  - No data consumer exists, so a stub is much more likely than a 3-byte data object that happens to match an entry address.
  - These are unreferenced and not claimed reachable. Revisit them if a data consumer turns up.
- **L-R2R-03: verified, Data 18 B, confidence 0.9.** I re-checked the ROM bytes and each consumer:
  - The FrameHandlerTable tail ($87CB-$87CF) points at SurfScoreFrame, SurfRideEnd and LicenceFrame, and the table ends before NameEntryFrame $87D0.
  - The BMX OAM image tail ($BEA3-$BEA7) is bounded by the copy loop at $BDE8 (`ldx #$1B` / `lda $BE8C,x`).
  - The player-offset copy ($EEB5-$EEB6) matches $EFC9. Its consumer is at $D10F, and CurrentPlayer is capped at 7.
  - BinToBcdTable entries 94-99 ($F14A-$F14F) fit the exact BCD(0..99) table; the next byte, $A2, is not BCD.
- **L-R2R-04: verified, generic Data 87 B, confidence 0.8.**
  - b2 $87DF-$87E6 is a run of $00 bytes after an always-taken `bne`, ending just before entry $87E7.
  - $9B46-$9B47 would decode as an `ora` whose result is discarded before PipeBodyShift8.
  - $F072-$F079 hits unofficial opcodes in every alignment.
  - $F883-$F8AF and $FFDF-$FFF6 contain ROM writes, unmapped addresses and illegal opcodes inside data blocks.
- **L-R3-01 (new): verified, b0 $8008-$800A Code 3 B, Static, confidence 0.55.** See the next section.

## $8008 comparison result
CPU $8000-$802F in banks 1-6 all have the same layout:
1. The $00..$07 bus-conflict table.
2. N 3-byte `JMP` entry instructions starting at $8008. All of them are already classified Code (Observed). Per bank, N is 3, 9, 3, 2, 1 and 6.
3. The text " (C) COPYRIGHT 1987 RARE LTD. ENGLAND".

Bank 0 has exactly one 3-byte slot in that position, `20 93 D8` (`jsr UpdateSound`, an established fixed-bank entry), followed by the same text.

- **Decision:** I classified it as Code Static at low confidence, as the bank-0 entry-0 instruction. This sibling structure is an independent constraint that the r02-a1 rejection did not consider.
- **Caveats recorded in the region evidence:** the opcode is JSR rather than JMP, so if anything called this entry, it would return into the copyright text. CallBank0 and Y=$08 into bank 0 both have no caller.
- **Not claimed:** that the entry works, or that it is reachable.
- **Revisit** if a data consumer of $8008 is found.

## Remaining unknowns (3 B)
- 0x400E-0x400F (bank 0 $BFFE-$BFFF) and 0x1C010 (bank 7 $C000). These bytes are the observed instruction `lda $C149,x`, which crosses the switchable/fixed window boundary.
- An instruction cannot be annotated across physically non-contiguous bytes (TK-04, L-R1-01 blocked).
- Annotating them as Data would contradict the observed opcode fetch.
- These can only be resolved with an authorized toolkit change.

## Toolkit
No new toolkit issues found. TK-01..05 are in analysis/toolkit-issues.md, and TK-04 is the only remaining blocker.

## Finished vs needs-resume
- This pass is finished. analysis/progress.md and analysis/investigation.md have Round 3 sections, and analysis/leads.json is updated.
- Commentary needs to cover the new regions:
  - Labels: CallBank0, Bank0Entry0, and the PlaySfx slot-6 sibling.
  - Notes and labels for the table tails (BmxInitOam, the $EEAF player offsets).
  - The redirect's commentary corrections: PlayerNames stride 7 and the FrameHandlerTable extent.
  - CallBank1 ($F2BD) is now an alternate entry hidden inside a BIT operand, like CallBank2..7.
- No further disassembly work is possible without a toolkit change, so the next redirect can be terminal.
- Disk: about 3.0G free. New files are scratch/r03_pre.json and scratch/leads.r03.before.json (disposable), plus generated/round03-final.
