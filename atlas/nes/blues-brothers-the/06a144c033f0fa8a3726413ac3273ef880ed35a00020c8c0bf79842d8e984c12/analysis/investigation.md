# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## Round 01 disassembly — interim evidence

- Baseline is verified independently from classification: analysis/baseline-verification.json.
- Observed CPU fetches retain effective 8 KiB PRG slots and storage identity in analysis/round01/rpc-live.jsonl. RAM fetch roots were excluded from ROM annotations (three initially attempted ranges rejected by map validation), not assigned a ROM upload origin. Artificial stage experiments remain separately identified in emulator state and scenarios.
- First code batch: 10123 bytes, 451 accepted regions. Exact decoded instruction extents, accepted roots and bounded branch discovery. Calls/mapper-changing writes/indirect branches stop propagation. Five ambiguous overlapping instruction candidates retained unknown pending inspection; candidates and diagnostics retained in classification-candidates.json. No code inferred from pointer values alone.
- Generic Data partial slices: physical [0x10+bank*0x4000, +0x2400) for bank=0..4, all 256-byte pages inspected for repeated small-index matrices and alternating bitfield patterns; adjoining bytes checked and no conflicting accepted code. Role search considered five parallel bank layouts and chapter-loader context, but exact format/consumers unresolved. These are partial non-code intervals, not inferred whole-object extents. Additional exact uniform runs >=32 bytes are generic Data, not unproven padding. Evidence data-inspection.json. Parsed header and six hardware vectors separately annotated.
- Broad gameplay: natural title/chapter1 and run/jump; artificial warehouse, prison, sewer; construction remains unsettled/artifacted; artificial terminal interstitial and staff names reached. See scenarios.json and gameplay.md. Loader entry $F5A2 sets chapter from $0750, selects resources and resets SP via preceding initialization; calling at a supported instruction boundary with SP=$FF allows bounded later-stage visits. Scroll artifacts warn of skipped screen initialization, not a demonstrated toolkit defect.

## Round 01 final results and verification

ROM classification: 48.71% (63847 / 131088 bytes classified; 67241 bytes unknown).

Code 12750 bytes; Data 51097 bytes (including 16-byte iNES header). No explicit Unknown annotations; 155 unknown complement intervals remain. Counts derive from unique physical intervals in internal store, not raw fallback or execution coverage.

| 8KiB PRG bank | Code | Data | Classified | Unknown |
|---:|---:|---:|---:|---:|
| 0 | 0 | 8192 | 8192 | 0 |
| 1 | 0 | 2109 | 2109 | 6083 |
| 2 | 0 | 8192 | 8192 | 0 |
| 3 | 0 | 1686 | 1686 | 6506 |
| 4 | 0 | 8192 | 8192 | 0 |
| 5 | 0 | 1412 | 1412 | 6780 |
| 6 | 0 | 8192 | 8192 | 0 |
| 7 | 0 | 1484 | 1484 | 6708 |
| 8 | 0 | 8192 | 8192 | 0 |
| 9 | 63 | 2852 | 2915 | 5277 |
| 10 | 544 | 181 | 725 | 7467 |
| 11 | 1787 | 97 | 1884 | 6308 |
| 12 | 401 | 34 | 435 | 7757 |
| 13 | 442 | 43 | 485 | 7707 |
| 14 | 3064 | 143 | 3207 | 4985 |
| 15 | 6449 | 80 | 6529 | 1663 |


Final current map: maps/current.json, state analysis/state.json. Authoritative checkpoint maps/round01-disassembly-corrected.json; source generated/round01-disassembly-corrected/game.asm; verification analysis/round01-disassembly-corrected-verification.json confirms ok=true and byte_identical=true. Generated diagnostics empty. ROM SHA256 unchanged 06a144c033f0fa8a3726413ac3273ef880ed35a00020c8c0bf79842d8e984c12. Earlier round01-interim and round01-disassembly checkpoints are historical/superseded; do not resume their rejected static path.

Final audit correction: apparent BPL/JSR at physical $17FDB/$17FDD is four-byte indexed sprite-layout Data. BNE/BEQ complement proves no fallthrough; observed $BFCF TXA begins immediately after table. Withdrawn bogus call to physical $14A38 and its unsupported descendants, revalidated all Static Code from accepted roots behind barrier. Prior evidence/checkpoints retained; see audit-correction.json, pre-audit-state.json, expansion-evidence-pre-audit.json. Fixed-bank continuation uses mapper2 invariant banks14/15 with null lower slots after writes, never invented return contexts; static accepted entries come from actual branches/direct calls with inspected boundaries. No legal-decode-only claims.

