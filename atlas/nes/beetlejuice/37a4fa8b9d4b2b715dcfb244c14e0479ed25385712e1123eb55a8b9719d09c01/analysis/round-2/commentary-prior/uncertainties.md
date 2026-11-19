# Commentary uncertainties

Carry forward relevant phase-1 ambiguities as their regions are reviewed.

## Round 1 current-source limits and redirect questions

All current classified code/data has been explained; none of the following is an unread naming/comment backlog. These are uncertainties about game meaning, opaque continuations or unclassified bytes. Physical offsets include the 16-byte header; bank contexts remain in review.json and the frozen map.

- **L-R1-ROOM:** FindRoomTransitionCell ($01BA90–$01BAE3 classified fragments) reduces player coordinates to 16-pixel cells, matches room/cell records and preserves a matched index. EnterRoomScheduler ($018D33–$018D59) installs task selectors and calls $8110, while ResumePpuUploadCoroutines ($016879 onward) and NmiTaskTrampoline ($01FEBF) use saved SP/indirect PC contracts. Follow the real matched-index continuation and task initializer before retrying later-room edits. Room number alone is insufficient; previous failed experiments remain rejected only as successful visits.
- **L-R1-GFX:** DecodeBitTreePatterns ($009333–$0093AF) uses table transitions, a root reset and a finite output counter. Read selector/format consumer $0092EE onward and inspect payload/source endpoint constraints before classifying compressed ranges. Decoding mechanics do not prove the whole $009800–$00F900 payload extent. The named codec is deliberately descriptive, not an asserted historical compression format.
- **L-R1-STATIC:** BankCallInterrupt variants establish two-inline-byte BRK operands, bank selection and a tail-call flag; ordinary JSR/BRK continuation remains a structural frontier. Prove selected callees' mapper effects and return/stack contracts before joining static graphs. Routine names do not establish natural execution or unused/dead status.
- **C-R1-MAP:** LocateMapCell/LookupMapSubtile/LookupTileProperty ($016DB7, $016E1F, $017058) expose row, metatile, subtile and property consumers. A bounded follow-up can combine their finite index domains with room base selection ($016F2B) to support table layouts/bounds; pointer values alone are not extent proof.
- **C-R1-SPRITE:** RenderSpriteCommandStream/RenderFlippedSpriteStream ($01DCDB–$01DE55 classified fragments) establish OAM layout and controls $80/$81/$82/$83/$FF. Follow a selected descriptor caller and independent termination before proposing ROM descriptor extents. The symbol names describe mechanics; item/actor/animation identities remain tentative.

RAM names represent observed instruction roles: held/newly pressed controller bytes, camera/player fixed-point coordinates, audio channel fields, PPU upload state and shared scratch. Scratch values may serve another purpose in another subsystem. Object field arrays, resource IDs, scene IDs, sound names and exact actor/item identities remain numeric where their semantics are unresolved. Generic Data preserves prior bounded pattern/consumer inspection without asserting a fill purpose. Unknown intervals retain explicit opaque-boundary notes and no invented decoding.

## Round1 redirect evidence update (source unchanged)

Connected findings/ready recipes are inanalysis/round1-redirect-handoff.md andanalysis/redirect/r1-20261119-a/report.md. ROOM scheduler prerequisite corrected withtask0RTI; overhead9/18 andlow1/2 visitedartificially; scene31issearchgate, victoryscript24dialoguecompletedartificially, no naturalterminal/creditsclaim. GFXthreeidentifiedbranches, maplocalizedconsumerdomains, spriteplayer/item/temporary descriptors andnewdisplay/text grammar now have ready structural proposals for independentdisassembly. Unknown complement remains105981 until acceptance.

Two narrow mechanical prose corrections required nextcommentary: RenderSpriteCommandStream_P01DCDB usesFFheaderprefixforflippedvariant and80commandtermination; ExecutePpuTextCommands_P0103B3 uses1return/unwind and2..7Fdictionaryselectors. This supersedes those local statements only; prior unrelated source review remains valid. Primaryconsumer/parse artifacts are linked inredirectreport.
