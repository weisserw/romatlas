# Commentary progress

Prepared verified baseline. All semantic review entries are pending.

## Round 1 commentary pass (2026-11-15) - INTERRUPTED (scope not complete)

Baseline: prepared from maps/current.json = maps/pass-001.json (2,953 regions; review.json scaffold 3,749 entries).

How source/ is produced (read before editing):
- source/ is rendered by `python3 commentary/tools/render.py` from the frozen before/game.asm plus the
  notes database commentary/notes/*.py (R = routine name+doc, L = label, D = data label, V = RAM symbol,
  K = constant, B/C = block/end-of-line comments, JTX/WT = jump/word tables with pair context, BANK, UNK).
  Names are keyed by (PRG bank, CPU address) so they can be carried by physical range in later rounds.
  Hand edits to source/*.asm would be overwritten: edit notes/ and re-render. before/ is never touched.
- Renderer features: bank-resolved labels for every branch/JSR/JMP/data reference (MMC3 mode 0 pairs,
  banks 30/31 fixed, cross-bank branches), cheap-local @loop/@skip/@return labels inside routines,
  fixed-bank `lda #n / jsr SelectPrgPair / jsr $8xxx` pair resolution, FarCall target comments,
  `.addr` rendering of $C316 inline jump tables (GameModeHandlers with per-mode pairs), FarCallTable,
  CPU vectors and InlineParamCall words (with text previews), hardware register symbols, RAM/PRG-RAM
  symbols with sizes, button-mask constants, per-bank overviews, unknown-region boundary notes, generic
  data notes and a mechanical summary for every routine not yet explained ("Purpose not yet established").
- Output split: game.asm (includes) + hardware.inc, ram.inc, header.asm, bank00-31.asm, chr.asm.

Covered (code read and explained; routine names, data structures, RAM symbols):
- Bank 30 ($C000-$DFFF) completely: banking, PPU queue/flush, inline-call helpers, dispatchers, FarCall
  and its table targets, PlaySound, math (sine scaling, divisions, decimal), text drawing, screen
  decompressor (metatiles), roster/team copy routines, modes 2-5 (at-bat and fielding loops), stats.
- Bank 31 $E000-$E370 and $F2DE-$FFFF: reset, main loop, mode dispatch tables, NMI/IRQ, layout CHR and
  split handling, palettes, joypads, clocks, sound glue, trampolines, InitGame/save-signature checks.
- Bank 0 $8000-$81FF (sound driver entry/update loop) and the driver RAM.
- All 42 game-mode table entries resolved and named where classified (modes 16-19, 30, 31, 36, 37, 40,
  41 point at unclassified bytes: lead L008).

Named from mechanical summaries/caller evidence only (still to be read, marked pending in review.json):
- Bank 31 $E371-$F2DD (sprites, scoreboard panel, batter/pitcher control, CPU manager), bank 0 $8200-
  $9FFF (driver internals), bank 1 (title modes and computer-player input).

Remaining scope (routines still carry only the mechanical auto summary; 692 routines):
- Banks 2 (79), 3 (75), 5 (18), 8 (56), 9 (53), 10 (51), 11 (52), 12 (99), 13 (84), 14 (21), 15 (14),
  24 (45), 25 (45) - bank N occupies physical 16+N*8192 .. 16+N*8192+8191 (e.g. bank 12 = $18010-$1A00F).
- Mode handlers in those banks are named; their internals are not.
- Intra-routine branch labels are mechanical (@skip/@loop/@return) almost everywhere; descriptive local
  labels exist only for key routines in banks 30/31.

review.json: 1,094 entries reviewed (bank 30, read parts of bank 31/0, data/unknown outside pending code
ranges, header, CHR), 2,655 pending; 297 units recorded (reviewed only where read). Updated by
commentary/tools/review.py. editorial_audit stays pending.

Verification: commentary.py verify -> byte_identical true (commentary/verification.json).
- Worker #3: banks 2/3 finished, banks 12/13 read (notes 75/76, bank-specific RAM aliases via BV in render.py); verified byte-identical. Remaining: 8, 9, 10, 11, 24, 25, 14, 15, 5.
- Worker #3: banks 8/9 read (notes 77/78); HomeStadium/StadiumChrSel renamed BatterForm/PitcherForm (record bytes). Remaining: 10, 11, 24, 25, 14, 15, 5.
- Worker #3: banks 10/11 read (notes 79/80). Remaining: 24, 25, 14, 15, 5.
- Worker #3: banks 24/25 read (notes 81/82). Remaining: 14, 15, 5.
- Worker #3: banks 14/15/5 read (note 83). No routine carries the mechanical summary any more; review.json 3,749 reviewed / 0 pending (997 units); verify byte-identical; reading-after.asm refreshed. Leads L010-L012 added.
- Worker #4 (batch 1, notes/84_labels_names.py): all 268 Loc<bank>_<addr> branch targets and all Tbl<bank>_<addr> data labels named from their consumers (tables of one record named once; the renderer shows +1..+3). Stale comment references to old auto names (Loc00_8204, Loc30_C43F, Tbl31_E81D, ...) replaced. No Loc/Tbl/Sub address label left in source (only the Sub05_A123 example in the game.asm legend). Verify byte-identical.
- Worker #4 (batch 2, notes/85_local_labels.py, helper LL()): 621 mechanical @skip/@loop/@return locals replaced by descriptive locals (2,255 -> 1,634) in the game-critical routines: bank 30 JudgePitch/JudgeStrike/AtBatWaitPitch/AtBatNextState/ForceRunnersOnWalk/CheckPlayOver/InitFieldObjects; bank 31 save validation (ValidateSavedTeams, CheckSramSignature, InitSaveData), MoveObject, NMI, sprite-0 split, VRAM jobs; bank 12 computer base running and throw choice, force outs (BallOnBaseCheck), tags, scoring (RunnerScores), pitch steering/strike zone/hit-by-pitch; bank 13 catches, throws, fence/wall, ball integration, home-run messages, fielder tasks; bank 14 bat contact/timing/hit speed/fatigue; bank 8 half-inning end rules; bank 9 scoreboard wait, team points, pitcher season totals, league end; banks 10/11 league menu, standings ranking, roster check; banks 1/2/3 computer batting/defensive subs, time-out/shift/relief menus; bank 25 save, price check and decimal subtraction. Remaining mechanical locals are in display/menu helpers. Verify byte-identical; reading-after.asm refreshed. Lead L013 added.

## Round 2 commentary pass (2026-11-16) - COMPLETE (current-source readability scope)

Baseline: maps/current.json = maps/pass-002.json (3,985 regions; sha256 ebcf3314...), unchanged since the fresh
baseline prepared at 01:48 (round-1 commentary preserved under analysis/round-001/).  review.json scaffold 4,068
entries.  Notes are carried by (PRG bank, CPU address); render.py printed no stale-key warnings.

Worker A (interrupted at ~02:11) wrote notes 86-96: scroller/bank-13 copies (tools/gencopies.py -> 86), fixed banks
30/31 (87), L019 tables (88), banks 9 (89), 0/30 (90), 2/3 (91), 10 (92), 11 (93), 24 (94/95), 25 (96).  Every
pass-002 code run (180 runs, 17,932 bytes; tools/newcode.py) has a named, documented routine or label.

Worker B (this continuation):
- Data of pass-002 (12,446 newly classified data bytes, plus the phase-1 generic pieces of the same structures):
  - render.py: RANGE() role notes for structures split into many map regions, and evidence-derived role notes
    (baseline.json group tags D-screens/D-metatiles/G-text/G-grid/I-island/I-pair3/E-/C-/remnants; E- groups name
    their consumer routines) instead of the generic-data note.  Generic notes 722 new pieces -> 173 left in total
    (phase-1 generic data with no consumer).
  - notes/98_round2_banks.py: stadium pairs 2/11/13/14 - FieldMap/FieldMapTail/FieldAttrMap/FieldMetatiles labels with
    the row/attribute/metatile formulas (spare map columns $70-$7F hold LookupFieldCell cell properties), at-bat set
    and screen copies; pair-10 metatile block; bank overviews refreshed for banks 4-6, 11, 14-29.
  - notes/97_round2_screens.py (tools/genscreens.py from screen_sites.json + ScreenDataTable): 92 screen records
    named after the routine that draws them (23 index-only ScreenRecordNN), 19 metatile sets, RANGE notes per record.
- Unknown runs: all 83 (1,727 bytes) carry an inspection note (notes/99_round2_unknown.py); settled table pieces
  (PositionLetters/PositionLettersB9 entries, TeamBlockDstPtrs tail, labelled consumer runs) published as lead L023.
  TeamBlockSrcPtrs/TeamBlockDstPtrs descriptions corrected (six words $B760/$77B0 + $150*n).
- Names: descriptive locals for the bank 5 scroller and bank 13 camera code (85b; carried to all 9 copies by
  gencopies), all forward branches in pass-002 code (96b), shape-based names for 177 generated @skip labels across
  the ROM (tools/genlocals.py -> 85c: @clamped/@wrapped/@no_carry/@sign_done/@no_<call>/@continue).  RAM:
  EditWorkBuf/EditWorkByte (pair 12), TradeSwapBuf, StripAttrBuffer, ColumnBuffer extent (+ operand fixes where
  $0101,x was shown as Page1Temp01), PpuBufferData extent.
- review.json: tools/review.py updated for round 2 - 4,068 reviewed / 0 pending, 1,925 units (routines plus
  documented data structures); editorial_audit stays pending.  uncertainties.md: round-2 section.

Remaining editorial residue (not blocking; same standard accepted in round 1): 888 generated @skip, 381 @loop and
238 @return branch labels inside documented routines (mostly display/menu helpers of banks 2, 25, 30, 12, 11, 10, 24,
31; exact per-bank @skip counts 0:48 1:41 2:92 3:41 8:35 9:38 10:63 11:69 12:60 13:54 14:20 15:25 24:75 25:92
30:79 31:56), and raw absolute operands for multi-use scratch RAM whose role is not established.

Verification: commentary.py verify -> byte_identical true (fresh, after the final render); reading-after.asm refreshed.

## Round 3 commentary pass (2026-11-16) - COMPLETE (current-source readability scope)

Baseline: maps/current.json = analysis/state.json = maps/pass-003.json (4,101 regions; sha256 c36cb8af...); the
commentary baseline was re-prepared from it (review scaffold 4,107 entries). The round-2 commentary is preserved
whole under analysis/round-002/commentary (manifest map sha ebcf3314... = pass-002); round 1 stays under
analysis/round-001. Notes carry by (PRG bank, CPU address). All round-1/2 notes rendered on the new listing with no
stale keys, and the carried source verified byte-identical before any round-3 edit.

Changed bytes (tools/diffbase.py): 116 runs, 1,725 bytes (Unknown->Code/Data plus 23 Data->Code in bank 25).
- notes/99c_round3_new.py: every pass-003 code run has a named routine with a mechanical description and the
  "no known caller" fact, e.g. DrawSideRosterSheet, PrintEditedTeamNameBox, DrawPlayerIcon, FormatMoneyLine +
  PutThreeDigits/PutCommaThenDigits, SaveSideTeamBlock, GridCellVramAddr, MainMenuReenterAtCell, QueueDigitOrHex,
  GenRenumberRecords/GenRemapLowBits, LeaderValueDigits, CheckRunnerAtThrowBase, InfoNumberToDigits/Divide16By8,
  DrawInfoRectLeft/Right/SelectInfoRect, QueueRemoveTeamMarks, ZeroIfNegative, ReloadBgChrSet/ReloadSprChrSet,
  JumpBigTextDirect, FlushPpuDirectFixed/ClearPpuDirectFixed. New data have consumer labels (GenLowBitsMap,
  InfoRectPtrs, PlayerIconTiles/PlayerIconVram, DollarSignPos, SideTeamBlockBase, FarCallEntry14) or REGION role
  notes (series tails, text runs, duplicates). No "Purpose not yet established", Sub/Loc/Tbl label or generic-data
  note remains in pass-003 bytes.
- The 6 unknown runs (25 bytes) carry round-3 inspection notes. Stale prose was corrected: "unclassified in
  pass-002" phrases, MarkChampionRow, MenuBlinkWaitStep, DigitTiles, EditedTeamNamePtrs, TeamBlockDstPtrs and the
  bank 2 PitcherListDrawAlt lead reference (-> L025).
- Editorial residue: notes/99d_round3_locals.py names 1,270 branch labels routine by routine across banks 0-3,
  8-15, 24, 25, 30, 31. Generated labels went from 888 @skip / 381 @loop / 238 @return to 0 / 0 / 242. The
  remaining @return labels each mark an RTS target. RAM aliases: OamWriteOffset ($048A; DigitOutIndex in bank 9),
  RosterSetupSide/RosterSetupIndex/RosterFillCandidate (bank 8), TeamPointsPtr (bank 25), InfoDigits (bank 15).
  Raw scratch operands: 1,573 -> 1,518.
- Tools: tools/genlocals.py keys labels of routines that cross into the next bank correctly (removes the
  long-standing bank 10 $A0EA render warning); tools/db.py accepts REGION; tools/review.py carries round-2
  credit for unchanged (offset, length, kind) entries and writes round-3 summaries. Pipeline order is unchanged:
  gencopies, genscreens, genlocals, render, review.
- review.json: 4,107 reviewed / 0 pending, 1,967 units; editorial_audit stays pending.
- uncertainties.md: round-3 section. Leads: L025 produced (bank 2 inline words still classified as code), L021
  updated.

Verification: commentary.py verify -> byte_identical true (fresh, after the final render); reading-after.asm refreshed.

## Round 4 commentary pass (2026-11-16) - COMPLETE (current-source readability scope)

Baseline: maps/current.json = analysis/state.json = maps/pass-004.json (4,107 regions, sha256 077d58cd...). The
commentary baseline was re-prepared from it, giving a review scaffold of 4,111 entries. The round-3 commentary is
preserved whole under analysis/round-003/commentary (copied and diff-checked before preparing; pass-003 c36cb8af...).
Rounds 1-2 stay under analysis/round-001 and analysis/round-002. Notes and tools were carried by (PRG bank, CPU
address), and the carried source verified byte-identical before any round-4 edit. The only render warnings were
the three end-of-line notes on the former F01 code bytes, which were then removed.

Changes:
- notes/99e_round4.py adds the pass-004 changes:
  - LoadChrBankTemp (bank 30 $C0C0) and AddRecordStrideTemp0C (bank 31 $F6C2, with its defective high-byte add
    commented), both "Static code without a known caller (pass-004)".
  - Role notes for the F01 inline words at bank 2 $8C52 (VRAM $2204) and $8CA0 (text buffer $00BA).
- tools/render.py now prints a REGION note above an inline-parameter `.addr` block when one exists.
- Superseded UNK notes for $C0C0 and $F6C2 (notes 87 and 99c) were removed. The three C() notes on the old
  inline-word "instructions" (notes 91) were removed.
- PitcherListDrawAlt prose was rewritten:
  - The L025 sentence was replaced with a description of the data split.
  - The value is described as "d.dd" (probably ERA).
  - The raw $053D was replaced with MenuChoiceB.
- The 4 residual unknown runs got a pass-004 re-check sentence.
- Concrete defect fixed across the older material: ten address-only trampoline names were renamed after their
  targets, and every reference followed. FarCall14_8F74 became FarDrawTeamEmblemBoard. The CallBankNN_XXXX names
  became:
  - CallLoadShiftPositions
  - CallPitcherFatigueStep
  - CallStadiumPicker
  - CallLoadPaletteSet
  - CallScoreboardBackground
  - CallHomeRunScoreboard
  - CallDrawInfoScreen
  - CallAnimateArrowTiles
  - CallCheckRostersPlayableAlt

  No `_XXXX` address-only label remains. The only labels with an address in them are PrintTeamNameAt2448 and
  DrawHeaderAt2052, where the address is the VRAM position they print to.
- 21 evidence-changed regions (11 Static -> Observed, 10 partly observed): routines and comments in bank 1
  CpuConsiderRelief, bank 2 PitcherOfRecordCheck, bank 9 CheckLeagueFinished, bank 10 DrawStandingsScreen and
  bank 11 RankStandings, LeagueMatchSelect, MarkChampions, GridHighlight, Mode40/41, ChampionBanner,
  ChampionScrollStart, PrintChampionName and ChampionBgColour were re-checked. Nothing contradicts the observed
  starts, and the region headers show the new provenance.
- tools/review.py takes credit from the round-3 review.json and gives round-4 summaries to new, re-split and
  evidence-changed entries. tools/diffbase.py and tools/shownew.py now compare against round 3.
- Pipeline order is unchanged: gencopies, genscreens, render, genlocals, render, review. The generated notes are
  unchanged.

State: 0 "Purpose not yet established" summaries and 0 @skipN/@loopN labels. 242 @return labels remain, each marking
an RTS target. review.json has 4,111 reviewed / 0 pending and 1,969 units. editorial_audit stays pending.

Verification: commentary.py verify gives byte_identical true. It was run fresh after the final render, and the result is
in commentary/verification.json. reading-after.asm was refreshed with commentary.py read.

## Round 5 carry-over (2026-11-17, pass-005 map, user-directed manual fallback)

- Map pass-005 (sha256 d09c3179...) differs from pass-004 only by four new Manual Data regions covering the former
  8 unknown bytes (b2 $80DA-$80DB, $9988-$998B, $9B48; b9 $A165). Fresh `prepare` showed generated code identical
  apart from region comments.
- The round-4 commentary is preserved unchanged at analysis/round-004/commentary/. Its source/ was carried into
  commentary/source; the 4 site headers were rewritten to state the manual fallback and keep the prior ambiguity note.
- review.json: 4,107 entries carried by identical id/offset/length/kind (source line refs shifted for the 8 inserted
  header lines in bank02/bank09 and re-checked line-for-line); 4 former unknown-* entries replaced by the new
  region-5155..5158 Data entries, marked reviewed with fallback summaries.

## Round 5 commentary pass (2026-11-17, pass-005 map)

- Scope: all classified source in commentary/source (banks 0-31, header, CHR) and the full review.json inventory.
- Inventory: all 1,356 Code entries that had no units now have them (1,484 new units; 3,453 units in total).
  Each unit is named after the routine or block label that holds it in source. If a fragment starts mid-routine, the
  unit is named after the enclosing commented routine (searching back into the previous bank of the pair for
  routines that cross from $8000 into $A000). The summary comes from that routine's source comment and lists the
  fragment's branch labels. Source refs point at the fragment's region header and the routine label. Labelled
  blocks inside an entry get their own unit, pointing at their comment and label. Scaffold id/offset/length/kind are
  unchanged and all 4,111 entries stay reviewed. editorial_audit stays pending.
- Source: added comments to 105 labelled blocks that had none. Each of these labels starts after an rts, jmp, data
  block or inline parameters, for example ShiftName*/ClearShift*/DrawShift* (bank 2), LeagueTeamRemove (10),
  CpuThrowIfLate/CpuThrowOrRun (12), FielderTryJump/FielderLowCatch (13), the card option handlers (25) and
  JudgeBallFour/JudgeStrike (30). Added inline target notes to 14 raw cross-pair calls in banks 30/31. These cover
  pair 6 targets in bank 12/13 and DrawPanelMessage/DrawCountPanel in pair 4. Operands stay as raw addresses
  because the phase-1 context for the switchable slots is unresolved there.
