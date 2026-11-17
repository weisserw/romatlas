# Round 2 disassembly handoff (pass-002)

ROM classification: 99.56% (391505 / 393232 bytes classified; 1727 bytes unknown). Status: incomplete.

Checkpoint: maps/pass-002.json -> generated/pass-002; analysis/pass-002-verification.json ok, byte_identical
(map sha256 ebcf33141ae8efb692f3dc5bf143a44f852bc6ffba84e5b4f3c71be98e6b92e1). Internal store analysis/state.json
(3,985 regions) = maps/current.json. Serve process (scratch/daemon.py) stopped after map.save.

## What was applied
- Redirect batch r1-redirect-a: L014-L020 (29,037 bytes) + L022 (569) = 29,606 bytes, after independent checks
  (reproduced proposals.json byte-identically; closure 0 bad paths/0 misaligned overlaps; 0 observed starts in Data
  groups; copy identity + instruction alignment re-verified). Nothing rejected; three partial inline jump tables
  narrowed to their unknown words. L014: 15 phase-1 Data regions split (code/inline/JT + Data remnants), plus the
  L022 $9F83 split.
- Provenance: code starts fetched in r1-redirect-a traces = Observed (evidence notes artificial scenarios S15-S19);
  everything else Static. Stale copies (banks 11/15/17/19/21/25) Code/Static conf 0.8, reachability none known.
- Static supplement on L021 residue: 520 bytes (details in analysis/investigation.md pass-002 section).
- Gameplay: S21 ('?' speciality), S22 (1P vs 2P, ERROR ON, port-2 input, into fielding; snapshot
  scratch/snaps/vs2p_errors_on_fielding.json). No residual bytes fetched; gameplay stopped for lack of gains.

## Code/data totals
Code 89,010 (Observed 53,697 / Static 35,313), Data 171,423, CHR Image 131,072. Per-bank counts: analysis/progress.md.

## Open questions (lead L021, analysis/pass-002-remaining.json, 83 runs)
- Biggest: bank 25 $A040 (169; starts PHP after table $A02B-$A03F of unproven extent), $A120 (164), $B82B (97),
  $B8DD (82), $B8A2 (39), $B6CC (38), $AC66 (32); bank 3 $B0B3 (157), $BAE2 (48); bank 15 $ADEC (96; routine + digit
  table $AE1E), $A252 (28), $AAA9/$AB1D; bank 11 $AFCE (70); bank 10 $8F10 (63), $8F5D (22); bank 24 $8337 (47).
- Missing fact: an entry (no JSR/JMP/word/immediate-pair reference found) or an independent extent proof. Most are
  coherent routines in pair 12 (player select / team editing). Possible routes: unvisited sub-states of modes 22/23
  (PLAYER SELECT) and league editing (28/29), VETERAN acquisition; or accept "Code/Static, no known entry" where the
  start follows a terminator and closure fits exactly (L022 standard) - redirect may evaluate per island.
- Many 1-12 byte residues sit between code and data (table heads/tails, orphan bytes); low value each.
- Deferred gameplay: VETERAN acquisition, >2-team league (L006).

## Leads
Consumed/verified: L014-L020, L022. Updated: L021 (open), L006 (deferred). Produced: none.

## Toolkit
No new issues (TI-001 traced-frame slowdown observed again). analysis/toolkit-issues.md.

## Scratch
scratch/p2/: chk1-4.py (batch checks), plan.py/plan.json (batch), plan2.py/plan2.json (supplement), apply.py,
pl.py (play helpers, port-2 input), frames/*.png. Commentary for pass-002 regions is pending (commentary phase).
