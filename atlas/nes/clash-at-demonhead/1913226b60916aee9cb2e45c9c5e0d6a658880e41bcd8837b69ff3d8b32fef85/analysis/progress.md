# Disassembly progress

## Round 001 — disassembly pass complete, ROM incomplete

ROM classification: 56.79% (148886 / 262160 bytes classified; 113274 bytes unknown).

Baseline and pass-001 both rebuild byte-identically. Internal validation passed; current portable map is maps/current.json, checkpoint map maps/pass-001.json, source generated/pass-001/game.asm, reports analysis/baseline-verification.json and analysis/pass-001-verification.json. Input SHA256 remains 1913226b60916aee9cb2e45c9c5e0d6a658880e41bcd8837b69ff3d8b32fef85.

All 523 classified regions carry structural evidence. Static header/CHR/vectors, exact observed-fetch code under recorded banks, initial vector static paths and seven uniform FF generic-data spans are classified. Uniform spans were checked against neighbors and observed code conflicts; no unsupported semantic fill/table purpose claimed. Reachability is separate: CHR/hardware/static regions need no natural caller claim; shortcut-only fetch evidence is artificial. Instruction execution coverage counts unique fetched starts, not classified-byte coverage. Per-scenario trace summaries and bank-aware root pages are retained under analysis/round001; disabled-navigation intervals make no fetch claims.

|Bank|Code|Data/image/header|Classified|Unknown|
|---|---:|---:|---:|---:|
|Header|0|16|16|0|
|PRG 00|66|0|66|8126|
|PRG 01|299|0|299|7893|
|PRG 02|1131|0|1131|7061|
|PRG 03|0|0|0|8192|
|PRG 04|610|0|610|7582|
|PRG 05|742|0|742|7450|
|PRG 06|75|0|75|8117|
|PRG 07|130|243|373|7819|
|PRG 08|259|1959|2218|5974|
|PRG 09|947|164|1111|7081|
|PRG 10|658|0|658|7534|
|PRG 11|0|0|0|8192|
|PRG 12|1852|0|1852|6340|
|PRG 13|1779|1|1780|6412|
|PRG 14|745|3589|4334|3858|
|PRG 15|2067|482|2549|5643|
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

Natural title, route1 map/action, jump/fire, inventory and Tom Guycot dialogue visited. Later-route and finale shortcuts attempted from a clean snapshot; $0342-only edits failed to reload terrain. Broad progression remains incomplete, tracked in analysis/scenarios.json. Sources and version caveats: analysis/gameplay.md.

No preexisting leads existed. L001 route initialization and L002 dialogue progression are useful redirect questions; L003 supplies static closure/dispatch frontier. No terminal exhaustion claimed. A prospective next classification batch needs consumer/transition facts; do not replay the same map-resume patch or unchanged A dismissal. All pass-local evidence and scripts retained.

No toolkit issues found; see analysis/toolkit-issues.md. Documented execution timeouts retained actual partial state; zero detail capacity intentionally dropped event detail while aggregates stayed bounded (aggregate_dropped=0 in retained summaries). Local broker/socket experiment failed at OS sandbox socket permission and was superseded by single-process Python RPC drivers; this is not a toolkit defect. No live server is needed for handoff: start configured run.py serve, load analysis/state.json explicitly, emulator.start, then snapshot.import a cited durable file and reassert intended buttons.

Retained root-page execution metrics: 5260 unique accepted instruction starts across 9223 effective contextual roots; these are separate from 11,360 classified code bytes. Final disk free2.5GiB; round001 evidence63MiB, scratch92KiB. No cleanup performed.

## Round 001 — commentary current-source readability complete

Reviewed all current classified source: 513 code fragments (11360 bytes), ten data/image regions and all explained unknown intervals. Classification remains 148886/262160 bytes (56.79%), with 113274 bytes unknown. Commentary review retains all 940 physical scaffold entries and 243 compact routine/structure units. Full original source inspection plus internal-entry/symbol consistency passes produced functional routines/branches, shared RAM/hardware/voice-field symbols, consumer table equates and local mechanical explanations. No classified readability range is deferred.

