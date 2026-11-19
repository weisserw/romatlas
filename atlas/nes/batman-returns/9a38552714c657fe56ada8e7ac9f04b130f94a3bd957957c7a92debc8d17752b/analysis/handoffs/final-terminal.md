# Terminal handoff — justified incomplete (2026-11-19)

Workflow state: terminal incomplete after Round 3 redirect rejection review.
This is workflow termination, NOT whole-ROM completion.

- ROM: Batman Returns (Konami 1993), NES MMC3, sha256 9a3855…752b (input/game.nes unchanged).
- Final checkpoint: pass-003 (maps/pass-003.json == maps/current.json;
  analysis/pass-003-verification.json ok, byte-identical). Map and state validate.
- ROM classification: 100.00% (393229 / 393232 bytes classified; 3 bytes unknown).
- Residual unknown bytes (one lone unreferenced RTS each): phys 0x14CCA (bank 10 $8CBA),
  0x17BC6 (bank 11 $BBB6), 0x1FD5A (bank 15 $FD4A). Code-vs-data materially ambiguous.
- Rejection review: analysis/handoffs/round-03-redirect.md ("Rejection review"); decisive
  artifacts analysis/redirect/r03-a/evidence/, analysis/redirect/r02-a/evidence/l017-*.json,
  analysis/tools-round3/l017-span.json. Coordinator inspected it: the rejection is a
  sufficiency judgement consistent with accepted per-instance standards (every accepted
  unreferenced short region carries a per-instance constraint; these bytes carry none);
  seven distinct methods (fall-through, record base, checksum readers, stack/indirect
  dispatch, twin search, two runtime read-watches) produced no discriminating fact.
  Reopen only with a reference, fetch, genuine data read, enclosing branch or relocated copy.
- Other open leads: L018 blocked (442 B no-caller routines, Static conf 0.7, no reachability
  claim); L006 ending reached only with assisted fight (boss HP/player health patched);
  password screen arrival unconfirmed. See analysis/leads.json, analysis/scenarios.json.
- Commentary: round-3 readability gate complete on pass-003; commentary.py verify
  byte-identical (commentary/verification.json, game.asm sha256 167d92ff…);
  reading copy commentary/reading-after.asm; review.json 1909/1909 regions, 2117/2117 units.
  Final strict editorial audit still pending (reserved for final acceptance/archive).
  Known editorial residue: mechanically named dispatch entries/states, unidentified enemy
  characters, ~110–322 mechanically named data tables with consumer notes
  (commentary/uncertainties.md).
- Prior commentary snapshots: analysis/round-1/, analysis/round-2/.
- Toolkit issues: analysis/toolkit-issues.md (TK-001..TK-005, none blocking).
- Archive selection: archive-options.json (no archive executed).
