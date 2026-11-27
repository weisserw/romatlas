# Investigation ledger

## Round001 disassembly

ROM classification: 56.79% (148886 / 262160 bytes classified; 113274 bytes unknown).

- Verified baseline and pass-001 checkpoint; exact counts/per-bank totals in progress.md and round001/counts.json; exact unknown complement round001/unknown-ranges.json. All current classifications have evidence and internal validation; assembly byte-identical.
- Vectors physical131082..131087 decode NMI FF00/reset C100/IRQ FF00. Initial state confirms reset C100 under [0,1,14,15]; inspected reset setup and MMC1 reset write. Static vector traversal stops conservatively at controller writes. runtime bank contexts preserved per accepted fetch.
- Hardware CHR section131088..262159 is 128KiB pattern graphics; header16 bytes and vector6 bytes independently bounded.
- Seven maximal uniform FF runs (round001/uniform-runs.json) are generic Data, not code. Nearby mixed records/code/vectors and exact change boundaries reviewed, no conflicting observed instruction regions. Specific purpose remains unresolved without impairing non-code classification.
- Bank8 physical74688 BIT $04A5 and74689 LDA $04 are both observed entries. The three primary BIT instruction bytes cover the alternate operand-entry path; primary code plus explicit skip-trick comment preserves classification and exact assembly. Local overlap decode artifacts retained.
- Natural action/menu/dialogue routes and artificial route-index edits recorded in scenarios.json; scripts preserve releases, budgets and trace configuration. $0342 display consumer in bank4 proves zero-based displayed route; independent patches10/42 plus map resume do not change terrain. This falsifies only that transition recipe, not all stage edits.
- Natural traversal reached Tom Guycot dialogue (not death, corrected after frame inspection). Four A2/release5 advances remain at dialogue. L002 changes method to inspect input/engine before any retry.
- Initial generic discover parameter budget was ignored by NES adapter; documented/source-confirmed NES max_instructions subsequently used. Invalid annotation attempts (missing/nonnull IDs, Graphics instead of supported Image) changed no accepted classifications; corrected forms validated and checkpointed. Errors are local usage, not toolkit defects.

Next: L001 genuine route load contract and late milestones, L002 text/control flow, L003 bank-aware dispatch/static closure. These are easy-gain handoff frontiers, not terminal rejected candidates. No whole-ROM exhaustion or full compatibility claim. No toolkit issues found: toolkit-issues.md.

## Round002 independent disassembly

ROM classification: 76.27% (199961 / 262160 bytes classified; 62199 bytes unknown).

Accepted51075bytes after independent consumer/context/predecessor/boundary review; see round002/review.md, static-review.json, continuation-review.json, epilogue-pool-proof.json and counts.json. Exact363gap complement: round002/unknown-ledger.json. Current checkpoint pass-002-final, validation success and byte-identical rebuild report pass-002-final-verification.json. L001–L006 verified; L007 open for remaining selector/entry-contract and mixed-gap work. No experiments repeated and no unsupported reachability claims. Original commentary preserved; updated commentary merge and editorial audit pending. Per-bank table in progress.md; no toolkit issues found (toolkit-issues.md).

## Round003 independent disassembly

ROM classification: 88.19% (231210 / 262160 bytes classified; 30950 bytes unknown).

Accepted31249static structuralbytes after fresh10302contextual decodes and consumer/boundary/conditional-contract review. Review: round003/review.md; exactcounts/perbank counts.json and progress.md; exactcomplement round003/unknown-ledger.json. L007/L008verified; L009open for local indirectbuffer/nestedbank-frame/mixedboundary facts. Checkpoint pass-003-final validates and rebuilds byte-identically (pass-003-final-verification.json). Execution coverage unchanged, saved progression reused. All prior evidence/source retained; fullcurrent-source commentary and EB2Dcorrection next; finaleditorial pending. No terminalexhaustion claim. Toolkit:1missingfeature TK001,0bugs/0blockers (toolkit-issues.md).

## Round004 independent disassembly

ROM classification: 92.52% (242561 / 262160 bytes classified; 19599 bytes unknown).

