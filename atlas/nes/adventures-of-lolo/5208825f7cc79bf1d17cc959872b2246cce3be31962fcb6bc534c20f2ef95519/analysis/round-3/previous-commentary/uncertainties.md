# Current semantic and structural limits — round 2

Current-source readability is complete for 15247 code bytes and 50302 data/image bytes. This does not complete classification: physical 8938..8940 and 17417..17418 remain unknown. Final editorial/archive acceptance is pending. Prior uncertainty history is preserved in analysis/round-2/previous-commentary/uncertainties.md; resolved backlog is not carried forward as an open prerequisite.

- Shared zero page and scene RAM are reused. Routine-local comments take precedence over array aliases. Object handlers use valid initialized slot/state preconditions; their names describe encoded mechanics rather than guessing creature names. Static routine contracts do not establish natural selection of every kind or state.
- Room tables now have 50 independently bounded entries, and all saved records consume their exact lengths while expanding to 15x16 cells. CHR banks 5/6 supply compressed byte records through PPUDATA, alongside graphical backing. This is not an all-CHR-is-artwork claim. The room decoder includes the additional emit at its common RLE exit.
- Positive video rectangles and all inline pointer barriers are reviewed. Width>=height takes row traversal; width<height takes column traversal. Destination bit seven is a generated increment-32 flag, not an input dimension flag. RAM-generated mode-two records retain caller-provided destination and width/height headers; no ROM backing is invented for them.
- Audio descriptors, streams and lookup bounds are now explicit. Individual tunes/effects remain named by selection/stream index where their audible identity has not been established. Five isolated FF separators are noncode without a runtime-stop claim. The increasing word sequence at E459 has uncertain meaning; no historical generator is inferred. Legacy wave pointers address RAM, and $4040/$4080/$4089 writes do not establish expansion-hardware support on this MMC1 cartridge.
- Generic noncode tails BFD2..BFFF and FF09..FFF9 retain uncertainty about original organization. Primary neighbor/reference/pattern inspection is in analysis/disassembly/round002/review.md and analysis/redirect/round001-a/residual-reference-review.txt; neither a fill purpose nor unused status is claimed.
- Generic mapper helpers D9D9/DA23 still require actual arguments to establish effective PRG context. All current classified CPU source uses effective physical 8 KiB banks [0,1,2,3]. NMI menu escape resets the CPU stack and is not an ordinary return. Inline consumers adjust their stacked return by two.
- Password insert/delete helpers shift displayed tiles without shifting the separate decoded-nibble buffer. Comments preserve this exact mechanical behavior rather than assuming synchronized editing. The respawn search's exhausted-list path similarly uses the current list cursor for its final active-flag store; no repair is made.

## Corrected residual premise for the next redirect

Physical 8938..8940 corresponds to CPU A2DA/A2DB, bytes 14 3C. Previous review stated no decoded reference selected them; that premise overlooks indexed addressing. Primary instructions, preserved unchanged in the maintained source:

1. 9F84 stores zero to $074A (ending_embrace_phase) after X=0 at 9F74. 9F9F stores $60 to $074B (ending_embrace_timer).
2. At A1AA, once the protagonist is aligned with the rescued figure, decrement the timer; on zero increment the phase at A1AF.
3. A1B2 loads X from the incremented phase; A1B5 reads $A2D9,X and A1B8 stores the result back to the timer.
4. A1CD..A1D2 repeats while phase<3. Phase one and two appear to read A2DA and A2DB; phase three also reads the already classified A2DC byte before the loop exits.

See AnimateEndingEmbrace and TickEndingEmbracePose in source/game.asm; bounded primary CPU A171..A1D9 and initializers 9F74..9FA2, physical offsets include header. Effective PRG banks are [0,1,2,3]. This is static consumer evidence, not a new observed read or natural reachability claim. Under evidence-policy.md it reopens the exact-base no-reference premise without repeating gameplay. Next redirect should check all phase/timer writers and the increment-before-read loop domain, then assess a localized timer-data role against the competing unofficial-instruction interpretation. Commentary has not changed maps or classified these bytes.

Physical 17417 (CPU C3F9) remains the single $60 byte after a tail jump and before a separately contracted handler. Literal versus isolated/trailing RTS remains unresolved; lack of fallthrough alone proves neither role.
