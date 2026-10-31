# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.


## Round 1 investigation result

ROM classification: 65.50% (171722 / 262160 bytes classified; 90438 bytes unknown).

Internal structural kinds: Code 23118, Data 17516, Image 131072, Text 16. Unique classified-byte union was checked for overlap; raw fallback and gaps do not count. Header16/16 classified. All sixteen CHR 8KiB banks8192/8192 classified as dedicated pattern Image;0 unknown. Exact physical residual intervals: [current-counts.json](evidence/current-counts.json).

| PRG 8KiB bank | Classified | Unknown |
|---:|---:|---:|
| 00 | 1050 | 7142 |
| 01 | 1006 | 7186 |
| 02 | 8001 | 191 |
| 03 | 8192 | 0 |
| 04 | 2563 | 5629 |
| 05 | 327 | 7865 |
| 06 | 93 | 8099 |
| 07 | 49 | 8143 |
| 08 | 2731 | 5461 |
| 09 | 2102 | 6090 |
| 10 | 2802 | 5390 |
| 11 | 779 | 7413 |
| 12 | 2931 | 5261 |
| 13 | 215 | 7977 |
| 14 | 4633 | 3559 |
| 15 | 3160 | 5032 |

Latest state [state.json](state.json), portable ROM-free map [current.json](../maps/current.json), exact checkpoint [pass-004.json](../maps/pass-004.json), source [game.asm](../generated/pass-004/game.asm). [Verification](pass-004-verification.json): ok=true, byte_identical=true; map SHA256 d8c5f7e719b59cb825ed5679edfa957367d48bd91ecf98947d097bf4b26e95c7. Baseline and pass001/002/003 histories preserved. Input SHA256 remains a2ae47c68eae003698dfee57ab29e64560785e1637699e05ce99d913e646ac39.

Evidence-backed gains: natural startup/title/eight portraits/first doorway/main menu/Goldmoon status/mage spell menu and Charm selection; artificial supported transition visits maps20/40/52 plus0F entrances1/3 and2B entrance2. Distinct rooms, combat/pits and waterfall cutscene observed; dragon and ending not reached. Broad best effort used clean supported transition snapshot and exact A/X edits, with no ROM patch. Short Select presses failed; Select30 plus fade wait works. Spells/items are not claimed fully understood; Charm selection returned to Main Menu, effect not established in an empty room. Clerical-menu follow-up was interrupted externally before durable result, so no successful clerical visit claimed. See [scenarios](scenarios.json), [gameplay](gameplay.md) and primary selected frames/snapshots.

Static gains: header/CHR layout; accepted contextual instruction fetches and bounded direct branches/callees; MMC1 literal bank-helper entry contracts including JMP tail transfers; bounded uniform non-code; sparse bank1 record-data block; exact eight-entry party/health/auxiliary tables; debug flag/title fragment; hexadecimal tile table; eleven four-byte room-transition hooks. Bank03 debug renderer has independent LDA3/JSR E97C/JMP8010 caller and meaningful PPU consumer/stack contracts, despite debug flag disabling natural use. Pointer targets alone were never marked execution. Generic record meaning remains unresolved without weakening supported non-code classification.

All cited trace summaries have aggregate_dropped=0. Detail drops were expected with capacity0 (or initial2000); untraced waits do not establish execution. Execution budgets/timeouts and externally terminated subprocesses are recorded, not silently counted as successful full runs. Direct bounded RPC batches now replace fragile long-lived shell queue sessions; no live server is required. Replay scripts retained in [replays](replays).

This is an incomplete single disassembly pass, not terminal exhaustion or whole-ROM/editorial completion. Sampled immediate code/data gains were taken; remaining unknown spans need focused consumer/dispatch/bank-return research, and boss/ending need a supported scene/event contract. Ready questions are in [leads](leads.json); next coordinator phase is full current-source commentary, then redirect. No commentary or redirect was run here.

