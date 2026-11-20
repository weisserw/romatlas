# Round 6 disassembly handoff

Status: complete. The pass started from pass-005, restored analysis/state.json and did not re-baseline.

Checkpoint **pass-006** (maps/pass-006.json, generated/pass-006, analysis/pass-006-verification.json):
- map.validate ok
- byte_identical true
- map sha256 333b439b...

ROM classification: 100.00% (262160 / 262160 bytes classified; 0 bytes unknown).

Kinds are unchanged:
- Code 37,907 (26,456 Observed, 11,451 Static)
- Data 93,181
- Image 131,072

There are 1,400 regions (was 1,401). No emulator runs were needed.

## Lead outcomes (all verified)
| Lead | Map change (16 KiB bank:CPU, phys) |
| --- | --- |
| L-029 | Region 2026 is now **StoryTileRecords**, 0:$90B2-$9180 ($010C2-$01190, 207 B). It holds 27 records (rows<<4 \| width header, then tile ids) plus 29 unread separator bytes. Regions 665 and 1726 were removed. Region 666 is now **StoryTextBlocks** head $9181-$9217 ($01191-$01227). |
| L-030 | Bank 2: new **ChannelOrderListPtrs** $83D0-$854F ($083E0-$0855F, 384 B, region 2028). New **InstrumentTable** $8610-$8707 ($08620-$08717, 31 x 8 B, region 2029). New **SoundOrderLists** $8708-$8C14 ($08718-$08C24, 1293 B, region 2030). Trimmed: 739 = $83C9-$83CF (NotePeriods tail), 1791 = $8550-$8569 (SongPatternTables head), 742 = $85C6-$860F (SongHeaders tail), 743 = $8C15-$8CC4 (pattern-offset tables). Removed: 740, 1790, 1793. |
| L-031 | New **Bank2FillFF** 2:$B65D-$BFFF ($0B66D-$0C00F, 2467 B, region 2031). Region 764 is now $B611-$B65C (champion colour/palette tables). Region 964 (F:$FF57-$FFF9) evidence now says $FF fill. |

Evidence:
- `python3 -I analysis/redirect/r05-redirect-a1/tools/checks.py` passes.
- The bytes were rechecked bank-aware; see analysis/investigation.md, Round 6.
- Plan/apply: analysis/disasm-r06/plan.json and tools/plan.py / tools/apply.py.
- The pre-edit state copy is analysis/disasm-r06/state-before-r06.json.

## For commentary round 6
The map labels StoryTileRecords, StoryTextBlocks, ChannelOrderListPtrs, InstrumentTable, SoundOrderLists and Bank2FillFF are now map symbols.

Commentary needs to do the following:
- Replace the "map/story data, no reader" notes at 0:$90B2/$90ED/$9138/$9154 with the StoryTileRecords format.
- Retire or redescribe SoundDataLow ($83D4) as ChannelOrderListPtrs at $83D0. NotePeriods effectively ends at $83CF.
- Make SongChannelOffsets "absolute bases of the four channel tables".
- Replace the "1329 bytes" description of InstrumentNoiseBytes with 31 records plus SoundOrderLists. InstrumentNoiseBytes stays as the +4 field label.
- Fix ChampionPictureData so it says "32-byte palette only; $FF fill follows". Fix the F:$FF57 note.
- Update uncertainties.md: remove 0:$90B2-$9180, 2:$83D4 and 2:$B63D+.

## Open (non-classification)
These are unchanged from the round-05-redirect.md rejection review:
- 0:$BDE8/$BE00 unread tables
- 2:$B059 spare pointers
- small/uniform generic blocks
- the LegalScreenTiles reader
- natural reachability of the Static code islands
- StoryTileRecords $9149/$9154 not observed read

No lead was rejected.

Toolkit: no new issues (TK-001..TK-006, analysis/toolkit-issues.md).
