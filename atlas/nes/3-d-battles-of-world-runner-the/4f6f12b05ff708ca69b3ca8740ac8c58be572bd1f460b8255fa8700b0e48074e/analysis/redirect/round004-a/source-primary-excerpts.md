## commentary/source/game.asm lines 11665..11694
11665: ; preserve or restore mapper 6. Join the shared initialization or call RAM $0770 under the NMI
11666: ; dispatch convention; residual bytes immediately after that call remain unclassified.
11667: ; Code; Static; physical 98717..98720
11668: .org $818d
11669: DispatchMainStateTransitions:
11670:     inc a:OamStagingPage
11671: .reloc
11672: 
11673: ; Code; Static; physical 98720..98722
11674: .org $8190
11675: region_0181a0:
11676:     ldx #$ff
11677: .reloc
11678: 
11679: ; Code; Static; physical 98722..98723
11680: .org $8192
11681: region_0181a2:
11682:     txs
11683: .reloc
11684: 
11685: ; Code; Static; physical 98723..98725
11686: .org $8193
11687: region_0181a3:
11688:     lda z:CoarsePositionChange
11689: .reloc
11690: 
11691: ; Code; Static; physical 98725..98727
11692: .org $8195
11693: region_0181a5:
11694:     cmp #$dd

## commentary/source/game.asm lines 11885..11910
11885: 
11886: ; Copy the stage palette tail, clear auxiliary/effect state and seed stage palette fields into
11887: ; three render records. Initialize multipart objects and alternate graphics, then enable the
11888: ; running PPU mode. The fixed graphics wrapper restores mapper 6 before the continuation.
11889: ; Code; Static; physical 98796..98799
11890: .org $81dc
11891: MainTransitionPalettePrefix:
11892:     jsr a:CopyStagePaletteTail
11893: .reloc
11894: 
11895: ; Clear auxiliary/effect state and seed the same two stage-record fields into three render
11896: ; records. Select special-motion state 1, run the object initializer and graphics wrapper, then
11897: ; restore the PPU mode. Stage fields $31/$32 are copied mechanically; their full game meaning
11898: ; remains unresolved.
11899: ; Code; Static; physical 98799..98801
11900: .org $81df
11901: ResetMainRunWorkingState:
11902:     lda #$00
11903: .reloc
11904: 
11905: ; Code; Static; physical 98801..98803
11906: .org $81e1
11907: region_0181f1:
11908:     sta z:AuxiliaryMotionState
11909: .reloc
11910: 

## commentary/source/game.asm lines 47633..47687
47633: FixedZeroBacking130797:
47634:     .byte $00, $00, $00
47635: 
47636: ; Clears the stack-page NMI mode byte, enables NMI using the PPU-control shadow, and loops forever locally. The fixed-bank NMI handler discards its interrupt frame and returns through the caller stack instead of RTI. This nonlocal return is an intentional calling convention; do not model the loop as a normal subroutine return.
47637: ; Code; Static; physical 130800..130802
47638: .org $fee0
47639: WaitForNmiReturn:
47640:     lda #$00
47641: .reloc
47642: 
47643: ; Code; Static; physical 130802..130805
47644: .org $fee2
47645: region_01fef2:
47646:     sta a:NmiReturnMode
47647: .reloc
47648: 
47649: ; Code; Static; physical 130805..130807
47650: .org $fee5
47651: region_01fef5:
47652:     lda z:PpuControlShadow
47653: .reloc
47654: 
47655: ; Code; Static; physical 130807..130809
47656: .org $fee7
47657: region_01fef7:
47658:     ora #$80
47659: .reloc
47660: 
47661: ; Code; Static; physical 130809..130812
47662: .org $fee9
47663: region_01fef9:
47664:     sta a:PpuControl
47665: .reloc
47666: 
47667: ; Code; Static; physical 130812..130815
47668: .org $feec
47669: WaitForNmiReturn_Continue1:
47670:     jmp a:WaitForNmiReturn_Continue1
47671: .reloc
47672: ; Unknown physical range; FEEFsingle00 afterselfJMP andbeforeFEF0resetentry;
47673: ; paddingversusBRKliteral unresolved locally.
47674: .incbin "image.nes", 130815, 1
47675: 
47676: ; Stores reset marker $35 in StartupMarker and jumps to lower-window CPU $8000. The preceding reset path selects mapper 6, giving physical PRG banks 12/13 for that entry.
47677: ; Code; Static; physical 130816..130818
47678: .org $fef0
47679: EnterMainBankWithResetMarker:
47680:     lda #$35
47681: .reloc
47682: 
47683: ; Code; Static; physical 130818..130821
47684: .org $fef2
47685: region_01ff02:
47686:     sta a:StartupMarker
47687: .reloc

