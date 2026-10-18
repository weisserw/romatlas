# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## Round 1 active evidence
R1-DISPATCH (verified): RST00 at 0000 executes POP HL; RST08; JP(HL). RST08 doubles A, adds to HL through carry handling, then RST28/read low+high/RET obtains table pointer. Thus RST00 consumes the source return and never ordinarily returns to inline bytes, for all arrivals under fixed bank 0. Contextual call profiles in analysis/runtime/discovery-parameters.json suppress the erroneous presumed continuation for exact C7 bytes in effective banks 1–3, without asserting that every listed byte is code. This is a semantic query safeguard, not execution evidence. Inspected main state table 044f–0462 is Data. Other unbounded inline tables remain unknown pending consumer/domain checks.

R1-HRAM-DMA (verified): 02a0 copies ten ROM bytes 02ae–02b7 to ff90–ff99. DMA template initializes ff46 from c3, spins 40, returns. ROM load image is Data; it is executable only at its HRAM destination. Observed ROM coverage never inferred from HRAM execution.

No preexisting lead IDs; these IDs identify discoveries made within this pass. More detailed final counts and outcomes follow at handoff.

R1-INLINE-TABLES (verified): additional RST00 inline pointer spans0536+0a,18cf+06,1982+04,1b3b+0a,2ead+0a,59a5+04. Candidate words inspected with analysis.pointers under fixed0 and source switch-bank context. Every entry names an independently decoded code start; consumer semantics and next known code boundary establish pointer-table structure without labeling target execution from values. Local exact reports analysis/runtime/table-0x*.json. 44 bytes classified Data, confidence0.95; selector domains/handler purpose remain for commentary. Do not blindly extend these extents.

R1-PADDING (verified structural inference): FF fill in restart/interrupt inter-entry gaps, pre-header reserved gap, and bank1/2/3 tails immediately before their bank-ID bytes. Bank identity bytes at physical7fff/bfff/ffff equal1/2/3 and are consumed through CPU7fff in save/restore bank routines009b/00ca. Constant zero runs and arbitrary raw export gaps were not treated as padding. Confidence0.95 for inspected fill; no execution is inferred.

## Round 1 final outcome

ROM classification: 18.57% (12173 / 65536 bytes classified; 53363 bytes unknown).

| Bank | Code | Data | Padding | Classified | Unknown |
| --- | ---: | ---: | ---: | ---: | ---: |
| 00 | 7825 | 166 | 73 | 8064 | 8320 |
| 01 | 1229 | 5 | 67 | 1301 | 15083 |
| 02 | 1972 | 1 | 37 | 2010 | 14374 |
| 03 | 108 | 1 | 689 | 798 | 15586 |

Final checkpoint round-001-disassembly passed exact65536-byte rebuild; report round-001-disassembly-verification.json. Detailed counts, unknown intervals and annotation/execution metrics in round-001-classification.json and round-001-unknown-ranges.json. Role/evidence metadata covers432/432 regions.

R1-OPENING-PLAY verified: opening forest, whip/jump, enemy defeat/score80, scrolling, damage and visible flying enemy. Gains decreased to169 then6 bytes; brief Start alternative added125. Stop incomplete; no difficult progression without a concrete substantial prospect. R1-UNREASSERTED-WALK superseded: snapshot input bug T001 explained stationary replay; corrected explicit-input route worked. R1-PAUSE verified as executed Start press/release experiment, not comprehensive pause semantics.

Unresolved: bulk unknowns include bank0 physical3505–3fff and0ef9–15a3, bank1 physical4000–5635/623b–6ebe, bank2 physical881d–bfd9, bank3 physicalc000–fcbf; these are unknown structural spans, not blanket data claims. Full smaller gaps in round-001-unknown-ranges.json. Dispatch selector domains, graphics/compression consumers, sound streams and later-stage routes remain deferred. No unchanged failed lead should be retried. Toolkit T001 is confirmed with effective reassert-input workaround; no blocker.

## Round 2 — disassembly pass finished (2026-10-17), ROM incomplete

ROM classification: 46.85% (30702 / 65536 bytes classified; 34834 bytes unknown).

Input phase/checkpoint: disassembly, `round-001-disassembly` (12173 classified). Output phase/checkpoint: disassembly finished, `round-002-disassembly`; commentary/redirect not executed. Preserved prior store/classifications. Exactly one static-first easy-gain pass consumed two new ready leads. Configured release binary unchanged; ROM hash confirmed by `python3 run.py inspect`.

| Bank | Code | Data | Padding | Classified | Unknown |
| --- | ---: | ---: | ---: | ---: | ---: |
| 00 | 7878 | 2322 | 73 | 10273 | 6111 |
| 01 | 1229 | 5 | 67 | 1301 | 15083 |
| 02 | 1972 | 4033 | 37 | 6042 | 10342 |
| 03 | 108 | 12289 | 689 | 13086 | 3298 |


Totals Code11187, Data18649, Padding866; Image/Text0. Added18529bytes:18476Data plus53Code. All478/478 regions have role/evidence; provenance Observed8251, Static22451, Manual0. New planar tile payloads are represented as Data with specific tile roles; no new stock image directives required. Classification is semantic/static evidence, separate from execution. No runtime, debugger patches, new snapshots or gameplay routes this pass; retained round1 cumulative evidence4890 fetched starts/9144 instruction bytes. Diminished normal gameplay was not repeated without a new concrete prospect. Gameplay familiarization unchanged: existing sourced routes remain recorded in gameplay.md, but do not contribute classification evidence.

Consumed `R1-TILE-TRANSFERS` verified: independently decoded wrappers/consumer0e47 and tile loop0eae/0ebd, parsed every six-byte record and zero first-word sentinel, checked all positive counts and bank-contained payload extents, physically read every payload in<=4096byte requests. Stage domain masks3 or explicit4 proves five-word0b8d pointer span. All original candidate17308bytes (17298 union+10 pointer table) supported; deduplicated overlap partitioned with precise per-record evidence. Source addresses below4000 remain physical bank0 even wrapper0e91 switches tobank1; higher pointers resolve from explicit effective switch banks2/3 in this4bank MBC1 ROM. The common loop multiplies count by16 and copies unchanged bytes toVRAM; every inspected record's upper selector bits14/15 arezero, so no broader flag behavior asserted.

Consumed `R1-FIXED-TITLE-MAP` verified:0caf..0ccd reads20bytes per18rows from0cce, writes9800 with32cell destination stride; exactly360bytes through0e35, adjacent0e36 independently code. Zero cells inside this exact consumer extent are map data; no nearby zero-padding inference.

Produced `R2-FIXED-INTRO-TILES` verified: existing classified0ec8 loads0ef9 and calls0e36. The eight-byte terminated list has destination01c0,count50,source0f01; establishes800fixed-bank payload bytes0f01..1220 plus8list bytes. Destination9400..971f; switchbank3 does not change fixed sources. Added53static code bytes in exact wrappers0c63..0c88 and0e7a..0e88 (decoded whole instructions ending shared tail jump, same save/select grammar as classified wrappers); no execution/natural reachability claim. Exact evidence [verified rows](round-002/verified-transfers.json), [inspection RPC](round-002/inspection-rpc.json), [annotation parameters/results](round-002/annotations.json).

Single-pass stopping: the two actionable substantial leads and obvious adjacent fixed-source transfer are resolved; bounded consumer call-site review exposes no further new substantial transfer payload. Remaining state/audio leads explicitly deferred without established domains; blind unchanged gameplay already diminished inround1. Stop incomplete under easy-gain rule, without exhaustive analysis of harder structures. Remaining unknown34834bytes with exact ranges [round-002-unknown-ranges.json](round-002-unknown-ranges.json); full metrics [round-002-classification.json](round-002-classification.json). Bulk remaining banks1/2, bank0 level/state tables and bank3 remainder need fresh consumer/grammar evidence; no blanket data claims.

Verification: map.validate returnedok after annotations; map.save analysis/state.json; map.export maps/current.sym. `python3 run.py checkpoint round-002-disassembly` passed stock map validation, generation and byte_identical=true for all65536bytes. Frozen map [round-002-disassembly.sym](../maps/round-002-disassembly.sym), bank-organized source [generated/round-002-disassembly](../generated/round-002-disassembly), report [round-002-disassembly-verification.json](round-002-disassembly-verification.json). Map SHA2567e8fb59747fe2531eff72ea4bdb277a28e82226107a201b661a0f2da6ee198ef; ROM unchanged SHA256edb101e924f22149bdcbcfe6603801fdb4ec0139a40493d700fa0205f6dab30c. Exact478region count and overlap-free physical union independently checked from internal store after save; raw fallback unknowns excluded.

Toolkit: existing one confirmed bugT001, zero suspected bugs/missing features, no new issues/blockers. No runtime snapshots restored; reassert buttons after future imports. [Persistent toolkit report](toolkit-issues.md). No toolkit orROM modification. Compact [round2 handoff](round-002-disassembly-handoff.md).

## Round 3 — disassembly pass finished (2026-10-17), ROM incomplete

ROM classification: 47.69% (31254 / 65536 bytes classified; 34282 bytes unknown).

Input checkpoint `round-002-disassembly`; output `round-003-disassembly`; phase disassembly finished, commentary next under coordinator. One bounded static pass preserved existing478regions and added552bytes:400Code,152Data. No runtime, debugger edits, snapshots, inputs or gameplay replay; sourced gameplay record and scenarios preserved. Prior opening-play gains had diminished; new ready static leads enabled this pass. Configured release binary and input ROM unchanged.

| Bank | Code | Data | Padding | Classified | Unknown |
| --- | ---: | ---: | ---: | ---: | ---: |
| 00 | 8278 | 2399 | 73 | 10750 | 5634 |
| 01 | 1229 | 5 | 67 | 1301 | 15083 |
| 02 | 1972 | 4033 | 37 | 6042 | 10342 |
| 03 | 108 | 12364 | 689 | 13161 | 3223 |

Totals Code11587, Data18801, Padding866, Image/Text0. Required role/evidence annotations490/490regions. Observed8251bytes; Static23003bytes; Manual0. Retained historical execution evidence4890ROM instruction starts/9144instruction bytes, unchanged; static classification is separate from execution coverage.

R2-SCROLL-TRANSITION-HANDLERS verified:2754 selects effective switch3, then2a7e calls2a86; c405 decrement/mask03 proves exact8inline bytes2a8d..2a94 naming2a95/2b38/2bce/2bdf. Verified RST00 semantics consume inline return. Bounded analysis.discover_context budget1800,use_observed=false,fixed0/switch3 completed347candidates/662unionbytes;400currently unknown instruction bytes. Independently decoded every new block and exact instruction ends in [blocks](round-003/blocks.json), [decoder](round-003/full-2a95.json), [discovery](round-003/discovery.json). These handlers manipulate window position ff4a/ff4b, scroll ff42 and tile/block buffers; precise phase meanings remain provisional. Seven partial blocks end at conservative indirect-store traversal stops; no unsupported continuation or mapper crossing added. Existing overlapping code remains untouched. Unresolved stores and tile arrays2e01/2e11 need independent extents/destination evidence.

R2-ADDRESSED-TILE-STREAMS verified:0ad2 reads little-endian destination, emits tile bytes, FE begins another destination, FF returns;0af0 waits for suitable LCD status. Intro0ec8 literal DE1221 yields six runs3/4/18/19/4/3, exact69bytes through1265. Bank3 literal loads in independently decoded7c20..7c96 and classified7cc0/7cf1 name11streams with suffix7cea overlapping7ce2;75unique bytes. All candidate bytes read with rom.read; exact run counts/destinations/terminators independently parsed in [verified streams](round-003/verified-streams.json). Raw callers were inspected as consumer evidence, not automatically accepted as executable from disassembly alone. Adjacent1266 and raw bank3 caller bytes remain unknown. Fixed pointers remain physicalbank0 despite bank switches; upper sources explicitly physicalbank3.

[Annotation requests/results](round-003/annotations.json) and [RPC](round-003/rpc.json) retain provenance and role/evidence. No smaller annotation overlapped an existing region, so no splits required. map.validate returnedok; map.save analysis/state.json; map.export maps/current.sym include_sym=false. `python3 run.py checkpoint round-003-disassembly` passed stock validation, generation and exact65536-byte rebuild: [verification](round-003-disassembly-verification.json), [stock map](../maps/round-003-disassembly.sym), [bank-organized listing](../generated/round-003-disassembly). Map SHA2568e3f9cc5b51f12403c097b95f0d55470877ff388d85957cb75a19ee9c300fc0d; ROM SHA256edb101e924f22149bdcbcfe6603801fdb4ec0139a40493d700fa0205f6dab30c. Full [metrics](round-003-classification.json), [unknown ranges](round-003-unknown-ranges.json).

Stopping incomplete: both new ready leads resolved. Remaining R2-LEVEL-TABLE-DOMAINS still lacks nested c412 bounds and caller bank evidence; historical global-state/audio leads remain deferred. Conservative indirect-store continuations promise only small extensions absent additional proof. No fresh obvious substantial scenario remains, so no blind repeat or difficult progression. Unknown34282bytes remain, principally banks1/2; raw export fidelity supplies no classification evidence. No new leads produced. Commentary/redirect not executed.

Toolkit summary: existingT001 confirmed snapshot-held-input bug;0new issues,0suspected bugs,0missing features, no blocker. Future snapshot restores must explicitly reassert input. No runtime this pass. [Persistent toolkit report](toolkit-issues.md). Compact [handoff](round-003-disassembly-handoff.md).

## Round 4 — disassembly pass finished (2026-10-17), ROM incomplete

ROM classification: 48.93% (32067 / 65536 bytes classified; 33469 bytes unknown).

Input checkpoint `round-003-disassembly`; output `round-004-disassembly`; phase disassembly finished, commentary next under coordinator. Exactly one bounded static pass consumed materially improved `R2-LEVEL-TABLE-DOMAINS`; no new IDs. Preserved all490prior regions and metadata, adding813Data bytes in20regions. No runtime, debugger edits, snapshots, inputs or unchanged gameplay replay; sourced gameplay and scenario routes retained. No research needed for this local consumer proof. Configured release binary and ROM unchanged.

