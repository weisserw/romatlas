# Finite script and bounded renderer-index proof

Primary barrier corrected: CE13->CE67 failure reason was generic data/mapper/indirect/stack-write barrier; CE67 is ReleaseUnusedTileAllocations, not a mapper routine. ActualfailinginstructionCE80 STA$006A,Y aliasesCPUstack only ifYunbounded. CDE1 initializes allocationlastindex6E=3; ReleaseUnused/AllocateDynamic/ConfigureSlot use3..0. OtherwritesSTX6E,Y belongdrawordersortingwithY>=1 anddo notchange6Eitself. Thus exactCE80/CF9F/D00D stores target006A..006D.

D3FC/D423 STA$006F,Y: fourdrawableobjects boundcollect/sortindices0..3; sentinelprevents4store. E0EA STA$004E,Y initializes12cells withY0B..0; E115/E11B stores0042/004E,Y onlyafterCMP0C/BCSskip. Alltheselow-absoluteindexstores staybelowCPU0100. Exactlyeightsites areadmitted; no genericabsoluteindexrelaxation.

Immediateupload CE98..CEE0: sourcePPUpointer,assetbankselect,32-bytecopy withY0..1F,advance/wrapsource,finiteXcount, clearPPUlatch,explicitLDA0/JSRC156,LDA0,RTS. CPUwritesarePPUI/O/RAM,callsordinarybalanced. Validkinddescriptor/source/countprecondition; normalreturnpair0. Thisis distinctfrom CE67allocationloop.

Composition: knownC567/C931/C8C0/E87F/E8DD/E8F6/E8FF/E941/CE98 component contracts returnpair0. Use them atomicallyinouterbank-preservationproof ONLYwithincomingpair0 andtheirvalidqueue/resource/UI/history/descriptorpreconditions. Queuecapacityactualrowcallers reviewedseparately; objecttileloaderCEF1 emits atmostthree negativecount2records perpublishedbatch,3*(4+32)+1=109bytes0100..016C, thenwaitsforflushbeforeanotherbatch. Validlive-stacknonoverlap precondition retained.

Finite script indirects: E357 handlertrampoline has30word/countrecords atE42D, allaccepted3045records respectcommandargumentgrammar; E67B predicate trampoline targetset27 from acceptedscriptarguments; E6C1 subcommandtrampoline9wordsatE6C4. Precondition activepointerbelongs toaccepted8families, scriptstackvalid, objectindices0..3, initializedpair0. These are staticlegal-scriptcontracts, not arbitraryRAMorall7bitopcodes. Resolveonlythese3exactJMPindirectsites via independentlyacceptedtables/argumentset; do notresolvegeneralengine807D or8DC1.

Proof distinguishes conditionalnormalreturnbank/CPUstackpreservation fromtermination/reachability. Any remaining indirectwrite/control or unbalancedCPUstack is a realstop, to inspectprimary lastinstruction instead of inheritinggenericlabel.
