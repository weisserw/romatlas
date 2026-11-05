# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## Round 1 disassembly live ledger
- R1-VECTORS verified: validated hardware vector words at physical 0x2000A..0x2000F (CPU FFFA-FFFF), reset entry physical 0x1FFE0, NMI 0x1C27C, IRQ 0x1C2BD; startup MMC1 effective PRG banks [0,1,14,15]. Static traversal stops at call/mapper barriers.
- R1-RUNTIME verified: natural title -> New Game -> player roster -> CPU roster -> direction screen -> arena intro -> first period/tipoff -> active court. Exact replay in round1-disassembly/runtime1.py through runtime9.py, with branch snapshots; screenshots are evidence of visited state, accepted fetch roots support code. Long transitions required settling, not unexplained state patches.
- R1-STATIC verified: bank-aware fetched-root and direct-call/JMP closure; corroborated bank service JMP tables. expanded-candidates/references/diagnostics preserved. Copied exact first-32-KiB duplicate sound routines as Static with bank indices adjusted; not Observed.
- R1-DATA verified: full 16-byte patterned record domain banks 2/6 and bounded sparse bitfield slices (generic data; semantics unresolved); exact duplicate physical block checked. data-pattern-review.json; no conflicting code annotations.
- R1-CLOCK verified mechanics: physical 0x18DA1 initializes ASCII `10m00` in RAM $6C-$70. Fixed-bank physical 0x1F167 countdown checks $9C, decrements divider $6B, borrows ASCII seconds/minutes, and sets expired $9C=1. Expiry handler physical 0x1D7A9 sets $9C=2, game mode $27=9 and substate $80=0. Natural active snapshot r9-freeplay.snapshot retained. Artificial patch $6B..$70=[1,$30,$30,$6D,$30,$30], 120 frames, actually reached expiry mode 9/substate2 with $9C=2 (r9-expire.snapshot/RAM/screen). This demonstrates artificial period expiration, not naturally elapsed ten minutes.
- R1-PERIOD underway: period index $92 increment/compare 4 at physical 0x1A393; ASCII score digits $77-$79 vs $7A-$7C compared at 0x1A3A8. Period-end substate2 waits camera $0F=0/$11=$90, then timer $01B4=$78; later timer advances period. Testing bounded transition before final-period shortcut. Finals password `BEOLEHPPHRMR` sourced but untested; no semantic classification rests on external FAQ.
- R1-PERIOD verified: r10-period-transition screen displays END OF 1ST PERIOD. RAM $0F=0/$11=$90 satisfies statically reviewed camera-return gate; mode9/substate3, timer53. Restoring that snapshot and setting understood cosmetic $01B4=1 twice (2-frame then120-frame budgets) produced actual 2ND PERIOD screen, $92=1/mode0/substate1, r11-nextperiod.snapshot.
- R1-RESULT verified: independent branch from first-period-end snapshot sets $92=3 and ASCII scores $77..$7C=100:000, plus the same two cosmetic timer skips (2 then160frames). Actual fourth-period completion chooses mode11/period4 and victory/score screen. Subsequent bounded result waits show WINNER 100:000, r13-resultconfirm and r13-resultcontinue artifacts. This is an artificial match result, not a naturally won game, league final, championship or credits.
- R1-NATURAL-MECHANICS verified route: from r9-freeplay.snapshot, runtime14 Start10/release10/Start10 changes paused/resumed processing; movement+A60, B10/release10/B10/release60 exercise active player/jump/shot input paths. Exact source scripts and per-step context/RAM/state/snapshot evidence retained. No basket/free-throw/dunk claim without corresponding frame/event evidence.

### Compact continuation/rejection review (not terminal exhaustion)
Strongest residual candidates are mixed PRG gaps around established engine routines and bank-aware indirect state dispatch. Legal pointer values and decodes alone do not classify these gaps; practical next evidence is consumer/selector closure or clear bounded repeated-format slices. The accepted bank2/6 aligned records and sparse bitfields demonstrate that consumer semantics are not required for obviously non-code portions; no blanket rejection of residual generic data is intended. Some short uniform runs were inspected but not promoted because neighboring record/code boundaries need tighter inspection; this does not reject narrower non-code slices.
The sourced finals password is a concrete late-game experiment. What is missing is this ROM's Continue editor buffer/character encoding and initialization route, not proof of natural final-game reachability as a prerequisite for code classification. The actual result-state graph now supplies productive evidence for redirect to connect password generation, tournament bookkeeping, final selection and ending rather than replaying menus or brute-forcing RAM.

