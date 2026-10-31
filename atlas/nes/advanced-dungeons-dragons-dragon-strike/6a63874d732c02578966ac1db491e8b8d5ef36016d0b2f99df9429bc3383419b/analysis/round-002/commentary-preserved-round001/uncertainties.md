# Commentary uncertainties

Carry forward relevant phase-1 ambiguities as their regions are reviewed.

## Round 001 reviewed mechanics

- Audio FE selects a preset, FF restarts using the track/channel alternate pointer,
  FC resets the cursor and shares FB's stop path. E2 stores its argument minus one
  at $05DD; its consumer/game meaning remains unresolved. Individual song/effect
  names and complete per-stream extents are not asserted. Select-track arithmetic
  uses the carry from its second ASL, which is clear for track values below 64;
  it does not depend on the incoming caller carry.
- Audio scratch aliases are used only inside the two driver ranges. Other banks
  reuse the same RAM addresses; their classified operands now use contextual names
  where mechanics are understood. An address alias never establishes global ownership.
- The earlier opaque fixed31 helper frontier is now resolved mechanically: $EA8B
  uploads alternate nametable row halves; $F78D updates scene-dependent CHR registers;
  $FC5F initializes a palette fade slot; $FA98 fills one nametable; $FBEF clears OAM.
  CHR value-to-graphic assignments and the exact visual identity of the meter remain
  unresolved. These mechanics do not classify unknown table targets or backing bytes.
- PPU queue records store [length, address_low, address_high, payload]. The consumer
  accepts aggregate payload below 32 and has no zero-length guard. The producer reserves payload+3 bytes in a 256-byte ring, retaining a gap at the
  reader boundary. The alternate updater stages 64 tile bytes and two attribute
  slices, transferring one nametable half per NMI; flag bit 6 selects the half and
  bit 7 remains busy until both halves finish. Valid callers must supply nonzero
  dimensions and coordinate ranges; natural-data violations have not been asserted.
- PollControllers delegates to the active bank's $8027 when $A2 is negative. The
  bank-dependent alternate-input contract remains unresolved.
- Unknown PRG intervals remain structurally ambiguous; generic uniform Data and
  CHR are already structurally complete without semantic object assignments.

- Fixed31 now explains five-byte scene records and eleven-byte object records,
  resource accounting, contact rectangles and movement command bytes. Their game
  identities and bank-local APIs $8015/$8024/$802A remain unresolved. Fixed30 $CF23 is now the meter subtraction/depletion path and $DA10 quantizes
  direction between points. Remaining classified fixed30 helpers are now reviewed mechanically; $D880 is in an
  unknown interval and has not been decoded or reclassified.
- The alternate class-table pointer path in $E2FE uses carry from actor-state
  shifts for its high-byte ADC, replacing low-byte addition carry. The $83 movement
  clone path transforms category bit 7 to scratch value 0/2 before an appender that
  consumes bit 0, so that emitted category bit is clear. These are exact observed
  mechanics; their authorial intent is not established.
- Object movement tables retain six fractional bits in bytes 4/5; byte additions
  discard overflow above 255. Relative direction wraps once modulo 12. Contact
  scaling and script decoding rely on valid positive descriptor counts and finite
  command chains. These assumptions are documented locally, not runtime-proven.

- Stage-bank $8027 is a no-op in several banks; bank4's classified player reads
  timed [input,duration] pairs, decrements duration on the load frame and uses
  $FF as an option-bit sentinel. Other classified playback entries end after
  saving Y and calling $FD26; their unknown continuation is not assumed identical.
- Fixed30 scene-reference controls, palette feedback, marker effects, composite
  descriptors and global sprite omission now have source explanations. Exact
  game identities and $CC57, $CB63, $DECB/$DEDC marker tails remain unresolved in
  unknown intervals. No classification change is requested merely to improve prose.
- Stage special-actor contacts/dispatch are now explained per bank and scene.
  Bank16 scene-$13 tile-row handlers, bank18 row-event payload consumer $8A5E,
  and several initialization/state-helper bodies remain in unknown intervals.
  A useful future bounded check would observe the bank18 event consumer's returned
  Y/cursor and payload reads at one event boundary, preserving effective [18,19,30,31]
  context. Current source documents the entry mechanics without inventing that payload.


## Menu/password questions for later redirect

These are unresolved meanings/structural leads, not pending commentary. They do
not authorize classifying unknown helper bodies or expanding known table extents.

- Both title variants use a four-input Up,Down,Left,Right matcher and a twelve-input
  Up,Right,Down,Left,Up,Up,Down,Down,Up,Down,Left,Right matcher. Carry-clear gates
  silence audio before unknown continuations. The resulting mode/option effect is
  still unknown locally. Bank26's twelve-input matcher at CPU $9C27 lies in physical
  220215..220248; bank28 has a classified counterpart at $9DC0. Compare only bounded
  supported code/dependency ranges, or replay one existing title route and inspect
  state writes after the gate with effective [26,27,30,31] or [28,29,30,31] context.
  Do not infer the unknown tails merely from the matching input pattern.
- Bank27 CPU $A148 points into Unknown221528..229096; bank29's password entry prefix
  $A176 is Unknown237958..238023. Cursor/letter initialization is outside the current
  code inventory. A bounded future trace from password-menu.snapshot through the
  existing natural password-entry route could isolate the actual initialization
  writes and execution context before the classified bank29 $A1B7 fragment.
- Bank29 keyboard movement/repeat continuation is Unknown238401..238513 (CPU
  $A331..$A3A1); the post-letter-store continuation is Unknown238660..238708
  ($A434..$A464). Observe one direction/release and one A/B edit from a stable
  password snapshot, preserving [28,29,30,31]. The shown special-key paths adjust
  the write cursor, but unknown three-byte tails at $A407..$A40A and $A41D..$A420
  determine whether/how they rejoin the letter store. No blank/delete or accumulator
  carry-through behavior is assumed. Track the player, cursor and six-letter buffer.
- The paired bank27/bank29 direct image loaders use zero-terminated RLE: positive
  repeat counts; high-bit literal counts with $80 wrapping to256. Their bank-local
  pointer tables and payload extents are still unknown. A future bounded consumer
  read trace for one known image can establish an exact payload extent without
  treating all neighboring bytes as one stream. Keep CPU pointer values and physical
  source identities separate; fixed-bank destination pointers target PPU nametables.
- Bank28 scene setup uses parallel actor-count/state/contact/coordinate tables and
  palette/CHR slices in unknown backing intervals. Source names the consumers and
  record stride but does not classify tables. Valid nonnegative actor/palette counts
  must be nonzero where loops lack zero guards. Check one selected scene's actual
  reads/count domain before proposing bounded table records; do not promote generic
  aliases into structural evidence by themselves.
- Bank28's unresolved second-title wait is CPU $9D82, physical236946..236965;
  bank26's counterpart $9BE9 is inside Unknown220126..220156. Their classified
  callers establish countdown inputs, but not internal contracts. Unknown panel
  $9978 and title gate continuations likewise remain local uncertainty notes.
