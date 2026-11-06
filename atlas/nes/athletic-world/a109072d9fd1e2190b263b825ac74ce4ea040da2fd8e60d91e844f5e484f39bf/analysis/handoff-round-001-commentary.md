# Round 1 commentary handoff

Full current-source readability complete; ROM incomplete. No known remaining naming/comment gaps. Final editorial/archive acceptance remains pending; no audit or archive was run.

Input verified checkpoint: `pass-001-final` (`maps/pass-001-final.json`, `generated/pass-001-final`, `analysis/pass-001-final-verification.json`). Output readable source: `commentary/source/game.asm` plus `symbols.inc` and existing local build/assets; reading copy `commentary/reading-after.asm`. Fresh `commentary/verification.json`: ok=true, byte_identical=true. Full scope assessment: `analysis/commentary-round-001/readability-assessment.md` and `scope-assessment.json`; review ledger `commentary/review.json`.

Scope: all 62,938 classified bytes (14,164 code; 48,774 data/image), all 122 scaffold entries and 307 routine/data/unknown units. 249 routine/entry contracts, 823 code symbols, 383 RAM/hardware/resource symbols. Every one of 18 unknown intervals retains exact source boundary/uncertainty notes; 2,614 bytes remain unknown. Naming/format work spans controller/mat, menus/profile/results/certificate, course/block/strip streaming, player movement/collision/continuations, objects, audio bytecode/note/effect engine, scheduler/reset and NMI/PPU/OAM. Existing bounded data and runtime evidence was reused. No new emulator execution or structural annotation was added.

Source consistency corrections include certificate/title entries at $8681/$872F, six-slot task removal, zero/nonzero audio conditional branches, mechanically neutral lane names and explicit RAM copy-base aliases. Shared scratch/task-relative storage, fixed PRG banks [0,1,2,3], eight-byte collision records and local-only asset identities are explicit. Generic corpora retain qualified internal schemas, not invented complete formats.

Source SHA256:
- `game.asm`: 2b4116a6fcc0ea09d56483de69b5dfe886bad421854d132e4f4aea54c56dd39e
- `symbols.inc`: 279878d10868cc13bc5e33b7bdfc224b35f7c167e61d85cb1ea28bf8785f7027
- `reading-after.asm`: 137f529a3eac38bb06a650d4b3632474569141cb5e7738302c03363d8ab1aa65

Map/current state/frozen commentary baseline and state remain SHA256 2f4dfde1c132c80d671777408ceb49043c4c25897ec35bb4244eb0e747d42e88. ROM unchanged SHA256 a109072d9fd1e2190b263b825ac74ce4ea040da2fd8e60d91e844f5e484f39bf. Prior checkpoints, source and evidence retained; commentary/before and reading-before remain frozen.

Next: redirect should investigate R1-MOVEMENT-RETURNS and R1-OBJECT-HANDLERS from the prior compact disassembly handoff/leads.json, using the readable producer/consumer contracts and connected unknown slices. `InstallPlayerContinuation` $AB5B and `DispatchPlayerState` $9C13 establish target-minus-one stack mechanics, not every unknown continuation body. Object type zero has a supported entry; seven adjacent words remain Data and the other handler contracts need review. Combine independent static constraints under evidence-policy.md; do not impose natural execution or a live selector as a universal prerequisite. Do not repeat completed startup/events, unchanged failed terminal shortcuts or failed uninitialized-CHR calls. Additional semantic limits and suggested checks are in `commentary/uncertainties.md`.

Toolkit report: one missing feature NES-001 (documented floor mat input scope), zero confirmed bugs, zero suspected bugs, no blocker; `analysis/toolkit-issues.md`. Intermediate assembly/scripting edit errors were corrected and final verification passes. No toolkit changes, ROM edits, subagents, commits, deletions, archive, publishing or shared assets. No live experiment/server process was started in this phase.
