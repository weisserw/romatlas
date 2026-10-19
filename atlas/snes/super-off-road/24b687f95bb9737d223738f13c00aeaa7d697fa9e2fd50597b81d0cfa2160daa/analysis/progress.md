# Disassembly progress

Workspace initialized; baseline verified. Round 1 disassembly active; classification gathering in progress.

## Round 1 — disassembly in progress
Baseline `generated/baseline` verified byte-identical; report `analysis/baseline-verification.json`. Initial trace detail capacity overflowed as expected; aggregate contexts retained. First session ended before snapshot export; restart replay underway using detached bridge; durable RPC log records completed requests. Black frame73 was audio loading; frame ~190 showed licensed title splash.

### Batch 1: reset/splash structural code
Inspected `analysis/evidence/discovery-363-listing.txt` and confirmed 26 exact-decode code regions, 1567 bytes. Static contexts supplemented by natural opcode observations; classifications deliberately use Static rather than claiming all candidates executed. Roles include native reset/PPU, frame fades, bounded VRAM/CGRAM loads, licensed splash, frontend initialization and two SPC upload streams. `map.validate`, map.save and map.export all returned `{ok:true}`. Unsupported continuations remain unknown. Runtime: 0→240 no buttons; Start240–243/release363 occurred during credits; untraced wait363–603; Start603–605/release625; no buttons625–655 reached two-driver selection with on-screen PUSH SELECT. Completed calls and exact budgets in analysis/scenarios.jsonl; RPC evidence includes initial timeout and aggregate drops. First 240-frame restart replay trace differs only instrumentation from old session. Baseline byte-identical already verified; classification checkpoint pending end of pass.

### Batch 2: frontend/NMI code
Captured natural frontend contexts and bounded discovery in `analysis/evidence/frontend-contexts.json` / `frontend.json`; inspected complete `frontend-listing.txt`. Added 1706 code bytes (total3273), preserving earlier regions. map.validate passed and internal/portable maps saved. New roles: vblank frame/OAM service, sprite DMA/packing, driver selection/name-entry handlers and text helpers. Select at655 was too early during setup; Select720–724/release739 successfully joined driver1 and opened alphabet grid. No state patches. Toolkit TI-001 recorded: SNES discovery lacks paging under1MiB cap;20-root/1000-instruction chunks work around it.

### Batch 3: consumer-bounded frontend data
Added38958 data bytes: validated SNES header/vectors; fixed-count font/sprite/VRAM/CGRAM spans; DMA $098000+$4DA0 and palette sources; ten-word rectangle descriptors with exact width×height map extents; fixed-count initialization tables. Evidence and physical bounds encoded in every region via `analysis/tools/data_batch.py`, sourced to inspected loaders. All map operations validated/saved/exported. Current code3273/data38958 =42231 classified bytes; unknown482057. Name-entry experiments ongoing: Select joins, B changes selection state but has not yet finished name entry.

### Batch 4: SPC uploaded-code classification
Added181 name-handler code bytes, then1486 SPC700 code bytes and16 upload-header bytes. Two uploader streams independently parsed: ROM070004..077E03→ARAM0400..81FF, terminating startFFC0; ROM078004..07FE03→ARAM8200..FFFF terminating start0400. Main uploader source/length/destination mechanics inspected; root0400 static SPC discovery terminates without budget exhaustion or diagnostics. Source/listing `analysis/evidence/spc-discovery-listing.txt`; ten Code views explicitly cpu=spc700/uploaded=true with ARAM aliases and storage bounds. Remaining payload bytes remain unknown rather than pretending every upload byte is code or sample data. map.validate/save/export passed. Natural gameplay reached Speed Shop at1217 after AAA name/EXIT confirmation.

### Batch 5: name/shop controls and sampled runtime code
Added449 observed-validated code bytes from shop sample (stored conservatively Static); exact source `analysis/evidence/shop-observed-listing.txt`, all individual opcode decode checks in RPC evidence. map.validate/save/export passed. Natural B1222–1225 purchased Nitro ($10000); Start did not exit. Ordinary cursor route Down1503–1506/release5, Down1511–1514/release5, Right1519–1522/release5, B1527–1530/release30 selected Start Next Race and displayed GET READY. Trace sampling across untraced waits avoids expensive repeated instruction histories; execution coverage excludes those waits.

### Batch 6: race physics/AI/display
Natural race input: B1740–1760 during countdown, untraced B1760–1940; traced B1940–1950, B+Y1950–1958, B+Right1958–1968. New unique traced instruction locations2595→4793 (all spaces; ROM contexts retained separately). Inspected2657 new-listing lines (`analysis/evidence/racing-new.txt`) and exact decode context corpus; added4982 code bytes with roles covering race scheduler, checkpoint/AI steering, collision/terrain interpolation, wheel suspension/rendering, acceleration/nitro/steering, HUD and pickups. map.validate/save/export passed. At3416 sample after1200 untraced acceleration frames only45 new locations, indicating same-track gains diminishing; continue toward results and an obvious next-track/attract alternative. No artificial state.
Intermediate checkpoint `round01-startup` verified byte-identical before this batch; final checkpoint pending.

### Batch 7: static race branch closure
Derived27 unknown direct-target roots using individually decoded observed entry contexts; static-only bounded discovery inspected in `analysis/evidence/racing-static-new.txt`. Added1171 bytes of unexecuted supported collision branches, lap-completion trigger, pause/start handling, ranking and object animation paths. All map checks/save/export passed. Alternative overlapping BIT/LDA entry starts near physical001792..001799 and0017FB..001802 excluded from automatic merging; existing primary view remains valid but alternate execution views need targeted editorial/structural follow-up. They are code, not data. Natural sample at5236 after1800 untraced B frames added only4 instruction locations; results not yet reached. Long foreground route command stopped after the completed1800-frame segment; standalone follow-up captured sample without assuming later queued steps occurred.

### Batch 8: bounded tables and finish comparison
Added253 data bytes: six countdown-effect words,31 countdown/OAM selector words, two32-word heading vectors, SPC DSP register list25+sentinel and25 values. Exact consumer/producer/domain evidence in comments; map.validate/save/export passed. Natural frame6461 snapshot exported. Inspection revealed DP2D was ALREADY2 and blue AI lap5, so natural winner prerequisite was met; artificial R1-L001 patch DP2D=1/WRAM04E5=5 changed winner state but is unnecessary for transition. Saved artificial context corpus then restored `analysis/evidence/natural-race-late.json` (artificial=false) and repeated30-frame natural comparison. Aggregate histories retain artificial tags; natural and artificial context claims must remain distinguishable. Do not infer natural reachability from patched segment.

Interruption durability note: local RPC bridge (workspace helper, not toolkit) exited on BrokenPipe when a queued foreground client was terminated during a long results replay. Last durable map267regions/50769bytes and natural6461 snapshot preserved. Bridge now ignores disconnected client writes; restored map via map.load and natural snapshot via import. Replaying results in30-frame chunks with exported snapshots; do not repeat the interrupted130-frame foreground script blindly. This helper issue is not a ROMAtlas finding.

### Batch 9: restored natural results route
Natural replay6461→6591 completed in30/30/30/40-frame segments, zero inputs, tracing enabled; unique fetch locations1461 in this new session. Exported snapshots after each chunk, final`analysis/evidence/podium-natural.json`. Podium screen confirmed; capture/inspection batch underway. Added289 previously inspected result/late-race bytes, map.validate/save/export passed. Earlier130-frame replay was interrupted and its unsaved trace is not treated as durable evidence.

### Batch 10: podium accounting and resource bundle
Inspected natural podium replay code (`analysis/evidence/podium-observed-listing.txt`), added647 code bytes: awards display/accounting, podium rank palettes, names, finish/record timer rendering and bounded display DMA. Added16384 data bytes for$078000+$4000 bundle explicitly transferred to VRAM by$00B121; palette$07B480+$200 and map$07AE40+$640 are supported alternate consumers of subspans, kept in one physical Data region. map.validate/save/export passed. B6591–6594/release initiates shrinking-window transition; traced through6654, next-state sample underway. Snapshots and ordered scenarios are durable.

### Batch 11: Wipeout/continue and score records
Inspected `analysis/evidence/continue-new.txt`; added777 code bytes: loss penalties, money/high-score accounting, continue screen resource loaders and HDMA window transition. Added17396 data bytes: direct VRAM sources$09D510+$2A80 and$0ADABA+$12E0, CGRAM$08E59A+$C0, ten-word descriptor$AA62 and32×23-word tilemap$08E65A. map.validate/save/export passed. At6699Wipeout appeared; B6719–6722accepted YES (later stack inspection proves return from$A9F9in$A3CEwindow fade). Repeated B/Select during transition gave no new code; this was ongoing display wipe, not evidence of rejected input. Next trace completes that transition.

### Batch 12: continued-driver panel
Added307 code bytes for continued-player shop rendering and confirmed Continue input branch. Added556 data bytes: fourteen-by-eighteen-word alternate driver map$09D198, ten-word descriptor$B3D6, and32-byte palette$09D390. map.validate/save/export passed. Continued shop reached naturally at6856 withcash100000. First Down during opening fade was missed; route at6900selectedTopSpeed andBspent100000 rather than exit. This is recorded as tested input outcome, not silently treated as correct exit. Corrective Down/B follows to reach lower-right Start Next Race; last next-track sampling will check whether new loader/code remains productive.

### Batch 13: shared BIT/LDA return tails
Verified R1-L002 instead of deferring obvious fixed-format structure. First tail primary$009790A0 uses BIT toskip LDA entries$009793E0/$00979620/$00979960; second primary$0097F9C0 similarly skips$0097FC00/$0097FF40/$00980280. Fixed encoded aliases and direct branches establish code role; no runtime proof required. Four small secondary fragments removed and consolidated into one14-byte primary Code region, adding4unknownbytes; first existing region comment now explains overlapping internal interpretations. map.validate/save/export passed. Earlier ledger referenced nearby BIT boundaries while candidates conflicted; corrected exact LDA internal entry addresses are authoritative. Editorial equates needed in commentary.
Corrective shop Down6923–6926/release5 thenB6931–6934 reachedGETREADY; subsequent normal transition sampled30frames at a time with persisted snapshots. Existing HDMA code gives only3–4newlocations per30frames while screen wipe completes.

### Batch 14 in progress: all-track consumer domains and asset extents
Natural next-track route reached Huevos Grande at7054;20-frame Bsample added20locations, eight-frame Leftsample added0. Final gameplay snapshot `analysis/evidence/pass1-final-gameplay.json` at7112, artificial=false. Inspected entire845-line `analysis/evidence/next-track-new.txt` before annotation.
Static proof: frontend increments selectorDP1C modulo64; track-order tablephysical000216..000255 contains IDs0..15 plus direction flag, all16 IDs present. $008200separates direction bit andtrackID. $0091FEloads sixteen-entry word/byte parameter arraysphysical00123A..001329 (next routine starts00132A). $00933Findexes sixteen three-byte DMA pointersphysical0013EF..00141E; all entries select$038000or$0C8000, each transfers$8000bytes. Terrain pointer assembled from$00930Aword and$0092FAbank; $00932Acopies Y=$16B0down0 inclusive ($16B1bytes) intoWRAM7F0000. Two BG map pointers from$00929A/$0092BA and$0092CA/$0092EA feed exact$600byte pair loops. These independent consumer contracts and fixed domains establish source extents without demanding each track execute.
Additional connected proofs: $009A9Csprite DMA$02D000+$3C00crosses the sixteen-bit DMA source bank boundary; conservatively propose ONLY physical015000..017FFFasROM source until exact bus wrap is independently reviewed (do not incorrectly mark next bank from a linear file extent). Second sprite block$04DAC0+$1040 andpalette$04EB00+$100staywithinCPUbank. Track HUD$00C14Bcopies$07B600+$2000; per-track titlemaps use C305triples andC335ten-word descriptor pointers. Object placement$00C560uses16pointers atD299to count-prefixed six-byte records; all16counts2..5andno negative early-exit entries. Waypoint loader$008570uses normal/reverse pointer slotsD7FAandends atFFxxword, independent record extent. Track order has31unique descriptors (reverse09absent), so natural reachability of that valid static slot remains separately unresolved.
Classification/save/verification of this coherent batch is next; no new ordinary gameplay wait is needed.

### Batch 14 classified: all-track storage contracts
Added1741codebytes from inspected next-track initializer corpus and233477databytes across17new regions. `analysis/tools/track_assets.py` implements the explicit source/consumer proposals retained in `analysis/evidence/track-asset-proposals.json`; every merged physical Data region retains the corresponding roles and independent extent evidence. Fixed-domain parameter/gfx tables, all-track terrain and BGmap spans, shared sprite/HUD sources, per-track title descriptors/maps, count-prefixed object-placement records and terminated normal/reverse waypoint streams are included. All code-overlap conflicts were explicitly checked/rejected; already-classified data roles preserved. `map.validate`, map.save and map.export passed.
ROM classification: 61.48% (322347 / 524288 bytes classified; 201941 bytes unknown).
Natural play stops at7112on second track after Bsample20frames (+20locations) andLeft8frames (+0); reusable snapshots/scenarios preserve route. One bounded static branch expansion from newly inspected initializer is the final easy-gain check; remaining unknowns require dedicated redirect investigation (audio payload structure, unclassified assets/code contexts), not repeated ordinary driving.

## Round 1 disassembly completed — next phase commentary

ROM classification: 61.50% (322448 / 524288 bytes classified; 201840 bytes unknown).

Final code15348/data307100bytes,390nonoverlappingregions; all390have structural label/comment/evidence (100%classified-region annotation coverage). Editorial readability has not been reviewed and must cover ALL classified source in commentary; no whole-ROM completion is claimed. Unknown bytes remain in173exactranges: [ledger](unknown-ranges.json). [Stats](pass1-stats.json) separates classification from execution.

Final bounded static check inspected next-track-static-new.txt:4roots/119candidates,41newcodebytes and52fixed-consumerdatabytes. Postrace counter producer$00D62Eextends countdown selector extent through34; eightadditionaldata bytes included. Maps validated, saved and exported after final edits. Natural second-track20-frame acceleration/8-frame steering samples gave20then0newlocations; normal replay/static direct-frontier gains have diminished. Further steps need the concrete producer/audio/contract investigations in leads.json rather than more unchanged play.

Checkpoint `round01-disassembly-final`: fresh generation and verification both succeeded, `ok=true`, `byte_identical=true`. Map SHA256 `58540bdd7b4a4f39000a7b09b140988a5c1b228612bc4372fa152ec4dcf465b4`; input SHA256 `24b687f95bb9737d223738f13c00aeaa7d697fa9e2fd50597b81d0cfa2160daa` unchanged. [Exact verification](round01-disassembly-final-verification.json); map `maps/round01-disassembly-final.json` matches current export; internalstate `analysis/state.json`; generated source `generated/round01-disassembly-final`. Raw-byte fallback preserves unknowns and is not counted as classified.

Observed natural execution from durable paged corpora:5705main-ROM instruction starts/12163physical executed-byte union,827SPC ARAMstarts,32IPLstarts. 563artificial-context locations kept separate. Zero aggregate drops checked, bounded detail histories dropped/disabled as recorded; tracing did not cover untraced waits. Aliases and relevant context remain in evidence files. Static targets/discoveries do not count as execution.

Replay: [scenarios.json](scenarios.json) indexes370orderedactions and112completednamedsegments, including starts, traces, presses/releases, bounds, snapshot imports/exports and explicit artificial patches. Final natural snapshot7112: `analysis/evidence/pass1-final-gameplay.json`. Session-local names are not assumed persistent. Earlier interrupted bridge/replay artifacts are explicitly marked above; saved maps/snapshots recovered progress.

Lead outcomes: R1-L001superseded (natural fifth-lap winner/results required no patches); R1-L002verified (intentional BIT/LDA operand-sharing tails, commentary must preserve internal entry symbols). New ready redirect recipes R1-L003vehicle sprite pose/storage domain, R1-L004SPC directory/samples/sequences, R1-L005remaining main-CPU context/contracts. See [leads](leads.json). No redirect or commentary prompt was read in this worker.

Per physical32KiBbank:

|Physical bank|Code|Data|Unknown|
|---|---:|---:|---:|
|00|13862|3814|15092|
|01|0|0|32768|
|02|0|12288|20480|
|03|0|32768|0|
|04|0|27648|5120|
|05|0|27506|5262|
|06|0|31104|1664|
|07|0|22326|10442|
|08|0|5280|27488|
|09|0|32592|176|
|0A|0|28058|4710|
|0B|0|23233|9535|
|0C|0|32768|0|
|0D|0|27648|5120|
|0E|1486|59|31223|
|0F|0|8|32760|

Toolkit findings: one missing feature TI-001 (SNES discovery paging); no confirmed/suspected toolkit bugs and no blocker. [Persistent report](toolkit-issues.md). Local RPC bridge disconnected-client handling was repaired only in this workspace; toolkit repository and input ROM were not changed.

Compact phase handoff: [round01-disassembly-handoff.md](round01-disassembly-handoff.md). Pass status: completed; ROM status: incomplete; coordinator may now launch commentary against the final checkpoint.

## Round 1 commentary active

Separate commentary worker prepared the frozen baseline and independent editable source using commentary.py prepare; initial byte-identical verification passed. Input checkpoint round01-disassembly-final, 390 classified regions / 322448 classified bytes and 173 unknown gaps. No editorial credit predates this phase. Current pointers: commentary/source/game.asm, commentary/review.json; phase remains commentary until all classified source meets the readability gate. Structural map and input remain unchanged.

Commentary batch C1: manual algorithm/name/prose review covered 45 code fragments through physical $000884. Source overlay commentary/annotations.json and commentary/source/game.asm saved. Global symbols/branches remain pending even in these fragments; remaining source is explicitly pending in commentary/progress.md. No editorial completion or final verification claimed.

Commentary batches C2/C3: 151 manual code prose/name records saved through physical $002817. Exact pending inventory commentary/pending-prose.json; current source is partial and global symbols/branches remain pending. Phase remains commentary.

Commentary C4/C5: all 319 classified main-CPU region prose/name records saved. See commentary/progress.md and pending-prose.json for remaining SPC/data and global symbol work. Phase active, not ready for redirect.

Commentary C6/C7: all 390 classified-region prose/name records now saved. Symbols integrated for 828 control destinations, understood main hardware/RAM fields and SPC state. Fresh commentary verify passed byte_identical=true after forcing explicit SPC operand sizes; both assembly sources are hashed. Data fields/source markers and final integrated readability review remain pending; phase stays commentary.

Commentary C8: data subfields/window markers and descriptive branches integrated, all unknown gaps explained inline; current source passes fresh byte-identical verification. Completing the current-source inventory/uncertainty handoff now; final archive audit remains separately pending.

## Round 1 commentary completed — next phase coordinator-owned redirect

ROM classification: 61.50% (322448 / 524288 bytes classified; 201840 bytes unknown).

Input round01-disassembly-final; output editorial checkpoint round01-commentary-final. All 390 classified regions (329 Code/61 Data) now have completed current-source readability, consistent routine/internal/branch/RAM/field/constants, meaningful algorithm/structure explanations and explicit uncertainty. All 173 unknown gaps are explained inline; classification and observed execution remain unchanged. review.json retains all 563 scaffold entries, 390 reviewed units and final editorial_audit pending. No archive audit was run.

Source and reading copy have practical navigation across all16 physical banks; see commentary/banks.md. Important examples include terrain triangle interpolation, stable sprite insertion sorting, sixteen-wheel suspension, two-player shop/name-entry controls, HDMA double buffers, audio command/voice processing and six operand-sharing terrain return entries. Unknown DB/DP views retain numeric reference caveats. Semantic questions remain honest and do not represent unreviewed source.

Fresh input inspect and stock map validation passed. Fresh commentary verify returned ok=true/byte_identical=true for both assembly sources; reading copy refreshed afterward. Maps/current.json, internal state and frozen baseline hashes unchanged (58540bdd7b4a4f39000a7b09b140988a5c1b228612bc4372fa152ec4dcf465b4). [Exact editorial verification](round01-commentary-final-verification.json), [compact handoff](round01-commentary-handoff.md), [readability progress](../commentary/progress.md), [uncertainties](../commentary/uncertainties.md), [redirect questions](../commentary/redirect-questions.md).

No new runtime scenarios or fetches were collected. Existing R1-L003/R1-L004/R1-L005 remain ready for redirect; this worker did not begin redirect. Preserve commentary before later changed-map preparation. Toolkit findings remain one nonblocking missing feature TI-001, no confirmed/suspected bugs or new commentary issue; [report](toolkit-issues.md). Original ROM/toolkit unchanged. Commentary phase complete; ROM incomplete.

## Round 1 redirect active — 2026-10-18

Separate redirect worker consumed `round01-commentary-final` handoff and `round01-disassembly-final` structural checkpoint. Read only redirect phase prompt. Current map remains 390 regions/322448 classified/201840 unknown bytes; commentary source and reading copy preserved. Starting R1-L003 pose-producer domain proof, then R1-L004 audio structure and R1-L005 branch-side closure. No map annotations belong to this phase. Durable findings will be recorded in analysis/investigation.md and analysis/leads.json; TI-001 workaround remains bounded discovery.

## Round 1 redirect completed — combined ready batch

ROM classification: 61.50% (322448 / 524288 bytes classified; 201840 bytes unknown in173ranges). No classification/source edits. Input editorial `round01-commentary-final`, structural `round01-disassembly-final`; fresh unchanged-map output `round01-redirect-final` validated and rebuilt byte-identically after map.save/export. Input/currentmap/internalstate/commentary hashes remain unchanged; exact checks in [redirect verification](round01-redirect-verification.json).

Consumed and materially improved R1-L003/R1-L004/R1-L005, preserving all earlier history in [leads.json](leads.json). Followed pose producer/source domain; main branch/fallthrough and10callee-summary closure; pause/track0/object/window/rectangle/palette tables; SPC envelopes/command handlers and selectedsong grammar/config/SRCN/BRR chains. Combined [manifest](evidence/round01-redirect-proposals.json) proposes103485newunionbytes (100616Data+2869Code), not accepted classification. All supported connected continuations and concrete independently promising resource consumers were investigated; exact excluded domains and actual missing facts in [compact handoff](round01-redirect-handoff.md), [durable investigation](investigation.md).

Natural audio-only observation7112..7122 retained9654events with0drops/artificial=false; exported/imported start snapshot restores7112, scenario recipe saved. Static proposal evidence is separate from opcode fetch coverage. Two existing BIT/LDA operand-sharing boundaries need consolidation during acceptance. Preserve all commentary before a changed baseline. Final archive audit remains separately pending. Recommend next separate disassembly worker validate/apply combined readybatch; coordinator owns advancement. Final-investigation scheduling ineligible at201840unknownbytes/0.384979248046875ratio.

Toolkit findings: existing TI-001 missing discovery paging remains nonblocking; no new issue or confirmed/suspected bug. [Persistent report](toolkit-issues.md). No toolkit/input modification. Redirect complete; ROM incomplete.

## Round 2 disassembly active — 2026-10-18

Separate disassembly worker; structural input round01-redirect-final (same classification as round01-disassembly-final), editorial input round01-commentary-final. All commentary/source, symbols, reading copies, review records and overlays copied intact to analysis/preserved/round01-commentary-final/commentary; hashes.json records exact preserved files. Prior 390-region current-source review credit remains reusable. Validating R1-L003/L004/L005 proposal manifest independently before annotation; no proposal quota. Phase stays disassembly; coordinator owns later phases.

Round 2 validation milestone V1: independently re-decoded all 2457 proposed instruction views (1308 distinct starts), checking ROM opcode, exact length and saved assembly wherever supplied; all pass. Saved full instruction/context results in analysis/evidence/round02-code-independent-inspection.json and readable listing. Inspected producer/consumer source for clamp/pose packing and paired-strip DMA, pickup/object initializers, dynamic timer/animation bounds, HDMA builder, audio loader and complete new instruction listing. Replayed pose domain, all eight selected-song parsers and BRR chains. Subsequent semantic repeat matches and all sample physical extents match the proposal manifest. Initial evidence-file byte-hash equality assertion failed (not used as structural acceptance); current replay results/limitations are persisted in analysis/evidence/round02-data-proof-validation.json. No map changes yet. Terrain 178F/17F8 must each replace the following shared-tail region with one physical region; old internal LDA entry descriptions/labels remain available in preserved source and explicit handoff. Unselected audio cells, songs and sample tail remain excluded. Next: build bounded validated physical regions, preserve all internal alternate contexts in inspection ledger, annotate and validate batch.

