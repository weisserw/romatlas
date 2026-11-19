# Disassembly progress

Historical initialization status (superseded below): baseline and classification had not run.

## Round 001 disassembly — active
Baseline established and verified byte-identical: maps/baseline.json,
analysis/baseline-verification.json, generated/baseline. ROM identity unchanged.
Mapper 4, 16 physical 8 KiB PRG banks and 128 KiB CHR; 262160 file bytes.
Runtime startup: reset CPU $FF30 physical $1FF40; initial effective banks
[0,1,14,15], startup changes to [2,6,14,15]. Tracing accepted Instruction and
ControlFlow aggregates, capacity 0; 20-frame bounded steps. Full all-event
180-frame tracing timed out; retry uses narrower trace and short chunks.
Frontier: capture title snapshot/frame, inspect identity, annotate hardware header,
CHR role and accepted instruction fetches; explore gameplay and static successors.
This pass remains active and has not claimed completion or exhaustion.

### Active frontier update
ROM classification: 51.09% (133939 / 262160 bytes classified; 128221 bytes unknown).
Header/CHR plus 2851 code bytes saved; baseline remains verified.
Game confirmed Bad Dudes US title. Correct input schema is buttons list/port;
earlier button/pressed menu attempts produced no input (NES-001, toolkit-issues.md).
Corrected start/A sequence reached briefing at frame1788, analysis/game2.png.
Currently bypassing briefing via SELECT and preparing stage RAM experiments.
Pass still active, not complete.

### City mechanics batch
ROM classification: 56.06% (146974 / 262160 bytes classified; 115186 bytes unknown).
Natural City snapshot frame3318; right+A+B 20 frames, release5 frames.
Accepted 4854 unique physical opcode starts (6652 contextual roots), 13035
new code bytes with bounded static successor support. No aggregate drops;
detail capacity0 intentionally drops detail, saved contexts/candidates retain evidence.
Now branching pre-stage briefing snapshot frame2898, patching $02C0/$0109
to level1..6 followed by ordinary initialization and brief mechanics probes.

## Round 001 disassembly — pass complete, ROM incomplete

ROM classification: 63.16% (165586 / 262160 bytes classified; 96574 bytes unknown).

Authoritative unique interval counts from analysis/state.json: 22423 Code,
12091 Data (including header/vector bytes), 131072 Image.
1208 nonoverlapping evidence-annotated regions; 390 unknown gaps.
Earlier intermediate arithmetic is superseded by round-001-stats.json.

| PRG 8 KiB bank | Code | Data | Unknown |
|---:|---:|---:|---:|
| 0 | 1822 | 264 | 6106 |
| 1 | 0 | 3952 | 4240 |
| 2 | 0 | 307 | 7885 |
| 3 | 0 | 240 | 7952 |
| 4 | 0 | 368 | 7824 |
| 5 | 85 | 693 | 7414 |
| 6 | 1882 | 0 | 6310 |
| 7 | 2280 | 0 | 5912 |
| 8 | 2579 | 2408 | 3205 |
| 9 | 1562 | 1596 | 5034 |
| 10 | 0 | 885 | 7307 |
| 11 | 0 | 1021 | 7171 |
| 12 | 521 | 174 | 7497 |
| 13 | 3328 | 0 | 4864 |
| 14 | 4889 | 12 | 3291 |
| 15 | 3475 | 155 | 4562 |

Header:16 Data; CHR:131072 Image, zero unknown.

Structural provenance: Static 152438 bytes; Observed 13148 bytes.
All classified regions have evidence strings. No claim of semantic/editorial review
completion: full current-source commentary is next. Code roots from direct calls
and hardware vectors are Static; accepted fetch contexts separately establish
execution. Runtime files union 7652 unique ROM opcode starts,
16540 bytes in fetched instructions; naturally exercised
subset 6561 opcode starts. These are coverage metrics, not
classification percentages. Trace aggregate drops were zero; capacity0 intentionally
retained no event detail. Contexts/candidates and RPC transcript preserve evidence.

Visited: title/selection/briefing/City naturally; Big Rig, Sewer, Forest, Train,
Cave, Factory artificially via safe hook; representative move/attack/jump probes.
Factory1200-frame health/time-assisted exploration reached later enemies, not a
confirmed boss/elevator/ending. Ending shortcut failed; no ending visit claimed.
See scenarios.json and gameplay.md.

Verification: internal map.validate ok; current portable map validates; fresh
checkpoint maps/round-001-disassembly.json and generated/round-001-disassembly
rebuild byte-identically (analysis/round-001-disassembly-verification.json).
SHA256 unchanged ad0ba0b8b4f6dc25d0244cf649a1a60e240c86218e4069dd40a3beff04fd9860.
Exact residuals: round-001-unknown.json; per-bank counts: round-001-counts.md.

Remaining productive questions L001–L004 are handed to redirect, not declared
exhausted: map/data boundaries, boss/ending contracts, sound sample extents and
unvisited actor handlers. This one easy-gain pass is complete; ROM is incomplete.
Toolkit summary: 1 confirmed documentation/API mismatch NES-001, 0 suspected
bugs, 0 missing-feature findings; no blockers. See toolkit-issues.md.

## Round 001 commentary — active

Supported prepare created frozen baseline and independent source; baseline
byte-identical. Current source readability is in progress, not ready for redirect.
First reviewed sound-driver slice and shared symbols are recorded in
commentary/progress.md and commentary/review.json. Exact pending inventory:
round-001-commentary-remaining.json. Preserve this work and resume same phase.
Batch verification passed byte-identical; classification/state/maps unchanged.

Commentary sound/non-code batch: all classified bank-zero Code through physical2507
reviewed, all Data/Image and unknown gaps explained. Exact remaining1113 Code
entries start at physical47542; inventory round-001-commentary-remaining.json.
Fresh source verification passed byte-identical and reading copy refreshed.
Structural question CPU$8566/physical$576 (BRK versus alternate duty-table read)
recorded in commentary/uncertainties.md. Classification preserved; gate pending.

Commentary next batch: bank5 initializer, bank6 classified screen setup through
physical51643 and reset physical130880..131058 now reviewed. Source explains
metatiles/attributes, stage tables, screen sequence mechanics and hardware setup.
Fresh byte-identical verify passed; exact pending inventory refreshed. First
pending Code physical51643. Same commentary phase remains active.

Commentary bank6 complete: all current classified bank6 Code reviewed, including
status-screen transfers/event masks and collision page/type/quadrant resolution.
Fresh byte-identical verification passed. First pending Code physical57360 in
bank7; exact inventory refreshed. Commentary remains the active phase.

### Coordinator phase status: commentary interrupted, resume commentary

Context-capacity checkpoint saved in round-001-commentary-handoff.md. Reviewed
Code3967 bytes; remaining18456 Code bytes across1060 exact pending entries,
first physical57360. Same commentary source/inventory must resume, not redirect.
All Data/Image and unknown-gap notes reviewed;118 meaningful entry-block units.
Fresh verification byte-identical; reading-after.asm refreshed; classification
unchanged. Full current-source readability gate and final editorial gate pending.

### Commentary resumed — active bank7 readability

Fresh-context resume preserved source/baseline and historical interrupted handoff
as round-001-commentary-a-handoff.md. Added bank7 stage/scroll/screen-record
readability through physical59458:981 Code bytes/70 entries reviewed. Fresh
byte-identical verify passed. Pending990 entries/17475 Code bytes, first59568.
Classification unchanged; full readability gate pending, continue same phase.

Commentary bank7 complete:2280 classified Code bytes reviewed. Tile/attribute
expansion, scroll transition/page mechanics, palette and partial OAM template
paths explained. Fresh byte-identical verify passed; subsequent local scratch
alias refinement will be included in next batch verification. Pending921 entries/
16176 Code bytes start65552(bank8); continue same phase, readability gate pending.

Commentary bank8 complete: all2579 classified Code bytes now readable, including
stage events, actor-class timers and collision rectangles/contact effects. Fresh
byte-identical verify passed, including bank7 final alias correction. Pending767
Code entries/13597bytes first73780(bank9). Classification unchanged; full current-
source gate remains pending. Continue commentary, not redirect.

Commentary bank9 complete:1562 classified Code bytes explained, including actor
family ABI, response/collision paths and asymmetric separation calculation.
Pending645 Code entries/12035bytes first98320(bank12); banks10/11 contain no
classified Code. Last batch byte-identical; final separation-name/prose correction
will be included in next verification. Continue commentary; no gate success yet.

Commentary bank12 complete:521 Code bytes cover the actor metasprite renderer,
stream formats, clipping/flips and bidirectional OAM output. Fresh verify passed
byte-identical including bank9 separation correction. Pending620 Code entries/
11514bytes first106512(bank13). Full readability and editorial gates pending.

Commentary bank13 complete:3328 classified Code bytes explain player actions,
health/immunity, animation/shape/motion streams, movement bounds, collision and
player OAM. Fresh byte-identical verification passed after local scratch aliases.
Pending463 Code entries/8186bytes first114710(bank14); continue same commentary.

### Commentary context checkpoint B — interrupted, resume commentary

Reviewed Code16667 bytes; remaining291 entries/5756 Code bytes, first117379(bank14
NMI). Added fixed API/actor primitive contracts and main stage/session setup; all
banks7/8/9/12/13 classified Code now reviewed. Preserved prior reset and all non-Code
credit. Fixed body evidence corrected bank9 contact/update-flag interpretation and
OAM-proven X/Y names. Final fresh byte-identical verify passed; reading copy
refreshed. Full readability gate remains incomplete. Resume exactly the remaining
inventory/source, without prepare/regeneration. Latest/historical B handoff records
capture hashes and frontier; classification unchanged. Editorial audit unrun.

## Active redirect round-001-r1
Current-source commentary C and metadata correction are complete; authoritative handoff analysis/round-001-commentary-handoff.md. Input checkpoint round-001-disassembly:165586 classified/262160,96574 unknown. Research only; map/state/source/ROM unchanged. Attempt directory analysis/redirect/round-001-r1. Frontier: L007 alternate pulse duty table extent and predecessor; L003 sound/sample consumer context; L001 patterned stage banks; L004 actual actor/data bank selection; L002 factory progression and stage7 versus subvariant contract. Starting primary-source inspection; existing socket transport unavailable under sandbox, direct run.py serve subprocess available as bounded alternative.

Redirect round-001-r1 milestone: findings.md records L007 invalid unconditional-branch fallthrough premise and four-byte Data correction candidate; L004 exact type partition (bank10 types$20..$26; bank11$27+) plus bounded47-entry RTS table and16 API roots/recursive direct calls in actor-discovery.json; L002 physical$1C3A0 continuation contains $02C1==$8E gated bank6$A00C terminal-screen contract. No map/state/source/ROM changes. Connected post-call handler continuations, stage-data extents and terminal screen prerequisites remain active.

Redirect milestone2: L003 resolved direct DAC sample contract physical$2710..$3110 via handler$9E6..$A08 (4011 writes, A700-to-B100 exclusive pointer loop). 51-word sound command table$656..$6BC and missing handlers decoded in sound-command-table.json/sound-discovery.json; all remain proposals. L001256-byte evidence survey supports repeated stage/graphics non-code prefix$4010..$B9B6, carefully split around known classifications; mixed bank5 tail remains code frontier. L004 post-call continuations and L002 terminal screen investigation ongoing. No accepted bytes or protected-file edits.

Redirect milestone3: L002 revised explicit terminal predicate shortcut(02C1=8E,PC=C399) reaches visibly distinct desk/official/flag terminal scene with typing text. Primary terminal-contexts.json confirms bank6A00C/AFAE; scenes saved/scenarios updated. PC sampled in RAM is not itself a corruption proof; withdraw premature inference from first shortcut while retaining actual first-test observations. L004 reviewed24 fixed-helper contracts (actor-helper-review.json), expanded graph2002 instructions/4706bytes across banks10/11;10 further helper contracts remain finite frontier. Continue terminal controls and actor closure in same redirect.

Redirect milestone4: actor graph5550bytes led to four concrete RTS state dispatches; added30 bounded states(23+7) and28 conditional state1..14 entries to connected investigation, actor-dispatch-expanded.json active. Bank5$9A3A90-byte initialization routine identified via bank7 mapper/call/restore, with explicit63/135byte parameter-table splits (findings.md). Terminal continuation interrupted143 after120frames typing, before A test; saved frame/results, next shortA60 test from preserved terminal snapshot. Continue same redirect; no map/state/source/ROM edits.

Redirect milestone5: actor-final-graph3848instructions/8943bytes with direct-call barriers closed; inline state/path data still reviewed separately. Graphics independent7tableextent proof/counts187/95/84/92/199/89/242; alternate is+156slice, not newtable. A60terminalcontrol test completed, same scene/typing, release[] snapshot saved. Bank5tail literalJSR9B1B found physicalF6CF, tied to source final-stage/subvariant dispatcher next. No accepted/map changes; exactproposal ledger being assembled after remaining finite source contract.

Redirect milestone6: Factory subvariant/phase/script graph1681instructions/3522bytes; source table3scriptentries proves bank5$9B1B caller under stage6bank5. CHR-only mapper sites X0/1 verified; single fixedC03Fhelper frontier. Terminal milestone/control outcomes already durable; no ongoing emulator wait. Prepare exact proposal/unknown complement ledger after this finite contract review; preserve map/state/source unchanged.

Redirect milestone7: C03Fhelper and soundlocalcallclosure complete. Provisionalproposal53034newbytes(13033Code/40001Data),1retype,0accepted; exact physical candidates saved, conflicts0. ThreebyteData/code conflict resolved by observed fourth soundAPI$8009JMP$89D6; Data narrowed to8pointerbytes. Leads/pass history upgraded. Exactunknownledger pending. Independentbank12metasprite avenue L009 active:301records valid;6910directlycovered of7496candidatebytes,586separator/extraformatted/palettebytes under boundedinspection; markerfield correction recorded. Research continues same redirect, no map/state/source changes.

Redirect milestone8: bank12L009 adds7497; playerbank13L010 adds1502 via53fullyconsistentrecords,5FFtail+bankmarker. Combinedproposals62033newbytes(13033Code/49000Data),accepted0,conditionalresidual34541. Exactresidualaudit exposes toolkitindexedwritebarrier after knownsafeRAMstores, so previous directcallclosure is onlypartialCFGclosure. New bank-safe indexedwrite proof (base+FF<8000) opens concrete sound/actor/finalstage continuations; indirectstores remainexplicitpointerbarriers. Continue SAMEredirect; no classificationchanges. Lead/ledgerupdates pending refreshedexpanded candidates, active coremetadata saved.