Runtime summary: 8562 unique instruction identities and 12113 contextual roots across natural/artifical scenarios, 13639 contextual edges. These include RAM identities, not all classified ROM. Detail capture intentionally capacity0 at end; 10415745 detail events dropped, aggregate_dropped=0. Retained deterministic RPC input/patch history and selected durable snapshots/captures. Replay resets trace before needing forward time; snapshots restore emulator state but not trace history. Final emulator state artificial, bank slots[12,13,14,15], frame513 after stage4 settling.

Progression: natural copyright/title/chapter1 running+jumping; artificial warehouse, prison and sewer visits with distinct scenes/action. Construction attempted with extra settling/health shortcut but artifacted display persists; ending WELCOME TO THE BAND, staff-name credits and color/player animation visited with artifacts. No natural late-game reachability or clean rendering claim. See analysis/scenarios.json, analysis/gameplay.md and progression.py. Further screen-initialization investigation is a concrete redirect question; no repeated long blind input runs warranted.

No toolkit issues found; analysis/toolkit-issues.md retained. No toolkit changes, cleanup, archive or publishing performed. Disk at handoff ~3.8GiB free; round01 evidence ~102MiB, scratch12KiB. Stop this disassembly pass at supported easy-gain handoff; commentary must cover all currently classified source before redirect. This is not terminal method exhaustion: compressed graphics/resource consumers, object-state dispatch contracts and rendering initialization remain promising targeted research.


## Round02 disassembly completed — incomplete classification

ROM classification: 97.32% (127569 / 131088 bytes classified; 3519 bytes unknown).

17,223 Code bytes;110,346 Data bytes including16-byte header. Added63,722 unique bytes:4,473 Code +59,249 Data. No explicit Unknown annotations;49 complement intervals remain,recorded with physical identities,effective contexts and frontier dispositions in analysis/round02/unknown-ledger.json. Internal store unique-byte counts are authoritative; neither raw export gaps,execution coverage nor readability count as classification.

|8KiB PRG bank|Code|Data|Classified|Unknown|
|---:|---:|---:|---:|---:|
|0|0|8192|8192|0|
|1|0|8192|8192|0|
|2|0|8192|8192|0|
|3|0|8192|8192|0|
|4|0|8192|8192|0|
|5|0|8192|8192|0|
|6|0|8192|8192|0|
|7|0|8192|8192|0|
|8|0|8192|8192|0|
|9|357|7835|8192|0|
|10|546|7615|8161|31|
|11|1787|6062|7849|343|
|12|1082|7019|8101|91|
|13|442|7701|8143|49|
|14|5100|283|5383|2809|
|15|7909|87|7996|196|


Accepted entire independently reviewed revision2 proposal:47,986Data/4,442Code. Primary packet/nested/sprite/spawn/descriptor/handler/player/auxiliary evidence retained in analysis/redirect/round01-contracts. Independent extent/context checks:analysis/round02/review.py,independent-review.json,decoded-proposal.json. Consumer/source/field/stack review and bounded graph reconstruction:graph-review.py/json;4,926 contextual nodes and1,920 proposed instructions,zero unsupported proposed nodes after impossible complementary fallthrough removal and explicit mapper/I/O write barriers. Applied exclusively through map.annotate Static;annotation-results.jsonl and apply-reviewed.py preserve edits. Code is statically structural,not newly Observed;natural selection/reachability unresolved beyond previous scenarios.

Supported follow-through beyond proposal:extra-review.json/extras.py add31Code bytes in auxiliary caller/restore sequences and4,344Data bytes (title block/attributes,generic graphic patterns,default voices,effect pointers,contact data,timer/mapper lookups). pattern-review.json/pattern-followthrough.py add6,919Data bytes:exact bank0 title192-byte copy,graphic continuation and six generic packed-audio-pattern slices. Pattern fallback followed practical reference/consumer and boundary inspection;no complete grammar/exclusive stream domain,exact generator,deadness or padding claim. Bank4 unselected music-pattern tail has248-byte immutable match with bank2BE04..BEFC. Generic17-byte sprite/10-byte spawn motifs remain selection-unresolved. Descriptor byte15,overlapping animation packing,count-zero use,null sprites and translation-view overlap retain their prior uncertainty.

