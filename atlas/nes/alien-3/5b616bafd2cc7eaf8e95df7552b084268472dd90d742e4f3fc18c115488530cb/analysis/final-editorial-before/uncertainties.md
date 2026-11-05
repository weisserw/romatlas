# Current-source uncertainties — pass-003

All 30,338 code bytes and 231,814 data bytes have current source explanations.
Generic data retains its non-code classification while original organization remains
unresolved. Prior uncertainties and valid review history are preserved intact in
analysis/round-2/prior-commentary; the superseded scope statements there describe
pass-001, not this source.

- R2-L01: four bank-14 physical half-open intervals remain unknown:
  1D0FD..1D100, 1D4F5..1D4F6, 1D59B..1D59E, 1DACE..1DACF.
  Their isolated JMP CF65/RTS encodings and adjacency do not establish code or data.
  Source preserves exact bytes without speculative labels/decoding. Redirect should
  compare the now-complete neighboring state handlers, termination paths and finite
  state dispatch domains against each fragment; retain bank-aware physical identity.
- State/action mechanics are described without inventing original animation, actor
  or weapon names. ResourcePool074B..074F, numeric player states and reserved actors
  retain mechanically supported names where visual meanings remain unresolved.
- All fifteen actor type domains and their matched frame/behavior state counts are
  explained. Frame bank 12 is temporary; behavior dispatch restores the level bank
  with bank 13 in A000. Representative low-slot views do not prove physical consumers
  of all low-slot data reads. Natural state selection remains separate.
- The audio domain is classified and its interpreter/table origins are explained.
  Exact composer stream packing and every original subtable boundary remain unknown.
  Screen resources have parsed consumer-selected tile/OAM streams; surrounding
  animation bytes retain generic explanations where their schema is unresolved.
- Standalone renderer occlusion helpers, DC26 nonlocal timeout abort and E58B
  coordinate convergence have static contracts, without established natural callers.
  PLA return-address removals throughout weapon/contact/title paths are intentional
  nonlocal control effects, documented beside their entries.
- Ordinary completion, bonus, ending and final-score return mechanics are explained.
  Existing artificial progression verifies the exit rectangle/continuation, not
  natural guardian defeat, natural ending reachability or complete hardware fidelity.

These are localized meaning/reachability/classification limits, not skipped source
readability. Final editorial/archive acceptance remains pending; this phase did not
run the final audit or change structural classification. Evidence: analysis/round2/
review.md and analysis/redirect/r1-20261105-a/findings.md; replay routes remain in
analysis/scenarios.json and analysis/gameplay.md.
