# Round 4 - redirect phase handoff (attempt r04-a1)

Status: **FINISHED** (nothing to resume).

Recommendation: **(a) one more commentary cycle**, to apply two small ready commentary-accuracy leads (L-R4R-01,
L-R4R-02). The disassembly step of that cycle has no proposals. After it, the classification residual is terminal
incomplete (3 B, TK-04; review below), and final editorial audit/acceptance can follow.

## Scope and checkpoint
- **Input** (all re-checked; see analysis/redirect/r04-a1/state.before.sha):
  - maps/round03-final.json = maps/current.json = analysis/state.json (aab2fb6c...).
    - Classified: 131085/131088.
    - Unknown: 3 B.
  - Source commentary/source/game.asm (e12e1cad...), with the commentary handoff gate met (round-04-commentary.md).
  - ROM 8e32ae84....
- **What changed:** only emulator calls and read-only static scripts were run. Map, state, source, ROM and
  toolkit are untouched.
- **Research directory:** analysis/redirect/r04-a1/. It contains:
  - NOTES.md
  - tools/xwatch.py, static_checks.py and rdwatch.py (abandoned, recorded)
  - evidence/
  - leads.before.json
- **Records:** leads.json is appended only (L-R4R-01/02 plus a pass record). Earlier attempts are untouched.
- **Proposed versus accepted bytes:** 0 / 0.

## Residual ledger (recomputed from the map; unchanged)
| Physical | CPU/bank | Bytes | Disposition |
|---|---|---|---|
| 0x400E-0x400F | b0 $BFFE-$BFFF | BD 49 | TK-04: cross-window instruction `lda $C149,x` (L-R1-01) |
| 0x1C010 | b7 $C000 | C1 | final operand byte of the same instruction |

Neighbours:
- Code Observed BmxFrameSprites (ends at $BFFD)
- bank 1 Data at 0x4010
- bank 6 tail Data at 0x1BFF0
- Code Observed $C001 (BmxFrameTail)

## Findings
1. **Bank-6 overlap question: resolved; no BMX conflict** (L-R4R-02).
   - CourseObjects arms bonus markers ($AE-$B1/$B6-$B9/$BE-$C1) only for CourseObjType == 1.
   - CourseObjectPalette claims palette slots ($A9-$AB) only for types 1..$7F.
   - All 642 records of the BMX object stream ($97C2, up to the bank-6 code at $9F4A) have type 0 or $FF.
     The skating stream has 106 records of type 1.
   - Runtime check:
     - In BMX, CourseObjects was entered 12 times (7 slot claims) and CourseObjectPalette 11 times. The
       marker/palette stores executed 0 times.
     - In the skating control run, the marker store executed once.
   - Result: the overlap is a skating-only reuse of the same bytes. A small header fix is also needed: in the
     CourseObjects "Writes" list, CoursePatternVramHi/CoursePatternMode should read CourseMarkerXHi/CourseMarkerX.
2. **The 10 "table addresses inside code" are all index-offset aliases, and their real extents are now
   established** (L-R4R-01; table in NOTES.md section 3, full output in evidence/static-checks.txt).
   - Every real extent lies inside existing Data regions, so there is no classification conflict and no
     structural lead.
   - Index domains:
     - from static gates and loop bounds: FrameBoxAddrs, TextSpecialSlot, PipeFallFramesA, SurfFaceTileIndex
       and SkateLeanFrames;
     - observed X sets (evidence/index-watches.txt): TeamLogo 10..18, rider frames 13..78, foot bag ASCII
       $30-$35, surf 2..4;
     - half pipe rider frames $0D-$5F: from five parallel 83-entry tables with identical $FF sentinel frames,
       packed end-to-start, with AnimTilePageGroup ending exactly at the code at $830B.
   - Ten more labels are also aliases: TeamCursorOam, SpriteListStart, PipeFrameDY/DX, PipeFallFramesB,
     PipeRiderAngleFrame, PipeBackwardsBoard, PipeWallX, FootBagLetterTiles and AnimTilePageGroup.
   - Five current comments are factually wrong:
     - the FootBagLetterTiles "$41+ = $A256" note;
     - "angles past 8 read PipeFallFramesB";
     - "PipeFallFramesB (13 bytes)";
     - TextSpecialSlot "lines $28+";
     - TeamLogo "team number".
