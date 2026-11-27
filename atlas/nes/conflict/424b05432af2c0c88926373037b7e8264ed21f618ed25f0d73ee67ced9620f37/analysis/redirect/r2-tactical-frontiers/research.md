# Round2 redirect research (active)

Input pass-002 / map38b30eb21a0c8e9c601f7d1b4d065e37185fec9a4855e2dc331582b1e632f959.
No ROM, state, map or source changes. All decoding uses configured release binary via run.py.

## Tactical bank1

Current-source explicit missing branches and A3E8 command dispatcher led to75 contiguous
absolute words A3EB..A481. A481 is an independently accepted code entry, and all75
entries point to coherent command routines (aliases retained). `tactical-inspection.json`
was preliminary and deliberately overran inline phase words after JSR A4B5; it is NOT
proposal code. `phase-graph.json` corrects this by consuming24 local phase tables.
No instruction overlaps or table/code overlaps occur. It contains2954 instruction
starts,5712 previously Unknown candidate bytes. One accepted Data prefix is challenged:
physical5836 / CPU9826 is RTS, reached independently by BNE981B and returning JSR9823
->A48A (INC7100/LDX7100/STA7100,X/RTS). Old pattern-review.md only classified printable
run5836..5847; it did not inspect these control contracts. Split first byte only;
retain16byte numeric Data suffix. This is analogous to old corrected42DD RTS conflict,
but does not rely on that analogy as proof. No new Observed provenance is claimed.

Forward/reverse renderer consumes directoryA69B and count+four-byte sprite pieces.
` sprite-frame-proof.json` records176words/171unique records. First payloadA7FB bounds
352byte directory; every payload meets the next encoded root exactly; final record
B9FD has8pieces and endsBA1E, exactly physical7A2E acceptedFF suffix. No gap/overlap.
This establishes4995 total Data bytes including prior pattern islands, not a claim
that every frame id is naturally selected. Count field max and byte-index mechanics
must still be independently reviewed before annotation.

Numeric gaps between tactical handler code have direct indexed read consumers (listed
by phase-graph instruction references), repeated parallel47byte equipment rows and
small2/3/4entry coordinate/frame/timer arrays. Bounded role/format checks will be recorded
in data candidates; generic non-code remains permissible where exact original schema
is unresolved. The unrelated19byte A494 helper and bank trailer remain separate.

## Independent current-source continuations

`source-question-index.txt` inventories explicit commentary questions; `source-frontier-
edges.json` inventories43 raw gaps after code fallthrough/calls. Initial bank0 direct
branches8639 and8F27 are ordinary service-loop and status-return continuations.
Setup8B02 fills CHR1/menu packet/latches and joins accepted8B28 loop.
`independent-graph.json` is exploratory: its rows are not an automatic proposal.
Methods use static call/stack/branch contracts, current source services and bank3
field wrapper identities; mapper writes/indirect dispatch are explicit barriers.
Two initial supplied roots were corrected to exact call fallthroughs C1AE/C269;
8750 replaces an erroneous exploratory874F operand root before final proposal.

D196 indirect dispatcher reads primary wordsD1B6..D1CD and apparent secondary
materialD1CE..D1F7. Four coherent targetsD1F8/D215/D3DB/D583 are being investigated.
A provisional fifth handler claim is WITHDRAWN: targetD71A's fresh immutable decode
(`bank-barriers.json`) is low-valued recipe-like bytes, not coherent code. The66byte
non-code decision is separate from universal pointer partition/domain. D71A is also
an encoded address in primary material; inspect actual selector producer/row domain
before claiming whether/why this target is selected. This localized counterexample
does not reject the four routines or all66bytes as non-code.

C547..C552 is literal4 / E06F selection / AC27 call / E072 restore / RTS. The middle
AC27 call and E072 execute under[8,9,14,15], final RTS under[12,13,14,15]. Generic
constant-context decoding across E06F is invalid. Other actual bank writes atDA94
andD52A remain explicit frontier: DA94 selects4 for A33B stage triples; D52A selects0
before battle8010, restoring6 later. These need context-specific continuation views.

A5FD inbank4 saves X/Y and scans6546 coordinate records. Matched exits reachA671
PLA/TAY/PLA/TAX/RTS; unmatched A63C RTS is unbalanced. Its normal-return summary
requires matching-coordinate precondition; do not reject all instruction bytes or
claim all callers supply it. Earlier script's attempted addition to setup-scoped
contracts used wrong bank scope; final candidate verification must scope bank4
[8,9,14,15], never setup[4,5,14,15]. Low-address manual bank0 service summaries must
also be scoped to[0,1,14,15]; generic global PC keys are insufficient.

