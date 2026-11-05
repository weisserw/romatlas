# Round 1 commentary complete for current source

Scope: all 7112 classified code bytes (3478 instruction regions), 35199 classified data/image bytes, plus honest notes on all 101 unknown ranges. Preserved frozen before/, baseline.json, phase1-state.json and the full 3674-entry review scaffold. No structural map, state, ROM or toolkit edits.

Reviewed 230 functional entries/control blocks and named 303 branch points (some coincide with entry names). Added 307 RAM/hardware/table aliases in symbols.inc. Functional navigation is in functional-units.json; per-entry inventory credit remains in review.json. Existing palette/header/vector/CHR labels were preserved. Known-format inline pointer and video slices now have mechanical labels. Address-only labels retained on individual instruction islands are storage scaffolding, not unexplained routine names.

Source explains startup/menu, game/room transitions, player movement and grid probes, object dispatch/contact, HUD/metatile/sprite output, PPU packet production/consumption, NMI/controller handling, mapper serial writes and both audio interpreters. Physical offsets include the header; reviewed CPU view has effective PRG banks [0,1,2,3]. Table aliases into unknown bytes assert consumer roles only, not bounds/classification.

Final consistency sweep corrected run decoding (extra common-exit emit), documented column traversal geometry, and acknowledged CHR-backed room records. See uncertainties.md for concrete remaining meanings and L005–L008 dependencies. Readability frontier: none in the current baseline. Structural unknowns: 23241 bytes, unchanged.

Fresh byte-identical verify passed after the complete edit batch; reading-after.asm refreshed. verification.json records source hashes. One intermediate alias edit temporarily replaced five .org literals; it failed assembly, was corrected before handoff, and the final source verifies. Final editorial_audit remains pending; no final audit/archive acceptance was run or claimed.
