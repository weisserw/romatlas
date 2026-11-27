# Current-source interpretation limits

These are explained source questions, not pending readability or permission to
classify unknown bytes. Physical offsets include the16-byte iNES header.

- Projectile allocation at physical01B916..01B935 scans seven16-byte records0200..0260;
  InitializeAllocatedProjectile at01B935 supplies position, motion, sprite and
  lifetime. Ordinary update/hit traversal still scans six. A distinct seventh-slot
  purpose, natural simultaneous occupancy and a game bug are not established.
- Main object dispatch words01D1DC..01D218 contain30 slots, with16..19 zero.
  Masked selectors30/31 reach helper opcodes, not two more proved table words.
  Secondary01D660..01D680 has16 words; spawn01EBC0..01EC20 has48. Natural global
  selector domains remain unresolved. Audio commands0004E7..0004F7 have eight
  words and zero slots4/5; names do not turn these slots into executable handlers.
- Palette01C22B..01C2AB is four32-byte presets. The former code interpretation is
  superseded. Prior PaletteBytesDecodedAsCode review is invalidated, preserved in
  analysis/round-002/commentary-invalidated.json and commentary-prior. No remaining
  palette code/data question is asserted.
- Ending/progression: the artificial final-presentation visit is now verified in
  analysis/redirect/r002-contracts/ending-global-contract.json and selected frame30.
  It uses the source-established active global state81, preserves player active/sign
  state and satisfies B047 object readiness before B08B. The completion message and
  normal presentation reset were observed. Natural boss defeat, staff credits and
  the alternative special-status presentation were not established. No further
  structural or readability gap follows from those limits.
- AddPackedDecimalScoreAmount (01E868) is a six-digit decimal score adder. Its
  later life-threshold intent is unresolved: the source retains its CPY07 condition
  rather than inventing a regular reward schedule. Exact arithmetic and conditional
  behavior are described, without claiming a game defect.
- Standalone PPU fill019B14, cadence fill01C728 and controller selector01CCCE have
  static mechanical contracts; their natural callers remain unresolved. Banked
  reads always depend on the supplied effective execution context.
- Audio, sprite, level, text and spawn blocks retain generic classification where
  individual record boundaries or game-facing roles remain unresolved. Nearby
  consumer/format notes document the established role without claiming exhaustive
  schemas. Shared scratch and later object fields have subsystem-specific overlays.
- The former28 residual ranges (397 bytes) are now independently classified:
  254 Static Code and143 Static Data in round003. New standalone palette upload,
  OAM hide, multiply, contact-object and type14-wait routines have explicit calling
  preconditions. Natural callers/selectors remain unknown; this limits reachability
  claims, not their established structural role or source readability.
- Tiny generic parameter blocks retain no invented consumer/schema. Counted readers
  can overlap code: the C742 text-source prefix and E6F2 hidden-sprite bytes do not
  imply exclusive full-size data objects. Exact accepted slices are documented in source.
