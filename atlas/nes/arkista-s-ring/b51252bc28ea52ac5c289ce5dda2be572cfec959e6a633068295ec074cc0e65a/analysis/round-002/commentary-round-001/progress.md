# Round 001 current-source review completed

All current classified source and every unknown boundary reviewed. Nine coherent
source/comment/symbol batches and the final table sweep preserve exact bytes.
Final source SHA256 656fb64587d11fae78f4fd79de3d0c1dd1ceac60349483c80372704bc7960172;
commentary/verification.json reports ok=true, byte_identical=true. reading-after.asm
was refreshed after the final source edits.

Coverage: 12,949 code bytes, 13,191 data bytes, 32,768 CHR image bytes, 6,644 unknown
bytes explained as raw unresolved gaps. Full 6,454-region scaffold retained, with
419 compact functional entries and 544 descriptive mechanical branch joins.
Hardware, RAM, primary/secondary working records, projectile records, reused scratch
pointers and consumer-established tables use consistent symbols in operands.
Comments explain contracts/algorithms and distinguish static structure, observed
execution and unknown game meanings. Forced operand widths and physical .org/.reloc
emission boundaries remain intact. No map/state/classification changes.

Examples: stage metatile/collision/attribute expansion, grouped text tokens, inventory
bit compaction, aligned player collision, projectile sprite clipping, primary versus
secondary entity layouts, audio command dispatch, private-stack substream calls and
envelope/APU mirror updates. Known uncertain selectors/fields are documented beside
consumers and in uncertainties.md. No current-source gap is deferred.

Review records use semantic_units plus unit_id references to avoid duplicating a
large functional entry in each instruction-sized region. Final editorial/archive
acceptance and its audit remain pending; this phase does not declare ROM completion.
See analysis/commentary-round-001-handoff.md for the compact next-phase handoff.
