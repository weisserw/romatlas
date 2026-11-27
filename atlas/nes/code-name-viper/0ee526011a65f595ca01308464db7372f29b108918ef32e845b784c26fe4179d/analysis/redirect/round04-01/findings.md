# Round04 redirect: combined structural constraints

This is a ready proposal, not an annotation or terminal exhaustion decision.
Input pass004/current/state SHA256 db5ce160d67a8705fb7da671627035cdd170680a7af6bccb90756afcdb8c010e.
Current classification262112/262160,48Unknown bytes21fragments. Propose32StaticCode bytes12fragments;
accepted0. If all accepted,16Unknown bytes9fragments remain. Exact proposals.json, unchanged
unknown-ledger.json, residual-after-proposals.json. No data, source, ROM or toolkit changes.
checkpoint.json protects all input identities; research.py executes only api.describe and21 bounded
analysis.decode calls, using configured binary through run.py serve. No runtime/replay.

## Method and changed premises

Prior primary rejection: ../round03-01/residual-dispositions.json and
../../pass-004/review.md. It correctly withheld pointer-only and legally decoded fragments,
but some judgments omitted independent already accepted consumer/stack/field/format constraints.
retry-plan.md records revised exact claims, prior barriers, discriminants and stop conditions.
No new instruction fetches or natural-state claims are made. Old history remains unchanged.
Source anchors below refer to current commentary/source/game.asm; source-contexts.txt preserves
all21 raw ranges with immediate neighbors. Exact decoder/context output is residual-decode.json.

## Eight empty dispatch handlers,8bytes

Physical22716 CPU98AC, banks[2,3,14,15]: source3061..3082 has accepted31stored player pairs
(virtual low8D74/high8D93,index1..31), actual dispatcher LDY0303/LDAlo,Y/STA20/LDAhi,Y/STA21/JMP(20).
ROM slots18/19(decimal;12/13hex) both encode98AC, using physical low19862/19863 and high19893/19894.
Known source callers6709,20328,24820 use JSR, with explicit pair2/3 selection in the fixed-bank
callers. Dispatch itself does not push or pop, so an RTS at98AC returns directly to their next
instruction. Target is exactly one byte60, separate from preceding terminalRTS and following
10-byte scene lookup. This is an empty handler implementation independently of natural production
of state18/19. No claim those selectors are naturally reached.

Same inference is independently checked for actor targets, not inherited from player banks:
accepted224entry low9816/high98F6/flags99D6 domain (source11180..11208), dispatcher source11171..11178
loads ObjectType,Y, forms20/21 and JMPs. Accepted callers17849 and24833 use JSR after explicit
pair8/9 selection. Earlier helper calls return balanced on the route that reaches dispatch;
the documented exceptional airborne unwind exits its own branch rather than leaving a half
return frame at final dispatch. X is a valid actor slot. Exact ROM targets from table-targets.json:

|Physical|CPU|Decimal table indices|
|---|---|---|
|65923|8173|78,79,116,117|
|67973|8975|180,181|
|68926|8D2E|190|
|75494|A6D6|182,183|
|77162|AD5A|42,43|
|77483|AE9B|40,41|

Each is60 and each returns with the accepted dispatcher caller's ordinary stack. Bounds1byte.
Empty placeholder targets are not a competing noncode interpretation: a placeholder implemented
by a dispatched RTS *is* an empty handler. A literal accidentally decoding60 would require
rejecting the independently established encoded target/consumer association. That association,
its bank context, ordinary stack and identical return role across repeated slots together
make the structural inference; neither pointer nor opcode alone does so.

Physical127108 CPUF074 in fixed bank15 adds an independent projectile instance. Source22745..22787
includes actual JSR to F024, then TAY/load F031,Y/store20/loadF052,Y/store21/JMP(20). The split table
has31consecutive low74 bytes at physical127042..127073 and31highF0 bytes at127075..127106,
encodingF074 for indices1..31. Stored index32 targetsD817 and index33 D820; the latter exact highD8
at127107 was already accepted with state20->21 initialization evidence. This proposal does not
need or claim any larger table extent or natural production of indices1..31. Repeated explicit
address encoding, independently accepted table consumer and JSR/tail-JMP stack form the same
empty-handler contract at the one60 byte; nextF075 is separate animation code.

## Two actor animation wrappers,10bytes

Physical68931..68936 CPU8D33: A9 04 4C E5 92, LDA#4/JMP92E5.
Physical78826..78831 CPUB3DA: A5 20 4C E5 92, LDA20/JMP92E5.
Both use banks[8,9,14,15]. Established92E5 at70389..70421(source10700) consumes animation A,
adds current actor facing, stores0401,X and flip0500,X, resets cursor/ticks/flags, and RTS.
It neither maps ROM nor changes stack depth. Preconditions X valid actor slot, ordinary
return address on stack; second entry additionally needs animation selector in Work20.
No actual incoming caller or natural selection is asserted.

New primary counterpart evidence is not mere substring frequency. Exact five-byte sequences
occur at *instruction boundaries*76334/76336 and76321/76323 in accepted bank8/9 code, confirmed
by ../round03-01/final-code-flow.json and source12135..12211. Those instructions use exactly
the same92E5 interface. The Work20 counterpart is the third field of the same three-byte
velocity/delay/animation script schema. The accepted preceding78743..78826 routine(source13190..13238)
selects the same B2EC..EF velocity quadruples, consumes duration then thirdfield into20, advances
cursor by3, and returns with20retained. At76321 this exact retained thirdfield is consumed for
animation. Thus field-flow semantics corroborate the Work20 wrapper's input contract, while
constant4 is independently a normal animation base at76334. The interveningRTS means actual
fallthrough is not claimed. Each standalone wrapper has a coherent complete tail-return contract.

Strongest competitor is a stored five-byte literal copying valid instruction bytes. Against it
are exact known-code boundaries, shared processor context, the known animation-argument interface,
matched thirdfield producer schema, and grouping among actor entry bodies. No local data consumer
or five-byte record format supports a literal role. These combined constraints discriminate the
wrappers more strongly than their original isolated decoding; no argument is extended to shorter
JMP-only or meaningless overwritten-load candidates.

## Coordinate-copy fragment,12bytes

Physical124314..124326 CPUE58A..E596, fixedbank15, context[null,null,14,15].
B9 DB E5 /8D 08 03 /B9 DC E5 /8D 09 03 = LDAE5DB,Y/STA0308/LDAE5DC,Y/STA0309.
The claim is **four-instruction coordinate-copy fragment**, not a complete callable routine.
Calling/flow entry and valid continuation remain unresolved. In particular, no return occurs
before established tileData atE596; entering it as a conventional routine would fall into data.
That negative falsifies a complete-routine claim, not automatically the local structural claim.
No obsolete/original-source-history explanation is asserted.

Independent constraints (source21258..21285,21988..22003):
1. Immediately preceding accepted initializer derives Y=CurrentStage*4, loadsE5DD/E5DE's paired
Xword into0305/0306, and E5DB/E5DC into cameraY34/35. E5DB/DC are independently accepted coordinate
sources, not addresses inferred solely from the residual. The preceding RTS ends that initializer.
2. Other accepted code establishes0308/0309 as low/high object worldY; e.g integration source19841..
19849 and room-descriptor copy22321..22325. The residual's paired source offsets and adjacent
low/high field destinations exactly preserve this word contract. Accepted actor stage initializer
source12274..12303 supplies the analogous stage*4, adjacent-table-byte to X/Yword-copy mechanics.
3. Observed PrepareStageDisplay source21988..22003 sets X44, copiesE596,X to0680,X and decrements
through0. Its independent69byte extent is exactly124326..124395. The twelve residual bytes lie
*outside* this HUDtemplate. Every byte in that69byte template is<=3C, dominated by24 and tile
clusters; residual12includes eight bytes>3C and exactly two legal six-byte load/store units.
This is a boundary/representation comparison, not proof from nonexecution.

Strongest competitor: arbitrary payload prefix with incidental decoding. It needs both exact
coordinate table addresses in order and exact independently established low/high object-field
addresses to be incidental, despite adjacent accepted matching initializer and different,
independently bounded downstream tile representation. Those combined constraints support Code
fragment over payload. Unsupported claim that it can run to a normal exit is explicitly excluded.
Disassembly should independently judge the fragment claim, not demand proof of historical intent.

## Palette-parameter prefix,2bytes

Physical31747..31749 CPUBBF3, banks[2,3,14,15]: A4 2F, LDY Work2F.
It falls directly into accepted31749..31771 BBF5 UploadScenePaletteAndWait(source7089..7103).
That consumer genuinely uses Y: highA6EF,Y and lowA6E5,Y select a palette pointer; 8BCD copies
16bytes to0180..18F(source2799..2810), then flags palette upload and waits through known frame
service. All retain ordinary caller return behavior and established2/3 mapping. Previous31742..
31747 explicitly does LDY#9/JMPBBF5, supplying a parallel parameter-wrapper role.

A further independent accepted source contract is25617..25749, source6221..6228:
PresentConditionalResultsText selects pictureY7, then LDA7/STAWork2F/JSRQueueSceneAttributes.
The latter source7043..7052 indexes attribute pointer words using Work2F*2. Work2F really can
hold a scene-resource selector here; it is also volatile scratch in other unrelated routines.
The ten low/high palette slots atA6E5/A6EF encode A709,A729,A749,A719,A739,A6F9,A779,A769,A6F9,A779,
so selector7 has a valid stored16byte palette, independent of assuming allRAM values are legal.
Exact entry precondition is Work2F holding valid scene-palette index and ordinary caller stack;
no call linking the results path to BBF3 is invented.

