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

## Round 002 current assessment (supersedes earlier structural leads)

The three round-001 redirect leads above are historical: pass-002 accepted their independently supported scripts, native callbacks, result/purchase helpers and data. Current source explains every classified interval; no readability range remains deferred. Generic data retains bounded inspection and honest schema limits. Classification remains 131076/131088 (99.99%), with twelve bytes unknown.

- CashDigits ($0575..$0577) is the three-digit cash account used by cash awards, affordability checks and equipment subtraction. TicketsRemaining is separate. NextTicketThresholdDigits ($0579..$057B) controls score-based ticket awards; the purchase operand base $057B plus interaction selector 5..8 addresses the four equipment levels. This is an encoded indexing origin, not evidence that equipment overlaps the threshold during normal accesses.
- Object-script command widths follow the thirty-four handler/count triples. Source now renders all 771 verified records, with explicit frame/command and callback/branch explanations. Native callbacks retain CPU stack conventions; script calls/repeats use the per-object RAM stack. Semantic animation/trick identities and natural script selection remain unresolved where not independently observed.
- Medal drawing reads bank-ten resources under [10,11,14,15] from D859 through D89B and in D8DC..D939. D89C resumes [0,1,14,15]. Variable resource copying at DAF1..DB50 establishes only fixed slots [null,null,14,15] until bank-zero restoration. Source comments do not resolve those switched slots from register values.
- RowSplitEncodedOrigin is CPU $E46C. The actual thirty-two-word continuation directory occupies $E46E..$E4AD, physical124030..124094. Counts one through thirty-two exclude the encoded-origin word; those two preceding bytes remain code.
- ShowParkInformationInteraction is the information map, not the purchase. Its supported entry requires type-$14 object existence; the absent-object path leaves saved timers on the CPU stack, so no ordinary game-bug claim follows. PurchaseSelectedEquipment is the script-$14 callback at BE90, separately checked in saved purchase evidence.
- Twelve isolated RTS bytes remain Unknown: physical7886 length3 in bank0; physical115437,118424,118459,118472,118494,119047 length1 each in bank14; physical126926 length3 in bank15. Source notes preserve their boundaries. Legal decoding, identical rebuilding and no observed use do not discriminate data from native entry. No semantic or classification change was forced.
- Full bank0 prefix record grammar, composite graphics/map asset schemas, collision terrain identities, exact trick/equipment identities and time/height units remain unresolved. Existing prose explains the established consumers and mechanics. Artificial event/result/purchase/ClassFinished observations do not establish natural park return after READY or a separate ending.

Redirect should evaluate the twelve residual role claims against evidence-policy.md before any terminal decision, combining independent boundary/control/consumer constraints and checking a concrete competing interpretation. No new ready classification proposal or terminal-exhaustion claim arises from this commentary pass. Final editorial acceptance remains pending; no audit or archive operation was run. No new toolkit issue was found.
