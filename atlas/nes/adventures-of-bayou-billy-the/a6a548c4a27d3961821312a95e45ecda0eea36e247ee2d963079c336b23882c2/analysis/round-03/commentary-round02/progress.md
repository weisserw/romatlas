# Commentary progress — round 02

Current-source readability is complete for pass-002-disassembly-20261031-a:
39,602 Code bytes (18,218 instructions), 91,479 Data bytes, 131,072 CHR bytes
and honest boundary notes for all seven Unknown bytes in five intervals.
All 8,271 scaffold records are reviewed with refreshed source locations; 677
combined code units and 409 named data/image structures are inventoried.
No current-source readability ranges remain deferred. Final editorial audit is
pending for the coordinator's terminal acceptance after redirect.

The entire prior commentary remains under analysis/round-02/commentary-round01/.
Carried 7,522 exact instruction records and 350 functional prose starts; only
five old C01 Static instructions (13 bytes) were invalidated. Added 328 curated
mechanical explanations for new code and helper subentries, retaining useful
old prose. All 328 new Code regions/23,909 bytes were inspected using the compact
listing, with primary consumer/format evidence reused for data. Source now has
2,829 named instruction targets, including descriptive meaningful unit starts.
Generic data retains concise inspected-boundary/uncertainty notes.

Specific improvements include bank10's five variant arrays; shooting state
$04C1 versus actor type $04D1; audio repeat/ownership paths; brawl direction,
collision and finite subtype controllers; shooting movement rollback, target
allocation and boss/ending transitions; driving fixed-point vectors, object
controllers and explicit 153-byte raster upload; scene/sprite/PPU/script formats;
credits selectors; demo input/duration streams; software DAC playback; and copied
reset-template versus actual post-reset fetch contexts.

Reviewed concrete older defects/dependencies: corrected twenty obsolete fragment/
unclassified claims; bounded generated target ownership with both preserved logical
routine starts and new physical intervals so footer names cannot leak into sprite
code; attached descriptive entry labels even when not referenced by another routine;
renamed 268 precisely identified data containers and supplied separate CPU aliases
for physical data labels; named the overlapping E84B LDA #3 entry without treating
it as proof of a residual stub's reachability. A small arithmetic comment was
corrected to distinguish effect-slot allocation carry success/failure.

All carried source and coherent naming/comment batches were verified. Early local
transformation failures (lost trailing unknown incbins, physical-vs-CPU table aliases,
CPU-alias mismatch in copied footers, duplicate draft labels) were corrected; they
were caller/editor mistakes, not toolkit defects. Final fresh verification in
verification.json is byte-identical with the unchanged input ROM; reading-after.asm
was refreshed after the final source edit. Final source SHA256:
- game.asm: c5d387b296d0e49aa63d650660be5ab8cad3bb0b195e0d3ca866a41aa31134ed
- symbols.inc: 582db20960dcf3c79b8195328b855f5652a88675cd4765be80de2b8682ce300f

Current map/state SHA256 remains
ff89defe97828358320b8a0b360c796abc3e5924cd63b1894c98f36c6fd6c359.
Classification remains INCOMPLETE: 262153/262160 bytes, 100.00% only after rounding.
Unresolved meanings and exact residual boundaries are in uncertainties.md. No new
entry/consumer premise discriminates the seven bytes; redirect must preserve that
uncertainty absent new evidence. No map/state mutation, toolkit edit, archive,
publication, cleanup/deletion or external asset download was performed.