Source: commentary/source/game.asm; refreshed reading copy: commentary/reading-after.asm. Fresh exact rebuild passed with source SHA256 2d174f96a7ee27a2d52638f68e91325bae8d3f690f4a0fb7a699e9fdd5950690 and unchanged input SHA256 1913226b60916aee9cb2e45c9c5e0d6a658880e41bcd8837b69ff3d8b32fef85. Verification: commentary/verification.json. Frozen baseline/state compare equal to current map/state. Unknown table extents, game-specific actor/status/item identities, special-mode branches and later progression remain unresolved explicitly in commentary/uncertainties.md. The corrected MMC1 interpretation distinguishes control/CHR0/CHR1 ($9fff/$bfff/$c000); split services change CHR1. Final editorial audit remains pending; no audit/archive/publication performed. Compact phase handoff: analysis/round001-commentary-handoff.md.

## Round 001 — redirect complete, combined batch ready

Input/output checkpoint unchanged pass-001. Consumed L001–L003; produced supported ready L001–L006 batch in analysis/round001-redirect-handoff.md and analysis/redirect/round001-20261126-a/proposals-final.json. Proposed union 49104 currently unknown bytes, none accepted; actual classification remains 148886/262160 with 113274 unknown. Independent static closure, dialogue pool, bank0/sprite/scene/audio data and later-route/final-state observations are included. Natural dialogue dismissal recovered; artificial route initialization and final special handler reached THE END. Exact patches, context, proof and captures are linked in handoff/attempt. Preservation check confirms ROM/map/state/source unchanged. Independent avenues were investigated and deferrals justified; no terminal exhaustion claimed. Next phase round002 disassembly for independent batch review; current commentary gate complete, final editorial acceptance pending. No toolkit issues found; no archive, cleanup or publication.

## Round 002 — independent disassembly pass complete, ROM incomplete

ROM classification: 76.27% (199961 / 262160 bytes classified; 62199 bytes unknown).

Input pass-001; output **pass-002-final**. Accepted 51075 new bytes: independently reviewed redirect union49104; new-context branch continuation1226; connected epilogue helper code215, sprite records30, attribute template32 and epilogue pointer/repeated-word/text block468. Internal map.validate passed. Fresh maps/pass-002-final.json and generated/pass-002-final/game.asm rebuild byte-identically; report analysis/pass-002-final-verification.json. Intermediate checkpoints pass-002/pass-002-connected are retained. Current state/map saved explicitly to analysis/state.json and maps/current.json. Counts come from unique structurally classified internal-state bytes; exact complement 363 intervals in analysis/round002/unknown-ledger.json.

Independent policy/boundary/context review: analysis/round002/review.md. Fresh4187-node decode and conservative predecessor walk supports all1493prior static candidates; fresh match1459saved observed contextual records; data/code and inconsistent instruction overlaps zero. New-root continuation1523roots in two bounded batches produced525new static instructions/1226bytes, independently predecessor-checked. Dialogue proof now checks firstFF for every string, not merely final terminators. Large noncode payloads keep generic Data roles where detailed ownership remains unresolved. Extra epilogue consumer proof localized11strings; repeatedBB64words point at following code and are not four more strings. Structural inference never becomes Observed by association.

|Bank|Code|Data/image/header|Classified|Unknown|
|---|---:|---:|---:|---:|
|Header|0|16|16|0|
|PRG 00|66|8110|8176|16|
|PRG 01|334|7858|8192|0|
|PRG 02|2159|3744|5903|2289|
|PRG 03|0|8192|8192|0|
|PRG 04|643|0|643|7549|
|PRG 05|859|0|859|7333|
|PRG 06|463|7206|7669|523|
|PRG 07|1607|1399|3006|5186|
|PRG 08|643|1959|2602|5590|
|PRG 09|959|7109|8068|124|
|PRG 10|1172|0|1172|7020|
|PRG 11|0|60|60|8132|
|PRG 12|3547|0|3547|4645|
|PRG 13|2684|1|2685|5507|
|PRG 14|968|3757|4725|3467|
|PRG 15|2892|482|3374|4818|
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

Consumed L001–L006 verified; historical failures preserved in leads.json. L001 initialized routes and42record extent; L002 full dialogue/window structure plus saved natural dismissal; L003 static closure; L004 bank0 tile records; L005 sprite/scene/audio payloads; L006 artificial ending and connected helpers. Produced L007 open redirect question for selector/entry-contract and mixed-gap classification. All result/retry details in leads.json and round002-handoff.md. Broad later-route/terminal visits reused saved scenariosS008–S010; no unchanged gameplay experiments repeated. Artificial stage/final/ending observations do not establish natural reachability.

