# Round 1 commentary progress

Full current-source readability review is in its final consistency/verification
step. Source is now bank-organized: `source/game.asm` includes the iNES header,
eight physical 16 KiB banks and `symbols.inc`. The frozen `before/`, baseline
map and phase-1 internal store remain unchanged.

Reviewed draft scope: all 6,034 instruction starts / 11,775 Code bytes and all
111 other classified bytes. There are 154 functional units, 879 routine/path
labels, and shared RAM/hardware/format symbols. Routine comments explain inputs,
outputs, algorithms, stack unwinds, bank restoration, important flag contracts
and unresolved meanings. Small paths retain functional parent names and exact
branch conditions when their game meaning is unknown. No source range is being
skipped for diminishing semantic returns.

Concrete semantic findings include the high-then-low target-minus-one inline RTS
dispatch convention; player slot0/attack slot1/other slots2..15; parallel object
motion/contact/animation arrays; rotating OAM priority; PPU header/payload queues;
CHR transfer tail descriptors; task suspension/cost; held versus changed input;
status spell affordability; deterministic seed arithmetic; and audio per-track
stream/envelope/call/repeat state. These are static interpretations, not new
runtime observations or additional structural classifications.

Edited source has passed ca65/ld65 verification after bank splitting, symbol
application and classified-data formatting: `ok=true`, `byte_identical=true`,
ROM SHA256 `115b50b36d4dfb7beaa032f868f1b6f11c8384764c79428fe821c1e23f5ebdc6`.
The final source hash report and reading-after refresh are still to be recorded
at handoff. Final whole-ROM editorial acceptance remains pending.

L001–L003 are preserved; see `uncertainties.md`. No new gameplay experiments,
artificial edits, toolkit changes or ROM changes. No toolkit issues found;
[report](../analysis/toolkit-issues.md).

## Completed handoff

Final consistency review completed across the entire scope above. All540 scaffold
entries have reviewed notes/source locations; all classified code has reviewed
functional-unit records, with no pending classified ranges. Explicit `.byte` /
`.word` fields replace raw inclusion for all classified data/text. Consumer aliases
for still-Unknown tables remain address-only and say so. Branch/path labels use
functional parents, meaningful decisions/loops and exact conditions where finer
meaning remains unresolved. Final whole-ROM editorial audit stays pending.

Fresh final `commentary.py verify` passed `ok=true`, `byte_identical=true` for all
11 source `.asm`/`.inc` files; `commentary.py read` refreshed the10159-line reading
copy. Reports: [coverage](../analysis/commentary-r1/coverage.json),
[verification](../analysis/commentary-r1/verification.json),
[unit catalog](../analysis/commentary-r1/unit-catalog.json),
[compact handoff](../analysis/commentary-r1/handoff.md). Classification remains
9.07%; no additional execution coverage claimed. No toolkit issues found.

## Round 2 completed current-source commentary

Preserved the complete prior phase at analysis/round-2/commentary-r1, then inserted new source only in exact prior Unknown gaps. Current scope: 10,442 starts / 20,931 Code bytes,57,374 Data bytes,25 Text bytes;303 mechanical units (154 carried,149 new). All745 current-round scaffold entries reviewed; no pending classified readability ranges. Corrected graphics-chain polarity and dependent old dispatch/audio/graphics prose. Added symbolized RAM/branch/format operands, bounded data formats, envelope/descriptor origins and mechanically named handlers. Concrete map/source rename rationale is in uncertainties.md and analysis/commentary-r2/source-name-crosswalk.json.

Fresh final verification: ok=true,byte_identical=true; all131,088 bytes rebuild with configured release toolkit/ca65/ld65. Refreshed reading-after.asm. Exact raw Unknown complement matches52,758 bytes;48,752 classified graphics asset bytes remain local. Full ROM classification remains59.75%; final editorial/archive audit pending. No added execution coverage. [Handoff](../analysis/commentary-r2/handoff.md), [coverage](../analysis/commentary-r2/coverage.json), [verification](../analysis/commentary-r2/verification.json). No toolkit issues found; [report](../analysis/toolkit-issues.md).

## Round3 current-source readability complete

