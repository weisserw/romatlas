# Round 2 commentary handoff

**Complete current-classified-source readability; writer quiescent.** Whole-ROM classification remains incomplete and final editorial acceptance is reserved. No final audit ran.

Current pass007 map/state SHA256 **bf28dac8f65894a4d686b50a3002fc514605f53ca143018d9220911f7e8136b4**, unchanged. Classification **84.49% (221500/262160; 40660 Unknown)**. Input ROM SHA256 remains a2ae47c68eae003698dfee57ab29e64560785e1637699e05ce99d913e646ac39. All 1725 scaffold records reviewed (1359 Code/52 Data/1 Text/1 Image/312 Unknown), 1163 units; [remaining readability frontier](../../commentary/pending-ranges.json) empty. Unknown bytes remain structurally unknown.

Preserved **entire** previous commentary at [commentary-pass004](../round-2/commentary-pass004/). All 1088 earlier classified records retained physical boundaries and valid source/review credit. New work: 306 Code fragments/4464 bytes in 71 groups, 19 Data intervals, 33 changed Unknown boundaries; affected older dependencies corrected. Local buildable [source](../../commentary/source/game.asm), [review](../../commentary/review.json), [progress](../../commentary/progress.md), [uncertainties](../../commentary/uncertainties.md).

Examples: anchored asymmetric collision test; word/byte division and eight-tile meter; result input/totals/decimal/inventory logic; audio page/repeat/single-return-slot semantics; dragon background phase rectangles and counted/clipped sprites; fixed bank-selection wrappers; inline table/data barriers and DMC hardware backing. Preserved implementation oddities: $C507 reads palette bytes without PPUDATA writes; PRG5/$96F7 writes the same sixteen bytes twice; dragon Y jitter is overwritten while X jitter survives; final DMC sample reads first $FD80 opcode.

Fresh [verification](../../commentary/verification.json): **ok=true, byte_identical=true**, source SHA256 **d7dfc2fae4eabff800b926d77686794e3874021d1c3af5e6f838c97ef03e76ea**. [Reading copy](../../commentary/reading-after.asm) refreshed from that exact source; [partition/status/location/hash checks](../commentary/r02/handoff-checks.json) passed. Editorial audit remains pending.

Remaining meanings: mixed room/sprite/audio/tile-container schemas and identities, weight/shape/palette selector domains, flag/mode meanings and natural gameplay reachability. Generic data reuses bounded primary reference/pattern/boundary evidence; no invented full schema. Concrete residual consumer questions are in uncertainties.md; reuse current312interval residual ledger and snapshots for the next redirect phase. No pending classified-source readability or active experiment.

No toolkit issue, toolkit edits, map changes, archive, cleanup/deletion or publication. ROM/source/assets/state/traces local. Free disk about 9.88 GiB; no cleanup authorization. Next phase may redirect under the coordinator; final acceptance remains separate.