Review corrections preserved alongside historical records:133 sprite slots contain129 nonzero slots but127 distinct nonzero starts (prior prose128 was inaccurate). BDB3 actual opcodeF0 is BEQ to BDBC,not BNE;independent branch entry remains valid. Historical17FDB false path remains rejected and classified Data;no new Code depends on it. Auxiliary creator structural call sites now additionally supported at physical1CB44..1CB4E,1E46C..1E47B,1E4A1..1E4A7;natural calling reachability remains unclaimed. Extra script's initial fixed-bank conversion error was rejected outside-image before save;reload/correct/apply succeeded,not a toolkit finding.

No runtime replays in this pass:reused broad representative scenarios and corrected scene-reset construction/ending captures from prior redirect. analysis/scenarios.json retains inputs,patches,budgets,snapshots and milestones;corrected construction/credits visit supersedes earlier shortcut artifacts. No new scenario claim or hardware/full-fidelity claim. R01 resource/state/player/nested leads verified and consumed;screen initialization remains verified. No repeated failed lead on unchanged premise.

Stopping at supported-batch handoff,not terminal method exhaustion. Concrete next redirect frontier:resolve unclassified object/helper bodies D286..D864 via actual state/caller/field contracts;audio indirect-command/return domains and finite period/scene arrays DBxx..DFxx;PPU helper jump tails B8DF/B972 family using shared routine return contracts;remaining mixed DMC-like fixed-bank patterns C000..C4EB and bank5/6 short data gaps via consumers/context. A legal decode or isolatedRTS is insufficient;no blanket residual import. Existing observed/static sources contain useful starting references,so no further gameplay grind is warranted before these targeted questions. Exact49 residual intervals and boundaries are in ledger;this pass does not claim a terminal rejection audit.

Final current map maps/current.json equals maps/round02-disassembly-final.json;internal store analysis/state.json and durable analysis/round02/final-state.json. Fresh authoritative checkpoint generated/round02-disassembly-final/game.asm;analysis/round02-disassembly-final-verification.json confirms ok=true,byte_identical=true;generated diagnostics empty. Earlier round02-interim and round02-disassembly files are preserved historical checkpoints. Final mapSHA256 fcc2c39c2cf13998983c1eeacc38cd048cb0db0429308c9060b1869ab1b591d5;ROM SHA256 unchanged 06a144c033f0fa8a3726413ac3273ef880ed35a00020c8c0bf79842d8e984c12.

Full old commentary/source/game.asm preserved byte-for-byte,SHA256 470f10f1379c921f74289eec50bb1ec2b5845b51fd9004f8761a636ca4abbc85. Current-source readability now needs next commentary carry/review for63,722 added bytes;this disassembly did not prepare/regenerate/edit commentary. Byte-identical generation alone is not editorial completion. No archive,cleanup,deletion,publishing or toolkit edits.

No toolkit issues found;analysis/toolkit-issues.md retained. Final disk free~3.64GiB. One disassembly pass complete;next phase is full current-source commentary,then targeted redirect if coordinator accepts.

## Round02 redirect — combined structural proposal ready

Unique attempt `analysis/redirect/round02-contracts`, frozen checkpoint round02-disassembly-final/current map fcc2c39c2cf13998983c1eeacc38cd048cb0db0429308c9060b1869ab1b591d5 and current commented source2c0110d4f4b7155dfef2b499fb8bd4e81a14303fbc66b970ea0bc461780a8ef9. No classification mutation. Bounded explicit30-class object tile-response domain,16-entry audio command domains, PPU helper return contracts, player/continue-screen conditional stack paths and data consumer/pattern inspection jointly support3515 unknown bytes (1847Code,1668Data) for independent review. No new runtime/execution claim. Existing full reset construction/credits/scenarios reused.

