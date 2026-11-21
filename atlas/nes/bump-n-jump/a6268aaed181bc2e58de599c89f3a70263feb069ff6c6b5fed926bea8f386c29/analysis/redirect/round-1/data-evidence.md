# Proposed data roles and boundaries

All physical intervals below are end exclusive, include the16-byte header, and have fixed CPU view $8000+(offset-$10), effective PRG[0,1,2,3]. Proposed Static roles, not observed execution. No map changes. `proposal.json` partitions the exact current unknown complement. `unknown-reference-bases.json` links immutable decoded operands from accepted code or supported handler closures; immediate XY pointers are visible in the cited commented source. Pointer-only legal decoding is not the acceptance criterion: independent consumers, finite domains, exact adjoining structures, semantic handler consistency and distinct byte patterns are combined.

## Screen/text/palette batch

| Physical | Proposed role and discriminating evidence |
|---|---|
|0205..0209|Four CNROM bus-conflict values30/31/32/33, consumed at01fe/0201 and course graphics0ca3. Low two bits encode each CHR bank. Finite four-bank domain; no code entry crosses them.|
|03ac..043f|Screen/attribute and sprite-text records. Initial five-byte address/value records have FF terminators; later PPU streams contain FE row/FF end controls, tile bytes and zero spaces. InitializeTitle's XY setup selects83be/83d5/83dd; upload grammar in named UploadTileStream (current source). Exact internal formats may differ; generic Data across this bounded interval is sufficient. Preceded by terminating title JMP, followed by AdvanceTitleState. No conflicting code closure.|
|04cb..04f1|Two16-byte palette blocks selected by InitializeTitle XY84bb/84cb, then six palette-animation values84db..84e1. TitleWaitAndStart switches index0/3 and reads three values; all within NES palette domain. Next code starts84e1.|
|0553..0564|Two[Y,X,tile...,FF] records for player/start text. Source callers DrawSpriteText, formatB636; positions50/60 and60/64, high tile alphabet, exact two FF ends.|
|05e9..05f9|Sixteen theme/CHR choices0..3, consumed by stage index at0594. Complete16-course layout and finite four-bank values support data; no code conflicts.|
|0752..076a|Scene text[Y=50,X=60,...FF] followed by16 digit tile values used at06d0 from874a. Scope deliberately remains Data when string versus digit sublayout not needed.|
|0875..0885|Sixteen audio IDs selected by course index at0862; all in accepted command table1..33. Bounded by pre-race return and DrivingFrame.|
|0a1a..0a3a|Two16-byte transition palettes, source BeginTransitionFrame XY8a0a/8a1a. Bytes0F/02/12/22/16/28 in NES palette domain, next routine8a2a.|
|0bca..0c97|Transition handler table5 words8bba..8bc4, eight text pointers8bc4..8bd4, bounded sprite text until8c75, then18 palette-cycle bytes(6triples) to8c87. Transition state init80,0..4 updates, phasebits0..7 and finite counter5b<6 independently establish consumers. `extra-roots.json`, transition-listing.txt and current TransitionDispatch. All pointed text records terminateFF within assigned interval.|
|0e19..0e56|Four[Y,X,tile..FF] results strings consumed by CourseResultsFrame XY8e09/8e1f/8e26/8e36. All four boundaries independently matchFF and next code8e46.|
|0ef5..0f03|Two game-over[Y,X,tile..FF] records at8ee5/8eec, selected by GameOverFrame. Next ending initializer at8ef3.|
|0f4d..0f6d|Two16-byte ending palettes, XY8f3d/8f4d in InitializeRescueEnding; finite palette values, immediately followed by EndingFrameDispatch.|
|10e3..11c0|Four ending code pointers90d3..90db;21 OAM offsets90db..90f0;21text pointers90f0..911a;21[Y,X,tile...,FF] records911a..91b0. Selector0500=0 init then increments only0→1→2→3; final handler resets main state. Text index0501=0 init, increment until15hex; byte21 is a control transition, not a22nd record. `ending-listing.txt` separately proves indexed consumer mechanics, exact offsets and termination.|
|1208..1218|Eight NMI work pointers91f8..9208; scan index0..7 from ServiceNmiWork, targets920a or9208(RTS).1218 is separately proposed Code;1219 remains unresolved.|
|130e..1316|Eight demo button masks92fe..9306, index masked07 atGenerateDemoInput. Full finite domain and next code9306.|

