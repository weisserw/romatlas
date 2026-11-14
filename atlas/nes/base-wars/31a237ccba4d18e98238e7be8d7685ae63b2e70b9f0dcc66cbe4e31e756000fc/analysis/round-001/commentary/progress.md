# Commentary progress — round 001 active

Prepared independent before/source baseline from pass-001; prepare verified byte-identical. Phase-1 state and portable map unchanged. No final audit.

Reviewed all 2,725 classified code bytes in PRG00 (physical $0010..$2010 bank scope), following every emitted instruction and its nearby gaps. Added 93 named behavioral blocks, 53 dedicated sound-state symbols, locally resolved control-flow references and explanations of six logical stream slots, command parsing, envelopes, pitch modulation, APU ownership, descriptor priority, fade/reset and DMC setup. Physical region aliases remain stable. Private scratch roles are scoped to audio code; no unproved interpretation applied to other banks. Unclassified inline tables/handlers remain raw and explicitly uncertain. Source annotations are original mechanical interpretation, not generic generated summaries.

Fresh verify after this coherent batch: byte_identical=true; source game.asm SHA-256 140863ede091ca0f79b73d9838448395a16fbb2471a2aa6d337107dbb010a027. Reading copy will refresh at handoff; none claimed yet.

Current operation: review PRG02 ending animation at physical $49DB..$4C2E, then PRG03 and remaining banks. Remaining readability: all classified code outside PRG00 (14,715 bytes), Data/header/CHR and unknown-boundary notes; final review scaffold credit and meaningful-unit records. No successful handoff until those are complete and fresh verification/reading copy pass. Full-ROM classification remains 61.39%.


Coherent second batch: all classified PRG02/03/04/05 code read instruction by instruction and annotated (2,977 additional code bytes). Ending animation, pennant HUD, roster editing, serialized shop output, price/funds check and ranking comparator have behavioral entry/branch names and uncertainty notes. No successful purchase or clean ending inferred. Review scaffold credit recorded only for code in PRG00/02/03/04/05; other source remains pending. The meaningful-unit records currently use small entry spans; final range consistency is pending.

Current operation: PRG08 at physical $1124F onward. Remaining code banks PRG08/09/10/11/12/13/14/15 (11,738 bytes). PRG01/06/07 contain no classified code. Data/header/CHR and unknown-boundary commentary remain pending. Shared RAM/sprite/hardware symbols, resolved local branch operands and cross-bank names will be harmonized after fixed-bank review. Full source readability is NOT complete.


## Authoritative continuation frontier — interrupted, same phase

PRG14 and PRG15 fully reviewed after the preceding note: 3,662 additional code bytes. Fixed-bank explanations cover generic multiply/decimal/dispatch/banked-call helpers, controller debounce/repeat, VRAM queue and compressed transfers, pause stack unwind, reset/save checks, frame interrupts and MMC3 splits/nesting. Shared hardware/state/sprite symbols added, including pause_flag; control operands resolve to semantic names only when all accepted contexts identify one physical target. Mapper register aliases apply to writes; CPU ROM reads/calls at the same addresses retain ROM identity. No toolkit source edited.

Authoritative totals: 9,364 of 17,440 classified Code bytes reviewed; 415 meaningful behavioral units. All current Data/Image/header/vector and unknown raw boundaries have source notes and scaffold review credit. Unknowns remain unknown. Review scaffold preserved. PRG00/02/03/04/05/14/15 code reviewed. Remaining code: PRG08 2,255 bytes; PRG09 3,140; PRG10 370; PRG11 25; PRG12 1,883; PRG13 403. Exact individual pending intervals: readability-remaining.json (all and only 8,076 pending Code bytes).

Resume at physical $1124F (PRG08 CPU $923F), original compact reading copy at the exact offset line. First unread main fragment starts $1125A (CPU $924A). Read full banks 08/09/10/11/12/13; do not merely propagate generic summaries. Baseline compact instruction view is analysis/commentary-001-tools/baseline-compact.txt. Editing helpers and exact earlier notes preserved in analysis/commentary-001-tools; annotation/harmonize scripts are NOT idempotent and must not be rerun wholesale. New notes can use add_notes.add. After new notes, resolve bank-aware control references using fix_mapper_aliases.py; update save_review.py reviewed-bank set only for genuinely completed banks. Its remaining JSON status is interrupted until full readability is complete.

Latest verification after shared-symbol/mapper-alias consistency fix: byte_identical=true; source SHA-256 83f97828b204d8ef26630da4fef3404a3bbe5c36b83d53e58e051094c38c86f0. Reading-after.asm refreshed. Current source is buildable but full current-source readability remains INCOMPLETE. This is a context interruption, not a successful commentary handoff and not ready for redirect. No audit run. No new toolkit issue found in commentary; existing NES-001 remains nonblocking.

## Resumed batch — PRG08/09 reviewed

Reviewed all 5,395 classified code bytes in PRG08/09 and added 135 behavioral blocks. Arc arithmetic, signed acceleration/clamping and fractional integration, field-boundary equations, CPU directional input, camera selection/substeps, metatile/attribute streaming, ball attachment/height, marker animation and proximity transitions are now explained. Existing sound/shared symbols retained; further gameplay field aliases pending after remaining banks.

Authoritative current scope: 14,759/17,440 Code bytes reviewed, 550 behavioral units. Exactly 2,681 pending Code bytes remain in PRG10/11/12/13 (readability-remaining.json). Continuing at compact baseline line 5608 / physical $14C2E. Fresh byte-identical verification true, source SHA-256 28f25fd85a6851e8d47d3efc0fe94b37b0f8b782249efb8a08240c71ccef41e0. Not yet a successful full-source handoff.

Potential structural question for redirect: physical $12130..$12133 is classified Code and also directly indexed by the camera speed-table consumer at CPU $A120. Source preserves the accepted classification and exact ORA/KIL encoding; intended execution is uncertain. Inspect accepted provenance and effective bank contexts before revising any role.


## Full current-source readability completed — authoritative final round 001 status

All remaining PRG10/11/12/13 code reviewed, including sparse/rectangular OAM frames, pitch perspective/motion/input/CPU/contact mechanics, release/power states and title transition. Added 90 behavioral blocks, then harmonized world/trajectory/object-field, scene-local menu/ending and pitch-local symbols. Corrected two prior helper descriptions (sprite-array clear spans image/Y/X; pointer addition returns updated low byte in A). Full known source now has 640 behavioral units spanning meaningful physical blocks; every classified code region falls within one. All 8,800 scaffold entries reviewed, no pending units or remaining readability ranges. Editorial audit deliberately remains pending.

Fresh final verify: byte_identical=true, ROM SHA-256 31a237ccba4d18e98238e7be8d7685ae63b2e70b9f0dcc66cbe4e31e756000fc; source SHA-256 f3508c1090a3cae5ed2ec820827032ed679a7479a87cb212a69c117ff58bf4a7. Reading-after refreshed after final edits. Current map/checkpoint and state/frozen state compare equal. No structural changes. Full-ROM classification remains incomplete at 61.39%.

Readability assessment: analysis/commentary-001-readability.json. Successful full-scope handoff: analysis/handoff-commentary-001.md. No source readability gaps remain; unresolved original meanings/unknown structural bytes are explained locally and in uncertainties.md. New concrete structural question at camera lookup physical $12130..$12134 is recorded in analysis/commentary-001-camera-question.json; consumer and impossible conservative fallthrough support a bounded data-role review in redirect. No redirect/audit performed here. Existing toolkit NES-001 nonblocking; no new toolkit findings.
