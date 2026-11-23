# Round 9 / disassembly handoff

## Scope
One disassembly pass on pass-008. It applied the ready redirect lead R8-L043 (metadata only).
- analysis/state.json was restored (== pass-008). The baseline was not re-created.
- input/game.nes is unchanged (sha256 335680fc...abfa). There were no toolkit edits.
- I used the binary named in workspace.json (/root/romatlas/target/release/romatlas) through a temporary socket bridge.
  Scratch /root/rascratch-r9d has been removed and my server stopped. No other task's process or directory was touched.
- No gameplay. r08-a1 judged structural research terminal and the lead needs no runtime evidence. scenarios.json is
  unchanged.
- R8-L044 (commentary prose names) was left for the commentary phase.

## Checkpoint
**pass-009**:
- maps/pass-009.json (sha256 18763c395f6636b1f8d9c827708e98b059ebca665921b90ebc6835c154f95de1)
- generated/pass-009
- analysis/pass-009-verification.json

## Verification
- `run.py checkpoint pass-009`: map validate ok, ok=true, byte_identical=true.
- analysis/state.json == maps/current.json == maps/pass-009.json (same sha256).
- Byte-wise comparison with pass-008 (analysis/tools/round09/compare.py, output in compare.txt):
  - no overlaps, and the same 2,590 region ids with identical offset/length/kind;
  - 0 kind, 0 provenance and 0 bank-context differences per byte;
  - only `confidence` and `evidence` changed, and only on the 20 lead ids.
- The live annotate log is identical to the `--dry` simulation (20 operations). The saved map matches the log.

ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).

Per bank (unchanged from pass-008):

| Bank | Code | of which Observed | Data | Padding | Unknown |
|---|---|---|---|---|---|
| header | 0 | 0 | 16 | 0 | 0 |
| b0 | 3 | 0 | 16090 | 291 | 0 |
| b1 | 3 | 0 | 16142 | 239 | 0 |
| b2 | 1546 | 1468 | 14407 | 431 | 0 |
| b3 | 4689 | 3311 | 9425 | 2270 | 0 |
| b4 | 10102 | 5852 | 5731 | 551 | 0 |
| b5 | 7124 | 4879 | 8263 | 997 | 0 |
| b6 | 3023 | 2678 | 12817 | 544 | 0 |
| b7 | 11436 | 8651 | 4663 | 285 | 0 |

Regions: 2,590 (1,670 code, 902 data, 18 padding). Observed code: 26,839 B. Execution coverage is a separate metric
from classification.

## Findings
**R8-L043 verified.** analysis/tools/round09/apply.py re-derives every claim directly from the ROM and from its own
operand scan of map Code regions, independent of r08-a1 extchk.py. All checks are true (changes.json "checks"):
- **PaletteSets b3 $BC80-$BF3F:**
  - all 44 16-byte records start with $0F, and the slot at $BF40 does not;
  - consumers form $BC80 + 16*n with adc #$80 / adc #$BC at b7 $C46A (LoadPaletteFromBank3, which requests bank 3
    first) and at b7 $EE67.
- **B4_TableFrameStructs b4 $8653-$8702:** the B4_TableFramePtrs words are $8653/$867F/$86AB/$86D7. The four
  contiguous 44-byte structs each end $00 $FF.
- **B5_WheelStrip b5 $99A2-$9B45:**
  - 84 columns of 5 bytes, each with two number tiles of $80 or above and rim triple E8 EB EA or E9 E5 E4;
  - the last 8 columns repeat the first 8, and the following slot breaks the pattern;
  - the base is loaded as an immediate pair at b5 $9CC0.
- **B5_ArrivedAtText b5 $B7A5-$B7B9:** header [19, 1] plus 19 tiles. The base is loaded as an immediate pair at b5
  $B65F.
- **CardBlockTable b7 $C5CE-$C637:** word k = $C638 + 37k for all 53 entries. It is read by lda $C5CE,x at b7 $C5C0.
- **CardBlocks b7 $C638-$CDE0:** 53 blocks, every header is 5, 7.
- **DpcmSampleCopyTail b7 $FEC1-$FF2F:**
  - byte-identical to $FD01-$FD6F;
  - none of the six B6_DpcmSampleTable sample spans reaches it. The spans are $FC00-$FC30, $FC40-$FDE0,
    $FE00-$FEC0, $FE00-$FEB0, $FE00-$FEA0 and $FD40-$FDF0.
- Every lead segment lies inside its structure extent. No non-Data region lies inside any checked extent.

Applied with map.annotate (TK-006 workaround: full region object resent):
- **Evidence:** the proposal prefix (tagged "[Round 9 disassembly (R8-L043)]"), then the round-9 check sentence, then
  the previous evidence.
- **Confidence:**
  - 0.85 for the 17 extent-checked rows (from 0.7-0.8);
  - unchanged for 2833 B5_PortraitTileLists (0.75) and for 2434 and 2438 (0.8). These segments contain sub-structures
    whose extents were not independently re-checked; only their Data kind was re-confirmed.
- No labels were added. Commentary labels stay authoritative.

## Leads
- Consumed and verified: R8-L043 (analysis/leads.json updated, with the pass appended).
- Not attempted: R8-L044 (commentary only; for the round-9 commentary/editorial phase).
- No new leads were produced.

## Regions changed (for commentary carry-forward)
Evidence and confidence only. Ids, extents, kinds, provenance and views are unchanged:
- 2179, 2180, 2181, 2182 (PaletteSets);
- 2810, 2186 (B4_TableFrameStructs);
- 2827, 2253, 2828, 2254, 2829, 2255, 2830 (B5_WheelStrip);
- 2833 (B5_PortraitTileLists);
- 2308 (B5_ArrivedAtText);
- 2428 (CardBlockTable);
- 2429 (CardBlocks);
- 2434 (ChipValuePtrTable segment);
- 2438 (MetaspriteSlotTable segment);
- 2458 (DpcmSampleCopyTail).

Only the provenance headers' evidence differs, so commentary regeneration should be byte-identical apart from any
header text taken from evidence.

## Files
analysis/tools/round09/:
- apply.py (`--dry` supported; the live run needs a recreated bridge in /root/rascratch-r9d)
- applied.jsonl
- changes.json
- compare.py
- compare.txt

## Remaining
- Structure is complete: 0 unknown bytes and 18 Padding regions. The semantic open items listed in the round-8
  handoff are unchanged.
- Next: round 9 commentary (carry by address, apply R8-L044, final editorial audit), then terminal handoff.

## Toolkit
analysis/toolkit-issues.md: no new toolkit issues found. The TK-005 and TK-006 workarounds were used again.