Round 2 accepted batch A1: independent validation supports all proposed physical bytes after boundary/context review: +100616 Data/+2869 Code = +103485 unique bytes. 411 new regions and two whole terrain shared-tail expansions, current801regions. Full exact edit/validation log: analysis/evidence/round02-acceptance-log.json. map.validate passed; state saved and portable current map exported. Two original tail IDs169/332 retained through transactional whole-region edits; internal old-entry views included in metadata. The first attempted remove/reinsert incorrectly reused a removed ID; live map was restored from unchanged on-disk state before any accepted edits, then complete-role edits succeeded. This was a local API misuse, not a toolkit issue. ROM classification: 81.24% (425933 / 524288 bytes classified; 98355 bytes unknown).

Ordinary targeted pause scenario next: natural second-track frame7112 is available. Start input should exercise the newly proved pause/window wrapper; bounded trace will distinguish fetch observation from static classification.

## Round 2 disassembly completed — 2026-10-18

ROM classification: 81.24% (425933 / 524288 bytes classified; 98355 bytes unknown).

801classifiedregions:18217Code/407716Data;149unknownranges. Accepted +103485bytes after independent evidence review; lead outcomes R1-L003/L004/L005 now verified in leads.json. Inputs round01-redirect-final/round01-commentary-final; output round02-disassembly-final. All388unchanged regions retain full review credit; two terrain tail roles retain old body/entry review with BIT-prefix reconciliation;411newregions await current-source commentary. Entire prior commentary tree preserved and hash-checked unchanged at analysis/preserved/round01-commentary-final.

Targeted ordinary pause attempt at7112Start2/release30 produced0target fetches; contexts show D580/B4C6 race-entry window. Changed prerequisite: settle120frames in six20framecalls then Start2/release30 to7264.751retained Instruction events,0dropped/0aggregate_dropped; all26distinct starts562E..5666 are alreadyclassified banner renderer. No56E1+pause-wrapper fetch, no unknown sampled starts and no pause reachability claim. Both routes restored durable original7112snapshot; tracing disabled, artificial=false. Initial40frame settle hit1Moperation budget at7134; restored/reduced chunk retry succeeded. Local trace pager/schema mistakes were corrected while preserving live evidence; no toolkit defect claimed. Gameplay/scenarios/investigation updated; no unchanged route repetition warranted.

[Exact accepted plan](evidence/round02-accepted-plan.json), [edit/validation log](evidence/round02-acceptance-log.json), [independent instruction/context inspection](evidence/round02-code-independent-inspection.json), [data proof checks](evidence/round02-data-proof-validation.json), [exact residual ledger](round02-unknown-ranges.json), [stats](round02-disassembly-stats.json), [handoff](round02-disassembly-handoff.md). Further selector/layout/return-contract barriers are retained honestly; no pointer-shaped gaps or unselected audio tail inferred as data. Main/SPC static classifications and runtime opcode-fetch coverage remain separate.

Fresh checkpoint `python3 run.py checkpoint round02-disassembly-final` passed mapvalidation/generation/ok=true/byte_identical=true; generated diagnostics empty. Internal validate/save/export succeeded. Input SHA25624b687f95bb9737d223738f13c00aeaa7d697fa9e2fd50597b81d0cfa2160daa; map/stateSHA2564f7122a3687d082b34e9a831df71a378f313eaf10ff38c313bd0bbb709a5767a. [Exact verification](round02-disassembly-verification.json). ROM/toolkit unchanged.

Per-bank counts follow; exact unknown ranges are in the ledger rather than inferred filler.
|Physical bank|Code|Data|Unknown|
|---|---:|---:|---:|
|00|16077|5553|11138|
|01|0|32768|0|
|02|0|32768|0|
|03|0|32768|0|
|04|0|27648|5120|
|05|0|27506|5262|
|06|0|31104|1664|
|07|0|22356|10412|
|08|0|6048|26720|
|09|0|32656|112|
|0A|0|28058|4710|
|0B|0|23233|9535|
|0C|0|32768|0|
|0D|0|27682|5086|
|0E|2140|15789|14839|
|0F|0|29011|3757|

Toolkit findings: existing nonblocking TI-001 missing SNES discovery paging; no new issue or confirmed/suspected bugs. [Report](toolkit-issues.md). Disassembly pass completed; ROM incomplete. Coordinator may schedule current-source commentary; worker has not launched another phase. Final-investigation scheduling ineligible at98355unknownbytes (ratio0.18759727478027344).

## Round 2 commentary active

Separate commentary worker started from round02-disassembly-final. Entire prior commentary copied to analysis/round02/commentary-input/commentary before fresh preparation; earlier immutable preservation remains available. Reuse 388 unchanged region reviews and reconcile IDs169/332 expanded BIT entries. Exact map delta saved to analysis/round02/commentary-input/delta.json. Pending current-source readability: 411 new regions and the two expanded entries, symbol integration, navigation, fresh verification and reading refresh. Phase remains commentary; no redirect handoff yet.

Round 2 batch C1 saved: all 108 changed/new code roles and 305 new data roles now have durable reviewed prose/name records (new-code-annotations.json, new-data-annotations.json), including selected split pointer bytes, overlapping 141-byte configurations, sequence semantics and BRR upload-gap handling. Integrated source retains prior388blocks. Symbol/entry integration is in progress: first check exposed three duplicate names (old branch/equate dependencies), being reconciled before byte fidelity can be claimed. Remaining gate: fix symbols, verify, add readable field/source-window markers, inspect integrated inventory, refresh bank navigation/review and reading copy. Phase remains commentary.

Round 2 C2: integrated current-map source now rebuilds byte-identically (commentary/verification.json, game.asm SHA25673507a3134bad9680294f0bf7dd535650ed128983cc18b98b83f31b13258f35e). Fixed retained branch-name collisions, removed obsolete equate/inline duplication, preserved all terrain internal entry aliases and corrected the indexed LDA suffix. New source includes field-formatted main structures, selected split pointer bytes, per-command SPC contracts, 149 exact unknown notes and sixteen bank headings. No review completion credit yet: finishing sequence/BRR format presentation and known data aliases, then integrated scope review, final fresh verify and reading refresh. Existing TI-001 only.

Round 2 C3 saved: new sequence source now emits 2385 semantically parsed note/command records using named SEQ constants; two bytes at shared/split boundaries remain explicit bytes with explanations. All64selected song/voice parses terminate at END/reload/repeated semantic state. Selected command vectors use handler symbols, split song pointer bytes use exact ARAM target expressions, main object/placement/descriptor records have readable field layouts, and BRR source carries block/end/loop contracts with SRCN20 upload-gap distinction. Known data table aliases and newly established SPC variable names are used in retained/new consumers. Fresh verification again passes byte_identical=true; source game.asm SHA256a9390bdd7f8adff32389a20a2686d264c737bd7492d34093483ee0c858d220c9, symbols.asm SHA2569eed820379fcc33fd14a97b6cfe86129d8c9c976fa0c68365e30c6294b97b457. Integrated inventory review and final reading/navigation still pending; no phase handoff yet.

## Round 2 current-source readability complete

Input structural checkpoint round02-disassembly-final; output round02-commentary-final. All 801 classified regions (435 Code/366 Data) now have reviewed source units. Retained 388 unchanged region credits and source prose, reviewed 108 changed/new Code plus305newData including the two BIT-prefix expansions. The 950-entry frozen review scaffold is intact;149unknown intervals are inspected/explained. No current classified-source editorial gap remains. Final editorial/archive audit stays pending and was not invoked.

Meaningful integration: terrain shared internal LDA entries remain exact CPU-address aliases; SPC command handlers/fields have individual contracts;2385sequence tokens use named commands,114CALL/JUMP records preserve low/high encoding through47ARAM target symbols, and2shared/split bytes retain explicit notes. Object/placement/rectangle/palette layouts have field-oriented records; selected split song pointers and overlapping141-byte configs retain separate roles; SRCN20 BRR continuation excludes the upload metadata gap. Fixed stale prior-source assertions that newly classified continuations were unknown, and retained genuine width/return/bank barriers. Bank navigation covers all16physical banks.

Fresh edited-source verify and reading refresh pass; structural checkpoint also validates/generates/rebuilds exactly. Separate combined identity/source/read/review hash report: ../analysis/round02-commentary-verification.json (from commentary) or analysis/round02-commentary-verification.json (from workspace). The structural checkpoint report alone is not edited-source proof. Original input and toolkit unchanged; map/state unchanged. Classification remains81.24% (425933/524288);98355unknown bytes in149ranges,18217Code/407716Data. No new execution coverage or gameplay scenario was added. Semantic uncertainty (units/material/object/instrument identities, natural pause entry, unsupported song/vector domains) is documented in uncertainties.md and redirect-questions.md; it is distinct from editorial gaps.

Toolkit findings: one existing nonblocking missing feature TI-001; no new finding or confirmed/suspected bugs. Report: analysis/toolkit-issues.md (workspace), ../analysis/toolkit-issues.md (commentary). Coordinator owns the next phase. Compact handoff: analysis/round02-commentary-handoff.md.

## Round 2 redirect active — 2026-10-18

Separate redirect worker starts from round02-commentary-final after completed current-source readability. No map/source/toolkit/input edits planned. Investigating new callee/width contracts and independent audio format/domain evidence against preserved round1 residuals; unchanged driving/Start routes are excluded. Existing TI-001 workaround retained. Coordinator owns later phase scheduling.

## Round 2 redirect completed — combined ready batch

Input round02-commentary-final; output unchanged-map round02-redirect-final. ROM classification:81.24% (425933/524288classified;98355unknown in149ranges),18217Code/407716Data/801regions. Fresh inspect/mapvalidate/save/export/checkpoint passes byte_identical=true. All11commentary identity/source/reading/review/navigation hashes independently checked unchanged; current-source readability preserved and final archive audit remains separate. No input/toolkit/classification/source modification.

NewR2-L001/R2-L002 ready:352newmainstarts/362views/785Codebytes from individually inspected return contracts, native caller paths and connected highscore/attract/countdown closure;26668newData bytes via bounded DMA, tables, rectangles and independently selected effectstream/SRCN27/28/BRR chains. Combined27453uniqueproposalbytes remain research only. PriorR1lead/history retained. No code/dataownership conflict or current-rolecrossing found. Obvious remaining avenues assessed with actual missing domains/entrycontracts, no productive work deferred for independent scope. [Manifest](evidence/round02-redirect-proposals.json), [investigation](investigation.md), [handoff](round02-redirect-handoff.md), [checks](round02-redirect-verification.json).

Changed natural pause prerequisite now successful:7112release/12x20frames->7352Start2/release20->7374; artificial=false.96pause/transition physicalstarts/127aggregatecontexts,20kretainedspinloop events,44891detaildrops/0aggregatedrops. No unknown retained pause starts. Tracingdisabled/original7112snapshot restored. Exactscenario/gameplay/evidence retained; earlier negativeStarttimings preserved. No unchanged retry warranted. Coordinator owns next separate disassembly; final-investigation ineligible98355>16384ratio0.18759727478027344. Toolkit: existing nonblockingTI-001 missingfeature, no new issue or confirmed/suspectedbug; report analysis/toolkit-issues.md. Redirect complete; ROM incomplete.


## Round 3 disassembly active — 2026-10-18

Separate disassembly worker; structural input round02-redirect-final, editorial input round02-commentary-final. Entire commentary tree preserved and every file hash checked at analysis/preserved/round02-commentary-final/commentary (hashes.json alongside). Prior801 classified regions/readability credit remain intact. Starting independent validation of R2-L001/R2-L002 combined proposal, exact byte ownership, native caller-specific contexts, count-bounded resources and selected effect/BRR bounds. No toolkit/input changes; TI-001 remains nonblocking.

### Round 3 independent validation milestone

All362proposed instruction views/352distinct starts independently decoded with exact opcode, length and assembly agreement (785unique bytes); byte-owner audit confirms no existing-region crossings or competing starts. Additional581supported producer/consumer contextual instructions independently decoded, using per-instruction CFG views rather than linear assumptions across calls. Countdown table34records/98byte rectangles, paired-heading firstFF at10bytes, BRR27/28directory and END boundaries independently checked. Evidence: analysis/evidence/round03-code-independent-inspection.json, round03-consumer-inspection.json, round03-independent-validation.json. No map edits yet.

Correction to redirect prose: SRCN27and28 terminal headers are $B1/$C1 (END set, LOOP clear), not END+LOOP. Both directory loop words equal their aligned start, but these loop addresses are inactive with terminal LOOP clear. Exact END bounds remain supported: ARAMBF5E..C83A(252blocks) andC83A..D6D1(415blocks). Original research preserved; acceptance annotations will carry corrected flags. This is a research prose error, not a toolkit bug. Initial local independent consumer-inspection draft used unsuitable linear contexts; replaced before acceptance with581individually supported CFG contexts, including D62E atphysical562E. No incorrect draft became map evidence.

### Round 3 classification acceptance milestone

Accepted77new physical regions (12Code/65Data), +27453unique bytes=785Code+26668Data after independent context/consumer inspection and exact effect2/3/5/6/7/8parser-result replay. map.validate/save/export passed. All801prior regions unchanged; full prior801-region readability credit reusable. New current total453386classified/70902unknown,878regions. Plan and edit results: round03-accepted-plan.json and round03-acceptance-log.json. SRCN27/28accepted annotations correct terminal LOOP-clear semantics. No duplicate physical roles; overlapping source windows split at boundaries. New bounded ordinary route follows ready completion into second-track acceleration/nitro/steering, with zero-detail aggregate fetch contexts, then restores durable7112.

## Round 3 disassembly complete — 2026-10-18

ROM classification: 86.48% (453386 / 524288 bytes classified; 70902 bytes unknown).

878regions,19002Code/434384Data,144unknownranges. Accepted +27453unique bytes (785Code/26668Data),77newregions (12Code/65Data); all801prior regions unchanged. All878regions labeled/commented/evidenced. R2-L001/R2-L002verified in leads.json; exact inspection/consumer/format/effect replay and edit logs retained. BRR27/28terminalB1/C1meansENDset/LOOPclear; original redirect END+LOOP prose superseded, exact bounds unchanged. Main static widths derive from specific callee/control contracts; weak alternatives retained, no global widths or universal natural-selection claim.

Bounded ordinary followup:7112release+12x20settle=7352, B+Y+Right20/B+Right20/B+Left20/B20=7432; natural/artificial=false.3275ROMviews/2415ROMstarts plus629ARAMstarts,0unknownROMcontexts; aggregate_dropped0, detailcapacity0and1128452intentionaldetaildrops. No new structural gain, route repetition reduced prospect. Original7112restored and tracing disabled. Prior successful source-derived pause evidence reused; no unchanged earlyStart replay. Gameplay/scenarios updated; exact route/result/contexts in analysis/evidence/round03-second-track-drive-*.json.

Internal validate/save/export passed; fresh round03-disassembly-final rebuild byte_identical=true, diagnostics[]. Current map/internal state SHA2562a2cdc090c6e3405b6dfc21592ad6b5f1050c6fa1871e4b852d8ae7bc91fe031. [Verification](round03-disassembly-verification.json), [checkpoint](round03-disassembly-final-verification.json), [stats](round03-disassembly-stats.json), [perbank](round03-bank-counts.md), [unknownledger](round03-unknown-ranges.json). All42preserved commentary files still hash-match live; reuse801oldregionreview,77newregions need readable commentary. No current-map editorial completion claimed. Next separate phase coordinator-owned; no later worker launched. Final-investigation threshold ineligible70902>16384 (ratio0.13523483276367188). ROM incomplete; disassembly pass complete.

Existing residual missing selectors/layouts/context/entry contracts documented in investigation.md; no supported connected resource/directcallee deferred. Toolkit: existing nonblocking TI-001missing discovery paging, no new issue/confirmed/suspectedbug. [Persistent report](toolkit-issues.md). Input/toolkit unchanged. [Compact handoff](round03-disassembly-handoff.md).

Round3 exact physical-bank counts (code/data/unknown):

|Physical bank|Code|Data|Unknown|
|---|---:|---:|---:|
|00|16862|5793|10113|
|01|0|32768|0|
|02|0|32768|0|
|03|0|32768|0|
|04|0|27648|5120|
|05|0|27506|5262|
|06|0|31104|1664|
|07|0|24818|7950|
|08|0|27552|5216|
|09|0|32656|112|
|0A|0|28058|4710|
|0B|0|23233|9535|
|0C|0|32768|0|
|0D|0|27682|5086|
|0E|2140|15983|14645|
|0F|0|31279|1489|


## Round 3 commentary active

Input round03-disassembly-final; entire prior commentary moved intact to analysis/round03/commentary-input/commentary before fresh preparation (additional hash-verified preservation remains analysis/preserved/round02-commentary-final). All801prior review credits reusable;77newregions and affected symbol dependencies pending. Source review/verification/read gates incomplete; final audit pending.

Round3 commentary milestone C1: fresh baseline prepared and all801prior editorial blocks carried without altering physical roles. All77new roles now have source prose and names (12Code/65Data), semantic high-score/window/countdown control, DMA transfers, field-oriented small tables, selected effect command records and SRCN27/28directory contracts. SRCN28aliases existing SRCN29physical415-block chain; terminal C1ENDset/LOOPclear, like SRCN27B1with252blocks. Exact edited-source verify passed (intermediate commentary/verification.json); integrated dependency/readability review/navigation and final reading refresh still pending. No current-source handoff or audit claimed; existingTI-001only.

## Round 3 commentary complete — current source gate passed

Input round03-disassembly-final; output round03-commentary-final. All878classified roles (447Code/431Data) are readable and reviewed; all801prior physical-role credits reused,77new roles reviewed (12Code/65Data), plus affected dependencies.879units, intact1022-entry full-ROM scaffold,144explained unknown intervals,16bank navigation headings, pending editorial ranges[]. Final editorial/archive audit remains pending and was not invoked. Entire prior42-file tree preserved at round03/commentary-input/commentary and all42hashes match prior preservation. No old source/history/symbols/reading/overlays lost.

Meaningful source additions: high-score six-digit/name order and paired glyphs; all-zero blanking and pre-increment scroll latch;301frame input wait; signed bouncing/growing window; countdown34->18wrap/growth; DMA1formats/transfers; heading/row/track/source/descriptor/tilemap records; split selected-effect ARAMpointers and named command streams; selectedACvector/internal step-interval-delay handler. Reconciled051Dwith sprite_animation_cursors+4 for truck contact effects, rather than dynamic-object state. SRCN27B1/SRCN28C1 ENDset/LOOPclear correction retained; SRCN28aliases existingSRCN29physical415-block chain, inactive loop=start for bothaliases. Exact uploads/lifetimes and SRCN20gap preserved. Updated stale pause/ACuncertainties from accepted prior research.

Final fresh edited-source verify ok=true/byte_identical=true, reading refresh ok=true. Sourcegame SHA2567c14cc960284017530319986fd11d176e91fe7e9874856b7979657f5a01ab4c6; symbolssha82ab365bc521438f3315f71866f20d3bc764fc93ddeab3fa96bd4398ed13106b. Separate combined proof [round03-commentary-verification.json](round03-commentary-verification.json) includes source/symbols/reading/review/input/map/state hashes,42priorfilepreservation checks and scope. Structural checkpoint round03-commentary-final validates map/internalstore and rebuilds byte-identically; diagnostics[]. [Checkpoint report](round03-commentary-final-verification.json). Current map/state unchanged SHA2562a2cdc090c6e3405b6dfc21592ad6b5f1050c6fa1871e4b852d8ae7bc91fe031; input unchanged. No runtime/coverage/scenario mutation. ROM still incomplete:86.48% (453386/524288),70902unknownbytes/144ranges;19002Code/434384Data; per-bank counts remain round03-bank-counts.md.

Semantic limits are explained, not skipped ranges: physical/audio/visual identities,21primary mainDB/DPcontexts, songs5..12/unselected effects/SRCNs/ARAMFC27..FFFF,8E/98/BAand entry/format/context barriers. No new ready lead. ExistingTI-001only, nonblocking missing feature; no new toolkit finding/confirmed/suspectedbug, input/toolkit unchanged. [Toolkit report](toolkit-issues.md). [Compact handoff](round03-commentary-handoff.md). Coordinator owns redirect; no later phase launched. Worker stopped writes after final handoff.

## Round 3 redirect — active
Input `round03-commentary-final`; current-source commentary gate verified by compact handoff and edited-source proof. Research-only phase; no map/source changes. Reviewing residual source consumers and complete-layout/copy-length alternatives; unchanged gameplay/audio selector retries excluded. Exact active frontier/evidence will be maintained in analysis/investigation.md and round03 redirect artifacts. Final-investigation threshold ineligible (70902 unknown >16384).

## Round 3 redirect complete — ready combined batch

Input `round03-commentary-final`; output unchanged structural checkpoint `round03-redirect-final`. Research-only: map/state/source/input unchanged. **ROM classification: 86.48% (453386 / 524288 bytes classified; 70902 bytes unknown)**,19002Code/434384Data,878regions/144unknownranges. Current-source readability remains complete; final archive audit separate/pending.

New ready leads **R3-L001/R3-L002/R3-L003**, combined **15816unique proposedbytes (110Code/15706Data),61instructionviews**. Source/count/domain audit found post-race07DAD2+0DC0 and08EC1A+3*60 resources plus frontend/truck/podium/grid/cursor/placement tables. Relevant-state fallthrough817E26bytes proves human race-music `(track_order_slot&7)+6`, SPC decrements to songs5..12. Parsed all64newvoice routes and followed98vector, two19byteauxstreams, SRCNs4/6/11directory/BRR and selected pitch/base/scale cells. Existingprimary-view audit447regions/8793instructions: exactcoverage/errors[],zero unknown directtargets; bothnonterminalunknownfallthroughs closed (817E/BC3F). Three additional callerless wait/hex/formatter graphs have independent mechanicalcontracts/localwidthfacts; unknownE/DB/DPretained, no executionclaim. Palette009EC7+0200read-overlapexistingA037code retained, sparseunsupportedcells excluded.

[Manifest](evidence/round03-redirect-proposals.json), [leads/history](leads.json), [durableinvestigation](investigation.md), [residualmethods/barriers](evidence/round03-redirect-residual-assessment.json), [researchscripts](redirect3/). Proposals are notclassification; acceptance wouldleave55086unknown. Unselected8E/BA/effects/SRCNs, ARAMFC27..FFFF/uploadpadding, sparsepitchinterval/timing, guardedzeroentries and unconsumed mostlyFFbanktails lack independentextent/role/contextfacts. No unchangedStart/drive/selectorretry. Currentandproposedunknownbothabove16384; final-investigation schedulingineligible, coordinatorownsnextphase.

Map.validatepassed; fresh structuralcheckpoint and **fresh edited-source verification** bothok/byte_identicaltrue;13commentaryhandoffartifacthashesallmatch (source/symbols/reading/reviews/banknavigation/map/state/input). [Combinedproof](round03-redirect-verification.json), [checkpoint](round03-redirect-final-verification.json). No runtime/coveragechange. ExistingTI-001only, no newtoolkitbug/suspectedbug/missingfeature; [report](toolkit-issues.md). [Compacthandoff](round03-redirect-handoff.md). Phasecomplete; workerstoppedwritesafterhandoff; no laterphase/workerlaunched.


## Round 4 disassembly active — independent acceptance
Input structural checkpoint round03-redirect-final; editorial round03-commentary-final. R3-L001/L002/L003 under independent review, not automatically accepted. Entire commentary tree preserved and every file hash checked at analysis/preserved/round03-commentary-final; all13 prior handoff hashes match. Reuse unchanged878 physical-role editorial credits. Input453386classified/70902unknown. No changed-map baseline generated. Active worker owns disassembly only.

### Round4 first independent correction milestone
Fresh read-only61candidate instruction views/110bytes exact;447currentCodeviews/8793instructions cover exact regions. Evidence analysis/round04/code-inspection.json and current-code-inspection.json. Resource review identifies byte-width correction:868B/9438/9458 use byte loads at even selectors; no fullword consumer established. Tentatively exclude64skipped odd bytes, preserving unknown; additional bounded A687 AIthreshold window may add5unknown bytes. Exact proof/correction ledger analysis/round04/corrections.md. Audio/pitch independent replay underway; no annotations yet.

### Round4 acceptance-ready bounded batch
Independent semantic replay matches all8new song byte-use unions/command/SRCN/auxpointer results;128songvoices stop in END/reload/repeatedstates. Exact15715noteproof tuples and68pitchindices match, including modulo249/254; intervalnonzero inference excluded. First-END BRR walks and both19byteauxstreams independently checked; perchainEND/LOOP retained. See analysis/round04/audio-validation.json. Sixcode contracts independently verified beyond decode: framecounterwait balancedstatus/X loop; wrapper/body/nibblehelper balancedP/A/X with localSEP30; formatter emits fourhexpairs from encodedfields, nativeword semantics conditional onEfalse. New fallthrough26+9bytes follows proven normalreturn paths and localwidth setups. No naturalfetch claim.
Corrected868B/9438/9458 bytewidths:64oddbytesexcluded; bounded A687..A68F thresholdwindow adds5unknownbytes. All other producer/count/domain/sentinel/uploadsource windows reviewed; existingA037..A0C7Code ownership preserved. Acceptance-ready plan analysis/round04/accepted-plan.json:15757unique newbytes=110Code+15647Data,363newregions;878oldrolesunchanged. Candidate labels/comments/evidence explicit; save/validate/export/checkpoint pending after annotations. Fullcommentary preserved; no source regeneration yet. Ordinary gameplay unchanged short-drive/earlyStart tests already yieldzero unknownROMstarts or timing-specificnegative; no materially new productive runtime prerequisite exists in this static batch, so no unchanged replay.

