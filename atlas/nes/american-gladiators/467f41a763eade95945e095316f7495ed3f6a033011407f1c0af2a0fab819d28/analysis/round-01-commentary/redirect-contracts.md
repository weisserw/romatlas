# Consumer contracts established during round 01 commentary

These are source-level interpretations from the verified classified instructions.
They do not change maps/current.json or analysis/state.json. Use the current commentary
source, not an inferred original layout. Physical offsets below include the iNES header.
All low CPU code references require the effective 8 KiB slot context in saved snapshots.
Fixed high code uses physical bank 15 at CPU $E000; physical bank 14 supplies $C000.
The existing observed contexts are authoritative over a generic bank assumption.

| Consumer | CPU / physical start | Established contract | Bounded redirect check |
|---|---|---|---|
| CreateActorFromInlineDescriptor | $F0F1 / $01F101 | Pops return address; consumes three following bytes: callback low, high, mapper PRG selector. Advances return by three, then allocates a twelve-slot actor. | At six selection calls $8999..$89B7 and event calls $946D..$9485, inspect exact three-byte gaps and callback bank contracts. Do not require another natural caller to identify the inline record. |
| DecodeTwoByteInlinePointer | $F199 / $01F1A9 | Advances caller return by two and delivers the two inline bytes through the stack to its consumer. | Check callers of $EE13, $EEAA, $EF5A, $F171 and $F17D; classify only the two supported bytes at each call before following their distinct consumers. |
| InstallInlineAnimation / ReadActorAnimationRecord | $EE13 / $01EE23; $EE3C / $01EE4C | Animation record is delay and metasprite pointer; delay zero redirects through pointer. Optional negative extension byte replaces attributes and adds signed X/Y deltas, extending cursor by three. Cursor normally advances three. | Inspect duration/pointer chaining and exact optional record boundaries, retaining bank identity from actor sprite source. Standalone pointer plausibility is insufficient. |
| InstallInlineMetasprite / DrawNextActorMetasprite | $EEAA / $01EEBA; $ED22 / $01ED32 | Width,height,attributes header; width*height tile/attribute pairs. Bottom-anchored rows, optional actor horizontal flip; source bank selected by actor PRG field. | Establish finite dimensions and complete record bounds from the consumer; compare conflicting code evidence. Header/pairs are data even without identifying the artwork. |
| QueueInlineTextAtXY | $EEC5 / $01EED5 | X/Y tile position; zero-terminated inline tile bytes; adjusted return skips terminator, restores X/Y. | Inspect the icon/prompt call gaps in PRG0 and locate the first zero within the call contract. Do not interpret tile values as text encoding without evidence. |
| QueueInlinePositionedText | $EEEB / $01EEFB | Inline column,row then zero-terminated tile bytes, adjusted return. | Inspect the call at CPU $8A0B; two coordinates and terminator supply a candidate bounded record. |
| QueueInlinePaletteStream | $E784 / $01E794 | First inline byte is destination offset in $3F00 page; subsequent nonnegative color bytes stop at a negative terminator. Saved return is advanced past all consumed bytes. | Inspect gaps after palette calls in bank0/1/3 and event description. Brightness cap is a rendering operation, not a structural requirement for accepting the record. |
| QueueTileRectangleFromInlinePointer | $EF5A / $01EF6A | Two-byte inline pointer to dimensions/cells. Height at offset one, width at zero, data from offset three; anchor is adjusted before row-wise queued tile writes. | Inspect exact two-byte call-site gaps independently of proving every pointed rectangle's extent. |
| DecodeBankedRuns | $E703 / $01E713 | Signed control selects repeated versus literal output; $00/$80 skip. Exact output budget bounds destination; source byte count depends on controls. | Reconstruct a supported stream up to its 1024-byte budget under the graphics bank selector; retain compressed bytes' physical identity. Do not classify whole gaps solely from a decompressor pointer. |
| Sound command dispatch | $8BA7 / $00CBB7, JMP $8BC4 / $00CBD4 | Negative opcode & $7F indexes two-byte handler directory at CPU $9925; next operand fetched before indirect JMP. X is slot stride, Y stream offset. Effective sound lower pair is banks 6/7 for mapper selector three. | Resolve exact directory entries/handler calling contracts in that bank. Consumer-supported static handlers need no new natural selector; preserve unresolved table extent separately. |
| Sound callback/envelope dispatch | JMP $8B4F / $00CB5F; JMP $8B5C / $00CB6C; JMP $8DBE / $00CDCE | Two per-slot callbacks and state-indexed envelope handler use RAM pointers. Source explains default trampolines, pitch slide, attack and release phases. | Trace/inspect writes to callback fields or envelope phase assignments, combine with calling preconditions and handler structure. Compare default addresses initialized by sound start. |
| DMC descriptor reader | $98D1 / $00D8E1 | Four-byte start/end pair at CPU $C000+4*index; start in 64-byte and length in sixteen-byte hardware units. | Confirm effective bank at $C000 using the retained sound/event snapshot before mapping descriptors. Mapper PRG value three identifies lower pair, not this fixed slot. |
| Event completion | $8B36 / $000B46 | Result flag zero adds event bit; $1F mask advances level. Four enters event five via $89F2, five enters $8B89. | Patch completion mask at a supported naturally active completion path, preserving dependent state. Existing direct PC/stack shortcut failures must not be repeated unchanged. |

The ready avenues are connected: inline descriptors expose actor callback roots and
sprite/animation structure; actor continuations expose event setup and supported
completion entry; sound callbacks expose independently supported handler contracts.
No semantic-name recovery is required to classify otherwise clear non-code bytes.
Where exact table layout lacks support, evaluate generic data separately under evidence-policy.md.