Toolkit report: **no toolkit issues found**;0 confirmed bugs,0 suspected bugs,0 missing features,0 toolkit blockers. [toolkit-issues.md](toolkit-issues.md). Disk at handoff:11GiB free, scratch344KiB, selected evidence about126MiB (before final small notes/scripts); no cleanup, deletion, archive, publishing or toolkit changes performed.

## Round 2 disassembly completed — current structural checkpoint

ROM classification: 84.49% (221500 / 262160 bytes classified; 40660 bytes unknown).

Unique internal non-Unknown union, no overlaps: {'Code': 27582, 'Data': 62830, 'Image': 131072, 'Text': 16}. Header16/16; all16CHR8KiB banks8192/8192Image,0unknown each. Counts and exact residual intervals: [final-counts](disassembly/r02-20261031/final-counts.json). Previous current-counts preserved in [input-pass004-counts](disassembly/r02-20261031/input-pass004-counts.json).

| PRG8KiB bank | Classified | Unknown |
|---:|---:|---:|
| 00 | 6223 | 1969 |
| 01 | 7469 | 723 |
| 02 | 8001 | 191 |
| 03 | 8192 | 0 |
| 04 | 2667 | 5525 |
| 05 | 327 | 7865 |
| 06 | 8176 | 16 |
| 07 | 8192 | 0 |
| 08 | 2731 | 5461 |
| 09 | 2224 | 5968 |
| 10 | 5962 | 2230 |
| 11 | 7938 | 254 |
| 12 | 7757 | 435 |
| 13 | 3863 | 4329 |
| 14 | 5166 | 3026 |
| 15 | 5524 | 2668 |

Gain49778bytes: reviewed redirect40274Staticbytes (3920Code/36354Data), saved-runtime255physicalinstructions/536Observedbytes, connected4instructions/8Staticbytes, PPUtilecontainers6784StaticDatabytes, hardware-derivedDMCpayload2176StaticDatabytes. Contextual duplicates are merged into physical views before counting. [Independent graph/data review](disassembly/r02-20261031/review.md), [connected/data review](disassembly/r02-20261031/extra-review.md), [accepted plan](disassembly/r02-20261031/accepted-plan.json), [extra plan](disassembly/r02-20261031/extra-plan.json).

All63redirect proposals independently accepted through map.annotate; no research map import. New graph nodes matched3721fresh immutable decodes, code bounds/overlaps checked, inline dispatch/indirect audio/mapper barriers preserved. D002 narrative correction now resolved in actualmap; R001/R002/R003 verified with histories preserved. Saved redirect gameplay evidence contributes bank-aware actual fetches, including artificial scenes; no new gameplay replay, natural item recovery/boss defeat/win notclaimed. Representative later-room/dragon/postboss/result/artificialterminal/cleric milestones retained and reviewed in scenarios.

Latest [state](state.json), ROM-free [current map](../maps/current.json), [pass007map](../maps/pass-007.json), [source](../generated/pass-007/game.asm); [verification](pass-007-verification.json) ok=true,byte_identical=true. MapSHA256bf28dac8f65894a4d686b50a3002fc514605f53ca143018d9220911f7e8136b4. pass006 preserves firstacceptedbatch; pass005 was unchanged-input verification after local pre-mutation guard rejected duplicatephysicalcontexts, then corrected before acceptance. All checkpoints retained. Input hash unchanged; [identities](disassembly/r02-20261031/final-identities.json). Prior commentary/source untouched and requires the separate full current-source commentary phase for new classifications.

Single disassemblypass incomplete; no terminalexhaustion or whole-ROM/editorial completion claim. Obvious connected discovery adds only8bytes after batch; broad residuals remain available for redirect, particularly mixed bank2/4 actor code/data, bank0 helper tails and bank6 subsequent dispatch families. Exact residuals are not blanketdata/reachability rejections. No further long gameplay repetition warranted by this batch: all representative milestones already have reusable snapshots/evidence, naturalinventorygate is semantic rather than structural prerequisite. Compact [handoff](handoffs/round-02-disassembly.md).

