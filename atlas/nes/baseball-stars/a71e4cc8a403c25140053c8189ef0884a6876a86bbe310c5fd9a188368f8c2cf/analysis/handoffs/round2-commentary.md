# Round 2: commentary phase handoff

Game: Baseball Stars (SNK 1989), MMC1, PRG 128 KiB, CHR 128 KiB.
Status: the current-source readability work is **complete** for every classified byte in
pass-004. Whole-ROM classification is unchanged (99.91%, 245 Unknown bytes) and remains
a separate requirement. editorial_audit stays pending, and the audit was not run.

## Scope and checkpoint
- Baseline: `maps/pass-004.json` (sha256 c1ceb883...), which equals `maps/current.json`
  and `analysis/state.json`. The map had changed since round 1 (pass-003), so the whole
  round-1 commentary directory was first preserved in `analysis/round-1/commentary/`
  (copied, then checked identical with diff). After that, `commentary.py prepare`
  built a fresh baseline with 1224 scaffold regions.
- Carry-over: names, comments and review are keyed by bank:CPU address, which here
  means by physical range, in `commentary/annotations/*.ann`. They were applied to the
  new frozen listing by `commentary/tools/build.py`. All 2995 round-1 names landed, and
  the first rebuild was byte-identical. Only the records whose region kind changed were
  invalidated: 2 Data->Code ranges and every former Unknown range. These were found with
  `tools/stale.py`, then rewritten or dropped. The new round-2 records live in
  `annotations/r2-*.ann`.
- Scope covered: all ~7.4 KB of new code and ~1.9 KB of newly classified data, plus
  the notes, names and RAM symbols that depend on them in older banks.
  - bank 2 $8A40-$9D21
  - bank 3 $A0ED-$A950, $AFC4-$B057, $B39E, $B6CB-$B8A2, $BC04-$BFFF
  - bank 7 $B36B-$BFFF
  - bank 11 $BA95-$BC74
  - bank 12 $8D91-$8DB4
  - bank 14 $C000/$C01C/$C038 and the small tables
  - bank 15 $EB7F-$F491
  - the small data tables in banks 0, 1, 9-14

## Verification
- `commentary.py verify` was run fresh after the last edit: ok, byte_identical=true,
  ROM sha256 a71e4cc8... (`commentary/verification.json`).
- `commentary.py read` refreshed `commentary/reading-after.asm`.
- `input/game.nes` is unchanged, the toolkit was not modified and no servers were
  started.

## Readability assessment (counts)
- 0 auto-named `sub_`/`loc_`/`dat_` labels.
- 0 address-only locals.
- 0 raw RAM operands (`tools/rawram.py` is empty).
- regioncheck: 0 Unknown or Data regions without an explanation.
- 5 labels of 12 or more instructions have no head of their own. All are fall-through
  entries that the parent comment covers: ViewTeam_ShowRosterAt (which also has a pre
  note), Mmc1WriteChr1_NoShadow, SetPrgBankNoSave, MulSin16 and Div8Loop.
- 3402 annotation names, 439 new in round 2. 151 new block comments. 22 scratch
  aliases.
- review.json: 1224/1224 regions reviewed, 1395 units. Code regions with no unit start
  now get continuation units.
- Tooling, all commentary-local:
  - `scratch` + `scope` give second names to zero-page and object RAM that menus reuse,
    applied only inside the routines that reuse it. Examples: MenuPtrA, MenuBuf,
    HireChoice, TradePick, ResultDigits.
  - `opnd` overrides a single operand.
  - `ctx` sets the bank context of a range. For example, the TRADE code in bank 15 now
    resolves `jsr CustomTeamList_Draw ; bank 07 (state bank from StateBankTable)`.

Examples:
- FieldCursorInput / FieldPage_* (defensive change: two picks, position swap or bench
  swap).
- PreGame_ChangePitcher (STARTING PITCHER list with "d.dd" ERA digits and league
  rest-marker sprites).
- HirePlayer_MakeCandidate (candidate generated from HireBatterTable /
  HirePitcherTable with random picks).
- Trade_Execute (ability balancing when the two players' nibble totals differ by 10 or
  more, then a record swap).
- Champion_ShowCard (W/L/PCT card).
- Result_RollDigits (prize-money digit roll).

## Findings and corrections
1. CustomTeamFlags meaning was inverted in round 1. 0 means the slot holds a team, and
   bit 7 means the slot is empty: MAKE TEAM writes 0, REMOVE writes $FF, and the menus
   skip bit-7 slots.
2. In LeagueSlotStats, $FF marks a fired player's free roster slot. FIRE writes it,
   HIRE searches for it and TRADE swaps it.
3. $0547 was renamed CpuScriptFlag -> FieldChangeCount, now that its writers are
   classified.
4. The bank 2 label FieldingMenuData was removed because it sat in the middle of a
   routine.
5. The bank 3 $A093 area was split into PitcherRecordPtrsB3, PositionTextPtr and
   RestMarkerTemplates.
6. MapTablePtrs entries 16-19, 22 and 23 have callers in the new code. Bank 3
   $BC30-$BEAF holds two champion screen maps, not metatile-table tails. Only entry 11
   ($87C0) is unused.
7. Main-menu item 8 is END PLAY (MainMenu_EndPlay). The former "MainMenu_GameModeAlt"
   is this item.
8. Bank 3 $B39E is the return point of the far call FarPrepareTeamNameText
   (TeamSelect_PreviewSide1).

## Unresolved meanings (kept as tentative prose; see `commentary/uncertainties.md`)
- The END PLAY purpose is inferred from the item text.
- The "d.dd" pitcher digits are read as the ERA.
- The prize-roll digits are probably AwardTeamMoney's prize.
- The figure animated on the result screen was not identified.
- HIRE record byte semantics are partly inferred.
- The intent of trade balancing is inferred.
- The $DB/$DC glyphs are read as W/L.

## Remaining Unknown (245 B, each explained in source)
- b3 $AC36, $AC73, $B496
- b7 $B820
- b9 $ACC2
- b10 $9B08
- b12 $8DE7, $9026
- b13 $A07A (TK-003)
- b14 $C126

## Remaining gaps
None in readability. Granularity is unchanged: songs, animations and field areas are
labelled by id or address.

## Leads for redirect (added to analysis/leads.json)
- **L-C2-01 (low):** State15_RestartStep (b15 $EB87, MenuStep 3) maps PRG value 4 and
  then falls into WaitSidePressA, whose `jsr $9D22` would reach bank 8 data. The
  question is whether step 3 ever occurs. Bounded check: a substitution run that
  watches MenuStep and fetches at $EB87.
- **L-C2-02 (low, semantic only):** confirm the END PLAY screen at runtime.
- Existing leads: L-R2-08 / L-R3-01 still have no new reference evidence, and L-R2-04
  remains blocked by TK-003.
