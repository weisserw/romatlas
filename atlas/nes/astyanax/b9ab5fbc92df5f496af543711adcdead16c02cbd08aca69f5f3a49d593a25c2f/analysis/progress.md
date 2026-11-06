# Disassembly progress

Workspace initialized. Baseline and classification have not run.

## Round 001 disassembly in progress
Baseline byte-identical: [verification](baseline-verification.json). Live configured server session 84268. Startup timeout after 44 frames retained state. 1158 unique instructions observed, aggregate drops 0; detail ring drops 3147304 (1000 retained). Inspecting contextual roots before annotation.

Startup classification saved/exported: 1364 regions. ROM classification: 51.09% (133928 / 262160 bytes classified; 128232 bytes unknown). Validated internal map; reset graph inspection is round-001/candidate-listing.txt. Natural runtime at frame265 displayed Astyanax title, banks12/13/14/15. Broad progression work continues using built-in selectors.

### Recovery and progression update
Earlier live session84268 exited143; mailbox clients stalled. This is an environment process exit, no toolkit defect established. Recovery uses self-contained bounded RPC subprocess scripts (round-001/direct.py), loading saved map and importing durable snapshots. Current batch64126 restores stage-select snapshot, selects Stage2-1, advances90 frames then right+B60/right+A30, saves per-step evidence. Stage selector accepted at frame481: round-001/stage_select.png, durable stage-select.snapshot. Main state dispatch consumer E07F–E096 selects bank and indirect destination from E097–E0CF, 19 contiguous triplets ending independently at NMI E0D0; supported handler closure inspection underway. Earlier startup-only statements above are historical.

### Classified runtime/data batch
Batch95325 completed, map validated and saved/exported: ROM classification: 62.74% (164492 / 262160 bytes classified; 97668 bytes unknown). Added3157 accepted fetched instructions in231 regions; zero overlaps/conflicts. Uniform PRG data runs inspected at maximal boundaries against neighboring bytes, discovered direct references, current classified code and runtime fetches (round-001/uniform-candidates.json, annotation-conflicts.json). Generic Data only; filler purpose not claimed. Main state table classified57 bytes from consumer stride3 and independent code boundaries, not selected-index extrapolation.
Stage2-1 Redroad real gameplay/HUD/player confirmed (stage2-mechanics.png). Stage6-2 castle room/HUD R6-2 and enemy sprites confirmed (stage6-patched-entry.png), then90 frames attack/jump actions retained (stage6-patched-play.snapshot). Stage6 is artificial RAM$03E4=9 before A, supported menu contract; Stage2 natural controller route. Ending cheat reaches final hero/princess dialogue (ending-view.png);16 Start advances show further ending dialogue, credits still uncertain. Middle Stage4 and further ending captures now completing.

### Final easy-gain closure in progress
Second observed batch added310 instructions/35 regions, reaching165150 classified (63.00%). Then bank-aware static closure from6564 accepted runtime contexts plus main-dispatch contracts added724 instructions/142 regions, all official/coherent code, no excluded opcodes; inspected static-frontier-listing.txt and saved valid state/export. ROM classification: 63.58% (166680 / 262160 bytes classified; 95480 bytes unknown). Broad Stage4-1 cave gameplay confirmed via known selector RAM$03E4=4 and A. Final Stage6 room advance and terminal ending capture finishing before one final evidence merge/checkpoint. Static discovery remaining barriers are JSR return bank contracts and indexed/indirect dispatches; redirect can trace these without replaying startup. No terminal exhaustion claim.

## Round 001 disassembly completed — incomplete ROM
ROM classification: 63.72% (167060 / 262160 bytes classified; 95100 bytes unknown).

Verified fresh checkpoint: [maps/pass-001.json](../maps/pass-001.json), [generated/pass-001/game.asm](../generated/pass-001/game.asm), [verification](pass-001-verification.json). Internal map validated; exported maps/current.json matches checkpoint. ROM SHA256 unchanged b9ab5fbc92df5f496af543711adcdead16c02cbd08aca69f5f3a49d593a25c2f; checkpoint map SHA256 1a328745e1ab3f23d4293e9ead30db6be74285b8424edc1ca61b589a5245ceed. Rebuilt byte-identical.

| Bank | Code | Data/image/text | Unknown |
|---|---:|---:|---:|
| header | 0 | 16 | 0 |
| PRG00 | 2010 | 0 | 6182 |
| PRG01 | 396 | 0 | 7796 |
| PRG02 | 372 | 359 | 7461 |
| PRG03 | 218 | 303 | 7671 |
| PRG04 | 0 | 0 | 8192 |
| PRG05 | 0 | 4093 | 4099 |
| PRG06 | 0 | 37 | 8155 |
| PRG07 | 0 | 2864 | 5328 |
| PRG08 | 0 | 0 | 8192 |
| PRG09 | 0 | 4466 | 3726 |
| PRG10 | 555 | 2040 | 5597 |
| PRG11 | 0 | 8192 | 0 |
| PRG12 | 1249 | 103 | 6840 |
| PRG13 | 0 | 978 | 7214 |
| PRG14 | 3045 | 309 | 4838 |
| PRG15 | 4022 | 361 | 3809 |
| CHR00 | 0 | 8192 | 0 |
| CHR01 | 0 | 8192 | 0 |
| CHR02 | 0 | 8192 | 0 |
| CHR03 | 0 | 8192 | 0 |
| CHR04 | 0 | 8192 | 0 |
| CHR05 | 0 | 8192 | 0 |
| CHR06 | 0 | 8192 | 0 |
| CHR07 | 0 | 8192 | 0 |
| CHR08 | 0 | 8192 | 0 |
| CHR09 | 0 | 8192 | 0 |
| CHR10 | 0 | 8192 | 0 |
| CHR11 | 0 | 8192 | 0 |
| CHR12 | 0 | 8192 | 0 |
| CHR13 | 0 | 8192 | 0 |
| CHR14 | 0 | 8192 | 0 |
| CHR15 | 0 | 8192 | 0 |

