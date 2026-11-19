# Round 1: commentary phase handoff (resume 4)

Game: Baseball Stars (SNK 1989), MMC1, PRG 128 KiB, CHR 128 KiB.
Status: the current-source readability backlog is **complete**. All three items the
coordinator rejected earlier are now closed (see "Scope / readability assessment").
Whole-ROM classification has not changed (96.38%, 9483 unknown bytes) and is still a
separate requirement. editorial_audit stays pending until final acceptance, and the
audit was not run.

## Checkpoint and verification
- Commentary baseline = `maps/pass-003.json` (sha256 0408226e...). `maps/current.json`
  is identical to `commentary/baseline.json`, so the existing commentary was resumed,
  not re-prepared.
- Fresh `commentary.py verify` after the last edit: ok, byte_identical=true, ROM sha256
  a71e4cc8... (`commentary/verification.json`). The reading copy was refreshed with
  `commentary.py read` (`commentary/reading-after.asm`). `input/game.nes` and the
  toolkit were not modified. No servers were started.

## Workflow
`commentary/annotations/*.ann` + frozen `commentary/before/game.asm` ->
`commentary/tools/build.py --dump` -> `commentary/source/`, then `tools/review.py`
fills `review.json`. This round added a new keyword, `alias $XXXX Expr`
(tools/ann.py, tools/build.py). It renders an operand address as an expression such
as `StripBuffer-1` or `CustomTeamFlags-8`, which is useful for loop bases and
team-number-indexed tables. Helper checks: `check.py`, `rawram.py` (raw RAM
operands, now empty), `nohead.py`, `regioncheck.py`, `todo.py`.

## Scope / readability assessment
- (a) Branch locals: 0 address-only locals (`^@(skip|loop|done|loc)_XXXX:`). The 15
  loops that had been named after their reload constant (`@loop_38`, ...) now have
  purpose names (zy-locals.ann). There are 1739 locals in total.
- (b) RAM: `tools/rawram.py` reports **no** raw RAM operand left in any bank.
  `annotations/ram3.ann` adds 126 syms and 15 aliases. Examples:
  RunnerProgressToBase/RunnerToBase (the former $04BF/$04C3 tables), SwingOrSideIdx
  ($0490), MusicStarted ($04F9), TextFontFlag/TeamPickDone ($0740/$0741),
  ObjRespPitcher ($058E, the pitcher charged for each runner), the score digits,
  camera history, the computer pitch plan, and WRAM statistics, custom-team, logo and
  league-rotation tables. Write-only and read-only bytes say so explicitly.
  Prose was made consistent: 4-digit RAM addresses in comments became symbols, ROM
  addresses became labels where unambiguous, stale `dat_*` names were removed, and a
  misplaced bank 3 head was removed.
- (c) Data:
  - The screen-map format is now decoded: 320-byte maps = 240 metatile ids, 16
    unused bytes and 64 attributes at +$100. Metatile sets are four 256-byte corner
    tables. Maps and sets are named in banks 1, 2, 5, 8, 9 and 10, and MapTablePtrs
    plus the $FF9E metatile words are explained.
  - Bank 4: field map layout (128 x 75 metatile ids with the skipped 16th row,
    attributes 64 x 40). The label FieldMapTiles was added.
  - Bank 6: animation records, frame lists and piece lists, plus bank 7
    AnimFacingTable. ObjPrevFrameCount was renamed ObjAnimRecord.
  - Bank 8: font, MetatileSetB/C, three screen maps, and the built-in team-record
    layout (18 x 16-byte players + 5 x 8-byte pitchers).
  - Bank 0: sound data split per id (Music00-Music15, Sfx20-Sfx3A, header formats).
  - Bank 1 $B111 block split into CpuPitchPlans (256 bytes), DeepBallFormationGrid,
    ShortBallFormations and FielderFormations (24 x 9; FielderPosTableBase = $B331).
  - Bank tails identified as $FF fill (banks 1, 9, 11) and described (banks 3, 7).
  - The 35 cross-bank operands left numeric in banks 14/15 carry "probably bank NN
    Label" comments (zx-xbank.ann).
- Counts: 2995 annotation names, 437 RAM syms. review.json has 990/990 regions
  reviewed and 1013 units. regioncheck shows 0 uncommented Unknown or Data regions.
  The only 12+ instruction labels without their own head are 4 fall-through entries
  that the parent comment explains.