Redirect milestone9: exact unknown-ledger.json saved for all390originalunknownranges/96574bytes, neighbors/bankcontexts/evidence-linked dispositions; accepted0. Currentproposal62033,conditionalresidual34541; ledger explicitlyactive andwillrefreshafternewgraphreview. Bank-safeindexedwrite method completed sound1019instructions/2174totalCodebytes,124uniqueprovablybank-safe indexedwrites,0directcallbarriers; actors banks9/10/11(31lowAPIsadded)7781instructions/18001totalCodebytes,475safeindexedwrites,11fixedhelpercontracttargetsremaining. These totalsincludepreclassifiedbytes; notaccepted/newcounts. Stagegraphsame-methodrunning. No ROM/map/state/source/toolkitmutations.

Redirect milestone10: elevenadditionalactorhelpersreviewed; C10Brequires literal67/68handlercontract, othersordinarybankpreserving. Corrected my lowbank9APIstrideassumption beforepromotion:31tabletypes→28distinctJMPslots; extra roots causedoverlappinginstructions. Oldsafe-storeactorgraph retained invalid-rootattempt; v2 uses exact47wordtypecontracts (all47targetsJMP) and isrunning. Current62033proposal/ledgerunaffected. L009ready7497Data, L010ready1502Data; lead/passhistoryupdated. Next v2overlapaudit/literalpointerwriter-consumer closure. No classification/protectedfilechanges.

Redirect milestone11: correctedactorsafev2 audit passed7780instr/17999totalCodebytes,0instructionoverlaps/knownDataoverlaps/BRK-KIL-RTI. 47literalhandlerpointerwrites→28conditionalABItargets nowlinked tofixedadapters; v3running with11newreviewedhelperreturncontracts. Stage/soundsafegraphscomplete0callbarriers. Current62033proposalunchangeduntilv3audit. Exactunknownledgerall390/96574current; samephaseactive.

Redirect milestone12: v3audited8701instr20226totalCodebytes,0overlap/Data/BRK-KIL-RTI/mapperwrites. V4finished8740instr20320totalbytes;6newhelpersreviewed, remainingC192reviewed collisionprobe preservebanks. Newdispatch15B60reuses23entryknownstatetable, no unseenstatefrontier. One minimalC192continuation thenfinalaudit/package/ledgerpromotion. No liveemulatortoolwait; completedgraphartifactsretained. Currentmapunchanged.

Redirect milestone13: finalactor8753instr20353totalbytes clean audit0barriers/overlaps/Data/BRK-KIL-RTI/PRGwrites/indirectwrites;5142completed. Safegraphs promoted; exactproposal72412new(23412Code/49000Data),0accepted,conditionalres24162; exactledgerrefreshed390/96574. Concrete ninthpointeradapter63/64discovered from bank9writer1338B→94CE and unknownC11Dtrampoline; finalconnected continuation nowactive beforehandoff. No arbitraryonebyteRTSclassification; protectedfilesunchanged.

Redirect milestone14: ninth63/64adapter exact17bytes and bank9target94CE44bytes supported; bounded targetcontinuation packaging next. ActorresidualData consumers identified, exclude isolatedNOP/RTS. L002actual8Ewriterfoundfinalsubvariant3phase12..15 scriptBD==0F. Source-backed8Dalternate-Factory initializerrecord57+graphics+156slice yields new safehooktest(level6,variant8D), distinctfromfailedstage7. Planrecorded before experiment; snapshot-before-RAMmutations required. Current72412package/ledgerstillauthoritativeuntilrefresh.

Redirect milestone15: variant8Dsafeinit testcomplete180frames, actualstage6→variantCD/subvariant3/BD0B and helicopter/player scene; scenariosupdated, representativebuttonsright+a+b60. Source-backedfinalphase15/BD0F/D3=1/D4=0 RAMonlywritertestplanned beforeexecution, restoresinitializedsnapshot, noPCedit; boundedresultwillclosepracticalnaturalrouteeffortwithoutarbitraryphasesweep. Ninthadapter/remainingactorData structuralpackaging ongoing; current72412proposalunchangeduntilrefresh.

Redirect milestone16: plannedRAMwriter testfinished normally(handle1986closed); factory-script-results/contextssaved. FB65actual8Ewriterfetchconfirmed, finalCE/phase15/BD15livehelicopter; noA00C/C399terminalfetch. Scenariosupdated. Missingearlierphase/player/variant-entryflags justify stoppingarbitraryforcing; independentdeskterminalvisitstands. Ninthadapter/44bytehandler closed, boundedactorData/helpers package refresh isnextactualoperation; no pendingruntimetool.

Redirect milestone17: currentpackage72971new(23565Code/49406Data),1retype,0accepted,conditionalres23603; exactledgerrefreshed390/96574,conflicts0. Ninth44+17Code,9staticlocalhelpers92Code,406field/markerData included. ToolkitNES002citationcorrectedvalidfinal555safewrites; old475invalidrootmeasurementexplicitlysuperseded. Actualresidualdeferrals documented includingC607operandcounterexample forbank5tail. One finiteconnectedfixedhelper-prefix method remainsactive (knownreviewedtrampolineroots only, notwholeROM); noemulatorpending.

Redirect milestone18: fixedhelperfirstboundedgraphcompleted(handle32766closed):61roots/2011candidateinstructions/4401totalbytes,113historicalstopsbut5unresolvedfinalreturneffects. Candidatesnotclassification. Actualbarriers HUDRAMpointer0262+A+Ymax0460 and guardedmapperF758/F766; explicitsource/parametercontractreview nowdocumented. Nextseparatehelperfinalgraph closesnamed5using RAMrange andX0..7/stage0..6contracts, exactaudit→package/ledger/handoff. No liveemulator; no globalrootexpansion or protectedchanges.

Redirect milestone19: fixedhelperfinalauditpassed61roots/2118instr/4634totalbytes;1992outsidebaselinebeforeproposaloverlap,0remainingreturneffects/CodeDataoverlap/instructionoverlap/BRK-KIL-RTI. Conditionalstack/HUD/mapper/terraincontractsexplicit;17280completed. Namedhelperavenueclosed, exactcandidateprefixespromoted; currentproposals/unknownledgerfreshcounts. Next finalleadcounts/residualdispositions/protectedhashes/compactcoherentbatchhandoff; no pendingruntime or newrootexpansion.

## Round001 redirect complete — ordinary combined ready batch
Attempt analysis/redirect/round-001-r1 is complete and handed off; no further writes
by this worker. Checkpoint round-001-disassembly unchanged. Proposed74695unique new
bytes:25289Code/49406Data, plus1L007Code→Data correction; accepted0. Actualunknown96574
in390ranges; conditionalresidual21879. Current-source commentary gate and allprotected
hashchecks unchanged. Exactproposals, disjointleadcounts, complementledger, primary
findings/negativehistory and compacthandoff are durable. Pass/leadhistory updated.
No terminalexhaustion/completion claim; independentdisassembly adjudication next.
Oneconfirmedtoolkitbug/onegroundedmissingfeature, no blocker. No runtime/queryhandle,
archive,cleanup,deletion,publication,toolkitedit or competingphase. Completedattempt
immutable after this boundary.

## Active ROUND002 disassembly
Input checkpoint round-001-disassembly (165586/262160;96574 unknown). Sole phase writer; independent structural review of completed immutable round-001-r1 proposals underway. Commentary history preserved. Active records: analysis/round-002-disassembly/phase.json.

ROUND002 accepted structural batch applied:74695 newbytes (25289Code/49406Data),
plus1Code→Data correction; map.validate passed. ROM classification: 91.65%
(240281 / 262160 bytes classified; 21879 bytes unknown). Exact328-gap complement,
per-bank Code/Data/unknown and independent baseline-preservation invariant checks
saved in analysis/round-002-disassembly/statistics.json and unknown-complement.json.
Structural rationale adjudication.md supports full batch with narrowed genericData
semantics; all11201 candidate views independently decoded. Fresh checkpoint
round-002-disassembly generated/verified byte-identical (verification JSON saved).
Prior commentary untouched. One terminal followthrough300-frame request timed out;
smaller60-frame bounded chunks now running from original saved snapshot to preserve
state/progress per chunk. No map writes in scenario process; pass remains active.

## ROUND002 disassembly — accepted structural counts and verification

ROM classification: 91.65% (240281 / 262160 bytes classified; 21879 bytes unknown).

Accepted74695 new bytes:25289Code and49406Data; corrected1 existingCode byte to
Data at physical576. Totals:47711Code,61498Data,131072Image,21879Unknown. Header16
andCHR131072 are classified. Exact physical unknown complement has328ranges.
Independent structural adjudication, boundaries and contracts:
analysis/round-002-disassembly/adjudication.md; independent-checks.json and
statistics.json. Accepted manifest and map.annotate transcript retained there.

| PRG bank | Code | Data | Unknown |
|---|---:|---:|---:|
| 0 | 2237 | 5954 | 1 |
| 1 | 0 | 8182 | 10 |
| 2 | 0 | 8192 | 0 |
| 3 | 0 | 8192 | 0 |
| 4 | 0 | 8192 | 0 |
| 5 | 258 | 6765 | 1169 |
| 6 | 2213 | 1 | 5978 |
| 7 | 5448 | 139 | 2605 |
| 8 | 2579 | 2409 | 3204 |
| 9 | 6509 | 1629 | 54 |
| 10 | 6841 | 1341 | 10 |
| 11 | 7139 | 1050 | 3 |
| 12 | 521 | 7671 | 0 |
| 13 | 3328 | 1502 | 3362 |
| 14 | 5933 | 12 | 2247 |
| 15 | 4705 | 251 | 3236 |

Execution is separate:13920 physical Code bytes carry Observed provenance,
33791 are Static. The new batch includes772 artificial fetched bytes and24517
Static bytes; existing natural/artificial observed provenance is preserved.
Observed map provenance is not a whole-runtime execution-coverage percentage.
Annotation coverage is240281 unique structural bytes; generic Data has evidence
and explicit role limits, with no invented consumer or natural-selection claims.

Fresh checkpoint: maps/round-002-disassembly.json and generated/round-002-disassembly.
analysis/round-002-disassembly-verification.json confirms byte_identical:true.
map.validate passed; internal state/export/checkpoint agree. Existing classifications
preserved except documented duty-byte correction; originalROM and priorcommentary
source/reading/historical checkpoints hashes unchanged (preservation-checks.json).
The expanded current map needs a fresh full-source commentary pass; prior completed
commentary is preserved, not reused as automatic credit for newly classified code.
Final editorial audit and whole-ROM completion remain pending.

Lead outcomes: L001/L002/L003/L004/L007/L008/L009/L010 verified for exact accepted
structural slices; global history appended. L005 verified/L006 local rejection
unchanged. Broader residual mixed screen/event/player/path/engine structures need
local entry/boundary/bank/schema investigation in redirect, not arbitrary byte
classification or repeated unchanged play. This is an ordinary incomplete handoff,
not a terminal whole-ROM exhaustion assessment.

Toolkit report: analysis/toolkit-issues.md retains1 confirmed documentation/API
bugNES-001,1 grounded missingfeatureNES-002,0 suspectedbugs,no blocker. Fullbuttons
lists and full-address-domain static bank checks worked. No toolkit edit/build
substitution, publication, cleanup, deletion or archive performed.

## ROUND002 disassembly COMPLETE — final counts supersede initial batch

ROM classification: 92.26% (241858 / 262160 bytes classified; 20302 bytes unknown).

This one pass accepted76272 newbytes (25858Code/50414Data) plus1Code→Data correction.
Initial74695byte batch/91.65% checkpoint remains historical. Productive terminal
continuation reached real WhiteHouse credits from saved artificial scene, adding
1577bytes:569Code(520Observed/49Static) and1008Data. Mechanical consumer/extent proof
and independent rationale are credits-structural-contracts.json / credits-adjudication.md
in analysis/round-002-disassembly. No naturalbosscompletion/fullcredits claim.

Final totals:48280Code,62506Data,131072Image. Header16 andCHR131072 fully classified.
ObservedCode provenance14440bytes; StaticCode33840bytes, separate from execution
coverage. NewpassCode1292Observed artificial/24566Static. Annotation coverage241858
unique structurally classified bytes; exact20302unknown complement in329ranges.

| PRG bank | Code | Data | Unknown |
|---|---:|---:|---:|
| 0 | 2237 | 5954 | 1 |
| 1 | 0 | 8182 | 10 |
| 2 | 0 | 8192 | 0 |
| 3 | 0 | 8192 | 0 |
| 4 | 0 | 8192 | 0 |
| 5 | 258 | 6765 | 1169 |
| 6 | 2706 | 1009 | 4477 |
| 7 | 5448 | 139 | 2605 |
| 8 | 2579 | 2409 | 3204 |
| 9 | 6509 | 1629 | 54 |
| 10 | 6841 | 1341 | 10 |
| 11 | 7139 | 1050 | 3 |
| 12 | 521 | 7671 | 0 |
| 13 | 3328 | 1502 | 3362 |
| 14 | 6009 | 12 | 2171 |
| 15 | 4705 | 251 | 3236 |

Final authoritative checkpoint: maps/round-002-disassembly-final.json,
generated/round-002-disassembly-final, analysis/round-002-disassembly-final-verification.json.
Fresh mapvalidate and byte_identical:true; state/export/finalcheckpoint identical.
OriginalROM, earliercheckpoints and completedcommentary source/symbols/reading
hashes unchanged. Preservation and baselineclassification invariants passed.
Wholeexpandedmap commentary/readability remains next; earlier commentaryhistory
was preserved without modification. Final editorialaudit remains unrun.

L001/L002/L003/L004/L007/L008/L009/L010 accepted exactstructuralslices; L002 also
creditsconnectedaddendum. Globalleadhistory and scenarios appended; L005/L006
history unchanged. Practical routes through7areas plushelicopter/terminal/credits
provide representativeprogression; no unchangedplay or arbitraryendingflags need
retry. Strongremaining mixedscreen/event/player/path/engineunknowns need local
ABI/schema/bank-boundary research. Isolated4C6 and369A..36A4 remain unsupported;
C607operandcounterexample rejects onlycanonicalbank5callinterpretation. No terminal
whole-ROMexhaustion claim. Ordinarypass complete; ROM incomplete.

Toolkit summary:1confirmed documentation/API bugNES-001;1groundedmissingfeature
NES-002;0suspectedbugs;no blocker. Workarounds fullbuttonslist and official
indexedwrite fulladdressdomain checks succeeded. Initial300frameterminal request
hit configuredtimeout; retry10x60completed, so timeout is not a newtoolkitbug.
Report analysis/toolkit-issues.md retained/updated. No toolkitedit/buildsubstitution,
archive,cleanup,deletion,publication or competingphase. No liveemulator/runtimequery.
Compacthandoff: analysis/round-002-disassembly-handoff.md.


