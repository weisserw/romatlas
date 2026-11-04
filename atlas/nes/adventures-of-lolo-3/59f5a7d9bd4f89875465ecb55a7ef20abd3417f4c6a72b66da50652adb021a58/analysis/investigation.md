# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## Round 1 disassembly — initial evidence and connected batch
No ready leads were supplied (analysis/leads.json initially empty); natural gameplay and validated vectors were the starting roots. Baseline was not imported as classification. Structural roles and natural reachability remain separate.

- Hardware: parsed 16-byte iNES header, mapper1; PRG [16,131088), CHR [131088,262160). Reset/NMI/IRQ vectors [131082,131088): $FFF0/$D677/$D6E4 in fixed final 16 KiB. Reset's first opcode at physical131072 performs MMC1 reset then enters initialization. Effective startup slots [0,1,14,15].
- Natural scenarios: startup/title, character swap/overworld, first Grandpa tutorial. Exact routes and snapshots in analysis/scenarios.json. Tutorial dialogue was sampled; no unsupported claim of tutorial puzzle completion.
- Artificial scenarios: supported password buffer patching accepted underwater09, caves4–7, final King and independent level17 codes. Underwater09 room1 visibly visited, movement/action/retry fetched. Cave/final castle maps and exteriors are useful route evidence but not boss/ending visits. Failed King/cave directions are retained; do not repeat unchanged blind permutations.
- Password contract: physical [0x52A0,0x52C8) display alphabet (40 symbols), [0x52C8,0x52F0) display-key→valid-symbol mapping (40 bytes, $FF invalid), [0x53D2,0x53F2) valid alphabet (32 symbols). The $9268 consumer indexes display keys using 10-column row/column arithmetic, stores ASCII to RAM$0600 and mapped5-bit index to $E9, count in$93. END is keyboard cursor x187/y124 ($7A/$82). ASCII-only and 40-alphabet-index patches rejected; corrected32-symbol indices accepted. Consumer disassembly in analysis/scratch/password-entry-code.json and related dumps. Known table extent follows consumer key domain and immediately adjacent code/table endpoints, not selected examples alone.
- Code closure: accepted contextual fetch roots, hardware vectors, direct JSR callees and ordinary static branches. Toolkit conservatively stops at indexed stores; continued only zero-page indexed or absolute-index writes whose full possible index range stays below mapper space. Direct-call return continuation additionally requires a complete bounded callee CFG, ordinary RTS, no explicit stack operations or stack/alias writes, no unresolved indirect control/calls/writes, and no mapper-affecting write. Bank-only early summaries were candidate-only drafts and were tightened before annotation because inline-data parsers may rewrite return addresses. No normal return assumed for those parsers. Static reachability remains unresolved absent actual fetch.
- CHR: entire iNES-bounded CHR ROM classified Image as 8192 2bpp tiles in PPU space, with visible title/map/puzzle output; never treated as CPU code.
- Uniform PRG data: exact maximal FF/02/01/00 runs independently justify generic non-code Data after neighbor/reference/code-conflict inspection. Specific fill/table meanings remain unresolved; see analysis/disassembly-001/uniform-data-review.json. Lack of execution is not the data argument.

Current continuation: strict return-contract closure and final annotation/checkpoint underway. Unknown gaps remain ambiguous code/data or structures not investigated in this pass; they are not classified by raw export fallback. Useful redirect prerequisites: final-castle map-node/entrance/progression contract; cave approach route; indirect-dispatch selector contracts; return/inline-data formats; bounded table consumers around residual PRG ranges. These are an ordinary easy-gain handoff, not terminal whole-ROM exhaustion.

### Round1 final checkpoint

ROM classification: 62.26% (163209 / 262160 bytes classified; 98951 bytes unknown).

