# Round 3 commentary handoff - readability gate COMPLETE (editorial audit pending by design)

Baseline: checkpoint pass-003 (maps/current.json == commentary/baseline.json). Classification unchanged by
this phase: 100.00% (393229 / 393232 B; 3 B unknown: 0x14CCA, 0x17BC6, 0x1FD5A). Readability and
classification are reported separately.

## Baseline change handling
- pass-002 -> pass-003 changed only 6 bytes (Unknown -> Code: 0x08743, 0x0B1BE, 0x0B7A2, 0x1C1E2-4); the
  1909-entry review scaffold is otherwise identical.
- Round-2 commentary preserved first: analysis/round-2/commentary/ and analysis/round-2/tools-commentary/
  (diff -r identical; README.txt). analysis/round-1/ untouched.
- Fresh `commentary.py prepare` (byte_identical) -> new before/, baseline.json, phase1-state.json, review.json.
- Carried by physical identity (ann keys = bank:CPU): ann/*.txt, uncertainties.md, progress.md. jtables.py
  re-run (identical 193 tables). Re-rendered source differed from round 2 only at the 4 changed ranges
  (checked by diff), so all other names/comments/review credit were reused.
- Invalidated and re-reviewed: the 4 new Code records (region-1903..1906) and their host routines; the
  3 still-unknown byte notes were rewritten (terminal residual, round-3 reject reason).

## Changes
- Dead tails: source comments at bank 4 $8733, bank 5 $B1AE/$B792, bank 14 $C1D2 - unreferenced,
  unreachable instruction tails (Static, no label, reachability not claimed, probably leftover jsr/rts ->
  jmp conversions; $C1D2 = JMP Reset). review.json summaries say the same (review.py DEAD table).
- L019: ObjT33_Handler `jmp ObjT32_Handler_St00_St01_Skip` / `jsr ObjT03_Spawn` (pair 6/7, previously raw
  $BE86/$8754); ObjT32/ObjT33 noted as stage $11 (5-1) wave-script spawns (records $970F/$9718/$973C);
  StageEventIndex 3 consumption at StageEvt_Stage3_1A, StageLoadGraphics, StageEventDispatch,
  StageEventByIndex; boss-loop census at ObjHandlerTable and UpdateAllObjects; wave-record note with the
  stage $11 example. Fixed stale name StageEvent_FinalBossCleared -> StageEvt_FinalBossCleared.
- Files: commentary/ann/r2-unknown.txt, r2-15.txt, r2-07.txt, r2-05.txt, p2-04.txt, p2-12.txt, 14a.txt,
  14b.txt, 91-data.txt; analysis/tools-commentary/review.py; commentary/uncertainties.md, progress.md.

## Verification
- commentary.py verify: ok=true, byte_identical=true (source game.asm sha256 167d92ff...).
- Reading copy refreshed: commentary/reading-after.asm. input/game.nes sha256 9a385527... unchanged.
- review.json: 1909/1909 reviewed with source lines, 2117 units; editorial_audit pending (final acceptance).
- Standard checks: 0 Sub_Bnn routine names, 0 numeric zero-page operands; raw switchable-bank operands
  down to 2 documented ($8000 bank-ID read, $8159 logo checksum).

## Remaining gaps (not part of this gate)
- 3 unknown bytes (terminal residual, L017); per-state game meaning of structurally named dispatch
  entries; enemy identities; wave-script terminators; 322 Data_Bnn mechanical equates; systematic local
  branch labels; final editorial audit. No runtime run in this phase (L019 facts from r02-a evidence).