## commentary/source/game.asm lines 48143..48221
48143: 
48144: ; Tests NmiReturnMode sign. Negative mode jumps indirectly through the installed callback pointer without saving registers. Nonnegative mode restores PPU control, reads PPU status, sets mode $FF, discards three interrupt-stack bytes and executes RTS to the waiting caller. This handler deliberately substitutes a caller return for the ordinary interrupt return; hardware correctness beyond the observed contract is not asserted.
48145: ; Code; Static; physical 131024..131027
48146: .org $ffc0
48147: NmiStackDispatch:
48148:     bit a:NmiReturnMode
48149: .reloc
48150: 
48151: ; Code; Static; physical 131027..131029
48152: .org $ffc3
48153: region_01ffd3:
48154:     bpl NmiStackDispatch_Nonnegative1
48155: .reloc
48156: 
48157: ; Code; Static; physical 131029..131030
48158: .org $ffc5
48159: region_01ffd5:
48160:     nop
48161: .reloc
48162: 
48163: ; Code; Static; physical 131030..131033
48164: .org $ffc6
48165: region_01ffd6:
48166:     jmp (NmiCallbackPointer)
48167: .reloc
48168: 
48169: ; Code; Static; physical 131033..131035
48170: .org $ffc9
48171: NmiStackDispatch_Nonnegative1:
48172:     lda z:PpuControlShadow
48173: .reloc
48174: 
48175: ; Code; Static; physical 131035..131038
48176: .org $ffcb
48177: region_01ffdb:
48178:     sta a:PpuControl
48179: .reloc
48180: 
48181: ; Code; Static; physical 131038..131041
48182: .org $ffce
48183: region_01ffde:
48184:     lda a:PpuStatus
48185: .reloc
48186: 
48187: ; Code; Static; physical 131041..131043
48188: .org $ffd1
48189: region_01ffe1:
48190:     lda #$ff
48191: .reloc
48192: 
48193: ; Code; Static; physical 131043..131046
48194: .org $ffd3
48195: region_01ffe3:
48196:     sta a:NmiReturnMode
48197: .reloc
48198: 
48199: ; Code; Static; physical 131046..131047
48200: .org $ffd6
48201: region_01ffe6:
48202:     pla
48203: .reloc
48204: 
48205: ; Code; Static; physical 131047..131048
48206: .org $ffd7
48207: region_01ffe7:
48208:     pla
48209: .reloc
48210: 
48211: ; Code; Static; physical 131048..131049
48212: .org $ffd8
48213: region_01ffe8:
48214:     pla
48215: .reloc
48216: 
48217: ; Code; Static; physical 131049..131050
48218: .org $ffd9
48219: region_01ffe9:
48220:     rts
48221: .reloc