### Round4 accepted map saved and exported
363newregions accepted; map.validateok, map.saveanalysis/state.json and map.exportmaps/current.json succeeded. All878oldregions exactly unchanged. Code19112/Data450031;469143classified/55145unknown in248intervals. All1241roles have labels/comments/evidence. Fresh checkpoint running. Local acceptance driver initially emitted an invalid digit-leading label and aborted; restored unmodified saved inputmap and corrected prefix before successful full acceptance. Local filename inspect.py also shadowed stdlib exceptionreporting and was renamed; neither is a toolkit finding. No invalid draft retained in acceptedmap.

### Round4 disassembly complete — ROM incomplete
ROM classification: 89.48% (469143 / 524288 bytes classified; 55145 bytes unknown).

Code19112/Data450031;1241physicalroles(453Code/788Data),248unknownintervals. Alllabels/comments/evidencepresent; no executioncoverageincrease. Accepted363newroles requirecommentary;878oldrolecreditpreserved. Whole38filecommentarytree hashesstillmatchlive and preservedcopy; fresh prioreditedsourcebuildbyte_identicaltrue, independentnewstructuralcheckpointbyte_identicaltrue/diagnostics[]. Validation/save/exportsuccessful; map/stateSHA2566f533bbfbe9fdfe9219eae56c766dca8dd73f0cf368487ad55ceb846344760ba. Inputunchanged. Exactverification analysis/round04-disassembly-verification.json; plan/corrections/independentproofs analysis/round04/.

AllreadyR3leadsconsumedwith64bytewidthrejectionsand5additionalAIthresholdbytes. No usefulunchangednormalgameplayretry: natural240settle/pauseandsecondtrackdrivingevidencepreserved; no new runtimeinputs/patches/snapshots. Statecheckednatural7112; priortracehistorydrops1128452detail/0aggregateunchanged. Remainingunknownsrequireindependentrole/extent/contextproof: sparseunselectedpitch/envelopetiming, effects/8E/BA/SRCNs, ARAMFC27..FFFF/uploadmetadata, guardedzerocells, isolatedcodegapsandmostlyFFtails. Newskippedoddlookuppbytesneedawordconsumerorindependentfullrecordformat; no adjacency/paddingguess. No furtherknownproductiveconnectedcontinuationdeferredatleadboundary. Finalinvestigationthresholdineligible55145>16384. Coordinatorownscommentarythenredirect; finaleditorial/archiveauditpending. ToolkitexistingnonblockingTI-001missingdiscoverypagingonly; no new/confirmed/suspectedissue. See analysis/toolkit-issues.md. No toolkit/inputchange.

|Physical bank|Code|Data|Unknown|
|---|---:|---:|---:|
|00|16972|7410|8386|
|01|0|32768|0|
|02|0|32768|0|
|03|0|32768|0|
|04|0|27648|5120|
|05|0|27506|5262|
|06|0|31104|1664|
|07|0|28338|4430|
|08|0|27840|4928|
|09|0|32656|112|
|0A|0|28058|4710|
|0B|0|23233|9535|
|0C|0|32768|0|
|0D|0|27682|5086|
|0E|2140|26205|4423|
|0F|0|31279|1489|

## Round 4 commentary active

Input checkpoint round04-disassembly-final. Entire prior commentary preserved in analysis/round04/commentary-input/commentary with hashes; 878 unchanged physical-role review credits carried. Fresh baseline prepared; 363 new roles have first-pass source prose/rendering (3815 sequence records), pending integration correction, dependency reconciliation and current-source scope review. First edited-source verification found mismatch at physical 8302; investigating local carry integration before continuing. No phase completion or new classification/runtime claim. Next milestone: exact source rebuild, then refreshed review/navigation/reading and dependency uncertainty cleanup.

## Round 4 commentary complete — current-source gate passed

Input round04-disassembly-final; output structural checkpoint round04-commentary-final. All 1241 classified roles (453 Code/788 Data) are readable: 878 prior physical-role review credits reused and 363 new roles reviewed (6 Code/357 Data), with affected dependencies reconciled. The 1489-entry full-ROM review scaffold partitions all 524288 bytes; 1243 meaningful units and all 248 unknown-range notes reviewed. No pending current-source editorial range. Final editorial/archive audit stays pending and was not invoked.

Added 3815 semantic sequence records for race songs5..12, split per-voice/configuration pointers, overlapping configuration slices, selected command98/vector/internal handler and two attack/release auxiliary streams. Source explains sparse pitch and instrument fields, individual BRR END/LOOP contracts and exact upload storage/lifetime. Main additions explain music selection, diagnostic field/byte/nibble helpers, counted-frame wait, text continuation and bounded table/transfer layouts. Local glyphs and sample identity remain tentative. Callerless E/DB/DP remain explicit; native field-word interpretation requires E=false. The 64 skipped heading/menu odd cells remain unknown, and the CGRAM source read overlap retains existing Code ownership.

Fresh edited-source verify ok=true/byte_identical=true; reading refresh ok=true. Separate structural checkpoint validates and rebuilds exactly, diagnostics empty. Combined proof: analysis/round04-commentary-verification.json. Map/state/frozen baseline unchanged SHA2566f533bbfbe9fdfe9219eae56c766dca8dd73f0cf368487ad55ceb846344760ba; input unchanged. Entire prior38-file commentary tree preserved at analysis/round04/commentary-input/commentary and every saved hash checked. Old source/history/overlays/symbols/reading/navigation retained. First local integration verification exposed a ROM/RAM symbol-name collision; renamed the new ROM signature label and reverified. This was an editorial-driver mistake, not a toolkit defect.

ROM remains incomplete: 89.48% (469143/524288 bytes classified), 55145 unknown bytes in248ranges;19112Code/450031Data. Exact residual/per-bank ledgers remain analysis/round04-unknown-ranges.json and round04-bank-counts.md. No new gameplay input, patch, emulator/trace or execution-coverage change. No new ready structural lead claimed. Unsupported effects/8E/BA/SRCNs, interval-dependent pitch, ARAMFC27..FFFF/uploadpadding, guarded cells, isolated gaps/tails and unknown entry/reference contexts remain honest semantic/research limitations, not skipped source work.

Existing sole nonblocking missing feature TI-001(discovery paging); no new toolkit finding, confirmed bug or suspected bug. See analysis/toolkit-issues.md. Input/toolkit unchanged. Phase complete; coordinator owns redirect. Stop writes after handoff.

[Compact handoff](round04-commentary-handoff.md), [combined exact proof](round04-commentary-verification.json), [checkpoint](round04-commentary-final-verification.json), [scope](../commentary/current-source-review.json).

## Round 4 redirect active
Input round04-commentary-final; current-source gate complete. Research only, map/editorial/source preserved. New methods target temporal pitch consumer contracts and a physical/ARAM-aware complete instruction edge audit (prior main-only target filter did not itself establish SPC closure); independent upload-record terminator/record-length and corrected odd-cell consumer review follow. No unchanged gameplay/selector repeats. Active phase persists until combined investigation completes.

### Round4 redirect resumed — temporal/edge milestone
Prior worker interrupted without completion handoff; same phase resumed from saved artifacts. Historical server-restoration artifact did not imply a live server: socket refused. Restarted configured release through run.py serve; loaded analysis/state.json and imported durable natural7112 snapshot. [Restoration](evidence/round04-redirect-resume-restoration.json). Prior live trace aggregates were lost with the process; durable traces remain intact. Local oversized proof-collector OOM is a research-driver resource error, not a toolkit issue.
Connected temporal model now includes AC targetstep/interval/delay and071C-before0742 ordering:1375deduplicated witness tuples,78indices,128routes stop END/reload or repeated complete pitchstate;19currentlyunknown physical readbytes. [Model](redirect4/pitch_temporal.py), [proof](evidence/round04-redirect-pitch-temporal.json). Eight bounded artificial ARAM scratch CALL071C/CALL0742 tests,100SPCoperations each, independently match current/target expectations including restzero,delay,clamp,modulo andACalternation; originalnatural7112restored. [Checks](evidence/round04-redirect-temporal-runtime-checks.json). No natural playback/newexecutioncoverage claim.
Existing453primaryCodeviews/8854instructions decode exactly; sole nominalunknownedge JMPFFC0 requires IPL-aware interpretation, not ROMtail classification. Uploadrecord/oddconsumer and other concrete avenue review continues before combined handoff. Map/source/input untouched; existingTI-001only, no newtoolkitfinding.

## Round 4 redirect complete — coherent ready batch
Input round04-commentary-final; unchanged output structuralcheckpoint round04-redirect-final. Researchonly; readyR4-L001 proposes25Data bytes, noCode:19temporal songpitch cells,2selectedeffectpitchcells,2instrumentbase+2scalecells. ConnectedAC071C-before0742method follows128songroutes(1375deduplicatedwitnesses,78indices) and6selectedeffectroutes(14witnesses); all stopEND/reload/repeatedcompletepitchstate. Eightboundedartificialhelperchecks pass, originaldurablenatural7112restored. No newbuttons/naturalcoverageclaim.
Independentall486executionview/8978instructionviewaudit exact/errors[],0effectiveunknownedges/references. SoleJMPFFC0is mappedIPL, notROMtail. Encodeduploadlength/zeroterminatorcontracts leave504bytepostterminatorblocksand985byteARAMtailwithoutnewinternalroleproof. Threebytewidth/evenindexconsumersdo notsupport64oddheading/menu cells; retainedunknown. Otherconcreteavenues/reasonsin round04-redirect-residual-assessment.json: no newselectedoperands/declaredextent/entry/containerfacts; no unchangedgameplayretry or outside-scope deferral.
ROM remainsincomplete:469143/524288classified(89.48%),55145unknown/248intervals;19112Code/450031Data/1241roles. Proposedacceptancewouldleave55120unknown. Bothabove16384;finalinvestigationschedulingineligible. Current-sourcecommentarygatecomplete;finaleditorial/archiveauditpending.
Freshmapvalidationandround04-redirect-finalstructuralcheckpointpass/byte_identicaltrue/diagnostics[]. Fresh editedsourceverifyalsoexact. All15priorcommentaryhandoffartifacthashesmatch;map/state/source/inputunchanged. Combinedproofround04-redirect-verification.json. LivepredecessortraceaggregateslostinlocalcollectorOOM;durabletracesretained,newconfiguredreleaseserverrestoredmap/natural7112. Newservertraceempty/disabled;noaggregatepreservationclaim. LocalOOMnottoolkitbug.
ToolkitexistingTI-001only:1nonblockingmissingfeature,0confirmedbugs/0suspectedbugs/0newfindings;analysis/toolkit-issues.mdupdated. [Compacthandoff](round04-redirect-handoff.md),[proposals](evidence/round04-redirect-proposals.json),[residualassessment](evidence/round04-redirect-residual-assessment.json),[verification](round04-redirect-verification.json). Phasecomplete;coordinatorownsnextstep. Workerstopwritesafterhandoff;nolaterphase/workerlaunched.


## Round5 disassembly complete — ROM incomplete
ROM classification: 89.49% (469168 / 524288 bytes classified; 55120 bytes unknown).

Code19112/Data450056;1266physicalroles;244unknownranges. Exact residual ledger: analysis/round05-unknown-ranges.json.

Independent consumer replay follows all128 selected songvoices and six supported effects to END/reload/repeated complete pitchstate (134 bounded stops). Reinspected actual SPC700 opcodes at04E7/068A/071C/092D/09F0/0A54 with DP0, diagnostics[]. Sequence tokens and operands are all in prior supported stream roles. Source clear068A plus note092D and tick06DB prove first-rest index0/song1voice6; song2voice0 ARAM1BD7 descent48 toward3 by2 proves sparse intermediate cells. AC071C updates target before0742 slew. All25 manifest cells independently reproduced, no additional unknown read surfaced. Effect entry does not clear current/instrument/every flag: all six reviewed prefixes set92instrument and reach a first nonzero intervalzero note before dependent reads; effects5/6 configure AC before that note, which is safe because the immediate pitch calculation/snapping precedes the tick update. Transpose0; one-shot skip cannot affect index. Fixed0 makes first duration read independent of stale one-shot duration state, and that read clears the one-shot flag. No unsupported blanket effect-state clear claim.

Accepted R4-L001: +25 Data, no Code:19songpitch+2effectpitch+2instrumentbase+2scale. Exact one-byte roles retain unknown neighbors; confirmed first upload ROM70004..77E04 -> ARAM0400..8200/DP0/physical=ARAM+6FC04. Static structural evidence; no new natural playback, runtime inputs, patches or execution coverage. Artificial predecessor tests are mechanics evidence only. Natural durable7112/artificialfalse remains; newserver traceempty/disabled, predecessor live aggregates lost but durable traces preserved.

Prior1241roles unchanged; all38files of round04 commentary preserved and hashes matched BEFORE changedmap checkpoint generation and again afterward. Reuse1241role readability credits;25newData roles pending commentary, full-source integration still required nextphase. Fresh preserved editedsource verification exact; independent structural checkpoint round05-disassembly-final exact/diagnostics[]; map.validate/saveanalysis/state.json/exportmaps/current.json pass. No commentary baseline/prep overwritten. All1266roles have label/comment/evidence. Evidence: round05/independent-temporal-review.json, consumer-inspection.json, cell-inspection.json, acceptance-log.json; combined round05-disassembly-verification.json.

No productive unchanged gameplay retry: prior completeview/directreference audit486views/8978instructions establishes no effective unknown edge; code roles unchanged. IPLFFC0 remains mappedIPL, not ROMtail. All64oddheading/menu cells remainunknown. Residual barriers unchanged beyond accepted cells: unselectedcommands/effects/SRCNs need new selected operands or independent format; isolated gaps need entry/context/role proof; upload tails need internal boundedformat/sourcecontract; graphics tails lack container/sourcecount evidence; guarded zero cells retain negativeconsumer evidence. Connected temporal consumers and all six supported effects were reviewed through their bounded stops, not deferred at first lead. No additional ready actionable lead; coordinator owns commentary then redirect. Final-investigation scheduling ineligible55120>16384; full-ROM classification/final editorial/archive gates incomplete.

Toolkit: sole existing nonblocking missing feature TI-001(discovery paging),0newfindings/0confirmedbugs/0suspectedbugs. Local review draft required removing an unjustified ACzero assertion for effect5/6 prefixes; the actual first-note ordering resolves this, not a toolkit defect. See analysis/toolkit-issues.md. Toolkit/input unchanged; no rebuild. Worker stops writes after handoff; no laterphase/worker launched.

|Physical bank|Code|Data|Unknown|
|---|---:|---:|---:|
|00|16972|7410|8386|
|01|0|32768|0|
|02|0|32768|0|
|03|0|32768|0|
|04|0|27648|5120|
|05|0|27506|5262|
|06|0|31104|1664|
|07|0|28338|4430|
|08|0|27840|4928|
|09|0|32656|112|
|0A|0|28058|4710|
|0B|0|23233|9535|
|0C|0|32768|0|
|0D|0|27682|5086|
|0E|2140|26230|4398|
|0F|0|31279|1489|

## Round5 commentary complete — current-source gate passed

Input round05-disassembly-final; structural output round05-commentary-final. Current-source readability gate passed for all1266 classified physical roles (453Code/813Data):1241unchanged role reviews reused and25new sparseData cells reviewed. Full-ROM1510entry scaffold remains intact;1268meaningful units and244unknown notes reviewed, no pending current-source range. Finaleditorial/archive audit remains pending; not invoked.

Integrated descriptive indexed pitch-low/high cells and instrument31/32 note-base/scale cells as exact db bytes; source documents ARAM upload alias, static consumer route and unknown neighbors. Reconciled affected consumers: reserved effects perform a partial reset; instrument prefixes and first nonzero intervalzero note initialize pitch before dependent tick reads. Effects5/6 configure AC before that note, so no ACzero-at-note assumption. Target stepping precedes slew; cleared songvoice first rest supports indexzero. Prior variables, branch symbols and explanations retained. No whole-array extent, natural playback or new execution claim.64oddheading/menu cells stayunknown.

Entire prior commentary preserved before fresh preparation at analysis/round05/commentary-input/commentary; every saved hash matches. Additional complete copy remains analysis/preserved/round04-commentary-final. Fresh editedsource verify ok=true/byte_identical=true; reading copy refreshed. Separate structural checkpoint validates and rebuilds exactly; diagnostics[]. Input/map/state/frozen baseline identity checks pass. Combined exact hashes and results: analysis/round05-commentary-verification.json.

ROM classification:89.49% (469168/524288bytes classified;55120unknown bytes in244ranges). Code19112/Data450056. Exact residual/perbank ledgers: analysis/round05-unknown-ranges.json and round05-bank-counts.md. Classification remains incomplete, independent of readable-source/fidelity gates. Existing unselected audio cells, unsupported tails and missing entry/context contracts remain honest uncertainty, not skipped editorial work. No new ready structural lead was identified; coordinator owns redirect.

Existing sole nonblocking missingfeatureTI-001(discovery paging), no new toolkit finding/confirmedbug/suspectedbug. See analysis/toolkit-issues.md. Local integration pattern was corrected before final source verification; no toolkit/input changes or rebuild. No runtime/input/trace/snapshot changes. Stopwrites after handoff; no laterphase/worker launched.

[Handoff](round05-commentary-handoff.md), [combined verification](round05-commentary-verification.json), [scope](../commentary/current-source-review.json).

## Round5 redirect active
Input round05-commentary-final; current-source gate complete, structural map unchanged. Comparing exact current source/map delta with prior R3/R4 residual methods before retries. R5-A1 examines a newly explicit callerless native direction helper at physical141F..1438: balanced PHP/PLP/RTS, internalREP20/SEP10, bounded nibble-indexed paired loads9438/9458. Missingentry E/DB are assessed separately; decode/table resemblance alone will not promote it or oddcells. Other residual avenues retain prior concrete negatives pending changedpremise assessment. No new gameplay inputs or arbitraryselectors. StaleRPCsocket required restarting configuredrelease run.pyserve; durable natural7112 imported, artificialfalse; newtraceempty.

## Round5 redirect complete — exhausted on current evidence
Input round05-commentary-final; output round05-redirect-final. No classificationchange;469168/524288classified (**89.4866943359375%**), Code19112/Data450056,55120unknownbytes/244ranges/1266physicalroles. Exactperbankcounts remain round05-bank-counts.md; exactledger round05-unknown-ranges.json independentlyrecomputed.
New blockedIDs R5-L001/R5-L002 retain rawgap semantic candidates:141F status-preserving pairedlookup/nativeE/DBmissing and03E1 collisionresponse/M8entrymissing. Native/emulationandM8/M16alternatives plusfixedprefix/suffixdecode, targetoccurrencesandknownbankwriters are durable in round05-redirect-candidate-milestone.json. No guessed E/M/DB; no oddtablecells or partialtailboundary promoted. Existingnegativeauditsreusedbecause25acceptedDatachangesleaveallcodeviews/oldrolesunchanged. Allidentifiedresidualavenues assessed with independentextent/initializer/context/containeralternatives; no readynewlead remains. No unchangedstartup/shortdrive/earlyStart/selectorsearch.
Map.validate/saveanalysis/state.json/exportmaps/current.json pass; freshstructuralcheckpoint andeditedsource disasm.verify bothbyte_identicaltrue/diagnostics[]. All14commentaryhandoff hashesmatch; fullpriorcommentarypreserved/current-sourcegatecomplete, finaleditorial/archiveauditpending. Combinedverification round05-redirect-verification.json. StaleRPCsocketrefused; restartedconfiguredrelease andimportednatural7112/artificialfalse. Newservertraceempty/disabled;predecessorliveaggregatesnotpreserved,durabletracesretained. No execution/input/patch/coveragegain.
Final-investigation scheduling **ineligible**:55120>16384 and55120/524288=0.105133056640625>0.05. Coordinatorowns scheduling; whole-ROMincomplete. Resume terminalresearchonlywithnewevidence/materiallyimprovedconcretemethod/explicitreassessment; donotrelaunchunchangedplanning. ToolkitsoleexistingnonblockingTI-001missingfeature;0newfindings/0confirmedbugs/0suspectedbugs; analysis/toolkit-issues.md updated. [Compacthandoff](round05-redirect-handoff.md),[assessment](evidence/round05-redirect-residual-assessment.json),[leads](leads.json). Workerstopwritesafterhandoff;nootherworker/laterphase.

## Coordinator stopped-state revalidation 1
Round05-redirect-final remains authoritative. Rechecked all14 recorded commentary/input/map artifact hashes: zero mismatches. Ready leads remain empty; all8 residual avenues have saved methods/results/missingfacts in analysis/evidence/round05-redirect-residual-assessment.json. No new evidence or materially improved untried method is identified. Missing independent entry E/DB/M8 contracts and residual format/extent/producer facts prevent further evidence-backed classification. 55120unknown/524288=0.105133056640625, so automatic final-investigation scheduling is ineligible. Goal incomplete; no phase relaunched on unchanged terminal handoff.

## Coordinator blocked audit — terminal evidence unchanged
Third consecutive goal turn with the same residual-evidence barrier (initial terminal redirect handoff plus two coordinator revalidations). No live phase worker; all14 artifact hashes still match, ready leads empty,55120unknown/244ranges, final-investigation thresholds unmet. No credible untried concrete method identified in combined assessment. Further progress needs new entry-context/extent/producer evidence or a materially improved concrete method. Goal is incomplete and marked blocked; preserve round05-redirect-final and all source/evidence. Existing nonblocking TI-001 only.


## 2026-10-18 user-requested unblock review

Requested /root/super-off-road-2 is absent on this host; reviewed /root/super-off-road, whose round05 stopped state matches the reported ~89%. Independently recomputed the current state.json complement: 55120 unknown bytes/244 ranges, matching round05-unknown-ranges.json exactly. No classification/source/runtime edits. Historical round05 exact-rebuild results inspected; no new rebuild claimed.

Metadata-only candidate inventory: [unblock-review-2026-10-18.json](unblock-review-2026-10-18.json). Nine >=100-byte unknown gaps contain exclusively FF, total43435bytes; eight end at32KiB physical bank boundaries, one ends at the validated header at007FC0. All have existing consumer-bounded object/code predecessors and classified successors. This provides a concrete alternative investigation: evaluate bank/header alignment fill using the combined object-end/layout/repeated-fill evidence, checking alias-aware access/reference counterevidence. Uniform bytes alone are insufficient; neither a live consumer nor proof of historical linker implementation is inherently required to infer padding. These are candidates, not accepted roles. If all supported,11685unknown bytes remain (97.7712631225586% classified), satisfying automatic final-investigation thresholds after required phase gates. Mixed tail02EB72..030000 is deliberately excluded: six leading zero bytes require separate analysis.

Secondary review: R5-L001 native141F helper and R5-L002 M8 collision variant should be reconsidered through independent routine/stack/control-flow/interface contracts. A static routine can have a documented native/width precondition without a proven live caller; distinguish that from asserting actual arrival context. DB association remains separately necessary to map table targets. Existing alternative decodes and semantic similarity are useful but do not alone settle acceptance.

Scheduling is another practical barrier: the 5%/16KiB rule controls automatic scheduling only; workflow explicitly allows user-requested final investigation at larger residual sizes. This reassessment does not launch another worker or mark a phase complete. TI-001 paging has already been fixed locally; no new toolkit issues found in this review, and no toolkit blocker identified. See [toolkit-issues.md](toolkit-issues.md).

## Round5 redirect reopened — focused combined evidence reassessment active
Explicit user reassessment supplies a materially different method: evaluate nine exact FF gaps using independently bounded predecessor objects, bank/header allocation endpoints, repeated layout, and competing access/format claims together. Uniformity alone is insufficient. Reconsider R5-L001/R5-L002 through independent static routine/interface contracts; a conditional native/M8 precondition need not have a live caller. DB/table association remains separate from instruction decoding. Previous blocked outcomes retained; no classification/source/runtime changes. Input round05-redirect-final and current editorial round05-commentary-final; preserve all14 artifact hashes. Intermediate evidence will be saved in round05-redirect-reopened artifacts.

