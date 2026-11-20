# Round 2 disassembly handoff

Status: pass complete. All 262,160 bytes are classified. The ROM is still incomplete overall, because the new regions need commentary and the final editorial audit is still to come.

## Checkpoint / verification
- pass-002: maps/pass-002.json, generated/pass-002, analysis/pass-002-verification.json. Map validate ok; byte_identical true.
- Internal store: analysis/state.json. Export: maps/current.json (identical to pass-002).
- ROM classification: 100.00% (262160 / 262160 bytes classified; 0 bytes unknown).

## What changed (from pass-001)
- 295 new regions and 1 edit (region 657, bank 0 $8055-$805D, Data -> Code: the WarmRestart $ED03 target).
- Code grew by 1,262 B, all Static with no known caller:
  - L-001 islands (568 B)
  - HillNmiJump handlers, bank 6 $8DA6-$9028 (643 B)
  - Four L-016 items (42 B)
  - the $8055 stub (9 B)
- Data grew by 13,435 B. It is split on the redirect structure bounds, with the role recorded in each region's evidence. No labels were added; naming is left to commentary.
- Per-lead outcomes: analysis/leads.json (all ready leads verified; L-007/L-008 deferred).
- Plan and tools: analysis/disasm-r02/plan.json and tools/plan.py, tools/apply.py. These need /tmp/rd populated with the redirect r01 tools; run unk.py, rle.py and cov5.py first.

## For commentary
- New regions to describe:
  - bank 1 RLE streams and status bar
  - bank 4 sample and text
  - bank 5 event data
  - bank 6 hill tables and the HillNmiJump code
  - bank 3 path lists
  - bank 0 palettes and scripts
  - fixed bank: LoadScreen_Tbl, SFX pointer table, debug hex print pair ($EC2A/$D537), sound and APU islands
- Low-confidence calls:
  - $B1E2 (pla/tax residue)
  - $A61E (lone RTS)
  - $8395 and $D827 (dead else-arms)

## Unresolved / for redirect
- Natural reachability of the Static islands and of HillSpecial values 1-12.
- Semantics of the generic data: bank 2 music format, bank 3 track tiles, the L-017 tables.
- Bank 0 palette pointer slots that hold $0000 and $0491.
- Questions carried over from round 1: NmiHandler2 $8592 bank; ScriptOp1D `jsr $0014`.
- Gameplay was not exercised this round: two-player mode, later races and the natural ending (L-007/L-008).

## Toolkit
- New: TK-006 (confirmed bug: map.annotate needs id 0).
- Earlier issues still apply: TK-001/TK-002 (confirmed bugs) and TK-003/TK-004/TK-005 (missing features). None is blocking. See analysis/toolkit-issues.md.