Strongest competitor is literal word2FA4 or two unrelated bytes. No established local record
consumer or word grid supports that reading. Compared together, the consumed parameter, immediately
adjacent matching9wrapper, independently used scene selector and valid bounded palette7 entry
support the parameter-prefix Code role. This is not a general rule that any legal two-byte load
before code is Code; the overwritten A907 candidate lacks consumed-value/interface evidence.

## Remaining9fragments16bytes: exact negative scope and practical data inspection

All current fragment contexts in source-contexts.txt; exact bytes/decodes in residual-decode.json.
No proposal is made for these. They have not been declared dead or unreachable. Current-source
reading and bounded literal-word triage were completed for each; literal-word-candidates.json
records hits only within already classified nonCode. The few raw words for9948 occur inside
large graphics containers with no matching bank/finite pointer consumer; unrelated raw byte
matches are not promoted to references. Existing final-code-flow/residual-references were read
with their actual scope; absence there is not proof of absent indirect references.

|Physical interval|Needed claim, primary constraint and strongest competitor|Actual deferral reason|
|---|---|---|
|6253..6255|CPU985D BEQ984C after scannerRTS; BNE985F bypasses it. Other F0ED occurrence is a relative branch with different target, not counterpart semantics. Alternate flag-dependent scanner entry versus2bytepayload.|No independent input/entry or payload format discriminates this single conditional branch. Lack of fallthrough rejects connection to preceding body only. Old986D CPU description was wrong; correct CPU985D.|
|21457..21460|CPU93C1 JMP9106 exactly repeats preceding accepted93BE JMP and another known instruction at22252. Knowncallee is player-airborne updater.|Duplicate lone forwardingJMP has no additional argument/field/table contract; its bytes remain equally explainable as stored3bytefragment. Clone of one absolute transfer alone is not the multi-constraint animation wrapper case.|
|21541..21542|CPU9415 RTS after terminatingJMP, before independently selected9416. No bounded dispatch target names it.|A lone unentered return has no dispatcher/stack association beyond opcode. Empty helper versus literal/padding still materially ambiguous.|
|22519..22522|CPU97E7 LDA0/RTS; preceding scene state helper tailJMPs; nearby9words do not name97E7. No exact supported zero-return counterpart.|No returned-A consumer/value contract or data schema supplies independent role. Coherent minimal returnzero is plausible but legal decoding remains its only specific evidence.|
|22872..22873|CPU9948 RTS after another completeRTS, before damage updater; raw word9948 exists in unrelated graphics.|No bounded pointer consumer/bank association. Empty helper versus shortliteral; raw word search neither proves role nor absence of other references.|
|31867..31869|CPUBC6B bytes00/09, after4x24spritebytes beforeBC6D text emitter.|BRK with signature09 could in principle invoke existing RTI IRQ handler then continue at text entry; this is not a demonstrated useful routine, but lowdomain2bytes do not independently prove data either. Four sprite records stop at96; extra2bytes are not a proven trailer/stride. GenericData still lacks clear pattern in this exact2byteinterval.|
|49141..49142|CPUBFE5 RTS afterJMPfixedfade service, before26zero bytes.|Distinct60 cannot inherit zero-run pattern; no accepted table/control edge gives ordinary return role. Do not expand uniformData through a differently meaningful possible opcode.|
|66866..66867|CPU8522 RTS between completed signedtimer return and next actor body; actor224targets do not name it.|No bounded dispatch association or independent return-body contract. Interface reasoning supports targetedRTSbytes but does not classify every60.|
|99608..99610|CPU8508 LDA#7 before850A; successorLDY14/LDA(record),Y overwritesA before use.|Falsifies meaningful A7argument to current successor. Redundant entryfragment versus2bytemetadata remain; otherA907 occurrences are common immediate loads, no matched argument/consumer.|

For these exact tiny intervals no repeated/uniform run or corroborated noninstruction record
format supports genericData after the bounded inspection. Missing semantics alone is not the
obstacle; their code-versus-data identity remains materially ambiguous. No existing promising
connected continuation is deferred merely for another actor family or phase. Raw full-word hits
were inspected as graphics-container coincidence; no credible new bounded consumer arose.
Unchanged broad gameplay/lateboss/ending routes remain reusable in scenarios.json and prior
handoffs, but would not discriminate these tiny static alternatives. No gameplay replay performed.

Next action is independent disassembly review of the32byte batch, then the required full
current-source commentary and redirect. The remaining16byte conditional complement is a frontier,
not terminal exhaustion. Final editorial audit remains reserved.
