# Commentary progress

Round002 current-source readability complete. Input is `maps/pass-002-disassembly-final.json` / `analysis/state.json`, frozen in `baseline.json` and `phase1-state.json`; mapSHA25695cea7452f7fb2954cb789d1339f11aa3bb65356381cb2f86311f97c4be4e410. The entire previous commentary remains at `analysis/round002/commentary-before/`.

Reused sound unchanged source/review for63,432 classified bytes. Read and explained the new67,654bytes (17,159Code/50,495Data), including all actor/spawn, terrain, audio, sprite/animation, player/reload/dialogue/progression/ending code and structures. Added346 routine/control explanations, context-aware routine/table/branch aliases and named RAM/SoA fields. Revised obsolete unknown notes in carried source and documentation. The current1302-entry scaffold is intact;1537 reviewed units describe current routines/structures and their fragments. `analysis/round002/commentary-scope.json` records scope and limitations. No remaining readability ranges.

Source verification is byte-identical and the reading copy is refreshed; exact hashes are in `verification.json` and `analysis/round002/commentary-output-checkpoint.json`. Source/asset/map/state/ROM data remain local. The source remains independently maintained; editorial scripts operate from the preserved round002 merged seed and never alter map classification.

Unresolved meanings and exact two-byte structural ambiguity are in `uncertainties.md`. No new independent residual method emerged; existing R002-AUDIO-ISOLATED stays unresolved. No new toolkit issues found; existing NES-001 documentation issue remains in `analysis/toolkit-issues.md`, with its bounded workaround. No toolkit changes.

Final editorial/archive acceptance audit remains pending; it was not run here. Readability completion and byte-identical rebuilding do not declare full ROM classification or natural gameplay coverage.
