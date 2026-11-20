# Disassembly progress

## Coordinator status
- Round: 7
- Current phase: TERMINAL HANDOFF (2026-11-20) — whole-ROM criteria met at pass-007; see analysis/final-handoff.md. R6 redirect rejection review reused for R7 (note-only map change, premises unchanged); no further rounds. Archive not run (not authorized here); archive-options.json prepared.
- Input checkpoint: pass-007 (verified byte-identical)
- ROM classification (final, pass-007): 100.00% (262160 / 262160 bytes classified; 0 bytes unknown). Commentary audit ok (1,400 regions / 1,330 units, editorial_audit true), byte-identical.
- Disk at start: 4.0G free on / (86% used) — workers must keep scratch bounded.

## Round history
(Workers append phase handoff summaries below; coordinator updates status above.)

### Round 1 — disassembly (2026-11-20)
- Baseline established (maps/baseline.json, verified byte-identical). Checkpoint: **pass-001** (maps/pass-001.json, generated/pass-001, analysis/pass-001-verification.json): validate ok, byte_identical true.
- ROM classification: 94.39% (247463 / 262160 bytes classified; 14697 bytes unknown).
- Kinds: Code 36,645 (24,280 Observed-provenance bytes), Data 79,746 (generic, Static), Image 131,072 (CHR ROM), header 16 (Data). 1,090 regions.
- Per bank classified/unknown (code/data): PRG00 7378/814 (4274/3104); PRG01 7927/265 (3575/4352); PRG02 6157/2035 (0/6157); PRG03 8192/0 ($FF fill); PRG04 7263/929; PRG05 7967/225; PRG06 7953/239; PRG07 7029/1163; PRG08 7736/456 (4-bit sample-like data); PRG09 7289/903; PRG10 4357/3835 (tables); PRG11 6670/1522; PRG12 6445/1747; PRG13 8107/85; PRG14 8099/93; PRG15 7806/386; header 16/0; CHR 131072/0.
- Execution coverage (separate metric): 15,708 observed instruction contexts; scenarios boot, title, menu, rally, map, cross-country race, shop, car crush; RAM-patched mud race, hill climb, tractor pull, drag race, championship ending.
- Leads consumed: none (fresh workspace). Produced: L-001..L-009 (analysis/leads.json).
- Toolkit: 2 confirmed bugs (TK-001 input params, TK-002 discover params), 2 missing features (TK-003 discovery stops at JSR/indexed stores, TK-004 trace overhead); none blocking. See analysis/toolkit-issues.md.
- Handoff: analysis/handoffs/round-01-disassembly.md. Status: incomplete (unknown bytes remain).

### Round 1 — commentary (2026-11-20)
- Prepared commentary/ from pass-001; source regenerated from frozen before/ + notes via commentary/tools/annotate.py (symbol substitution only).
- Scope: all classified source in every bank (748 code regions, 310 data, header, CHR) plus notes on all 271 unknown ranges; 449 commented routines, 216 named tables, 533 RAM names (mode-specific aliases), register names; review.json 1361 entries reviewed / 666 units (editorial audit pending).
- Verification: commentary.py verify byte_identical true; reading copy refreshed (commentary/reading-after.asm).
- Remaining gaps: none in readability scope; tentative meanings and 5 structural redirect questions (warm-start stub $8055, ScriptOpTable extent, bank 6 $8D84 jump table, ~35 consumed short tables) in analysis/handoffs/round-01-commentary.md. Toolkit: TK-005 (missing feature, non-blocking).
- Status: complete.

### Round 1 — redirect (2026-11-20, attempt r01-redirect-a1)
- Input checkpoint pass-001 (14,697 unknown). Static research only; no map, source or toolkit changes.
- Leads consumed/updated: L-001, L-002, L-003, L-004, L-005, L-006, L-009. Produced: L-010 to L-017.
- Outcomes: bank 1 RLE nametable streams (exact 1,024-byte decodes); bank 4 digitized sample ($2474 bytes); bank 5 complete event-data layout; bank 6 HillNmiJump table + 643 bytes of code + terrain tables; bank 3 CPU path lists; bank 0 script tables/scripts, palettes and the warm-start code fix ($8055-$805D Data->Code); fixed screen records, SFX pointer table, code islands with contracts.
- Ready expected bytes: about 13,624 of 14,697. Residual: L-017 1,073 bytes (open). L-007/L-008 gameplay still deferred.
- Evidence: analysis/redirect/r01-redirect-a1/{findings.json,unknown-ledger.json,tools/}. Handoff: analysis/handoffs/round-01-redirect.md.
- Recommendation: another disassembly round (not terminal).

