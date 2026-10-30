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