# Commentary uncertainties

Carry forward relevant phase-1 ambiguities as their regions are reviewed.
Round 1 commentary (2026-11-17) on checkpoint pass-003; round 2 (2026-11-18) on pass-004
(map sha256 c1ceb883...); round 3 (2026-11-18) on pass-005 (map sha256 407f32c4...); round 4 (2026-11-18) on
pass-006 (map sha256 ac37c43f...); round 5 / final acceptance (2026-11-18) on pass-007
(map sha256 d0dd6e1f...).

## Structural questions (for redirect; classification unchanged here)

1. Overlapping entry at bank 13 $A07B (phys $1A08B).  `FieldCommonUpdate` (bank 14
   $DB7F) calls $A07B every field frame with banks 12/13 mapped.  Since pass-006 the map
   classifies $A07A-$A0B7 as code from $A07A (`sbc #1` fall-through: `bcc +1 / rts /
   jmp ThrowToBase`, label CpuThrow_ThrowIfBaseNear, L-R2-04).  The second reading from
   $A07B (`ora ($60,x)` / `jmp ThrowToBase`) cannot be a map instruction boundary
   (TK-003); the source defines `FieldUpdateThrowEntry = * + 1` on the BCC and the
   bank-14 call uses it, byte-identical.  Open: whether the overlap is intentional and
   what the ORA does to A on entry.  Check: bounded trace of one field play, confirm the
   executed start and A before/after.
2. (Round 2) The table-referenced handlers listed here in round 1 (states 16, 30-37,
   40/41, the pre-game/result/fielding-side pages, runner state 5, bank 3 $B39E, the
   bank 14 prologues) were classified in pass 4 and are now commented code.
   (Round 3, pass-005) The code-shaped fragments bank 3 $AC36, $AC73, $B496; bank 7
   $B820; bank 9 $ACC2; bank 12 $8DE7; bank 14 $C126 are now Static code (L-R4-01) and
   commented as such (SetupSprite0Split3_Copy, TileXYToPpuAddr_B3,
   MainMenu_EraseCursorItem, MarkTeamEmptyIfAllFired, PutHexBlankPadded,
   RunnerSafeAtThrowBase, PpuBufAppendHexInline).  None has a known caller (only
   TileXYToPpuAddr_B3 is called, by MainMenu_EraseCursorItem); reachability is
   unresolved, so the source says "no known caller", not "dead".
   (Round 4, pass-006) bank 12 $9026-$9028 is now Static code (L-R5-01),
   RunnerInput_ReturnOldTail, `jmp RunnerMoveAlongPath`, no known caller (read as a
   superseded tail of RunnerInput_Return).  bank 13 $A07A: item 1.
   (Round 5, pass-007) bank 10 $9B08 (phys $15B18, 1 byte, $60), the last Unknown byte,
   is now generic Data only by a user-directed Manual fallback (confidence 0.5, no
   label).  This is not evidence of a data role; its meaning is unknown.  The source
   writes it as `.byte $60` with a note.  Still open (L-R3-01): a leftover RTS after
   NameGrid_Right's `jmp PlaceGridCursor` (a jsr->jmp tail-call edit) or a pad byte.
   No reference, branch, fall-through or fetch reaches it; only execution or a consumer
   could decide, and no further research is planned.
3. Bank 12 $9DD4-$9E17: CpuFieldingInput's jump table (34 words) is phase-1 Data; the
   source formats it as words (annotation `word 12:9DD4 34`); targets were not
   re-checked against classification.
4. (Round 2, new) State15_RestartStep (bank 15 $EB87, MenuStep 3 of MainState 15) maps
   PRG value 4 and falls into WaitSidePressA, whose `jsr $9D22` then reaches bank 8 data
   instead of bank 2 CpuMenuScript (the state dispatcher re-maps PRG value 1 only on the
   next frame).  Either step 3 is never selected or the call is skipped (forced change,
   PitcherChangeState = 2).  Bounded check: watch MenuStep in a substitution run and
   whether $EB87 ever executes.
5. Data-structure readings that cut across region boundaries (classification now data
   in all cases): NameEntryMap/ViewTeamMap (bank 10 $8A7C-$8AAF), ScreenMapGameResult
   attributes (bank 2 $8A1B-$8A3F), TitleMetatiles (bank 1 $AA26-$AA42), LineupMapB
   continuing into bank 9, bank 1 formation tables inside CpuPitchPlans.  MapTablePtrs:
   entries 16-19, 22 and 23 are now used by the FIRE/HIRE/champion/pitcher-list
   screens (bank 3 $BC30/$BD70 are the champion team maps, not metatile tails); only
   entry 11 ($87C0) has no caller.

## Lead correction (confirmed in round 1 redirect)

- L-R1-01 (leads.json) states the main-menu conditions partly inverted.  State6_MainMenu
  (bank 3 $AE85-$AF56) counts used custom slots ($6790-$6795, bit 7 clear) in $0545 and
  erases: TRADE ($21C2) when count < 2; HIRE ($21D2), REMOVE ($22C2), FIRE ($22D2) when
  count = 0; MAKE TEAM ($2252) when count = 6.  So one custom team should already show
  HIRE/REMOVE/FIRE, two show TRADE.  If one MAKE TEAM did not show them, check whether
  the slot flag ($6790+slot, cleared by GenerateCustomTeam via $6788+TeamForSide) was
  actually written in that run.