- Name fix: the bank-25 pitcher/batter card labels were the wrong way round. PlayerSelectPickPlayer sets
  ObjTask+1 = 0 for roster places 0-5 (pitchers), and DrawPlayerCard jumps to the old "DrawBatterCard" when
  ObjTask+1 = 0. The names are now swapped back:
  - DrawBatterCard became DrawPitcherCard.
  - @pitcher_card became @batter_card.
  - PitcherCard{Bars,EditOptions,Mode21,Mode23,DefaultOptions} and BatterCard* swapped names.
  - PitcherCardNoOption became BatterCardNoOption and PitcherCardOptionsAlt became BatterCardOptionsAlt.
  - Screen/metatile records became ScreenPitcherCard and MetatilesPitcherCard (bank 18/20).
- Unchanged: the 8 manual-fallback bytes (regions 5155-5158) remain generic Data of unknown meaning.
- Verification: commentary.py verify gives byte_identical true. It was run after each edit batch and again at the
  end. reading-after.asm was refreshed with commentary.py read.

## Round 5 final acceptance: editorial audit (2026-11-17)

- Scope: a targeted final review of commentary/source and review.json. This was not a full reread, and earlier
  review credit was kept. Details are in analysis/handoffs/round-005-final-audit.md.
- PendingPitcherChange ($0576): the comments disagreed about it and are now consistent. The code sets it to 1 when
  the pitcher is replaced by a pinch hitter or runner, either from the time-out menu or by the computer manager.
  Mode9HalfInningEnd then raises it to 2, which forces the time-out pitching change. Mode14TimeOut and
  TimeOutChooseSide had called value 2 "computer-vs-computer play", and TimeOutChooseSide fakes A for side 1, not
  side 0. ram.inc and both routine headers are corrected.
