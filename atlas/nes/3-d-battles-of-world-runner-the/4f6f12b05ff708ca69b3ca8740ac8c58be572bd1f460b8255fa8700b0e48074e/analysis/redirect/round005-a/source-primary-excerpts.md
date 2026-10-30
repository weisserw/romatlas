## commentary/source/game.asm lines 333..366
333: ; Image; Static; physical 14352..16400
334: ; Part of eight 1 KiB NES 2bpp tile sets under physical banks 0/1. The fixed supplementary
335: ; loader selects stage*1024 and copies four pages to PPU $0C00. Separately listed uniform spans
336: ; preserve earlier generic Data classification; Part labels identify surviving slices, not new
337: ; tile-set boundaries.
338: SupplementaryStagePatternSets_Part4:
339: .incbin "image.nes", 14352, 2048
340: 
341: ; Returns immediately; no state is changed. This isolated entry has no further body.
342: ; Code; Static; physical 16400..16401
343: .org $8000
344: Bank2ReturnStub:
345:     rts
346: .reloc
347: ; Unknown physical range; Single00 after independent8000RTS andbefore8002fragment:
348: ; paddingversusBRK/metadata unresolved; valuealoneinsufficient.
349: .incbin "image.nes", 16401, 1
350: 
351: ; Rendering mask is supplied in A. With banks 2/3 mapped, copy three pages from $8100 to PPU
352: ; $0000, then four from the advanced $8400 pointer to $2000. Install the local palette and
353: ; finish with a nested software delay. No natural entry into this fragment is established; the
354: ; leading byte remains unknown.
355: ; Code; Static; physical 16402..16405
356: .org $8002
357: InitializeDormantPpuFragment:
358:     sta a:PpuMask
359: .reloc
360: 
361: ; Code; Static; physical 16405..16407
362: .org $8005
363: region_004015:
364:     lda #$81
365: .reloc
366: 

## commentary/source/game.asm lines 482..515
482: .reloc
483: 
484: ; Code; Static; physical 16446..16447
485: .org $802e
486: region_00403e:
487:     nop
488: .reloc
489: 
490: ; Code; Static; physical 16447..16448
491: .org $802f
492: region_00403f:
493:     rts
494: .reloc
495: ; Unknown physical range; Fourbytes00002F80 after dormantinitializer:
496: ; possibletrailingtag/pointer versus instruction remnants; no supportedconsumer. Priorbounded
497: ; inspection reused.
498: .incbin "image.nes", 16448, 4
499: 
500: ; Data; Static; physical 16452..16528
501: ; Uniform non-code run, bounded by byte changes or established regions.
502: ; See analysis/round-001/uniform-runs.json; consumer and fill purpose remain unknown.
503: .incbin "image.nes", 16452, 76
504: 
505: ; Write sixteen local palette bytes to $3F00, reset the palette/address latch and scroll, wait
506: ; for vblank, then enable the background mask $0A. This initializer uses its own palette and
507: ; graphics, separate from the running-game loaders.
508: ; Code; Static; physical 16528..16531
509: .org $8080
510: InstallDormantPalette:
511:     lda a:PpuStatus
512: .reloc
513: 
514: ; Code; Static; physical 16531..16533
515: .org $8083

## commentary/source/game.asm lines 4665..4698
4665: ; Code; Static; physical 67061..67064
4666: .org $85e5
4667: region_0105f5:
4668:     jmp a:InitializeBank8SceneGraphics
4669: .reloc
4670: 
4671: ; Data; Static; physical 67064..67088
4672: ; Twenty-four zero bytes after a tail jump and before the unresolved truncated byte at $8600.
4673: ; Boundaries and neighboring control flow support generic noncode; fill purpose remains unknown.
4674: SceneZeroTail:
4675:     .byte $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00, $00
4676:     .byte $00, $00, $00, $00, $00, $00, $00, $00
4677: ; Unknown: A lone $86 precedes the coherent $8601 wrapper. It may be the tail of a truncated
4678: ; instruction; this byte has no supported instruction boundary or Data role.
4679: ; Unknown physical range; Lone86 at8600 before coherent8601wrapper is trailingbyte of
4680: ; possibletruncatedinstruction; cannot assign role/bounds fromonebyte.
4681: .incbin "image.nes", 67088, 1
4682: 
4683: ; Load follower-three coordinates into the drawing accumulator and append the selected scene
4684: ; record. This coherent wrapper starts after the unresolved truncated byte at $8600.
4685: ; Code; Static; physical 67089..67091
4686: .org $8601
4687: AppendLegacySceneFollower:
4688:     ldx z:SceneFollowerThreeY
4689: .reloc
4690: 
4691: ; Code; Static; physical 67091..67093
4692: .org $8603
4693: region_010613:
4694:     stx z:SceneDrawX
4695: .reloc
4696: 
4697: ; Code; Static; physical 67093..67095
4698: .org $8605

