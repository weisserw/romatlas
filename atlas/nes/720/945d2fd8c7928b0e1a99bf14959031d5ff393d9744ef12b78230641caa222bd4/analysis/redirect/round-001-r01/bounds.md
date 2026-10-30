# Data boundary proposals — Static, not accepted

CPU $CF0C..$CF71; physical $01CF1C..$01CF81; script handlerLE16/operand-count directory. 34 commands5E..7F after high-bit prefix removal; exact102byte table before handlerCF72.

CPU $B4C8..$B4E7; physical $0034D8..$0034F7; terrain handler pointer table. high-nibble domain0..15 in B470; 16LE16 entries; first handlerB4E8.

CPU $B599..$B5A0; physical $0035A9..$0035B0; terrain direction impulse lookup. B589 indexmasked7, eightvalues, next entryB5A1.

CPU $B779..$B904; physical $003789..$003914; directional frame/velocity parameter payload. B766/B6FF/B726 consumers of basesB779,B78E,B7A3,B7B8,B7CD,B83D,B895; repeated offset/sign/frame patterns; bracketed RTSB778 and directory-root scriptB905. Generic block preserves exact subtable extent uncertainty.

CPU $A7CA..$A7ED; physical $0027DA..$0027FD; park information map marker and text parameters. A750 consumes six regularly spaced arraysA7CA..A7E7 and sixbytes copiedA7E8..ED; RTS A7C9/nextcodeA7EE independent boundaries; generic block does not assert selector completeness.

CPU $D7E0..$D7EB; physical $01D7F0..$01D7FB; result grade message/prize lookup. selector0..3 proven by threshold comparison chainD794..D7AE; threefourbytearrays; result routine RTS D7DF and nextD7EC.

CPU $DA1A..$DA1D; physical $01DA2A..$01DA2D; class price packeddecimal lookup. class resource index clamped0..3 atD48D..D49E; RTSDA19/entryDA1E.

CPU $DA5C..$DAB1; physical $01DA6C..$01DAC1; world interaction and medal display parameters. WorldCellInteractionTableDA5C, grade sourceLE16DA7C, destLE16DA84, grade masksDA8C, attribute-list directoryDA90 and terminated attribute pairsDA98 onward; neighborRTSDA5B and codeDAB2; generic compositeblock avoids excessive tableextent claims.

CPU $F61D..$F626; physical $01F62D..$01F636; signed callback speed lookup. F5AB uses command68 args03/08 and04/09 decrementedup3, yielding indices0..9; nextscriptF627.

CPU $F954..$F963; physical $01F964..$01F973; heading-relative marker coordinate offsets. twoeightvalue arraysF954/F95C read by F90D; low offsets -1/0/1 repeated; RTSF953 and scriptF964.

CPU $FA09..$FA28; physical $01FA19..$01FA38; world marker coordinate payload. F9E6 consumes fourregularlyspaced coordinatebasesFA09/FA11/FA19/FA21; repeated position fields; RTSFA08 and scriptFA29; generic block preserves unproved parameterdomain.

CPU $FABA..$FFF9; physical $01FACA..$020009; unresolved noncode pair-pattern payload. FAB7 directory-root script executes6C01,71 endingFAB9; suffix has recurringFF00,FF02,FF04,EF00,FF80 etc bytepairs and sparse-bit fields, inconsistent with coherent instruction flow; no callback/branch target enters it; independent vector boundaryFFFA. Generic Data, no unused/fill/generator claim.

CPU $9EC1..$9EF2; physical $001ED1..$001F02; HUD placement, tile and cursor parameters. Consumers A1D4/A21B/A22C/A232 read9EC1/9ED5/9EE9/9EEE; dense bounded numeric patterns, previous three return bytes excluded. Generic Data..

CPU $A0AB..$A0B4; physical $0020BB..$0020C4; HUD animation parameter tables. A080/A093 read basesA0AB/A0B0; paired five-byte pattern; preceding native RTS and next codeA0B5..

CPU $A2A1..$A4C0; physical $0022B1..$0024D0; HUD template pointers and encoded strings. A27E consumes LE pointer directoryA2A1, destination/character records terminatedFF; A242/A26B useA2DD; regular bounded pointers/text throughout until independently known ResetHudState A4C1. Generic compound data avoids unproved subtable widths..

CPU $A611..$A74F; physical $002621..$00275F; HUD graphics, lengths and sprite coordinates. A503/A594 useA611/A620/A62F/A63E/A6C7; repeated small selector, length, position patterns; previous native RTS A610 and next established information-map routine A750..

CPU $A835..$A84D; physical $002845..$00285D; object collision-response classes. A80C indexesA835 by ObjectType; noncode flag/response patterns; next collision routine A84E; generic data..

CPU $A9DC..$AA3F; physical $0029EC..$002A4F; collision rectangle offsets and sizes. A936/A93E/A968/A970 read four regularlyspaced25byte arraysA9DC/A9F5/AA0E/AA27; native collision rectangle consumers establish role, manyzero/small dimensions; endAA40 known code..

CPU $ADBC..$ADC3; physical $002DCC..$002DD3; ramp interpolation parameters. ADA7 consumesADBC; eight bounded decreasing interpolation thresholds90,76,6C,62,58,4E,44,40; native boundaries ADBB and ADC4..

CPU $B20E..$B279; physical $00321E..$003289; ramp frame directory and frame/flip payload. B1ED readsLE16 B20E table then indexespointed framebytes byWorldCellIndex; repeated pointers toB22A/B234/etc and frameflag values; next native entryB27A. Generic data does not assert complete globalmotion-state domain..