## Actor/state/sprite batch

`dispatch-tables-reviewed.json` records exact code-pointer tables with corrected endpoints. This supersedes provisional `dispatch-tables-hypotheses.json` and its overlong/shorter estimates. Code targets were followed through coherent fields, strides and shared returns, not accepted from pointer values alone.

| Physical | Proposed role and evidence |
|---|---|
|18aa..18ce|16player update pointers989a..98ba selected low nibble0..15; two collision sample handlers98ba..98be selected sample counter0/1. Synthetic return address9662 makes9663(code1673) independently reachable under the two-sample contract.|
|195a..1986|14terrain handler words994a..9966, exact14-bit scan; eight one-hot masks9966..996e; eight variant values996e..9976 consumed with&07. Next helper9976 independently delimits data.|
|1b05..1b13|Seven auxiliary states99a2/9a7a/9ac0/9ad4/9ad4/9ad9/9aeb; low state0→1 collision,2defeat,3stop,4/5countdown,6ending transition, shared return9af4. Next UpdateSpeedAndDigits9b03. Domain and handler consistency jointly establish7words.|
|203d..2068|11traffic statesA02D..A043;7behavior/type wordsA043..A051;7spawn Y positionsA051..A058. Types selected by course bit masksFFe0 and circular three-bit search; all masks excludebit7, while sprite table still provides8explicit variants. Role of type7 sprite entry does not imply an eighth behavior word. Next secondary dispatcherA058.|
|23e2..242e|10secondary update wordsA3D2..A3E6;4type behavior wordsA3E6..A3EE;8periodsA3EE..A3F6 indexed course>>1;32type bytesA3F6..A416 selected course*2+phase;4spawn coordinatesA416..A41A;4direction valuesA41A..A41E. Type array0..3; states0..9, coherent handlers and table-next-table boundaries discriminate code/data. Earlier manifest's11thupdate word was first behavior entry; corrected.|
|2811..28b1|16course bitset pointersA801..A821 followed by128bytes bitmask data toA8A1. Source AdvanceRoadStream indexes course*2 then route>>3 and extracts route&7. Pointer origins allA821..A899, eight-byte origin stride and finite128-byte container; ordered sparse small masks conflict with intended code. No claim that every bit corresponds to a visited road event.|
|29e8..2a3e|16player sprite handlersA9D8..A9F8; three16-byte tile-grid recordsA9F8..AA28; packed flip dataAA28..AA2E. DrawPlayerTileGrid computes baseA9F8+variant*16 and reads16; DrawPlayerMetasprite usesother variants.6flip bytes adjacent structure delimit them. |
|2b0e..2b25|Seven auxiliary sprite handlersAAFE..AB0C, then nine sequential tile IDs72..7A consumed by bounded3x3DrawAuxiliaryTiles.7state domain agrees update table.|
|2bdc..2c02|8type sprite handlersABCC..ABDC,11state sprite handlersABDC..ABF2; sibling update/state domain and next dispatcherABF2 match.|
|2cf4..2d38|4type sprite wordsACE4..ACEC,10state wordsACEC..AD00;10metasprite IDsAD00..AD0A;20tile bytesAD0A..AD1E;10attribute valuesAD1E..AD28. Source DispatchSecondarySpriteState/DrawSecondarySlot bounds variant0..9 and emits2bottom tiles. |
|2d8b..2d8f|Four metasprite choicesAD7B..AD7F selected special_state&03, directconsumer2d75.|
|2e3c..2e51|Five state-sprite IDsAE2C..AE31 (zero state uses separate path),16theme/group variantsAE31..AE41. DrawFourActorPool source masks state07 but writer domain0..4; theme*4 plus group low2 in known4theme domain.|
|2f6b..2f6f|Two tile pairsAF5B..AF5F used by DrawTransientEffect. Initialization effectstate80, age and clears without writing low state bits; decoded consumer has broader&07 but exact table role limited by following code and two explicit pairs. Generic data classification does not require full selector reachability.|
|2f8f..310a and312a..313d|Four-byte repeated tile-pattern prefix2f8f..2f93 (same D2/D0/D3/D1 pattern recurs within tiles; next encoded origin independently delimits it), then metasprite tile/palette/flip container. DrawMetasprite encodes tile baseAF83,4bytes per selector, attributesB09F, packedflipB0E6. First differenceB09F-AF83=284=71*4; next differenceB0E6-B09F=71. LastB0E6+71=B12D exactly nextroutine. These independent encoded origins plus complete71record consistency establish exact container(tail includes previously classified32zero bytes310a..312a). Palette values all&23-compatible, repeated tile/flip records; classify non-code without original art naming.|