## ROUND002 commentary active

Complete prior directory preserved with SHA256 completeness manifest at round-002/commentary-preservation.json; fresh preparation and exact-range carry succeeded, freshly byte-identical. 1207 blocks carried; exact remaining inventory round-002/commentary-pending.json (25858 Code, 50415 Data, 20302 Unknown bytes). Active frontier: round-002/commentary-active-phase.md. Current-source readability incomplete; same commentary phase must resume until full scope is complete.


ROUND002 commentary batch001:416 newCode bytes and9920 Data bytes reviewed,329 exact Unknown boundary notes; successful freshbyteidentical rebuild. Remaining600 records/25442Code40495Data. Active frontier and exact inventory under round-002; fullsource task remains active/incomplete, no redirect handoff.


ROUND002 commentary batch002: all remaining87 Data records/40495bytes explained using adjudicated consumers/record contracts; freshly byteidentical, reading refreshed. Exact remaining513Code records/25442bytes (grouped under round-002/commentary-pending-by-bank.json);85 candidate stale dependency notes queued for scoped review. Samephase active/incomplete; no finalhandoff/audit.

ROUND002 commentary worker C active: latest completed B batches003..005 cover all bank5/6/7 Code; all Data and Unknown notes already complete. Exact resume233 Code records/21277 bytes, actors9..11 and fixed14/15. Fixed helper mechanical review and source batch006 in progress, followed by actors and scoped dependency reconciliation. Same phase incomplete; no final audit/handoff.

ROUND002 commentary batch006 C: all131 fixed14/15 pendingCode records /2350bytes reviewed; freshly byteidentical and reading refreshed. Exactremaining102actorCode records /18927bytes (bank9:78/4947;10:5/6841;11:19/7139), plus scoped dependency notes. Source scope incomplete; samephase active.

ROUND002 commentary batch007 C:30 bank9 Code continuations /595bytes reviewed, six actor stale notes corrected; freshbyteidentical/read refreshed. Remaining72 Code records /18332bytes, actor9..11. Fixed-bank scoped candidates complete (16 corrected/18 retained);26 actor candidates remain. Current-source readability incomplete; samephase active.

ROUND002 commentary batch008 C:347 actorbank9 Codebytes/two records reviewed as four meaningful routines, conversion Data format clarified. Freshbyteidentical and read refreshed after exactCPUalias repairs. Metadataerrors0. Pending70Code records/17985bytes (bank9:46/4005;10:5/6841;11:19/7139),26 scoped actor dependency candidates. Capacitycontinuation commentary-continuation-c.md/latest; samephase incomplete, no redirect/finalaudit.

Worker D batch009: reviewed all433 bytes of bank9 CPU$87C4..$8975 as six scene-scroll/boundary/terminal/delay routines, plus15 response-veneer bytes. Named branches, response fields and scene coordination bytes; explicit timer/clear-record exits and unresolved A=2 call to fixed $C09F. Fresh byte-identical verification passed; reading copy refreshed for this batch. Remaining68 Code records17537bytes: bank9 44/3557; bank10 5/6841; bank11 19/7139.26 scoped actor dependency candidates remain. Same phase active/incomplete; next bank9 CPU$8977 family setup/state graph. No classification changes or final audit.

Worker D batch010: six pending Code records452bytes reviewed as nine units: four state-parameter family entries, their shared installer, three complete contact motion/movement/path handlers and state154 dispatcher. Named all branch targets, scoped actor fields and fixed helper operands; preserved high/low scratch-pointer format and conditional state/path ABI. Verification byte_identical=true, reading copy refreshed, metadataerrors0. Remaining62 Code records17085bytes: bank9 38/3105, bank10 5/6841, bank11 19/7139.26 scoped dependency candidates remain. Continue bank9 CPU$8B7D actor family graph; same phase incomplete, no final audit.

Worker D batch011: five pending bank9 records502bytes completed as11 meaningful units: repeat-family installation/entries/lifecycle dispatcher and the entire334-byte class/repeat/range/movement/path graph. Documented six-repeat shared counter/latch, lifecycle overwrite ordering after synchronous response-path calls, type7 timer/state distinction, signed movement results and direction update relative to camera. $C0E4 remains an unresolved fixed-address call; no shape behavior invented. Fresh verify byte_identical=true, reading refreshed, metadataerrors0. Remaining57 Code records16583bytes: bank9 33/2603, bank10 5/6841, bank11 19/7139.26 scoped actor dependency candidates still pending. Same phase incomplete; next CPU$8D78 graph, no final audit.

Worker D scoped dependency check: bank9 physical75306 still branches to an Unknown outside-margin tail. Retained that uncertainty and replaced stale motion-mode0 wording with the actual zero-step horizontal viewport helper contract.25 scoped dependency candidates remain. No structural edits.

Worker D capacity checkpoint: batches009..011 completed13 Code records1402bytes/27units; final dependency wording reverified byte-identical and reading refreshed. Metadataerrors0. Exactremaining57 Code records16583bytes (bank9 33/2603;10 5/6841;11 19/7139),25 scoped candidates. Unique commentary-continuation-d.md and latest saved; next CPU8D78. Interrupted samephase, not a successful handoff.

Worker E batches012/013: five bank9 Code records438bytes reviewed as eight meaningful units: mode-motion installer/three state entries, full mode-motion response and side-state dispatcher, then both range/direction and player-side handlers. Named branches/fields and fixed helper operands; documented preincremented motion cursor, low-nibble/low-three-bit distinction, zero-health/contact exits, stage4 side inversion, distance112 gate and immediate handler transition. Corrected the newly introduced camera-side name to player-side after reading actual RAM operands; no existing dependency candidates adjudicated. Fresh verify byte_identical=true after each batch; latest reading refreshed and source metadataerrors0. Remaining52 Code records16145bytes: bank9 28/2165, bank10 5/6841, bank11 19/7139;25 scoped candidates. Same phase incomplete, next CPU$8F46 family graph; no final audit.

Worker E batch014: reviewed three bank9 records52bytes (duplicate family-one entry and two bounded continuations) and corrected concrete family-one source contradictions: child creation occurs for nonzero motion bytes, zero terminator reverses direction/selects common-first, and25 is a timer duration rather than a state. Full52-byte special handler contract now explains contact/timer paths. Fresh verify byte_identical=true, reading refreshed and metadataerrors0. Remaining49 Code records16093bytes: bank9 25/2113; bank10 5/6841; bank11 19/7139. One dependency candidate adjudicated,24 remain. Next CPU$9033/family-three motion graph. Same phase incomplete; no final audit.

Worker E batch015: four records115bytes reviewed: alternate family-three entry plus all remaining motion/contact/range/action continuations. Full148-byte motion routine now explains signed decoder branches, range/probe/action modes and ordinary step5/timer6 behavior. Corrected stale unknown claims and renamed misleading StateSix to MotionTimer;5/6 are step/timer values. Two dependency candidates corrected. Batch016: three records498bytes/eight routines reviewed in full: mode/path response, selected-path continuation, phased terrain-probe response, simple motion response, extended dispatcher, direction-dependent probe offset, type19 terrain predicate and clamped coordinate-step helper. Named all internal branches/fields, explained contact flag differences, negative-phase path mode, terrain0 checks, contact-shape writes and repeated identical type19 sample pass. Newly resolved20-byte table now named B9FamilyThreeProbeDeltaWords with two five-word X/Y arrays; no classification change. Fresh verify byte_identical=true and reading refreshed after each batch; metadataerrors0. Remaining42 Code records15480bytes: bank9 18/1500; bank10 5/6841; bank11 19/7139.22 scoped dependency candidates remain. Next CPU$9353 installer/family graph. Same phase incomplete; no final audit.

Worker E capacity checkpoint: batches012..016 completed15 pending Code records1103bytes, with concrete family-one/three dependency fixes and resolved delta-table consumer. Final verify byte_identical=true; reading refreshed; metadataerrors0. Exactremaining42 Code records15480bytes (bank9 18/1500;10 5/6841;11 19/7139),22 candidates. Unique commentary-continuation-e.md/latest saved, nextCPU9353. Interrupted same phase; not a successful handoff.

Worker F batch017: completed three bank9 Code records205bytes as four units: high-byte-first handler installer, two state-family entries and full state selector. Named all internal dispatcher branches and extra handler pointer, explained threshold/state substitutions and conditional bank9 CPU-address contract. Fresh verification byte_identical=true, reading refreshed; metadataerrors0. Remaining39 Code records15275bytes: bank9 15/1295, bank10 5/6841, bank11 19/7139;22 scoped dependency candidates. Same phase active/incomplete; next CPU$9423 connected handler graph.

Worker F batch018: three bank9 Code records453bytes reviewed as nine complete routines/entries across CPU$9423..$95E7. Explained contact latch, signed movement results, step4/deletion and timers, inner/outer distance predicates, path5/path18 selection, wider-range mode latch/counter, action64 and conditional handler/state decoder outputs. Named every internal branch and reused established fixed-helper contracts; depth tests retain exact zero/nonzero meaning without invented gameplay roles. Fresh verify byte_identical=true, reading refreshed, F metadataerrors0. Remaining36 Code records14822bytes: bank9 12/842, bank10 5/6841, bank11 19/7139;22 scoped dependency candidates. Next bank9 CPU$964C alternate family-four entry and full $969B..$99C1 graph. Same phase incomplete; no final audit.

Worker F batch019: seven bank9 pending Code records450bytes/nine units completed: alternate family-four entry, full primary movement/range/action handler, both simple-motion tails, path7/phase-mode/secondary graph and both decoder tails. Corrected concrete State14 misname to MotionTimer20 and obsolete Unknown-tail claims; source and review now contain full40-byte simple-motion and41-byte decoder contracts with bounded shared references. One dependency candidate resolved,21 remain. Fresh byte-identical verification passed; reading refreshed and metadataerrors0. Remaining29 Code records14372bytes: bank9 5/392, bank10 5/6841, bank11 19/7139. Next bank9 CPU$986C/$98C0 graph then three entry veneers. Same phase incomplete; no final audit.

Worker F batch020: completed final five bank9 Code records392bytes as nine units. Full state$B2..$B5 installer/dispatch, primary action-counter/range/movement graph, state$B3 motion/left-boundary deletion, annulus predicate, unsigned decoder threshold and synchronous state entry are explained, with every branch and scoped latch/counter/scroll field named. Three earlier bank9 entry-veneer scopes now link to reviewed functional contracts. Bank9 currently classified readability is complete; the21 scoped dependency candidates still require their concrete checks. Fresh verify byte_identical=true, reading refreshed, metadataerrors0. Exactremaining24 Code records13980bytes: bank10 five/6841 and bank11 nineteen/7139. Both large bank graphs remain mandatory. Same phase incomplete; no final audit.

Worker F capacity checkpoint: batches017..020 completed all18 remaining bank9 Code records1500bytes/31newunits, preserving prior valid review. Final verify byte_identical=true and reading refreshed; F metadataerrors0. Exactremaining24 Code records13980bytes (bank10 5/6841; bank11 19/7139),21 scoped candidates. Unique commentary-continuation-f.md and latest saved; next bank10 CPU8000 large actor graph. Interrupted samephase; no successful handoff or final audit.

Worker G batch021: twelve meaningful units cover bank10 CPU$8000..$81EB (physical81936..82428,492bytes): entry veneers, raised type$26 allocation/placement, full timer/motion tick, shared interval and five-slot response/mask/palette utilities. Two complete scaffold records349bytes reviewed; third record82285..83529 stays pending with prefix82285..82428 reviewed. Exact next CPU$81EC/physical82428 in commentary-coverage-g.json. Fresh verify byte_identical=true, reading refreshed, G metadataerrors0. Remaining22 scaffold records13631bytes include143 reviewed bytes in the partial record; actual untouched classified bytes13488.21 scoped candidates remain. Same phase incomplete.

Worker G batch022: seven meaningful units cover CPU$81EC..$83DA (physical82428..82923,495bytes): full first-family motion/dispatch loop, child allocation helper, paired-byte state selection, contact/health transitions, one-shot award sentinel and stage-variant gate. Third scaffold remained pending with covered prefix82285..82923; fresh verify byte_identical=true, reading refreshed and metadataerrors0.

Worker G batch023: ten meaningful units cover CPU$83DB..$8638 (physical82923..83529,606bytes), completing the third scaffold: full range/facing/path chooser, conditional RTS-minus-one dispatcher, class/shape loader, timed hold, path and movement handlers, phase transitions and event reconciliation. Explicitly explained class1..14 dispatch precondition, ineffective masked-nibble guard, unbounded reversal/move retry and class14 shape reads crossing the two14-byte classified fields. No structural change. Fresh verify byte_identical=true, reading refreshed and metadataerrors0. G total29 units1593bytes, three scaffold records complete. Exact remaining21 Code records12387bytes: bank10 two records5248bytes (83615..85435 CPU868F..8DAB;85509..88937 CPU8DF5..9B69), bank11 nineteen7139bytes.21 scoped dependency candidates remain. Exact next CPU$868F/physical83615; no partial record remains. Same phase incomplete; no final audit.

Worker G batch024: five complete units cover bank10 CPU$868F..$879E (physical83615..83887,272bytes): family wrapper/repeat loop, state refresh/paired-byte selector, complete update gate, RTS dispatcher and shared188-frame window bookkeeping. Explained the state-$54 Z-flag ABI rather than assuming normalized A. Fresh verify byte_identical=true, reading refreshed and metadataerrors0. Scaffold83615..85435 remains pending with completed prefix83615..83887.

Worker G batch025: four full planner units cover CPU$879F..$8987 (physical83887..84376,489bytes): special/return/ordinary/alternate response selection, flag-based side/state gates, distance bands, direction/control latch changes and three independent mode counters. Checked exact branch addresses and corrected the within72 latch-retention/outside48 counter-bypass prose before final verification. Fresh verify byte_identical=true, reading refreshed and metadataerrors0. G total38 units2354bytes. Pending21 scaffold records12387bytes include761 reviewed bytes in partial83615..85435; actual unread11626bytes. Next CPU$8988/physical84376; partial suffix84376..85435 remains1059bytes. Other bank10 record85509..88937 remains3428bytes; bank11 nineteen records7139bytes and21 scoped candidates remain. Same phase incomplete.

Worker G batch026: two complete units cover bank10 CPU$8988..$8A33 (physical84376..84548,172bytes): contact eligibility/forced$70 transition, response-indexed update masks, living/zero-health states12/13, shape/mode latches and full award/type$21 completion. Fresh verify byte_identical=true, reading refreshed and metadataerrors0. G total40 units2526bytes. Pending21 scaffold records12387bytes include933 reviewed prefix bytes in83615..85435; actual untouched11454bytes. Exact next CPU$8A34/physical84548; pending suffix84548..85435 is887bytes. Other bank10 record85509..88937 remains3428bytes; bank11 nineteen7139bytes and21 scoped candidates remain. Same phase incomplete.

Worker G capacity checkpoint: batches021..026 completed40 units2526bytes, including three complete bank10 scaffolds1593bytes and933-byte reviewed prefix of scaffold83615..85435. Exact next CPU8A34/physical84548; suffix84548..85435 (887bytes), untouched bank10 85509..88937 (3428), all bank11 7139 and21 scoped candidates remain. Pending scaffold21/12387 includes933 reviewed bytes; actual untouched11454. Final verify byte_identical=true, read refreshed, metadataerrors0. Unique G/latest continuation and coverage-g saved. Interrupted same phase, not a successful handoff.

Worker H batch027: six complete movement/path/completion units cover CPU$8A34..$8BA5 (physical84548..84918,370bytes). Named internal branches and established RAM/fixed-entry operands. Fresh verify byte_identical=true, reading refreshed, H metadataerrors0. Partial scaffold83615..85435 has reviewed prefix83615..84918 (1303bytes), remaining517bytes CPU$8BA6..$8DAB. Pending scaffold21/12387 includes1303 reviewed bytes; actual untouched11084bytes. Other bank10 record3428bytes, bank11 7139bytes and21 scoped candidates remain. Same phase active/incomplete.

Worker H batch028 and boundary correction: completed13 meaningful helpers CPU$8BA7..$8DAB exclusive/physical84919..85435 (516bytes), completing scaffold83615..85435. Immutable ROM establishes inherited G frontier8A34 is Award RTS; current Award unit extends1 byte and H batch027 actually spans8A35..8BA7 /84549..84919 (370bytes). Corrected current summaries: the movement handlers always start their timer, bypassing only cursor reset on nonzero motion. Historical G artifacts remain unchanged. H total19 units886bytes plus1 prior Award RTS corrected, leaving20 Code scaffold records10567bytes: bank10 85509..88937 CPU8DF5..9B69 (3428), all bank11 nineteen7139bytes;21 scoped dependency candidates remain. Next CPU8DF5/physical85509. Same phase active/incomplete; no final audit.

Worker H batch029: five complete coordinated-family entry/window units cover CPU$8DF5..$8F1B exclusive / physical85509..85803 (294bytes). Explained low5-bit RTS dispatcher, initial path/timer, four-slot initialization, palette/slot window and upper-response delegation; preserved downstream helper uncertainty. Fresh verify byte_identical=true/read refreshed/H metadataerrors0. Pending20 Code scaffold records10567bytes include294 reviewed prefix; actual untouched10273bytes. Exactnext CPU8F1B/physical85803; bank10 suffix85803..88937=3134bytes, bank11 nineteen7139bytes and21 scoped dependencies remain.

Worker H capacity checkpoint: interrupted same phase;24 units1180bytes and inherited Award RTS boundary correction completed. Current exact frontier8F1B/85803; actual untouched10273bytes,21 dependencies. Unique continuation-h/coverage-h authoritative; H stopped writing. No successful handoff.

Worker I batches030..032:15 meaningful units1468bytes reviewed continuously CPU8F1B..94D7 / physical85803..87271. Explained active response/counter gates, response4/5/20 motion, selected/alternating/cursor path handlers, timed wait, contact adjustment and completion award; reused established fixed/RAM names and named every internal branch. Each coherent batch freshly byte-identical. Exact next94D7/87271; partial scaffold85509..88937 has1762 reviewed prefix bytes,1666 remain; raw pending20records10567bytes includes1762 reviewed, actual untouched8805=1666bank10+7139bank11.21 dependency candidates remain. Active/incomplete same phase; no successful handoff or audit.

Worker I batch033:12 meaningful units544bytes CPU94D7..96F7/physical87271..87815. Explained packed two-bit slot-class94 operations (renamed prior window-counter alias to packed-slot-classes), palette bits, health-window ABI,15-byte actor clone preserving timer flags and slot counters. Literal small-total stores use A despite unused Y class values; documented without inventing repair. Fresh byte-identical verify. Next96F7/87815;1122bank10+7139bank11=8261actual untouched,21dependencies; raw scaffold pending unchanged20/10567. Same phase incomplete.

Worker I batch034:7units432bytes CPU96F7..98A7/87815..88247: contact initialization, slot removal/sound gate, packed-field count/replacement/search and mode-field reader. Explained Y5 mask overflow returning0 despite separate middle-field increment helper. Fresh byte-identical verify. Next98A7/88247;690bank10+7139bank11=7829actual untouched,21dependencies. Same phase incomplete.

Worker I batch035:16code units690bytes CPU98A7..9B59 /88247..88937 complete bank10 scaffold85509..88937. Added4dependent-data units in88985..89242 explaining shape pairs/state groups/direction groups and observed selector fields. The return-word base9B5B maps physical88939, matching the existing23-word table; no overlap or structural question. Fresh byte-identical verify. I total50code units3134bytes, plus4data units257bytes clarified. Pending now19bank11 Code scaffolds7139bytes,21dependency candidates; bank10 fully reviewed. Next bank11 physical90128 CPU8000. Same phase incomplete.

I boundary correction: actual emitted bank10 scaffold ends CPU9B59/physical88937, not inherited9B69. Current last-unit length and coverage corrected; historical H artifacts unchanged. A draft overlap question used a mistaken16-byte physical conversion and is explicitly rejected; no map or ROM change.

Worker I capacity checkpoint: interrupted same phase after50Code units3134bytes and4dependent Data units257bytes clarified. Bank10 fully reviewed; actual next bank11 CPU8000/physical90128. Exact remaining19Code scaffolds7139bytes and21scoped dependencies. Current CPUend9B59 correction recorded; false arithmetic overlap rejected. Fresh verify/read and metadataerrors0. Unique continuation-i/latest and coverage-i authoritative; I stopped writing. No successful handoff.

Worker J batch036:9units654bytes bank11 CPU8000..828E/90128..90782 reviewed. Fresh byte-identical verify/read and metadataerrors0. Scaffold90128..91735 remains pending:654 reviewed prefix,953 remaining. Raw pending19/7139 includes654reviewed; actual untouched6485 plus21scoped dependencies. Exactnext90782/828E. Same phase incomplete.

Worker J batch037:14units953bytes CPU828E..8647/90782..91735 complete the first bank11 scaffold90128..91735. Explained state6E contact/live phase/completion, state6C and68/69/6A/63 timers, global-control dispatcher and five timed motion entries. Corrected phase3 to4-frame timer (not movement decoder) during verification review. Fresh byte-identical verify/read and metadataerrors0. J total23units1607bytes. Pending18Code scaffolds5532bytes plus21scoped dependencies. Next91736/CPU8648;91735..91736 remains Unknown. Same phase incomplete.

Worker J batch038:3units126bytes CPU8648..86C6/91736..91862 complete the next scaffold. Explained response setter paired state-byte ABI, shared97 sign rearm handshake and seven-word bounds/RTS dispatch; named dependent table CPU bases and carried names into reviewed veneers. Fresh byte-identical verify/read metadataerrors0. J total26units1733bytes. Pending17Code5406bytes plus21scoped dependencies. Next91890/CPU86E2 after existing28-byte Data tables. Same phase incomplete.

Worker J scoped dependency batch: checked all21remaining candidates by physical anchors/current source and map boundaries (85total now85checked,0pending). Corrected6affected source regions: pending-event reader classified setup vs Unknown update, actor search direct found RTS, threshold classified allocator entry vs Unknown post-call tail, classified301-record bank12 metasprite table, complete local type4/6 collision returns, and complete vertical-crossing alternative/PLA exits. Added local reader setup prose. Retained15valid Unknown-boundary or semantic-limit claims; compact per-candidate evidence in commentary-dependency-review-j.json and candidate inventory. Fresh verify byte_identical=true/read refreshed metadataerrors0. Code readability still17Code scaffolds5406bytes; next91890/86E2. Same phase incomplete.

Worker J capacity checkpoint: interrupted same phase after26Code units1733bytes across first two bank11 scaffolds and all21remaining scoped dependencies checked (85/85checked0pending). Six stale dependency prose claims corrected. Remaining17Code5406bytes,next91890/86E2; no partial scaffold. Fresh verify/read metadataerrors0. Unique continuation-j/latest and coverage-j authoritative. J stopped writing; no successful handoff.

Worker K batch039:8 meaningful units998bytes CPU86E2..8AC8/physical91890..92888 complete the seven-state contact/phase, completion, timed path and next-dispatch scaffold. Named all internal branches and known RAM/fixed helpers; propagated new local entries into bank11 callers. Fixed C0E4/C1EF and downstream local helper meanings remain explicitly unresolved pending their bodies. Fresh byte-identical verify/read and source-relative metadataerrors0. Pending16Code scaffolds4408bytes; exactnext92889/8AC9 after1Unknownbyte. Historical dependencies85/85checked0pending. Same phase active/incomplete; no successful handoff.

Worker K batch040:13meaningful units913bytes cover six complete bank11 scaffolds92889..93803 excluding preservedUnknown93194..93195. Explained state6F distance/packed selection, state72/74 live/death/completion, state75 award, state98/70/73 motion and timer tails. Documented literal missing packed-field store at8CA1; no repair or structural claim. Fresh byte-identical verify/read and source-relative metadataerrors0. Ktotal21units1911bytes. Pending10Code scaffolds3495bytes; exactnext93803/8E5B,no partial. Historical dependencies85/85checked0pending. Same phase active/incomplete.

Worker K batch041:28meaningful units1791bytes complete bank11 scaffold93803..95594/CPU8E5B..955A. Explained state71/76family activation/contact/detour/completion, three pattern variants, allocation/component construction, stateC4/C5 motion/removal, coordinate probes and direction helpers. Named all branch targets and understood RAM fields; propagated local entries into reviewed bank11 callers. Literal branch/subtraction behavior retained. Fresh byte-identical verify/read and source-relative metadataerrors0. Ktotal49units3702bytes. Pending9Code1704bytes,next95594/955A,no partial;85historical dependencies checked. Same phase active/incomplete.

Worker K batches042..043:73meaningful units1704bytes complete all remaining bank11 Code scaffolds95594..97298/CPU955A..9C02. Explained completion latch/shapes, six-step path including wrapped underflow, flags, response/state setters, division, shared coordinates/companion actor, pattern initialization and shared interval/count gates. Corrected K unit boundary: displacement return940D, projectile dispatcher940E as original veneer; ROM unchanged. Ktotal122units5406bytes. Fresh byte-identical verify and source-relative metadataerrors0. Pending Code scaffold0/0bytes; current phase remains active/incomplete while required caller uncertainty prose integration and full current-source scope assessment finish.85historical dependencies remain checked. No successful handoff/final audit yet.

Worker K completion gate: all current-source readability COMPLETE, distinct from classification and final editorial acceptance. K122meaningful units5406bytes completed the final17bank11 Code scaffolds. All2162scaffolds and2193units reviewed;0pending. FullROM partition/source boundaries checked, Kphysical/CPU/unit anchors checked, bank11 numeric localcall operands0,35caller paragraphs integrated and5probe-related descriptions corrected to literal predicate polarity.85scoped dependencies checked0pending. No map changes/new structural leads. Fresh verify after final source corrections byte_identical=true; reading-after refreshed; metadataerrors0. Classification92.26%/20302Unknown unchanged. Successful phase handoff:analysis/round-002-commentary-handoff.md; scopeassessment-k JSON/MD and coverage-k authoritative. Final editorial_audit remains pending; no audit/archive/publication/toolkit edits. K stopped writing after this handoff.


Coordinator ROUND002 commentary gate accepted: independently checked all2162 scaffold records/2193units,262160-byte partition,source-relative locations,85dependency checks0pending and example routine mechanics; fresh supported verify byte_identical=true/read refreshed. Evidence analysis/round-002-coordinator-commentary-check.json. Classification remains92.26%/20302Unknown. K stopped writing. Current phase REDIRECT, separate sequential worker round2_redirect, input checkpoint round-002-disassembly-final; new research attempt analysis/redirect/round-002-r1/. No terminal decision or final editorial audit yet.

ROUND002 REDIRECT active (round-002-r1): consumed accepted commentary handoff/current checkpoint round-002-disassembly-final. Protected identities in analysis/redirect/round-002-r1/protected-before.json. Fixed-source call scan and life-state finite tables are productive structural continuations; evidence/findings and exact active barriers saved in that directory. Candidates only;0 bytes accepted, no map/source/state changes. Continuing connected bank6 and independent residual data inspection; not a terminal assessment or phase handoff.

ROUND002 redirect bank audit: preliminary per-bank graph candidates are preserved as *-pre-bank-audit.json and superseded pending correction. bank-correction.md localizes fixed-wrapper remapped-lower-call hazard (C045 maps sound0/1; direct mapper API cannot preserve callerlowercontext). No graph proposal accepted; map/source/state protected. Independent consumer-anchored movement/motion/path data findings saved in data-contracts.json. Active frontier remains corrected graph/bank6 contracts and residualData patterns.

ROUND002 redirect corrected contracts established: bank6A003/A00F/A012 ordinary screen/helper returns, fixedC571 finite0..4 return domain; bank7 seven-stage update/init dispatch, CHR-only and bank5 override mapping, RAM0481..04C0 attribute packing under row0..7. Exact evidence in round-002-r1 findings.md, mapper/bank frontiers and graphs. New localized bank13 relative-record write ambiguity (F0 supplied asROM) retained, not generalized to Data rejection. Candidate audits still unaccepted; now accumulating data format/fallback proposals and corrected bank7 graph before combined handoff.

ROUND002 redirect independent formats: player-animation-contracts.json validates three37-word frame-field tables, all37 FF sprite streams and corresponding per-frame shape lengths; follows38-entry action/common RTS table in same graph. Narrow bank5 physical11-byte prefix and16-byte palette, and bank6 physicalDE31..E00F OAM-pattern fallback separated from code-like prefix. Active independent whole-table check combines197parallel fixed fields with bank8 five/four-byte shape origins and74/25 record bounds. Counts/proposals remain unaccepted; findings record exact physical-versusCPU addresses.

ROUND002 redirect exact shape197/74/25contract complete. Expanded five37-word/two37-byte animation field comparison completed with524positive consumed bytes and0Code overlap. Concrete new collision control proof at bank13CPU9614/9615 (ten guarded RTS targets) and corrected FA8E impossible literalBNE fallthrough are in control-corrections.json. Followingall10collisionhandlers in same graph, plus bank7 identified finitephase tables. Current candidate audits are superseded by subsequent graph work until refreshed; acceptance remains0 and all protectedfiles unchanged.