| Bank | Code | Data | Padding | Classified | Unknown |
| --- | ---: | ---: | ---: | ---: | ---: |
| 00 | 8278 | 3212 | 73 | 11563 | 4821 |
| 01 | 1229 | 5 | 67 | 1301 | 15083 |
| 02 | 1972 | 4033 | 37 | 6042 | 10342 |
| 03 | 108 | 12364 | 689 | 13161 | 3223 |

Totals Code11587, Data19614, Padding866, Image/Text0. Required role/evidence annotation coverage510/510regions. Stored provenance Observed8251bytes, Static23816, Manual0; historical actual execution4890ROM instruction starts/9144instruction bytes unchanged. Classification and execution coverage remain separate. Full [metrics](round-004-classification.json), complementary [unknown ranges](round-004-unknown-ranges.json).

Lead verified for c412=0 only: independently decoded082e reset, raw07b2 stage-reset sequence (consumer evidence, not newly classified code), RST08/28 index helpers,27c2 width guard,27f0 assignments,2815 five-row loop,2864 switch1 wrapper,2875 screen-major addressing,2918 and2d0d threshold scans. c02b&3 closes each outer four-word directory3505/3567/357b:24bytes. Nested selector0 supplies four words at350d/3527/351d/3547 and four widths3583=25,3590=30,358b=53,35a0=45. Layout index equals25*floor(column/5)+5*row+column%5 for column0..width-1,row0..4; independently simulated subtraction quotient/remainder and checked exact index-set union765bytes. Exact ranges35b0+125,3bf0+150,3853+253,3952+3,3957+3,395c+3,3961+3,3ede+225. Stage2 deliberately excludes ten unused final-screen cells; explicit post-save check confirms all remain unknown. No adjacent padding or broader nested extents inferred.

Three threshold vectors356f/3573/3577 each endFF at entry3.2918/2d0d compare unsigned, stop on A<=threshold; exhaustive256input check per vector proves class0..3 independently of level block data. Twelve threshold bytes accepted; stages0/2 share356f. All accepted tables/layouts are CPU<4000 physicalbank0; wrapper switch1 does not relocate them. c400 payload bounds and262f/44a7 effective context remain unresolved. Static data proof makes no natural execution/reachability claim for all stages. [Verified domains](round-004/verified-domains.json), [annotations](round-004/annotations.json), [inspection RPC](round-004/inspection-rpc.json), [supplemental](round-004/supplemental-inspection-rpc.json), [mutation/save RPC](round-004/rpc.json), [preservation check](round-004/preservation-check.json).

map.validate returnedok before/after annotations; map.save analysis/state.json; map.export maps/current.sym include_sym=false. `python3 run.py checkpoint round-004-disassembly` passed stock map validation, generation and byte_identical=true for65536bytes. [Verification](round-004-disassembly-verification.json), [stock map](../maps/round-004-disassembly.sym), [bank-organized listing](../generated/round-004-disassembly). Map SHA256db27386b327e14e3fbe336f12c35bfc1082271dff57108f93fbf5c3757c453fb; original ROM SHA256edb101e924f22149bdcbcfe6603801fdb4ec0139a40493d700fa0205f6dab30c confirmed with run.py inspect.

Easy-gain stopping: improved ready recipe fully consumed. Broader c412 domains, bank1 objects, bank2 audio grammar, global states and small indirect-store continuations lack fresh closed substantial evidence. No unchanged diminished gameplay replay or speculative block classification. Stop incomplete with33469unknown bytes; all15lead IDs/history preserved, accepted lead nowverified with explicit partial-scope retry policy. Commentary/redirect not executed by this worker; required phase handoff in [round-004-disassembly-handoff.md](round-004-disassembly-handoff.md).

Toolkit summary: existing1confirmed bugT001,0suspected bugs,0missing-feature findings;0new findings/no blocker. No snapshot used; future restore must reassert held/released buttons. [Persistent toolkit report](toolkit-issues.md). No toolkit orROM edits.

## Round 5 — disassembly finished (2026-10-17), ROM incomplete

ROM classification: 50.48% (33083 / 65536 bytes classified; 32453 bytes unknown).

Input checkpoint `round-004-disassembly`; output `round-005-disassembly`; exactly one bounded disassembly pass, commentary next under coordinator. Consumed `R4-FIRST-LAYOUT-BLOCKS`, now verified for conservative first-layout scope; no IDs produced. All510prior regions/metadata preserved;64new Data regions add1016bytes. No runtime, snapshots, inputs, debugger changes, new gameplay research or unchanged diminished route replay. Sourced gameplay/scenarios retained; static evidence sufficed.

| Bank | Code | Data | Padding | Classified | Unknown |
| --- | ---: | ---: | ---: | ---: | ---: |
| 00 | 8278 | 3212 | 73 | 11563 | 4821 |
| 01 | 1229 | 13 | 67 | 1309 | 15075 |
| 02 | 1972 | 4033 | 37 | 6042 | 10342 |
| 03 | 108 | 13372 | 689 | 14169 | 2215 |

Totals Code11587, Data20630, Padding866; Image/Text0. Required role/evidence coverage574/574regions. Provenance Observed8251bytes, Static24832bytes, Manual0. Actual historical execution4890ROM instruction starts/9144instruction bytes unchanged, separate from classification. [Metrics](round-005-classification.json), [unknown ranges](round-005-unknown-ranges.json).

Independent bank/consumer proof: initialization0150 selects bank1;0862 caller retains it across0e36 bank3 transfer wrapper, with interrupt sound wrappers preserving prior bank.262f reads four little-endian words atbank1:41fe via c02b&3/RST08, stores H/L toc400/c401:7000,7510,7320,77c0. This is bank-context-specific static proof; no global reachability assertion for every descriptor.2754 selects3;2815 calls2838 then28ce;2864 temporarily selects1 for2875 then restores3.2875 reads exact first-layout IDs, adds16*rawID to base and returnsDE.2838 reads4rows*4bytes consecutively;28ce resetsDE and calls2918 four times, each reads4bytes. Thus exact payload physical address=CPU+8000. RST08 and28 arithmetic independently decoded. Decoder probes initially starting014b/061d were unaligned exploratory probes, corrected to0150/061c; no code classifications derive from those probes.

Recomputed layout coordinate sets from round004 base/width, checked existing classified cells, exact16/17/15/18ID sets against ROM, excluded stage3 IDs65..67 and read each candidate before annotation. Accepted63records1008bytes plus8byte directory, exactly1016unique new bytes. No min/max spans, missing-ID records, later nested selectors or surrounding bytes inferred. Post-save preservation proves48withheld bytes and10unused stage2 cells remain unknown, all prior metadata unchanged and exact new-byte union equals accepted records. [Records](round-005/verified-records.json), [annotations](round-005/annotations.json), [RPC](round-005/rpc.json), [preservation](round-005/preservation-check.json).

map.validate before/after passed; map.save analysis/state.json; map.export maps/current.sym include_sym=false. Fresh `python3 run.py checkpoint round-005-disassembly` passed map validation, generation, byte_identical=true for65536bytes. [Report](round-005-disassembly-verification.json), [portable stock map](../maps/round-005-disassembly.sym), [listing](../generated/round-005-disassembly). Map SHA2568a9b2fc8be45fecb613ef41c9100f767d94c7bc94311ccc521dd6ea165eece25. run.py inspect confirmed original ROM SHA256edb101e924f22149bdcbcfe6603801fdb4ec0139a40493d700fa0205f6dab30c. Toolkit/configured release unchanged.

Easy-gain stop: ready conservative recipe fully consumed. Flag-sensitive stage3 IDs, global c412 domains, bank1 objects, bank2 audio and global states lack fresh substantial closed evidence. No unchanged blocked leads retried; no speculative filling or difficult progression. ROM remains incomplete with32453unknown bytes. Commentary/redirect not executed. [Compact handoff](round-005-disassembly-handoff.md).

Toolkit: existing1confirmed bugT001,0suspected/0missing-feature findings;0new findings/no blocker. No restore used; future restores must explicitly reassert all buttons. [Persistent report](toolkit-issues.md).

## Round 5 redirect — bounded empty result

No new ready substantial lead; preserved16lead IDs and all outcomes. Raw scrolling versus masked transition and nested selector writers rechecked, plus generic/uncommented/pending source. No classification/runtime. Stop cycle incomplete at33083classified/32453unknown. [Findings and verification](round-005-redirect-handoff.md); existing T001 only, no new toolkit findings.

## Round 5 resumed redirect — bounded descriptor domain replaces global prerequisite

Prior empty outcome retained above. Revised policy permits supported partial slices without solving globalc412.0968 outputs14exact threshold rows for stage0..3, inputc412<99. Fourteen constructed inputs prove c02a43..50 versus returned nestedc412;262f bank1 five-byte descriptors supply exact additional layout indices. Fresh evidence, not unchanged retry. Two ready IDs and exact1251/205unknown-byte role sets in [resumed evidence](redirect/round-005-resumed-evidence.json); [lead registry](leads.json). All28function calls returned under150/200operation limits; no natural gameplay claim or map edits. Stop redirect at actionable handoff, preserving unresolved global domains, stage3 raw/masked applicability, remaining transition descriptors, bank1 object grammar and bank2 stream grammar. Next disassembly independently checks exact ranges/contexts and can narrow slices. Historical stop requiring substantial closed gain superseded; no claims of complete ROM classification.

## Round 6 disassembly: bounded respawn/layout and transition slices verified

Consumed R5-BOUNDED-RESPAWN-DESCRIPTORS and R5-TRANSITION-RECORD-SLICES with independent0968/262f/27c2/2875/1e98/1fab consumer checks.14bounded threshold cases,13unique stage/nested pairs; exact coordinate enumeration and terminal-led five-byte parsing agree with lead artifacts.1251+205previously unknown bytes accepted; direct exact-ID bank3 consumer extension adds1264bytes. No new execution or natural reachability claim. All old metadata preserved;48flag-sensitive bytes and incomplete-screen tails remain unknown. Full evidence, byte ranges and deferred targets:[handoff](round-006-disassembly-handoff.md). Classification54.63%,29733unknown; fresh checkpoint byte_identical65536bytes. Local presave checkpoint race corrected and retained as explicitly superseded recovery, no toolkit defect. Final round006 map independently equalcurrent.sym. Both IDs nowverified; further transition selectors can support bounded descriptor research without global domain closure. No unchanged failed route retried. Toolkit existingT001only, no new findings/no blocker;[report](toolkit-issues.md).

## Round 006 redirect — conditional finite descriptor expansion

Verified source1e98 ordinary gates and stage-specific fifth-byte selector fields against262f and downstream27c2/2875/2838/28ce/4206 consumers.19constructed loader tests return94..98operations under200cap, matching nested/width. Exact1339currently unknown bytes are actionable across2ready leads; no classification edits. [Handoff](round-006-redirect-handoff.md) and [evidence](redirect/round-006/evidence.json) retain exact patches, bounds, uncertainties and preservation hashes. Stop this research at a concrete independently reviewable classification recipe; further selector expansion remains unverified, stage3 rawIDs>=64 withheld, no global closure or natural reachability claimed. Existing unrelated stalls unchanged. Toolkit T001 only, no new finding/blocker.

## Round 007 disassembly
R6-TRANSITION-DESCRIPTOR-LAYOUTS and R6-TRANSITION-DOWNSTREAM-SLICES verified through independent source-consumer decoding and exact physical-bank/coordinate/list reconstruction;1339new Data bytes(605+734),109regions. No new execution or natural reachability. Prior classifications, metadata and lead history preserved. [Evidence](round-007evidence/verified.json), [RPC](round-007evidence/rpc.json), [preservation](round-007evidence/preservation-check.json). Fresh round-007-disassembly checkpoint byte-identical65536bytes;56.67%classified,28394unknown. Exact unknown complement round-007-unknown-ranges.json. Ready recipes consumed; no unchanged failed gameplay repeated. New transition fifth fields (including52,54,55,60,61) are stage-specific hypotheses for redirect descriptor proof; stage3rawIDs>=64withheld. Existing T001 only; no new toolkit finding/no blocker. Editorial review pending. [Handoff](round-007-disassembly-handoff.md).

## Round007 redirect

New finite transition cases independently reconstructed and debugger-checked;461unknown Data candidates and independent153unknown continue-handler Code candidates are ready. Corrected source-list versus descriptor nested distinction52→6,54→4,55→2,60→8,61→6. Four bounded artificial raw-wrapper calls returned; natural reachability remains unproved. First bulk sentinel overshoot retained and corrected to per-operation detection. No unchanged stalled avenue reopened. [Handoff](round-007-redirect-handoff.md), [durable evidence/recipes](redirect/round-007/evidence.json), [lead registry](leads.json). No classification changes.


## Round 008 — disassembly finished, ROM incomplete

ROM classification: 57.62% (37760 / 65536 bytes classified; 27776 bytes unknown).

Input round-007-disassembly; output round-008-disassembly. R7-TRANSITION-DESCRIPTOR-LAYOUTS verified336Data; R7-TRANSITION-DOWNSTREAM-SLICES verified125Data; R7-CONTINUE-HANDLERS verified153Code plus4direct06ed tail bytes.26stage/selector cases independently rederived; parent source-list nested is separate from262f-loaded nested. Exact five-row/width cells, bank1 upper/fixed0 lower layouts, bank3 restored raw-ID blocks and high-E0 sentinel-led lists; no adjacent fill/global closure. Four artificial wrapper calls returned988/19/279/278operations; only49new instruction starts/114Code bytes actually fetched,43Code bytes Static. No natural reachability, pointer-target execution or later-stage route claimed. T001 workaround releases all8buttons after each clean session-local restore. Reproduction in scenarios.json and [raw probe](round-008evidence/raw-probe.py); [independent proof](round-008evidence/verified.json).

All782prior regions/metadata and historical lead/recovery artifacts preserved exactly. New68regions/618bytes;850/850role/evidence. Per-bank Code/Data/Padding/Unknown:00=8316/4737/73/3258;01=1229/1136/67/13952;02=1972/4033/37/10342;03=227/15244/689/224. [Metrics](round-008-classification.json), [unknown ranges](round-008-unknown-ranges.json), [preservation](round-008evidence/preservation-check.json). Map validate/save/export sequentially completed before fresh checkpoint; byte-identical65536bytes [verified](round-008-disassembly-verification.json).

