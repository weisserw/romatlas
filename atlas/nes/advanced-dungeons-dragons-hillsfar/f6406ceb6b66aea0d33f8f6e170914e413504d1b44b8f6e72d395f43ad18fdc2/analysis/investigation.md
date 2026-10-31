# Current coordinator terminal outcome

Justified terminal incomplete at pass-004:262124classified/36Unknown in14intervals. Final source readability/editorial audit and byte-identical verification passed; no unsupported residual roles were assigned. See [coordinator rejection review](coordinator/rejection-review.md), [terminal audit](coordinator/terminal-audit.md) and [handoff](coordinator/terminal-handoff.md). No archive, deletion or publication. Historical investigations follow unchanged.

---

# Round004 disassembly complete — ROM incomplete

ROM classification: 99.99% (262124 / 262160 bytes classified; 36 bytes unknown).

Input pass-003:238740 classified,23420 unknown/176 intervals. Independently accepted23384 unique bytes (6631 Code,16753 Data), allStatic. Now36 unknown bytes/14 intervals. Exact count evidence: [counts](round-004/classification-counts.json), [unknown complement](round-004/unknown-complement.json), [annotation ledger](round-004/annotation-ledger.json). Existing3067 accepted regions, labels and provenance preserved exactly. Structural classification, actual execution, natural callability, current-source readability and final editorial acceptance remain separate.

Fresh3063 proposed instruction starts and870 accepted graph instructions match; independently checked relocation158 differences, finite24-way arrays, exact storedtailCode/Data partitions and50/33 textorigin extents. Data consumers/patterns/boundaries checked before generic fallback. Shared substrings/direct strings narrow the old “allrecordstarts” wording; no directoryextent change. See [independent review](round-004/independent-review.md), [fresh decode](round-004/fresh-decode.json), [structural checks](round-004/structural-validation.json), [data review](round-004/data-boundary-review.json), [lead outcomes](round-004/lead-outcomes.json).

Residual isolatedRTS/prefix5F/20/0720 andtwo10bytefragments remainUnknown. Arena21 B115 PLP-stack counterexample and town23 B115 interior-of-STA boundary conflict are scoped independently by actualbank; copiedoriginaltail evidence is not extended backward. [Residual review](round-004/residual-review.json). No new useful narrow test in this boundedpass; no wholeROM or terminalexhaustion claim.

No repeated startup/UI/pagination gameplay. Reused actual artificial maze object route: ordinaryRight after documentedpatches reachedF06D A0/X1/Y0, four actualoperations savedF=[0,1,0]. No naturalcompletion/award/ending claim. Scenario checklist remains in analysis/scenarios.json; commentary corrects old Ycaller and12byte-residue descriptions.

Map.validate passed; saved internalstate and current map match fresh maps/pass-004.json. [Fresh byte-identical checkpoint](pass-004-verification.json): mapSHA256 29170e008fbe17f8b44f002d3296ebae0f30a7eb656f93f7ea14b155f334c337; generated/pass-004. Commentary tree unchanged for next full current-source phase; nextphase copies entire tree before prepare. ROM and configuredbinary unchanged. No toolkit issues found: [persistent report](toolkit-issues.md). No toolkit edit/archive/delete/publication.

Header16bytes Data. Per physical8KiB PRGbank:

| Physical bank | Code | Data/Text | Unknown |
|---:|---:|---:|---:|
| 0 | 0 | 8192 | 0 |
| 1 | 1864 | 6326 | 2 |
| 2 | 0 | 8192 | 0 |
| 3 | 1864 | 6326 | 2 |
| 4 | 0 | 8192 | 0 |
| 5 | 1864 | 6326 | 2 |
| 6 | 0 | 8192 | 0 |
| 7 | 1864 | 6326 | 2 |
| 8 | 0 | 8192 | 0 |
| 9 | 7 | 8185 | 0 |
| 10 | 0 | 8192 | 0 |
| 11 | 7 | 8185 | 0 |
| 12 | 0 | 8192 | 0 |
| 13 | 7 | 8185 | 0 |
| 14 | 0 | 8192 | 0 |
| 15 | 7 | 8185 | 0 |
| 16 | 997 | 7195 | 0 |
| 17 | 2062 | 6130 | 0 |
| 18 | 1629 | 6563 | 0 |
| 19 | 3768 | 4423 | 1 |
| 20 | 4565 | 3627 | 0 |
| 21 | 2900 | 5281 | 11 |
| 22 | 2718 | 5474 | 0 |
| 23 | 3807 | 4373 | 12 |
| 24 | 1938 | 6254 | 0 |
| 25 | 1287 | 6904 | 1 |
| 26 | 3416 | 4775 | 1 |
| 27 | 1887 | 6305 | 0 |
| 28 | 5009 | 3183 | 0 |
| 29 | 4051 | 4141 | 0 |
| 30 | 7015 | 1177 | 0 |
| 31 | 3578 | 4612 | 2 |