Checkpoint pass-003-reviewed:99342/131088 classified,31746Unknown. All11307 starts/22619Code bytes and all currently classified data/text reviewed;347 code units (303 carried,44 new),801 new data fragments and1625 scaffold entries. All170 raw spans exactly match the Unknown complement. Archived previous complete phase under analysis/round-3/commentary-r2 before incremental merge. Named projection/damage/line/strip/status/PPU mechanisms and typed data grammars; corrected stale extent prose and RAM/button alias misuse. R2 crosswalk preserved, portable maps unchanged. Final verify ok=true,byte_identical=true, reading-after refreshed; source hashes checked against verification. No deferred classified readability gaps; final editorial audit pending. Handoff: ../analysis/commentary-r3/handoff.md. No toolkit issues found; ../analysis/toolkit-issues.md.

## Round4 current-source readability complete

Input pass-004-reviewed:114568/131088 classified (87.40%),16520Unknown. Archived full previous phase at analysis/round-4/commentary-r3 before inserting315 new Data regions/15226 bytes only into prior Unknown spans. All11307 starts/22619Code bytes and all91924Data/25Text bytes explained;347 carried code units,1860 reviewed scaffold entries. Ninety raw spans exactly equal the Unknown complement. Full-scope review corrects old hierarchy/state/spawn/sentinel backing-role prose and names five supported attack/auxiliary/fallback aliases without changing values. Original per-area dimensions, row boundaries, Work8 event/transition widths and natural eligibility remain separate uncertainties. Fresh final verify ok=true,byte_identical=true; reading-after refreshed43077lines; exact source/reading hashes and preservation checks in analysis/commentary-r4. No deferred classified readability ranges; final editorial audit pending. [Handoff](../analysis/commentary-r4/handoff.md), [scope assessment](../analysis/commentary-r4/scope-review.md). No toolkit issues found; [report](../analysis/toolkit-issues.md).

## Round5 current-source readability complete

Input pass-005-reviewed:117138/131088classified (89.36%),13950Unknown. Entire prior phase archived at analysis/round-5/commentary-r4. Integrated17Data regions/2570bytes only into prior Unknown spans. All11307starts/22619Code,94494Data,25Text and16header bytes explained;347carried code units,1871reviewed scaffold entries,84exactUnknown raw spans. No deferred classified readability gaps. Second hierarchy now separates2525finite fields from35backing fields within2560minimum prefix;412suffix remains Unknown. Corrected internal attribute/action backing prose, $9C45 OAM-attribute meaning and sentinel exclusion; natural domains and original allocation remain uncertain. Two CPU aliases/one data label renamed without value changes; old crosswalk retained. Fresh final verify ok=true,byte_identical=true,131088bytes; reading-after refreshed and exact hashes checked. No new runtime scenario. Final editorial/archive acceptance pending. [Handoff](../analysis/commentary-r5/handoff.md), [scope](../analysis/commentary-r5/scope-review.md), [verification](../analysis/commentary-r5/verification.json). No toolkit issues found; [report](../analysis/toolkit-issues.md).

## Round6 current-source readability complete

Input pass-006-reviewed:117150/131088classified (89.37%),13938Unknown. Full prior phase archived at analysis/round-6/commentary-r5. Integrated12one-byte StaticData reserved coarse-result index-zero alignment slots only in prior Unknown spans; explained common key/result1-based indexing and original-allocation limits in source/uncertainties. All347code units/11307starts,94506Data,25Text plus16header bytes reviewed;1871reviewed scaffold entries,72exactUnknown spans, no deferred classified readability gaps. Symbols/instruction lines/assets/prior crosswalk retained. Fresh verify ok=true,byte_identical=true for131088bytes; reading-after refreshed and source hashes checked. Final editorial/archive acceptance pending. [Handoff](../analysis/commentary-r6/handoff.md), [scope](../analysis/commentary-r6/scope-review.md). No toolkit issues found; [report](../analysis/toolkit-issues.md).

## Round6 coordinator status-prose correction complete