## Semantic interpretations kept tentative in source

- Object 13 treated as the umpire (pose follows the OUT/SAFE/FOUL call); the earlier
  "ball" reading in ram.ann was revised.
- `SpecialPlayObject` ($0577) and AssignFieldersSpecialPlay: a special fielding setup
  (probably pick-off/steal); exact play unconfirmed.
- Win/loss/save bookkeeping ($67C1-$67C6, DrawPitchersOfRecord), ERA-like ratio
  (UpdatePitcherSeasonStats), batting averages: inferred from bookkeeping patterns.
- TeamMoney ($6839, decimal digits) and POWER UP costs; the $6899/$689B team totals.
- MAKE TEAM secret code (SecretCodeSequence / strength 5 picked four times) and the
  special names/rosters: mechanics clear, intended purpose inferred.
- Field-map bits used by MoveObject/CheckFenceCollision/CheckCatchableHeight (top two
  bits = obstacle/fence): inferred.
- Ball flight units ($B0-$B9, BallHeight $D9, BallAirTime $04B1) and CPU AI thresholds.
- The two CHR/attribute meanings of FontTiles quarters in bank 8 are inferred from the
  4x256 layout.

## RAM reuse to keep in mind when reading

Several per-object arrays and ball/zero-page variables are reused as scratch by menu
code: ObjXHi/ObjYHi pairs as record pointers (bank 7), ObjState.. as menu variables,
ObjTimer ($0660) and $068A as text buffers, ObjBase ($0698) as team-record pointers,
TeamScore/Balls/Outs as scratch in MAKE TEAM and lineup states, BaseDist*/OverlayMap*
in league screens, PitchResultFlags as the leader page index.  Symbols follow the
dominant (field-play) role; the reuse is noted in ram2.ann and the bank headers.
Zero-page ball variables ($B0-$BF, $DB) used by menu code now carry scoped scratch names
(annotation keywords `scratch`/`scope`): rounds 2-3 for banks 2/3/7/15 pages, round 4 for
the rest (MenuCursor/MenuCursor2, NameCharPos, PointSum/PointMax/PointRoom, LeaguePick*,
Champ*, Num*, Leader*, LogoFade*, Msg*, SwapPick1/2, ResultTilePtr, ...).  No Ball*
operand is left outside field-play code (bank 1's CPU batting/pitching AI uses them in
their field role).  Per-object arrays reused as menu variables (ObjState.., ObjTimer,
ObjBase, TeamScore etc.) still use their field names, as described above.

## RAM roles named in resume 4 (ram3.ann) that stay tentative

- PlayEndHold ($2C): only written as 0 or by its own SELECT toggle, so the hold appears
  unreachable (debug leftover?).  WriteOnly04F4, BallReleaser ($048C), BallSpriteX/Y,
  BatterByte11Lo, BenchListWriteOnly, MenuResetFlag: written only.  $0547 is now
  FieldChangeCount (writers found in the pass-4 defensive-change pages).
- Shared/multi-role bytes named after their dominant use: SwingOrSideIdx ($0490),
  HitAdvance ($0482), HitStatState ($04BC), BallHomeDist ($04BD, also contact angle),
  menu scratch $053E-$056B (MenuToggle, MenuPick, ...).
- WRAM statistics: PitcherRunsAllowed/PitcherInnings (index pitcher*2+team),
  TeamHits/TeamErrors, LeagueRankOrder, LeagueSlotStats, LeaguePitcherRest/Played,
  PitcherStaminaSave - roles inferred from the update code, not from screen output.
- Bank 2 $8000-$83FF as a metatile set is a shape-based reading (no reader found).

## Round 2 interpretations kept tentative in source

- CustomTeamFlags: corrected to 0 = slot holds a team, bit 7 = empty (round 1 had it
  inverted; MAKE TEAM writes 0, REMOVE TEAM writes $FF, the menus skip bit-7 slots).
- LeagueSlotStats: $FF marks a fired player's free roster slot (FIRE writes $FF, HIRE
  searches for it and stores 0; TRADE swaps the bytes).
- END PLAY (main-menu item 8, MainMenu_EndPlay): scroll one page, wait, then idle for
  good - read as "stop before power-off" from the item text; not observed at runtime.
- Pitcher list "d.dd" digits (pitcher record bytes 4-5) read as the ERA; the rested /
  normal sprite markers come from LeaguePitcherRest.
- Result-screen prize roll (GameResultSteps 1-5): digits taken from ObjYLo used as a
  6-digit buffer (ResultDigits), probably the prize money of AwardTeamMoney; the
  8-sprite figure animation's picture was not checked.
- HIRE PLAYER generation: record bytes are filled from HireBatterTable /
  HirePitcherTable / HireTypeTable with random picks; byte meanings follow the bank 8
  team-record notes and stay partly inferred.  The contract price compared against
  TeamMoney sits in ObjFlagC (as for POWER UP).
- TRADE balancing (Trade_Execute): when the two players' nibble totals differ by 10
  or more, the stronger player's abilities move toward the weaker one's; exact intent
  (fairness rule) inferred.
- Champion card glyph tiles $DB/$DC read as W/L.