- Card labels, following the round-5 swap: ScreenPlayerCard is now ScreenBatterCard (bank 18, drawn on the batter
  path of DrawPlayerCard), and MetatilesPitcherCard is now MetatilesPlayerCard (bank 20, used by both cards). The
  swap itself was checked: ObjTask+1 = 0 for roster places 0-5, in both ReleasePickPlayer and
  PlayerSelectPickPlayer.
- Cross-pair notes in bank 30: the first jsr $8274 is now confirmed as pair 6. The other one still says "probably
  pair 6", and the note names the home-run paths that were not checked.
- ram.inc: removed the doubled "banks N: bank N:" prefixes.
- Fallback blocks: in two of them, the prior-ambiguity notes for b2 $80DA and $9988 now use the past tense.
- review.json: scaffold ids, offsets, lengths and kinds are unchanged. Unit names, offsets and lengths are unchanged
  except for the two renames.
  - 4,078 boilerplate entry summaries were replaced with the CPU range, the bank, and that region's own source
    note or unit list.
  - About 130 unit summaries that had been cut off mid-word were restored from their source comments.
  - 904 long fragment summaries no longer copy the whole routine comment; they point to it instead.
  - 33 hyphenation artifacts were fixed.
- editorial_audit is reviewed. `commentary.py audit` passed: ok, byte_identical, 4,111 regions, 3,453 units. A
  separate `commentary.py verify` run afterwards also gave byte_identical true.