### Round 2 — disassembly (2026-11-20)
- Restored analysis/state.json (pass-001 classifications kept; no new baseline). Applied 296 map.annotate edits (295 new regions + region 657 Data->Code). Checkpoint: **pass-002** (maps/pass-002.json, generated/pass-002, analysis/pass-002-verification.json): validate ok, byte_identical true, map sha256 e5f7e095...
- ROM classification: 100.00% (262160 / 262160 bytes classified; 0 bytes unknown).
- Kinds: Code 37,907 (24,280 Observed, 13,627 Static), Data 93,181 (Static), Image 131,072 (CHR). 1,385 regions.
- Per 8 KiB PRG bank code/data (unknown 0 in every bank): PRG00 4351/3841; PRG01 3577/4615; PRG02 0/8192; PRG03 0/8192; PRG04 1433/6759; PRG05 1121/7071; PRG06 2157/6035; PRG07 2895/5297; PRG08 0/8192; PRG09 2185/6007; PRG10 0/8192; PRG11 1353/6839; PRG12 4181/4011; PRG13 1260/6932; PRG14 6752/1440; PRG15 6642/1550; header 16 Data; CHR 131072 Image.
- Leads verified (bank-aware recheck before annotate): L-001 (568 B code islands + 78 B tables), L-002 805, L-003 5,267, L-004 2,035, L-005 (643 B code + 38 B data), L-006 793, L-009 157 + $8055-$805D Data->Code, L-010 459, L-011 1,046, L-012 363, L-013 95, L-014 64, L-015 735 (generic), L-016 (436 B data + 42 B code: $8395, $8588, $8998, $D827), L-017 1,073 (generic, per-gap review). Rejected: none. Deferred: L-007, L-008 (gameplay; no byte gain left).
- Scenario progress: no new gameplay this round (static verification only); checklist entry added in analysis/scenarios.json. Execution coverage unchanged at 15,708 observed instruction contexts; new Static code (1,262 B) has no observed execution or known caller.
- Low-confidence calls to revisit: bank 0 $B1E2-$B1E3 (pla/tax residue, 0.5), bank 6 $A61E (lone RTS, 0.5), $8395/$D827 dead else-arms (0.6).
- Toolkit: TK-006 new (confirmed bug: map.annotate needs id=0 despite docs); TK-001..TK-005 unchanged; none blocking. See analysis/toolkit-issues.md.
- Evidence/tools: analysis/disasm-r02/{plan.json,tools/plan.py,tools/apply.py}. Handoff: analysis/handoffs/round-02-disassembly.md.
- Status: all bytes classified; ROM completion still requires commentary of the new regions and the final editorial audit.

### Round 2 — commentary (2026-11-20)
- Map changed (pass-002), so round-1 commentary was preserved whole at analysis/round-1/commentary/ and a fresh baseline prepared from pass-002; round-1 notes carried by bank/CPU address, 82 stale blocks and 10 drifted table-name references rewritten, four wrong cross-bank tags and several RAM-alias defects fixed.
- Scope: all classified source. New regions named/explained (bank 1 screen streams, bank 0 palettes/scripts/story text, bank 2 sound tables, bank 3 track and CPU path data, bank 4 sample/SFX/text/font map, bank 5 event layout, bank 6 HillNmi step handlers and terrain tables, fixed-bank screen records/SFX table/debug helpers); low-confidence code ($B1E2, $A61E, $8395, $D827) described with explicit uncertainty. Every code label (530) now has a header; branch targets get descriptive mechanical names (210 plain @skip remain).
- review.json: 1385 entries reviewed (1089 carried, 296 new), 1310 units; editorial audit pending.
- Verification: commentary.py verify byte_identical true; reading copy refreshed (commentary/reading-after.asm).
- Redirect questions: HillSpecial reachability, readers of 0:$A000 / 2:$A000 blocks, SfxPtrTable length, NmiHandler2/13 banks, ScriptOp1D `jsr $0014`. Handoff: analysis/handoffs/round-02-commentary.md.
- Status: complete.