## commentary/source/game.asm lines 11520..11553
11520: .reloc
11521: 
11522: ; Code; Static; physical 98586..98589
11523: .org $810a
11524: region_01811a:
11525:     jsr a:UpdateImmediateSoundEffects
11526: .reloc
11527: 
11528: ; Code; Static; physical 98589..98592
11529: .org $810d
11530: region_01811d:
11531:     jsr a:DispatchMainStateTransitions
11532: .reloc
11533: ; Unknown physical range; Unresolved three-byte legacy call encoding at CPU $8110: JSR $E345
11534: ; targets established fixed-bank zero data. No overlay or ordinary-return contract is
11535: ; established. The separately accepted $8113 stop loop does not validate this call.
11536: .incbin "image.nes", 98592, 3
11537: 
11538: ; Closed bank-local stop loop at CPU $8113 under banks [12,13,14,15]. Its only successor is
11539: ; itself, independent of registers or stack. No incoming accepted reference or natural
11540: ; reachability is established. The preceding $8110 call remains separately unknown.
11541: ; Code; Static; physical 98595..98598
11542: .org $8113
11543: LegacyMainLocalStopLoop:
11544:     jmp a:LegacyMainLocalStopLoop
11545: .reloc
11546: 
11547: ; Derives a small scroll perturbation from alternate-display state and the frame tick, subtracts it from the two-byte scroll position, and writes horizontal scroll plus nametable selection. Enables rendering with mask $1E and decrements the first sprite Y byte.
11548: ; Code; Static; physical 98598..98600
11549: .org $8116
11550: ApplyMainScrollAndMask:
11551:     lda z:AlternateDisplayState
11552: .reloc
11553: 

## commentary/source/game.asm lines 12127..12160
12127: .reloc
12128: 
12129: ; Join the shared state initialization without changing the bank.
12130: ; Code; Static; physical 98789..98792
12131: .org $81d5
12132: JoinMainStateInitialization:
12133:     jmp a:DispatchMainStateTransitions_Nonzero3
12134: .reloc
12135: 
12136: ; Code; Static; physical 98792..98795
12137: .org $81d8
12138: DispatchMainStateTransitions_Nonzero3:
12139:     jsr a:$0770
12140: .reloc
12141: ; Unknown physical range; 81DBRTS followsRAM0770call; knownuploaded0770isJMP0770selfloop,
12142: ; notordinaryreturn. Isolatedopcodealoneinsufficient; do notinferreturnedcontrol.
12143: .incbin "image.nes", 98795, 1
12144: 
12145: ; Copy the stage palette tail, clear auxiliary/effect state and seed stage palette fields into
12146: ; three render records. Initialize multipart objects and alternate graphics, then enable the
12147: ; running PPU mode. The fixed graphics wrapper restores mapper 6 before the continuation.
12148: ; Code; Static; physical 98796..98799
12149: .org $81dc
12150: MainTransitionPalettePrefix:
12151:     jsr a:CopyStagePaletteTail
12152: .reloc
12153: 
12154: ; Clear auxiliary/effect state and seed the same two stage-record fields into three render
12155: ; records. Select special-motion state 1, run the object initializer and graphics wrapper, then
12156: ; restore the PPU mode. Stage fields $31/$32 are copied mechanically; their full game meaning
12157: ; remains unresolved.
12158: ; Code; Static; physical 98799..98801
12159: .org $81df
12160: ResetMainRunWorkingState:

## commentary/source/game.asm lines 15975..16008
15975:     stx z:$e4
15976: .reloc
15977: 
15978: ; Code; Static; physical 100653..100656
15979: .org $891d
15980: region_01892d:
15981:     jmp a:InitializeMainScreen
15982: .reloc
15983: 
15984: ; Code; Static; physical 100656..100657
15985: .org $8920
15986: UpdateModeAndPauseInput_Zero5:
15987:     rts
15988: .reloc
15989: ; Unknown physical range; 8921isolatedRTS afteralreadycomplete8920RTS,
15990: ; noidentifiedadditionalentry or independentcontract.
15991: .incbin "image.nes", 100657, 1
15992: 
15993: ; Code; Static; physical 100658..100660
15994: .org $8922
15995: UpdateModeAndPauseInput_Zero4:
15996:     lda z:PauseState
15997: .reloc
15998: 
15999: ; Code; Static; physical 100660..100662
16000: .org $8924
16001: region_018934:
16002:     bne UpdateModeAndPauseInput_Nonzero10
16003: .reloc
16004: 
16005: ; Code; Static; physical 100662..100664
16006: .org $8926
16007: region_018936:
16008:     lda z:StartEdgeCount