Same commentary phase resumed after coordinator identified stale status claims. Archived complete pre-correction phase at analysis/round-6/commentary-r6-before-status-correction. Bounded audit reviewed226remaining status occurrences, corrected overlay/action/status-icon/PPU/event and other stale helper/table/continuation prose, and refreshed25affected code-unit summaries/source positions. All347code units/1871scaffold entries reviewed;11307starts/22619Code,94506Data including16header bytes,25Text;117150classified (89.37%),13938Unknown across72exact raw spans. Eight active historical UnclassifiedBank operands replaced with supported semantic CPU aliases; every prior symbol/label retained and crosswalk unchanged. Fresh final verify ok=true,byte_identical=true131088bytes; read/source hashes refreshed. No new structural lead or ROM/map/toolkit changes. Final editorial/archive acceptance pending. [Review](../analysis/commentary-r6/status-prose-review.md), [handoff](../analysis/commentary-r6/handoff.md). No toolkit issues found.


## Commentary round7 — complete current-source readability

Current-source readability complete for117550/131088classified bytes (89.67%):22619Code/11307starts,94906Data including header,25Text. All347functional code units and prior labels/symbols/crosswalk/reviews preserved from the fully archived corrected r6 (analysis/round-7/commentary-r6). Two new Data structures400bytes merged only into the prior44672..45072Unknown span; no whole-map source regeneration. New names NesTileStripeContinuation and NesTileBorderNotchPairs describe static NES2bpp tile backing, with visual planar closed-border/notch grammar and independent endpoints. No natural upload, original standalone25tile declaration or original padding intent asserted.

Graphics-gap Unknown prose reconciled in source/review/uncertainties; other unresolved roles retained. Full source inventory exact: all1872scaffold entries reviewed,347code units reviewed,11307starts unchanged,71raw spans exactly13538Unknown. See coverage.json, scope-review.md, preservation.json and accepted-scope-review.json. Whole-ROM classification incomplete; final editorial/archive audit pending. No redirect run.

Fresh `commentary.py verify`: ok=true,byte_identical=true for131088bytes; ROM SHA256115b50b36d4dfb7beaa032f868f1b6f11c8384764c79428fe821c1e23f5ebdc6. Fresh `read` refreshed reading-after.asm; exact verification source hashes checked and source-hashes.json saved. Map/state match pass007and frozen comparison copies; ROM/configured release/workspace preserved. No new execution/scenario. No toolkit issues found; [report](../toolkit-issues.md).

Evidence: analysis/commentary-r7/handoff.md, coverage.json, preservation.json, verification.json and source-hashes.json.

## Round 8 current-source completion

All earlier dated progress sections are historical. Current classification100.00%, zeroUnknown; full current-source readability complete for11451starts/22881Code,108182Data includingheader,25Text. All1873scaffold entries and349code units reviewed,347valid prior units carried. Integrated72new regions only in71old Unknown spans; generic Data remains unlabeled with short uncertainty/non-code notes. New9354alternate updater andC780signed scaling entry have variable/branch/stack/bank contracts. Reconciled old structural Unknown claims in active source, uncertainties and metadata. Full previous commentary archived at analysis/round-8/commentary-r7. Preservation verifies all11307old instruction lines/3313labels/symbols/sevenassets/1801old map objects unchanged. Fresh verify ok=true,byte_identical=true131088bytes, fresh read/hash agreement; no deferred readability range. Final editorial_audit pending. [Handoff](../analysis/commentary-r8/handoff.md), [scope](../analysis/commentary-r8/scope-review.md), [final gates](../analysis/commentary-r8/final-gates.md). No redirect or new execution. No toolkit issues found; ../analysis/toolkit-issues.md.

## Round8 final editorial acceptance checks complete

ROM classification: **100.00% (131088 / 131088 bytes classified; 0 Unknown)**. Full current-source readability and targeted final editorial review are complete. The required final audit passes after the metadata-corrected authoritative checkpoint **pass-008-editorial**. Coordinator independent acceptance/goal completion remains separate.

## Actual gates and outputs