## commentary/source/game.asm lines 10695..10792
10695: .org $802e
10696: region_01803e:
10697:     sta z:$e3
10698: .reloc
10699: 
10700: ; Code; Static; physical 98368..98370
10701: .org $8030
10702: region_018040:
10703:     sta z:$e1
10704: .reloc
10705: 
10706: ; Code; Static; physical 98370..98372
10707: .org $8032
10708: region_018042:
10709:     sta z:PaletteEffectActive
10710: .reloc
10711: 
10712: ; Code; Static; physical 98372..98374
10713: .org $8034
10714: region_018044:
10715:     lda #$00
10716: .reloc
10717: 
10718: ; Code; Static; physical 98374..98377
10719: .org $8036
10720: region_018046:
10721:     sta a:PpuControl
10722: .reloc
10723: 
10724: ; Code; Static; physical 98377..98380
10725: .org $8039
10726: region_018049:
10727:     sta a:PpuMask
10728: .reloc
10729: 
10730: ; Code; Static; physical 98380..98382
10731: .org $803c
10732: region_01804c:
10733:     sta z:$fe
10734: .reloc
10735: 
10736: ; Code; Static; physical 98382..98384
10737: .org $803e
10738: region_01804e:
10739:     sta z:PpuControlShadow
10740: .reloc
10741: 
10742: ; Code; Static; physical 98384..98387
10743: .org $8040
10744: region_018050:
10745:     jsr a:SilenceMainChannels
10746: .reloc
10747: 
10748: ; Code; Static; physical 98387..98389
10749: .org $8043
10750: region_018053:
10751:     lda z:ProgressionIndex
10752: .reloc
10753: 
10754: ; Code; Static; physical 98389..98391
10755: .org $8045
10756: region_018055:
10757:     cmp #$08
10758: .reloc
10759: 
10760: ; Code; Static; physical 98391..98393
10761: .org $8047
10762: region_018057:
10763:     bcc InitializeMainScreen_CarryClear1
10764: .reloc
10765: 
10766: ; Code; Static; physical 98393..98396
10767: .org $8049
10768: region_018059:
10769:     jsr a:HideMainSprites
10770: .reloc
10771: 
10772: ; For progression index >=8, tail-jump to the fixed mapper-4 selection entry and enter the bank8
10773: ; scene at CPU $8000. This path does not return through the main-bank initializer or establish
10774: ; natural ending completion.
10775: ; Code; Static; physical 98396..98399
10776: .org $804c
10777: EnterSceneAfterFinalStage:
10778:     jmp a:SelectBank8SceneWindow
10779: .reloc
10780: 
10781: ; Code; Static; physical 98399..98401
10782: .org $804f
10783: InitializeMainScreen_CarryClear1:
10784:     ora #$f0
10785: .reloc
10786: 
10787: ; Code; Static; physical 98401..98403
10788: .org $8051
10789: region_018061:
10790:     sta z:StageRecordPointerHigh
10791: .reloc
10792: 

## commentary/source/game.asm lines 970..1072
970:     sta a:PpuControl
971: .reloc
972: 
973: ; Code; Static; physical 65559..65562
974: .org $8007
975: region_010017:
976:     sta a:PpuMask
977: .reloc
978: 
979: ; Code; Static; physical 65562..65565
980: .org $800a
981: region_01001a:
982:     jsr a:Bank8InstallStackByteAndSetup
983: .reloc
984: 
985: ; Continue scene setup after the reviewed initializer, hide OAM, request sound $41 and run
986: ; palette, character and segmented-object animation phases. The final loop waits through the
987: ; fixed NMI service and ticks this bank's sound driver. Entry/return safety depends on the saved
988: ; artificial SP=$F8 contract; this is not proof of normal ending completion.
989: ; Code; Static; physical 65565..65568
990: .org $800d
991: RunBank8AnimationSequence:
992:     jsr a:HideBank8Oam
993: .reloc
994: 
995: ; Code; Static; physical 65568..65570
996: .org $8010
997: region_010020:
998:     lda #$14
999: .reloc
1000: 
1001: ; Code; Static; physical 65570..65573
1002: .org $8012
1003: region_010022:
1004:     sta a:PpuMask
1005: .reloc
1006: 
1007: ; Code; Static; physical 65573..65575
1008: .org $8015
1009: region_010025:
1010:     lda #$41
1011: .reloc
1012: 
1013: ; Code; Static; physical 65575..65577
1014: .org $8017
1015: region_010027:
1016:     sta z:PendingSoundCommand
1017: .reloc
1018: 
1019: ; Code; Static; physical 65577..65580
1020: .org $8019
1021: region_010029:
1022:     jsr a:AnimateBank8PaletteTriples
1023: .reloc
1024: 
1025: ; Code; Static; physical 65580..65583
1026: .org $801c
1027: region_01002c:
1028:     jsr a:RunEightBank8SegmentAnimations
1029: .reloc
1030: 
1031: ; Code; Static; physical 65583..65586
1032: .org $801f
1033: region_01002f:
1034:     jsr a:RunBank8HeroTravel
1035: .reloc
1036: 
1037: ; Code; Static; physical 65586..65589
1038: .org $8022
1039: region_010032:
1040:     jsr a:RunBank8HeroIntro
1041: .reloc
1042: 
1043: ; Code; Static; physical 65589..65592
1044: .org $8025
1045: region_010035:
1046:     jsr a:MoveBank8HeroToEdge
1047: .reloc
1048: 
1049: ; Wait for the fixed NMI service and update sound repeatedly after the scene sequence.
1050: ; Code; Static; physical 65592..65595
1051: .org $8028
1052: Bank8IdleSoundLoop:
1053:     jsr a:WaitForNmiReturn
1054: .reloc
1055: 
1056: ; Code; Static; physical 65595..65598
1057: .org $802b
1058: region_01003b:
1059:     jsr a:TickBank8Sound
1060: .reloc
1061: 
1062: ; Code; Static; physical 65598..65601
1063: .org $802e
1064: region_01003e:
1065:     jmp a:Bank8IdleSoundLoop
1066: .reloc
1067: 
1068: ; Data; Static; physical 65601..65616
1069: ; Fifteen coordinate/configuration bytes followed by the first byte of the next routine. The
1070: ; active initializer copies sixteen bytes starting here; that final copied source byte remains
1071: ; Code. This source boundary does not imply a sixteen-byte Data record.
1072: SceneCoordinateSeed:

## commentary/source/game.asm lines 20798..20877
20798: region_018fe6:
20799:     lda #$80
20800: .reloc
20801: 
20802: ; Code; Static; physical 102376..102379
20803: .org $8fd8
20804: region_018fe8:
20805:     sta a:Pulse2TimerLow
20806: .reloc
20807: 
20808: ; Code; Static; physical 102379..102381
20809: .org $8fdb
20810: region_018feb:
20811:     lda #$0a
20812: .reloc
20813: 
20814: ; Code; Static; physical 102381..102384
20815: .org $8fdd
20816: region_018fed:
20817:     sta a:Pulse2TimerHigh
20818: .reloc
20819: 
20820: ; Code; Static; physical 102384..102386
20821: .org $8fe0
20822: region_018ff0:
20823:     lda #$50
20824: .reloc
20825: 
20826: ; Code; Static; physical 102386..102388
20827: .org $8fe2
20828: region_018ff2:
20829:     sta z:PulseEffectDuration
20830: .reloc
20831: 
20832: ; Code; Static; physical 102388..102389
20833: .org $8fe4
20834: region_018ff4:
20835:     rts
20836: .reloc
20837: 
20838: ; Advance the progression index, increment the bounded remaining counter, clear RAM $0650 and
20839: ; jump to the common main-screen initialization. The >=8 stage path is handled by that
20840: ; initializer, not by an inferred out-of-range stage record.
20841: ; Code; Static; physical 102389..102391
20842: .org $8fe5
20843: AdvanceProgressionPrefix:
20844:     inc z:ProgressionIndex
20845: .reloc
20846: 
20847: ; Code; Static; physical 102391..102394
20848: .org $8fe7
20849: region_018ff7:
20850:     jsr a:IncrementRemainingCount
20851: .reloc
20852: 
20853: ; Clear RAM $0650 and rejoin the ordinary main-loop continuation.
20854: ; Code; Static; physical 102394..102396
20855: .org $8fea
20856: FinishProgressionAdvance:
20857:     lda #$00
20858: .reloc
20859: 
20860: ; Code; Static; physical 102396..102399
20861: .org $8fec
20862: region_018ffc:
20863:     sta a:$0650
20864: .reloc
20865: 
20866: ; Code; Static; physical 102399..102402
20867: .org $8fef
20868: region_018fff:
20869:     jmp a:region_018044
20870: .reloc
20871: 
20872: ; Data; Static; physical 102402..102498
20873: ; Noncode motion backing between the preceding reset branch and the independently anchored jump
20874: ; table. Repeated signed increments and existing indexed consumers were inspected. Its full
20875: ; curve/table organization remains unresolved; no original generator or exact semantic extent is
20876: ; asserted.
20877: .incbin "image.nes", 102402, 96