L009verified:11351Static structuralbytes (+1138Code/+10213Data),569fresh decodes;19477instructionviews/5151directtargets zeroData conflicts;137266additionalcarry1functionalchecks pass. Primary review round004/review.md and exactcounts round004/counts.json; complement round004/unknown-ledger.json. map.validate and pass-004-final byte-identical checkpoint pass. AlternateBIT/ADC preserved; malformedhelper28bytes remainunknown; sound23first/repeat scopes retained; PPUstreams classified independently of origincode. No new runtime observations. L010local pointer/frame/bank/audio/mixedboundary facts after fullcurrent-source commentary; no terminalexhaustion. Prior reviewed commentary SHA unchanged, all evidence retained. Toolkit existingTK001,0newbugs/0blockers (toolkit-issues.md).

|Bank|Code|Data/image/header|Classified|Unknown|
|---|---:|---:|---:|---:|
|Header|0|16|16|0|
|PRG 00|75|8117|8192|0|
|PRG 01|334|7858|8192|0|
|PRG 02|2731|4474|7205|987|
|PRG 03|0|8192|8192|0|
|PRG 04|1806|5085|6891|1301|
|PRG 05|2284|1177|3461|4731|
|PRG 06|612|7252|7864|328|
|PRG 07|2424|2988|5412|2780|
|PRG 08|1642|6524|8166|26|
|PRG 09|1045|7147|8192|0|
|PRG 10|5780|633|6413|1779|
|PRG 11|6701|863|7564|628|
|PRG 12|6879|502|7381|811|
|PRG 13|4430|933|5363|2829|
|PRG 14|1929|4041|5970|2222|
|PRG 15|5437|1578|7015|1177|
|CHR 00|0|8192|8192|0|
|CHR 01|0|8192|8192|0|
|CHR 02|0|8192|8192|0|
|CHR 03|0|8192|8192|0|
|CHR 04|0|8192|8192|0|
|CHR 05|0|8192|8192|0|
|CHR 06|0|8192|8192|0|
|CHR 07|0|8192|8192|0|
|CHR 08|0|8192|8192|0|
|CHR 09|0|8192|8192|0|
|CHR 10|0|8192|8192|0|
|CHR 11|0|8192|8192|0|
|CHR 12|0|8192|8192|0|
|CHR 13|0|8192|8192|0|
|CHR 14|0|8192|8192|0|
|CHR 15|0|8192|8192|0|

## Round005 independent disassembly — ROM incomplete

ROM classification: 94.02% (246470 / 262160 bytes classified; 15690 bytes unknown).

Input pass-004-final -> fresh pass-005-final. L010 verified in full:3909 new Static structural bytes (2467Code/1442Data),104annotations (69Code groups/35Data). Fresh1082contextual instruction checks and selected-context corroboration;20551unique instruction views/5404direct control targets, zero control-to-Data conflicts. Physical identity/bounds and immutable Data bytes independently checked. Primary decisions analysis/round005/review.md, structural-review.json, consumers.json, post-selector-review.json and control-audit.json. No new Observed execution or natural caller/selector evidence.

C9E4 return is service4 only, matched select/pop under explicit mode3/current-shadow/noncollision stacks. E06C has two intact frames, two inline bytes and JSR+5 continuation; independent actual streams48/33bytes. Two local control writes preserve mode3. CB43 status precondition A0>=2/health<=51 is feasible structural entry, restricted to45467/60743; unrestricted underflow/selector overwrite counterexamples retained. Six widgets5..0, not seven. Wrapper structural role separate from downstream return and selected contexts separate from entry. Complete standalone PPU/OAM/actor/clamp/subtract mechanisms classified independently of natural caller. Generic noncode after bounded role inspection does not claim full record layouts. Weak100592..100597 and four malformed7byte fragments remain Unknown.

map.validate passed; state saved and current map exported. Fresh checkpoint maps/pass-005-final.json, generated/pass-005-final/game.asm and analysis/pass-005-final-verification.json validate and rebuild byte-identically. Current map equals checkpoint. Exact counts/kind/provenance/per-bank in round005/counts.json; complement 178intervals in round005/unknown-ledger.json. Counts use unique evidence-backed nonUnknown physical bytes, including header and CHR; raw fallback excluded. Execution coverage unchanged. Broad later routes/mechanics/final/THE END/artificial and natural dialogue scenarios retained without unchanged replay. Ordinary combined supported batch finished, no terminal exhaustion claim.

