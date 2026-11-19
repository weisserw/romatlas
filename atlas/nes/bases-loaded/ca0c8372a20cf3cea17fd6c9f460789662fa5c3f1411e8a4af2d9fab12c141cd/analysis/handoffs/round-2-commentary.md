# Round 2 — commentary phase handoff (2026-11-19)

Gate: current-source readability complete for ALL classified source; fresh byte-identical
verification passed; reading copy refreshed. Final editorial audit not run (archive only).
ROM classification unchanged: 100.00% (327695 / 327696; 1 unknown byte, L-028).

## Scope and method
- Later-round rule applied: maps/current.json and analysis/state.json differed from the
  round-1 baseline (pass-001 -> pass-002). The whole round-1 commentary directory was
  copied to analysis/round-1/commentary/ (diff-verified identical) before a fresh baseline
  was prepared with `commentary.py prepare` from pass-002 (1,497 scaffold regions).
- Round-1 notes/tools were carried by CPU address within each 32 KiB bank (= physical
  range); the first rebuild over pass-002 was byte-identical with no note displaced by the
  new boundaries. build.py now flags dropped notes, which exposed ~20 round-1 notes that
  had never rendered (bank 0 reset/mode-restart area, off by 1-2 bytes) - fixed.
- Covered: every 32 KiB bank 0-7, header, vectors, CHR. All new round-2 code/data (L-009,
  L-015..L-024) and the older backlog: ~540 round-1 "Mechanical summary" routines (banks 0,
  1, 3 mostly) replaced by hand-written comments; all automatic Sub_/L/Data_ names replaced
  (only Data_9490 kept deliberately, explained); stale "unclassified bytes" remarks fixed.
- Every L-024 code region carries a generated note: no caller/jump/pointer known,
  reachability unresolved. Debug routines behind lone RTS hooks are named Debug* / NullHook*.
- review.json: 1,497 entries reviewed (1,886 units); 1,278 carried by identical physical
  range from round 1 (re-verified by the build), 219 reviewed new in round 2;
  editorial_audit pending.

## Readability assessment
Good for porting: routine-level comments everywhere, RAM symbols for all recurring
operands, record layouts, data formats at their tables. Weaker spots: branch labels are
still condition-derived (`@ifHalfInningBelow`) where no game meaning applies; bank 3 scene
figure animations are explained mechanically (which figure is drawn is often unknown);
bank 1 computer-opponent decision tables are not decoded.

## Examples (source/)
- prg32_0.asm AtBatChangeSides ($C000): change of sides and the full game-end test.
- prg32_0.asm CoverFirstBase..BackUpHomePlate: cover points named from runner-path
  coordinates; AtBatHomeRun/AtBatOutOfPlayAward/AtBatHitBatterScene state handlers.
- prg32_3.asm DecodePassword*/EncodePassword*: password field layout (team, opponent, 3+3
  digits, checksum letter).
- prg32_3.asm DebugShortcutNinth / DebugEditSeason: developer shortcuts with no entry.
- ram.inc: HomeScore/VisitorScore (L-025 swap), HitsBySide/ErrorsAgainst, RecentStarters.

## Verification
`python3 /root/romatlas/scripts/commentary.py verify <workspace>`: ok, byte_identical true
(commentary/verification.json, ROM sha256 ca0c8372...). `read` refreshed
commentary/reading-after.asm.

## Unresolved meanings (details in commentary/uncertainties.md)
Scene meanings 3/4/19 (mechanical only); scene 11 probably a fight after a hit batter;
scene 21 probably intentional walk; why AddError is called on a fourth bounce; Port7000;
ScreenImageA/B identity; voice sample meanings; CPU decision tables; RunsTally purpose.

## Structural questions / leads for redirect
- L-029 (new, low): bank 3 $A39C ($40) decoded as RTI at the start of L-024 code; probably
  the 5th byte of AnimationStepTimes ($A398). Boundary only, byte-identical either way.
- L-030 (new, low, semantics): bank 1 ScaleLaunchByHomeLead ($85BE) FarCalls bank-0
  MulU8 ($90B2) with MulU8Z operands; bounded runtime check suggested in leads.json.
- L-028 (open): unknown byte $9CAC explained in source as a possible disabled hook.
- L-026 (open, unchanged): ending stall after SaveGameState.
- Closed: L-025 (label swap done), L-027 (new code commented). L-012 updated (password
  layout decoded statically).

## Deferred work
Final editorial audit (`commentary.py audit`) and any game-meaning renames of branch
labels; optional runtime confirmation of scene meanings.