Easy gains resolved; do not retry unchanged leads. Additional new list selectors require independent source-stage/loader/bank checks. Stage3flag applicability,natural gates,bank1object grammar,bank2audio4712/4916,global dispatch and later gameplay remain unresolved. Remaining224bank3unknown bytes are not inferred data/code by proximity. No difficult/blind gameplay replay without new evidence. Commentary next; final editorial audit pending. [Compact handoff](round-008-disassembly-handoff.md). Toolkit existingT001 only,0new/no blocker; [report](toolkit-issues.md).

## Round008 redirect
Input round-008-disassembly/commentary handoff. Finite newly exposed stage/fifth fields independently reconstructed;33bounded262f tests passed.334exact unknown descriptor/layout/downstream bytes actionable asR8-TRANSITION-EXACT-SLICES. Stage3flagged IDs remain excluded. Independent audio avenue materially improves historicalR1-AUDIO-DATA-DOMAINS: selected literal/masked-stage commands5,6,7,21,40,48,49 define exact priority/mask/channel-pointer headers without global command-domain closure;7constructed4712tests passed108..334operations.74unknown bytes including18six-channel table bytes actionable asR8-AUDIO-EXACT-HEADERS. Prior stalled history preserved; no unchanged failed scenario repeated.
Stopped research with concrete useful bounded leads; downstream newly exposed selectors remain for independent later verification after parent slices accepted. Audio stream grammar,global domains,stage3flag applicability,bank1object grammar,natural continue/gates and late gameplay remain unresolved; pointer targets alone prove no execution/extent. No map/source/classification/toolkit changes. [Handoff](round-008-redirect-handoff.md), [verification](redirect/round-008/verification.json).


## Round 009 — disassembly finished, ROM incomplete

ROM classification: 58.24% (38168 / 65536 bytes classified; 27368 bytes unknown).

Input round-008-disassembly; output round-009-disassembly. Independently verified R8-TRANSITION-EXACT-SLICES (334Data) and R8-AUDIO-EXACT-HEADERS (74Data). Added408Static Data bytes in61regions. All850prior regions/metadata and unrelated internal fields preserved exactly;911/911regions have role/evidence. [Proof](round-009evidence/verified.json), [RPC](round-009evidence/rpc.json), [preservation](round-009evidence/preservation-check.json).

33source-stage/selector cases rederived from classified transition fifth fields, loader262f, exact directories and five-row width-limited screen-major indexing. Parent list nested differs from loaded nested;53→5,62→7,63→9,64→10. Upper layouts/lists bank1, lower fixed0;2754 selects3 and2864 restores3 after temporary1. Exact16-byte raw-ID payloads only, stage3IDs>=64withheld. Five-byte records stop before explicit high-E0 two-byte terminal; incomplete-screen gaps remain unknown. New records expose selectors56/57/65 for later independent source/descriptor/context investigation, not recursively accepted here.

Audio fixed00bd selects switch2. Literal command sources include bank2:46a3 command5 via4706; fixed3471command6,21b7command7,042ecommand21,04e3command49 via00bd. Masked stage source0a0e/0a1e yields40/48. 4033 indexes table4916 by command-1;4712 reads priority/mask,4763 consumes2bytes per selected low-six bit even if priority rejected: header extent2+2*popcount(mask&63). Selected commands5/6/7/21/40/48/49 only. 405b..40cc bounds c244=0..5,40a6 word consumer proves4672table12bytes;467e proves4689table6bytes. Stream pointer targets establish no stream extent or execution. Static proof only; no emulator/runtime, natural sound execution, gate success or new gameplay route claimed. Existing diminished routes were not repeated without new evidence.

Per-bank Code/Data/Padding/Unknown:00=8316/4827/73/3168;01=1229/1332/67/13756;02=1972/4107/37/10268;03=227/15292/689/176. TotalsCode11744/Data25558/Padding866;Observed8365/Static29803. No new fetched instruction starts. Historical natural and constructed execution figures remain separate; no uncomputed cumulative union. [Metrics](round-009-classification.json), [exact unknown complement](round-009-unknown-ranges.json).

Preservation audit caught a local script map.load path using round007 rather than frozen round008 input. Earlier provisional58.24%/checkpoint claim withdrawn before handoff. Superseded byte-faithful but classification-incomplete output retained as round-009-input-recovery with truthful report; original850-region input retained. Corrected script reloaded frozen input, repeated exact408byte additions and verified all850regions/metadata plus unrelated fields before fresh final checkpoint. This is local orchestration, not toolkit defect. Final map.validate/save/export finished sequentially before `python3 run.py checkpoint round-009-disassembly`; stock validation/generation and byte_identical65536bytes passed. [Final verification](round-009-disassembly-verification.json), [superseded recovery](round-009-input-recovery-verification.json). Current.sym equals frozen final map; hash1c4b75b43ac85fe6e79db0cfa002ac57b7c390445cb4e88fefa76f8bfd07516d. ROM unchanged; configured release unchanged.

Bounded ready recipes resolved; no commentary/redirect performed. Remaining selectors require independent proof;stage3flags,natural gates/continue,bank1objects,bank2stream grammar,global domains and later gameplay remain unresolved. Final editorial audit pending. Toolkit existing1confirmedT001,0suspected/0missing features,0new/no blocker; [report](toolkit-issues.md). [Compact handoff](round-009-disassembly-handoff.md).

## Round009 redirect

Independent56/57/65 stage/context reconstruction and3loader tests resolve95unknownData-byte opportunity. Independentselectedaudio grammar bounds80unknownstreambytes;31artificialparsercalls terminate5/7/21;command6finitecounts253/8static,no thousands-of-eventsreplay. Found actual StaticCode misclassification of inlineaudio pointers;exact30tablebytes include18Code→Data+12unknownData;8adjacentmisalignedStaticCode requireindependentproof/retraction,withObservedneighborsretained. No redirectclassificationmutation. Three concrete readyrecipesinleads.json;historicalglobalaudiostall retainedandupdatedwithnew finiteevidence. New58/59andmusicstreams,deferredflags/bank1objects/naturalroutes remainunverified. Stop atusefulnextdisassembly,not globalclosure. [Findings](redirect/round-009/findings.md),[handoff](round-009-redirect-handoff.md).

## Round 010 disassembly — bounded slices and corrective structure

Verified all three R9 ready leads using independent consumer decoding and stage/channel context reconstruction. Accepted 95 exact transition bytes and 80 selected finite audio stream bytes. Static control interpretation confirms FF termination and command 6 FE repeat counts 253/8 without thousands of emulator calls. No natural reachability inferred. Exact evidence and reproducible local script: [proof](round-010evidence/verified.json), [replay](round-010evidence/verify.py).

Corrected 18 Static Code bytes to inline word Data and accepted 12 adjacent unknown table bytes. Retracted 82af..82b6 eight unsupported Static Code bytes to Unknown rather than classifying on pointer shape. The real 42b7 handler and 41a9 dispatcher prefix have independent exact code evidence; observed neighbors are untouched. All 908 unaffected records/metadata preserved exactly. Earlier affected fallthrough/prose review is superseded; see [invalidation](round-010evidence/commentary-invalidation.json).

Fresh checkpoint round-010-disassembly is byte-identical for 65536 bytes after sequential map validation/save/export. Classification 58.51%, 27189 bytes unknown; [per-bank metrics](round-010-classification.json) and [unknown ranges](round-010-unknown-ranges.json). No runtime or ordinary gameplay route added; unchanged stalled avenues not retried. Remaining selectors 58/59, unselected nested audio entries, music streams, stage 3 applicability flags, bank 1 objects, global dispatch domains and later natural routes require new evidence. This phase ends with the ready recipes consumed; no redirect was run. Existing T001 only; no new toolkit issue or blocker. [Handoff](round-010-disassembly-handoff.md).

## Round010 redirect
Source-supported stage1 selectors58/59 and command49 streams resolve finite next slices;162Unknown candidatebytes in two ready leads. EA parameters independently reopen only nested entries1/2;remaining82b3..82b6Unknown.42cbhandler currentCode corroborated,selected low-nibble word reads bounded. No ordinary reachability/globaldomain claim;selector66 newly exposed but deferred until independent verification. [Findings and exact recipes](redirect/round-010/findings.md),[handoff](round-010-redirect-handoff.md). No unchanged stall repeated;map/source unchanged.

## Round 011 — disassembly finished, ROM incomplete

ROM classification: 58.76% (38509 / 65536 bytes classified; 27027 bytes unknown).

Input round-010-disassembly; output round-011-disassembly. Exactly one pass consumed R10-TRANSITION-58-59 and R10-AUDIO49-STREAMS-EA12. Added 162 Static Data bytes in 13 regions: 51 transition bytes, 101 finite audio stream bytes, four reclaimed inline dispatch bytes and six selected parameter-word bytes. All 928 prior region objects, metadata and unrelated internal fields match frozen input exactly; all 941 current regions have role/evidence. No code correction or new Code claim. Recovery/history artifacts preserved. [Preservation](round-011evidence/preservation-check.json), [independent proof](round-011evidence/verified.json), [inspection](round-011evidence/inspection-rpc.json).

Stage1 classified parent rows43dd/43e4 source58/59 through1e98;262f reloads nested12/13,width5. Exact five-row screen-major cells, selected descriptor/layout/width/list pointers, five-byte rows and high-E0 terminals independently checked. Bank3 raw-ID payloads already classified; no redundant regions added. New selector66 is an unresolved hypothesis. No natural gate/later-stage reachability claim.

Command49 header independently selects channels2/3/4, finite stream lengths35/29/37 with FF endpoints. Aligned EA16/2a/21/20 parameters establish nested1/2 inline words82af..82b2. 82b3..82b6 remain Unknown. Independent decoding caught a redirect inference error: EA16 selects42c0, which ends JP41a2, so it does not read8aa9. EA20 selects42cb with low nibble0; DEC/ADD wraps toFE before base4a9f addition, reading CPU4b9d,physical8b9d. Accepted exact words8a9f/8ab1/8b9d;8aa9..8aaa remain Unknown. Prior evidence/lead history retained with correction appended. No target data extent inferred, including non-pointer valueFB51 at8b9d. 42cb is already Code and unchanged.

Four constructed42a5 calls with parameters16/21/2a/20, fixed0/switch2, DEc300,SPdff0, stopPC41a2, cap100 operations, returned22/31/31/32operations. Exact ROM Read events confirm inline words and selected parameter bytes; detail/aggregate drops0. Clean session-local restore and explicit eight-button releases used(T001). These tests corroborate local consumer behavior only, not natural sound execution or gameplay progression. [Read verification](round-011evidence/word-read-verification.json), [replay](round-011evidence/word-probe.py), [RPC](round-011evidence/word-probe-rpc.json). Whole-history execution coverage was not recomputed; classification provenance is Observed8365/Static30144 bytes, distinct from runtime coverage.

Totals Code11718/Data25925/Padding866. Per-bank Code/Data/Padding/Unknown: 00=8316/4911/73/3084;01=1229/1394/67/13694;02=1946/4328/37/10073;03=227/15292/689/176. [Metrics](round-011-classification.json), [unknown complement](round-011-unknown-ranges.json). No new natural route or external gameplay research; unchanged diminished routes not repeated. Ready finite slices exhausted; further selector66,music40/48 calls/loops,stage3 flags,bank1 objects and natural gates remain for fresh investigation. Commentary/redirect not executed and prior review credit remains historical pending reconciliation.

Map validate/save/export completed sequentially before fresh checkpoint. Stock validation/generation and fresh byte_identical=true for65536bytes passed. [Verification](round-011-disassembly-verification.json), [stock map](../maps/round-011-disassembly.sym), [bank listing](../generated/round-011-disassembly). Current.sym equals frozen map SHA256bb22398c95f1691d6a8d7806cef67d0dd73390e4580807d2810f37bd6d3c1fcb. Original ROM SHA256edb101e924f22149bdcbcfe6603801fdb4ec0139a40493d700fa0205f6dab30c and configured release unchanged. No toolkit changes or alternative build.

Toolkit existing one confirmed T001, zero suspected/missing features, zero new findings/no blocker. Incorrect initial local trace-space filter yielded empty detail, corrected to unfiltered Read capture before claims; not toolkit defect. [Report](toolkit-issues.md). [Compact handoff](round-011-disassembly-handoff.md).

### Round011 redirect
Selector66 from accepted stage1 nested13 now independently resolves nested14 width10; seven descriptor bytes actionable. Music40/48 exact consumer-state graphs close with single-slot call/return and finite/absolute repeat semantics;571 union unknown stream bytes plus2selectedEA28word bytes actionable. Seven isolated control branches runtime-corrobated; full graph static. No natural gate/audio timing/finiteplayback claim. No map/classification/source/toolkit edits. [Findings and precise limitations](redirect/round-011/findings.md), [durable recipes](leads.json). Downstream selectors30/31 already known; generic/unreviewed source search yielded no independently justified alternative beyond those graphs. Preserve prior stalled histories.

## Round 012 — one disassembly pass finished; ROM incomplete

ROM classification: 59.65% (39089 / 65536 bytes classified; 26447 bytes unknown).

Input round-011-disassembly and round011 redirect handoff; output round-012-disassembly. Consumed R11-TRANSITION-66 and R11-MUSIC40-48-STATEFUL-GRAPHS. Independently recomputed exact sparse event/edge sets and repeated decoder states for seven source-selected music graphs. Added 580 Static Data bytes in five regions: seven descriptor bytes, 571 stream bytes and exact EA28 word8aad..8aae. All 941 prior region objects, metadata and unrelated internal fields preserved exactly; all 946 current regions carry role/evidence. No existing classification changed. History/recovery and previous commentary remain preserved. [Proof](round-012evidence/verified.json), [raw inspection and decode](round-012evidence/inspection-rpc.json), [preservation](round-012evidence/preservation-check.json).

Stage1 nested13 row43eb fifth field66 selects directory452b and five-byte descriptor4675: nested14,width10. Downstream layout/list/payload bytes already classified; only the exact seven new bytes accepted. The source table at fixed0:0a1e is decimal40,40,40,48; c02b&3 selects command, wrapper00bd selects switch2,4712 indexes4916 by command-1 and4763 initializes selected channels. Header mask and packed pointer sequences independently checked. Audio physical addresses are CPU+4000 with fixed0/switch2.

