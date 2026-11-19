# Round 1: redirect handoff (attempt r1-redirect-01, incl. resume)

Status: COMPLETE - ready for disassembly. No map/state/source/ROM changes (state.json sha256 = pass-003
0408226e..., input/game.nes a71e4cc8...). Toolkit untouched; configured binary used; all servers stopped.

## Scope and input
- Input checkpoint maps/pass-003.json (sha256 0408226e...), 9483 Unknown bytes; commentary handoff
  round1-commentary.md (findings 1-7) and leads L-R1-01..07.
- Attempt dir analysis/redirect/r1-redirect-01/: proposal-static.json (+tools/proposal.py), residual-ledger.json,
  residual-dispositions.json, runtime-coverage.json, notes.md, snapshots mainmenu/menu1team/menu2team/league-final.snap.

## Findings
1. Static bank-aware descent from table-referenced handlers (MainStateHandlers 30-41, step tables, runner
   state 5, substitution/pre-game/result pages) gives 7746 Unknown bytes of code/inline/table with 0 starts
   misaligned against pass-003 code; 2 Data->Code corrections (b2 $8A78-$8A90, b3 $BC04-$BC2F, both jsr targets).
2. Runtime (ordinary inputs unless marked): HIRE/REMOVE/FIRE (1 custom slot) and TRADE (2 slots) all
   exercised to completion; VS pre-game menu all 5 options; league season end -> CHAMPION (MS 40/41)
   via a 2-team CPU league with simulated games. Artificial: fielding-side substitution (MenuSide patch),
   MS 40 direct, MS 24 with PitchResultFlags=1 (result money/attendance pages). Observed starts:
   L-R2-01 1312/1727, L-R2-02 156/209, L-R2-03 764/1476 instruction starts, 0 misaligned anywhere.
3. $A07A/$A07B overlapping instruction streams (commentary Q1) resolved statically (L-R2-04).
4. Bank 14 $C000/$C01C/$C038 are shadow-reload prologues parallel to ReloadPrgBank $C054 (L-R2-09).
5. Commentary correction confirmed: TRADE needs >=2 used slots, HIRE/REMOVE/FIRE >=1; HIRE refuses
   with TOO MANY PLAYERS until a player is fired.

## Ready leads for the next disassembly pass (bytes = pass-003 Unknown covered)
- L-R2-01 (high) MainStates 30-37 bank 7 + bank 15: 3858
- L-R2-02 (high) MainStates 40/41 bank 11: 480
- L-R2-03 (high) bank 2/3/15 pages, tables, 2 Data->Code corrections: 3477 (+69 Data bytes reclassified)
- L-R2-05 (medium) runner state 5 bank 12: 29
- L-R2-04 (low) b13 $A07A: 1
- L-R2-06 (medium) consumer-referenced residual data: 1058 (92 ranges)
- L-R2-07 (low) pattern-only generic data/strings/map continuations: 404 (23 ranges; b0 $87E7 = SndMusicTable head)
- L-R2-09 (low) bank 14 prologues: 6
Total ready: ~9215 of 9483. Recipes: load proposal-static.json groups (code/inline/tables with given
prg_banks), residual-dispositions.json for per-range assignment; rebuild and verify byte-identical.
Apply L-R2-01..05 first, then re-run residual scan before L-R2-06/07 boundaries.

## Deferred / rejected (compact rejection review)
- L-R2-08 (268 bytes: b3 $AC36-$AC46, $AC73-$AC97, $B39E-$B3AE, $B496-$B4C4; b7 $B820-$B83A; b9 $ACC2-$ACD4;
  b12 $8DE7-$8DFD; b14 $C126-$C16B): code-shaped, no caller/selector. Checked: JSR/JMP/word byte search
  (negative; JSR $C000 matches only in CHR), all resume traces (no fetch), contracts ($C126 ~ PpuBufAppendInline
  variant, but no consumer). Decoding alone is not structural proof; data reading is equally unsupported.
  Remains Unknown unless the reviewer accepts a contract argument; retry only with new reference evidence.
- L-R1-06 closed (season end reached). L-R1-05: $B71A resolved via GameResultSteps; $830F has no residual impact.
- L-R1-02: remaining $C178 tables lie in classified regions; no residual depends on them.
- Not pursued: uncalled MapTablePtrs maps (already Data; meaning only), $2C PlayEndHold (semantic only),
  $BC04 runtime (artificial result pages stalled at MenuToggle 3; static jsr evidence suffices),
  further state-25 result pages (need a real completed game with special result; long, no byte gain).

## Remaining redirect work
None identified with classification value; next step is disassembly of the ready batch, then commentary
for the new code (states 30-41 menus, pre-game pages, result pages) using runtime-coverage.json and
scenarios S12-S16 for observable behaviour. Toolkit: TK-002 (WRAM debug writes) reproduced, worked around.
