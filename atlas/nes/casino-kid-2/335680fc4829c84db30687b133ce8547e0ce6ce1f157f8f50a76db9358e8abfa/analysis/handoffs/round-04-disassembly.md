# Round 4 / disassembly handoff

Scope: one disassembly pass on pass-003, covering ready redirect leads R3-L027 (independent review), R3-L028 and R3-L029.
- The baseline was not re-created. input/game.nes is unchanged and there were no toolkit edits.
- No gameplay: the natural ending was already reached in S13, and no question needed new runtime evidence.
- Static checks only. I used the binary named in workspace.json through a temporary socket bridge. The scratch
  dir /root/rascratch-r4d has been removed and my server stopped.

Checkpoint: **pass-004**
- maps/pass-004.json (sha256 eb7719e32423547dfa961a0c1b0fb166feccea651ad018e4f6186f0075969519)
- generated/pass-004
- analysis/pass-004-verification.json

Verification:
- map.validate ok.
- analysis/state.json == maps/current.json == maps/pass-004.json (same sha256).
- `run.py checkpoint pass-004`: ok=true, byte_identical=true.
- No overlapping regions.

ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).

Per-bank counts are in analysis/progress.md under "Round 4 / disassembly". Bank 4 is now 10,102 code + 6,282
data/padding with 0 unknown; the other banks are unchanged. Regions went from 2,976 to 2,977 (1,604 code,
1,247 data, 126 padding).

## Findings (details in analysis/investigation.md, "Round 4")
- **R3-L027 accepted** (decision with reasoning in investigation.md). Bank 4 $BB30-$BB31 (physical 80704, 2 B)
  is now Code/Static at confidence 0.5, with view $BB30 and prg_banks [8,9,14,15]. It is an unreferenced
  leftover `lda zpLocal3` between rts $BB2F and entry $BB32.
  - Independent check: no 2F/30/31 BB byte pair occurs ROM-wide.
  - The dispatcher at $BA81 and the $BB32 body both use zpLocal3.
  - The data competitor has no support.
  - The claim is structural only: reachability is unresolved and the bytes are not claimed dead.
- **R3-L028**: $9DB2-$9DB5 is one 4-byte suit table (merged from 2190+2783). The bounds come from the base
  operand and the observed entry $9DB6.
- **R3-L029**: $8798-$879B is the "YOU" string. $879C-$87CB holds 8 x 6-byte panel-label records read by the
  mask-driven printer at $8734. The $A8B5-$A8D5 strings are exactly the record targets (region 2890 evidence
  updated).

## Leads
Consumed and verified: R3-L027, R3-L028, R3-L029. No new leads produced.

## Regions changed (for commentary carry-forward)
- New Code: b4 $BB30-$BB31. Render it as an instruction with the note "unreferenced leftover; reachability
  unresolved". It may take a local label or none, and it stays outside B4_TravelPlaneFrame.
- Data b4: removed 2190 ($9DB2-$9DB3) and 2783 ($9DB4-$9DB5), replaced by one region $9DB2-$9DB5.
  - Suggested names: B4_SuitOrderTable; rename B4_HandAnalysis_9DAD to something like B4_SuitOrderLookup.
  - Note that the result is discarded.
- Data b5: removed 2245 ($8798-$87CB), replaced by $8798-$879B ("YOU") and $879C-$87CB (panel-label records).
  - Suggested names: B5_PanelLabelYou, B5_PanelLabelRecords.
  - Rename B5_PrintMessageValue to B5_PrintPanelLabels (mask bit n selects record n; mask $80 = YES/NO).
- Data b5 region 2890 ($A8B5-$A8D5): only the evidence and confidence changed. The B5_PanelLabelStrings reader
  is now known.
- No other boundaries changed. Log: analysis/tools/round04/applied.jsonl.

## Unresolved / deferred
- $BB30 reachability is unknown, and the 0.5 confidence reflects that. An independent final review may still
  reject it.
- Possible original bug in B4_CompareHighCards ($9D5B stores the card to $80, but DivideMulti still divides
  $8A/$89). The result is discarded. This is commentary-only, and the Api_DivideMulti divide-by-zero behaviour
  is unchecked.
- Unchanged from round 3: no consumers found for B4_LuckyStrings $AD9C-$ADC2 or b4 $A2DE-$A46B; the meaning of
  $94F3 items, $A0ED tier bytes and Bank5_Data_8268 is still open; the generic R2-D02 windows remain.

## Next
Commentary round 4 (rebuild from pass-004), then the final editorial audit/acceptance. Toolkit: no new toolkit
issues found (analysis/toolkit-issues.md; TK-001..TK-006 unchanged).