Classified code11867 bytes, CHR image131072, Data24104, Text17. All 1818 regions have structural evidence (100% annotated classified regions); semantic/editorial readability remains for commentary. Runtime coverage is separate: 4976 unique physical accepted instruction starts across retained traces, natural 4198, artificial 3086 (sets overlap). Untraced transition intervals deliberately excluded from execution claims. All retained scenario trace summaries show aggregate_dropped=0; detail buffers bounded/zero; startup detail ring dropped3147304 events while aggregates remained complete.

[Exact unknown complement](round-001/unknown-complement.json): 225 intervals; [bank counts](round-001/counts.json), [summary](round-001/summary.json). Startup vectors/static control closure, main-dispatch table, observed gameplay/ending instructions, supported static branch closure and maximal generic uniform data runs verified. Last merge added169 fetched instructions/16 regions, zero overlap conflicts. Ending-final.png explicitly shows THE END; credits were not seen. Stage6 advanced right+attack540 untraced/60traced reached right edge of same room; no definitive boss defeat claim. Stage4-1 cave, Stage2-1 forest, Stage6-2 castle mechanics visited; Stage4 and Stage6 menu RAM patches are artificial and documented in scenarios.json.

Ordinary easy-gain handoff, not terminal exhaustion. Concrete open leads: dispatcher/call bank-return summaries, level/object data consumers and audio/cutscene indirect handlers; see leads.json and handoffs/round-001-disassembly.md. No prior leads existed; newly established main-dispatch table verified, broad progression verified, unsupported bank/indirect continuations retained unknown. Next phase: full current-source commentary, then redirect connected investigation; no empty new disassembly round.

Toolkit: no toolkit issues found; [persistent report](toolkit-issues.md). No blockers; environment exit143 recovered through durable snapshots and bounded subprocess batches. ROM unchanged; no toolkit edits, publication, archive or cleanup.

## Round 001 commentary completed — current-source readability, incomplete ROM

Completed all currently classified source: 1818 regions /167060 bytes, including11867 code bytes and5721 instructions. Source consolidated into204 contiguous code islands with740 meaningful group/routine/branch labels and shared field/hardware aliases;157 semantic units added to the intact2043-entry review scaffold. No pending readability ranges. Unknown95100 bytes/225 intervals remain explicitly opaque; classification unchanged63.72%. Final whole-ROM editorial/archive acceptance remains pending.

Fresh [commentary verification](../commentary/verification.json) byte-identical=true/ok=true; [reading copy](../commentary/reading-after.asm) refreshed. Final source SHA25686daa9065ecf285aaeb66dcd00d1e46287ac7675a7a76f66b75c6b67492cd95d; symbols b036e6927b97c7acea2403d1bc8ac8773896bbb07c1fdff21ff0b01248ff110b. ROM and current structural map hashes unchanged. Frozen commentary baseline/before preserved. [Scope/checkpoint](../commentary/current-source-checkpoint.json), [progress](../commentary/progress.md), [uncertainties](../commentary/uncertainties.md), [handoff](handoffs/round-001-commentary.md).

Next redirect: source-backed bank-return contracts, scene/audio indirect domains and level/metatile/spawn consumers. These are structural questions, not deferred classified-source readability. No toolkit issues found; [report](toolkit-issues.md). No runtime experiment, toolkit change, cleanup, archive or publication in commentary.

## Round001 redirect active
Input pass-001 unchanged,167060 classified/95100 unknown225 intervals. Attempt [round001-20261106](redirect/round001-20261106/plan.md). Tracing concrete call-return contracts and scene/audio/stage resource consumers. No classification proposal accepted; no terminal exhaustion.

Redirect findings: [active findings](redirect/round001-20261106/findings.md).133 call summaries/102 preserving; audio decimal20 controls0C..1F independently bounds40-byte table. Resource6/8 chains/counts support large noncode containers; pair4 special descriptors expose shared/nonmonotone pointers, so split claims rather than infer entire bank. Ending/actor and resource bounds review active. No map changes.

## Round001 redirect completed — coherent batch ready for independent disassembly

[Handoff](handoffs/round-001-redirect.md), [attempt findings](redirect/round001-20261106/findings.md), [summary](redirect/round001-20261106/summary.json), [225-range ledger](redirect/round001-20261106/unknown-ledger.json). Updated3 ready leads and added actor/text leads; histories preserved. Proposed55183 unique unknown bytes (code27632/data27551), accepted0. Current checkpoint remains167060 classified/95100 unknown63.72%, map SHA2561a328745e1ab3f23d4293e9ead30db6be74285b8424edc1ca61b589a5245ceed. No code/data/start/decode proposal conflicts found; conditional bank and selector prerequisites explicitly retained for independent review. All20 audio commands, intro11/ending19 plus10 scene families,104 actor entries/11 connected state families,6 special initializers, textFB and18 resource records inspected. Resource generic-data fallback supplied where schema details remain uncertain. No terminal exhaustion claim. ROM/current map/verified commentary hashes unchanged; no runtime execution claims or mutation. No toolkit issues found; no blockers. Next phase disassembly of combined batch, then full current-source commentary and redirect.