### Round 2 — redirect (2026-11-20, attempt r02-redirect-a1)
- Input checkpoint pass-002 (0 unknown). Research only; no map, source, ROM, state or toolkit changes. Runtime: 11 bounded traced runs plus untraced play, served without --store.
- Leads consumed/updated: L-001, L-005, L-007, L-008, L-016. Produced (ready): L-018 (bank 0 MetatileTerrainMap $9C55-$A844, 191 x 16; TerrainEffectA-D 4 x 43), L-019 (bank 2 RaceSpriteLayouts pointer table + 50 x 64 B layouts to $AC36), L-020 (bank 2 tail: palette scripts to $B0B6, TitleReveal pointers, NmiHandler13 rows), L-021 (provenance: 99 Static code regions / 5,628 B executed), L-022 (F:$FE12 four secret button sequences). All 0-byte refinements of already classified bytes.
- Runtime checks resolved: HillSpecial 0-12 natural and HillNmi handlers executed; 0:$A000 and 2:$A000 readers found; SFX ids {0,3,7,8,9}; NmiHandler2 $8592 = bank 0, NmiHandler13 rows = bank 2; ScriptOp1D never used (op $1D absent from all 71 scripts). No code/data contradictions (0 executions outside Code). Weak-code review: all retained.
- Gameplay: two-player mode visited; stop-4 race + BROKE game over; input-only secret stop-select; championship ending reached naturally (stops 7-8, no RAM patch). L-008 verified; L-007 mostly visited.
- Evidence: analysis/redirect/r02-redirect-a1/. Handoff: analysis/handoffs/round-02-redirect.md.
- Recommendation: disassembly round 3 for the L-018..L-022 corrections, then commentary (G-series notes) and final editorial audit. Not terminal.

### Round 3 — disassembly (2026-11-20)
- Restored analysis/state.json (= pass-002, no re-baseline). Applied 118 map.annotate edits (13 new regions, region splits/relabels/provenance only; no byte changed kind). Checkpoint: **pass-003** (maps/pass-003.json, generated/pass-003, analysis/pass-003-verification.json): validate ok, byte_identical true, map sha256 fb2e3407...
- ROM classification: 100.00% (262160 / 262160 bytes classified; 0 bytes unknown).
- Kinds: Code 37,907 (26,456 Observed, 11,451 Static), Data 93,181 (Static), Image 131,072 (CHR). 1,398 regions.
- Per 8 KiB PRG bank code/data (unknown 0 in every bank; unchanged): PRG00 4351/3841; PRG01 3577/4615; PRG02 0/8192; PRG03 0/8192; PRG04 1433/6759; PRG05 1121/7071; PRG06 2157/6035; PRG07 2895/5297; PRG08 0/8192; PRG09 2185/6007; PRG10 0/8192; PRG11 1353/6839; PRG12 4181/4011; PRG13 1260/6932; PRG14 6752/1440; PRG15 6642/1550; header 16 Data; CHR 131072 Image.
- Leads (all verified bank-aware against saved evidence, no reruns needed): L-018 TerrainEffectA-D 4x43 + MetatileTerrainMap 191x16 (0:$9BA9-$A844); L-019 RaceSprOffsetX/Y/XFlip/YFlip 4x48, RaceSpriteLayoutPtrs (51 words), RaceSpriteLayoutData 50x64 (2:$9E91-$AC36) + $AC37 1-byte generic; L-020 PaletteScriptData tail / TitleRevealScriptPtrs / TitleRevealRowTiles / generic $B0E7-$B4A9; L-021 72 Static code regions (2,176 B) -> Observed, 27 (3,452 B) partly-executed notes; L-022 SecretButtonSequences F:$FE12. Rejected: none.
- Gameplay: none this pass (no lead required new runtime evidence); scenarios.json note added. Toolkit: no new issues (TK-001..TK-006 unchanged; see analysis/toolkit-issues.md).
- Evidence/tools: analysis/disasm-r03/{plan.json,tools/}. Handoff: analysis/handoffs/round-03-disassembly.md.
- Status: all bytes classified; completion still requires commentary of the relabelled regions and the final editorial audit.

