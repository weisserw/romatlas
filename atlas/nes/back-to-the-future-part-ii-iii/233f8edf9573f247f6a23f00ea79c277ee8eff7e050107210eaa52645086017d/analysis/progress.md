# Disassembly progress

Round 001 disassembly in progress; coordinator awaiting this separate worker. Baseline inspected and verified byte-identical: [report](baseline-verification.json). ROM unchanged; MMC1, 131072 PRG and 131072 CHR bytes, total 262160. Startup reset vector $C000 validated with effective banks [0,1,14,15]. Bounded runtime and static discovery underway; no classification checkpoint yet. Evidence: round-001/. Baseline is fidelity only.

## Durable intermediate milestone
789 regions verified in internal store: 132917 classified bytes /262160 =50.700717%, unknown129243. Checkpoint pass-001-classified is byte-identical and has a nonempty map (SHA256 fb7f7838cf49ac8165f1d4dfab4a24f9095b9bd7801f534d3c5073f8d7807dee). Earlier pass-001-disassembly was an empty-map fidelity checkpoint due to rejected annotations; preserved and superseded. Strict RPC checks now enforced. Corrected NES input buttons-array reached intro almanac screen and1599 contextual fetch roots; next waits through intro and exercises street gameplay. Toolkit input/discovery discrepancies recorded in toolkit-issues.md.

Expanded milestone: 1688 regions, 134799 classified bytes, 127361 unknown. Street reached via bounded intro skips; active mechanics and Part III shortcut investigation ongoing. Checkpoint pass-001-expanded verified byte-identical.

## Round001 disassembly complete, ROM incomplete

ROM classification: 55.07% (144372 / 262160 bytes classified; 117788 bytes unknown).


| Physical section | Code | Data/image | Unknown |
|---|---:|---:|---:|
| header | 0 | 16 | 0 |
| PRG00 | 274 | 0 | 7918 |
| PRG01 | 0 | 32 | 8160 |
| PRG02 | 0 | 0 | 8192 |
| PRG03 | 148 | 34 | 8010 |
| PRG04 | 0 | 429 | 7763 |
| PRG05 | 223 | 0 | 7969 |
| PRG06 | 0 | 320 | 7872 |
| PRG07 | 0 | 370 | 7822 |
| PRG08 | 0 | 0 | 8192 |
| PRG09 | 249 | 246 | 7697 |
| PRG10 | 0 | 35 | 8157 |
| PRG11 | 1090 | 837 | 6265 |
| PRG12 | 1902 | 0 | 6290 |
| PRG13 | 529 | 0 | 7663 |
| PRG14 | 3648 | 0 | 4544 |
| PRG15 | 2793 | 125 | 5274 |
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

