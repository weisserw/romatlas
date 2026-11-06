# Round 1 commentary complete for current source

Baseline: maps/pass-001.json, frozen commentary/baseline.json and phase1-state.json. Preserved before/ and reading-before.asm; maintained source/ independently. Maps, classification and input are unchanged.

Readability scope: every current classified physical span, including all 18,680 code bytes, 1,947 data bytes, 63,146 image bytes; the data count includes the 16-byte header. All 982 full-ROM scaffold entries have source locations and a reviewed explanation; 124 physical routine/structure groups describe mechanics, inputs, control flow, table formats and dependencies. All 262 unknown spans have local uncertainty notes; their 47,315 bytes remain unknown. Semantic uncertainty is documented, not treated as skipped commentary.

Naming: physical-aware routine/branch labels; shared RAM/hardware definitions and consumer-established constants in source/symbols.inc. Player motion, fractional accumulators, OAM fields, score/clock digits, sound streams, AI input and PPU ports use consistent names. Mode-specific intermission/entry/selection aliases explain shared RAM reuse. Fixed-width array members use base-plus-field offsets. Exact ca65 address-size overrides and encoded values remain intact.

Examples: reset/frame dispatch and RAM helper bank constraints; four-player update and eight-direction movement; ball ownership/trajectory and metasprite composition; audio envelope/modulation and effect streams; depth sorting and row crowding; ASCII quarter clock/score updates; delimited nametable streams and rectangular uploads; CHR-RAM transfer sizes and computer-generated controller bits.

Consistency refinement corrected adjacent movement-entry names, separated CPU/PPU/physical identities, added local control-flow explanations, and replaced raw input masks and sprite sentinels. A local array-expression syntax error and an alias-induced branch-name mismatch were corrected before the final fresh byte-identical verification. No toolkit bug was involved.

Unresolved meanings: exact special move names, court-object identity, some shared scratch/fields, complete dispatch extents, bank-preserving return contracts, per-tile semantics, and state $17 initialization/natural reachability. See uncertainties.md. No pending current-source readability range remains. Final editorial_audit stays pending; this phase does not establish whole-ROM classification or archive acceptance.

Verification: final fresh commentary.py verify passed byte_identical=true in verification.json; reading-after.asm was refreshed after that verification. No final audit, toolkit edits, archive, publishing or cleanup performed. NES-001 remains the only documented toolkit issue; no new toolkit issues found.