|Bank|Code|Data/image/header|Classified|Unknown|
|---|---:|---:|---:|---:|
|Header|0|16|16|0|
|PRG 00|75|8117|8192|0|
|PRG 01|334|7858|8192|0|
|PRG 02|2731|4490|7221|971|
|PRG 03|0|8192|8192|0|
|PRG 04|1806|5085|6891|1301|
|PRG 05|2804|1251|4055|4137|
|PRG 06|759|7433|8192|0|
|PRG 07|3404|3317|6721|1471|
|PRG 08|1642|6530|8172|20|
|PRG 09|1045|7147|8192|0|
|PRG 10|5834|729|6563|1629|
|PRG 11|6880|863|7743|449|
|PRG 12|6903|514|7417|775|
|PRG 13|4444|970|5414|2778|
|PRG 14|2358|4679|7037|1155|
|PRG 15|5557|1631|7188|1004|
|CHR 00|0|8192|8192|0|
|CHR 01|0|8192|8192|0|
|CHR 02|0|8192|8192|0|
|CHR 03|0|8192|8192|0|
|CHR 04|0|8192|8192|0|
|CHR 05|0|8192|8192|0|
|CHR 06|0|8192|8192|0|
|CHR 07|0|8192|8192|0|
|CHR 08|0|8192|8192|0|
|CHR 09|0|8192|8192|0|
|CHR 10|0|8192|8192|0|
|CHR 11|0|8192|8192|0|
|CHR 12|0|8192|8192|0|
|CHR 13|0|8192|8192|0|
|CHR 14|0|8192|8192|0|
|CHR 15|0|8192|8192|0|

Prior current commentary source preserved SHA256 4d36606264ee0634a7e0ae3506dad5908766e6cc2306922706d9313b332c8fc2, backed up round005/commentary-input.asm; all review history retained. Full refreshed commentary next; final editorial pending. Required corrections:106235 six widgets5..0 and A0/9F caveat;116263..116291 seven fillvectors consumed60532;116135..11620772ROMtile rectangle consumed60470. No current-source readability completion claim for new bytes. L011 exact residual local contracts open. Toolkit: existing TK001 missing atomic batch annotation;0newbugs/0blockers; analysis/toolkit-issues.md. Disk inspected ~1.9GiB free. No cleanup/deletion, toolkit edit, archive or publication. ROM unchanged.

## Round006 independent disassembly — ROM incomplete

ROM classification: 95.00% (249050 / 262160 bytes classified; 13110 bytes unknown).

Input pass-005-final -> fresh pass-006-final. L011 verified in full:2580 new Static structural bytes (2317Code/263Data),24new annotations (18Code groups/6Data), plus same-id historical map-comment/evidence repair. Fresh1020contextual instruction checks;21571unique instruction views/5656direct control targets, zero control-to-Data conflicts. Full physical identity/bounds/known-overlap and split Code/Data checked. Independent decisions round006/review.md; fresh reviewed-instructions.json, structural-review.json, consumer-code.txt, mechanical-checks.json and control-audit.json. No new Observed execution or natural caller/selector evidence.

Complete324byte relocated PPU family matches accepted source except11local operand high bytes;308Code/16Data. Explicit A0..3, finite records/two intact frames, mode3 and noncollision stacks retained. All48table-selected callbacks117866..118931 form coherent complete gate/update/reward mechanisms including bounded loops and table-family trivial stubs; downstream CABA/CA3C return stays conditional. Paired menu/dialogue restoration tails match saved0336/PHA/software-bank prefixes. Seven small algorithms/setup/wrappers mechanically supported, selected124480tuple8/9 fresh. Actual dispatch ASL/TAX overwrites X26/28:27bytecopy/leadingzero boundA3..BD, not intendedtwo-digit semantics. Thousands0..65, modulo0..132only. Complete814byte password/sprite/text graph and247Data plus16cloneData supported; generic pattern roles retain unresolved dynamic domains/original ownership. Five exact terminated streams and duplicate pointer origins independently checked.

