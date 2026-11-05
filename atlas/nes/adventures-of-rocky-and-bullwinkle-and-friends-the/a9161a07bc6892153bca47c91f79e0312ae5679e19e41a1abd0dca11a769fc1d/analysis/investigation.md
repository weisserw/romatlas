# Investigation ledger

No regions reviewed yet. Follow workflow.md's completion and investigation contract.

## Round 001 — startup and progression work

Baseline and round-001-startup checkpoints verify byte-identically. Header/CHR storage/vector data and maximal uniform bank-tail blocks have independent physical evidence; 3745 connected code bytes have explicit CPU execution views. Early limited discovery prefix used wrong budget key; corrected NES parameter is `max_instructions`. Early input calls used generic schema and were ignored; corrected arrays are now used (NES-001).

Natural gameplay lead: first real scene reached at frame1355, analysis/evidence/warp-1.png and warp-1.snapshot, with Bullwinkle near Frostbite Falls and HUD. First controller-2 sequence was sent during black/loading interval at frame1201; its failure is localized to timing, not a rejection of the cheat. Long six-warp batch was externally terminated with exit143 after frame1601; snapshots warp-1..warp-3 preserve useful states; state was not overwritten by that batch. Do not reuse unchanged during-load recipe.

Lead CHEAT-001 (ready/under verification): exact eight-byte sequence at physical $3D82 is `40 80 80 10 80 40 40 80`; independent indexed consumer at $3CC7-$3CE4 compares $041D against table and stops at eight. Handler uses $078F enabled flag, and $0790 scene selector modulo11, indexes little-endian pointers at physical $3D6C..$3D82 (CPU $BD5C), invokes $D117 on selected script. Pointer values: $B93F,$B956,$B96D,$B984,$B99B,$B9B2,$B9C9,$B9E0,$B9F7,$BA0E,$BA25; 23-byte spacing suggests fixed scene records, but record extent/semantics beyond bounded selector remain hypotheses. See analysis/evidence/cheat-nearby.json (linear decode includes surrounding data and is not itself blanket code proof). Exact supported flag/index patches tested from clean initialized warp-1.snapshot; debug edits will establish artificial execution only. Controls are reasserted after restore.


## Round 001 disassembly completed — ROM incomplete

ROM classification: 67.21% (176201 / 262160 bytes classified; 85959 bytes unknown).

Code 13090 bytes; non-code 163111 bytes, comprising Data31878 + Text161 + Image131072. Unknown gaps/raw export fallback are excluded. All503 internal regions have evidence; all code has explicit CPU execution views. Saved-context union: 4342 unique accepted physical opcode starts (separate execution metric; includes natural and artificial scenarios). No saved opcode root conflicts with any classified data. Coverage is bounded and not full compatibility or complete gameplay.

Final state analysis/state.json and maps/current.json. Verified checkpoint [maps/round-001-disassembly.json](../maps/round-001-disassembly.json), [generated source](../generated/round-001-disassembly/game.asm), [verification](round-001-disassembly-verification.json). Byte-identical true; map SHA256 96563b18c7a8148113b94ee6122cc64360a8f5c3e4f4be32785fdea61627b8df; unchanged ROM SHA256 a9161a07bc6892153bca47c91f79e0312ae5679e19e41a1abd0dca11a769fc1d. Generation diagnostics empty.

| Bank | Code | Data incl. text/image | Classified | Unknown |
|---|---:|---:|---:|---:|
| header | 0 | 16 | 16 | 0 |
| PRG-00 | 2981 | 0 | 2981 | 5211 |
| PRG-01 | 2898 | 929 | 3827 | 4365 |
| PRG-02 | 0 | 0 | 0 | 8192 |
| PRG-03 | 0 | 2444 | 2444 | 5748 |
| PRG-04 | 0 | 0 | 0 | 8192 |
| PRG-05 | 0 | 2011 | 2011 | 6181 |
| PRG-06 | 0 | 0 | 0 | 8192 |
| PRG-07 | 0 | 5457 | 5457 | 2735 |
| PRG-08 | 0 | 0 | 0 | 8192 |
| PRG-09 | 0 | 6369 | 6369 | 1823 |
| PRG-10 | 0 | 0 | 0 | 8192 |
| PRG-11 | 0 | 5543 | 5543 | 2649 |
| PRG-12 | 0 | 3116 | 3116 | 5076 |
| PRG-13 | 0 | 842 | 842 | 7350 |
| PRG-14 | 6320 | 0 | 6320 | 1872 |
| PRG-15 | 891 | 5312 | 6203 | 1989 |
| CHR-000..CHR-015 (each) | 0 | 8192 | 8192 | 0 |