| Space/bank | Code | Data/image | Classified | Unknown |
|---|---:|---:|---:|---:|
| header | 0 | 16 | 16 | 0 |
| PRG 00 | 1607 | 0 | 1607 | 6585 |
| PRG 01 | 0 | 3209 | 3209 | 4983 |
| PRG 02 | 498 | 112 | 610 | 7582 |
| PRG 03 | 0 | 1109 | 1109 | 7083 |
| PRG 04 | 0 | 0 | 0 | 8192 |
| PRG 05 | 81 | 1578 | 1659 | 6533 |
| PRG 06 | 0 | 0 | 0 | 8192 |
| PRG 07 | 0 | 1850 | 1850 | 6342 |
| PRG 08 | 0 | 0 | 0 | 8192 |
| PRG 09 | 0 | 2792 | 2792 | 5400 |
| PRG 10 | 2801 | 0 | 2801 | 5391 |
| PRG 11 | 1213 | 1822 | 3035 | 5157 |
| PRG 12 | 1835 | 0 | 1835 | 6357 |
| PRG 13 | 0 | 2839 | 2839 | 5353 |
| PRG 14 | 4638 | 0 | 4638 | 3554 |
| PRG 15 | 731 | 3406 | 4137 | 4055 |
| CHR 00 | 0 | 8192 | 8192 | 0 |
| CHR 01 | 0 | 8192 | 8192 | 0 |
| CHR 02 | 0 | 8192 | 8192 | 0 |
| CHR 03 | 0 | 8192 | 8192 | 0 |
| CHR 04 | 0 | 8192 | 8192 | 0 |
| CHR 05 | 0 | 8192 | 8192 | 0 |
| CHR 06 | 0 | 8192 | 8192 | 0 |
| CHR 07 | 0 | 8192 | 8192 | 0 |
| CHR 08 | 0 | 8192 | 8192 | 0 |
| CHR 09 | 0 | 8192 | 8192 | 0 |
| CHR 10 | 0 | 8192 | 8192 | 0 |
| CHR 11 | 0 | 8192 | 8192 | 0 |
| CHR 12 | 0 | 8192 | 8192 | 0 |
| CHR 13 | 0 | 8192 | 8192 | 0 |
| CHR 14 | 0 | 8192 | 8192 | 0 |
| CHR 15 | 0 | 8192 | 8192 | 0 |

Verification: analysis/pass-001-verification.json confirms fresh byte-identical generated/pass-001 against unchanged input. Final closure/proofs in analysis/disassembly-001/. All lead outcomes in analysis/leads.json; one documentation discrepancy in analysis/toolkit-issues.md, no blocker. Ordinary easy-gain handoff; terminal rejection review intentionally not claimed.

## Round1 redirect r1-node-and-residual — ready batch
Resolved cave/King entry through map-loop flag20/node contract, completed representative bosses and King phases, late17 puzzle, finalrescue/story/THE END artificially. Independent graphics36 and room110 finite-container proofs plus tile/palette/audio/dispatch data support303 proposals/45444 bytes;860 unknown opcode starts observed. Methods, scoped negative results, exact recipes and remaining frontier: analysis/redirect/r1-node-and-residual/HANDOFF.md. No terminal exhaustion assertion; independent disassembly review required.

## Round 2 — disassembly pass complete, classification incomplete

ROM classification: 82.21% (215534 / 262160 bytes classified; 46626 bytes unknown).

Added52325 unique physicalbytes:2783 Code,49542 Data. Totals Code16187,Data68275,Image131072;169 unknownranges/46626bytes,1206 classifiedregions. Exact unique accounting and perbank counts: [counts](disassembly-002/counts.json); exact complement [unknownranges](disassembly-002/unknown-ranges.json). No unknown/raw export bytes count towardclassification.

