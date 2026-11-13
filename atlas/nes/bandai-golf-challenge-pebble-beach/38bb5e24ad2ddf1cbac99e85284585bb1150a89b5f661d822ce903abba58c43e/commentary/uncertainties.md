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
- Sound commands, software-slot fields and APU writes are explained. Round002 now
  establishes all 22 descriptor and stream extents, including final physical $3664
  terminator; first six loop-tail FF bytes are not claimed reached. Original
  song/instrument names and natural selection of every descriptor remain unresolved.
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

## Resolved historical setup metadata

Round002 pass-002-refined corrected physical $0556..$0557 in current map and state
to SetupChoiceDestinations. Consumer LDA $8546,X, TAY, STA $0000,Y establishes
the bytes $44/$6A as zero-page destinations twoPlayerMode/teeChoice. Current source
already used SetupChoiceDestinationBytes and retains that valid name. The freshly
prepared commentary baseline carries the corrected annotation. The former coordinate
claim remains only in preserved history at analysis/round-001/commentary; no current
metadata correction remains pending. No Data boundary or code bytes changed.

Round003 parameter refinement resolves the former array-extent question for accepted streams: three 16-phase vibrato banks and envelope IDs 1/2, each 16 phases. CPU $B291 is a biased base; ID0 bypasses. Unrestricted selectors and natural selection remain unresolved. See analysis/disassembly/round003/review.md.