## Round002 disassembly active
Independent review of combined redirect batch; pass001 map and current-source commentary preserved. Reviewing caller/stack and conditional MainPrgPair restoration, fixed-bank aliases, resource chain and partial table boundaries. No annotations accepted yet. Review artifacts will be in analysis/round-002/.

## Round002 disassembly completed — incomplete ROM

ROM classification: 84.77% (222243 / 262160 bytes classified; 39917 bytes unknown).

Accepted55183 unique formerly unknown bytes:27632Code and27551Data,399 additional regions, all Static. Fresh [pass-002 verification](pass-002-verification.json) ok=true/byte_identical=true; internal map validated. [maps/pass-002.json](../maps/pass-002.json) and maps/current.json SHA256aa5bc54ceebed0be0c35d2a9012b6b6de91dcd9345a4927ff8db4850addee119; generated/pass-002 is separate from preserved commentary/source and all frozen/previous checkpoints. ROM SHA256 unchanged b9ab5fbc92df5f496af543711adcdead16c02cbd08aca69f5f3a49d593a25c2f.

Independent review: [decisions](round-002/review-decisions.md), [11926-instruction/305-group mechanical audit](round-002/independent-review.json), [fresh consumers](round-002/consumer-review.txt), [accepted exact regions](round-002/accepted-regions.json). Every candidate effective view and physical boundary freshly checked; coherent indexed consumers/routine contracts provide structural support beyond decoding. Audio continuations explicitly require RAM08 matching caller main pair. Fixed14/15 views use C000/E000. Partial actor/scene selector slices do not claim full domains/natural selection. Resource18 descriptors, all layout cells and inclusive spawn ends independently checked; chain exceptions13→15 (222-byte gap) and15→16 (alternate/shared row order) invalidate universal chain model only. Own bounded resource spans accepted; gap unchanged. Special4/5 shared86E5 excluded.

| Bank | Code | Data/image/text | Unknown |
|---|---:|---:|---:|
| header | 0 | 16 | 0 |
| PRG00 | 6454 | 52 | 1686 |
| PRG01 | 6359 | 32 | 1801 |
| PRG02 | 988 | 381 | 6823 |
| PRG03 | 5964 | 493 | 1735 |
| PRG04 | 5359 | 1447 | 1386 |
| PRG05 | 3271 | 4093 | 828 |
| PRG06 | 0 | 8192 | 0 |
| PRG07 | 0 | 8192 | 0 |
| PRG08 | 0 | 8192 | 0 |
| PRG09 | 0 | 7973 | 219 |
| PRG10 | 740 | 2040 | 5412 |
| PRG11 | 0 | 8192 | 0 |
| PRG12 | 2017 | 143 | 6032 |
| PRG13 | 0 | 978 | 7214 |
| PRG14 | 3722 | 517 | 3953 |
| PRG15 | 4625 | 739 | 2828 |
| CHR00 | 0 | 8192 | 0 |
| CHR01 | 0 | 8192 | 0 |
| CHR02 | 0 | 8192 | 0 |
| CHR03 | 0 | 8192 | 0 |
| CHR04 | 0 | 8192 | 0 |
| CHR05 | 0 | 8192 | 0 |
| CHR06 | 0 | 8192 | 0 |
| CHR07 | 0 | 8192 | 0 |
| CHR08 | 0 | 8192 | 0 |
| CHR09 | 0 | 8192 | 0 |
| CHR10 | 0 | 8192 | 0 |
| CHR11 | 0 | 8192 | 0 |
| CHR12 | 0 | 8192 | 0 |
| CHR13 | 0 | 8192 | 0 |
| CHR14 | 0 | 8192 | 0 |
| CHR15 | 0 | 8192 | 0 |

[Exact residual complement](round-002/unknown-complement.json):331 intervals; [counts](round-002/counts.json), [summary](round-002/summary.json). Code39499 bytes; structural evidence present in all2217 classified regions (100% annotation coverage). Runtime coverage unchanged:4976 unique physical fetched starts,4198 natural and3086 artificial overlapping; no execution claim added. Representative Stage2/4/6 mechanics and THE END already visited (scenarios.json); no unchanged replay needed. Current-source readability was verified for pass001 only; new map requires full current-source commentary next. Original source hashes verified unchanged, recorded in round-002/source-preservation.json; frozen baseline/before and original review retained.

All five ready leads verified on their exact bounded proposals; histories retained in leads.json. Remaining questions are mapper-shadow/caller preconditions, unbounded indirect selector states, stack-reset transitions and mixed residuals, not proof of dead code or terminal exhaustion. No obvious connected accepted proposal left pending; refreshed map should guide next redirect after commentary. No toolkit issues found; [persistent report](toolkit-issues.md). No toolkit edits, input mutation, archive, cleanup, deletion or publication.

## Round002 commentary completed — current-source readability, incomplete ROM