All317 redirect proposals/45556bytes independently accepted after exactbyte/physicalbank/nonoverlap checks and consumer review. Independent graphics36/room110 reparsing reproduced endpoints; generic audio noncode distinct from individualstream/table semantics; resetcopies require explicit mode3/effectivebank/BFF0alias contract. Strict contextual/static callback/directcall/indexsafe/return closure has8079 physical instructionstarts/13482 views, no overlappingstarts or Data conflicts;1400 newstarts/2723bytes.476 stack-safe return summaries,22 narrowly bankedcall contracts (deduplicated contractrecords may include round repeats), inline-wordskip contracts preserve barriers. Observed provenance11583bytes totals actual accepted fetches; static203951bytes include code/data/image, and neither is wholeROM reachability.

Connected work beyond originalmanifest accepted4046 extraData:32 inlinewordsbytes,26 audio register/maskbytes,3532 exacthalf-square/reciprocal/collision/template/dialogue/PPU bytes,456 direction/glyph/angle/attribute numericbytes. All consumer and finiteformat reviews in disassembly-002; overlapping oldregions preserved. Broad saved gameplay checklist through THE END reviewed and reused without replay.

Fresh [pass-002 verification](pass-002-verification.json) ok=true,byte_identical=true; [frozen map](../maps/pass-002.json), [source](../generated/pass-002/game.asm), currentstate/maps saved and validated. MapSHA256 0f7d34a9b9671ad226a8c8fdc207d6028d4daa3033f0b008f9b8f10824d0aebb; inputSHA256 unchanged 59f5a7d9bd4f89875465ecb55a7ef20abd3417f4c6a72b66da50652adb021a58. Old commentary/sourcehash83d224cc948e45460fdc28d4b585b0bbd187a35db3e139d3e5cca611471a57b5 and all oldcheckpoints/history preserved. New source needs its own full commentaryreview; prior gate does not cover newbytes. No finaleditorial acceptance.

Lead outcomes: R1-NODE-DATA,GRAPHICS-CONTAINERS,ROOM-CONTAINERS,AUDIO-CORPUS,BOSS-ENDING-CODE,ACTOR-DISPATCH,RESET-MIRRORS verified; prior unresolvedPRG status superseded by freshledger, R2-MIXED-SCRIPTS-AND-CALLBACKS open research. Ready finitebatch/connected easygains exhausted for thispass; remaining opaque scripts/native mixedranges lack bounded interpreter/entry contracts. This is ordinary incomplete handoff, not terminal exhaustive rejectionreview. Unique [handoff](disassembly-002/HANDOFF.md); old disassembly-handoff preserved.

Toolkit: existing [NES-DOC-001](toolkit-issues.md) documentation/APIcontract issue remains nonblocking; no newissues, no toolkitedits/rebuild. Bounded subprocesses stopped; disk9.0GiB free, scratch14MiB, round2 durable evidence6.8MiB before finalnotes. No cleanup/deletion/archive/publication.

| Space/bank | Code | Data/image | Classified | Unknown |
|---|---:|---:|---:|---:|
| header | 0 | 16 | 16 | 0 |
| PRG00 | 1617 | 5917 | 7534 | 658 |
| PRG01 | 6 | 8186 | 8192 | 0 |
| PRG02 | 498 | 1086 | 1584 | 6608 |
| PRG03 | 6 | 8186 | 8192 | 0 |
| PRG04 | 0 | 0 | 0 | 8192 |
| PRG05 | 87 | 4623 | 4710 | 3482 |
| PRG06 | 0 | 8192 | 8192 | 0 |
| PRG07 | 6 | 8186 | 8192 | 0 |
| PRG08 | 341 | 10 | 351 | 7841 |
| PRG09 | 1300 | 2892 | 4192 | 4000 |
| PRG10 | 3154 | 138 | 3292 | 4900 |
| PRG11 | 1219 | 5651 | 6870 | 1322 |
| PRG12 | 1854 | 445 | 2299 | 5893 |
| PRG13 | 6 | 8186 | 8192 | 0 |
| PRG14 | 5315 | 823 | 6138 | 2054 |
| PRG15 | 778 | 5738 | 6516 | 1676 |
| CHR00 | 0 | 8192 | 8192 | 0 |
| CHR01 | 0 | 8192 | 8192 | 0 |
| CHR02 | 0 | 8192 | 8192 | 0 |
| CHR03 | 0 | 8192 | 8192 | 0 |
| CHR04 | 0 | 8192 | 8192 | 0 |
| CHR05 | 0 | 8192 | 8192 | 0 |
| CHR06 | 0 | 8192 | 8192 | 0 |
| CHR07 | 0 | 8192 | 8192 | 0 |
| CHR08 | 0 | 8192 | 8192 | 0 |
| CHR09 | 0 | 8192 | 8192 | 0 |
| CHR10 | 0 | 8192 | 8192 | 0 |
| CHR11 | 0 | 8192 | 8192 | 0 |
| CHR12 | 0 | 8192 | 8192 | 0 |
| CHR13 | 0 | 8192 | 8192 | 0 |
| CHR14 | 0 | 8192 | 8192 | 0 |
| CHR15 | 0 | 8192 | 8192 | 0 |

## Round 3 disassembly — accepted actor/native/connected batch

ROM classification: 99.98% (262120 / 262160 bytes classified; 40 bytes unknown).

Added46586 unique Static bytes:8870Code/37716Data. Totals Code25057/Data105991/Image131072;1856classified regions,40unknownbytes/12ranges. All1206 oldannotations retained byte-for-byte includingIDs/evidence/provenance/views. Unique counts/perbank: [counts](disassembly-003/counts.json); exact physical complement+rawbytes: [unknownranges](disassembly-003/unknown-ranges.json). Observed11583bytes unchanged, Static250537bytes; execution coverage and natural reachability are separate fromclassification.

R2-MIXED-SCRIPTS-AND-CALLBACKS verified: all46583 proposedbytes independently accepted. Fresh configured-binary decode checked12315 proposedphysicalstarts/19496views; 6633script/context records and frame/composite/rectangle/palette/container formats independently reparsed against immutable ROM. All4236 proposednewstarts structurally connect to reviewed typed native/callback, finite interpreter/entity dispatch, manualdataflow and prioracceptedcode interfaces. These checks supplement finiteCPUstack/indirectRAM/bank/return contracts; legaldecode/pointers alone do not provecode. Native/manual semantic and exactbankreview: [contracts](disassembly-003/connected-contract-review.md), [formatcheck](disassembly-003/review-format-checks.json), [connectedgraph](disassembly-003/connected-structure-audit.json), [acceptanceledger](disassembly-003/accepted-claim-ledger.json).

Connected continuation beyond manifest: lowcommand21CF31 field0..1 plusactor0..7 limitsdestination0090..009F; operator0..2 dispatches coherentAND/OR/ADC bodies whose STA cannot touchmapper/liveframes/scratch. Their exactRTS tails118661/118668/118676 independently accepted (3bytes), freshdecoded. No currently parsedcommand21/naturaluse claimed. Tightened D7C8 boundedordinaryreturn precondition to endlowbyte!=FF; role remains coherentfillloop. D433/D439/D43F/D10B preserve inline2bytebarriers, C119 haspartiallowercontext until literals, selector5 means10/11, C228successcalls8150, A536 aliascorrect. Partialselector71665 boundssevenwords despiteencodedcount10. Primarycorrections/history retained.

Fresh [pass-003 verification](pass-003-verification.json) ok=true,byte_identical=true; [frozenmap](../maps/pass-003.json), [generatedsource](../generated/pass-003/game.asm). State/current/pass003 areidentical; mapSHA25677e3a1d0900145aa22eeab2afdf67423bfd07b4b8585bd0fd2ba11ec86139d20. InputROM hashunchanged. Full priorcommentary/source/review preservedunder disassembly-003/preserved-commentary; sourcehashc47a0895d2e576321ce33bf333b1e18691791edc4716cdfa2e3ee3810bec506d. Old1375reviewed/0pending gate appliesoldsource only; fullpass003commentary andfinaleditorial acceptance pending. No commentaryrefresh orfinalaudit performed.

