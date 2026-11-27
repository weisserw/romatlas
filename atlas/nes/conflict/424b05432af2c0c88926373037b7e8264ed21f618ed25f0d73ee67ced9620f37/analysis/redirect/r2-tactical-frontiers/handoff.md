# Round2 redirect ready batch — ROM remains incomplete

Accepted checkpoint is pass-002, map `38b30eb21a0c8e9c601f7d1b4d065e37185fec9a4855e2dc331582b1e632f959`.
Accepted classification is still **223826/262160**, with **38334Unknown bytes/244intervals**.
No ROM, map, state or commentary-source changes occurred.

The combined Static proposal covers **38310uniqueUnknown bytes:15344Code +22966Data**,
plus **one existingData byte5836 ->Code** while preserving its16byteData suffix.
If everyproposal is independently accepted, **24Unknown bytes/11intervals** remain.
These are proposed counts, not accepted classification or a complete-ROM claim.
[Summary/per8KiBcounts](summary.json), [physical/conflict audit](candidate-audit.json),
[exactaccepted complement/dispositions](unknown-ledger.json).

Current source hash remains `9e2e25c7d996ddf316c0a6351ee5d1b7278f52437a55a6a96a5acd83b8e2fcb0`;
its12598entry/641unit commentary gate is unchanged. Next acceptance requires independent
disassembly and fullcurrent-source commentary, not reuse of this gate for newbytes.
Input ROM hash remains424b05432af2c0c88926373037b7e8264ed21f618ed25f0d73ee67ced9620f37.

## Review package

* [Code proposals](code-proposals.json) provide everyinstruction, exactphysical/CPU/bankview,
  newunknown ranges and the5836retype. No newinstruction crosses acceptedCode bytes;
  there are no candidateoperand overlaps or Code/Data conflicts. All are Static,
  including standalone contract-supported helpers; no newexecution was observed.
* [Data proposals](data-proposals.json) retain wholetypedclaim evidence but list exact
  Unknownintersections for annotation. Preserve existingText/Data islands; do not
  blindlyreplace entireassetcontainers. Meaning/partition uncertainty is explicit.
* [Bank-view correction](code-view-overrides.json) is primary forE006/E009/E00C: their
  row/pattern services requirebank5[10,11,14,15], not the exploratoryfield6view.
  IncorrecthypotheticalE4DB/8010 datareferences are not roleproof for fieldbank6 bytes.
* [Research/history](research.md) preserves exploratoryfailures, correctedpremises,
  copier/consumer boundaries, bank/stack conditions and allnegative scopes.
* [Residual review](residual-review.md) explains all24retainedbytes. This is a ready
  batch rather than terminal exhaustion; a new current-source pass may expose more.

Persistent ready leads are R2-L02..R2-L08 in `analysis/leads.json`:

| Lead | NewCode | NewData | Main proof |
|---|---:|---:|---|
|R2-L02|5712|416|75commandwords,24phase tables/allcoherenthandlers;5836RTS correction separate|
|R2-L03|0|4964|176framewords/171unique counted4byte sprite records; exactA69B..BA1E|
|R2-L04|0|1984|Tactical indexednumeric/position/timer/effect rows; genericwherepartitionuncertain|
|R2-L05|28|89|Seven resettemplates withmode2->3 physicalremap; prefix/footerData; +4EAunknown|
|R2-L06|9604|0|Battle/setup/fieldcontrollers, bankedwrappers, fixedservices andboundedstandalonehelpers|
|R2-L07|0|12992|Battle/setup/creditsPPU/OAM/text/lookup/queuedrecord containers|
|R2-L08|0|2521|Directconsumer-backedshortparameter tables,64unitpatterns/58triples andgenericneighborhoods|

## Decisive contracts and independent recipes

1. **Tactical animation:** [phase graph](phase-graph.json) and
   [tacticalData](tactical-data-proposals.json) followall75commandwords and24nested
   phase tables, with aliases. A4B5 pops its inlineJSR return andJMPs absolute;
   do notdecode words ascontinuations. 5836is9826RTS reached by981B BNE andreturning
   9823JSR A48A. Originalpatternclaim covered onlyprintablemembership; repairfirst
   byteonly. [Sprite proof](sprite-frame-proof.json) has176words/171unique records,
   count1..32, everyrecord meetingnextroot and final8piece recordendingBA1E at the
   independentFFsuffix. Natural use ofallcommands/frames is unclaimed.