### Round 3 — commentary (final acceptance) (2026-11-20)
- Map changed (pass-003), so the whole round-2 commentary was preserved at analysis/round-2/commentary/ (round-1 copy untouched). A fresh baseline was prepared from pass-003; notes and review were carried by physical range. 1,381 entries kept prior credit; 17 new or changed entries were reviewed in round 3. Name conflicts were reconciled (RaceSpriteLayoutPtrs and the other new map labels; LoadScreen_Tbl -> ScreenRecords).
- G1-G14 written into source: terrain lookup chain, race layouts, title reveal data, SecretButtonSequences and stop-select, shop exit rule, ScriptOp1D dead, NMI handler banks now symbolic, palette slot 29, SFX ids 0-10, bank 5 $8388, HillSpecial reached in play. Resolved items are marked in commentary/uncertainties.md.
- Final editorial cleanup: 166 reader-named tables renamed by role; all 210 @skip labels and 71 of 73 plain @loop labels given mechanical descriptive names; stale cross-mode RAM aliases and raw RAM operands fixed; 18 region notes and 14 label comments added. Remaining generic names: 0 @skip, 2 @loop, 5 @join, 12 subjectless condition labels.
- Audit (`commentary.py audit`, run once at the end): ok. Fresh rebuild byte_identical true; 1,398 regions and 1,317 units reviewed; editorial_audit reviewed; reading copy refreshed.
- Handoff: analysis/handoffs/round-03-commentary.md. Toolkit: no new issues.
- Status: complete.

