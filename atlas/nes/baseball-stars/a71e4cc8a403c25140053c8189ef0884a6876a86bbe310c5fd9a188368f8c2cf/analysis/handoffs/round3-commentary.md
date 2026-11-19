# Round 3: commentary phase handoff

Game: Baseball Stars (SNK 1989), MMC1, PRG 128 KiB, CHR 128 KiB.
Status: the current-source readability work is **complete** for every classified byte in
pass-005. Whole-ROM classification is unchanged at 262155 / 262160 bytes (5 Unknown bytes)
and remains a separate requirement. editorial_audit stays pending; the audit was not run.

## Scope and checkpoint
- Baseline: `maps/pass-005.json` (sha256 407f32c4...), which equals `maps/current.json` and
  `analysis/state.json`. Because the map had changed since round 2 (pass-004), the whole
  round-2 commentary directory was first copied to `analysis/round-2/commentary/` (checked
  identical with `diff -r`; `analysis/round-1/` untouched). Then `commentary.py prepare`
  built a fresh baseline with 1226 scaffold regions.
- Carry-over: annotations, tools, notes, progress.md and uncertainties.md were copied back.
  All 3402 names landed by bank:CPU address (= physical range), and the first rebuild was
  byte-identical.
- Records invalidated (only these): the old Unknown notes on the 7 L-R4-01 fragments, plus
  the notes for the 3 remaining Unknown ranges, which were rewritten. New records are in
  `commentary/annotations/r3-commentary.ann`.
- Scope covered: the 240 newly classified bytes, their dependants, and the concrete defects
  found nearby (listed below).

## New fragments (Static code, no known caller, not called dead)
| range | name | content |
|---|---|---|
| b3 $AC36-$AC46 | SetupSprite0Split3_Copy | byte copy of SetupSprite0Split3. It sits before ClearSplitMode2, as the original sits before ClearSplitMode |
| b3 $AC73-$AC97 | TileXYToPpuAddr_B3 | 37-byte copy of b2 TileXYToPpuAddr. Its only caller is the next fragment |
| b3 $B496-$B4C4 | MainMenu_EraseCursorItem | blanks the menu item under the cursor, adds it to the MenuCount skip list and fakes Down. Same erase as State6_MainMenu's inline code. Inline .addr TxtBlanksThenVs at $B4A9 |
| b7 $B820-$B83A | MarkTeamEmptyIfAllFired | if all 18 LeagueSlotStats bytes are $FF, writes the REMOVE TEAM flags (CustomTeamFlags, LogoFree; not the Copy) |
| b9 $ACC2-$ACD4 | PutHexBlankPadded | values below $10 print as a blank plus a digit, otherwise falls into PutHexByte |
| b12 $8DE7-$8DFD | RunnerSafeAtThrowBase | if the runner's target base is ThrowTargetBase: UmpireCall = SAFE, ObjSlot = 60 |
| b14 $C126-$C16B | PpuBufAppendHexInline | PpuBufAppendInline variant: inline PPU address, then X stacked bytes printed as hex. Probably a debug print |

## Other readability fixes
- TileXYToPpuAddr (b2 $8CA2): the head said Y*32, but the routine computes Y*64 (2x2-tile cells). Corrected.
- `LogoUsed` was renamed `LogoFree` because the old meaning was inverted: $FF means free (set by
  CheckSaveSignature and REMOVE TEAM) and 0 means taken (set by MAKE TEAM). The RAM comment and the
  CheckSaveSignature, REMOVE TEAM and MAKE TEAM heads were updated.
- Bank 3 grid cursor and team select: field-play ball names used as menu scratch were replaced by
  scoped scratch names: GridCursorPad, GridCursorShape, GridPtr, TeamSelPicked0/1, TeamSelSide and
  MenuBuf. The MultiplyBA, DivideBA, DrawGridCursor, MoveGridCursor and TeamSelect heads were
  rewritten to match. Loop labels now follow the scoped names, via a local build.py fix.
- b13 FieldUpdateThrowEntry: the overlapping decode `ora (OPC_RTS,x)` now names the RTS byte
  instead of the misleading `ObjYHi+11`.

## Verification
- `commentary.py verify`: ok=true, byte_identical=true (commentary/verification.json, fresh run).
- `commentary.py read`: reading-after.asm refreshed (43181 lines). input/game.nes is unchanged
  (sha256 a71e4cc8...).

## Readability counts
0 auto sub_/loc_/dat_ labels; 0 address-only locals; 0 raw RAM operands (rawram.py empty);
regioncheck 0 unexplained Unknown/Data ranges; check.py: all 3412 names and every comment landed.
nohead: 5 entries of 12+ instructions have no head of their own. These are the same explained
fall-through entries as in round 2 (ViewTeam_ShowRosterAt, Mmc1WriteChr1_NoShadow,
SetPrgBankNoSave, MulSin16, Div8Loop).
review.json: 1226/1226 regions reviewed, 1404 units; editorial_audit pending.

## Unresolved (in source and in commentary/uncertainties.md)
- Remaining Unknown bytes, each with a note in the source:
  - b10 $9B08: a lone RTS (L-R3-01).
  - b12 $9026-$9028: a `jmp RunnerMoveAlongPath` that follows a JMP, so nothing reaches it (L-R3-01).
  - b13 $A07A: an overlapping instruction stream (L-R2-04, toolkit issue TK-003).
- Purpose and reachability of the 7 fragments are unknown. "Leftover/debug" is written as tentative.

## Remaining gaps
- No gaps remain in the readability backlog.
- One documented convention is kept: outside bank 3's grid and team-select code, menu code
  still uses the dominant field-play names for reused zero-page ball variables (see "RAM reuse"
  in uncertainties.md).

## Leads
- No new structural leads. MarkTeamEmptyIfAllFired and MainMenu_EraseCursorItem look like an
  abandoned "auto-free empty team" / "erase menu item" feature. A bounded trace of FIRE PLAYER
  firing a team's last player could confirm that $B820 never executes (low value).
