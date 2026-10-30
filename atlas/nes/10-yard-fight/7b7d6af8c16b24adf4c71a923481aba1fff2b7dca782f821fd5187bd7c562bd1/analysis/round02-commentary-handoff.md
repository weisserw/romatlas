# Round02 commentary handoff

Current-source readability is complete for pass-003. All 40,976 bytes are represented by 133 reviewed physical scopes and 309 reviewed units; no pending readability range remains. Whole-ROM structural classification is also complete, but final editorial acceptance remains pending after redirect. No final audit was run.

Fresh byte-identical verification and reading refresh succeeded. Source SHA-256 `d21ffc43c25297a386d3955220258d342cc66b9c8acf0ddfecf4949a50513444`. [Verification](../commentary/verification.json), [current source](../commentary/source/game.asm), [reading copy](../commentary/reading-after.asm), [scope and pointer accounting](round-02/commentary-accounting.json). Map/state SHA-256 remain `a2c637a340703bcd0cbafb8a4e8106d75d5ddbfa82060fd22bd82d615ca09960`. Input unchanged SHA-256 `7b7d6af8c16b24adf4c71a923481aba1fff2b7dca782f821fd5187bd7c562bd1`.

Entire prior commentary preserved at [commentary-pass-002](round-02/commentary-pass-002/). Carried 260 valid prior units across 98 unchanged classified regions by physical identity, refreshed source locations and affected dependencies. The 19 obsolete unknown scopes became 35 new segments (+1,884 Code/+216 Data), with 49 new semantic units. Coverage: Code 13,174, Data 19,610, CHR 8,192; total 40,976. All sixteen player-state and both four-stage pointer tables resolve to named instruction boundaries.

New explanations cover contact/sequence state dispatch, countdown and animation, signed relative steering/grid clipping, nonlocal player-return stack behavior, seven-record OAM number display, bounded half presentation, pending score addend/selection, demo audio guard and stream initialization. Numeric direct operands use RAM/hardware/record/table symbols. Old unknown-island prose was replaced; source does not invent a complete sixteen-cell domain for eleven-byte animation data.

The $CC0E prefix is explained narrowly: the initial STA targets the following indexed LDA source-page operand in immutable NROM and cannot change it. Its suffix disables rendering and copies **64 bytes** from OamBuffer to OAMDATA, then restores PPU state; caller supplies OAMADDR. No working self-modification, RAM relocation, natural caller or historical intent is claimed.

Remaining meanings are [documented limitations](../commentary/uncertainties.md): natural selectors/callers, exact football meanings of contact/results, some animation domains and reused scratch fields. No structural contradiction, deferred readability or concrete editorial defect is known. Coordinator should complete redirect gate, then perform final editorial acceptance using valid carried review; no mandatory whole-source reread is requested.

No toolkit changes/issues added, map/classification edits, runtime execution claims, publication, archive phase or deletion. ROM/source/assets/state remain local. Approximately 12 GiB disk free. Historical records and frozen before baselines retained.