## Round5 reopened redirect — combined evidence milestone and validation
Nine exact homogeneousFF gaps total43435bytes now have individual independently terminated object/consumer ends plus bank/header allocation endpoints, repeated tail layout and explicit competing-claim review. Physical intervals:00609C..007FC0,026C00..028000,037980..038000,03EEB2..040000,046D3A..048000,04FF90..050000,056D9A..058000,05DAC1..060000,06EC22..070000. Existing8978instruction/486view audit remains applicable(allCode unchanged);4106main reference/base screens yieldedzero gap hits, and declared track/round2 source windows havezero overlap. These negatives have boundedcoverage, do notproveunused; combinedpositivebounds/layoutsupportpadding. Fullpergap proofs: analysis/evidence/round05-redirect-reopened-bankfill.json. Mixed02EB72..030000 remains excluded(sixleadingzeros, noindependent02EB78objectboundary).

Improvedstaticcontractreview makes R5-L00125Code/nativeE0 and R5-L00240Code/M8 ready independently of naturalcaller. Minimaldecodecontext excludesDB/DP assumptions; namedstate semantics conditional. SeparateR5-L00432oddmenu highbytes has inferredDB00 tableassociation from paired32bytebases directlyafterroutine, bounded16wordreads, independentlyknownM8samebaseconsumer and consistentdirectiontruth table; notruntimeDBproof. CompetingEtrue/M16decodes retained. Contractdetails/confidence: analysis/evidence/round05-redirect-reopened-routine-contracts.json.

Connectedrawgaptriage additionally identifies two25byte moneyawardsiblings36E8..3701/3701..371A(localSEP30; PHP/PLP restoresoriginalwidthbeforePLX; independentaccepted371Asibling andnormalizer), plus12byte nativeREP10/LDX/LDY/JSRBABC/RTS stub3457..3463 and12byte inlineFFterminatedtext3463..346F. KnownBABCparser+inlinepointer+firstFFproveexacttextwindow underinferredDB00interface. R5-L005/R5-L006; freshminimalcontextdecodes andremaininggapnegativeassessment: analysis/evidence/round05-redirect-reopened-continuations.json.

Combinedreadyproposalmanifest analysis/evidence/round05-redirect-reopened-proposals.json:43606bytes=43435Padding+127Code+44Data across47newphysicalroles, sixreadyIDs R5-L001..R5-L006. Allareproposals, noclassificationperformed. Ifallindependentlyaccepted:11514unknown/199ranges,97.80387878417969%;fillonly11685unknown. Currentremains469168classified/55120unknown/244ranges/1266roles. Otherpromisingavenuesassessed throughsupportedcontinuations; actualmissingfacts andunchangednegatives: analysis/evidence/round05-redirect-reopened-residual-assessment.json.

Freshround05-redirect-reopened-final structuralcheckpoint validatesandbyteidenticalrebuilds; freshcommentary/source verifyok/byte_identicaltrue, diagnostics[]. All14priorartifacthashesmatch; input/map/state/editorialsourceunchanged. Combinedverification analysis/round05-redirect-reopened-verification.json. Noexecution/input/patch/trace/snapshotmutation. IsolatedconfiguredreleaseAPI test confirmsindependentdiscoverypaging(total4,nextoffset1,limit1); TI-001 alreadyfixedlocalbinary, notanactiveblocker. No toolkit edits/rebuild; no new toolkit issues found. See analysis/toolkit-issues.md. Separateacceptance/commentary/redirectrequired; coordinatorownsfuturefinalinvestigationscheduling.

## Round5 reopened redirect complete — ready combined batch
AllsixreadyIDs and47exactproposals persisted; independentmanifestnonoverlap/currentunknown/uniquelabel/arithmetic checks pass. Compact handoff analysis/round05-redirect-handoff.md now supersedes terminalstop, old handoff preserved as round05-redirect-handoff-pre-reopening.md. Fullpriorattempts retained. No identified improved actionable avenue remains unworked; no classification performed. Exactnewcheckpoint round05-redirect-reopened-final unchanged map andfullsource fidelity pass, all14preservationhashesmatch. Coordinator owns separateacceptance/commentary/redirect andfuturefinalinvestigation. No new toolkit issues found; report analysis/toolkit-issues.md. Workerstopwrites; no laterphase/worker launched.


## Round6 disassembly ACTIVE
Input structural round05-redirect-reopened-final; editorial round05-commentary-final. Separate worker independently reviewing R5-L001..006, no annotations yet. Entire commentary preserved at analysis/preserved/round05-commentary-final; hash proof analysis/round06/preservation.json. Reuse1266 unchanged-role review credits. Only disassembly prompt read.

### Round6 independent acceptance milestone
All47roles/43606bytes supported by exact fresh minimal-context decodes, independently recomputed consumer/pointer/descriptor ends and nine combinedfill proofs. Separate inferredDB00 associations accepted for32menuhighbytes and12inlinebytes. Static reachabilityunclaimed. CorrectedYhigh andPLXflags contracts, collisionmerge0406. No map annotation yet. Evidence round06/independent-review.json, independent-decodes.json, accepted-plan.json. Current37file commentary fullypreserved; missinghistoricalsequence-layout separatelycopied, discrepancyrecorded.


## Round6 disassembly COMPLETE — ROM incomplete
ROM classification: 97.80% (512774 / 524288 bytes classified; 11514 bytes unknown).

Code19239/Data450100/Padding43435;1313roles;199unknownranges. Exactpercentage97.80387878417969. Inputround05-redirect-reopened-final/outputround06-disassembly-final; editorialround05-commentary-final.

Independently accepted all six ready R5-L001..006, exact47 physicalroles/43606bytes:43435Padding+127Code+44Data. Existing1266roles byte-for-byte metadata/view unchanged; no overlap/split required because proposals occupy exact unknown gaps. All1313roles retain label/comment/evidence. Static provenance for every newrole; no observed entry, actual arrival status, DB0 or natural gameplay claim.

NineFFtails independently recomputed:00609C..007FC0,026C00..028000,037980..038000,03EEB2..040000,046D3A..048000,04FF90..050000,056D9A..058000,05DAC1..060000,06EC22..070000. For each: exactFF/hash plus independent terminating predecessor code/consumer end plus validated header/32KiB allocation edge and established next object. Repeated layout corroborates. Re-screened4106main references from unchanged453Code/486views and explicit source-window manifests: zero competinghits/overlaps. Source palette/terrain/bg/countdown pointer/descriptor operands independently rechecked against ROM. Boundednegative screen is not proof of universalunread; alternativeSBC/graphics/BRR interpretations have no established object contract here. Historical linker identity/unusedstatus unclaimed. Mixed02EB72..030000 remainsunknown; sixzeros and absent02EB78independentboundary prevent transplanting fillproof.

Native141F..1438 balancedPHP/PLP/RTS and localREP20/SEP10 justify conditionalE0 structure with incomingM/X/DB/DP otherwise unconstrained. Separate inferredDB00 association accepts32highbytes of16word tables9438/9458: boundedwordloads, samebase independently acceptedM8consumer and exactsigned directiontruth table. Remaining32oddheading cells remainunknown. M8collision03E1..0409 has closed19instructiongraph, no candidate stackeffect and known8492tail; minusstore03F8/BRA0406, plusstore0403/merge0406. DB/DP only affect namedsemantics. AlternativesEtrue/M16 retained as nonsupported structuralviews, not globally impossible arrivals.

Two25byte money siblings36E8/3701 have localSEP30, PHXoriginalwidth/PHP thenPLPbeforePLX; exact371Asibling/table/normalizerinterface supports structure. Correctedcontract: PLXoverrides restoredN/Z, andSEP30clearsYhigh ifincomingnativeX16. DirectionSEP10also clearsYhigh; do not claimYunchanged. Existingaccepted371A sibling prose also needs this reconciliation during commentary; its oldphysicalrole remainsunchanged. Native12byte3457stub usesREP10/literal16bitargs/knownBABCparser/RTS; separateinferredDB00inline12byte format includesfirstFF346E beforeknownnextcode346F. All fresh decodesdiagnostics[], exactends. Evidence round06/independent-review.json, independent-decodes.json, proposal-bytes.json, accepted-plan.json, acceptance-log.json.

Entirecurrent37file commentary preserved/hashverified BEFORE annotations/generation and again afterward at analysis/preserved/round05-commentary-final. Earlier38file count differs onlyby absentcurrentsequence-layout.json; historicalfile remains in priorpreservation and additionallyround06/historical-sequence-layout.json. No stale metadata restored. Reuse1266unchangedrole reviewcredits;47newroles and affectedconsumercontracts require nextcommentary integration. Fresh existingeditedsourceverifyok/byte_identicaltrue; current source intentionallystill oldmap until separatecommentary. No baseline/prep/source overwritten. Newstructuralcheckpoint round06-disassembly-final validates/byte_identicaltrue/diagnostics[]; map.savedanalysis/state.json/exportmaps/current.json. Combinedverification round06-disassembly-verification.json.

No emulatorstarted/newexecution/input/release/patch/snapshot/trace changes. No coveragegain claimed. Saved natural7112 scenario remains prior evidence; no newruntime route or actualentryclaim. Boundedstaticchecks settle readyclaims without artificialexecution. Existingconnectedcontinuations inspected through tables,collisiontail,moneysibling/normalizer andtextparser; no additional new selecteddomain or productive unchangedgameplay prerequisite. Residual audio/heading/upload/mixedgraphics/guardedzero questions retain explicit missingformat/producer/allocation facts in round05-redirect-reopened-residual-assessment.json and exactnew199range ledger round06-unknown-ranges.json. Do not repeat unchangedStart/drive/arbitraryselector methods.

ROM incomplete; exact11514/524288=0.021961212158203125, both final-investigation thresholds met. Coordinator owns subsequent fullcurrent-sourcecommentary thenredirect and eligiblefinal-investigation scheduling; this worker launches no laterphase. Finaleditorial/archiveacceptance pending. Toolkit: no new toolkit issues found; historicalTI-001fixed/confirmedconfiguredrelease,0activefindings/blockers. See analysis/toolkit-issues.md. Toolkit/inputunchanged; no rebuild. Worker stopswrites afterhandoff.

|Physical bank|Code|Data|Padding|Classified|Unknown|
|---|---:|---:|---:|---:|---:|
|00|17099|7454|7972|32525|243|
|01|0|32768|0|32768|0|
|02|0|32768|0|32768|0|
|03|0|32768|0|32768|0|
|04|0|27648|5120|32768|0|
|05|0|27506|0|27506|5262|
|06|0|31104|1664|32768|0|
|07|0|28338|4430|32768|0|
|08|0|27840|4806|32646|122|
|09|0|32656|112|32768|0|
|0A|0|28058|4710|32768|0|
|0B|0|23233|9535|32768|0|
|0C|0|32768|0|32768|0|
|0D|0|27682|5086|32768|0|
|0E|2140|26230|0|28370|4398|
|0F|0|31279|0|31279|1489|


## Round6 commentary ACTIVE
Input round06-disassembly-final; editorial round05-commentary-final. Entire current commentary additionally copied/hash recorded under round06/commentary-input before fresh preparation; carried all1266 unchanged physical role source blocks, symbols and uncertainty notes.47 new roles and affected dependencies pending. No runtime/toolkit/input changes.


## Round6 commentary COMPLETE — current-source readable, ROM incomplete
Input round06-disassembly-final/editorial round05-commentary-final; output round06-commentary-final. Classification unchanged97.80387878417969% (512774/524288);11514unknown bytes/199ranges;Code19239/Data450100/Padding43435. All1313roles reviewed with1266sound prior credits reused and47new roles integrated;1512scaffold entries/1315units reviewed,199unknown notes,0pending editorial ranges. Final archive audit pending.

Names/prose cover conditional native direction, collision deflection with corrected0406merge, original-width money stack and finalPLXflags, inline tile-text/renderer,32menu highs and nine allocation fills using combined positive object/allocation proof. Existing low cells, byte-direction helper and371Asibling reconciled. SEP10/30 Yhigh loss explicit; DB00 associations inferred independently; no arrival/natural/universalunread/linker claim.32track-heading odd cells and other residual neighbors remain unknown.

Entire prior37file current tree preserved additionally under round06/commentary-input before prep; all hashes match both copies. Historicalsequence-layout kept separately, no stale metadata restored. Fresh edited-source verifyok/byte_identicaltrue and refreshed readingcopy equality pass; separate structuralcheckpoint valid/byteidentical/diagnostics[]. Exact hashes and evidence: round06-commentary-verification.json. Local driver matcher falsely labeled some incbin body slices as unknown; inventory comparison removed those notes before final verify; not a toolkit defect.

No runtime/input/patch/trace/snapshot/toolkit edits or newcoverage. No new toolkit issues found; TI-001historicallyfixed,0activefindings/blockers. See toolkit-issues.md. Worker stopswrites after compact handoff; coordinator owns redirect then eligible finalinvestigation (11514<=16384 and11514/524288<0.05). No laterphase/worker launched.

## Round6 redirect active
Input round06-commentary-final; current-source gate complete;15handoff hashes verified before work. R6-A1 audits newly accepted conditional routine references/continuations rather than repeating unchanged486views. R6-A2 tests whether the unclassified128byte coordinate-like block has an independently encoded base/consumer; R6-A3 compares trailing tilemap zero/allocation boundaries; R6-A4 tests upload-tail content provenance with longest exact matches and BRR alignment. These are structural research methods, no classification/runtime edit.

## Round6 redirect complete — ROM incomplete
Input round06-commentary-final/output round06-redirect-final. No classification/source edit:512774/524288classified (97.80387878417969%),11514unknown/199ranges;19239Code/450100Data/43435Padding,1313roles. R6-A1 fresh127newCodebyteinspection closes allnewreferences inknowninterfaces; no newheadingword/audio/sourcewindow. R6-A2 exact1AFB64wordformula andencodedbaseownershipscreen finds onlyBRRsamplehit, notconsumer. R6-A3 sixnamedtilemapboundarycomparison/C4FAprogression suppliesnegativealignmentproof; sixleadingzeros before02EB78 stilllackrole. R6-A4 alternateuploadtailcontent/DSPpagecontracttest findsuniformmatchesonly, noreservedpage/BRR/sourcecontract. R6-A5 newroutineschange noaudioselector/guard/gameplayprerequisite; unchangedplayback/Start/drive not repeated. Ready[];blockednewR6-L001..003 retainactualmethods/missingfacts. Assessment analysis/evidence/round06-redirect-residual-assessment.json; collectors/evidence analysis/redirect6/. All identified concreteconnectedandindependentavenues followed; noquota/firstlead/outside-scope stop.
Freshmap.validate/save/export pass, exactledgerrecomputed, structuralcheckpoint validates/byte_identicaltrue/diagnostics[]. Fresh editedcommentary/source verificationok/byte_identicaltrue. All15commentaryhandoffhashes match; current28filecommentarytreehashmanifest captured; noinput/toolkit/source/mapcontentchanges. Noemulatorstart/input/patch/snapshot/trace/coverage changes. Current-sourcereadabilitygate completeunchanged; finaleditorial/archiveauditpending. Proof analysis/round06-redirect-verification.json; compacthandoff analysis/round06-redirect-handoff.md.
Automaticfinal-investigationeligible:0<11514<=16384 and exact11514/524288=0.021961212158203125<=0.05. Coordinator schedulesseparatephase; thisredirectdoesnotclaimfinal-investigationexhaustion orwholeROMcompletion. No ownlaterphase/workers. Toolkit no new toolkit issues found;TI-001historicallyfixed,0activefindings/blockers; analysis/toolkit-issues.md. Workerstopwritesafterhandoff.

## Final investigation attempt01 ACTIVE — independent allocation methods
Input checkpoint round06-redirect-final; editorial round06-commentary-final remains complete. Separate final-investigation worker; no map/source edits or later phases. Exact internal-map complement independently recomputed: 11514/524288 unknown (ratio0.021961212158203125), 199 ranges, 512774 classified. Method comparison recorded BEFORE substantive tests in analysis/final-investigation/attempt01/method-plan.json; prior round3/4/5/reopened5/6 residual assessments retained by reference/hash. No earlier final-investigation attempt exists.

FI01-M1 differs from selected-SRCN/live-selector methods: combine DSP DIR50, independently encoded note-base5084/scale50A4, complete 33-record directory5000..5084 and directory-selected silent block50C4..50CD, then verify all start/loop/BRR allocation contracts under correct logical ARAM mapping. All33 entries terminate, all loop targets are in-chain block boundaries,25 distinct chains tile50C4..FC27 with zero gaps, and known sample0 begins50CD exactly after silent03+8zero block. Two32-byte instrument arrays fit5084..50A4..50C4. This is a joint anchored allocation inference, not pointer-pattern-only or natural selection evidence; independent acceptance still required. Critical correction during preliminary exploration: a BRR chain crossing ARAM8200 must skip physical77E04..78004 (record terminator,504-byte tail and second header). Final collector uses confirmed two-segment upload mapping; no claim from the preliminary physical-contiguous calculation. Evidence directory-allocation.json and replayable investigate.py in attempt directory.

FI01-M2 evaluates effect low1060/high1070/priority1080 arrays against encoded seven-byte envelope origin1090;16-cell geometry coherent but independent unselected stream extent remains to be checked. FI01-M3 pitch bases0CED/0D42 differ85 and complete word recurrence checked; unbounded modular index and no independent endD97 prevent frequency-pattern-only classification. FI01-M4 enclosing copy/storage contracts for mixed graphics/upload tails planned. Full199-range neighbor/bank/ARAM ledger saved; statuses remain provisional pending connected followups. No runtime/input/patch/trace/snapshot/toolkit changes. No new toolkit issues found; historical TI-001 fixed,0activefindings/blockers; analysis/toolkit-issues.md.

Attempt01 connected milestone: FI01-M2 bounded semantic replay of all16 candidate effect records ends cleanly. Effect1 is13bytes434A..4357 and effect4 is17bytes437B..438C, filling exact gaps adjacent to selected effect streams. Other spare entries0/9..15 select existing END-only1567, and effect6 shares5. Both newly identified streams set an instrument before a nonzero note with interval0; connected parameter operands now under review. Equal base spacing alone is not claimed sufficient. Added FI01-M5 BEFORE test: independently bounded writable song-configuration destination1097..1124 may identify exact141-byte ROM initialization backing from confirmed upload, even though later song copy overwrites it. This differs from upload-length-only inference and source-only immutable consumers. Plan/discriminator persisted in method-plan.json; no classification yet.

## Final investigation attempt01 COMPLETE — ready batch, ROM incomplete
Input/output structural classification checkpoint remains round06-redirect-final; editorial round06-commentary-final remains complete/unchanged. Actual512774/524288classified=97.80387878417969%;11514unknown/199ranges,1313roles. Accepted0bytes. Proposed329bytes/30claims across FI01-L001..004:49directory/instrument/silence,60effecttables+streams1/4,196initialenvelopeworkspace,24conditionalorphanstream+dependentpitchhighcell. If all independently accepted11185unknown; not currentclassification.

Eight compared methods recorded in analysis/final-investigation/attempt01/method-plan.json. New joint allocation proofs differ from old selected-source searches: exact directory/array/sample tiling; all16effectdescriptor/closedprogram contracts; initial-upload image of explicitly bounded141-byte writable parameter destination and full28x7 envelope allocation. Already-supportedAE27 ends at independentnextsource1154; exact overwrite1097..1124 includes record21timer. Full provenance/lifetime retained. Effect1/4 connected pitches44/43/1 alreadyclassified. Additional exact23byte interval43A3..43BA is a complete meaningful conditional effect program with initialized instrument12/envelope24/modulation/AC/note78+END and independent previousend/nextindexedstart. Descriptor6 aliases5, so no selector6 or naturalreachabilityclaim. Conditional temporal indices79/86/2 add only highbyte07099C; dependent claim rejected if standalone structure rejected.

M3/M8 wholepitch test rejected: firsttwo base-relative words are voice-mask aliases, exact floor128*2^(k/12) has3mismatches, finalword3FFF not4000. No posthoc fitted rule or blanketpitch expansion. M4 independent enclosing copy/container method finds no encoded8000storage record or postrectangle source endpoint; two504byte tails/mixed5262graphics bytes remainunknown. No new audio contract changes heading/guard/graphics domains; unchangedplayback/Start/drive not repeated. Exact199range ledger records dispositions/remainingfacts and proposal intersections. Full oldattempt histories retainedbyreference; no priorfinalinvestigationattempt existed.

Fresh inspect/currentstockmapvalidation pass; both fresh structuralgenerated/round06-redirect-final and editedcommentary/source verifybyte_identicaltrue. All15priorhandoffhashesmatch and entireexact28filecommentarytree matches redirect6manifest. No map/state/export/source/input/toolkitedits/rebuild, no newcheckpoint because evidence-onlyphase preserves inputcheckpoint. No runtime/input/patch/snapshot/trace/coveragechange. One preliminary orphanhandler decode started in operand/incorrectCPUalias; discarded/replaced with correct7078F/ARAM0B8B before final evidence; researchdriver correction, nottoolkitissue.

Final manifest/verification/ledger/status and compacthandoff: analysis/final-investigation/attempt01/. Latestindex analysis/final-investigation/handoff.md. Fourready leads and passrecord saved analysis/leads.json. Coordinatorowns separateindependentacceptance, fullcurrentcommentary,redirect, then eligiblefinalinvestigation on refreshedresidual; do not repeatunchangedM1..M8. No useful immediateconnectedcontinuation leftunworked; notterminalexhaustion orwholeROMcomplete. No new toolkit issues found;TI-001historicallyfixed,0activefindings/blockers; analysis/toolkit-issues.md. Attemptcomplete/artifactsimmutable; workerstopswrites.


## Round7 disassembly ACTIVE — independent final-investigation acceptance
Input round06-redirect-final; editorial round06-commentary-final. Verified all15 handoff hashes and exact28file current commentary tree, preserved entire tree in analysis/round07/commentary-input before map changes or generation. All1313 prior physical-role reviews retained. Reviewing FI01-L001..004 (329bytes/30claims) independently against ROM and consumer contracts; attempt01 immutable, collectors copied for independent replay. No new runtime/playback prerequisite; no unchanged gameplay retry, toolkit/input edits or rebuild.


## Round7 disassembly COMPLETE — ready final-investigation batch accepted, ROM incomplete
ROM classification: 97.87% (513103 / 524288 bytes classified; 11185 bytes unknown).

Independently accepted FI01-L001..004, exact30 Static Data roles/329bytes (49+60+196+24). All1313 prior roles, labels, comments, evidence and execution views unchanged; all1343 current roles annotated. No split required: claims occupy exact unknown complements. Counts and complete residual ledger recomputed from current internal map, not raw export fallback.

L001 independently reconstructs both upload segments (skip physical77E04..78004), all33 directory start/loop contracts and25 distinct terminated BRR chains tiling ARAM50C4..FC27. Joint DIR50, encoded5084/50A4, explicit silent entry32 at50C4 and independent sample0 at50CD support allocation, with no encoded33/count32 safety guard or natural missing-selector claim. Existing instrument33 reads alias beyond nominal32byte arrays; preserve physical roles and explain overflow aliases in commentary.

L002 independently parses all16 closed effect programs and joint1060/1070/1080 + independently supported envelope1090 boundary. New effect1 is13bytes434A..4357 and effect4 is17bytes437B..438C, with meaningful initialized instrument/volume/envelope/pitch setup and exact adjacent selected stream boundaries. Count16 allocation inference is explicit, not an encoded guard. Their connected pitch/instrument cells were already classified.

L003 independently verifies AE multiply7, fixed-width absolute readers1090..1096, selected AE27 and independent next configuration source1154. Full28x7 initial uploaded envelope image is196bytes, including defaultrecord0, exact141byte writable configuration destination1097..1124, and record21 tail..27. Copy loop Y8C..1 plus explicitY0 overwrites exactly141bytes, including record21 timer1123; initial zeros are not persistent configuration/read-before-overwrite evidence. Confirmed ROM070004..077E04 -> ARAM0400..8200 supplies upload provenance, not byte equality alone.

L004 independently accepts meaningful23byte conditional program43A3..43BA with initialized instrument12/envelope24/negative modulation/AC/note78+60ticks/envelope0/note1+1tick/END, preceding selected END43A2 and next indexed43BA. Natural caller remains unresolved; descriptor6 aliases5 at438C. Independent hand derivation from AC-before-slew, intervalzero and delay3 yields indices79,86,2, adding only highcell7099C. No orphanselector6 or new natural use claim; broad pitch recurrence and pre-directory gap remain unsupported.

All15 prior handoff hashes verified BEFORE writes; entire exact28file current editorial tree copied/hashverified in round07/commentary-input, and remains unchanged. Attempt01 artifacts unchanged/hashverified; copied collectors replayed into round07/replay and five key output objects exactly equal prior evidence. Earlier37file historical preservation trees untouched. Fresh prior edited-source verifyok/byte_identicaltrue; new structural checkpoint validates/byte_identicaltrue/diagnostics[]. Saved internal analysis/state.json and portable maps/current.json. Proof round07-disassembly-verification.json.