Earlier full commentary source is preserved byte-for-byte at commentary/source/game.asm (SHA2562d174f96a7ee27a2d52638f68e91325bae8d3f690f4a0fb7a699e9fdd5950690) and commentary/reading-after.asm (SHA256b21f884f1672219c683f194f71be33d1cc77b0f5768b88335dd74f170ffeb567). The new generated source needs commentary merge/review before claiming current-source readability complete; final editorial gate remains pending. No archive, cleanup, publication or toolkit edit. No toolkit issues found; see analysis/toolkit-issues.md. Input ROM unchanged, hash verified. Stop incomplete at this substantive batch handoff; no terminal exhaustion claim.


## Round002 commentary successor — current-source readability complete

The preceding disassembly handoff is historical. Commentary has merged/reviewed pass-002-final and passed a fresh byte-identical rebuild, with refreshed reading copy. All4193scaffold entries/636units reviewed; no classified readability gaps deferred. Classification remains199961/262160,62199unknown. Prior full commentary preserved at analysis/round-002/commentary-pass001. Detailed successor scope and source hashes: analysis/round002-commentary-handoff.md; review/verification in commentary/. L007 updated with source evidence; final editorial audit pending.

## Round002 redirect — active research checkpoint

Readability handoff met. Unique attempt analysis/redirect/round002-20261126-a preserves new static methods and primary decoder/control evidence. Conservative RAM-indexed-write/directcallee closure plus independently proved bank/stack-safe helper returns proposes5172distinct unknown bytes; optional explicit-mode3 CHR refinement adds429bytes and selectedbank wrapper transfer adds348bytes (dedup pending). Independent exact menu/status consumers support324data bytes. No map/state/source changes. Actual classification remains199961/262160;62199unknown. Active frontier: inline-pointer PPU caller return+5, literal MMC1 control writer contracts, remaining indirect actor dispatch. This is a same-phase resumable checkpoint, not terminal exhaustion; see attempt progress.md for concrete prerequisites and counterexamples.

## Round002 redirect — coherent batch ready

Research supersedes the active frontier above. L007/L008ready;31244distinctunknownbytes proposed (23976Code,7268Data),0accepted. Currentmap remains199961classified/62199unknown; ifallaccepted30955unknown remain. Conflictchecks zero. Full handoff analysis/round002-redirect-handoff.md; evidence/proposals/exact363-rangeledger in analysis/redirect/round002-20261126-a/. Static contract/conditional entry assumptions and EB2D two-byteinlinepointer correction are explicit. All96actorbehavior/constructor entries and50inlinepools plus identified independent menu/player/terrain/container avenues inspected reasonably. Remaining specificbank/frame/pointerbounds and honest mixed-gap ambiguity documented; notterminalexhaustion. Source/map/state/ROM unchanged, prior evidence and gameplay routes preserved, no publication/cleanup/toolkit edits.

## Round003 — independent disassembly complete, ROM incomplete

ROM classification: 88.19% (231210 / 262160 bytes classified; 30950 bytes unknown).

Input pass-002-final; output pass-003-final. Accepted31249bytes (23975Code,7274Data) from coherent L007/L008 connected batch. Fresh10302contextual instruction decodes (10222physical starts after splitting seven bootstrap routines), exact code/data conflict checks and consumer/domain/return/frame review passed. Detailed decision and limits: analysis/round003/review.md; primary fresh checks structural-review.json and consumers.json. One Code claim excluded at113327 and replaced by independently consumer-proved six-byte comparisonData113327..113333, adding5bytes beyond proposedunion; compare-data-proof.json/brk-exclusion.json. Reset post-switch instruction.after metadata was freshly normalized under its correct existing proposed view. Connected six-byte scroll/scene comparison table proved with literalX0/1 callers andY3..0 loop; impossible LDA1/STA/BNE fallthrough rejects proposedBRK. Generic proof stack predicate omitted RAM mirrors, but concrete mirror-intersection audit found zero affected writes; no batch bytes invalidated and no toolkit defect inferred.

Internal map.validate passed; analysis/state.json saved and maps/current.json exported. Fresh checkpoint maps/pass-003-final.json, generated/pass-003-final/game.asm and analysis/pass-003-final-verification.json passed byte-identical rebuild. Exact complement 313intervals in analysis/round003/unknown-ledger.json. Counts derive from unique internal structurally classified bytes, including all16header and131072CHR bytes; raw export fallback/Unknown never count. Evidence coverage all classified regions; existing comments preserved; 523regions lack comment fields (evidence coverage and full-source editorial review are separate). Structural provenance counts are in counts.json; observed execution unchanged, no new runtime observations/reachability.