Full current pass002 source reviewed:2217 classified regions /222243 bytes, including39499 code bytes. Incremental sync added27632 code/27551 data bytes in399 slices while preserving prior authored islands, names/prose, review credit and original frozen baseline/before. Prior commentary preserved at round-002/commentary/prior-pass001; no blind regeneration. Current full inventory [commentary/current-review.json](../commentary/current-review.json) is distinct from the retained2043-range preparation scaffold;834 semantic units, no pending readability. Unknown39917 bytes/331 intervals remain opaque. Classification unchanged84.77%, map SHA256aa5bc54ceebed0be0c35d2a9012b6b6de91dcd9345a4927ff8db4850addee119.

Source explains named actor families/finite states, pickups/linked parts, scene/ending text/page/palette/object progression, six special encounter controllers, audio command/nesting/envelope mechanics, reverse-counted PPU transfer retries, resource formats and descriptor-specific bounds. Conditional RAM08 return-bank and partial selector contracts are explicit; no natural-selection or whole-game compatibility claim. Contextual RAM/table/branch names and71 additional context-resolved ROM operands improve readability without changing bytes. Corrected free-slot/different-type scan prose to the actual branches.

Fresh [commentary verification](../commentary/verification.json) ok=true/byte_identical=true, [reading copy](../commentary/reading-after.asm) refreshed. Source SHA256b034c91bd266ced05f757cae33747d535f31dd5614e5bf9869a2979fc4128832, symbolse7d638845e8d8ea0522f2bcebaa559127916e86d87b1ae54db39c8ed18c5a095. [Checkpoint](../commentary/current-source-checkpoint.json), [handoff](handoffs/round-002-commentary.md), [uncertainties](../commentary/uncertainties.md). Frozen preparation remains pass001 provenance; current readability covers pass002. Final editorial/archive acceptance pending, not run here.

Next redirect: mapper-shadow preconditions, partial actor/scene selectors, stack-reset transitions and mixed resource residuals, guided by exact unknown complement round-002/unknown-complement.json. Runtime coverage unchanged4976 unique starts; no repeated gameplay or artificial execution added. No toolkit issues found; [report](toolkit-issues.md). No blockers, toolkit edits, ROM mutation, cleanup, deletion, archive or publication.

## Round002 redirect active (2026-11-06)
Unique attempt `redirect/round002-20261106/plan.md`; input current map aa5bc54ceebed0be0c35d2a9012b6b6de91dcd9345a4927ff8db4850addee119, unknown39917/331. Current-source gate complete. New methods use exact residual post-indirect-call continuations, actor low-bit writers and residual stream/container consumers. No accepted graph replay/map/source edits; research in progress.
Round002 redirect primary results persisted in `redirect/round002-20261106/findings.md`, `target-decodes.json`, `references.json`, and exact residual inventory. Three analogous post-dispatch16-byte gaps and mixed004211/0180BA reviewed; whole-gap noncode inference explicitly excluded there. Frontier remains active.

## Round002 redirect completed — ready batch, not terminal exhaustion
Unique attempt `redirect/round002-20261106` complete. Proposed35743 unique unknown bytes:3504code/32239data, accepted0. Current map/state unchanged aa5bc54ceebed0be0c35d2a9012b6b6de91dcd9345a4927ff8db4850addee119; current39917 unknown/331 intervals, projected4174 if all proposals accepted. New leads R002-RETURN-CONTEXT/R002-SCENE12/R002-SECONDARY-HUD/R002-NONCODE-CONTAINERS. Current direct edges, conditional return witnesses, explicit bank12 scene, secondary/HUD bounded pointer slices and independent generic data researched together. Exact candidates, boundary/reference checks and residual ledger durable; no current/candidate direct branch/call data conflicts. Source/symbol/ROM unchanged final hashes in final-audit.json. No new runtime/no toolkit issues. Next independent disassembly then full current-source commentary; final editorial gate pending. Handoff `handoffs/round-002-redirect.md` includes all preconditions/corrections and actual residual barriers.

## Round003 disassembly active
Input pass002, map aa5bc54ceebed0be0c35d2a9012b6b6de91dcd9345a4927ff8db4850addee119;222243 classified/39917 unknown. Independently reviewed scene12 finite step writers and outer exit, secondary/HUD partial pointer slices and mixed7EA0 routine/records. Fresh configured decoder audit and exact physical/control-boundary checks underway in round-003. Proposed35743, accepted0 so far; source/frozen history preserved. CA87 mapper restoration and distinct CA8A restored0/1 precondition checked independently against source next.

## Round003 disassembly completed — incomplete ROM

ROM classification: 98.41% (257986 / 262160 bytes classified; 4174 bytes unknown).

Accepted35743 formerly unknown bytes:3504Code/32239Data across203 additional Static regions. [Fresh pass003 verification](pass-003-verification.json) ok=true/byte_identical=true; internal map validated, state/current/checkpoint byte-identical SHA25612504fb181283bedd0308ead6ddd422844df02882751d05249744e47c8b82ea5. ROM unchanged b9ab5fbc92df5f496af543711adcdead16c02cbd08aca69f5f3a49d593a25c2f. [Review decisions](round-003/review-decisions.md), [independent audit](round-003/independent-review.json), [fresh consumers](round-003/consumer-review.txt), [accepted regions](round-003/accepted-regions.json).1612 fresh instruction starts/80 groups; pointer words/countXY endpoints and physical identities verified, zero errors.