No emulator started/new input/release/patch/snapshot/trace/coverage. No unchanged failed gameplay/playback retry: accepted connected contracts change no missing selector/guard/graphics container prerequisite. Residual heading odd cells, guarded isolated bytes, mixedgraphics02EB72..030000, sparse pitch/unselected command/sequence gaps, pre-directory allocation and post-upload/sample tails retain missing structural facts. Full exact residual ledger round07/residual-ledger.json; no byte quota or forced acceptance.

ROM incomplete. All1313 existing source-role review credits reusable;30 new physical roles and changed allocation/initialization dependencies require separate commentary integration. Current source remains prior edited round06 tree and needs refresh before redirect. Coordinator owns commentary, redirect and eligible iterative final investigation (0<11185<=16384 and11185/524288<0.05), final editorial/archive audit pending. This worker launches no laterphase. No new toolkit issues found;TI-001 historically fixed,0activefindings/blockers; analysis/toolkit-issues.md. Toolkit/input unchanged, no rebuild. Worker stopswrites after handoff.

|Physical bank|Code|Data|Padding|Classified|Unknown|
|---|---:|---:|---:|---:|---:|
|00|17099|7454|7972|32525|243|
|01|0|32768|0|32768|0|
|02|0|32768|0|32768|0|
|03|0|32768|0|32768|0|
|04|0|27648|5120|32768|0|
|05|0|27506|0|27506|5262|
|06|0|31104|1664|32768|0|
|07|0|28338|4430|32768|0|
|08|0|27840|4806|32646|122|
|09|0|32656|112|32768|0|
|0A|0|28058|4710|32768|0|
|0B|0|23233|9535|32768|0|
|0C|0|32768|0|32768|0|
|0D|0|27682|5086|32768|0|
|0E|2140|26559|0|28699|4069|
|0F|0|31279|0|31279|1489|



## Round7 commentary ACTIVE
Input round07-disassembly-final; editorial round06-commentary-final. Entire current commentary additionally copied/hash recorded under round07/commentary-input before fresh preparation; carried all1313 unchanged physical role source blocks, symbols and uncertainty notes.30 new roles and affected dependencies pending. No runtime/toolkit/input changes.


## Round7 commentary COMPLETE — current source readable, ROM incomplete
Input structural round07-disassembly-final; editorial round06-commentary-final; output round07-commentary-final.

All 1,343 current physical roles are readable: 1,313 unchanged structural-role review credits reused, 30 new static data roles explained, affected consumers reconciled. The 1,520 full-ROM scaffold entries and 1,345 meaningful units are reviewed; 177 unknown intervals have honest boundary notes. No pending current-source readability ranges. Final editorial/archive audit remains pending.

Explains inferred 33-record directory, two 32-byte instrument arrays, 16 effect descriptors/programs, silence BRR and initial 28-by-7 envelope image. Counts are allocation inferences, never encoded guards. Song selection overwrites exactly 141 bytes at $1097..$1124, including record21 timer; original zeros are initialization only. Conditional $43A3..$43BA program has an initialized-voice contract and unresolved natural caller; descriptor6 aliases5. Only its exact dependent pitch-high index86 is newly supported; whole pitch table remains unresolved. Two-segment initial upload skips ROM $077E04..$078004 and retains mutable ARAM lifetime.

Corrected 14 prior instrument slice suffixes/summaries which counted from the four-byte upload header rather than payload: their actual physical indices are documented. Encoded addresses and bytes unchanged. Real instrument33 effective aliases remain $50A5/$50C5; no extra instrument records are invented. New commands use shared sequence symbols; complete effect-pointer/priority and envelope-timer operands now use descriptive constants. Reused branch/RAM/source comments elsewhere remain valid.

Fresh edited-source verify passed, ok=true/byte_identical=true. Reading copy equals fresh include expansion. Separate structural checkpoint round07-commentary-final passed map/store validation and byte-identical rebuild, diagnostics[]. Structural map/state unchanged. ROM classification remains 513103/524288 = 97.86663055419922%; 11185 unknown bytes in 177 ranges. No new execution or natural-reachability claim. Entire prior 28-file commentary tree preserved and hashes rechecked before/after preparation. No runtime/input/toolkit changes or rebuild. No new toolkit issues found; TI-001 historically fixed, zero active findings/blockers; report analysis/toolkit-issues.md.

Evidence: round07-commentary-verification.json; compact handoff round07-commentary-handoff.md. No later phase/workers launched. Worker stops writes; coordinator owns redirect then eligible iterative final investigation.

## Round7 redirect ACTIVE
Input round07-commentary-final; all19handoffhashes and entire currenteditorialtree captured before research. FI01-L001..004 consumed; oldattempt01 immutable. Compare newlyaccepted allocation/initialization contracts with residualconsumerdomains. Newmethod R7-A1 tests complete command-vector/implemented-handler partition for unselected8E/BA, independent of natural selector reachability. R7-A2 checks neweffect/envelope lifetimes for changed pitch/source domains. R7-A3 triages independent main/graphics/upload alternatives against priorM1..M8; no unchanged startup/drive/retry or speculative classification. No laterphase launched.

## Round7 redirect research milestone
R7-A1: all34 command vectors map to existing handler instruction boundaries; exact missing8E/BA cells target complete one-shot-duration and alternate auxiliary-mode handlers with parser06BF exits. Connected duration and auxiliary-pointer behavior inspected: no new sequence source root. R7-A2 independent18-row song/config serialization test verifies all18 absolute indexed origins, common32-byte rows, uniform voice sparezero columns16..31 and config fallback1567 columns15..31, endpoint1060. Inferred reserved-capacity proposal289bytes retains explicit interrow-fill alternative/no encodedcount32. Two ready leads total293bytes; none classified here. R7-A3..A5 compare effects/envelope lifetime, instrument correction, pitch and independent main/graphics/upload avenues against prior attempts; no new relevant consumer/bank/container prerequisites. Evidence analysis/evidence/round07-redirect-residual-assessment.json and analysis/redirect7/. Fresh checkpoint/source proof pending.

## Round7 redirect COMPLETE — ready combined batch, ROM incomplete
Input round07-commentary-final; output round07-redirect-final. Ready R7-L001 (4bytes command8E/BA vectors) and R7-L002 (289bytes inferred serialized song/config reserved rows). Total293 proposed, zero classified here; exactclaims analysis/redirect7/proposals.json. R7-L002 has an explicit allocated-capacity versus interrow-fill discriminator, no encodedcount32, no NULL-guard/playable-spare claim. Independent acceptance must evaluate the combined18-origin/content/boundary proof, not accept merely base spacing or uniform zeros. All34 vector targets are known instruction boundaries; missingcells target complete alreadyclassified handlers. Both connected state interfaces and independent song/config avenue completed before handoff; other changed-domain/lifetime/pitch/main/graphics/upload avenues have actual unchanged prerequisites/missing facts in analysis/evidence/round07-redirect-residual-assessment.json. No quota, first-lead or outside-scope stop.
Classification unchanged513103/524288=97.86663055419922%;11185unknownbytes/177ranges;19239Code/450429Data/43435Padding,1343roles. Exact residualledger analysis/redirect7/residual-ledger.json; allacceptedproposal outcome would be10892unknown, not current.
Fresh map.validate/save/export passed; structuralcheckpoint round07-redirect-final valid, byte_identicaltrue, diagnostics[]. Fresh editedcommentary/source verifyok/byte_identicaltrue. All19handoff hashes and entire28-file currenteditorialtree unchanged. Proof analysis/round07-redirect-verification.json; handoff analysis/round07-redirect-handoff.md. No emulatorstart/input/patch/snapshot/trace/coverage changes; no input/toolkit/source/mapcontent edits or toolkitrebuild. Final archive audit pending.
Coordinator runs independent disassembly acceptance, full current-source commentary, then redirect; iterativefinalinvestigation attempt02 remains eligible after cycle while residualunknowns remain (current11185/524288=0.021333694458007812). Old attempt01 and M1..M8 outcomes retained; no terminalexhaustion claim. No own laterphase/workers launched. No new toolkit issues found; TI-001 historicallyfixed, zero active findings/blockers; analysis/toolkit-issues.md. Worker stops writes after final handoff.


## Round8 disassembly ACTIVE
Independent R7-L001/L002 acceptance. All19 handoff hashes checked; complete28file commentary preserved before generation in round08/commentary-input/commentary. All1343 prior role credits retained. Static consumer/ROM replay only; no changed productive runtime prerequisite.


## Round8 disassembly ACTIVE
Independent R7-L001/L002 acceptance. All19 handoff hashes checked; complete28file commentary preserved before generation in round08/commentary-input/commentary. All1343 prior role credits retained. Static consumer/ROM replay only; no changed productive runtime prerequisite.


## Round8 disassembly COMPLETE — ROM incomplete
ROM classification: 97.92% (513396 / 524288 bytes classified; 10892 bytes unknown).

Accepted R7-L0014bytes and R7-L002289bytes independently, 21 new Static Data roles; all1343 prior physical roles unchanged. Both34-cell typed vector partition and complete implemented8E/BA interfaces verified against fresh SPC700 decode. Song allocation accepted jointly:18 independently encoded indexed origins, eight low/high selected-root pairs, zero spare voice fields, field-specific paired nonzero1567 configuration tails, and independent next accepted effect origin1060. Interrow-fill alternative explicitly assessed: it explains voice zeros but does not explain paired configuration-rest initialization as directly as reserved serialized capacity. Count32 inferred, not encoded. No playable spare voices, guarded NULL, natural selector or source-copy claim. Exact supported unknown complements inserted without splits. Mutable initial upload provenance retained.

Fresh saved/exported map validated; exact checkpoint round08-disassembly-final byte-identical, diagnostics[]. Prior edited commentary source freshly rebuilt byte-identically; entire28file tree preserved unchanged, all19 handoff hashes checked before writes, immutableFI01 unchanged. No emulator/runtime/input/patch/snapshot/trace/coverage changes or unchanged failed route replay. Independent collector replay copied into round08/replay. Initial local handler assertion used ABA inside predecessor JMP; corrected to actual vector-selected ABB before annotation, not a toolkit issue.

Exact156remaining unknown ranges in round08/residual-ledger.json; per-bank counts below. Connected duration and auxiliary modes install no new stream roots. No changed graphics/container/copy/source or pitch-domain prerequisite; unchanged old methods not repeated. All1343 unchanged source-role credits reusable; 21newroles require separate current-source commentary integration. ROM incomplete; final archive audit pending. Coordinator owns commentary then redirect then eligible iterativeFI02; no laterphase launched. No new toolkit issues found; TI-001 historically fixed,0activefindings/blockers; analysis/toolkit-issues.md. No toolkit/input changes or rebuild. Worker stopswrites after handoff.

|Physical bank|Code|Data|Padding|Classified|Unknown|
|---|---:|---:|---:|---:|---:|
|00|17099|7454|7972|32525|243|
|01|0|32768|0|32768|0|
|02|0|32768|0|32768|0|
|03|0|32768|0|32768|0|
|04|0|27648|5120|32768|0|
|05|0|27506|0|27506|5262|
|06|0|31104|1664|32768|0|
|07|0|28338|4430|32768|0|
|08|0|27840|4806|32646|122|
|09|0|32656|112|32768|0|
|0A|0|28058|4710|32768|0|
|0B|0|23233|9535|32768|0|
|0C|0|32768|0|32768|0|
|0D|0|27682|5086|32768|0|
|0E|2140|26852|0|28992|3776|
|0F|0|31279|0|31279|1489|


## Round8 commentary ACTIVE
Input round08-disassembly-final; editorial round07-commentary-final. Entire current commentary additionally copied/hash recorded under round08/commentary-input before fresh preparation; carried all1343 unchanged physical role source blocks, symbols and uncertainty notes.21 new roles and affected dependencies pending. No runtime/toolkit/input changes.


## Round8 commentary COMPLETE — current source readable, ROM incomplete
Input structural round08-disassembly-final; editorial round07-commentary-final; output round08-commentary-final.

All 1,364 current physical roles are readable: 1,343 unchanged physical-role reviews reused and 21 new Static Data roles integrated. The 1,520 full-ROM scaffold entries and 1,366 meaningful units are reviewed; all 156 unknown intervals have honest boundary notes. No pending current-source readability ranges. Final editorial/archive audit remains pending.

The complete 34-cell command-vector allocation now includes named $8E/$BA vectors and handler entries. $8E enters at $0ABB, forces one duration read, and takes no operand. $BA selects secondary auxiliary mode without installing a stream pointer. Corrected adjacent B6/B8 explanations to match their actual flags. Shared RAM/command symbols and prior branch labels are retained.

The 18 independently encoded song/config row origins support inferred 32-column reserved serialization through $1060. Zero spare voice fields and paired $1567 configuration-rest fields are explained separately. Capacity32 is inferred, not encoded; no playable spare voices, NULL guard, natural extra songs or later source-copy claim. Initial upload initializes mutable ARAM. Prior selected-only and neighboring-unknown descriptions were reconciled with the complete allocations.

Fresh edited-source verification passed, ok=true/byte_identical=true; reading copy equals fresh include expansion. Separate structural checkpoint round08-commentary-final passed exact rebuild, map/store validation and diagnostics[]. Map/state unchanged. ROM remains incomplete: 513396/524288 = 97.92251586914062% classified, 10892 unknown bytes in156 ranges. Prior entire28-file tree preserved with all hashes matching. No runtime/input/patch/snapshot/trace/coverage or toolkit changes; no rebuild. No new toolkit issues found; TI-001 historically fixed, zero active findings/blockers. Report: analysis/toolkit-issues.md.

Proof: round08-commentary-verification.json; compact handoff round08-commentary-handoff.md. No later phase/workers launched. Worker stops writes.

## Round8 redirect ACTIVE
Input round08-commentary-final; all19 handoff hashes and entire28-file editorial tree checked/captured. R7-L001/L002 consumed. Compare accepted full row/vector interfaces with FI01-M1..M8 and R7-A1..A5; inspect current unknown/generic source directly. Test new typed-reference boundary audit of pre-directory interval; no speculative classification or unchanged runtime. Later phase belongs to coordinator.

## Round8 redirect research milestone
Fresh987instruction typed SPC reference audit found no explicit base/target in pre-directory4425..5000. Sole unindexed apparent residual alias is JMPFFC0: supported timer CONTROL80/81 selects IPL overlay, so no ROM fetch/root claim for uploaded tail. This correct storage distinction is retained in evidence, not classified. Completed8E/BA, B6/B8, row spare/config interfaces supply no new stream or pitch/source count. Independent main/coordinate/heading/graphics/upload alternatives compared against FI01-M1..M8/R7A1..A5 with exact missing facts, no unchanged runtime. Assessment analysis/evidence/round08-redirect-residual-assessment.json; no ready leads. Fresh fidelity/preservation checkpoint pending.

## Round8 redirect COMPLETE — no ready lead, ROM incomplete
Input round08-commentary-final; output round08-redirect-final. Fresh987instruction typed-reference audit and changed audio-interface/source-gap assessment completed; no ready structural lead, no classification change. Assessment analysis/evidence/round08-redirect-residual-assessment.json; lead history preserved in analysis/leads.json. Remaining bounds/producer/context/container barriers recorded against priorFI01-M1..M8/R7-A1..A5, not terminal exhaustion or unchanged retries.
Classification513396/524288 =97.92251586914062%;10892unknownbytes/156ranges,1364roles;Code19239/Data450722/Padding43435. Exact ledger analysis/redirect8/residual-ledger.json. Fresh map.validate/save/export, structural checkpoint and edited-source rebuild pass byte-identical;diagnostics[]. All19handoff hashes and entire28-file editorial tree unchanged; reading copy remains fresh. Proof analysis/round08-redirect-verification.json; handoff analysis/round08-redirect-handoff.md. No emulator/input/patch/snapshot/trace/coverage/toolkit changes or rebuild.
Coordinator automatically schedules iterative final-investigation attempt02:10892<=16384 and10892/524288=0.02077484130859375<=0.05. Current-source readability complete; final archive audit pending. Final investigation must design alternate methods against residual facts; prior attempts immutable. No later phase/worker launched. No new toolkit issues found;TI-001historicallyfixed,zeroactivefindings/blockers;analysis/toolkit-issues.md. Worker stops writes.

## Final investigation attempt02 ACTIVE
Input round08-redirect-final; current-source commentary round08-commentary-final complete. Prior FI01 M1..M8 and redirect histories compared before substantive retry; immutable attempt01 and entire28-file editorial tree hash-captured. Materially different methods FI02-M1 paired animation allocation, M2 closed-target composite pointer allocation, M3 declared-state versus read-domain storage are recorded with discriminators/stop conditions in final-investigation/attempt02/method-plan.json. No map/source/runtime changes authorized in this evidence-only phase. Current513396/524288 classified,10892 unknown/156 ranges.

### FI02 research milestone / method extension before test
M1 whole paired31-cell allocation supports14 proposed bytes; M2 eight repeated pointers remain unsupported because duplicate-word reserve satisfies same facts without declared capacity. Fresh13-region decode confirms consumers. Before further test, FI02-M4 added: exact four multi-byte song suffix semantic/stack contracts, materially narrower than prior broad tail triage and different target from FI01 orphan effect. It seeks complete independently bounded callable phrase, rejecting generic END trailers/jump fragments; no new pitch claim from inherited instrument. M3 guarded-domain assessment remains pending.

## Final investigation attempt02 COMPLETE — ready23-byte batch, ROM incomplete
Input/output classification checkpoint round08-redirect-final; current-source commentary round08-commentary-final remains complete. Actual513396/524288=97.92251586914062% classified;10892 unknown bytes/156 ranges,1364 roles. Accepted0/proposed23 bytes in three exact claims, two ready leads FI02-L001/L002. If accepted later,10869 unknown; not current.
FI02-M1 independently encoded paired31-cell animation allocation and synchronized terminal geometry support14bytes; FI02-M4 exact bounded callable song phrase supports9bytes under initialized voice/caller-stack ABI, fixed-duration alternatives preserve physical boundaries. No natural producer/caller or inherited pitch indices claimed. FI02-M2 repeated-pointer capacity8 remains unsupported; M3 guarded zero-state andAI tails lack whole-storage declarations. Four exact multi-byte suffixes reviewed; other generic cleanup/reentry tails remain unknown. All priorFI01/R3..R8 methods compared before substantive retries; M4 extension recorded before test. No unchanged pitch formula, selector, copy/container or gameplay repeat.
All156 ranges have context/neighbors/disposition and concrete alternative-method assessments in final-investigation/attempt02/range-ledger.json and residual-method-assessment.json. Method plan and outcome artifacts preserved. No immediate useful connected continuation remains; ready proposals require independent acceptance, not empty cycle or terminal-exhaustion claim. Coordinator handles separate acceptance, full commentary, redirect, then eligible iterative final investigation. No later phase/worker launched.
Fresh inspect, stockmap/map-store validation, existing structural and edited-source exact rebuilds pass byte-identical. All19 handoff hashes, entire28-file editorial tree, immutableFI01 tree and configured binary unchanged; reading copy fresh. No input/map/state/source/toolkit modifications; no save/export/new checkpoint because exact files preserved. No runtime/inputs/patches/snapshots/traces/coverage changes. Local nested decode-field collector correction before evidence, not toolkit issue. No new toolkit issues found; historicalTI-001 fixed, zero active findings/blockers; analysis/toolkit-issues.md. Handoff final-investigation/attempt02/handoff.md; proof verification.json. Worker stops writes after completed handoff.

## Round9 disassembly ACTIVE
Independent FI02-L001/L002 acceptance. All19 handoff hashes verified and complete28-file commentary preserved before generation. All1364 prior role credits retained. FI01/FI02 immutable; fresh bounded static replay only, no unchanged runtime.


## Round9 disassembly COMPLETE — ROM incomplete
ROM classification: 97.93% (513419 / 524288 bytes classified; 10869 bytes unknown).

Accepted FI02-L001 (14 bytes) and FI02-L002 (9 bytes), three new Static Data roles; all1364 prior roles unchanged. Two independently encoded coindexed animation origins8804/8823, independent following code8842, complete31cell paired layout and synchronizedFF boundaries jointly establish final six-frame suffix. Count31 inferred, not encoded; no natural cursor24 producer. Song phrase ROM07168D..071696 / ARAM1A89..1A92 accepted jointly from meaningful closed note/slur/return object, established priorRETURN1A88, next selectedphrase1A92 and inherited initialized-voice/caller-stack ABI. Explicit-duration56/54/51 notes11/11/10 ticks; fixed-duration alternative retains physical command/end boundaries. No natural caller/instrument/pitch claims. Counterclaims and connected barriers: round09/independent-review.json. No additional supported extension from this batch; generic cleanup/reentry tails, repeated placement pointers and guarded slots remain unknown with existing exact missing facts. No unchanged runtime route repeated.

Map validated/saved/exported; exact round09-disassembly-final rebuild byte-identical, diagnostics[]. Prior edited commentary source freshly verified byte-identical. Entire28file editorial tree and all19 pre-write handoff hashes checked; immutableFI01/FI02 unchanged. All1364 prior source-role credits reusable;3newroles and affected consumers need separate commentary integration. Exact residual ledger has153ranges/10869bytes; per-bank counts below. No emulator/input/patch/snapshot/trace/coverage/toolkit changes or rebuild. No new toolkit issues found; TI-001 historically fixed,0activefindings/blockers; analysis/toolkit-issues.md. ROM incomplete, finalarchive auditpending. Coordinator owns commentary, redirect then eligible iterativeFI03; worker stopswrites after handoff.

|Physical bank|Code|Data|Padding|Classified|Unknown|
|---|---:|---:|---:|---:|---:|
|00|17099|7468|7972|32539|229|
|01|0|32768|0|32768|0|
|02|0|32768|0|32768|0|
|03|0|32768|0|32768|0|
|04|0|27648|5120|32768|0|
|05|0|27506|0|27506|5262|
|06|0|31104|1664|32768|0|
|07|0|28338|4430|32768|0|
|08|0|27840|4806|32646|122|
|09|0|32656|112|32768|0|
|0A|0|28058|4710|32768|0|
|0B|0|23233|9535|32768|0|
|0C|0|32768|0|32768|0|
|0D|0|27682|5086|32768|0|
|0E|2140|26861|0|29001|3767|
|0F|0|31279|0|31279|1489|


## Round9 commentary ACTIVE
Input round09-disassembly-final; editorial round08-commentary-final. Entire current commentary additionally copied/hash recorded under round09/commentary-input before fresh preparation; carried all1364 unchanged physical role source blocks, symbols and uncertainty notes.3 new roles and affected dependencies pending. No runtime/toolkit/input changes.


## Round9 commentary COMPLETE — current source readable, ROM incomplete
Input structural round09-disassembly-final; editorial round08-commentary-final; output round09-commentary-final.

All 1,367 current physical roles are readable: 1,364 unchanged reviews reused and three new Static Data roles integrated. All 1,520 scaffold entries and 1,369 meaningful units reviewed; 153 unknown intervals explained, zero pending readability ranges. Final archive audit pending.

The paired contact animation arrays have inferred capacity31, including six final frames and terminators. Natural cursor24 producer remains unknown. The callable phrase ROM07168D..071696 / ARAM1A89..1A92 requires initialized voice and inherited call frame. Explicit duration note values56/54/51 last11/11/10ticks; fixed-duration alternative preserves physical boundaries. No caller, instrument or effective pitch claim. Changed consumers and prior selected-only limits reconciled.

Fresh edited-source verification byte-identical; reading copy equals fresh expansion. Separate structural checkpoint round09-commentary-final byte-identical, diagnostics empty; internal and portable validation passed. ROM incomplete: 513419/524288 =97.9269027709961% classified;10869unknown bytes in153ranges. Entire28file prior tree preserved with matching hashes; FI01/FI02 immutable. No runtime/input/patch/snapshot/trace/coverage/toolkit changes or rebuild. No new toolkit issues found; TI-001 historically fixed, zero active findings/blockers; analysis/toolkit-issues.md.

Proof: round09-commentary-verification.json; compact handoff round09-commentary-handoff.md. No later phase/workers launched. Worker stops writes.


## Round9 redirect ACTIVE
Input round09-commentary-final; all19handoff hashes verified; complete28-file editorial tree copied/hash captured, FI01/FI02 immutable. Compare accepted31-cell animation and callable1A89 phrase with priorFI01M1..M8/FI02M1..M4 and redirects before substantive retry. Changed-contract dependency and generic/source-gap triage underway; no classification/runtime edits or unchanged failed routes.

## Round9 redirect research milestone
R9-A1 follows newlyaccepted attribute2B namebit1 through OBSEL03, OAM large-size defaults and sharedsprite DMA:24component references for sixframes allinside alreadyclassified ROM016C80..016F60; smallsize subset and palette5 alsoalreadyclassified. R9-A2 exactphrase key-on/RET interface installs no source/instrument/transpose/interval, so no new initialized pitch or continuation target. R9-A3 directlyreconciles153source unknownnotes/generic gaps and compares independentresidualgroups to FI01M1..8/FI02M1..4/R3..R8 withoutunchangedretries. No ready structural lead; actualmissingfacts/comparisons persisted in analysis/evidence/round09-redirect-residual-assessment.json. Freshfidelity/preservation checkpoint pending.

