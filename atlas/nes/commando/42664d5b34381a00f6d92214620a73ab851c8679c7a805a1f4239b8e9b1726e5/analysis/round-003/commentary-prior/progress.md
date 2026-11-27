# Round002 current-source commentary

Current-source readability complete for the round002 checkpoint. The entire prior
commentary remains in analysis/round-002/commentary-prior. Fresh preparation uses
map/state SHA2560572498f2f2c0849b7d65ca3ac0e45d211b1544b91aeaa29c2ab796ff9ce9ee5.
Physical-range carry retained198 source blocks,281 prior meaningful-unit reviews
and useful prose. The single superseded PaletteBytesDecodedAsCode unit is preserved
in commentary-invalidated.json; palette data and affected consumers are corrected.

Reviewed all9381 code records,105 data records, seven image banks and28 unknown
boundaries in the9521-entry full-ROM scaffold. This includes all6096 newly accepted
physical instructions, restored changed-block instructions, all new data, the41-byte
palette retype and stale prior descriptions of newly classified continuations.
Added372 functional code anchors and35 data units;444 flow labels now describe their
routine/loop/continuation roles. Shared record fields, audio/inline-clear/division
scratch, score/menu/mapper state and supported table CPU aliases are named. Table
labels retain physical identity while CPU operand constants preserve execution views.

Source explains audio parsing, startup/menu, inline RAM clears, player/environment
transitions, area/final-mission presentations, projectile/thrown-object behavior,
actor and pickup dispatch, persistent records, decimal score arithmetic, spawn
formats and terrain/PPU transfers. Three thousand redundant emission wrappers were
collapsed where physical/CPU/context continuity allowed. Existing sound prose and
labels remain. Unknown purpose is stated near established mechanics.

Fresh configured-binary verification passed byte_identical=true after final source
edits; reading-after.asm refreshed. Exact hashes are in verification.json. Input ROM,
state and portable current map are unchanged. No current-source gaps remain pending;
analysis/round-002/commentary-pending.json is empty. Final editorial acceptance and
archive audit remain pending by workflow; no final audit was run.

Uncertainties are recorded in uncertainties.md: seven-versus-six projectile slots,
sparse selector domains, standalone natural callers, exact data schemas, score
threshold intent, ending transition prerequisites and397 residual bytes. No runtime
or redirect research was performed in this phase. Existing NES-001 remains
nonblocking; no new toolkit issues or toolkit modifications.
