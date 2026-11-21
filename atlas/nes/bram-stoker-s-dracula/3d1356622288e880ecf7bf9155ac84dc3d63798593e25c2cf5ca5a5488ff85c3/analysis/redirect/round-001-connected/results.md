# Round 001 connected redirect results

This is a nonterminal proposal batch for independent disassembly. Input classification remains181447/262160;80713 bytes remain unknown in the actual map. Research changed no map, state, source or ROM. No byte quota was used.

## R002: stage maps and graphics

Fourteen consumer-selected compressed streams occupy25194 physical bytes,25189 previously unknown. Exact source banks, CPU pointers and terminating-command boundaries are in stage-streams.json. Bank11 B187 copies command+1 literals for commands0..127; commands128..255 supply a destination-relative backward distance256-command and a following copy count. Count zero returns. Overlapping copies work. All14 streams terminate within their8KiB source banks and produce3072 or6144 bytes.

Preserved negative observation: saved stage6/13 begin snapshots initially matched only1714/1762 decoded bytes, followed byFF. State inspection showed these snapshots were mid-decompressor atB1DD/B1DA with matching source and output pointers. This rejects the assumption that initialization had finished. Advancing each50000 accepted operations then produced byte-identical3072/6144-byte cartridge-RAM maps, zero differences. See stage-runtime-finish.json. No debugger patches in this confirmation; original artificial snapshot provenance remains. Do not replay the intro or unchanged decoder experiments.

Eight metatile families have independent tile, attribute and collision-property pointers. Their differences establish N*4 tile bytes followed by N attribute and N property bytes. All14 decoded maps use indices below N;12 maps attain N-1, while stages1/11 share a129-entry family and attain127. Palette copying independently iterates32 bytes. The spawn initializer C9E2 reads8-byte records through a leadingFF sentinel; all14 lists terminate within17..52 records. graphics-records.json contains exact ranges and records:9946-byte union,9928 unknown. Do not merge unused gaps with structured objects automatically.

Bank13 A000..A1C0 comprises32 adjacent14-byte stage arrays. Direct consumers establish bank, split pointer, geometry, camera, palette and spawn references for many arrays. Remaining non-code fields can use generic Data without invented names. A1C0/A1DC are14-word coordinate arrays read by B517: both unsigned player-coordinate differences must be below32 to set carry.

## R001: screen selector

Bank8 8046..8057 indexes word slots at8160 with054E*2. Nineteen slots8160..8185 end immediately before supported handler8186. Fixed C370 installs banks8/9. Accepted fixed-bank startup/progression code supplies selector18 and other constants; screen handlers increment10 to11 and12 to13. No complete natural selector-domain claim is needed for the38-byte Data proposal. Handler entry contracts, rendering/text helpers, waits and downstream routines were followed in review-cfg.json. Selection of every slot is not claimed observed.

## R003 and actor continuations

Boss table FCB8..FCC5 contains seven words selected by stage&FE under stage0..13. All seven handlers were inspected, including the ten-dispatch threshold that increments058D, display-bit clearing, projectile initializers and coordinate helpers. Existing coffin-ending evidence is retained; no natural boss defeat or natural credits route is claimed. Helpers read bank13 A1F8/A214 using0594; a complete extent for those coordinate tables still requires a producer contract beyond current selected uses.

All28 concrete spawn identifiers from the14 lists were followed. C9E2 stores record-first-byte minus1 at7D80; D6C1 copies that value to actor type0500. D2AD subtracts another1 to select initialization words atFC70. Supported initializer families were inspected; B60B stores the requested actor state and resets the animation frame. No actor names are inferred from record identifiers.

Actor dispatch EB33..EB4D reads type pointers atFDA8 and then state words. The113-word state blockFCC6..FDA7 lies between the independently bounded boss table and separately consumed type-pointer table. All words resolve to coherent handler entry contracts. Forty-five concrete type/state-zero contracts come from bounded spawn records and literal initializers. All113 state-word targets were also inspected under static entry preconditions, without claiming every state has a known natural producer. Exact per-family state extents remain tentative where tables share entries. Generic Data can cover the words without inventing family counts. The type-pointer prefix through type111 is supported; do not claim that111 is the maximum type or thatFE88 ends the whole type table.

## Independent sound, animation and generic data

Sound entry8003 is supported by accepted fixed-bank CD8B channel calls with X0..3 and bank10. Its initialization and command-parser continuations were inspected. The parser distinguishes FC/FD/FE/FF and threshold ranges E0/C0/80/60, uses channel-indexed parameters, pattern loops and instrument records. Full bytecode-grammar recovery is not claimed.

Bank10 841F..9FFF contains period, pointer, envelope, instrument and pattern payload after the last code RTS at841E. Existing consumers and repeated record/pointer patterns support generic Data where a specific subtable layout has not been established. Bank12 8000..9A5D contains animation, sprite, frame and pointer records consumed by E80A/E8D0 after explicit temporary bank12 mapping. Bank11 A000..A3FF contains renderer lookup/pointer records before the four JMP entries atA400. generic-data-inspection.json preserves exact bounds, representative samples and zero accepted-code conflicts. These are generic Data proposals, not claims about original layouts, asset identities or fill purposes.

## Review contracts and deferred questions

review-cfg.json is a bounded static proposal graph, not accepted classification or observed execution. Each JSR return edge explicitly requires independent callee stack/bank review. Some collision contracts discard return addresses with PLA/PLA; review must preserve their caller-specific behavior instead of blindly accepting speculative fallthrough. Toolkit discovery candidates are also review inputs, not classification.

Seven explicit barriers reduce to known classes: graphics-bank mapping from stage array0D (banks1/3/5/7), temporary animation bank12 and saved restoration, MMC3 select0 without a bank-data write, and indirect BA69 dispatch. Separate roots cover the supported continuations. Unknown saved restore values do not prevent decoding a fixed-bank RTS, but do prevent asserting the restored8000-slot bank without a caller fact. The indirect trampoline is resolved here for supported boss/item/spawn contracts; other selectors retain their own missing facts.

The identified productive continuations are accumulated in this batch. Additional coordinate cells, selector values or actor-state domains require a concrete producer or independently bounded consumer beyond the contracts above. Arbitrary sweeps would not establish those structures. Unproposed gaps retain code-versus-data or boundary ambiguity; this is not terminal exhaustion. Semantic sprite identities and rare natural routes have diminished prospects of improving this structural batch. Reassess remaining gaps after accepted annotations change their neighboring contracts. No terminal rejection review is required for this ready, nonterminal handoff.
