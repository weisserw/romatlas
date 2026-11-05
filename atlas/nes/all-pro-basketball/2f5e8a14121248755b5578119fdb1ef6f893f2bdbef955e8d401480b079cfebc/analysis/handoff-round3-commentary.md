# Round 3 commentary — full current-source readability complete; writer released

Fresh final byte-identical verification passed; reading-after.asm refreshed. Verification: analysis/round3-commentary/verification.json. Exact inventory, carry checks and hashes: analysis/round3-commentary/readability-check.json. No final editorial audit was run; editorial_audit remains pending for final acceptance.

All 30074 Code bytes (13977 decoded instructions), 232042 classified non-code bytes and the 44 unknown bytes across ten boundaries have source explanations. The intact 5194-entry full-ROM review scaffold is reviewed, with 576 meaningful semantic units. All 475 prior semantic unit names and physical starts were retained, as were all 1764 prior source names at their physical instruction positions. Entire prior commentary is preserved at analysis/round-3/commentary-round2. Source was rebuilt from the fresh frozen baseline and physically carried semantic records/aliases, not copied stale assembly. Four older summaries lost obsolete gap qualifications; other prior credit and useful prose remain.

The complete delta covers 2768 Code / 129 Data bytes, plus affected older dependencies. New source includes named direct column drawing and 24-byte pointer calculation, mirrored sprite rendering, sixteen-byte PPU queue transfers, boundary/trajectory/AI/pairing/score continuations and selected-bank wrappers. Source distinguishes direct C0CF writes (safe rendering interval required) from queued C0CC rendering. Saved PRG latches and possession-half flags have explicit stack ownership/restoration. The sixteen-byte velocity table is four rows of X integer/fraction and Y integer/fraction, bounded by a return and the next routine. Reserved sound DEY/BRK entries skip the following $60 signature; that signature is non-code, not an executed RTS.

Upper-window menu/password copies inherit established mechanical routine explanations under effective physical banks [10,11,14,15]. Relative branches remain inside the matched copy while absolute B references retain original bank-11 dependencies. Natural entry into these historical copies is unresolved. The dangling source-branch operand and entire routine branching into a differing prefix remain unknown; no actor_flags != 2 precondition was invented.

No classified-source readability gaps remain deferred. Unknown intervals have local notes for two JMP0000 stubs, three isolated RTS bytes, two unreferenced jump stubs, the single $1C ambiguity, dangling copied operand and 27-byte invalid-prefix copy. Exact graphics identities, some AI/event names and natural copy selection remain uncertain; established mechanics are described nearby. Existing detailed claim-local questions remain in analysis/redirect/r2-contracts-20261105/contract-review.md and unresolved-review.json. Redirect can seek narrower discriminating entry/consumer or copy-origin evidence; repeated artificial guard-false execution would not resolve intended contract. No additional structural acceptance or terminal exhaustion is claimed.

ROM classification remains 262116/262160 (99.98%); 44 bytes unknown. This readability completion does not establish whole-ROM classification, universal natural reachability, full compatibility or final editorial acceptance.

Map/state SHA256: 37e56ce7aa76bf00e828d92335bd4d37c2da693ee14460e31dec4b9ba1ea7425.
Source game.asm SHA256: 016213bfac10c0b76d775faa31a6fa49bea8cdf80bf1d7baefd73ebdf645c3a6.
Reading copy SHA256: e40c8441047e359cb3b94e1316ac172d0640689c7bd3b3e022891a616cebbf7f.
Input SHA256 unchanged: 2f5e8a14121248755b5578119fdb1ef6f893f2bdbef955e8d401480b079cfebc.

No live runtime/server operations, toolkit edits, archival cleanup, deletion, publishing or external sharing. All artifacts remain local. No new toolkit issue found. Writer released after full source gate.