## Round 1 final verification and counts

ROM classification: 70.76% (185492 / 262160 bytes classified; 76668 bytes unknown).

Checkpoint pass-001-handoff validated and byte-identical. Exact unknown intervals, opcode/context counts and input identity are in round1-disassembly/final-counts.json. Commentary pending; no terminal exhaustion assessment.

| Bank | Code | Data/image/text | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG 00 | 1180 | 0 | 7012 |
| PRG 01 | 0 | 3072 | 5120 |
| PRG 02 | 0 | 8192 | 0 |
| PRG 03 | 0 | 5120 | 3072 |
| PRG 04 | 1575 | 0 | 6617 |
| PRG 05 | 89 | 3072 | 5031 |
| PRG 06 | 0 | 8192 | 0 |
| PRG 07 | 0 | 5120 | 3072 |
| PRG 08 | 626 | 0 | 7566 |
| PRG 09 | 78 | 2816 | 5298 |
| PRG 10 | 699 | 0 | 7493 |
| PRG 11 | 1728 | 2048 | 4416 |
| PRG 12 | 919 | 0 | 7273 |
| PRG 13 | 699 | 0 | 7493 |
| PRG 14 | 4637 | 0 | 3555 |
| PRG 15 | 4536 | 6 | 3650 |
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

## Round 2 disassembly — completed pass, ROM incomplete

ROM classification: 98.87% (259204 / 262160 bytes classified; 2956 bytes unknown).

Checkpoint pass-002-handoff: map.validate, CLI map validation, generation and byte-identical rebuild succeeded; [verification](pass-002-handoff-verification.json). State/export SHA256 0deaa3379ced9a0d2ef44df6546ff30ca2718fb923902a1006121e0dde4b8cb8. Input unchanged. +73712 newly classified (10525 Code/63187 Data), totals 27291 Code/231913 non-code. Exact counts/residuals [final-counts](round2-disassembly/final-counts.json), [residual](round2-disassembly/residual.json). All classified regions retain structural evidence; execution and readability are separate. Existing commentary/source remains intact; new classification requires next full current-source commentary.

Six ready leads independently reviewed and verified with localized bounds: indirect scenes/records; password/finals; mixed data; news contexts/physical duplicate; fixed callback domains; narrow late-copy RAM routines/patterns. Consumer and primary byte/pattern review plus [mechanical audit](round2-disassembly/review.json) supports acceptance of combined 73605 bytes. No broader late-copy execution claim, no table overrun, no unchanged gameplay replay. Additional connected [review](round2-disassembly/extra-review.json):14 static code bytes from direct targets and93 Data bytes with seven bounded consuming/patterned intervals. Stack-sensitive EE4A PLA remains unknown pending its entry/stack contract.

Saved runtime evidence already exercises final match, trophy/championship terminal and Start exit through documented artificial routes. No natural tournament completion or separate credits claim. Existing natural/artificial execution evidence retained; no new runtime scenario. Deferred free-throw/dunk initialization requires actor/scene contract; representative active mechanics and final/ending progression already visited.

Productive pass handoff, not terminal exhaustion:2956 residual bytes include coherent but entry/stack-sensitive fixed fragments, additional bank8 composite helper continuations, and broader late-copy dependent fragments. Direct branch/call closure found only15 new candidate bytes:14 accepted; one stack-sensitive PLA deferred. Strongest remaining paths need consumer/stack or effective-mapping research, not pointer decoding or unchanged replay. Full-source commentary then redirect should inspect these localized contracts.

No toolkit issues found; [report](toolkit-issues.md). No toolkit edits, archival, deletion, publishing or live operations. Writer released at this pass handoff.

| Bank | Code | Data/header/image | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG 00 | 1674 | 6515 | 3 |
| PRG 01 | 173 | 8004 | 15 |
| PRG 02 | 0 | 8192 | 0 |
| PRG 03 | 686 | 7505 | 1 |
| PRG 04 | 1674 | 6515 | 3 |
| PRG 05 | 173 | 8004 | 15 |
| PRG 06 | 0 | 8192 | 0 |
| PRG 07 | 686 | 7505 | 1 |
| PRG 08 | 626 | 7492 | 74 |
| PRG 09 | 78 | 8114 | 0 |
| PRG 10 | 699 | 7361 | 132 |
| PRG 11 | 3500 | 4692 | 0 |
| PRG 12 | 4129 | 4050 | 13 |
| PRG 13 | 2926 | 5233 | 33 |
| PRG 14 | 5199 | 2338 | 655 |
| PRG 15 | 5068 | 1113 | 2011 |
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

