# Round 003 disassembly complete — ROM incomplete

ROM classification: 91.07% (238740 / 262160 bytes classified; 23420 bytes unknown).

Accepted 58,555 previously Unknown bytes: 29,381 Code, 29,159 Data and 15 Text. Corrected four existing Static Code bytes to Data at the pagination prompt. **Final checkpoint: pass-003**, byte-identical rebuild verified. Current map, saved state and frozen checkpoint map are byte-identical. All prior region fields, labels and provenance are preserved except region2573 shortened from51 to47 bytes, retaining the preceding code.

Independent review freshly decoded13,928 instruction starts with exact physical offsets, CPU aliases and effective bank contexts. Role/control/stack contracts were assessed separately from legal decoding. Reexecuted ROM-backed validators for20 relocated library blocks (zero operand issues),49 animation resources covering2,302 bytes and168 effect stream/42 envelope origins covering1,718 bytes with no parser frontier. Independently recomputed all281 metasprite record lengths from renderer grammar to exact B3A7 endpoint. Compared copied assets, reset prefixes and vector fields; reviewed bounded consumer references, table formats and non-code patterns for other data. Generic Data claims retain unresolved semantics.

Evidence: [independent review](../round-003/independent-review.json), [fresh decode](../round-003/fresh-code-decode.json), [independent frames](../round-003/metasprite-independent.json), [accepted code](../round-003/code-applied.json), [accepted data](../round-003/data-applied.json), [integrity](../round-003/integrity.json), [counts and exact complement](../round-003/classification-counts.json), [verification](../pass-003-verification.json). Primary contracts and parsers remain in immutable redirect/r002-a01. All annotations used map.annotate, followed by validation, save and export.

The17 bytes253734..253751 are uploaded tile codes HIT/00/SELECT/00/BUTTON. Fresh DEE7 consumer decode proves Y=0..16. DF10 AND04 sets Z; DF12 BEQ fallthrough implies Z clear, forcing DF14 BNE DEC5. No actual saved fetch root lies inside the prompt. The correction removes infeasible decoder fallthrough, preserving preceding47 Code bytes. [Fresh pagination decode](../round-003/pagination-fresh.json).

Observed provenance applies only to exact matching physical/CPU/bank fetch aliases in redirect/r002-a01/observed-roots.json. All saved scenario observations are artificial. Other code stays Static. Copied helper callbacks may be unusable in inactive copies; stored code recognition does not assert full callability. Lower BFD8 reset aliases preserve their selected physical pair after MMC1 reset; upper nonfinal aliases instead continue in fixed bank31. These limits remain explicit.

Reused substantial prior artificial gameplay evidence: ring consumption and successful lock opening, transition to visible maze room/player/ladder/timer and representative movement; synthetic mentor-story ABI. No startup or unchanged UI replay. **Next commentary must correct the former X0 reward premise:** F6/F379 tests saved Y low nibble0; ordinary A-search Y=FF fails, synthetic X64/Y0 passes. Currency/stage remained unchanged; no award or natural completion observed. Entire commentary tree remains unchanged; next commentary should archive it before fresh preparation and carry forward prior readable work with the correction.

This substantive pass stops after the supported ready batch and connected static closures. It is **not terminal exhaustion**. There remain176 exact Unknown intervals/23,420 bytes, chiefly mixed banks16/17 and auxiliary travel/arena, town/archery and maze structures. Twelve bytes at BF4B remain ambiguous because the JSR00A9 RAM contract is unresolved; repetition/legal decoding/RTS alone do not discriminate code from stale or opaque data. Existing consumer/entry closures have no concrete unresolved local continuation. Further redirect can isolate narrower bounds or consumers in mixed residuals and derive the natural maze object/room acceptance contract for the full reward route. Avoid unchanged rare arena/UI permutations without a concrete gain. No full hardware/game compatibility claim.

Next phase: full current-source commentary from pass-003, then redirect. Persistent leads retain accepted outcomes and retry limits. **No toolkit issues found**; analysis/toolkit-issues.md is preserved. ROM, configured binary and commentary hashes are unchanged. No toolkit edit, archive, deletion or publication. All runtime processes exited; approximately9.6GiB free.

| Bank | Code | Text | Data | Classified | Unknown |
| --- | ---: | ---: | ---: | ---: | ---: |
| 0 | 0 | 7381 | 811 | 8192 | 0 |
| 1 | 1853 | 3864 | 2431 | 8148 | 44 |
| 2 | 0 | 7122 | 1070 | 8192 | 0 |
| 3 | 1853 | 3869 | 2426 | 8148 | 44 |
| 4 | 0 | 7088 | 1104 | 8192 | 0 |
| 5 | 1853 | 5314 | 981 | 8148 | 44 |
| 6 | 0 | 6953 | 1239 | 8192 | 0 |
| 7 | 1853 | 5314 | 981 | 8148 | 44 |
| 8 | 0 | 0 | 8192 | 8192 | 0 |
| 9 | 7 | 0 | 8185 | 8192 | 0 |
| 10 | 0 | 0 | 8192 | 8192 | 0 |
| 11 | 7 | 0 | 8185 | 8192 | 0 |
| 12 | 0 | 0 | 8192 | 8192 | 0 |
| 13 | 7 | 0 | 8185 | 8192 | 0 |
| 14 | 0 | 0 | 8192 | 8192 | 0 |
| 15 | 7 | 0 | 8185 | 8192 | 0 |
| 16 | 0 | 0 | 6715 | 6715 | 1477 |
| 17 | 7 | 1252 | 1215 | 2474 | 5718 |
| 18 | 1629 | 0 | 6563 | 8192 | 0 |
| 19 | 3687 | 265 | 3594 | 7546 | 646 |
| 20 | 4565 | 1803 | 762 | 7130 | 1062 |
| 21 | 1451 | 1220 | 604 | 3275 | 4917 |
| 22 | 1849 | 1624 | 2410 | 5883 | 2309 |
| 23 | 2901 | 2481 | 293 | 5675 | 2517 |
| 24 | 1938 | 5148 | 1106 | 8192 | 0 |
| 25 | 1276 | 5954 | 950 | 8180 | 12 |
| 26 | 3410 | 279 | 3082 | 6771 | 1421 |
| 27 | 1887 | 808 | 5494 | 8189 | 3 |
| 28 | 5006 | 633 | 908 | 6547 | 1645 |
| 29 | 4051 | 1236 | 2734 | 8021 | 171 |
| 30 | 6875 | 0 | 787 | 7662 | 530 |
| 31 | 3508 | 3161 | 707 | 7376 | 816 |

