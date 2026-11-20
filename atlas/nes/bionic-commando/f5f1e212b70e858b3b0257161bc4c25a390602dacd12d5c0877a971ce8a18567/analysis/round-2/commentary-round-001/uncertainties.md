# Investigated limitations after round 001 commentary

All current classified source has been read and explained. These limits are semantic
or structural frontiers, not omitted readability batches. Unknown spans retain
verbatim inclusions and nearby uncertainty notes; no classification was changed.

- Late cinematic: physical 0x7913, 0x7B33 and 0x7E42 expose dispatch, OAM setup
  and palette-fade mechanics. The actual continuation, inline state tables and
  credits prerequisites cross unknown spans. See source/game.asm:460 and the
  existing late-cinematic snapshots. Redirect should decode the reachable BB33
  body and inspect CE/CF transitions with a short trace from the saved snapshot;
  another blind wait is unlikely to answer the question.
- Room 101 / final cockpit: player, projectile and grapple initialization are
  explained mechanically (source/game.asm:3022 and :3512), but these fragments
  do not establish the final weapon/projectile entry or cockpit collision state.
  Use the saved boss snapshot to inspect selected weapon, active projectile slots,
  type-dependent bounds and transition writes. No natural victory is asserted.
- Resource schemas: screen-cell and tile-structure intervals retain evidence-backed
  resource roles. Exact room assignment, tuple fields, packed-font glyph indexing
  and dialogue-script/metasprite boundaries remain unresolved. Prior block/flank
  review was reused; uniform runs receive no invented padding or table purpose.
- Audio: stream timing, priority masks, 31-byte channel records, note lookup,
  modulation and APU writes are explained in source. Exact original song names,
  instrument intent and commands in unclassified gaps remain unknown. Scratch
  bytes retain mechanical names; this is a static interpretation, not runtime
  confirmation of all sound sequences.
- Fixed-bank and banked calls: known decoded targets have names by physical
  identity. Numeric control targets enter unknown bytes or require an unresolved
  switchable-bank return contract. Source view comments preserve effective 8 KiB
  bank context; MMC1 register values are explicitly distinct.

No unresolved game meaning is being used to claim additional code, complete
reachability, whole-ROM classification, or full hardware compatibility. Editorial
archive acceptance remains pending and separate from this readability pass.
