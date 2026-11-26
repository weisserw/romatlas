# Redirect attempt r6-a1 (round 6)

- **Input:** checkpoint pass-006 (512 regions, 100.00% classified, map sha256 865f4d5a...) and round-6 commentary
  (byte-identical).
- **Not modified:** map, state, commentary source, ROM, toolkit (TK-001..TK-005 kept), configured binary.
- **Runtime:** none. No server started. Scratch /tmp/r6scratch removed.
- **Reused by reference:** r3-a1 tools/dec.py (linear decode of all Code regions) and evidence/closure.json.

## Target claim reopened: $FC23 (region 348, 15 B)

- **Prior record (r5-a1 rejection review, round-6 disassembly note):** the clip is "data-dependent". The review said
  no constant or table bounds ObjY + dy, and that there was no discriminating test.
- **Corrected premises:**
  1. The path is not only a top-edge clip. PlaceSprite reaches $FC23 whenever the 16-bit sum ObjY + dy' has a nonzero
     high byte. That covers a sum below 0 and a sum above 255.
  2. Object Y is not unbounded. Every writer of ObjYLo, ObjYHi or ObjFlags in every Code region was enumerated.
     Every one stores a template constant, a bounded loop result or bounded arithmetic.
- **Discriminating check:** a static computation of the per-object range of Y + dy' against the bounds 0..255. A
  single object that leaves these bounds would refute dormancy.
- **Stop condition:** a full census with no counterexample.

| Tool | Output | Finding |
|---|---|---|
| labels.py | (library) | Maps data labels to physical offsets from commentary/source/game.asm |
| dyrange.py | evidence/frame_dy_ranges.json | Covers 26 frame-pointer tables and their records. No $FD control byte at any record start. dy ranges are computed per table |
| objwriters.py | evidence/objy_writers.json | Census of all writes to the Y/flag fields of the 4 ObjTable slots. Results below the table |
| fc23_bounds.py | evidence/fc23_bounds.json | 17 object contexts with derived Y ranges and source lines. Smallest Y + dy' = 9 (deck pins lifted by the pinsetter). Largest = 232 (bowler at Y $F0 with dy -8). **No object leaves 0..255** |
| ledger.py | evidence/static_ledger.json | Exact accounting of the 1937 Static Code bytes (46 regions) by disposition |

Census findings (objwriters.py):
- **Y high byte:** ObjYHi is written only by the 10-byte ROM template copy loops (`cpx #$0a`) (all Y high bytes $00) and by the
  Reset RAM clear.
- **Other indexed stores:** their indexes stay inside their own arrays.
- **Indirect stores:** they target the APU, TileBuffer ($041B+) or the dormant VRAM-read job.
- **Frame-table pointers:** they come only from ROM templates, BowlerFrameTables ($8CDD) and PerfectFramePtrs ($BFD2).
  No sprite list is ever built in RAM, which also closes the scope gap the r5 $FD disposition had left open.

**Outcome.** $FC23 is **dormant by data flow**, the same standard accepted for the fades and the gated jobs.
- Structure and provenance are unchanged: Code, Static.
- The commentary sentence "Nothing in the data rules it out" is now wrong. Correct it in the final editorial audit
  (L-023).

## Static ledger (1937 B)

| Group | Bytes | Disposition / decisive artifact |
|---|---|---|
| L-001/L-002 runs outside the vector closure | 1461 | Unreachable: r3-a1 closure.json and computed-transfer census |
| Gated NMI/library jobs ($F2E6, $F319, $F330, $F4D8, $F520, $F588, $FDD4, $FE4C) | 264 | Dormant by data flow: r3-a1 gate_writers.json, r4-a1 indexed_stores.json |
| FillNametable2/3 ($F656) | 6 | Unreachable: r4-a1 residual_preds.json |
| Fade variants (regions 333, 335, 337, 405, 407) | 160 | Dormant by FadeDelay data flow: r5-a1 fadedelay_flow.json, fade_closures.json |
| Sound ($E267, $E2BE, $E3D4) | 8 | Dormant by ROM sound data: r5-a1 residual_static.json, clamp_sim*.json |
| Page carries ($AAF7, $AB07, $BDEB) | 6 | Impossible by arithmetic: r5-a1 residual_static.json |
| DrawObject $FD jump ($FB8F) | 14 | Dormant: no ROM list encodes it (r5-a1), and no RAM list exists (r6-a1 objwriters/frame tables) |
| DrawObject Y-range clip ($FC23) | 15 | Dormant by data flow: r6-a1 fc23_bounds.json |
| $A84E | 3 | Impossible by frame logic (r3-a1) |
