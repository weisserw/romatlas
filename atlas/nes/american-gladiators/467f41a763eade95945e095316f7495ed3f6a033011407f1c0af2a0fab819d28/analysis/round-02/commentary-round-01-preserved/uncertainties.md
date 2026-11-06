# Current-source uncertainties and redirect contracts

All classified source has mechanical explanations; no classified range is deferred
for lack of a game-specific meaning. Unclassified incbins have explicit physical
boundary notes and remain outside code/data claims. Counts remain 54.94% classified.

- Event-local $07D0..$07FF state overlaps across banks. The source uses address-qualified
  aliases unless a stable consumer role is established. Exact scoring units of the two
  displayed player counters, many actor signal bits and some direction/phase variants
  remain unresolved. Shared actor phase/direction names describe consumer mechanics.
- PRG bank 0 selection/completion, bank 2/3 event actors and bank 4 motion/collision code
  are fragmented by unclassified continuations. A commented return/branch does not
  assert that its entire routine or all target blocks have been classified.
- Inline consumers explain several gaps: three-byte callback/bank records at $F0F1;
  two-byte pointers via $F199 at animation, sprite, hook and rectangle entry points;
  zero-terminated strings with adjusted return at $EEC5/$EEEB; negative-terminated
  inline palette commands at $E784. These contracts support redirect inspection, but
  names/comments do not themselves annotate those currently unknown physical bytes.
- Audio command dispatch at physical $00CBB7 (CPU $8BA7; indirect JMP at $8BC4),
  callbacks at $00CB5F/$00CB6C and envelope dispatch at $00CDCE require the bank-six
  consumer contracts, not bare CPU pointer values. Directory extent and unvisited
  handler semantics are unresolved. Attack/release pointers, phase updates and hardware
  ownership are explained even where musical names are unknown.
- The DMC descriptor reader at physical $00D8E1 (CPU $98D1) reads four-byte start/end
  records at CPU $C000+4*sampleIndex. The effective $C000 physical bank must be supplied
  before classifying descriptor bytes; do not assume the sound PRG selector identifies it.
- Completion code at physical $000B46..$000B96 requires normal actor/scene state.
  Level four launches event five via $89F2, while level five reaches $8B89. Prior direct
  register entry from menu froze; it rejects that incomplete recipe only. Trace natural
  completion and inspect dependent hooks/banks/actors before another shortcut attempt.
- IRQ vector points at CPU $E46E, immediately before the classified reset handler.
  That target byte is still unknown; no IRQ body is inferred by commentary.
- Uniform non-code intervals keep their generic data role after prior boundary/pattern
  inspection. Specific fill purpose is unresolved. CHR tile allocation is explained,
  while scene/art identities and fine tile-group boundaries are not established.

Bounded next checks and precise consumer entry ranges are in
[redirect-contracts.md](../analysis/round-01-commentary/redirect-contracts.md).