CPU $B3CD..$B41E; physical $0033DD..$00342E; heading and terrain response parameters. B392/B2BF/B42E/B44F useB3CD/B3E7/B40F/B417; repeated signed/heading/direction/coordinate values; source RTS B3CC and next native B41F. Generic compound block..

CPU $BA86..$BA8F; physical $003A96..$003A9F; event state timing/impulse parameters. BA79/BA7F indexBA86/BA8B; two boundedfivebyte value patterns, previous native RTSBA85 and next routineBA90..

CPU $BD91..$BD98; physical $003DA1..$003DA8; terrain probe offset parameters. BD76/BD7B loadBD91/BD95 byY; twofourbyte arrays; previous RTSBD90 and nextcallbackBD99..

CPU $BF96..$BFFF; physical $003FA6..$01C00F; score/event lookup and unresolved pair-pattern bank tail. BF21 score consumer and nearby state helpers inspected; BF96 starts bounded1..9 values followed repeatedFF00/FFbit/sparse-bit pairs, same noncode representation as fixed-tail suffix. No callback or control target enters; last native RTSBF95 and physical bankboundaryC000. Generic Data, no unused/fill claim..

CPU $CED6..$CEDF; physical $01CEE6..$01CEEF; object script stack bounds payload. CECF/CEFD lowerboundsCED6 and CEE3 upperboundsCED7 indexedobjectslot0..5;00,10..90 numeric run, three extra values purposeunresolved; nextnativeCEE0. Generic block avoids claiming extra values are consumed..

CPU $D0F0..$D0F6; physical $01D100..$01D106; script sound-selector lookup. D0E5/D0E7 tableD0F0,Y; seven bounded soundindices05,08,07,0B,06,0A,09; following unreferencedRTS D0F7 explicitly excluded..

CPU $D52F..$D538; physical $01D53F..$01D548; palette and initialscene selection parameters. D528 andD513 readD52F/D534; small repeated selector values; nextD539 native; generic Data..

CPU $D5C8..$D5D0; physical $01D5D8..$01D5E0; scene-resource selection parameters. D5BA readsD5C8,Y; nine bounded selector values0,1,1,1,1,5,2,3,4; next ReadControllers D5D1..

CPU $D637..$D656; physical $01D647..$01D666; controller remap nibble tables. D612/D61E indexD637/D647 with nibble0..15; exactly two16byte tables toWaitForControllerRelease D62D? Physical CPUtable D637..D656, following CheckWorldInteraction D657. Numeric permutation/bit masks establishdata..

CPU $DABD..$DAC2; physical $01DACD..$01DAD2; map resource indirection lookup. DAB7/DADA indexDABD,Y; six immediate selectorbytes02,03,05,06,07,1F between JMP loaderDABA and clear transfer routineDAC3..

CPU $DC50..$DDAB; physical $01DC60..$01DDBB; PPU resource directory and headers. DC1A readsLE16 directoryDC50 then bounded8byte resourceheader fields bank/mode/length/source/dest. Repeated aligned LE pointers and header patterns; endDDAC independently acceptedpalettecode; generic compoundblock leaves exact entrycount unresolved..

CPU $DE35..$DFBC; physical $01DE45..$01DFCC; palette pointers and color payload. DDAC/DDF0 consume fourparallel pointer arraysDE35/DE43/DE51/DE5F and 16byte background/sprite palettes. Pointers address palettealignedcolor blocks,14entryspacing and336payloadbytes; nextnativeDFBD bounds genericblock..

CPU $E316..$E329; physical $01E326..$01E339; attribute quadrant mask and unpacking lookup. E30A/E30D indexE316/E31A, firstmasksFC,F3,CF,3F then two-bit quadrant progression; nextnativeE32A; genericdata..

CPU $E46E..$E4AD; physical $01E47E..$01E4BD; row-split unrolled-continuation directory. E44E masksScrollTileX31, computes32-scroll in1..32, doublesindex. EncodedoriginE46C is two bytes BEFORE actual table; therefore actual32LE16 entries E46E..E4AD. All targets E6FA,E700,...E7B4 advance6bytes matching independently existing unrolled rowuploader. Next codeE4AE bounds table. Do not classify originwordE46C as table..

CPU $EC92..$ECA1; physical $01ECA2..$01ECB1; uniform noncode block. EC92..ECA1 sixteen FF bytes between independently accepted traversal routines; no native/script entry or controltarget, uniformity supports Generic Data only, no fill/unused claim..

CPU $ED49..$EDDD; physical $01ED59..$01EDED; map geometry directories and stream parameters. E8BC..E90E readED49/ED53/ED5D/ED67/ED6C/ED80/ED94/EDA8/EDB2/EDBC; repeated pointer/count/bank/coordinate structures, independent RTS ED48 and ReadStableObjectPosition EDDE. Generic compositeData, not complete format claim..

CPU $F2C2..$F2F1; physical $01F2D2..$01F301; highscore navigation and cursor parameters. Highscore entry/timing consumers aroundF117..F2C1 inspected with new supported nativeCFG; F2C2..F2F1 repeated0,4,FC,1,FF,0A,F6 direction/offset bytes, independently bracketedrestoreRTSF2C1 and ClearHighScoreNametableF2F2; genericData retains schema uncertainty..

CPU $F316..$F422; physical $01F326..$01F432; highscore text and initial record payload. WriteHighScoreTextF423 consumer and initializer referencesF39A inspected; F316..F422 repeating character codes/encoded skips, zero-record fields and bounded initialscore records; native RTSF315 and nextconsumerF423 establish exact genericData boundaries..

CPU $99EC..$9A04; physical $0019FC..$001A14; rendered-object sound policy lookup. New statichelper99CF..99EB indexes99EC by ObjectType and passes first nonnegativeentry to D0E7 sound selector.25 flag/sound bytes to independently accepted FindObjectOfType9A05; genericData..