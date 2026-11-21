# Commentary progress

Prepared verified baseline. All semantic review entries are pending.

## Round 001 full-source readability review

Input pass-001-final, no previous commentary credit. Read all 4844 classified instruction fragments across 214 contiguous physical runs. Source edits now cover all classified code banks 8/9/10/11/13/14/15, generic data, header/vectors, CHR backing and all 223 unknown intervals. 163 mechanical units describe routine/block purpose, control/data contracts and missing continuations; 511 bank-specific entry/branch labels and 508 used hardware/RAM/array/field aliases replace understood numeric roles. Unknown callees/record extents retain uncertainty rather than fictional behavior.

Examples: screen selector and option handlers; indexed paired-glyph text; sound-channel duration/pattern/effect processing; row/column metatile streaming; collision cache and world-map arithmetic; actor structure-of-arrays working copies; OAM metasprite normal/mirrored/single-tile traversal; IRQ raster delays; frame/task scheduler; stage/boss gates and sound/controller dispatch. Input classification is unchanged; body gaps remain explicit binary references.

Initial edited-source verification passed byte-identically, SHA256 3d1356622288e880ecf7bf9155ac84dc3d63798593e25c2cf5ca5a5488ff85c3. Final consistency check, fresh verification and reading-copy refresh remain in progress. Final editorial audit is pending and deliberately not invoked. No new toolkit finding; existing NES-001 remains nonblocking. Reproducible local editorial transform and coverage in analysis/round-001-commentary/.

### Completed current-source handoff

Final naming consistency review corrected menu direction/button names against the established serial bit order and named consumer-established ROM table references without changing their structural status. Final scope: 163 canonical routine units, 511 bank-specific entry/branch labels, 630 used operand symbols, 4844 instructions /10547 code bytes, 214 contiguous runs, all 5082 physical inventory entries (4859 classified regions plus 223 unknown intervals). Source locations and scaffold coverage checked, exact .org/.incbin directives unchanged, frozen baseline hashes unchanged; see analysis/round-001-commentary/consistency.json. Instruction-granularity scaffold entries carry canonical routine anchors; top-level units retain the compact 163-unit index.

Fresh final verification passed byte-identically after the last edit; reading-after.asm refreshed. Saved verification: analysis/round-001-commentary/verification.json. Game source SHA256 de09637f536aca1e1695626f9e752a3565632182fa7b4a529be1a68208d772be; symbols include SHA256 2867f1bb30aa51f30c41cb5391234107f9558565756deb5fa2c03950fe88221e. No classified-source readability backlog or deferred ranges remain. Unknown game meanings and unclassified continuations are documented in source and uncertainties.md; R001–R003 remain open for redirect. Final editorial audit remains pending, and whole-ROM classification remains incomplete at 69.21%. Existing NES-001 is the sole confirmed nonblocking toolkit finding; no new bug/missing feature found.
