# Round 4 - commentary phase handoff

Status: FINISHED. The current-source readability pass covers all classified source. The final editorial audit was
not run; it is reserved for final acceptance.

## Input / output identity
- Input checkpoint: maps/round03-final.json (sha256 aab2fb6c6b8120365d94d57a881bad3c0da726c7f5ccf5126a5504d8367b297b).
  maps/current.json, analysis/state.json and the frozen commentary/phase1-state.json all have this hash, so the map
  and state are unchanged since the frozen baseline. Existing commentary was resumed without a re-prepare, and
  before/ and baseline.json were left untouched.
- Output source: commentary/source/game.asm, sha256 e12e1cadbb18da7d8154aaa9177019c1a24cfc2bf96e5df8754028934f5b8f17.
  It is built from commentary/annotations.txt (sha256 9d6bd663...) by analysis/tools/cmt_apply.py. The round-3
  annotations are copied to scratch/r04c/annotations.r03.txt (the round-3 source b70bdbb2... can be rebuilt from them).
- Review inventory: commentary/review.json (sha256 be41c0ad...). All 2293/2293 scaffold entries are reviewed, with
  951 code units plus table-symbol units. editorial_audit is pending.

## Verification and reading copy
- `commentary.py verify` returned ok:true and byte_identical:true (analysis/round04-commentary-verification.json).
- `commentary.py read` refreshed commentary/reading-after.asm (sha256 eb19ccde...) after the final build.

## Classification (unchanged by this phase)
131085 / 131088 bytes are classified and 3 bytes are unknown. Those 3 bytes are the TK-04 cross-window instruction
(bank 0 $BFFE-$BFFF + bank 7 $C000), and the source explains the blocker at both spots. Whole-ROM classification
is therefore not complete.

## Leads
All nine leads were re-checked against the code before they were applied. None was rejected. Leads.json status is
now "consumed", with a round-4 attempt entry on each.

- **L-R3R-01, flying disk.** Confirmed: the routines named for a catcher are really the thrower's two-stage throw
  meter. Names added for the meter, disk and catcher variables, with renamed routines.
  - New: $91-$9C are four 24-bit parallax layers (Parallax1-4). AddToFourCounters moves them 1-4 times per frame.
    UploadObjectColumns was renamed UploadParallaxStrips, and Bank6BuildObjectStrips was renamed
    Bank6BuildParallaxStrips.
  - New: an overview strip of minimap sprites (DiskMapOam).
  - New: after a catch the disk bobs in the catcher's hand (DiskFlightUpdate).
  - New: DiskUncaught has a double use: it is also the gravity step.
- **L-R3R-02 and L-R3R-09, half pipe.**
  - The board record is at $91 and the rider record at $99. The rider record copies the board each frame until a
    fall is pending (the L-R3R-09 gate).
  - Trick and fall variables are named, and the trick tables are labelled.
  - The reaction sprite is named PipeReact*. Its meaning is tentative.
- **L-R3R-03.** TextKindFirstLine and TextKindCursorX were added.
- **L-R3R-04.** The observation notes were added to DiskFrame_NextThrow and DiskFlightRising.
- **L-R3R-05, BMX.**
  - Renames: BmxTakeoff, BmxAirStep, BmxAirRising, BmxTouchdown, BmxHazardCheck and BmxFinishCourse. The bonus
    handlers were renamed to match the reversed table order. Air-score tables BmxAirScoreLo/Hi were added, with
    the index-21 overrun.
  - New: BmxCallBank4 shows a random comment (bank 4 PickRandomMessage). It is not a sound.
  - New: LoadSurfColumnSet was renamed LoadBmxRiderTiles. Only BMX calls it, and it loads 80 rider pattern bytes.
- **L-R3R-06, skating.** The ball variables are named SkateBall*. SkateDepth and SkateCrashGrace were added. The
  4-byte SkateBallSpawnMasks table is read with indexes 0-15, and the source documents this.
- **L-R3R-07, surfing.** Names added for the shark fin, the aerial (state 5), carving and power, the curl
  position and SurfWaveUp. The state-2 prose was rewritten. The SurfFrame header is attached to SurfFrame.
- **L-R3R-08, foot bag.** Gull* variables and routines were added.
  - New: the foot bag trick-pattern cursors (FootBagTrickPtrs), FootBagTrickCounts and the variety bonus.
  - New: LoadTerrainStrip was renamed LoadGullTiles. It loads the seagull patterns.

## Whole-source sweep (backlog from the round-3 handoff)
- **Bank 6 course builders.** The BMX stream interpreter has Course* names: stream, levels, heights, repeat and
  call pointers, pattern strip. The skating stream pointers have SkateSegList/SkateColStream names, and the
  object-marker and palette slots are named. Course command prose was rewritten with these names.
- **RAM arrays.** New R spans for OAM, StatusLine, HighScoreTable, NumberText, PpuQueue, the PipeScoreText /
  PaletteBuffer / PlayerScores / sound-slot areas, the TerrainTableA/B rows, the course object slots and the glyph
  buffer. Operands inside these spans print as Name+offset, for example OamTile+4 or TerrainTableA+32.
- **Tables.** Table symbols went from 46 to 286. They cover surfing (bank 0 $B701-$B8DF sea/wave tiles), course
  tables, BMX, skating, foot bag, menus, awards, text, graphics loaders and RAM routine images. Each has a short
  role/format note. Ten table bases fall inside code (index-offset aliases); these are emitted as commented
  constants at the end of the source.
- **Other prose fixes.** Event prose that still used raw addresses now uses the names. The overview header in the
  annotations describes the scoped-name and Name+offset conventions.
- **Remaining unreviewed ranges:** none.

## Readability assessment
- **Raw RAM operands: 922 to 62.**
  - 50 of the 62 are deliberate: the $00/$01/$02,X base-pair operands of the 16/24-bit helpers.
  - The other 12 are $0167/$0172 (role unknown, commented), the RAM-routine jump targets $0310/$042A (commented),
    the page clears $0400/$0500, the BIT-operand bytes $01A9/$04A9, and the OamBuffer-1 and PpuQueue-1 index bases
    (commented).
- **Raw ROM operands: 415 to about 120.** These are mostly 64 page-interleaved CHR copies in bank 1, the
  bank-select writes to $8000, and a few small loader/award tables.
- **Names.** Scoped overlay names went from 67 to 299 and routine/branch labels from 579 to 589.
- **Overall.**
  - All six events now read with named state.
  - System code was already good.
  - Branch labels inside routines are still mostly structural (Owner_Skip/Loop).
  - Adequate for porting.

## Remaining editorial issues (explicit in commentary/uncertainties.md "Round 4 additions")
- **Tentative readings:**
  - the rider/board record reading in the half pipe;
  - the PipeReact* sprite;
  - BmxLift and BmxWheelA/B;
  - SurfRideFlag;
  - the DiskUncaught double use.
- **Overlapping variables.** The shared bank 6 object code writes $A9-$C1 slots that overlap BMX variables.
  Presumably the BMX course has no such objects; this was not checked.
- **Unknown extents.** The real extents of the in-code table aliases are not established.
- **Unchanged.** AwardLayout, PresetScores medal reading, the round-3 no-caller code, TK-04.

## Tools / toolkit
- Workspace tool analysis/tools/cmt_apply.py changed:
  - R spans;
  - table+1..3 and bank-0-assumed data operands;
  - in-code table constants.
- cmt_review.py is unchanged.
- No toolkit changes and no new toolkit issues.
- Scratch is in scratch/r04c/ (small helper scripts plus a copy of the round-3 annotations). About 3.0G is free.
