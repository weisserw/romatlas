# Round 3 redirect handoff (attempt r03-redirect-a1)

Status: COMPLETE. The input checkpoint is pass-003: 100.00% classified, 0 unknown bytes, map sha256 fb2e3407...
- This round changed research and lead records only. The map, source, ROM, state.json and toolkit were not changed.
- Runtime sessions used `romatlas serve --rom` without `--store`. There were 22 bounded Read-watch runs plus 2 short reader-PC runs. No RAM edits were made in this round; some snapshots from earlier rounds are artificial (patched) states, as noted.
- Evidence is in analysis/redirect/r03-redirect-a1/:
  - findings.json (H1-H7)
  - runs.json (compact per-run summary)
  - static.json and refs.json
  - frames/ (local renders made from ROM data)
  - tools/: static.py, refs.py, hiscan.py, watch.py, reader.py, ntcompare.py, ntrender.py, l024.py and batch.sh. Run them with `python3 -I`.
- Scratch in /tmp/r3 was removed.

## Findings
- **H1 Static control-flow closure (code/data audit).**
  - A bank-aware static flow pass starts from four kinds of root: Observed code, the instruction starts executed in r02, the round-1 dispatch roots (FarCallTable, 10 FarCallAddress immediate sites, NmiHandlerTable, the jump tables at $F8EF, 0:$AD59, 0:$B1F1 and 6:$88F7/$8904, and the HillNmi targets), and $FF36 -> 6:$8BD4.
  - From those roots it reaches every instruction of 266 of the 288 Static code regions.
  - All 27 partly executed regions are fully flow-connected: their unexecuted parts are reachable branch arms, not data.
  - 0 resolved control-flow targets land in Data or mid-instruction.
  - 0 bank-unresolved jsr/jmp targets lack a Code start.
- **H2 The 22 flow-unreached islands (650 B).** These are the known L-001/L-016 islands.
  - No PRG word, jsr or jmp references them (start or start-1), apart from coincidental data words.
  - 3:$AA3F follows the observed RaceNmiStub `rts` at $AA3E.
  - The r02 rejection-review premises are unchanged.
- **H3 2:$B0E7-$B4E6 is a 1024-byte nametable image: a legal/trademark screen.** It leads to L-023.
  - Layout: 960 tile bytes ($B0E7-$B4A6) plus 64 attribute bytes ($B4A7-$B4E6).
  - Rendered with CHR 4K page 2, it shows the Bigfoot/4X4X4 trademark text, "(C) 1990 Acclaim" and "Licensed by Nintendo of America", plus the flag logo.
  - The start ($B0E6, from L-020) and end (ChampionScreen code at $B4E7) are proven independently.
  - The current internal split at $B4AA is 3 bytes after the real tile/attribute boundary.
  - Seven rows are byte-identical to rows of RLE screen 25 (the Acclaim/Beam Software credits screen). The block is not a copy of any RLE screen.
  - No static reader was found. It was not read in the boot (900 frames), title, race, 2P race or ending runs. Its reader is unknown and it may be unused.
- **H4 4:$A474-$A873.** This is exactly 1024 bytes between the end of the played sample ($2474) and SfxRecords. Its byte statistics match the sample, so it is probably unplayed sample-format data.
  - It was unread in race1 and event stage 3.
  - This is a semantic note only.
- **H5 5:$8608-$8777 is the per-column ground-height table.** It leads to L-024.
  - Reader: UpdateGroundHeight at F:$D2AC does `lda (Tmp14),y` with pointer = $8600 + id*8 + Tmp1C + 8.
  - The column-id domain is 0-45, so the table is exactly 46 x 8 bytes and ends at ColumnRecordPtrs ($8778).
  - Observed: 1,718 reads in event stage 1, with banks [10,11].
  - $8388-$8607 holds 640 zero bytes and is unread.
  - **Correction:** r02 said this range was "not read in traced runs", but r02 never watched it.
- **H6 Read-watch results (runs.json).**
  - RampAirVel and RampBumpSpeed: only entry 1 ($AA47/$AA4C) is read in three race runs. These are observed values only.
  - F:$FF2C and 6:$8701: the reads seen are dummy reads by `rts` (the byte after `rts` at F:$FF2B and 6:$8700), not a consumer.
  - These ranges were not read: 3:$894D-$89B4, 0:$8F84-$9180, 0:$BE6F, F:$E9AC-$E9B8 and 4:$A474.
