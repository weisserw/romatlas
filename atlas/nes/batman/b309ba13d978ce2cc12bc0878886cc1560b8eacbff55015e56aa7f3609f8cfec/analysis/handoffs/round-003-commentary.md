# Round 3 — commentary handoff

Status: **COMPLETE** for current-source readability (all classified source of checkpoint
pass-003). Final editorial audit is reserved for final acceptance.

## Input / carry
- Input pass-003 (maps/current.json == analysis/state.json, map_sha256 2d5d5cb5…48edd).
- Map changed since round 2, so the full round-2 commentary is preserved at
  analysis/round-2/commentary/ (+ analysis/round-2/scratch-commentary/); round-1 copy untouched.
  Fresh baseline via `commentary.py prepare`; notes/tools carried by physical identity
  (bank + CPU address). Pre-edit regenerated source differed from round 2 only in region /
  provenance lines; all 2,653 labels and 459 routine headers carried 1:1; no record invalidated
  beyond the L-019 targets below.

## Edits (lead L-019, commentary part — now consumed)
- `$051F` named **ExitDelay** (ObjXLo entry 15 reused as a global): delayed-exit countdown of
  bank-8 AreaExitCheck; loaded by every boss-defeat path (kind 13: 1, BossDefeated: 128,
  kind 39: 16, ObjKind42Defeated: 5); also kind 42's pattern counter during the fight.
- AreaExitCheck header rewritten: wrong `$050F` removed; five exit tables, boss areas need
  BossDefeatFlag bit7, bit5 delayed exit, area 12/13 special cases, exit sequence; r2-a1
  artificial clears cited. AreaExitFlags note lists boss areas (2, 6, 10, 14, 16, 17) and
  left exits (9, 13). Label .checkYBelow -> .checkAboveLine (matches the test).
- uncertainties.md ObjKind42Defeated item resolved (slot-0 clamp read remains open).
- Provenance: annotate.py now heads artificial-only observed regions "Observed, artificial
  scenario" (r1-a2 and r2-a1) and says natural reachability is not claimed.
- Concrete defect fixed: ObjKind33 note claimed its defeat skips area 16; r2-a1 met it in
  area 16, where the bit has no effect (open question recorded).

## Gate
- Readability: all 16 PRG banks + header/CHR/vectors; 0 Sub_/L_/Dat_ labels, 0 mechanical
  branch labels; symbol ExitDelay used consistently in all 7 operand sites.
- review.json 2,192/2,192 entries reviewed, 608 units reviewed; editorial_audit pending.
- [commentary/verification.json](../../commentary/verification.json): ok=true, byte_identical=true.
  Reading copy refreshed: commentary/reading-after.asm (32,792 lines).
- Classification unchanged: 262,158 / 262,160 B; 2 B unknown (0x1C5AE-0x1C5AF, L-018).

## Open (commentary/uncertainties.md)
- ObjKind42Defeated slot-0 non-indexed clamp; SkipArea16Flag bit7 purpose for kind 33;
  earlier items (object identities, area-map byte formats, weak-role data, orphan code).

## Toolkit
No toolkit issues found.