|Bank|Code|Data/image/header|Classified|Unknown|
|---|---:|---:|---:|---:|
|Header|0|16|16|0|
|PRG 00|75|8117|8192|0|
|PRG 01|334|7858|8192|0|
|PRG 02|2704|3833|6537|1655|
|PRG 03|0|8192|8192|0|
|PRG 04|1726|1151|2877|5315|
|PRG 05|2031|332|2363|5829|
|PRG 06|516|7220|7736|456|
|PRG 07|2362|1399|3761|4431|
|PRG 08|1636|6524|8160|32|
|PRG 09|959|7109|8068|124|
|PRG 10|5748|449|6197|1995|
|PRG 11|6701|370|7071|1121|
|PRG 12|6864|85|6949|1243|
|PRG 13|4115|273|4388|3804|
|PRG 14|1923|3757|5680|2512|
|PRG 15|5277|482|5759|2433|
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

L007/L008 verified; L009 opens specific residual pointer/bank/frame/mixed-boundary questions after commentary. Supported graph extends well beyond initial copy-loop lead, including all96actor entries,50inlinepools367records and related ordinary helper paths. Saved broad later-route/final/THE END and natural dialogue scenarios retained without unchanged replay; artificial observations remain artificial. Concrete deferrals are linked in analysis/round003-handoff.md; no terminal exhaustion claim.

Current commentary source preserved byte-for-byte SHA2569c658e925f43c7810444b822d7dc780dfda4f3859578d9347702694053f49a69; full newly classified source requires commentary merge/review. EB2D correction mandatory: TWO inline fallback pointerbytes, contactnonzero/nonFF continuation at callingJSR+5, contact0 nonlocalRTS, FFpointerjump; never decode inline words as instructions. Final editorial audit pending. InputROM hash unchanged. No cleanup/deletion, toolkit edit, archive or publication. Toolkit:1missingfeature TK001 (atomicbatchannotation),0confirmed/suspectedbugs,0blockers; analysis/toolkit-issues.md retained.

## Round003 commentary completion

Current-source readability complete for pass-003-final: 231210 classified/262160 total, 30950 unknown. Preserved pass002 commentary at `analysis/round-003/commentary-pass002/`, carried 3827 unchanged classified blocks, reviewed all added 23975 Code bytes (465 groups) and 7274 Data bytes (117 structures). All 14481 scaffold entries and 1516 distinct units have current source locations; 313 unknown boundaries remain honestly explained. No readability bytes deferred. `analysis/round003/commentary-frontier.json` is exact.

Source corrects EB2D's two-byte fallback and JSR+5/nonlocal paths, E06C/E0DC's grandparent frame adjustment, mode-three/stack/bank contracts and the six comparison bytes at 113327..113333. Fresh byte-identical source verification and reading refresh complete; source SHA256 `e01e1c861b2b444d1d831f4700e5a740fb77c2821c43190caabc8175d1219e32`. See `analysis/round003-commentary-handoff.md`. Final editorial audit remains pending; whole-ROM classification is incomplete. L009 retains exact residual questions. No toolkit edits, deletion/cleanup, archive or publication.

## Round003 redirect ready batch

Input pass-003-final unchanged:231210classified/262160total,30950unknown in313intervals. L009ready with11351proposed distinct unknown bytes (1138Code/10213Data),0accepted;19599unknown if all independently accepted. Unique attempt analysis/redirect/round003-20261126-a; final proposals.json, proposal-summary.json, unknown-ledger.json, handoff.md and deferred-avenues.json. Compact handoff analysis/round003-redirect-handoff.md.

Independent bounded consumer/format/pattern proofs recover menu/scene/motion/audio/text data; complete arithmetic and accepted-dispatch copies have explicit static contracts. Corrected prior seen=set(cache) premise with accepted-branch counterexample103181->103183; cached decoding is not visited evidence. Sound23 proof repairs locally contradictory empty-queue assumptions using actual three-voice scripts and a running/inactive invariant at two audited repeat sites; no universal/natural caller claim. Copied I/O block localized to190complete helper bytes; malformed7byte fragments and unknown source counterparts retained. Accepted/proposed instruction/data boundaries freshly audited, conflicts0. ROM/source SHA unchanged and current map byte-equals saved checkpoint. No runtime replay, toolkit change, cleanup/deletion, archive or publication. Ordinary combined ready batch; no terminal exhaustion claim. Independent disassembly then full current-source commentary required; final editorial audit pending.

