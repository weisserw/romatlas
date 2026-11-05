# Round 001 disassembly handoff

ROM classification: 78.61% (206092 / 262160 bytes classified; 56068 bytes unknown).

Outcome **incomplete**. Exactly one substantive disassembly pass completed. Commentary should now review all current classified source; redirect has concrete next investigations. This is not a terminal exhaustion assessment.

## Verified checkpoint

- Final [verification](../pass-001-final-verification.json), [portable map](../../maps/pass-001-final.json), [assembly](../../generated/pass-001-final/game.asm).
- Internal [state](../state.json), current [portable map](../../maps/current.json). Final map SHA256 `9311c48c78733a3ba4333c5140b905cb26fd0c32def1e1ffb49f7c16bc69094a`.
- Baseline and interim `pass-001` also rebuild byte-identically; retained. No deletion/cleanup, toolkit edits, archive execution or publishing performed. Input SHA256 remains `f5008a36287c1a627d73e601d3b0d7d91e7f11fe999956b161c4d4ea51d99e70`.
- Configured `/root/romatlas/target/release/romatlas` used throughout. Direct self-contained session helper [direct_rpc.py](../round-001/direct_rpc.py) works for future experiments; earlier long-lived file-queue daemon stopped externally and is not required.

## Counts and structural evidence

649/649 regions carry evidence: 12,637 Code (8,887 Observed, 3,750 Static), 62,383 Data, 131,072 CHR Image. [Exact counts/per-bank breakdown](../round-001/counts.json), [unknown complement](../round-001/unknown-ranges.json), [progress](../progress.md).

Header physical [0,16) is validated iNES Data. CHR [131088,262160) is declared 16-byte NES pattern-tile backing, Image. Seven lower scene halves are generic Data: [16,8208), [16400,24592), [32784,40976), [49168,57360), [65552,73744), [81936,90128), [98320,106512). Existing 10-byte descriptor fields and maximal uniform-zero annotations retained within those intervals; remaining complements classified without inventing schemas. Evidence combines header-copy consumers ($A0D2/$A5A7 copy CPU $8000..$8009), adjacent upper-half handler contracts, repeated low-domain map/resource patterns over all 32 chunks per bank, and inspection of apparent call bytes inside repeated records. Absence of execution alone was not used. No unsupported full table layout or original object extent is claimed. [Review](../round-001/scene-data-review.json), [hexdump](../round-001/scene-data-hexdump.txt), [exact proposals](../round-001/scene-data-proposals.json).

Code annotations come from accepted contextual opcode roots and bounded conservative control flow. [Static expansion](../round-001/static-call-expansion.json) follows direct JSR targets under pre-call bank context and validated dispatcher entries; it does **not** assume post-call bank preservation. No overlapping interpretations found ([conflict review](../round-001/classification-conflicts.json)). Physical views preserve effective 8 KiB banks. [Initial discovery](../round-001/startup-menus-discovery.json) and per-stage discovery artifacts contain exact boundaries. Runtime and static reachability claims remain separate.

CPU $E333 consumes the JSR return address as an inline little-endian dispatch-table origin, indexes by A*2, then jumps. This independently identifies inline table barriers: title [0x1A088,0x1A0B4), opening [0x1A461,0x1A473), main [0x1F9A1,0x1F9E9), debug actions [0x1A286,0x1A296), ending [0x1A869,0x1A88F). Bounds meet following first handler starts; debug action domain 0..7 additionally has explicit ceiling. Supporting inspections: [helper](../round-001/inline-dispatch.json), [title](../round-001/title-dispatch.json), [main](../round-001/main-dispatch.json), [ending](../round-001/ending-candidate.json).

## Progression and execution evidence

Identified visually as **Adventures of Tom Sawyer**, SETA USA 1989. Natural title/player/start menus and classroom opening visited. [Gameplay notes/sources](../gameplay.md), [milestone checklist](../scenarios.json). No external guide independently establishes structural classification.

Six artificial debug-menu stage routes exercised, with per-stage entry snapshots and mechanics framebuffers. Stage1 platform/sky [8,9,14,15]; stage2 rafting [2,3,14,15]; stage3 forest [0,1,14,15]; stage4 mansion [4,5,14,15]; stage5 clouds [6,7,14,15]; stage6 cave [10,11,14,15]. Frames confirm distinct initialized scenes and movement/jump/attack. Exact [script](../round-001/progression.py), [stage2 resume](../round-001/stage2-resume.py). Stage2 example [image](../round-001/stage-2-mechanics.png). Each `stage-N-contexts.json`, `stage-N-discovery.json`, `stage-N-results.json` retains accepted instruction evidence and trace budgets. Aggregate roots were not dropped; expected detail drops reflect capacity zero.

Recipe: restore `pre-shortcut.snapshot.json`, reassert released inputs; RAM $72=0/$75=5, four frames; $F0=stage+1. The experimental Start pulse does not select debug action; following **A** pulse does. New A pulse after release advances stage title. See scenarios for exact order and budgets. Local inspection establishes $B2=stage/$B4=5*stage−2 setup, normal clear routines and title transitions. One-time $0472=13 does not guarantee persistent invulnerability: normal code clears bit $10 unless $A7 negative. Earlier direct-counter-only shortcut superseded, preserved as attempted evidence.

Ending initialization: restore pre-shortcut; $72=34/$75=3; twenty frames, then $0482=1/A twenty frames. Accepted handler fetches and mode34/substate7 observed, but malformed composition means **ending/credits not verified visually**. [Result](../round-001/ending-results.json), [frame](../round-001/ending-later.png), [durable snapshot](../round-001/ending.snapshot.json). Boss visits deferred because dependent spawn/checkpoint contract was not established; controller inspection retained instead of arbitrary RAM brute force.

Execution metric: 4,450 unique accepted ROM opcode starts across saved natural/artificial scenarios ([summary](../round-001/execution-summary.json)); separate from structural percentage. Commentary readability/editing remains pending.

## Concrete redirect questions and lead outcomes

No inherited ready leads existed. Verified: dispatch helper/tables, six-stage menu route, generic lower-half scene data. Superseded: direct counters without menu initializers. Attempted, unresolved: ending graphics prerequisite. No whole region rejected only for lack of caller/execution.

New [leads](../leads.json):
1. **R001-CALL-CONTINUATIONS**: prove safe leaf routines preserve mapper state, then seed JSR return continuations. Static discovery intentionally stops every call; large fixed-bank residuals remain. This is the clearest next code gain; source/graph inspection needed, not repeated gameplay.
2. **R001-STAGE-HANDLERS**: follow seven scene-header init pointers and observed upper-half routines; preserve inline barriers and bank context, inspect adjacent palette/resource regions rather than requiring semantic recovery for generic data.
3. **R001-ENDING-PRECONDITION**: trace natural writes to $72=$22 and CHR preparation before $A856 (including $AC5F/$C824); compare stage6 snapshot to ending snapshot. Mode edit alone misses a dependency.
4. **R001-BOSS-CHECKPOINT**: inspect stage6 $B848 under [10,11,14,15], object maps and $047A/$047E/$0472 references. Identify a supported spawn/checkpoint transition before trying another shortcut.

No terminal rejection review is asserted: these concrete avenues justify redirect after commentary. Easy startup, broad entry gameplay, direct-call graph and clear lower-half non-code gains have been taken. Remaining connected work needs explicit call-return/dispatch/resource extent or boss/CHR initialization facts, beyond repeating the same bounded routes.

Toolkit: **no toolkit issues found**, no blockers. [Persistent report](../toolkit-issues.md). No implementation changes authorized or performed.