Broadscenarios throughKingphases/rescue/story/THE END reviewed/reused; no startup/endingreplay ornewruntimecoverage. Singlepassready/connected batchcomplete; unresolved40bytes retainambiguousstructure, includingD07Dsame-bytepointerprefix. No speculativeorphanPC, exhaustive residualsweep or terminalexhaustionclaim. Next separatecommentary, thenredirect withfreshcomplement. [Handoff](disassembly-003/HANDOFF.md).

Toolkit: existing [NES-DOC-001](toolkit-issues.md) remainsnonblocking; no newissues. No toolkit edit/rebuild, ROMchange, publication, archive, deletion/cleanup. Subprocesses stopped; disk8.9GiBfree/scratch14MiB/disassemblyevidence17MiB.

| Space/bank | Code | Data/image | Classified | Unknown |
|---|---:|---:|---:|---:|
| header | 0 | 16 | 16 | 0 |
| PRG00 | 1638 | 6554 | 8192 | 0 |
| PRG01 | 6 | 8186 | 8192 | 0 |
| PRG02 | 1139 | 7053 | 8192 | 0 |
| PRG03 | 6 | 8186 | 8192 | 0 |
| PRG04 | 10 | 8182 | 8192 | 0 |
| PRG05 | 108 | 8084 | 8192 | 0 |
| PRG06 | 0 | 8192 | 8192 | 0 |
| PRG07 | 6 | 8186 | 8192 | 0 |
| PRG08 | 1179 | 7010 | 8189 | 3 |
| PRG09 | 1872 | 6314 | 8186 | 6 |
| PRG10 | 7497 | 694 | 8191 | 1 |
| PRG11 | 1693 | 6496 | 8189 | 3 |
| PRG12 | 2127 | 6062 | 8189 | 3 |
| PRG13 | 6 | 8186 | 8192 | 0 |
| PRG14 | 6688 | 1480 | 8168 | 24 |
| PRG15 | 1082 | 7110 | 8192 | 0 |
| CHR00 | 0 | 8192 | 8192 | 0 |
| CHR01 | 0 | 8192 | 8192 | 0 |
| CHR02 | 0 | 8192 | 8192 | 0 |
| CHR03 | 0 | 8192 | 8192 | 0 |
| CHR04 | 0 | 8192 | 8192 | 0 |
| CHR05 | 0 | 8192 | 8192 | 0 |
| CHR06 | 0 | 8192 | 8192 | 0 |
| CHR07 | 0 | 8192 | 8192 | 0 |
| CHR08 | 0 | 8192 | 8192 | 0 |
| CHR09 | 0 | 8192 | 8192 | 0 |
| CHR10 | 0 | 8192 | 8192 | 0 |
| CHR11 | 0 | 8192 | 8192 | 0 |
| CHR12 | 0 | 8192 | 8192 | 0 |
| CHR13 | 0 | 8192 | 8192 | 0 |
| CHR14 | 0 | 8192 | 8192 | 0 |
| CHR15 | 0 | 8192 | 8192 | 0 |

## Round 4 — disassembly complete; ROM incomplete

ROM classification: 100.00% (262154 / 262160 bytes classified; 6 bytes unknown).

Exact percentage 99.99771132133048%; rounded100.00does not mean complete. Added34Staticbytes31Code/3Data in6regions; totalsCode25088/Data105994/Image131072,1862regions. Preserved all1856oldregion objects byte-for-byte. Counts/perbank: [counts](disassembly-004/counts.json); complement [unknownranges](disassembly-004/unknown-ranges.json). Observed11583unchanged; no newruntime or naturalreachability evidence.