CA87 remaps saved12; CA8A separately requires restored0/1. Audio returns conditional on08 matching caller; CD14 restores2/3 specifically. Actor5/5/6, active-secondary3 andHUD9 slices retain bounded claims. Scene12 finite0..4 proof follows state writers/outer5 exit. Mixed7EA0 independently coherent16Data/43Code/16Data; no live caller requirement. Generic audio/text/scene/sprite/resource noncode classification does not claim exact full schemas; FE tile skip retained.1E759 only516Data;16-byte suffix unknown. All four ready leads verified on exact supported proposals; earlier history preserved.

| Bank | Code | Data/image/text | Unknown |
|---|---:|---:|---:|
| header | 0 | 16 | 0 |
| PRG00 | 6556 | 800 | 836 |
| PRG01 | 6430 | 1561 | 201 |
| PRG02 | 1307 | 6839 | 46 |
| PRG03 | 6201 | 1991 | 0 |
| PRG04 | 5359 | 2457 | 376 |
| PRG05 | 3271 | 4770 | 151 |
| PRG06 | 0 | 8192 | 0 |
| PRG07 | 0 | 8192 | 0 |
| PRG08 | 0 | 8192 | 0 |
| PRG09 | 0 | 8192 | 0 |
| PRG10 | 740 | 7237 | 215 |
| PRG11 | 0 | 8192 | 0 |
| PRG12 | 2678 | 5072 | 442 |
| PRG13 | 0 | 8192 | 0 |
| PRG14 | 5096 | 1479 | 1617 |
| PRG15 | 5365 | 2537 | 290 |
| CHR00 | 0 | 8192 | 0 |
| CHR01 | 0 | 8192 | 0 |
| CHR02 | 0 | 8192 | 0 |
| CHR03 | 0 | 8192 | 0 |
| CHR04 | 0 | 8192 | 0 |
| CHR05 | 0 | 8192 | 0 |
| CHR06 | 0 | 8192 | 0 |
| CHR07 | 0 | 8192 | 0 |
| CHR08 | 0 | 8192 | 0 |
| CHR09 | 0 | 8192 | 0 |
| CHR10 | 0 | 8192 | 0 |
| CHR11 | 0 | 8192 | 0 |
| CHR12 | 0 | 8192 | 0 |
| CHR13 | 0 | 8192 | 0 |
| CHR14 | 0 | 8192 | 0 |
| CHR15 | 0 | 8192 | 0 |

[Exact unknown complement](round-003/unknown-complement.json):179 intervals/4174bytes; [counts](round-003/counts.json), [summary](round-003/summary.json). Code43003bytes;2420 classified regions with structural evidence (100% annotation coverage). Execution coverage unchanged4976 unique physical starts,4198 natural/3086 artificial overlapping; no new runtime claim. Representative Stage2/4/6 mechanics and THE END retained in scenarios.json; no unchanged replay needed. Authored source hashes preserved [audit](round-003/source-preservation.json), frozen/before/review/history unchanged. Current-source readability was verified for pass002 only; full current-source commentary required next.

Remaining4174bytes localized mixed player/actor/menu/fixed-helper fragments have unsupported entry/stack/mapping contexts or ambiguous code/record endpoints; these are questions for refreshed-complement redirect after commentary, not a terminal exhaustion claim. No supported proposal left pending in this batch. No toolkit issues found; [report](toolkit-issues.md). No toolkit edits, ROM mutation, cleanup, deletion, archive or publication.

## Round003 current-source readability complete — ROM incomplete

All2420 classified regions /257986 bytes, including43003 code bytes, are readable. Added203 slices /35743 bytes (3504Code,32239Data), preserving valid prior review and authored islands in prior-pass002.113 new code units explain finite power-selection steps, row/metatile transfers, HUD bands, secondary effects, packed-decimal arithmetic and camera/PPU updates.116 Data units explain established formats and bounded generic containers. Contextual variable names distinguish scene/HUD/secondary work and actor-owned extension storage. Source records all effective banks and material conditional return/selector contracts.

The original2043-region scaffold remains intact. Full current2420-region inventory is embedded in review.json.current_review, with current-review.json as a supporting copy;1063 semantic units, zero pending readability ranges.179 unknown intervals /4174 bytes have exact opaque notes. Unknown text/score identities, generic record schemas and natural selector use remain explained uncertainties, not skipped source. CA87 replaces temporary12/13 using saved RAM12; CA8A has separately restored0/1. Audio returns depend on RAM08; CD14 restores2/3. Partial actor5/5/6, active-secondary3 and HUD9 tables remain bounded. Sprite tile $FE skips drawing; it is not termination. Mixed7EA0 has explicit data/code/data mechanics without a live-caller claim.

Fresh verification ok=true/byte_identical=true; reading-after.asm refreshed. game.asm SHA25679e9d65097e1056277fff8e8d9aada4e6fe54d7e6cc4358de83443727b8b98eb; symbols.inc8dd0bd7bc93bbeebd153962e2df60777d68ec33b8101e8ae3e135ac0835aae14. Map/state SHA25612504fb181283bedd0308ead6ddd422844df02882751d05249744e47c8b82ea5. ROM unchanged. No runtime or structural map edits; final editorial/archive acceptance remains pending. No toolkit issues found.

