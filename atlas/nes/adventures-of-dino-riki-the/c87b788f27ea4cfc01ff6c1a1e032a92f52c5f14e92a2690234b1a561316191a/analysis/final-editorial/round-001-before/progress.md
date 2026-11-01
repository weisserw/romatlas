# Commentary progress

Prepared verified baseline. All semantic review entries are pending.

### Batch 1 — startup, background, player, map objects and enemy behavior
Reviewed physical $0010-$1FCA (CPU $8000-$9FBA), with notes through the
player-follower handler at CPU $9EDE. Added 126 functional routine/control names,
217 RAM/hardware symbols and consistent operand substitutions. Source now explains
the NMI update queue, metatile palette merging, stage initialization/progression,
player movement/jump/shot logic, collision-map coordinates, map animation stamps,
enemy slot dispatch and movement contracts. Shared enemy fields retain mechanical
names because individual kinds reuse them. Unknown species and exact pickup names
remain tentative; source explains established effects instead.
Fresh commentary verify passed byte-identical after this coherent batch.
Remaining review: CPU $9FC2-$BCB0 plus all data/header/CHR boundary descriptions,
branch naming refinement, review inventory and final verification/reading copy.

### Batch 2 — remaining enemy families, bosses, HUD, presentation and audio
Reviewed the remaining code through CPU $BCCB. Added mechanical names and purpose
comments for follower/aimed motion, metasprite composition, boss phase/timer logic,
score carry/extra lives, title/continue command streams, CNROM bank selection,
sound priority/forty-four-entry dispatch and all sound effect programs. Music notes
explain duration/note tokens, envelope ownership, repeat/substream contracts and
pitch conversion. Sound effects use audible/register mechanics instead of guessed
game-event names. Fresh commentary verify again passed byte-identical.
Remaining: semantic cross-check/refinement of first-batch entry boundaries, named
data aliases and boundary notes, descriptive local branches, audio RAM names and
review inventory. The scaffold remains pending until this review is consolidated.

### Batch 3 — full data inventory and naming consolidation
All code has now been read through CPU $BCCB, and all 50 classified non-code
regions have source-level format/consumer/boundary explanations. Data aliases name
palette/stage tables, pointer directories, motion waves, frame layouts, APU presets
and music/map structures. Added queue field symbols ($0601-$0607), audio channel
state and APU register names. Replaced generated Block labels with local action or
state names, retaining address suffixes only for disambiguation. Cross-check
corrected early entry-note placements, including the actual NMI vector $8093,
IRQ RTI $808F, new-game entry $84CC and the death path $85EB.
Fresh consolidated-source verification passed byte-identical. Remaining before
handoff: review scaffold/unit consolidation, explicit uncertainty record and final
source checks/byte-identical verification/refreshed reading copy. Audit remains
pending for final editorial acceptance.

### Round 001 commentary complete — current source readable; ROM incomplete
The batch-3 shorthand count of 50 non-code regions was inaccurate: the complete
inventory is 96 code regions, 53 data regions, one CHR image region and one unknown
region (151 scaffold entries). All are reviewed for source readability; 350 compact
units retain physical ranges, source locations and execution context. There are
295 functional notes, named table views, RAM/field symbols and local control labels.
Reviewed all 11837 classified code bytes and 53714 classified non-code bytes; no
readability ranges remain deferred. Data payloads retain honest partial semantics.
The single unknown byte has a local boundary/uncertainty explanation.

Final refinement replaced remaining inferable instruction-derived local names with
functional branches (movement bounds, collision extents, decimal carry, audio
commands), and documented the music command-directory token limitation. Significant
uncertainties are in uncertainties.md. Source is authoritative; drafting helpers
under analysis/round-001/commentary are retained investigation records, not a
regeneration workflow for independently maintained source.

Fresh final `commentary.py verify` passed byte-identical and `commentary.py read`
refreshed reading-after.asm. verification.json records source hashes; game.asm
SHA256 is 997335c0aaded0801ed904e07969d6e7f5ab2530bc7b7f986e245793d544b0cf.
ROM, maps/current.json and analysis/state.json still match the frozen identities.
Final editorial_audit remains pending and was not run. This completes commentary
across current source, not full-ROM classification or final archive acceptance.
No new toolkit issues; the existing NES-DOC-001 confirmed issue remains unblocked.