## Round9 redirect COMPLETE — no ready lead, ROM incomplete
Input round09-commentary-final; output round09-redirect-final. Changed animation name-bit asset footprint, callablephrase interfaces and independent residual source/generic-gap triage completed against priorFI01M1..M8/FI02M1..M4/R3..R8. No ready structural leads or classification change. Newlyaccepted2B namebit1 references all24conditional16x16 component tiles inside alreadyclassified016C80..016F60 and existingpalette5; smallsize alternative subset. Phrase notes/key-on/RET install no pointer/instrument/transpose/interval and inherit unresolvedcallframe. Independent groups retain exact format/allocation/context barriers in evidence/round09-redirect-residual-assessment.json; no unchanged capacity/guard/pitchformula/copy/content/reference/gameplay retry.
Classification513419/524288=97.9269027709961%;10869unknownbytes/153ranges/1367roles. Exactledger analysis/redirect9/residual-ledger.json. Fresh map.validate/save/export, separate structuralcheckpoint and editedcommentary/source rebuild allbyte-identical; diagnostics[]. All19handoffhashes, entire28file currenteditorialtree and immutableFI01/FI02 hashes unchanged; readingcopy fresh. Proof analysis/round09-redirect-verification.json; handoff analysis/round09-redirect-handoff.md. No runtime/input/patch/snapshot/trace/coverage/toolkit changes or rebuild.
Coordinator automatically schedules iterativeFI03:10869<=16384 and10869/524288=0.020730972290039062<=0.05. Current-source readability complete; finalarchiveaudit pending. No terminalexhaustion claim; FI03 must design alternative methods against barriers, preserve priorattempts and avoid unchanged retries. No laterphase/worker launched. No new toolkit issues found; TI-001historicallyfixed,zeroactivefindings/blockers; analysis/toolkit-issues.md. Worker stopswrites afterhandoff.

## Final investigation attempt03 ACTIVE
Input round09-redirect-final; editorial round09-commentary-final complete. FI01-M1..M8, FI02-M1..M4 and redirect residual histories compared and hash-linked before tests. FI03-M1 uses conditional continuation/stack contracts for exact five-byte restart transfer, separately retaining unreachable END; FI03-M2 tests bounded indexed destination ownership of pre-song/pitch holes, extending the accepted initial-image method to distinct destinations. Plan, discriminators and stop conditions in final-investigation/attempt03/method-plan.json. ROM/map/source/editorial/immutable prior attempts/binary hash captured; evidence-only phase. Current513419/524288 classified,10869unknown/153ranges.

## Final investigation attempt03 substantive milestone
FI03-M1 now separates ROM07143E..071443 / ARAM183A..183F conditional transpose-reset/JMP object from unconsumed END183F. Abstract target-contract execution follows existing1818 CALL197D: instrument1D initialized before firstnote, internal call/loop frames balanced, maximum5stackbytes,259note events/635targettokens, reaching establishedEND1839. Exact5byte stub adds2tokens and same302alreadyclassified targetbytes; sibling selected reset-transfer tails179F/1801 independently checked. Under initializedactivevoice/fixed0/nooverride/validstack ABI this supports a5byte structural proposal,0accepted; actual map unchanged10869unknown/153ranges. Ownership still requires independent assessment of joint priorEND boundary, meaningful typed transfer, established target and complete ABI; no naturalentry, trailingEND or newpitchclaim. Proof final-investigation/attempt03/restart-target-contract.json.
FI03-M2 freshly decoded all987classifiedSPC instructions and inventoried254explicitMOV stores. Conservative all256index domains are disjoint fromARAM0CED..0E1F; no indirectMOVstore. Existing1097workspace is the only high destination-copy and alreadyconsumed. This rejects the specific initializedwritable-image alternative for129pre-song/22sparsepitchbytes, without claimingnonuse/fill or excluding unknowncode/DSP/mainreupload. Proof destination-image-assessment.json. Current exactledger and independentresidual alternative assessments persisted; no map/source/runtime/input/toolkit edits. Fresh stockmap validation, structuralround09-redirect-final and editedsource rebuilds byteidentical;19handoffhashes,28editorialfiles,45immutableFI01/FI02files and binary preserved. Remaining work: final connected/dependent and independent alternative planning assessment before coherent handoff, not a firstlead stop.

## Final investigation attempt03 COMPLETE — ready proposal, ROM incomplete
Input/outputclassificationcheckpoint round09-redirect-final; editorial round09-commentary-final complete unchanged. FI03-L001 proposes exact5byte conditionalrestarttransfer07143E..071443 /ARAM183A..183F;0accepted. FI03-M1 resolves prior whole6byte suffix instrument/stack objection through established1818/197D continuation: instrument1D beforefirstnote, internalframesbalanced, max5stackbytes,635targettokens/259noteevents toEND1839. Fixed0/nooverride initializedactivevoice ABI; naturalentry unresolved, finalEND183F retained. Independentreview must assess joint boundary/target/semanticownership rather than only validdecoding. FI03-M2 negativeconservative254MOVstore domains over987freshlydecodedSPC instructions exclude pre-song/pitch initialwritable-image proof fromexistingstores; no fill/nonuseclaim. Connectedtargetclosure and independentresidualalternative assessments complete; no immediate credibleuntriedmethod identified on changedpremises, notterminalexhaustion whileproposalawaitsreview.
Actualclassification513419/524288=97.9269027709961%;10869unknown/153ranges/1367roles. Ifaccepted10864unknown; actualunchanged. All153rangecontexts/neighbors/dispositions and exactgroups in final-investigation/attempt03/range-ledger.json; evidence/recipes/outcomes in method-plan, restart-target-contract, destination-image-assessment, residual-method-assessment and proposals. LeadFI03-L001 ready inleads.json; priorattempt/lead historypreserved. Handoff final-investigation/attempt03/handoff.md; latestindexrefreshedwith preservedinputindex. Separateindependentdisassemblyacceptance/fullcommentary/redirect next; coordinatorreturnsFI whileeligible. No laterphase/worker launched.
Fresh run.pyinspect, stockmap/RPCvalidation and structuralround09-redirect-final pluseditedcommentary/source rebuilds passbyteidentical. All19handoffhashes,28editorialfiles,45immutableFI01/FI02files and configuredbinaryunchanged; readingcopyfresh. Verification final-investigation/attempt03/verification.json. No ROM/map/state/source/toolkit edits/rebuild, no save/export/newcheckpoint, no runtime/input/patch/snapshot/trace/coverage changes. No newtoolkitissuesfound; historicalTI-001fixed,0activefindings/blockers, analysis/toolkit-issues.md. Finaleditorial/archiveauditpending. Worker stopswrites atcompletedhandoff.

## Round10 disassembly ACTIVE
Independent acceptance FI03-L001 from round09-redirect-final, editorial round09-commentary-final. All19 handoff hashes checked, entire28-file editorial tree copied before generation; FI01..FI03 immutable hashes captured. Reuse1367 prior role credits. Exact5-byte conditional transfer ownership and established continuation contract reviewed independently; no unchanged runtime replay.

## Round10 disassembly review milestone
FI03-L001 independently supported by joint END1839 boundary, meaningful transpose-reset/JMP1818 transfer, established sibling179F/1801 idiom and finite target CALL197D contract. Fresh bounded handlers decoded; independent traversal635tokens/259notes/302alreadyclassifiedbytes, max5stackbytes, balancedEND1839; instrument1D beforefirstnote. Accept exact5bytes, retain trailingEND183F unknown. No connected extension supported; no runtime change. Evidence round10/independent-review.json and target-traversal.json.


## Round10 disassembly COMPLETE — ROM incomplete
ROM classification: 97.93% (513424 / 524288 bytes classified; 10864 bytes unknown).

Accepted FI03-L001, exact5-byte Static Data restart-transfer object ROM07143E..071443 / ARAM183A..183F; all1367 prior roles unchanged. Joint prior END1839 boundary, coherent transpose-zero/JMP1818 semantics, established selected sibling179F/1801 idiom and balanced existing target CALL197D continuation establish conditional ownership. Independent target traversal635tokens/259notes/302alreadyclassifiedbytes, maximum5stackbytes, instrument1D before firstnote, balanced return/loops through existing END1839. Initialized active voice/fixed0/noone-shot override/validauxiliary-volume-DSP state and5free stackbytes are conditional ABI, not inferred natural entry. Separate trailingEND183F remains unknown; no new pitch or playback claim. Counterclaims and connected barriers: round10/independent-review.json. Target supplies no new source extent/pointer/bank or pitch domain; no other connected supported extension. No changed runtime prerequisite; unchanged routes not repeated.

Map validated/saved/exported; exact round10-disassembly-final rebuild byte-identical, diagnostics[]. Prior edited commentary source freshly verified byte-identical. All19 pre-write handoff hashes checked, entire28file editorial tree copied before generation and unchanged; immutableFI01..FI03 and configured release binary unchanged. Reuse1367 prior role reviews;1newrole and affected consumers require separate commentary integration. Exact residual ledger has153ranges/10864bytes; per-bank counts below. No emulator/input/patch/snapshot/trace/coverage/toolkit changes or rebuild. No new toolkit issues found; TI-001 historically fixed,0activefindings/blockers; analysis/toolkit-issues.md. ROM incomplete; finalarchive auditpending. Coordinator owns commentary, redirect then eligible iterativeFI04; worker stopswrites after handoff.

|Physical bank|Code|Data|Padding|Classified|Unknown|
|---|---:|---:|---:|---:|---:|
|00|17099|7468|7972|32539|229|
|01|0|32768|0|32768|0|
|02|0|32768|0|32768|0|
|03|0|32768|0|32768|0|
|04|0|27648|5120|32768|0|
|05|0|27506|0|27506|5262|
|06|0|31104|1664|32768|0|
|07|0|28338|4430|32768|0|
|08|0|27840|4806|32646|122|
|09|0|32656|112|32768|0|
|0A|0|28058|4710|32768|0|
|0B|0|23233|9535|32768|0|
|0C|0|32768|0|32768|0|
|0D|0|27682|5086|32768|0|
|0E|2140|26866|0|29006|3762|
|0F|0|31279|0|31279|1489|


## Round10 commentary ACTIVE
Input round10-disassembly-final; editorial round09-commentary-final. Entire current commentary additionally copied/hash recorded under round10/commentary-input before fresh preparation; carried all1367 unchanged physical role source blocks, symbols and uncertainty notes.1 new role and affected dependencies pending. No runtime/toolkit/input changes.


## Round10 commentary COMPLETE — current source readable, ROM incomplete
Input structural round10-disassembly-final; editorial round09-commentary-final; output round10-commentary-final.

All 1,368 current physical roles are readable: 1,367 unchanged reviews reused and one new Static Data role integrated. All 1,521 scaffold entries and 1,370 meaningful units reviewed; 153 unknown intervals explained, zero pending readability ranges. Final archive audit pending.

The five-byte restart transfer ROM07143E..071443 / initial mutable ARAM183A..183F clears transpose and jumps to existing CALL1818/197D. Its conditional ABI requires initialized active voice, fixed duration zero, no one-shot override, valid auxiliary/volume/DSP state and five free stack bytes. No inherited frame is consumed. Existing197D selects instrument1D before first note; internal frames balance through END1839. Static traversal635tokens/259notes/302prior bytes adds no pitch domain, playback or natural-entry claim. Former six-byte gap is reconciled to this transfer and separate unsupported trailing END183F. Sound prior labels, variables, branches and mechanical descriptions retained across all1,368 roles; zero readability backlog.

Fresh edited-source verification byte-identical; reading copy equals fresh expansion. Separate structural checkpoint round10-commentary-final byte-identical, diagnostics empty; internal and portable validation passed. ROM incomplete: 513424/524288 =97.9278564453125% classified;10864unknown bytes in153ranges. Entire28file prior tree preserved with matching hashes; FI01/FI02/FI03 immutable. No runtime/input/patch/snapshot/trace/coverage/toolkit changes or rebuild. No new toolkit issues found; TI-001 historically fixed, zero active findings/blockers; analysis/toolkit-issues.md.

Proof: round10-commentary-verification.json; compact handoff round10-commentary-handoff.md. No later phase/workers launched. Worker stops writes.

## Round10 redirect ACTIVE
Input round10-commentary-final; current-source readability gate met. Preserve 19 proof hashes, whole 28-file editorial tree, 62 immutable FI files and configured binary. Compare accepted restart ABI to residual ownership and independent source-gap questions before any retry. Methods and exact barriers persist under analysis/redirect10; no classification or runtime changes authorized in redirect.

## Round10 redirect research milestone
R10-A1 compares accepted restart contract to adjoining END ownership and downstream domains without redoing consumed target parsing. JUMP has no183F return; END deactivates voice, supplies no trailer schema. R10-A2 reconciles153 source-gap notes/current-review scope and independently compares all11 residual groups with immutable FI01M1..8/FI02M1..4/FI03M1..2 and prior redirects. No ready lead; retain structural ownership/allocation/context barriers, not merely reachability objections. Fresh checkpoint/preservation verification pending. Evidence analysis/evidence/round10-redirect-residual-assessment.json.

## Round10 redirect COMPLETE — no ready lead, ROM incomplete
Input round10-commentary-final; output round10-redirect-final. Redirect complete, ROM incomplete. R10-A1 accepted-transfer interface review finds no183F return/fallthrough or new source/storage domain. Prior635-token traversal, instrument1D and balanced frames are consumed evidence, not rerun. R10-A2 directly reconciles153 source unknown notes and current review scope; zero generic physical-role labels. All11 residual groups compared with immutable FI01M1..8/FI02M1..4/FI03M1..2 and previous redirect outcomes. No unchanged parser/destination254MOV/selector/pitchformula/copy/content/reference/gameplay retry. No ready structural lead; FI03-L001 history updated. Structural role/bounds/context remain separate from natural reachability.

Classification513424/524288=97.9278564453125%;10864unknown bytes/153ranges/1368roles. Exact ledger analysis/redirect10/residual-ledger.json. Fresh map.validate/save/export, separate structuralcheckpoint and edited commentary/source rebuild byte-identical; diagnostics[]. All19handoff hashes, whole28-file editorialtree,62immutableFI files and configuredbinary unchanged; readingcopy fresh. Proof analysis/round10-redirect-verification.json. No runtime/input/patch/snapshot/trace/coverage/toolkit changes or rebuild. Current-source readability complete; finalarchiveaudit pending.

Coordinator automatically schedules iterativeFI04:10864<=16384 and10864/524288=0.020721435546875<=0.05. No ready redirect lead is required for that phase. Actual allocation/schema/entry/source-context barriers and prior negative methods are research starting points; this is not terminal exhaustion. No laterphase/workers launched; worker stopswrites. No new toolkit issues found; TI-001historicallyfixed,zeroactivefindings/blockers; analysis/toolkit-issues.md.

Compact handoff analysis/round10-redirect-handoff.md.

## Final investigation attempt04 ACTIVE
Input/output classification checkpoint round10-redirect-final; complete editorial round10-commentary-final. Immutable FI01-M1..M8/FI02-M1..M4/FI03-M1..M2 and all redirect histories compared before substantive retries (attempt04/history-comparison.json). Fresh exact internal complement agrees with all153 ledger ranges:513424/524288=97.9278564453125%,10864unknown bytes. Evidence-only planning evaluates independent allocation, serialized endpoint and conditional ABI alternatives; no map/source/runtime/toolkit changes. No quota or forced classification; terminal decision pending actual alternative-method assessment.

## Final investigation attempt04 research milestone
FI04-M1 tests a materially different complete signed-nibble coordinate storage schema rather than finding a nibble8 producer. Encoded direction origin9CC7 and independently established sine origin9DC7 bracket256cells. All224defined classified cells and31omitted coordinate-8 cells obey independently understood -atan2(y,x) circular-heading semantics within1unit; no fitted generator/rounding correction or holdout claim. Complete nonlinear finite-object plus independent origins supports31byte allocation proposal requiring independent ownership review.225existing cells include undefinedzero-vectorFF. Quantizer excludes nibble8 naturally; no naturalread claim. Remaining38mainbytes and allothergroups lack this whole-object contract; no dependent asset/source domain. Evidence attempt04/signed-grid-contract.json,proposals.json,residual-method-assessment.json; no map/source/runtime edits.

FI04 connected/dependent closure: new stored bearings do not establish a new produced index. Collision consumer subtracts the byte normal before sector quantization; AI adds40 and turns modulo256. Existing256cell heading-sine and heading domains cover these byte values; no pointer/count/bank/source/serializer emerges. All11residual groups compared to allprior accepted/rejected methods, including independent bank-container/checksum, DSP destination-image, compiler-trailer, conditional entry, heading-word/OAMpair and pre-directory compression alternatives (attempt04/residual-method-assessment.json). None supplies another independent structural discriminator on current premises. No unworked immediate connected proposal or credible improved experiment identified;31byte proposal still requires independent review, so no terminalexhaustion claim.

## Final investigation attempt04 COMPLETE — ready31byte proposal, ROM incomplete
Input/outputclassificationcheckpoint round10-redirect-final; editorial round10-commentary-final complete unchanged. FI04-L001 proposes16exactunknownclaims/31bytes of complete signed-nibble direction-grid allocation;0accepted. Consumer-defined nonlinear two-coordinate bearing semantics, independently encoded9CC7origin/9DC7nextsource and all255definedvectorpositions jointly support inferred256cell storage domain, distinct from actual quantizer read domain. All224definedclassified and31omittedcells satisfy circular -atan2(y,x)*128/pi within1unit. No fitted roundinggenerator/holdout/naturalnibble8claim. Independent ownership review must reject if joint evidence is judged merely a plausible numerical pattern; capacity256 is inferred. Full comparison FI01M1..8/FI02M1..4/FI03M1..2/R1..R10 beforetest; exact evidence/counterclaims in attempt04/method-plan.json,signed-grid-contract.json andproposals.json.
Connected output/consumer closure yields no new pointer/count/bank/root. All11independent residualgroups and alternative methods assessed; no immediate credible untried productive method identified, but proposalawaitsreview so notterminalexhaustion. LeadFI04-L001ready inleads.json, allpriorhistorypreserved. Actual513424/524288=97.9278564453125%;10864unknown/153ranges/1368roles. Ifaccepted10833unknown; actualmapunchanged. Exact all153ranges/contexts/dispositions inattempt04/range-ledger.json andexact-ranges.json.
Freshinspect, internal/stockmapvalidation and structuralround10-redirect-final/editedsource rebuilds byteidentical;readingcopyequalsfresh expansion. All19handoffhashes,28editorialfiles,62immutableFI01..03files and configuredbinaryunchanged. Verification attempt04/verification.json; no save/export/newcheckpoint becauseexactmap/statepreserved. No ROM/source/toolkit/runtime/input/patch/snapshot/trace/coverage edits,rebuild ornewgameplayroute. No newtoolkitissuesfound;TI-001historicallyfixed,0activefindings/blockers,analysis/toolkit-issues.md.
Handoff attempt04/handoff.md; latestindexrefreshed preservingfullpreviousindex. Coordinator independentdisassembly/fullcommentary/redirect theneligibleiterativeFI next. Finaleditorial/archiveauditpending; no wholeROMcompletion. Attempt04immutableuponhandoff;worker stopswrites,nootherworker/laterphaselaunched.

## Round11 disassembly ACTIVE
Independent FI04-L001 whole signed-coordinate bearing storage review, input round10-redirect-final/editorial round10-commentary-final. All19handoff hashes match; entire28file editorial tree preserved before generation; FI01..04 immutable. Prior1368role credits retained. No unchanged runtime replay.

## Round11 disassembly review milestone
FI04-L001 accepted31StaticData bytes in16exactclaims after independent whole256cell ROM check, fresh quantizer/consumers/sine-source decode and65536input scalar quantizer-model audit. Consumer-defined signed-coordinate type, finite nonlinear whole-object agreement and independent9CC7/9DC7 origins jointly establish storedextent; no encoded count, exactgenerator/rounding, blindedholdout or naturalnibble8read claim.225priorcells preserved, no connected new source/domain. Evidence round11/independent-review.json and independent-grid.json.


## Round11 disassembly COMPLETE — ROM incomplete
ROM classification: 97.93% (513455 / 524288 bytes classified; 10833 bytes unknown).

Accepted FI04-L001, exact31 Static Data bytes in16claims within ROM001CC7..001DC7 / CPU009CC7..009DC7; all1368 prior roles unchanged. Independent finite256cell test confirms255defined coordinate bearings within1unit of -atan2(y,x)*128/pi, including all31omitted cells; undefinedzero-vectorFF already classified. Independent decoded actual collision and AI consumers establish heading type and coordinate layout. Encoded9CC7 origin plus independent992A/9DC7 sine-source contract, complete nonlinear finite two-dimensional object and225prior typed cells jointly support inferred full256byte storageextent. No encoded count or exactgenerator/rounding/holdout claim. Numerical pattern, adjacency and indexcapacity alone are insufficient; counterclaims reviewed explicitly in round11/independent-review.json. Exhaustive65536input scalar model of decoded simultaneous magnitude reduction confirms produced components-7..7 and225indices: coordinate-8 storage is distinct from natural read-domain. No naturalnibble8read, unused/dead, newexecution or universalhardware claim.

Connected consumers are already classified byte-heading arithmetic; all256sinecells already classified. No new pointer/source/bank/upload/length domain. Existing other residuals retain their documented missing ownership/extent facts; no productive changed runtime prerequisite, unchanged routes not repeated. Map validated/saved/exported; exact round11-disassembly-final rebuild byte-identical, diagnostics[]. Prior edited commentary source freshly verified byte-identical. All19pre-write handoff hashes checked; entire28file editorial tree copied before generation and unchanged; FI01..04 immutable and configuredreleasebinary unchanged. Reuse1368prior role reviews;16newroles and affected grid descriptions require separate commentary integration. Exact residual ledger has137ranges/10833bytes; per-bank counts below. No emulator/input/patch/snapshot/trace/coverage/toolkit changes or rebuild. No new toolkit issues found; TI-001 historicallyfixed,0activefindings/blockers; analysis/toolkit-issues.md. ROM incomplete; finalarchiveauditpending. Coordinator owns commentary, redirect then eligible iterativeFI05. Worker stopswrites after handoff.

|Physical bank|Code|Data|Padding|Classified|Unknown|
|---|---:|---:|---:|---:|---:|
|00|17099|7499|7972|32570|198|
|01|0|32768|0|32768|0|
|02|0|32768|0|32768|0|
|03|0|32768|0|32768|0|
|04|0|27648|5120|32768|0|
|05|0|27506|0|27506|5262|
|06|0|31104|1664|32768|0|
|07|0|28338|4430|32768|0|
|08|0|27840|4806|32646|122|
|09|0|32656|112|32768|0|
|0A|0|28058|4710|32768|0|
|0B|0|23233|9535|32768|0|
|0C|0|32768|0|32768|0|
|0D|0|27682|5086|32768|0|
|0E|2140|26866|0|29006|3762|
|0F|0|31279|0|31279|1489|


## Round12 commentary ACTIVE
Input round11-disassembly-final; editorial round10-commentary-final. Entire current commentary additionally copied/hash recorded under round11/commentary-input before fresh preparation; carried all1368 unchanged physical role source blocks, symbols and uncertainty notes.16 new roles and affected dependencies pending. No runtime/toolkit/input changes.


## Round11 commentary COMPLETE — current source readable, ROM incomplete
Input structural round11-disassembly-final; editorial round10-commentary-final; output round11-commentary-final.

All 1,384 current physical roles are readable: 1,368 prior role reviews reused and 16 new Static Data roles integrated, with affected dependencies updated. All 1,521 scaffold entries and 1,386 meaningful units reviewed; 137 unknown intervals explained, zero pending readability ranges. Final archive audit pending.

The complete stored signed-nibble bearing grid ROM $001CC7..$001DC7 has 256 inferred cells, including 31 coordinate -8 cells formerly excluded by the consumer-domain test. Decoded collision and AI consumers establish heading type and signed-coordinate layout; encoded CPU $009CC7 origin, independent sine origin $009DC7, full nonlinear finite grid and 225 prior typed cells jointly support storage extent. All 255 defined bearings agree within one circular heading unit; zero-vector $FF was already known. No encoded count, exact generator, rounding or holdout claim. Exhaustive 65,536-pair scalar quantizer modeling confirms only 225 produced indices (-7..7); stored -8 capacity is separate. Source labels describe the added row/cells, field comments explain packing and heading units, collision subtraction/sectors and AI quarter-turn steering. Old gap assertions are reconciled; natural nibble-$8 reads remain unresolved. No unused/dead or new execution claim. Sound prior routine, variable, branch and data explanations retained across all classified source.