Music40 channels2/3/4 have67/169/111 unique event starts and246/190/240 states; music48 channels2/3/4/5 have23/25/33/14 events and23/25/119/15 states. Exact sparse union, events, edges and cycle-entry states match independent enumeration; overlapping channel graphs counted once. FD overwrites the single return slot; FC resumes it without popping. FB changes marker without resetting counter. FE FF consumes absolute target; other FE increments eight-bit c106, repeats marker unless equal operand, then clears counter and skips operand. Actual source bytes and boundaries inspected before annotation. Initial return/marker zero are never read before assignment on these paths;4763 clears counter. Exact repeated state proves structural cyclic closure, with no finite-duration, natural playback or global music domain claim. No min/max span padding inferred.

Seven independently replayed isolated FB/FE/FD/FC controls returned23/32/30/31/31/23/32 operations under100-operation caps, including FF-to00 counter wrap. Setup restores clean session-local snapshot, explicitly releases all eight buttons(T001), selects bank2 by2180 write, patchesc106 andc11b..c11e from before-state, setsPC41a3/DEevent/SPdff0, and steps one operation to next41a3. [Exact before/after values](round-012evidence/control-tests.json), [RPC](round-012evidence/rpc.json), [replay](round-012evidence/verify.py). These are artificial local consumer tests; full graph remains static. Whole-history execution coverage not recomputed; no new Code or Observed classification.

Aligned EA28 selects exact physical8aad word4ac5 through42cb eight-bit doubled decrement. Destination4ac5 has no inferred extent/classification. Prior EA20 correction8b9d and42c0 endingJP41a2 preserved;8aa9..8aaa and82b3..82b6 remain Unknown. No new EA high-nibble domain inferred.

Totals Code11718/Data26505/Padding866. Per-bank Code/Data/Padding/Unknown:00=8316/4911/73/3084;01=1229/1401/67/13687;02=1946/4901/37/9500;03=227/15292/689/176. [Metrics](round-012-classification.json), [exact unknown complement](round-012-unknown-ranges.json). Required annotations946/946. Classification is distinct from execution coverage and pending commentary review. Stage3 flagged IDs, bank1 object data, other music domains, natural gate success and later-stage progression remain open. No materially improved natural route was available; diminished prior routes not repeated. Ready leads verified; no unchanged stalled hypothesis reopened or other phase executed.

Sequential map.validate, map.save toanalysis/state.json and stock map.export tomaps/current.sym completed before fresh checkpoint. Stock validation/generation passed and fresh byte_identical=true covers65536bytes. [Verification](round-012-disassembly-verification.json), [portable stock map](../maps/round-012-disassembly.sym), [bank-organized listing](../generated/round-012-disassembly). Current map equals checkpoint hash8bea2d22ce7218162a61bcee8f4ca1a7b130e0c95f6238a9e81c9597e886142c. Original ROM identity and configured release unchanged; no toolkit changes or alternate binary.

Toolkit report retains one confirmed T001, zero suspected bugs, zero missing features, zero new findings and no blocker. Explicit input releases used after every restore. [Report](toolkit-issues.md). [Compact handoff](round-012-disassembly-handoff.md). Next coordinator phase is commentary; no final editorial audit performed here.

## Round 013 — one disassembly pass finished; ROM incomplete

ROM classification: 65.23% (42749 / 65536 bytes classified; 22787 bytes unknown).

Input round-012-disassembly and round012 redirect handoff; output round-013-disassembly. Verified R12-STAGE-MUSIC37-41-42-43 (3613 bytes) and R12-LITERAL-AUDIO20-28 (47 bytes), adding 3660 Static Data bytes in 18 regions. All 946 prior region objects, metadata and unrelated internal fields are preserved exactly; all 964 current regions carry role/evidence. Recovery and retraction history remain on disk. [Independent proof](round-013evidence/verified.json), [consumer/byte inspection](round-013evidence/inspection-rpc.json), [preservation](round-013evidence/preservation-check.json).

Actual source05aa reads table05bc values37/42/41/43 with no local mask. This slice is explicitly conditional on stage0..3, consistent with stage initialization and masked progression; it does not bound every possible c02b writer. Source0a0e masks c02b&3 locally and selects40/48 at0a1e. Existing Code bank1:5971 submits decimal28 and7af7 submits20. Fixed00bd saves caller bank via7fff, selects2 via2180, calls4706, then restores caller bank. Bank2 physical offsets equal CPU+4000. 4712 uses4916+2*(command-1); priority and low-six-bit mask precede packed selected-channel pointers. 4763 consumes a pointer before unsigned priority comparison: equal/greater new priority accepted, lower rejected while maintaining alignment. Exact selected headers/directories only; no global command directory extent inferred.

Independent reconstruction starts all18selected channel graphs from their actual packed pointers, with single return/marker slots and zero counter. FD overwrites return; FC resumes it; FB sets marker without clearing count. FE FF consumes an absolute target, other FE increments eight-bit count then either repeats marker or clears count and advances. E0 widths are4/3/4/4/4/2 by channel, E7..EF consume one parameter. Return/marker are assigned before use. Exact event bytes, event boundaries, edges, state counts and stop/cycle closure match redirect evidence; shared events are deduplicated. Additions comprise8source bytes,12directory bytes,48header bytes,4inline dispatch bytes,2selected-word bytes and3586event/operand bytes. Sparse consumed bytes alone are classified, with no min/max fill. Command20/28 FF endpoints are structural; stage graphs have exact state cycles. No natural execution, playback duration or global selector claim follows.

Fresh aligned EA30/34/38/40/42 events establish the previously missing high-nibble3/4 domain, justifying82b3..82b6 as inline words selecting existing Code42e7/42f3. Prior round010 retraction was correct for its evidence and is preserved; these new source events reopen that narrow slice. Handler42e7 sets envelope through c111/45e6;42f3 toggles c105 bit1. Neither changes parser-control slots. Exact EA27 selected word8aab..8aac is new; other selected words remain preserved. EA20 still wraps to8b9d, EA16 endsJP41a2 and does not fall through,8aa9..8aaa remain Unknown. Selected values prove no target extent.

This pass is static: no emulator, snapshot, input route or new external gameplay research. Prior11bounded artificial loader/priority/EA tests remain linked in [redirect evidence](redirect/round-012/evidence.json) and [replay](redirect/round-012/research.py), with exact patches, caps and all-eight-button releases(T001); they were not rerun. Existing sourced controls/routes and diminished natural progression history are preserved; no materially improved normal route emerged and unchanged stalled hypotheses were not repeated. Classification provenance remains separate from execution coverage: Observed8365/Static34384 bytes; whole-history fetch coverage was not recomputed and no new Code was added.

Totals Code11718/Data30165/Padding866. Per-bank Code/Data/Padding/Unknown:00=8316/4919/73/3076;01=1229/1401/67/13687;02=1946/8553/37/5848;03=227/15292/689/176. [Metrics](round-013-classification.json), [exact unknown complement](round-013-unknown-ranges.json). Unbounded object-driven sound sources, apparent literal patterns in Unknown source, stage3 flags, bank1 objects, natural gates and later gameplay remain unresolved. Remaining ready lead recipes are consumed; no commentary/redirect performed in this worker. Prior review credit needs reconciliation for new bytes; final editorial acceptance is pending.

Sequential map.validate, map.save to analysis/state.json and map.export to maps/current.sym completed before the fresh checkpoint. Stock validation/generation passed; fresh byte_identical=true covers65536bytes. [Verification](round-013-disassembly-verification.json), [portable stock map](../maps/round-013-disassembly.sym), [bank-organized assembly](../generated/round-013-disassembly). Current.sym equals the checkpoint map SHA2563bff0d75c9f7ef54fbb739ed6c741708d8b9bded0e172a63451bd30fc0149751. ROM and configured release hashes match the redirect input. No toolkit edits or alternate build.

Toolkit report retains one confirmed T001, zero suspected bugs, zero missing features, zero new findings and no blocker. No runtime/snapshot use in this pass. [Report](toolkit-issues.md). [Compact handoff](round-013-disassembly-handoff.md). Next coordinator phase is commentary; ROM remains incomplete.

## Round013 redirect — bounded object source contexts

Ready `R13-OBJECT-7BB9-SELECTED-LISTS`:858 exact Unknown candidate bytes from40 historical descriptor-backed stage/nested contexts,138 unique records plus selected directories and coordinate boundaries. Source/consumer method,52 final bounded tests, negative equality and local assertion retry history, exact sparse ranges and deferred avenues are preserved in[findings](redirect/round-013/findings.md) and[evidence](redirect/round-013/evidence.json). No classification/natural reachability/global domain claim. Protected artifacts unchanged;[verification](redirect/round-013/verification.json). No new toolkit findings;T001 workaround applied throughout.

## Round 014 — one disassembly pass finished; ROM incomplete

ROM classification: 66.54% (43607 / 65536 bytes classified; 21929 bytes unknown).

Input round-013-disassembly and round-013-redirect-handoff; output round-014-disassembly. Verified R13-OBJECT-7BB9-SELECTED-LISTS, adding 858 Static Data bytes in 69 regions. All 964 prior region objects, metadata and unrelated internal fields match frozen input exactly; all 1033 current regions carry role/evidence. Historical corrections, retractions and recovery artifacts remain preserved. [Independent reconstruction](round-014evidence/verified.json), [actual consumer/byte inspection](round-014evidence/rpc.json), [preservation](round-014evidence/preservation-check.json).

Independently recovered all 40 conditional stage/nested contexts from preserved descriptor cases and checked all 98 selector/descriptor source links against actual 44a7 directory words and descriptor first bytes. Loader262f writes that first byte to c412;082e independently resets stage/nested zero. Bank1 CPU addresses equal physical offsets under fixed0/switch1. Source5679 uses7bb9+2*stage then selected-table+2*c412 via RST08. Exact sparse consumption comprises8 stage-directory bytes,80 selected nested-word bytes,690 bytes in138 distinct five-byte records and80 bytes in40 coordinate boundaries. No min/max gap fill, global stage/nested domain or pointer-target code inference.

56b4 subtracts DE from the little-endian coordinate, records unsigned borrow in c3c8, conditionally negates signed BC via0365 and compares magnitude with10. Negative differences outside the window advanceHL by5; positive differences outside it stop. Equality leaves c3c8 zero, so56a5 does not spawn; historical successful constructed tests use coordinate+1. E000 is a comparison boundary for nonnegative DE<2000, not an unconditional sentinel opcode. Exhaustively checked all8192 values in that stated domain: no unsigned borrow and magnitude>=10. No conclusion is extended beyond this condition. Stage1/nested12 at7ed7 retains coordinates0070 then0038; records are not sorted and no natural selection of the second row is claimed.

56e1 reads record+3 as type and+4 as slot, computes c600+32*slot and skips active slots.312e copies record coordinate to object+11/12, parameter at record+2 to object+18, type at record+3 to object+1 and initializes active80. Exact field roles follow consumers, without speculative enemy names. Historical52 constructed calls remain corroboration of local consumers:40 directory caps100(27/29 operations),7 isolated coordinate+1 spawns caps500(199..331),5 DE1000 scans caps2000(41..385). Equality negative tests and parameter-offset correction history remain linked in [redirect findings](redirect/round-013/findings.md), [replay](redirect/round-013/probe.py) and scenarios.json. They were not rerun; no emulator, snapshot, new external research or natural route in this static pass. Existing sourced gameplay and diminished normal-route history are preserved. No materially improved natural route emerged, and unchanged stalled tests were not repeated.

Totals Code11718/Data31023/Padding866. Per-bank Code/Data/Padding/Unknown:00=8316/4919/73/3076;01=1229/2259/67/12829;02=1946/8553/37/5848;03=227/15292/689/176. [Metrics](round-014-classification.json), [exact unknown complement](round-014-unknown-ranges.json). Classification provenance is Observed8365/Static35242 bytes, separate from whole-history fetch coverage, which was not recomputed. No new Code/Observed claim. Stage3 flags, other bank1 objects, source tables1b7b/2134/2f7e, unresolved audio boundaries/dynamic45d6, global selector domains, natural gates and later-stage routes remain unresolved. Ready lead consumed; no other phase performed. Commentary review and final editorial acceptance remain pending.

Sequential map.validate, map.save to analysis/state.json and map.export to maps/current.sym completed before fresh checkpoint. Stock validation and generation passed; byte_identical=true covers all65536bytes. [Verification](round-014-disassembly-verification.json), [portable stock map](../maps/round-014-disassembly.sym), [bank-organized assembly](../generated/round-014-disassembly). Map SHA256 fc21ab998b2e4486ddffa920002ee52ad493ef2c7453e77e4f6f413103200c5f matches current.sym. Input ROM and configured release hashes remain unchanged; no toolkit edits or alternate binary. [Final verification](round-014evidence/final-verification.json).

Toolkit retains one confirmed T001, zero suspected bugs, zero missing features, zero new findings and no blocker. Static-only pass; historical input restoration workaround retained. [Report](toolkit-issues.md). Next coordinator phase: commentary. [Compact handoff](round-014-disassembly-handoff.md).

## Round014 redirect — independent bank2 placement records
Fresh source-bound avenue from fixed3079/308a to bank2:663f, reusing only accepted descriptor-context identities. Found exact40 selected contexts/155 distinct persistent-placement records,943 Unknown candidate bytes.30d7/30ee prove CBxx flag-index field, c580 four-slot pool and runtime type2 conversion; bank1 slot semantics do not transfer. Unsigned30a2 comparisons bound E000 only under stated lower/upper assumptions; nonmonotonic1/15 retained.51 final artificial tests passed with exact source-contract setup; two failed local setup attempts retained and corrected. [Findings](redirect/round-014/findings.md),[evidence](redirect/round-014/evidence.json),[ready lead](leads.json),[handoff](round-014-redirect-handoff.md). No map/classification/source/toolkit mutation. Concrete new batch ends this pass; generic/raw object consumers and2134 inspected, but transformed types do not establish their independent caller/bank contexts. Remaining main-state/audio boundaries,globaldomains,stage3flags,dynamic45d6,natural row selection remain unresolved; no unchanged stalled experiment rerun. Existing T001 only,no new toolkit findings/blocker.

## Round 015 — one disassembly pass finished; ROM incomplete

