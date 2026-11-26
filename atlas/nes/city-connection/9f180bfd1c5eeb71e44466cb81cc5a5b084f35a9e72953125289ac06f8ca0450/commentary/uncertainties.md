# Commentary uncertainties — round 2

Current-source readability covers all 104 physical scaffold entries and all classified source; 65552/65552 bytes classified, zero unknown. Final editorial acceptance remains pending.

D001–D004 and C001/C002 were accepted independently in pass 002. Fresh source explains their code/data contracts and corrected table boundaries. No known outside caller or natural execution is established for the newly accepted fill, title-OAM/score, alternate-pointer or patch helpers. Static contracts suffice to explain mechanics. Attract data has eight supported local duration/button pairs but its consumer lacks a local global-index guard.

The stored patch utility probably targets a writable development image; this is historical inference. Writes to PRG CPU aliases do not modify CNROM PRG backing and may affect CHR bank selection through bus conflicts. The OAM quarter uploader requires the caller to set OAMADDR appropriately.

Car/road/paint, score, lives, oil, sprite composition and sound stream mechanics have descriptive names. Four-sprite hazard identity (probably the cat) and periodic collectible identity (probably a balloon) are tentative; source explains their actual slot, spawn, movement, collision and reward behavior. Special reward art identities are unresolved. Exact original author intent is not claimed.

The sound and compressed-background areas have consumer-derived formats but not exhaustive per-stream extents, original tune names or complete per-stage art labels. Unreferenced bytes, uniform runs, two pointer-looking words at physical [17438,17442), identification trailer fields and some alignment bytes retain honest generic-data explanations. Prior bounded whole-span non-code inspection is reused from analysis/disassembly/pass-001/generic-data-review.json and uniform-runs.json. Unknown purpose does not mean unknown classification.

The active-player 32-byte snapshots contain known stage, lives, oil, travel, completion and trigger fields; unnamed tail fields are copied as opaque saved state. Work bytes $10..$17 are reused as pointers, arithmetic temporaries, sprite state and sound event pointers, with local contracts explained beside routines. These names describe storage, not permanent types.

Title counter progression is resolved statically: RunTitleFrame jumps to RestoreNmiRegisters, which first calls IncrementFrameCounter. With its high byte cleared, the title comparison reaches two in about 512 NMIs depending on the low byte. A natural attract transition was not demonstrated by the timed-out redirect wait. No new runtime was needed here. Busy waits and PPU writes remain mechanical descriptions; no complete hardware/game compatibility claim follows.