Fresh edited-source verification byte-identical; reading copy equals fresh expansion. Separate structural checkpoint round11-commentary-final byte-identical, diagnostics empty; internal and portable validation passed. ROM incomplete: 513455/524288 =97.93376922607422% classified; 10833 unknown bytes in137 ranges. Entire28file prior tree preserved with matching hashes;79 FI01..04 files and configured release binary immutable. No runtime/input/patch/snapshot/trace/coverage/toolkit changes or rebuild. No new toolkit issues found; TI-001 historically fixed, zero active findings/blockers; analysis/toolkit-issues.md.

Proof: round11-commentary-verification.json; compact handoff round11-commentary-handoff.md. No later phase/workers launched. Worker stops writes.

## Round11 redirect ACTIVE
Input round11-commentary-final; current-source readability gate met. Preserve 19 proof hashes, whole 28-file editorial tree, 79 immutable FI files and configured binary. Compare accepted signed-grid storage extent with connected heading consumers and independent source-gap questions before any retry. Methods and exact barriers persist under analysis/redirect11; no classification or runtime changes authorized in redirect.

## Round11 redirect research milestone
R11-A1 inspects newly complete grid interfaces and existing collision/separation/sine/steering/waypoint continuations without repeating geometry or65536input producer modeling. Heading storage adds no new pointer/width/bank/source extent; all256sine cells already classified. Start-grid M8even consumer and separate unrooted coordinate-like block do not inherit grid ownership. R11-A2 directly reconciles137source gaps and11independent residual groups against15immutableFI01..04methods and prior redirect histories. Actual missing structural contracts retained; no ready lead or unchanged experiment. Fresh checkpoint/preservation verification pending. Evidence analysis/evidence/round11-redirect-residual-assessment.json.

## Round11 redirect COMPLETE — no ready lead, ROM incomplete
Input round11-commentary-final; output round11-redirect-final.
R11-A1 inspects newly complete bearing-grid interfaces through collision sectors, pair separation, sine/terrain probes and AI steering/waypoint continuation. They operate on byte headings and already classified sine storage; no new pointer, bank, source/count, index producer or asset. Independent start-heading odd bytes still lack a stored-word schema; the coordinate-like block still lacks a typed producer/allocation. No repeated grid fit,65536input quantizer, unchanged source/pitch/selector/parser/copy/reference/gameplay experiment. R11-A2 directly reconciles137source gaps/neighboring raw sections, zero generic physical-role labels and complete current-source review; all11independent residual groups compared with15immutableFI01..04methods and prior redirect histories. No ready lead; FI04-L001 history updated. Specific structural barriers retained separately from natural reachability.

Classification513455/524288=97.93376922607422%;10833unknownbytes/137ranges/1384roles. Exactledger analysis/redirect11/residual-ledger.json. Fresh internal/portable validation, map.save/export, separate structuralcheckpoint round11-redirect-final and edited commentary/source rebuild byte-identical; diagnostics[]. All19handoff hashes, whole28-file editorial tree,79immutableFI01..04files and configuredbinary unchanged; readingcopy fresh. Proof analysis/round11-redirect-verification.json; bankcounts analysis/redirect11/per-bank-counts.json. No runtime/input/patch/snapshot/trace/coverage/toolkit changes or rebuild. Current-source readability complete; finaleditorial/archiveaudit pending.

Coordinator automatically schedules iterativeFI05:10833<=16384 and10833/524288=0.020662307739257812<=0.05. No ready redirect lead is required. Design concrete alternative methods against exact barriers before any substantive retry; this is not terminal exhaustion. No laterphase/workers launched; worker stopswrites. No new toolkit issues found; TI-001 historicallyfixed,zeroactivefindings/blockers; analysis/toolkit-issues.md.

|Physicalbank|Code|Data|Padding|Classified|Unknown|
|---|---:|---:|---:|---:|---:|
|00|17099|7499|7972|32570|198|
|01|0|32768|0|32768|0|
|02|0|32768|0|32768|0|
|03|0|32768|0|32768|0|
|04|0|27648|5120|32768|0|
|05|0|27506|0|27506|5262|
|06|0|31104|1664|32768|0|
|07|0|28338|4430|32768|0|
|08|0|27840|4806|32646|122|
|09|0|32656|112|32768|0|
|0A|0|28058|4710|32768|0|
|0B|0|23233|9535|32768|0|
|0C|0|32768|0|32768|0|
|0D|0|27682|5086|32768|0|
|0E|2140|26866|0|29006|3762|
|0F|0|31279|0|31279|1489|

Compacthandoff analysis/round11-redirect-handoff.md.

## Final investigation attempt05 ACTIVE
Input/output classification checkpoint round11-redirect-final; complete editorial round11-commentary-final. Recomputed exact internal complement: 513455/524288 =97.93376922607422% classified;10833unknown bytes in137ranges/1384roles. All15FI01..04 methods, outcomes and redirect histories compared before substantive retry; immutable copies and configured release/editorial hashes recorded under attempt05. FI05-M1 separately tests whether complete output-bit semantics from explicit AA and paired-bit packing can establish source ownership for four skipped OAM masks; numerical pattern alone will not qualify. Other independent schema/container/extent alternatives are being assessed, without map/source/runtime/toolkit edits or unchanged experimentation.

## Final investigation attempt05 research milestone
FI05-M1 distinguishes actual complete output-bit semantics from storage ownership:008AE4 ORA #AA and independent009189/00918C two-bit packing manufacture flags without reading the skipped odd OAM masks. They do not prove a stored eight-cell source selector, so four bytes remain unknown. FI05-M2 closes a separate instruction-effect class omitted from FI03-M2: all987prior classified-SPC instructions reconciled with current map,25non-MOV direct-page memory mutations freshly decoded,108implicit stack-write sites inventoried, no absolute RMW/bit mutations or destination domain reaching pitch/pre-song/pre-directory/sample-tail storage. No new live execution or allocation claim. All11residual groups separately evaluated for independent schema, extent, ownership and context alternatives against all15prior FI methods and redirect histories. No credible untried immediate discriminator survives; terminal incomplete decision pending fresh validation/preservation. Exact artifacts attempt05/oam-output-versus-source.json,nonmov-destination-assessment.json,residual-method-assessment.json.

## Final investigation attempt05 COMPLETE — terminal incomplete
Final investigation attempt05 COMPLETE — terminal incomplete. Input/retained checkpoint round11-redirect-final; complete current-source editorial round11-commentary-final.513455/524288=97.93376922607422%classified;10833unknown bytes/137ranges/1384roles;0proposed/accepted. All15prior FI methods and redirect histories compared before two new discriminating methods. FI05-M1 rejects complete output-bit semantics as source ownership:AA and paired-bit packing manufacture flags without consuming odd masks. FI05-M2 independently extends prior MOV audit to non-MOV writes:987preserved SPC instructions reconciled with current views,25direct-page mutations freshly decoded,108implicit stack-write sites,0absolute RMW/bit candidates reaching residual images. No dependent allocation or producer. All11residual groups independently assessed for other credible schema/container/extent/context alternatives; none remains immediately productive on unchanged evidence. This is genuine current-method/planning exhaustion, not missing facts alone, quota or universal runtime requirement. Exact137ranges, aliases,neighbors,hypotheses,missingfacts and dispositions:analysis/final-investigation/attempt05/range-ledger.json; comparisons and terminal reasons:residual-method-assessment.json. Stop unchanged automatic scheduling; resume requires new evidence, materially improved concrete discriminator or explicit user reassessment. No invented next lead.

Fresh inspect/internal and portable validation, structural and edited-source byte-identical rebuilds pass;diagnostics[];fresh reading expansion matches. All19handoff hashes,28editorial files,79immutable FI01..04files and configured binary unchanged. Exact map/state/source preserved, no save/export/new checkpoint because evidence-only final-investigation contract. No ROM/toolkit/runtime/input/patch/snapshot/trace/coverage changes or rebuild. Current-source readability complete; finaleditorial/archiveaudit pending. No whole-ROM completion. No new toolkit issues found;TI-001historicallyfixed,0activefindings/blockers;analysis/toolkit-issues.md. Compact handoff analysis/final-investigation/attempt05/handoff.md. Attempt05 immutable upon handoff;worker stopswrites,nootherworkers/laterphases launched.

## Coordinator FI05 stopped-state revalidation 1
Terminal attempt05 handoff reviewed against current artifacts; all19 round11 commentary/input/map hashes still match. No new structural evidence or materially improved concrete discriminator identified; terminal137range/10833byte barriers remain. User-supplied ninefill/routine reassessment was carried through acceptance and iterative FI01..05; no unchanged planning relaunched. Goal remains incomplete, final archive audit pending. No new toolkit issues found; TI-001 fixed.

## Coordinator blocked audit — FI05 terminal barrier unchanged
Third consecutive goal turn with same terminal residual-evidence barrier (FI05 completion, first stopped-state revalidation, current audit). All phase workers terminal; all19 protected artifact hashes still match. 10833unknown bytes/137ranges; no ready proposal or credible untried concrete discriminator after FI01..05 and redirect assessment. Further evidence-backed progress needs new ownership/extent/context/serializer facts or a materially improved concrete method. Goal incomplete and marked blocked, not achieved. Retain round11-redirect-final and round11-commentary-final; final archive audit pending. No new toolkit issues found; TI-001 fixed, zero active blockers.


## 2026-10-18 second user-requested unblock reassessment

User explicitly requested reassessment of the terminal FI05 stop. Recomputed current map complement:513455/524288 classified (97.93376922607422%),10833unknown bytes/137ranges, exact FI05 ledger match. Reviewed FI01..05 method/result barriers and current assembly. Two concrete hypotheses for independent review are recorded in [unblock-review-02.json](unblock-review-02.json); reproducible read-only collector [tools/unblock_review_02.py](tools/unblock_review_02.py). These are candidate evidence, not accepted map edits or an achieved classification result. Prior terminal FI records remain immutable.

**UNBLOCK02-PITCH (19bytes):** FI01-M8 exact floor-generator test failed at semitones63,75,84. Fresh ownership check shows all three failing low/high pairs are ALREADY classified. Two differ from ideal by less than one integer unit; final3FFF is the14bit maximum, independently supported by local DSP implementation vendor/jgenesis/backend/snes-core/src/apu/dsp.rs:183. This does not recover the historical generator. A predeclared capped equal-tempered semitone representation with <1quantum error holds for all85words, including75previously complete known words and10incomplete words. Max absolute error0.9973544308732016. The19missing bytes belong only to lowARAM0CEF..0D44/high0D44..0D99; the3D9bytes ROM07099D..0709A0 are OUTSIDE and excluded. Review joint storage inference using actual split-base consumer0CED/0D42, independent prior accepted index2/86 temporal endpoints,85base displacement and full musical semantic agreement. Exact formula mismatch is evidence against exact original rounding recovery, not automatically against the consumer-defined quantized pitch role. No correction fitted to missing cells; prior files expose all values, so no blinded holdout claim. Formula/base-spacing alone still does not establish allocation, and no natural index production or execution is asserted. This is materially different from another exact-floor refit or arbitrary inherited-state experiment.

**UNBLOCK02-FILL-SUFFIX (5256bytes):** prior exhausted methods sought an enclosing graphics allocation/trailer for the entire mixed5262-byte gap. Evaluate ONLY the homogeneousFF suffix ROM02EB78..030000, keeping six zeros02EB72..02EB78 unknown. Actual byte transition is reproducible; it is a candidate boundary, not role proof alone. Joint review should compare exact terminal-bank position, independently finished software tilemap at02EB72, established next-bank object and nine acceptedFFfill analogues, plus source-window/alias-aware reference counterevidence. Missing six-byte trailer semantics need not automatically prevent a separate later fill role. This does not assume shared exporter alignment, enlarge a graphics copy, claim the original linker or prove universal absence of reads. Prior enclosing-record and common-alignment failures remain valid; do not repeat them. If independent review still finds suffix-role evidence insufficient, reject this specific hypothesis with that distinction explicit.

If both accepted:5558unknown bytes remain,98.93989562988281%classified. Current map remains97.93376922607422%; no proposals applied. Remaining3035pre-directory/985sample-tail/1008post-upload and other sparse bytes retain actual ownership/format barriers; no unsupported whole-ROM completion promise or renamed unchanged retry. Suggested coordinator action is one independent disassembly review of these precise two candidates, then normal full current-source commentary/redirect and changed-evidence final investigation only as warranted.

Fresh portable map validation passes; fresh edited commentary/source rebuild returns ok=true/byte_identical=true. State/current-map bytes and game.asm/symbols.asm hashes match prior accepted checkpoint; no source text, map, runtime, input, patches, snapshots, traces, coverage or toolkit implementation changes. Source verification may refresh local assembler output. No new toolkit issues found; TI-001 remains historically fixed. See [toolkit-issues.md](toolkit-issues.md).


## 2026-10-18 autonomous inference prompt migration

User requested correction of the goal-agent prompting behind repeated external unblock reviews. Added shared evidence-policy.md and updated AGENTS.md, workflow.md, goal.md and all four phase prompts from the current SNES profile. Verified each original instruction file exactly matched its original generated profile before replacing it; originals and hashes retained under analysis/prompt-migrations/inference-policy-2026-10-18/before/. Migration manifest records matching protected ROM/map/state/commentary/final-investigation hashes. No classification, source, immutable attempt, runtime or goal-status changes; no worker or phase launched.

Diagnosis: insufficient positive guidance for combining independent constraints; rejection summaries treated as vetoes; whole-gap and exact-generator failures propagated to narrower structural claims; coordinator accepted self-certified exhaustion after preservation checks. New policy requires scoped claims/counterexamples, consistent semantic representation tests, justified inference corrections on unchanged data, and a compact rejection review inspected by the coordinator before a terminal stop. It retains exact extent/bank/role evidence, disassembly validation, full readability/fidelity gates and a genuine incomplete stop; no mandatory repeated scan or speculative completion.

On next resume the coordinator and workers must reread evidence-policy.md and their updated prompts before inheriting terminal barriers. Evaluate the reasoning on the strongest residual candidates, not merely another preservation audit or unchanged experiment. Existing external-review candidates remain evidence for independent assessment, not automatic accepted classifications. Toolkit documentation: /root/romatlas/docs/agent-inference-audit.md. This instruction migration does not demonstrate live-agent judgment; subsequent work must establish that.


## 2026-10-18 continuous redirect workflow migration

User authorized retiring the separate final-investigation phase and folding its methods into ordinary redirect. Updated goal.md, workflow.md, AGENTS.md, evidence-policy.md and the three active phase prompts. Removed prompts/final-investigation.txt after preserving its exact original alongside all other pre-migration instructions in analysis/prompt-migrations/continuous-redirect-2026-10-18/before/. Manifest records before/after instruction hashes and matching protected ROM/map/state/commentary/legacy-investigation hashes. No map, source, classification, old investigation record, goal-status or runtime change; no worker or phase launched.

Investigation now proceeds in redirect at every completion percentage, without a separate final phase, size threshold or fixed pass quota. Whole-table/consumer/container/selector/context investigations and scoped rejection review remain normal research. Supported combined proposals go through disassembly, full current-source commentary and redirect. Useful constraints or a justified next method without a proposal continue redirect directly, without an empty classification cycle.

On resume, reread the updated coordinator and redirect prompts. Continue unfinished legacy research as redirect using the saved frontier, ledgers and decisive artifacts; preserve analysis/final-investigation/attempt01..05 and handoffs as historical evidence. Write new research to unique analysis/redirect/<attempt-id>/ directories. A completed attempt is not a stop condition; a sound rejection review with no useful revised inference/test permits an honest incomplete stop. Final editorial/archive acceptance and byte-identical verification remain distinct gates. This migration supplies instructions, not proof of autonomous live-agent improvement or accepted residual classification.

## Updated-policy redirect policy-resume-01 ACTIVE
Explicit user reassessment reopens ordinary redirect at round11-redirect-final / complete editorial round11-commentary-final. New evidence-policy requires claim-local rejection reasoning: FI01-M8 exact floor failures do not themselves reject quantized split pitch storage; FI01-M4 mixed-container failure does not itself reject separate FF suffix. Before testing, changed inference, independent constraints, semantic criterion and stopping rules recorded in analysis/redirect/policy-resume-01/method-plan.json. Legacy attempts remain immutable. No classification/source/runtime/toolkit changes. Counts513455classified/10833unknown/137ranges.

## Updated-policy redirect policy-resume-01 research milestone
PR01-M1 tests consumer-needed quantized semitone representation, not historical floor generator:85words pass identical<1integerquantum capped14bit criterion;75complete known maxerror0.9973544308732016,10incomplete maxerror0.9894177234928065. All3oldfloor failures alreadyclassified. Independent encoded85spacing and typed index2/86 anchors jointly support19missingcells;3D9suffix excluded. PR01-M2 separately evaluates5256byteFFbanktail02EB78..030000 beyond6unresolvedzeros;8978oldinstructions reconciled plus250fresh consumer/newcode instructions,4171mainreferences and bounded source windows yield0competing claims. Nine acceptedFFtail roles compared by actual end/layout evidence; no invented trailer/alignment.11group claim-local primary rejection review complete, including strong8pointer allocation counterclaim and differing accepted nonlinear/typed endpoint standards. Two ready leads PR01-L001/002,5275proposed,0accepted. Independent disassembly next after verification; map/source unchanged.

## Updated-policy redirect policy-resume-01 COMPLETE — ready combined batch
Retain round11-redirect-final and full-readable round11-commentary-final;513455/524288=97.93376922607422%classified,10833unknownbytes/137ranges/1384roles. PR01-L00119missing pitchcells of22unknowngroupbytes,3D9excluded; PR01-L0025256FFsuffixbytes,6zero prefixexcluded.5275proposed/0accepted; conditional ifbothaccepted5558unknownbytes. Changed claim-local inference and all11group primary rejection review persisted; no firstlead/quota stop. Independent disassembly next; not terminal exhaustion. Fresh inspect/internal+portable validation/structural+editedsourcebyteidentity,diagnostics[],readingfresh allpass.19handoffhashes/28editorialfiles/104legacyfiles/configuredbinary preserved(136protectedfiles); no ROM/map/source/runtime/toolkit modifications or rebuild. Current-source readabilitycomplete, finalarchiveauditpending. Primaryreview analysis/redirect/policy-resume-01/rejection-review.json; combinedproposals proposals.json; compacthandoff handoff.md; verification verification.json in same directory. No new toolkit issues found;TI001fixed,0active/blockers;analysis/toolkit-issues.md. Worker stops writes;nootherworkers/laterphaseslaunched.

## Round12 disassembly active milestone
Input round11-redirect-final / round11-commentary-final; independent policy-resume-01 PR01-L001/L002 review. All 19 current handoff hashes checked; entire 28-file commentary preserved before map/generation changes; 104 immutable legacy files and release binary inventoried. No runtime or toolkit changes. Evidence analysis/round12/preservation.json.


## Round12 disassembly COMPLETE — ROM incomplete
ROM classification: 98.94% (518730 / 524288 bytes classified; 5558 bytes unknown).

Accepted PR01-L001: 19 Static Data bytes in 19 exact cells of the split85word pitch table, ROM0708F3..07099D / uploaded ARAM0CEF..0D99. Independent raw-ROM full85word check plus fresh0984 integer/scaling decode, encoded85byte base displacement and independently accepted temporal endpoint indices2/86 jointly establish bounded storage. Every known/omitted word agrees within one integer quantum with seven-octave semitones. Known75 versus incomplete10 criterion is identical. Exact floor failures63/75/84 are already typed; they reject original uncapped floor generation only.14bit field masks;3FFF is inferred table cap, not hardware saturation. No original rounding, natural selection or newexecution claim. ThreeD9suffix bytes07099D..0709A0 remain unknown.

Accepted PR01-L002: 5256 Static Padding bytes ROM02EB78..030000 / CPU05:EB78..05:FFFF. Fresh source/rectangle consumer decode and independently exact23x17word end02EB72, maximalFFsuffix, independently rooted next-bank track source030000, nine accepted allocation-fill analogues and reconciled9228instruction/4171mainreference plus source-window counterchecks jointly support inferred bank fill. Uniformity and absence of references alone are insufficient. Sixzeros02EB72..02EB78 remain unknown; no trailer/historical linker/universal unread or saturation claim.

All1384 prior roles unchanged; reuse their review credits.20new roles and affected pitch/fill descriptions need separate full current-source commentary integration. Entire28file editorial tree copied before changed generation and remains identical; all19input handoff hashes verified beforehand,104legacy attempts/configuredrelease binary unchanged afterward. Fresh prior edited source and new exact round12-disassembly-final checkpoint rebuild byte-identically; diagnostics[]. Maps validated/saved/exported. Exact residual ledger:123ranges/5558bytes; bank totals below. No emulator creation, runtime/input/patch/snapshot/trace/coverage changes, ROM/toolkit edits or rebuild. Local draft Data execution views rejected safely and corrected per documented schema; no new toolkit issues found, TI-001 historically fixed, zero active findings/blockers; report analysis/toolkit-issues.md.

Connected continuation excludes pitchD900 neighbor cells; instrument/scaling consumers and downstream track allocation already classified. Sixzero and interior graphics gaps still lack independent ownership. New pitch/fill roles supply no new producer/source/context or changed productive gameplay prerequisite; unchanged routes not repeated. Prior11group claim-local rejection review retained at analysis/redirect/policy-resume-01/rejection-review.json. Easy gains complete for this pass; this is not terminal exhaustion or whole-ROM completion. Ordinary workflow next is commentary then redirect, with no separate final-investigation phase or percentage rule. Coordinator owns later phases; worker stops writes after handoff. Current prior-source readability verified; new-role integration and final editorial/archive acceptance remain pending.

|Physical bank|Code|Data|Padding|Classified|Unknown|
|---|---:|---:|---:|---:|---:|
|00|17099|7499|7972|32570|198|
|01|0|32768|0|32768|0|
|02|0|32768|0|32768|0|
|03|0|32768|0|32768|0|
|04|0|27648|5120|32768|0|
|05|0|27506|5256|32762|6|
|06|0|31104|1664|32768|0|
|07|0|28338|4430|32768|0|
|08|0|27840|4806|32646|122|
|09|0|32656|112|32768|0|
|0A|0|28058|4710|32768|0|
|0B|0|23233|9535|32768|0|
|0C|0|32768|0|32768|0|
|0D|0|27682|5086|32768|0|
|0E|2140|26885|0|29025|3743|
|0F|0|31279|0|31279|1489|


## Round12 commentary ACTIVE
Input round12-disassembly-final; editorial round11-commentary-final. Entire current commentary additionally copied/hash recorded under round12/commentary-input before fresh preparation; carried all1384 unchanged physical role source blocks, symbols and uncertainty notes.20 new roles and affected dependencies pending. No runtime/toolkit/input changes.

## Historical Round12 commentary interruption — resumed and superseded
Commentary remains interrupted after preparation/carry-forward, before full current-source integration and verification. Original worker round12_commentary failed model selection repeatedly across three goal turns; fresh same-phase context round12_commentary_resume also immediately failed: Selected model is at capacity. Both handles terminal, no competing writer. Preserve partial commentary and analysis/round12/commentary-input/commentary; do not reprepare over saved edits. Classification round12-disassembly-final remains518730classified/5558unknown. Goal blocked by external model service capacity, not ROM evidence or toolkit. Resume same commentary phase when capacity returns or user explicitly requests another model. No new toolkit issues found; report analysis/toolkit-issues.md.


## Round12 commentary ACTIVE
Input round12-disassembly-final; editorial round11-commentary-final. Entire current commentary additionally copied/hash recorded under round12/commentary-input before fresh preparation; carried all1384 unchanged physical role source blocks, symbols and uncertainty notes.20 new roles and affected dependencies pending. No runtime/toolkit/input changes.


## Round12 commentary COMPLETE — current source readable

All 1,404 classified roles are covered in readable source: 1,384 unchanged roles carried from the preserved commentary, plus 20 round12 roles integrated and described. All 123 unknown intervals (5,558 bytes) have explicit local uncertainty comments. Current source inventory has zero pending records; final editorial/archive audit remains pending for acceptance.

Fresh `commentary.py verify` returned `ok=true`, `byte_identical=true`; ROM SHA-256 `24b687f95bb9737d223738f13c00aeaa7d697fa9e2fd50597b81d0cfa2160daa`. `commentary.py read` refreshed `commentary/reading-after.asm`. Independent `run.py inspect` identified the unchanged 512 KiB LoROM, and `run.py checkpoint round12-commentary-final` validated the current portable map and returned `ok=true`, `byte_identical=true` (map SHA-256 `753c448fbe334505bb882e573b1b2fddd7615075a47eeda2e13342be8df81bac`). All 28 files in the preserved prior commentary copy, 104 protected historical files, current state/map against the round12 disassembly output, ROM, and configured release binary checks passed. Full proof: `analysis/round12/commentary-verification.json`; phase handoff: `analysis/round12-commentary-handoff.md`. No runtime or toolkit changes. No new toolkit issues found; `analysis/toolkit-issues.md`.