- **H7 Pointer high-byte scan.** It found only known consumers, plus the L-024 reader.

## Ready leads (both refine already-classified bytes; 0 new bytes)
| Lead | Priority | Change |
| --- | --- | --- |
| L-024 | medium | Split region 824 (phys $142C7, 1235 B) at $14398, $14618 and $14788. The new parts are a TerrainAttrRecords continuation (209 B), a generic zero run (640 B), EventGroundHeights (46 x 8 = 368 B, reader F:$D2AC) and a ColumnRecordPtrs continuation (18 B). |
| L-023 | low | Merge regions 2021 and 1812 into LegalScreenTiles (phys $0B0F7, 960 B) and LegalScreenAttrs (phys $0B4B7, 64 B), reader unknown. |

The recipes are in analysis/leads.json. The check scripts are tools/l024.py and tools/ntcompare.py, and no reruns are needed. For commentary, both need notes in source and updates to uncertainties.md: 5:$8388 and 2:$B0E7/$B4AA are no longer "role unresolved", RampAirVel index 1 is confirmed by observation, and $FF2C/$8701 should be marked as rts dummy reads.

## Rejection review (residuals; evidence-policy)
- **27 partly executed Static regions (3,452 B).**
  - Claim: code throughout.
  - Decisive evidence: H1 closure. Every unexecuted instruction is a static successor of executed or dispatch-root code, and no target lands in data.
  - This rules out a data tail inside these regions.
  - Competing interpretation (inline data after a branch): rejected, because each region decodes contiguously and is entered by flow.
  - Verdict: retain. Execution of the remaining arms is a reachability question, not structure.
- **Low-confidence islands 0:$B1E2 (pla/tax), 6:$A61E (rts), 0:$8395 and F:$D827 (skipped else-arms).**
  - Decisive evidence: H2 found no reference by word, rts-trick or jsr/jmp. The r02 idiom evidence still applies: $B1E2 matches the txa/pha/pla/tax idiom at $B1D1, and the sibling arm $D822 was executed.
  - Data alternative: no reader and no table structure; the regions are 1-4 bytes between code.
  - No discriminating test exists, because executing these bytes would need a caller that does not exist.
  - Verdict: retain as code.
- **2:$AC37 (1 byte).**
  - Evidence: L-019. Pointer 50 would point here, and the producers stay at 49 or below.
  - Verdict: generic data stays. Nothing further is possible.
- **2:$B0E7-$B4A9.** Resolved structurally by L-023. Its reader remains unknown: there is no static reference and no read in 6 scenario runs.
  - Next step only if wanted: watch it during a full attract-mode cycle. Prospect is low.
- **Remaining unread generic blocks** (0:$8F84, $9087-$9180, $BE6F; 2:$83D4 sound area, $B059 spare script pointers; 3:$894D-$89B4; 4:$A474; 5:$8388-$8607 zero run; 6:$8400 and $8701; F:$E9AC-$E9AD and $FF2C-$FF2F).
  - Checks: an absolute-operand scan, a lo/hi immediate-pair scan, a high-byte pointer scan, a data-word scan, and Read watches in the scenarios that map each bank.
  - Result: no consumer and no code evidence. They are already generic data, which the policy accepts.
  - Strongest competing interpretation: an unreferenced code island. Rejected, because none decodes as code reached by flow and their contents are zero runs or tile/nibble/attribute patterns.
  - Further watching has diminishing prospect: only semantic gain, and every mode that maps these banks has now been watched once.
- **Static code reachability (288 regions).** This is not a structural question, because H1 shows they are flow-connected. Natural execution of the remaining arms would need rare branch conditions, and brute-forcing them is excluded.
- **~19 generic labels (2 @loop, 5 @join, 12 condition labels).** These are editorial, and the commentary final audit accepted them. No redirect method applies.
- **Gameplay.** Broad coverage is complete (scenarios.json). CPU forfeit and a finished 2P race remain unexamined; neither has a structural prospect.

## Recommendation
Run one more short cycle: disassembly for L-024 and L-023 (region splits and roles only, 0 new bytes), then commentary for the two tables and the uncertainty updates.

After that, no further redirect avenue is credible: every residual above has a recorded disposition. The coordinator may treat the next redirect as terminal unless the disassembly review rejects a lead.
