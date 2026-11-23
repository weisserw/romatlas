# Round 3 disassembly handoff

Status: the pass is complete. Every byte is classified, but the ROM is still incomplete because the new current source has not been through commentary or editorial review.

ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).

## Scope
- One disassembly pass over the round-2 checkpoint, applying the ready redirect leads L-007 (as amended by r2-a2), L-008 and L-010. I re-checked each lead myself before applying it. L-009 is commentary-only and goes to the commentary phase.
- All edits came from one computed plan, analysis/round3-tools/plan.py. It removes 7 parent regions and adds 100. It asserts that the new regions cover each parent exactly, that nothing overlaps, and that labels stay unique. analysis/round3-tools/apply.py applied the plan with map.load, map.remove and map.annotate over `run.py serve` (through the r1-a1 daemon socket).
- No gameplay this pass. Every milestone was already visited in r1-a1, and these leads are static corrections backed by the saved r2-a2 read-watch, so more play would only add provenance (scenarios.json notes_r3_disasm).

## Checkpoint and verification
- Input: maps/round2-disasm.json (sha256 6b48e02b...e055), 928 regions.
- Output: maps/round3-disasm.json = maps/current.json = analysis/state.json (sha256 24099db1a8311da17f7abf3fe5598c5af5380e2c4c0e9fb1cbb6c13210afbed3). It has 1021 regions and 185 labels, and its regions cover the whole ROM with no gaps.
- `python3 run.py checkpoint round3-disasm` reported ok=true and byte_identical=true (analysis/round3-disasm-verification.json). The source is in generated/round3-disasm, and diagnostics.json is empty. map.validate passed.
- input/game.nes is unchanged (sha256 153068655b...).
- Totals: Code 26,374 (19,753 Observed, 6,621 Static), Data 92,964, Padding 11,750, Unknown 0. Per-bank counts are in analysis/progress.md under "Round 3 disassembly".

## Findings / lead outcomes (analysis/leads.json)
- **L-008 verified.** 7:$CFCF-$D024 (86 bytes) moved from Data to Code: Static, confidence 0.8, statically unreferenced with no known entry. It is not called dead.
  - The view is main $CFCF with prg_banks [null,null,14,15].
  - I re-decoded the run. It has no gaps, both branches stay inside it, and it ends with JMP $CFCF just before PrintText at $D025.
  - A fresh scan with refs.py and ptrs.py found no reference to $CFCF or $CFEE.
  - MetaspriteTable is now $CFB5-$CFCE (13 words).
  - $D084-$D12B is split into:
    - CharToTileTail $D084-$D0BF;
    - TextCursorColBase $D0C0, TextCursorRowBase $D0CC and TextCursorWidth $D0D8, 12 bytes each, read only by the unreferenced routines;
    - two generic spans with no consumer: $D0E4 (12) and $D0F0-$D12B (row-address words).
- **L-007 verified.** 77 consumer-bounded records (74 labeled), including the r2-a2 amendments:
  - FourReelCoinTiles is 24 bytes and FourReelDenomText is 12;
  - three new win-text records: FourReelWinText, SlotWinTextA and SlotWinTextB (via WinTextPtrs 6:$AD12);
  - five stop-table records: FourReelStopTable0/1 (64 bytes each, AND #$3F) and ReelStopTableA0-2 (66 bytes each, Mod66).

  I re-decoded these consumers: 6:$ACF8, $8AD1/$8D23, $87A9/$8993, $AF82, $9511, $BDD3, $AF34, and 7:$DC36/$DC52. I also checked the NametableRowAddr users. Bounds read directly from the consumer get confidence 0.9. Bounds that are anchored by the next table or inferred from stride get 0.85. The 7:$DCA3-$DCEF head keeps its original evidence.
- **L-010 verified.** 13 Mspr* records at 7:$C049-$C0D2 (confidence 0.9), plus a generic 436-byte block of 14 frames at $C0D3-$C286 (0.8) with a note on its format.
- **L-009:** commentary-only and still ready.
- The redirect generators (splits.py, l007fix.py, msplits.py) were rerun and gave byte-identical outputs with their assertions passing.

## Changed physical ranges (for commentary carry-over)
Old region (now split) and its new contents:
- 6:$B0EB-$BAC7, phys 0x1B0FB +2525: 53 labeled records (BigSixAreaPos … HoldCardTiles), plus generic 6:$B554-$B567.
- 7:$C049-$C286, phys 0x1C059 +574: MsprCursor … MsprBounceBall, plus generic $C0D3-$C286.
- 7:$CFB5-$D024, phys 0x1CFC5 +112: MetaspriteTable $CFB5-$CFCE, then **Code $CFCF-$D024**. The commentary should name these routines (for example AdvanceTextCursor_Unref $CFCF and PrintCharAtCursor_Unref $CFEE).
- 7:$D084-$D12B, phys 0x1D094 +168: CharToTileTail, TextCursorColBase/RowBase/Width, plus generic $D0E4 and $D0F0.
- 7:$D6FD-$D9CA, phys 0x1D70D +718: PowersOfTenHi/Mid/Lo, FourReelStrip0/1, ReelStripA, ReelStripB0-2, FourReelStopTable0/1, ReelStopTableA0-2, ReelStopTableB, RandomSeedTable.
- 7:$DCA3-$DD7F, phys 0x1DCB3 +221: the head $DCA3-$DCEF is unchanged as generic data; ClockHourTable is $DCF0-$DD7F.
- 7:$F378-$F8F7, phys 0x1F388 +1408: AttrShadowPtr0/1, generic $F398, NametableRowAddr (60 bytes), generic $F3DC, CardFaceTiles.

Commentary should replace the stale equates and sizes listed in L-007 and L-009 with these map labels. It should also fix the metasprite byte order: attribute bytes come before tile bytes.

## Unresolved
- No unknown bytes.
- Generic spans with no consumer:
  - 6:$B554 (20) and 6:$85CE (32);
  - 7:$F398 (8), $F3DC (60), $D0E4 (12), $D0F0 (60), $C0D3 (436), $E195 (15) and $E000 (5).
- Code with no known entry: $CFCF-$D024 (86) and the 675-byte L-004 residue.
- Weakest classifications: 0x18010, a single byte classified by layout analogy, and the unreferenced code above.

## Deferred
- Round-3 commentary and editorial review of the new source (1021 regions).
- More traced gameplay to raise Observed coverage: provenance only, and tracing is slow (TK-002).
- A runtime read-watch for the $F3xx and $D0xx no-consumer spans on text and floor screens: low prospect.

## Links
- Progress: analysis/progress.md ("Round 3 disassembly").
- Ledger: analysis/investigation.md.
- Leads: analysis/leads.json (passes[] round 3).
- Scenarios: analysis/scenarios.json.
- Tools: analysis/round3-tools/ (plan.py, plan.json, apply.py).
- Evidence: analysis/redirect/r2-a1/, analysis/redirect/r2-a2/.
- Toolkit: analysis/toolkit-issues.md (TK-001..TK-005). No new toolkit issues were found this pass.
