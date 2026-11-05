# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## R1 initial findings
- Baseline byte-identical verified; see baseline-verification.json.
- Vector/observed static closure selection-discovery.json supports 2694 code bytes (zero overlaps). Initial captures through intro-wait are natural idle attract mode due NES-001 input schema mismatch; preserve as valid fetched code, corrected reachability description.
- Runtime actual-menu.ppm proves ship bridge; A advances HELLO/teleport/Cyberia/security dialog. Further route under test.
- Sample candidate physical [0x4010,0x7F52): smooth amplitude bytes. Consumer discovered by bounded search for STA $4011: physical 0x1C83B onward sets pointer $2A/$2B=$8000, maps PRG bank pair 1, loops LDA ($2A),Y / LSR / STA $4011, increments pointer until high byte $BE. This establishes CPU $8000-$BDFF = physical [0x4010,0x7E10) PCM bytes with 2-byte prefix included by loop. Exact consumer endpoint independently bounds sample slice. Remaining suffix [0x7E10,0x7F52) visually continues sample; zero tail [0x7F52,0x8010) generic data. Decode contract and banks to confirm before annotation.

## R1 static discovery continuation candidate
NES discovery implementation conservatively stops all static propagation at JSR (changes_banks flag), including traversal to callee; this is documented conservatism, not evidence against callee code. A direct JSR in validated code supplies a supported entry contract with current effective banks at the target. Follow explicit call targets without assuming return-bank preservation; this can yield useful static-only paths. Do not seed return continuation unless separately supported or observed. Inspect mapper serial-write helper remaining instructions separately: fixed-bank helper shifts A and emits five writes then RTS, bank context of fixed code unchanged but lower slot mapping changes.

## Connected redirect frontier (not yet annotations)
Asset resolver fixed CPU$C77C (physical$1C78C) shifts selector, calls MMC1 bank helper, loads pointer base from CPU$8000/1, masks low selector-derived index with$3F, reads adjacent two bytes via($2A),Y and adds base. This is more than plausible pointer values: bounded consumer supports a finite relative-offset domain, but effective selected bank and actual object endpoints need exact decoded contract before a table annotation. PRG bank0 header word$99CE and analogous bank4/$999D, bank8/$9BE7, bank10/$A463 suggest packed asset containers. Header examples and repeated8-byte records inspected with xxd. Do not label all of those banks data blindly; bank10 contains proven code too. Useful redirect: decode resolver, establish selector mapping/32-entry relative table per relevant bank, inspect separate record/compressed extents, annotate supported connected structures plus generic non-code subranges.

## Round 1 phase close

ROM classification: 63.78% (167214 / 262160 bytes classified; 94946 bytes unknown).

Exact evidence-backed totals/code-data separation and per-bank counts below; raw export fallback excluded. maps/round1-disassembly.json validates and generated/round1-disassembly rebuilds byte-identically (analysis/round1-disassembly-verification.json). Full frontier and resume instructions: analysis/round1/handoff.md. Code15643, Data20499, CHR Image131072;715 evidence-bearing regions. Execution6487 distinct physical ROM opcode starts separate from classification, contextual roots13411; no aggregate drop. Final state/snapshot: analysis/round1/phase-final.snapshot and phase-final-state.json. Boss-engaged is a useful pre-ending artificial checkpoint; no ending or natural late-stage reachability claim.

R1-CALLS explicit JSR target inference is supported by source instructions and effective contexts, not bare pointers.7 contexts yielded22 classified bytes; revised scan0 remaining. R1-PCM accepted exact coherent routine68 bytes after complete source/control and hardware sink inspection; classified consumer-bounded sample15872 bytes, then322-byte smooth generic residual plus190 zeros. Strongest competing code interpretation of residual is not materially credible given whole-residual restricted amplitude pattern, adjoining sample and exact abrupt zero boundary; no semantic sample extent or original fill claim. Long uniform runs individually checked for conflicting current code; no generic all-bank claim. Asset-container candidates remain materially mixed with code and need bounded consumer/extent review, recorded as a ready redirect recipe rather than rejected for lack of execution. Boss/ending lead needs damage and transition contract, not arbitrary field brute force; normal shooting repetition gave17 bytes and did not defeat boss.

Toolkit: NES-001 confirmed documentation/API mismatch, no blocker, workaround validated. Other errors were bounded operation/time budgets or duplicate capture paths handled locally, not claimed toolkit defects. Preserve toolkit report and all prior files.

ROM classification: 63.78% (167214 / 262160 bytes classified; 94946 bytes unknown).

