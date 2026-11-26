# Commentary uncertainties

Current source readability covers every classified range and explains all four unknown boundaries. Classification remains 65022/65552, with 530 unknown bytes; final editorial acceptance is pending.

The two code/table overlaps identified through concrete consumers are retained exactly in source and described in redirect-questions.md (C001 pitch timers, C002 first road-pointer byte). Neither an implausible decode nor lack of runtime fetch alone proves a correction; redirect must evaluate exact control-flow and consumer bounds.

D001 [379,421), D002 [1976,2325), D003 [7648,7662), D004 [18032,18157) remain unknown. Referenced data aliases in D001/D002 are consumer descriptions, not structural promotion. The CNROM PRG-write candidate in D004 cannot be interpreted as ordinary writable ROM.

Car/road/paint, score, lives, oil, sprite composition and sound stream mechanics have descriptive names. Four-sprite hazard identity (probably the cat) and periodic collectible identity (probably a balloon) are tentative; source explains their actual slot, spawn, movement, collision and reward behavior. Special reward art identities are unresolved. Exact original author intent is not claimed.

The sound and compressed-background areas have consumer-derived formats but not exhaustive per-stream extents, original tune names or complete per-stage art labels. Unreferenced bytes, uniform runs, two pointer-looking words at physical [17438,17442), identification trailer fields and some alignment bytes retain honest generic-data explanations. Prior bounded whole-span non-code inspection is reused from analysis/disassembly/pass-001/generic-data-review.json and uniform-runs.json. Unknown purpose does not mean unknown classification.

The active-player 32-byte snapshots contain known stage, lives, oil, travel, completion and trigger fields; unnamed tail fields are copied as opaque saved state. Work bytes $10..$17 are reused as pointers, arithmetic temporaries, sprite state and sound event pointers, with local contracts explained beside routines. These names describe storage, not permanent types.

Title timeout counter progression is uncertain, as explained in redirect-questions.md. Busy-wait delays and NMI PPU writes are described mechanically; no cycle-accurate or complete hardware/game compatibility claim follows. Existing natural/artificial runtime evidence remains distinct from static commentary review.
