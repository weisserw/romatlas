# Round 3 redirect combined handoff

Input **pass-003-reviewed**. Accepted classification remains **112126/131088 (85.53%);18962unknown**. Map/state SHA e71e545e01a1437d4ffbff9ffaef3b80ea2e5e94fe30d1ce7a39f59dbf217ae7; source SHA5215e633bf60f026a995bf7c10e39f8c341e16f12ee20115dd8a476be40333c4. ROM/source/prior commentary/maps/state/configured release binary unchanged; audit.json records identities.

**536 proposed views /18925 unique currently unknown bytes=1107Code+17818Data**, all proposed Static, accepted0. If all survive independent review,37bytes remain. This is a productive combined handoff, not terminal exhaustion or acceptance. Overlapping views/per-lead gains are not additive. unknown-ledger.json preserves the exact163-range/18962-byte input complement with overlays and residual dispositions.

| Ready ID | Unknown bytes in its union |
|---|---:|
| L-R3-UPLOAD |873|
| L-R3-PROPERTY |4744|
| L-R3-DATA |7037|
| L-R3-OBJECT |922|
| L-R3-SPRITE |1841|
| L-R3-TABLE |3254|
| L-R3-CODE |299|

## Generated writers and bank/stack contracts

upload-results.json/decode.json plus the source preparer establish concrete PCs, not arbitrary saved-pointer execution. Column X0..29: E88C+4X pops30-X bytes, switches PPU address, jumps through01D2=E985-4X and pops X more. There are **30**, not31, first-block pairs. SP AF consumes prepared01B0..01CD; restore PPUCTRL, original SP03E3, RTS. Row X0..31: E9AB+4X pops32-X,01D6=EAB2-4X pops X, restore SP/RTS. SP D7 consumes01D8..01F7. No nested JSR/mapper write occurs during these temporary stacks; accepted metatile expanders prepare the buffers.

Bank3's8E42 prefix has explicit D0D6 producer, task SP27 and48 pairs joining the known writer/SP26 scheduler continuation. The9522/9538/9552/961D graph closes two prepared row phases with explicit57/59 pointers and saved-EB-SP restoration. Wider scheduler-PC domains remain unclaimed.

vector-results.json/vector-decode.json inspect vector words independently: NMI FFEE/reset FFE6/IRQ FFAC in all banks. Bank2 FFEE saves A and jumps FE81; FE81 saves X/Y, uses returning00/18 audio service, delays, restores X/Y, calls accepted E3FC, restores A and RTIs at FE9D. Inline00/18 is two Data bytes; separate FE9E RTI remains unknown. Reset prefixes banks1/2/3 stop at the mapper store selecting physical banks12..15. Old physical FFEB is not inherited as reset fallthrough.

Four FF9C copies match a complete save/select/JSR8000/pop/restore/RTS skeleton. Entry: Y0..3 selected bank, supported RAM dispatch pointer, ordinary returning selected service. Copied return instructions restore the pushed literal caller bank. This is not arbitrary-callee return or hardware/mirroring compatibility.

Inventory EAB6 is accepted EAB0's ordinary fallthrough. EB5F's five zero-fill stores have eleven independently encoded biased EB92/EB9D continuation slots and0B row-count contract.

## Map, property, objects and motion

EC5B/ED22/ED8B prove the nibble swap constructs16*reference property offsets. Much low-room storage was already accepted; that fact alone claims no gain. BE4C explicitly installs metatile B15E/property B42C. B42C..BE4C is162*16 bytes to known code; all718 reference bytes B15E..B42C lie within0..161,max161. Property structure and bounded generic reference storage are separate claims.

AC45..B15E's1305bytes were inspected completely for repeating map-like rows, references, neighbors and code conflicts, then proposed generic non-code. The subsequent biased-source audit retrieves22 independently encoded room8..29 origins in EF6F..EF85/EF8C..EFA2; rooms27/28 share B0BC. Geometry/original serialization remain unclaimed. A possible180-record read view ending B42E would overlap two property bytes; that specific extent is not promoted. See late-results.json/motion-results.json.

Accepted low-room metatile views collectively produce IDs0..84. EE6C/EFBA terrain consumers index EEC6; all85 values0..3 end at independent initializer EF1B.

objects-results.json closes all three23-slot room8..30 pointer slices: counted five-field zero-page records fit four slots; FF-terminated transient/support records fit six/seven. Every endpoint agrees with another source or BFDF/C0EE/C15B(next pointer arrays/code). motion-results.json parses six nonnull counted thirteen-byte lists from room12..30 slices, exact A065..A1BD to known code. Full global/natural room selection is not required.

## Streams, fields and sprites

data-results.json closes nine backward zero-run streams, each128output bytes, contiguous8396..872B. Six forward PPU sequences00/00-terminate: five coverF9E2..FED5; sixth8206..82FD is an encoded source, not an invalid cross-region pointer. DE45 uses **FF address / FE termination**, ending DF00; preceding16 palette bytes separately bounded. Six five-byte actor seed lists end9919. Pattern buffers are six64-byte windows D4AE..D5EE and three96-byte windows D5FA..D71A. Bitmap8069..81AB has46seven-row records and independent following routine.