## Round2 authoritative final checkpoint — pass-002-reviewed

ROM classification: 98.88% (259219 / 262160 bytes classified; 2941 bytes unknown).

Final connected wrapper review accepted15 more Static bytes at physical1F66F..1F67E (CPU F65F). Complete save-latch/select3/callA400/restore wrapper has a balanced stack contract, demonstrated separately from unchanged-bank traversal. See round2-disassembly/wrapper.py. Previous pass-002-handoff remains preserved as an intermediate checkpoint. Final total added73727 bytes (10540 Code/63187 Data); totals27306 Code/231913 non-code. Map/state SHA256 c729ce4e85bf60f52a390ee488c35c291e915a0fa27ab927b23d8e0501adb176. Fresh maps/pass-002-reviewed.json, generated/pass-002-reviewed and analysis/pass-002-reviewed-verification.json validate and rebuild byte-identically. ROM input unchanged; existing commentary/source untouched.

All six ready leads verified; exact final per-bank counts and2941-byte complement saved. No terminal exhaustion claim. Full-source commentary then redirect remains appropriate; closed F65F wrapper is accepted, no longer a deferred candidate. No toolkit issues found. Writer released; no live server operations.

| Bank | Code | Data/header/image | Unknown |
|---|---:|---:|---:|
| Header | 0 | 16 | 0 |
| PRG 00 | 1674 | 6515 | 3 |
| PRG 01 | 173 | 8004 | 15 |
| PRG 02 | 0 | 8192 | 0 |
| PRG 03 | 686 | 7505 | 1 |
| PRG 04 | 1674 | 6515 | 3 |
| PRG 05 | 173 | 8004 | 15 |
| PRG 06 | 0 | 8192 | 0 |
| PRG 07 | 686 | 7505 | 1 |
| PRG 08 | 626 | 7492 | 74 |
| PRG 09 | 78 | 8114 | 0 |
| PRG 10 | 699 | 7361 | 132 |
| PRG 11 | 3500 | 4692 | 0 |
| PRG 12 | 4129 | 4050 | 13 |
| PRG 13 | 2926 | 5233 | 33 |
| PRG 14 | 5199 | 2338 | 655 |
| PRG 15 | 5083 | 1113 | 1996 |
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

## Round3 disassembly — productive pass complete, ROM incomplete

ROM classification: 99.98% (262116 / 262160 bytes classified; 44 bytes unknown).

Accepted2897 new Static bytes:2768 Code/129 Data; totals30074 Code/232042 non-code. Independent localized contract review and fresh1309-instruction/317-edge audit support all92 proposed regions; this was not acceptance by byte count or valid decoding alone. [Review](round3-disassembly/review.json), [fresh decode](round3-disassembly/fresh-decode.json), [fresh edge audit](round3-disassembly/fresh-edges.json). Each region carries its exact rationale and primary evidence. Proposed direct targets were independently checked at instruction boundaries.

Four ready leads verified: R2-RESIDUAL-CONTRACTS (1410Code), R2-UPPER-COPY-CONTRACTS (1354Code), R2-SOUND-RESERVED-BRK (4Code/2Data), R2-RESIDUAL-DATA (127Data). Preserved direct-PPU C0CF versus queued C0CC distinction, safe-rendering precondition, inline16byte velocity barrier, saved-latch and saved-halfcourt stack balance, upperF context[10,11,14,15], and skipped BRK signatures. Upper-copy comparison excludes dangling operand and entire bad-branch routine. Runtime differential and sound tests are saved artificial mechanics corroboration; no universal natural-caller requirement or new observed-execution claim. All129Data bytes retain bounded consumer/neighbor review.

Saved/exported every eight regions. map.validate and CLI validation passed; fresh [pass-003-handoff verification](pass-003-handoff-verification.json) confirms generated byte-identical rebuilding. [Checkpoint map](../maps/pass-003-handoff.json), local generated/pass-003-handoff. State/export SHA256 37e56ce7aa76bf00e828d92335bd4d37c2da693ee14460e31dec4b9ba1ea7425; input unchanged. [Exact counts](round3-disassembly/final-counts.json), [44-byte/10-interval complement](round3-disassembly/residual.json). Classification annotations cover262116 bytes. Execution coverage is unchanged and separate; all added structure is Static. Existing commentary remains intact and now needs full current-source commentary refresh.

Connected batch includes graphics helpers, fixed AI/trajectory/pairing/score continuations, selected-bank wrappers and bounded tables; the saved94interval ledger was reviewed together with exact local exclusions. Remaining44 bytes are two JMP0000 stubs, three isolatedRTS, unreferencedJMP ED2C, single1C ambiguity, unreferencedJMP EEAC, dangling copied operand and27byte copy with branch into different prefix. [Localized review](redirect/r2-contracts-20261105/unresolved-review.json) and [contracts](redirect/r2-contracts-20261105/contract-review.md) retain claim-specific uncertainty. No unsupported RAMcode contract or0190!=2 precondition introduced. This is a productive pass handoff, **not terminal exhaustion**. Redirect should seek a genuinely discriminating narrower interpretation/consumer or copy-origin structural review, preserving scope of existing rejections; unchanged artificial success would not establish missing intended contract.

Broad progression evidence already includes artificial finals, championship terminal and Start exit. Existing successful routes and failed alternatives remain in scenarios.json; no unchanged gameplay replay offered a new discriminator for this frontier. No additional execution experiment this pass. Toolkit report: no toolkit issues found;0confirmed/0suspected/0missing-feature findings, no blockers. [Persistent report](toolkit-issues.md). No toolkit edits, archive/deletion/cleanup, publishing or live operations. Writer released after this checkpoint and records.

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
| PRG 15 | 7020 | 1141 | 31 |
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


## Round5 disassembly — 2026-11-05

ROM classification: 99.99% (262137 / 262160 bytes classified; 23 bytes unknown).

Accepted R4-MODE13-RETURN: physical [0x1C253,0x1C254), CPU C243, fixed PRG bank14, context [12,13,14,15], one Static Code RTS. Independent actual ROM/table/caller/dispatcher review and configured-binary fresh decode establish selected handler plus ordinary JSR owner return contract. Natural mode13 selection remains unresolved and unclaimed. The previous no-entry rejection overlooked interior table index13; correction is recorded without rewriting history. Evidence: analysis/round5-disassembly/review.json, fresh-decode.json, rpc.json. No gameplay, runtime edits or unchanged failed-lead replay. Connected continuation returns to accepted C1A7; no additional supported unknown byte. Other residual code/data decisions remain ambiguous per analysis/redirect/r4-mode-return-20261105/claim-review.md. No terminal exhaustion claim.

Saved and exported; map validation and fresh pass-005-handoff generation/rebuild succeeded byte-identically (analysis/pass-005-handoff-verification.json). All incoming regions preserved exactly; one region added. Map/state SHA256 82d320766ec544075688f17d16b57ad0e578d8ac6f49216312ba65db284ac439. Current generated source now requires commentary carry-forward/review; existing commentary files preserved. Final editorial audit remains pending. No toolkit issues found; see analysis/toolkit-issues.md.

Residual intervals: [2410,2413), [659D,659E), [A410,A413), [E59D,E59E), [1D357,1D35A), [1DF41,1DF42), [1EE13,1EE16), [1F7C9,1F7CA), [1F7DE,1F7E5). Full counts and exact per-bank ledger: analysis/round5-disassembly/result.json.

| Bank | Code | Other classified | Unknown |
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
| PRG 14 | 5816 | 2372 | 4 |
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


## Round6 disassembly — 2026-11-05

ROM classification: 99.99% (262142 / 262160 bytes classified; 18 bytes unknown).

Accepted R5-PROMPT-STATE-PREDICATE only [0x1F7DE,0x1F7E3), five Static Code bytes, LDA $0190/CMP #$02, CPU F7CE/F7D1 context [10,11,14,15]. Original specific paired-prompt predicate, current independent zero/two writes with C2/BF and C1/C0 text identities, and later same-field text/route consumers jointly establish local finite comparison role beyond decode/origin. Executable-form residue remains a global alternative, but these specific combined constraints support the local fragment. No standalone routine, complete guard, actual caller, natural use or observed execution claim. The BEQ [1F7E3,1F7E5) targets indicator fields at1F7C4 and remains Unknown. No state!=2 premise. Independent exact rationale: analysis/round6-disassembly/review.json. Fresh three-instruction decode includes excluded BEQ for negative scope; rpc.json records only supported five-byte annotation.