See [checkpoint](../commentary/current-source-checkpoint.json), [handoff](handoffs/round-003-commentary.md), [reading copy](../commentary/reading-after.asm), [toolkit report](toolkit-issues.md).

## Round003 redirect active — pass003 input
Map/state12504fb181283bedd0308ead6ddd422844df02882751d05249744e47c8b82ea5; unknown4174/179, source79e9d65097e1056277fff8e8d9aada4e6fe54d7e6cc4358de83443727b8b98eb. Research analysis/redirect/round003-20261106. Fresh current-edge/return scans yield no unknown successor under their existing summaries; scope local only. Player partial5-word dispatch supplies3 setter targets into0000D7+27; independent directly referenced numeric/palette/animation fragment inspection active. No source/map changes or terminal claim. Current operation: bank-aware target decode and bounded consumer/neighbor noncode review.
Round003 redirect checkpoint2: targeted fresh24 post-JSR callee contracts7 returning,300 candidateCode bytes (not accepted); corrected earlier zero shortcut scope. Artifacts fresh-return-* under current attempt. Inspecting conditional indirect return witnesses, independent static actor/arithmetic contracts, and local numeric/palette noncode. MixedC460/C8BB now require code/data splits. No terminal claim.
Round003 redirect checkpoint3: connected848Code candidate bytes (overlap fresh300),141Data slices2763 bytes; provisional union3666 leaves508/23 fragments. Exact proposal-complement.json and candidate files durable. Independent short routine stack/mapping/semantic contracts active; no source/map edits, no terminal claim.
Round003 redirect checkpoint4: fixed candidate1128bytes including reusedconnected subset, independent input/IRQ/arithmetic/scene tails. Decimal-table corrected21bytes plus11byte clear helper. Active final mapper/CHR/static-wrapper contract audit; source/map unchanged. See findings checkpoint4.
Round003 redirect final combined proposal audit:4171 unique currentlyunknown bytes=1403Code+2768Data; accepted0; only3bytes01CD35..01CD38 would remain. Final candidate/context/overlap/direct-control audit durable; exact input179-range ledger and proposed complement. Coherent connected+independent investigation complete; ready for independent disassembly review, not terminal exhaustion. C519 saturatedincrement prose and power21-byte boundaries corrected before packaging. ROM/map/source/state unchanged.
Round003 redirect COMPLETE coherent batch ready:4171proposed1403Code2768Data accepted0. FinalD546 mapping corrected Resource4/init4/explicitMapMain0, exact candidate views4/5 beforeD55D and0/1 onward; provisionalResource0 superseded. Current4174unknown unchanged, ifaccepted3bytesCD25 remain. Handoff analysis/handoffs/round-003-redirect.md and final summaries/leads frozen. No terminal exhaustion claim. ROM/state/source/symbol hashes unchanged,8.4GiB free,no toolkit issues found.

## Round004 disassembly active
Input pass003 state12504fb181283bedd0308ead6ddd422844df02882751d05249744e47c8b82ea5; reviewing combined4171-byte proposal independently. Current operation: fresh configured decoding, exact physical views/control boundaries and data pattern/reference review. D546 Resource4/Main0 correction supersedes preliminaryResource0; CD25 threebytes stay unknown. Source/ROM preserved.

## Round004 disassembly complete — incomplete ROM

ROM classification: 100.00% (262157 / 262160 bytes classified; 3 bytes unknown).

Rounded100.00% is not completion: physical01CD35..01CD38 remains unknown. Accepted4171 formerly unknown bytes1403Code+2768Data in190 Static regions. All641 proposed instruction views independently fresh-decoded in48groups; exact physical identities, control boundaries, noncode patterns and primary routine/stack/mapping contracts checked. [Review decisions](round-004/review-decisions.md), [audit](round-004/independent-review.json), [fresh consumers](round-004/consumer-review.txt), [accepted slices](round-004/accepted-regions.json). All2420 old regions preserved exactly. Total2610regions,100% evidence annotation coverage; code44406bytes.

Address correction: physical01D557/CPU D547 JSR8000 usesResource4/5 selector0→824F; physical01D55A/CPU D54A JSRMapMain restores0/1 from physical01D55D/CPU D54D onward. Candidate views correct; redirect prose suffixes had mixed physical/CPU spaces. Historical artifacts preserved. IRQ CHR2..5 retainsPRG2/3; audio returns explicit Main shadow08; resource6 tails retain conditions. Partial player5word/3setter and7dispatch-return witnesses do not imply full selectors. Seven24-bit powers10..10^7 endphysical01F180, then11byte history-clear code. C519 increments/clamps20, no loop. Generic data supported by references/neighbors/patterns without invented schemas.