Internal store independently checked for nonoverlap, unique physical-byte count, and equality with exported current map. 5164 regions,10856 Code bytes and133516 Data/Image bytes. All regions have evidence; code has explicit CPU views. Reachability differs: ordinary startup/title/intro/street observed; PartIII artificial password-buffer scenario observed; connected static successors not claimed reached. Execution coverage is retained per-scenario in round-001/*summary.json and *contexts.json, not this classification percentage.

Final checkpoint [pass-001-final](pass-001-final-verification.json) validates, generates and rebuilds byte-identically; map SHA25632ffeb8013e4cd14cbba45d0f914858256ca73eaacee6be15d1f9fbbc726a538. Input SHA256 unchanged. Baseline and superseded checkpoints retained. Current phase: disassembly finished; coordinator should run commentary then redirect. Current-source readability is not yet completed.

Easy connected observed/static batch complete; practical street and PartIII shortcut tested. Further gameplay requires area/time/inventory transition contract recovery, not repeated basic traversal. Remaining indirect dispatch and patterned data questions are ready for redirect, not a terminal exhaustion claim. Two confirmed API/documentation discrepancies in [toolkit report](toolkit-issues.md), no blockers, no toolkit edits. No publishing/archive done.

## Round001 commentary active
Input checkpoint pass-001-final; current map SHA256 32ffeb8013e4cd14cbba45d0f914858256ca73eaacee6be15d1f9fbbc726a538. Prepared frozen commentary baseline and editable source; preparation verified byte-identical. Full current-source readability is pending (10856 Code bytes, classified data/image plus unknown boundary notes). No structure or toolkit changes authorized. Disk has 8.0 GiB free; durable analysis is 98 MiB. Commentary will review physical contiguous code runs, preserving effective bank views and expressing unresolved mechanics locally. Final editorial audit remains pending.

## Round001 commentary complete; ROM incomplete
Full current-source readability completed in commentary/source and reading-after.asm. All 5331 physical review entries (5114 Code,49 Data,1 Image,167 Unknown) are reviewed; 129 run units cover all10856 Code bytes. Consistent routine/branch/RAM/hardware symbols and purpose/algorithm explanations added; unknown spans retain explicit fidelity-only notes. Final editorial_audit remains pending. Fresh commentary verification passes byte-identically; maps/current.json,analysis/state.json and input SHA unchanged. Compact handoff: [round-001-commentary](handoffs/round-001-commentary.md); detailed semantic limits and two Static Code/table conflicts: [uncertainties](../commentary/uncertainties.md). Next phase redirect; no readability backlog. ROM remains55.07% classified,117788 bytes unknown. Existing two toolkit API/documentation discrepancies, no blockers; no new toolkit issues found during commentary. No archive/publish/toolkit/ROM changes or cleanup.

## Round001 redirect active — round-001-a
Input pass-001-final; map SHA256 32ffeb8013e4cd14cbba45d0f914858256ca73eaacee6be15d1f9fbbc726a538. Current-source commentary complete. Research only in analysis/redirect/round-001-a; map/ROM unchanged. Camera conflict has infeasible LDY #$1F/BNE fallthrough and independently clamped Y=0..31, supporting three lookup arrays. Terrain conflict, hooks, progression/password and patterned bank data under investigation. Disk free 8.0 GiB; no cleanup.

Round001-a milestone: camera and terrain apparent static fallthrough conflicts resolved by flag/field flow, hook setters and three additional callbacks recovered, exact area initializer contract recovered. Runtime trials area-* and area-init-* failed register setup (no visit claim); bounded cycle-aligned transition retry active. 27 patterned data candidates (5711 unknown bytes) triaged, consumer review pending. See redirect/round-001-a/findings.md.

Round001-a second milestone: successful artificial area1F/2F visits confirmed by current76, rendered captures and movement/jump inputs. Terrain F76B alias prerequisite resolved (read-only indirect helper, only zp scratch writes). Inventory bitset contract recovered; ending gateC32D->C462/CD7F actually fetched after E3=C0/E4=FF patch. Ending continuation currently shows DeLorean animation; terminal screen unconfirmed. Pattern generic-data review5826bytes ready, no new fetch conflict. Password41 paired strings +two41-pointer arrays independently cover1046data bytes, proposal pending consolidation. See active findings.md for exact files/errors/frontier.
Round001-a connected batch: thirteen bank2 callback callees inspected with exact4/5 contexts,289bounded CFG candidates saved; arbitrary fixed helper return-bank contracts remain explicit barriers. Password paired strings/pointer independent bounds1046bytes ready plusB144tail3. Ending actual gate and720animation frames exercised; finite12record scene list recovered, final-scene continuation via terminator-index skip active (ending-terminal.py), outcome pending.

## Round001 redirect complete — combined supported batch, ROM incomplete
Attempt round-001-a completed; input/output structural checkpoint remains pass-001-final, map/state SHA25632ffeb8013e4cd14cbba45d0f914858256ca73eaacee6be15d1f9fbbc726a538, ROM unchanged. THE END confirmed after supported artificial inventory gate and explicit finite-animation skip (redirect/round-001-a/ending-final-7.png). Area1F/2F scenes and gameplay visited. Seven ready leads in leads.json: camera-arrays, terrain-policy, frame-hooks, small-patterns, area-transition, password-structure, later-observed-code (allR001prefix). Core proposes9717unknownbytes (5826patternedData+1046passwordData+179camera/terrainData+2666observedCode), plus bounded static callback/input/initializer candidates and22Code->Data corrections; none accepted during redirect. Exact current complement176bank-split intervals117788bytes, classified144372/262160=55.07%. No new toolkit issues; two existing confirmed discrepancies nonblocking. Current-source commentary remains complete for unchanged map, final editorial audit pending. Next: independent disassembly review of coherent batch. Handoff analysis/handoffs/round-001-redirect.md; no archive/publish/cleanup.

## Round002 disassembly — active
Input pass-001-final (map SHA256 32ffeb8013e4cd14cbba45d0f914858256ca73eaacee6be15d1f9fbbc726a538). Reviewing seven R001 ready leads independently, preserving current-source commentary. Output checkpoint pending. Saved exact artificial fetch evidence reused without unchanged replay.

Round002 core accepted: 9717 unique unknown bytes (2666 artificial Observed Code;7051 Static Data),22Code→Data corrections; state/export validated and saved. Direct independent roots/decode and password extent checks in round-002/observed-review.json and apply.py. First conservative connected static batch3021additional bytes, return-bank frontier retained; expanding only explicit selector contracts, with exact decodes saved. Three stale pattern preview fields correspond to redirect documented final boundary corrections; actual final ranges independently inspected. Current commentary files preserved; reconciliation belongs next commentary phase.

## Round002 disassembly complete — ROM incomplete
Input pass-001-final; output pass-002-disassembly. ROM classification: 60.47% (158530 / 262160 bytes classified; 103630 bytes unknown).

Accepted14158new unique physical bytes:9717core(2666artificial Observed Code+7051Static Data),4441connected Static Code; corrected22Code bytes to Data. Internal map/export equal, nonoverlap and exact unique-byte counts independently checked; 8566regions. Map SHA256 40ea782f16e1af1e4512f2388dc161d07561ab70976924ec3ed75c6422d275e5. ROM SHA unchanged.

Evidence: [review](round-002/review-notes.md), [core](round-002/core-applied.json), [observed checks](round-002/observed-review.json), [static accepted batch1](round-002/static-review-accepted-batch1.json), [static final batch](round-002/static-review.json), [counts](round-002/counts.json). Historical preliminary static reviews retained but superseded by stricter indirect-store audit; final static accepted records total4441bytes. All seven R001 lead outcomes verified in leads.json; structural semantics and natural reachability kept separate.

Fresh checkpoint validates/generates/rebuilds byte-identically; verification analysis/pass-002-disassembly-verification.json. Commentary files preserved; they cover previous map and require reconciliation for new code/data next phase, final editorial audit pending. Saved broad street/PartIII/area1F/2F/ending THE END scenarios reviewed, no unchanged replay or new runtime patches. Concrete unproved return-bank and indirect selector contracts retained for redirect, not terminal exhaustion. Existing [toolkit issues](toolkit-issues.md):two nonblocking API/documentation discrepancies; no new toolkit issues found. No toolkit/ROM edits, archive, publish or cleanup.

| Bank | Code | Data/Image | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG00 | 367 | 353 | 7472 |
| PRG01 | 0 | 118 | 8074 |
| PRG02 | 0 | 440 | 7752 |
| PRG03 | 148 | 34 | 8010 |
| PRG04 | 0 | 2296 | 5896 |
| PRG05 | 2168 | 94 | 5930 |
| PRG06 | 0 | 617 | 7575 |
| PRG07 | 0 | 438 | 7754 |
| PRG08 | 0 | 84 | 8108 |
| PRG09 | 1634 | 2459 | 4099 |
| PRG10 | 0 | 619 | 7573 |
| PRG11 | 1173 | 1057 | 5962 |
| PRG12 | 2067 | 0 | 6125 |
| PRG13 | 1305 | 96 | 6791 |
| PRG14 | 5143 | 0 | 3049 |
| PRG15 | 3936 | 796 | 3460 |
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

Round002 final handoff: [round-002-disassembly](handoffs/round-002-disassembly.md). Current phase: disassembly complete; next commentary reconciliation, then redirect. Fresh verification report/map/state hashes rechecked equal. ROM classification: 60.47% (158530 / 262160 bytes classified; 103630 bytes unknown).

## Round002 commentary active
Input pass-002-disassembly SHA256 40ea782f16e1af1e4512f2388dc161d07561ab70976924ec3ed75c6422d275e5. Prior complete commentary preserved at analysis/round-002/commentary-input-round001/. Fresh baseline preparation and physical-range reconciliation underway; all changed and affected source readability pending, no completion claim.

## Round002 commentary complete
Full current-source readability complete against pass-002-disassembly (map/state SHA256 40ea782f16e1af1e4512f2388dc161d07561ab70976924ec3ed75c6422d275e5). All 8812 scaffold spans and 325 routine/structure units reviewed, covering all 17941 Code bytes; no readability ranges pending. Fresh byte-identical verify passed and reading copy refreshed. Prior commentary retained, no map/state/ROM changes, final editorial acceptance pending. Compact handoff: analysis/handoffs/round-002-commentary.md. Classification remains 60.47%, 103630 bytes unknown.

## Round002 redirect active
Attempt analysis/redirect/round-002-a, input pass-002-disassembly map/state SHA25640ea782f16e1af1e4512f2388dc161d07561ab70976924ec3ed75c6422d275e5; current-source readability complete. Investigating pointer destinations, dispatcher tables and nested PRG return contracts from saved barriers; existing attempts preserved. Map/state/source/ROM remain unchanged. 103630 unknown bytes. Disk7.8GiB free, reusable scratch32KiB.

Redirect milestone: analysis/redirect/round-002-a/findings.md records corrected CHR-only and finite input-dispatch contracts, conditional object-type domain and metatile sentinel counterexample. Active read-only batch2496unknown Code bytes; not accepted/classified. Following newly exposed direct contracts and independent residual opportunities; no exhaustion claim.

Redirect further milestone: read-only candidate batch3457Code bytes,1615starts with no self-overlap; added17terrain handlers, correct bank-zero BCD score helper F1E1->811A and bank-five D36D->A639 graph. Independent EF49 area-record222-byte copy contract yields eight overlapping data sources92627..94028; field03E8 initialization counterexample retained. See findings.md latest section; remaining direct scene spawn helpers being followed. No map/source/toolkit changes.

## Round002 redirect complete
Combined6146unknown-byte proposal (4301Code at2048starts,1845Data),accepted0; map/state40ea782f16e1af1e4512f2388dc161d07561ab70976924ec3ed75c6422d275e5 unchanged. Classification remains158530/262160(60.47%),103630unknown. Ready R002-bank-return-continuations,R002-consumer-bounded-tables,R002-area-copy-contracts inleads.json; coherent connected and independent avenues packaged with localized static preconditions. Exact unknown254bank-split interval ledger refreshed. No terminal whole-ROM exhaustion conclusion; independent disassembly review next. Existing two toolkit discrepancies remain nonblocking, no new issues. Compact handoff analysis/handoffs/round-002-redirect.md.

## Round003 disassembly active
Input pass-002-disassembly, map/state SHA25640ea782f16e1af1e4512f2388dc161d07561ab70976924ec3ed75c6422d275e5. Independent fresh primary decodes and localized static calling-contract review underway in analysis/round-003/. No new classifications yet;158530classified/103630unknown. Prior source/commentary preserved; no toolkit/ROM edits.

## Round003 disassembly complete — ROM incomplete
Input pass-002-disassembly; output pass-003-disassembly. ROM classification: 62.82% (164676 / 262160 bytes classified; 97484 bytes unknown).

Accepted6146new unique physical bytes:4301Static Code at2048starts,1845Static Data. ThreeR002ready leads verified; full fresh candidate/context decodes and independent physical data/consumer/wrapper audits in [review](round-003/review-notes.md), [fresh decodes](round-003/fresh-candidate-decodes.json), [primary](round-003/primary-audit.json), [supplement](round-003/supplemental-primary.json), [accepted](round-003/accepted.json), [counts](round-003/counts.json). Generic Data used after bounded role inspection where full schema/domain unproved. Static calling contracts separate from natural reachability. Internal map/export equal and validated; exact unknown complement [here](round-003/unknown-complement.json). Map SHA256291758555315a83340815774c6461b664d55eabcb52f36c9fb692deda6b80d72; ROM hash unchanged.

Fresh checkpoint validates/generates/rebuilds byte-identically:analysis/pass-003-disassembly-verification.json. Existing commentary/source/history preserved; next commentary must reconcile full current source and correct bank0F1E1->811A six-digit score-adder semantics. Broad saved progression/ending milestones reviewed, no unchanged replay. Remaining unrestricted selector/map-coordinate invariants do not invalidate accepted conditional slices. Residual compressed maps/scripts/audio/resource extents need concrete structural research; ordinary easy-gain handoff, no terminal whole-ROM exhaustion claim. Existing [toolkit issues](toolkit-issues.md)NES-001/NES-002 nonblocking;no new toolkit issues found. No ROM/toolkit edits,archive,publish or cleanup.

| Bank | Code | Data/Image | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG00 | 436 | 353 | 7403 |
| PRG01 | 0 | 118 | 8074 |
| PRG02 | 0 | 440 | 7752 |
| PRG03 | 148 | 34 | 8010 |
| PRG04 | 0 | 2296 | 5896 |
| PRG05 | 3017 | 94 | 5081 |
| PRG06 | 0 | 617 | 7575 |
| PRG07 | 0 | 1134 | 7058 |
| PRG08 | 0 | 84 | 8108 |
| PRG09 | 2148 | 2459 | 3585 |
| PRG10 | 0 | 619 | 7573 |
| PRG11 | 1231 | 1838 | 5123 |
| PRG12 | 2084 | 0 | 6108 |
| PRG13 | 3519 | 304 | 4369 |
| PRG14 | 5143 | 0 | 3049 |
| PRG15 | 4516 | 956 | 2720 |
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

Round003 commentary active: preserved full round002 commentary, prepared verified current baseline and carried 8566 exact physical regions. New classified source reconciliation/readability pending; remain in commentary.

Round003 commentary complete: all current 10862 scaffold entries and 453 units reviewed; all 22242 Code bytes covered, 106 Data spans/one Image/225 Unknown boundaries explained, no deferred readability range. Prior8566 physical edits and review credit retained, with no changed prior execution views. Added29 new-code run explanations/76 internal entries, current Data/Unknown notes and consistent symbols; corrected F1E1 bank0 BCD score call and affected wrapper/dispatch/audio/spawn descriptions. Static object/terrain/coordinate/RAM-map/serializer preconditions remain local, not natural domain claims. Fresh verify byte-identical; reading-after and source locations refreshed. ROM/current/frozen map/state unchanged; final editorial_audit pending. Handoff: [round-003-commentary](handoffs/round-003-commentary.md). Next phase belongs coordinator; no archive/publish/cleanup/toolkit edits or replay. Classification remains164676/262160(62.82%),97484unknown.


## Round003 redirect active
Attempt analysis/redirect/round-003-a; input pass-003-disassembly SHA256291758555315a83340815774c6461b664d55eabcb52f36c9fb692deda6b80d72. Current-source readability complete. Researching consumer-bounded resource pointer/descriptors, packed upload extents and residual script/audio continuations; preserving prior attempts, map/source/state/ROM unchanged. Classified164676/262160,unknown97484. Disk7.7GiB free,scratch32KiB.

Round003 redirect milestone: resource/descriptor/palette/spawn/script graph now proposes10215Data+1627Code unknownbytes (791starts),accepted0.49scriptcommands,10subcommands,1573bounded script tokens and138concrete predicate references investigated. Exact read-only artifacts/findings in redirect/round-003-a; prior artifacts preserved, no Code/Data overlap so far. Continue F76B/DE45 return contracts and independent audio/map extents before combined handoff.

Round003 redirect further milestone: independent audiograph376command/1214eventtokens/28instruments and58area mapheaders/nested8cell/17bytegraphics contracts now yield53230proposedunknownData,plus2421Codependingfinalconsolidation. All mapgraphpointersvalid/noexistingCodeconflicts; map/state/ROM/sourceunchanged. Explicitlocaldomains retained; final overlap andnewlyexposed continuations underreview.

Round003 redirect resource milestone:58spawn+58proximityplacementstreams/2988records resolved viaEF49fieldwriters andED12/C399bankedconsumers. Sprite505payloads,scene/textandmetatile/boxdata plusboundedgenericlowerbank2recordreview now74609proposedData unknownbytes;Code2421pendingfinalwrappertriage. Accepted0; noCodeconflicts. Latestfindings.md hasphysicalclaims/methodsandfrontier, finalpackagingactive.

Round003 redirect finalresearch milestone: correctedfixed-onlycontinuationwith[null,null,14,15]/normalreturncontract andnew78initializer/sixeffecthandlers; final79577proposedunknownbytes=74798Data+4779Code/2210starts. Accepted0; alloverlapauditsclear;17907residualafterhypotheticalacceptance. Currentmapclassificationremains164676/262160=62.82%,97484unknown. Findings/primary/unknownledgerupdated; finallead/handoffpackagingactive,notterminalexhaustion.

## Round003 redirect complete — ready combined batch
Two ready leads in leads.json: R003-consumer-resource-data and R003-command-predicate-continuations. Proposed 79577 unique unknown bytes:74798 Static Data and4779 Static Code at2210 starts; accepted0. All self/Code-Data/existing overlaps clear. Current map/state remain pass-003-disassembly SHA256291758555315a83340815774c6461b664d55eabcb52f36c9fb692deda6b80d72,164676classified/97484unknown(62.82%). Exact complement/primary claims/local preconditions/frontier retained in redirect/round-003-a. If all accepted17907unknownremain. No terminal exhaustion claim. Independent disassembly review next, then full commentary/redirect. Saved broad gameplay/ending evidence reused. No new toolkit issue, NES-001/NES-002 nonblocking. No ROM/map/source/toolkit/archive/publish/cleanup mutation. Compact handoff: handoffs/round-003-redirect.md.


## Round004 disassembly active
Input pass-003-disassembly; map/state SHA256291758555315a83340815774c6461b664d55eabcb52f36c9fb692deda6b80d72. Independently reviewing R003 consumer/resource Data and command/predicate/fixed continuation Code proposals. No accepted bytes yet; 164676classified/97484unknown. Preserve source, state/history and broad saved progression/ending evidence; no unchanged gameplay replay.

Round004 review milestone: fresh2210Codeviews match4779bytes; independent11196claim physical/context/unknown audits confirm74798Data, zeroexisting/self/Codeoverlap. Reparsed20PPUdescriptors/19sources each1024output,58maps/801macros/1426graphicobjects,116placements/2988records,505spriteframes,17texts,376audio commands/1214events. Consumer/control/local-precondition review inround-004; acceptance pending.


## Round004 disassembly complete — ROM incomplete
Input pass-003-disassembly; output pass-004-disassembly. ROM classification: 93.17% (244253 / 262160 bytes classified; 17907 bytes unknown).

Accepted79577new unique physical bytes:74798Static Data in751bank-split runs and4779Static Code at2210starts. BothR003ready leads verified, prior histories preserved. Complete independent fresh views, primary consumer/wrapper/context/physical audits and exact bound reparses in [review](round-004/review-notes.md), [Code decodes](round-004/fresh-candidate-decodes.json), [primary](round-004/primary-audit.json), [supplement](round-004/supplemental-primary.json), [Data audit](round-004/data-audit.json), [physical audit](round-004/physical-audit.json), [accepted](round-004/accepted.json). Generic fallback follows practical role/reference/neighbor/pattern review and asserts noncode, not a full original schema. Normal-return/fixed-only views and local selector/predicate/object/queue contracts remain separate from natural reachability.

Internal map equals current export; validation passed. Counts independently use unique classified physical bytes, ignoring raw export fallback:Code27021,Data86160,Image131072. Exact17907unknown bytes in793bank-split intervals with structure-group/frontier dispositions in [ledger](round-004/unknown-complement.json), [counts](round-004/counts.json). Map/state/checkpoint SHA25662ceac56bdceca6f4f957fcf6a979a2fa18333bbb96d94a8ce9538a7d30a451f. Fresh checkpoint validation/generation/rebuild byte-identical: [verification](pass-004-disassembly-verification.json). ROM hash unchanged; current commentary source SHA256adfe90d4e9d9d200af346b71ae6af854a4a2b8bdc2560f95de26081326d9bfb2 unchanged/preserved. Full new-current-source commentary remains next phase.

Saved street/PartIII/area1F/2F/THE END observed capture/fetch routes reviewed, no unchanged replay. Supported connected command/subcommand/predicate, initializer/effect, map/sprite/audio/script/placement/resource families fully reviewed; no original lead-range ceiling. Remaining mixed code/data, lower-window return contexts and statefulscriptreturn/loop extents require specific local bank/state/boundary evidence in next redirect, not another unchanged broad gameplay run. This is an ordinary finite-batch handoff, not whole-ROM exhaustion. Toolkit report [NES-001/NES-002](toolkit-issues.md) preserved, both nonblocking;no new toolkit issues found. No toolkit/ROM/current-commentary-source edits,archive,publish orcleanup. Disk7.4GiBfree, no scratch batch generated. Compact next-phase handoff: [round-004-disassembly](handoffs/round-004-disassembly.md).

| Bank | Code | Data/Image | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG00 | 646 | 5937 | 1609 |
| PRG01 | 0 | 6474 | 1718 |
| PRG02 | 0 | 7724 | 468 |
| PRG03 | 148 | 7575 | 469 |
| PRG04 | 0 | 8192 | 0 |
| PRG05 | 3017 | 94 | 5081 |
| PRG06 | 0 | 7914 | 278 |
| PRG07 | 0 | 7867 | 325 |
| PRG08 | 0 | 7540 | 652 |
| PRG09 | 2611 | 4432 | 1149 |
| PRG10 | 0 | 7584 | 608 |
| PRG11 | 1231 | 6244 | 717 |
| PRG12 | 3479 | 2070 | 2643 |
| PRG13 | 3828 | 3997 | 367 |
| PRG14 | 6846 | 154 | 1192 |
| PRG15 | 5215 | 2346 | 631 |
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

Round004 commentary active: input pass-004-disassembly, map/state 62ceac56bdceca6f4f957fcf6a979a2fa18333bbb96d94a8ce9538a7d30a451f. Full prior commentary preserved in analysis/round-004/commentary-input-round003 before fresh baseline preparation. Entire current-source readability reconciliation pending; no final editorial audit.

Round004 commentary complete: full 14391-entry current source reconciled, 1410 routine/structure units, no pending readability ranges. Fresh byte-identical verification and reading refresh passed; ROM/map/state unchanged. Full prior commentary preserved. Final editorial audit pending; ROM remains incomplete with17907unknown. See analysis/handoffs/round-004-commentary.md and analysis/round-004/commentary-coverage.json. Existing NES-001/NES-002 nonblocking; no new toolkit issues found.

Round004 redirect active: exact input pass-004-disassembly/map-state62ceac56bdceca6f4f957fcf6a979a2fa18333bbb96d94a8ce9538a7d30a451f. Unique evidence analysis/redirect/round-004-a. Generalized accepted immediate-selector proof derives67bank-establishing roots, reopening specific lower aliases missed by prior caller allowlist; bounded traversal proposes548Codebytes pending review. Residual generic non-code and stateful script investigations active; findings.md records exact frontier. No map/source/ROM changes, no terminal assessment.
Round004 redirect milestone: current candidateCode2725bytes, scene finite dispatch/explicit return contracts localized; complete graphics/macro paired-table inference now derives exact whole-table bounds from independent origins plus full stride consistency and payload endpoint. See round-004-a/findings.md and whole-table-review.json; investigating remaining connected motion/script structure. No terminal stop.
Round004 redirect script milestone: 2437tokens/8930symbolicstates, no budget frontier,2213previousunknownscriptbytes before consolidation. Exact call/return/loop cursor state replaces missing-object-state barrier;8F corrected to shared8556conditional-transfer semantics;9A/9D stop at820Efamily mutation. Predicate preservation checked locally. New script operands reopen concrete pair6callee entries; no code/script overlap. Current graph expansion enters summarized directcallee bodies; finite scene/motion and pause/terrain tables remain active.
Round004 redirect callback/noncode milestone: accepted encoded hookD724/26/28 establishesD72Bcallback, connected finite menu/display graph and tables inspected. final-data-evidence.json proposes9776Data bytes withzeroCode/Imageconflicts; includes whole-table finiteconstraints and bounded genericpatternfallback. Localpair2motion/menu/secondaryactionnormalreturn contracts underreview; final candidate/residual consolidation remains. AcceptedROM/map/source unchanged.
Round004 redirect player-action milestone:20paired87C0..87E8targets independently bounded byfirstpointedRTS, allhandlers followed underlocal0..19contract. CurrentCode4440bytes,Data9870bytes plus symbolic script2213beforeunion. Graph10189nodes nobudgetstop. Correctedthreegraphicsmatchinghighprefixes to avoid originalfulltableextent claim. Nextbounded candidateBA38debug/controller-two graph andBBEB8bytedescriptors; remainingphysicalbankaliases retainexplicitnullcontext.
Round004 exact symbolic-count validation finished101908states withno frontier;2437tokenaddresses andpredicate targetset equal supersededcoarse result; dependent graph rebuilt fromcorrectedresult. Consolidatedcheckpoint17496proposeduniqueunknownbytes (5270Code,12226Data incl2213script),411provisional residual bytes/31intervals. Last finite secondary-action/debugreturn contracts and clearlookupcells underfinalvalidation; no terminalexhaustion oracceptedclassificationchange.

Round004 redirect COMPLETE READY: unchangedpass004/mapstate62ceac56bdceca6f4f957fcf6a979a2fa18333bbb96d94a8ce9538a7d30a451f. Combined17740uniqueunknownbytesproposed (5443StaticCode/12297StaticData incl2213script), accepted0. Exactscriptgraph101908states/2437tokens,nobudgetfrontier; code2575instructions/11604nodes,nobudgetfrontier. ZeroCode/Data/scriptconflicts or internalinstructionoverlap. Provisionalremaining167bytes/21intervals; actualacceptedunknown17907/793unchanged. Three readyR004leads appended withrecipes andpreservedhistory. Exactledger/preconditions/frontierreview andcompactanalysis/handoffs/round-004-redirect.md saved. This isproductivecombinedbatch, notterminalexhaustion. Independentdisassemblythenfullcurrent-sourcecommentary required. ROM/map/state/source/binary/toolkitunchanged; noarchive/publish/cleanup. ExistingNES001/NES002nonblocking,no newtoolkitissues. Disk7.3GiBfree,attempt19MiB.


Round005 disassembly active: input pass-004-disassembly/map-state62ceac56bdceca6f4f957fcf6a979a2fa18333bbb96d94a8ce9538a7d30a451f. Independent fresh exact Code views, every physical Data claim, whole finite table constraints and corrected exact symbolic script contracts under review. Prior source/history preserved; saved broad gameplay/ending routes reused. No accepted bytes yet.

Round005 review milestone: all2575freshCodeviews/5443bytes and5164physicalData/scriptclaims/12297bytes independently audited;22wholefamilies reparsed,threehighprefixlimits preserved. Independent exact101908state/2437token traversal matches, nofrontier. Primarylocalbank/return/dispatchcontracts reviewed; review-notes.md records scope, genericfallback andsavedgameplayreuse. Applying complete supported batch next.


## Round005 disassembly complete — ROM incomplete
Input pass-004-disassembly; output pass-005-disassembly. ROM classification: 99.94% (261993 / 262160 bytes classified; 167 bytes unknown).

Accepted17740new unique physical bytes:5443StaticCode at2575starts/views and12297StaticData (10084resource/generic +2213script) in738bank-split runs. All three R004 ready leads verified; earlier outcomes/history preserved. Independent exact freshCode/physicalData/primarycontract audits and own complete101908state/2437token traversal,22wholefamilies withthreeprefix-onlyhigharrays: [review](round-005/review-notes.md), [freshCode](round-005/fresh-candidate-decodes.json), [primary](round-005/primary-audit.json), [supplement](round-005/supplemental-primary.json), [physical](round-005/physical-audit.json), [structures](round-005/structure-audit.json), [accepted](round-005/accepted.json). Exact local finite bank/selector/stack/RAM-grid/normalreturn contracts retained; naturalreachabilityunclaimed. GenericData follows bounded role/reference/neighbor/pattern review, no originalgenerator/schema asserted.

Internalmap equals currentexport;mapvalidationpassed. Independent unique classified totals:Code32464,Data98457,Image131072. Exact167unknownbytes/21bank-splitintervals in [ledger](round-005/unknown-complement.json), [counts](round-005/counts.json). Currentmap/state/checkpoint SHA256 504f8246c54888d097dec9bc54bce35a53575b00f2518c8d443e76d339adc34f. Freshcheckpoint validation/generation/rebuild byteidentical: [verification](pass-005-disassembly-verification.json). ROM hash unchanged. Full existing commentarysource/artifacts retained byte-for-byte, [preservation](round-005/commentary-preservation.json); resulting-source reconciliation/readability pending next commentary phase. No finaleditorialaudit claim.

Saved street/PartIII/areas1F/2F/THE END actual fetch/capture/routes reviewed; no changed discriminating question warrants unchangedreplay. Supported connected batch accepted together. Remaining short code/stack/return, record/script fragments retain material local structural/extent/bank uncertainty; exact redirectfrontier explanations remain [frontier](redirect/round-004-a/frontier-review.json), nowcorroboratedbyactualledger. Full new-current-sourcecommentary should inspect newlyacceptedpredecessors/consumers before redirect. This is ordinaryproductivebatchhandoff, not terminalexhaustion; no arbitraryquotaor speculativecompletion. [Toolkitissues](toolkit-issues.md) NES-001/NES-002 remainnonblocking;no new toolkit issues found. Configuredreleasebinary/toolkit/ROM unchanged. Noarchive,publish,cleanup. Compacthandoff: [round005](handoffs/round-005-disassembly.md).

| Bank | Code | Data/Image | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG00 | 852 | 7340 | 0 |
| PRG01 | 0 | 8192 | 0 |
| PRG02 | 0 | 8192 | 0 |
| PRG03 | 148 | 8044 | 0 |
| PRG04 | 0 | 8192 | 0 |
| PRG05 | 6487 | 1705 | 0 |
| PRG06 | 0 | 8192 | 0 |
| PRG07 | 0 | 8192 | 0 |
| PRG08 | 0 | 8192 | 0 |
| PRG09 | 2753 | 5403 | 36 |
| PRG10 | 0 | 8192 | 0 |
| PRG11 | 1231 | 6961 | 0 |
| PRG12 | 3922 | 4191 | 79 |
| PRG13 | 3855 | 4314 | 23 |
| PRG14 | 7658 | 511 | 23 |
| PRG15 | 5558 | 2628 | 6 |
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

Round005 handoff diskcheck:7.2GiBfree,analysis/scratch32KiB,round0054.1MiB;no cleanup. All26commentaryartifacts match preservation hashes after checkpoint.

Round005 commentary current-source readability complete. All 16,932 scaffold entries and 2,304 meaningful units reviewed; every classified code/data region and all 21 Unknown boundaries explained, no pending readability ranges. Fresh byte-identical verification and reading-copy refresh passed. Preserved all 26 prior artifacts and 13,598 exact physical block reviews; map/state SHA remains 504f8246c54888d097dec9bc54bce35a53575b00f2518c8d443e76d339adc34f. ROM classification remains incomplete: 261,993/262,160 (99.94%), 167 unknown. Final editorial_audit pending. See handoffs/round-005-commentary.md and round-005/commentary-{coverage,final-checks}.json. No archive/publish/cleanup/toolkit/ROM edits.

### Round005 redirect active — pass-005-disassembly

Worker attempt analysis/redirect/round-005-a; frozen input map/state SHA256 504f8246c54888d097dec9bc54bce35a53575b00f2518c8d443e76d339adc34f. Readability gate complete. Exact residual 167 bytes/21 intervals unchanged. New reasoning checks independent fixed-bank copy/list consumers and leaf/helper stack/bank contracts against primary source; no runtime replay or map/source edit. Durable findings and exact active frontier: analysis/redirect/round-005-a/findings.md and phase.json. New proposals remain unaccepted pending independent disassembly.

Round005 redirect complete as productive combined batch, not terminal exhaustion. Attempt round-005-a proposes160Staticbytes (111Code+49Data), accepted0; existing exact167unknown/21intervals unchanged. Reopened localized leaf/stack/explicit pair6 return contracts and independent C779 nine-byte copy/B99D25-byte list footprints; repeated script/pointer/payload patterns reviewed independently of natural selectors. Fresh decode corrected BD76 continuation to BNE BD83. All21residuals dispositioned; projected7bytes/fourintervals if fully accepted. ReadyR005-local-leaf-stack-code andR005-independent-copy-list-pattern-data in leads.json. Handoff analysis/handoffs/round-005-redirect.md; proposal/primary/freshdecode/physical/findings/frontier artifacts in analysis/redirect/round-005-a. Hash checks confirm ROM/map/state/source unchanged. No runtime replay; broad saved gameplay retained. No new toolkit issues; NES-001/NES-002 nonblocking. No archive/publish/cleanup/toolkitchanges. Independent disassembly review required.

Round006 disassembly active: input pass-005-disassembly; independently reviewing R005 local helper/stack/bank and bounded data proposals. Saved broad gameplay ending evidence reused; no unchanged replay.

Round006 audit milestone: all17 physical claims and58 exact new instructions plus14 helper windows independently checked;111Code49Data supported. Exact bank/stack/consumer/pattern contracts in round-006/review-notes.md. Applying whole160-byte batch; four residual intervals remain ambiguous.

## Round006 disassembly complete — ROM incomplete

ROM classification: 100.00% (262153 / 262160 bytes classified; 7 bytes unknown).

Rounded100.00% does not establish completion. Accepted160new unique physical bytes in17Static intervals:111Code49Data. Both R005 ready leads verified, prior history retained. Independent all17 immutable byte/bank/boundary checks,58 exact new instruction decodes,14 fresh helper windows and primary local stack/leaf/explicit bank-return/finite-consumer/repeated-pattern contracts: [review](round-006/review-notes.md), [claims](round-006/fresh-claim-audit.json), [helpers](round-006/fresh-helper-decodes.json), [accepted](round-006/accepted.json). Correct BD76 BNEBD83 sharedLDY1/upload16 retained. GenericData follows bounded role/reference/neighbor/pattern inspection; no original schema/selector or new natural/observed reachability claim.

Internal state equals currentexport. Independent unique totals: Code32575,Data98506,Image131072; [counts/per-bank](round-006/counts.json). Exact residual7bytes/fourintervals:78036..78037 RTS(PRG09),105403..10540580FF(PRG12),118249..118252JMPCCCB(PRG14),130858..130859RTS(PRG15), [complement](round-006/unknown-complement.json). Accepted batch supplies no fallthrough or separate entry/stack/pattern envelope for these; legal decoding alone insufficient. Concrete remaining discriminator is separate entry/control or non-code-envelope evidence; preserve unknown. Productive pass handoff, not terminal exhaustion; commentary should reconcile17new regions before redirect.

Validation passed; fresh pass-006-disassembly source generation/rebuild byte-identical: [verification](pass-006-disassembly-verification.json). Map/state/checkpoint SHA2513dedafed5398b7ab7d62b63f7ca347d721c6958d1f2137be98aa35c6f8084. ROM hash unchanged. Prior full commentary and pass005 source retained byte-for-byte: [preservation](round-006/preservation-verified.json). New-current-source readability and final editorial audit remain separate pending phases.

Saved broad street/PartIII/areas1F/2F/THE END routes/captures/fetches reused; no changed discriminating gameplay experiment warrants unchanged replay. [Toolkit issues](toolkit-issues.md) NES-001/NES-002 remain nonblocking; no new toolkit issues found. Configuredreleasebinary/toolkit unchanged. No archive,publish,cleanup or source edits. Disk7.1GiBfree, scratch32KiB; no bulk evidence. Handoff [round006](handoffs/round-006-disassembly.md).

| Bank | Code | Data/Image | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG00 | 852 | 7340 | 0 |
| PRG01 | 0 | 8192 | 0 |
| PRG02 | 0 | 8192 | 0 |
| PRG03 | 148 | 8044 | 0 |
| PRG04 | 0 | 8192 | 0 |
| PRG05 | 6487 | 1705 | 0 |
| PRG06 | 0 | 8192 | 0 |
| PRG07 | 0 | 8192 | 0 |
| PRG08 | 0 | 8192 | 0 |
| PRG09 | 2762 | 5429 | 1 |
| PRG10 | 0 | 8192 | 0 |
| PRG11 | 1231 | 6961 | 0 |
| PRG12 | 3999 | 4191 | 2 |
| PRG13 | 3864 | 4328 | 0 |
| PRG14 | 7669 | 520 | 3 |
| PRG15 | 5563 | 2628 | 1 |
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

Round006 handoff disk check:7.0GiB free, analysis/scratch32KiB. All32 prior commentary/pass005 generated files preserved.

## Round006 commentary complete — ROM incomplete

Full current-source readability complete for pass-006-disassembly (map/state SHA256 2513dedafed5398b7ab7d62b63f7ca347d721c6958d1f2137be98aa35c6f8084). Preserved full prior commentary before prepare in round-006/commentary-input-round005. Reused16,911 exact physical classified reviews with unchanged views; reconciled all17newregions (111Code49Data), shared upload/stack/allocator dependencies and four Unknown boundaries. Full16,932-region/2,321-unit review has no pending readability ranges, missing code coverage or missing data prose; refreshed source locations: round-006/commentary-coverage.json. Final editorial audit remains pending.

Fresh commentary/verification.json byte-identical; reading-after.asm refreshed. Map/state/ROM unchanged. Classification262153/262160 (rounded100.00%) still incomplete with7unknownbytes/fourintervals; no natural reachability/global selector claims. Source explains local contracts and unresolved generic data meanings. Compact handoff handoffs/round-006-commentary.md. Existing toolkit issues NES-001/NES-002 nonblocking; no new toolkit issues found. No runtime replay, archive, publish, cleanup or toolkit changes.

## Round006 redirect active — narrowed script-prefix inference

Input pass-006-disassembly, map/state2513dedafed5398b7ab7d62b63f7ca347d721c6958d1f2137be98aa35c6f8084, current-source readability complete. New research analysis/redirect/round-006-a; primary-source.txt, consumer-primary.txt and physical-reference-check.json preserve inspected current evidence. Independent complement remains7bytes/fourintervals; no map/state/source/ROM changes.

Materially changed inference for105403..105405: earlier token-origin requirement is stronger than genericData requires. 80FF precedes accepted27-byte script;13other accepted80FF89 prefixes, one exact80FF8908, plus independently encoded command80/89 widths and RAM-only frame/yield semantics support generic non-code. Entire29-byte local sequence fits descriptor widths and ends at9BC8 independent code. Proposal2bytes StaticData pending independent review; no natural entry/original-schema claim.

Three other fragments: accepted predecessorJMP never falls through; no contiguous direct address/encodedRTS-return evidence selects the exact physical fragments. B0C4 word at110809 belongs different bank script; FF19 word at36848 lies in encodedData and is not demonstrated return dispatch. Negative scope excludes those evidence routes only, not all indirect callers. RTS bytes lack discriminating structure beyond opcode+ordinary-stack tautology; CDD9 JMP has valid ResetSceneState target but no independent thunk selection/extent evidence. Compact combined-constraint rejectionreview and persistent lead packaging in progress. No terminal whole-ROM exhaustion claim; no new runtime replay or toolkit issue.

## Round006 redirect complete — ready2-byte Data proposal, ROM incomplete

Completed narrowed generic script-prefix review and primary rejectionreview of remaining RTS/JMP fragments. Ready R006-generic-script-prefix proposes105403..10540580FF Static genericData, accepted0. Exact complement7bytes/fourintervals unchanged; if accepted5bytes/threeintervals remain. Proposal, repetition/descriptor catalog, primary source, physical reference scope and combined-constraint review preserved in analysis/redirect/round-006-a. No actionable independent avenue deferred; no terminal whole-ROM decision. Coordinator must inspect rejectionreview before later terminal assessment. Handoff analysis/handoffs/round-006-redirect.md; map/state/ROM/source unchanged, no new toolkit issues/runtime replay/cleanup.

## Round007 disassembly active

Independent bounded review of R006-generic-script-prefix against physical bytes, accepted neighboring script, descriptor widths and RAM-only handlers. Input pass-006-disassembly; existing progression and ending evidence reused unchanged. No competing map writers. Evidence in analysis/round-007.

## Round007 disassembly complete — incomplete ROM

ROM classification: 100.00% (262155 / 262160 bytes classified; 5 bytes unknown).

Accepted R006-generic-script-prefix:2bytes Static genericData [105403,105405). Independent physical/repeated-pattern/descriptor/RAM-handler/full29-byte bounded representation and competing-code review: analysis/round-007/review-notes.md; fresh-audit.json. Lead verified; original stream origin, global schema and natural reachability unclaimed. Complete connected supported batch already classified downstream. Saved broad progression/ending evidence reused; no unchanged gameplay replay. Exact residuals:78036..78037 RTS,118249..118252 JMPCCCB,130858..130859 RTS; rationale preserved in redirect/round-006-a/rejection-review.md for coordinator inspection.

map.validate ok; state/current equal; new maps/pass-007-disassembly.json SHA256 56271f635aa2e50716e6ebb1c1b3f33ae4a0e9eca4233b9b446dd56984493ea4; generated/pass-007-disassembly verifies byte_identical true. ROM SHA unchanged. Existing commentary/source and history preserved untouched; updated classification needs next commentary reconciliation. No new toolkit issues; existing [NES-001/NES-002](toolkit-issues.md) nonblocking. No toolkit edits, archive, publish or cleanup. Independent unique-byte/per-bank counts in round-007/counts.json; exact complement in round-007/unknown-complement.json.

| Bank | Classified | Unknown |
|---|---:|---:|
| Header | 16 | 0 |
| PRG00 | 8192 | 0 |
| PRG01 | 8192 | 0 |
| PRG02 | 8192 | 0 |
| PRG03 | 8192 | 0 |
| PRG04 | 8192 | 0 |
| PRG05 | 8192 | 0 |
| PRG06 | 8192 | 0 |
| PRG07 | 8192 | 0 |
| PRG08 | 8192 | 0 |
| PRG09 | 8191 | 1 |
| PRG10 | 8192 | 0 |
| PRG11 | 8192 | 0 |
| PRG12 | 8192 | 0 |
| PRG13 | 8192 | 0 |
| PRG14 | 8189 | 3 |
| PRG15 | 8191 | 1 |
| CHR00 | 8192 | 0 |
| CHR01 | 8192 | 0 |
| CHR02 | 8192 | 0 |
| CHR03 | 8192 | 0 |
| CHR04 | 8192 | 0 |
| CHR05 | 8192 | 0 |
| CHR06 | 8192 | 0 |
| CHR07 | 8192 | 0 |
| CHR08 | 8192 | 0 |
| CHR09 | 8192 | 0 |
| CHR10 | 8192 | 0 |
| CHR11 | 8192 | 0 |
| CHR12 | 8192 | 0 |
| CHR13 | 8192 | 0 |
| CHR14 | 8192 | 0 |
| CHR15 | 8192 | 0 |

## Round007 redirect — terminal-incomplete recommendation

Started from completed current-source commentary handoff and pass-007-disassembly SHA25656271f635aa2e50716e6ebb1c1b3f33ae4a0e9eca4233b9b446dd56984493ea4. Reused substantive round006 primary rejection review after inspecting actual current predecessor/neighbor code, primary consumers/references and the newly accepted80FF non-code rationale. That local pattern/consumer inference and definition-label removal change no structural premise for isolated B0C4/CDD9/FF1A. Fresh internal complement confirms32575Code/98508Data/131072Image,262155classified,5unknown in3intervals. No proposal, new runtime replay or ready lead; all18 existing leads verified. No useful corrected inference or discriminating method survives; coordinator must inspect audit before terminal decision. Current-source readability16932regions/2322units and fresh byte-identical verification remain complete; final editorial audit pending. Findings/reasons: analysis/redirect/round-007-a/rejection-review.md; compact handoff: analysis/handoffs/round-007-redirect.md. No new toolkit issues (existing NES-001/NES-002 nonblocking), map/source/ROM/toolkit unchanged;~6.75GiB free and no new scratch.

## Coordinator terminal acceptance

All seven sequential phase rounds concluded; current checkpoint pass-007-disassembly. ROM classification: 100.00% (262155 / 262160 bytes classified; 5 bytes unknown). Rounded percentage is not whole-ROM completion. Coordinator accepted the round007 terminal-incomplete recommendation after substantive rejection/primary-evidence inspection; no ready lead or useful corrected method remains. Final official commentary audit passed after metadata path/coverage repair:16932 regions,22431 nested associations/2322 distinct units,editorial_audit=true,byte_identical=true. Store/map validation passed, source hashes/reading/frozen current checkpoint/ROM identity independently matched. Final handoff: terminal-handoff.md; requirement audit: completion-audit.md; machine acceptance: terminal-acceptance.json. archive-options.json and archive-document-selection.json account for Markdown under the inspected toolkit contract. No toolkit modification, archive execution, publication, cleanup or deletion performed. Orchestration complete; ROM classification incomplete.