All incoming regions retained exactly; one region added. Connected suffix/services already classified, with no newly supported unknown continuation. Other residual claims retain the localized competing-role review in analysis/redirect/r5-frontier-20261105/claim-review.md; no unchanged runtime replay, RAM edits or gameplay repetition. Representative finals/championship terminal/Start-exit evidence reused. No terminal exhaustion claim.

Fresh map.validate/save/export and pass-006-handoff byte-identical checkpoint succeeded (analysis/pass-006-handoff-verification.json). Map/state SHA256 66d4dd2c3cd62fce5a5ac6b79879c9c5ac3a862ca9b81094401a93c33fb72e30. ROM unchanged. Exact per-bank and residual machine ledger: analysis/round6-disassembly/result.json and residual.json. Existing commentary preserved; new generated source requires full current-source commentary carry/review, including narrowing seven-byte guard notes to only excluded BEQ and preserving indicator FA exclusion. Final editorial audit pending. Commentary then redirect required. No toolkit issues found; analysis/toolkit-issues.md. No toolkit edits, cleanup, archive, publication or external sharing; server exited with no live operations.

Residual18bytes/9intervals: [2410,2413), [659D,659E), [A410,A413), [E59D,E59E), [1D357,1D35A), [1DF41,1DF42), [1EE13,1EE16), [1F7C9,1F7CA), [1F7E3,1F7E5).

| Bank | Code | Other classified | Unknown |
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
| PRG 14 | 5816 | 2372 | 4 |
| PRG 15 | 7045 | 1141 | 6 |
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


## Round7 disassembly — 2026-11-05

ROM classification: 99.99% (262144 / 262160 bytes classified; 16 bytes unknown).

Accepted R6-PROMPT-CONDITIONAL-FRAGMENT only [0x1F7E3,0x1F7E5), two Static Code bytes F0 DF, CPU F7D3 context [10,11,14,15]. Independently accepted LDA0190/CMP02 supplies Z directly to BEQ; accepted not-taken suffix writes the same state to two and displays C1/C0. Current paired zero/two prompt writes and later same-field consumers plus original specific conditional arrangement jointly support local instruction role beyond legal decoding or copy origin. Executable-form residue remains possible as use history, but no inline-data grammar or consumer supports splitting only this operation between matching accepted predicate and update. Exact review: analysis/round7-disassembly/review.json.

Taken edge F7B4 maps to physical1F7C4 inside accepted indicator Data and remains malformed. No target or bytes changed. Classification does not establish correct complete guard, callable routine, actual incoming edge, observed execution, natural use or state!=2. Fresh four-instruction decode confirms predecessor, conditional and not-taken successor; both successors mapped independently.

All incoming regions retained exactly; one region added. Connected predicate/suffix/services already classified; no new connected unknown callee. Other16 bytes retain localized competing-role review in analysis/redirect/r6-branch-role-20261105/claim-review.md. No unchanged runtime replay, RAM edits or gameplay repetition; representative finals/championship terminal/Start-exit records reused. No terminal exhaustion claim.

Fresh map.validate/save/export and pass-007-handoff byte-identical checkpoint passed (analysis/pass-007-handoff-verification.json). Map/state SHA256 fe9c9215f9062aef290d6c7d93d66aee3cc6419eb79ca892ac949aa8c6c3262c. ROM unchanged. Per-bank and residual machine ledgers: analysis/round7-disassembly/result.json and residual.json. Existing commentary preserved intact; next full current-source commentary must carry prior prose while replacing obsolete unknown-BEQ assertions with limited conditional Code role and malformed taken-edge warning. Keep FA exclusion. Final editorial audit pending. Commentary then redirect required. No toolkit issues found; analysis/toolkit-issues.md. No toolkit edits, cleanup, archive, publication or sharing. Server exited with no live operations.

Residual16bytes/8intervals: [2410,2413), [659D,659E), [A410,A413), [E59D,E59E), [1D357,1D35A), [1DF41,1DF42), [1EE13,1EE16), [1F7C9,1F7CA).

| Bank | Code | Other classified | Unknown |
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
| PRG 14 | 5816 | 2372 | 4 |
| PRG 15 | 7047 | 1141 | 4 |
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
