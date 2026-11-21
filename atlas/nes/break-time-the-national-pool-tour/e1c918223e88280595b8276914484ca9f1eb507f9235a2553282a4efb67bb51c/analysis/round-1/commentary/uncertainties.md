# Commentary uncertainties

Carry forward relevant phase-1 ambiguities as their regions are reviewed.

## Round 1
- CPU shot-type routines in bank 6 (ShotType*, TryCushions*, TryCombination*): the
  mechanics (paths, cushion mirroring, trial simulation) are clear; the exact game
  meaning of each variant (bank vs kick vs combination onto the 9 ball) is tentative.
- MatchGames ($9F): bits 0-1 games played, bits 2-3 target, bits 4-6 per-game winner
  bits (from GameResult/ShowMatchScore) - inferred from code, not observed in a match.
- RoundState ($A3) values 1/2/3 (foul, break/first shot, lag) are inferred from their
  tests; only $80 (rack over) is certain.
- WatchCpu1 ($CC) doubles as the title timeout flag set by a sound-script command.
- MetaStringStep ($CA6D) appears idle (no classified writer of MetaStringIndex).
- Bank 7 $A41B reads ROM $C054 with LDA where JSR Random was probably intended; the
  branch it controls is never taken.
- Data labels for tables in unknown bytes give the CPU address used by the code only;
  table extents are not proven (L-009).
- Unknown gaps: notes are heuristic from neighbours (string terminators, observed-read
  tables, code gaps); specific notes for $82E8, $83C2, $8E39/$9CD1 area, bank 6 $95FF,
  bank 14 $C33E.