## commentary/source/game.asm lines 38032..38065
38032: 
38033: ; Code; Static; physical 110661..110663
38034: .org $b035
38035: region_01b045:
38036:     sta z:MetaspritePointerHigh
38037: .reloc
38038: 
38039: ; Code; Static; physical 110663..110666
38040: .org $b037
38041: region_01b047:
38042:     jmp a:EmitMetaspriteRecord_Continue1
38043: .reloc
38044: ; Unknown: An isolated RTS lies after a tail jump and before the multipart pointer prefix. Stub
38045: ; code and literal padding remain unresolved.
38046: ; Unknown physical range; B03AisolatedRTS aftertailJMP andbeforeB03Bpointerprefix; couldbestub
38047: ; orliteralpadding, valuealoneinsufficient.
38048: .incbin "image.nes", 110666, 1
38049: 
38050: ; Data; Static; physical 110667..110787
38051: ; Sixty little-endian pointers ending at the first record. All targets parse complete four-
38052: ; byte/FF records before the independent multipart initializer. Shared targets and suffixes are
38053: ; valid; this does not assert a larger selector domain.
38054: MultipartSpritePointers:
38055:     .word $b0b3, $b0b8, $b0bd, $b0ce, $b0df, $b0f0, $b101, $b126
38056:     .word $b14b, $b170, $b195, $b19a, $b19f, $b1b0, $b1c1, $b1d2
38057:     .word $b1e3, $b208, $b22d, $b252, $b277, $b27c, $b281, $b292
38058:     .word $b2a3, $b2b4, $b2c5, $b2ea, $b30f, $b334, $b359, $b35e
38059:     .word $b363, $b36c, $b375, $b386, $b397, $b3b0, $b3c9, $b3e2
38060:     .word $b3fb, $b3fb, $b400, $b400, $b411, $b411, $b422, $b422
38061:     .word $b433, $b433, $b458, $b458, $b469, $b469, $b47a, $b47a
38062:     .word $b48b, $b48b, $b4b0, $b4b0
38063: 
38064: ; Data; Static; physical 110787..110792
38065: ; Multipart sprite tuples; see MultipartSpritePointers for the shared four-byte format. $FF

## commentary/source/game.asm lines 47917..47950
47917:     ora #$80
47918: .reloc
47919: 
47920: ; Code; Static; physical 130809..130812
47921: .org $fee9
47922: region_01fef9:
47923:     sta a:PpuControl
47924: .reloc
47925: 
47926: ; Code; Static; physical 130812..130815
47927: .org $feec
47928: WaitForNmiReturn_Continue1:
47929:     jmp a:WaitForNmiReturn_Continue1
47930: .reloc
47931: ; Unknown physical range; FEEFsingle00 afterselfJMP andbeforeFEF0resetentry;
47932: ; paddingversusBRKliteral unresolved locally.
47933: .incbin "image.nes", 130815, 1
47934: 
47935: ; Stores reset marker $35 in StartupMarker and jumps to lower-window CPU $8000. The preceding reset path selects mapper 6, giving physical PRG banks 12/13 for that entry.
47936: ; Code; Static; physical 130816..130818
47937: .org $fef0
47938: EnterMainBankWithResetMarker:
47939:     lda #$35
47940: .reloc
47941: 
47942: ; Code; Static; physical 130818..130821
47943: .org $fef2
47944: region_01ff02:
47945:     sta a:StartupMarker
47946: .reloc
47947: 
47948: ; Code; Static; physical 130821..130824
47949: .org $fef5
47950: region_01ff05:

## commentary/source/game.asm lines 1056..1104
1056: 
1057: ; Wait for the fixed NMI service and update sound repeatedly after the scene sequence.
1058: ; Code; Static; physical 65592..65595
1059: .org $8028
1060: Bank8IdleSoundLoop:
1061:     jsr a:WaitForNmiReturn
1062: .reloc
1063: 
1064: ; Code; Static; physical 65595..65598
1065: .org $802b
1066: region_01003b:
1067:     jsr a:TickBank8Sound
1068: .reloc
1069: 
1070: ; Code; Static; physical 65598..65601
1071: .org $802e
1072: region_01003e:
1073:     jmp a:Bank8IdleSoundLoop
1074: .reloc
1075: 
1076: ; Data; Static; physical 65601..65616
1077: ; Fifteen coordinate/configuration bytes followed by the first byte of the next routine. The
1078: ; active initializer copies sixteen bytes starting here; that final copied source byte remains
1079: ; Code. This source boundary does not imply a sixteen-byte Data record.
1080: SceneCoordinateSeed:
1081:     .byte $3c, $8c, $00, $28, $5a, $f0, $82, $f0, $f0, $f0, $78, $78, $78, $66, $62
1082: 
1083: ; Wait for NMI, upload sprite palette and DMA page $02, then hide the next OAM buffer and reset
1084: ; its cursor. Tick sound and advance the scene frame counter. The wait service deliberately
1085: ; returns through the NMI stack contract, not an ordinary interrupt return.
1086: ; Code; Static; physical 65616..65619
1087: .org $8040
1088: TickBank8Scene:
1089:     jsr a:WaitForNmiReturn
1090: .reloc
1091: 
1092: ; Code; Static; physical 65619..65622
1093: .org $8043
1094: region_010053:
1095:     jsr a:UploadBank8SpritePalette
1096: .reloc
1097: 
1098: ; Code; Static; physical 65622..65624
1099: .org $8046
1100: region_010056:
1101:     lda #$02
1102: .reloc
1103: 
1104: ; Code; Static; physical 65624..65627

## commentary/source/game.asm lines 21098..21146
21098: ; jump to the common main-screen initialization. The >=8 stage path is handled by that
21099: ; initializer, not by an inferred out-of-range stage record.
21100: ; Code; Static; physical 102389..102391
21101: .org $8fe5
21102: AdvanceProgressionPrefix:
21103:     inc z:ProgressionIndex
21104: .reloc
21105: 
21106: ; Code; Static; physical 102391..102394
21107: .org $8fe7
21108: region_018ff7:
21109:     jsr a:IncrementRemainingCount
21110: .reloc
21111: 
21112: ; Clear RAM $0650 and rejoin the ordinary main-loop continuation.
21113: ; Code; Static; physical 102394..102396
21114: .org $8fea
21115: FinishProgressionAdvance:
21116:     lda #$00
21117: .reloc
21118: 
21119: ; Code; Static; physical 102396..102399
21120: .org $8fec
21121: region_018ffc:
21122:     sta a:$0650
21123: .reloc
21124: 
21125: ; Code; Static; physical 102399..102402
21126: .org $8fef
21127: region_018fff:
21128:     jmp a:region_018044
21129: .reloc
21130: 
21131: ; Data; Static; physical 102402..102498
21132: ; Noncode motion backing between the preceding reset branch and the independently anchored jump
21133: ; table. Repeated signed increments and existing indexed consumers were inspected. Its full
21134: ; curve/table organization remains unresolved; no original generator or exact semantic extent is
21135: ; asserted.
21136: .incbin "image.nes", 102402, 96
21137: 
21138: ; Data; Static; physical 102498..102562
21139: ; 64 motion increments for jump state 2. The phase branches either index 0..63 directly or
21140: ; complement the second half into that range; negative/hold phases are handled separately.
21141: JumpState2Increments:
21142:     .byte $05, $05, $05, $04, $04, $04, $03, $04, $03, $04, $03, $03, $03, $03, $03, $03
21143:     .byte $02, $02, $02, $02, $02, $02, $02, $02, $02, $02, $02, $02, $02, $02, $02, $02
21144:     .byte $01, $01, $01, $01, $01, $01, $01, $01, $01, $01, $01, $01, $01, $01, $01, $01
21145:     .byte $00, $01, $01, $00, $01, $00, $01, $00, $01, $00, $00, $01, $00, $00, $00, $00
21146: 