ROM classification: 67.98% (44550 / 65536 bytes classified; 20986 bytes unknown).

Input round-014-disassembly and round-014-redirect-handoff; output round-015-disassembly. Verified R14-PERSISTENT-PLACEMENTS-663F with943 new Static Data bytes in68 regions. All1033 prior region objects/metadata and unrelated internal fields preserved exactly;1101/1101 current regions carry role/evidence. Historical corrections, retractions and recovery artifacts remain untouched. [Independent proof](round-015evidence/verified.json), [consumer and ROM inspection](round-015evidence/rpc.json), [preservation](round-015evidence/preservation-check.json).

Independently reconstructed40 accepted conditional stage/nested contexts from prior verified descriptor contexts, rechecked98 selector/descriptor links against44a7 ROM words and descriptor first bytes;262f writes first byte to c412 and082e resets initial stage/nested zero.3079 explicitly selectsbank2 through2180 and restores prior bank.308a gates c009&7==1,3095 selects663f directory using stage&3 then c412 through RST08. Physical bank2 aliases CPU=physical-4000 with fixed0/switch2. Exact sparse additions comprise8stage-directory bytes,80nested-word bytes,775bytes in155 distinct five-byte rows and80bytes in40 E000 coordinate boundaries. No gaps inferred from ranges or export fallback.

30a2 uses RST20 unsigned HL-DE subtraction: skip coordinate<lower, stop coordinate>=upper, otherwise check flag/spawn then advance5. E000 is an upper comparison boundary only when lower and upper<=E000, checked for all57345 unsigned bound values in that domain; it is not an unconditional sentinel and bank1 signed-distance grammar does not apply. Stage1/nested15 retains coordinates0088 then0058 without sorting or natural second-row selection claim.30d7 uses field+4 as CBxx persistence flag index.30ee searches four32-byte slots at c580;312e copies coordinate to+11/12,parameter+2 to+18 and original type+3 to+1.30ee then writes runtime type2, preserves original type at+1b and flag index at+1c, and marks CBxx flag80. No global original-type/index domains or sprite/enemy names inferred.

Historical51 bounded artificial tests corroborate local consumers only:40directory28ops cap100,6isolatedspawns171/172ops cap500,5scans30..562ops cap3000. Exact RAM/register patches, all8button releases after restore(T001), budgets and corrected setup retries remain in [redirect replay](redirect/round-014/research.py), [tests](redirect/round-014/tests.json), [findings](redirect/round-014/findings.md) and scenarios.json. These were not rerun; no emulator/snapshot/new external research/natural route this static pass. Existing sourced gameplay routes and diminished natural-route results retained. No materially improved ordinary route emerged; unchanged stalled leads were not repeated.

Totals Code11718/Data31966/Padding866. Per-bank Code/Data/Padding/Unknown:00=8316/4919/73/3076;01=1229/2259/67/12829;02=1946/9496/37/4905;03=227/15292/689/176. [Metrics](round-015-classification.json), [exact unknown complement](round-015-unknown-ranges.json). Provenance Observed8365/Static36185 bytes; execution coverage remains a separate historical metric, not recomputed. No new observed/code claim. Other raw main-state/audio boundaries,1b7b/2134/2f7e selectors,dynamic45d6,stage3flags,global domains and natural later gameplay remain unresolved. No other phase or worker; commentary/final editorial review remain pending.

Sequential map.validate, map.save analysis/state.json and map.export maps/current.sym completed before fresh checkpoint. Stock validation/generation passed; byte_identical=true for65536bytes. [Verification](round-015-disassembly-verification.json), [portable stock map](../maps/round-015-disassembly.sym), [bank-organized assembly](../generated/round-015-disassembly). Current/checkpoint map SHA256564c50144aced8cc93555119f375ce7654bff05935610a854dafe40e8f438d0b. ROM and configured release unchanged; no toolkit changes or alternate binary. [Final verification](round-015evidence/final-verification.json).

Toolkit retains one confirmed T001, zero suspected bugs/missing features, zero new findings and no blocker. [Report](toolkit-issues.md). [Compact handoff](round-015-disassembly-handoff.md). Next coordinator phase: commentary.

## Round015 redirect — finite downstream object consumers

New evidence from accepted bank1types/bank2originals resolves sparse dispatch/sprite structure without global domains.30 runtime-type words and7 converted-type animation bytes feed9 bank1 animation pointers; real160c grammar closes22 stream nodes/sparse payloads.51 dispatch/conversion/initializer and44 renderer calls pass. Additional22 state0 wrapper calls capture zero-drop Instruction/Read evidence:102 unknown executed prefix bytes and36 first-word data bytes; destination bodies only reached. Combined375Unknown bytes ready in two newR15leads; no classification/source edits. [Question,contracts,source sets,exact recipes](redirect/round-015/findings.md),[verification](redirect/round-015/verification.json),[leads](leads.json),[handoff](round-015-redirect-handoff.md).

Do not repeat unsupported global-domain/gameplay closure or assume original+1b feeds2f7e(+9). Do not transfer bank1 animation table to bank2:46d9, which yields RAMc238 at index0. Persistent wrapper's later state0 resets bank1 animation13. Further reached state0 bodies and other direct target forms are next concrete static opportunities if independently inspected, not execution evidence. All tests artificial;T001 releases applied. Local scratch/schema mistakes corrected and recorded; no new toolkit issue. Protected artifacts unchanged; existing byte-identical checkpoint retained.

## Round 016 — one disassembly pass finished; ROM incomplete

ROM classification: 68.55% (44925 / 65536 bytes classified; 20611 bytes unknown).

Input round-015-disassembly, round-015 redirect and existing commentary history; output round-016-disassembly. Both ready R15-FINITE-OBJECT-DISPATCH-SPRITES and R15-FINITE-INITIAL-STATE-WRAPPERS verified. Added375 bytes in115 regions:102 Observed Code,273 Static Data. All1101 prior region objects/metadata and unrelated internal fields preserved exactly;1216/1216 current regions have role/evidence. Prior names, reviews, history, retractions and recovery files retained. [Independent proof](round-016evidence/verified.json), [inspection/mutation journal](round-016evidence/mutation-rpc.json), [preservation](round-016evidence/preservation-check.json).

Reconstructed29 accepted bank1 placement types plus persistent runtime2 from prior exact context rows;312e retains bank1 runtime type while30ee separately stores original+1b and sets runtime2.33a8/RST00 selects thirty exact words at33ae+2*type,60 new Data bytes. Accepted original0..8 convert through2f45 to0..6:7 maps0/2/5 by c51c,8 maps2/3 by tick low nibble<13.2f0a overwrites+1b with conversion, then selects seven2f77 animation bytes.2f7e reads separate+9; no selector domain inferred. State0 animation13 and initializer0 plus selected14/15/16/17/58/73/74 give nine bank1:46d9 pointer words. Fixed0/switch1; bank1 physical=CPU. Coincident bank2:46d9 is excluded.

Independent finite grammar walk closes22 nodes. FE contains count/control,duration/reserved and payload pointer; FF contains jump pointer; count0 is stop. Payload count/duration precede sparse x/y/tile components: odd tile inherits flags, even tile adds explicit attribute XOR flags.16fe reads duration even for animation0; RST30 offsets destination, not source. Only exact roles classified; skipped/reserved fields retain structural record roles, no intervening gaps. Pointer/frame/payload slice adds170 Data bytes, plus7 converted selectors and60 type words=237. Consumer decoding and every added exact range inspected before annotation.

Twenty-two historical state0 cases cover20 unique6-byte wrappers. Independent trace audit checks actual bytes3e02cddc32c7; physical/CPU/bank aliases; fetched entry,entry+2,entry+5; first inline word's two Read events; zero detail/aggregate drops;26operations/cap100. First-state target bodies were reached but NOT fetched. Only17 unknown wrapper prefixes102bytes and18 first words36bytes added. No global inline-table extent or body execution inferred. Historical51 dispatch/conversion/initializer plus44 renderer tests corroborate static data; no new emulator/snapshot or natural route this pass. Exact setup, budgets, inputs and local retry history preserved in [redirect findings](redirect/round-015/findings.md) and scenarios.json. Ordinary gameplay previously diminished; no materially improved route identified, unchanged stalled leads not repeated.

Totals Code11820/Data32239/Padding866. Per-bank Code/Data/Padding/Unknown:00=8316/4986/73/3009;01=1331/2465/67/12521;02=1946/9496/37/4905;03=227/15292/689/176. [Metrics](round-016-classification.json), [exact unknown complement](round-016-unknown-ranges.json). Provenance Observed8467/Static36458 bytes; execution coverage remains separate historical evidence, not recomputed. Reached first-state bodies and other direct handler forms remain useful contextual inspection candidates only. Raw main-state/audio boundaries,selectors1b7b/2134/2f7e,dynamic45d6,stage3flags,global domains and natural nonmonotonic-row/later-stage reachability remain unresolved. No other phase or worker performed; final editorial acceptance pending.

Sequential map.validate, map.save analysis/state.json and stock map.export maps/current.sym completed before fresh checkpoint. Validation/generation passed and byte_identical=true for65536 bytes. [Checkpoint verification](round-016-disassembly-verification.json), [portable stock map](../maps/round-016-disassembly.sym), [bank-organized listing](../generated/round-016-disassembly). Map SHA2569e20470536766a3cdc9cd5db06d6f2b9c04d7fc1e7f5cd11dcc497f453be20a6. ROM/configured release/commented source unchanged; no toolkit edits or substitution. [Final verification](round-016evidence/final-verification.json).

Toolkit: existing1confirmedT001,0suspected/0missingfeatures,0new/no blocker. [Persistent report](toolkit-issues.md). [Compact handoff](round-016-disassembly-handoff.md). Next coordinator phase commentary.

## Round016 redirect — actual first-state consumers

Concrete question: prior wrapper targets were reached but unfetched; can proper finite actors/context establish body/initializer/sprite roles? Inspected22 targets plus helpers,18 safe constructed calls returned state1 (78..377ops/cap1000);571 actual fetched starts independently decoded with per-event banks,0drops. Two ready leads bound791 Unknown bytes(437Code/354Data); sparse initializer records and ten literal animation selectors close26frame nodes,52renderer assertions pass. No state1-body execution/natural selection/global closure implied. Types25(predecessor),33(companions),45/46(variant parsing) remain separate unsupported setups; no unchanged failed retries. Maps/source unchanged. [Durable findings/limits](redirect/round-016/findings.md),[replay/verification](redirect/round-016/verification.json),[local retry history](redirect/round-016/local-retry.md),[handoff](round-016-redirect-handoff.md). ReadyR16-FETCHED-FIRST-STATE-BODIES andR16-FIRST-STATE-INITIALIZERS-SPRITES.

## Round017 — first-state bodies and sparse initializer/sprite structures

ROM classification: 69.76% (45716 / 65536 bytes classified; 19820 bytes unknown).

Input round-016-disassembly and round016 redirect handoff; output round-017-disassembly. One disassembly pass complete, ROM incomplete. Both ready leads R16-FETCHED-FIRST-STATE-BODIES and R16-FIRST-STATE-INITIALIZERS-SPRITES are verified: 437 Code plus 354 Data bytes added, 105 new regions. All 1216 prior region objects and metadata, unrelated store fields, recovery history, ROM, configured release and commented source remain unchanged. All 1321 regions have role and supporting evidence.

| Bank | Code | Data | Padding | Classified | Unknown |
| --- | ---: | ---: | ---: | ---: | ---: |
| 00 | 8316 | 4986 | 73 | 13375 | 3009 |
| 01 | 1768 | 2819 | 67 | 4654 | 11730 |
| 02 | 1946 | 9496 | 37 | 11479 | 4905 |
| 03 | 227 | 15292 | 689 | 16208 | 176 |

Independent analysis.structure/analysis.decode inspection of all 571 actual fetched starts retained physical/CPU aliases and effective context; type3 temporarily fetches physical8706 as CPU4706 in bank2 audio before restoring bank1. Historical18 constructed wrapper-to-return tests retain separate Instruction/Read/Write arrays, zero detailed/aggregate drops,78..377operations/cap1000, returnSPdff2 and state+2=1. No new runtime or snapshot used this pass. Execution evidence is historical and finite, distinct from classification: writing state1 proves no state1-body execution. Code added only actual fetched instruction extents, not static reachability or reached targets.

Independently inspected32b2/31d8 confirms exact4+4+2 and4+4 initializer copies, respectively; all78 record bytes have actual CPU Read coverage. Ten literal16fe entry selectors resolve exact bank1 pointer words and26 closed frame nodes. Sparse FE indirect payloads, FF jumps, count0 stops and component count/duration/Y/X/tile/optional attributes are independently reconstructed; 52 historical renderer cases cover every node at timers1/2, max423operations/cap1000. Grammar data provenance is Static, confidence0.97; fetched code Observed, confidence0.97. Sparse roles exclude adjacent gaps and do not infer whole actor/state/animation domains.

Sequential map.validate(ok=true), map.save analysis/state.json and map.export maps/current.sym completed before fresh checkpoint. python3 run.py checkpoint round-017-disassembly validated, generated bank-organized source and rebuilt all65536bytes byte-identically; map SHA256 db63b6031c819c689a80c4848d634b0606e1b69ee4dc83c7a35ec9501caf3edf. [Verification](round-017-disassembly-verification.json), [independent proof](round-017evidence/verified.json), [preservation/final audit](round-017evidence/final-verification.json), [unknown intervals](round-017-unknown-ranges.json). Stock map maps/round-017-disassembly.sym; local source generated/round-017-disassembly.

Stop at verified finite easy-gain slice: no unchanged gameplay/70-case replay or recursive actor closure. Types25 predecessor,33 companion allocation,45/46 variant parser, stage flags, dynamic tables, state1 bodies and later natural selection remain deferred with their earlier hypotheses. Types13/14 direct fields do not establish sprite pointers;26/27 prove only observed no-match companion scan. Existing sourced routes and T001 workaround preserved; no new external research or natural gameplay claim.

Toolkit: existing one confirmed T001; zero new, suspected bugs or missing features and no blocker. No toolkit changes/rebuild/substitution. [Persistent report](toolkit-issues.md).