For8B34..9864, seven coherent PPU sources/eight72-byte-copy mechanics support finite windows; reads can cross accepted Code, which stays Code. The otherwise unreferenced512-byte8D84..8F84 slice visibly forms NES bitplane artwork(ppu-gap-pattern.png). Generic storage claims do not assert original asset boundaries.

fields-results.json closes five28-entry spawn columns, six9-entry presets, nine forward packed-timer streams. Decisive writer: bank0 A7BD **INC07EE**, then81/63 tail service into bank1 reader. An unrelated cooldown DEC did not establish backward reading. All nine forward0 endpoints equal another start or known code91B3. Palette86E0..87A8 has200 multiples-of-three selectors,max234;87A8..8895 has79 valid three-color triplets and independently encoded next coordinate origin8895. Dense resource/coordinate storage has local bounds without world geometry.

sprites-results.json records coherent pointer families, optionalFF/header/81/82/83/80 grammar, shared suffixes and literal/default paths. Storage versus natural domains are separate: AF11 has eleven storage pairs including known B043 despite the ordinary type0B diversion; E5E8 has six pairs, ending E61F. Transient selectors are biased81/82 or41/42; slot0 overlapped a preceding80. Third null storage is Data without inventing a renderer null skip.

Rejected B092/B098 sprite parsing remains recorded. Its no80/Code-overlap failures reject the pointer interpretation only: A152 compares B092 against SupportType and stores B098 into0594, supporting numeric Data. generic-results.json records full-byte/reference/neighbor inspection for other bounded numeric and sprite-format bodies. No semantic name or original serializer is a generic-Data prerequisite.

Inventory EC4B is another grammar:22 Y/tile/X triples+80=67bytes to next known code(remaining-data-results.json). biased-references.json fixes searches that missed bases just before unknown storage: C218,C9BC/C9BE,CBD3,D2D2,F9BE. F9BF's apparent RTS60 is copied setup data. Next commentary should correct inherited counts:82FC reads X0C..1=**12**, not13 bytes; FEEA reads Y14..1=**20**, with neighboring first byte only a generic storage claim.

## Source flow, rejection scope and residuals

The old postcall scan only covered regions containing missing direct callees. Whole-current-source scanning finds A025's balanced register restore and inventory E8B6 post-wait loop. C0AB's existing task-sensitive return context is required; it was not called neutral. E8BE's00/18 are inline data; C0E1 polling returns and BEQ reuses E8B3. F116 alternate renderer,998A score/glyph helper,CC13 shifts,C0A8 fill/wait andFE71/FE82 display prefixes have coherent whole-routine contracts; actual callers unresolved.

Null-target JMP slots are Code proposals from the complete executable3-byte JMP table and validated8000 indirect-dispatch ABI. CPU enters the ROM opcode itself. RAM0000 remains unresolved/unclassified, with no natural-use/dead/trap claim.

gfx-container-results.json retrieves [primary external codec records](../r1-20261119-a/external-codec-results.json) and [r1 report](../r1-20261119-a/report.md). Slot41's1byte/slot51's2bytes are not consumed by finite-output parsing. That rejects consumed-payload claims. Narrow generic storage combines finite read ends, separate positive encoded ends, next independent sources46/49 and graphics-container context. Slots40/42 end=start still falsify a global compressed-end rule. This localized three-byte proposal explicitly needs independent review.

residual-assessment.json primary-links all37 conditional bytes:16heterogeneous852E..853E; six isolated RTS; nine old-bank FFEB shadow JMP bytes+two RTIs; four ambiguous BC11..BC15 bytes. These are role ambiguities, not missing names. Static typed reads, meaningful whole-routine contracts or container/table evidence could resolve them; natural callers are not universal prerequisites. No identified concrete method is deferred merely because independent. Remaining37 is not an acceptance target or terminal-exhaustion claim.

## Actual artificial milestone and validation

New room29 initialization reused active1/task0 with Room1D,PC8D23/SP26. Eight bounded load calls retained partial timeouts. Initial1376image shows a distinct three-white-fixture/planked room/player/door. New066F=2F edit follows F65D's old-A comparison: increment30 and actual bridge1F682/loader18D33 fetches; room7 by1392. Settled1568image confirms yellow-striped chair/banquet room7. Scripts/snapshots/inputs/budgets/state/captures are local; room29-execution.json distinguishes artificial edits from actual fetches. Settle import lacked another trace, so final settlement uses state/frame/image. Scenario history preserved; no unchanged ending replay or full natural/credits/boss claim.

audit.json checks physical offsets including header, CPU/effective8KiB banks, exact complement, identities and zero kind conflicts. boundary-check.json freshly decodes552instructions/45Code views with exact official ends: **bookkeeping, not structural proof**. Producer/consumer/bank/SP/control/format evidence supplies that proof.

No ROM/map/state/source/toolkit changes, cleanup/deletion/archive/publication/external message. Configured /root/romatlas/target/release/romatlas unchanged; all servers closed. Only existing NES-001 remains. Independently review exact unknown subintervals, preserve accepted overlaps/history, annotate accepted Static claims, verify a new checkpoint and complete full current-source commentary. Do not replay solved routes or force37bytes.