## commentary/source/game.asm lines 48402..48450
48402: 
48403: ; Tests NmiReturnMode sign. Negative mode jumps indirectly through the installed callback pointer without saving registers. Nonnegative mode restores PPU control, reads PPU status, sets mode $FF, discards three interrupt-stack bytes and executes RTS to the waiting caller. This handler deliberately substitutes a caller return for the ordinary interrupt return; hardware correctness beyond the observed contract is not asserted.
48404: ; Code; Static; physical 131024..131027
48405: .org $ffc0
48406: NmiStackDispatch:
48407:     bit a:NmiReturnMode
48408: .reloc
48409: 
48410: ; Code; Static; physical 131027..131029
48411: .org $ffc3
48412: region_01ffd3:
48413:     bpl NmiStackDispatch_Nonnegative1
48414: .reloc
48415: 
48416: ; Code; Static; physical 131029..131030
48417: .org $ffc5
48418: region_01ffd5:
48419:     nop
48420: .reloc
48421: 
48422: ; Code; Static; physical 131030..131033
48423: .org $ffc6
48424: region_01ffd6:
48425:     jmp (NmiCallbackPointer)
48426: .reloc
48427: 
48428: ; Code; Static; physical 131033..131035
48429: .org $ffc9
48430: NmiStackDispatch_Nonnegative1:
48431:     lda z:PpuControlShadow
48432: .reloc
48433: 
48434: ; Code; Static; physical 131035..131038
48435: .org $ffcb
48436: region_01ffdb:
48437:     sta a:PpuControl
48438: .reloc
48439: 
48440: ; Code; Static; physical 131038..131041
48441: .org $ffce
48442: region_01ffde:
48443:     lda a:PpuStatus
48444: .reloc
48445: 
48446: ; Code; Static; physical 131041..131043
48447: .org $ffd1
48448: region_01ffe1:
48449:     lda #$ff
48450: .reloc

## commentary/source/game.asm lines 11924..11972
11924: ; preserve or restore mapper 6. Join the shared initialization or call RAM $0770 under the NMI
11925: ; dispatch convention; residual bytes immediately after that call remain unclassified.
11926: ; Code; Static; physical 98717..98720
11927: .org $818d
11928: DispatchMainStateTransitions:
11929:     inc a:OamStagingPage
11930: .reloc
11931: 
11932: ; Code; Static; physical 98720..98722
11933: .org $8190
11934: region_0181a0:
11935:     ldx #$ff
11936: .reloc
11937: 
11938: ; Code; Static; physical 98722..98723
11939: .org $8192
11940: region_0181a2:
11941:     txs
11942: .reloc
11943: 
11944: ; Code; Static; physical 98723..98725
11945: .org $8193
11946: region_0181a3:
11947:     lda z:CoarsePositionChange
11948: .reloc
11949: 
11950: ; Code; Static; physical 98725..98727
11951: .org $8195
11952: region_0181a5:
11953:     cmp #$dd
11954: .reloc
11955: 
11956: ; Code; Static; physical 98727..98729
11957: .org $8197
11958: region_0181a7:
11959:     bne DispatchMainStateTransitions_Nonzero1
11960: .reloc
11961: 
11962: ; Code; Static; physical 98729..98732
11963: .org $8199
11964: region_0181a9:
11965:     jsr a:MainTransitionPalettePrefix
11966: .reloc
11967: 
11968: ; Set the meter dirty bit, queue sound $44 and join the common initialization path.
11969: ; Code; Static; physical 98732..98734
11970: .org $819c
11971: MarkTimedMeterAndQueueSound:
11972:     lda z:TimedMeterValue

## commentary/source/game.asm lines 995..1043
995: ; fixed NMI service and ticks this bank's sound driver. Entry/return safety depends on the saved
996: ; artificial SP=$F8 contract; this is not proof of normal ending completion.
997: ; Code; Static; physical 65565..65568
998: .org $800d
999: RunBank8AnimationSequence:
1000:     jsr a:HideBank8Oam
1001: .reloc
1002: 
1003: ; Code; Static; physical 65568..65570
1004: .org $8010
1005: region_010020:
1006:     lda #$14
1007: .reloc
1008: 
1009: ; Code; Static; physical 65570..65573
1010: .org $8012
1011: region_010022:
1012:     sta a:PpuMask
1013: .reloc
1014: 
1015: ; Code; Static; physical 65573..65575
1016: .org $8015
1017: region_010025:
1018:     lda #$41
1019: .reloc
1020: 
1021: ; Code; Static; physical 65575..65577
1022: .org $8017
1023: region_010027:
1024:     sta z:PendingSoundCommand
1025: .reloc
1026: 
1027: ; Code; Static; physical 65577..65580
1028: .org $8019
1029: region_010029:
1030:     jsr a:AnimateBank8PaletteTriples
1031: .reloc
1032: 
1033: ; Code; Static; physical 65580..65583
1034: .org $801c
1035: region_01002c:
1036:     jsr a:RunEightBank8SegmentAnimations
1037: .reloc
1038: 
1039: ; Code; Static; physical 65583..65586
1040: .org $801f
1041: region_01002f:
1042:     jsr a:RunBank8HeroTravel
1043: .reloc
