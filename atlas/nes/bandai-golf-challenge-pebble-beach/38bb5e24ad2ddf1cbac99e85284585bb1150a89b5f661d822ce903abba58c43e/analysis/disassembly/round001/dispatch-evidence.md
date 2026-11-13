# Inline dispatcher structural barrier

Fixed CPU PRG slots [0,1,2,3]. Routine $8D31 doubles A, sets Y=2*A+1,
pulls JSR return low/high into $00/$01, reads two indexed pointer bytes,
then JMP ($0000). Thus it consumes the direct return address; no ordinary
fallthrough into the inline words. Caller-provided outer stack return governs
handler return. Natural selector reachability is separate.

Ten JSR byte patterns were found in physical PRG; all have adjacent coherent
little endian handler vectors bounded by the next supported handler entry:
$846D+32, $8498+28, $8755+14, $8848+12, $88A6+16,
$88E5+18, $8956+14, $89A7+12, $94A5+16, $9B87+10.
Static pattern scan is only candidate evidence; bounds also use dispatcher
consumer and aligned handler/next-routine boundaries. All entries agree.
Startup observed edges independently show dispatch use. Original table layout
and full natural selector domain are not claimed.
