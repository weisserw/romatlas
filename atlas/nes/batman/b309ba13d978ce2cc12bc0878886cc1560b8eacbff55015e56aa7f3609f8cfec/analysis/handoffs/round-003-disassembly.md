# Round 3 — disassembly handoff

- Scope: apply ready lead L-019 (provenance part). No other ready lead; L-018 is exhausted-unknown and has no new evidence. No gameplay was run: S1-S7 are all visited, so more play adds coverage only.
- Input: pass-002. Output checkpoint: **pass-003**. Files: maps/pass-003.json and generated/pass-003.
  [analysis/pass-003-verification.json](../pass-003-verification.json): ok=true, byte_identical=true, map_sha256 2d5d5cb5…48edd.
- analysis/state.json is identical to maps/current.json. It has 2,191 regions, and map.validate returns ok.
- Tools: analysis/tools/r3_build.py checks the fetches and builds scratch/r3/edits.json. analysis/tools/r3_apply.py applies the edits, then validates, saves and exports.
  The pre-edit state is backed up at scratch/r3/state-pass002.json.

## Completion
ROM classification: 100.00% (262158 / 262160 bytes classified; 2 bytes unknown).
The unrounded value is 99.9992%. **Status: incomplete.** File 0x1C5AE-0x1C5AF (bank 14 $C59E, L-018) is still unknown.
Per-bank counts are unchanged from round 2 (see round-002-disassembly.md table). The only unknown bytes are in PRG 14.
Code is 41,123 B: 28,544 B Observed and 12,579 B Static. That is 308 B moved from Static to Observed-artificial.

## L-019 (provenance part) — verified, applied
- The r2-a1 contexts were recomputed independently against pass-002.
  - 142 new opcode starts, the same set as observed-merge.json.
  - Each start is on a Static instruction boundary, with matching CPU address and bank slot.
  - No fetch lands in a non-code or unknown byte.
- 17 Static regions were split into 19 Observed (artificial) runs totalling 308 B and 7 Static runs totalling 128 B.
  - The Observed runs are tagged r2-a1:clr6/clr6b/clr16/clr16b/clr17.
  - The Static runs keep their original evidence, confidence and views. Label and comment go to the first run.
- Split regions, by file offset:
  - 1157F, 11E14
  - Bank 13: 1A0DB, 1A197, 1A1CB, 1A32C, 1A350, 1A49F, 1A4C9, 1A4D4, 1A4F3, 1A57B, 1A587, 1A592, 1BD53
  - 1ED3A, 1EEAA
  - Run detail is in scratch/r3/edits.json "log".
- This is artificial execution, not natural reachability.

## For commentary (still open under L-019)
- In bank08.asm, the AreaExitCheck header gives the countdown as `$050F`. The operand is **$051F** (ObjXLo+15), so the header needs correcting.
- ObjKind42Defeated stores 5 into $051F. This value is the AreaExitCheck delayed-exit countdown (flag bit5), so the related uncertainties.md item can be resolved.
- Evidence: analysis/redirect/r2-a1/clear-results.txt and analysis/handoffs/round-002-redirect.md.
- The new region splits change only provenance and evidence. The source should be refreshed from pass-003, keeping names and comments by physical range.

## Leads
- L-019: changed to partially-verified (the commentary part is open).
- L-018: unchanged (exhausted-unknown).
- See [analysis/leads.json](../leads.json).

## Toolkit
- No toolkit issues found this pass.
- TK-001..TK-003 are unchanged in [analysis/toolkit-issues.md](../toolkit-issues.md).