Exact machine-readable counts [classification-counts.json](classification-counts.json), [CSV](classification-counts.csv), and [unknown complement](unknown-ranges.json). Physical offsets include16-byte iNES header.

Broad visits: natural title/credits/level1 intro and Frostbite Falls; source-supported artificial selectors2 railway/logs,6 desert/plane,7 ship/water,8 island/Asian architecture,10 final indoor stage. Natural jump/bomb and character/special-input retries were exercised. Ending-edge attempt showed incomplete streaming initialization and no ending/boss verification. Full replay recipes, presses/releases, trace-enabled/disabled budgets, patches, observed outcomes and durable snapshots are in [scenarios.json](scenarios.json) and [gameplay.md](gameplay.md). Aborted long batches and unsuccessful control/timing attempts remain preserved; successful shorter recipes supersede them.

Static gains include header/CHR, vector words, exact maximal uniform non-code tails/row slices,11 scene pointers,11 independently consumer-bounded23-byte descriptors and five encoded episode captions. Structural provenance and natural reachability are separate; artificial execution is explicitly marked. Current-source readability/commentary remains incomplete and belongs to the next phase.

Lead outcomes: CHEAT-001 verified; SCENE-DATA-001 and STATIC-CALL-001 ready for redirect research; ENDING-001 blocked on coupled streaming/boss transition contract, with concrete next method and unchanged retry prohibited. These are ordinary phase leads, not a terminal exhaustion judgment. Productive first-pass static/gameplay easy gains are complete; no deep redirect or commentary phase performed.

Toolkit: one confirmed protocol/input-schema bug NES-001, zero suspected bugs or missing-feature findings; workaround available, no blockers. See [toolkit-issues.md](toolkit-issues.md). No toolkit changes, archive, deletion, publication or ROM modification performed. Disk at final batch:9.2GiB available; scratch2.1MiB, retained evidence173MiB before final small records. No cleanup authorized or performed. Resume requires a new configured-binary serve process, map.load analysis/state.json, emulator.start and import a cited snapshot; old session-local handles are not required.


## Round 002 disassembly completed — ROM incomplete

ROM classification: 100.00% (262157 / 262160 bytes classified; 3 bytes unknown).

Accepted 6381 new code and 79575 new data bytes after independent review of immutable ROM, consumer contracts, exact boundaries and effective contexts. All prior 503 regions preserved verbatim. All3360 graphics columns independently decoded to26tiles with exact payload adjacency; all screen dimensions/counts, final event lengths/FF terminals and34 descriptor/frame counts checked. All7797 distinct proposed code execution views matched configured analysis.decode opcode lengths/assembly. Generic data byte records matched immutable ROM; bounded consumer/pattern inspection reused data-review.md without inventing semantic ownership. Mapper/stack contracts and separate pair0/4/6 preconditions reviewed; natural reachability not asserted. All new annotations Static.

Evidence: [independent review result](round-002-disassembly/review-result.json), [repeat decoder RPC](round-002-disassembly/rpc.jsonl), [review script](round-002-disassembly/review-apply.py), [proposal review recipe](redirect/r001-static-streams/review-recipe.md), [routine contracts](redirect/r001-static-streams/routine-contracts.json), [data review](redirect/r001-static-streams/data-review.md). Existing primary artifacts and scripts inspected; wrong temporary-event extrapolation not inherited. No fresh gameplay replay was needed: broad prior representative natural/artificial routes and newly verified artificial YOU WIN!!/Start dismissal remain in scenarios/gameplay. Boss/natural terminal progression unresolved.

| Bank | Code | Data incl. text/image | Classified | Unknown |
|---|---:|---:|---:|---:|
| header | 0 | 16 | 16 | 0 |
| PRG-00 | 4041 | 4151 | 8192 | 0 |
| PRG-01 | 6912 | 1277 | 8189 | 3 |
| PRG-02 | 0 | 8192 | 8192 | 0 |
| PRG-03 | 0 | 8192 | 8192 | 0 |
| PRG-04 | 0 | 8192 | 8192 | 0 |
| PRG-05 | 0 | 8192 | 8192 | 0 |
| PRG-06 | 0 | 8192 | 8192 | 0 |
| PRG-07 | 0 | 8192 | 8192 | 0 |
| PRG-08 | 0 | 8192 | 8192 | 0 |
| PRG-09 | 0 | 8192 | 8192 | 0 |
| PRG-10 | 0 | 8192 | 8192 | 0 |
| PRG-11 | 0 | 8192 | 8192 | 0 |
| PRG-12 | 0 | 8192 | 8192 | 0 |
| PRG-13 | 0 | 8192 | 8192 | 0 |
| PRG-14 | 7214 | 978 | 8192 | 0 |
| PRG-15 | 1304 | 6888 | 8192 | 0 |
| CHR-00 | 0 | 8192 | 8192 | 0 |
| CHR-01 | 0 | 8192 | 8192 | 0 |
| CHR-02 | 0 | 8192 | 8192 | 0 |
| CHR-03 | 0 | 8192 | 8192 | 0 |
| CHR-04 | 0 | 8192 | 8192 | 0 |
| CHR-05 | 0 | 8192 | 8192 | 0 |
| CHR-06 | 0 | 8192 | 8192 | 0 |
| CHR-07 | 0 | 8192 | 8192 | 0 |
| CHR-08 | 0 | 8192 | 8192 | 0 |
| CHR-09 | 0 | 8192 | 8192 | 0 |
| CHR-10 | 0 | 8192 | 8192 | 0 |
| CHR-11 | 0 | 8192 | 8192 | 0 |
| CHR-12 | 0 | 8192 | 8192 | 0 |
| CHR-13 | 0 | 8192 | 8192 | 0 |
| CHR-14 | 0 | 8192 | 8192 | 0 |
| CHR-15 | 0 | 8192 | 8192 | 0 |

Internal state and maps/current saved, map.validate OK. Fresh [checkpoint](../maps/round-002-disassembly.json), [byte-identical verification](round-002-disassembly-verification.json), [new generated source](../generated/round-002-disassembly/game.asm). Map/state SHA256 d6af065d7fd00334045ea58046b2704e3d4a51caa5f3b6be8db0abc7461ea863. Input SHA256 unchanged. Maintained commentary/source/game.asm unchanged SHA256 cf1851d19476ee60e2409f7ac72cf136d5080616cf4964682e23502bc03b794c; new source commentary/rebase required next.

SCENE-DATA-001, STATIC-CALL-001, EVENT-TERRAIN-001, GRAPHICS-ASSETS-001 and AUDIO-STRUCTURES-001 verified structurally; ENDING-001 verified artificial presentation plus static structure, natural boss route unresolved. ORPHAN-JUMP-001 retains exactly physical$3327..$332A CPU$B317 [0,1,14,15]:4C4DB3. Predecessor JMP bypasses it; adjacent instruction shape does not discriminate orphan code versus data. No arbitrary PC injection or unchanged retry performed. See [prior focused review](redirect/r001-static-streams/deferred-review.md). This is an ordinary disassembly-to-commentary handoff, not terminal exhaustion/archive acceptance.

Toolkit: existing NES-001 one confirmed input-schema bug, zero new findings, no blocker; [toolkit issues](toolkit-issues.md). No toolkit edits, publication, cleanup, deletion or archive. Disk9.0GiB available; scratch2.1MiB retained.

## Round 002 redirect residual review

Completed [r002-orphan-review](redirect/r002-orphan-review/rejection-review.md). ORPHAN-JUMP-001 retains3unknown bytes at physical3327..332A, CPU B317 [0,1,14,15]. Explicitly challenged prior rejection using narrow static epilogue contract; no universal live-caller premise. Shared exit/placement remains compatible with residual storage, and no bounded data pattern establishes generic non-code. Fresh primary complement and literal-origin scan saved; no discriminating new test remains. No independent promising lead after current source/progression/uncertainty review. Terminal incomplete recommendation pending coordinator independent review, not final editorial/archive acceptance. Historical results preserved; no replay or classification changes.
