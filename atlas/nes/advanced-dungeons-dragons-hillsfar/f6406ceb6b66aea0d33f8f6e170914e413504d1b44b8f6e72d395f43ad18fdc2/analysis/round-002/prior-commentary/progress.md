# Round 001 commentary

Phase=commentary, round=1. Prepared frozen pass-001 baseline and verified byte identity. Current-source review in progress; no successful handoff yet. Remaining: all 11,482 classified code bytes and classified text/data boundaries across physical 0..262160. Unknown complement remains 172,578 bytes. Final editorial audit pending.

Read the entire classified code listing: 449 fragments, 7,369 lines. Durable annotation driver analysis/commentary/annotate_round001.py contains specific explanations for the switchable-bank audio, travel, town and sprite/animation groups. Remaining application scope: fixed-bank 30/31 explanations, global symbols, source application and inventory reconciliation, verification and reading refresh. No review credit recorded until source edits are applied.

## Applied source batch

All 449 classified code fragments / 5,574 instructions now have routine-family explanations and 919 bank-aware direct-flow labels; 307 described entry locations and 309 shared symbols are present. All 2,420 physical scaffold entries retain their exact boundaries and source links. First fresh verification after resolving a variable/routine name collision is byte-identical (verification.json). Remaining quality work: replace understood ROM table operands, reconcile comments for reused scratch fields, data/unknown boundary note inspection, then fresh handoff verification/read refresh. Final editorial audit remains pending.

## Round 001 current-source readability complete

Remaining classified-source review ranges: **none**. Covered all 11,482 code bytes (449 fragments / 5,574 instructions), all 72,754 text bytes (869 named runs with previews), all 5,346 other data bytes and explanatory notes for all 1,060 unknown intervals. The complete 262,160-byte review scaffold is intact, with source locations and summaries. Review status records completed readability; final editorial/archive audit remains pending and is not claimed.

Source explains audio music/effect parsing and envelope/APU state, travel route generation/scroll/motion, town side descriptions/maps/status/action flow, character-record checksum, animation/metasprite packing, reset/NMI/controller/DMA, packed graphics and screen layout, window/text wrapping/pagination, decimal formatting, time/recovery and script dispatch. Specific examples include correcting packed negative controls to `(control & $7F)+1` literals, separating town description direction from entry semantics, naming OAM/animation fields and explaining return-address removal in parsers. Operands use 309 shared RAM/hardware symbols and 96 bank-specific consumer aliases; 919 direct-flow labels include 612 mechanically descriptive branch/continuation actions.

Unknown semantics remain explicitly local: character flag meanings, building-entry neighbor contract, missing script selectors/tails, resource stream bounds, sound/frame identities and generic uniform-data purpose. These are research questions, not skipped explanations. See uncertainties.md and analysis/commentary/round001-review-summary.json. New leads R001-COMMENTARY-SCRIPT and R001-COMMENTARY-PACKED support existing redirect avenues.

Fresh byte-identical verification and reading-after.asm were produced after the final source edit. Source hashes are in verification.json; ROM identity is unchanged. Both maps/current.json and analysis/state.json are semantically identical to their frozen commentary baseline. No toolkit issues found. Disk check: about 10 GiB free; commentary 8.3 MiB, commentary evidence 88 KiB, temporary code listing 104 KiB. No archive/delete/publish action performed.
