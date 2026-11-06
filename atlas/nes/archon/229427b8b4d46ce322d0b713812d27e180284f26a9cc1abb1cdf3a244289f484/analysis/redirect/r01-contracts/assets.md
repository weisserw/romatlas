# Graphics consumer bounds
Input six generic Data banks already accepted. Refinement adds no classification bytes.

C317/C381 (physical1C327/1C391) derive selected16KiBbank =4+((ActorTypes[X]&18)>>3), sourceCPU=8000+((type&7)<<11), length0300 (12 buffered40-bytechunks or directPPU copy). Each source is exactly48 NES planar tiles. Bit operations independently clearcarry beforeADC; page origins and exactlength established. Existing bitmap inspection all-bank sheets supplies independent graphics content evidence.

The40-entry PieceTypeById table atphysical01CA5 has ordinaryIDs0..35 ->types0..15, plusIDs36..39 ->10,11,12,13. Elemental producerphysical1E09E..1E0CE uses RNG&03, thenADC24, loads9C95[X], proving allfour extraID slots. Existing encounter consumercomparesID>=24 separately. Hence supported actor graphicsdomain0..19: all8 source0300 chunks at0800stride in16KiB banks4and5, first4 inbank6. Twentychunks/15360bytes may be relabeled specific actor graphics within alreadygenericData, preservingintervening500byte gaps generic. ActorTypes65 transient67 occurs as introductionstate before subsequentexplicitwriter; no claim it reachesgraphicsconsumer. Do not extenddomain to20..31 merely from bitmask.

Physicalchunk:10+bank16K*4000+(type&7)*800; count300. EffectiveCPUview foruploadbankb is[2b,2b+1,14,15]. Xparity selectsCHRdestination0000/0800. NoCPUexecutioninassetbanks claimed.

Bank3arena uploaderFDD9 physical1FDE9 has source8000 and source8E/8F from caller; exact descriptor flow retained in graph and prior commentary. Boardstartup selectedbank1 descriptor8000->PPU1000 length1000 is already documented, pairedtilemap/metatile consumers now clarified by inline4-bytecontracts. Genericassetbanksremaincorrect where subdivisions notbounded; no need complete original graphicscontainer schema.
