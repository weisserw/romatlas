# Commentary progress

Prepared verified baseline. All semantic review entries are pending.

## Round 001 — full current-source readability complete

Reviewed all 804 physical inventory partitions: 605 code, 43 data, one CHR image and 155 explicitly unexplained structural gaps. Preserved the partition scaffold and frozen baseline. All 12,637 classified code bytes now sit under functional routine/control explanations, alongside data format/boundary notes for 62,383 Data bytes and 131,072 CHR bytes. The 56,068 unknown bytes remain unknown.

Added 201 manually described functional review units, bank-aware routine/control labels, shared RAM/object-array/hardware symbols and sound-channel field offsets. Explanations cover title/debug/continue/classroom/ending states, stage/checkpoint initialization, six scene bounds/frame wrappers, camera and VRAM streaming, map collision and object contact, metasprite/OAM generation, three object streams, sound sequencing, MMC1 serial transactions and reset/NMI. Branch names include camera stopping/threshold/death paths, clipping outcomes, map out-of-bounds, metasprite substreams and frame dispatch. Numeric scratch identities remain where callers reuse the storage; unresolved targets retain explicit CPU aliases.

Fresh final `verification.json` confirms byte-identical assembly against the unchanged input. `reading-after.asm` refreshed after the final edits. Machine-readable scope report is `../analysis/round-001-commentary/scope-assessment.json`. No current-source readability ranges remain pending. Final `editorial_audit` intentionally remains pending: this phase does not run archive/final acceptance or establish whole-ROM completion.

Significant semantic uncertainties are in `uncertainties.md`; all unknown/generic ranges also have local source notes. The four existing concrete redirect questions remain actionable. No toolkit issues found; no toolkit changes, cleanup, publishing or archive execution performed.