Mandatory existingData id14401 offset119027 length282 comment/evidence corrected from246RAMtiles to246ROMtiles atCPU D107/physical119063..119309. Immediate03=07low/04=D1high, sixwidth×height products246 independently verified; no role/extent change. Round006/map-comment-repair.json preserves original and corrected records; primary round005/commentary-rectangle-correction.json. Original erroneous redirect/round004/rectangle-proof.json and round005-handoff.md retained unchanged, their reversed pointer premise superseded.

map.validate passed; state saved and current map exported. Fresh maps/pass-006-final.json equals maps/current.json. Generated/pass-006-final/game.asm rebuilds byte-identically; analysis/pass-006-final-verification.json. Exact unique-byte classification/kind/provenance/per-bank round006/counts.json; complement 163intervals round006/unknown-ledger.json. Header/CHR included, raw fallback excluded. Execution coverage unchanged. Broad saved later-level/mechanic/artificialending/THE END and natural dialogue scenarios reviewed and retained; no changed contract justifies replay. Ordinary full connected batch complete, no exhaustive recursive closure/terminal exhaustion claim.

|Bank|Code|Data/image/header|Classified|Unknown|
|---|---:|---:|---:|---:|
|Header|0|16|16|0|
|PRG 00|75|8117|8192|0|
|PRG 01|334|7858|8192|0|
|PRG 02|2731|4490|7221|971|
|PRG 03|0|8192|8192|0|
|PRG 04|1806|5085|6891|1301|
|PRG 05|2804|1251|4055|4137|
|PRG 06|759|7433|8192|0|
|PRG 07|3404|3317|6721|1471|
|PRG 08|1642|6530|8172|20|
|PRG 09|1045|7147|8192|0|
|PRG 10|5834|729|6563|1629|
|PRG 11|6880|863|7743|449|
|PRG 12|7449|570|8019|173|
|PRG 13|4737|1161|5898|2294|
|PRG 14|3512|4679|8191|1|
|PRG 15|5881|1647|7528|664|
|CHR 00|0|8192|8192|0|
|CHR 01|0|8192|8192|0|
|CHR 02|0|8192|8192|0|
|CHR 03|0|8192|8192|0|
|CHR 04|0|8192|8192|0|
|CHR 05|0|8192|8192|0|
|CHR 06|0|8192|8192|0|
|CHR 07|0|8192|8192|0|
|CHR 08|0|8192|8192|0|
|CHR 09|0|8192|8192|0|
|CHR 10|0|8192|8192|0|
|CHR 11|0|8192|8192|0|
|CHR 12|0|8192|8192|0|
|CHR 13|0|8192|8192|0|
|CHR 14|0|8192|8192|0|
|CHR 15|0|8192|8192|0|

Previously fully reviewed commentary source preserved SHA256 ad192b9a17c0ca30a733177f589b616caab214c11454c177acf6fa5e2633ac99, backup round006/commentary-input.asm; current reading-after.asm and all history untouched. Full refreshed commentary next, with normal merge/rebaseline required for new map and repaired comment/evidence. Final editorial pending; no new current-source readability completion claim. L012 open for exact residual local contracts; weak100592..597 and four malformed7byte fragments remain Unknown. Toolkit: existing TK001 missingatomicbatchannotation,0newissues/0blockers; [analysis/toolkit-issues.md](toolkit-issues.md). Disk inspected1.8GiBfree; no scratch directory existed. No cleanup/deletion, toolkit edit, archive, publication or ROM change.

## Round007 independent disassembly — ROM incomplete

ROM classification: 99.43% (260667 / 262160 bytes classified; 1493 bytes unknown).

Input pass-006-final -> fresh pass-007-final. L012 verified in full:11617 new Static structural bytes (10759 Code/858 Data),119 annotations (78 Code groups/41 Data). Fresh4570 contextual instruction checks;26141 total instruction views/7023 direct control targets, zero control-to-Data conflicts. Every new direct/pool target is an actual instruction start. Eleven E06C inline words freshly parsed to finite already-classified streams; all resume starts checked. Full physical identity/bounds/known-overlap and split Code/Data checked. Independent decisions [round007/review.md](round007/review.md); structural-review.json, reviewed-instructions.json, consumers.json, mechanical-checks.json and control-audit.json. No new Observed execution/natural caller or selector claim.