Toolkit: **no toolkit issues found** (0confirmed/0suspected/0missingfeatures/0blockers), [report](toolkit-issues.md). Configuredbinary unchanged; no toolkit changes, archive, cleanup/deletion or publishing.

## Round 3 disassembly completed — pass-008

ROM classification: 100.00% (262156 / 262160 bytes classified; 4 bytes unknown).

**Incomplete:** percentage rounds to100.00%; four physical bytes are stillUnknown. Unique nonoverlapping union {'Code': 48230, 'Data': 82838, 'Image': 131072, 'Text': 16}. Header16/16classified; each of16CHR8KiB banks8192classified/0unknown. Exact counts and four residual intervals: [final-counts](disassembly/r03-20261031/final-counts.json). Prior current-counts preserved as input-pass007-counts.json in the same directory.

| PRG8KiB bank | Classified | Unknown |
|---:|---:|---:|
| 00 | 8192 | 0 |
| 01 | 8192 | 0 |
| 02 | 8192 | 0 |
| 03 | 8192 | 0 |
| 04 | 8191 | 1 |
| 05 | 8192 | 0 |
| 06 | 8192 | 0 |
| 07 | 8192 | 0 |
| 08 | 8191 | 1 |
| 09 | 8192 | 0 |
| 10 | 8192 | 0 |
| 11 | 8192 | 0 |
| 12 | 8190 | 2 |
| 13 | 8192 | 0 |
| 14 | 8192 | 0 |
| 15 | 8192 | 0 |

Accepted40656new Staticbytes (20648Code/20008Data),503regions,0rejections. [Independent review](disassembly/r03-20261031/review.md), [decisions](disassembly/r03-20261031/decisions.json), [annotation plan](disassembly/r03-20261031/accepted-plan.json). Fresh configured-binary27041context instruction decodes plus16manualcode regions matched exact bounds; no physical overlaps or checked control-flow references into data. Independent least-fixed-point return computation agrees on999functions after5iterations,164literalPRGswitches verify actualLDA predecessor, and1032stack entry checks have0failures with explicitNMI/indirect contracts.133data intervals reviewed through consumers, pattern and boundary checks; independent checksum sums and1608byteRAMcopy source union confirmed. Structural role and natural reachability remain separate.

Only physical8345/11CF8/18267/18D4D, each60, remainUnknown. Fresh direct-reference scan finds no resolved reference; prior raw8257pointer has bank3sprite context, not bank6audio. Supported ordinary graph continuations and existing downstream structures included, no imposed byte quota. No useful additional ordinary continuation into these isolated bytes is established; emptyroutine/separator distinction remains open for redirect, not a universal missingcaller rejection. Singlepasscomplete, nonterminalhand-off; no forcedfullclassification/editorialclaim.

Representative laterroom/dragon/postboss/result/artificialterminal/cleric scenarios reused; gameplay routes/patches/budgets preserved. No newemulatorrun orstartuprepetition. Natural questrecovery/win unresolved, not a structural prerequisite.

Latest [state](state.json), [ROM-freecurrentmap](../maps/current.json), [pass008map](../maps/pass-008.json), [source](../generated/pass-008/game.asm), [verification](pass-008-verification.json): ok=true,byte_identical=true. MapSHA256 3df957a4581777415f8a52d2a00afc96e303f49425924871cca2c2fb82fc38a1. Prior commentary/source/game.asm untouched (SHA d7dfc2fae4eabff800b926d77686794e3874021d1c3af5e6f838c97ef03e76ea); full separate current-source commentary is next. InputROM unchanged; [identities](disassembly/r03-20261031/final-identities.json). Compact [handoff](handoffs/round-03-disassembly.md).

**No toolkit issues found** (0confirmed/0suspected/0missingfeatures/0blockers), [report](toolkit-issues.md). No toolkitmodifications, archive, cleanup/deletion or publishing. All raw state/source/assets remain local. Disassembly writerquiescent.
