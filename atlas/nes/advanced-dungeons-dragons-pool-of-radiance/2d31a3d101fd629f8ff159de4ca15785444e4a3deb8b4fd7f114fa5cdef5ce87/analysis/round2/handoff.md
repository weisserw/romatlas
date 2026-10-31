# Round2 disassembly handoff

**Incomplete ROM; this disassembly pass is complete. Next: full current-source commentary, then redirect.** No terminal investigation-exhaustion claim. No emulator/server/session is live.

ROM classification: **84.08% (551040 / 655376 bytes classified; 104336 bytes unknown).** Code46398/Data283439/Text90131/Image131072;10068regions, all with evidence. Exact residual596gaps: `unknown-ranges.json`; exact per-bank counts: `counts.json`/`counts.md` (also saved in progress.md).

Input: pass-001 map/state e80c440104c673dbfdd0e6feca9ca142015884e5c38339402b6ac44e9cda49f7. Output state/current/pass-002 hash **e0da8d0e0a7ea6d7065db7d3276a7302f01d59ca1c7751249e1c74dc166c3b67**. Fresh `analysis/pass-002-verification.json`, `maps/pass-002.json`, `generated/pass-002`: **byte_identical=true,ok=true**, original ROM identity unchanged. Old map/checkpoints, commentary/source/game.asm and all historical evidence preserved; `protected-hashes.json` checks frozen source/old checkpoint/ROM.

Accepted new unique bytes **168616**:163159main redirect candidate union +4029generic uniform Data +1428fresh inventory (1408Code/20Data). Main union is Code22868/Data138777/Text1514. Six existing Static Code bytes become Data separately; totals are unions, not summed overlapping lead totals. Region5274 retains only836E..8371 native TAX/BEQ;8371 begins four-entry split pointer data, empty-list X0 branches back to835B.

## Independent evidence review

`review.md`, `consumer-review.json/.asm`, `grammar-review.json`, `code-audit.json`, `acceptance.json` retain independent role/bounds/effective-bank review of all6441claims. All2226Code claims independently decoded:10460new instructions/22868bytes, exact endpoints,0competing boundaries/context conflicts. Native entry/dispatch/control/stack contracts establish role; legal decoding only checks boundaries.

Independent fresh byte parser reproduces16,314VMcommands/199977bytes/19357modeled edges from29mapped headers, without framing conflicts; native operand/count/control contracts reviewed. Audio6275musiccommands/7445bytes and579effectscommands/652bytes separately reproduce exact bounds. Actor108-byte stride/bank split,117counts/219three-byte actions, six compressed panels,62native VM destinations, EA11stubs/arrays, bounded ending names/credit/tile suffix, inventory/attribute graphs, panelVM and mode9 sparse callbacks independently supported. All accepted ready lead IDs/outcomes/history are in leads.json; no requirement that every Static path execute.

Important historical evidence correction: `effect_streams.py` wrote `audio-streams.json`, overwriting its music result; `effect-streams.json` never existed. Both streams were independently reconstructed into durable **round2/grammar-review.json/review_grammars.py**. New audio annotations and authoritative lead evidence cite these results. Historical scripts/text preserved; this is local research bookkeeping, not a toolkit bug.

Return/bank hazards remain explicit: EA11 and FD00 consume inline/stub returns, no ordinary stub/table fallthrough. Fixed62C783 maps58 in first slot then restores; fixed physical bytes stay62.55:98E0 maps2 intoA000,98D9 maps53; actual55:8F3A calls2:A275, stale53alias excluded.8D94 is RTS; only8D95..8D9B table bytes. Physical cartridge-RAM backing is authoritative when CPU reads are disabled/open bus.

`uniform-review.json`:72exact00/FFruns totaling4029bytes, adjacent content/boundaries and21292native instruction views inspected,0reference/control conflicts. Generic Data; no invented consumer/format/fill meaning.

## Fresh ordinary menu exploration

`fresh-gameplay-review.md` preserves exact input/snapshot/budget/effective-bank details. From prior coherent FB99-enabled overworld snapshot, ordinary controls reached Use item ->Use/Trade/Drop ->Who will use it? ->Gwydion equipped inventory rows. Allowing redraw to settle resolved early-input timing. **Inventory rendering is observed; Sword activation is only attempted, no item-use result/effect claim.** A30-frame wait timed out;12-frame chunks recovered durable evidence. No new RAM/register patches; inherited artificial lineage remains explicit.

`fresh-code-union.json` accepts1408nativebytes/627instructions (327artificial fetched bytes,1081Static) and20finite dispatch/UI Data bytes. Connected code covers physical3inventory renderer/cursor,55Use/Trade/Drop handlers, actual2callbacks and11-byte fixedDA24 helper. Reviewed EA11-restored ordinary continuations; finite four-cursor and three-action callbacks followed. New concrete external destinations resolve to accepted/newCode, with mapper/nonlocal/FD00 boundaries preserved. No more input permutations planned for this sample.

Fresh five-scenario execution union only:6185artificial contextual roots/3477unique ROM starts/6848instructionbytes; all withinCode,0conflicts,0aggregate overflow. Capacity0 detail drops expected. Do not add this union to historical totals. Natural execution unchanged9223starts/18950bytes; no natural world/boss/ending route claimed. Broad natural early/battle plus artificial world/dragon interaction/credits were already visited and preserved in scenarios.json.

## Limits and next phase

Full **current-source commentary is pending** for the enlarged pass-002 source. Prior full commentary is valid for its frozen pass-001 scope and must be preserved/migrated; no source editing was performed here. Existing commentary/source/game.asm hash unchanged. Final editorial audit and complete classification remain separate gates.

Accepted connected batches and a bounded fresh ordinary-input sample are closed. Remaining104336bytes/596gaps are not whole-workflow exhaustion. After commentary, redirect should review newly readable code/data references and largest residual nonuniform patterns, including physical19/24/26 tile/map-looking neighborhoods; generic Data needs no complete semantic schema. Uniform subranges do not automatically prove entire containers. Preserve lower54mixed-prefix bounds, bank17portrait suffix, unbounded60:90F7sequence and global AA83domain limits from redirect. Do not replay unchanged failed area patches, startup or inventory rendering, or misinterpret an unresolved meaning as unresolved non-code classification.

No toolkit issues found:0confirmed/0suspected/0missing features; no blockers (`analysis/toolkit-issues.md`). Configured release binary unchanged. No ROM/toolkit mutation, archive, publication, deletion or cleanup performed.