Important correction: redirect menu-grid-domain.json modeled the shop cursor loop incorrectly. Exact bytes47785..47788 are JMP BA89, bypassing the LSR BA87 after the first iteration. Independent enumeration gives16terminating Y inputs/240repeating inputs, not30/226. Y=$20/$40/$60/$80 produce rows0/5/10/15; no universal three-row quantizer. Complete routine role and precise Code/Data bounds remain supported. Primary redirect proofs are preserved; round007 mechanical-checks/review supersede only the erroneous numeric-domain claim.

Accepted combined producer/password/actor/timer/contact/probe/queue, numeric/menu/six-slot/shop/item, ending/manual-frame/epilogue and scene/debug graphs. Five pose bytes have supported0..4producer/consumer contract; five actual passwordRAMfield pointers bound writes. Actual synthetic savedreturn A408 resumesA409; physical58392 remainsUnknown. E0DC edits E06C caller's savedreturn+2, not outer synthetic frame. Timer PLA/PLA discards actual JSR8DD7 frame. Fixed FDE9 calls bank13 BDCE physical114142 under[12,13,14,15]. Four malformed7byte and two46byte fragments stayUnknown. Generic noncode patterns accepted after bounded neighbor/consumer/role inspection; no invented full schemas or universal dynamic domains. Conditional matched normal-call/valid-record/mode3/noncollision stack prerequisites retained independently of natural reachability.

map.validate passed; state saved/currentmap exported. Fresh maps/pass-007-final.json equals maps/current.json. generated/pass-007-final/game.asm rebuilds byte-identically; [pass-007-final-verification.json](pass-007-final-verification.json). Exact unique-byte kind/provenance/perbank counts round007/counts.json; exact complement111intervals round007/unknown-ledger.json. Header/CHR included; raw fallback excluded. All historical region metadata compared byte-for-byte by region ID to pass-006-final, unchanged, including repaired D107/246ROMtile evidence. Execution coverage unchanged. Saved broad later-level/mechanic/artificial ending/THE END/natural dialogue scenarios reviewed; no changed practical contract warrants replay. Ordinary complete supported batch, no exhaustive recursive closure or terminal exhaustion claim.

|Bank|Code|Data/image/header|Classified|Unknown|
|---|---:|---:|---:|---:|
|Header|0|16|16|0|
|PRG 00|75|8117|8192|0|
|PRG 01|334|7858|8192|0|
|PRG 02|3678|4498|8176|16|
|PRG 03|0|8192|8192|0|
|PRG 04|2898|5288|8186|6|
|PRG 05|6316|1683|7999|193|
|PRG 06|759|7433|8192|0|
|PRG 07|4736|3365|8101|91|
|PRG 08|1642|6530|8172|20|
|PRG 09|1045|7147|8192|0|
|PRG 10|7161|804|7965|227|
|PRG 11|7158|875|8033|159|
|PRG 12|7449|575|8024|168|
|PRG 13|6679|1220|7899|293|
|PRG 14|3512|4679|8191|1|
|PRG 15|6210|1663|7873|319|
|CHR 00|0|8192|8192|0|
|CHR 01|0|8192|8192|0|
|CHR 02|0|8192|8192|0|
|CHR 03|0|8192|8192|0|
|CHR 04|0|8192|8192|0|
|CHR 05|0|8192|8192|0|
|CHR 06|0|8192|8192|0|
|CHR 07|0|8192|8192|0|
|CHR 08|0|8192|8192|0|
|CHR 09|0|8192|8192|0|
|CHR 10|0|8192|8192|0|
|CHR 11|0|8192|8192|0|
|CHR 12|0|8192|8192|0|
|CHR 13|0|8192|8192|0|
|CHR 14|0|8192|8192|0|
|CHR 15|0|8192|8192|0|

Prior fully reviewed commentary source preserved SHA256 5e8c291d908d7660f0475b4569d0cabe7587ac3ea53d695ddd5e19388e9993ab, backed up round007/commentary-input.asm; reading-after.asm and history untouched. Full refreshed commentary next; final editorial acceptance pending. New source readability is not yet complete. L013 open for localized residual roles; preserve shop-loop arithmetic correction during commentary and D107ROMrectangle metadata. Toolkit: existing TK001 missing atomic batch annotation;0newissues/0blockers; [analysis/toolkit-issues.md](toolkit-issues.md). Disk inspected~1.6GiBfree, no scratch directory created. No toolkit edit, cleanup/deletion, archive, publication or ROM change.

