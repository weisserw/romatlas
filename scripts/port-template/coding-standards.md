## Write readable native code from the first implementation

Implement behavior in terms of the current game's concepts, not a transcription
of assembly operations with the same unexplained numbers. Understand the relevant
source and consumers, define meaningful types and names, then use them while
writing the logic. Do not postpone naming or comments to a later cleanup pass.

Do not use bare numeric literals for values whose meaning comes from the game
or original hardware. The following examples illustrate categories, not mechanics
or names that every game must have:

| Avoid in executable game logic | Express the meaning instead |
| --- | --- |
| `rom[0xf4db]` or repeated numeric ROM ranges | A named ROM location or asset-loader definition, followed by decoded data |
| `memory[0xc120]` for game state | A named field in a native state structure |
| `flags |= 1 << 14` or `flags & 0x40` | A named flag/mask or typed flag operation |
| `state == 3` or `set_state(0x2a)` | An enum variant, or a named ID when an integer representation is needed |
| `distance < 16` or `position >= 176` | A constant identifying the range or boundary, with units/coordinate space |
| `speed -= 4` or `timer = 30` | A named rate, increment or duration in the appropriate units |

Small decimal integers need meaningful names just as hexadecimal addresses do.
Names must explain the role: `STATE_3`, `VALUE_16` and `DATA_F4DB` only repeat the
encoding and are insufficient. Reuse existing names instead of creating aliases.
Use enums for finite semantic choices and typed interfaces where practical; use
scoped named constants when numeric representation is appropriate. Keep original
encodings and numeric locations in definitions, loaders or serialization code,
not scattered at call sites. Distinguish banked CPU addresses from physical ROM
offsets. Original RAM addresses should become native state fields, not a second
memory map. Preserve exact data encodings and deterministic behavior.

Write useful comments alongside the implementation. Document major modules and
important type/interface contracts. For a condition, calculation or transition
whose purpose is not apparent from its names, explain what its inputs represent,
what rule it implements and what effect it has. Also explain relevant invariants,
update ordering, source quirks and intentional differences. Naming each constant
alone is insufficient if the combined expression still obscures the behavior.
Derive names and explanations from this game's source; distinguish facts from
assumptions. If a meaning is unresolved, use an honest provisional name and state
what is known without inventing intent. Cite source labels or addresses when
useful for traceability. Avoid comments that only paraphrase the Rust statements.

Ordinary literals whose meaning is evident, such as zero initialization, and
opaque ROM table contents need no artificial constants. Executable conditions,
call arguments and game parameters are not opaque data merely because they came
from assembly. No abstraction or comment-count quota applies. These are coding
requirements, not an additional review phase, report or gameplay traversal.