Prior history follows unchanged.

# Round004 disassembly active

Fresh bank-aware decode: 3063 proposed instruction starts and 870 accepted graph instructions agree. Reviewing structural contracts and data boundaries independently before annotations; accepted new bytes still 0. Input pass-003: 238740/262160 classified, 23420 unknown. Current frontier: analysis/round-004/frontier.json. Prior history follows unchanged.

# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## Round 001 — disassembly pass

Started from initialization, empty lead queue and no prior handoff. Baseline verified, then reset/NMI roots (CPU $FFD8/$C003, banks [0,1,30,31]) discovered and inspected. Initial process lost its live aggregates on host termination; saved startup discovery was decoded and accepted as Static, never mislabeled as persisted Observed evidence. Caller pagination mistakes were corrected; bulk historic query ledger is retained. All later emulator batches import/export durable snapshots and retain contextual fetch roots before exit. [Startup graph](round-001/discovery-startup.json), [accepted decoded spans](round-001/decoded-batch1.json).

Clear ASCII prose was classified only over exact printable runs after nearby script/pointer inspection and code-conflict checks. Script/control bytes around text remain unknown. Long 00/FF runs were classified generic non-code at exact value boundaries, with no asserted generator or padding purpose. Header and fixed hardware vectors have independent iNES/hardware evidence. [Text bounds](round-001/text-review.json). Five accepted code batches use explicit views and supported direct paths; unobserved suspicious unofficial paths were skipped. Static and observed provenance remain separate. [Latest graph](round-001/batch05-code.json).

**R001-RIDE-COMPLETE — verified.** Ordinary sample-cleric route reached overworld and horseback; jump/speed inputs exercised. Source-backed shortcut: route extent $03F4=35, set $03F5=34/$03DB=8 and advance60 frames. Consumer increments progress and sets $03E1 completion; city entrance prompt verified. This establishes artificial execution and supported transition, not natural reachability of patched progress. [Contract](round-001/riding-progress-contract.json), [patch](round-001/gameplay07-actions.py), [result](round-001/gameplay08.png). Reuse saved state; unchanged retry unnecessary.

**R001-TOWN-MAPS — verified.** City entry required horizontal Left/A at prompt; Up did not select Yes. Town code banks [22,23,30,31]. Movement/search verified. Grid state $7C/$7D and facing $7E persist to $071C/$071D/$074B; district $074A selects four 512-byte maps through pointer table physical2D434. Four origins $957A/$977A/$997A/$9B7A have independent 512 stride; consumer indexes y*16+x, page and nibble by facing. Fourth endpoint is next distinct pointer-record block. Accepted physical2D58A..2DD8A as four Static Data maps, replacing only contained generic uniform runs. [Data evidence](round-001/town-map-data-evidence.json).

**R001-BUILDING-ENTRY — unresolved, actionable.** Initial x13,y4,west hypothesis plus Left moved to x12 and searched empty area. Corrected adjacent-cell reasoning: patch x14,y4,west and persistent copies, then Left produces x13,y4 and explicit “YOUR RIGHT SIDE TARGET SHOOTING.” This corrects tentative arena interpretation; no arena visit claimed. A at tested locations searches empty area; B toggles status/map overlay. Relation among location-message side, adjacent-cell collision and actual entry action still unresolved. Repeated A at same empty location adds no evidence; inspect the town interaction dispatcher before next experiment. [Source consumer](round-001/town-facing-consumer.json), [successful location](round-001/gameplay21.png), [seed snapshot](round-001/gameplay21.snapshot), [B result](round-001/gameplay22.png).

**R001-GRAPHICS-CONSUMER / R001-GRAPH-CONTINUE — actionable redirect.** Loader around $D2FC..D33F reads through zero-page pointer and sends decompressed bytes to PPU; positive/negative counts and terminator provide promising stream boundaries. Pointer origins/complete formats are not yet established, so patterned PRG banks8..17 were not blanket-annotated. Current graphs stop at mapper-changing wrappers and indirect handlers. Derive safe bank/consumer contracts from current source and durable contexts before extending.

**R001-LATER-ENDING — unvisited, prerequisite unresolved.** Reviewed class-specific dialogue/script prefixes in banks0..7 and town handler references to $0720/$0721. No safe complete quest/stage/boss/terminal initializer was established. A random flag edit or guessed PC would not constitute useful supported progression. Redirect should follow guild/quest interpreter and building handlers, derive exact fields, then use ordinary supported transitions with patches. Arena, maze, archery minigame and ending targets remain explicit in [scenario checklist](scenarios.json), including reasons and snapshots. This pass does not claim gameplay or method exhaustion.

