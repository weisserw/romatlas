# Round 1 current-source readability assessment

All currently classified source was read, including the entire 607-block compact
instruction inventory (6462 decoded starts), all 32 data records and each unknown
interval in the source scaffold. This is a full first commentary pass, not a
selected-routine repair. The source now has 223 functional routine/fragment
explanations and 136 local control/data-flow notes, with actual semantic entry and
branch labels, shared hardware/RAM symbols and consumer-address names. The 762
scaffold records retain their original physical partition; reviewed unknown means
its uncertainty is explained, not that its bytes became classified.

The explanations establish why code runs and how state flows. RunObjectTasks
explains flags, countdowns, saved instruction continuations and callback transfer;
ExtractInlineWord explains the two stack levels and skipped arguments, avoiding
false fallthrough into inline data. Map lookup explains layout ID -> mutable RAM
identity -> metatile pointer -> 5x5 tile, and scrolling explains incoming row/column
buffers and the 32x60 nametable ring. Animation explains the five-byte duration,
dX, dY, image-pointer record, progress reset/retry, facing and cooperative delay.
Music explains stream loops/rests/termination, temporary bank selection and channel
state rather than repeating register stores. Player and inventory prose explains
priority of terrain/input decisions, action continuations, task suspension and
reload on menu close. Encounter fragments describe known mechanics and explicitly
localize unknown middle intervals; no unsupported identity names were added.

Review corrected fourteen inherited stage-table names according to their actual
consumers, including map bank/root offset, map width/height, player initial X/Y,
camera origins and music ID. Table structure/evidence and all maps remain unchanged.
Each unknown raw interval has physical bounds and bank identities; uniform generic
data blocks have concise purpose/boundary uncertainty notes. Reference symbols for
unclassified tables make consumers readable without claiming extent or data status.

The consistency review caught and corrected the NMI gate/callback distinction:
$CAB7 skips reentrant callback work, $CABA dispatches, and $CABD is the default body.
This illustrates why assembled identity alone was insufficient for prose acceptance.
Remaining uncertain enemy names, effect field reuse, score units and asset endpoints
are explicit in commentary/uncertainties.md. No classified-source readability gaps
remain. Final whole-ROM editorial/archive audit remains pending; it was not invoked.

Fresh verification passed after the final source changes, with the exact source
hashes in analysis/round-01-commentary-verification.json. reading-after.asm was
refreshed afterward. Input SHA256, internal state, current map and frozen checkpoint
were compared: input identity unchanged; state equals phase1-state; map equals
round-01-disassembly map; map validation succeeds. Overall ROM classification is
still incomplete at 19055/131088 bytes, with 112033 unknown bytes.
