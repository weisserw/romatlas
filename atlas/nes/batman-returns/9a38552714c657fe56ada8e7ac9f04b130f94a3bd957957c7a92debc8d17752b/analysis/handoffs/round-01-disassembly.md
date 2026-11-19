# Round 1 disassembly handoff

- Scope: first pass; baseline established; static + observed code classification; whole-bank and
  gap data classification; stage tour via RAM $003E.
- Checkpoint: pass-001 (maps/pass-001.json, generated/pass-001, analysis/pass-001-verification.json):
  ok, byte-identical. Internal store analysis/state.json; export maps/current.json (validated).
- Result: ROM classification: 86.76% (341171 / 393232 bytes classified; 52061 bytes unknown). INCOMPLETE.

## Findings
- Batman Returns (Konami 1993). MMC3 PRG mode 0; switchable windows loaded as even/odd pairs.
- Vectors: NMI $FC75, RESET $FBCC, IRQ $CF29. JumpEngine $C294 (bank 14) and SoundJumpEngine $8BA9
  (bank 0) take inline word tables after the JSR (38 tables, 251 entries parsed).
- SetPrgPairFromStack $D156: pops caller-pushed bank Y, maps Y/Y+1 at $8000/$A000, returns normally.
- Banks 1,2,3,8 and most of 9: data-only. Banks 12/13: data with code islands.
- Stage index RAM $003E (0x00-0x15 valid); writing it in snapshot pre_stage (traces/round1/pre_stage.snap,
  same-build only) loads any stage.

## Leads
- Consumed: none. Produced: L001 (object dispatch table $DE19, biggest expected gain), L002 (local
  pointer tables for indirect JMPs), L003 (banks 12/13 code islands), L004 (bank 0 sound data),
  L005 (bank 15 unreached gaps), L006 (bosses/ending RAM shortcut).

## Unresolved questions
- Per-entry bank context for $DE19 table entries (stage -> bank pair mapping).
- Extents of indirect-JMP pointer tables (L002) and inline jump tables (inferred; see evidence text).
- Boss/stage-clear/ending RAM contract.

## Deferred
- Password screen / TCRF level-select password (unconfirmed sources).
- Data-role semantics for banks 1-3, 8, 9 (generic data only).

## Artifacts
- analysis/progress.md, analysis/investigation.md, analysis/leads.json, analysis/scenarios.json,
  analysis/gameplay.md, analysis/toolkit-issues.md
- traces/round1/observed-contexts.json, observed-edges.json, static-tables.json, pre_stage.snap
- analysis/tools-round1/ (analyzer scripts; paths inside refer to the pruned scratch dir /root/rascratch)