Final: ROM classification: 34.17% (89582 / 262160 bytes classified; 172578 bytes unknown). Per-bank counts and exact complement are [machine-readable](round-001/classification-counts.json) and [reported in progress](progress.md). Annotation evidence covers every classified byte. Readability/commentary gate pending. [Pass-001 verification](pass-001-verification.json) is byte-identical. No toolkit issues found; [report](toolkit-issues.md). Continue commentary then redirect; do not repeat baseline or completed experiments.


## Round 002 disassembly acceptance

ROM classification: 68.73% (180185 / 262160 bytes classified; 81975 bytes unknown).

Accepted ready leads R001-GRAPH-CONTINUE, R001-GRAPHICS-CONSUMER, R001-COMMENTARY-SCRIPT, R001-COMMENTARY-PACKED, R001-TAVERN-DISPATCH, R001-GENERIC-FRAMING-TAILS after independent consumer/boundary/context review. Additional tavern continuation +1,358 bytes includes51 Static Code and1,307 Data. Decisive evidence: round-002/independent-review.json, code-contract-review.md, fresh-code-decode.json, connected-tavern-review.json, tavern-final-review.json. Existing classifications retained. Exact 353-interval complement/per-bank totals: round-002/classification-counts.json. Full outcome/count table in progress.md; byte-identical final pass-002-verified. No new runtime; saved representative scenarios reused; no terminal exhaustion claim. Consumed/produced lead outcomes and retry prerequisites are in leads.json. Next commentary must carry correction R001-COMMENTARY-CONTROLS; no source edits here. No toolkit issues found; toolkit-issues.md preserved.

| Bank | Code | Text | Data | Classified | Unknown |
| --- | ---: | ---: | ---: | ---: | ---: |
| 0 | 0 | 7381 | 811 | 8192 | 0 |
| 1 | 0 | 3864 | 175 | 4039 | 4153 |
| 2 | 0 | 7122 | 1070 | 8192 | 0 |
| 3 | 0 | 3869 | 174 | 4043 | 4149 |
| 4 | 0 | 7088 | 1104 | 8192 | 0 |
| 5 | 0 | 5314 | 217 | 5531 | 2661 |
| 6 | 0 | 6953 | 1239 | 8192 | 0 |
| 7 | 0 | 5314 | 213 | 5527 | 2665 |
| 8 | 0 | 0 | 8192 | 8192 | 0 |
| 9 | 0 | 0 | 8152 | 8152 | 40 |
| 10 | 0 | 0 | 8192 | 8192 | 0 |
| 11 | 0 | 0 | 8152 | 8152 | 40 |
| 12 | 0 | 0 | 8192 | 8192 | 0 |
| 13 | 0 | 0 | 8152 | 8152 | 40 |
| 14 | 0 | 0 | 8192 | 8192 | 0 |
| 15 | 0 | 0 | 8152 | 8152 | 40 |
| 16 | 0 | 0 | 6715 | 6715 | 1477 |
| 17 | 0 | 1252 | 1182 | 2434 | 5758 |
| 18 | 1488 | 0 | 0 | 1488 | 6704 |
| 19 | 732 | 250 | 549 | 1531 | 6661 |
| 20 | 2287 | 1803 | 100 | 4190 | 4002 |
| 21 | 949 | 1220 | 220 | 2389 | 5803 |
| 22 | 647 | 1624 | 2102 | 4373 | 3819 |
| 23 | 2163 | 2481 | 226 | 4870 | 3322 |
| 24 | 1938 | 5148 | 1106 | 8192 | 0 |
| 25 | 1269 | 5954 | 917 | 8140 | 52 |
| 26 | 785 | 279 | 581 | 1645 | 6547 |
| 27 | 559 | 808 | 116 | 1483 | 6709 |
| 28 | 99 | 633 | 74 | 806 | 7386 |
| 29 | 1409 | 1236 | 428 | 3073 | 5119 |
| 30 | 5023 | 0 | 84 | 5107 | 3085 |
| 31 | 2755 | 3161 | 533 | 6449 | 1743 |


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


## Round004 redirect r004-a01

Primary exact-role review complete with zero new structural proposals. Tested combination of independently bounded neighboring libraries, exact pattern identities, current27,563-start reference inventory and bank-local decodes; narrowed10-byte candidates into3/7/4/1-byte subclaims and separately considered5FgenericData fallback. Unchanged directrefs are not a universal code prerequisite or data proof. No independently supported role or useful revised test survives; [compact rejection review](redirect/r004-a01/rejection-review.json) and [terminal-incomplete recommendation](handoffs/round-004-redirect.md) require coordinator inspection. Saved broad representative scenarios and actualmazeABI reused; no unchanged replay. Six source editorial phrases saved for separate final targeted phase. No structural/source/toolkit changes.