Remaining gaps: none in the readability backlog. Individual songs, animations and
field areas are labelled mechanically (by id or address), and their musical or visual
content is not transcribed. This is a deliberate granularity choice.

## Findings (prior list kept; details in `commentary/uncertainties.md`)
1. Overlapping entry at bank 13 $A07B: FieldCommonUpdate calls it every field frame.
   Phase 1 decoded it as `ora ($60,x) / jmp $A165`, which overlaps the tail
   `bcc +1 / rts` read from $A07A (left unknown). Suggested bounded check: trace one
   field play and compare executed starts at $A07A and $A07B.
2. Table-referenced handlers in phase-1 Unknown ranges decode to normal game idioms:
   - MainState 16 ($EBE7) and the state-15 step table ($EB7F)
   - TRADE (states 36/37, bank 15 $EE05-$F491)
   - REMOVE/FIRE/HIRE (states 30-35, bank 7 $B394.. and $B62F-$BE83)
   - league champion (states 40/41, bank 11 $BA95)
   - pre-game/result pages (bank 3 $A0ED, $A12C, $A149, $A290, $A58B, $A689,
     $B03C, $B746)
   - fielding-side substitution pages (bank 2 $9715.., $985C, $99FD..)
   - runner state 5 (bank 12 $8D98)

   These are static redirect candidates, with the tables as entry evidence.
3. Lead L-R1-01 conditions are partly inverted: TRADE needs >= 2 used custom slots,
   HIRE/REMOVE/FIRE need >= 1, and MAKE TEAM is hidden at 6 (bank 3 $AE85).
4. Bank 12 $9DD4 (CpuFieldingInput jump table, 34 words) is Data; its targets were not
   re-checked.
5. New (resume 4): data structures that cross phase-1 boundaries, by the 320-byte map
   stride:
   - bank 10 Unknown $8A7C-$8AAF = NameEntryMap tail + ViewTeamMap start
   - bank 2 Unknown $8A1B-$8A3F = attributes of ScreenMapGameResult
   - bank 1 Unknown $AA26-$AA42 lies inside TitleMetatiles
   - bank 8 LineupMapB runs into bank 9 $A000-$A069
   - bank 1 formation tables sit inside the CpuPitchPlans data block
6. New: these maps have no caller in classified code, so they are probably used by the
   unclassified menu states:
   - MapTablePtrs entries $87C0 (bank 2), $8F80/$90C0/$9200 (bank 10), $A06A (bank 9)
   - bank 8 maps $8900/$8A40/$8B80
   - metatile word $86F5
7. New: PlayEndHold ($2C) is only cleared or set by its own SELECT toggle, so this
   play-end hold looks unreachable (a debug leftover?).

## Unresolved meanings (kept tentative in source)
- Object 13 (read as the umpire) and SpecialPlayObject.
- Pitcher-of-record / W-L-S bookkeeping, ERA-like ratios, and the per-pitcher WRAM
  stats (PitcherRunsAllowed/PitcherInnings).
- TeamHits/TeamErrors, TeamMoney/POWER UP economics, and the MAKE TEAM secret code.
- Field-map terrain bits, ball-flight units and CPU AI thresholds.
- Multi-role scratch bytes named by their dominant use (SwingOrSideIdx, HitAdvance,
  HitStatState, menu scratch $053E-$056B).
- Shape-based readings with no reader found: bank 2 $8000-$83FF (metatile set?) and
  bank 3 $BC40-$BEAF.

## Deferred work
- Final editorial audit (`commentary.py audit`) is reserved for acceptance.
- Classifying the Unknown ranges listed above (redirect), and checking finding 1 by
  trace.

## Leads for redirect
Use the MainState table, the JumpTableAfterJsr/step tables and the unused
MapTablePtrs entries (finding 6) as entry and data evidence for the TRADE / HIRE /
REMOVE / FIRE / league-champion states. Use the 320-byte map stride to reclassify the
small Unknown ranges inside map runs (finding 5).

## Artifacts
`commentary/progress.md`, `commentary/uncertainties.md`, `commentary/review.json`,
`commentary/annotations/` (new this round: ram3.ann, zw-data.ann, zx-xbank.ann),
`commentary/tools/`, `commentary/source/`, `commentary/reading-after.asm`,
`commentary/verification.json`.