## Round004 — independent disassembly complete, ROM incomplete

ROM classification: 92.52% (242561 / 262160 bytes classified; 19599 bytes unknown).

Input pass-003-final; fresh output pass-004-final. L009 verified in full:11351new Static bytes (1138Code/10213Data),143annotations (48contiguous code groups/95data regions). Independent569contextual decodes, physical mapping/bounds/role checks and19477existing/proposed instruction views with5151direct targets, zero control-to-Data conflicts. Additional137266functional arithmetic checks with initialcarry1/nonzero scratch passed; reuse documented bounded interpreter, separate from hardware execution. review.md, structural-review.json, control-audit.json, consumers.json, arithmetic-checks.json and data-review.json preserve review. All direct targets from new Code already classified; no unreviewed supported direct continuation. See analysis/round004/review.md for exact static calling/precondition/format evidence and localized uncertainty.

Intentional74056BIT/74057ADC alternate paths retained in fresh instruction evidence and map comment; three physical bytes counted once. Existing74688BIT/74689LDA preserved. Corrected actual-visit/cache distinction independently confirmed103181branch ordinaryfallthrough103183..103195exclusive. Firstsound23proof follows actual immediatequeue23prefix; repeat42849/42861use running/inactivevoice invariant and sevenhelperwriteaudit, never reused inactivepremise. Other audio sites remain barriers. Exact204byte I/O duplicate promotes only190complete bytes; four malformed7byte fragments remain Unknown. Seven PPU-format streams567bytes independent pattern support Data while inline-call-shaped origin code remains Unknown. Sprite26frames662bytes,17inventoryrecords238bytes,20x25and19x6tile rectangles and32entry sequences have independent extent mechanisms; remaining numeric/tile/OAM/text/pointer patterns use honest generic Data after full local role inspection. No invented natural selectors, source layouts or full waveform extents.

map.validate passed, analysis/state.json saved, maps/current.json exported. Fresh maps/pass-004-final.json and generated/pass-004-final/game.asm validate and rebuild byte-identically (analysis/pass-004-final-verification.json); current map byte-equals checkpoint. Counts from unique evidence-backed nonUnknown internal physical bytes; all16header/131072CHR included; raw fallback excluded. Exact complement 227intervals in analysis/round004/unknown-ledger.json. Internal kind/provenance/per-bank counts: analysis/round004/counts.json. Execution coverage unchanged; no new Observed or natural execution. Saved initialized later-route/finalscene/artificialTHE END and naturaldialogue scenarios reused without unchanged replay.

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

L009verified; L010opens exact residual pointer/bank/frame/audio/mixedboundary questions, primary deferred-avenues.json and refreshed complement. Supported connected batch finished; not a terminal exhaustion claim. Current commentary/source/game.asm preserved SHA256 e01e1c861b2b444d1d831f4700e5a740fb77c2821c43190caabc8175d1219e32, including previous complete readability work and EB2Dcorrection. Newly generated current source now requires full commentary merge/review; final editorial audit pending. All historical evidence/review/source preserved. Disk inspected (~2GiB free); no scratch directory or cleanup/deletion. ROM unchanged. No toolkit edits/archive/publication. Toolkit: existing TK001 atomicbatchannotation missingfeature retained;0newbugs/0blockers, analysis/toolkit-issues.md.

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

# Round009 current-source readability

Complete over pass-009-final:262140/262160 bytes classified;20 bytes in14 intervals remain unknown. Preserved the entire prior commentary at `analysis/round-009/commentary-pass008/` and carried14667 unchanged classified entries with sound review credit. All14686 scaffold entries and2404 distinct routine/structure units are reviewed with current source locations. No classified readability range remains deferred; final editorial acceptance remains pending and audit was not invoked.

Reviewed all34 new Static Code bytes in five annotations/16instructions: four supplied-A PPUCTRL mask-and-commit entries and one actor-field assignment. Source names the actual fields and hardware/shadow setter, documents preserved registers and matched-frame/bank/hardware contracts, and states that zero would reset the actor fields without asserting an actual A=0 entry. No natural caller or original missing opcode is inferred. Narrow unknown prefixes remain separate; stale neighboring malformed-fragment descriptions were corrected. D107/shop16/240/ending/timer/FDE9 and other reviewed corrections remain intact.