## Commentary010 static correction (no disassembly/redirect pass)
The normal2fc9 contact path reads actor+1b, not base+9: field0b accessor ->INC L ->ADD6 ->ADD9. Earlier round015/016 prose confusing the interior offset9 with a base field is superseded. Frozen maps/history retained unchanged. No new structural/domain/execution claim; exact bytes/static proof in commentary-catchup-010/contact-selector-correction.json, current assembly/uncertainties updated.

## Commentary014 static audio corrections (no disassembly/redirect)
EA3x changes C111 duty/wave selection, with45e6 output to register1 or wave RAM; prior envelope wording superseded editorially. Pitch-sequence sign bit is sticky locally; FE tests current-count equality before FF marker-repeat; work offset24 overlaps pause marker. Code/data classifications/maps untouched; see commentary-catchup-014/changes.json and current source.

## Round017 redirect — connected finite parent/child batch complete

Resolved deferred25 predecessor contract and finite33/45/46 allocation/parser contracts, following actual spawned children49/37 through supported next states, bounds/nonlocal-return and class/tilemap helpers, exact records/commands and literal animations. Ready1347bytes(972actual-fetchedCode+375sparseData);56 actor calls/26renderer checks,22 independent record/pool checks. No classification edits/natural route claim. See [complete research](redirect/round-017/findings.md), [exact sparse proof](redirect/round-017/verification.json), [compact next-phase recipe](round-017-redirect-handoff.md).

Excluded forced-current-FF variants preserve negative analysis history; valid preceding rows proveFF. Remaining neighboring type/speed domains, live slot exhaustion, natural scheduling and older unrelated dynamic/layout/audio obstacles require specific new source facts; no unchanged failed route replay. Redirect complete, next disassembly should verify both R17 ready leads together. Map/source/release preserved;T001 only,no new toolkit issue/no blocker.

## Round018 — connected finite parent/child batch; disassembly pass finished

ROM classification: 71.81% (47063 / 65536 bytes classified; 18473 bytes unknown).

Input round-017-disassembly, accepted commentary catch-up016 and complete redirect017; output round-018-disassembly. Both R17-CONNECTED-PARENT-CHILD-HANDLERS and R17-FINITE-CHILD-STRUCTURES-SPRITES verified together. Added1347bytes:972Observed Code in8 intervals,375Static Data in22 sparse intervals;69 role-specific new map regions. All1321 prior region objects/metadata and unrelated internal fields preserved exactly;1390/1390 resulting regions have role/evidence. No explicit Unknown regions existed to split: additions occupy unrepresented gaps; neighboring old regions remain exact. Prior source/commentary/retractions/history retained.

Independent verification decoded every historical fetched start through the configured server, checked actual opcode bytes and per-event physical/CPU/bank aliases, zero detailed/aggregate drops and SPdff2 returns; matched the saved exact code/role proof. Rewalked literal animation IDs0/56/61/76/77 through13 closed FE/FF/zero nodes and sparse payloads, independently derived finite counts2/3/4, six/three-byte records, four/five-byte commands and single-byteFF, dispatch/pointer words, all four class masks and only speed0/4. All972newCode bytes are fetched instructions, including bounds/nonlocal-POP and class/tilemap continuations; inline dispatch barriers77be..77c2 and78d1..78d7 remain Data. Sentinel padding and intervening speed/directory gaps remain Unknown. Historical56 constructed actor calls(max770,caps1000/1500),22 independent full-pool/record assertions and26 renderer cases(max839/cap1000) reused without unchanged replay. Pointer values/reached targets are not execution evidence; debugger construction does not prove natural selection/timing or general allocator safety. Excluded forced-current-FF tests remain excluded.

| Bank | Code | Data | Padding | Classified | Unknown |
| --- | ---: | ---: | ---: | ---: | ---: |
| 00 | 8316 | 4986 | 73 | 13375 | 3009 |
| 01 | 2740 | 3194 | 67 | 6001 | 10383 |
| 02 | 1946 | 9496 | 37 | 11479 | 4905 |
| 03 | 227 | 15292 | 689 | 16208 | 176 |

Totals Code13229/Data32968/Padding866; execution coverage is separate historical finite evidence. [Metrics](round-018-classification.json), [unknown complement](round-018-unknown-ranges.json), [independent proof](round-018evidence/verified.json), [inspection and mutation](round-018evidence/mutation-rpc.json), [preservation](round-018evidence/preservation-check.json). Sequential map.validate(ok), map.save analysis/state.json, stock map.export maps/current.sym then fresh python3 run.py checkpoint round-018-disassembly passed validation/generation/byte_identical=true for65536bytes. Map SHA256c6d732d75b1a374d1c85e6f9ff9f205f9bfdf47e10389aefcdcf6f98d39cabff. [Verification](round-018-disassembly-verification.json); stock map maps/round-018-disassembly.sym; local bank-organized source generated/round-018-disassembly.

Supported connected continuations are included through parent parsers, child49states0/1 and child37states0/1/2, bounds and class/tilemap consumer. No additional obvious supported unknown interval emerged: neighboring type33 variants and speed entries lack selected source domains; live allocator pressure/natural scheduling requires scene invariants; older dynamic45d6/1b7b/2134/2f7e, stage3flags, later/nonmonotonic selections and audio8aa9 remain distinct missing prerequisites. Redirect017 already investigated these reasonably; no improved natural route or new gameplay condition justified repeating stalled scenarios. Existing sourced gameplay/scenario routes and exact artificial patches remain valid, no new runtime/snapshot/research. One disassembly pass complete; ROM incomplete. Next coordinator phase commentary, preserving accepted source and completing new-region readability; no commentary/redirect performed here.

Toolkit: existing one confirmedT001; zero new/suspected/missing-feature findings, no blocker; no toolkit edits/rebuild/substitution. [Persistent report](toolkit-issues.md).

## Round018 redirect — coherent companion/spawn batch complete
Independently reconciled source speeds0..4 and historical actualReads, correcting
redirect017's0/4-only omission with preserved history. Followed additional
source-produced state1 of types26/27/31 through matched-companion transfer,
bounded type7 allocation, all type7 states,16-byte periodic sequence, finite
animation28 and literal sound26 finite stream. Reviewed persistent-delay caller
also proves31ee8-byte initializer.851unknownbytes now have ready recipes:
657Code(478Observed/179Static)+194Data; no annotations.7actor/20renderer/
16audio calls pass; exact provenance/bank context/negative oracle history retained.
Natural timing/ownership, other7fa4indices and unrelated actor families remain
outside the finite source contracts; no interrupted productive frontier.
[Findings and deferrals](redirect/round-018/findings.md),
[handoff](round-018-redirect-handoff.md),[audit](redirect/round-018/final-audit.json).

## Round019 — disassembly complete; ROM incomplete

ROM classification: 73.11% (47914 / 65536 bytes classified; 17622 bytes unknown).
Input round-018-disassembly; output round-019-disassembly. Verified all three ready
R18 leads: R18-CLASS-CHILD-SPEED123, R18-COMPANION-SPAWN-TYPE7,
R18-PERSISTENT-MOTION-SEED. Added851bytes:657Code(478Observed/179Static),194StaticData.
Independent configured-toolkit decode/structure equality across353 instruction starts,
ROM-based sparse animation/sequence/speed/motion/audio role reconstruction and
historical zero-drop actor Instruction/child Read audit agree with redirect proposals.
Seven actor calls,20renderer cases,16audio calls reused; no new runtime/snapshot or
natural route. Exact recipes remain redirect/round-018/probe.py,consumer.py,audio-probe.py
and scenarios.json. Static code is not reported executed. Preserve736f..7374 inline
barrier; actor fixed0/switch1, bank2 audio CPU=physical-4000. Speed1/2/3 finite
row domain is corrected; no speed5/adjacent selector inference. Animation28 closes
8 payload frames plus jump; audio26 closes20events/24stream bytes toFF. Data roles
and exact additions: [independent proof](round-019evidence/verified.json),
[inspection RPC](round-019evidence/inspection-rpc.json).

|Bank|Code|Data|Padding|Classified|Unknown|
|---|---:|---:|---:|---:|---:|
|00|8316|4996|73|13385|2999|
|01|3397|3348|67|6812|9572|
|02|1946|9526|37|11509|4875|
|03|227|15292|689|16208|176|

Totals Code13886/Data33162/Padding866. All1456regions have role/evidence metadata;
all1390prior region objects/metadata and unrelated fields preserved exactly. Existing
commentary/source unchanged: no readability credit claimed for66new structural
regions. Independent execution metric for this batch478bytes only; historical
execution coverage remains separate. Unknown ranges: round-019-unknown-ranges.json.
Sequential map.validate/map.save/map.export passed; fresh `python3 run.py checkpoint
round-019-disassembly` passed validation,generation and byte_identical=true65536.
Stock map maps/current.sym and maps/round-019-disassembly.sym SHA256
8ffcd2909a343dc6767c8ab4a28a59eba6f242fca7d1608151d4b3047c2d0042.
Generated bank source generated/round-019-disassembly; report
round-019-disassembly-verification.json; [final audit](round-019evidence/final-verification.json).
ROM/configured release and all accepted commentary source files unchanged.

Easy gains in this connected batch are exhausted: remaining parent/state transitions
reach existing helpers/known nodes; wider7fa4 selector domain needs other writers,
natural companion ownership/contact/allocation needs scene prerequisites. Prior
unchanged stalled natural routes were not replayed. Independent actor families
are redirect questions, not unsupported adjacency gains. Next phase commentary:
carry accepted018 source and finish readability for these new regions, then redirect
may investigate other actor wrappers' later states with concrete writer prerequisites.
No commentary or redirect executed in this worker.
Toolkit: existing confirmedT001 only;zero new/suspected/missing-feature findings,
no blocker,no edits/build/substitution. [Report](toolkit-issues.md).

## Round019 redirect — finite type6/type8 state machines closed
New source-directed continuation from accepted first-state writers,not repeat of
prior companion/spawn or failed natural routes. Type6states1..4 andtype8states1..8
share terrain/facing/countdown helpers;local writers close the selected state tables.
401Code plus94Data bytes proposed,with377actual fetched Code/24Static.11finite
animation nodes35/36/41/42 andsound12's terminating6event channel1 stream close
connected assets.34actor/22renderer/6audio bounded artificial calls and18output
contracts;exact negative zero-cache repetition retained,not natural gameplay proof.
Adjacent5c2b/random table is outside selected caller graph;needs independent entry
prerequisite. Other wrappers' later states remain separate opportunities,no unchanged
failed route reopened. No productive work interrupted;readyR19-TYPE6-TYPE8-LATER-STATES
andR19-TYPE6-TYPE8-ASSETS. Maps/source unchanged;classification73.11%,17622Unknown.
[Findings](redirect/round-019/findings.md),[audit](redirect/round-019/final-audit.json),
[handoff](round-019-redirect-handoff.md). ExistingT001 only,no blocker/new toolkit finding.


## Round020 — disassembly complete; ROM incomplete

ROM classification: 73.87% (48409 / 65536 bytes classified; 17127 bytes unknown).
Input round-019-disassembly; output round-020-disassembly. Verified both ready
R19-TYPE6-TYPE8-LATER-STATES and R19-TYPE6-TYPE8-ASSETS. Added495bytes:
401Code(377Observed/24Static),94StaticData. Independently checked configured-toolkit
decode/structure equality for208 starts, graph successor closure, current-actor WRAM
contracts for21 indirect stores, exact state words and finite animation35/36/41/42
roles. Sound12 directory/header/channel1 E0 width and FF stop independently reconstructed;
actual bank2 loader/parser Reads cover all14new audio bytes. Exact sparse roles and
proof: [verified](round-020evidence/verified.json),[RPC](round-020evidence/inspection-rpc.json).

Reused34 actor calls(cap1800,max923),22renderer cases and6audio calls,18output
contracts;historical actor/audio detailed and aggregate drops0. No new runtime,
snapshot or natural gameplay route. T001 explicit8button release recipes remain
in redirect/round-019/probe.py,consumer.py,audio-probe.py and scenarios.json.
Physical actor=CPU bank1; sound wrapper maps bank2 with CPU=physical-4000.
Static24bytes remain Static; executed377bytes is a batch metric,not total coverage.
Artificial terrain caches/contact/marker/timer fields prove local mechanics only.

|Bank|Code|Data|Padding|Classified|Unknown|
|---|---:|---:|---:|---:|---:|
|00|8316|4996|73|13385|2999|
|01|3798|3428|67|7293|9091|
|02|1946|9540|37|11523|4861|
|03|227|15292|689|16208|176|

Totals14287Code/33256Data/866Padding;1489regions all have role/evidence.
All1456prior region objects/metadata and unrelated fields preserved exactly;
33new structural regions await commentary. Existing accepted019commentary source
unchanged;1702regions/1784units prior review credit must be carried. No new source
readability credit claimed. Unknown complement: round-020-unknown-ranges.json.
Sequential map.validate/save/export passed; fresh `python3 run.py checkpoint
round-020-disassembly` validated, generated and verified byte_identical=true65536.
Stock maps/current.sym and maps/round-020-disassembly.sym hash
b82a9709f56fa8649eceab6fe60c4eff45a4c8e8b470e77d1ba37fa6eca31a2c.
[Verification](round-020-disassembly-verification.json),
[final audit](round-020evidence/final-verification.json),
[preservation](round-020evidence/preservation-check.json).
Configured release,ROM and accepted source hashes unchanged.

Connected easy gains exhausted: type6state1..4/type8state1..8 domains and selected
assets close through known helpers. Inspected adjacent5c2b continuation/random
inline table: no selected state/call enters it, so no adjacency classification.
Other actor wrappers require separate source-produced state-domain investigation.
Natural later-state/contact/timing and old stage3flag questions lack a materially
new route prerequisite; prior failed routes not repeated. Redirect may investigate
another accepted wrapper's later states after commentary. No productive connected
work interrupted. Next phase commentary: carry accepted019source/review credit,
explain33new regions with consistent symbols, fresh byte-identical audit and reading
refresh; no commentary or redirect performed here.

Toolkit: existing confirmedT001 only;zero new/suspected/missing-feature findings,
no blocker,no toolkit modifications/build/substitution. Local verification helper
schema/alias corrections were orchestration errors,not toolkit defects.
[Persistent report](toolkit-issues.md).

