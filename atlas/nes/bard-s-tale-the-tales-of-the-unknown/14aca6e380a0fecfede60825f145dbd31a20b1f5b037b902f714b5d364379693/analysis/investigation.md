# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## Round 01 disassembly, active record

Baseline: inspect matched workspace identity, mapper1/128KiB PRG/CHR RAM, 131088 physical bytes. Raw baseline rebuilt byte-identically; not counted classified. Initial startup tracing timed out at142frames with accepted partial execution, then durable natural startup snapshot exported. Later traces exclude PRG-RAM backing from ROM classification; no upload origin inferred.

Observed ROM batches: title, Start-to-Guild, menu/character creation, party formation and Leave Guild. Sources are numbered contexts/observed-decoded files and request/response logs under round01. NES input must use `buttons` array; prior generic button/pressed requests were ignored and are idle routes. Snapshots export input state but an executed release interval is needed before a new edge. Cursor03D9=5 shortcut entered Create after source/menu diff validation. Name-grid end coordinate03D9=5,03DA=8 follows source's own full-name placement. One Warrior named AA was created/added; postpatch lineage is artificial even though subsequent inputs are ordinary.

Static batches: vector/reset supported entry, NMI flag dispatch, audio-record alternatives; connected menu input/rotation/wrapped-coordinate/character-state branches; JSR callee destinations under pre-call effective PRG context (JSR itself cannot change mapping). Calls/mapper-affecting writes conservatively stop propagation. Candidate instructions individually decoded and inspected; no inferred branches labeled Observed. Evidence: reset-decode.json, static-inspected.json, static2-inspected.json, callee-inspected.json and callee-roots.json.

Data batches: iNES header/vector words; seven82-byte bank descriptor prefixes with referenced absolute slots and repeated noninstruction layout; bounded96-byte compact area-property arrays; mixed pointer/message block DA46..FFAD and89-byte zero suffix FFAD..10006 (dialogue-review.md). Last10 reset/vector-like bank-tail bytes remain unknown because execution alias contract not established. Generic data used for mixed streams without inventing per-string schema.

Progression: Guild/menu/character systems visited. Leave Guild produced a valid Skara Brae viewport/minimap (leave-guild.png), unlike early area1/15 directPC C108 probes, which acknowledged area selection/text but retained malformed Guild tile layout. Area1 Up continuation did not repair it. Those failures falsify only the incomplete transition recipe, not code/data structure or gameplay reachability. Source Leave Guild handler includes EC93 PPU disable before stack reset/C108; corrected transition1 probe tests this concrete missing precondition. Later visits/ending remain conditional on results; no arbitrary RAM brute force.

No toolkit changes. No toolkit issues found so far; expected zero-capacity trace detail drops are not aggregate loss. Unresolved graphics consumers and RAM-upload provenance are redirect questions, not grounds to classify by pointer values alone.

## Round01 pass outcome

ROM classification: 21.57% (28276 / 131088 bytes classified; 102812 bytes unknown).

Code 17920; noncode 10356. Per-bank counts and exact complement in progress.md and round01/counts.json. Fresh byte-identical checkpoint: analysis/round01-disassembly-verification.json. Final connected static894bytes reviewed; no unsupported bytes swept by raw fallback. Last10byte pair-tail unknown decision corrected: seven dual vector/code reset bridges70bytes independently supported (SEI, mapper reset write, three INC zp,X vector words, fallthrough C000), reachability unresolved.

Combat completed Attack target selection; enemy Arc Fire narration and AA HUD/action observed, no combat victory/defeat claimed. Last bounded narration continuation90frames yielded42new code bytes, so stopped repetitive text/menu cycling. Corrected high-area15 and coordinate approach did not reach boss/ending. Castle6 operation interrupted externally; preserve attempt rather than call it a negative result. Source event records near C816/C877/C8A8 and upload/graphics consumers are concrete next redirect questions. No terminal rejection review asserted here: easy-gain handoff only.