ROUND002 redirect working combinedpackage16006proposednewbytes/0accepted, actualUnknown20302 unchanged. Depth0/2shared collisionRTS and finite returnproof closure corrected; concrete bank6RAM0481..04C0 attributewrites proven. New complete184-slot fixedAPI structural extent avenue in fixed-api-table-contract.json, targetbody graph active; earlier graph frozen before expansion. Four tiny completecoordinate/recordpointer helpers under mechanicalABI/clone review. No terminalclaim; current activefrontier is connected fixedAPI target/boundary audit then proposal/ledger/history refresh. Data candidateaudit0Codeoverlap/0directcontroltargets. Protectedfiles unchanged.

ROUND002 redirect full184targetboundaryaudit completebad0, four finitefixedIRQ/transfer tablefamilies followed; latest8205candidateCodebytes/3639instructions,0instruction/Data/partialoverlap. Playercollision/animation ordinaryreturnchain closed with exactdepth0/2contracts. Activefinalchecks: actualfixedmapper selectmode/slotvalues, positive residualpalette/HUD/actorbase/timing/fillData, finiteeight-wordOAMtable, thencombinedproposal/329-rangeledger/history/deferrals. Old17500workingpackage notfinalupdatedtotal. Allacceptance0; protectedfiles unchanged. ExistingIRQRTI/resetTXS have acceptedprovenance and are notnewcandidateinstructions.

ROUND002 redirect mapperaudit/OAM8-target/actor-template/residualData checks COMPLETE. Fifteenliteralmapperselector definitions plusF758conditionalX0..7 documented,0unreviewed; safe perbankreturnseed frozen. Workingproposal19004(8242Code/10762Data),0accepted,conditional1298; protectedmapactualUnknown20302 remains. Bank5 local C5CA ordinarycall counterexample scoped (C607 alternatelegalentry is not impossibility proof), and independentpureRAM60+34byte helpercontracts discovered outside rejectedcalls, nowpackaging. Currentfrontier is addthose94StaticCodebytes thenexactaudit/329rangeledger/stablelead-pass/history andcombinedhandoff. No currenttoolrunning; no terminalexhaustionclaim.

ROUND002 redirect research scope CLOSED for combineddisassemblybatch; no terminalassessment. Finalproposal19098newbytes =8336StaticCode +10762StaticData,0accepted, conditional1204Unknown. Exactcurrent329-rangeledger refreshed andpartitionvalidated against unchangedinternalmap:20302actualUnknown. Remainingonlyhistory/deferral/handoff tasks: finalnativeview/Data-boundary/protectedhash validation, sixready-lead/passupdates(two newIDs), gameplaychecklist reuse summary, toolkitpasssummary andcompactcombinedhandoff. Bank5scoped94bytepureRAM helpers promotedindependently oflocalized C5CArejection; residual1048bank5bytes retainactualcode/data/bankambiguity. Allidentifiedpromising connected/independent avenues reviewed; no newexperimentrunning or pending.

ROUND002 REDIRECT HANDOFF COMPLETE: analysis/redirect/round-002-r1/handoff.md packages19098proposednewbytes (8336StaticCode/10762StaticData),0accepted; actual20302Unknown/329ranges unchanged, conditional1204. Allsixready leads/passhistory, avenue deferrals, gameplayreuse, toolkit summary and protectedhash/nativeview/Data/ledger checks complete. Consumedround-002-disassembly-final andacceptedcurrentcommentary; producedround-002-r1,L011/L012, no map/statecheckpoint. No terminalassessment or ROMcompletion claim. Worker stoppedwriting; sequential independentdisassembly maybegin aftercoordinator review. Allprimary/history artifacts remainlocal andpreserved.


Coordinator accepted ROUND002 redirect combined-research handoff after inspecting handoff.md, avenue-assessment.json, gameplay-review.json and decisive contract summaries. Exact329-range ledger partition independently checked; proposals remain unaccepted pending structural adjudication. Research worker stopped writing. Current phase ROUND003 DISASSEMBLY, separate worker round3_disassembly; input round-002-disassembly-final, six ready leads L001/L002/L004/L008/L011/L012, proposed19098bytes. No terminal assessment; current-source commentary verified before this pass and must be preserved for next round.

## ROUND003 DISASSEMBLY — completed single pass, ROM incomplete

Input round-002-disassembly-final; output round-003-disassembly-final. Independently adjudicated all six ready leads L001/L002/L004/L008/L011/L012: accepted19098 formerlyUnknown bytes (8336StaticCode/10762StaticData), rejected0 proposalbytes. Current map260956classified,1204Unknown in84ranges. No new Observed or naturalreachability claim. Primary role/boundary/bank/frame rationale: analysis/round-003-disassembly/adjudication.md; exact per-lead outcomes/outcomes.json,295Data subclaim review/data-review.json and3693fresh instruction decodes/fresh-decodes.json with220run boundaries/code-runs.json. Kept IRQhardware/savedframes separatefrom ordinary/syntheticRTS, frozen perbankreturnseed, actualmapperrestore andlocalbank5helper boundaries. No supersededgraph used.

ROM classification: 99.54% (260956 / 262160 bytes classified; 1204 bytes unknown).

| Physical PRG bank | Code | Data | Classified | Unknown |
|---|---:|---:|---:|---:|
|0|2237|5954|8191|1|
|1|0|8182|8182|10|
|2|0|8192|8192|0|
|3|0|8192|8192|0|
|4|0|8192|8192|0|
|5|352|6792|7144|1048|
|6|4836|3355|8191|1|
|7|6538|1650|8188|4|
|8|3790|4372|8162|30|
|9|6509|1629|8138|54|
|10|6841|1341|8182|10|
|11|7139|1050|8189|3|
|12|521|7671|8192|0|
|13|4499|3685|8184|8|
|14|7438|732|8170|22|
|15|5916|2263|8179|13|

Header16/16; CHR131072/131072 classified. Observed Code provenance unchanged: 14440 bytes; newly Observed0. Structural classification is separate from execution coverage and source readability.

Per-lead acceptedCode/Data/newtotal: L001 0/3066/3066; L002 2130/14/2144; L004 2640/3754/6394; L008 94/27/121; L011 2301/1718/4019; L012 1171/2183/3354. All verified for these exact structural subsets; earlier history/rejections retained in leads.json. Remaining mixedbank5context1048bytes, bank8variant27+isolatedjump3, bank13ROMindirectwrite8 and sparse sound/opcode/separatorbytes retain Unknown. No terminalexhaustion decision.

Reused23-record scenario checklist and successful sevenstage/helicopter/terminal/WHITEHOUSEcredits routes; current static roles pose no concrete useful remainingruntime discriminator, so no startup repeat/arbitraryRAM sweep/newruntime. Naturalboss/fullcredits completion separate. See analysis/round-003-disassembly/gameplay-review.json.

map.validate true; configuredbinary generated fresh checkpoint and verified byte_identical:true. State/export/savedmap are byte-identical, mapSHA256 4f2424c84e630ea624df17aa8dfa32fbdd5cb1422ca3a095e04a5b9354b22f12. Earlier regions preserved exactly; ROM/configuredbinary/currentcommentary source unchanged. Verification analysis/round-003-disassembly-final-verification.json; preservation-checks.json and exact unknown-complement.json in this phase directory.

Toolkit report analysis/toolkit-issues.md: NES-001 confirmed documentation/API mismatch, NES-002 grounded indexedRAMwrite missingfeature;0suspectedbugs,0blockers, no newissues. No toolkit edits/buildsubstitution, cleanup/deletion/archive/publication. Current-source readability must refresh after preserving the entire prior commentary directory; previous review credit remains valid for unchanged units, but all new classification needs commentary. Final editorialaudit deferred. Compacthandoff analysis/round-003-disassembly-handoff.md.


Coordinator ROUND003 disassembly boundary accepted: ROM classification: 99.54% (260956 / 262160 bytes classified; 1204 bytes unknown). Explicitly printed before commentary. Internal/current/checkpoint maps identical, fresh saved verification byte_identical=true; independent count/identity record analysis/round-003-coordinator-boundary-verification.json. Worker stopped writing. Current phase ROUND003 COMMENTARY, separate round3_commentary worker; preserve entire prior commentary before new baseline/carry. No terminal decision or final editorial audit.


## ROUND003 commentary baseline and exact carry

ROUND003 COMMENTARY active and incomplete. Exact carry: 1833 reviewed records; pending 540 records (220 Code runs / 8336 bytes, 236 Data regions / 10762 bytes, 84 Unknown intervals / 1204 bytes). Prior 26-file commentary tree preserved with SHA inventory equality at analysis/round-003/commentary-round002-preserved. Fresh supported prepare and post-carry verify both byte-identical. New code/data, changed Unknown boundaries, and scoped affected dependency review remain. Exact pending inventory: analysis/round-003/commentary-pending.json. Final editorial audit remains pending; no redirect handoff.


### ROUND003 commentary batches001–003

ROUND003 commentary remains active and incomplete. Batches001/002 explain all84Unknown intervals (1204bytes) and both independent bank5RAM helpers (94Codebytes), with named count/frame/direction/X fields and exact wrap behavior. Batch003 reviews all236newData scaffolds (10762bytes), checks local hashes for all295accepted subclaims, and adds explicit palette, PPU/RAM/CPU pointer, RTS-frame, shape, motion, animation and bounded generic residual explanations. Fresh post-batch verify/read results are in commentary/verification.json and reading-after.asm. Current pending: 218 Code records / 8242bytes; all current newCode beyond the two bank5helpers and scoped carried dependency contradictions remain. No final audit or redirect handoff.


### ROUND003 commentary batch004

Batch004 completes 21 fixed helper Code runs with 27 routine contracts, including asymmetric DB85 coordinate-step tests, second-shape disable, update-bit clears, probe/path helpers, interval release, 16-bit division and palette helpers. Fresh verify/read pass byte-identical after correcting two operand aliases during this batch. Current pending 197 Code runs / 7833 bytes; scoped carried dependencies also remain. Next: fixed API slots by named target contracts. Phase remains incomplete; no audit or redirect handoff.


### ROUND003 commentary batches005/006 and dependency inventory

Batches005/006 and scoped dependency upkeep: all 94 newly accepted fixed API slots now have individual named shared-helper units across 32 runs (282 bytes). The C09F directional Y-step/asymmetric X/Y-limit contract and C0E4/C1EF second-contact-shape disable/enable are propagated through 25 paragraph edits and 21 call operand substitutions. Corrected stage palette, five-phase RTS, eight-OAM RTS, thirty-path-pointer and copied path-flag claims. Current pending 165 Code scaffolds / 7551 bytes. One pending 113-byte palette/fade scaffold retains exact 23-byte reviewed queue suffix; its first90bytes remain. Scoped carried-dependency inventory has139candidates,3reviewed/136pending; these may retain sound uncertainty after checking. Fresh verify/read pass. All source is local; final audit remains pending, phase incomplete.


### ROUND003 commentary continuation A — incomplete

Sequential capacity checkpoint A saved; ROUND003 COMMENTARY remains incomplete and must resume before REDIRECT. Current2373scaffolds:2208reviewed/165pending,2738units. Pending165Code scaffolds7551bytes include one23-byte reviewed suffix;7528Codebytes remain unread.136/139scoped dependency candidates remain pending. All84Unknown notes and236newData scaffolds are reviewed. Final fresh verify byte-identical and reading-after refreshed; physical partition and source-relative metadata checks pass with0errors. Exact remaining ranges/partial credit/dependency frontier: analysis/round-003/commentary-continuation-latest.md, commentary-pending.json, commentary-pending-by-bank-a.json and commentary-dependency-candidates-a.json. No successful commentary handoff or final editorial audit. Writer A stopped writing; next sequential commentary worker only.


### ROUND003 commentary batches007–009 (writer B; phase incomplete)

Reviewed all 13 remaining bank15 Code scaffolds (941 bytes including the previously reviewed 23-byte queue suffix), plus three bank8 action-copy continuations (132 bytes). Added 25 net meaningful units; current 2224 reviewed /149 pending scaffolds, 2763 units. Pending Code is 6478 bytes, all unread; no remaining partial scaffold credit. Corrected fixed-bank response/fade/OAM contracts, player-X comparison naming, big-endian division aliases, interleaved actor templates and terrain-X thresholds. Checked 17 additional carried dependency candidates (DEP124..139 including already-reviewed DEP138, plus DEP066/070): 20 total reviewed /119 pending. C018 is unsigned multiplication, not a table reader; corrected all source operands/API and bank8 multiplier aliases, and bank10 distance-times-three/divide-by-ten account. All batches freshly verify byte-identical and reading-after is refreshed. Classification unchanged 99.54%, 1204 Unknown bytes. Final editorial audit remains pending; continue remaining code and dependency checks before any successful handoff.


### ROUND003 commentary batches010–011 (writer B; phase incomplete)

Completed the bank8 249-byte action application/palette helper and 197-byte stage-depth/explicit-coordinate helpers, with six meaningful units. Completed seven auxiliary/contact continuations (174 bytes), with state/health lookup, literal motion-table byte replication, collision-direction gates, mirrored-box completion and first contact-effect placement. Reviewed scope is now2233/2373 scaffolds,2776units;140Code scaffolds/5858bytes remain unread. Scoped carried dependency frontier remains20reviewed/119pending; old source review credit remains valid. All batches freshly verify byte-identical and reading-after is refreshed. Concrete special cases remain explicit: stage6 CMP #0/BCC cannot take the branch; the action-motion helper copies one table byte twice, and its INY does not read a second field. The actor/player X API and its bank9 caller now consistently use player naming. No structural changes, audit or successful handoff.


### ROUND003 commentary continuation B — incomplete

Batches007–013 complete ALL remaining new bank8/15 Code:54scaffolds2152bytes, including A23-byte suffix (2129new unread bytes);68net meaningful units added. Current2373scaffolds2262reviewed/111pending,2806units. Remaining111Code ranges5399bytes in banks6/7/13/14; no partial scaffolds. Scoped dependency inventory36reviewed/103pending after33additional checks. Fresh byte-identical verify/read and source-relative metadata/partition checks pass0errors. Classification99.54%,1204Unknown unchanged. Resume exact remaining commentary scope from analysis/round-003/commentary-continuation-latest.md, commentary-pending.json and commentary-pending-by-bank-b.json; dependencies in commentary-dependency-candidates-a.json. Final audit pending; no successful handoff or redirect. Writer B stopped writing for next sequential context.


### ROUND003 commentary batch014 (writer C; phase incomplete)

