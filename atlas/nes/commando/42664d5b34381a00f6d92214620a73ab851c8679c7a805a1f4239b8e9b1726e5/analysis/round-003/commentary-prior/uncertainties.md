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
- Ending/progression has a more readable contract: CPU B02A/B08B (physical01B03A/
  01B09B) chooses the area3 completion presentation; B17E (01B18E) checks the final
  mission and B1C8..B1E8 waits/resets progression. The earlier B047 object-readiness
  scan (01B057) and player/environment transitions remain prerequisites. A bounded
  redirect can inspect these records and their producer state before designing a
  shortcut from the saved final-area snapshot. Repeating the failed raw-scroll
  warp alone has no new rationale. No ending/boss visit or natural route is claimed.
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
- All28 residual ranges,397 bytes remain unknown. Exact bounds and existing narrow
  redirect proposal are in analysis/round-002/final-residual.json and leads.json
  R002-RESIDUAL. Newly readable consumers can inform that review; no blanket decode,
  pointer-only code claim, natural execution claim or terminal exhaustion follows.