## Round008 independent disassembly — ROM incomplete

ROM classification: 99.98% (262106 / 262160 bytes classified; 54 bytes unknown).

Input pass-007-final -> fresh pass-008-final. Independently accepted1439 Static bytes (1306 Code/133 Data),550 fresh contextual decodes and109 annotations; exact physical/control/pool/frame/mask/footer contracts. Fixed contact/queue joins resolve to bank13 physical114126/114239. No new Observed/natural reachability claim. Every new direct/pool target is an actual instruction start; zero new direct-to-Data or existing-control-to-new-Data conflicts. Independent review [round008/review.md](round008/review.md), structural-review.json, consumers.json and mechanical-checks.json. Official EOR/CMP actual actor algorithms accepted without missing-LDA premise; queue copies require valid list capacity and matched projection return. Inventory four-byte and sprite two-byte stack restorations checked; nonlocal helper frames remain distinct from normal continuation. Reward default01E4=484 supplies amounts to C443, not OAM pointers. Inline pools support generic Data without full original schema; direction masks40/80 cover supported facing1/2 only; aligned paired banktails support generic Data without checksum or bank13 vector claim.

map.validate passed; analysis/state.json saved and maps/current.json exported. Fresh maps/pass-008-final.json equals current map; generated/pass-008-final/game.asm rebuilds byte-identically: [pass-008-final-verification.json](pass-008-final-verification.json). Exact counts [round008/counts.json](round008/counts.json) and14-interval54-byte complement [round008/unknown-ledger.json](round008/unknown-ledger.json). All historical annotations compared by ID to exact prepass internal state, unchanged, including repaired D107246-ROMtile and shop16terminating/240repeating evidence. ROM SHA unchanged. No new execution coverage. Prior broad later-route/mechanic/artificial ending/THE END/natural dialogue scenarios reviewed and retained; no changed practical contract justifies replay. Easy ready batch exhausted; terminal rejection review belongs to subsequent redirect and no terminal exhaustion claimed.

|Bank|Code|Data/image/header|Classified|Unknown|
|---|---:|---:|---:|---:|
|Header|0|16|16|0|
|PRG 00|75|8117|8192|0|
|PRG 01|334|7858|8192|0|
|PRG 02|3694|4498|8192|0|
|PRG 03|0|8192|8192|0|
|PRG 04|2902|5288|8190|2|
|PRG 05|6505|1683|8188|4|
|PRG 06|759|7433|8192|0|
|PRG 07|4826|3365|8191|1|
|PRG 08|1662|6530|8192|0|
|PRG 09|1045|7147|8192|0|
|PRG 10|7297|894|8191|1|
|PRG 11|7301|891|8192|0|
|PRG 12|7614|577|8191|1|
|PRG 13|6942|1235|8177|15|
|PRG 14|3512|4679|8191|1|
|PRG 15|6490|1673|8163|29|
|CHR 00|0|8192|8192|0|
|CHR 01|0|8192|8192|0|
|CHR 02|0|8192|8192|0|
|CHR 03|0|8192|8192|0|
|CHR 04|0|8192|8192|0|
|CHR 05|0|8192|8192|0|
|CHR 06|0|8192|8192|0|
|CHR 07|0|8192|8192|0|
|CHR 08|0|8192|8192|0|
|CHR 09|0|8192|8192|0|
|CHR 10|0|8192|8192|0|
|CHR 11|0|8192|8192|0|
|CHR 12|0|8192|8192|0|
|CHR 13|0|8192|8192|0|
|CHR 14|0|8192|8192|0|
|CHR 15|0|8192|8192|0|