Completed three bank14 raster IRQ continuation scaffolds (253bytes), with five units explaining fixed delays, literal PPU internal-address reads, player nametable selection, two-phase control, shared guard and saved interrupt-frame restoration. Checked DEP109..112 against the classified stage/phase/mode RTS tables and downstream code; removed stale unclassified-table/continuation claims and used explicit CPU aliases for table operands. Current2265reviewed/108pending scaffolds,2811units;5146Codebytes and99dependency candidates remain. Fresh verify byte-identical and reading-after refreshed. Initial link check caught use of physical data labels as CPU aliases; corrected with distinct fixed-bank CPU constants before passing verification. No map/ROM/toolkit changes or final audit; continue full readability scope.


### ROUND003 commentary batch015 (writer C; phase incomplete)

Completed14bank14Code scaffolds314bytes with17units: three-phase vertical tile/attribute upload, HUD label/counter/timer/lives/actor-health tails, contact flag consumption and damage returns, bank6 wrappers, NMI transfer joins and literal CHR selection. Kept Unknown bank6 return boundaries explicit and distinguished lower-bank preservation from fixed-bank execution. DEP108/115/119 now reviewed after concrete consumer checks. Current2279reviewed/94pending scaffolds,2828units;4832unreadCodebytes/96dependencies remain. Fresh supported verify is byte-identical; reading-after refreshed, source-relative anchors/partition pass0errors. Bank14 life/pause graph remains; banks6/7/13 remain in full. No final audit or successful handoff.


### ROUND003 commentary batch016 (writer C; phase incomplete)

Completed all12remainingbank14Code scaffolds441bytes with24units across pause/resume, life-result RTS dispatch, first/second-player availability/counter/switch paths, saved-life choice, timer-health fallback and44-byte player restores. Two further dependencies DEP099/104 checked. Current2291reviewed/82pending,2852units;4391unreadCodebytes/94dependencies. Bank14 newCode is complete; banks6(24/2130bytes),7(29/1090),13(29/1171) remain. Fresh byte-identical verify/read pass and metadata anchors/partition0errors. Initial checks caught a missing second-entry symbol and incorrect C238 alias (C235 would reset lives); corrected to the actual bank6 screen-initialization call preserving lives before passing. No final audit or successful handoff.


### ROUND003 commentary batch017 (writer C; phase incomplete)

Completed7bank6Code scaffolds509bytes with16units: life/decision API jumps, status/continue screens with actual input0/1 result, PPU tile strings, session-selected player marker, selector OAM, pattern-match initialization and52-byte ending OAM. Explicit bank6 requirement propagated to fixed wrappers. DEP009/016 checked: corrected false repeated-register CHR claim (Y advances, writes registers0/1) and obsolete unclassified status-template claim. Current2298reviewed/75pending,2868units;3882Codebytes/92dependencies remain. Remaining banks6(17/1621bytes),7(29/1090),13(29/1171), no partial scaffold credit. Fresh byte-identical verify/read and source-relative anchors/partition0errors. Initial verify caught AB58 palette-record entry mistaken for AB49 command entry; corrected exact alias before pass. Final audit pending; same commentary phase continues.


### ROUND003 commentary batch018 / continuation C — incomplete

Completed15bank6Code scaffolds474bytes with21units across palette/screen-sequence completion, ending runtime reset and nine actors, terminal timer/input/sound-cycle, descriptor source/destination transfer pairs, alternate sprite stride and coordinate carry. Updated affected carried summaries to match corrected source. Current2313reviewed/60pending scaffolds,2889units;3408unreadCodebytes and92dependency candidates remain. Bank6 has two wholly unread large graphs(585bytes A46E..A6B7 and562bytes B76E..B9A0); banks7/13 remain29scaffolds each1090/1171bytes. No partial credit. Fresh supported verify byte-identical and reading-after refreshed; full physical partition/source-relative metadata0errors. Sequential C capacity checkpoint saved, same COMMENTARY phase resumes; no final audit or successful handoff.

## ROUND003 D batch019 — incomplete; resume commentary

Reviewed bank6 physical50302..50887 (585bytes) in11meaningful units: intro/options, input-pattern side effects, acceptance, OAM marker toggle and16-step blink, controller2 selection index, briefing state machine, character submission/comma/end-line/pending tails. Branch and RAM roles named; gameplay meanings of input patterns remain unresolved. Fresh supported verify is byte-identical; reading copy refreshed. Current2314/2373scaffolds reviewed,2900units;59Code scaffolds/2823bytes unread (bank6 one562byte graph;bank7 29/1090;bank13 29/1171). No partial scaffolds. Dependencies47reviewed/92pending of139. No successful handoff or final editorial audit.

## ROUND003 D batch020 — incomplete; resume commentary

Completed the full bank6 player-choice graph physical55166..55728 (562bytes),10meaningful units covering screen loop, conditional redraw, held-input choices, one-time confirmation, four source-pair redraw families and descriptor selector. Source-pair data consumer identifies the first48bytes at55728 as twelve two-origin records; remaining56bytes retain uncertainty. Fresh supported verify byte-identical/read refreshed; partition/anchors0errors. Current2315/2373reviewed,2910units;58Code scaffolds/2261bytes remain wholly unread (bank7 29/1090;bank13 29/1171). No partial coverage. Dependency47reviewed/92pending. No successful handoff/audit.

## ROUND003 D batch021 — incomplete; resume commentary

Completed all29remaining bank7Code scaffolds/1090bytes with39units: scroll request acknowledgement, secondary/vertical timer row pairs, four phase-layer continuations, initialization families, reverse-row publication, alternate palette/CHR setup, factory opening rows/palette tail and four-byte OAM template tail. Explicit IRQ-side wait contracts retained. Corrected D-local unit entries placed on preceding RTS (including B6 option entryA4A6); branch return aliases and routine entry aliases now separate. C225 identified as ApiQueueBackgroundPaletteBuffer in the palette copy tail. Fresh supported verify byte-identical/read refreshed; partition/anchors0errors. Current2344/2373reviewed,2949units;29Code scaffolds/1171bytes remain entirely unread, allbank13. No partialscaffolds. Dependencies47reviewed/92pending. No successful handoff/audit.

## ROUND003 D batches022–023 — incomplete; dependency checks remain

Completed all29remaining bank13Code scaffolds/1171bytes:022action/animation/terrain-timer/movement-bound tails21units768bytes,023collision-coordinate/axis-response/OAM tails14units403bytes. Collision relations explicitly use high nibbleY/low nibbleX (before2,same1,after0); quadrant and collision type remain distinct. Honest Unknown boundary after ROM/mapper-directed record write preserved. Current2373/2373scaffolds reviewed,2984units,0unreadCode/0partial. This is NOT phase completion:92of139dependencycandidates remain pending (47reviewed), plus concrete scope assessment/caller-name integration. Fresh supported verify byte-identical/read refreshed,partition/anchors0errors. Finaleditorialaudit pending; no successfulhandoff.

## ROUND003 D batch024 — incomplete;67dependency checks remain

Checked25specific dependencies (DEP001,003..008,010..015,017..028), each with preserved physical anchors and individual findings. Corrected modebit7 AND7F clear vs prior retain claim; removed obsolete Unknown completion/timer/ending boundary claims; named bank6 table CPU aliases and fixed APIs; identified seven eight-byte stage map records and retained locally unused/external-bank uncertainty. Input-pattern/CHR-loop/C235vsC238/AB49vsAB58 distinctions preserved. Fresh supported verify byte-identical/read refreshed,partition/anchors0errors. All2373scaffolds/2984units reviewed;0Code/partial. Dependency72reviewed/67pending of139. No successfulhandoff/audit.

## ROUND003 D batch025 — incomplete;39dependency checks remain

Checked28specific bank7dependencies DEP029..039,041..057 with individual evidence/findings. Classified RTS/CHR/row/palette/OAM data consumers now use explicit CPU aliases; obsolete unclassified-continuation disclaimers removed. Retained unchecked selector domains, lower-bank99A6identity uncertainty and auxiliary-table purpose uncertainty. Fresh supported verify byte-identical/read refreshed;partition/anchors0errors. All2373scaffolds/2984units reviewed,0Code/partial. Dependencies100reviewed/39pending of139. No successfulhandoff/audit.

## ROUND003 D batch026 — incomplete;16dependency checks remain

Checked23actor/player dependencies DEP076..098 individually. Known fixedterrain helpers now have descriptive aliases and B10caller guards; B9zero-step viewport result returns through an already classified RTS (removed false Unknown claim). Preserved four genuine actor-boundary Unknowns and unresolved enemy/scene/visual identities. Named thirteen animation/action/movement/collision table CPU views; obsolete terrain-timer/completion/movement-join gaps corrected. Fresh supported verify byte-identical/read refreshed,partition/anchors0errors. All2373scaffolds/2984units reviewed;0Code/partial. Dependencies123reviewed/16pending of139. No successfulhandoff/audit.

## ROUND003 D batch027 — final dependency batch; scope gate still active

Checked final16dependencies DEP100..103,105..107,113..114,116..118,120..123. All139candidates reviewed with individual outcomes. Corrected D2E5sound-request table role, D95964HUDtemplate/48awardoffset/mixedamount layout, and DCC3exactzero classified health-zero store. Fixed false bank7Data match for C3ACA00Ccall using explicit prior bank6map. C245now named stage timer/position restart (skips C242spritepalette queue), distinct C238screen-init/C235life-init. E6A9/E705/E71Fconsumer views retain unresolved selectorbounds rather than invent disjoint extents. Fresh supported verify byte-identical/read refreshed;partition/anchors0errors. All2373scaffolds/2984units reviewed,0Code/partial/dependencies. Caller-name/fullscope assessment remains before successful handoff. No finaleditorialaudit.

## ROUND003 D batches028–029 — current-source readability complete

The fullcurrent-source gate passes:2373/2373scaffolds reviewed,3000/3000units reviewed,139/139dependencies checked,0pending/partial. Reviewed-unit union covers all56616classifiedCodebytes with0uncovered intervals. Batch028corrects ten further stale API/row-return prose claims and integrates known fixed/localcaller names; batch029adds16exact entry/tail-head records with purpose notes, retaining prior valid units. Current bankzero uncertainty overview now distinguishes classified mechanics/tables from genuine boundary ambiguity. Additional bounds/context exclusions and qualitative assessment are recorded in analysis/round-003/commentary-scope-assessment-final.json.

Fresh supported verify byte-identical and reading-after refreshed after finalsourceedits. Map equals frozenbaseline; state equals frozenphase1state; ROMpartition/unitbounds/sourceanchors pass0errors. Classification remains99.54%,1204Unknown; gameplay/external-bank/Unknown-boundary limitations stay explicit. No finaleditorialaudit (stillpending), no map/state/toolkit/ROMchanges, no publication/archive/cleanup/deletion. Successful current-source handoff: analysis/round-003-commentary-handoff.md. WriterDstoppedediting after this record.


Coordinator ROUND003 commentary gate accepted after inspecting successful handoff, final qualitative assessment and concrete source examples; independent2373scaffold/3000unit metadata,262160partition,56616Code union0gaps and139dependency0pending checks pass. Fresh supported verification byte_identical=true and read refreshed. D stopped writing. Current phase ROUND003 REDIRECT, separate round3_redirect worker, input round-003-disassembly-final;1204Unknown. New attempt analysis/redirect/round-003-r1/. No terminal decision or final editorial audit yet.


ROUND003 REDIRECT round-003-r1 active: consumed completed current-source commentary gate, checkpoint round-003-disassembly-final (1204 Unknown/84ranges). Unique research directory and protected hashes created. Scoped bank5 RAM-only tails and exact bank13 even-select/odd-data contract under investigation; no classification/source/state changes, no runtime replay. Configured binary preserved; free disk6.8GiB, no cleanup/deletion. See analysis/redirect/round-003-r1/plan.md and findings.md.

ROUND003 REDIRECT round-003-r1 complete as ordinary combinedresearch handoff: proposed1090 newbytes (1020StaticCode/70StaticData), accepted0; actual1204Unknown/84ranges unchanged, conditional114/11. Corrected nativecallee/partialmotion/recordABI, typedframe/flags, bank13MMC3 parity/mode and repeatedboundary evidence in analysis/redirect/round-003-r1/handoff.md. Updated stable L001/L004/L008/L012, no new IDs; all histories retained. Current protected identities match; configuredreleasebinary unchanged; no runtime/Observed/source/map/state/ROM/toolkit changes, no cleanup/deletion/publication or new checkpoint. No new toolkit issues; NES-001/NES-002 nonblocking. No productive experiment remains pending. Worker stopped writing after handoff.


Coordinator ROUND003 redirect combined-batch gate: inspected handoff, primary bank5/mapper/boundary evidence, avenue assessment and package/residual ledger. 1090proposed=1020Code+70Data;0accepted, actual1204Unknown. Research worker stopped; no pending productive avenue in handoff and no terminal decision. Protected six identities verified unchanged at boundary. Current phase ROUND004 DISASSEMBLY; separate round4_disassembly worker independently adjudicates ready L001/L004/L008/L012 from round-003-r1. Input round-003-disassembly-final. Redirect handoff selected for preservation; no archive executed.


## ROUND004 disassembly — single pass complete; ROM incomplete

ROM classification: 99.96% (262046 / 262160 bytes classified; 114 bytes unknown).

| Physical PRG bank | Code | Data | Classified | Unknown |
|---|---:|---:|---:|---:|
|0|2237|5954|8191|1|
|1|0|8182|8182|10|
|2|0|8192|8192|0|
|3|0|8192|8192|0|
|4|0|8192|8192|0|
|5|1302|6822|8124|68|
|6|4836|3356|8192|0|
|7|6538|1651|8189|3|
|8|3790|4372|8162|30|
|9|6551|1641|8192|0|
|10|6841|1351|8192|0|
|11|7141|1050|8191|1|
|12|521|7671|8192|0|
|13|4507|3685|8192|0|
|14|7453|738|8191|1|
|15|5919|2273|8192|0|

Header16/16; CHR131072/131072 classified. Observed Code provenance unchanged: 14440 bytes; newly Observed0. Structural classification is separate from execution coverage and source readability.

Accepted1090previouslyUnknownbytes:1020StaticCode/70StaticData; L008950/30,L00462/0,L0128/0,L0010/40. No prior retyping or newObserved/natural-use evidence. Independent narrow mechanical/partial-prefix, actual callee/MMC3, typedframe and repeatedlayout adjudication: analysis/round-004-disassembly/adjudication.md. All444instructions freshlydecoded and26Data additions checked;44tailJMP/RTS layouts checked. Existing23scenarios/sevenlevels/helicopter/terminal/credits reviewed, no repetitive runtime replay. Exact114Unknown/11ranges in unknown-complement.json; actual residual scopes retained in redirect/round-003-r1/residual-review.json, no terminal exhaustion decision.