No toolkit issues found; analysis/toolkit-issues.md.

## Round02 disassembly completed

ROM classification: 60.09% (78767 / 131088 bytes classified; 52321 bytes unknown).

Gain50491 (Code8909,Data41582); finalCode26829/Data51938; exact164gap complement and perbankcounts round02/counts.json/unknown-complement.json. Independent proposalreview/initialhelpercorrection/additionalwrapper/staticlocal contracts in round02/review.md,graph-review.json,summary-review.json,saved-selector-wrappers.json,local-data-review.json,local-code-review.json. E7E0invalidseedhaszerodependencies. TemplateROMinputsData,notRAMCode; artificialfetchesremainObservedonlywithprimarycontextidentities; allotherextensionsStatic. Sourceablelookups/text/genericDataacceptedafterconsumer/patterninspection; no objectextentfromoverread. Allready leadsverified with exactoverlap-aware counts in round02/lead-outcomes.json.

| PRG bank | Code | Data | Classified | Unknown |
|---:|---:|---:|---:|---:|
| 0 | 141 | 3717 | 3858 | 4334 |
| 1 | 45 | 5988 | 6033 | 2159 |
| 2 | 181 | 8011 | 8192 | 0 |
| 3 | 10 | 8182 | 8192 | 0 |
| 4 | 873 | 4769 | 5642 | 2550 |
| 5 | 146 | 5065 | 5211 | 2981 |
| 6 | 3790 | 1977 | 5767 | 2425 |
| 7 | 10 | 8182 | 8192 | 0 |
| 8 | 1014 | 162 | 1176 | 7016 |
| 9 | 765 | 808 | 1573 | 6619 |
| 10 | 3501 | 82 | 3583 | 4609 |
| 11 | 961 | 1011 | 1972 | 6220 |
| 12 | 2135 | 82 | 2217 | 5975 |
| 13 | 720 | 374 | 1094 | 7098 |
| 14 | 7274 | 654 | 7928 | 264 |
| 15 | 5263 | 2858 | 8121 | 71 |


Connected additionalservice rootsfollowed through boundedfinalqueue; final-connected-barriers.json retains unknownbank/indirect/mapper frontiers. No arbitraryquota, no exhaustivewholeROMorcompatibilityclaim. Priorboss/ending/terminalscenariosreused; furtherreplayoffersnogainwithoutnewcontextquestion. Nextcurrent-sourcecommentarythenredirect; handoff-round02-disassembly.md. Fresh finalcheckpointmaps/round02-disassembly-complete.json andanalysis/round02-disassembly-complete-verification.json byte-identical; ROMandalloriginalregions/commentarysourceunchanged. Earlierround02-disassemblyinputcheckpointretained ashistorical, notfinalmap. No toolkitissuesfound; toolkit-issues.md.


## Round03 disassembly completed

ROM classification: 85.70% (112348 / 131088 bytes classified; 18740 bytes unknown).

Gain33581bytes (Code6807/Data26774), totalCode33636/Data78712 (includes16headerbytes), Unknown18740/156gaps. Exact counts/complement: round03/counts.json and unknown-complement.json. All1325previous regions preserved exactly by id/content. New165regions are Static. Fresh checkpoint maps/round03-disassembly-complete.json and generated/round03-disassembly-complete passed byte-identical verification in analysis/round03-disassembly-complete-verification.json. Map/stateSHA256 cb5dc575fcb4fee0da113f80281923c2112a323c5180e50fec162fda10e30cf5.