| Bank | Code | Data/image/text | Unknown |
|---|---:|---:|---:|
| header | 0 | 16 | 0 |
| PRG00 | 6862 | 1330 | 0 |
| PRG01 | 6430 | 1762 | 0 |
| PRG02 | 1353 | 6839 | 0 |
| PRG03 | 6201 | 1991 | 0 |
| PRG04 | 5362 | 2830 | 0 |
| PRG05 | 3271 | 4921 | 0 |
| PRG06 | 0 | 8192 | 0 |
| PRG07 | 0 | 8192 | 0 |
| PRG08 | 0 | 8192 | 0 |
| PRG09 | 0 | 8192 | 0 |
| PRG10 | 740 | 7452 | 0 |
| PRG11 | 0 | 8192 | 0 |
| PRG12 | 2707 | 5485 | 0 |
| PRG13 | 0 | 8192 | 0 |
| PRG14 | 5869 | 2320 | 3 |
| PRG15 | 5611 | 2581 | 0 |
| CHR00 | 0 | 8192 | 0 |
| CHR01 | 0 | 8192 | 0 |
| CHR02 | 0 | 8192 | 0 |
| CHR03 | 0 | 8192 | 0 |
| CHR04 | 0 | 8192 | 0 |
| CHR05 | 0 | 8192 | 0 |
| CHR06 | 0 | 8192 | 0 |
| CHR07 | 0 | 8192 | 0 |
| CHR08 | 0 | 8192 | 0 |
| CHR09 | 0 | 8192 | 0 |
| CHR10 | 0 | 8192 | 0 |
| CHR11 | 0 | 8192 | 0 |
| CHR12 | 0 | 8192 | 0 |
| CHR13 | 0 | 8192 | 0 |
| CHR14 | 0 | 8192 | 0 |
| CHR15 | 0 | 8192 | 0 |

[Counts](round-004/counts.json), [exact complement](round-004/unknown-complement.json), [summary](round-004/summary.json). State/current/[pass004map](../maps/pass-004.json) byte-identical SHA256d32f95247f3c547c153f2215edb55e3ad99070fb0dea4fe852c0f4952229a5f8; fresh [verification](pass-004-verification.json) ok=true/byte_identical=true. ROM unchanged b9ab5fbc92df5f496af543711adcdead16c02cbd08aca69f5f3a49d593a25c2f. [Source hashes](round-004/source-preservation.json) unchanged; frozen/before/review/history retained. Full current-source readability is verified for previous pass003 only; commentary must cover newly classified4171bytes, then redirect reviews strongest residual reasoning. No terminal exhaustion or final editorial claim.

Residual3bytes A001FF atphysical01CD35..01CD38/CPUCD25..CD28 have no fallthrough from preceding JMPMapMain and no known direct reference; LDY1+unofficial instruction versus short data remains ambiguous. All three R003 ready leads verified for exact combined batch; no unrestricted context/selector claim. No new runtime evidence:4976unique physical starts4198natural/3086artificial overlap unchanged; representative Stage2/4/6 mechanics/THE END retained, no unchanged replay. No toolkit issues found; [report](toolkit-issues.md). No toolkit edits, ROM mutation, cleanup, archive, deletion or publication.

## Round004 current-source readability complete — ROM incomplete

All 2610 classified regions / 262157 bytes, including 44406 code bytes, have current readable coverage. Reviewed all 190 new slices: 48 Code / 1403 bytes and 142 Data / 2768 bytes. Preserved the entire prior commentary in analysis/round-004/commentary/prior-pass003 and reused valid prior review. Added 57 code units and 142 data units; current inventory is embedded in review.json.current_review, with current-review.json supporting it. Original scaffold is unchanged; zero pending readability ranges. Final editorial/archive acceptance remains pending.

New source explains player motion setters and the partial five-word dispatch prefix, actor contact velocity/gravity, target-relative velocity, HUD threshold/reward work, scene camera initialization, meter drain/refill phases, IRQ/CHR updates, input debounce, indexed word arithmetic and spawn-history clearing. Data notes distinguish known signed words, collision probes, gravity increments, scene setup records, seven 24-bit decimal powers, and generic bounded noncode where exact schema remains unknown. No invented full selector domains or natural reachability claims.

The source explicitly corrects address spaces: physical01D557 / CPU D547 JSR8000 uses Resource4/5 selector0 to824F; physical01D55A / CPU D54A calls MapMain; physical01D55D / CPU D54D onward uses Main0/1. IRQ CHR2..5 preserves PRG2/3, audio restores Main shadow08, and resource6 tails retain their local contracts. C519 adds1 once and clamps20. Seven powers end at physical01F180, followed by an11-byte clear helper. Player and seven return witnesses remain partial contracts.

Only physical01CD35..01CD38 / CPU CD25..CD28 (A001FF) stays opaque. Preceding code tail-jumps to MapMain, and the next actor-clear entry is independent; no direct reference establishes code or data. This is an explained structural uncertainty, not a readability gap. Exact score/dialogue/effect identities, generic record schemas and natural selector production remain unresolved locally. Existing Stage2/4/6 and THE END evidence is retained; no new runtime replay.

Fresh configured-binary verification passed ok=true / byte_identical=true and reading-after.asm was refreshed. Map/state SHA256 d32f95247f3c547c153f2215edb55e3ad99070fb0dea4fe852c0f4952229a5f8; ROM unchanged. No structural map edits, toolkit changes, cleanup, deletion, archive or publication. No toolkit issues found. Next redirect must review the three-byte residual under evidence-policy; terminal editorial audit is reserved for final acceptance.

See [current checkpoint](../commentary/current-source-checkpoint.json) and [round004 handoff](handoffs/round-004-commentary.md).

## Round004 redirect — residual inference review complete

