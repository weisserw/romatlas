# Commentary uncertainties

Carry forward relevant phase-1 ambiguities as their regions are reviewed.

## Round1 source mechanics and unresolved meanings

The audio engine (physical457..2317) uses ten parallel voices, five effect and five music slots, stream note/envelope interpreters and per-voice RAM stacks. Names describe those mechanics; request IDs, instrument identities, song names and exact note tuning remain unresolved. Some recognized command handlers end at unknown gaps; source explicitly limits those descriptions.

Password source physical20812..23618 distinguishes display bytes at CPU$0600 from sixteen five-bit symbol indices at zero-page$E9. Special-password matching reuses Y after a mismatch rather than restarting every comparison at fifteen; the source records that exact behavior. Ordinary unpacking and checksum are described without assigning every decoded progression bit. Checksum ADC carries persist between bytes, so the comment does not claim a plain modulo sum.

Gameplay source physical81936..95670 supports movement, leading-edge collision probes, packed map coordinates, entity allocation, timed cell updates, resource consumption and two-sprite rendering. Tile IDs and entity species remain mechanical where consumers do not establish their game names. Player and entity coordinates are separate arrays; neither is a physical ROM identity. Inputs are named held/pressed from their copy/use patterns; fixed-bank controller normalization still needs the pending review.

Concrete redirect question from provisional (uncredited) bank12 reading: CPU$991D (physical104749) tests actor position against parallel map-node rectangles; matching writes progression/area index CPU$014F, calls fixed-bank eligibility$C49A then gate$9E7E. Gate$9E7E adds special constraints for index14 via$9E26 (requires four predicate successes for indices10..13) and flag$04 of$014E. This is a promising entrance/progression contract for the saved cave/King maps. Before runtime edits, finish commentary of bank12/fixed-bank predicate, inspect exact rectangle fields$9983/$9996/$99A9/$99BC/$99CF and mode partitions. These are hypotheses from code mechanics, not a claimed King-entry proof or classification proposal.

## Round1 completion correction / actionable entrance contract

The earlier provisional bank12 note is now reviewed and explained in source. Fixed-bank CPU$C49A is specifically the packed progression completion predicate: even node indices test upper nibble, odd lower nibble of$0150+index/2, with fifteen meaning complete. CPU$991D accepts only an incomplete matched node and requires carry set from$9E7E. Node index fourteen additionally needs nodes ten..thirteen complete and map flag$014E bit four; other indices pass this extra gate. Rectangle fields use actor page/X/Y and presentation-specific index partitions. This identifies a bounded redirect experiment: log the candidate node selected by actor page/X/Y on saved cave/final maps, inspect its packed completion nibble, and test the index-fourteen gate only when that node actually matches. Do not infer every cave/final entrance is node fourteen, or call a state visit from patching alone.

Fixed-bank controller source confirms held$08/$09 and newly pressed$0A/$0B, with A80/B40/Select20/Start10/Up08/Down04/Left02/Right01. Actor engine (eight objects) and puzzle entity engine (32 objects) reuse RAM with different layouts; symbols preserve mode-local aliases rather than asserting a shared structure. Script/native helper$D10B and PPU-transfer helpers$D439/$D43F rewrite CPU return addresses to skip inline words, explaining why unresolved JSR continuations must not be blindly classified.

All currently classified code is now mechanically explained. Remaining uncertainties concern original game meanings, unknown ROM boundaries and reachability; none is a skipped classified-source range. Final editorial acceptance remains a separate pending gate.

## Round2 reconciliation

The former entrance experiment is resolved by redirect evidence: node 18 King and node 5 cave initialization use `$014F=node,$014E=$20` through the existing map transition; boss phases and THE END were visited artificially. These findings supersede the provisional missing-entry question above. Ordinary node completion explicitly excludes node 18. Source now names the 19-node rectangle/presentation fields and the inline four-by-four completion overlays. See analysis/redirect/r1-node-and-residual/HANDOFF.md and analysis/disassembly-002/HANDOFF.md.

The 31 actor templates supply split initial-script, simple-frame table and composite-frame table pointers. Native handlers and script bytes remain distinct. Residual mixed banks 2/4/8/9/12 need interpreter operand/record bounds and supported native callback contracts; script pointers alone do not establish either. This is the next redirect question, not pending commentary. Audio corpus classification remains generic where individual stream extents are unknown; counted voice lists and finite request metadata are specifically explained. Graphics 36-stream and puzzle 110-room formats are bounded and documented without claiming all selectors naturally occur.

Shared zero page has mode-local aliases. In particular the angle helper uses AngleMagnitude/AngleVelocity work words at $D5..$DC, which alias puzzle and projectile fields in other modes. Arithmetic comments explain exact half-square multiplication and quantized reciprocal scaling; original trigonometric/tuning generators remain unresolved. Tile/entity identities and dialogue selection meanings remain mechanical. Unknown ROM gaps retain boundary notes; all current classified-source mechanics are covered.