## Audio and route data

| Physical | Proposed role and evidence |
|---|---|
|3719..3721|Per-channel software-stack base low bytes90/A0/B0/C0 and high bytes04/04/04/04; ReadEffectEvent indexes each with channel0..3.|
|3769..37f9|72little-endian pitch periods from encoded default pointerB759; decreasing positive16-bit values, high bytes06→00, neighboring initializer/SubmitAudioCommand endpoints. WriteMusicPitch reads note+transpose doubled. Full intended pitch-domain/reconstruction algorithm not asserted. Generic Data is independently clear.|
|3889..3891|Eight one-hot masks80/40/20/10/08/04/02/01 consumed priority index&07; upper/lower overlapping four channel masksB87D indexed0..3.|
|3ce8..3d4a|49paired opcode handlersBCD8..BD3A, full semantic graph in audio-handler-listing.txt, entry/restored X contract and zero conflicting decode boundaries.49values terminate exactly at first handlerBD3A; all code matches command parameter/stack semantics. Supported static domain90..C0, malformed opcodes beyond not claimed safe.|
|405c..5010|Audio command pointersC04C..C08E for IDs1..33,33headers betweenC08E..C164, priority bitmapsC164..C16C, stream graphC16C..CFFC, fourFFtailCFFC..D000. Header mask dictates3+2*popcount(mask&7F) record size; all masks onlyfourchannels.33pointer count independently bounded by minheaderC08E and record chain. Script audio-streams.py implements49handler operand counts, triangle4/pulse-noise5register events, jump/call edges.3382tokens zero invalid/out-of-bound addresses. Music90 is timedrest, A2stopmusic, B6stopeffect, BFreturn; only four redundantA2,one redundantB6 not traversed. Header priority pointers selectC164/C168; bitmaps fourbytes each consistent33command arbitration(IDs1..33 needs5bitsbytes but last relevant incoming33 accessesbyte4 atnextbitmap; no extra full-domain bitmap claim). Entire residual non-code evident independently of exact per-track semantics; use Data for mixed container if desired. Table baseC04A includes preceding codePLA/RTS as unusable ID0, do not annotate405A..405C as data.|
|5010..5100|Sixteen course route pointersD000..D020 then small-symbol chunk lists17..1F withFFend markers; known ReadCourseChunk and final/loop marker consumer discriminate grammar. Existing Data5100..800A begins midcontainer; do not claim5100 is original object endpoint. Exact proposed boundary5100 simply meets accepted classification; generic Data is sufficient on5030..5100.|

No unknown data meaning is used as a blocker. Generic classifications above rely on clear non-code patterns plus bounds/consumer checks and no conflicting supported code; finer layouts are optional. Tiny unproven stubs and isolated00 are retained unknown, never swept into generic data.