- Manual editorial assessment: `commentary/editorial-audit.md`, mirrored here as `editorial-audit.md`. Preserved valid full prior review credit and reviewed concrete new-entry/branch/RAM/bank contracts, stale numeric aliases, names/bounds/contradictory prose, generic-data uncertainty and copyright/local-only notices.
- Final configured `python3 /root/romatlas/scripts/commentary.py audit /root/rygar`: **ok=true, byte_identical=true, editorial_audit=true, reviewed_regions=1873, reviewed_units=1936**. `reviewed_units` counts per-region unit records; the functional code-unit catalog has349units. The audit freshly reassembled/linked all131088bytes and refreshed reading-after.asm. Results: `commentary/audit.json`, `commentary/verification.json` and copies `audit.json`, `verification.json` here. `commentary/review.json` records actual final findings and status=reviewed.
- Fresh `python3 run.py checkpoint pass-008-editorial`: map validation/generation/ca65+ld65 byte-identical rebuild succeeded; map `maps/pass-008-editorial.json`, report `analysis/pass-008-editorial-verification.json`, generated comparison `generated/pass-008-editorial`. The maintained source was not regenerated.
- `maps/current.json`, `analysis/state.json`, `maps/pass-008-editorial.json`, `commentary/baseline.json` and `commentary/phase1-state.json` are identical. New frozen comparison tree/reading-before and manifest match this checkpoint; previous comparisons and complete pre-final phase are preserved under `analysis/editorial-r8/before-editorial` and `previous-before`.
- Final source hashes exactly match the audit and verification. `source-hashes.json` records every local source/asset hash plus reading-after, findings, review, manifest and audit hashes. `preservation.json` proves all11451instruction lines unchanged except the37documented operand-alias substitutions,3330source labels unchanged, original symbol definitions retained,7assets identical, and exact structural map identity. `coverage.json`/`scope-inventory.json`:1873reviewed entries,349functional code units,11451starts,22881Code,108182Data including16header,25Text; no pending readability/editorial ranges.
- ROM SHA256 unchanged: `115b50b36d4dfb7beaa032f868f1b6f11c8384764c79428fe821c1e23f5ebdc6`. Current map/state/checkpoint SHA256: `f96f6908cab8a8ec1f89a4510375620d8fbeba7eea0023278851f30f86c6ca65`. Existing configured release used; no toolkit changes/rebuilds.

## Concrete editorial changes

Replaced25active address-alias names across37operands using established CPU aliases or explicit field offsets; kept historical aliases. Corrected two `Work31` comments/metadata to `WorldObjectCount`, ending-threshold “positive” to “unsigned” with boundary uncertainty retained, and explicitly scoped the no-new-gameplay sentence to Round2commentary. All instruction bytes remain unchanged. Intermediate rebuilds caught an editorial BF70-vs-BF6E entry-alias mistake; fixed four affected calls to the precise caller-X `$BF70` entry and preserved the original full-entry calls. Subsequent final audit passes; this was a local editing error, not a toolkit defect.

Coordinator-authorized map-only corrections: ID534 physical118046/$CD0E `QueueSecondarySound` -> `FindFreeObjectSlot`; ID535 physical118725/$CFB5 `UpdatePlayerObjectControl` -> `QuantizeSignedPlayerVector`. Configured serve/map.annotate preserved IDs, offsets, lengths, kind, confidence, provenance, views and all other metadata except the intended label/comment/evidence updates; all1871other objects identical. Concise mechanics/evidence appended. Old names remain in immutable checkpoints/historical crosswalks. `map-label-corrections.json` contains exact before/after and API results. There is no byte reclassification.

## Genuine limitations and next action

Natural callers of the static alternate entries, exact enemy identities, original schemas/layouts and generic-data meanings remain unresolved. Seventy generic Data blocks need no invented semantics and remain unlabeled with short uncertainty notes. Gameplay remains artificial best effort: five later areas, successful spell, two boss attacks without damage/victory, full22-step ending to visibleEND. No natural completion, rope traversal or full hardware compatibility claim. No new runtime replay/redirect occurred during final review.

Coordinator should independently inspect the final audit, metadata-only map diff, editorial findings and preservation/hash records before declaring the goal complete. No worker gate remains pending. No separate compressed archive or publication is required. Deliverables are local readable bank-organized source and portable NES JSON; only the map is distributable. ROM/source/assets/internal state/snapshots/traces remain local.

No toolkit issues found; see `analysis/toolkit-issues.md`.