Round13 redirect audit: reviewed the policy-resume-01 rejection and decisive pitch/fill tests, plus round12 independent review and exact residual ledger. No changed premise or useful discriminator for remaining groups; no new leads or classification. Current state remains 518730/524288 (98.93989562988281%), 5558 unknown bytes across123 ranges, 1404 roles; current-source commentary complete, final editorial/archive audit pending. Terminal incomplete handoff: analysis/redirect/round13-redirect-audit/handoff.md; claim-local review: analysis/redirect/round13-redirect-audit/rejection-review.json. No runtime or ROM/toolkit/source/map changes. No toolkit issues found.

## Round14 redirect continuation — coordinate comparison
Input round12-commentary-final; structural map remains round12-disassembly-final. Tested a new, concrete possible link for the unresolved CPU $009AFB 64-word block: compared its full little-endian progression against the current terrain row-offset lookup at $009F87 and inspected the relevant source contract. Candidate values follow row stride $0400 / column stride $0040; terrain values begin 0,$0084,$0108 and differ. The adjacent sprite uploader has no reference to $009AFB. This excludes that specific link, not unrelated grid semantics or indirect consumers; no classification or ready lead resulted. Evidence: analysis/redirect/round14-coordinate-audit/terrain-table-comparison.json. Exact state remains 518730/524288 classified (98.93989562988281%), 5558 unknown bytes/123 ranges. ROM, map, source and toolkit unchanged. No new toolkit issues found; analysis/toolkit-issues.md.

Round14 redirect continuation — heading stride-two assessment: Reopened the 32 unknown odd bytes at physical 0x068C..0x06CA using the combined selector contract, full two-direction finite domain, all 32 accepted even heading cells, exact final cell, and adjacent code boundary. Static arithmetic establishes a complete 32-heading lookup at stride two, and all 32 intervening bytes are zero. It does not establish that those bytes are fields or padding owned by the lookup: the concrete competing model of one-byte heading cells interleaved with independent/reserved zero bytes satisfies every constraint equally. Retain the 32 bytes unknown; no ready lead absent an odd-offset consumer or independent storage-format link. Assessment: analysis/redirect/round14-coordinate-audit/heading-slot-assessment.json. Classification remains 518730/524288 (98.93989562988281%), 5558 unknown bytes/123 ranges; no runtime, ROM, map, source, or toolkit changes. No new toolkit issues found; analysis/toolkit-issues.md.

## Round14 redirect handoff — incomplete
Round14 tested a terrain-row-offset linkage for the CPU $009AFB block and a combined stride/domain/boundary model for the 32 odd heading bytes. Neither yields an ownership contract; both groups remain unknown. Coordinator-inspected terminal review and compact handoff: analysis/redirect/round14-coordinate-audit/terminal-review.json and handoff.md. Exact state unchanged at 518730/524288 classified (98.93989562988281%), 5558 unknown bytes in123 ranges. No new ready leads; the remaining claim-local rejection review remains valid on unchanged premises. No runtime, ROM, source, map or toolkit edits. No new toolkit issues; analysis/toolkit-issues.md.

## Round15 redirect continuation — audio directory and effect pointers
Inspected the complete supported DSP register initialization path and dynamic per-voice DSP writer, then reconstructed all16 stored low/high effect-sequence pointers. DIR register $5D is initialized once to $50; bounded voice writes use literal bases $00..$70 and offsets0..4,7, never $5D. None of the16 effect-pointer values enters the unresolved SPC ARAM $4425..$5000 interval; the highest named sequence starts $43EF and its mapped source ends at $4425. This excludes alternate-DIR and direct effect-pointer ownership through these contracts, while leaving other unknown formats/indirect consumers open. No bytes classified. Evidence: analysis/redirect/round15-audio-directory-audit/effect-directory-frontier.json. State remains518730/524288 classified (98.93989562988281%),5558 unknown in123 ranges. ROM/map/source/toolkit unchanged. No toolkit issues found; analysis/toolkit-issues.md.

Round15 redirect handoff: inspected and bounded the complete current DSP DIR write path and all16 stored effect-sequence pointer targets. There is one supported DIR=$50 initialization; dynamic voice writes cannot reach register $5D. No effect pointer enters unknown ARAM $4425..$5000; last named sequence ends exactly at its start. This closes only the alternate-directory/direct-effect-pointer hypothesis and leaves the 3035-byte interval unknown. Coordinator-reviewed rejection/handoff: analysis/redirect/round15-audio-directory-audit/rejection-review.json and handoff.md. State unchanged at518730/524288 classified,5558 unknown in123 ranges. No runtime, ROM, source, map or toolkit changes. No new toolkit issues; analysis/toolkit-issues.md.

## Round16 redirect continuation — supported song pointer roots

Reconstructed the low/high sequence pointers for all eight voices across explicitly supported song indices0..4 and13..15, using the established ROM-upload-to-ARAM mapping and the exact spc_load_song table layout. Zero of64 targets enter the unresolved pre-directory interval ARAM4425..5000; the largest is431F. This is a separate root set from round15's effect-pointer check. It supplies no stream extent and does not assert role from target values. Songs5..12 still lack independently supported selector/table-layout evidence, so their pointer-shaped cells do not open a parser or map proposal. Evidence: analysis/redirect/round16-song-pointer-audit/song-pointer-frontier.json; coordinator review: analysis/redirect/round16-song-pointer-audit/rejection-review.json. State unchanged at518730/524288 classified (98.93989562988281%),5558 unknown in123 ranges. No runtime, ROM, source, map or toolkit changes. No new toolkit issues; analysis/toolkit-issues.md.

## Round17 redirect continuation — full supported song stream union

Correction to round16: its selector-domain premise was stale because it cited the initial round01 handoff. Round03 independently established the ready-transition producer for songs5..12; preserve round16 unchanged as historical evidence, but do not reuse its claim that those songs lack a producer. Recomputed the full domain and compared exact parsed physical byte ranges, not only pointer starts: all16 songs /128 voice pointers are covered by the existing bounded parser reports, with no parsed source byte intersecting physical074029..074C04 (ARAM4425..5000). Highest parsed source end-exclusive is073F4E (ARAM434A), 219 bytes before the interval. See analysis/redirect/round17-supported-song-stream-union/song-stream-frontier.json and coordinator rejection review. This removes supported song-sequence streams as a consumer of that gap, but supplies no role for its bytes; other formats remain unresolved. No map changes; exact state remains518730 classified /5558 unknown in123 ranges. No new toolkit issues; analysis/toolkit-issues.md.

Verification for round17: a fresh ROM-byte comparison recomputed all128 split low/high pointer targets, checked all16 parser reports and their 128 voice stop records, and asserted zero source-range intersections with the unknown interval; result passed and input SHA256 remained24b687f95bb9737d223738f13c00aeaa7d697fa9e2fd50597b81d0cfa2160daa. Evidence/review/lead JSON parse successfully. No map, internal map state, assembly source, emulator state, ROM or toolkit change occurred, so no map edit/checkpoint or byte-identical rebuild was needed. Whole-ROM classification, final editorial/archive audit and final byte-identical verification remain incomplete.


## Round18 redirect — focused attract-loader boundary review

Input checkpoint round12-commentary-final / round12-disassembly-final; ROM SHA-256 unchanged. Current classification remains 518730/524288 (98.93989562988281%), 5558 unknown bytes in 123 ranges, 1404 classified roles. Reviewed the exact 64-byte unknown 0x45300..0x45340 (LoROM CPU 08:D300..D340) against the complete attract-display DMA consumer at [current commented source](round12/commentary-input/commentary/source/game.asm#L11295) and the adjacent data labels at [source bounds](round12/commentary-input/commentary/source/game.asm#L19184). The loader has exact source windows 0x40000..0x44D00, 0x44D00..0x45300, and 0x45340..0x45440; none intersects the gap. This excludes a third source in that loader and confirms the two neighboring transfer endpoints, but supplies no role or extent for the gap and does not exclude another consumer. No proposal or ready lead; retain all 64 bytes unknown. Compact claim-local review: [rejection-review.json](redirect/round18-attract-loader-gap/rejection-review.json); bounded evidence: [consumer-boundary.json](redirect/round18-attract-loader-gap/consumer-boundary.json). Other previously reviewed residual barriers and the corrected round17 song-domain conclusion remain unchanged; no broad source scan or runtime experiment was run. No map/state/source/ROM/toolkit changes. No new toolkit issues found; see [toolkit-issues.md](toolkit-issues.md).

## Round19 redirect — window-shape selector frontier

Input checkpoint `round12-commentary-final` / structural checkpoint `round12-disassembly-final`. For unknown physical `0x46260..0x4629A` (58 bytes, LoROM CPU `08:E260..E29A`), checked the complete `window_shape_index` write domain and `build_window_scanline_buffer` source extent against current source and accepted map. The selector is initialized with both bytes zero; all subsequent full-width stores use 0, 1, or 2, and byte-wide resets only clear the low byte. The exact three-entry directory targets physical `0x4629A`, `0x4639A`, `0x4649A`; builder consumes 256 bytes / 128 pairs per selected source. Thus the unknown interval cannot be a fourth source for this builder. This is a new consumer-specific constraint relative to round12's broader source-end/container rejection; it does not establish an independent role or extent. Retain all 58 bytes Unknown; no proposal or ready lead. Evidence and claim-local review: `analysis/redirect/round19-window-shape-frontier/window-shape-domain.json`, `rejection-review.json`, `handoff.md`. `analysis/leads.json` records round19 with no added/updated leads. State remains 518730 classified / 5558 unknown / 123 ranges / 1404 roles. No ROM/map/state/source/toolkit changes. No new toolkit issues found; `analysis/toolkit-issues.md`.

## Round20 coordinator correction — window source address bounds

Coordinator primary-source review found overbroad wording in round19: screen-clipped iterations can skip actual sample loads, although the loop still advances across 128 pair positions and caps possible Y offsets at 0x00..0xFF. Corrected scope: this establishes the maximum 256-byte address window per selected shape record, not that every invocation fetches every byte. The selector-domain result still excludes the unknown 0x46260..0x4629A interval as a fourth base in the three-record window-source table; no role/extent or map proposal follows. Correction evidence: [audit.json](redirect/round20-window-shape-readbound-correction/audit.json). Map/state/source/ROM unchanged; 518730 classified /5558 unknown. No new toolkit issues; see [toolkit-issues.md](toolkit-issues.md).

Verification: recomputed input SHA256; compared physical 0x3616..0x361F directory bytes to the three bank/address records; checked current map region IDs383/431 and confirmed no region overlaps 0x46260..0x4629A; enumerated all eight source references to `window_shape_index`; parsed evidence and lead JSON. All assertions passed. No classification or byte-identical rebuild run because ROM/map/source were unchanged.


## Round21 redirect — corrected premise for bounded grid table

Input checkpoint `round20-window-shape-readbound-correction`; structural checkpoint `round12-disassembly-final`. Reopened physical `0x1AFB..0x1B7B` (128 bytes; bank-00 LoROM alias `00:9AFB`) under a narrower claim than round14: a neutral Static 64-entry little-endian 8x8 affine offset-grid table, with consumer and purpose unresolved. Verified ROM SHA-256/size, exact neighboring accepted code endpoints/views, no map overlap, and all64 words against `word[n]=(n//8)*0x0400+(n%8)*0x0040`; all distinct and no exceptions. Exact bounds plus full progression jointly support a ready structural proposal; strongest alternatives are structured reserved/initializer data or a linear constants sequence, which leave active use and semantic purpose unresolved. Reachability not claimed. `R21-L001` proposes128 bytes for independent review;0 accepted/map changed. Evidence and compact rejection review: `analysis/redirect/round21-grid-role-review/evidence.json`, `rejection-review.json`, `handoff.md`. Map/state/ROM/source/toolkit unchanged; no runtime or rebuild. No new toolkit issues found; see `analysis/toolkit-issues.md`.


## Round22 disassembly — R21-L001 accepted

Independently rechecked ROM SHA-256, exact neighboring accepted bank-00 LoROM code endpoints/views, and all64 little-endian words over physical `0x1AFB..0x1B7B`. Accepted the neutral Static 8x8 affine offset-grid table proposal and annotated exactly128 bytes as `offset_grid_8x8` (confidence0.78). The annotation establishes the complete value layout only; consumer, purpose, natural selection, storage history, terrain identity and reachability remain unresolved. Review: [independent-review.json](round22/independent-review.json); redirect evidence: [evidence.json](redirect/round21-grid-role-review/evidence.json), [rejection-review.json](redirect/round21-grid-role-review/rejection-review.json).

`map.validate` passed; `map.save` wrote `analysis/state.json`; `map.export` wrote portable SNES JSON `maps/current.json`. `python3 run.py checkpoint round22-disassembly-final` passed with `ok=true`, `byte_identical=true` for all 524288 bytes; ROM SHA-256 24b687f95bb9737d223738f13c00aeaa7d697fa9e2fd50597b81d0cfa2160daa; map SHA-256 42c445c6d45e3f8d6a50c6d21251a10d2d1fe1563c44e4f1c743f9cdcd629ad6. Checkpoint source: [generated listing](../generated/round22-disassembly-final/game.asm); checkpoint map: [round22 map](../maps/round22-disassembly-final.json); verification: [round22-disassembly-final-verification.json](round22-disassembly-final-verification.json).

Current structural totals: 19239 Code, 450928 Data, 48691 Padding, 5430 Unknown across 122 exact ranges (518858/524288, 98.964310% classified). Per-bank code/data/padding/unknown and exact remaining offsets are in [round22-unknown-ranges.json](round22-unknown-ranges.json). Execution coverage did not change; no scenario/runtime sequence was run. Current-source commentary readability/editorial audit remains outstanding; ROM and toolkit unchanged. No new toolkit issues found; see [toolkit-issues.md](toolkit-issues.md).


## Round22 — commentary complete

Carried forward round12 source and review credit after preserving the old commentary tree in [round22/commentary-round12-original](round22/commentary-round12-original/). Refreshed the commentary baseline against round22 map/state and integrated the 128-byte static Data role `offset_grid_8x8` at physical `0x001AFB..0x001B7B` (bank 00 alias `00:9AFB..00:9B7A`). The source documents all 64 little-endian cells as an 8x8 affine offset grid (row step `$0400`, column step `$0040`); consumer, purpose, coordinate system and reachability remain unresolved. No new gameplay scenario/input or runtime observation.

Current source review: all 1,527 inventory entries are reviewed, all Code entries have reviewed units, the new table has its own reviewed unit, and all source locations resolve. Editable source mapping equals portable map. Fresh `python3 /root/romatlas/scripts/commentary.py verify /root/super-off-road` passed byte-identical with the configured release binary; ROM SHA-256 `24b687f95bb9737d223738f13c00aeaa7d697fa9e2fd50597b81d0cfa2160daa`. `commentary.py read` refreshed `commentary/reading-after.asm`. Exact proof: [round22 commentary verification](round22-commentary-verification.json); handoff: [round22 commentary](round22-commentary-handoff.md). Classification remains 518,858/524,288 (98.964310%): 5,430 bytes in 122 unknown ranges. Final editorial/archive audit remains pending; this phase does not claim whole-ROM completion. No new toolkit issues found; TI-001 remains historically fixed, no active issues; [report](toolkit-issues.md).


## Round23 redirect — offset-grid direct-window consumer audit

Input `round22-commentary-handoff` / structural checkpoint `round22-disassembly-final`. Recomputed the current complement from the portable map and matched all122 intervals in `analysis/round22-unknown-ranges.json`: 5,430 unknown bytes (bank00 70/43 ranges; bank05 6/1; bank08 122/2; bank0E 3,743/74; bank0F 1,489/2). ROM SHA-256 and map SHA-256 remain unchanged. Followed the newly accepted `offset_grid_8x8` as an unresolved consumer lead with a broadened direct-window scan: searched current Code source for literal 16-bit operands within physical alias window `00:9AFB..9B7A` and its bank80 mirror, and symbolic uses of the table label. No direct operand or symbolic consumer was found. This does not rule out computed/indirect access or alter the accepted Static Data role; purpose, use and reachability remain unresolved. No proposal/ready lead. No map/source/state/ROM/toolkit changes or runtime/rebuild. `analysis/leads.json` records round23; evidence and claim-local review are in `analysis/redirect/round23-grid-consumer-window/`. No new toolkit issues found; TI-001 remains historically fixed, zero active issues/blockers; see `analysis/toolkit-issues.md`.


## Round24 coordinator residual rejection review

Input `round22-commentary-handoff` / structural checkpoint `round22-disassembly-final`. Recomputed the unknown complement from `maps/current.json`: 5,430 bytes in122 ranges; per-bank counts and exact offsets match `analysis/round22-unknown-ranges.json`. ROM and map hashes remain `24b687f95bb9737d223738f13c00aeaa7d697fa9e2fd50597b81d0cfa2160daa` and `42c445c6d45e3f8d6a50c6d21251a10d2d1fe1563c44e4f1c743f9cdcd629ad6`. Revisited the attempt05 group rejections under the updated no-caller-prerequisite standard, explicitly correcting the former coordinate-like-block scope: R21-L001 is now accepted for the 128-byte neutral 8x8 table. Round23's direct-window negative only narrows one consumer route and adds no allocation/extent evidence for other groups. The remaining exact groups and their primary-evidence review are documented in [round24 rejection review](redirect/round24-residual-recheck/rejection-review.json), with compact handoff at [round24 handoff](redirect/round24-residual-recheck/handoff.md). No other current group gained a supported role/bounds/context discriminator; no ready lead or concrete unchanged-evidence method remains. This is a review-only nonterminal incomplete result, not a whole-ROM completion claim. No map/source/state/ROM/toolkit edits, runtime, or checkpoint/rebuild. No new toolkit issues found; TI-001 remains historically fixed, with no active issues; see [toolkit-issues.md](toolkit-issues.md).

## Coordinator stop decision — incomplete

After inspecting the round24 rejection review and its decisive primary evidence, I accept a stop for this pass as incomplete: no concrete discriminatory method or ready lead remains on unchanged evidence. This is not inherited from attempt05; its overbroad coordinate-block rejection was corrected and 128 bytes were subsequently classified under the updated static-evidence rule. Current status remains 518,858/524,288 classified (98.964310%), with exactly 5,430 unknown bytes in 122 ranges: bank00 70/43 ranges, bank05 6/1, bank08 122/2, bank0E 3,743/74, bank0F 1,489/2. Final structural checkpoint `round22-disassembly-final` and fresh commentary verification both passed byte-identical over 524,288 bytes; ROM SHA-256 `24b687f95bb9737d223738f13c00aeaa7d697fa9e2fd50597b81d0cfa2160daa`, map SHA-256 `42c445c6d45e3f8d6a50c6d21251a10d2d1fe1563c44e4f1c743f9cdcd629ad6`. Commentary readability review is complete for the current source and reading copy refreshed; final editorial/archive audit remains pending. The portable map is valid at this checkpoint. Exact decision and reviewed primary artifacts: [coordinator terminal decision](redirect/round24-residual-recheck/coordinator-terminal-decision.json). No new toolkit issues found; see [toolkit-issues.md](toolkit-issues.md). Whole-ROM completion is not claimed.

## Coordinator stop decision — incomplete

After inspecting the round24 rejection review and its decisive primary evidence, I accept a stop for this pass as incomplete: no concrete discriminatory method or ready lead remains on unchanged evidence. This is not inherited from attempt05; its overbroad coordinate-block rejection was corrected and 128 bytes were subsequently classified under the updated static-evidence rule. Current status remains 518,858/524,288 classified (98.964310%), with exactly 5,430 unknown bytes in 122 ranges: bank00 70/43 ranges, bank05 6/1, bank08 122/2, bank0E 3,743/74, bank0F 1,489/2. Final structural checkpoint `round22-disassembly-final` and fresh commentary verification both passed byte-identical over 524,288 bytes; ROM SHA-256 `24b687f95bb9737d223738f13c00aeaa7d697fa9e2fd50597b81d0cfa2160daa`, map SHA-256 `42c445c6d45e3f8d6a50c6d21251a10d2d1fe1563c44e4f1c743f9cdcd629ad6`. Commentary readability review is complete for the current source and reading copy refreshed; final editorial/archive audit remains pending. The portable map is valid at this checkpoint. Exact decision and reviewed primary artifacts: [coordinator terminal decision](redirect/round24-residual-recheck/coordinator-terminal-decision.json). No new toolkit issues found; see [toolkit-issues.md](toolkit-issues.md). Whole-ROM completion is not claimed.


## Round25 redirect — heading lookup stride-window reassessment

Input `round22-commentary-handoff` / structural checkpoint `round22-disassembly-final`. Reassessed only the exact32 odd unknown bytes inside physical `0x068B..0x06CB` (64-byte window), confronting round14 and policy-resume rejection reviews. In the accepted bank-00 source at `commentary/source/game.asm:1041-1074`, `selected_track_id & $0F` is doubled; nonzero reverse state adds `$20`; the M=8 indexed load therefore selects exactly32 even offsets `0..$3E` from `$068B`. The final heading byte is `$06C9`; its two-byte-stride slot ends at `$06CB`, where accepted code region124 begins. All32 unknown interleaved odd bytes are `$00`; first heading cell `$068B` is accepted Data region884.

Changed inference: the candidate is only the bounded 32-slot Static Data window; no high-byte, padding, field, word-value, write/use, reachability or source-layout semantics are assigned to odd bytes. This is materially narrower than round14’s rejected semantic ownership claim. The strongest competing layout remains one-byte headings at even offsets plus independent/reserved odd zeros, so the proposed storage-window role is ready for independent review, not pre-accepted. `R25-L001` proposes exactly the32 current unknown bytes; no map/source/state/ROM edits, runtime, checkpoint, or rebuild. Evidence, claim-local rejection review and recipe: `analysis/redirect/round25-heading-stride-window/{evidence.json,rejection-review.json,handoff.md}`. Current state remains518858 classified /5430 unknown /122 ranges pending review. No toolkit issues found; see `analysis/toolkit-issues.md`.


Round26 disassembly: accepted R25-L001 as a bounded static two-byte-stride start-heading storage window, annotating only the 32 odd unknown bytes as Static Data. Selector mask/doubling/reverse arithmetic and bank-00 DB=0/M8/X8 load establish the 32-slot address domain; region 884 is the first selected cell and region 124 begins exactly at the window end. Internal slot semantics/reachability remain unresolved; sparse one-byte lookup plus separate zeros remains viable at that finer level. No gameplay/runtime/input/trace/snapshot activity. ROM classification: 98.97% (518,890 / 524,288 bytes classified; 5,398 bytes unknown). Per-bank code/data/unknown and exact residual ranges are in analysis/round26/independent-review.json. Map validated, saved analysis/state.json, exported maps/current.json; round26-disassembly-final exact byte-identical verification passed for all 524,288 bytes. Handoff: analysis/round26/handoff.md. No new toolkit issues found; analysis/toolkit-issues.md. Commentary handoff pending.


## Round27 — commentary complete

Input: `round26-disassembly-final` / `analysis/round26/handoff.md`. Preserved the full round22 commentary tree at `analysis/round27/commentary-round22-original/`, refreshed the commentary baseline to round26, and carried all prior source prose and review credit forward. Integrated 32 interleaved-byte Static Data regions at physical `0x068C..0x06CA` with mechanical labels/comments describing only the bounded two-byte-stride start-heading storage-window role and stored `$00` values; slot-internal semantics remain unresolved. Updated selected-byte prose and the active annotations/uncertainty notes. No high-field, padding, word, use/write, reachability or source-layout claims were added.

The current source covers all 1,437 classified map regions. All1,527 review scaffold entries are `reviewed`, every source location resolves, and all Code entries retain reviewed units. The remaining5,398 unknown bytes are in90 ranges: bank00 38, bank05 6, bank08 122, bank0E 3,743, bank0F 1,489. Classification is518,890/524,288 =98.970413%; exact map/residual evidence remains in `analysis/round26/independent-review.json`. Fresh configured-binary `commentary.py verify` passed byte-identical over524,288 bytes and `commentary.py read` refreshed the reading copy. ROM SHA-256 remains `24b687f95bb9737d223738f13c00aeaa7d697fa9e2fd50597b81d0cfa2160daa`. See [round27 commentary verification](round27-commentary-verification.json) and [handoff](round27-commentary-handoff.md). No map/state/ROM/runtime/toolkit changes. No new toolkit issues found; TI-001 remains historically fixed with no active issues; see [toolkit-issues.md](toolkit-issues.md). Final editorial/archive audit remains pending; whole-ROM classification is incomplete.
