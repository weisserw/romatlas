# Commentary progress

Round 001 commentary complete for **all current classified source**. No unreviewed classified backlog remains. This is separate from whole-ROM classification and final editorial acceptance.

Frozen checkpoint `pass-001-final`, map/state SHA256 `77d79b7545def6fc5d9738059f783b53336ee32af10757ce39ae0a1e8d84ee59`. Maps, internal state and input ROM unchanged. Classification remains 186573/262160 bytes (71.17%): Code 4899, Image 139264, Data 42410 including header/vectors; Unknown 75587.

Reviewed all 1246 scaffold entries with source locations; inventoried 174 meaningful code entries/control blocks with exact classified slices in review.json and units.json. All code in banks 0/28/30/31 now has coherent mechanical explanations, descriptive routine/branch symbols and established hardware/RAM/parallel object fields. Known consumers name their referenced tables without inventing their unknown bounds. Examples include six-byte menu records, board-side checks and modulo-eight cursor movement, seven audio stream slots, stack-page PPU queue, palette fades, object script dispatch and clipped/mirrored composite sprites. Scratch aliases are contextual. Unknown gaps remain explicitly unresolved raw bytes.

Reviewed raster slices in banks 2..18, mixed raster/nibble bank 19, nibble streams in banks 20..23 and exact uniform-$FF extents. Reused retained raster, boundary/distribution and consumer evidence; no speculative semantic classification. Header and interrupt vector meanings are explained locally. All raw spans have boundary/uncertainty notes, including spans crossing banks.

Verification after coherent edit batches passed. An intermediate upper-audio return/entry rename altered a branch target; byte comparison caught it and the return and callable entry aliases were separated. Final fresh configured-binary verification passes byte identity; commentary/verification.json records current source hashes and unchanged input SHA. reading-after.asm refreshed after final edits. analysis/round-001-commentary-review.json validates full partition, source references and reviewed coverage. Final editorial_audit remains pending; no final audit run.

No deferred current-source readability ranges. Unresolved meanings/structural leads are in uncertainties.md: Setup Done callback/ready contract, six-byte menu base producer, script callback/count domain, PPU resource descriptors and bank-28 audio tables. Hand off these questions to redirect; no terminal exhaustion claimed. Preserve source/assets and frozen baseline locally.
