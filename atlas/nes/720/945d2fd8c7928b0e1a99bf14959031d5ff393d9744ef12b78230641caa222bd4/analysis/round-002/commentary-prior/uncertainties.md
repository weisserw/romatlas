# Commentary uncertainties

Carry forward relevant phase-1 ambiguities as their regions are reviewed.

## Round 001 reviewed mechanical limits

All currently classified instructions and data boundaries are explained in source/game.asm; generic data retains the accepted bounded inspection evidence. Names are descriptive static interpretations, not original author symbols. The 10,954 unknown bytes in 126 intervals remain undecoded with local notes. ROM table equates may point into these intervals: an understood consumer/base is not an extent or classification proof.

- PlayerMotionState, PlayerHeading, PlayerHeight and trick/equipment fields describe their observed/static mechanics. Exact trick names, equipment upgrade identities, height/time units and animation-state identities remain unresolved. The ramp geometry interpolator, modulo-16 turning, signed velocity approach, gravity and scoring accumulator behavior are explained without those names.
- Map records expose a 4x4 metatile, two-bit palette quadrants and separate 4x4 collision grid. Collision-byte terrain identities and the full bank0 record/script grammar remain unresolved. Graphics/map payload banks2..11 remain generic Data, not a claimed complete tile schema.
- Zero-page transfer, script and collision scratch are overlaid. High-score records also overlay collision working storage. symbols.inc documents these contracts; high-score indexed accesses use record-base aliases to avoid implying live collision rectangles there.
- Audio sequence commands, pattern events, duration/tie fields, relative pointers and four APU voice paths are explained. Individual tune/effect identities and complete stream extents/subdivisions are unresolved. Music/effect bank selectors remain mapper register values, separate from the effective 8 KiB indices stored in map views.
- The IRQ poll and DMC-enable clearing mechanics are visible, but the intended interrupt timing role is unresolved. The supplied debugger is not a hardware-compatibility proof.

## Concrete redirect questions

1. Recover the remaining fixed-bank mixed code/data around physical $1F4CE..$1F98F and $1F9EE..$1FFFA (L005). ReadHighScoreText, FindHighScoreInsertion and WasObjectRenderedPredicate now identify known consumers and entry contracts. Inspect adjacent consumers/control flow and split independently supported code/table subintervals. Do not require natural entry merely to classify a statically supported routine; pointer plausibility alone remains insufficient.
2. Follow CheckWorldInteractionOrEventExit (CPU $D657) through $D6B1/$D780/$D7D6 to establish the result/timer/class transition contract (L006). Existing event scenes supply repeatable inputs. Read EventCountdownOnes/Tens, TimeExpired, EventExitReady and EventResults together before selecting a controlled transition. A changed RAM selector alone would not prove a successful result/medal/ending visit.
3. Follow DispatchWorldCellBehavior ($B470) and its high-nibble handler directory together with ProbeWorldCollision ($EE0F), WorldCellInteractionTable and BeginParkSpecialInteraction ($D980) for shop interactions (L007). The low-nibble flags and per-cell indices are established mechanically; exact handler roles and result updates require bounded connected analysis. Unknown handler-table entries must not be marked code solely because they look like pointers.

These are useful connected leads, not a terminal rejection/exhaustion assessment. No new structural annotations were proposed or accepted during commentary.
