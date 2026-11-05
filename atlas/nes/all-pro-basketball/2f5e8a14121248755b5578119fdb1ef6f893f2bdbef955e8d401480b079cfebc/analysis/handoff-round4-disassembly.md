# Round4 disassembly handoff — writer released

## Round4 disassembly — productive pass complete, ROM incomplete

ROM classification: 99.99% (262136 / 262160 bytes classified; 24 bytes unknown).

Accepted only20 new Static Code bytes [0x1F7E5,0x1F7F9), TeamChoicePromptSecondSound_UpperCopy, effective physical banks[10,11,14,15], CPUF7D5..F7E9. Totals30094Code/232042non-code. Independent original TeamChoicePromptSecond contract, same-context neighboring sound/first-prompt idiom and ordinary state/selector/return semantics establish this narrow suffix independently of the invalid preceding guard. Fresh configured-toolkit review confirms8instructions/3control edges to accepted C0C9/C0EA services, exact immutable-ROM hash and copy equality, correct physical contexts and exact boundaries. [Review](round4-disassembly/review.json), [decode](round4-disassembly/fresh-decode.json), [edges](round4-disassembly/fresh-edges.json), [primary claim review](redirect/r3-local-20261105/claim-review.md). No actor_flags!=2 prerequisite, Observed annotation or natural reachability claim. Connected tail-call/following wait graph already classified.

R3-PROMPT-SECOND-SUFFIX verified; R3-RESIDUAL-44 narrowed to24bytes. Remaining10intervals: [0x2410,0x2413), [0x659d,0x659e), [0xa410,0xa413), [0xe59d,0xe59e), [0x1c253,0x1c254), [0x1d357,0x1d35a), [0x1df41,0x1df42), [0x1ee13,0x1ee16), [0x1f7c9,0x1f7ca), [0x1f7de,0x1f7e5). Two JMP0000, three isolatedRTS, JMPED2C, byte1C, JMPEEAC, byteFA and7-byte differing-prefix guard retain code-versus-data ambiguity; review did not establish additional structural contracts. [Exact residual](round4-disassembly/residual.json). Prior rejection excludes guard only, no overbroad whole-suffix veto. No unchanged gameplay/artificial mechanics replay: existing representative progression includes finals/championship terminal and Start exit; existing scenarios and gameplay records preserved. This useful pass is not terminal exhaustion; full-source commentary and redirect remain required.

map.validate passed; saved analysis/state.json and exported maps/current.json; fresh [pass-004-handoff verification](pass-004-handoff-verification.json) and [checkpoint map](../maps/pass-004-handoff.json) confirm byte-identical rebuild. Exact state/export SHA256 0da986fefcd95e57c554fd52f993ad269cd2567d92cf94f8c6c45f2cbd056b06; input SHA256 unchanged 2f5e8a14121248755b5578119fdb1ef6f893f2bdbef955e8d401480b079cfebc. [Counts](round4-disassembly/final-counts.json). Execution coverage separate/unchanged. Existing commentary unchanged and must be refreshed.

MANDATORY NEXT COMMENTARY CORRECTION: Data Static physical128925..128969 prose wrongly says records endingFA. It is exactly eleven four-byte indicator records ending6C. FA at128969 is the following unknown byte and is not consumed by fixed-count F765 loop (X+=4, compare2C). Correct commentary/reading-after.asm physical anchor and regenerated source; preserve historical prior rationale and unknownFA. Also split previous27-byte unknown-copy prose to retain7-byte guard and describe newly accepted20-byte suffix.

Toolkit report: no toolkit issues found (0confirmed/0suspected/0missing-feature findings, no blockers); [persistent report](toolkit-issues.md). Configured release binary unchanged. No toolkit edits, archive/deletion/cleanup, publishing or subagents. Disk8.5GiB available; no scratch cleanup required. Writer released after completed checkpoint and records; no live RPC operations.

| Bank | Code | Data/header/image | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG 00 | 1676 | 6516 | 0 |
| PRG 01 | 173 | 8016 | 3 |
| PRG 02 | 0 | 8192 | 0 |
| PRG 03 | 686 | 7505 | 1 |
| PRG 04 | 1676 | 6516 | 0 |
| PRG 05 | 173 | 8016 | 3 |
| PRG 06 | 0 | 8192 | 0 |
| PRG 07 | 686 | 7505 | 1 |
| PRG 08 | 700 | 7492 | 0 |
| PRG 09 | 78 | 8114 | 0 |
| PRG 10 | 831 | 7361 | 0 |
| PRG 11 | 3500 | 4692 | 0 |
| PRG 12 | 4129 | 4063 | 0 |
| PRG 13 | 2931 | 5261 | 0 |
| PRG 14 | 5815 | 2372 | 5 |
| PRG 15 | 7040 | 1141 | 11 |
| CHR 00 | 0 | 8192 | 0 |
| CHR 01 | 0 | 8192 | 0 |
| CHR 02 | 0 | 8192 | 0 |
| CHR 03 | 0 | 8192 | 0 |
| CHR 04 | 0 | 8192 | 0 |
| CHR 05 | 0 | 8192 | 0 |
| CHR 06 | 0 | 8192 | 0 |
| CHR 07 | 0 | 8192 | 0 |
| CHR 08 | 0 | 8192 | 0 |
| CHR 09 | 0 | 8192 | 0 |
| CHR 10 | 0 | 8192 | 0 |
| CHR 11 | 0 | 8192 | 0 |
| CHR 12 | 0 | 8192 | 0 |
| CHR 13 | 0 | 8192 | 0 |
| CHR 14 | 0 | 8192 | 0 |
| CHR 15 | 0 | 8192 | 0 |
