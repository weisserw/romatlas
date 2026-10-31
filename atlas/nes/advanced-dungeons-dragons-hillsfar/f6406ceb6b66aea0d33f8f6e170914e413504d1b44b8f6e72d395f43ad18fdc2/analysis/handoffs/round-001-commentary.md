# Round 001 commentary handoff

**Full current-source readability complete. ROM incomplete.** Ready for separate redirect; no remaining classified-source ranges are deferred. Final whole-ROM editorial/archive acceptance remains pending and was not invoked.

Scope: all 11,482 code bytes (449 fragments / 5,574 instructions), 72,754 text bytes (869 content-named runs with previews), 5,346 other data bytes and boundary/ambiguity explanations for all 1,060 unknown intervals. Full 262,160-byte scaffold retained: 2,420 reviewed entries with source locations; meaningful routine records retain physical scope/context. Classification is unchanged: 89,582/262,160 (34.17%), 172,578 unknown. Gameplay coverage is unchanged.

[Readable source](../../commentary/source/game.asm), [shared/table symbols](../../commentary/source/symbols.inc), [reading copy](../../commentary/reading-after.asm), [review inventory](../../commentary/review.json), [scope/consistency checks](../commentary/round001-review-summary.json), [progress](../../commentary/progress.md), [semantic uncertainties](../../commentary/uncertainties.md).

307 explained entry locations, 919 direct-flow labels (612 continuation/branch labels name the destination's mechanical action), 309 RAM/hardware symbols and 96 consumer-named ROM aliases cover music/effects, travel generation/scroll/motion, town maps/status/descriptions/actions, character records, sprite/animation packing and fixed-bank reset/NMI/controller/DMA/graphics/window/text/number/time/script services. Names describe established mechanics; classification fragments are not presented as original routine boundaries.

Examples: [packed graphics](../../commentary/source/game.asm:13259) and [screen tiles](../../commentary/source/game.asm:15140) use zero termination, positive control+1 repeat copies and negative `(control & $7F)+1` literals. [Town side descriptions](../../commentary/source/game.asm:7499) inspect front/left/right separately; [tile lookup](../../commentary/source/game.asm:8134) uses page0 north-high/east-low and page1 south-high/west-low. [Metasprite renderer](../../commentary/source/game.asm:10743) explains packing, palette cache, clipping and OAM output. [Script entry](../../commentary/source/game.asm:15744) describes saved action parameters, bank selection and handler dispatch. [Random helper](../../commentary/source/game.asm:15693) documents X/Y preconditions and the FF exclusive-bound wrap edge case.

Remaining meanings are localized, with no readability backlog: building-entry side/neighbor/action contract; script selector/parameter/handler tails; exact graphics resource bounds; character class/quest flags and clock scale; specific sound/frame identities; prose record/control ownership; generic uniform-data purpose. Unknown ranges remain unknown, including numeric ROM aliases named from a consumer without a complete table-extent claim. Fixed-bank calls into newly switched CPU aliases retain unresolved numeric targets where physical context is unestablished.

New leads in [leads](../leads.json):

- **R001-COMMENTARY-SCRIPT:** connect town C5C9 search to F06D, $07A7 bank selector / $079B script index and four-byte directory; follow opcode and $074E conditional-branch consumers. Supports existing R001-BUILDING-ENTRY / R001-LATER-ENDING. No guessed handler jump or flag brute force.
- **R001-COMMENTARY-PACKED:** corrected exact repeat/literal counts plus D2B9 bank grouping / D362 little-endian pointer pairs / E4BD screen definitions give a concrete route to bound streams for R001-GRAPHICS-CONSUMER. No whole-bank inference.

Preserve and continue the prior independent graph, building, graphics and later-ending leads; this is not terminal exhaustion. The saved gameplay21 snapshot remains the useful target-range location seed. No new emulator scenario was needed for this mechanical readability work.

Fresh final [verification](../../commentary/verification.json) is byte-identical and the reading copy was refreshed after the final source edit. Source SHA-256: game.asm `7684f2079aa8db918251379d86815a935d5cbe5a9ed05f1c5a79cbe1615449b4`; symbols.inc `7f6a3ee11c152d65b60e3dd0957878987eba53fd602e70615c021b8228ba6082`. Input SHA-256 unchanged. Current portable map and internal state equal the frozen commentary baseline/phase1 state; configured toolkit/binary unchanged. Prior coherent-batch verification retained in analysis/commentary/batch02-verification.json. Annotation/refinement scripts retained as local working evidence, with final source authoritative.

**No toolkit issues found**; zero new bugs/features/blockers ([report](../toolkit-issues.md)). Approximately 10 GiB disk space free at final check. All ROM/source/assets/traces remain local. No archive, deletion or publication performed.