### Round 3 — redirect (2026-11-20, attempt r03-redirect-a1)
- Input checkpoint pass-003 (0 unknown). Research only; no map, source, ROM, state or toolkit changes. Runtime: 22 bounded Read-watch runs plus 2 reader-PC runs (serve without --store; no RAM edits).
- Static audit: bank-aware flow closure from observed code + dispatch roots reaches all 266 non-island Static regions; all 27 partly executed regions are fully flow-connected; 0 control-flow targets in data; 22 islands (650 B) still unreferenced (r02 review holds).
- Produced (ready): L-024 (5:$8608-$8777 EventGroundHeights 46 x 8, reader UpdateGroundHeight F:$D2AC, observed reads; $8388-$8607 640-B zero run; corrects r02's "not read" claim), L-023 (2:$B0E7-$B4E6 = 1024-B legal/trademark nametable image, tiles + attributes split at $B4A7; reader unknown, unread in boot/title/race/ending). Both 0-byte refinements.
- Other outcomes: RampAirVel/RampBumpSpeed only index 1 read; F:$FF2C and 6:$8701 reads are RTS dummy reads; remaining generic blocks unread (dispositions in handoff rejection review).
- Evidence: analysis/redirect/r03-redirect-a1/. Handoff (with rejection review): analysis/handoffs/round-03-redirect.md.
- Recommendation: one short disassembly/commentary cycle for L-024 and L-023; afterwards no credible redirect avenue remains (expect terminal).

### Round 4 — disassembly (2026-11-20)
- Restored analysis/state.json (= pass-003, no re-baseline). Applied 6 map.annotate edits (3 new regions; splits/relabels only, no byte changed kind; 1,398 -> 1,401 regions). Checkpoint: **pass-004** (maps/pass-004.json, generated/pass-004, analysis/pass-004-verification.json): validate ok, byte_identical true, map sha256 70b38672...
- ROM classification: 100.00% (262160 / 262160 bytes classified; 0 bytes unknown).
- Kinds unchanged: Code 37,907 (26,456 Observed, 11,451 Static), Data 93,181 (Static), Image 131,072 (CHR). Per-bank counts identical to Round 3 (unknown 0 in every bank).
- L-024 verified: bank 5 region 824 split -> $82B7-$8387 terrain attribute records tail (209 B), $8388-$8607 generic zero run (640 B, reader unknown), $8608-$8777 EventGroundHeights (46 x 8, reader UpdateGroundHeight F:$D2AC, observed reads in artificial event-stage snapshot), $8778-$8789 ColumnRecordPtrs head (18 B).
- L-023 verified: bank 2 $B0E7-$B4A6 LegalScreenTiles (960 B) + $B4A7-$B4E6 LegalScreenAttrs (64 B) replace regions split at $B4AA; legal/trademark nametable image; reader unknown (reachability unresolved, not claimed unused). Rejected: none.
- Gameplay: none (no concrete question needed runtime evidence; saved r03 watches reused). Toolkit: no new issues (TK-001..TK-006; analysis/toolkit-issues.md).
- Evidence/tools: analysis/disasm-r04/{plan.json,tools/}. Handoff: analysis/handoffs/round-04-disassembly.md.
- Status: all bytes classified; commentary must document the new/changed regions (EventGroundHeights, LegalScreenTiles/Attrs, zero run) and update uncertainties.

### Round 4 — commentary (final acceptance) (2026-11-20)
- Map changed (pass-004), so the whole round-3 commentary was preserved at analysis/round-3/commentary/ (round-1/2 copies untouched). Fresh baseline prepared from pass-004; notes/tools carried by physical range (checknotes 0 misplaced). 1,244 entries kept prior credit; 157 entries whose source text changed (per-region diff vs round 3) were re-reviewed as round 4.
- New/changed regions documented: LegalScreenTiles/LegalScreenAttrs (raw legal nametable, reader unknown), Data_5_8388 (640-B zero run, unread), EventGroundHeights (46 x 8 format) with UpdateGroundHeight pointer arithmetic, averaging and dead `cmp` (A = 0). Redirect corrections applied: RampAirVel entry-1 reads observed; F:$FF2C and 6:$8701 reads are rts dummy reads.
- Editorial fixes: event/hill $0493.. LaneTerrainBuf (80 operands, was PaletteBuf+N); MetatileRowOfTileRow named (was wrongly "unread"); race $04AF RespawnSearchCount; CheckCanAfford pointer constants symbolic; 223 comment blocks lost a stray "| " prefix; all 17 remaining generic branch labels renamed (0 generic left).
- Audit (`commentary.py audit`, run once at the end): ok, byte_identical true, 1,401 regions and 1,321 units reviewed, editorial_audit reviewed, reading copy refreshed.
- Handoff: analysis/handoffs/round-04-commentary.md. Toolkit: no new issues. Status: complete.

### Round 4 — redirect (2026-11-20, attempt r04-redirect-a1)
- Input checkpoint pass-004 (0 unknown). Research only; no map, source, ROM, state, scenario or toolkit changes. Runtime: 2 bounded Read-watch runs (2 x 2400 frames, 1P and 2P race, A held; serve without --store, no RAM edits).
- Changed premise: round-4 commentary showed MetatileRowOfTileRow is read through an indexed base below the table; r03 refs.py ignored such operands. New indexed-base scan (tools/idxscan.py, control on F:$D157) plus consumer index-domain review over all unread generic blocks.
- Produced (ready, 0 new bytes): L-027 (3:$88B5-$89B4 = 256-B 16x16 product table, reader MetatileTileIndex in ExpandMetatileTiles/Attrs; explains 3:$894D and $8991 head), L-026 (0:$9087-$90B1 = MapTruckPath tail, 8 stops x 8 tiles), L-025 (0:$BE6F-$BE77 = SparkStartY entries 15-23; tail not observed - no natural truck-1 repair in the watches), L-028 (commentary: LaneNametableHi+2 / TileReplaceLaneHi+2 are read).
- Islands (22, 650 B) and other unread generic blocks: unchanged dispositions; rejection review in the handoff.
- Evidence: analysis/redirect/r04-redirect-a1/. Handoff (with rejection review): analysis/handoffs/round-04-redirect.md.
- Recommendation: one short disassembly/commentary cycle for L-025..L-028; afterwards terminal unless a lead is rejected.

### Round 5 — disassembly (2026-11-20)
- Restored analysis/state.json (= pass-004, no re-baseline). Applied 12 map.annotate edits + 3 map.remove (3 new regions; merges/splits/relabels only, no byte changed kind; region count 1,401 -> 1,401). Checkpoint: **pass-005** (maps/pass-005.json, generated/pass-005, analysis/pass-005-verification.json): validate ok, byte_identical true, map sha256 184532e6...
- ROM classification: 100.00% (262160 / 262160 bytes classified; 0 bytes unknown).
- Kinds unchanged: Code 37,907 (26,456 Observed, 11,451 Static), Data 93,181 (Static), Image 131,072 (CHR). Per-bank counts identical to Round 4 (unknown 0 in every bank).
- L-027 verified: bank 3 MulTable16x16 $88B5-$89B4 (phys $0C8C5-$0C9C4, 256 B, region 2025) replaces 766 tail + 1814 + 767 + 1815 + 768 head; 766 = $88AB-$88B4 RowPpuOffsetsB tail; 768 now $89B5-$90F6.
- L-026 verified: bank 0 MapTruckPath $9072-$90B1 (64 B, region 1725; 8 stops x 8 tiles); 664 = $8FFE-$9071; new generic 2026 $90B2-$90EC (59 B, unresolved).
- L-025 verified: bank 0 689 = $BD58-$BE2F; SparkVelY $BE30-$BE47 (1742), SparkStartX $BE48-$BE5F (2027), SparkStartY $BE60-$BE77 (690). Truck-1 tail not observed read; static Y-domain extent.
- L-028 verified: commentary-only; evidence notes on fixed-bank regions 932/938 now name readers F:$C72B / F:$D514. Rejected: none.
- Gameplay: none (no concrete runtime question). Toolkit: no new issues (TK-001..TK-006; analysis/toolkit-issues.md).
- Evidence/tools: analysis/disasm-r05/. Handoff: analysis/handoffs/round-05-disassembly.md.
- Status: all bytes classified; commentary must apply the new labels/extents and fix the L-025..L-028 notes and uncertainties.

### Round 5 — commentary (final acceptance) (2026-11-20)
- Map changed (pass-005): whole round-4 commentary preserved at analysis/round-4/commentary/ (round-1..3 copies untouched). Fresh baseline prepared from pass-005; notes/tools carried by physical range (7 stale split keys removed; checknotes 0 problems). 1,384 entries kept prior credit; 17 changed entries re-reviewed as round 5.
- Changes: MetatileTileIndex -> MulTable16x16 (product table; inline notes at the 4 reads; stale 3:$892C/$894D/$8975/$8991 notes removed); MapTruckPath = 8 stops x 8 sprite tile ids, AnimateMapTruck header fixed; 0:$90B2 generic note; spark-table notes (stale $BE40/$BE6F removed, truck-1 tail unobserved); L-028 names TruckEndBitMasks (F:$C723) and TileReplaceRowStarts (F:$D450); PieceTableUnreadA/B (0:$BDE8/$BE00). uncertainties.md updated. Generic labels: 0.
- Audit (`commentary.py audit`, once at the end): ok, byte_identical true, 1,401 regions and 1,325 units reviewed, editorial_audit reviewed, reading copy refreshed.
- Handoff: analysis/handoffs/round-05-commentary.md. Toolkit: no new issues. Status: complete.

### Round 5 — redirect (2026-11-20, attempt r05-redirect-a1)
- Input checkpoint pass-005 (0 unknown). Only research, lead and scenario records changed; map, source, ROM, state and toolkit untouched. Runtime: 1 natural-input Read watch of the tour intro (640 frames, no RAM edits).
- Corrected premise: the r03 word hits were never attributed. Data-pointer words inside accepted record structures were followed to their consumers (tools/wordscan.py, storyrec.py, orderlists.py, patterns.py).
- Produced, all ready with 0 new bytes:
  - L-029: 0:$90B2-$9180 = StoryTileRecords, 27 records reached through the StoryTextBlocks data pointers; confirmed read in the intro.
  - L-030: 2:$83D0-$854F = four 48-word channel order-list pointer tables; InstrumentTable $8610-$8707; order lists $8708-$8C14.
  - L-031: 2:$B65D-$BFFF and F:$FF57-$FFF9 are $FF fill.
- Rejection review for the remaining residuals and the gameplay checklist review are in analysis/handoffs/round-05-redirect.md. Evidence: analysis/redirect/r05-redirect-a1/.
- Recommendation: one short disassembly/commentary cycle for L-029..L-031; the next redirect can then be terminal.

### Round 6 — disassembly (2026-11-20)
- Restored analysis/state.json (= pass-005, no re-baseline). Applied 12 map.annotate edits + 5 map.remove (4 new regions; merges/splits/relabels only, no byte changed kind; regions 1,401 -> 1,400). Checkpoint: **pass-006** (maps/pass-006.json, generated/pass-006, analysis/pass-006-verification.json): validate ok, byte_identical true, map sha256 333b439b...
- ROM classification: 100.00% (262160 / 262160 bytes classified; 0 bytes unknown).
- Kinds unchanged: Code 37,907 (26,456 Observed, 11,451 Static), Data 93,181 (Static), Image 131,072 (CHR). Per-bank counts identical to Round 5 (unknown 0 in every bank).
- L-029 verified: bank 0 StoryTileRecords $90B2-$9180 (phys $010C2-$01190, region 2026; 665/1726 removed); 666 = StoryTextBlocks head $9181-$9217.
- L-030 verified: bank 2 ChannelOrderListPtrs $83D0-$854F (2028), InstrumentTable $8610-$8707 (2029), SoundOrderLists $8708-$8C14 (2030); 739 = $83C9-$83CF, 1791 = $8550-$8569, 742 = $85C6-$860F, 743 = $8C15-$8CC4; 740/1790/1793 removed.
- L-031 verified: bank 2 Bank2FillFF $B65D-$BFFF (2031); 764 = $B611-$B65C; 964 (F:$FF57-$FFF9) evidence = $FF fill. Rejected: none.
- Gameplay: none (no concrete runtime question). Toolkit: no new issues (TK-001..TK-006; analysis/toolkit-issues.md).
- Evidence/tools: analysis/disasm-r06/. Handoff: analysis/handoffs/round-06-disassembly.md.
- Status: all bytes classified; commentary must apply the new labels/extents and the L-029..L-031 note fixes.

### Round 6 — commentary (final acceptance) (2026-11-20)
- Map changed (pass-006): whole round-5 commentary preserved at analysis/round-5/commentary/ (round-1..4 copies untouched). Fresh baseline prepared from pass-006; notes/tools carried by physical range (5 stale split keys at 0:$90ED/$9138/$9154, 2:$8B45/$8B74 removed; checknotes 0 problems). 1,350 entries kept prior credit; 50 changed entries re-reviewed as round 6.
- Changes: StoryTileRecords format note (replaces the stale no-reader notes; StoryTextBlocks/StepTextBlock linked); bank 2 sound: ChannelOrderListPtrs (SoundDataLow retired), SongChannelOffsets = absolute table bases, SongHeaders renamed SongPatternBases (pattern bases, not song headers), InstrumentTable 31 x 8 field layout (InstrumentNoiseBytes "1329 bytes" removed), SoundOrderLists command format, SoundPatternArea $8C15-$9C85 (28 continuation notes repointed from removed $8B45), NotePeriods 28 bytes, sound-driver routine/RAM comments and branch labels; ChampionPictureData -> ChampionPalette2 (32-byte palette, $FF fill follows); Bank2FillFF and F:$FF57 FixedBankFillFF described as unread $FF fill. uncertainties.md updated (0:$90B2-$9180, 2:$83D4, 2:$B63D+ removed). Generic labels: 0.
- Audit (`commentary.py audit`, once at the end): ok, byte_identical true, 1,400 regions and 1,330 units reviewed, editorial_audit reviewed, reading copy refreshed.
- Handoff: analysis/handoffs/round-06-commentary.md. Toolkit: no new issues. Status: complete.

### Round 6 — redirect (2026-11-20, attempt r06-redirect-a1)
- Input checkpoint pass-006 (0 unknown). Static only; no emulator runs. Map, source, ROM, state and toolkit untouched.
- Corrected premise: r03 refs.py never populated its 'split' key, so split lo/hi tables were unscanned. New consumer-driven split-table scan (tools/splitscan.py, 77 pairs) and island immediate/split-pair scan (tools/islandscan.py) find no reference to any residual block or no-caller island (one spurious pairing and one coincidental cross-pair explained). Byte-code streams (object/sound/story/palette scripts) and arithmetic pointers were already covered (r02-r05); DMC_ADDR is never written.
- Ready leads: none (leads.json pass entry added, empty). Deferred note: region 742 evidence text still says "SongHeaders".
- Recommendation: terminal handoff. Rejection review and handoff: analysis/handoffs/round-06-redirect.md. Evidence: analysis/redirect/r06-redirect-a1/.

### Round 7 — disassembly (2026-11-20)
- Restored analysis/state.json (= pass-006, no re-baseline). Editorial map-text fix only: 1 map.annotate (evidence text), 0 removals, no offset/length/kind/provenance/confidence/label change; regions 1,400 -> 1,400. Checkpoint: **pass-007** (maps/pass-007.json, generated/pass-007, analysis/pass-007-verification.json): validate ok, byte_identical true, map sha256 9687eb97...
- ROM classification: 100.00% (262160 / 262160 bytes classified; 0 bytes unknown).
- Kinds unchanged: Code 37,907 (26,456 Observed, 11,451 Static), Data 93,181 (Static), Image 131,072 (CHR). Per-bank counts identical to Round 6 (unknown 0 in every bank).
- Changed region ids: 742 only (bank 2 $85C6-$860F: "tail of SongHeaders" -> "tail of SongPatternBases", pattern base address per sound request, consumers 2:801D/$8022, per commentary round 6).
- Obsolete-name scan of all 1,400 region label/comment/evidence texts (SongHeader, SoundDataLow, InstrumentNoise, ChampionPicture, MetatileTileIndex, NotePeriods/1329, $83D4/$8B45/$B63D, PieceTable, story/truck/spark/palette): only 742 contradicted current evidence; 739 (NotePeriods $83B4-$83CF = 28 B), 1791 (SongPatternTables), 743, 764 ($B63D 32-byte palette), 2025 (MulTable16x16), 2028-2031 already consistent.
- Leads: none ready (round-06 redirect). Gameplay: none. Toolkit: no new issues (TK-001..TK-006; analysis/toolkit-issues.md).
- Evidence/tools: analysis/disasm-r07/ (plan.json, tools/apply.py, state-before-r07.json). Handoff: analysis/handoffs/round-07-disassembly.md.

### Round 7 — commentary (final acceptance) (2026-11-20)
- Map changed (pass-007: region 742 evidence text only). Whole round-6 commentary preserved at analysis/round-6/commentary/ (round-1..5 untouched); fresh baseline prepared from pass-007; notes/tools/progress/uncertainties carried.
- Rebuilt source/game.asm, ram.inc, hardware.inc byte-identical to round 6; checknotes 0 problems; 0 generic labels. 1,399 entries keep prior review credit; region 742 re-reviewed (round 7), source SongPatternBases note agrees with new evidence; the round-6 map/commentary name mismatch is resolved.
- Final audit (run once): ok, byte_identical true, 1,400 regions and 1,330 units reviewed, editorial_audit true, reading copy refreshed. No remaining known editorial issues; open meanings in commentary/uncertainties.md. Handoff: analysis/handoffs/round-07-commentary.md.

### Coordinator — terminal handoff (2026-11-20)
- Final checkpoint pass-007 (validate ok, byte_identical). Coordinator re-ran map validate and commentary verify: ok / byte_identical.
- ROM classification: 100.00% (262160 / 262160 bytes classified; 0 bytes unknown).
- Terminal basis: R6 rejection review accepted; R7 note-only fix did not change its premises. Residual semantic uncertainties in commentary/uncertainties.md.
- Final handoff: analysis/final-handoff.md; archive-options.json written (archive not executed).