All262046classifiedbytes have structural evidence,100%annotationcoverage; Static247606/Observed14440. Structural coverage separate from execution/readability. Fresh round-004-disassembly-final mapvalid/byte_identical=true; map/state/checkpoint SHA256 1db9d5c4f74b94f28be6f6627f33831e9924b1679dd73058e5ce48a8d28fa140. Priorregions,ROM,binary and entirecommentary unchanged; sourceSHA dee01ad895c0ba99837a630fc53ad01b9310edd5a30f324ce18afba24e8d0e20. Preserve previouscommentarydirectory before nextbaseline/carry. Handoff analysis/round-004-disassembly-handoff.md; fullcurrent-source commentary then redirect next; finaleditorialaudit pending.

Toolkit: no newtoolkitissues; NES-001confirmed documentation/API mismatch,NES-002missingfeature,0suspectedbugs/0blockers; analysis/toolkit-issues.md. No toolkitedit/buildreplacement,live server,archive,cleanup/deletion/publication. Worker stoppedwriting afterphasehandoff.


Coordinator ROUND004 boundary accepted: independent union262046/262160=99.96%,114Unknown; maps/current,state,checkpoint SHA1db9d5c4f74b94f28be6f6627f33831e9924b1679dd73058e5ce48a8d28fa140 identical, saved freshverify byteidentical. User-facing percentage printed beforecommentary. Disassembly stopped. Currentphase ROUND004 COMMENTARY with separate round4_commentary worker; must preserve wholeRound003commentary beforeprepare, reuse soundreview and complete changedsource/dependencies/fullscope gate. No finaleditorialaudit or terminaldecision. Disassembly finalMarkdown selectedforpreservation.


## ROUND004 commentary — current-source readability complete

Resumed existing prepared/carry/batch work and completed integration. All2386scaffolds/3064units reviewed,0pending/partial;57636Codebytes covered by reviewed-unit union,0gaps. 24concrete dependency findings/103adjoining boundaries checked;139priorvalidreviews preserved. Current maps/state equal frozencommentaryreferences. Fresh supportedverify byte_identical=true and readingcopy refreshed after finaledits. Unknown114bytes/11intervals explicitlyexplained; ROMclassification remains99.96%, notcomplete. Scope assessment analysis/round-004/commentary-scope-assessment-final.json; successfulhandoff analysis/round-004-commentary-handoff.md. Editorialaudit pending. No newtoolkitissues; NES-001confirmed/NES-002missingfeature nonblocking. No toolkitedit,ROM/map/statechange,archive,cleanup/deletion/publication. Writer stoppedafter handoff.


Coordinator ROUND004 commentary gate accepted after current handoff, qualitative assessment and source-example inspection; independent 2386 scaffolds/3064 units, 57636 Code bytes with zero uncovered/pending and identical current/frozen map/state confirmed. Fresh supported byte-identical verification and refreshed reading copy retained. Worker stopped. Current phase ROUND004 REDIRECT, input round-004-disassembly-final, 114 Unknown bytes. Separate worker required; no terminal decision or final editorial audit. Archive selection contract re-inspected; commentary handoff selected, no archive executed.

## ROUND004 redirect — ready combined research batch

ROM classification unchanged99.96%(262046/262160);114Unknown/11ranges. Proposed27StaticCode/0Data,accepted0; conditional87Unknown/10ranges. ExistingL011 improved with completed bank8actor-health override computation prefix68569..68596, exactnative8662search,032Fhealth/02C3overrideconsumer and independently bounded13typebytes/7stagepointerstructure. ExplicitDatafallthrough means no completecallablehandler/successor/naturaluse claim.364bounded independentmechanicalcases pass;0newObserved/runtimefetches. Initialprototypefixture/expected-output corrections retained. CombinedA00016bank/8target/canonicalboundary,actualcallbackwriters,narrowersuffix,predicate,sparseboundary,sound andcurrent-sourceuncertainty avenues assessed withscoped limits.23scenario/sevenstage/helicopter/terminal/WhiteHousecredits evidence reused;no unchangedgameplayreplay. Handoff analysis/redirect/round-004-r1/handoff.md; exactledger/package/primaryproofs linked. ROM/map/state/source/symbols/configuredbinary identities unchanged;research/leads/progress only,nonewcheckpoint/toolkitedit/archive/cleanup/deletion/publication.4.1GiBfree,~1.4MiBattempt. No newtoolkitissues;existingNES-001/NES-002nonblocking. No productiveinterruption or activeexperiment;notterminalexhaustion. Worker stoppedwriting afterhandoff.


Coordinator ROUND004 redirect gate accepted ordinary combined ready batch after primary health/search/consumer/boundary and avenue assessment inspection; L011 proposes27StaticCode,0accepted,114Unknown unchanged. Protected six identities independently match. Worker stopped. Current phase ROUND005 DISASSEMBLY, input round-004-disassembly-final; independent review required. No terminal decision or final editorial audit; final redirect Markdown selected, no archive executed.


## ROUND005 disassembly — single pass complete; ROM incomplete

ROM classification: 99.97% (262073 / 262160 bytes classified; 87 bytes unknown).

| Physical PRG bank | Code | Data | Classified | Unknown |
|---|---:|---:|---:|---:|
|0|2237|5954|8191|1|
|1|0|8182|8182|10|
|2|0|8192|8192|0|
|3|0|8192|8192|0|
|4|0|8192|8192|0|
|5|1302|6822|8124|68|
|6|4836|3356|8192|0|
|7|6538|1651|8189|3|
|8|3817|4372|8189|3|
|9|6551|1641|8192|0|
|10|6841|1351|8192|0|
|11|7141|1050|8191|1|
|12|521|7671|8192|0|
|13|4507|3685|8192|0|
|14|7453|738|8191|1|
|15|5919|2273|8192|0|

Header16/16; CHR131072/131072 classified. Observed Code provenance unchanged: 14440 bytes; newly Observed0. Structural classification is separate from execution coverage and source readability.

Accepted27previouslyUnknownbytes, all Static Code, L011. No prior retyping or newObserved/natural-use evidence. Exact87Unknown/10ranges in analysis/round-005-disassembly/unknown-complement.json. Independent native search flags/record identity, health producer/override consumer and local type/pointer boundaries reviewed in analysis/round-005-disassembly/adjudication.md. Existing23scenarios/sevenstages/helicopter/terminal/credits reused; no runtime replay.

Fresh round-005-disassembly-final mapvalid/byte_identical=true. Map/state/checkpoint SHA25633b5a1a7c8b5626339ff98029dc260d1be7622766c0cfd7a7b5eacf4aae66c60. All262073classifiedbytes carry evidence (Static247633/Observed14440). Wholecommentary, priorregions,ROM andconfiguredbinary unchanged; no toolkit edits/archive/cleanup/deletion/publication. Current-source commentary then redirect next; no terminal decision or final editorial audit. 4.0GiBfree; attempt1.7MiB before final small records.

Toolkit: no new toolkit issues found; existing NES-001 documentation/API mismatch and NES-002 indexedRAManalysis missingfeature nonblocking;0new/suspectedbugs,0blockers. analysis/toolkit-issues.md. Worker stopped writing at handoff.


Coordinator ROUND005 disassembly boundary accepted: currentmap/state/checkpoint independently hash-identical, fresh savedverification byte_identical=true,262073/262160=99.97%,87Unknown. Exact percentage printed to user beforecommentary. Disassembly stopped. Currentphase ROUND005 COMMENTARY; separateworker must preserve entireRound004commentary beforeprepare, carry soundreview and finish fullcurrent-source gate. Finalaudit/terminaldecision pending; noarchive.

## ROUND005 commentary complete

Current-source readability complete:2386scaffolds/3065units reviewed,0pending,
all57663Codebytes covered. Whole-ROM classification remains262073/262160
(99.97%),87Unknown bytes/10ranges. Prior full commentary preserved intact at
round-005/commentary-round004-preserved before supported fresh prepare;2375 exact
physical/kind/view blocks carried. New27-byte health-override computation has
named operands/branches, finite variant domain, search Z/X ABI and no entry/return/
successor claim; localized dependent source/metadata corrected. Fresh supported
verify byte_identical:true, reading copy refreshed; editorial audit pending.
Scope:round-005/commentary-scope-assessment-final.json. No new toolkit issues;
NES-001/NES-002 nonblocking,0suspectedbugs/0blockers. No ROM/map/state/toolkit
edits, archive, cleanup/deletion or publication. See round-005-commentary-handoff.md.


Coordinator ROUND005 commentary gate accepted: handoff, qualitative scope and concrete prefix/helper/data prose inspected; independently2386scaffolds/3065units reviewed,57663Codebytes covered,0pending/uncovered and frozenmap/state equality. Fresh supported byte-identical verify and reading refreshed. Worker stopped. Currentphase ROUND005 REDIRECT,87Unknown/10ranges,input round-005-disassembly-final. Separateworker next; no terminaldecision/finalaudit/archive. Finalhandoff and preserved progress/uncertainties selected; duplicate instructions explicitly omitted in archive-options.

## ROUND005 redirect complete — ready narrow repeated-gate inference

Input round-005-disassembly-final; classification unchanged262073/262160
(99.97%),87Unknown/10ranges. ReopenedL008 proposes15StaticCode bytes at49152..49167
as truncated repeated sign/low7 gate prefix; accepted0, conditional72Unknown/9.
Fresh configured-binary decode and immutable byte comparison show exact six-
instruction template repeated at two accepted neighboring gate entries, differing
only RAM/JMP operands. Prior demand for completed store or supported successor
was too strong for this fragment role. Known identityData fallthrough and unresolved
A000 successors remain explicit; no safe entry, missing countdown tail, naturaluse
or originalpackaging claim. See analysis/redirect/round-005-r1/gate-prefix-contract.json.
All other identified avenues/current uncertainties reviewed with retrieved primary
artifacts; broad23scenario/sevenstage/credits coverage reused, no unchanged replay.
Exact ledger87/10, map/state/ROM/source/symbols/configuredbinary unchanged. No new
toolkit issues; NES-001/NES-002 nonblocking,0suspectedbugs/0blockers. No toolkit
edits/archive/cleanup/deletion/publication. Ordinary ready handoff, not terminal;
independent disassembly next if coordinator accepts narrow proof. Worker stopped
writing: analysis/redirect/round-005-r1/handoff.md.


Coordinator ROUND005 redirect combined-batch gate inspected handoff, gate-prefix-contract, exact repeated template/native evidence and all-avenue assessment. L008 proposes15StaticCode,0accepted,current87Unknown. Prior successor counterexample localized to safe execution; positive repeated prefix/boundary evidence merits independent disassembly. Worker stopped. Currentphase ROUND006 DISASSEMBLY,input round-005-disassembly-final. No terminaldecision/audit; final researchMarkdown selected; noarchive.


## ROUND006 disassembly — single pass complete; ROM incomplete

ROM classification: 99.97% (262088 / 262160 bytes classified; 72 bytes unknown).

|Physical PRG bank|Code|Data|Classified|Unknown|
|---|---:|---:|---:|---:|
|0|2237|5954|8191|1|
|1|0|8182|8182|10|
|2|0|8192|8192|0|
|3|0|8192|8192|0|
|4|0|8192|8192|0|
|5|1317|6822|8139|53|
|6|4836|3356|8192|0|
|7|6538|1651|8189|3|
|8|3817|4372|8189|3|
|9|6551|1641|8192|0|
|10|6841|1351|8192|0|
|11|7141|1050|8191|1|
|12|521|7671|8192|0|
|13|4507|3685|8192|0|
|14|7453|738|8191|1|
|15|5919|2273|8192|0|

Header16/16 and CHR131072/131072 classified. Accepted15StaticCode/0Data, L008 exact49152..49167 truncated repeated gate; no prior retyping or newlyObserved. All262088classifiedbytes carry evidence; Static247648/Observed14440. Structural classification differs from execution coverage and commentary readability. Exact72Unknown/9ranges: analysis/round-006-disassembly/unknown-complement.json; per-bank/provenance counts in counts.json. Independent fresh decode/pattern/boundary adjudication in adjudication.md. Existing23scenarios/sevenstages/helicopter/terminal/credits reused; no unproductive replay. Connected alternatives reviewed with scoped unresolved contracts; no terminal exhaustion decision.

Supported validate/save/export and fresh round-006-disassembly-final-accepted verification succeeded, byte_identical=true. Currentmap/state/checkpoint SHA256e1ee3c2a1d69f6f1258d573cbb1e0ad666e16ded6f9f8c5e490777dbc0db67c2. Earlier mistakenly invoked round-006-disassembly-final retains unchanged ROUND005 map; preserved and explained in adjudication.md, not the accepted checkpoint. Whole commentary, prior regions, ROM, config and configured binary unchanged; protected-check.json retains identities. Preserve whole current commentary before next preparation. No toolkit edits/archive/cleanup/deletion/publication. Around4GiBfree; small evidence only. Current-source commentary then redirect next, final editorial audit pending.

Toolkit: no new toolkit issues found; existingNES-001 confirmed documentation/API mismatch andNES-002 indexedRAManalysis missingfeature remain nonblocking;0suspectedbugs/0blockers. See analysis/toolkit-issues.md. Worker stops writing at analysis/round-006-disassembly-handoff.md.


Coordinator ROUND006 disassembly acceptedcheckpoint boundary verified: currentmap/state/checkpoint hashidenticale1ee3c2a,byte_identical=true,262088/262160=99.97%,72Unknown. Exactpercentage printed beforecommentary. Premature no-gain checkpoint retained transparently; acceptedinput round-006-disassembly-final-accepted. Worker stopped. Currentphase ROUND006 COMMENTARY; preserve entire priorcommentary beforeprepare, reuse soundreview and complete fullcurrent-source gate. Terminaldecision/finalaudit pending; noarchive.


Coordinator ROUND006 commentary gate accepted after handoff/scope and concrete gate/identity/source inspection; independent2386scaffolds/3066units reviewed,57678Codecovered,0pending/uncovered,262160partition,current/frozenmapstate equality. Fresh byte-identical verification and readingrefresh retained. Worker stopped. Currentphase ROUND006 REDIRECT,input round-006-disassembly-final-accepted,72Unknown/9ranges. Separateworker next; finalaudit/terminaldecision pending,noarchive. FinalMarkdown selected.


## ROUND006 redirect — combined stored-Code fragment proposal

