# Game familiarization

Probable game: AD&D Pool of Radiance, North American NES release; observed SSI/TSR/Pony Canyon 1991 copyright and Character/Order/Environ main menu match the NES manual. Exact revision remains unidentified.

Source: https://www.mocagh.org/ssi/poolnes-manual.pdf (pages 3–4, consulted 2026-10-31). D-pad highlights, A accepts, B exits. Start enters main menu. Order can select the cartridge's ready-made characters (up to five); selected names gain an asterisk. Once a party exists, Begin adventure appears. This supplies a practical natural route past character creation. Source describes city Phlan, exploration, encampment and tactical combat; later milestones require local confirmation. These instructions are hypotheses until tested against this ROM.

Tested: unmodified reset, 284 idle frames (two bounded requests timed out after partial progress), then Start held 2 frames/released and 90 more frames reaches matching main menu. Copyright screen is not evidence that title progression failed; Start worked. Snapshot: round1/title.snapshot.json is copyright screen before Start, despite historical filename.

Manual pages 9–10: holding A speeds messages; a new A press is still needed at observed waiting dialogs. Quick environment option accelerates picture display. Local tour test confirms Rolf portrait/welcome and three prebuilt party members; a prolonged continuous hold leaves the welcome waiting. Repeated edges are the improved recipe. Long text waits can run with tracing disabled; trace re-enabled for distinct states, and untraced instruction execution is not counted as observed evidence.
