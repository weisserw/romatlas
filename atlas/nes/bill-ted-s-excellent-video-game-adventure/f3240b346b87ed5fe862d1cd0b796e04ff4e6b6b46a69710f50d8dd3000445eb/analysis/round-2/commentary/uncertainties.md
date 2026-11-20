# Round 1 source uncertainties

The source explains all currently classified mechanics and preserves all 51780 unknown bytes. Classification, current-source readability and final editorial acceptance are separate. Effective 8-KiB PRG context remains in review.json; upper-bank CPU aliases are not global ROM identities. Unresolved call targets remain numeric, with local notes, rather than becoming speculative code labels.

Resource bodies at physical $08010–$13FEF and $14010–$179DF have supported non-code roles, pointer headers, tile/map/palette/mask or metasprite-like records. Their internal object extents and complete game meanings remain unresolved. Comments document the bounded prior consumer/pattern inspection. The renderer prefix ends before actual code at $179E0; no full-bank Data shortcut has been restored.

The text interpreter is nibble-addressed, caches packed bytes, uses nested stream state and switches mapper selectors six/seven at a special $7FC0 packed-domain boundary. Source describes that encoding; it does not classify bank-six/seven payload bytes. A practical redirect can inspect finite stream consumers and mixed audio/data boundaries using these named entries. Array nesting has no visible overflow guard; supported caller depth remains an assumption.

Scene parameter pairs at RAM $06E2–$06E5, some interaction tags, actor state bytes and course-state game meanings remain tentative. Their exact state changes, bounds and pointer mechanics are explained. Course mode overlays ordinary rendering/text zero-page descriptors; separate Course* symbols now express its contract. Audio RAM $0374/$0375 has recovered channel/transpose roles; using artificial RAM edits during startup does not establish natural game progression.

The text-yield routine temporarily replaces both NMI continuations and removes the interrupt return stack entries before resuming the suspended decoder. Its named continuations describe the observed stack mechanism; transferring it to another runtime requires preserving that contract. Sprite-zero polling/raster delays assume the configured NTSC display state, and are not a full timing/hardware compatibility claim.

## Correction to the inherited layout-consumer domain

The disassembly handoff/data evidence described a nine-cell lookup consumed at Y=8..0. The actual classified loop at CPU $B6DE (physical $036EE) loads at Y=8, then DEY/BNE repeats through Y=1 and exits at zero. Thus its demonstrated consumption is exactly the eight Data suffix bytes at physical $036C6–$036CD. Lookup index zero would overlap the prior BNE displacement at physical $036C5, but this loop does not consume it. The source now states this narrower domain; the prior handoff/evidence remains preserved as history. No classification, map or state change follows. Supporting source: InitializePhoneLayoutParameters and PhoneLayoutParameters in source/game.asm; original inline review analysis/round1/inline-branches.json and suspicious-inline.json.

Redirect questions remain the disassembly handoff's text/resource bounds, unknown descriptor/interaction handlers, renderer gaps and broader course/world routes. These are structural/gameplay work for another phase, not skipped commentary. Source caller comments explicitly distinguish unknown helper bytes from the established caller behavior.