Prior fully reviewed commentary/source/game.asm SHA fe25b1789e22910c7dc47a26e25d26c6cb4b86ee6e249125c69a2d729e688b2d preserved and backed up round008/commentary-input.asm; reading-after.asm and history unchanged. Full refreshed current-source commentary next; maintain review locations relative to commentary/source (game.asm:line). Final editorial acceptance pending. L013 verified; L014 open for exact residual reasoning after commentary. Toolkit: existing TK001 missing atomic batch annotation;0newissues/0blockers; [analysis/toolkit-issues.md](toolkit-issues.md). Disk inspected2.9GiB free; no scratch cleanup/deletion, toolkit edits, archive, publication or ROM change.

## Round009 independent disassembly — ROM incomplete

ROM classification: 99.99% (262140 / 262160 bytes classified; 20 bytes unknown).

Input pass-008-final -> fresh pass-009-final. L014 verified:34 new Static Code bytes in five suffixes,16 fresh instructions, five annotations. Independently reviewed full useful supplied-A MMIO/shadow commit and actor-field contracts, exact banks [12,13,14,15], matched caller/nested frames and coherent entry state. No natural caller, execution or missing original opcode claim. Fresh primary decodes and neighbor/setter consumers: [round009/review.md](round009/review.md), reviewed-instructions.json, consumers.json and reviewed-code.txt. Four leading2B bytes and two leading5C03 bytes remain Unknown. Connected setter and neighbors already classified; no unsupported closure. No new execution coverage. Prior broad later-game/mechanics/artificial ending/THE END/natural-dialogue scenarios reviewed; unchanged practical contracts do not justify repetition. No terminal exhaustion claim.

map.validate passed; state saved/currentmap exported. Fresh maps/pass-009-final.json equals maps/current.json; generated/pass-009-final/game.asm byte-identical rebuild passed [pass-009-final-verification.json](pass-009-final-verification.json). Exact whole-ROM unique-byte counts/per-bank complements [round009/counts.json](round009/counts.json);20 unknown bytes/14 intervals [round009/unknown-ledger.json](round009/unknown-ledger.json). Historical annotations unchanged by ID, including D107/246-ROMtile and corrected shop16terminating/240repeating metadata. Input ROM SHA unchanged.

|Bank|Code|Data/image/header|Classified|Unknown|
|---|---:|---:|---:|---:|
|Header|0|16|16|0|
|PRG 00|75|8117|8192|0|
|PRG 01|334|7858|8192|0|
|PRG 02|3694|4498|8192|0|
|PRG 03|0|8192|8192|0|
|PRG 04|2902|5288|8190|2|
|PRG 05|6505|1683|8188|4|
|PRG 06|759|7433|8192|0|
|PRG 07|4826|3365|8191|1|
|PRG 08|1662|6530|8192|0|
|PRG 09|1045|7147|8192|0|
|PRG 10|7297|894|8191|1|
|PRG 11|7301|891|8192|0|
|PRG 12|7614|577|8191|1|
|PRG 13|6954|1235|8189|3|
|PRG 14|3512|4679|8191|1|
|PRG 15|6512|1673|8185|7|
|CHR 00|0|8192|8192|0|
|CHR 01|0|8192|8192|0|
|CHR 02|0|8192|8192|0|
|CHR 03|0|8192|8192|0|
|CHR 04|0|8192|8192|0|
|CHR 05|0|8192|8192|0|
|CHR 06|0|8192|8192|0|
|CHR 07|0|8192|8192|0|
|CHR 08|0|8192|8192|0|
|CHR 09|0|8192|8192|0|
|CHR 10|0|8192|8192|0|
|CHR 11|0|8192|8192|0|
|CHR 12|0|8192|8192|0|
|CHR 13|0|8192|8192|0|
|CHR 14|0|8192|8192|0|
|CHR 15|0|8192|8192|0|

Prior fully reviewed commentary/source/game.asm SHA b40d7ae1d434138052b430100299b381146a8102ac79de9eeff6e497b31ba42f preserved with round009/commentary-input.asm; reading-after.asm/history retained. Full refreshed current-source commentary next; review locations relative to commentary/source, game.asm:line. Final editorial acceptance pending. L015 open for localized residual reasoning after commentary; ROM incomplete. Toolkit: existing TK001 atomic batch feature request,0newissues/0blockers; [analysis/toolkit-issues.md](toolkit-issues.md). Disk inspected2.7GiBfree, scratch92KiB. No cleanup/deletion, toolkit edits, archive or publication. All server subprocesses exited; no live writer.
