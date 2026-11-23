# Round 5 - commentary phase handoff

Status: **FINISHED**. The current-source readability pass covers all classified source, and the leads are applied.
The final editorial audit and archive were not run; they are reserved for final acceptance.

## Scope and identity
- **Input:** maps/current.json = analysis/state.json = commentary/phase1-state.json = maps/round03-final.json
  (aab2fb6c6b8120365d94d57a881bad3c0da726c7f5ccf5126a5504d8367b297b).
  - The map and state are unchanged since the frozen baseline, so the existing commentary was resumed.
  - There was no re-prepare. before/ and baseline.json are untouched.
- **Source before:** commentary/source/game.asm e12e1cad... (annotations 9d6bd663...). Round-4 copies are kept in
  scratch/r05c/: annotations.r04.txt, cmt_apply.r04.py, cmt_review.r04.py, review.r04.json and leads.before.json.
- **Source after:** commentary/source/game.asm
  **a2b9634c2c312f97c682ef97b3da510c3258382741ac5ace0ee97361f9c8afbd**.
  - It is built from commentary/annotations.txt (b74b89f5...) by analysis/tools/cmt_apply.py.
  - review.json is 6c078302...: 2293/2293 entries reviewed, 951 code units and 1237 units in total.
    editorial_audit is pending.
- **Classification (unchanged):** 131085/131088 bytes. The 3 Unknown bytes are TK-04 and are explained in the
  source at both sites.

## Leads
All leads were applied, and none was rejected. leads.json now has status "consumed" plus a round-5 attempt entry.
- **L-R4R-01: alias extents.**
  - The consumers were re-checked in the source:
    - TextSpecialSlot: cpx #$3E/#$4E gate.
    - FootBagScrollMessage: cmp #$41 split between letters and digits.
    - TeamCursorOam: ldx #4 .. dex/bne.
    - PipeBodyStoreAngle: angle X 4..12 into both PipeFallFrames tables.
    - DrawTeamLogo / DrawSpriteList.
  - The extents match analysis/redirect/r04-a1/evidence/static-checks.txt.
  - All 20 aliases are now defined at the end of the source as `Name = Name_XX - $XX`. The label `Name_XX` sits
    at the first real byte, with a pointer comment, so the misleading labels at the bases inside data/code are
    gone. Assembly is byte-identical.
  - The 5 wrong comments were rewritten:
    - FootBagLetterTiles/DigitTiles: the "$41+ = $A256" note.
    - PipeFallFramesA: "angles past 8 read B".
    - PipeFallFramesB: "13 bytes" (it is 9).
    - TextSpecialSlot: "lines $28+" is now $3E-$4D.
    - TeamLogo: "team number" is now graphics block $0A-$12.
  - New notes: the packing of the half pipe parallel tables and the PipeBackwardsBoard mirror mapping.
- **L-R4R-02: course object overlap.**
  - The CourseObjects Writes list now shows CourseMarkerXHi/CourseMarkerX. The cause was that the CoursePattern*
    scope spans ended inclusively at $8980; they now end at $897F.
  - The CourseObjects and CourseObjectPalette headers now state that BMX never writes these bytes.
  - The uncertainty in commentary/uncertainties.md is marked resolved.

## Other work
- **Branch labels:** 63 structural branch labels now have descriptive names, in HalfPipeFrame, BmxFrame,
  SurfFrame/SurfAdvanceWave, FootBagFrame, PipeBodyStoreAngle, CourseObjects and CourseObjectPalette.
  - Example: CourseObjects_FindFreeSlot / _ClaimSlot / _ArmMarker / _NoNewObject.
  - No prose referenced the old generated names.
- **Overview header:** updated for the alias convention and the branch-label convention.
- **Tool changes (workspace only, no toolkit change):**
  - cmt_apply.py: the `D bank base Name @real` alias syntax and the "rounds 1-5" banner.
  - cmt_review.py: maps the alias real labels to their annotation blocks.

## Remaining unreviewed ranges
None. Every review.json entry is reviewed.

## Verification and reading copy
- `commentary.py verify`: ok true, byte_identical true (analysis/round05-commentary-verification.json).
- `commentary.py read`: commentary/reading-after.asm refreshed (0b066b71...) after the final build.

## Readability assessment
- **Events and system code:** all six events and the system code read with named state and tables.
- **Index-offset tables:** these were the main misleading spot. They now show their true extents and are
  expressed as real-label minus offset.
- **Raw operand counts:** unchanged from round 4, about 62 RAM (50 deliberate base-pair operands) and about 120
  ROM (mostly bank 1 CHR page copies and bank-select writes).
- **Branch labels:** 843 generated Owner_Skip/Loop/Return labels remain, against 908 before. They are consistent
  and correct but mechanical. This is the main residual editorial weakness.
- **Overall:** adequate for porting and archival reading.

## Remaining editorial issues (for the final audit; see commentary/uncertainties.md)
- **Generated branch labels:** most branch labels remain generated; the largest counts are SurfBuildWave (33) and
  NameEntryFrame (20).
- **Tentative names (naming only):**
  - the half pipe board/rider record reading;
  - PipeReact*;
  - BmxLift/BmxWheelA/B;
  - SurfRideFlag;
  - the DiskUncaught double use;
  - AwardLayout and the PresetScores medal reading.
- **Other open items:**
  - The round-3 no-caller code.
  - Raw RAM $0167/$0172.
  - The 3 bytes held by TK-04 stay Unknown: a terminal incomplete for whole-ROM classification, pending
    authorized toolkit support.
- **Line length:** about 1000 phase-1 region header lines are longer than 110 characters. They are generated,
  not prose.

## Disk
About 3.0G free. Scratch for this round is scratch/r05c/ (about 2.5 MB, mostly review.r04.json).