Checkpoint pass-004 remains unchanged (state/map SHA256 d32f95247f3c547c153f2215edb55e3ad99070fb0dea4fe852c0f4952229a5f8). Read-only configured decoder and current-map reference audit verify exact3 unknown bytes physical01CD35..01CD38, CPUCD25..CD28, fixed bank14. Corrected review establishes legal ISC overlap and an entirely official CD26 ORA prefix; neither supports a structural role without independent semantics. A001/FF subclaims and genericData were tested separately. No ready/proposed/accepted bytes. See [rejection review](redirect/round004-20261106/rejection-review.md), [primary audit](redirect/round004-20261106/primary-audit.json), [frontier](redirect/round004-20261106/frontier.json). Terminal incomplete rationale requires coordinator inspection; final editorial pending. All2610 classified regions remain readable and current verification byte-identical. Prior Stage2/4/6 mechanics and THE END retained without unchanged replay. No toolkit issues found; no ROM/map/source/toolkit edits, archive or publication.

Round004 redirect primary-evidence correction: coordinator detected local audit CPU/physical confusion in initial context-CD*.json (CPU+16 incorrectly selected bank6). Corrected audit.py uses16+14*8192+(CPU-C000); new context-corrected-CD*.json verify physical118069LDY1,118070ORA($FF,X),118071ISC00A9X,118072LDA0 with[2,3,14,15]. Invalid originals retained and flagged superseded. CLI/raw/map audit already addressed correctly. Inference unchanged but decisive citations corrected; coordinator inspection pending. This was a local analysis bug, not toolkit defect.

## Final editorial acceptance — classified source accepted, ROM incomplete

Accepted all2610 current classified regions /262157bytes,44406codebytes and1262 unique semantic units, zero pending readability. Reused valid review; fixed gravity-data unit reference, normalized source-relative review paths and linked existing semantic units to1785 previously empty baseline code entries. Preserved2043-entry scaffold identities and full current embedded inventory. Targeted source checks retain corrected C519/decimal-table/resource/Main/audio/IRQ contracts and honest partial selector/semantic limits. Source header and residual physical/CPU note corrected; no instruction bytes changed.

Supported commentary.py audit passed ok=true /byte_identical=true and refreshed reading-after.asm. Its2043regions/1785overlap-unit references describe frozen scaffold only; current-inventory-audit separately verifies2610current ranges,1262uniqueunits, source locations/physical blocks and bank views. Current map/state SHA256 d32f95247f3c547c153f2215edb55e3ad99070fb0dea4fe852c0f4952229a5f8 unchanged. Current source SHA256763ca2e85d249a1efe6365363715fa473ff532fe0fb00a38294cfd10b1e9a20f; symbols unchanged be3f6ffd5664f378f5cc7e3eb4da10f7d5d6e45757af4c431c6f070a99a60900. Exact3 unknown A001FF atphysical[01CD35,01CD38)/CPU[CD25,CD28),PRG14 remain unresolved; coordinator accepted terminal-incomplete rationale using corrected primary decodes. Whole-ROM classification remains incomplete. See commentary/editorial-audit.md, commentary/audit.json and analysis/final-editorial/current-inventory-audit.json. No toolkit issues found; no ROM/map/state/toolkit changes, archive, cleanup, deletion or publication. Coordinator document accounting/orchestration completion remains separate.


## Coordinator terminal handoff — justified incomplete ROM

Four sequential disassembly/commentary/redirect rounds completed with separate workers, followed by a separate final editorial worker. The coordinator inspected the corrected primary contextual decodes, neighboring source and scoped rejection reasoning; accepted terminal-incomplete rationale in [coordinator review](coordinator-rejection-review.md). The initial wrong-offset context artifacts remain preserved and explicitly invalidated. No corrected inference or useful discriminating next test remains on these premises; no actionable ready lead or external blocker. The residual remains resumable upon independent new structural evidence.

ROM classification: 100.00% (262157 / 262160 bytes classified; 3 bytes unknown).

Rounded100.00% is not ROM completion. Residual physical[01CD35,01CD38), fixedPRG14, CPU[CD25,CD28), A001FF. Final checkpointpass-004/current/state SHA256d32f95247f3c547c153f2215edb55e3ad99070fb0dea4fe852c0f4952229a5f8. Fresh configured store/map validation passes; checkpoint and final authored source rebuild byte-identically. All2610 classified regions and1262 unique semantic units have full current readability/final editorial acceptance; original frozen2043 scaffold/history preserved. Supplemental full-current inventory audit prevents a baseline-only conclusion. Source763ca2e85d249a1efe6365363715fa473ff532fe0fb00a38294cfd10b1e9a20f; symbolsbe3f6ffd5664f378f5cc7e3eb4da10f7d5d6e45757af4c431c6f070a99a60900.

[Terminal handoff](terminal-handoff.md), [completion audit](current-audit.json), [final editorial](handoffs/final-editorial.md), [editorial findings](../commentary/editorial-audit.md), [final verification](../commentary/verification.json), [reading copy](../commentary/reading-after.asm). Archive document contract inspected read-only: [options](../archive-options.json) explicitly account for all53 Markdown paths,34 additional final documents selected,9 identical copied instruction files omitted,0 unaccounted or missing paths. No final results omitted. No archive or cleanup executed.

Original ROM SHA256b9ab5fbc92df5f496af543711adcdead16c02cbd08aca69f5f3a49d593a25c2f unchanged. No toolkit issues found. No toolkit modification, workspace deletion, publication or external sharing. The orchestration reaches its documented terminal-incomplete handoff; ROM classification remains incomplete.
