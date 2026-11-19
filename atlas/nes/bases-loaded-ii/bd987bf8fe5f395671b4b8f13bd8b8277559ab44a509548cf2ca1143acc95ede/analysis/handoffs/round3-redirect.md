# Handoff: round 3, phase 3 (redirect), attempt r3-redirect-a

## Scope
- Checkpoint maps/round3-disassembly.json (0 unknown bytes); map not changed. input/game.nes
  unchanged; toolkit not modified. Research: analysis/redirect/r3-redirect-a/ (findings.json,
  experiments/, tools/, logs, snapshots).
- Questions: (1) evidence against existing classifications (low-confidence code, generic
  data bounds); (2) L-020 and related commentary semantics; (3) milestone checklist.

## Findings
- **L-020 refuted.** PitcherStintMark ($06C4/$06C5) is a per-side pitcher stamina counter.
  - Bank-0 pointer table D_B00_919B points to $06C5 and $06C4.
  - LaunchPitchedBall decrements it once per pitch (floor 0) through `sta (Math0),y` at $925B.
  - PlayState_22 and CpuPitcherDecision read it.
  - The round-3 scan covered direct operands only (writers-06c4.json).
  - Runtime with no edits (x2): one decrement per pitch on each side. The value persists
    across at-bats and the side change.
  - Artificial check (x3): setting $06C4 to $40 produced CPU relief (CpuInputState $70-$78,
    substitution screens), then a reset to $FF.
  - The limit tables give 135..215, so relief needs roughly 40-120 pitches. It is reachable
    in long games but was not observed naturally.
- **Region 1985 (bank 0, $919B-$91D4, 58 bytes) has an exact layout:** 2 stamina RAM
  pointers, 3 limit-table pointers, then three 16-byte descending tables.
  - The first pointer target equals the end of the pointer table.
  - The last table ends at the end of the region.
- **Password/CONTINUE now visited with real inputs (x4b):** the password was entered and
  accepted, and the season was restored. The open-loop first attempt failed and is kept.
  - The letter roll covers 0-25, but the source says 0-24.
- **Low-confidence code (42 regions ≤0.6, 1,295 B):** no data-access references and no
  external control references (lowconf-refs.json). No contradiction, so no correction.
- **ScreenLoadId:** the immediate ids seen are all ≤$15, consistent with 22 layouts (partial).

## Ready leads
- **Classification/map (label/split only, Data stays Data):** L-021, the region-1985 split
  and labels.
- **Commentary only:**
  - L-020: rename to a stamina name and fix the "unreachable" notes (symbol,
    CpuPitcherDecision, $FF stores, LaunchPitchedBall, PlayState_22, uncertainties).
  - L-022: the password roll range is 0-25; add the CONTINUE observation.

## Deferred / rejected
- **L-018 (debug code):** no references again. An artificial jump would only show
  straight-line mechanics, and no natural route exists.
- **L-005:** no new evidence. Its only value would be a trace, with no classification
  impact.
- **L-004 (other generic blocks):** meaning only. One block was resolved (L-021).
- **L-009:** voice and CPU pad are static readings. Diminished prospect.
- **Natural stamina relief:** needs a long, unpatched game. The mechanics are already shown.
- **Attract-demo watch (x1):** exceeded its background time limit (7,200 frames with
  tracing) and was not rerun because L-020 was answered without it.
- **Pitch-type byte 5 domain (0..2 expected):** not enumerated.

## Rejection review
- No classification contradictions were found, and no unknown bytes remain.
- Remaining questions are semantics-only, or need long natural play or unsupported
  assumptions.

## Milestones
- All later-game and ending milestones are visited: the ending through the r1 artificial
  route, and CONTINUE now as well.
- Added: MS-cpu-relief (artificial) and MS-stamina-natural.

## Recommendation
- Do a commentary-only update (L-020, L-022), plus a small map label/split for L-021.
  This can be one short disassembly-label step, or it can be folded into final acceptance.
- No further disassembly research cycle is needed. Then proceed to final acceptance.