Input round-006-disassembly-final-accepted; current262088/262160classified,72Unknown/9ranges. L008 materially reopened:53proposedStaticCode/0Data,accepted0, conditional19Unknown/6ranges. Four separately adjudicable bank5 roles: encoded emptyRTS head1, repeated global-phase preparation prefix7, four-call/phase-advance prefix14, ten-call/return object31. Primary repeated accepted headers, independent three four-word layouts/common encoded-head relation and accepted object boundaries distinguish stored Code from opaqueData. Prior safe-native C5CA/callback/A000 ABI failures remain exactly scoped; no original relocation/naturalentry/Observed/whole native routine claim. All independent residuals evaluated under same fragment standard and retain localized uncertainty;23scenario/sevenstage/terminal/credits coverage reused. See analysis/redirect/round-006-r1/handoff.md,contracts.json,avenue-assessment.json and current ledger. Protected ROM/map/state/source/symbols/configuredbinary unchanged; research/leads/progress only,~4GiBfree. No new toolkit issues found; existingNES-001/NES-002 nonblocking. No archive/cleanup/deletion/publication. Ordinary ready handoff, not terminal. Worker stops writing at handoff.


Coordinator ROUND006 redirect combinedbatch accepted for independentreview: handoff/contracts/exactheadlayout/native/avenueassessment inspected,53proposedStaticCode in fourclaims1+7+14+31,L008,0accepted/current72Unknown. Narrow storedstructureclaim preserves nativeABIcounterexamples. Worker stopped. Currentphase ROUND007 DISASSEMBLY,input round-006-disassembly-final-accepted. No terminaldecision/audit/archive; finalresearchMarkdown selected.


## ROUND007 disassembly

ROM classification: 99.99% (262141 / 262160 bytes classified; 19 bytes unknown).

ROUND007 independently accepted 53 Static Code bytes in four L008 stored fragments: 48803+1, 48804+7, 48819+14, 48896+31. No Data, retyping or new Observed bytes. Exact unknown complement is 19 bytes/6 ranges. Structural classification remains incomplete. Existing 23 scenarios/seven stages/later Factory/helicopter/terminal/credits reused: no concrete new runtime discriminator. No original relocation, actual entry, natural use, return edges across calls, complete native program or safe continuation inferred.

Separate adjudication: analysis/round-007-disassembly/adjudication.md. Supported native decode/annotate/validate/save/export succeeded; fresh round-007-disassembly-final verifies byte_identical=true. Map SHA256 db38356a967d277f509a5b853281ebe12f0c75d135c09f6c60837e48b5823afd. All prior regions and their commentary preserved exactly; ROM/configured binary/configuration and previous generated source unchanged (protected-check.json). Static247701/Observed14440. Exact per-bank and residual counts: counts.json and unknown-complement.json. No toolkit changes/archive/cleanup/deletion/publication. About4GiB free. No new toolkit issues found; existing NES-001/NES-002 remain nonblocking,0suspectedbugs/0blockers (analysis/toolkit-issues.md). Whole current-source commentary next; worker stops writing at handoff.

|Bank|Code|Data|Classified|Unknown|
|---|---:|---:|---:|---:|
|0|2237|5954|8191|1|
|1|0|8182|8182|10|
|2|0|8192|8192|0|
|3|0|8192|8192|0|
|4|0|8192|8192|0|
|5|1370|6822|8192|0|
|6|4836|3356|8192|0|
|7|6538|1651|8189|3|
|8|3817|4372|8189|3|
|9|6551|1641|8192|0|
|10|6841|1351|8192|0|
|11|7141|1050|8191|1|
|12|521|7671|8192|0|
|13|4507|3685|8192|0|
|14|7453|738|8191|1|
|15|5919|2273|8192|0|

Header16 and CHR131072 remain fully classified.


Coordinator ROUND007 disassembly boundary accepted after handoff/adjudication review, currentmap/state/checkpoint hash equality and saved fresh byte-identicalverify. Exact262141/262160=99.99%,19Unknown/6ranges printed beforecommentary. Worker stopped. Currentphase ROUND007 COMMENTARY,input round-007-disassembly-final; preserve entirepreviouscommentary beforeprepare/carry,complete fullcurrent-source gate. No terminaldecision/finalaudit/archive.


Coordinator ROUND007 commentarygate accepted afterhandoff/scope/sourceexamples: independently2387scaffolds/3070units,57731Codecovered,0pending/gaps,262160partition,currentfrozenmap/state equality. Freshbyteidenticalverify/readretained. Worker stopped. Currentphase ROUND007 REDIRECT,input round-007-disassembly-final,19Unknown/6ranges. Separateworker next; terminaldecision/finalaudit pending,noarchive. FinalMarkdownselected.

## ROUND007 redirect — repeated factory paired-tail proposal

Input round-007-disassembly-final;262141/262160classified,19Unknown/6ranges. L002 materially reopened with3proposedStaticCode/0Data,accepted0; conditional16Unknown/5ranges. Fresh primary whole-layout comparison finds four exact JMPBABC/JMPBABF factory boundary pairs, three independently accepted conditionalbranch secondtails, and missing64305..64308 before independently acceptednextbody. Narrow stored detachedtail role; no actualentry/fallthrough/naturaluse/termination/originalcompiler claim. Prior “validJMP-only/no repeatedformat” premise corrected. All55acceptedJMP/JMP andthreeRTS/RTS sourcegroups compared consistently; otherJMP/RTS residuals lack the same whole-layout or independentframe/encodedhead role; soundzero/island remain code-versus-data ambiguous with exact negative scopes. Reused23scenario/sevenstage/Factory/helicopter/terminal/credits review; no unchanged runtime replay or useful newshortcut. Combined ordinaryready handoff, not terminalexhaustion/interruption. Evidence analysis/redirect/round-007-r1/handoff.md,paired-tail-contract.json,unknown-ledger.json andavenue-assessment.json. ROM/map/state/source/symbols/configuredbinary unchanged; research/leads/progress only,~3.9GiBfree. No new toolkit issues found; existingNES-001/NES-002 remain nonblocking. No toolkit edits/archive/cleanup/deletion/publication. Final editorialaudit pending; worker stopswriting at handoff.


Coordinator ROUND007 redirect ordinaryreadygate accepted after pairedtailcontract/nativewholelayout/sharedtarget and independentavenueassessment inspection. L002 proposes3StaticCode64305+3,0accepted,current19Unknown/6ranges;4completefactorypairs/3acceptedconditionalentries provide materiallynew combinedproof. Otherresiduals consistently assessed; no productivefrontierdeferred. Worker stopped. Currentphase ROUND008 DISASSEMBLY,input round-007-disassembly-final. No terminaldecision/finalaudit/archive. FinalresearchMarkdownselected.


## ROUND008 disassembly

ROM classification: 99.99% (262144 / 262160 bytes classified; 16 bytes unknown).

ROUND008 independently accepted 3 Static Code bytes for L002 physical64305..64308: detached stored factory-wait transfer, supported by four exact paired-tail layouts, three independently accepted conditional entries, shared target and exact next-body boundary. No Data, retyping or new Observed bytes. Exact residual16Unknown/5ranges:1222+1;13978+10;66280+3;91735+1;117378+1. Structural classification remains incomplete; reachability and eventual return unresolved. Existing23 scenarios/seven stages/later Factory/helicopter/terminal/credits reused; no useful new runtime discriminator.

Separate adjudication: analysis/round-008-disassembly/adjudication.md. Supported native decode/annotate/validate/save/export succeeded; fresh round-008-disassembly-final verifies byte_identical=true. Map SHA256 f8ffa36af6bf8a03285d7c52e8eac89a23f7db1099bddde94667325e559df8dc. All prior regions and whole commentary preserved exactly; ROM/configured binary/configuration/prior source unchanged (protected-check.json). Static247704/Observed14440. Counts/provenance and complement retained in counts.json and unknown-complement.json. About3.9GiB free, small evidence only. No toolkit edits/archive/cleanup/deletion/publication. No new toolkit issues found; existing NES-001 confirmed documentation mismatch and NES-002 missingfeature remain nonblocking,0suspectedbugs/0blockers; see analysis/toolkit-issues.md. Whole current-source commentary next; final editorial audit pending.

|Bank|Code|Data|Classified|Unknown|
|---|---:|---:|---:|---:|
|0|2237|5954|8191|1|
|1|0|8182|8182|10|
|2|0|8192|8192|0|
|3|0|8192|8192|0|
|4|0|8192|8192|0|
|5|1370|6822|8192|0|
|6|4836|3356|8192|0|
|7|6541|1651|8192|0|
|8|3817|4372|8189|3|
|9|6551|1641|8192|0|
|10|6841|1351|8192|0|
|11|7141|1050|8191|1|
|12|521|7671|8192|0|
|13|4507|3685|8192|0|
|14|7453|738|8191|1|
|15|5919|2273|8192|0|

Header16 and CHR131072 remain fully classified.


Coordinator ROUND008 disassemblyboundary accepted afterhandoff/adjudication and currentmap/state/checkpoint hash equality, freshbyteidenticalverify. Exact262144/262160=99.99%,16Unknown/5ranges printedbeforecommentary. Worker stopped. Currentphase ROUND008 COMMENTARY,input round-008-disassembly-final; preservewholepriorcommentary beforeprepare/carry and finishfullcurrent-sourcegate. No terminaldecision/finalaudit/archive.


ROUND008 COMMENTARY complete:2387 scaffolds/3071 units reviewed,zero pending or uncovered Code;2381 exact physical/kind/view blocks carried,whole prior commentary preserved. New detached factory wait tail64305+3 and affected boundaries explained without entry/fallthrough/natural-use/return claims. Fresh supported verify byte_identical=true and read refreshed; final editorial audit pending. Handoff: analysis/round-008-commentary-handoff.md. No new toolkit issues; NES-001/002 nonblocking,0suspectedbugs/0blockers. Worker stops writing.


Coordinator ROUND008 commentarygate accepted afterhandoff/scope/sourceexamples: independently2387scaffolds/3071units,57734Codecovered,0pending/gaps,262160partition,currentfrozenmap/state equality. Freshbyteidenticalverify/readretained. Worker stopped. Currentphase ROUND008 REDIRECT,input round-008-disassembly-final,16Unknown/5ranges. Separateworker next; terminaldecision/finalaudit pending,noarchive. FinalMarkdownselected.

## ROUND008 redirect — primary-evidence terminal-incomplete recommendation

Input round-008-disassembly-final;262144/262160classified (99.99%),16Unknown/five ranges:1222+1;13978+10;66280+3;91735+1;117378+1. Proposed0/accepted0; no ready batch or productive frontier deferred. Retrieved decisive primary before reusing scoped decisions, freshened exact source/bytes/neighbors and configured native decode. Applied accepted B7 whole-pair standard consistently: unique B8 wait/active pair lacks equivalent instruction-bearing layout; B11 busy branch targets firstRTS8646, not extra8647; closest accepted timer/doubleRTS independently encoded second84918; F14 ordinary wrapper and NMI frames bound but do not classify extraCA72. Soundzero and nonuniform island remain code/data ambiguous; fixedDAC extent excludes sample extension only, genericData assessed independently of meaning. Required compact rejection review gives exact claims, primary, actual negative scopes, combined independent constraints, strongest competitors and specific insufficiency, not a missing-fact list. Coordinator independent primary inspection required before terminal acceptance: analysis/redirect/round-008-r1/rejection-review.md and handoff.md. Full current-source gate/current uncertainties and23scenario seven-stage/laterFactory/helicopter/terminal/credits review reused; no useful shortcut/new target test or unchanged replay. ROM/map/state/source/symbols/configured binary/configuration protected; research/leads/progress only,~3.9GiBfree. No new toolkit issues found; NES-001/002 nonblocking,0suspectedbugs/0blockers. Final editorialaudit pending; no archive/cleanup/deletion/publication. Worker stops writing.


Coordinator ROUND008 terminal rejectionreview accepted after per-range reasoning AND decisive primary/source/native/boundary inspection plus independent immutablebyte, B11branchfirstRTS and finite returnword85491 checks. Exact16Unknown/5ranges,262144/262160=99.99%; no readybatch/productivefrontier. Negative scopes localized; no universalcaller/runtime/fullroutine/semanticgenerator requirement; genericData considered independently. Workflow investigation reaches justifiedterminalincomplete, ROMclassification NOTcomplete. Redirectworker stopped. Currentphase FINAL EDITORIAL ACCEPTANCE; separateworker will reuse valid fullreview,check remainingconcreteissues,audit/freshverify/read before terminalhandoff. Archiveoptions selects finalrejectionreview/handoff/findings; noarchive/cleanup.

## Final editorial acceptance

Current-source qualitative editorial acceptance completed with sound prior reviews retained and targeted current dependency/boundary review. Corrected bank7 byte-duration/index wording and bank10 unused $03E4 alias/comment with corresponding summaries and unchanged line anchors. Supported commentary.py audit freshly passed ok=true, byte_identical=true, editorial_audit=true; reading copy refreshed. All2387scaffolds/3071units reviewed,57734Codecovered,0pending/gaps. ROM classification remains terminal incomplete:262144/262160=99.99%,16Unknown/five ranges. See commentary/editorial-audit.md, commentary/audit.json, analysis/final-editorial-results.json and analysis/final-editorial-handoff.md. Protected ROM/map/state/configuration/configured binary unchanged. No new toolkit issues found; NES-001/002 nonblocking,0suspectedbugs/0blockers. No archive,cleanup,deletion,publication or toolkit edits. Worker stops writing at final handoff.


## Coordinator terminal handoff — justified incomplete

ROM classification:99.99%(262144/262160classified;16Unknown/5ranges). Finalcheckpoint round-008-disassembly-final; map/state/checkpoint SHA256f8ffa36af6bf8a03285d7c52e8eac89a23f7db1099bddde94667325e559df8dc. Requiredprimary rejectionreview accepted; no ready lead or productiveinterruption remains. Finaleditorialacceptance passed after targetedtimer/indexcorrections;2387scaffolds/3071units reviewed,57734Codecovered,0pending/gaps. Supportedfresh auditbyte_identical=true/editorial_audit=true,sourcehashes independentlymatch; readingcopy expansion independentlymatches. Allworkersstopped. Coordinatorcompletionaudit11requirements passed,0pending. Archivecontract inspected; archive-options selects105finalMarkdown/omits27duplicateinstructiontemplates,0unrecognized/missing/overlap. All artifactslocal; noarchive/cleanup/deletion/publication/toolkitedits. Orchestrationgoal terminalhandoff achieved; ROMclassification remainsINCOMPLETE. See analysis/coordinator-final-handoff.md,coordinator-completion-audit.json,current-audit.json,final-archive-selection-verification.json. Resume only with newevidence/justifiedcorrectedmethod or explicitreassessment; preserved histories remainavailable.
