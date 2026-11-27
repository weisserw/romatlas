# Round 2 commentary complete; ROM classification incomplete

round=2 phase=commentary status=current-source-readability-complete.
No pending classified readability ranges. Final targeted editorial acceptance/audit
remains reserved for the coordinator; next redirect follows that gate.

Covered all731 code regions (26849bytes,12426 instruction starts),175 data regions
(104238bytes including header) and explicit unknown1CAF1 note. Original907-entry
scaffold intact;361 functional contracts (223 carried+138new),841 exact code-fragment
attachments. Entire prior commentary preserved at
[prior-commentary](../round-02/prior-commentary/).639 unchanged classified physical
regions carried useful source/review credit and were reverified.

[Source](../../commentary/source/game.asm),[symbols](../../commentary/source/symbols.inc),
[reading copy](../../commentary/reading-after.asm),[review](../../commentary/review.json),
[prose assessment](../round-02/commentary-readability-assessment.md),
[uncertainties](../../commentary/uncertainties.md),
[scope check](../round-02/commentary-scope-check.json).

New prose explains connected handler/phase/channel/link/carry/item mechanics with
conservative names, shared fields, bank contracts and useful local aliases. Asset
notes explain maps/metatiles, compression, animation/image formats, raw CHR groups,
music/palettes, spawn records and DMC storage. Prior unknown-body/backing assertions
were invalidated by the new evidence and replaced, while valid prose was preserved.

Decoder inversion corrected everywhere:01..3Frepeat,40..7Fzero,81..FFliteral,
00/80skip. Additional static edge40 emits128zeroes unless output ends first;
none of6554saved token records uses40, so bounds unchanged
([check](../round-02/decoder-edge-check.json)).26-byte metatiles are25tiles+palette.
Preserve NMI CAB7reentry skip/CABAcallback/CABDdefault and CE9F/CEA0 inline05A7
viaD360->D611, with CEA1resuming code. Ground65=1falling/0support; ground probes
may snap Y. Player action damage lookup has no local range clamp.

Fresh final [verification](../round-02-commentary-verification.json):ok=true,
byte_identical=true; reading refreshed. ROM SHA256 unchanged; state equals frozen
phase1-state; map equals baseline and round-02-disassembly checkpoint; map validation
passes. No structural/toolkit/ROM edits, deletion/cleanup, publication/archive or
commit. All source, assets and evidence stay local.

Residual structural question: isolated physical1CAF1..1CAF2/CPUCAE1 byte60 between
NMI tailJMP and IRQentry lacks incoming edge/discriminating entry or format contract.
Remain unknown; 131087/131088classified rounds100.00% but is incomplete. Redirect
should assess this exact frontier, without assuming RTS/dead-byte classification or
repeating unchanged gameplay. Actor/item identities, score units, natural selector
use, generic fill purpose and fourth zero decimal scale remain semantic uncertainties,
not skipped readability. No new toolkit issue or live process remains.