2. **IndependentCode:** [graph](independent-graph.json), [smallcontracts](small-contract-code.json),
   [fixedservices](fixed-veneer-proof.json). C549 selects4; AC27/E072 arebank4 and
   finalC552returnsfield[12,13,14,15]. DA94 switches4/restoresDABC orDB4B;D52A switches0/
   restoresD535; normalCA96battle continuation restores atCA9B. AE63/B0E8 already
   executein4 and pairedrestoresreturn4. Bank4A5FD has a matched-coordinate return
   precondition; its unmatchedA63C isunbalanced. D72C's missinglookup path similarly
   leaves2savedbytes; do not infer universalAItermination. Conditional/staticroutine
   contracts are sufficient for theirCode roles without inventing actualcallers.
   BD9F has953D/95C1/9677tail-callanchors, score/RAM/dataflow, knownservices andBE47
   boundary. AF13->FFD7 isreset/nonreturn; neverextend to1FFEF.
3. **Packet/record containers:** [battleassets](battle-assets-proof.json) proves
   20sequencewords/20descriptors/101PPUroots cover3525bytes9B1C..A8E1. [Setupassets](setup-assets-proof.json)
   and [creditsassets](credits-assets-proof.json) use the actual81D9 grammar. BF29's
   bit40 meansfill;81D9's bit40 meansliteral. Both haveFFterminators/count0=256.
   Credits'7packets meetexactnextroots,7FF-OAMsources meetAD31,7two-counttexts meetAE3E.
   [ConnectedData](connected-data-proposals.json) separates68bytecopy91E1 into
   36packet+32targetpalette; copy9467's56bytes include949ECode, so only55usefulbytes
   areData. BC0C's40words include4RAMaliases;03stringunion endsBD9F, notBE47.
4. **Queued/fieldData:** [queuedtargetproof](queued-target-proof.json) has16stage
   directories/63declaredvariable records endingE2A0;46bytesafterstage0's4records
   fit8more but aregenericData, not a selectedcountclaim. [FinalData proof](final-data-proposals.json)
   has64word92A2dictionary/64regular8byterecords ending9522, then16sharedstagewords
   and58type/resource triples ending95F0. There isNOFFsentinel in those58triples:
   matchedtypes0..2D/34..3F constrainnormalreturn; do notparsebeyondindependentbounds.
   18byteD71A selectorData ispositivelyread atD71A/D720/D726 andduplicatesCDA1; it is
   not a fifthCodehandler merelybecause awordpoints there. D1B6's66wordmaterial is
   non-code with itsuniversalnestedpartition unresolved.
5. **Resettemplates:** [proof](reset-template-proof.json) combines encodedC000,
   sevenSEI/INCFFFD sequences andconfiguredMMC1 highbitreset behavior. Underexplicit
   mode2view[0,1,2b,2b+1], nextfetchC004 maps tophysical1C014 inlastbank. Onlyfirst4bytes
   becomeCode; original+4EA remainsambiguous. Generic00/FF/patternedslabs and actual
   vector/footerwords areseparate, withno inventedfillgenerator/C01Bmirrors.

## Why other questions do not prolong this batch

The sourcequestion/frontier indexes were checked against actualsource andprimary
contracts; lexical“draw/redraw” hits are not independent uncertainties. Productive
command/phase, battle, setupstates, fieldalternate/AI, fixedservice andassociateddata
avenues were followed together rather than deferred atfirsthandler orfirstreadylead.
No unexploredreadyhandler is reserved merelybecause it isindependent.

All remainingphysicalunknowns are exactlythe24 inresidual-review: sevenEAbytes,
982D's3byteendingcontinuation,1E00D's3bytetag,1FFEF'sRTS and20000's10bytefooter.
Theiractualmissingrole/entry/format constraints andcompetinginterpretations arelinked.
Unitbyte3'sdisplayname, exactnestedselectorpartition, extra finalthirdslab behavior,
composer/generatoridentity andnaturalcampaignwinning remain semantic/reachability
limits; supportedData classification does notwait for these. Bank5/audio/outputassets
have no remainingstructuralunknown payloads needingunchangedroutes. Reusing saved
broadgameplay/ending evidence is sufficient; no arbitraryRAM edits, unchangedreplay,
exhaustiveactor/state search or universalnaturalcaller requirement was imposed.

No new toolkit issues found; NES-DOC-001 remains in `analysis/toolkit-issues.md`.
Configuredreleasebinary was used, withouttoolkitedits/rebuild. No cleanup, deletion,
archive/publication or portablemap/state export. All evidence/ROM/assets remainlocal.
Allread-onlyserverhandles completed; no livewriter/server. Disk was2.6GiBfree atlastcheck.