Coherent edited and fresh final byte-identical rebuilds passed; reading-after.asm refreshed. Full scaffold/code-unit/bounds/current source locations/map-state baseline/frozen prior/input-ROM checks passed. Locations are `game.asm:line` relative to commentary/source, checked using cached line counts. Source SHA `92be237214f38f63ef84c959375061ed704ee60ac4ff4944299502ad43474f83`. Exact inventory/dependencies and compact handoff are in `analysis/round009/commentary-frontier.json`, `commentary-dependencies.json` and `commentary-handoff.md`. L015 remains open for exact residual reasoning; no new execution, terminal exhaustion or whole-ROM completion is claimed. Toolkit: existing TK001 missing atomic-batch feature request,0newissues/0blockers; see analysis/toolkit-issues.md. No toolkit edits, cleanup/deletion, archive or publication occurred.

## Round009 redirect — terminal incomplete accepted; final acceptance active

Input/output pass-009-final unchanged. L015 reviewed-incomplete; no ready lead. Exact20 Unknown bytes/14intervals remain after primary rejection review. Redirect handoff and review: analysis/redirect/round009-20261126-a/handoff.md and residual-rejection-review.json. Coordinator independently inspected actual bytes, local decodes, synthetic return mechanics and neighboring data bounds; accepted scope-limited terminal-incomplete reasoning in analysis/coordinator-rejection-review.md. This is not blocked or whole-ROM completion.

Current phase: final editorial acceptance. One source uncertainty-wording repair only; source SHA4201260c152256ce179e63bb4ebd96f52adfd7c7e9e2a5e8e3dcfbea4170566d, map/state unchanged. Official audit session2471 owned by final_acceptance was confirmed live after339seconds; retain/poll that handle, never restart from observation timeout. Full cached-line independent location check passed29840locations; official audit retains all checks and fresh rebuild. Resume frontier: analysis/coordinator-frontier.json. Remaining gates: official final audit/reading completion, final handoff, archive-options document accounting and coordinator completion audit. No archive, cleanup, toolkit edits or publication authorized/performed. Toolkit TK001/TK002 performance reports,0blockers.

## Coordinator final terminal handoff — workflow complete, ROM incomplete

Current round009; phase final acceptance/document accounting. Terminal checkpoint pass-009-final:262140/262160classified,20Unknownbytes14intervals (99.99%). RedirectL015 reviewed-incomplete with no ready leads; coordinator primary reasoning accepted in coordinator-rejection-review.md. Final editorial audit/byte-identical rebuild/reading refresh passed, source SHA4201260c152256ce179e63bb4ebd96f52adfd7c7e9e2a5e8e3dcfbea4170566d. All14686 scaffold entries/2404distinct units reviewed,15154unit records; no classified readability backlog. Official audit session2471 completed, no live writer. Map/state/input unchanged. Exact requirement and document accounting are in coordinator-handoff.md, completion-audit.json, archive-options.json and archive-document-accounting.json. No archive/cleanup/toolkit edit/publication. All prior evidence/handoffs/source snapshots preserved.

|Round|Input checkpoint|Output checkpoint|Consumed leads|Redirect outcome|Classified / Unknown|
|---|---|---|---|---|---|
|1|initialized / verified baseline|pass-001|startup|L001–L006|148886 / 113274|
|2|pass-001|pass-002-final|L001–L006|L007/L008|199961 / 62199|
|3|pass-002-final|pass-003-final|L007/L008|L009|231210 / 30950|
|4|pass-003-final|pass-004-final|L009|L010|242561 / 19599|
|5|pass-004-final|pass-005-final|L010|L011|246470 / 15690|
|6|pass-005-final|pass-006-final|L011|L012|249050 / 13110|
|7|pass-006-final|pass-007-final|L012|L013|260667 / 1493|
|8|pass-007-final|pass-008-final|L013|L014|262106 / 54|
|9|pass-008-final|pass-009-final|L014|L015 reviewed-incomplete|262140 / 20|

The phase-specific input/output checkpoints, verification, consumed/produced leads and outcomes remain linked by each round's durable handoffs. Final acceptance closes current-source editorial scope only. Twenty bytes remain structurally Unknown and resumable on the specific new evidence conditions recorded in L015/rejection review. TK001/TK002 are throughput feature requests,0correctness blockers.