| Section | Bank | Code | Data | Image | Unknown |
|---|---:|---:|---:|---:|---:|
| header | - | 0 | 16 | 0 | 0 |
| PRG | 0 | 0 | 0 | 0 | 8192 |
| PRG | 1 | 0 | 179 | 0 | 8013 |
| PRG | 2 | 0 | 8192 | 0 | 0 |
| PRG | 3 | 0 | 8192 | 0 | 0 |
| PRG | 4 | 0 | 76 | 0 | 8116 |
| PRG | 5 | 0 | 1000 | 0 | 7192 |
| PRG | 6 | 862 | 0 | 0 | 7330 |
| PRG | 7 | 0 | 843 | 0 | 7349 |
| PRG | 8 | 0 | 0 | 0 | 8192 |
| PRG | 9 | 0 | 1241 | 0 | 6951 |
| PRG | 10 | 1306 | 0 | 0 | 6886 |
| PRG | 11 | 0 | 569 | 0 | 7623 |
| PRG | 12 | 2607 | 0 | 0 | 5585 |
| PRG | 13 | 1626 | 185 | 0 | 6381 |
| PRG | 14 | 6774 | 0 | 0 | 1418 |
| PRG | 15 | 2468 | 6 | 0 | 5718 |
| CHR | 0 | 0 | 0 | 8192 | 0 |
| CHR | 1 | 0 | 0 | 8192 | 0 |
| CHR | 2 | 0 | 0 | 8192 | 0 |
| CHR | 3 | 0 | 0 | 8192 | 0 |
| CHR | 4 | 0 | 0 | 8192 | 0 |
| CHR | 5 | 0 | 0 | 8192 | 0 |
| CHR | 6 | 0 | 0 | 8192 | 0 |
| CHR | 7 | 0 | 0 | 8192 | 0 |
| CHR | 8 | 0 | 0 | 8192 | 0 |
| CHR | 9 | 0 | 0 | 8192 | 0 |
| CHR | 10 | 0 | 0 | 8192 | 0 |
| CHR | 11 | 0 | 0 | 8192 | 0 |
| CHR | 12 | 0 | 0 | 8192 | 0 |
| CHR | 13 | 0 | 0 | 8192 | 0 |
| CHR | 14 | 0 | 0 | 8192 | 0 |
| CHR | 15 | 0 | 0 | 8192 | 0 |

## Round 2 disassembly completed — incomplete

ROM classification: 98.80% (259011 / 262160 bytes classified; 3149 bytes unknown).

Accepted 91797 unique new bytes:3977 Code (3780 redirect-proposed plus197 independently recovered direct continuations),87820 Data. Whole-ROM totals Code19620,Data108319,Image131072; unique counts exclude unknown raw export fallback. All prior regions preserved exactly. 2815 evidence-bearing regions; fresh checkpoint maps/round2-disassembly.json SHA256a50228a71f93e34e2c3da8fd3a35aeddaee01cc6f511437c6ed8bfc85c0f3e9a. analysis/round2-disassembly-verification.json confirms byte_identical=true, generated/round2-disassembly. State/current/checkpoint byte-identical; input unchanged.

Independent review: analysis/round2/review.md, independent-review.json, code-review.json/txt and accepted-continuations.json. Recomputed template records and sorted complete pointer domains (bank2 has a two-record permutation), packed/tree endpoints, all128 dispatch words, every code view and all conflicts. Unsupported operand-overlap KIL entry rejected at19C77; valid JSR still classifies that byte.39 ordinary-fallthrough continuation spans added197 bytes after complete local contract review. No runtime replay: prior representative late-game, machine/ship boss and initialized ending captures reused, preserving artificial/natural distinctions.

Ready leads R1-ASSETS,R1-ENDING,R1R-SCRIPT-DISPATCH,R1R-TASK-UI,R1R-STAGE-CONNECTED verified for their bounded structural batch; unresolved natural selection and original stream semantics remain separate. Exact residual complement: analysis/round2/unknown.json (35 gaps). Larger bridge/title/script gaps are concretely promising for redirect, but need inline-text/control boundaries and source entry context; unused/terminal claims are not made. This pass does not assert terminal exhaustion.

Retained actual opcode-root union:7282 physical starts,15612 contextual roots,all classified Code; this is separate from structural coverage and not a whole-game execution metric. See classification.json for artifact scope. Existing full-source commentary from round1 preserved byte-for-byte and remains valid for unchanged source; fresh generated source now needs separate full current-source commentary. Final editorial audit pending.

Toolkit:1 confirmed documentation/API issue NES-001,0 suspected,0 separate missing features,0 blockers; analysis/toolkit-issues.md. Configured release binary only; no toolkit, ROM, commentary, archive, publication, deletion or cleanup changes. Runtime review/apply processes closed.

