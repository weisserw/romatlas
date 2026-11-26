# Round001 unresolved meanings and next structural questions

Current-source mechanical readability is reviewed; exact game identities of actors,
audio selectors and encoded status fields remain tentative. Generic data bodies keep
their prior pattern-based classification without claiming original table boundaries.
The 162 unknown intervals are explicitly noted in source and remain unknown.

- `Bank0_WaitForNmi` / `Bank0_NmiServiceBody`: the main-thread semaphore is set at
  $A491 and decremented in NMI at $A626. The saved progression attempts stalled in
  $A493. A bounded snapshot-based check should compare PPUCTRL shadow/hardware write
  sequence, mapper shadow and actual NMI entry/acknowledgment before changing inputs.
  A waiting PC alone does not identify a missing Start input or prove an emulator bug.
- `Bank0_IrqOrBrkDispatcher`, `Bank*_ResolveAxromService`: BRK inspects/removes its
  interrupt frame and enters the banked service convention. The bank tail resolves
  X through $8000 and invokes a RAM trampoline. Static byte/stack analysis can test
  the inline return convention and resolve isolated prefix/tail bytes without requiring
  natural execution of every selector. Mapper values and physical 8 KiB identities
  must stay separate; each current source label belongs to a 32 KiB physical image.
- `Bank0_RunBossSequence` and `Bank2_UpdateBossScriptScheduler`: banked setup,
  transfer count, part state and projectile records explain useful mechanical
  contracts, but artificial PC entry did not initialize active boss behavior. Inspect
  the missing setup continuations and dependencies before repeating a forced visit.
- `Bank0_RunIllustratedEnding`: the rendered ending is confirmed only in the saved
  artificial scenario. Natural selection and any high-score terminal continuation are
  unresolved. Text pointer, row count and queued PPU byte are named separately from
  the same RAM bytes' ordinary sprite-rendering aliases.
- Signed RLE copies in banks 1, 2 and 3 have different exhaustion bookkeeping.
  Bank1 tracks output bytes and includes additional count-zero checks; other copies
  stop through high-byte borrow. Preserve each implementation. Named pointer tables
  identify consumer roles, not recovered table extents or independently classified
  records. Generic data boundaries still come from round001 structural evidence.
- Player/actor/projectile field names reflect array indexing and consumers. Some
  state nibbles, script shorthand, palette flags and encoded status digit carries are
  partly unknown. Source describes the established mechanics at fragment entries;
  unknown gaps interrupt inferred control flow. A named continuation does not prove
  a missing predecessor or an original routine boundary.

The four broad generic data ranges were already inspected bytewise and through their
suspicious decode runs (analysis/evidence/round001/data-full-inspection.json and
data-suspicious-decodes.json). This phase reuses that sound evidence; it does not
reinterpret unsupported byte patterns as named graphics/audio assets. RAM aliases
are explicitly phase-local where scratch storage is reused. No upload origin is
attributed to ROM solely because a service calls executable RAM.