3. **$97D5-$97DD is PipeBackwardsBoard**, another alias: base $97D1, board frame 4..12. With it the bank-0 pipe
   tables pack with no gaps from $96DC to $980D, which independently confirms the rider-frame maximum $5F
   (DX ends at $97D4).

## Ready leads (analysis/leads.json)
| Lead | Priority | Content |
|---|---|---|
| L-R4R-01 | medium | Commentary: real alias extents and index domains; fix the 5 wrong comments; mark the 10 additional alias labels. |
| L-R4R-02 | low | Commentary: replace the "not checked" overlap uncertainty with the result above; fix the CourseObjects Writes names. |

Recipes and re-check commands are in leads.json. Neither lead needs replaying unchanged experiments.

## Compact rejection review (TK-04; reused, premises re-checked)
- **Candidate claim:** 0x400E-0x400F + 0x1C010 form one Code instruction (`BD 49 C1`, `lda $C149,x`) executed
  with prg_banks [0,1,14,15]. The opcode is at b0 $BFFE and the last operand byte at fixed b7 $C000.
- **Decisive evidence:**
  - the observed opcode fetch at 0x400E and the successor fetch at $C001 (L-R1-01, Observed region 0x1C011);
  - the toolkit contract (r03-a1 review: romatlas-core RegionKind; romatlas-nes mapping.rs decode_region needs
    each instruction byte at physical offset+i under one view).
- **What it rules out:**
  - every non-code role (Data/Text/Padding/Image contradict the fetch);
  - an alternative alignment, since Code at 0x1C010 alone would contradict the fetch at $C001;
  - any single Code region, since 0x400F and 0x1C010 are physically non-adjacent under every view.
- **Independent constraints considered together:** fetch, successor fetch, bytes and validator contract. They
  are consistent and admit no representable alternative.
- **Strongest competing interpretation:** none credible.
- **Changes this round:** none. The map hash, bytes and toolkit are unchanged, and no new evidence or corrected
  premise exists, so the r03-a1 review stands (analysis/handoffs/round-03-redirect.md).
- **Remaining reason:** the representation is missing, not the inference. These bytes stay Unknown until
  authorized toolkit support for cross-window instructions (TK-04, analysis/toolkit-issues.md). An explicit
  Unknown region carrying the evidence text is the only optional, documentation-only step (0 bytes gained).
- **Why no useful next redirect step remains:**
  - All other bytes are classified.
  - The remaining open items are commentary-semantic (L-R4R-01/02, tentative names).
  - All 15 gameplay milestones are visited, including the natural ending.

## Deferrals (with concrete reasons)
- **Tentative names** (half pipe board/rider records, PipeReact*, BmxLift, BmxWheelA/B, SurfRideFlag,
  DiskUncaught double use):
  - These are naming only. The data flow is already described mechanically, with explicit uncertainty.
  - A bounded watch cannot tell the semantic alternatives apart (bike part vs wheel, spectator vs flag); it would
    only repeat r03 value-range observations.
  - Expected gain is wording only.
- **Runtime confirmation of the static-only domains** (TextSpecialSlot, FrameBoxAddrs, SkateLeanFrames,
  PipeFallFramesA): these consumers did not run in the bounded menu/event watches. The static gates are exact
  (cpx bounds or enumerated producer values), so runtime adds nothing.
- **Optional disassembly refinement:** splitting the Data regions at the real table starts listed in NOTES.md
  section 3 and labelling them. This is a readability improvement, not a classification change, and it can be
  folded into the commentary cycle or the final editorial pass.

## Recommendation
**(a)** One commentary cycle applying L-R4R-01 and L-R4R-02; the disassembly step has nothing to apply. Then
terminal incomplete (3 B, TK-04) using the review above, followed by final editorial audit/acceptance.

## Disk
About 3.0G free. Scratch is scratch/r04r (batch logs and a script, <30 KB); trace jsonl files were deleted. The RPC
bridge (analysis/tools/rpcbridge.py) is left running and holds no state that needs keeping.
