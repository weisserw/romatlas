# Commentary progress

Prepared verified baseline (commentary/before, reading-before.asm) from checkpoint pass-001
(maps/current.json == maps/pass-001.json, sha256 ed42492a...).

## Round 1 method
source/game.asm is produced by commentary/tools/build.py from the frozen before/game.asm
plus annotation modules commentary/tools/ann/*.py (routine/branch/data names, RAM symbols and
bank aliases, routine and block comments, data labels, unknown-range notes, inline-argument
helpers, JMP-stub tables).  Edit the annotation modules and rerun
`python3 commentary/tools/build.py` then `python3 commentary/tools/review.py`, then
`python3 /root/romatlas/scripts/commentary.py verify <workspace>` and `... read <workspace>`.
Do not hand-edit source/game.asm (it is regenerated).  tools/q.py lists instructions with CPU
addresses; `build.py --addr --out X` + tools/compact.py give a compact reading view.
Annotation keys are (PRG bank, CPU address) -> physical offset, so they carry forward by
physical range when a later checkpoint changes the map (re-check moved boundaries).

Source layout changes (byte-identical): one .org section per 8 KiB PRG bank; inline JSR
arguments of CreateTask/SetSpriteDefInline/SetIrqHookInline rendered as `.word Label`; data
split at referenced labels; all absolute/zero-page operands with understood roles symbolic.

## Round 1 result (COMPLETE for current-source readability)
- Scope: all 1568 scaffold entries (whole ROM) reviewed; PRG banks 3,6,7,8,12,13,14,15 code,
  data-only banks 0,1,2,4,5,9,10,11, header, vectors, CHR.
- 1132 global code labels, all descriptive (no sub_/loc_/task_ names left; 52 are
  `Parent_XXXX` internal targets or kernel trampolines whose suffix is the bank entry offset);
  530 carry routine/block comments, including every task, kernel routine and bank entry.
  Local branch labels are named by their first action (@callX, @checkX, @setX, @caseNN,
  @returnCarrySet, ...), otherwise by branch condition (@ifLevelEq03) or iteration (@forY);
  0 generic @skip/@loop remain (20 flag-only conditions such as @ifCarryClear).
- ~150 RAM symbols (+ bank aliases), PPU/APU/MMC3 register names, 8 JMP-stub tables named
  (Vec*, B3_*, B6_*, B7_*, B8_*, B12_*, B13_*).  ~637 raw RAM operand uses remain for state
  bytes whose role is not understood (mostly $07xx event variables).
- All 71 Unknown ranges (19288 bytes) carry a boundary/role note; see uncertainties.md.
- review.json: every scaffold entry status=reviewed with summary/source; 1779 routine units;
  editorial_audit left pending (final acceptance is a separate gate).

## Names/interpretations changed during the pass
- $F5 renamed EventPhase (not just a palette timer); $FA/$FB PlayerFacingX/Y (shared by both
  stage types); CHR shadow mapping corrected (ChrBanks+4/+5 -> R0/R1, +0..+3 -> R2-R5).

## Verification
`commentary.py verify`: ok=true, byte_identical=true (commentary/verification.json);
reading copy refreshed (commentary/reading-after.asm).
