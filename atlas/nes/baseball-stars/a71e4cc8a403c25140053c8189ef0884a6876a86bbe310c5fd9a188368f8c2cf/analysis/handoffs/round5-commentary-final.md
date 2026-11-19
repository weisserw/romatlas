# Round 5: commentary phase, final acceptance

Game: Baseball Stars (SNK 1989), MMC1, PRG 128 KiB, CHR 128 KiB.
Status: **complete**. Current-source readability is complete for pass-007, the final editorial
review is done, and `commentary.py audit` passes.

## Scope and checkpoint
- Baseline: `maps/pass-007.json` (sha256 d0dd6e1f...), which equals `maps/current.json` and
  `analysis/state.json`. The only change from pass-006 is phys $15B18 (b10 $9B08, 1 byte). It is now
  region 3989: generic Data by a user-directed Manual fallback (confidence 0.5, no label).

## Preservation
- The whole round-4 commentary directory was copied to `analysis/round-4/commentary/` and checked
  identical with `diff -r`. Rounds 1-3 (`analysis/round-1..3/`) were not touched.
- `commentary.py prepare` then built a fresh baseline (1225 scaffold regions). Annotations, tools,
  notes, progress.md and uncertainties.md were carried over by physical range. The first rebuild was
  byte-identical and all 3414 names landed. The temporary scratch copy of the old directory was
  checked identical to the preserved copy and then removed.

## Changed records
- **Invalidated:** only the pass-006 Unknown note for 10:9B08, which was in r4-commentary.ann.
- **b10 $9B08** (`commentary/source/bank10.asm`, the region before GridRightWrap):
  - It is emitted as `.byte $60` with a note covering four points:
    - It is a user-directed Manual fallback.
    - Its meaning is unknown and is not backed by evidence.
    - No reference, branch, fall-through or fetch reaches it.
    - Whether it is a leftover RTS or a pad byte is still open (L-R3-01).
  - `review.json` region-3989 is reviewed, with that summary and the unit Bank10_9B08_FallbackByte.
  - The same text is in `commentary/uncertainties.md` item 2.
- **New records:** `commentary/annotations/r5-commentary.ann`, plus small edits in bank00/02/07/10/15,
  r2-bank02, ram/ram3 and bank03 annotations.

## Editorial review findings and fixes
Method:
- Automated sweeps over all banks.
- One sampled block per bank, read in full.
- All known open items.

Fixed:
1. **Wrong role and names.** CallMenuBoxDrawer patches the inline text pointer of a `jsr InlinePtrTo14_Far`
   trampoline, not a JSR target. It is renamed DrawSubMenuText, and its table SubMenuTextPtrs (formerly
   MenuBoxDrawPtrs). The five records are named TxtSubSystemError, TxtSubReliefPitcher,
   TxtSubChangeFielder, TxtSubPinchHitter and TxtSubPinchRunner. The trampoline and MenuTrampoline
   comments are corrected.
2. **Missing structures and boundaries.** These tables are now pointer words with named targets:
   - Bank 2: TeamNameTextPtrs (TeamNameBlock00-11) and MenuTextBlockPtrs (MenuTextBlock0-3).
   - Bank 3: TeamCursorShapes (CursorShape0-4), GridLayoutPtrs (GridLayout0-2) and ResultMascotPtrs (MascotTiles0-11).
   - Bank 9: DefaultLineupPtrs (DefaultLineup0-7, plus WRAM entries) and ScoreboardMessagePtrs (Msg*, with the message format).
   - Bank 0: SndCommandTable and SndTablePtrs.

   TeamBufPtrs (bank 7) is now split from the cursor sprites that follow it (ViewTeamCursorSprites),
   and LoadMenuCursorSprites has a head. The bank 2 menu table cluster has a format head.
3. **FixedTables_FF80.** Entries 16-24 are now named: the bank-10 NameEntry_* routines, which bank 7's
   CHANGE NAME page reaches by far jumps, and FarDrawPortrait. The four `jmp FarCallViaFF80` lines now
   have target comments. A draft wording that called these entries unused was caught and corrected
   before the build.
4. **Misleading certainty.** Unused04F4 and BenchListUnused are renamed WriteOnly04F4 and
   BenchListWriteOnly. No reader was found, and their purpose is unknown.
5. **Vague names and raw addresses.** MenuMiscTable is renamed Sprite0SplitTemplate. The SigConst* RAM
   symbols are commented. The bank-0 period-table head now uses labels instead of raw addresses.
6. **Local tool change.** A "Referenced from" line for a pointer held in a data table now names that
   table instead of the routine before it (`commentary/tools/build.py`). Other local tool changes: a
   `bytes` annotation keyword, and the fallback summary in review.py.

Checked and accepted as they are:
- Fixed-bank cross-bank operands stay numeric with "probably bank NN Label" comments. All of those
  comments were checked against labels.json.
- Short self-describing tables and texts are named by role, and their consumers' heads document the format.
- Field RAM reused as menu scratch is documented in uncertainties.md.

The audit tool cannot judge prose. These findings come from the bounded review above, not from a
full reread.

## Audit and verification
- `commentary.py audit`: ok=true, byte_identical=true, reviewed_regions=1225, reviewed_units=1411,
  editorial_audit=true (`commentary/audit.json`).
- `commentary.py verify` (fresh run afterwards): ok=true, byte_identical=true (`commentary/verification.json`).
- `reading-after.asm` was refreshed (43404 lines). input/game.nes is unchanged (sha256 a71e4cc8...).

## Readability counts
- 0 auto sub_/loc_/dat_ labels, 0 address-only locals and 0 raw RAM operands.
- regioncheck: 0 unexplained Unknown or Data ranges.
- check.py: 3477 names, and every head, eol and pre landed.
- nohead: the same 5 fall-through entries as before, each explained by its parent's comment.

## Unresolved items (in source and in commentary/uncertainties.md)
- b10 $9B08: its meaning is unknown and it is only a fallback classification. Whether it is an RTS or
  a pad byte is not decided.
- $A07B overlap: it is unknown whether the overlap is intentional and what the ORA does to A.
- The fragments with no known caller (from round 3, plus RunnerInput_ReturnOldTail) have unknown reachability.

## Remaining gaps
None in the readability backlog. About 130 small data regions have no head of their own beyond a
descriptive name and "Referenced from", and their consumer routines document the format. This is an
accepted convention, not missing work. Whole-ROM classification is reported separately: 262160/262160
bytes are classified, and 1 of them is the Manual fallback, not evidence-backed.