| PRG bank | Code | Data | Classified | Unknown |
|---:|---:|---:|---:|---:|
| 0 | 141 | 8051 | 8192 | 0 |
| 1 | 45 | 8147 | 8192 | 0 |
| 2 | 181 | 8011 | 8192 | 0 |
| 3 | 10 | 8182 | 8192 | 0 |
| 4 | 940 | 7243 | 8183 | 9 |
| 5 | 600 | 7568 | 8168 | 24 |
| 6 | 4074 | 3029 | 7103 | 1089 |
| 7 | 10 | 8182 | 8192 | 0 |
| 8 | 1190 | 6383 | 7573 | 619 |
| 9 | 765 | 7427 | 8192 | 0 |
| 10 | 4276 | 82 | 4358 | 3834 |
| 11 | 2488 | 1011 | 3499 | 4693 |
| 12 | 3816 | 156 | 3972 | 4220 |
| 13 | 2414 | 1712 | 4126 | 4066 |
| 14 | 7423 | 654 | 8077 | 115 |
| 15 | 5263 | 2858 | 8121 | 71 |

Independent review: round03/review.md, independent-decode.json (24695 exact contextual decodes), graph-review.json, summary-review.json (95), wrapper-review.json (27), accepted-boundary-review.json (12276), data-review.json. R03-CONTEXTUAL-RETURNS-DISPATCH and R03-RESIDUAL-NONCODE-CONTAINERS verified in full. AA80 following boundary correction: physicalAC3D is accepted Data, not Code; data pattern remains independently sufficient. Natural reachability remains separate; no Static byte called Observed.

Additional bounded ordinary connected work resolves CB6D map-window return: descriptor802A ->885B initializes pointer7CA8,25 finite cells;88B1 writes7CA8..7D0B inclusive and restoresX/Y. Exact derivation round03/map-window-contract.md. Generic88B1 remains preconditioned; wrapperCB6D can preserve banks because it locally initializes destination. Fresh walk from every current accepted instruction/context with this correction yields0additionalbytes; window-connected-walk/new/barriers.json. This is useful resolved evidence, not an unchanged rejection for redirect.

Broad scenario checklist and previous town/Cellar/finalarea/combat/Mangar/ending/credits/THEEND evidence reused. No unchanged replay or new natural reachability claim. Easy-gain batch complete, not terminal exhaustion. Remaining bank6/8 and10..15 intervals need actual pointer/write/selector/return contracts; exact156gaps in round03/unknown-complement.json. Deferred-frontiers.md remains relevant except CB6D now resolved. No arbitrary byte ceiling or exhaustive closure claim. Next full current-source commentary, then redirect.

ROM SHA25614aca6e380a0fecfede60825f145dbd31a20b1f5b037b902f714b5d364379693 unchanged. Entire commentary tree left untouched; sourceSHA2567ed5020bc57dd7657ebd941720ebc06f7262ad4c5473abbec917167cc1761b77 unchanged, full retained inventory round03/commentary-preserved-hashes.json. Current-source readability gate must be refreshed for new classified bytes. No archive/publication/toolkit edits/cleanup/deletion. archive-options preserved. Disk6.0GiBfree, round03 evidence47MiB, scratch4KiB.

No toolkit issues found; see [toolkit-issues.md](toolkit-issues.md). Separate structural/readability/editorial/archive gates remain pending.



## Round04 disassembly completed

ROM classification: 93.71% (122849 / 131088 bytes classified; 8239 bytes unknown).

Accepted gain10,501bytes:8,472Code/3,883starts plus2,029Data/21regions. TotalCode42,108/Data80,741(including16headerbytes);8,239unknown/72gaps. All1,490priorregions preserved exactly;142newStaticregions. Exact per-bank counts and complement: analysis/round04/counts.json and unknown-complement.json. Evidence-backed annotations cover122849bytes; allCode regions carry execution views. Observed-provenance bytes remain15837; this separate execution-related metric is not whole-ROM structural coverage or full execution coverage.