R3-RESIDUAL-CONTRACTS verified/consumed ready=false; six independent role proofs accepted after fresh contextualdecode and phase/branch/queue/typed-record checks. [Review](disassembly-004/role-review.md), [acceptance](disassembly-004/accepted-regions.json). Connected consumer/renderer/entity/queue graphs alreadyclassified; five loneRTS andzero retain separate ambiguity. No forcedcompletion/terminalexhaustion claim. Prior broadgameplay throughTHEEND reused without replay.

Fresh [pass004verification](pass-004-verification.json):ok=true,byte_identical=true; state/current/pass004 identical mapSHA2569b4691491780172c94178a7c5a1fad550a10f91c0b0f71302ee664d76c486382. ROMunchanged. Full commentarypreserved [here](disassembly-004/preserved-commentary/); oldsourcehash9cb507770898f551ac529e642d13702b02f5da1036f32a65e34fc3f4fd5bbfa7. Newcurrent-sourcecommentary andfinaleditorial acceptance pending; no commentary/redirect run inthispass. [Handoff](disassembly-004/HANDOFF.md).

Toolkit: existingNES-DOC-001nonblockingdocumentationissue only; no newtoolkitissues found. No toolkitedit/rebuild,ROMchange,publication,archive,deletion/cleanup. Configuredserver stopped; assets/source/evidence local; disk8.9GiBfree,scratch14MiB.

| Space/bank | Code | Data/image | Classified | Unknown |
|---|---:|---:|---:|---:|
| header | 0 | 16 | 16 | 0 |
| PRG00 | 1638 | 6554 | 8192 | 0 |
| PRG01 | 6 | 8186 | 8192 | 0 |
| PRG02 | 1139 | 7053 | 8192 | 0 |
| PRG03 | 6 | 8186 | 8192 | 0 |
| PRG04 | 10 | 8182 | 8192 | 0 |
| PRG05 | 108 | 8084 | 8192 | 0 |
| PRG06 | 0 | 8192 | 8192 | 0 |
| PRG07 | 6 | 8186 | 8192 | 0 |
| PRG08 | 1179 | 7010 | 8189 | 3 |
| PRG09 | 1878 | 6314 | 8192 | 0 |
| PRG10 | 7497 | 694 | 8191 | 1 |
| PRG11 | 1693 | 6499 | 8192 | 0 |
| PRG12 | 2129 | 6062 | 8191 | 1 |
| PRG13 | 6 | 8186 | 8192 | 0 |
| PRG14 | 6711 | 1480 | 8191 | 1 |
| PRG15 | 1082 | 7110 | 8192 | 0 |
| CHR00 | 0 | 8192 | 8192 | 0 |
| CHR01 | 0 | 8192 | 8192 | 0 |
| CHR02 | 0 | 8192 | 8192 | 0 |
| CHR03 | 0 | 8192 | 8192 | 0 |
| CHR04 | 0 | 8192 | 8192 | 0 |
| CHR05 | 0 | 8192 | 8192 | 0 |
| CHR06 | 0 | 8192 | 8192 | 0 |
| CHR07 | 0 | 8192 | 8192 | 0 |
| CHR08 | 0 | 8192 | 8192 | 0 |
| CHR09 | 0 | 8192 | 8192 | 0 |
| CHR10 | 0 | 8192 | 8192 | 0 |
| CHR11 | 0 | 8192 | 8192 | 0 |
| CHR12 | 0 | 8192 | 8192 | 0 |
| CHR13 | 0 | 8192 | 8192 | 0 |
| CHR14 | 0 | 8192 | 8192 | 0 |
| CHR15 | 0 | 8192 | 8192 | 0 |

## Round 5 — disassembly complete; ROM incomplete

ROM classification: 100.00% (262155 / 262160 bytes classified; 5 bytes unknown).