## Round020 redirect — productive research; SAME PHASE CONTINUES
Accepted020 commentary gate; input/output structural round-020-disassembly,
48409classified/17127Unknown(73.87%),unchanged. Followed all initially identified
accepted later-state wrappers and direct converters through shared allocation,
produced actors, finite animation/audio consumers and missing tails. Current
supported proposal3395bytes=2292Code(1971Observed/321Static)+1103Data. Ready
R20-ACCEPTED-ACTOR-STATE-GRAPHS andR20-ACCEPTED-ACTOR-LITERAL-ASSETS are accumulated
research,not a reason to end redirect.60actor/152renderer/117audio calls passed;
171new audio bytes have actual Reads,all detail/aggregate drops0. Exact sparse
proof,tests,recipes/source context and all protected hashes:[findings](redirect/round-020/findings.md),
[audit](redirect/round-020/research-checkpoint.json). No classification/source/map edits.

Context checkpoint preserves productive frontier: alternate stage622b/6237 now
has newly understood type53/shared helpers; finite2134 sound consumers need
contact/depletion caller filters for newly produced12/15/17/18; raw audioE9
handler8293 needs body/consumer inspection (no selectedE9 in this batch). Reassess
other concrete explicit leads under source evidence,not global-domain requirements
or arbitrary state injection. Unchanged stage3flags/dynamic45d6/natural later routes
remain prior stalled history unless a new prerequisite/method appears. Do not
advance to disassembly before same-phase triage. [Compact handoff](round-020-redirect-handoff.md).
Toolkit existingT001 only,no new findings/no blocker;no toolkit edits.

## Round021 disassembly — supported combined batch; ROM incomplete

ROM classification: 96.95% (63535 / 65536 bytes classified; 2001 bytes unknown).

Added 15126 unique previously Unknown bytes: 7369 Code (4269 Observed, 3100 Static)
and 7757 Data. All five ready R20 lead IDs are verified; their overlapping ranges
are counted once. Exact per-lead retained histories: [leads](leads.json).
Freshly decoded 3625 saved instruction starts with CPU/bank contexts and recomputed
both ROM-driven sparse recipes exactly. Independently audited actual saved Instruction
events, zero detail/aggregate drops and safe RAM return sentinels; reached addresses
and pointer targets alone were not promoted to Observed. Source-state contracts,
local WRAM/VRAM pointer setup and finite consumer roles remain explicit.
[Independent proof](round-021evidence/verified.json), [RPC](round-021evidence/inspection-rpc.json),
[source evidence](redirect/round-020/findings.md). Confidence .97 for new regions.

Supported connected graphs include accepted later actor states/direct converters,
actual runtime17 (historical5d7d/5e86 names corrected to18/19), alternate stage actors,
normalized persistent contact0..6, completion/death/ending0..14, player/action/auxiliary
window/terrain/scroll/camera/HUD continuations and the1719 bank-restoring animation
wrapper. Sparse data covers source-produced words, copied seeds, finite shapes/boxes,
frames/audio/credit/palette/timer/tile structures and conditional stage2/nested1
transition and phase2 placement records. Labels/roles describe mechanical support;
full semantic source commentary is the next separate phase.

Preserved barriers: CALL1a54 discards its immediate return through1a6f;1a57..1a6e
stays Unknown. Type20state7/8 words stay Unknown. Nested1 transition major0..47
and cross0..4 supports exactly240cells3966..3a55; width358c and last25cells3a56..3a6e
are withheld because normal scrolling is bypassed until nested2. Type56 has exactly
two states; AE78..79 remains dual-use Code and waveform copy8523..8532 remains Code.
Wrapper1719 uses bank2 before1720, bank1 at1723..1728 and restores2 before172b.
Conditional HUD/timer/source-state bounds are local lifecycle contracts, not global
invariants or natural reachability. Unsupported parameter cursor5 remains excluded.

|Bank|Code|Data|Padding|Classified|Unknown|
|---|---:|---:|---:|---:|---:|
|00|10076|5676|73|15825|559|
|01|8405|6953|67|15425|959|
|02|2948|13092|37|16077|307|
|03|227|15292|689|16208|176|

Totals21656 Code/41013 Data/866 Padding;2565 regions all have role and evidence.
All1489 prior region objects/metadata and unrelated fields remain byte-for-byte
equivalent in internal JSON;1076 new regions added. No old classification was
split/retracted. [Preservation](round-021evidence/preservation-check.json).
Unknown complement: [exact ranges](round-021-unknown-ranges.json). Execution coverage
is separate:4269 new Code bytes have actual historical instruction fetch evidence;
3100 new Code bytes are static. No whole-ROM execution percentage is claimed.

No new runtime, snapshot, input route or external gameplay research was needed.
Reused historical60actor/36alternate/6contact/11parameter/1EE/10type17/14fixed/
12player/1wrapper/16nested1 case records, original152 and supplemental274 renderer
cases, and source-selected audio records. Seven contact histories include a retry;
only six final contact-tests records are counted above. Exact patches, bank contexts,
button sequences and caps remain in [scenarios](scenarios.json) and redirect020 scripts.
All historical restores explicitly release all8buttons(T001). Artificial observations
prove local mechanics; natural later-stage/contact/ending/timing/ownership/visibility
remain unproved. Prior natural routes and unchanged failures were not replayed.

Connected easy gains are exhausted: no uninspected external Unknown callee remains
in the supported graphs; nested1 phase2 suffix closes its only new code continuation,
placementtypes32/8 are already closed, persistent list is empty and bank3 payloads
repeat known records. Width/tail needs a descriptor or normal-queue producer at
nested1. Stage3 flagged IDs still lack a decoding rule, dynamic45d6 a bounded source
selector, and uncalled0376/1266/2fbd/34af/0a58 a new inspected caller. Failed natural
routes/global domains have no changed prerequisite or useful new method. These
missing facts justify deferral; no arbitrary Unknown sweep or unchanged replay.

Sequential map.validate, map.save analysis/state.json and map.export maps/current.sym
(include_sym=false) passed before fresh `python3 run.py checkpoint round-021-disassembly`.
Stock map validation, generation and rebuild returned ok=true,byte_identical=true
for all65536 bytes. [Verification](round-021-disassembly-verification.json),
[final audit](round-021evidence/final-verification.json). Stock map hash
102123cf64822812ad5f340581918b70dd16a72a18a280390acbf110fd2b6081.
ROM/configured release/workspace config/accepted020 commentary hashes unchanged.
No toolkit edits/build/substitution.

Next phase commentary must preserve accepted020 source and1728region/1810unit review
credit by physical ranges/dependencies, explain all1076 new structural regions with
consistent routine/branch/field/constant symbols, reconcile affected older prose,
finish the entire current-source readability backlog, verify byte identity and
refresh the reading copy. Accepted source remains intact; no commentary or redirect
was run here. ROM remains incomplete until every byte meets structural and final
readability acceptance. Toolkit existing confirmedT001 only;zero new/suspected/
missing-feature findings,no blocker. [Persistent report](toolkit-issues.md).

## Round021 redirect source-frontier reconciliation

[Findings](redirect/round-021/findings.md) and [handoff](round-021-redirect-handoff.md)
close corrected ordinaryCALL1a54/leftfacing tails, sixotherordinarygaps,source-selected
runtime20seeds/playeraction/runtime1/animationgraphs and source-preserving incoming
contact. Ready396bytes=56Code(32fetched/24static)+340Data;classificationunchanged,
2001Unknown. Eleven bounded constructedcalls passed withzero drops/all8buttons
releasedafterrestores. Priorrawinputcachefailure retained,correct sampledinputs used.
No naturalrouteclaim. All oldfailedleads retained; no unchangedreplay.

Finalsourceavenue reassessment covers12controlfrontiers,8immediatepointerroots,
50explicitboundarycontracts/currentuncertainties; no productivefrontier remains.
Deferrals have actual missingproducer/selector/flag/context facts or repeatedsame
mechanics,documented in findings.md. RST0words are not executablefallthrough.
Incominggatebit5,auxiliarybit6;postconversiontypes7/15/17 differ from41/42/43.
Animation60from E=3c@2192 accepted,unrelated3cworkdelaynot source. Secondpacked
spritepayloadbyte05 ignored;nextcommentarymust remove eightlegacy'shape5'claims.
No source/map/state/ROM/release change. ExistingT001only,no newtoolkitissues.

## Round022 disassembly — three supported leads verified; ROM incomplete

ROM classification: 97.55% (63931 / 65536 bytes classified; 1605 bytes unknown).

Input checkpoint round-021-disassembly and completed021 commentary2716records/2854units
(pending0). Output fresh round-022-disassembly; only disassembly ran. Added396unique
previouslyUnknown bytes:56Code(32Observed/24Static)+340Data,89newregions. Exact
all2565prior region objects/metadata/unrelated fields preserved;2654current regions
all have role/evidence. [Preservation](round-022evidence/preservation-check.json).
No baseline import or reclassification; accepted021commentary source unchanged.

Consumed three ready IDs,now verified with exact histories retained in[leads](leads.json):
R21-ORDINARY-CONTINUATIONS(56Code),R21-SOURCE-SELECTED-ASSETS(324Data),
R21-CONDITIONAL-INCOMING-CONTACT(16Data). Classification is their exact union,not
range hulls or an unchecked lead sum. [Recomputed recipe](round-022evidence/recompute.py),
[contexts/decodes/roles](round-022evidence/verified.json),[independent audit](round-022evidence/audit.json),
[mutation RPC](round-022evidence/mutation-rpc.json). New confidence .97.

Independent fresh decode/structure inspected every instruction start with CPU aliases
and effective fixed0/switch1 or bank2 context; the ROM-driven recipe was recomputed
in a separate output directory and matched redirect021 exactly. Independent component
walking closed18frame nodes for source-selected animations1/3/4/7/60:298newData bytes.
Additional Data: two9byte sparse runtime20 seeds, six saved-ground/air actionbytes,
runtime1dispatchword, twelve source-compatible incoming-contact amountbytes and
shape6/12boxpairs. No neighboring selector/pointer/state or graph gap was inferred.
Data provenance remains Static; actual saved Reads separately corroborate17newData
bytes in the combined tests,including13incoming-contact bytes.

Historical actualInstruction events were audited against ROM opcode values,physical
banks,CPU aliases and safe return sentinels;32Code bytes have actual fetch support.
Two climbing-A calls215ops/cap600 returnC302/SPdff4; nine source-output-preserving
overlaps494ops/cap1200 returnC300/SPdff2;all detail/aggregate drops0. Exact sampled
C418/C41A inputs,bank/actor/player/stack patches, persistent snapshot and explicit
all8buttonrelease recipe remain in[scenarios](scenarios.json),redirect/round-021/
probe.py/contact.py and tests.json/contact-tests.json. No replay was necessary;
failed rawC00C/C00D experiment remains negative evidence only. No new natural route,
external research,runtime,input or snapshot. Execution support is separate from
classification:24newCode bytes remain statically justified ordinary continuations.
No whole-ROM execution percentage or natural timing/visibility/contact claim.

Code roles:1962+9 clears facing flag20;1a57+23 initializes jump/motion after ordinary
CALL1a54->1a6f and finallyPOP1a6c discards CheckA's caller;20ae+5 submits sound12;
22a1+6 propagates velocity subtraction borrow;2316+2 clamps vertical highbyte7;
75cb+3 sets carry before existingclassmask logic;7659+1 returns from selectedtileclear;
physical8564+7 CPU4564/effectivebank2 clears repeatcount,advances pitchcursor twice,
loops4506. The skippedPOP1a6e is DISTINCT from finalPOP1a6c. Incomingcontactrequires
shape bit5,auxiliarycontactbit6;converters41/42->runtime7,43->15,child17 supply
postconversion amount indices. Type25 effect E=3c@2192 establishesanimation60;
workdelay3c@5c3d is unrelated. Payload secondbyte05 ignored,never actorshape5.

|Bank|Code|Data|Padding|Classified|Unknown|
|---|---:|---:|---:|---:|---:|
|00|10121|5688|73|15882|502|
|01|8409|7281|67|15757|627|
|02|2955|13092|37|16084|300|
|03|227|15292|689|16208|176|

Totals21712Code/41353Data/866Padding. Exact remaining complement:
[unknown ranges](round-022-unknown-ranges.json). Unknown counts502/627/300/176.
Classification and fidelity gates are separate. Sequential map.validate,map.save
analysis/state.json,map.export maps/current.sym(include_sym=false) passed before
fresh `python3 run.py checkpoint round-022-disassembly`: stock map validation,
generation and rebuild ok=true,byte_identical=true for all65536bytes.
[Verification](round-022-disassembly-verification.json),[final audit](round-022evidence/final-verification.json).
Maphash879ee77c6e8220af0bc278ca0eecd7dd484df2e33333a46187ced78c7ee50c78.
ROM,workspaceconfig,configuredrelease and accepted021source hashes unchanged;
no toolkit edits/build/substitution.

Easy gains exhausted under changed-evidence policy: inspected connected routines
and accepted downstream structures close; redirect021's source-frontier audit found
no additional productive continuation. Ordinary gameplay's preserved failed routes
have no changed prerequisite/method or credible new substantial gain; unchanged
replay would repeat negative history. RST words require selectorproducers;runtime0/
20state0/7/8/21state0 needactual occupied/statewriters;remaining contactslots need
compatible source type/bit5/shape rather than arbitrary patches. Nested1width358c/
layouttail3a56..6e needdescriptor/normalqueueproducer;stage3flagIDs needdecode rule;
dynamic45d6 needsboundedselector;uncalledhelpers neednewcaller;audio/graphics
neighbors lacksource-selected consumers. No arbitraryunknown sweep. Supporting
negative/source-domain reasoning retained in[redirect findings](redirect/round-021/findings.md).

Next ONLY commentary: carry accepted021source and all2716records/2854units prior
review credit by physical dependencies. Complete readability for89newregions and
affectedolderprose,including newlyclosed unknown-boundary notes and eightlegacy
TypeSevenContactSprite1..8Header descriptions: replace'shape5'with ignoredheaderbyte.
Preserve endingheldA,converter18/19field1eFF,replenished21quota,stage3A/HL/preDEC
anddualuse findings. Finish entire current-source scope,verify byteidentity,refresh
reading;no partialrepair gate. Full-ROM finalacceptance still requires1605Unknown
resolved plus finaleditorial audit. Toolkit existingconfirmedT001 only;zero new/
suspected/missingfeatures,no blocker;[persistent report](toolkit-issues.md).