Primary reasoning and narrower corrected premises: `analysis/redirect/round02-contracts/structure-evidence.md`; reproducible bounded static evidence `proposal.json`, `table-domains.json`, `accepted-instructions.json`, `residual-decode.json`, `flow-review.json`, `data-inspection.json`, `final-check.json`. Copy image BDAF body10 bytes differs from explicit16-byte upload to06F0; final6 copied bytes overlap graphics. Data code-image proposal uses supported representation, no uploaded view invention. Every initial49gap disposition retained in `residual-dispositions.json`. Four unproposed bytes D25C,D5F5..7 are ambiguous isolatedRTS, not rejected because merely unobserved. No actual incoming structural evidence differentiates stubs/data; pointer scan and exact controls retained. All identified promising connected/independent avenues followed before combined ready handoff; no terminal exhaustion claim/rejection audit required now. Leads R02-OBJECT-RESIDUAL, R02-AUDIO-PPU-RESIDUAL, R02-MIXED-PATTERN-RESIDUAL updated ready; new R02-PLAYER-SESSION-TAILS ready. No toolkit issues found; documented unsupported uploaded view has honest Data image workaround and no blocker.


## Round03 disassembly completed — incomplete classification

ROM classification: 100.00% (131084 / 131088 bytes classified; 4 bytes unknown).

19070 Code bytes; 112014 Data bytes including16-byte header. Added3515 unique bytes:1847 Code (802 instructions),1668 Data, all Static. Rounded100.00% is not completion:4bytes in2physical intervals remain Unknown. Internal unique physical-byte complement is authoritative; raw export fallback, source readability and observed execution remain separate.

|8KiB PRG bank|Code|Data|Classified|Unknown|
|---:|---:|---:|---:|---:|
|0|0|8192|8192|0|
|1|0|8192|8192|0|
|2|0|8192|8192|0|
|3|0|8192|8192|0|
|4|0|8192|8192|0|
|5|0|8192|8192|0|
|6|0|8192|8192|0|
|7|0|8192|8192|0|
|8|0|8192|8192|0|
|9|357|7835|8192|0|
|10|577|7615|8192|0|
|11|1832|6360|8192|0|
|12|1085|7107|8192|0|
|13|459|7733|8192|0|
|14|6655|1533|8188|4|
|15|8105|87|8192|0|

Independent actual consumer/table/branch/stack/bank review: analysis/round03/independent-review.md. All802 exact instruction decodes match configured release binary; tables independently recomputed;3515-byte union has zero overlap with accepted bytes. All four ready R02 leads verified, including full response graph, returning PPU/audio paths, orphanDE7B static helper, specific bounded data and independent session tails. Copier Data source-image10bytes distinguished from16-byte upload and6-byte graphic overlap; unsupported uploaded views not invented.

Residual ledger: analysis/round03/unknown-ledger.json. Physical1D26C CPU D25C and1D605..1D608 CPU D5F5..D5F7 retain code-stub versus return-byte-data ambiguity. Missing live caller is not a universal prerequisite or the rejection premise. Ready connected easy gains exhausted; next redirect independently assesses residual frontier. Successful broad progression/construction/credits routes reused from analysis/scenarios.json; no unchanged runtime replay.

map.annotate58 runs (39 Code,19 Data), map.validate ok, saved analysis/state.json and maps/current.json, fresh checkpoint round03-disassembly-final validates/generates/verifies byte-identical. Verification: analysis/round03-disassembly-final-verification.json. Current/state/checkpoint map SHA256 f122f1e8046caf14315839029f0238d0770ed8036142d8e5e65f329e60ce45b4; separately verified state SHA matches. ROM SHA unchanged 06a144c033f0fa8a3726413ac3273ef880ed35a00020c8c0bf79842d8e984c12. Entire commentary untouched; game.asm SHA 2c0110d4f4b7155dfef2b499fb8bd4e81a14303fbc66b970ea0bc461780a8ef9. Commentary describes prior source and must be carried intact into next commentary phase before reviewing new source. No commentary generation performed in disassembly.

No toolkit issues found; analysis/toolkit-issues.md preserved. No toolkit edits, cleanup/deletion, archive or publication. Pass stops incomplete; next phases controlled by coordinator.

## Batch-only user fallback disassembly

Exact authoritative preflight complement4bytes: physical119404 length1,120325 length3. User-authorized Manual generic Data fallback applied solely to these ranges; unknown meaning and RTS-code versus return-valued-data ambiguity remain explicit. Prior rejection review is preserved and not reopened. Map now131088/131088classified,0Unknown;131084bytes retain evidence-backed classification and4are explicitly Manual fallback. Fresh checkpoint manual-fallback-disassembly validates and rebuilds byte-identically. Full actions/counts/context/identities and next commentary phase are in analysis/manual-fallback-disassembly-handoff.md and analysis/manual-fallback/. No new toolkit issues found.
