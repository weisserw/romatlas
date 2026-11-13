# Investigated commentary limitations

The complete currently classified source has readable mechanical contracts. These
uncertainties are documented in source and do not denote skipped ranges.

- Ball/world X is three bytes and Y two bytes, with fractional low bytes. Height,
  horizontal speed and vertical velocity use two bytes, but original physical units
  and calibration are unknown. Source preserves all shifts and saturations rather
  than assigning invented real-world measurements. The optional ground-contact
  coefficient at RAM $BC probably models spin; its exact effect is documented at
  ApplyShotAccuracy and ground-contact blocks without asserting original intent.
- The third ball-related sprite is allocated only when zoom and its suppression
  flag are zero. Its Y depends on height; its exact display meaning is unresolved.
  Source uses suppressThirdBallSprite, avoiding an inverted or guessed shadow flag.
- Wide division is not assumed to implement an unrestricted 32-bit divisor: its
  high trial subtraction literally subtracts the high remainder byte from itself.
  Current distance/angle callers supply small divisors; source documents the actual
  algorithm. The random mixer also preserves carry between its two additions.
- Sound commands, software-slot fields and APU writes are explained, but original
  song/instrument names and each sound stream's full event extent are unresolved.
  Compressed PPU output has literal, escape, compact and chained-run modes. It is
  not a generic literal-versus-repeat encoding; source explains its actual flags.
- Broad tail data contains several consumer-backed formats and exact uniform runs.
  Original linker fill purpose, every table's unused selector domain and internal
  record boundaries remain unresolved. Generic Data notes state these limits.
  CHR banks may hold both displayed tiles and course data read through PPUDATA.
- Natural reachability of SetRenderingMask and the complete selector domains of
  static dispatch tables remain open. Operand-entry tricks are supported statically
  and preserved, without claiming every alternate entry was naturally observed.
- RAM $45 and $81 remain mechanically named reset/launch work bytes: their wider
  gameplay meanings were not recovered. Scratch bytes are shared by multiple
  algorithms, with routine-local contracts rather than invented permanent roles.

## Known upstream metadata correction before final acceptance

Physical $0556..$0557 (CPU $8546..$8547, PRG context [0,1,2,3]) is currently labeled
HandicapCursorPositions in maps/current.json and analysis/state.json. Its evidence
says "Two sprite coordinate values $44/$6A consumed by $850B indexed load".
The actual consumer loads the byte at $850B, moves it to Y at $850E, then stores
menuSelection to $0000,Y at $8511. These are zero-page destinations $44
(twoPlayerMode) and $6A (teeChoice). Readable source now names the bytes
SetupChoiceDestinationBytes and explains that contract. Data classification,
Static provenance, physical length two and boundaries are correct.

Redirect should make a narrow metadata-only label/evidence correction in the
current map and internal state, preserving this frozen baseline and existing
source. Suggested label: SetupChoiceDestinations; evidence: two zero-page
addresses used by the indexed store to selected setup field. No new experiment
or code/data change is needed. Final acceptance must reconcile that correction;
this commentary phase does not silently mutate its input baseline.