Bank0 B32F dialogue renderer recursively handles nested bounded record references;
existing current-source explains saved3F/40/41 stack restoration. BEF8 resolves a
BC0C pointer and copies through terminator03 into7300/7310/7320; indirect destination
is fixed73xx RAM, not a mapper write. These are concrete improved contract summaries,
not new runtime or natural reachability evidence. Bank0 inline tablesB5FE..B604
(3words) andB77A..B786(6words) have independent accepted next-code boundaries; handlers
are being followed with their relevant message/selector domains.

## Saved continuation

No terminal exhaustion asserted. Finish exact bank-scoped graph/phase/stack audits,
inspect four D196 handlers plusDA94/D52A bank crossings, and associated bank0 tables.
Then assess remaining current-source questions/generic raw-pattern gaps and refresh
exact244-range ledger with source/neighbor/bank/disposition links before final combined
handoff. All proposal accounting is separate from accepted223826 classified bytes.
No new toolkit issues; NES-DOC-001 remains. No cleanup, build, archive or publication.

### Latest independent constraints

Graphs20057/58072 completed; last10533decodedinstructions/8831view-bytes provisional,
with seven duplicate fixed-bank physicalview bytes previously noted. Bank0 result
matrix98CE..98E0 has9absolute words, selector3*29+2A from saved0..2 options; its96F9
handler constructs two carrybooleans in45 and dispatches4words9725..972D. Earlier
chooser AND0F independently bounds16words8137..8157. AF13 tailJMPFFD7 is a supported
nonreturn outcome/reset sink, so graph must stop there instead of following mapper
selection under unchangedbank4 or touching1FFEF.

D72C's unbalancedRTS diagnosis localized to missing16-coordinate lookup after
saving X/Y atD888/D88A: failure D8ED->DB9E->D76F returns with two saved bytes still
on stack. This is not evidence against its many ordinary codepaths or matched
lookup return contract. Conditional static return0418!=0 independently reaches
D731RTS; universal valid-AI-state termination remains unproved. No runtime replay.

`battle-assets-proof.json`: directory9B1C..9B44 has20words;20descriptors contain
pattern byte/nonzero pointer count;101PPU roots follow BF29's flag/count grammar.
Union covers ALL3525bytes9B1C..A8E1 with zero gaps and independent accepted dialogue
directory endpointA8E1. This is typed non-code; original asset-generator identity
and natural use of everydescriptor are separate.

`reset-template-proof.json`: seven banks share SEI;INCFFFD. Encoded reset C000 and
MMC1mode2 entry [0,1,2b,2b+1] make a concrete resetcallingcontract. RMW oldC0 writes
highbit1, forcing mode3; nextfetchC004 is physical1C014 andlowerselectedbank b.
Configured mapper's primary implementation confirms resetmode change before
consecutive-write filtering. Propose28Code bytes,89generic prefix/footer Data bytes.
Physical+4EA in each sourcebank remainsUnknown: reset-remap skips it and mereNOP
legality/mirrored-prefix membership is insufficient. FFFF/0000 footers retain actual
values, never falseC01B vectors.

ROM9467..949E contains55useful helper source bytes. Installer934C uses X37 and copies
56bytes, so finalcopied byte949E is also firstopcodeA5 of independently supported
result handler949E. Do not turn all56 copied bytes intoData. Proposed splitData55 /
Code949E preserves this dual source/read role; RAMexecution does not prove ROMfetch.

Residualpatterntriage is bookkeeping over currently investigated contexts, not an
exhaustive unknownexperiment. It exposed existing table-selected8686 initializer
and clear controller8178 algorithm, plus accepted AC18 veneer->AEA2 orphan body.
These are concrete supported source continuations, now followed before packaging.

### Connected data and final fixed-service check

32154 completed11636instructions/9215view-bytes, no decodebudget stop. The raw
BC0C directory has40words:4RAM aliases and36ROM slots with aliases/interior suffix
references preserved. Their03-terminated stringunion ends exactlyBD9F; subsequent
score/result body is Code, not partoftext-adjacentgenericData. It saves/reconstructs
nested effect state through existing battle services, updates7020+side withsaturation,
converts score todecimal tiles andprints result records; inspect exact graph rows,
caller/control references andstackpreconditions independently.

`connected-data-proposals.json` packages9240 additionalUnknownData bytes with no
acceptedCode or graphCode overlap. This includes902D's420bytePPU stream ending91D1;
112byte929B OAMsource;55useful9467 helper bytes;128byteB6BE maskedlookup; BC0C directory
andstrings;5finite inline tables; direct-index numeric neighborhoods; fieldqueued
records; and1817clear bit-pattern bytes inthreefixed-bank slabs. Numeric semantic
limits are explicit; no whole-raw-gap inference is used.