## Round022 redirect — completed reassessment; no actionable new lead

ROM classification remains97.55% (63931/65536),1605Unknown; perbank502/627/300/176.
Input/output checkpoint round-022-disassembly unchanged; current readability complete
2788records/2926units,pending0. Fresh bounded scan11266classified starts finds only
4knownRSTinline-tablefrontiers, no ordinary missingcallee/fallthrough; immediate
Unknown roots8->4,all unchanged conditional bases. Current102changedcontracts,
source boundaries/new seeds/assets/converters/contact and older layout/audio/state
prerequisites reassessed. No new/improved ready lead; passes entry added with empty
added/updated/ready arrays, all prior outcomes retained. [Findings and actual missing
prerequisites](redirect/round-022/findings.md),[compact handoff](round-022-redirect-handoff.md).

Fresh configuredrelease map.validate ok=true; all protectedROM/state/map/config/
release plus7commentarysourcehashes/readingcopy match. [Verification](redirect/round-022/verification.json).
Accepted022disassembly and freshcommentary byteidentical65536 checks remain valid;
no new checkpoint,generation,archiveaudit,map/source edits or reclassification.
No runtime/input/snapshot/external research; unchanged failednaturalroutes not replayed.
Existing scenarios and explicitall8buttonreassertion T001 workaround retained.
Remaining unknown complement round-022-unknown-ranges.json unchanged. Stopworkflow
incomplete until new selector/producer/caller facts or improved justified method;
this is not wholeROM completion. Toolkit existing1confirmedT001,zero new/suspected/
missingfeatures,no blocker; [report](toolkit-issues.md). No toolkit edit/build/substitution.


## Round023 — disassembly complete, ROM incomplete

ROM classification: 99.03% (64900 / 65536 bytes classified; 636 bytes unknown).

Input round-022-disassembly plus authoritative final-investigation FI022 final969
manifest (preliminary311 superseded). Output round-023-disassembly. Accepted exact
60-span969-byte union:198StaticCode and771StaticData,zero newlyObserved. Independent
configured-release decode/structure checks cover128starts/128edges and full bank/CPU
aliases; five ROM grammar recipes recomputed into this pass's artifacts match the
saved proofs exactly. Reviewed arithmetic/carry, stack/return, saved-target accessor,
palette/serial contracts; FE/FF animation graphs and parity-counted payloads; explicit
0..5 NR51 channel bound; explicit high-E0 transition termination and fifth-field
propagation through28descriptor-output contexts; coordinate-bounded placements;
source34/35 setup counts5/2,FFcommand closure and literal child37; noise mask02
headers/FFstop; pitch note-initialized count0/cursor0 FEFF loops. Exact sparse extents
only, no arbitrary alias/table/placement expansion. All four FI22 ready leads verified.

|Bank|Code|Data|Padding|Classified|Unknown|
|---|---:|---:|---:|---:|---:|
|00|10278|5731|73|16082|302|
|01|8413|7801|67|16281|103|
|02|2992|13236|37|16265|119|
|03|227|15356|689|16272|112|

Total Code21910/Data42124/Padding866. All2714regions have role/evidence metadata;
all2654prior regions and metadata plus unrelated state fields preserved exactly.
Observed-provenance Code remains15032bytes; static classification does not increase
execution coverage. Accepted022commentary source and reading copy remain unchanged;
2788records/2926units pending0 credit is prior-source credit, not a new-source review.

Caveats:2fbd uses saved contact-targetc3d2/c3d3,not currentactorc416/c417. Palette
index3 touches c016;B0 wraps the final loop. Serial software handshake writer and
working feature unproved. Animation29/37,sound4/25 natural selection unproved.
PitchFEFF closure assumes ordinary timed-note count0 initialization,not inherited255.
No hardware compatibility,timing,dead-code or natural gameplay claim.

No emulator/input/snapshot/debugger patch/external research or new normal route.
Reused valid evidence; unchanged failed routes not repeated. No concrete improved
natural route or further supported connected extension remains: types34/35/37 close
through accepted handlers/assets; new noise streams introduce no further selectors;
pitch graphs close within scoped initialization. Residual methods/barriers remain
in final-investigation/range-ledger.json. Exact636-byte complement saved to
[unknown ranges](round-023-unknown-ranges.json),matching FI projected complement.
Isolated RETs,unbounded frequency/alias tables,unsupported selector amounts,weak
fixed records,stage2ordinary-width and stage3flagged-ID questions remainUnknown.

Sequential configured-release map.validate/save analysis/state.json/export
maps/current.sym include_sym=false passed. Fresh `python3 run.py checkpoint
round-023-disassembly` passed stock map validation,generation and byte_identical=true
for all65536bytes; mapSHA256
712619d00b78c4033a2b9a8fa4ce9c012538d91f7af323d5a932ecade09c9a0e.
[Verification](round-023-disassembly-verification.json),
[independent proof](round-023evidence/audit.json),
[preservation](round-023evidence/preservation-check.json).
ProtectedROM,configuredrelease,configuration and accepted022source hashes match.
Initial annotation attempt used unsupported capitalized provenance and was rejected
before persistent mutation; lowercase retry passed. Local API usage correction,
not a toolkit defect. No toolkit edit/build/substitution or commit.

Next phase commentary: complete ALL current classified source, preserve sound prior
credit, explain/name the60new regions and update old source claims narrowed to type33/
selector0 or formerly unclassified slices. Preserve all caveats above and existing
dual-use code waveform interpretation. Then redirect; this worker launched no other
phase. Remaining unknown bytes prevent completion regardless of byte fidelity.
Toolkit:existing one confirmedT001,zero suspected/missingfeatures/new findings,
no blocker. [Persistent report](toolkit-issues.md).

## Round023 redirect — no new structural avenue

Completed changed-source and connected/independent reassessment on64900classified/
636Unknown.128new contextual instruction starts have no Unknown direct successor;
28transition contexts, parent34/35/child37 command/speed contracts, independent
noise4/25 and note-initializedpitch7/9/10 close over accepted objects. Residual
producer/extent/bank barriers unchanged; no arbitrary state sweep or natural-route
replay. [Detailed findings and exact missing facts](redirect/round-023/findings.md),
[protection/API verification](redirect/round-023/verification.json).
One stale channel1-unclassified sentence in AudioRoutingMaskChannelZero remains
an editorial correction for final acceptance; source preserved. No structural
ready lead or interrupted frontier. FI022-20261018-v1 final969 completed and accepted;
new round number alone does not authorize automatic repetition. ExistingT001 only,
no new issue/blocker/runtime/toolkit mutation.

## Round024 disassembly — FI23 ending credit block8 accepted

ROM classification: 99.07% (64929 / 65536 bytes classified; 607 bytes unknown).

Input round-023-disassembly and accepted023commentary2798records/2993units pending0;
output round-024-disassembly; phase complete, next commentary. Consumed
FI23-ENDING-CREDIT-BLOCK8: verified exact29StaticData, zeroObserved, two new regions.
Independent configured-release207 contextual responses match; literal branch review
and fresh abstract byte recurrence draw0..8/erase0..7, state6 exit after4130 calls.
Index8 draws at phase1 before phase17 cp8 exit; states7/8 preserve it, state9 phase36
erases it. Word physicalac42/CPU6c42 -> CPU6d1a bank2; count3 addressed FF runs
9/2/6tiles to9b45/9b89/9bc7 terminate exactlyphysicalad35/CPU6d35 acceptedCode.
Prior range(8) extraction was a processing limit, not a supported selector domain.
[Independent audit](round-024evidence/audit.json),[API checks](round-024evidence/verification-rpc.json).

|Bank|Code|Data|Padding|Classified|Unknown|
|---|---:|---:|---:|---:|---:|
|00|10278|5731|73|16082|302|
|01|8413|7801|67|16281|103|
|02|2992|13265|37|16294|90|
|03|227|15356|689|16272|112|

TotalCode21910/Data42153/Padding866; all2716regions have role/evidence. All2714prior
objects/metadata and unrelated state preserved exactly. ObservedCode15032bytes
unchanged; structural classification is separate from execution coverage. Exact
607-byte complement in82ranges matches FI023 projected residuals:
[Unknown ranges](round-024-unknown-ranges.json). Accepted023source and reading copy,
ROM, workspace configuration and configuredrelease hashes preserved.
[Preservation](round-024evidence/preservation-check.json).

Sequential map.validate/save analysis/state.json/export maps/current.sym
include_sym=false passed. Fresh `python3 run.py checkpoint round-024-disassembly`
passed validation, stock generation and byte_identical=true for all65536bytes.
MapSHA2568055aa2c6f8a255518ee939a27a063d649adee45e517660fc1519d78a9d48b95.
[Verification](round-024-disassembly-verification.json). Rebuild fidelity does not
classify remaining unknown bytes; ROM remains incomplete.

No runtime/input/snapshot/debugger patch/external research or new natural route.
Reused valid saved evidence; unchanged failed gameplay not replayed. Connected
credit graph closes at accepted renderer/eraser and directVRAM tile IDs; no further
command/actor/copy selector. Six literal ending initializer copies cover no residual
byte. Shape terminal count4 has the recorded odd side-counter obstruction; 11-bit
frequency values do not prove allocation length. No further supported extension
without materially improved residual method; retain FI023 fullledger/history.

Next commentary must complete ALL current-source readability while preserving sound
2798record/2993unit credit, carrying two new regions and correcting prior descriptions
that state6 draws only0..7 or advances before drawing8. Also resolve the recorded stale
channel1-unclassified sentence in AudioRoutingMaskChannelZero. No source editing here.
Then redirect and iterative final investigation while eligible607Unknown remain;
this pass is not a terminal handoff. No other phase/goaltools/toolkit edits/builds.
Toolkit: existing confirmedT001 only, zero new/suspected/missing-feature findings,
no blocker. [Persistent report](toolkit-issues.md).

## Round025 disassembly — conditional screen-major nested1 accepted

ROM classification: 99.11% (64955 / 65536 bytes classified; 581 bytes unknown).

Input round-024-disassembly and accepted024commentary2798records/2994units,pending0.
Output round-025-disassembly; disassembly complete, next commentary. Consumed
FI24-SCREEN-MAJOR-NESTED1 verified:26StaticData,zeroObserved,six new regions:
358c+1,3a56+13,3a65+3,3a6a+3,3a6f+3,3a74+3. Independent literal repeated-subtraction
and repeated-addition simulates all five contexts, rather than copying a contiguous
extent assumption. Queue uses stage&3/nested width, column<53; activates cross0;
consumer completes after cross5. Resolver uses identical stage/nested globals and
25*(column//5)+5*row+column%5. Stage2 directory roots351d/358b, independently rooted
stage1 starts3527/3590, and all five pointer/width pairs establish the specific
conditional structure. Widths53/53/53/10/10 give exact five screen envelopes;
accepted ordinary nested0/2 corroborate sparse consumption. This is structural
ownership through consumer contracts, not adjacency or a maximum observed selector.

Nested1 has265cells within275bytes;25new cells pluswidth358c accepted. Ten holes
3a63..64,3a68..69,3a6d..6e,3a72..73,3a77..78 remain Unknown. Source fixedbank0;
resolver temporarily switch1, wrapper restores3 for full-byte rawID payloads
CPU7320+16*ID,physicalf320+16*ID. All18rawID payloads alreadyclassified; no new
connected payload frontier. Transition mask3f does not apply to ordinary resolver.
Natural ordinarynested1 selection remains unproved; known transition increments1
to2 before ordinary queue. No runtime/input/snapshot/externalresearch or unchanged
failed gameplay replay. [Independent proof](round-025evidence/audit.json),
[385context responses and sequential mutation](round-025evidence/rpc.json).

|Bank|Code|Data|Padding|Classified|Unknown|
|---|---:|---:|---:|---:|---:|
|00|10278|5757|73|16108|276|
|01|8413|7801|67|16281|103|
|02|2992|13265|37|16294|90|
|03|227|15356|689|16272|112|

TotalsCode21910/Data42179/Padding866;2722regions. All2716prior objects/metadata
and unrelatedstate preserved exactly; accepted024source/reading,ROM,release and
configuration hashes unchanged. ObservedCode15032 unchanged. Exact581Unknown
in85ranges matches FI024 projected residuals; [complement](round-025-unknown-ranges.json),
[preservation](round-025evidence/preservation-check.json).

Sequential map.validate/save analysis/state.json/export maps/current.sym with
include_sym=false passed before fresh `python3 run.py checkpoint round-025-disassembly`.
Stock validation/generation/verification passed byte_identical=true for65536bytes;
mapSHA2569e884470ae4551c0b8e50e0a8ad1d14db8807d5c6779eea4f8f5886d712c624f.
[Checkpoint](round-025-disassembly-verification.json). ROM incomplete; fidelity
and classification remain separate gates. Local verifier initially named next
independently rooted directory stage3 instead of actualstage1 and compared the
residual wrapper object to a bare list; corrected locally, not toolkit defects.

Commentary next must complete ALLcurrent-source readability, preserving sound
2798records/2994units credit; carry sixnewregions and update sparse-screen geometry,
width and conditional reachability descriptions. No commentary/source edits here.
Then redirect and iterative final investigation while581Unknown remain eligible.
FI024 residual ledger/methodoutcomes remain starting points; shape copy termination
alone lacks ownership, destination tilemap budgets do not prove ROM container extent,
and unrelated indexed tables do not inherit domains. No extra supported extension
or unchanged runtime test is deferred from this connected contract.
Toolkit: existing confirmedT001 only;zero new/suspected/missing-feature findings,
no blocker. [Persistent report](toolkit-issues.md). No toolkit edits/build/substitution.

## Round025 redirect — sparse-address closure
Changed FI024 geometry justified bounded45-entry coindexed layout address-set check;
no selected source byte remains Unknown. Connected full-ID bank3 payloads already
classified. Shape ownership,distinct selector roots and source-container ends retain
FI024 barriers; no unchanged retries. No ready lead. Iterative final-investigation
method planning required next,not terminal exhaustion. [Findings](redirect/round-025/findings.md),
[handoff](round-025-redirect-handoff.md).
