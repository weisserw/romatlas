# Round 1 commentary handoff

Current-source readability complete; ready for redirect. Whole ROM remains incomplete58.35% (382,424/655,376 classified;272,952 unknown). No classification/map/ROM/toolkit changes and no final editorial audit.

Reviewed all9,832 physical scaffold entries:1,130 Code fragments/22,128 bytes across PRG banks0,1,49,50,52,55,56,57,58,59,60,61,62,63;4,100 Text;68 Data;1 Image;4,533 Unknown. All classified bytes have source-local role/format explanations; every unknown gap explicitly retains uncertainty.496 connected units cover the exact union of classified code (0 missing/extra bytes), including49 internal entries not at scaffold starts. Source spans the complete physical image; units crossing gaps inventory only their classified fragments. See [coverage](readability-coverage.json), [review](../../../commentary/review.json), [progress](../../../commentary/progress.md).

Source now uses descriptive routine/branch/continuation labels with physical bank qualification, consistent RAM/hardware names and known fields/copy lengths/controller constants. No generator region_ labels remain. Shared scratch, unknown field meanings and unclassified/mapped targets retain honest local mechanical explanations. Known examples:
- `B55EmptyActionListTrampoline`: `commentary/source/game.asm:37382`.
- `B58TestBoundedLineReachability`: `commentary/source/game.asm:40944`.
- `B59ScriptChangeAreaSelector`: `commentary/source/game.asm:43478`.
- `B60BeginAudioRepeat`: `commentary/source/game.asm:44606`.
- `B63CallBankedService`: `commentary/source/game.asm:50102`.
- `B63QueuePPUByte`: `commentary/source/game.asm:51692`.
- `B63RandomInWordRange`: `commentary/source/game.asm:52381`.

Residual meanings: record A/B/C game identities, some tactical/stat/action/geometry codes, generic paired graphics-like data consumers and unexplored continuations remain unresolved. No current-source naming/mechanical explanation backlog is known. Source names established mechanics, not original-author intent or unsupported schema extents. [Uncertainties](../../../commentary/uncertainties.md) records significant limitations.

Concrete structural correction lead: physical bank55 CPU$8371..$8376 inclusive / physical[$6E381,$6E387) is presently Static Code but overlaps split four-entry dispatch tables at CPU$8371..$8378 / physical[$6E381,$6E389). Effective8KiB banks[55,53,62,63]. Caller$8330: JSR$83FF, TYA, BEQ$836E; with zero entries A=0, then$836E TAX/BEQ$835B cannot fall through into table bytes. Consumer$835B reads low$8371,X and high$8375,X with X<4. Raw$836E..$8378: AA F0 EA 79 82 B1 CA 83 83 83 83; targets$8379/$8382/$83B1/$83CA. Pointer values do not classify targets. Source explicitly preserves the competing table interpretation; no map changes. [Primary conflict record](bank55-dispatch-conflict.json), frozen source `commentary/before/game.asm`, original static evidence `analysis/round1/static3-candidates.json` and `static-expand3-rpc.jsonl`. Investigate/correct narrowly in redirect/disassembly rather than inheriting the static-successor claim blindly.

Other productive redirect questions retained: coherent later-area transition via VM$9461 restoring saved stack/area loop (saved tuple loader in bank50$95C7); complete VM command/operand slices and native/bytecode boundaries; callee bank-return and inline-dispatch effects. Audio selector bank61 uses8-byte descriptor stride whereas fixed block-A starter copies10 bytes; verify consumer before inventing disjoint record extents. These are bounded research leads, not terminal exhaustion.

Final byte-identical verification true: [verification](../../../commentary/verification.json). Reading copy refreshed afterward: `commentary/reading-after.asm`. ROM SHA2562d31a3d101fd629f8ff159de4ca15785444e4a3deb8b4fd7f114fa5cdef5ce87. Current map/state remain equal to frozen manifest hash e80c440104c673dbfdd0e6feca9ca142015884e5c38339402b6ac44e9cda49f7. Final source hashes:
- `constants.inc`: `28382f17f165f07f4f0e36700fd735430c0c5c37d934e443a92db8742cf6374f`
- `fixed-services.inc`: `494e2de6be037c74d26c952243060500372a3f2bda1752abc94323a0d03ccac1`
- `game.asm`: `feef19921de0f17f40073d03aa82e159c5038c1f529a8ee141ca28b753afa3ba`
- `symbols.inc`: `3479483bb1f3939c6816d880af1a180a76bb41ffc7c9415c5111b49d31c35c29`

Exact reading hash, examples and scope: [summary](summary.json). Final editorial audit remains pending; readable explained unknown bytes are still unknown. No compatibility claim. No toolkit issues found (0 confirmed/0 suspected/0 missing; no blockers); [persistent report](../../toolkit-issues.md). All local evidence, frozen baselines and checkpoints preserved.