Exact percentage 99.9980927677754%; rounded100.00% does not establish completion. Added1 Static Data byte101752; totals Code25088/Data105995/Image131072,1863 regions. All1862 prior region objects/IDs/provenance/views/evidence preserved. Observed11583 unchanged, no new runtime/reachability claims. [Counts/per-bank](disassembly-005/counts.json), [complement](disassembly-005/unknown-ranges.json).

R4-SCRIPT-BOUNDARY-DATA verified/consumed ready=false after independent primary byte/parent/corpus review. Generic noncode only; no stop/endcap/padding/container/natural-entry claim. [Review](disassembly-005/role-review.md), [checks](disassembly-005/independent-checks.json). Five RTS67407/68116/70230/86712/120565 remain ambiguous. Connected graphs already classified; broad gameplay through THE END reused without replay. Single pass ends at easy-gain exhaustion; no terminal redirect assessment.

Fresh [pass005 verification](pass-005-verification.json) ok=true,byte_identical=true; state/current/pass005 mapSHA2569ed86bef8db138cb02991ec2a718f4febe4c972a607a962a32558b900843601a. ROM unchanged. Prior full commentary preserved byte-for-byte [here](disassembly-005/preserved-commentary/), sourceSHA256428802d744ac32fad35012a1e4f1e3039551ea10ff07ae79cc8059c040967911. New current-source commentary and final editorial audit pending. [Handoff](disassembly-005/HANDOFF.md).

Toolkit: existing [NES-DOC-001](toolkit-issues.md) confirmed documentation bug remains nonblocking; no new toolkit issues. No toolkit edit/rebuild, ROM change, deletion/cleanup, archive or publication. Configured server stopped; all artifacts local; disk8.8GiBfree/scratch14MiB.

| Space/bank | Code | Data/image | Classified | Unknown |
|---|---:|---:|---:|---:|
| header | 0 | 16 | 16 | 0 |
| PRG00 | 1638 | 6554 | 8192 | 0 |
| PRG01 | 6 | 8186 | 8192 | 0 |
| PRG02 | 1139 | 7053 | 8192 | 0 |
| PRG03 | 6 | 8186 | 8192 | 0 |
| PRG04 | 10 | 8182 | 8192 | 0 |
| PRG05 | 108 | 8084 | 8192 | 0 |
| PRG06 | 0 | 8192 | 8192 | 0 |
| PRG07 | 6 | 8186 | 8192 | 0 |
| PRG08 | 1179 | 7010 | 8189 | 3 |
| PRG09 | 1878 | 6314 | 8192 | 0 |
| PRG10 | 7497 | 694 | 8191 | 1 |
| PRG11 | 1693 | 6499 | 8192 | 0 |
| PRG12 | 2129 | 6063 | 8192 | 0 |
| PRG13 | 6 | 8186 | 8192 | 0 |
| PRG14 | 6711 | 1480 | 8191 | 1 |
| PRG15 | 1082 | 7110 | 8192 | 0 |
| CHR00 | 0 | 8192 | 8192 | 0 |
| CHR01 | 0 | 8192 | 8192 | 0 |
| CHR02 | 0 | 8192 | 8192 | 0 |
| CHR03 | 0 | 8192 | 8192 | 0 |
| CHR04 | 0 | 8192 | 8192 | 0 |
| CHR05 | 0 | 8192 | 8192 | 0 |
| CHR06 | 0 | 8192 | 8192 | 0 |
| CHR07 | 0 | 8192 | 8192 | 0 |
| CHR08 | 0 | 8192 | 8192 | 0 |
| CHR09 | 0 | 8192 | 8192 | 0 |
| CHR10 | 0 | 8192 | 8192 | 0 |
| CHR11 | 0 | 8192 | 8192 | 0 |
| CHR12 | 0 | 8192 | 8192 | 0 |
| CHR13 | 0 | 8192 | 8192 | 0 |
| CHR14 | 0 | 8192 | 8192 | 0 |
| CHR15 | 0 | 8192 | 8192 | 0 |
