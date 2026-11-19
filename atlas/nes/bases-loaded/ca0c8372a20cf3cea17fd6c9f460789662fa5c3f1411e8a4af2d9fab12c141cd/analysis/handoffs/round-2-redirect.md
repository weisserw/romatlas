# Round 2 — redirect phase handoff (attempt r2-a1, 2026-11-19)

## Scope and checkpoint
- Input: checkpoint pass-002 (byte_identical true), commentary handoff round-2-commentary.md, leads
  L-012, L-026, L-028, L-029, L-030. ROM classification at input: 100.00% (327695 / 327696; exact unknown
  complement = 1 byte, phys $19CBC, bank 3 $9CAC, prg_banks [12,13,14,15]).
- Redirect changed research/lead records only (maps/, analysis/state.json, commentary/, input/ and the
  configured binary untouched; ROM sha256 ca0c8372... re-checked). Research: analysis/redirect/r2-a1/
  (notes.md F1-F6, proposals.json, l028-checks.json, deadrts.json, experiments/x1-x4 + JSON results, 2
  screenshots). r1-a1 left unchanged. Server stopped.

## Findings
1. L-028 ($9CAC): code-vs-data is not materially ambiguous. The byte is an RTS filling the exact 1-byte
   gap between FillNametableTile1's final JMP $9C94 and DrawBatterLine's observed entry $9CAD. 26 other
   standalone RTS bytes in that position exist and all are Code, including 7 unreferenced L-024 null hooks
   (NullHookABDD/BB35/C052/C38B ...) accepted in round 2 with no more per-byte evidence. No pointer, word,
   immediate pair or indexed data consumer can reach $9CAC. The old rejection treated it as an isolated
   return opcode; the combined constraints are new (notes.md F1).
2. L-029: $A39C ($40) belongs to AnimationStepTimes (5 bytes, like the sibling tables FigureStepTimesA/B);
   RTI cannot be the first instruction of the JSR-style DrawStepFigure, which starts at $A39D (F2).
3. L-030: ScaleLaunchByHomeLead has a static caller (bank 0 $9B6C, FarCall A=$10, computer mode, top half).
   DivU16Z(wins, games<<8) = 0 for games >= 1, so it always jumps to $86A9 and forces LaunchSpeed=$0400
   while the home side leads. Runtime X1: 5/5 entries took that path. The MulU8 mismatch only matters
   when games = 0, which does not occur in pennant play (F3).
4. L-026: both season-end exits stall in natural play too. SceneInit17 starts its music before
   SaveGameState zeroes DpcmRequest. The main loop then never performs the requested scene change. X2
   showed that the losing exit (A press, then scene 9) stalls and that the $FF control reaches the title (F4).
5. The "Start" comments at SceneFrame17/18 and EndingStartToTitle are wrong. They test bit 7, which is
   the A button (ram.inc; X2) (F5).
6. L-012: X4 decoded the natural password EDCBAAK to team 3, opponent 4, games 2 and wins 1, which matches
   game-end RAM. PennantCounterA is the game number: it is 1 during game 1 (F6).

## Ready leads (exact ranges: analysis/redirect/r2-a1/proposals.json)
| Lead | Kind | Bytes | Recipe |
|---|---|---|---|
| L-028 | code | +1 (last unknown) | annotate phys $19CBC+1 Code/Static conf ~0.6, view $9CAC [12,13,14,15], label NullHook9CAC, reachability unresolved; review with l028_checks.py/deadrts.py |
| L-029 | boundary | 0 (byte-identical) | AnimationStepTimes Data $1A3A8+5; DrawStepFigure Code $1A3AD+31 ($A39D) |
| L-031 | commentary | — | A-button comments; ScaleLaunchByHomeLead caller/effect; SceneInit17 stall note; PennantCounterA = game number |
Projected after L-028: 327696 / 327696 classified (0 unknown). Final editorial audit is still required afterwards.

## Resolved / unchanged
- Resolved: L-026 (behaviour), L-030 (semantics), L-012 (decode verified). These now feed L-031 only.
- Unchanged: L-005..L-008, L-013, L-014. They are label-only with no structural question.
- Round-1 heuristic generic-data rule and L-024: the survey found no contradicting control flow. The three
  heuristic data blocks that start with $60 are long, visibly-data tables. Not reopened.

## Remaining avenues and deferral reasons
- Natural 132-game season end was not played. The stall conclusion rests on a deterministic code path plus
  artificial X2/E4 runs, and further play would add no bytes.
- S12 DPCM capture is still only attempted. The samples are already classified, so a capture has no
  structural value.
- No other promising structural avenue remains. The unknown complement is empty once L-028 is accepted.

## Gameplay (analysis/scenarios.json)
Added S14 (season-end losing exit, artificial), S15 (password decode), and S16 (LaunchSpeed handicap,
artificial). The S10/S11/S13 artificial routes are reused. No new snapshots were exported.

## Recommendation
(a) Run another disassembly round with ready leads L-028 and L-029, then commentary with L-031.
Not terminal, so no rejection review is needed.
