# QueuePpuTransfer conditional normal-return contract

C567 has explicitPHA(mode),optionalbankselectfromX, four-bytePPUrecordheader,PLA(mode),normalpayloadYbytecount or negativepayload16*Ybytecount,FFsentinel, savenewcursor, selectPRGreg0,RTS. Header/payload stores targetCPU0100+X and Xwraps8bit. Its paired C63Cqueueconsumer temporarily setsSPFF and usesPLA, then restores savedSP; source already identifies queuebuffer sharing theCPUstackpage.

Supported callingprecondition: nonzerocount and queuecapacity/cursor keep everyconstructedrecord byte/sentinel disjoint from live CPUreturn addresses, temporarymodepush and nestedJSR/interruptstack usage duringconstruction. Normalmodepayload=count; negativemodepayload=16*count. Do not assert everyarbitraryRAMcursor/count satisfiesit, or that allpage01writes alterlivecontrolstack. This is exactlythestandard validqueuecontract impliedby thepairedwriter/consumer and knownqueue layout.

Under that precondition onlyPHA/PLA andordinaryJSR changeSP, allpayloadwritesremaininCPU page01 (notmapper), and bothmodepaths selectPRGreg0 beforeRTS. Thusconditionalnormalreturn providespair0/1/30/31. No globalstackcapacityorallcaller-domainproof is claimed. Unknowncapacity violations wouldbe invalidcallingstates ratherthan evidence thatthevalid queuebuilder cannotreturn.

Static callers may classify coherentpostcallcode usingthisexplicitprecondition; actual/naturalexecution remainsseparate. Verificationrecipe: inspect C567..C63B exact instructions and consumerC63C..C6BE; independently check push/pop count/headerstride/payloadloops/sharedrestore0. For a reachabilityexperiment additionally deriveactualSP/cursor/count and reserve allnestedstackbytes; no suchdynamicclaim inthisproposal.