| PRG bank | Code | Data | Classified | Unknown |
|---:|---:|---:|---:|---:|
| 0 | 141 | 8051 | 8192 | 0 |
| 1 | 45 | 8147 | 8192 | 0 |
| 2 | 181 | 8011 | 8192 | 0 |
| 3 | 10 | 8182 | 8192 | 0 |
| 4 | 948 | 7243 | 8191 | 1 |
| 5 | 624 | 7568 | 8192 | 0 |
| 6 | 4973 | 3084 | 8057 | 135 |
| 7 | 10 | 8182 | 8192 | 0 |
| 8 | 1456 | 6451 | 7907 | 285 |
| 9 | 765 | 7427 | 8192 | 0 |
| 10 | 7030 | 166 | 7196 | 996 |
| 11 | 4545 | 2094 | 6639 | 1553 |
| 12 | 4906 | 233 | 5139 | 3053 |
| 13 | 3672 | 2368 | 6040 | 2152 |
| 14 | 7530 | 660 | 8190 | 2 |
| 15 | 5272 | 2858 | 8130 | 62 |

Independent review: analysis/round04/review.md, independent-decode.json(29,036fresh exact identities), graph-review.json, contract-review.json(134body/stackgraphs), data-review.json and preservation-audit.json. BothR04ready leads verified in full. Exactconditional domains retained: eightselectorwrappers andrefresh/copyhelpers are not globalreturns. Cash/loan60is evenwordoffset0..10; stale outerhistoricalgenericindexwording does not override correctedconditionalcontract. B76613-bytepairedwordresolver accepted independently of naturalcaller evidence. Save26calls/layout2+16*43+8*16+88 andcompactiontwo-counter invariant independentlyreviewed. Naturalreachability remains separate; no newStaticbyte calledObserved.

Fresh all-current-root continuation walk after annotations found0additionalinstructions/bytes. analysis/round04/window-connected-walk.json/new.json/barriers.json retain128unresolvedbank/RAM,43indirectdispatch and31mapperwrite barriers. Small residualfragments reviewed; no isolatedRTS/pointer/legaldecode shortcut accepted. Dynamic32/3A upload/callback origins,06F6audio selector/context, mapper/nonrestoringcall context, broadattack/defeatstate predicates,name-retry stack/callback andmixedfragment boundaries remain concrete redirectquestions. This pass is complete, not terminalexhaustion. No supported ordinarycontinuation deliberately held for anothercycle.

Fresh validated byte-identical checkpoint maps/round04-disassembly-complete.json and generated/round04-disassembly-complete; verification analysis/round04-disassembly-complete-verification.json. Map/state/currentSHA256d9c0309c8498244467257fff3e319114080b4b9bf9422f4b33373560d52fe258. ROM identity unchanged. Entirecommentarytree unchanged; sourceSHA256d38771dc648e7feb91e608734ae43ddf4bc67ab4f1ca6f280828d9fad400d1cb. Priorcommentarygate covers the priorclassifiedsource; next full current-source commentary must cover thisnewbatch. Finaleditorialacceptance remains pending.

Toolkitreport:0confirmedbugs,0suspectedbugs,1documentedmissingfeature(cartridgeRAMdebuggerwrites); existinginternaldamage/store-entry workaround remains nonblocking. No newtoolkitfindings; see analysis/toolkit-issues.md. No toolkit/ROM/commentary edits, archive/publication,cleanup/deletion. archive-options.json remains coordinator-owned.


## Round05 disassembly outcome

# Round05 disassembly complete; ROM incomplete

Current phase: round05 disassembly complete; next full current-source commentary, then redirect. Final editorial acceptance remains pending.

ROM classification: 100.00% (131085 / 131088 bytes classified; 3 bytes unknown).

The rounded percentage does not establish completion: exact fraction131085/131088 = 99.997711461%, with three Unknown one-byte gaps. Accepted gain8236bytes:8036StaticCode/3540starts plus200StaticData/sevenregions. TotalCode50144/Data80941(including16headerbytes). All1632priorregions preserved exactly;75newStaticregions. Exact counts/per-bank/complement: analysis/round05/counts.json and unknown-complement.json. Structural coverage is separate from execution/readability; no new runtime or natural reachability claims.