A failed assertion localized the shared68byte91E1 source: its PPU packet ends9205,
then32target-palettebytes becomeRAM73AA and are read by8063 fade helper. Both roles
share the exact68byte copy bound; the claim that all68form onepacket is withdrawn.
Similarly queuedtarget tableE0A5's16stage rows consume461bytes, with46bytesafter
stage0's4countedrecords unconsumed bythatcount. Those46 fit8additional count+3records
ending exactlynextstageE113. Preserve them asgenericrecord-likeData; no claim that
stage0selectsall12 or thatoriginalwhole-table organization wasrecovered. All other
rows meetnextpointer, lastrow endsE2A0 independentlyknowncode. Thus507non-codebytes
are supported without inventing naturalselectors for extra records.

`fixed-veneer-proof.json` inspected55exactJMP veneers E000..E0A5 andtheir current
source-labeled services, recording2330decoded instructions/133candidateUnknownCode
viewbytes. Changed/restored bank or indirect-pointer barriers remain explicit;
these do not require classifying their subsequent rawbytes. FirstsixJMP veneers
are split fromprecedingbitpattern: physical1E00D..1E010 holds01/29/C0 tagbytes and
remainsUnknown, notautomaticallydata or code. No missingcaller requirementisused.

Local scratch scriptnameinspect.py shadowedPythonstdlibinspect duringan assertion
exception'ssystemapporthook; this incident reran preliminaryread-onlystaticinspection
andclosednormally, withoutROM/map/state/source mutation. It is not a toolkitbug.
Subsequentstandalones usepython3 -I. No file cleanup/rename/delete performed.

Remaining work: reconcile unique physical Codeviews with allscope/bank proofs;
complete practical setupasset/pattern triage from alreadyidentified data roots;
update exact244-range complement dispositions, persistentleads andcombinedhandoff.
No live server after32154/73701; no terminal exhaustion asserted.

### Completed combined package

Finalphysicalaudit proposes38310newUnknown bytes:15344Code/22966Data, plus existing
Data5836 ->Code. Ifallaccepted,24Unknownbytes/11intervals remain. Acceptedclassification
still223826/262160 and38334Unknown/244intervals. No acceptedmap/state/source/ROMwrites.
Code/data sets aredisjoint; newinstructions crossnoacceptedCode orotherinstruction
operands. ThreefreshE006/E009/E00C views usebank5[10,11,14,15], matching sourceE4C7/
E4D6/E4F4'srow/pattern lookups. Forcedfield6 inspectionofexistinghelperdatareads is
notprimaryproofthatfield6frame-service bytes areData. New133fixed-serviceCodebytes
areexactJMP veneers orRAM/PPU-onlyF800/F8B3 routines, withpreconditionsinpackage.

Final88Data neighborhoods add2521bytes after bounded role/reader/format/boundary
inspection. This is genericData where semantics/exactpartitionremainunknown,
not an absence-of-fetch/raw-export classification. Primaryproofs include nineBF29
panelpackets,64regular8byteunitoverlaypatterns,58sharedresource triples withNOFF
sentinel before95F0, andpositiveD71A/D720/D726 selectorData reads duplicatingCDA1.
An initialpanelassertionusedbank2physicalaliasforbank0 andwascorrected; it isnotROM
negativeevidence. OpcodeB45E'smechanical91E9searchdescription isnotused to invent
anexactkey/value interpretation ofthecommon9209parameterprefix. Specificoverlay
consumerE8FF..E915 andF301..F31B establishmask3F/8bytepatterns independently.

Standalone9169/9179OAMcopy,AECDpaletteadvance andother20smallcontracts produced263
Codebytes. Eachhasexplicitmechanicalrole, knownhelpers, memory/stack/bank effects
andbounds; noactualcaller ismadeauniversalprerequisite. Setup82A5 serializerreturn
requiresF3&7=2; normalCA96battle continuation restoresfield atCA9B, unrelatedtoending
reset-sinks. BD9F additionallyhas953D/95C1/9677static tail-callanchors.

Allidentifiedactionable connected+independent avenues wereclosed intoR2-L02..R2-L08.
Sourcequestionsaboutgamemeanings/originalgenerators andnaturalwinning remainlimits,
notclassificationblockers foralready-supportedData. Residual24bytes haveexplicit
localreview/competinginterpretations inresidual-review.md. This isnotterminal
exhaustion anddoesnotprejudge nextdisassembly/commentary/redirect. Summary,exact
244-rangeledger andhandoff arecomplete; allread-onlyserverhandlesclosed, noactive
run.pyserve verified. Disk2.6GiBfree, attempt25MiB/scratch12KiB, no cleanup performed.