| Section | Bank | Code | Data | Image | Classified | Unknown |
|---|---:|---:|---:|---:|---:|---:|
| header | - | 0 | 16 | 0 | 16 | 0 |
| PRG | 0 | 0 | 8192 | 0 | 8192 | 0 |
| PRG | 1 | 0 | 8192 | 0 | 8192 | 0 |
| PRG | 2 | 0 | 8192 | 0 | 8192 | 0 |
| PRG | 3 | 0 | 8192 | 0 | 8192 | 0 |
| PRG | 4 | 0 | 8192 | 0 | 8192 | 0 |
| PRG | 5 | 0 | 8192 | 0 | 8192 | 0 |
| PRG | 6 | 873 | 7207 | 0 | 8080 | 112 |
| PRG | 7 | 0 | 8192 | 0 | 8192 | 0 |
| PRG | 8 | 0 | 8192 | 0 | 8192 | 0 |
| PRG | 9 | 0 | 8192 | 0 | 8192 | 0 |
| PRG | 10 | 1306 | 6886 | 0 | 8192 | 0 |
| PRG | 11 | 0 | 8192 | 0 | 8192 | 0 |
| PRG | 12 | 3706 | 2021 | 0 | 5727 | 2465 |
| PRG | 13 | 2126 | 5803 | 0 | 7929 | 263 |
| PRG | 14 | 7591 | 428 | 0 | 8019 | 173 |
| PRG | 15 | 4018 | 4038 | 0 | 8056 | 136 |
| CHR | 0 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 1 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 2 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 3 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 4 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 5 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 6 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 7 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 8 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 9 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 10 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 11 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 12 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 13 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 14 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 15 | 0 | 0 | 8192 | 8192 | 0 |

## Round 3 disassembly completed — incomplete

ROM classification: 100.00% (262157 / 262160 bytes classified; 3 bytes unknown).

Rounded percentage is not completion. Unique whole-ROM totals Code21976, Data109109, Image131072; header included in Data. Accepted3146new bytes (2356Code/790Data),42proposal intervals split47regions at explicit bank-change boundaries;1114static instructions. All prior regions/evidence preserved exactly. 2862regions, 2862 evidence-bearing. Review analysis/round3/review.md independently checks allocator/text/scanner ordinary-return contracts, explicit text records, audio JMP and split handler entries, inventory dispatch, coherent CAF6 reservoir and C807 bank wrapper, and bounded arithmetic non-code tail. Fresh decode primary analysis/round3/independent-decode.json; checks/log alongside it.

State/current/fresh maps/round3-disassembly.json SHA256869ab8e8ec00b1c2902b1ad0aeb3379a741b6d162ebaba24cf881e57274388a9; map validated, generated/round3-disassembly verified byte_identical=true in analysis/round3-disassembly-verification.json. Input ROM unchanged. Existing commentary game.asm/symbols.inc hashes match redirect preservation. New generated source now needs full current-source commentary; final editorial audit pending. No terminal exhaustion claim.

Verified R2R-AUDIO-CONTRACTS,R2R-BRIDGE-INLINE,R2R-FIXED-CONTRACTS,R2R-FIXED-TAIL and pending R1R-TASK-UI structural ranges; natural use/selector identities remain separately unresolved. Exact unknown complement analysis/round3/unknown.json: physical1C74D..1C74F and1E93D..1E93E, all60. Null-entry versus separator/padding remain indistinguishable without another independent constraint; absent callers alone was not a rejection. Keep3unknown and forward review to separate commentary then redirect. No additional supported connected unknown interval remains outside that localized ambiguity; broad progression/scenarios and late/ending artificial observations reused without unchanged replay.

Execution metric unchanged: retained union7282physical opcode starts/15612context roots; scope retained primary artifacts, separate from structural classification, not whole-game coverage. No new runtime observation. Toolkit unchanged1confirmed NES-001 documentation/API issue,0suspected,0separate missing features,0blockers; analysis/toolkit-issues.md. Buttons-array workaround. Configured release only; no toolkit/ROM/commentary edits,archive/publication,deletion/cleanup. All processes closed.

| Section | Bank | Code | Data | Image | Classified | Unknown |
|---|---:|---:|---:|---:|---:|---:|
| header | - | 0 | 16 | 0 | 16 | 0 |
| PRG | 0 | 0 | 8192 | 0 | 8192 | 0 |
| PRG | 1 | 0 | 8192 | 0 | 8192 | 0 |
| PRG | 2 | 0 | 8192 | 0 | 8192 | 0 |
| PRG | 3 | 0 | 8192 | 0 | 8192 | 0 |
| PRG | 4 | 0 | 8192 | 0 | 8192 | 0 |
| PRG | 5 | 0 | 8192 | 0 | 8192 | 0 |
| PRG | 6 | 985 | 7207 | 0 | 8192 | 0 |
| PRG | 7 | 0 | 8192 | 0 | 8192 | 0 |
| PRG | 8 | 0 | 8192 | 0 | 8192 | 0 |
| PRG | 9 | 0 | 8192 | 0 | 8192 | 0 |
| PRG | 10 | 1306 | 6886 | 0 | 8192 | 0 |
| PRG | 11 | 0 | 8192 | 0 | 8192 | 0 |
| PRG | 12 | 5513 | 2679 | 0 | 8192 | 0 |
| PRG | 13 | 2389 | 5803 | 0 | 8192 | 0 |
| PRG | 14 | 7762 | 428 | 0 | 8190 | 2 |
| PRG | 15 | 4021 | 4170 | 0 | 8191 | 1 |
| CHR | 0 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 1 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 2 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 3 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 4 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 5 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 6 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 7 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 8 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 9 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 10 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 11 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 12 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 13 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 14 | 0 | 0 | 8192 | 8192 | 0 |
| CHR | 15 | 0 | 0 | 8192 | 8192 | 0 |
