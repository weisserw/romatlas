# Round 002 redirect handoff

Four ready leads propose 14,996 unique unknown bytes: 2,506 Code and 12,490 Data. Nothing has been accepted or annotated. Current classification remains 115,977 bytes, with 15,111 bytes unknown in 77 fragments. Independent acceptance of the whole batch would leave 115 bytes in 12 fragments.

- `R2-scene-bank8`: 5,079 bytes. Bounded configuration/pose/palette consumers, 49 scene pointers and sprite records, local sound backing, coherent dormant helper fragments. Preserve the lone truncated byte at $8600 and the inline palette at $864A.
- `R2-object-graphs-and-lookups`: 5,952 bytes. RAM record contracts, 47 handlers, finite pose/action dispatch, explicit caller-frame aborts, connected continuations, 60 multipart and four auxiliary sprite pointers/records, and bounded or generic lookup backing. CC29 toggles field $0C; it does not initialize it.
- `R2-fixed-sound-patterns`: 3,842 bytes. Bounded sound pointer/command graph and independently inspected repeated binary backing. Generic Data does not assert a fill purpose or original table schema.
- `R2-small-boundaries`: 123 bytes. Bank10 delay/palette preludes and palette backing, a fixed controller helper, and bounded zero runs.

Review `proposal-batch.json` against `verification-batch.json`, `findings.md`, primary proof/parse artifacts and the concrete recipes in `analysis/leads.json`. All 1,237 candidate instruction contexts were freshly decoded; no boundary, overlap, control-into-Data or proposal conflicts were found. This verifies candidate accounting and encoding, not final structural acceptance. Provenance is Static; natural reachability and saved artificial execution remain separate.

Current checkpoint: `pass-002-complete`. Map/state SHA256: `3924f32566ee7e567a902af723f59d57721a7bbd465169eb5dfaf43448bcb7ca`. Commentary source SHA256: `317db909e3cc51dcff3cf40dd7e53a01e9af80159c08c984be4cc53418c9fcf8`. ROM SHA256 unchanged. No classification, source, ROM or toolkit changes.

`unknown-dispositions.json` covers every current unknown fragment, with neighbors, bank context and evidence-linked proposals/residuals. `residual-dispositions.json` records the narrow remaining ambiguities and actual obstacles. The phase investigated connected and independent useful avenues before packaging; this is a ready batch, not terminal exhaustion. No live process or interrupted productive test remains. Broad progression reuses saved stage3/7 actions and bank8 animation; the final scene idle loop has no input-driven progression consumer, so unchanged held input was not replayed. Complete credits and natural completion remain unconfirmed.

Next: independently review/classify the combined batch, verify a new checkpoint, complete whole current-source commentary, then redirect. Final editorial audit remains pending. Existing NES-001/NES-002 findings are unchanged; no new toolkit issues or blockers. No cleanup, deletion, archive or publication was performed.
