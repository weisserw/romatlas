# Round 4 — commentary handoff

Status: **DONE**. Current-source readability is complete for checkpoint pass-004. The final editorial audit was not
run, by design. Classification is unchanged from disassembly: 262157/262160 bytes, 3 unknown, so the ROM is **not complete**.

## Scope and carry-over
- The map changed from pass-003 to pass-004 (sha256 4582999d...). The only change is the two bank-15 NOPs, which went
  from Unknown to Static Code.
- Round-3 commentary was preserved first in `analysis/round-3/commentary/` (diff -r identical, game.asm sha256
  2a5b8906...). The round-3 commentary_build.py, commentary_review.py, r03c tools and handoff are kept alongside it
  (analysis/round-3/README.md).
- `commentary.py prepare` then re-created commentary/ from pass-004. The new before/, baseline.json,
  phase1-state.json and reading-before.asm are frozen at pass-004. notes/annotations.txt, progress.md and
  uncertainties.md were restored from round 3.
- Carry check: analysis/tools/r04c/carry.py, output in analysis/evidence/round-04/commentary-carry-check.txt.
  Every fn/lbl/cmt/op/xb/dat/rgn record still hits the same instruction or region. The only notes invalidated were
  the two bank-15 unknown-range notes.
- review.json: the scaffold has 3,272 entries and all are reviewed. 3,270 were carried from round 3 with the same
  range and kind; 2 are new (the NOPs). There are 1,081 units (1,079 + 2), all reviewed. editorial_audit is pending.
- Tool updates (workspace only): the source title line now says round 4 / pass-004 (it had wrongly said round 2 /
  pass-002 in round 3). The review tool now compares against the round-3 archive.

## Source changes (diff vs round 3: 50 lines, all intended)
- `NopBeforeWaitFrame` ($E0F6, phys 0x1E106) and `NopBeforeIrq` ($E1B3, phys 0x1E1C3) each get a header. They
  are described as statically classified NOPs (L-022) that fall straight into an observed entry (WaitFrame or IRQ),
  with no known reference. They are not called unused or dead, and the padding-versus-leftover question is left open.
- The 3 bank-1 RTS unknown notes ($ADB6, $B2EA, $B343) were rewritten to cite the round-4 L-023 rejection. They
  state that an unreferenced RTS after a tail `jmp` has no control-flow contract, so code versus data stays open.
  The $B343 note contrasts this with the two RTS-less `jmp` exits of FightFish_PassiveDirection.
- uncertainties.md has a new round-4 section.

## Readability counts (analysis/evidence/round-04/commentary-labstats.txt; game.asm 34,955 lines)
- Address-only names: **0**.
- Numbered generic labels: **0**.
- Global `_N` suffixes: 0.
- Raw RAM operands: 0.
- Unreviewed scaffold entries: **0**. Unreviewed units: **0**.
- Routine headers: 707 (705 before, plus the 2 NOPs). Globals: 1,257. Locals: 686.
- Unknown bytes: 3, and all 3 have notes.

## Verification and reading copy
- `commentary.py verify`: ok true, byte_identical true. This is commentary/verification.json, game.asm sha256
  938f3bed2ae3ff3f16ac948560e0b497cbd380f08366c00e978a32ecbebba26c.
- `commentary.py read`: commentary/reading-after.asm refreshed.

## Unresolved
- Everything listed in round 3 is unchanged (commentary/uncertainties.md):
  - FishState $A0;
  - FishAheadHandlers facing 8;
  - FatigueDrainFlag and BitArrayPtr have no writers;
  - the roles of the OffsetPair16/18 operands;
  - species per HookedFishType;
  - fight-model quantities;
  - sound semantics.
- The 3 RTS bytes in bank 1 remain unknown (terminal residual per round-4 disassembly).

## Structural questions for redirect
- None new. Commentary found no new evidence on the 3 RTS bytes. L-023 can be reopened only with one of the
  revised-test facts listed in round-04-disassembly.md.

## Deferred
- Final editorial audit (`commentary.py audit`) at archive acceptance.
- Optional headers for the 343 header-less stubs/continuations.

## Leads / toolkit
- Leads: L-022 and L-023 are documented; no new leads.
- No toolkit issues found; analysis/toolkit-issues.md is unchanged.