## commentary/source/game.asm lines 11255..11285
11255: .reloc
11256: 
11257: ; Code; Static; physical 98583..98586
11258: .org $8107
11259: region_018117:
11260:     jsr a:SubmitSoundChannelShadows
11261: .reloc
11262: 
11263: ; Code; Static; physical 98586..98589
11264: .org $810a
11265: region_01811a:
11266:     jsr a:UpdateImmediateSoundEffects
11267: .reloc
11268: 
11269: ; Code; Static; physical 98589..98592
11270: .org $810d
11271: region_01811d:
11272:     jsr a:DispatchMainStateTransitions
11273: .reloc
11274: ; Unknown physical range; Unresolved three-byte legacy call encoding at CPU $8110: JSR $E345
11275: ; targets established fixed-bank zero data. No overlay or ordinary-return contract is
11276: ; established. The separately accepted $8113 stop loop does not validate this call.
11277: .incbin "image.nes", 98592, 3
11278: 
11279: ; Closed bank-local stop loop at CPU $8113 under banks [12,13,14,15]. Its only successor is
11280: ; itself, independent of registers or stack. No incoming accepted reference or natural
11281: ; reachability is established. The preceding $8110 call remains separately unknown.
11282: ; Code; Static; physical 98595..98598
11283: .org $8113
11284: LegacyMainLocalStopLoop:
11285:     jmp a:LegacyMainLocalStopLoop

## commentary/source/game.asm lines 10300..10330
10300: ; established at $8180. The following code-like remnant is excluded.
10301: PresentationInitialPalette:
10302:     .byte $0f, $0f, $0f, $0f, $0f, $0f, $0f, $0f, $0f, $0f, $0f, $0f, $0f, $38, $2a, $1a
10303: ; Unknown: These 36 bytes match main-bank code at CPU $81DC..$8200. Under this bank10
10304: ; $8000-window alias, its local $8229 callee is presentation Data and fallthrough reaches zeros.
10305: ; Fragment origin and local call contract remain unresolved; byte equality does not establish
10306: ; Code here.
10307: ; Unknown physical range; 36bytesexactmatch mainbank12Code81DC..8200, butlocal8229callee
10308: ; ispresentationData andnextphysicalbyteszeros. Couldbe remnantcode or copiedinstructiondata;
10309: ; missingfragment/origincontract, not missingnaturalcaller.
10310: .incbin "image.nes", 82412, 36
10311: 
10312: ; Data; Static; physical 82448..82960
10313: ; Uniform non-code run, bounded by byte changes or established regions.
10314: ; See analysis/round-001/uniform-runs.json; consumer and fill purpose remain unknown.
10315: .incbin "image.nes", 82448, 512
10316: 
10317: ; Data; Static; physical 82960..83030
10318: ; Uniform non-code run, bounded by byte changes or established regions.
10319: ; See analysis/round-001/uniform-runs.json; consumer and fill purpose remain unknown.
10320: .incbin "image.nes", 82960, 70
10321: 
10322: ; Data; Static; physical 83030..83458
10323: ; Slice of the 1 KiB presentation nametable at CPU $8400 under physical banks 10/11.
10324: ; InitializeBank10Graphics copies four pages to PPU $2000. Separate uniform spans keep their
10325: ; earlier Data boundaries.
10326: PresentationNametable_Part1:
10327: .incbin "image.nes", 83030, 428
10328: 
10329: ; Data; Static; physical 83458..83540
10330: ; Uniform non-code run, bounded by byte changes or established regions.

## commentary/source/game.asm lines 15710..15742
15710:     ldx #$02
15711: .reloc
15712: 
15713: ; Code; Static; physical 100651..100653
15714: .org $891b
15715: UpdateModeAndPauseInput_Nonzero8:
15716:     stx z:$e4
15717: .reloc
15718: 
15719: ; Code; Static; physical 100653..100656
15720: .org $891d
15721: region_01892d:
15722:     jmp a:InitializeMainScreen
15723: .reloc
15724: 
15725: ; Code; Static; physical 100656..100657
15726: .org $8920
15727: UpdateModeAndPauseInput_Zero5:
15728:     rts
15729: .reloc
15730: ; Unknown physical range; 8921isolatedRTS afteralreadycomplete8920RTS,
15731: ; noidentifiedadditionalentry or independentcontract.
15732: .incbin "image.nes", 100657, 1
15733: 
15734: ; Code; Static; physical 100658..100660
15735: .org $8922
15736: UpdateModeAndPauseInput_Zero4:
15737:     lda z:PauseState
15738: .reloc
15739: 
15740: ; Code; Static; physical 100660..100662
15741: .org $8924
15742: region_018934:
