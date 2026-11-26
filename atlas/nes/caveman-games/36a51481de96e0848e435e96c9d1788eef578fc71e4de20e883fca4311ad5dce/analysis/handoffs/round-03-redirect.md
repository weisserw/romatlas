# Round 3 — redirect handoff (attempt r03-a1)

**Recommendation: (a)** Run another disassembly → commentary → redirect round on the ready leads L-019 and L-020.
- They do not change the code/data split or the 100% count.
- They correct structural **role** claims in the map that the code contradicts. The map calls about 2.2 KB "Padding,
  no consumer" or "Text", but code consumes those bytes as binary data.
- They also correct false "no reference" statements in the commentary.

Final editorial acceptance should follow that round.

## Scope and checkpoint
- Input: pass-003 (maps/current.json == maps/pass-003.json; analysis/state.json).
- 100.00% classified: 262160/262160, 0 Unknown. The unknown complement is empty.
- Not modified: map, state, source, ROM (sha256 36a51481...), toolkit. Nothing was published or archived.
- No emulator or serve session was used (static work only). No processes are left running.
- Record: analysis/redirect/r03-a1/notes.md. Scripts and outputs are in the same directory.

## Findings
1. **Code graph re-audit is clean** (audit2.py; complements the r02-a1 audit).
   - 0 absolute reads into Code. 0 non-uniform Padding. All Code view bank contexts are consistent.
   - 4 indexed reads with a base inside Code. All are benign:
     - CL_BracketHandlers is based at $93DB.
     - bank 5 `LDA $8002,X` is a dummy timing read.
     - bank 6 `CMP $AE2B,X` means DV_OutcomeTable-12.
2. **The bank 2 "unreferenced" 612-byte block is consumer-anchored** (L-020).
   - FS_FlameFramePtrs ($937A) holds 19 pointers $8A9D..$8CDD (stride 36). They are read by FS_SetFlameFrame2 and
     drawn by DrawMetasprite.
   - The last frame ends exactly at Code $8D01. The commentary statement "no reference found" is false.
   - FS_FlameFramePtrs is only 38 bytes. $93A0-$941F holds the FS_PoseFramePtrs frames.
3. **Bank 5 $AD28-$BEFF splits exactly into three consumer-anchored parts:**
   - Arena metatile map: 64 B.
   - Metatile set: 24×17 B, ending at $AEFF.
   - CL sprite lists: 32×128 B, $AF00-$BEFF, ending at the reset stub.

   The commentary covers this range with a mispropagated B5_SfxStreamPtrs label and leaves the rest unlabelled.
   **Bank 1 MT frames** run $A911-$BD58 and end at Code. The two map "Text" regions inside them are OAM records.
4. **Metatile maps and sets** (mtile.txt). There are 17 uses of UploadMetatileScreen. The set extents are computed as
   max map index × 17-byte records. Independent ends agree (bank 1 $A8AC, bank 5 $AEFF), and no Code lies inside.
   - All 23 sixteen-byte "Padding" runs after bank 3 set $9F23 sit on record+1 boundaries.
   - All 48 such runs after bank 4 set $A666 do too.

   They are uniform metatiles, not fill.
5. **Padding read directly by code**:
   - bank 1 MT_CavemanYOfs.
   - bank 3 DR_ActionRiderY/X, inside the 83-entry tables proved in L-013.
   - bank 4 SR_RunnerDX/DY and SR_ChaserDX.
6. **Text that is numeric data**:
   - bank 3 $84CF-$8567 (128 B).
   - bank 5 CL_PoseY, bank 2 FS_AmbientY, bank 6 DV_CavemanDX rows.
   - bank 6 metatile-set runs ('FFFF').
   - bank 1 sound rows.
   - bank 7 $D257, the 6th StandingsNameOffsets entry. This is the same error type as L-014.

## Ready leads (analysis/leads.json)
| ID | Scope | Action | Effect |
|---|---|---|---|
| L-019 (medium) | 125 regions listed in r03-a1/l019_candidates.json (file offset, CPU address, bank and consumer reason per item) | Change kind Padding/Text → Data (Static, conf 0.9) with consumer evidence. Split 3 bank 0 partial regions ($A581, $A9AC, $AAC7) at the set starts. Optional or borderline: bank 0 $A3E5/$A3FA (letter tiles in metatile records) and bank 7 $D899 (unanchored; leave as is). Then validate/save/export and run checkpoint pass-004 (byte-identical expected). | Role correctness for 1700 Padding + 505 Text bytes. Count unchanged. |
| L-020 (low) | bank 2 $8A9D-$8D00 and $937A-$942B; bank 5 $AD28-$BEFF; bank 1 $A911-$BD58; mtile.txt maps/sets | Disassembly: merge the fragmented generic Data in these extents into role-labelled regions with consumer evidence. Commentary: fix the false "no reference" text, the frame-table extents, the propagated B*_SfxStreamPtrs blocks that swallow following data (banks 1-6), the DR_MsprFrames_43 / B4_SfxStreamPtrs / DV_ActorDefaults blocks that actually span metatile sets, and the two numeric operand notes. | Specific roles for about 10 KB. Corrects commentary errors. |

The two leads can be done together. In both, only kinds, labels and evidence change.

## Deferred avenues (with reasons)
- **L-004, 7 roots:** reachability only.
  - Their structure was re-checked as coherent.
  - The anchor scans (immediate pairs, absolute operands, labelled-table words) found no new reference.
  - No selector is known, so a natural run would be an unchanged replay.
- **L-005 remainder** (bank 3 $AEF9-$B01D, bank 7 $CEDB-$CEE7, IrqSplitParam odd bytes, bank 2 $9420-$944B):
  anchors.py found no consumer. They stay generic Data; there is no code-vs-data question.
- **S16, natural Games run:** unchanged reason. Placings, results and Caves of Fame are already visited, and nothing
  depends on the run.
- **CL_Bracket4/5:** same selector as the demonstrated N=3/6 cases.
- **Runtime confirmation of the frame reads:** optional. The static pointer chain plus exact extents already supports
  the Data role. A recipe is in L-020.

## Rejection review (compact)
- **"100% and audits clean ⇒ ready for final acceptance": rejected.**
  - Decisive artifacts: stride17.txt (71/71 aligned "Padding" runs), padanchor.txt (direct ADC/LDA reads into
    Padding) and FS_FlameFramePtrs words (ptrscan2.txt).
  - These rule out the "fill / no consumer / text" role claims for the listed regions only. They say nothing about
    code vs data. The audits found no Code inside any extent.
- **Prior "no consumer" dispositions** (phase-1 generic evidence, the bank 2 $8A9D commentary, L-005 deferrals).
  - Their scope: they rested on a missing reference found by local scans, which never decoded the pointer tables.
  - This is corrected by combining independent constraints: consumer routine semantics, pointer words, record
    stride, and an end boundary at Code or the stub.
- **Strongest competing interpretation:** coincidental values.
  - Exact stride alignment (71/71) and extents ending at independent boundaries make coincidence implausible.
  - Where only one constraint exists (bank 7 $D899, bank 2 $9420), no claim is made.
- **Retained limits:** natural reachability of L-004 roots; meanings of the remaining L-005 blocks.

## Artifacts
analysis/redirect/r03-a1/{notes.md, audit2.py/.txt, ptrscan.py/.txt, ptrscan2.py/.txt, anchors.py/.txt, mtile.py/.txt,
rolefix.py/.json, stride17.txt, padanchor.py/.txt, textaudit.py/.txt, framespan.py/.txt, labelled_padding.txt,
l019_candidates.py/.json, leads.before.json, scenarios.before.json}.

Updated: analysis/leads.json (L-019 and L-020 added; L-004 and L-005 attempts appended; pass record),
analysis/scenarios.json (S16 history note) and analysis/progress.md.

Toolkit: no new issues found.
