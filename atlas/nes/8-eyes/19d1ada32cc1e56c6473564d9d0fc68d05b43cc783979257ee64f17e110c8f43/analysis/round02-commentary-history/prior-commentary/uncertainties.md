# Commentary uncertainties and redirect questions

Current classified-source readability is complete. The 34,189 structurally unknown
bytes remain unknown; the explanatory notes on their raw inclusions do not classify
them. Numeric scratch operands remain where multiple unrelated uses prevent one
stable semantic name. Names recover mechanics rather than original author intent.

## Call and dispatch contracts — productive structural continuation

The source now distinguishes ordinary returning helpers, inline data and tail dispatch:

- `RequestInterruptMode` ($F84D, physical 129117) writes mode and waits for two
  NMI decrements, then RTS. `WaitForFrame` ($F858, physical 129128) waits one.
  Its interior `$F85A` writes caller A into the countdown; `$3C` waits 60 decrements.
  No special stack rewriting occurs in either helper.
- `ReadInlinePointerAndSkip` ($F409, physical 128025) reads two pointer bytes
  after its caller's caller JSR and advances that saved return by two.
  `TransferInlineVramCommands` ($F3BA) calls it; `$F3BD` accepts a supplied pointer.
  Some two-byte raw gaps after these calls therefore have concrete format evidence.
- `DispatchInlineJumpTable` ($F7CA, physical 128986) consumes the immediate
  JSR return, reads an A-selected inline word and tail-jumps. Dispatch target
  domains/table bounds still need independent call-site constraints. A target RTS
  returns to the caller of the dispatching routine, not to its inline words.
- NMI temporarily maps audio banks 8/9, then restores cached PRG selections.
  Mapper writes in helpers still require explicit effective-bank reasoning.

R01-CALLS should inspect these contracts at each conservative discovery stop and
collect one coherent continuation batch, including inline pointer records where
supported. Do not classify an arbitrary two-byte gap merely by visual resemblance.
Use current CPU views and static return mechanics; natural execution is separate.

## Ending — narrower remaining prerequisite

`ValidateJewelOrder` compares all eight jewel bytes. Success reaches `$D582`,
then waits 60 NMI decrements. Unknown physical 120215..120271 contains its
continuation; unknown physical 120531..120539 contains `$D6C3` scene prefix.
`FlashSceneScroll` starts at `$D6CB` and toggles scroll bits probabilistically while
waiting frames. These code fragments do not themselves prove credits/ending.

R01-ENDING: inspect the success continuation and scene setup, then trace the
60-count wait to its actual return with a bounded operation budget rather than
assuming 60 rendered frames implies the main loop resumed. NMI split display
polls sprite-zero clear/set (`NmiApplySplitScroll`, $FD06, physical 130326).
Check current PC, handshake, mode flags, stack and valid sprite-zero scene before
another artificial scene-entry probe. Preserve the earlier unsuccessful snapshots.
No specific cause of the previous stalled altar continuation is established here.

## Boss and room initialization

`DeriveRoomMetadata` combines country and room, `InitializeRoomSpecialState`
chooses room controller identity, and `SelectObjectBehavior` resolves 32-byte
behavior descriptors. `RoomIndex=7` is not by itself a complete boss encounter.
R01-ROOMS should inspect actual initialized object slots, health, controller kind,
player/door state and visibility, then try a supported room transition. The
source distinguishes ordinary object scans from controller dispatch, and documents
artificial-state limitations. Enemy species and many descriptor-selected state
machines remain unnamed where their mechanics extend into unknown code.

## Data and graphics

Low banks retain generic Data: grid/array/record evidence supports non-code, but
individual formats and consumers remain unresolved. Higher classified Data spans
are independently confirmed uniform $FF runs; no original fill purpose is inferred.
CHR is planar pattern storage; exact tile identity/grouping is not recovered.

R01-DATA can use bank-aware consumers documented in source: parallel metasprite
frame tables at mapped $8000..8700; 32-byte descriptors based at $9800; four-byte
room object records; collision-map copies of $1C0 bytes; palette records of 32
bytes; metatile definitions with four tile bytes and separate attribute fields.
These consumer formats constrain specific unknown intervals but do not establish
all table extents. Use exact physical views and retain generic Data where adequate.
Audio period/envelope directories and music/effect streams likewise remain largely
unknown despite classified consumers. Unknown meaning does not block generic data
classification when non-code and boundaries are independently supported.

Current source anchors for bounded redirect inspection:

- [RequestInterruptMode](source/game.asm:8710)
- [WaitForNmiDecrements](source/game.asm:8728)
- [ReadInlinePointerAndSkip](source/game.asm:8012)
- [DispatchInlineJumpTable](source/game.asm:8623)
- [ValidateJewelOrder](source/game.asm:5064)
- [NmiApplySplitScroll](source/game.asm:9347)
- [InitializeRoomSpecialState](source/game.asm:7001)
- [RenderObjectMetasprite](source/game.asm:7782)
- [ResolveObjectDescriptor](source/game.asm:6620)
- [LoadObjectRecord](source/game.asm:7325)
- [CopyCollisionMapToRam](source/game.asm:8243)
- [DrawMetatileAndAttributes](source/game.asm:8401)
