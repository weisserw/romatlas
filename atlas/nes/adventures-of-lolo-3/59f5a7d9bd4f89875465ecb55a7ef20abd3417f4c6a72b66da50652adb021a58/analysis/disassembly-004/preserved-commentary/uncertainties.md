# Commentary uncertainties

Carry forward relevant phase-1 ambiguities as their regions are reviewed.

## Round1 source mechanics and unresolved meanings

The audio engine (physical457..2317) uses ten parallel voices, five effect and five music slots, stream note/envelope interpreters and per-voice RAM stacks. Names describe those mechanics; request IDs, instrument identities, song names and exact note tuning remain unresolved. New bounded register-write continuations now resume the note/envelope parsers; song identities and unbounded stream selection remain unresolved.

Password source physical20812..23618 distinguishes display bytes at CPU$0600 from sixteen five-bit symbol indices at zero-page$E9. Special-password matching resets Y=0F for every pointer candidate and compares sixteen display bytes backward. Ordinary unpacking and checksum are described without assigning every decoded progression bit. Checksum ADC carries persist between bytes, so the comment does not claim a plain modulo sum.

Gameplay source physical81936..95670 supports movement, leading-edge collision probes, packed map coordinates, entity allocation, timed cell updates, resource consumption and two-sprite rendering. Tile IDs and entity species remain mechanical where consumers do not establish their game names. Player and entity coordinates are separate arrays; neither is a physical ROM identity. Inputs are named held/pressed from their copy/use patterns; fixed-bank controller normalization was reviewed in round one.

Concrete redirect question from provisional (uncredited) bank12 reading: CPU$991D (physical104749) tests actor position against parallel map-node rectangles; matching writes progression/area index CPU$014F, calls fixed-bank eligibility$C49A then gate$9E7E. Gate$9E7E adds special constraints for index14 via$9E26 (requires four predicate successes for indices10..13) and flag$04 of$014E. This is a promising entrance/progression contract for the saved cave/King maps. Before runtime edits, finish commentary of bank12/fixed-bank predicate, inspect exact rectangle fields$9983/$9996/$99A9/$99BC/$99CF and mode partitions. These are hypotheses from code mechanics, not a claimed King-entry proof or classification proposal.

## Round1 completion correction / actionable entrance contract

The earlier provisional bank12 note is now reviewed and explained in source. Fixed-bank CPU$C49A is specifically the packed progression completion predicate: even node indices test upper nibble, odd lower nibble of$0150+index/2, with fifteen meaning complete. CPU$991D accepts only an incomplete matched node and requires carry set from$9E7E. Node index fourteen additionally needs nodes ten..thirteen complete and map flag$014E bit four; other indices pass this extra gate. Rectangle fields use actor page/X/Y and presentation-specific index partitions. This identifies a bounded redirect experiment: log the candidate node selected by actor page/X/Y on saved cave/final maps, inspect its packed completion nibble, and test the index-fourteen gate only when that node actually matches. Do not infer every cave/final entrance is node fourteen, or call a state visit from patching alone.

Fixed-bank controller source confirms held$08/$09 and newly pressed$0A/$0B, with A80/B40/Select20/Start10/Up08/Down04/Left02/Right01. Actor engine (eight objects) and puzzle entity engine (32 objects) reuse RAM with different layouts; symbols preserve mode-local aliases rather than asserting a shared structure. Script/native helper$D10B and PPU-transfer helpers$D439/$D43F rewrite CPU return addresses to skip inline words, explaining why unresolved JSR continuations must not be blindly classified.

All currently classified code is now mechanically explained. Remaining uncertainties concern original game meanings, unknown ROM boundaries and reachability; none is a skipped classified-source range. Final editorial acceptance remains a separate pending gate.

## Round2 reconciliation

The former entrance experiment is resolved by redirect evidence: node 18 King and node 5 cave initialization use `$014F=node,$014E=$20` through the existing map transition; boss phases and THE END were visited artificially. These findings supersede the provisional missing-entry question above. Ordinary node completion explicitly excludes node 18. Source now names the 19-node rectangle/presentation fields and the inline four-by-four completion overlays. See analysis/redirect/r1-node-and-residual/HANDOFF.md and analysis/disassembly-002/HANDOFF.md.

The 31 actor templates supply split initial-script, simple-frame table and composite-frame table pointers. Native handlers and script bytes remain distinct. Round three resolves supported mixed script/frame/native families in banks 2/4/8/9/12 using finite operand, callback and record contracts. The remaining forty bytes retain their localized structural ambiguity. Script pointers alone still establish neither code nor full table extent. Audio corpus classification remains generic where individual stream extents are unknown; counted voice lists and finite request metadata are specifically explained. Graphics 36-stream and puzzle 110-room formats are bounded and documented without claiming all selectors naturally occur.

Shared zero page has mode-local aliases. In particular the angle helper uses AngleMagnitude/AngleVelocity work words at $D5..$DC, which alias puzzle and projectile fields in other modes. Arithmetic comments explain exact half-square multiplication and quantized reciprocal scaling; original trigonometric/tuning generators remain unresolved. Tile/entity identities and dialogue selection meanings remain mechanical. Unknown ROM gaps retain boundary notes; all current classified-source mechanics are covered.

## Round3 current limits

All 1856 classified regions and twelve unknown notes belong to pass003. New source explains actor/native/finite entity state families, counted frames/composites, PPU modes and inline words, password descriptors, ten replay streams, palette raw-byte attenuation and register-bounded DMC samples. Natural use is not asserted for the Static additions. Generic patterned data remains generic where specific consumers/field partitions are unresolved.

D7C8 bounded ordinary return needs start<=end<8000, end low byte not FF and a destination disjoint from pointers, interpreter state and live frames. It is not a universal terminating fill. Counter command21 field0/1 plus actor0..7 and operator0..2 supports AND/OR/ADC return tails; no parsed current command21 or natural use is claimed. Marker scratch01C3..01CB is disjoint from live frames under its bounded entry contract. C119 preserves partial lower context until literal selectors; register5 maps10/11.

Selector71665 encodes ten but only seven independently supported pointer words precede script71681. Its default-skip overlap and full saved-result domain remain unresolved. Composite templates may point into following script/native placeholders; nominal frame-slot count does not instantiate all frames. Tile/entity names remain mechanical where game identity is not established. D07D same-byte pointer low/high mechanics still lack a supported selected entry/format. Final editorial acceptance remains pending.