| PRG bank | Code | Data | Classified | Unknown |
|---:|---:|---:|---:|---:|
| 0 | 141 | 8051 | 8192 | 0 |
| 1 | 45 | 8147 | 8192 | 0 |
| 2 | 181 | 8011 | 8192 | 0 |
| 3 | 10 | 8182 | 8192 | 0 |
| 4 | 948 | 7243 | 8191 | 1 |
| 5 | 624 | 7568 | 8192 | 0 |
| 6 | 5068 | 3124 | 8192 | 0 |
| 7 | 10 | 8182 | 8192 | 0 |
| 8 | 1741 | 6451 | 8192 | 0 |
| 9 | 765 | 7427 | 8192 | 0 |
| 10 | 8025 | 166 | 8191 | 1 |
| 11 | 6098 | 2094 | 8192 | 0 |
| 12 | 7867 | 325 | 8192 | 0 |
| 13 | 5756 | 2436 | 8192 | 0 |
| 14 | 7531 | 660 | 8191 | 1 |
| 15 | 5334 | 2858 | 8192 | 0 |

Independent review: analysis/round05/review.md, independent-decode.json(7896fresh contextual identities), contract-review.json(140local graphs), footprint-review.json(188graphics states;231graphics/562input/127render helperinstructions), finite-domain-review.json, pointer-write-windows.json, data-review.json and preservation-audit.json. BothR05leads verified in full; older broadR02dispatchreadyrecipe superseded by these newer accepted leads. Exact scoped domains remain mandatory: count-stable/nonlethal99DE forA177/A1BB, no-empty-finish interaction protocol forname paths, PendingEventSelector0..32 andvalid savedselector for9169. No global return/defeat/retry correctness assertion.

Ordinary nearby continuation review seeded all acceptedinstructionviews and reused supported contextual contracts;0additionalinstructions/bytes. window-connected-walk/new/barriers.json retain261unresolved effectivebank/RAM,46indirectdispatch and31mapperwrite observations; these are conservative walk barriers, not counts of Unknown structural bytes. Three residual isolatedRTS remain physical00943D(bank4,CPU942D),014E64(bank10,CPU8E54),01D284(fixedbank14,CPUD274). No direct fallthrough/known pointer/role distinguishes no-op entry from surplus return/fill. D95B independently differs because acceptedD0A3cell6 andtheD07F functionconsumer supply a static no-op role. Ready-batch residual review is not terminal workflow exhaustion; next phases still follow.

Broad scenario checklist and successful laterarea/Mangar/endingnarrative/credits/THEEND plus targeted defeat-bookkeeping evidence reused; no unchanged replay. Historical failed area6 shortcut and broader runtime uncertainties remain preserved. No obvious supported nearby continuation was deferred.

Fresh validated byte-identical checkpoint maps/round05-disassembly-complete.json; generated/round05-disassembly-complete; verification analysis/round05-disassembly-complete-verification.json. Map/state/currentSHA256165f81bd4144e5ddb8712506c572218948bc6e214a3d87b968dea3d8c715a123. ROM/configuredreleasebinary/workspace/archiveoptions unchanged. Entire commentary tree unchanged(25files), sourceSHA25658419299aa63f2f2a94b54c7eec0640696bd176c6dd62134a5d586f6ca773901. Prior readability gate covers prior source only; newclassifiedbytes require fullcurrent-sourcecommentary. Required accepted857Fprose correction: formationID0..7,three8-word/16-byte directoriesBB8D/BB9D/BBAD; separate F759characterID0..15,43-byte records. Dependency: analysis/redirect/round04-r05/commentary-dependencies.json. Source left unchanged here.

Toolkitreport:0confirmedbugs,0suspectedbugs,1existingdocumentedmissingfeature(R04-MF-PRGRAM-WRITE); supported internal-state/store-entry workaround remains nonblocking. No new toolkit findings; see analysis/toolkit-issues.md. No toolkit/ROM/commentary edits, archive/publication,cleanup/deletion. archive-options.json remains coordinator-owned. Handoff disk free5423030272bytes.

